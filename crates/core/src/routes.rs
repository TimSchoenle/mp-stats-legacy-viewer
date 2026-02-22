use crate::models::PlatformEdition;

/// Centralized data path definitions shared between frontend client and backend server.
/// All functions return **relative** paths (no leading slash).
/// - The WASM client prefixes with `/data/`
/// - The server joins with its `data_root` directory

/// Format chunk filename using standard pattern
fn chunk_filename(index: u32) -> String {
    format!("chunk_{:04}.bin.xz", index)
}

pub fn meta_map_bin(edition: &PlatformEdition) -> String {
    format!("{}/meta/map.bin.xz", edition.directory_name())
}

pub fn game_bin(edition: &PlatformEdition, game_id: &str) -> String {
    format!("{}/games/{game_id}.bin.xz", edition.directory_name())
}

pub fn leaderboard_chunk_bin(
    edition: &PlatformEdition,
    board: &str,
    game: &str,
    stat: &str,
    chunk: u32,
) -> String {
    let filename = chunk_filename(chunk);
    format!(
        "{}/leaderboards/{board}/{game}/{stat}/latest/{filename}",
        edition.directory_name()
    )
}

pub fn history_leaderboard_chunk_bin(
    edition: &PlatformEdition,
    board: &str,
    game: &str,
    stat: &str,
    snapshot_id: &str,
    chunk: u32,
) -> String {
    let filename = chunk_filename(chunk);
    format!(
        "{}/leaderboards/{board}/{game}/{stat}/history/{snapshot_id}/{filename}",
        edition.directory_name()
    )
}

pub fn player_shard_bin(edition: &PlatformEdition, shard: &str) -> String {
    format!("{}/players/{shard}.bin.xz", edition.directory_name())
}

pub fn names_index_bin(edition: &PlatformEdition, prefix: &str) -> String {
    format!("{}/names_index/{prefix}.bin.xz", edition.directory_name())
}
