//! Every path into the converted tree, as a function rather than a string.
//!
//! The two halves of the platform reach the same file from different sides: the WASM client
//! prefixes `/data/`, the server joins its `data_dir`. So the paths here are relative and carry
//! no leading slash, and the root each caller puts in front is that caller's business.
//!
//! Nothing validates that the file exists. A path to a board, snapshot or shard that was never
//! written is a well-formed path, and the 404 is the caller's to handle.

use crate::models::PlatformEdition;

/// The name of page `index`, zero-padded to four digits.
fn chunk_filename(index: u32) -> String {
    format!("chunk_{index:04}.bin.xz")
}

/// Path to `edition`'s ID map, which is what every numeric board, game and stat id in a profile
/// resolves through.
#[must_use]
pub fn meta_map_bin(edition: &PlatformEdition) -> String {
    format!("{}/meta/map.bin.xz", edition.directory_name())
}

/// Path to one game's aggregated metadata: its boards, their snapshots and their leading entries.
#[must_use]
pub fn game_bin(edition: &PlatformEdition, game_id: &str) -> String {
    format!("{}/games/{game_id}.bin.xz", edition.directory_name())
}

/// Path to one page of a board's current leaderboard. `chunk` is zero-based, so the site's page 1
/// is `chunk_0000`.
#[must_use]
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

/// Path to one page of an archived snapshot of the same board, numbered the same way.
///
/// `snapshot_id` is the name the dump's history archive gave the snapshot directory, which
/// reaches the client through [`crate::models::HistoricalSnapshot::snapshot_id`].
#[must_use]
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

/// Path to the profile shard holding every player whose UUID starts with `shard`, which is that
/// prefix uppercased.
#[must_use]
pub fn player_shard_bin(edition: &PlatformEdition, shard: &str) -> String {
    format!("{}/players/{shard}.bin.xz", edition.directory_name())
}

/// Path to the names index for `prefix`, which is a three-character name prefix lowercased.
#[must_use]
pub fn names_index_bin(edition: &PlatformEdition, prefix: &str) -> String {
    format!("{}/names_index/{prefix}.bin.xz", edition.directory_name())
}
