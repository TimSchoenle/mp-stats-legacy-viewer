//! The numbers the on-disk layout is built out of.

/// Constants describing the raw dumps and the pages cut out of them.
pub mod raw {
    /// Entries the converter puts on one leaderboard page.
    ///
    /// The dumps page their own chunks at ten thousand. Re-paging to a thousand is what makes one
    /// page of the site one file, so paging forward costs a fetch rather than a tenth of one
    /// already in hand.
    pub const ENTRIES_PER_PAGE: usize = 1000;

    /// [`ENTRIES_PER_PAGE`] as a float, for the page arithmetic in the client.
    pub const ENTRIES_PER_PAGE_F64: f64 = 1000.0;

    /// The shape a page file's name takes, for reference. `format!` needs a literal, so the
    /// spelling that is actually used is in `mp_stats_core::routes`.
    pub const CHUNK_FILENAME_PATTERN: &str = "chunk_{:04}.bin.xz";

    /// Characters of a UUID that name its profile shard, so at most 4096 shards per edition.
    pub const MIN_PREFIX_LENGTH: usize = 3;

    /// Characters of a name that name its search index shard, and therefore the shortest query
    /// the search can answer at all.
    pub const MIN_NAME_LENGTH: usize = 3;

    /// Player ids per dictionary file in the dumps.
    ///
    /// The dumps chose it, not this repository. The conversion reads every dictionary file it
    /// finds regardless, so nothing downstream depends on the value being right.
    pub const DICTIONARY_CHUNK_SIZE: i32 = 10000;
}

/// The name the dumps give the JSON beside each snapshot.
pub const FILE_META: &str = "_meta.json";
