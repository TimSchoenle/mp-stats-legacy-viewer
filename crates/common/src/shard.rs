//! Which file a UUID, a name or a player id belongs in.
//!
//! Each function here is the only definition of its key: the converter writes a file under it and
//! the client fetches the file under it, so the two cannot disagree about where an entry lives.
//! For ASCII input every key is the leading three characters, cased; the functions differ only in
//! what they do with anything else.

use crate::error::{DataError, Result};
use crate::formats::raw;

/// The profile shard a UUID belongs in: its first
/// [`MIN_PREFIX_LENGTH`](raw::MIN_PREFIX_LENGTH) bytes, uppercased.
///
/// Bytes rather than characters, because that is what the tree has always been cut by; a Bedrock
/// "UUID" is a player name, and counting characters would move a non-ASCII one to another file.
/// Input whose prefix would end inside a multi-byte character has no shard and is refused rather
/// than split.
///
/// # Errors
///
/// [`DataError::Validation`] when the UUID is shorter than that, or when that many bytes end
/// inside a multi-byte character. Either leaves the player with no shard to be found in.
pub fn uuid_shard(uuid: &str) -> Result<String> {
    leading_bytes(uuid, raw::MIN_PREFIX_LENGTH, "UUID").map(str::to_uppercase)
}

/// A name or search query in the case-folded form the names index and the search compare by.
///
/// Lowercasing is per character ([`char::to_lowercase`]) rather than per string, because
/// [`str::to_lowercase`] lowercases a word-final `Σ` to `ς` and otherwise to `σ`: a query ending
/// in `Σ` would then differ from the longer name it is a prefix of. A character that lowercases to
/// several, such as `İ`, becomes each of them.
#[must_use]
pub fn normalize_name(name: &str) -> String {
    name.chars().flat_map(char::to_lowercase).collect()
}

/// The search index shard a name, or a search query, belongs in: the first
/// [`MIN_NAME_LENGTH`](raw::MIN_NAME_LENGTH) characters of its [`normalize_name`] form.
///
/// The converter files a name under this key and the search looks a query up under it, so a query
/// that is a prefix of a name always lands on that name's shard. Characters rather than bytes, so
/// a name is searchable whatever script it is written in.
///
/// # Errors
///
/// [`DataError::Validation`] when the lowercased name is shorter than that. Such a name is also
/// shorter than the shortest query the search accepts, so it is unreachable either way.
pub fn name_shard(name: &str) -> Result<String> {
    let key: String = normalize_name(name)
        .chars()
        .take(raw::MIN_NAME_LENGTH)
        .collect();
    if key.chars().count() < raw::MIN_NAME_LENGTH {
        return Err(DataError::Validation(format!(
            "Name too short for sharding: '{name}'"
        )));
    }
    Ok(key)
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
        assert_eq!(name_shard("AÉzx").unwrap(), "aéz");
        assert_eq!(name_shard("abÉ").unwrap(), "abé");
        assert_eq!(name_shard("😀😀😀😀").unwrap(), "😀😀😀");
        assert!(matches!(name_shard("Öx"), Err(DataError::Validation(_))));
        assert!(matches!(name_shard("😀😀"), Err(DataError::Validation(_))));
    }

    #[test]
    fn test_name_shard_query_keys_like_the_name_it_prefixes() {
        // `str::to_lowercase` would key the query as "αβς" and the name as "αβσ".
        assert_eq!(name_shard("ΑΒΣ").unwrap(), name_shard("ΑΒΣΔ").unwrap());
        assert_eq!(name_shard("ΑΒΣ").unwrap(), "αβσ");
        // "İ" lowercases to two characters, so two characters of input fill the key.
        assert_eq!(name_shard("İa").unwrap(), name_shard("İabc").unwrap());
    }

    #[test]
    fn test_normalize_name_matches_a_prefix_against_the_longer_name() {
        // `str::to_lowercase` gives "αβς" for the query but "αβσδ" for the name.
        assert!(normalize_name("ΑΒΣΔ").contains(&normalize_name("ΑΒΣ")));
        assert!(normalize_name("ΑΒΣΔ").starts_with(&normalize_name("ΑΒΣ")));
        assert_eq!(normalize_name("PlayerOne"), "playerone");
    }

    #[test]
    fn test_player_id_chunk() {
        assert_eq!(player_id_chunk(12345), 1);
        assert_eq!(player_id_chunk(99999), 9);
        assert_eq!(player_id_chunk(100000), 10);
    }
}
