use binary_layout::binary_layout;
use serde::{Deserialize, Serialize};

binary_layout!(binary_leaderboard, BigEndian, {
   player_id: u64,
    score: u64
});

pub const BINARY_LEADERBOARD_SIZE: usize = binary_leaderboard::SIZE.unwrap();

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct LeaderboardMeta {
    pub save_time: String,
    pub save_time_unix: u32,
    pub save_id: u16,
    pub total_entries: u32,
    pub total_pages: u16,
}
