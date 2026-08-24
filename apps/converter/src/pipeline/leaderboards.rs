//! Step 3: the dumps' id-and-score buffers, re-paged and re-ranked into columnar pages.

use crate::models::leaderboard::binary_leaderboard;
use anyhow::Result;
use mp_stats_common::compression::{decompress_file_auto, read_lzma_raw, write_lzma_bin};
use mp_stats_common::formats::raw::ENTRIES_PER_PAGE;
use mp_stats_core::models::{CompetitionRanker, LeaderboardPage, PlatformEdition};
use rayon::prelude::*;
use smol_str::SmolStr;
use std::collections::HashMap;
use std::fs::{self};
use std::io::Read;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

const LEADERBOARD_SIZE: usize = crate::models::leaderboard::BINARY_LEADERBOARD_SIZE;

/// Converts every board under `java_in`, in parallel, one board at a time.
///
/// A board that fails is dropped silently and the rest of the run continues, so a missing output
/// directory is a partial conversion rather than a failed one.
///
/// # Errors
///
/// Never. The walk cannot fail and every per-board failure is swallowed; the result type matches
/// the other steps.
pub fn process_java_leaderboards(
    platform: &PlatformEdition,
    java_in: &Path,
    output_dir: &Path,
    lookup_map: &HashMap<String, (String, String)>,
) -> Result<()> {
    let lb_in = java_in.join("leaderboards");

    let walker = WalkDir::new(&lb_in).into_iter();
    // Filter for .../latest directories
    let latest_dirs: Vec<PathBuf> = walker
        .filter_map(std::result::Result::ok)
        .filter(|e| e.file_type().is_dir() && e.file_name() == "latest")
        .map(|e| e.path().to_path_buf())
        .collect();

    println!(
        "Found {} leaderboard 'latest' directories.",
        latest_dirs.len()
    );

    latest_dirs.par_iter().for_each(|latest_dir| {
        let _ = process_single_leaderboard(platform, latest_dir, output_dir, lookup_map);
    });

    Ok(())
}

/// Converts one `<board>/<game>/<stat>` directory: its current pages, then its history archive.
fn process_single_leaderboard(
    platform: &PlatformEdition,
    latest_in: &Path,
    output_dir: &Path,
    lookup_map: &HashMap<String, (String, String)>,
) -> Result<()> {
    // Structure: .../[board]/[game]/[stat]/latest
    let stat_dir = latest_in.parent().unwrap();
    let game_dir = stat_dir.parent().unwrap();
    let board_dir = game_dir.parent().unwrap();

    let stat_name = stat_dir.file_name().unwrap();
    let game_name = game_dir.file_name().unwrap();
    let board_name = board_dir.file_name().unwrap();

    // Output Paths
    // TODO: Correctly migrate to routes
    let out_stat_dir = output_dir
        .join(platform.directory_name())
        .join("leaderboards")
        .join(board_name)
        .join(game_name)
        .join(stat_name);
    std::fs::create_dir_all(&out_stat_dir)?;
    let out_latest = out_stat_dir.join("latest");
    std::fs::create_dir_all(&out_latest)?;

    // Process Latest Chunks
    process_latest_chunks(latest_in, &out_latest, lookup_map)?;

    // Process History (now using rich format with lookup_map)
    process_history(stat_dir, &out_stat_dir, lookup_map)?;

    Ok(())
}

/// Converts the current snapshot, copying the dump's `_meta.json` beside the pages.
///
/// The chunks are sorted by name before they are read, because ranking is a running count across
/// all of them and reading them out of order would number the whole board wrongly.
fn process_latest_chunks(
    latest_in: &Path,
    out_latest: &Path,
    lookup_map: &HashMap<String, (String, String)>,
) -> Result<()> {
    let mut chunk_files = Vec::new();

    for entry in fs::read_dir(latest_in)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().is_some_and(|e| e == "xz") {
            chunk_files.push(path);
        } else if path.file_name().unwrap() == "_meta.json" {
            fs::copy(path, out_latest.join("_meta.json"))?;
        }
    }

    // Sort chunks by filename to ensure correct order - CRITICAL for maintaining rank order
    chunk_files.sort();

    // Load compressed chunks and decompress them
    let decompressed_chunks: Vec<Vec<u8>> = chunk_files
        .iter()
        .filter_map(|path| match decompress_file_auto(path) {
            Ok(data) => {
                println!(
                    "Processing chunk {:?} with {} entries...",
                    path,
                    data.len() / LEADERBOARD_SIZE
                );
                Some(data)
            }
            Err(e) => {
                eprintln!("Failed to decompress chunk {path:?}: {e}");
                None
            }
        })
        .collect();

    // Process chunks using shared logic
    process_binary_chunks(&decompressed_chunks, out_latest, lookup_map)?;

    Ok(())
}

/// Turns a run of dump chunks into numbered pages, and returns how many pages were written and
/// how many entries went into them.
///
/// `chunks` has to be in board order: ranks are assigned as the entries stream past, and one page
/// is closed every [`ENTRIES_PER_PAGE`] entries. Entries whose player id is zero or is not in
/// `lookup_map` are dropped rather than written unnamed, so a page can hold fewer entries than the
/// dump chunk it came from — and a page that fails to write is not counted, which shifts every
/// page after it down by one.
fn process_binary_chunks(
    chunks: &[Vec<u8>],
    output_dir: &Path,
    lookup_map: &HashMap<String, (String, String)>,
) -> Result<(u32, u32)> {
    let mut output_index = 0;
    let mut current_page = LeaderboardPage {
        ranks: Vec::with_capacity(ENTRIES_PER_PAGE),
        uuids: Vec::with_capacity(ENTRIES_PER_PAGE),
        names: Vec::with_capacity(ENTRIES_PER_PAGE),
        scores: Vec::with_capacity(ENTRIES_PER_PAGE),
    };
    // Standard competition ranking ("1224"): entries sharing the same score
    // receive the same rank, and the next distinct score jumps to its positional
    // index. The shared `CompetitionRanker` keeps this identical to the
    // player-profile pipeline.
    let mut ranker = CompetitionRanker::new();
    let mut total_entries_written = 0u32;

    for chunk_data in chunks {
        let count = chunk_data.len() / LEADERBOARD_SIZE;

        for i in 0..count {
            let offset = i * LEADERBOARD_SIZE;
            if offset + LEADERBOARD_SIZE > chunk_data.len() {
                eprintln!(
                    "Invalid chunk data: offset {} + size {} > chunk size {}",
                    offset,
                    LEADERBOARD_SIZE,
                    chunk_data.len()
                );
                break;
            }

            let view =
                binary_leaderboard::View::new(&chunk_data[offset..offset + LEADERBOARD_SIZE]);
            let pid = view.player_id().read();
            let score = view.score().read();

            if pid == 0 {
                eprintln!("Invalid player ID: {pid}");
                continue;
            }

            // Resolve Name/UUID
            let pid_str = pid.to_string();
            if let Some((uuid, name)) = lookup_map.get(&pid_str) {
                // Compute rank: same score shares the rank of the first entry
                // that achieved it, otherwise it takes the current position.
                let rank = ranker.next_rank(score);

                // Add to current page (columnar format)
                current_page.ranks.push(rank);
                current_page.uuids.push(SmolStr::new(uuid));
                current_page.names.push(SmolStr::new(name));
                current_page.scores.push(score);

                total_entries_written += 1;

                // If page full, write it
                if current_page.ranks.len() >= ENTRIES_PER_PAGE {
                    let dest_name = format!("chunk_{output_index:04}.bin.xz");
                    let dest_path = output_dir.join(dest_name);
                    if let Err(e) = write_lzma_bin(&dest_path, &current_page) {
                        eprintln!("Failed to write page {dest_path:?}: {e}");
                    } else {
                        output_index += 1;
                    }
                    // Reset page
                    current_page = LeaderboardPage {
                        ranks: Vec::with_capacity(ENTRIES_PER_PAGE),
                        uuids: Vec::with_capacity(ENTRIES_PER_PAGE),
                        names: Vec::with_capacity(ENTRIES_PER_PAGE),
                        scores: Vec::with_capacity(ENTRIES_PER_PAGE),
                    };
                }
            } else {
                eprintln!("Failed to resolve player ID: {pid_str}");
            }
        }
    }

    // Write remaining entries
    if !current_page.ranks.is_empty() {
        let dest_name = format!("chunk_{output_index:04}.bin.xz");
        let dest_path = output_dir.join(dest_name);
        if let Err(e) = write_lzma_bin(&dest_path, &current_page) {
            eprintln!("Failed to write final page {dest_path:?}: {e}");
        } else {
            output_index += 1;
        }
    }

    println!(
        "Processed {} chunks with {} total entries",
        chunks.len(),
        total_entries_written
    );

    Ok((output_index, total_entries_written))
}

/// Converts every archived snapshot in the board's `history.tar.xz` into its own directory of
/// pages.
///
/// The archive is solid: the chunks inside it are uncompressed and LZMA covers the whole tarball
/// at once, so it has to be expanded in memory before any snapshot can be read. A board with no
/// archive is not an error.
fn process_history(
    stat_dir: &Path,
    out_stat_dir: &Path,
    lookup_map: &HashMap<String, (String, String)>,
) -> Result<()> {
    let history_in = stat_dir.join("history.tar.xz");
    if !history_in.exists() {
        println!("No history found for {}", stat_dir.display());
        return Ok(());
    }

    let history_out = out_stat_dir.join("history");
    fs::create_dir_all(&history_out)?;

    println!("Extracting history archive: {}", history_in.display());

    // Decompress the .xz file first
    let decompressed_tar = read_lzma_raw(&history_in)?;

    // Now extract the tar archive
    let mut archive = tar::Archive::new(std::io::Cursor::new(decompressed_tar));

    let mut snapshot_data: HashMap<String, Vec<Vec<u8>>> = HashMap::new();

    // Extract all files and group by snapshot
    for entry_result in archive.entries()? {
        let mut entry = entry_result?;
        let path = entry.path()?;
        let path_str = path.to_string_lossy().to_string();

        if let Some(slash_pos) = path_str.find('/') {
            let snapshot_name = path_str[..slash_pos].to_string();
            let file_name = path_str[slash_pos + 1..].to_string();

            if file_name.starts_with("chunk_") && file_name.ends_with(".bin") {
                // Read chunk data (uncompressed in the tar)
                let mut contents = Vec::new();
                entry.read_to_end(&mut contents)?;
                snapshot_data
                    .entry(snapshot_name)
                    .or_default()
                    .push(contents);
            }
        }
    }

    println!(
        "Processing {} history snapshots in parallel...",
        snapshot_data.len()
    );

    snapshot_data
        .par_iter()
        .for_each(|(snapshot_name, chunks)| {
            let snapshot_out = history_out.join(snapshot_name);
            if let Err(e) = fs::create_dir_all(&snapshot_out) {
                eprintln!("Failed to create directory {snapshot_out:?}: {e}");
                return;
            }

            println!("Processing history snapshot: {snapshot_name}");

            // Process chunks using shared logic
            let (output_index, total_entries_written) =
                match process_binary_chunks(chunks, &snapshot_out, lookup_map) {
                    Ok(result) => result,
                    Err(e) => {
                        eprintln!("Failed to process chunks for {snapshot_name}: {e}");
                        return;
                    }
                };

            println!(
                "  {snapshot_name} - Wrote {output_index} pages with {total_entries_written} total entries"
            );
        });

    Ok(())
}
