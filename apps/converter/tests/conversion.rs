use mp_stats_common::compression::read_lzma_bin;
use mp_stats_converter::{ConversionCache, Converter};
use mp_stats_core::models::{GameLeaderboardData, PlatformEdition};
use mp_stats_core::routes;
use std::path::PathBuf;
use std::sync::Mutex;

/// Both integration tests drive `Converter::convert`. Each converter now uses
/// its own process-unique staging directory, so the two tests are safe to run
/// concurrently - including under `cargo nextest`, which runs every test in a
/// separate process where a process-local lock could not serialize them anyway.
/// This guard is kept only as cheap in-process defense-in-depth.
static CONVERT_GUARD: Mutex<()> = Mutex::new(());

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

    let _guard = CONVERT_GUARD.lock().unwrap_or_else(|e| e.into_inner());

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

/// Recursively collect every file under `root` as a map of its path relative to
/// `root` (with forward slashes) to its raw bytes. Used to compare two output
/// trees for byte-for-byte equality.
fn collect_tree(root: &PathBuf) -> std::collections::BTreeMap<String, Vec<u8>> {
    let mut out = std::collections::BTreeMap::new();
    let mut stack = vec![root.clone()];
    while let Some(dir) = stack.pop() {
        let Ok(entries) = std::fs::read_dir(&dir) else {
            continue;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
            } else {
                let rel = path
                    .strip_prefix(root)
                    .unwrap_or(&path)
                    .to_string_lossy()
                    .replace('\\', "/");
                out.insert(rel, std::fs::read(&path).unwrap_or_default());
            }
        }
    }
    out
}

/// End-to-end verification that the incremental conversion cache is fully
/// working: a second run over unchanged input must take the cache-hit path and
/// produce output that is byte-for-byte identical to the first (uncached) run.
#[test]
fn cached_run_reuses_output_and_matches_uncached_run() {
    let Some(input) = data_test_dir() else {
        eprintln!("data-test fixture not found; skipping integration test");
        return;
    };

    let unique = format!(
        "{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    );
    let tmp = std::env::temp_dir();
    let cache_root = tmp.join(format!("mp_stats_cache_e2e_{unique}"));
    let output_cold = tmp.join(format!("mp_stats_out_cold_{unique}"));
    let output_warm = tmp.join(format!("mp_stats_out_warm_{unique}"));

    let _guard = CONVERT_GUARD.lock().unwrap_or_else(|e| e.into_inner());

    // First run: empty cache -> full conversion that also populates the cache.
    let cold = Converter::with_cache(
        input.clone(),
        output_cold.clone(),
        ConversionCache::new(cache_root.clone()),
    )
    .expect("cold converter setup");
    cold.convert().expect("cold conversion succeeds");

    // The cache must now hold a stored output + matching fingerprint for the
    // Java edition, proving `store` ran and the next run will hit the cache.
    let fingerprint =
        ConversionCache::fingerprint_dir(&input.join(PlatformEdition::Java.directory_name()))
            .expect("fingerprint java input");
    let probe = tmp.join(format!("mp_stats_probe_{unique}"));
    let restored = ConversionCache::new(cache_root.clone())
        .restore(PlatformEdition::Java.directory_name(), fingerprint, &probe)
        .expect("restore probe");
    assert!(
        restored,
        "expected a populated cache entry with a matching fingerprint after the first run"
    );

    // Second run: same cache root -> must reuse the cached output.
    let warm = Converter::with_cache(
        input.clone(),
        output_warm.clone(),
        ConversionCache::new(cache_root.clone()),
    )
    .expect("warm converter setup");
    warm.convert().expect("warm conversion succeeds");

    // The cached (warm) output must be byte-for-byte identical to the cold one.
    let cold_tree = collect_tree(&output_cold);
    let warm_tree = collect_tree(&output_warm);
    assert!(
        !cold_tree.is_empty(),
        "cold run produced no output files at {output_cold:?}"
    );
    assert_eq!(
        cold_tree.keys().collect::<Vec<_>>(),
        warm_tree.keys().collect::<Vec<_>>(),
        "cached run produced a different set of output files"
    );
    assert_eq!(
        cold_tree, warm_tree,
        "cached run output differs from the uncached run output"
    );

    // Cleanup best-effort.
    let _ = std::fs::remove_dir_all(&cache_root);
    let _ = std::fs::remove_dir_all(&output_cold);
    let _ = std::fs::remove_dir_all(&output_warm);
    let _ = std::fs::remove_dir_all(&probe);
}
