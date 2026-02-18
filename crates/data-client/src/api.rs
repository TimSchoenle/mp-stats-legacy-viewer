use gloo_net::http::Request;
use mp_stats_core::models::{
    BedrockLeaderboardChunk, BedrockPlayerProfile, GameLeaderboardData, IdMap,
    JavaLeaderboardChunk, JavaLeaderboardPage, JavaMeta, JavaPlayerProfile, JavaPlayerProfileDirty,
    LeaderboardEntry, NameLookup,
};
use mp_stats_core::routes;
use smol_str::SmolStr;
use std::collections::HashMap;
use std::io::Read;

// Helper to fetch and decode binary data (LZMA or Zlib -> Postcard)
async fn fetch_bin<T: serde::de::DeserializeOwned>(url: &str) -> Option<T> {
    match Request::get(url).send().await {
        Ok(resp) if resp.ok() => {
            let bytes = match resp.binary().await {
                Ok(b) => b,
                Err(e) => {
                    gloo_console::warn!(format!("Failed to get binary from {}: {}", url, e));
                    return None;
                }
            };

            let mut cursor = std::io::Cursor::new(bytes);
            let mut decompressed = Vec::new();

            // Try LZMA
            let is_lzma = lzma_rs::xz_decompress(&mut cursor, &mut decompressed).is_ok();

            if !is_lzma {
                // Reset cursor
                cursor.set_position(0);
                // Fallback Zlib
                let mut decoder = flate2::read::ZlibDecoder::new(cursor);
                decompressed.clear();
                if let Err(e) = decoder.read_to_end(&mut decompressed) {
                    gloo_console::warn!(format!("Decompression failed for {}: {}", url, e));
                    return None;
                }
            }

            match postcard::from_bytes(&decompressed) {
                Ok(data) => Some(data),
                Err(e) => {
                    gloo_console::warn!(format!(
                        "Postcard deserialization failed for {}: {}",
                        url, e
                    ));
                    None
                }
            }
        }
        _ => None,
    }
}

// Helper to fetch Raw LZMA data (Vec<u8>)
async fn fetch_raw_lzma(url: &str) -> Option<Vec<u8>> {
    match Request::get(url).send().await {
        Ok(resp) if resp.ok() => {
            let bytes = resp.binary().await.ok()?;
            let mut cursor = std::io::Cursor::new(bytes);
            let mut decompressed = Vec::new();
            if lzma_rs::xz_decompress(&mut cursor, &mut decompressed).is_ok() {
                Some(decompressed)
            } else {
                None
            }
        }
        _ => None,
    }
}

pub async fn fetch_game_leaderboards(
    game_id: &str,
) -> Result<GameLeaderboardData, gloo_net::Error> {
    let url = format!("/data/{}", routes::java_game_bin(game_id));
    if let Some(data) = fetch_bin::<GameLeaderboardData>(&url).await {
        return Ok(data);
    }
    Err(gloo_net::Error::GlooError(
        "Failed to fetch game leaderboards".to_string(),
    ))
}

pub async fn fetch_java_meta() -> Result<JavaMeta, gloo_net::Error> {
    let id_map = if let Some(map) =
        fetch_bin::<IdMap>(&format!("/data/{}", routes::java_meta_map_bin())).await
    {
        map
    } else {
        Request::get(&format!("/data/{}", routes::java_meta_map_json()))
            .send()
            .await?
            .json()
            .await?
    };

    let mut games: Vec<mp_stats_core::models::Game> = id_map
        .games
        .values()
        .map(|name| mp_stats_core::models::Game {
            id: name.clone(),
            name: name.clone(),
        })
        .collect();

    games.sort_by(|a, b| a.name.cmp(&b.name));

    Ok(JavaMeta { games })
}

pub async fn fetch_id_map() -> Result<IdMap, gloo_net::Error> {
    if let Some(map) = fetch_bin::<IdMap>(&format!("/data/{}", routes::java_meta_map_bin())).await {
        Ok(map)
    } else {
        Request::get(&format!("/data/{}", routes::java_meta_map_json()))
            .send()
            .await?
            .json()
            .await
    }
}

pub async fn fetch_java_leaderboard(
    board: &str,
    game: &str,
    stat: &str,
    chunk: u32,
) -> Result<Vec<LeaderboardEntry>, gloo_net::Error> {
    // Fetch .bin.xz (Postcard JavaLeaderboardPage)
    let bin_path = format!(
        "/data/{}",
        routes::java_leaderboard_chunk_bin(board, game, stat, chunk)
    );

    if let Some(page) = fetch_bin::<JavaLeaderboardPage>(&bin_path).await {
        // Convert columnar format (SoA) to row format (AoS)
        let entries = page
            .ranks
            .into_iter()
            .zip(page.uuids)
            .zip(page.names)
            .zip(page.scores)
            .map(|(((rank, uuid), name), score)| LeaderboardEntry {
                rank,
                uuid,
                name,
                score,
            })
            .collect();

        return Ok(entries);
    }

    Err(gloo_net::Error::GlooError(
        "Failed to fetch leaderboard".to_string(),
    ))
}

pub async fn resolve_java_names(
    uuids: &[smol_str::SmolStr],
) -> HashMap<smol_str::SmolStr, smol_str::SmolStr> {
    let mut resolved = HashMap::new();
    for uuid in uuids {
        if let Ok(profile) = fetch_java_player(uuid).await {
            if let Some(name) = profile.name {
                resolved.insert(uuid.clone(), name);
            }
        }
    }
    resolved
}

pub async fn fetch_java_player(uuid: &str) -> Result<JavaPlayerProfile, gloo_net::Error> {
    let is_valid_len = uuid.len() == 32 || uuid.len() == 36;
    let is_hex = uuid.chars().all(|c| c.is_ascii_hexdigit() || c == '-');

    if !is_valid_len || !is_hex {
        gloo_console::error!(format!("Invalid UUID format: {}", uuid));
        return Err(gloo_net::Error::GlooError("Invalid UUID format".into()));
    }

    let shard = &uuid[..3].to_uppercase();

    // java/players/SHARD.bin (LZMA Postcard)
    let bin_path = format!("/data/{}", routes::java_player_shard_bin(shard));
    if let Some(mut shard_map) = fetch_bin::<HashMap<String, JavaPlayerProfile>>(&bin_path).await {
        if let Some(mut profile) = shard_map.remove(uuid) {
            profile.uuid = uuid.into();
            return Ok(profile);
        } else {
            gloo_console::warn!(format!(
                "Player {} not found in binary shard {}",
                uuid, shard
            ));
            return Err(gloo_net::Error::GlooError(
                "Player not found in shard".into(),
            ));
        }
    }

    Err(gloo_net::Error::GlooError(
        "Failed to fetch player".to_string(),
    ))
}

pub async fn fetch_bedrock_leaderboard(
    board: &str,
    game: &str,
    stat: &str,
    chunk: u32,
) -> Result<Vec<BedrockLeaderboardChunk>, gloo_net::Error> {
    let bin_path = format!(
        "/data/{}",
        routes::bedrock_leaderboard_chunk_bin(board, game, stat, chunk)
    );
    if let Some(data) = fetch_bin::<Vec<BedrockLeaderboardChunk>>(&bin_path).await {
        return Ok(data);
    }

    let path = format!(
        "/data/{}",
        routes::bedrock_leaderboard_chunk_csv(board, game, stat, chunk)
    );
    let resp = Request::get(&path).send().await?;
    let text = resp.text().await?;

    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(text.as_bytes());

    let mut records = Vec::new();
    for result in reader.deserialize() {
        let record: BedrockLeaderboardChunk =
            result.map_err(|e| gloo_net::Error::GlooError(e.to_string()))?;
        records.push(record);
    }
    Ok(records)
}

pub async fn fetch_bedrock_player(name: &str) -> Result<BedrockPlayerProfile, gloo_net::Error> {
    let prefix = &name[..2];

    let bin_path = format!("/data/{}", routes::bedrock_player_bin(prefix, name));
    if let Some(profile) = fetch_bin::<BedrockPlayerProfile>(&bin_path).await {
        return Ok(profile);
    }

    let path = format!("/data/{}", routes::bedrock_player_json(prefix, name));
    Request::get(&path).send().await?.json().await
}

pub async fn find_player_uuid(name: &str) -> Result<Option<NameLookup>, gloo_net::Error> {
    if name.len() < 3 {
        return Ok(None);
    }
    let prefix = &name[..3].to_lowercase();

    // Fetch names_index/{prefix}.bin
    let url = format!("/data/java/names_index/{}.bin", prefix);

    if let Some(map) = fetch_bin::<HashMap<String, String>>(&url).await {
        if let Some(uuid) = map.get(name) {
            return Ok(Some(NameLookup {
                uuid: SmolStr::new(uuid),
                shard_path: SmolStr::new(&uuid[..3].to_uppercase()),
            }));
        }
    }

    Err(gloo_net::Error::GlooError(
        "Failed to fetch player".to_string(),
    ))
}

pub async fn fetch_bedrock_meta() -> Result<mp_stats_core::models::BedrockMeta, gloo_net::Error> {
    if let Some(meta) = fetch_bin::<mp_stats_core::models::BedrockMeta>(&format!(
        "/data/{}",
        routes::bedrock_meta_bin()
    ))
    .await
    {
        Ok(meta)
    } else {
        Err(gloo_net::Error::GlooError(
            "Failed to fetch bedrock meta".to_string(),
        ))
    }
}

pub async fn fetch_bedrock_game_stats(
    game_id: &str,
) -> Result<mp_stats_core::models::BedrockGameData, gloo_net::Error> {
    let url = format!("/data/{}", routes::bedrock_game_bin(game_id));
    if let Some(data) = fetch_bin::<mp_stats_core::models::BedrockGameData>(&url).await {
        Ok(data)
    } else {
        Err(gloo_net::Error::GlooError(
            "Failed to fetch bedrock game stats".to_string(),
        ))
    }
}

pub async fn fetch_history_snapshots(
    board: &str,
    game: &str,
    stat: &str,
) -> Result<Vec<mp_stats_core::HistoricalSnapshot>, gloo_net::Error> {
    let url = format!(
        "/data/{}",
        mp_stats_core::routes::java_history_snapshots_meta(board, game, stat)
    );

    match Request::get(&url).send().await {
        Ok(resp) if resp.ok() => {
            let metadata = resp
                .json::<mp_stats_core::models::HistoryMetadata>()
                .await?;
            Ok(metadata.snapshots)
        }
        Ok(resp) if resp.status() == 404 => {
            // No history available for this leaderboard
            Ok(Vec::new())
        }
        _ => Ok(Vec::new()),
    }
}

pub async fn fetch_history_leaderboard(
    board: &str,
    game: &str,
    stat: &str,
    snapshot_id: &str,
    chunk: u32,
) -> Result<Vec<mp_stats_core::models::LeaderboardEntry>, gloo_net::Error> {
    let bin_path = format!(
        "/data/{}",
        routes::java_history_leaderboard_chunk_bin(board, game, stat, snapshot_id, chunk)
    );

    if let Some(page) = fetch_bin::<mp_stats_core::models::JavaLeaderboardPage>(&bin_path).await {
        // Convert columnar format (SoA) to row format (AoS)
        let entries = page
            .ranks
            .into_iter()
            .zip(page.uuids)
            .zip(page.names)
            .zip(page.scores)
            .map(
                |(((rank, uuid), name), score)| mp_stats_core::models::LeaderboardEntry {
                    rank,
                    uuid,
                    name,
                    score,
                },
            )
            .collect();

        return Ok(entries);
    }

    Err(gloo_net::Error::GlooError(
        "Failed to fetch history leaderboard".to_string(),
    ))
}
