use anyhow::Result;
use mp_stats_common::compression::{decompress_file_auto, read_lzma_raw, write_lzma_bin};
use mp_stats_common::formats::raw::ENTRIES_PER_PAGE;
use mp_stats_common::formats::FILE_META;
use mp_stats_core::models::{IdMap, JavaLeaderboardPage};
use rayon::prelude::*;
use smol_str::SmolStr;
use std::collections::HashMap;
use std::fs::{self};
use std::io::Read;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

use crate::models::leaderboard::binary_leaderboard;

const LEADERBOARD_SIZE: usize = crate::models::leaderboard::BINARY_LEADERBOARD_SIZE;

/// Process all Java leaderboards
pub fn process_java_leaderboards(
    java_in: &Path,
    java_out: &Path,
    _id_map: &IdMap,
    lookup_map: &HashMap<String, (String, String)>,
) -> Result<()> {
    let lb_in = java_in.join("leaderboards");
    let lb_out = java_out.join("leaderboards");

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
        let _ = process_single_leaderboard(latest_dir, &lb_in, &lb_out, lookup_map);
    });

    Ok(())
}

/// Process a single leaderboard directory
fn process_single_leaderboard(
    latest_in: &Path,
    _root_in: &Path,
    root_out: &Path,
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
    let out_stat_dir = root_out.join(board_name).join(game_name).join(stat_name);
    let out_latest = out_stat_dir.join("latest");
    fs::create_dir_all(&out_latest)?;

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

    // Update metadata with correct page count
    update_metadata(
        &out_latest.join(FILE_META),
        total_entries_written,
        output_index,
    )?;

    Ok(())
}

/// Shared logic to process binary chunks and convert to rich format
fn process_binary_chunks(
    chunks: &[Vec<u8>],
    output_dir: &Path,
    lookup_map: &HashMap<String, (String, String)>,
) -> Result<(u32, u32)> {
    let mut output_index = 0;
    let mut current_page = JavaLeaderboardPage {
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
                eprintln!("Invalid chunk data: offset {} + size {} > chunk size {}", offset, LEADERBOARD_SIZE, chunk_data.len());
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
                    current_page = JavaLeaderboardPage {
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

    println!("Processed {} chunks with {} total entries", chunks.len(), total_entries_written);

    Ok((output_index, total_entries_written))
}

/// Update metadata file with new page count and entry count
fn update_metadata(meta_path: &Path, total_entries: u32, total_pages: u32) -> Result<()> {
    if meta_path.exists() {
        if let Ok(meta_str) = fs::read_to_string(meta_path) {
            if let Ok(mut meta_json) = serde_json::from_str::<serde_json::Value>(&meta_str) {
                meta_json["total_entries"] = serde_json::json!(total_entries);
                meta_json["total_pages"] = serde_json::json!(total_pages);

                if let Ok(updated_meta) = serde_json::to_string(&meta_json) {
                    let _ = fs::write(meta_path, updated_meta);
                }
            }
        }
    }
    Ok(())
}

/// Extract timestamp from metadata file
fn extract_timestamp(meta_path: &Path) -> u64 {
    if meta_path.exists() {
        if let Ok(meta_str) = fs::read_to_string(meta_path) {
            if let Ok(meta_json) = serde_json::from_str::<serde_json::Value>(&meta_str) {
                return meta_json["save_time_unix"].as_u64().unwrap_or(0);
            }
        }
    }
    0
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

    let mut snapshot_data: HashMap<String, (Vec<u8>, Vec<Vec<u8>>)> = HashMap::new();

    // Extract all files and group by snapshot
    for entry_result in archive.entries()? {
        let mut entry = entry_result?;
        let path = entry.path()?;
        let path_str = path.to_string_lossy().to_string();

        if let Some(slash_pos) = path_str.find('/') {
            let snapshot_name = path_str[..slash_pos].to_string();
            let file_name = path_str[slash_pos + 1..].to_string();

            if file_name == "_meta.json" {
                // Read metadata
                let mut contents = Vec::new();
                entry.read_to_end(&mut contents)?;
                snapshot_data
                    .entry(snapshot_name)
                    .or_insert_with(|| (Vec::new(), Vec::new()))
                    .0 = contents;
            } else if file_name.starts_with("chunk_") && file_name.ends_with(".bin") {
                // Read chunk data (uncompressed in the tar)
                let mut contents = Vec::new();
                entry.read_to_end(&mut contents)?;
                snapshot_data
                    .entry(snapshot_name)
                    .or_insert_with(|| (Vec::new(), Vec::new()))
                    .1
                    .push(contents);
            }
        }
    }

    println!(
        "Processing {} history snapshots in parallel...",
        snapshot_data.len()
    );

    // Process each snapshot concurrently using rayon
    use std::sync::Mutex;
    let snapshots = Mutex::new(Vec::new());

    snapshot_data
        .par_iter()
        .for_each(|(snapshot_name, (meta_contents, chunks))| {
            let snapshot_out = history_out.join(snapshot_name);
            if let Err(e) = fs::create_dir_all(&snapshot_out) {
                eprintln!("Failed to create directory {:?}: {}", snapshot_out, e);
                return;
            }

            println!("Processing history snapshot: {}", snapshot_name);

            // Write metadata
            if !meta_contents.is_empty() {
                if let Err(e) = fs::write(snapshot_out.join("_meta.json"), meta_contents) {
                    eprintln!("Failed to write metadata for {}: {}", snapshot_name, e);
                    return;
                }
            }

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

            // Extract timestamp from _meta.json if available
            let meta_path = snapshot_out.join(FILE_META);
            let timestamp = extract_timestamp(&meta_path);

            // Update metadata with correct page count
            if let Err(e) = update_metadata(&meta_path, total_entries_written, output_index) {
                eprintln!("Failed to update metadata for {}: {}", snapshot_name, e);
            }

            // Add snapshot metadata
            let snapshot_info = serde_json::json!({
                "snapshot_id": snapshot_name,
                "timestamp": timestamp,
                "total_pages": output_index,
                "total_entries": total_entries_written,
            });

            if let Ok(mut snapshots_vec) = snapshots.lock() {
                snapshots_vec.push(snapshot_info);
            }
        });

    let snapshots = snapshots.into_inner()?;

    // Generate _snapshots.json metadata file
    if !snapshots.is_empty() {
        let snapshots_metadata = serde_json::json!({
            "snapshots": snapshots
        });

        let snapshots_path = history_out.join("_snapshots.json");
        if let Ok(json_str) = serde_json::to_string_pretty(&snapshots_metadata) {
            if let Err(e) = fs::write(&snapshots_path, json_str) {
                eprintln!("Failed to write _snapshots.json: {}", e);
            } else {
                println!(
                    "Generated _snapshots.json with {} snapshots",
                    snapshots.len()
                );
            }
        }
    }

    Ok(())
}
