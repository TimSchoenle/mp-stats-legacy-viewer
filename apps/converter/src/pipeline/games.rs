use anyhow::Result;
use mp_stats_common::compression::write_lzma_bin;
use mp_stats_core::models::{GameLeaderboardData, IdMap, LeaderboardMeta, MetaFile};
use rayon::prelude::*;
use smol_str::SmolStr;
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::BufReader;
use std::path::Path;
use walkdir::WalkDir;

/// Process and aggregate game metadata from leaderboards
pub fn process_game_metadata(java_in: &Path, java_out: &Path, _id_map: &IdMap) -> Result<()> {
    let lb_in = java_in.join("leaderboards");
    let games_out = java_out.join("games");
    fs::create_dir_all(&games_out)?;

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
            let latest = stat_path.join("latest");
            let mut count = 0;
            if latest.exists() {
                if let Ok(file) = File::open(latest.join("_meta.json")) {
                    if let Ok(meta) = serde_json::from_reader::<_, MetaFile>(BufReader::new(file)) {
                        count = meta.total_entries;
                    }
                }
            }

            meta_stats
                .entry(SmolStr::new(stat))
                .or_default()
                .insert(SmolStr::new(board), LeaderboardMeta { count });
        }

        let game_data = GameLeaderboardData {
            game_id: SmolStr::new(game_id),
            game_name: SmolStr::new(game_id),
            stats: meta_stats,
        };

        let out_path = games_out.join(format!("{}.bin", game_id));
        let _ = write_lzma_bin(&out_path, &game_data);
    });

    Ok(())
}
