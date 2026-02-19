/// Binary format constants and utilities
pub mod raw {
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

/// Common file names
pub const FILE_META: &str = "_meta.json";
