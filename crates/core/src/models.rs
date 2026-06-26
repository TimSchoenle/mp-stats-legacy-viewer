use serde::{Deserialize, Serialize};
use smol_str::SmolStr;
use std::collections::HashMap;
use std::error::Error;
use std::fmt::Display;
use std::str::FromStr;

/// Name of the global (all-time) leaderboard board. This is the board whose
/// latest snapshot is used to expose the per-category "game list" stats.
pub const GLOBAL_BOARD: &str = "All";

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Game {
    pub id: SmolStr,
    pub name: SmolStr,
    pub description: Option<SmolStr>,
    pub icon: Option<SmolStr>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct PlatformMeta {
    pub games: Vec<Game>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct LeaderboardChunk {
    pub rank: u32,
    pub uuid: SmolStr,
    pub name: SmolStr,
    pub score: f64,
}

/// Structure-of-Arrays (SoA) format for leaderboard pages
/// Stores 1,000 entries per page in columnar layout for better compression
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct LeaderboardPage {
    pub ranks: Vec<u32>,
    pub uuids: Vec<SmolStr>,
    pub names: Vec<SmolStr>,
    pub scores: Vec<u64>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct LeaderboardEntry {
    pub rank: u32,
    pub uuid: SmolStr,
    pub name: SmolStr,
    pub score: u64,
}

/// Highlighted leaderboard entry used for the per-category "game list" stats,
/// i.e. the `#1 holder` of a board's latest snapshot and their top score.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct TopEntry {
    pub uuid: SmolStr,
    pub name: SmolStr,
    pub score: u64,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct LeaderboardMeta {
    pub snapshots: Vec<HistoricalSnapshot>,
    /// The `#1 holder` of this board's latest snapshot (highest score).
    /// Defaults to `None` for legacy payloads.
    #[serde(default)]
    pub top: Option<TopEntry>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct GameLeaderboardData {
    pub game_id: SmolStr,
    pub game_name: SmolStr,
    pub description: Option<SmolStr>,
    pub icon: Option<SmolStr>,
    pub stats: HashMap<SmolStr, HashMap<SmolStr, LeaderboardMeta>>,
    /// Total number of ranked entries across all of the game's latest
    /// leaderboards. Defaults to `0` for legacy payloads.
    #[serde(default)]
    pub total_entries: u64,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct IdMapValue {
    pub name: SmolStr,
    pub description: Option<SmolStr>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct IdMap {
    pub boards: HashMap<u32, IdMapValue>,
    pub games: HashMap<u32, IdMapValue>,
    pub stats: HashMap<u32, IdMapValue>,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct StatRaw {
    pub board_id: u32,
    pub game_id: u32,
    pub stat_id: u32,
    pub score: u64,
    pub rank: u32,
    pub save_time: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct PlayerProfile {
    pub uuid: SmolStr,
    pub name: Option<SmolStr>,
    pub stats: Vec<StatRaw>,
}

/// Aggregated, ready-to-display ranking metrics derived from a
/// [`PlayerProfile`]. Centralizing this here keeps rank calculation
/// consistent between the converter, server and frontend.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Default)]
pub struct PlayerSummary {
    /// Number of ranked stat entries (board/game/stat combinations).
    pub total_categories: u32,
    /// Number of distinct games the player appears in.
    pub games_played: u32,
    /// Best (lowest) rank achieved across all stats. `0` means "no rank".
    pub best_rank: u32,
    /// Number of stats where the player ranks in the top 10.
    pub top_ten: u32,
    /// Number of stats where the player ranks in the top 100.
    pub top_hundred: u32,
    /// Sum of all scores across the player's ranked stats.
    pub total_score: u64,
}

impl PlayerProfile {
    /// Compute aggregated ranking metrics for this profile.
    ///
    /// Only entries with a valid rank (`rank > 0`) contribute to the
    /// rank-based metrics, while every entry contributes to the score total
    /// and the set of games played.
    pub fn summary(&self) -> PlayerSummary {
        use std::collections::BTreeSet;

        let mut summary = PlayerSummary::default();
        let mut games: BTreeSet<u32> = BTreeSet::new();

        summary.total_categories = self.stats.len() as u32;

        for stat in &self.stats {
            games.insert(stat.game_id);
            summary.total_score = summary.total_score.saturating_add(stat.score);

            if stat.rank > 0 {
                if summary.best_rank == 0 || stat.rank < summary.best_rank {
                    summary.best_rank = stat.rank;
                }
                if stat.rank <= 10 {
                    summary.top_ten += 1;
                }
                if stat.rank <= 100 {
                    summary.top_hundred += 1;
                }
            }
        }

        summary.games_played = games.len() as u32;
        summary
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct NameLookup {
    pub uuid: SmolStr,
    #[serde(rename = "shard")]
    pub shard_path: SmolStr,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct MetaFile {
    pub save_time: String,
    pub save_time_unix: u64,
    pub save_id: u32,
    pub total_entries: u32,
    pub total_pages: u32,
}

// --- Historical Leaderboard Models ---

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct HistoricalSnapshot {
    pub snapshot_id: SmolStr,
    pub timestamp: u64,
    pub total_pages: u32,
    pub total_entries: u32,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Hash)]
pub enum PlatformEdition {
    Java,
    Bedrock,
}

impl Display for PlatformEdition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.directory_name())
    }
}

#[derive(Debug, Clone)]
pub struct PlatformEditionParseError {
    input: String,
}

impl Display for PlatformEditionParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "invalid platform edition '{}'; expected '{}' or '{}'",
            self.input,
            PlatformEdition::Java.directory_name(),
            PlatformEdition::Bedrock.directory_name()
        )
    }
}

impl Error for PlatformEditionParseError {}
impl FromStr for PlatformEdition {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let normalized = s.trim().to_ascii_lowercase();

        PlatformEdition::iter()
            .find(|edition| normalized == edition.directory_name().to_ascii_lowercase())
            .cloned()
            .ok_or(Box::from(PlatformEditionParseError {
                input: s.trim().to_string(),
            }))
    }
}

impl PlatformEdition {
    pub fn directory_name(&self) -> &'static str {
        match self {
            PlatformEdition::Java => "java",
            PlatformEdition::Bedrock => "bedrock",
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            PlatformEdition::Java => "Java",
            PlatformEdition::Bedrock => "Bedrock",
        }
    }

    pub const VARIANTS: [Self; 2] = [Self::Java, Self::Bedrock];

    pub fn iter() -> std::slice::Iter<'static, Self> {
        Self::VARIANTS.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn stat(game_id: u32, score: u64, rank: u32) -> StatRaw {
        StatRaw {
            board_id: 0,
            game_id,
            stat_id: 0,
            score,
            rank,
            save_time: 0,
        }
    }

    #[test]
    fn summary_aggregates_rank_metrics() {
        let profile = PlayerProfile {
            uuid: SmolStr::new("abc"),
            name: Some(SmolStr::new("Player")),
            stats: vec![
                stat(1, 100, 5),  // top 10 & top 100
                stat(1, 50, 80),  // top 100 only, same game
                stat(2, 200, 0),  // unranked, new game
                stat(3, 25, 150), // ranked but outside top 100
            ],
        };

        let summary = profile.summary();

        assert_eq!(summary.total_categories, 4);
        assert_eq!(summary.games_played, 3);
        assert_eq!(summary.best_rank, 5);
        assert_eq!(summary.top_ten, 1);
        assert_eq!(summary.top_hundred, 2);
        assert_eq!(summary.total_score, 375);
    }

    #[test]
    fn summary_of_empty_profile_is_default() {
        let profile = PlayerProfile {
            uuid: SmolStr::new("abc"),
            name: None,
            stats: vec![],
        };

        assert_eq!(profile.summary(), PlayerSummary::default());
    }

    #[test]
    fn summary_without_ranks_keeps_best_rank_zero() {
        let profile = PlayerProfile {
            uuid: SmolStr::new("abc"),
            name: None,
            stats: vec![stat(1, 10, 0), stat(1, 20, 0)],
        };

        let summary = profile.summary();
        assert_eq!(summary.best_rank, 0);
        assert_eq!(summary.top_ten, 0);
        assert_eq!(summary.top_hundred, 0);
        assert_eq!(summary.total_score, 30);
        assert_eq!(summary.games_played, 1);
    }
}
