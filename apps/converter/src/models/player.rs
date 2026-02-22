use binary_layout::binary_layout;

binary_layout!(binary_player, BigEndian, {
   board_id: u32,
   game_id: u32,
   stat_id: u32,
   save_id: u32,
   score: u64,
   rank: u32,
   timestamp: u64
});

pub const BINARY_PLAYER_SIZE: usize = binary_player::SIZE.unwrap();
