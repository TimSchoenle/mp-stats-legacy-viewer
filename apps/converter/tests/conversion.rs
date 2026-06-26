use mp_stats_common::compression::read_lzma_bin;
use mp_stats_core::models::{GameLeaderboardData, PlatformEdition};
use mp_stats_core::routes;
use mp_stats_converter::{ConversionCache, Converter};
use std::path::PathBuf;

/// Locate the workspace-level `data-test` fixture directory regardless of the
/// directory the test binary is executed from.
fn data_test_dir() -> Option<PathBuf> {
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    for candidate in [
        manifest.join("../../data-test"),
        manifest.join("../../../data-test"),
        PathBuf::from("data-test"),
    ] {
        if candidate.join("java").is_dir() {
            return Some(candidate);
        }
    }
    None
}

#[test]
fn conversion_populates_per_category_top_and_total_entries() {
    let Some(input) = data_test_dir() else {
        eprintln!("data-test fixture not found; skipping integration test");
        return;
    };

    let unique = format!(
        "mp_stats_converter_test_{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    );
    let output = std::env::temp_dir().join(unique);

    // Disable the on-disk cache so the test always exercises a full conversion.
    let converter = Converter::with_cache(input, output.clone(), ConversionCache::disabled())
        .expect("converter setup");
    converter.convert().expect("conversion succeeds");

    let game_path = output.join(routes::game_bin(&PlatformEdition::Java, "ABarbariansLife"));
    assert!(
        game_path.exists(),
        "expected game metadata file at {:?}",
        game_path
    );

    let game: GameLeaderboardData = read_lzma_bin(&game_path).expect("read game bin");

    assert!(
        game.total_entries > 0,
        "total_entries should be populated, got {}",
        game.total_entries
    );

    // At least one category/board should expose its `#1 holder`.
    let top = game
        .stats
        .values()
        .flat_map(|boards| boards.values())
        .find_map(|meta| meta.top.as_ref())
        .expect("at least one category should have a top holder");
    assert!(!top.uuid.is_empty(), "top holder must have a uuid");
    assert!(top.score > 0, "top holder score should be positive");
    assert!(!top.name.is_empty(), "top holder name should be set");

    // Cleanup best-effort.
    let _ = std::fs::remove_dir_all(&output);
}
