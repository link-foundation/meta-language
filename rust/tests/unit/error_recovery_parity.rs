use std::fs;
use std::path::PathBuf;

use meta_language::{LinkNetwork, ParseConfiguration};
use serde_json::Value;

use crate::cst_sexpression::{normalize, render_network};

#[test]
fn malformed_input_recovers_to_the_same_tree_as_the_native_tree_sitter_runtime() {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../parity/fixtures/issue-195-conformance/error-recovery.json");
    let recovery: Value =
        serde_json::from_str(&fs::read_to_string(path).expect("recovery fixture")).expect("JSON");
    for fixture in recovery["cases"].as_array().expect("cases") {
        let id = fixture["id"].as_str().expect("id");
        let source = fixture["source"].as_str().expect("source");
        let language = fixture["language"].as_str().expect("language");
        let network = LinkNetwork::parse(source, language, ParseConfiguration::default());
        assert_eq!(
            normalize(&render_network(&network, language)),
            fixture["expected"].as_str().expect("expected"),
            "{id}"
        );
        assert_eq!(network.reconstruct_text(), source, "{id}");
        assert!(!network.verify_full_match(None).is_clean(), "{id}");
    }
}
