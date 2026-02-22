use serde::{Deserialize, Serialize};
use smol_str::SmolStr;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Game {
    pub id: SmolStr,
    pub name: SmolStr,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct JavaMeta {
    pub games: Vec<Game>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct LeaderboardChunk {
    pub rank: u32,
    pub uuid: SmolStr,
    pub name: SmolStr,
    pub score: f64,
}

/// Structure-of-Arrays (SoA) format for leaderboard pages
/// Stores 1,000 entries per page in columnar layout for better compression
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct LeaderboardPage {
    pub ranks: Vec<u32>,
    pub uuids: Vec<SmolStr>,
    pub names: Vec<SmolStr>,
    pub scores: Vec<u64>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct LeaderboardEntry {
    pub rank: u32,
    pub uuid: SmolStr,
    pub name: SmolStr,
    pub score: u64,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct LeaderboardMeta {
    pub count: u32,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct GameLeaderboardData {
    pub game_id: SmolStr,
    pub game_name: SmolStr,
    pub stats: HashMap<SmolStr, HashMap<SmolStr, LeaderboardMeta>>,
}

use std::collections::HashMap;
use std::error::Error;
use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct IdMap {
    pub boards: HashMap<u32, SmolStr>,
    pub games: HashMap<u32, SmolStr>,
    pub stats: HashMap<u32, SmolStr>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StatRaw {
    pub board_id: u32,
    pub game_id: u32,
    pub stat_id: u32,
    pub score: u64,
    pub rank: u32,
    pub save_time: u64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct JavaPlayerProfile {
    pub uuid: SmolStr,
    pub name: Option<SmolStr>,
    pub stats: Vec<StatRaw>,
}

impl<'de> Deserialize<'de> for JavaPlayerProfile {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Strict timestamp helper: strictly expects u64
        struct StrictTimestamp(u64);
        impl<'de> Deserialize<'de> for StrictTimestamp {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct Visitor;
                impl<'de> serde::de::Visitor<'de> for Visitor {
                    type Value = StrictTimestamp;
                    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                        formatter.write_str("an integer")
                    }
                    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E> {
                        Ok(StrictTimestamp(v as u64))
                    }
                    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E> {
                        Ok(StrictTimestamp(v))
                    }
                    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E> {
                        Ok(StrictTimestamp(v as u64))
                    }
                }
                // Key fix: Use strict hint for Postcard
                deserializer.deserialize_u64(Visitor)
            }
        }

        #[derive(Deserialize)]
        struct RawProfile(
            Option<SmolStr>,
            Vec<(u32, u32, u32, u64, u32, StrictTimestamp)>,
        );

        let RawProfile(name, stats_raw) = RawProfile::deserialize(deserializer)?;

        let stats = stats_raw
            .into_iter()
            .map(|s| StatRaw {
                board_id: s.0,
                game_id: s.1,
                stat_id: s.2,
                score: s.3,
                rank: s.4,
                save_time: s.5.0,
            })
            .collect();

        Ok(JavaPlayerProfile {
            uuid: SmolStr::default(),
            name,
            stats,
        })
    }
}

// DIRTY mode: Handles legacy JSON with mixed types (String/Int/Null).
// Used ONLY for parsing raw JSON files.
#[derive(Debug, Clone)]
pub struct JavaPlayerProfileDirty(pub JavaPlayerProfile);

impl<'de> Deserialize<'de> for JavaPlayerProfileDirty {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Loose timestamp helper: allows ANY (Value)
        #[derive(Deserialize)]
        struct RawProfileDirty(
            Option<SmolStr>,
            Vec<(u32, u32, u32, u64, u32, serde_json::Value)>,
        );

        let RawProfileDirty(name, stats_raw) = RawProfileDirty::deserialize(deserializer)?;

        let stats = stats_raw
            .into_iter()
            .map(|s| {
                // Manual cleanup logic
                let time = match &s.5 {
                    serde_json::Value::Number(n) => n.as_u64().unwrap_or(0),
                    serde_json::Value::String(str) => {
                        if str == "CSV" {
                            0
                        } else {
                            str.parse().unwrap_or(0)
                        }
                    }
                    _ => 0,
                };

                StatRaw {
                    board_id: s.0,
                    game_id: s.1,
                    stat_id: s.2,
                    score: s.3,
                    rank: s.4,
                    save_time: time,
                }
            })
            .collect();

        Ok(JavaPlayerProfileDirty(JavaPlayerProfile {
            uuid: SmolStr::default(),
            name,
            stats,
        }))
    }
}

// Serialization (if needed) would match the format, but we are viewer-only mostly.
// Implementing Serialize for completeness to match the struct, but we probably won't use it to write back to this format.
impl Serialize for JavaPlayerProfile {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeTuple;
        let mut tup = serializer.serialize_tuple(2)?;
        tup.serialize_element(&self.name)?;
        let stats_vec: Vec<(u32, u32, u32, u64, u32, u64)> = self
            .stats
            .iter()
            .map(|s| {
                (
                    s.board_id,
                    s.game_id,
                    s.stat_id,
                    s.score,
                    s.rank,
                    s.save_time,
                )
            })
            .collect();
        tup.serialize_element(&stats_vec)?;
        tup.end()
    }
}

// --- Shared / Lookup Models ---

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct NameLookup {
    pub uuid: SmolStr,
    #[serde(rename = "shard")]
    pub shard_path: SmolStr,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct MetaFile {
    pub total_entries: u32,
}

// --- Historical Leaderboard Models ---

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct HistoricalSnapshot {
    pub snapshot_id: SmolStr,
    pub timestamp: u64,
    pub total_pages: u32,
    pub total_entries: u32,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct HistoryMetadata {
    pub snapshots: Vec<HistoricalSnapshot>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub enum PlatformEdition {
    Java,
    Bedrock,
}

impl Display for PlatformEdition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.directory_name())
    }
}

#[derive(Debug, Clone)]
pub struct PlatformEditionParseError {
    input: String,
}

impl Display for PlatformEditionParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "invalid platform edition '{}'; expected '{}' or '{}'",
            self.input,
            PlatformEdition::Java.directory_name(),
            PlatformEdition::Bedrock.directory_name()
        )
    }
}

impl Error for PlatformEditionParseError {}
impl FromStr for PlatformEdition {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let normalized = s.trim().to_ascii_lowercase();

        PlatformEdition::iter()
            .find(|edition| normalized == edition.directory_name().to_ascii_lowercase())
            .map(|edition| edition.clone())
            .ok_or(Box::from(PlatformEditionParseError {
                input: s.trim().to_string(),
            }))
    }
}

impl PlatformEdition {
    pub fn directory_name(&self) -> &'static str {
        match self {
            PlatformEdition::Java => "java",
            PlatformEdition::Bedrock => "bedrock",
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            PlatformEdition::Java => "Java",
            PlatformEdition::Bedrock => "Bedrock",
        }
    }

    pub const VARIANTS: [Self; 2] = [Self::Java, Self::Bedrock];

    pub fn iter() -> std::slice::Iter<'static, Self> {
        Self::VARIANTS.iter()
    }
}
