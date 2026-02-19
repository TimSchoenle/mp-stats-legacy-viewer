pub mod api;

use anyhow::Result;
use mp_stats_core::DataProvider;
use mp_stats_core::models::*;

pub struct ClientDataProvider;

#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl DataProvider for ClientDataProvider {
    async fn fetch_meta(&self, edition: &PlatformEdition) -> Result<JavaMeta> {
        api::fetch_meta(edition)
            .await
            .map_err(|e| anyhow::anyhow!(e))
    }

    async fn fetch_id_map(&self, edition: &PlatformEdition) -> Result<IdMap> {
        api::fetch_id_map(edition)
            .await
            .map_err(|e| anyhow::anyhow!(e))
    }

    async fn fetch_leaderboard(
        &self,
        edition: &PlatformEdition,
        board: &str,
        game: &str,
        stat: &str,
        chunk: u32,
    ) -> Result<Vec<LeaderboardEntry>> {
        api::fetch_leaderboard(edition, board, game, stat, chunk)
            .await
            .map_err(|e: gloo_net::Error| anyhow::anyhow!(e))
    }

    async fn fetch_player(
        &self,
        edition: &PlatformEdition,
        uuid: &str,
    ) -> Result<JavaPlayerProfile> {
        api::fetch_player(edition, uuid)
            .await
            .map_err(|e| anyhow::anyhow!(e))
    }

    async fn find_player_uuid(
        &self,
        edition: &PlatformEdition,
        name: &str,
    ) -> Result<Option<NameLookup>> {
        api::find_player_uuid(edition, name)
            .await
            .map_err(|e| anyhow::anyhow!(e))
    }

    async fn fetch_game_leaderboards(
        &self,
        edition: &PlatformEdition,
        game_id: &str,
    ) -> Result<GameLeaderboardData> {
        api::fetch_game_leaderboards(edition, game_id)
            .await
            .map_err(|e| anyhow::anyhow!(e))
    }

    async fn fetch_history_snapshots(
        &self,
        edition: &PlatformEdition,
        board: &str,
        game: &str,
        stat: &str,
    ) -> Result<Vec<HistoricalSnapshot>> {
        api::fetch_history_snapshots(edition, board, game, stat)
            .await
            .map_err(|e| anyhow::anyhow!(e))
    }

    async fn fetch_history_leaderboard(
        &self,
        edition: &PlatformEdition,
        board: &str,
        game: &str,
        stat: &str,
        snapshot_id: &str,
        chunk: u32,
    ) -> Result<Vec<LeaderboardEntry>> {
        api::fetch_history_leaderboard(edition, board, game, stat, snapshot_id, chunk)
            .await
            .map_err(|e| anyhow::anyhow!(e))
    }
}
