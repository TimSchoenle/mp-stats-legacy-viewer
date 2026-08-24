//! The leaderboard chunk record in the dumps: a player id and a score, big-endian, no header.

use binary_layout::binary_layout;

binary_layout!(binary_leaderboard, BigEndian, {
   player_id: u64,
    score: u64
});

/// Bytes one entry occupies, which is also the stride a chunk is walked at.
pub const BINARY_LEADERBOARD_SIZE: usize = binary_leaderboard::SIZE.unwrap();
