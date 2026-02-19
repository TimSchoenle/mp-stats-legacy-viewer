use crate::error::{DataError, Result};
use crate::formats::raw;

/// Calculate shard key from UUID (first 3 characters, uppercase)
pub fn uuid_shard(uuid: &str) -> Result<String> {
    if uuid.len() < raw::MIN_PREFIX_LENGTH {
        return Err(DataError::Validation(format!(
            "UUID too short for sharding: '{}'",
            uuid
        )));
    }
    Ok(uuid[..raw::MIN_PREFIX_LENGTH].to_uppercase())
}

/// Calculate shard key from player name (first 3 characters, lowercase)
pub fn name_shard(name: &str) -> Result<String> {
    if name.len() < raw::MIN_NAME_LENGTH {
        return Err(DataError::Validation(format!(
            "Name too short for sharding: '{}'",
            name
        )));
    }
    Ok(name[..raw::MIN_NAME_LENGTH].to_lowercase())
}

/// Calculate dictionary chunk ID from player ID
pub fn player_id_chunk(player_id: i32) -> i32 {
    player_id / raw::DICTIONARY_CHUNK_SIZE
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uuid_shard() {
        assert_eq!(uuid_shard("abc123-456").unwrap(), "ABC");
        assert_eq!(uuid_shard("XyZ789-000").unwrap(), "XYZ");
        assert!(uuid_shard("ab").is_err());
    }

    #[test]
    fn test_name_shard() {
        assert_eq!(name_shard("Player123").unwrap(), "pla");
        assert_eq!(name_shard("TestUser").unwrap(), "tes");
        assert!(name_shard("AB").is_err());
    }

    #[test]
    fn test_player_id_chunk() {
        assert_eq!(player_id_chunk(12345), 1);
        assert_eq!(player_id_chunk(99999), 9);
        assert_eq!(player_id_chunk(100000), 10);
    }
}
