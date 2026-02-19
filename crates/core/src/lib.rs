pub mod models;
pub mod routes;

use anyhow::Result;
pub use models::HistoricalSnapshot;
use models::*;

// Re-export common constants for frontend use
pub use mp_stats_common::formats::raw::ENTRIES_PER_PAGE_F64;

// --- Data Provider Type Aliases ---
// When used in a WASM/client context, use Rc (single-threaded)
// When used in a server context, use Arc (multi-threaded)
#[cfg(not(target_arch = "wasm32"))]
pub type DataProviderType = std::sync::Arc<dyn DataProvider + Send + Sync>;
#[cfg(target_arch = "wasm32")]
pub type DataProviderType = std::rc::Rc<dyn DataProvider>;

#[derive(Clone)]
pub struct DataProviderWrapper(pub DataProviderType);

impl PartialEq for DataProviderWrapper {
    fn eq(&self, other: &Self) -> bool {
        #[cfg(not(target_arch = "wasm32"))]
        {
            std::sync::Arc::ptr_eq(&self.0, &other.0)
        }
        #[cfg(target_arch = "wasm32")]
        {
            std::rc::Rc::ptr_eq(&self.0, &other.0)
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct PreloadedLeaderboardData(pub Vec<LeaderboardEntry>);

// Define the DataProvider trait
#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
pub trait DataProvider {
    async fn fetch_meta(&self, edition: &PlatformEdition) -> Result<JavaMeta>;
    async fn fetch_id_map(&self, edition: &PlatformEdition) -> Result<IdMap>;

    async fn fetch_leaderboard(
        &self,
        edition: &PlatformEdition,
        board: &str,
        game: &str,
        stat: &str,
        chunk: u32,
    ) -> Result<Vec<LeaderboardEntry>>;
    async fn fetch_player(
        &self,
        edition: &PlatformEdition,
        uuid: &str,
    ) -> Result<JavaPlayerProfile>;

    async fn find_player_uuid(
        &self,
        edition: &PlatformEdition,
        name: &str,
    ) -> Result<Option<NameLookup>>;

    async fn fetch_game_leaderboards(
        &self,
        edition: &PlatformEdition,
        game_id: &str,
    ) -> Result<GameLeaderboardData>;

    // History leaderboard methods
    async fn fetch_history_snapshots(
        &self,
        edition: &PlatformEdition,
        board: &str,
        game: &str,
        stat: &str,
    ) -> Result<Vec<HistoricalSnapshot>>;

    async fn fetch_history_leaderboard(
        &self,
        edition: &PlatformEdition,
        board: &str,
        game: &str,
        stat: &str,
        snapshot_id: &str,
        chunk: u32,
    ) -> Result<Vec<LeaderboardEntry>>;
}
