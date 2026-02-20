pub mod models;
pub mod routes;

pub use models::HistoricalSnapshot;
use models::*;

// Re-export common constants for frontend use
pub use mp_stats_common::formats::raw::ENTRIES_PER_PAGE_F64;

#[derive(Clone, PartialEq, Debug)]
pub struct PreloadedLeaderboardData(pub Vec<LeaderboardEntry>);
