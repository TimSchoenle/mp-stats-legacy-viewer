use anyhow::Result;
use mp_stats_common::compression::{read_lzma_raw, write_lzma_bin};
use mp_stats_core::models::{GameLeaderboardData, LeaderboardMeta, MetaFile, PlatformEdition};
use mp_stats_core::{routes, HistoricalSnapshot};
use rayon::prelude::*;
use smol_str::SmolStr;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use walkdir::WalkDir;

fn read_history_data(history_in: &Path) -> Result<Vec<HistoricalSnapshot>> {
    // Decompress the .xz file first
    let decompressed_tar = read_lzma_raw(&*history_in)?;

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

            if file_name == "_meta.json" {
                if let Ok(meta) = serde_json::from_reader::<_, MetaFile>(BufReader::new(entry)) {
                    snapshots.push(HistoricalSnapshot {
                        snapshot_id: SmolStr::new(&snapshot_name),
                        timestamp: meta.save_time_unix,
                        total_pages: meta.total_pages,
                        total_entries: meta.total_entries,
                    });
                }
            }
        }
    }

    Ok(snapshots)
}

/// Process and aggregate game metadata from leaderboards
pub fn process_game_metadata(platform: &PlatformEdition, in_path: &Path, base_out: &Path) -> Result<()> {
    let lb_in = in_path.join("leaderboards");

    // Group by Game using WalkDir
    let mut game_dirs: HashMap<String, Vec<(String, String, std::path::PathBuf)>> = HashMap::new();

    let walker = WalkDir::new(&lb_in).min_depth(3).max_depth(3);

    for entry in walker.into_iter().filter_map(|e| e.ok()) {
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

    game_dirs.par_iter().for_each(|(game_id, stats)| {
        let mut meta_stats: HashMap<SmolStr, HashMap<SmolStr, LeaderboardMeta>> = HashMap::new();

        for (board, stat, stat_path) in stats {
            // Check latest folder for count
            let latest_meta = stat_path.join("latest");
            let mut latest = None;
            if latest_meta.exists() {
                if let Ok(file) = File::open(latest_meta.join("_meta.json")) {
                    if let Ok(meta) = serde_json::from_reader::<_, MetaFile>(BufReader::new(file)) {
                        latest = Some(HistoricalSnapshot {
                            snapshot_id: SmolStr::new("latest"),
                            timestamp: meta.save_time_unix,
                            total_pages: meta.total_pages,
                            total_entries: meta.total_entries,
                        });
                    }
                }
            }

            let history_in = stat_path.join("history.tar.xz");
            let snapshots = read_history_data(&history_in).unwrap_or_default();

            if !snapshots.is_empty() {
                println!("Found {} snapshots for {}/{}/{}", snapshots.len(), platform, game_id, stat);
            }

            meta_stats
                .entry(SmolStr::new(stat))
                .or_default()
                .insert(SmolStr::new(board), LeaderboardMeta {
                    snapshots,
                    latest,
                });
        }

        let game_data = GameLeaderboardData {
            game_id: SmolStr::new(game_id),
            game_name: SmolStr::new(game_id),
            stats: meta_stats,
        };

        let relative_out_path = routes::game_bin(platform, game_id);
        let out_path = base_out.join(relative_out_path);
        let _ = write_lzma_bin(&out_path, &game_data);

        // Write debug json
        let debug_out_path = out_path.with_extension("json");
        let _ = serde_json::to_writer_pretty(File::create(debug_out_path).unwrap(), &game_data);
    });

    Ok(())
}
