//! The batch job that turns a directory of server dumps into the tree the site is served out of.
//!
//! What goes in is documented in `data/README.md`; what comes out is documented at the root of
//! [`mp_stats_core`]. This is the record of the conversion between them.
//!
//! # The two record layouts it reads
//!
//! A leaderboard chunk is a raw buffer of big-endian pairs, a `u64` player id and a `u64` score,
//! with no strings and no header. Rank is not in the file at all: the dumps left it to be derived
//! from the chunk number and the offset, which puts two players who scored the same in different
//! places.
//!
//! A player shard is JSON, `{"<player id>": [...]}`, whose array is a flat run of integers read
//! seven at a time: board id, game id, stat id, save id, score, rank, timestamp. The rank in it is
//! sequential and has the same problem.
//!
//! Both are replaced by standard competition ranking, computed over the whole population rather
//! than per file, which is what makes a player's position on their profile the position they have
//! on the board. The dump's own rank column is read and discarded.
//!
//! # The order of the steps
//!
//! Every step of [`Converter::convert`] is forced into place by the one after it.
//!
//! 1. The ID map is read first and written out, because every later step resolves numeric ids
//!    against it and the run cannot start without it.
//! 2. The dictionary is read next, into a player id to UUID and name lookup. Nothing that follows
//!    can name a player without it.
//! 3. Leaderboards are converted before game metadata, because the leading entry on a game page is
//!    read back off the first page this step just wrote rather than computed a second time.
//! 4. Players are converted after game metadata, and the names index is built last of all: an
//!    index entry records whether the player has a profile, and that is only known once the
//!    profile shards exist. Without it the search would offer names that land on an empty page.
//!
//! # What a failure looks like
//!
//! A missing `meta/map.json` fails the run. An edition whose input directory is absent is skipped
//! with a line on stdout, which is the normal case for a dump set carrying only one platform.
//!
//! Everything below that is per-file and does not stop the run: a chunk that will not decompress,
//! a player id the dictionary cannot resolve, a page that will not write. Each prints to stderr
//! and the conversion carries on, so a run that exits zero can still have produced a tree with
//! holes in it. That is deliberate for a one-shot job over a decade of dumps, and it is the reason
//! the output is assembled in a staging directory and moved into place only once, at the end.

pub mod io;
pub(crate) mod models;
pub mod pipeline;

use anyhow::Result;
use mp_stats_config::ConverterConfig;
use mp_stats_core::models::{IdMap, PlatformEdition};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

pub use io::{
    ConversionCache, copy_dir_all, finalize_output, read_json, setup_staging_directory,
    validate_different_paths, validate_directory,
};
use mp_stats_core::routes;
pub use pipeline::{
    build_names_archive, process_dictionary_and_names, process_game_metadata,
    process_java_leaderboards, process_java_players,
};

/// A staging directory name no other converter can pick.
///
/// Process id, nanoseconds and a counter: the first separates `cargo nextest`, which runs every
/// test in its own process, the second separates runs, and the third separates two converters
/// built inside one of them.
fn unique_staging_name() -> String {
    static COUNTER: AtomicU64 = AtomicU64::new(0);

    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |d| d.as_nanos());
    let seq = COUNTER.fetch_add(1, Ordering::Relaxed);

    format!("converter_staging_{}_{}_{}", std::process::id(), nanos, seq)
}

/// One conversion run, over both editions.
///
/// Constructing one validates the directories; nothing is read or written until
/// [`Converter::convert`].
pub struct Converter {
    /// Directory holding the raw dumps, one subdirectory per edition.
    pub input_dir: PathBuf,
    /// Directory the converted tree is moved into, replacing whatever was there.
    pub output_dir: PathBuf,
    /// Private workspace under `target/`, unique to this converter, emptied at the start of a run
    /// and gone by the end of one.
    pub staging_dir: PathBuf,
    /// Where a previous run's output is restored from and this run's is stored, or a no-op when
    /// disabled.
    pub cache: ConversionCache,
}

impl Converter {
    /// Builds a converter from the `[converter]` block.
    ///
    /// # Errors
    ///
    /// As [`Converter::with_cache`].
    pub fn from_config(config: &ConverterConfig) -> Result<Self> {
        Self::with_cache(
            config.input_dir.clone(),
            config.output_dir.clone(),
            ConversionCache::from_config(&config.cache),
        )
    }

    /// Builds a converter over an explicit cache, which is what lets a test disable one.
    ///
    /// # Errors
    ///
    /// If `input_dir` is not an existing directory, or if it and `output_dir` resolve to the same
    /// place — a run that converted the dumps into themselves would destroy them.
    pub fn with_cache(
        input_dir: PathBuf,
        output_dir: PathBuf,
        cache: ConversionCache,
    ) -> Result<Self> {
        validate_directory(&input_dir, "Input")?;
        validate_different_paths(&input_dir, &output_dir)?;

        // Use a unique staging directory per converter instance.
        //
        // The staging area is a private, intermediate workspace that is moved
        // into `output_dir` at the end of a run. A shared, hardcoded path would
        // be clobbered whenever two conversions run at the same time. In
        // particular, `cargo nextest` executes each test in its own process, so
        // a process-local lock cannot serialize access - the integration tests
        // would race over the same staging area and produce corrupt/partial
        // output. A unique path keeps concurrent runs fully isolated while
        // staying under `target/` so the final `rename` into the output stays
        // on the same filesystem (and thus cheap) in the common case.
        let staging_dir = PathBuf::from("target").join(unique_staging_name());

        Ok(Self {
            input_dir,
            output_dir,
            staging_dir,
            cache,
        })
    }

    /// Converts both editions and replaces `output_dir` with the result.
    ///
    /// Progress goes to stdout and per-file failures to stderr; see the crate root for which of
    /// them stop the run. `output_dir` is untouched until the last step, so a run that fails part
    /// way leaves the previous output in place.
    ///
    /// # Errors
    ///
    /// If the staging directory cannot be prepared, if an edition's input is unreadable or its
    /// `meta/map.json` is missing, or if the staging tree cannot be moved into `output_dir`.
    pub fn convert(&self) -> Result<()> {
        println!("Starting data conversion...");
        println!("Input: {:?}", self.input_dir);
        println!("Output: {:?}", self.output_dir);

        // Setup staging
        setup_staging_directory(&self.staging_dir)?;

        let edition_iter = PlatformEdition::iter();
        for edition in edition_iter {
            println!("Processing {}", edition.display_name());

            // Setup directories
            let directory_in = self.input_dir.join(edition.directory_name());

            if !directory_in.exists() {
                println!(
                    "  Input directory {:?} missing, skipping {}",
                    directory_in,
                    edition.display_name()
                );
                continue;
            }

            // Incremental cache: reuse a previous run's output when the input
            // for this edition is byte-for-byte unchanged.
            let edition_key = edition.directory_name();
            let staging_edition = self.staging_dir.join(edition_key);
            let fingerprint = ConversionCache::fingerprint_dir(&directory_in)?;

            if self
                .cache
                .restore(edition_key, fingerprint, &staging_edition)?
            {
                println!(
                    "  Cache hit for {} - reusing previous output",
                    edition.display_name()
                );
                continue;
            }

            // Step 1: Process Metadata & Build ID Maps
            println!("Step 1: Processing Metadata...");
            let mut id_map = self.process_metadata(edition, &directory_in, &self.staging_dir)?;

            // Step 2: Dictionary & Names
            // Builds the player_id -> (uuid, name) lookup map and gathers the
            // raw names map. The names index is written later (Step 4) once we
            // know which players actually have a profile.
            println!("Step 2: Processing Dictionary & Names...");
            let (lookup_map, names_map) =
                process_dictionary_and_names(edition, &directory_in, &self.staging_dir)?;

            // Step 3: Process Leaderboards
            println!("Step 3: Processing Leaderboards...");
            process_java_leaderboards(edition, &directory_in, &self.staging_dir, &lookup_map)?;

            // Step 3b: Process Game Metadata
            println!("Step 3b: Processing Game Metadata...");
            let snapshot_totals =
                process_game_metadata(edition, &directory_in, &self.staging_dir, &id_map)?;

            // Enrich the edition metadata with per-game snapshot counts and
            // re-persist the map so the frontend can show total snapshots.
            for value in id_map.games.values_mut() {
                if let Some(total) = snapshot_totals.get(value.name.as_str()) {
                    value.total_snapshots = *total;
                }
            }
            self.write_metadata(edition, &self.staging_dir, &id_map)?;

            // Step 3c: Process Java Players
            println!("Step 3c: Processing Players...");
            let profiled_uuids = process_java_players(
                edition,
                &directory_in,
                &self.staging_dir,
                &id_map,
                &lookup_map,
            )?;

            // Step 4: Build Names Index (with has_profile flag)
            // Done after players so each name entry can record whether the
            // player actually has a profile, letting the frontend filter out
            // suggestions that would lead to an empty profile page.
            println!("Step 4: Building Names Index...");
            build_names_archive(edition, &self.staging_dir, names_map, &profiled_uuids)?;

            // Persist this edition's output for future incremental runs.
            if let Err(e) = self.cache.store(edition_key, fingerprint, &staging_edition) {
                eprintln!("  Failed to update conversion cache for {edition_key}: {e}");
            }
        }

        // Step 5: Finalize
        println!("Step 5: Finalizing Output...");
        finalize_output(&self.staging_dir, &self.output_dir)?;

        println!("Conversion Complete!");
        Ok(())
    }

    fn process_metadata(
        &self,
        platform: &PlatformEdition,
        java_in: &Path,
        output_dir: &Path,
    ) -> Result<IdMap> {
        let map_path = java_in.join("meta/map.json");
        if !map_path.exists() {
            anyhow::bail!("map.json not found at {map_path:?}");
        }

        let id_map: IdMap = read_json(&map_path)?;

        // Serialize map to bin (LZMA)
        self.write_metadata(platform, output_dir, &id_map)?;

        Ok(id_map)
    }

    // Written twice per edition: once as read, and once more after the per-game snapshot counts
    // are known, since they are only counted while the leaderboards are walked.
    fn write_metadata(
        &self,
        platform: &PlatformEdition,
        output_dir: &Path,
        id_map: &IdMap,
    ) -> Result<()> {
        let relative_path = routes::meta_map_bin(platform);
        let map_out = output_dir.join(relative_path);
        mp_stats_common::compression::write_lzma_bin(&map_out, id_map)?;
        Ok(())
    }
}
