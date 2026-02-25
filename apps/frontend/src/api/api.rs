use gloo_net::http::Request;
use mp_stats_common::compression::uncompress_lzma;
use mp_stats_core::models::{
    GameLeaderboardData, IdMap, LeaderboardEntry, LeaderboardPage, PlatformEdition, PlatformMeta,
    PlayerProfile,
};
use mp_stats_core::routes;
use smol_str::SmolStr;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use web_sys::js_sys::Date;

fn now_ms() -> f64 {
    Date::now()
}

#[derive(Clone, Debug)]
struct CacheEntry {
    expires_at_ms: f64,
    /// Decompressed bytes.
    bytes: Arc<Vec<u8>>,
}

impl CacheEntry {
    fn is_fresh(&self) -> bool {
        now_ms() < self.expires_at_ms
    }
}

#[derive(Clone, Debug)]
pub struct Api {
    cache: Rc<RefCell<HashMap<String, CacheEntry>>>,
    last_sweep_ms: Arc<AtomicU64>,
}

impl PartialEq for Api {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

impl Default for Api {
    fn default() -> Self {
        Self {
            cache: Rc::new(RefCell::new(HashMap::new())),
            last_sweep_ms: Arc::new(AtomicU64::new(0)),
        }
    }
}

pub type ApiResult<T> = Result<T, gloo_net::Error>;

impl Api {
    const TTL_GAME_MS: f64 = 60.0 * 60.0 * 1000.0; // 1 Hour
    const TTL_ID_MAP_MS: f64 = 60.0 * 60.0 * 1000.0; // 1 Hour
    const TTL_PLAYER_SHARD_MS: f64 = 1.0 * 60.0 * 1000.0; // 1 Minute
    const TTL_LEADERBOARD_CHUNK_MS: f64 = 1.0 * 60.0 * 1000.0; // 1 Minute
    const TTL_NAME_INDEX_MS: f64 = 3.0 * 60.0 * 1000.0; // 3 Minutes

    const TTL_ERROR_MS: f64 = 10.0 * 1000.0; // 10 Seconds

    const SWEEP_INTERVAL_MS: u64 = 30_000; // every 30s at most

    fn maybe_sweep_expired(&self) {
        let now = now_ms() as u64;

        let last = self.last_sweep_ms.load(Ordering::Relaxed);
        if now.saturating_sub(last) < Self::SWEEP_INTERVAL_MS {
            return;
        }
        self.last_sweep_ms.store(now, Ordering::Relaxed);

        let mut cache = self.cache.borrow_mut();
        cache.retain(|_, entry| entry.is_fresh());
    }

    fn get_cached_bytes(&self, url: &str) -> Option<Arc<Vec<u8>>> {
        let cache = self.cache.borrow();
        cache.get(url).and_then(|entry| {
            if entry.is_fresh() {
                Some(entry.bytes.clone())
            } else {
                None
            }
        })
    }

    fn put_cache_bytes(&self, url: String, ttl_ms: f64, bytes: Arc<Vec<u8>>) {
        let mut cache = self.cache.borrow_mut();
        cache.insert(
            url,
            CacheEntry {
                expires_at_ms: now_ms() + ttl_ms,
                bytes,
            },
        );
    }

    async fn fetch_decompressed_bytes(&self, url: &str) -> ApiResult<Arc<Vec<u8>>> {
        let resp = Request::get(url).send().await?;
        if !resp.ok() {
            return Err(gloo_net::Error::GlooError(format!(
                "HTTP error fetching {}",
                url
            )));
        }

        let bytes = resp.binary().await.map_err(|e| {
            gloo_net::Error::GlooError(format!("Failed to read binary from {}: {}", url, e))
        })?;

        let cursor = std::io::Cursor::new(bytes);
        let decompressed = uncompress_lzma(cursor).map_err(|e| {
            gloo_net::Error::GlooError(format!("Failed to decompress {}: {:?}", url, e))
        })?;

        Ok(Arc::new(decompressed))
    }

    async fn get_decompressed_cached(&self, url: &str, ttl_ms: f64) -> ApiResult<Arc<Vec<u8>>> {
        // Try to cleanup cache
        self.maybe_sweep_expired();

        // Hot cache
        if let Some(entry) = self.get_cached_bytes(url)
        {
            return Ok(entry);
        }

        // Fetch
        match self.fetch_decompressed_bytes(url).await {
            Ok(bytes) => {
                self.put_cache_bytes(url.to_string(), ttl_ms, bytes.clone());
                Ok(bytes)
            }
            Err(e) => {
                // Short negative cache to reduce rapid retry storms.
                self.put_cache_bytes(url.to_string(), Self::TTL_ERROR_MS, Arc::new(Vec::new()));
                Err(e)
            }
        }
    }

    async fn fetch_bin_cached<T: serde::de::DeserializeOwned>(
        &self,
        url: &str,
        ttl_ms: f64,
    ) -> ApiResult<T> {
        let bytes = self.get_decompressed_cached(url, ttl_ms).await?;
        if bytes.is_empty() {
            return Err(gloo_net::Error::GlooError(format!(
                "Empty payload for {}",
                url
            )));
        }

        postcard::from_bytes(&bytes).map_err(|e| {
            gloo_net::Error::GlooError(format!(
                "Postcard deserialization failed for {}: {}",
                url, e
            ))
        })
    }

    async fn get_name_index(
        &self,
        edition: &PlatformEdition,
        prefix: &str,
    ) -> Option<HashMap<String, String>> {
        let url = format!("/data/{}", routes::names_index_bin(edition, prefix));
        self.fetch_bin_cached::<HashMap<String, String>>(&url, Self::TTL_NAME_INDEX_MS)
            .await
            .ok()
    }

    pub async fn fetch_game_leaderboards(
        &self,
        edition: &PlatformEdition,
        game_id: &str,
    ) -> ApiResult<GameLeaderboardData> {
        let url = format!("/data/{}", routes::game_bin(edition, game_id));
        self.fetch_bin_cached::<GameLeaderboardData>(&url, Self::TTL_GAME_MS)
            .await
            .map_err(|_| {
                gloo_net::Error::GlooError("Failed to fetch game leaderboards".to_string())
            })
    }

    pub async fn fetch_meta(&self, edition: &PlatformEdition) -> ApiResult<PlatformMeta> {
        let id_map = self.fetch_id_map(edition).await?;

        let mut games: Vec<mp_stats_core::models::Game> = id_map
            .games
            .into_values()
            .map(|value| mp_stats_core::models::Game {
                id: value.name.clone(),
                name: value.name.clone(),
                description: value.description.clone(),
                icon: None,
            })
            .collect();

        games.sort_by(|a, b| a.name.cmp(&b.name));

        Ok(PlatformMeta { games })
    }

    pub async fn fetch_id_map(&self, edition: &PlatformEdition) -> ApiResult<IdMap> {
        self.fetch_bin_cached::<IdMap>(
            &format!("/data/{}", routes::meta_map_bin(edition)),
            Self::TTL_ID_MAP_MS,
        )
        .await
        .map_err(|_| gloo_net::Error::GlooError("Failed to fetch id map".to_string()))
    }

    pub async fn fetch_leaderboard(
        &self,
        edition: &PlatformEdition,
        board: &str,
        game: &str,
        stat: &str,
        chunk: u32,
    ) -> ApiResult<Vec<LeaderboardEntry>> {
        let bin_path = format!(
            "/data/{}",
            routes::leaderboard_chunk_bin(edition, board, game, stat, chunk)
        );

        let page = self
            .fetch_bin_cached::<LeaderboardPage>(&bin_path, Self::TTL_LEADERBOARD_CHUNK_MS)
            .await
            .map_err(|_| gloo_net::Error::GlooError("Failed to fetch leaderboard".to_string()))?;

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
    ) -> ApiResult<PlayerProfile> {
        let is_valid_len = uuid.len() == 32 || uuid.len() == 36;
        let is_hex = uuid.chars().all(|c| c.is_ascii_hexdigit() || c == '-');
        let is_bedrock = *edition == PlatformEdition::Bedrock;

        // Bedrock UUIDs are not real UUIDs and more the names, since we do no know the bedrock ids or UUIDs
        if !is_bedrock && (!is_valid_len || !is_hex) {
            gloo_console::error!(format!("Invalid UUID format: {}", uuid));
            return Err(gloo_net::Error::GlooError("Invalid UUID format".into()));
        }

        let shard = &uuid[..3].to_uppercase();

        let bin_path = format!("/data/{}", routes::player_shard_bin(edition, shard));
        let mut shard_map = self
            .fetch_bin_cached::<HashMap<String, PlayerProfile>>(
                &bin_path,
                Self::TTL_PLAYER_SHARD_MS,
            )
            .await
            .map_err(|_| gloo_net::Error::GlooError("Failed to fetch player".to_string()))?;

        if let Some(mut profile) = shard_map.remove(uuid) {
            profile.uuid = uuid.into();
            Ok(profile)
        } else {
            gloo_console::warn!(format!(
                "Player {} not found in binary shard {}",
                uuid, shard
            ));
            Err(gloo_net::Error::GlooError(
                "Player not found in shard".into(),
            ))
        }
    }

    pub async fn search_players_by_name(
        &self,
        query: &str,
    ) -> ApiResult<Vec<(PlatformEdition, String, String)>> {
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
    ) -> ApiResult<Vec<LeaderboardEntry>> {
        let bin_path = format!(
            "/data/{}",
            routes::history_leaderboard_chunk_bin(edition, board, game, stat, snapshot_id, chunk)
        );

        let page = self
            .fetch_bin_cached::<LeaderboardPage>(&bin_path, Self::TTL_LEADERBOARD_CHUNK_MS)
            .await
            .map_err(|_| {
                gloo_net::Error::GlooError("Failed to fetch history leaderboard".to_string())
            })?;

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
