use anyhow::{Context, Result};
use mp_stats_common::compression::read_lzma_bin;
use mp_stats_core::models::*;
use std::collections::HashMap;
use std::path::PathBuf;

pub struct JavaLoader {
    data_root: PathBuf,
}

impl JavaLoader {
    pub fn new(data_root: PathBuf) -> Self {
        Self { data_root }
    }

    pub async fn fetch_meta(&self) -> Result<JavaMeta> {
        let id_map = self.fetch_id_map().await?;
        let mut games: Vec<Game> = id_map
            .games
            .values()
            .map(|name| Game {
                id: name.clone(),
                name: name.clone(),
            })
            .collect();
        games.sort_by(|a, b| a.name.cmp(&b.name));
        Ok(JavaMeta { games })
    }

    pub async fn fetch_id_map(&self) -> Result<IdMap> {
        let path = self.data_root.join("java/meta/map.bin");
        read_lzma_bin(&path)
            .with_context(|| format!("Failed to load ID map from {}", path.display()))
    }

    pub async fn fetch_leaderboard(
        &self,
        board: &str,
        game: &str,
        stat: &str,
        chunk: u32,
    ) -> Result<Vec<LeaderboardEntry>> {
        let path = self.data_root.join(format!(
            "java/leaderboards/{}/{}/{}/latest/chunk_{:04}.bin.xz",
            board, game, stat, chunk
        ));

        let page: JavaLeaderboardPage = read_lzma_bin(&path)
            .with_context(|| format!("Failed to load leaderboard page from {}", path.display()))?;

        // Convert columnar format (SoA) to row format (AoS)
        let entries = page
            .ranks
            .into_iter()
            .zip(page.uuids)
            .zip(page.names)
            .zip(page.scores)
            .map(|(((rank, uuid), name), score)| LeaderboardEntry {
                rank,
                uuid,
                name,
                score,
            })
            .collect();

        Ok(entries)
    }

    pub async fn fetch_player(&self, uuid: &str) -> Result<JavaPlayerProfile> {
        if uuid.len() < 3 {
            anyhow::bail!("UUID too short: {}", uuid);
        }

        let shard = &uuid[..3].to_uppercase();
        let path = self.data_root.join(format!("java/players/{}.bin", shard));

        let shard_map: HashMap<String, JavaPlayerProfile> = read_lzma_bin(&path)
            .with_context(|| format!("Failed to load player shard from {}", path.display()))?;

        shard_map
            .get(uuid)
            .cloned()
            .ok_or_else(|| anyhow::anyhow!("Player not found in shard: {}", uuid))
    }

    pub async fn fetch_game_leaderboards(&self, game_id: &str) -> Result<GameLeaderboardData> {
        let path = self.data_root.join(format!("java/games/{}.bin", game_id));
        read_lzma_bin(&path)
            .with_context(|| format!("Failed to load game leaderboards from {}", path.display()))
    }

    pub async fn fetch_history_snapshots(
        &self,
        board: &str,
        game: &str,
        stat: &str,
    ) -> Result<Vec<HistoricalSnapshot>> {
        use smol_str::SmolStr;

        let history_dir = self.data_root.join(format!(
            "java/leaderboards/{}/{}/{}/history",
            board, game, stat
        ));

        if !history_dir.exists() {
            return Ok(Vec::new());
        }

        let mut snapshots = Vec::new();

        for entry in std::fs::read_dir(&history_dir)? {
            let entry = entry?;
            let path = entry.path();

            if !path.is_dir() {
                continue;
            }

            let snapshot_id = path.file_name().unwrap().to_string_lossy().to_string();

            // Read metadata to get timestamp and entry count
            let meta_path = path.join("_meta.json");
            if let Ok(meta_str) = std::fs::read_to_string(&meta_path) {
                if let Ok(meta_json) = serde_json::from_str::<serde_json::Value>(&meta_str) {
                    let timestamp = meta_json["save_time"].as_u64().unwrap_or(0);
                    let total_pages = meta_json["total_pages"].as_u64().unwrap_or(0) as u32;
                    let total_entries = meta_json["total_entries"].as_u64().unwrap_or(0) as u32;

                    snapshots.push(mp_stats_core::models::HistoricalSnapshot {
                        snapshot_id: SmolStr::new(&snapshot_id),
                        timestamp,
                        total_pages,
                        total_entries,
                    });
                }
            }
        }

        // Sort by timestamp descending (newest first)
        snapshots.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));

        Ok(snapshots)
    }

    pub async fn fetch_history_leaderboard(
        &self,
        board: &str,
        game: &str,
        stat: &str,
        snapshot_id: &str,
        chunk: u32,
    ) -> Result<Vec<LeaderboardEntry>> {
        let path = self.data_root.join(format!(
            "java/leaderboards/{}/{}/{}/history/{}/chunk_{:04}.bin.xz",
            board, game, stat, snapshot_id, chunk
        ));

        let page: JavaLeaderboardPage = read_lzma_bin(&path).with_context(|| {
            format!(
                "Failed to load history leaderboard page from {}",
                path.display()
            )
        })?;

        // Convert columnar format (SoA) to row format (AoS)
        let entries = page
            .ranks
            .into_iter()
            .zip(page.uuids)
            .zip(page.names)
            .zip(page.scores)
            .map(|(((rank, uuid), name), score)| LeaderboardEntry {
                rank,
                uuid,
                name,
                score,
            })
            .collect();

        Ok(entries)
    }
}
