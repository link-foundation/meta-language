//! Bidirectional enforcement of the shared Rust <-> JavaScript parity manifest.
//!
//! `parity/language-features.json` is the single source of truth for the
//! feature set both language implementations must keep in sync. The JavaScript
//! side validates it with `js/scripts/check-js-rust-parity.mjs`; this test gives
//! the Rust side an equivalent guard so the Rust test suite itself fails when the
//! manifest and the crate's public API drift apart. Together they implement the
//! issue #163 rule: a feature added to one language forces the other to follow.

use std::fs;
use std::path::{Path, PathBuf};

use meta_language::API_OPERATIONS;
use serde_json::Value;

/// Walk up from the crate manifest to the repository root that owns the shared
/// `parity/` directory. The Rust crate lives under `rust/` in the multi-language
/// layout, while `parity/` stays at the repository root.
fn repository_root() -> PathBuf {
    let mut dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    loop {
        if dir.join("parity/language-features.json").is_file() {
            return dir;
        }
        assert!(
            dir.pop(),
            "could not locate parity/language-features.json from the crate manifest"
        );
    }
}

fn manifest() -> (PathBuf, Value) {
    let root = repository_root();
    let path = root.join("parity/language-features.json");
    let text = fs::read_to_string(&path).expect("parity manifest should be readable");
    let value: Value = serde_json::from_str(&text).expect("parity manifest should be valid JSON");
    (root, value)
}

#[test]
fn operation_families_match_the_rust_api_operations_registry() {
    let (_root, manifest) = manifest();

    let mut families: Vec<String> = manifest["operationFamilies"]
        .as_array()
        .expect("operationFamilies must be an array")
        .iter()
        .map(|entry| {
            entry
                .as_str()
                .expect("operation family must be a string")
                .to_string()
        })
        .collect();
    families.sort();

    let mut operations: Vec<String> = API_OPERATIONS
        .iter()
        .map(|entry| entry.name().to_string())
        .collect();
    operations.sort();

    assert_eq!(
        families, operations,
        "manifest operationFamilies must list exactly the Rust API_OPERATIONS names"
    );
}

#[test]
fn every_feature_row_declares_both_languages_with_existing_rust_evidence() {
    let (root, manifest) = manifest();

    let features = manifest["features"]
        .as_array()
        .expect("features must be an array");
    assert!(!features.is_empty(), "manifest must list feature rows");

    for feature in features {
        let id = feature["id"]
            .as_str()
            .expect("each feature must declare a string id");
        let required = feature["required"].as_bool().unwrap_or(false);

        for language in ["rust", "javascript"] {
            let cell = feature
                .get(language)
                .unwrap_or_else(|| panic!("{id} is missing the {language} cell"));

            if required {
                assert_eq!(
                    cell["status"].as_str(),
                    Some("implemented"),
                    "{id} {language} status must be implemented because the feature is required"
                );
            }

            let evidence = cell["evidence"]
                .as_array()
                .unwrap_or_else(|| panic!("{id} {language} cell must include an evidence array"));
            assert!(
                !evidence.is_empty(),
                "{id} {language} cell must cite at least one evidence path"
            );

            // The Rust test owns checking that the Rust evidence physically
            // exists; the JavaScript checker owns the JavaScript evidence. Each
            // language thus guards its own half of every manifest row.
            if language == "rust" {
                for path in evidence {
                    let relative = path
                        .as_str()
                        .unwrap_or_else(|| panic!("{id} rust evidence entries must be strings"));
                    assert!(
                        root.join(relative).exists(),
                        "{id} rust evidence does not exist: {relative}"
                    );
                }
            }
        }
    }
}

#[test]
fn rust_evidence_paths_live_inside_the_rust_or_shared_trees() {
    // Guard against regressions where a manifest evidence path silently drops
    // the `rust/` prefix after the crate moved into the language folder.
    let (_root, manifest) = manifest();

    for feature in manifest["features"].as_array().expect("features array") {
        let id = feature["id"].as_str().expect("feature id");
        for path in feature["rust"]["evidence"]
            .as_array()
            .expect("rust evidence array")
        {
            let relative = path.as_str().expect("rust evidence path string");
            let in_rust_tree = Path::new(relative).starts_with("rust");
            let in_shared_tree = ["docs/", "parity/"]
                .iter()
                .any(|prefix| relative.starts_with(prefix));
            assert!(
                in_rust_tree || in_shared_tree,
                "{id} rust evidence {relative} must live under rust/ or a shared docs/parity tree"
            );
        }
    }
}
