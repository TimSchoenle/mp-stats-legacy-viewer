pub mod io;
pub mod models;
pub mod pipeline;

use anyhow::Result;
use mp_stats_core::models::{IdMap, PlatformEdition};
use std::path::{Path, PathBuf};

pub use io::{
    copy_dir_all, finalize_output, read_json, setup_staging_directory, validate_different_paths,
    validate_directory,
};
use mp_stats_core::routes;
pub use pipeline::{
    process_dictionary_and_names, process_game_metadata, process_java_leaderboards,
    process_java_players,
};

/// Main conversion orchestrator
pub struct Converter {
    pub input_dir: PathBuf,
    pub output_dir: PathBuf,
    pub staging_dir: PathBuf,
}

impl Converter {
    pub fn new(input_dir: PathBuf, output_dir: PathBuf) -> Result<Self> {
        validate_directory(&input_dir, "Input")?;
        validate_different_paths(&input_dir, &output_dir)?;

        let staging_dir = PathBuf::from("target/converter_staging");

        Ok(Self {
            input_dir,
            output_dir,
            staging_dir,
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

            // Step 1: Process Metadata & Build ID Maps
            println!("Step 1: Processing Metadata...");
            self.process_metadata(edition, &directory_in, &self.staging_dir)?;

            // Step 2: Dictionary & Names Index
            println!("Step 2: Processing Dictionary & Names...");
            let lookup_map =
                process_dictionary_and_names(edition, &directory_in, &self.staging_dir)?;

            // Step 3: Process Leaderboards
            println!("Step 3: Processing Leaderboards...");
            process_java_leaderboards(edition, &directory_in, &self.staging_dir, &lookup_map)?;

            // Step 3b: Process Game Metadata
            println!("Step 3b: Processing Game Metadata...");
            process_game_metadata(edition, &directory_in, &self.staging_dir)?;

            // Step 3c: Process Java Players
            println!("Step 3c: Processing Players...");
            process_java_players(edition, &directory_in, &self.staging_dir, &lookup_map)?;
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
        let relative_path = routes::meta_map_bin(platform);
        let map_out = output_dir.join(relative_path);
        mp_stats_common::compression::write_lzma_bin(&map_out, &id_map)?;

        Ok(id_map)
    }
}
