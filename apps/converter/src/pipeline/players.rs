use anyhow::Result;
use mp_stats_common::compression::{decompress_file_auto, write_lzma_bin};
use mp_stats_core::models::{PlatformEdition, PlayerProfile, StatRaw};
use mp_stats_core::routes;
use rayon::prelude::*;
use smol_str::SmolStr;
use std::collections::HashMap;
use std::path::Path;
use walkdir::WalkDir;

pub fn process_java_players(
    platform: &PlatformEdition,
    java_in: &Path,
    output_directory: &Path,
    lookup_map: &HashMap<String, (String, String)>,
) -> Result<()> {
    let players_in = java_in.join("players");

    if !players_in.exists() {
        return Ok(());
    }

    let walker = WalkDir::new(&players_in).into_iter();

    // Collect all .json.xz files
    let mut files = Vec::new();
    for entry in walker.filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            let path = entry.path();
            if let Some(name) = path.file_name() {
                let name = name.to_string_lossy();
                if name.ends_with(".json.xz") {
                    files.push(path.to_path_buf());
                }
            }
        }
    }

    println!("Found {} player shards to process.", files.len());

    // Sharded storage: Prefix (e.g. "EF4") -> Map<UUID, Profile>
    let shards: HashMap<String, HashMap<String, PlayerProfile>> = files
        .par_iter()
        .map(|path| {
            process_player_shard(path, lookup_map).unwrap_or_else(|e| {
                eprintln!("Failed to process player shard {:?}: {}", path, e);
                HashMap::new()
            })
        })
        .reduce(HashMap::new, |mut acc, file_shards| {
            for (prefix, mut uuid_map) in file_shards {
                acc.entry(prefix).or_default().extend(uuid_map.drain());
            }
            acc
        });

    println!("Writing {} player shards...", shards.len());

    // Write Shards
    shards.par_iter().for_each(|(prefix, profile_map)| {
        let relative_path = routes::player_shard_bin(platform, prefix);
        let out_path = output_directory.join(relative_path);

        let _ = write_lzma_bin(&out_path, profile_map);
    });

    Ok(())
}

/// Process a single player shard file
fn process_player_shard(
    path: &Path,
    lookup_map: &HashMap<String, (String, String)>,
) -> Result<HashMap<String, HashMap<String, PlayerProfile>>> {
    // Read & Decompress
    let decompressed = decompress_file_auto(path)?;

    // Parse JSON: {"15432": [stride...]}
    let raw_map: HashMap<String, Vec<serde_json::Value>> = serde_json::from_slice(&decompressed)?;

    let mut shards: HashMap<String, HashMap<String, PlayerProfile>> = HashMap::new();

    for (player_id_str, stride_data) in raw_map {
        // Resolve Identity
        let (uuid, name) = if let Some(info) = lookup_map.get(&player_id_str) {
            (SmolStr::new(&info.0), Some(SmolStr::new(&info.1)))
        } else {
            (SmolStr::new("unknown"), None)
        };

        if uuid == "unknown" {
            continue;
        }

        // Parse Stride Data
        let count = stride_data.len() / 7;
        let mut stats = Vec::with_capacity(count);

        for i in 0..count {
            let offset = i * 7;
            if offset + 7 > stride_data.len() {
                break;
            }

            // Safe extraction from Value
            let board_id = stride_data[offset].as_u64().unwrap_or(0) as u32;
            let game_id = stride_data[offset + 1].as_u64().unwrap_or(0) as u32;
            let stat_id = stride_data[offset + 2].as_u64().unwrap_or(0) as u32;
            let score = stride_data[offset + 4].as_u64().unwrap_or(0);
            let rank = stride_data[offset + 5].as_u64().unwrap_or(0) as u32;
            let save_time = stride_data[offset + 6].as_u64().unwrap_or(0);

            stats.push(StatRaw {
                board_id,
                game_id,
                stat_id,
                score,
                rank,
                save_time,
            });
        }

        let profile = PlayerProfile {
            uuid: uuid.clone(),
            name,
            stats,
        };

        // Determine target shard from UUID
        if uuid.len() >= 3 {
            let prefix = uuid[..3].to_uppercase();
            shards
                .entry(prefix)
                .or_default()
                .insert(uuid.to_string(), profile);
        }
    }

    Ok(shards)
}
