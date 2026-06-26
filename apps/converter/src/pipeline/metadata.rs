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

/// Process the dictionary and gather the raw names map.
///
/// Returns a tuple of:
/// * `lookup_map`: player_id -> (uuid, name) used by later pipeline steps.
/// * `names_map`: name prefix -> [(name, uuid)] used to build the names index.
///
/// The names index itself is intentionally *not* written here: it is built
/// later (via [`build_names_archive`]) once the set of players that actually
/// received a profile is known, so each entry can be stamped with a
/// `has_profile` flag.
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

    Ok((global_id_map, names_map))
}

/// Build names archive and index.
///
/// Each index entry maps a player name to `(uuid, has_profile)`, where
/// `has_profile` is `true` when the player's UUID is present in
/// `profiled_uuids` (i.e. an actual profile shard was produced for them).
/// The frontend uses this flag to hide search suggestions that would lead to
/// an empty "no profile data" page.
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
