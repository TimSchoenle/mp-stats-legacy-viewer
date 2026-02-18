use anyhow::Result;
use mp_stats_common::compression::read_lzma_bin;
use mp_stats_core::models::NameLookup;
use smol_str::SmolStr;
use std::collections::HashMap;
use std::path::PathBuf;

pub struct PlayerResolver {
    data_root: PathBuf,
}

impl PlayerResolver {
    pub fn new(data_root: PathBuf) -> Self {
        Self { data_root }
    }

    pub async fn find_player_uuid(&self, name: &str) -> Result<Option<NameLookup>> {
        if name.len() < 3 {
            return Ok(None);
        }

        let prefix = &name[..3].to_lowercase();
        let path = self
            .data_root
            .join(format!("java/names_index/{}.bin", prefix));

        let map: HashMap<String, String> = match read_lzma_bin(&path) {
            Ok(m) => m,
            Err(_) => return Ok(None),
        };

        if let Some(uuid) = map.get(name) {
            return Ok(Some(NameLookup {
                uuid: SmolStr::new(uuid),
                // Shard for player profile is usually UUID[..3]
                shard_path: SmolStr::new(&uuid[..3].to_uppercase()),
            }));
        }

        Ok(None)
    }
}
