//! Steps 2 and 4: the player dictionary going in, and the search index coming out.

use anyhow::Result;
use mp_stats_common::compression::write_lzma_bin;
use mp_stats_common::formats::raw;
use mp_stats_core::models::PlatformEdition;
use mp_stats_core::routes;
use rayon::prelude::*;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use walkdir::WalkDir;

/// Reads every dictionary file under `java_in` into the two lookups the rest of the run needs.
///
/// The first maps a player id to their UUID and name, and is what turns the integer in a chunk or
/// a stride into a person. A player the dump gave no name to is given their UUID as one, so the
/// map has no holes. The second groups names by their lowercased three-character prefix, which is
/// the shard [`build_names_archive`] will write; a name shorter than that prefix is dropped, since
/// there is no shard for it to go in and no query short enough to find it.
///
/// Writes nothing. `platform` and `output_dir` are taken for symmetry with the other steps and
/// are unused.
///
/// # Errors
///
/// If a dictionary file cannot be opened or does not parse as the map the dumps write.
pub fn process_dictionary_and_names(
    platform: &PlatformEdition,
    java_in: &Path,
    output_dir: &Path,
) -> Result<(
    HashMap<String, (String, String)>,
    HashMap<String, Vec<(String, String)>>,
)> {
    let _ = (platform, output_dir);
    let dict_in = java_in.join("dictionary/ids");

    let walker = WalkDir::new(&dict_in).into_iter();

    // Gather all JSONs first
    let mut files = Vec::new();
    for entry in walker.filter_map(std::result::Result::ok) {
        if entry.path().extension().is_some_and(|e| e == "json") {
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
                        local_ids.insert(id, (uuid.clone(), uuid.clone()));
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

    Ok((global_id_map, names_map))
}

/// Writes one search index file per name prefix, each mapping a name to its UUID and to whether
/// that player has a profile.
///
/// The flag is the reason this runs last: it is `true` exactly for the UUIDs in `profiled_uuids`,
/// which is only known once the profile shards have been written. Without it the search would
/// suggest names that open an empty page.
///
/// Two players sharing a name leave only one entry, the last one written.
///
/// # Errors
///
/// If an index file cannot be written.
pub fn build_names_archive(
    platform: &PlatformEdition,
    output_dir: &Path,
    names_map: HashMap<String, Vec<(String, String)>>,
    profiled_uuids: &HashSet<String>,
) -> Result<()> {
    for (prefix, entries) in names_map {
        // Write Index Bin (Name -> (UUID, has_profile))
        let mut index_map: HashMap<String, (String, bool)> = HashMap::with_capacity(entries.len());

        for (name, uuid) in &entries {
            let has_profile = profiled_uuids.contains(uuid);
            index_map.insert(name.clone(), (uuid.clone(), has_profile));
        }

        // Save Index Bin (LZMA)
        let relative_path = routes::names_index_bin(platform, &prefix);
        let index_path = output_dir.join(relative_path);
        write_lzma_bin(&index_path, &index_map)?;
    }

    Ok(())
}
