/// Binary format constants and utilities
pub mod raw {
    /// Size of a leaderboard entry in bytes (i32 pid + u64 score)
    pub const ENTRY_SIZE_OLD: usize = 12;

    /// Size of a leaderboard entry (i64 pid + f64 score)
    pub const ENTRY_SIZE_NEW: usize = 16;

    /// Size of a stat stride in V13 format (7 values)
    pub const STAT_STRIDE_SIZE: usize = 7;

    /// Maximum chunk size for leaderboards (deprecated, use ENTRIES_PER_PAGE)
    pub const MAX_CHUNK_SIZE: usize = 1000;

    /// Entries per leaderboard page (1k entries per page for optimal compression)
    pub const ENTRIES_PER_PAGE: usize = 1000;

    /// Entries per page as f64 for division operations (frontend pagination)
    pub const ENTRIES_PER_PAGE_F64: f64 = 1000.0;

    /// Chunk file name pattern format string (use with format!("chunk_{:04}.bin.xz", index))
    pub const CHUNK_FILENAME_PATTERN: &str = "chunk_{:04}.bin.xz";

    /// Minimum prefix length for sharding
    pub const MIN_PREFIX_LENGTH: usize = 3;

    /// Minimum name length for indexing
    pub const MIN_NAME_LENGTH: usize = 3;

    /// Dictionary chunk size for player ID sharding (old format: 10k chunks)
    pub const DICTIONARY_CHUNK_SIZE: i32 = 10000;
}

/// File extensions
pub const EXT_JSON: &str = "json";
pub const EXT_BIN: &str = "bin";
pub const EXT_XZ: &str = "xz";
pub const EXT_TAR: &str = "tar";

/// Common file names
pub const FILE_META: &str = "_meta.json";
pub const FILE_MAP: &str = "map.bin";

/// Directory names
pub const DIR_LATEST: &str = "latest";
pub const DIR_HISTORY: &str = "history";
pub const DIR_PLAYERS: &str = "players";
pub const DIR_LEADERBOARDS: &str = "leaderboards";
pub const DIR_DICTIONARY: &str = "dictionary";
pub const DIR_NAMES_INDEX: &str = "names_index";
pub const DIR_GAMES: &str = "games";

/// Validate file extension
pub fn has_extension(path: &std::path::Path, ext: &str) -> bool {
    path.extension()
        .and_then(|e| e.to_str())
        .map(|e| e == ext)
        .unwrap_or(false)
}

/// Sanitize name for filesystem usage
pub fn sanitize_name(name: &str) -> String {
    name.chars()
        .map(|c| {
            if c.is_alphanumeric() || c == '-' || c == '_' {
                c
            } else {
                '_'
            }
        })
        .collect()
}
