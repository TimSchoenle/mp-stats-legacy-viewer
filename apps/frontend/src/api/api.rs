use gloo_net::http::Request;
use mp_stats_common::compression::uncompress_lzma;
use mp_stats_core::models::{
    GameLeaderboardData, IdMap, JavaMeta, JavaPlayerProfile, LeaderboardEntry, LeaderboardPage,
    PlatformEdition,
};
use mp_stats_core::routes;
use smol_str::SmolStr;
use std::collections::HashMap;

use dashmap::DashMap;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct Api {
    // Cache for names_index: (Edition, Prefix) -> HashMap<Name, UUID>
    name_index_cache: Arc<DashMap<(PlatformEdition, String), HashMap<String, String>>>,
}

impl PartialEq for Api {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

impl Default for Api {
    fn default() -> Self {
        Self {
            name_index_cache: Arc::new(DashMap::new()),
        }
    }
}

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
                let decompressed = match uncompress_lzma(cursor) {
                    Ok(d) => d,
                    Err(e) => {
                        gloo_console::warn!(format!("Failed to decompress {}: {:?}", url, e));
                        return None;
                    }
                };

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

    async fn get_name_index(
        &self,
        edition: &PlatformEdition,
        prefix: &str,
    ) -> Option<HashMap<String, String>> {
        let key = (edition.clone(), prefix.to_string());

        // Check cache first
        if let Some(cached) = self.name_index_cache.get(&key) {
            return Some(cached.clone());
        }

        // Fetch if not in cache
        let url = format!("/data/{}", routes::names_index_bin(edition, prefix));
        if let Some(map) = self.fetch_bin::<HashMap<String, String>>(&url).await {
            self.name_index_cache.insert(key, map.clone());
            Some(map)
        } else {
            self.name_index_cache.insert(key, HashMap::new());
            None
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
        let id_map = self
            .fetch_bin::<IdMap>(&format!("/data/{}", routes::meta_map_bin(edition)))
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
        if let Some(map) = self
            .fetch_bin::<IdMap>(&format!("/data/{}", routes::meta_map_bin(edition)))
            .await
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

        if let Some(page) = self.fetch_bin::<LeaderboardPage>(&bin_path).await {
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
            if let Ok(profile) = self.fetch_player(edition, uuid).await
                && let Some(name) = profile.name
            {
                resolved.insert(uuid.clone(), name);
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
        let is_bedrock = *edition == PlatformEdition::Bedrock;

        // Bedrock UUIDs are not real UUIDs and more the names, since we do no know the bedrock ids or UUIDs
        if !is_bedrock && (!is_valid_len || !is_hex) {
            gloo_console::error!(format!("Invalid UUID format: {}", uuid));
            return Err(gloo_net::Error::GlooError("Invalid UUID format".into()));
        }

        let shard = &uuid[..3].to_uppercase();

        // java/players/SHARD.bin (LZMA Postcard)
        let bin_path = format!("/data/{}", routes::player_shard_bin(edition, shard));
        if let Some(mut shard_map) = self
            .fetch_bin::<HashMap<String, JavaPlayerProfile>>(&bin_path)
            .await
        {
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

    pub async fn search_players_by_name(
        &self,
        query: &str,
    ) -> Result<Vec<(PlatformEdition, String, String)>, gloo_net::Error> {
        let name_lower = query.to_lowercase();
        if name_lower.len() < 3 {
            return Ok(Vec::new());
        }

        let prefix = name_lower.chars().take(3).collect::<String>();

        // Concurrent fetch mapping from cache
        let java_future = self.get_name_index(&PlatformEdition::Java, &prefix);
        let bedrock_future = self.get_name_index(&PlatformEdition::Bedrock, &prefix);

        let (java_res, bedrock_res) = futures::future::join(java_future, bedrock_future).await;

        let mut results = Vec::new();

        if let Some(map) = java_res {
            for (name, uuid) in map {
                if name.to_lowercase().contains(&name_lower) {
                    results.push((PlatformEdition::Java, name, uuid));
                }
            }
        }

        if let Some(map) = bedrock_res {
            for (name, uuid) in map {
                if name.to_lowercase().contains(&name_lower) {
                    results.push((PlatformEdition::Bedrock, name, uuid));
                }
            }
        }

        // Sort results: exact match first, then starts with, then contains
        results.sort_by(|a, b| {
            let a_name = a.1.to_lowercase();
            let b_name = b.1.to_lowercase();

            let a_exact = a_name == name_lower;
            let b_exact = b_name == name_lower;

            if a_exact != b_exact {
                return b_exact.cmp(&a_exact);
            }

            let a_starts = a_name.starts_with(&name_lower);
            let b_starts = b_name.starts_with(&name_lower);

            if a_starts != b_starts {
                return b_starts.cmp(&a_starts);
            }

            a_name.cmp(&b_name)
        });

        // Limit results to top 10
        results.truncate(10);

        Ok(results)
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

        if let Some(page) = self.fetch_bin::<LeaderboardPage>(&bin_path).await {
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
