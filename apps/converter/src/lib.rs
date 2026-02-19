pub mod io;
pub mod pipeline;
pub mod models;

use anyhow::Result;
use mp_stats_core::models::IdMap;
use std::path::{Path, PathBuf};

pub use io::{
    copy_dir_all, finalize_output, read_json, setup_staging_directory, validate_different_paths,
    validate_directory,
};
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

        // Setup Java directories
        let java_in = self.input_dir.join("java");
        let java_out = self.staging_dir.join("java");
        std::fs::create_dir_all(&java_out)?;

        // Step 1: Process Metadata & Build ID Maps
        println!("Step 1: Processing Metadata...");
        let id_map = self.process_metadata(&java_in, &java_out)?;

        // Step 2: Dictionary & Names Index
        println!("Step 2: Processing Dictionary & Names...");
        let lookup_map = process_dictionary_and_names(&java_in, &java_out)?;

        // Step 3: Process Java Leaderboards
        println!("Step 3: Processing Leaderboards...");
        process_java_leaderboards(&java_in, &java_out, &id_map, &lookup_map)?;

        // Step 3b: Process Game Metadata
        println!("Step 3b: Processing Game Metadata...");
        process_game_metadata(&java_in, &java_out, &id_map)?;

        // Step 3c: Process Java Players
        println!("Step 3c: Processing Java Players...");
        process_java_players(&java_in, &java_out, &lookup_map)?;

        // Step 4: Bedrock (Copy optimized)
        println!("Step 4: Processing Bedrock (Copying)...");
        self.process_bedrock()?;

        // Step 5: Finalize
        println!("Step 5: Finalizing Output...");
        finalize_output(&self.staging_dir, &self.output_dir)?;

        println!("Conversion Complete!");
        Ok(())
    }

    fn process_metadata(&self, java_in: &Path, java_out: &Path) -> Result<IdMap> {
        let map_path = java_in.join("meta/map.json");
        if !map_path.exists() {
            anyhow::bail!("map.json not found at {:?}", map_path);
        }

        let id_map: IdMap = read_json(&map_path)?;

        // Serialize map to bin (LZMA)
        let map_out = java_out.join("meta/map.bin");
        if let Some(p) = map_out.parent() {
            std::fs::create_dir_all(p)?;
        }
        mp_stats_common::compression::write_lzma_bin(&map_out, &id_map)?;

        Ok(id_map)
    }

    fn process_bedrock(&self) -> Result<()> {
        let bedrock_in = self.input_dir.join("bedrock");
        let bedrock_out = self.staging_dir.join("bedrock");

        if !bedrock_in.exists() {
            return Ok(());
        }

        copy_dir_all(&bedrock_in, &bedrock_out)?;
        Ok(())
    }
}
