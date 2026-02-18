/// Centralized data path definitions shared between frontend client and backend server.
/// All functions return **relative** paths (no leading slash).
/// - The WASM client prefixes with `/data/`
/// - The server joins with its `data_root` directory

/// Format chunk filename using standard pattern
fn chunk_filename(index: u32) -> String {
    format!("chunk_{:04}.bin.xz", index)
}

// ─── Java: Meta ─────────────────────────────────────────────

pub fn java_meta_map_bin() -> &'static str {
    "java/meta/map.bin"
}

pub fn java_meta_map_json() -> &'static str {
    "java/meta/map.json"
}

// ─── Java: Games ────────────────────────────────────────────

pub fn java_game_bin(game_id: &str) -> String {
    format!("java/games/{game_id}.bin")
}

// ─── Java: Leaderboards ─────────────────────────────────────

pub fn java_leaderboard_chunk_bin(board: &str, game: &str, stat: &str, chunk: u32) -> String {
    // internal `latest` folder and `.xz` extension
    let filename = chunk_filename(chunk);
    format!("java/leaderboards/{board}/{game}/{stat}/latest/{filename}")
}

pub fn java_leaderboard_chunk_csv(board: &str, game: &str, stat: &str, chunk: u32) -> String {
    format!("java/leaderboards/{board}/{game}/{stat}/chunk_{chunk:04}.csv")
}

// ─── Java: History Leaderboards ─────────────────────────────────

pub fn java_history_leaderboard_chunk_bin(
    board: &str,
    game: &str,
    stat: &str,
    snapshot_id: &str,
    chunk: u32,
) -> String {
    let filename = chunk_filename(chunk);
    format!("java/leaderboards/{board}/{game}/{stat}/history/{snapshot_id}/{filename}")
}

pub fn java_history_snapshots_meta(board: &str, game: &str, stat: &str) -> String {
    format!("java/leaderboards/{board}/{game}/{stat}/history/_snapshots.json")
}

// ─── Java: Players ──────────────────────────────────────────

pub fn java_player_shard_bin(shard: &str) -> String {
    format!("java/players/{shard}.bin")
}

pub fn java_player_shard_json(shard: &str) -> String {
    format!("java/players/{shard}.json")
}

// ─── Java: Name Lookup ──────────────────────────────────────

pub fn java_names_index_bin(prefix: &str) -> String {
    format!("java/names_index/{prefix}.bin")
}

pub fn java_name_lookup_csv(prefix: &str, name: &str) -> String {
    format!("java/names/{prefix}/{name}.csv")
}

// ─── Bedrock: Meta ──────────────────────────────────────────

pub fn bedrock_meta_bin() -> &'static str {
    "bedrock/meta/meta.bin"
}

// ─── Bedrock: Games ─────────────────────────────────────────

pub fn bedrock_game_bin(game_id: &str) -> String {
    format!("bedrock/games/{game_id}.bin")
}

// ─── Bedrock: Leaderboards ──────────────────────────────────

pub fn bedrock_leaderboard_chunk_bin(board: &str, game: &str, stat: &str, chunk: u32) -> String {
    format!("bedrock/leaderboards/{board}/{game}/{stat}/chunk_{chunk:04}.bin")
}

pub fn bedrock_leaderboard_chunk_csv(board: &str, game: &str, stat: &str, chunk: u32) -> String {
    format!("bedrock/leaderboards/{board}/{game}/{stat}/chunk_{chunk:04}.csv")
}

// ─── Bedrock: Players ───────────────────────────────────────

pub fn bedrock_player_bin(prefix: &str, name: &str) -> String {
    format!("bedrock/players/{prefix}/{name}.bin")
}

pub fn bedrock_player_json(prefix: &str, name: &str) -> String {
    format!("bedrock/players/{prefix}/{name}.json")
}
