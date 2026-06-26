use anyhow::Result;
use mp_stats_common::compression::{decompress_file_auto, write_lzma_bin};
use mp_stats_core::models::{
    IdMap, PlatformEdition, PlayerProfile, StatRaw, competition_ranks_by_score,
};
use mp_stats_core::routes;
use rayon::prelude::*;
use smol_str::SmolStr;
use std::collections::{HashMap, HashSet};
use std::path::Path;
use walkdir::WalkDir;

/// Process the player snapshot files into profile shards.
///
/// Returns the set of UUIDs that actually received a profile shard entry. This
/// set is later used to stamp a `has_profile` flag onto the names index so the
/// frontend can hide search suggestions for players without any profile.
pub fn process_java_players(
    platform: &PlatformEdition,
    java_in: &Path,
    output_directory: &Path,
    id_map: &IdMap,
    player_lookup_map: &HashMap<String, (String, String)>,
) -> Result<HashSet<String>> {
    let players_in = java_in.join("players");

    if !players_in.exists() {
        return Ok(HashSet::new());
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

    // Find all board id
    let all_board_id = id_map
        .boards
        .iter()
        .find(|(_, board)| board.name.to_lowercase() == "all")
        .map(|(id, _)| *id);
    println!("Found all board id: {:?}", all_board_id);

    println!("Found {} player shards to process.", files.len());

    // Sharded storage: Prefix (e.g. "EF4") -> Map<UUID, Profile>
    let mut shards: HashMap<String, HashMap<String, PlayerProfile>> = files
        .par_iter()
        .map(|path| {
            process_player_shard(path, all_board_id, player_lookup_map).unwrap_or_else(|e| {
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

    // Recompute tie-aware positions across the whole player population so a
    // player's rank on their profile matches the leaderboard: players sharing a
    // score must share a position. The source stride only carries a sequential
    // rank, so we override it with standard competition ranking ("1224").
    assign_competition_ranks(&mut shards);

    println!("Writing {} player shards...", shards.len());

    // Collect the set of UUIDs that received a profile.
    let profiled_uuids: HashSet<String> = shards
        .values()
        .flat_map(|profile_map| profile_map.keys().cloned())
        .collect();

    // Write Shards
    shards.par_iter().for_each(|(prefix, profile_map)| {
        let relative_path = routes::player_shard_bin(platform, prefix);
        let out_path = output_directory.join(relative_path);

        let _ = write_lzma_bin(&out_path, profile_map);
    });

    Ok(profiled_uuids)
}

/// Recompute every profile's per-stat rank using standard competition ranking
/// ("1224") so players who share a score share a position.
///
/// Ranks are computed independently for each `(board_id, game_id, stat_id)`
/// group across the entire player population: a stat's rank is `1 + (number of
/// entries with a strictly greater score)`. This mirrors the leaderboard
/// pipeline exactly, keeping a player's position identical between the
/// leaderboard and their profile.
fn assign_competition_ranks(shards: &mut HashMap<String, HashMap<String, PlayerProfile>>) {
    // Pass 1: tally how many entries achieved each score per stat group.
    let mut counts: HashMap<(u32, u32, u32), HashMap<u64, u64>> = HashMap::new();
    for profile_map in shards.values() {
        for profile in profile_map.values() {
            for stat in &profile.stats {
                *counts
                    .entry((stat.board_id, stat.game_id, stat.stat_id))
                    .or_default()
                    .entry(stat.score)
                    .or_insert(0) += 1;
            }
        }
    }

    // Build a score -> rank lookup for each stat group.
    let rank_tables: HashMap<(u32, u32, u32), HashMap<u64, u32>> = counts
        .into_iter()
        .map(|(key, score_counts)| (key, competition_ranks_by_score(&score_counts)))
        .collect();

    // Pass 2: stamp the tie-aware rank back onto every stat entry.
    for profile_map in shards.values_mut() {
        for profile in profile_map.values_mut() {
            for stat in &mut profile.stats {
                if let Some(rank) = rank_tables
                    .get(&(stat.board_id, stat.game_id, stat.stat_id))
                    .and_then(|table| table.get(&stat.score))
                {
                    stat.rank = *rank;
                }
            }
        }
    }
}

/// Process a single player shard file
fn process_player_shard(
    path: &Path,
    all_board_id: Option<u32>,
    player_lookup_map: &HashMap<String, (String, String)>,
) -> Result<HashMap<String, HashMap<String, PlayerProfile>>> {
    // Read & Decompress
    let decompressed = decompress_file_auto(path)?;

    // Parse JSON: {"15432": [stride...]}
    let raw_map: HashMap<String, Vec<serde_json::Value>> = serde_json::from_slice(&decompressed)?;

    let mut shards: HashMap<String, HashMap<String, PlayerProfile>> = HashMap::new();

    for (player_id_str, stride_data) in raw_map {
        // Resolve Identity
        let (uuid, name) = if let Some(info) = player_lookup_map.get(&player_id_str) {
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

            // Only show all stats on player profile
            if let Some(all_board_id) = all_board_id
                && board_id != all_board_id
            {
                continue;
            }

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

#[cfg(test)]
mod tests {
    use super::*;

    fn stat(game_id: u32, stat_id: u32, score: u64, rank: u32) -> StatRaw {
        StatRaw {
            board_id: 0,
            game_id,
            stat_id,
            score,
            rank,
            save_time: 0,
        }
    }

    fn profile(uuid: &str, stats: Vec<StatRaw>) -> PlayerProfile {
        PlayerProfile {
            uuid: SmolStr::new(uuid),
            name: Some(SmolStr::new(uuid)),
            stats,
        }
    }

    fn rank_of(
        shards: &HashMap<String, HashMap<String, PlayerProfile>>,
        uuid: &str,
        game_id: u32,
        stat_id: u32,
    ) -> u32 {
        shards
            .values()
            .flat_map(|m| m.values())
            .find(|p| p.uuid == uuid)
            .and_then(|p| {
                p.stats
                    .iter()
                    .find(|s| s.game_id == game_id && s.stat_id == stat_id)
            })
            .map(|s| s.rank)
            .expect("stat present")
    }

    #[test]
    fn tied_scores_get_the_same_position() {
        // Three players: A and B tie at 100, C trails at 50. The original
        // stride ranks are sequential (1, 2, 3) and must be corrected so the
        // tie shares position #1 and C jumps to #3.
        let mut shards: HashMap<String, HashMap<String, PlayerProfile>> = HashMap::new();
        shards.insert(
            "AAA".to_string(),
            HashMap::from([
                (
                    "aaa-1".to_string(),
                    profile("aaa-1", vec![stat(1, 2, 100, 1)]),
                ),
                (
                    "aaa-2".to_string(),
                    profile("aaa-2", vec![stat(1, 2, 100, 2)]),
                ),
                (
                    "aaa-3".to_string(),
                    profile("aaa-3", vec![stat(1, 2, 50, 3)]),
                ),
            ]),
        );

        assign_competition_ranks(&mut shards);

        assert_eq!(rank_of(&shards, "aaa-1", 1, 2), 1);
        assert_eq!(rank_of(&shards, "aaa-2", 1, 2), 1);
        assert_eq!(rank_of(&shards, "aaa-3", 1, 2), 3);
    }

    #[test]
    fn ranks_are_scoped_per_stat_group() {
        // The same player ranks differently across two distinct stats, and a
        // tie in one stat must not affect the other.
        let mut shards: HashMap<String, HashMap<String, PlayerProfile>> = HashMap::new();
        shards.insert(
            "AAA".to_string(),
            HashMap::from([
                (
                    "p1".to_string(),
                    profile("p1", vec![stat(1, 1, 10, 0), stat(2, 1, 5, 0)]),
                ),
                (
                    "p2".to_string(),
                    profile("p2", vec![stat(1, 1, 10, 0), stat(2, 1, 9, 0)]),
                ),
            ]),
        );

        assign_competition_ranks(&mut shards);

        // Stat (game 1, stat 1): both tie at 10 -> both #1.
        assert_eq!(rank_of(&shards, "p1", 1, 1), 1);
        assert_eq!(rank_of(&shards, "p2", 1, 1), 1);
        // Stat (game 2, stat 1): 9 beats 5 -> p2 #1, p1 #2.
        assert_eq!(rank_of(&shards, "p2", 2, 1), 1);
        assert_eq!(rank_of(&shards, "p1", 2, 1), 2);
    }
}
