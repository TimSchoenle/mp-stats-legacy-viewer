//! Which file a UUID, a name or a player id belongs in.
//!
//! A shard is named by a fixed number of leading *bytes*, not characters. For the ASCII that UUIDs
//! and player names are written in the two are the same; for anything else, counting bytes is what
//! the tree has always been cut by, and counting characters instead would move existing entries
//! to a different file. Input whose prefix would end inside a multi-byte character has no shard
//! and is refused rather than split.

use crate::error::{DataError, Result};
use crate::formats::raw;

/// The profile shard a UUID belongs in: its first
/// [`MIN_PREFIX_LENGTH`](raw::MIN_PREFIX_LENGTH) bytes, uppercased.
///
/// # Errors
///
/// [`DataError::Validation`] when the UUID is shorter than that, or when that many bytes end
/// inside a multi-byte character. Either leaves the player with no shard to be found in.
pub fn uuid_shard(uuid: &str) -> Result<String> {
    leading_bytes(uuid, raw::MIN_PREFIX_LENGTH, "UUID").map(str::to_uppercase)
}

/// The search index shard a name belongs in: its first
/// [`MIN_NAME_LENGTH`](raw::MIN_NAME_LENGTH) bytes, lowercased.
///
/// # Errors
///
/// [`DataError::Validation`] when the name is shorter than that, or when that many bytes end
/// inside a multi-byte character. A name shorter than the prefix is also shorter than the
/// shortest query the search accepts, so it is unreachable either way.
pub fn name_shard(name: &str) -> Result<String> {
    leading_bytes(name, raw::MIN_NAME_LENGTH, "Name").map(str::to_lowercase)
}

/// The dictionary file a player id was dumped in, by integer division against
/// [`DICTIONARY_CHUNK_SIZE`](raw::DICTIONARY_CHUNK_SIZE).
#[must_use]
pub fn player_id_chunk(player_id: i32) -> i32 {
    player_id / raw::DICTIONARY_CHUNK_SIZE
}

/// The first `len` bytes of `value`, provided they end on a character boundary.
fn leading_bytes<'a>(value: &'a str, len: usize, kind: &str) -> Result<&'a str> {
    if value.len() < len {
        return Err(DataError::Validation(format!(
            "{kind} too short for sharding: '{value}'"
        )));
    }
    value.get(..len).ok_or_else(|| {
        DataError::Validation(format!(
            "{kind} prefix for sharding splits a multi-byte character: '{value}'"
        ))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uuid_shard() {
        assert_eq!(uuid_shard("abc123-456").unwrap(), "ABC");
        assert_eq!(uuid_shard("XyZ789-000").unwrap(), "XYZ");
        assert_eq!(uuid_shard("abc").unwrap(), "ABC");
    }

    #[test]
    fn test_uuid_shard_rejects_short_and_empty() {
        assert!(matches!(uuid_shard("ab"), Err(DataError::Validation(_))));
        assert!(matches!(uuid_shard(""), Err(DataError::Validation(_))));
    }

    #[test]
    fn test_uuid_shard_non_ascii() {
        // "é" is two bytes: three bytes end after it in "aéz" and inside it in "abé".
        assert_eq!(uuid_shard("aéz").unwrap(), "AÉ");
        assert!(matches!(uuid_shard("abé"), Err(DataError::Validation(_))));
        // One character, two bytes: too short.
        assert!(matches!(uuid_shard("é"), Err(DataError::Validation(_))));
        // One character, three bytes: exactly the prefix.
        assert_eq!(uuid_shard("€x").unwrap(), "€");
        // One character, four bytes: long enough, but has no three-byte prefix.
        assert!(matches!(uuid_shard("😀"), Err(DataError::Validation(_))));
    }

    #[test]
    fn test_name_shard() {
        assert_eq!(name_shard("Player123").unwrap(), "pla");
        assert_eq!(name_shard("TestUser").unwrap(), "tes");
        assert_eq!(name_shard("ABC").unwrap(), "abc");
    }

    #[test]
    fn test_name_shard_rejects_short_and_empty() {
        assert!(matches!(name_shard("AB"), Err(DataError::Validation(_))));
        assert!(matches!(name_shard(""), Err(DataError::Validation(_))));
    }

    #[test]
    fn test_name_shard_non_ascii() {
        assert_eq!(name_shard("AÉz").unwrap(), "aé");
        assert!(matches!(name_shard("abÉ"), Err(DataError::Validation(_))));
        assert!(matches!(name_shard("Ö"), Err(DataError::Validation(_))));
        assert!(matches!(name_shard("😀😀"), Err(DataError::Validation(_))));
    }

    #[test]
    fn test_player_id_chunk() {
        assert_eq!(player_id_chunk(12345), 1);
        assert_eq!(player_id_chunk(99999), 9);
        assert_eq!(player_id_chunk(100000), 10);
    }
}
