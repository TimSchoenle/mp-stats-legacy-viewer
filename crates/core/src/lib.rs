//! The records the converted tree is made of, and the only module that spells a path into it.
//!
//! Nothing on either side of this crate answers a query. `apps/server` serves a directory and
//! `apps/frontend` fetches files out of it, so the directory layout is the query interface, and
//! [`routes`] is the one place a path into it is written down.
//!
//! # The converted tree
//!
//! Every file below is a Postcard value wrapped in LZMA, written by `apps/converter` and read
//! back through `mp_stats_common::compression`. `<edition>` is `java` or `bedrock`, and the two
//! trees are converted and browsed independently from the same dump layout.
//!
//! ```text
//! <edition>/meta/map.bin.xz                                IdMap
//! <edition>/games/<game>.bin.xz                            GameLeaderboardData
//! <edition>/leaderboards/<board>/<game>/<stat>/
//!         latest/chunk_0000.bin.xz                         LeaderboardPage
//!         latest/_meta.json                                copied from the dump, unread
//!         history/<snapshot>/chunk_0000.bin.xz             LeaderboardPage
//! <edition>/players/<UUID[..3] uppercased>.bin.xz          HashMap<String, PlayerProfile>
//! <edition>/names_index/<name[..3] lowercased>.bin.xz      HashMap<String, (String, bool)>
//! ```
//!
//! `data/README.md` documents the dumps going in. This is what comes out, and nothing else
//! records it.
//!
//! # Why the shards are cut where they are
//!
//! A leaderboard page holds [`mp_stats_common::formats::raw::ENTRIES_PER_PAGE`] entries and is
//! stored column by column rather than row by row: four runs of like-typed values compress far
//! harder under LZMA than the same values interleaved. Chunk numbers are zero-based and
//! zero-padded to four digits, so the site's page 1 is `chunk_0000` — the dumps number theirs
//! from one, and the conversion does not carry that over.
//!
//! A profile shard is keyed on the first three characters of the player's UUID, uppercased, and a
//! names-index shard on the first three of the name, lowercased. Both are the shortest prefix
//! that keeps a search or a profile lookup to a single fetch instead of an index of everyone. A
//! player whose UUID or name is shorter than three characters has no shard, and is therefore
//! unreachable.
//!
//! The names index maps a name to its UUID and to whether that player has a profile shard. The
//! flag exists because the two sets differ: a name is known for every player the dictionary
//! carries, while a profile exists only for players who placed on the all-time board. Without it
//! the search would offer suggestions that land on an empty page.
//!
//! # Ranks are computed, never carried
//!
//! A leaderboard chunk in the dumps stores no rank at all, and the rank in a profile stride is
//! sequential, which puts two players who scored the same in different places. Both are replaced
//! here by standard competition ranking, from [`models::CompetitionRanker`] over an ordered
//! stream and [`models::competition_ranks_by_score`] over an unordered one. The two agree by
//! construction, which is what makes a player's position on their profile the same as their
//! position on the board.

pub mod models;
pub mod routes;

pub use models::HistoricalSnapshot;
use models::LeaderboardEntry;

pub use mp_stats_common::formats::raw::ENTRIES_PER_PAGE_F64;

/// A batch of leaderboard rows.
// Nothing constructs one, so there is nothing truthful to add about when it holds what. Left as it
// is rather than given a purpose it does not have.
#[derive(Clone, PartialEq, Debug)]
pub struct PreloadedLeaderboardData(pub Vec<LeaderboardEntry>);
