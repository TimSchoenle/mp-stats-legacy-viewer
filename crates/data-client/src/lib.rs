pub mod api;

use anyhow::Result;
use mp_stats_core::DataProvider;
use mp_stats_core::models::*;

pub struct ClientDataProvider;

#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl DataProvider for ClientDataProvider {
    async fn fetch_java_meta(&self) -> Result<JavaMeta> {
        api::fetch_java_meta().await.map_err(|e| anyhow::anyhow!(e))
    }

    async fn fetch_id_map(&self) -> Result<IdMap> {
        api::fetch_id_map().await.map_err(|e| anyhow::anyhow!(e))
    }

    async fn fetch_java_leaderboard(
        &self,
        board: &str,
        game: &str,
        stat: &str,
        chunk: u32,
    ) -> Result<Vec<LeaderboardEntry>> {
        api::fetch_java_leaderboard(board, game, stat, chunk)
            .await
            .map_err(|e: gloo_net::Error| anyhow::anyhow!(e))
    }

    async fn fetch_java_player(&self, uuid: &str) -> Result<JavaPlayerProfile> {
        api::fetch_java_player(uuid)
            .await
            .map_err(|e| anyhow::anyhow!(e))
    }

    async fn fetch_bedrock_leaderboard(
        &self,
        board: &str,
        game: &str,
        stat: &str,
        chunk: u32,
    ) -> Result<Vec<BedrockLeaderboardChunk>> {
        api::fetch_bedrock_leaderboard(board, game, stat, chunk)
            .await
            .map_err(|e| anyhow::anyhow!(e))
    }

    async fn fetch_bedrock_player(&self, name: &str) -> Result<BedrockPlayerProfile> {
        api::fetch_bedrock_player(name)
            .await
            .map_err(|e| anyhow::anyhow!(e))
    }

    async fn find_player_uuid(&self, name: &str) -> Result<Option<NameLookup>> {
        api::find_player_uuid(name)
            .await
            .map_err(|e| anyhow::anyhow!(e))
    }

    async fn fetch_bedrock_meta(&self) -> Result<BedrockMeta> {
        api::fetch_bedrock_meta()
            .await
            .map_err(|e| anyhow::anyhow!(e))
    }

    async fn fetch_bedrock_game_stats(&self, game_id: &str) -> Result<BedrockGameData> {
        api::fetch_bedrock_game_stats(game_id)
            .await
            .map_err(|e| anyhow::anyhow!(e))
    }

    async fn fetch_game_leaderboards(&self, game_id: &str) -> Result<GameLeaderboardData> {
        api::fetch_game_leaderboards(game_id)
            .await
            .map_err(|e| anyhow::anyhow!(e))
    }

    async fn fetch_history_snapshots(
        &self,
        board: &str,
        game: &str,
        stat: &str,
    ) -> Result<Vec<mp_stats_core::HistoricalSnapshot>> {
        api::fetch_history_snapshots(board, game, stat)
            .await
            .map_err(|e| anyhow::anyhow!(e))
    }

    async fn fetch_history_leaderboard(
        &self,
        board: &str,
        game: &str,
        stat: &str,
        snapshot_id: &str,
        chunk: u32,
    ) -> Result<Vec<LeaderboardEntry>> {
        api::fetch_history_leaderboard(board, game, stat, snapshot_id, chunk)
            .await
            .map_err(|e| anyhow::anyhow!(e))
    }
}
