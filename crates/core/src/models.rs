use serde::{Deserialize, Serialize};
use smol_str::SmolStr;
use std::collections::HashMap;
use std::error::Error;
use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Game {
    pub id: SmolStr,
    pub name: SmolStr,
    pub description: Option<SmolStr>,
    pub icon: Option<SmolStr>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct PlatformMeta {
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
    pub snapshots: Vec<HistoricalSnapshot>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct GameLeaderboardData {
    pub game_id: SmolStr,
    pub game_name: SmolStr,
    pub description: Option<SmolStr>,
    pub icon: Option<SmolStr>,
    pub stats: HashMap<SmolStr, HashMap<SmolStr, LeaderboardMeta>>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct IdMapValue {
    pub name: SmolStr,
    pub description: Option<SmolStr>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct IdMap {
    pub boards: HashMap<u32, IdMapValue>,
    pub games: HashMap<u32, IdMapValue>,
    pub stats: HashMap<u32, IdMapValue>,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct StatRaw {
    pub board_id: u32,
    pub game_id: u32,
    pub stat_id: u32,
    pub score: u64,
    pub rank: u32,
    pub save_time: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct PlayerProfile {
    pub uuid: SmolStr,
    pub name: Option<SmolStr>,
    pub stats: Vec<StatRaw>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct NameLookup {
    pub uuid: SmolStr,
    #[serde(rename = "shard")]
    pub shard_path: SmolStr,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct MetaFile {
    pub save_time: String,
    pub save_time_unix: u64,
    pub save_id: u32,
    pub total_entries: u32,
    pub total_pages: u32,
}

// --- Historical Leaderboard Models ---

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct HistoricalSnapshot {
    pub snapshot_id: SmolStr,
    pub timestamp: u64,
    pub total_pages: u32,
    pub total_entries: u32,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Hash)]
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
            .cloned()
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
