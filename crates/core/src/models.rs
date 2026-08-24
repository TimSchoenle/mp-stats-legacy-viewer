//! Every record the converted tree holds, plus the ranking the converter and the client both
//! read positions out of.
//!
//! The layout of the tree these live in is at the crate root. What is not here is any path into
//! it: those are [`crate::routes`].

use serde::{Deserialize, Serialize};
use smol_str::SmolStr;
use std::collections::HashMap;
use std::error::Error;
use std::fmt::Display;
use std::str::FromStr;

/// The directory the dumps give the all-time board, matched case-insensitively.
///
/// It is the only board whose leading entry is published on the game page, and the only one whose
/// stats reach a player profile; the periodic boards beside it are browsable but nothing is
/// aggregated out of them.
pub const GLOBAL_BOARD: &str = "All";

/// Assigns standard competition ranks ("1224") to a stream of entries already in descending score
/// order.
///
/// Feeding a score greater than the previous one produces a rank, but not a meaningful one: the
/// ranker keeps no history beyond the last entry, so it cannot notice the stream went backwards.
/// The batch counterpart for unordered input is [`competition_ranks_by_score`], and the two agree
/// on every ordering the converter actually produces.
///
/// ```
/// use mp_stats_core::models::CompetitionRanker;
///
/// let mut ranker = CompetitionRanker::new();
/// assert_eq!(ranker.next_rank(100), 1);
/// assert_eq!(ranker.next_rank(100), 1);
/// assert_eq!(ranker.next_rank(90), 3);
/// ```
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct CompetitionRanker {
    position: u32,
    last_score: Option<u64>,
    last_rank: u32,
}

impl CompetitionRanker {
    /// A ranker positioned before the first entry, so the next call returns rank 1.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// The rank of the next entry: the previous entry's rank if `score` ties it, otherwise this
    /// entry's 1-based position in the stream.
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

/// Ranks a whole multiset at once, given how many entries achieved each score.
///
/// A score's rank is one more than the number of entries scoring strictly above it, which is the
/// same answer [`CompetitionRanker`] reaches one entry at a time. Every score in `counts` appears
/// in the result; an empty input gives an empty table.
#[must_use]
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

/// One game as the landing page lists it.
///
/// Assembled in the client out of [`IdMap::games`] rather than fetched, so no file in the tree
/// holds one.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Game {
    /// The directory the game's leaderboards sit under, which is also what
    /// [`crate::routes::game_bin`] takes.
    pub id: SmolStr,
    /// What the game is called on screen. The dumps carry no separate label, so this equals
    /// [`Self::id`].
    pub name: SmolStr,
    /// The blurb from the dump's ID map, absent when it carried none.
    pub description: Option<SmolStr>,
    /// Always absent. The dumps hold no icons and the converter writes none.
    pub icon: Option<SmolStr>,
    /// Snapshots archived across every one of the game's boards, `0` in payloads written before
    /// the count existed.
    #[serde(default)]
    pub total_snapshots: u64,
}

/// The game list of one edition.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct PlatformMeta {
    /// Sorted by name, since the ID map they came out of is unordered.
    pub games: Vec<Game>,
}

/// One ranked row, carrying its score as a float.
// Nothing writes or reads one: the tree stores `LeaderboardPage`, and the client hands out
// `LeaderboardEntry`.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct LeaderboardChunk {
    /// Competition rank, 1-based.
    pub rank: u32,
    /// The player's UUID.
    pub uuid: SmolStr,
    /// The player's display name.
    pub name: SmolStr,
    /// The score the rank was derived from.
    pub score: f64,
}

/// One page of a leaderboard, stored column by column.
///
/// The four vectors are the same length and index `i` of each belongs to the same entry. Four
/// runs of like-typed values compress far harder under LZMA than the same values interleaved,
/// which is the whole reason for the shape. Rows are in rank order, so index 0 is the best entry
/// on the page.
///
/// Every page but the last holds [`mp_stats_common::formats::raw::ENTRIES_PER_PAGE`] entries.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct LeaderboardPage {
    /// Competition ranks, 1-based and counted from the start of the board rather than the page.
    pub ranks: Vec<u32>,
    /// Player UUIDs.
    pub uuids: Vec<SmolStr>,
    /// Player display names.
    pub names: Vec<SmolStr>,
    /// The scores the ranks were derived from.
    pub scores: Vec<u64>,
}

/// One row of a [`LeaderboardPage`], zipped back together for display.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct LeaderboardEntry {
    /// Competition rank, 1-based and counted from the start of the board.
    pub rank: u32,
    /// The player's UUID, which is what a link to their profile is built from.
    pub uuid: SmolStr,
    /// The player's display name.
    pub name: SmolStr,
    /// The score the rank was derived from.
    pub score: u64,
}

/// Whoever leads a board's current snapshot, read off the first row of its first page.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct TopEntry {
    /// The leader's UUID.
    pub uuid: SmolStr,
    /// The leader's display name.
    pub name: SmolStr,
    /// The leading score.
    pub score: u64,
}

/// What is browsable for one board of one stat.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct LeaderboardMeta {
    /// The current snapshot and every archived one, in no particular order. The snapshot selector
    /// orders them by timestamp.
    pub snapshots: Vec<HistoricalSnapshot>,
    /// The leader of the current snapshot. Absent on every board but [`GLOBAL_BOARD`], and in
    /// payloads written before the field existed.
    #[serde(default)]
    pub top: Option<TopEntry>,
}

/// Everything one game's page needs, which is one fetch of `games/<game>.bin.xz`.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct GameLeaderboardData {
    /// The directory the game's leaderboards sit under.
    pub game_id: SmolStr,
    /// What the game is called on screen, which the dumps make the same string as
    /// [`Self::game_id`].
    pub game_name: SmolStr,
    /// The blurb from the dump's ID map, absent when it carried none.
    pub description: Option<SmolStr>,
    /// Always absent. The dumps hold no icons and the converter writes none.
    pub icon: Option<SmolStr>,
    /// Stat name, then board name. Stat is the outer key because the page groups by category and
    /// offers the boards as a switch inside one.
    pub stats: HashMap<SmolStr, HashMap<SmolStr, LeaderboardMeta>>,
    /// Ranked entries summed over the current snapshot of every board, `0` in payloads written
    /// before the count existed.
    #[serde(default)]
    pub total_entries: u64,
    /// Snapshots summed over every board, counting the current one, `0` in payloads written
    /// before the count existed.
    #[serde(default)]
    pub total_snapshots: u64,
}

/// What one numeric id in a profile stands for.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct IdMapValue {
    /// The name, which for a game and for a board is also its directory in the tree.
    pub name: SmolStr,
    /// The blurb from the dump's ID map, absent when it carried none.
    pub description: Option<SmolStr>,
    /// Snapshots archived for this game. `0` on boards and stats, and on a game the converter
    /// found no leaderboard directory for.
    #[serde(default)]
    pub total_snapshots: u64,
}

/// The three id tables every [`StatRaw`] resolves through, one fetch for a whole edition.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct IdMap {
    /// Board ids, one of which names [`GLOBAL_BOARD`].
    pub boards: HashMap<u32, IdMapValue>,
    /// Game ids.
    pub games: HashMap<u32, IdMapValue>,
    /// Stat ids, which are shared across games rather than scoped to one: two games measuring
    /// wins resolve to the same entry here.
    pub stats: HashMap<u32, IdMapValue>,
}

/// One player's standing in one category, with the three ids left unresolved.
///
/// Resolving them costs the whole [`IdMap`], which is one fetch for the edition rather than three
/// strings repeated on every row of every profile shard.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct StatRaw {
    /// Key into [`IdMap::boards`]. Only [`GLOBAL_BOARD`] survives into a profile.
    pub board_id: u32,
    /// Key into [`IdMap::games`].
    pub game_id: u32,
    /// Key into [`IdMap::stats`].
    pub stat_id: u32,
    /// The player's score in this category.
    pub score: u64,
    /// Competition rank, 1-based, recomputed across the whole player population rather than taken
    /// from the dump.
    pub rank: u32,
    /// When the dump this entry came from was taken, in Unix seconds.
    pub save_time: u64,
}

/// One player, as a profile shard stores them under their UUID.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct PlayerProfile {
    /// The player's UUID, whose first three characters name the shard this came out of.
    pub uuid: SmolStr,
    /// The display name. The converter substitutes the UUID where the dump's dictionary carried
    /// no name, so a converted profile always has one.
    pub name: Option<SmolStr>,
    /// Every category the player placed in, in no particular order.
    pub stats: Vec<StatRaw>,
}

/// The headline numbers on a profile page, counted over one [`PlayerProfile`].
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
    /// Counts this profile's stats into a [`PlayerSummary`].
    ///
    /// An entry with `rank == 0` still counts towards the score total and the games played, and
    /// towards nothing rank-based. The total saturates rather than wrapping.
    #[must_use]
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

/// A name resolved to the player it belongs to and the shard holding them.
// Nothing writes or reads one: the names index in the tree is a plain map.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct NameLookup {
    /// The player's UUID.
    pub uuid: SmolStr,
    /// The shard the player's profile is in.
    #[serde(rename = "shard")]
    pub shard_path: SmolStr,
}

/// The `_meta.json` beside a snapshot in the dumps, read by the converter and by nothing else.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct MetaFile {
    /// When the snapshot was taken, as the dump spells it: ISO 8601 with milliseconds.
    pub save_time: String,
    /// The same instant in Unix seconds, which is the one that reaches the client.
    pub save_time_unix: u64,
    /// The dump's own identifier for the save this snapshot came out of.
    pub save_id: u32,
    /// Ranked entries in the snapshot.
    pub total_entries: u32,
    /// Pages the dump split those entries across, which is not the page count the conversion
    /// produces.
    pub total_pages: u32,
}

/// One browsable state of a board, whether current or archived.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct HistoricalSnapshot {
    /// The directory the snapshot's pages sit in: `latest` for the current one, a timestamped
    /// name from the dump's history archive for an older one.
    pub snapshot_id: SmolStr,
    /// When the snapshot was taken, in Unix seconds. The selector orders by this.
    pub timestamp: u64,
    /// Pages the dump split the snapshot across, which is not the page count the conversion
    /// produces.
    pub total_pages: u32,
    /// Ranked entries in the snapshot.
    pub total_entries: u32,
}

/// Which of the two Minecraft platforms a set of statistics came from.
///
/// The two are converted and browsed separately from the same dump layout, so an edition names a
/// subtree, a route segment and a theme, and no record is ever shared between them.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Hash)]
pub enum PlatformEdition {
    /// The Java server, whose players are identified by real UUIDs.
    Java,
    /// The Bedrock server, whose dumps carry names where a UUID would be, so nothing here can
    /// assume a profile key parses as one.
    Bedrock,
}

/// Renders [`PlatformEdition::directory_name`], not [`PlatformEdition::display_name`], because
/// this is what the router interpolates into a URL.
impl Display for PlatformEdition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.directory_name())
    }
}

/// A string that named neither edition.
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

/// Accepts either [`PlatformEdition::directory_name`] in any case, with surrounding whitespace
/// trimmed. This is what turns the `:edition` route segment back into a value.
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
    /// The edition's subtree in the converted data, which is also its route segment and the
    /// spelling [`FromStr`] parses.
    #[must_use]
    pub fn directory_name(&self) -> &'static str {
        match self {
            PlatformEdition::Java => "java",
            PlatformEdition::Bedrock => "bedrock",
        }
    }

    /// The edition's name as it is written on screen.
    #[must_use]
    pub fn display_name(&self) -> &'static str {
        match self {
            PlatformEdition::Java => "Java",
            PlatformEdition::Bedrock => "Bedrock",
        }
    }

    /// Both editions, in the order the navigation and the conversion run walk them.
    pub const VARIANTS: [Self; 2] = [Self::Java, Self::Bedrock];

    /// Iterates [`Self::VARIANTS`], so nothing has to spell both editions out to visit them.
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
