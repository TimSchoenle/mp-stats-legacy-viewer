use anyhow::Result;
use mp_stats_common::compression::write_lzma_bin;
use mp_stats_common::formats::raw;
use rayon::prelude::*;
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::BufReader;
use std::path::Path;
use walkdir::WalkDir;

/// Process dictionary and generate names index
/// Returns a map of player_id -> (uuid, name) for lookups
pub fn process_dictionary_and_names(
    java_in: &Path,
    java_out: &Path,
) -> Result<HashMap<String, (String, String)>> {
    let dict_in = java_in.join("dictionary/ids");

    let walker = WalkDir::new(&dict_in).into_iter();

    // Gather all JSONs first
    let mut files = Vec::new();
    for entry in walker.filter_map(|e| e.ok()) {
        if entry.path().extension().map_or(false, |e| e == "json") {
            files.push(entry.path().to_path_buf());
        }
    }

    // Parallel Process Dictionary
    println!("Processing {} dictionary files in parallel...", files.len());

    let (names_map, global_id_map) = files
        .par_iter()
        .map(
            |path| -> Result<(
                HashMap<String, Vec<(String, String)>>,
                HashMap<String, (String, String)>,
            )> {
                // Read Dict
                let file = File::open(path)?;
                let map: HashMap<String, (String, Option<String>)> =
                    serde_json::from_reader(BufReader::new(file))?;

                let mut local_names: HashMap<String, Vec<(String, String)>> = HashMap::new();
                let mut local_ids = HashMap::new();

                // Collect Names & IDs
                for (id, (uuid, name_opt)) in map {
                    if let Some(name) = name_opt {
                        if name.len() >= raw::MIN_NAME_LENGTH {
                            let prefix = name[..raw::MIN_NAME_LENGTH].to_lowercase(); // Normalized prefix
                            local_names
                                .entry(prefix)
                                .or_default()
                                .push((name.clone(), uuid.clone()));
                        }
                        local_ids.insert(id, (uuid, name));
                    } else {
                        local_ids.insert(id, (uuid.clone(), uuid.to_string()));
                    }
                }
                Ok((local_names, local_ids))
            },
        )
        .reduce(
            || Ok((HashMap::new(), HashMap::new())),
            |acc, item| {
                let (mut acc_names, mut acc_ids) = acc?;
                let (item_names, item_ids) = item?;

                // Merge Names
                for (k, v) in item_names {
                    acc_names.entry(k).or_default().extend(v);
                }
                // Merge IDs
                acc_ids.extend(item_ids);

                Ok((acc_names, acc_ids))
            },
        )?;

    println!("Found {} names.", names_map.len());

    build_names_archive(java_out, names_map)?;

    Ok(global_id_map)
}

/// Build names archive and index
fn build_names_archive(
    java_out: &Path,
    names_map: HashMap<String, Vec<(String, String)>>,
) -> Result<()> {
    let index_out = java_out.join("names_index");
    fs::create_dir_all(&index_out)?;

    for (prefix, entries) in names_map {
        // Write Index Bin (Name -> UUID)
        let mut index_map: HashMap<String, String> = HashMap::with_capacity(entries.len());

        for (name, uuid) in &entries {
            index_map.insert(name.clone(), uuid.clone());
        }

        // Save Index Bin (LZMA)
        let index_path = index_out.join(format!("{}.bin", prefix));
        write_lzma_bin(&index_path, &index_map)?;
    }

    Ok(())
}
