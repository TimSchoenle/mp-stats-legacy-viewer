use crate::models::leaderboard::binary_leaderboard;
use anyhow::Result;
use mp_stats_common::compression::{decompress_file_auto, read_lzma_raw, write_lzma_bin};
use mp_stats_common::formats::raw::ENTRIES_PER_PAGE;
use mp_stats_core::models::{LeaderboardPage, PlatformEdition};
use rayon::prelude::*;
use smol_str::SmolStr;
use std::collections::HashMap;
use std::fs::{self};
use std::io::Read;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

const LEADERBOARD_SIZE: usize = crate::models::leaderboard::BINARY_LEADERBOARD_SIZE;

/// Process all Java leaderboards
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
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_dir() && e.file_name() == "latest")
        .map(|e| e.path().to_path_buf())
        .collect();

    println!(
        "Found {} leaderboard 'latest' directories.",
        latest_dirs.len()
    );

    latest_dirs.par_iter().for_each(|latest_dir| {
        let _ = process_single_leaderboard(platform, latest_dir, &output_dir, lookup_map);
    });

    Ok(())
}

/// Process a single leaderboard directory
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
    let out_stat_dir = output_dir.join(platform.directory_name()).join("leaderboards").join(board_name).join(game_name).join(stat_name);
    std::fs::create_dir_all(&out_stat_dir)?;
    let out_latest = out_stat_dir.join("latest");
    std::fs::create_dir_all(&out_latest)?;

    // Process Latest Chunks
    process_latest_chunks(latest_in, &out_latest, lookup_map)?;

    // Process History (now using rich format with lookup_map)
    process_history(stat_dir, &out_stat_dir, lookup_map)?;

    Ok(())
}

/// Process latest leaderboard chunks
fn process_latest_chunks(
    latest_in: &Path,
    out_latest: &Path,
    lookup_map: &HashMap<String, (String, String)>,
) -> Result<()> {
    let mut chunk_files = Vec::new();

    for entry in fs::read_dir(latest_in)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().map_or(false, |e| e == "xz") {
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
                eprintln!("Failed to decompress chunk {:?}: {}", path, e);
                None
            }
        })
        .collect();

    // Process chunks using shared logic
    let (output_index, total_entries_written) =
        process_binary_chunks(&decompressed_chunks, out_latest, lookup_map)?;

    Ok(())
}

/// Shared logic to process binary chunks and convert to rich format
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
    let mut global_rank = 1;
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

            if pid <= 0 {
                eprintln!("Invalid player ID: {}", pid);
                continue;
            }

            // Resolve Name/UUID
            let pid_str = pid.to_string();
            if let Some((uuid, name)) = lookup_map.get(&pid_str) {
                // Add to current page (columnar format)
                current_page.ranks.push(global_rank);
                current_page.uuids.push(SmolStr::new(uuid));
                current_page.names.push(SmolStr::new(name));
                current_page.scores.push(score);

                global_rank += 1;
                total_entries_written += 1;

                // If page full, write it
                if current_page.ranks.len() >= ENTRIES_PER_PAGE {
                    let dest_name = format!("chunk_{:04}.bin.xz", output_index);
                    let dest_path = output_dir.join(dest_name);
                    if let Err(e) = write_lzma_bin(&dest_path, &current_page) {
                        eprintln!("Failed to write page {:?}: {}", dest_path, e);
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
                eprintln!("Failed to resolve player ID: {}", pid_str);
            }
        }
    }

    // Write remaining entries
    if !current_page.ranks.is_empty() {
        let dest_name = format!("chunk_{:04}.bin.xz", output_index);
        let dest_path = output_dir.join(dest_name);
        if let Err(e) = write_lzma_bin(&dest_path, &current_page) {
            eprintln!("Failed to write final page {:?}: {}", dest_path, e);
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

/// Process historical leaderboard data using rich format (same as latest)
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
    let decompressed_tar = read_lzma_raw(&*history_in)?;

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
                    .or_insert_with(|| Vec::new())
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
                eprintln!("Failed to create directory {:?}: {}", snapshot_out, e);
                return;
            }

            println!("Processing history snapshot: {}", snapshot_name);

            // Process chunks using shared logic
            let (output_index, total_entries_written) =
                match process_binary_chunks(chunks, &snapshot_out, lookup_map) {
                    Ok(result) => result,
                    Err(e) => {
                        eprintln!("Failed to process chunks for {}: {}", snapshot_name, e);
                        return;
                    }
                };

            println!(
                "  {} - Wrote {} pages with {} total entries",
                snapshot_name, output_index, total_entries_written
            );
        });

    Ok(())
}
