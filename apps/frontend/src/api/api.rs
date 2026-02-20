use gloo_net::http::Request;
use mp_stats_common::compression::uncompress_lzma;
use mp_stats_core::models::{
    GameLeaderboardData, IdMap, JavaLeaderboardPage, JavaMeta, JavaPlayerProfile, LeaderboardEntry,
    NameLookup, PlatformEdition,
};
use mp_stats_core::routes;
use smol_str::SmolStr;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub struct Api;

impl Api {
    // Helper to fetch and decode binary data (LZMA or Zlib -> Postcard)
    async fn fetch_bin<T: serde::de::DeserializeOwned>(&self, url: &str) -> Option<T> {
        match Request::get(url).send().await {
            Ok(resp) if resp.ok() => {
                let bytes = match resp.binary().await {
                    Ok(b) => b,
                    Err(e) => {
                        gloo_console::warn!(format!("Failed to get binary from {}: {}", url, e));
                        return None;
                    }
                };

                let cursor = std::io::Cursor::new(bytes);
                let decompressed = uncompress_lzma(cursor).unwrap();

                match postcard::from_bytes(&decompressed) {
                    Ok(data) => Some(data),
                    Err(e) => {
                        gloo_console::warn!(format!(
                        "Postcard deserialization failed for {}: {}",
                        url, e
                    ));
                        None
                    }
                }
            }
            _ => None,
        }
    }

    // Helper to fetch Raw LZMA data (Vec<u8>)
    async fn fetch_raw_lzma(&self, url: &str) -> Option<Vec<u8>> {
        match Request::get(url).send().await {
            Ok(resp) if resp.ok() => {
                let bytes = resp.binary().await.ok()?;
                let cursor = std::io::Cursor::new(bytes);

                uncompress_lzma(cursor).ok()
            }
            _ => None,
        }
    }

    pub async fn fetch_game_leaderboards(
        &self,
        edition: &PlatformEdition,
        game_id: &str,
    ) -> Result<GameLeaderboardData, gloo_net::Error> {
        let url = format!("/data/{}", routes::game_bin(edition, game_id));
        if let Some(data) = self.fetch_bin::<GameLeaderboardData>(&url).await {
            return Ok(data);
        }
        Err(gloo_net::Error::GlooError(
            "Failed to fetch game leaderboards".to_string(),
        ))
    }

    pub async fn fetch_meta(&self, edition: &PlatformEdition) -> Result<JavaMeta, gloo_net::Error> {
        let id_map = self.fetch_bin::<IdMap>(&format!("/data/{}", routes::meta_map_bin(edition)))
            .await
            .ok_or(gloo_net::Error::GlooError(
                "Failed to fetch id map".to_string(),
            ))?;

        let mut games: Vec<mp_stats_core::models::Game> = id_map
            .games
            .values()
            .map(|name| mp_stats_core::models::Game {
                id: name.clone(),
                name: name.clone(),
            })
            .collect();

        games.sort_by(|a, b| a.name.cmp(&b.name));

        Ok(JavaMeta { games })
    }

    pub async fn fetch_id_map(&self, edition: &PlatformEdition) -> Result<IdMap, gloo_net::Error> {
        if let Some(map) = self.fetch_bin::<IdMap>(&format!("/data/{}", routes::meta_map_bin(edition))).await
        {
            return Ok(map);
        }

        Err(gloo_net::Error::GlooError(
            "Failed to fetch id map".to_string(),
        ))
    }

    pub async fn fetch_leaderboard(
        &self,
        edition: &PlatformEdition,
        board: &str,
        game: &str,
        stat: &str,
        chunk: u32,
    ) -> Result<Vec<LeaderboardEntry>, gloo_net::Error> {
        // Fetch .bin.xz (Postcard JavaLeaderboardPage)
        let bin_path = format!(
            "/data/{}",
            routes::leaderboard_chunk_bin(edition, board, game, stat, chunk)
        );

        if let Some(page) = self.fetch_bin::<JavaLeaderboardPage>(&bin_path).await {
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

            return Ok(entries);
        }

        Err(gloo_net::Error::GlooError(
            "Failed to fetch leaderboard".to_string(),
        ))
    }

    pub async fn resolve_names(
        &self,
        edition: &PlatformEdition,
        uuids: &[SmolStr],
    ) -> HashMap<SmolStr, SmolStr> {
        let mut resolved = HashMap::new();
        for uuid in uuids {
            if let Ok(profile) = self.fetch_player(&edition, uuid).await {
                if let Some(name) = profile.name {
                    resolved.insert(uuid.clone(), name);
                }
            }
        }
        resolved
    }

    pub async fn fetch_player(
        &self,
        edition: &PlatformEdition,
        uuid: &str,
    ) -> Result<JavaPlayerProfile, gloo_net::Error> {
        let is_valid_len = uuid.len() == 32 || uuid.len() == 36;
        let is_hex = uuid.chars().all(|c| c.is_ascii_hexdigit() || c == '-');

        if !is_valid_len || !is_hex {
            gloo_console::error!(format!("Invalid UUID format: {}", uuid));
            return Err(gloo_net::Error::GlooError("Invalid UUID format".into()));
        }

        let shard = &uuid[..3].to_uppercase();

        // java/players/SHARD.bin (LZMA Postcard)
        let bin_path = format!("/data/{}", routes::player_shard_bin(edition, shard));
        if let Some(mut shard_map) = self.fetch_bin::<HashMap<String, JavaPlayerProfile>>(&bin_path).await {
            if let Some(mut profile) = shard_map.remove(uuid) {
                profile.uuid = uuid.into();
                return Ok(profile);
            } else {
                gloo_console::warn!(format!(
                "Player {} not found in binary shard {}",
                uuid, shard
            ));
                return Err(gloo_net::Error::GlooError(
                    "Player not found in shard".into(),
                ));
            }
        }

        Err(gloo_net::Error::GlooError(
            "Failed to fetch player".to_string(),
        ))
    }

    pub async fn find_player_uuid(
        &self,
        edition: &PlatformEdition,
        name: &str,
    ) -> Result<Option<NameLookup>, gloo_net::Error> {
        if name.len() < 3 {
            return Ok(None);
        }
        let prefix = &name[..3].to_lowercase();

        // Fetch names_index/{prefix}.bin
        let url = format!(
            "/data/{}/names_index/{}.bin",
            edition.directory_name(),
            prefix
        );

        if let Some(map) = self.fetch_bin::<HashMap<String, String>>(&url).await {
            if let Some(uuid) = map.get(name) {
                return Ok(Some(NameLookup {
                    uuid: SmolStr::new(uuid),
                    shard_path: SmolStr::new(&uuid[..3].to_uppercase()),
                }));
            }
        }

        Err(gloo_net::Error::GlooError(
            "Failed to fetch player".to_string(),
        ))
    }

    pub async fn fetch_history_snapshots(
        &self,
        edition: &PlatformEdition,
        board: &str,
        game: &str,
        stat: &str,
    ) -> Result<Vec<mp_stats_core::HistoricalSnapshot>, gloo_net::Error> {
        let url = format!(
            "/data/{}",
            routes::history_snapshots_meta(edition, board, game, stat)
        );

        match Request::get(&url).send().await {
            Ok(resp) if resp.ok() => {
                let metadata = resp
                    .json::<mp_stats_core::models::HistoryMetadata>()
                    .await?;
                Ok(metadata.snapshots)
            }
            Ok(resp) if resp.status() == 404 => {
                // No history available for this leaderboard
                Ok(Vec::new())
            }
            _ => Ok(Vec::new()),
        }
    }

    pub async fn fetch_history_leaderboard(
        &self,
        edition: &PlatformEdition,
        board: &str,
        game: &str,
        stat: &str,
        snapshot_id: &str,
        chunk: u32,
    ) -> Result<Vec<LeaderboardEntry>, gloo_net::Error> {
        let bin_path = format!(
            "/data/{}",
            routes::history_leaderboard_chunk_bin(edition, board, game, stat, snapshot_id, chunk)
        );

        if let Some(page) = self.fetch_bin::<JavaLeaderboardPage>(&bin_path).await {
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

            return Ok(entries);
        }

        Err(gloo_net::Error::GlooError(
            "Failed to fetch history leaderboard".to_string(),
        ))
    }
}
