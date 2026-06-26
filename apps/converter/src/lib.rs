pub mod io;
pub mod models;
pub mod pipeline;

use anyhow::Result;
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

/// Build a process-unique staging directory name.
///
/// Combines the process id, a high-resolution timestamp and a monotonically
/// increasing counter so that no two `Converter` instances - whether in the
/// same process or across concurrently running test processes (`cargo
/// nextest`) - ever share a staging area.
fn unique_staging_name() -> String {
    static COUNTER: AtomicU64 = AtomicU64::new(0);

    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    let seq = COUNTER.fetch_add(1, Ordering::Relaxed);

    format!("converter_staging_{}_{}_{}", std::process::id(), nanos, seq)
}

/// Main conversion orchestrator
pub struct Converter {
    pub input_dir: PathBuf,
    pub output_dir: PathBuf,
    pub staging_dir: PathBuf,
    pub cache: ConversionCache,
}

impl Converter {
    pub fn new(input_dir: PathBuf, output_dir: PathBuf) -> Result<Self> {
        Self::with_cache(input_dir, output_dir, ConversionCache::from_env())
    }

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

    /// Run the full conversion pipeline
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
            build_names_archive(
                edition,
                &self.staging_dir,
                names_map,
                &profiled_uuids,
            )?;

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
            anyhow::bail!("map.json not found at {:?}", map_path);
        }

        let id_map: IdMap = read_json(&map_path)?;

        // Serialize map to bin (LZMA)
        self.write_metadata(platform, output_dir, &id_map)?;

        Ok(id_map)
    }

    /// Persist the edition's ID map to its LZMA-compressed bin file.
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
