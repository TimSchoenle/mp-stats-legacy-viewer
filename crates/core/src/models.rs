use serde::{Deserialize, Serialize};
use smol_str::SmolStr;
use std::collections::HashMap;
use std::error::Error;
use std::fmt::Display;
use std::str::FromStr;

/// Name of the global (all-time) leaderboard board. This is the board whose
/// latest snapshot is used to expose the per-category "game list" stats.
pub const GLOBAL_BOARD: &str = "All";

/// Stateful helper implementing **standard competition ranking** ("1224").
///
/// Entries must be fed in *descending* score order. Entries that share a score
/// receive the same rank, and the next distinct (lower) score jumps to its
/// 1-based positional index, leaving gaps where ties occurred. Centralizing the
/// algorithm here keeps position/rank calculation identical between the
/// leaderboard and player-profile pipelines so a player with a given score is
/// always shown at the same position in both views.
///
/// ```
/// use mp_stats_core::models::CompetitionRanker;
///
/// let mut ranker = CompetitionRanker::new();
/// assert_eq!(ranker.next_rank(100), 1); // 1st place
/// assert_eq!(ranker.next_rank(100), 1); // tie -> same rank
/// assert_eq!(ranker.next_rank(90), 3);  // next distinct score skips #2
/// ```
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct CompetitionRanker {
    /// 1-based ordinal of the most recently ranked entry.
    position: u32,
    /// Score of the most recently ranked entry, if any.
    last_score: Option<u64>,
    /// Rank assigned to the most recently ranked entry.
    last_rank: u32,
}

impl CompetitionRanker {
    /// Create a fresh ranker positioned before the first entry.
    pub fn new() -> Self {
        Self::default()
    }

    /// Feed the next entry's `score` (which must be `<=` the previously fed
    /// score) and obtain its competition rank.
    pub fn next_rank(&mut self, score: u64) -> u32 {
        self.position += 1;
        let rank = if self.last_score == Some(score) {
            self.last_rank
        } else {
            self.position
        };
        self.last_score = Some(score);
        self.last_rank = rank;
        rank
    }
}

/// Compute a `score -> rank` lookup applying standard competition ranking
/// ("1224") to an unordered multiset of scores supplied as `score -> count`.
///
/// The rank of a score equals `1 + (number of entries with a strictly greater
/// score)`, so every entry sharing a score gets the same position. This is the
/// batch counterpart to [`CompetitionRanker`] (used for the streaming,
/// already-sorted leaderboard case) and yields identical positions.
pub fn competition_ranks_by_score(counts: &HashMap<u64, u64>) -> HashMap<u64, u32> {
    let mut scores: Vec<u64> = counts.keys().copied().collect();
    // Highest score first so prefix-summing the counts gives "entries ahead".
    scores.sort_unstable_by(|a, b| b.cmp(a));

    let mut table = HashMap::with_capacity(scores.len());
    let mut ahead: u64 = 0;
    for score in scores {
        table.insert(score, (ahead + 1) as u32);
        ahead += counts.get(&score).copied().unwrap_or(0);
    }
    table
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Game {
    pub id: SmolStr,
    pub name: SmolStr,
    pub description: Option<SmolStr>,
    pub icon: Option<SmolStr>,
    /// Total number of snapshots collected for this game across all of its
    /// leaderboards. Defaults to `0` for legacy payloads.
    #[serde(default)]
    pub total_snapshots: u64,
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
    /// Total number of distinct snapshots collected for this game across all of
    /// its leaderboards. Defaults to `0` for legacy payloads.
    #[serde(default)]
    pub total_snapshots: u64,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct IdMapValue {
    pub name: SmolStr,
    pub description: Option<SmolStr>,
    /// Total number of snapshots collected for this entry (games only).
    /// Defaults to `0` and is populated by the converter after game metadata
    /// processing.
    #[serde(default)]
    pub total_snapshots: u64,
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

    #[test]
    fn competition_ranker_shares_rank_for_equal_scores() {
        let mut ranker = CompetitionRanker::new();
        // Scores fed in descending order; ties share a rank and the next
        // distinct score skips the consumed positions ("1224" ranking).
        let ranks: Vec<u32> = [100, 100, 100, 90, 80, 80, 70]
            .into_iter()
            .map(|s| ranker.next_rank(s))
            .collect();

        assert_eq!(ranks, vec![1, 1, 1, 4, 5, 5, 7]);
    }

    #[test]
    fn competition_ranker_strictly_decreasing_is_sequential() {
        let mut ranker = CompetitionRanker::new();
        let ranks: Vec<u32> = [50, 40, 30]
            .into_iter()
            .map(|s| ranker.next_rank(s))
            .collect();
        assert_eq!(ranks, vec![1, 2, 3]);
    }

    #[test]
    fn competition_ranks_by_score_matches_streaming_ranker() {
        // Same multiset as `competition_ranker_shares_rank_for_equal_scores`.
        let counts: HashMap<u64, u64> = HashMap::from([(100, 3), (90, 1), (80, 2), (70, 1)]);

        let table = competition_ranks_by_score(&counts);

        assert_eq!(table.get(&100), Some(&1));
        assert_eq!(table.get(&90), Some(&4));
        assert_eq!(table.get(&80), Some(&5));
        assert_eq!(table.get(&70), Some(&7));
    }

    #[test]
    fn competition_ranks_by_score_handles_empty_input() {
        let table = competition_ranks_by_score(&HashMap::new());
        assert!(table.is_empty());
    }
}
