//! Fetching, decompressing and decoding one file of the converted tree, with a cache in front.
//!
//! Every method here is a path from [`mp_stats_core::routes`], a fetch, an LZMA pass and a
//! Postcard decode. The cache holds decompressed bytes rather than decoded values, because the
//! same names index is decoded into two different shapes and the expensive half is the
//! decompression either way.

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
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use web_sys::js_sys::Date;

fn now_ms() -> f64 {
    Date::now()
}

/// One cached response: the decompressed bytes, and when they stop counting as fresh.
///
/// An empty `bytes` is the negative entry a failed fetch leaves behind.
#[derive(Clone, Debug)]
struct CacheEntry {
    expires_at_ms: f64,
    bytes: Arc<Vec<u8>>,
}

impl CacheEntry {
    fn is_fresh(&self) -> bool {
        now_ms() < self.expires_at_ms
    }
}

/// The client's whole data layer: a cache keyed by URL, and the fetches that fill it.
///
/// Cloning shares the cache rather than copying it, which is what makes it safe to hand to every
/// component through a Dioxus context. One is provided above the router and lives as long as the
/// tab does, so nothing here survives a reload - and nothing here is rebuilt by a navigation.
///
/// It is deliberately not [`PartialEq`]. Under Dioxus a fetch re-runs when the values named in its
/// [`use_reactive!`](macro@dioxus::prelude::use_reactive) dependencies change, and this is never
/// one of them: the cache is shared, its contents are not a component's business, and an `Api`
/// that compared unequal to itself would restart every resource on every render.
#[derive(Clone, Debug)]
pub struct Api {
    cache: Rc<RefCell<HashMap<String, CacheEntry>>>,
    last_sweep_ms: Arc<AtomicU64>,
}

impl Default for Api {
    fn default() -> Self {
        Self {
            cache: Rc::new(RefCell::new(HashMap::new())),
            last_sweep_ms: Arc::new(AtomicU64::new(0)),
        }
    }
}

/// [`Result`] with the fetch error already filled in.
///
/// A missing file and a malformed one both arrive as
/// [`GlooError`](gloo_net::Error::GlooError) carrying a message, so a caller that needs to tell
/// them apart has to read it.
pub type ApiResult<T> = Result<T, gloo_net::Error>;

impl Api {
    // The lifetimes are about memory, not staleness: the tree is rewritten by a batch job between
    // deployments and never while a tab is open, so every one of these could be infinite and still
    // be correct. What they bound is how long a page the reader has navigated away from goes on
    // occupying the tab. The order reflects size and reuse — a game's metadata is small and
    // revisited on every page of it, a leaderboard page is large and read once.
    const TTL_GAME_MS: f64 = 60.0 * 60.0 * 1000.0;
    const TTL_ID_MAP_MS: f64 = 60.0 * 60.0 * 1000.0;
    const TTL_PLAYER_SHARD_MS: f64 = 60.0 * 1000.0;
    const TTL_LEADERBOARD_CHUNK_MS: f64 = 60.0 * 1000.0;
    const TTL_NAME_INDEX_MS: f64 = 3.0 * 60.0 * 1000.0;

    // A failed fetch is remembered too, for long enough to stop a component that re-renders on
    // every keystroke from turning one missing file into a request per frame.
    const TTL_ERROR_MS: f64 = 10.0 * 1000.0;

    // Expired entries are dropped on the next fetch after this interval rather than on a timer.
    // Nothing here runs while the tab is idle, and a sweep the reader is not waiting on is a sweep
    // that costs nothing to defer.
    const SWEEP_INTERVAL_MS: u64 = 30_000;

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
        if let Some(entry) = self.get_cached_bytes(url) {
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

    /// The names index for one three-character prefix, or `None` if it cannot be fetched or
    /// decoded.
    ///
    /// A prefix with no index is an ordinary answer: it means no player's name starts that way.
    async fn get_name_index(
        &self,
        edition: &PlatformEdition,
        prefix: &str,
    ) -> Option<HashMap<String, (String, bool)>> {
        let url = format!("/data/{}", routes::names_index_bin(edition, prefix));

        let bytes = self
            .get_decompressed_cached(&url, Self::TTL_NAME_INDEX_MS)
            .await
            .ok()?;

        decode_name_index(&bytes)
    }

    /// One game's categories, boards, snapshots and leaders, which is one file.
    ///
    /// # Errors
    ///
    /// If the game has no metadata file, or it does not decode.
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

    /// The edition's game list, sorted by name.
    ///
    /// Built out of [`Self::fetch_id_map`] rather than fetched: no file in the tree holds a game
    /// list, because the ID map already names every game.
    ///
    /// # Errors
    ///
    /// If the ID map cannot be fetched or decoded.
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
                total_snapshots: value.total_snapshots,
            })
            .collect();

        games.sort_by(|a, b| a.name.cmp(&b.name));

        Ok(PlatformMeta { games })
    }

    /// The edition's board, game and stat name tables, which every id in a profile resolves
    /// through.
    ///
    /// # Errors
    ///
    /// If the map is missing or does not decode.
    pub async fn fetch_id_map(&self, edition: &PlatformEdition) -> ApiResult<IdMap> {
        self.fetch_bin_cached::<IdMap>(
            &format!("/data/{}", routes::meta_map_bin(edition)),
            Self::TTL_ID_MAP_MS,
        )
        .await
        .map_err(|_| gloo_net::Error::GlooError("Failed to fetch id map".to_string()))
    }

    /// One page of a board's current standings, in rank order.
    ///
    /// `chunk` is zero-based, so the site's page 1 is chunk 0. A page past the end of the board
    /// is a missing file and comes back as an error rather than an empty vector.
    ///
    /// # Errors
    ///
    /// If the page does not exist or does not decode.
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

    /// Looks up a display name for each UUID, skipping the ones with no profile.
    ///
    /// One profile shard fetch per UUID, in sequence. Given for a handful of leaders; a list is
    /// better served by the names already on a leaderboard page.
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

    /// One player's profile, out of the shard their UUID falls in.
    ///
    /// The UUID is checked before anything is fetched, against 32 or 36 characters of hex and
    /// dashes. Bedrock is exempt: its dumps carry names where a UUID belongs, so there is no shape
    /// to check against.
    ///
    /// # Errors
    ///
    /// If the UUID fails that check, if the shard is missing or does not decode, or if the shard
    /// exists and holds no entry for this player — which is what a player who never placed looks
    /// like, rather than a fault.
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

    /// Up to ten players whose name contains `query`, across both editions, best match first.
    ///
    /// A query shorter than three characters returns nothing: three is the length of a names index
    /// shard, so there is no file to look in. Both editions are fetched at once, and one that has
    /// no index for the prefix contributes nothing rather than failing the search. Players with no
    /// profile are left out, so no suggestion leads to an empty page.
    ///
    /// Ordering is exact match, then names starting with the query, then the rest, alphabetically
    /// within each group.
    ///
    /// # Errors
    ///
    /// Never. A prefix neither edition has an index for gives an empty result.
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
            for (name, (uuid, has_profile)) in map {
                // Skip players without a profile so suggestions never lead to an
                // empty "no profile data" page.
                if has_profile && name.to_lowercase().contains(&name_lower) {
                    results.push((PlatformEdition::Java, name, uuid));
                }
            }
        }

        if let Some(map) = bedrock_res {
            for (name, (uuid, has_profile)) in map {
                if has_profile && name.to_lowercase().contains(&name_lower) {
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

    /// One page of an archived snapshot of a board, numbered as [`Self::fetch_leaderboard`].
    ///
    /// # Errors
    ///
    /// If the page does not exist or does not decode.
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

/// Decodes a names index, in either of the two layouts the tree has held.
///
/// The current one maps a name to its UUID and a has-profile flag; the one before it mapped a name
/// straight to a UUID. An entry in the older layout is reported as having a profile, which keeps
/// it searchable: dropping it would make a tree that has not been reconverted look like a tree
/// with no players in it.
fn decode_name_index(bytes: &[u8]) -> Option<HashMap<String, (String, bool)>> {
    if bytes.is_empty() {
        return None;
    }

    // Current layout: name -> (uuid, has_profile).
    if let Ok(map) = postcard::from_bytes::<HashMap<String, (String, bool)>>(bytes) {
        return Some(map);
    }

    // Legacy layout: name -> uuid (no has_profile flag).
    if let Ok(legacy) = postcard::from_bytes::<HashMap<String, String>>(bytes) {
        return Some(
            legacy
                .into_iter()
                .map(|(name, uuid)| (name, (uuid, true)))
                .collect(),
        );
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decodes_current_layout_preserving_has_profile() {
        let mut index: HashMap<String, (String, bool)> = HashMap::new();
        index.insert("relyh".into(), ("68b61e3c".into(), true));
        index.insert("ghost".into(), ("deadbeef".into(), false));
        let bytes = postcard::to_stdvec(&index).unwrap();

        let decoded = decode_name_index(&bytes).expect("current layout should decode");
        assert_eq!(decoded.get("relyh"), Some(&("68b61e3c".into(), true)));
        assert_eq!(decoded.get("ghost"), Some(&("deadbeef".into(), false)));
    }

    #[test]
    fn falls_back_to_legacy_layout_as_has_profile() {
        // Legacy data: name -> uuid (no has_profile flag).
        let mut legacy: HashMap<String, String> = HashMap::new();
        legacy.insert(
            "relyh".into(),
            "68b61e3c-4be0-4c0c-8897-6a8d3703fe9a".into(),
        );
        legacy.insert("geno".into(), "ddd3b782-ba30-4cc1-9c43-8829eeed5b0e".into());
        let bytes = postcard::to_stdvec(&legacy).unwrap();

        let decoded = decode_name_index(&bytes).expect("legacy layout should decode via fallback");
        assert_eq!(decoded.len(), 2);
        // Legacy entries must be treated as having a profile so they stay
        // searchable.
        for (_name, (_uuid, has_profile)) in &decoded {
            assert!(
                has_profile,
                "legacy entries must be marked has_profile=true"
            );
        }
        assert_eq!(
            decoded.get("relyh").map(|(u, _)| u.as_str()),
            Some("68b61e3c-4be0-4c0c-8897-6a8d3703fe9a")
        );
    }

    #[test]
    fn empty_payload_returns_none() {
        assert!(decode_name_index(&[]).is_none());
    }
}
