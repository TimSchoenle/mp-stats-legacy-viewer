use anyhow::{Context, Result};
use mp_stats_common::compression::read_lzma_bin;
use mp_stats_core::models::*;
use std::path::PathBuf;

pub struct BedrockLoader {
    data_root: PathBuf,
}

impl BedrockLoader {
    pub fn new(data_root: PathBuf) -> Self {
        Self { data_root }
    }

    pub async fn fetch_leaderboard(
        &self,
        board: &str,
        game: &str,
        stat: &str,
        chunk: u32,
    ) -> Result<Vec<BedrockLeaderboardChunk>> {
        let path = self.data_root.join(format!(
            "bedrock/leaderboards/{}/{}/{}/chunk_{:04}.bin",
            board, game, stat, chunk
        ));

        read_lzma_bin(&path)
            .with_context(|| format!("Failed to load bedrock leaderboard from {}", path.display()))
    }

    pub async fn fetch_player(&self, name: &str) -> Result<BedrockPlayerProfile> {
        if name.len() < 2 {
            anyhow::bail!("Invalid bedrock name length: {}", name);
        }

        let prefix = &name[..2];
        let path = self
            .data_root
            .join(format!("bedrock/players/{}/{}.bin", prefix, name));

        read_lzma_bin(&path)
            .with_context(|| format!("Failed to load bedrock player from {}", path.display()))
    }

    pub async fn fetch_meta(&self) -> Result<BedrockMeta> {
        let path = self.data_root.join("bedrock/meta.bin");
        read_lzma_bin(&path)
            .with_context(|| format!("Failed to load bedrock meta from {}", path.display()))
    }

    pub async fn fetch_game_stats(&self, game_id: &str) -> Result<BedrockGameData> {
        let path = self
            .data_root
            .join(format!("bedrock/games/{}.bin", game_id));
        read_lzma_bin(&path)
            .with_context(|| format!("Failed to load bedrock game stats from {}", path.display()))
    }
}
