use anyhow::Result;
use mp_stats_core::DataProvider;
use mp_stats_core::models::*;
use std::path::PathBuf;

mod loaders;
mod resolver;

use loaders::{BedrockLoader, JavaLoader};
use resolver::PlayerResolver;

/// Server-side data provider implementation
pub struct ServerDataProvider {
    java_loader: JavaLoader,
    bedrock_loader: BedrockLoader,
    resolver: PlayerResolver,
}

impl ServerDataProvider {
    pub fn new(data_root: PathBuf) -> Self {
        let java_loader = JavaLoader::new(data_root.clone());
        let bedrock_loader = BedrockLoader::new(data_root.clone());
        let resolver = PlayerResolver::new(data_root.clone());

        Self {
            java_loader,
            bedrock_loader,
            resolver,
        }
    }
}

#[async_trait::async_trait]
impl DataProvider for ServerDataProvider {
    async fn fetch_java_meta(&self) -> Result<JavaMeta> {
        self.java_loader.fetch_meta().await
    }

    async fn fetch_id_map(&self) -> Result<IdMap> {
        self.java_loader.fetch_id_map().await
    }

    async fn fetch_java_leaderboard(
        &self,
        board: &str,
        game: &str,
        stat: &str,
        chunk: u32,
    ) -> Result<Vec<LeaderboardEntry>> {
        self.java_loader
            .fetch_leaderboard(board, game, stat, chunk)
            .await
    }

    async fn fetch_java_player(&self, uuid: &str) -> Result<JavaPlayerProfile> {
        self.java_loader.fetch_player(uuid).await
    }

    async fn fetch_bedrock_leaderboard(
        &self,
        board: &str,
        game: &str,
        stat: &str,
        chunk: u32,
    ) -> Result<Vec<BedrockLeaderboardChunk>> {
        self.bedrock_loader
            .fetch_leaderboard(board, game, stat, chunk)
            .await
    }

    async fn fetch_bedrock_player(&self, name: &str) -> Result<BedrockPlayerProfile> {
        self.bedrock_loader.fetch_player(name).await
    }

    async fn find_player_uuid(&self, name: &str) -> Result<Option<NameLookup>> {
        self.resolver.find_player_uuid(name).await
    }

    async fn fetch_bedrock_meta(&self) -> Result<BedrockMeta> {
        self.bedrock_loader.fetch_meta().await
    }

    async fn fetch_bedrock_game_stats(&self, game_id: &str) -> Result<BedrockGameData> {
        self.bedrock_loader.fetch_game_stats(game_id).await
    }

    async fn fetch_game_leaderboards(&self, game_id: &str) -> Result<GameLeaderboardData> {
        self.java_loader.fetch_game_leaderboards(game_id).await
    }

    async fn fetch_history_snapshots(
        &self,
        board: &str,
        game: &str,
        stat: &str,
    ) -> Result<Vec<mp_stats_core::models::HistoricalSnapshot>> {
        self.java_loader
            .fetch_history_snapshots(board, game, stat)
            .await
    }

    async fn fetch_history_leaderboard(
        &self,
        board: &str,
        game: &str,
        stat: &str,
        snapshot_id: &str,
        chunk: u32,
    ) -> Result<Vec<LeaderboardEntry>> {
        self.java_loader
            .fetch_history_leaderboard(board, game, stat, snapshot_id, chunk)
            .await
    }
}
