//! Step 3b: one metadata file per game, assembled from the leaderboards step 3 just wrote.

use anyhow::Result;
use mp_stats_common::compression::{read_lzma_bin, read_lzma_raw, write_lzma_bin};
use mp_stats_core::models::{
    GLOBAL_BOARD, GameLeaderboardData, IdMap, LeaderboardMeta, LeaderboardPage, MetaFile,
    PlatformEdition, TopEntry,
};
use mp_stats_core::{HistoricalSnapshot, routes};
use rayon::prelude::*;
use smol_str::SmolStr;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use walkdir::WalkDir;

fn read_history_data(history_in: &Path) -> Result<Vec<HistoricalSnapshot>> {
    // Decompress the .xz file first
    let decompressed_tar = read_lzma_raw(history_in)?;

    // Now extract the tar archive
    let mut archive = tar::Archive::new(std::io::Cursor::new(decompressed_tar));

    let mut snapshots = Vec::new();

    // Extract all files and group by snapshot
    for entry_result in archive.entries()? {
        let entry = entry_result?;
        let path = entry.path()?;
        let path_str = path.to_string_lossy().to_string();

        if let Some(slash_pos) = path_str.find('/') {
            let snapshot_name = path_str[..slash_pos].to_string();
            let file_name = path_str[slash_pos + 1..].to_string();

            if file_name == "_meta.json"
                && let Ok(meta) = serde_json::from_reader::<_, MetaFile>(BufReader::new(entry))
            {
                snapshots.push(HistoricalSnapshot {
                    snapshot_id: SmolStr::new(&snapshot_name),
                    timestamp: meta.save_time_unix,
                    total_pages: meta.total_pages,
                    total_entries: meta.total_entries,
                });
            }
        }
    }

    Ok(snapshots)
}

/// The leading entry of a board, read back off the first row of the first page this run wrote.
///
/// Reading it beats recomputing it: the page is already in rank order, so the answer is the row
/// at index 0. `None` when the page is missing, unreadable or empty.
fn read_top_entry(
    platform: &PlatformEdition,
    base_out: &Path,
    board: &str,
    game: &str,
    stat: &str,
) -> Option<TopEntry> {
    let relative = routes::leaderboard_chunk_bin(platform, board, game, stat, 0);
    let page_path = base_out.join(relative);
    if !page_path.exists() {
        return None;
    }

    let page: LeaderboardPage = read_lzma_bin(&page_path).ok()?;

    let uuid = page.uuids.into_iter().next()?;
    let name = page.names.into_iter().next()?;
    let score = page.scores.into_iter().next()?;

    Some(TopEntry { uuid, name, score })
}

/// Writes `games/<game>.bin.xz` for every game found under `in_path`, and returns each game's
/// snapshot count so the caller can fold it back into the ID map.
///
/// A game's directory names it. Its boards, their snapshots and their totals come from the dumps'
/// own `_meta.json` and history archives, and the leading entry comes from the pages under
/// `base_out` — which is why this has to run after the leaderboards and not beside them.
///
/// A game whose metadata will not write is reported by nothing and leaves no file. The run
/// continues.
///
/// # Errors
///
/// If the leaderboard tree cannot be walked.
///
/// # Panics
///
/// If a directory three levels under `leaderboards` has no parent or no name, which the walk it
/// came from cannot produce.
pub fn process_game_metadata(
    platform: &PlatformEdition,
    in_path: &Path,
    base_out: &Path,
    id_map: &IdMap,
) -> Result<HashMap<SmolStr, u64>> {
    let lb_in = in_path.join("leaderboards");

    // Group by Game using WalkDir
    let mut game_dirs: HashMap<String, Vec<(String, String, std::path::PathBuf)>> = HashMap::new();

    let walker = WalkDir::new(&lb_in).min_depth(3).max_depth(3);

    for entry in walker.into_iter().filter_map(std::result::Result::ok) {
        if entry.file_type().is_dir() {
            let path = entry.path();
            let stat_name = path.file_name().unwrap().to_string_lossy().to_string();
            let game_dir = path.parent().unwrap();
            let game_name = game_dir.file_name().unwrap().to_string_lossy().to_string();
            let board_dir = game_dir.parent().unwrap();
            let board_name = board_dir.file_name().unwrap().to_string_lossy().to_string();

            game_dirs.entry(game_name.clone()).or_default().push((
                board_name,
                stat_name,
                path.to_path_buf(),
            ));
        }
    }

    let snapshot_totals: HashMap<SmolStr, u64> = game_dirs
        .par_iter()
        .map(|(game_id, stats)| {
            let mut meta_stats: HashMap<SmolStr, HashMap<SmolStr, LeaderboardMeta>> =
                HashMap::new();
            let mut total_entries: u64 = 0;
            let mut total_snapshots: u64 = 0;

            for (board, stat, stat_path) in stats {
                let mut all_snapshots = Vec::new();

                // Check latest folder for count
                let latest_meta = stat_path.join("latest");
                if latest_meta.exists()
                    && let Ok(file) = File::open(latest_meta.join("_meta.json"))
                    && let Ok(meta) = serde_json::from_reader::<_, MetaFile>(BufReader::new(file))
                {
                    total_entries = total_entries.saturating_add(u64::from(meta.total_entries));
                    all_snapshots.push(HistoricalSnapshot {
                        snapshot_id: SmolStr::new("latest"),
                        timestamp: meta.save_time_unix,
                        total_pages: meta.total_pages,
                        total_entries: meta.total_entries,
                    });
                }

                // Only the global (all-time) board exposes the per-category "game
                // list" stats. Read its already-produced latest leaderboard page to
                // find the `#1 holder` (highest score); other boards stay `None`.
                let top = if board.eq_ignore_ascii_case(GLOBAL_BOARD) {
                    read_top_entry(platform, base_out, board, game_id, stat)
                } else {
                    None
                };

                let history_in = stat_path.join("history.tar.xz");
                if let Ok(history_snapshots) = read_history_data(&history_in) {
                    all_snapshots.extend(history_snapshots);
                }

                total_snapshots = total_snapshots.saturating_add(all_snapshots.len() as u64);

                meta_stats.entry(SmolStr::new(stat)).or_default().insert(
                    SmolStr::new(board),
                    LeaderboardMeta {
                        snapshots: all_snapshots,
                        top,
                    },
                );
            }

            let mut game_friendly_name = game_id.clone();
            let mut description = None;

            for val in id_map.games.values() {
                if val.name == game_id.as_str() {
                    game_friendly_name = val.name.to_string();
                    description = val.description.clone();
                    break;
                }
            }

            let game_data = GameLeaderboardData {
                game_id: SmolStr::new(game_id),
                game_name: SmolStr::new(game_friendly_name),
                description,
                icon: None,
                stats: meta_stats,
                total_entries,
                total_snapshots,
            };

            let relative_out_path = routes::game_bin(platform, game_id);
            let out_path = base_out.join(relative_out_path);
            let _ = write_lzma_bin(&out_path, &game_data);

            (SmolStr::new(game_id), total_snapshots)
        })
        .collect();

    Ok(snapshot_totals)
}
