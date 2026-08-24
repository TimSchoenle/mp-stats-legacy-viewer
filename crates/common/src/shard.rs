//! Which file a UUID, a name or a player id belongs in.

use crate::error::{DataError, Result};
use crate::formats::raw;

/// The profile shard a UUID belongs in: its first
/// [`MIN_PREFIX_LENGTH`](raw::MIN_PREFIX_LENGTH) characters, uppercased.
///
/// # Errors
///
/// [`DataError::Validation`] when the UUID is shorter than that, which leaves the player with no
/// shard to be found in.
pub fn uuid_shard(uuid: &str) -> Result<String> {
    if uuid.len() < raw::MIN_PREFIX_LENGTH {
        return Err(DataError::Validation(format!(
            "UUID too short for sharding: '{uuid}'"
        )));
    }
    Ok(uuid[..raw::MIN_PREFIX_LENGTH].to_uppercase())
}

/// The search index shard a name belongs in: its first
/// [`MIN_NAME_LENGTH`](raw::MIN_NAME_LENGTH) characters, lowercased.
///
/// # Errors
///
/// [`DataError::Validation`] when the name is shorter than that. Such a name is also shorter than
/// the shortest query the search accepts, so it is unreachable either way.
pub fn name_shard(name: &str) -> Result<String> {
    if name.len() < raw::MIN_NAME_LENGTH {
        return Err(DataError::Validation(format!(
            "Name too short for sharding: '{name}'"
        )));
    }
    Ok(name[..raw::MIN_NAME_LENGTH].to_lowercase())
}

/// The dictionary file a player id was dumped in, by integer division against
/// [`DICTIONARY_CHUNK_SIZE`](raw::DICTIONARY_CHUNK_SIZE).
#[must_use]
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
