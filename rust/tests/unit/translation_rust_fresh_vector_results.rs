//! Deferred original-source results, complete predecessor corrections and codecs.
#[path = "../../src/translation/rust_fresh_vector_result_rules.rs"]
mod result_rules;
use meta_language::translation::{check::check_program, ir::Decl, rust::parse_rust};
use meta_language::{
    BinaryLinksOptions, LinkNetwork, decode_link_network_binary, emit_typed_program_links,
    encode_link_network_binary, encode_typed_program_links,
};
use serde_json::Value;
fn fixture() -> Value {
    meta_language::project_data::read_project_data_file(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../parity/fixtures/rust-fresh-vector-results.lino"),
    )
    .unwrap()
}
pub(super) fn assert_original_array_result_case(entry: &Value) {
    super::translation_string_array_copy_sort::require_rust_profile();
    assert!(
        cfg!(target_pointer_width = "64"),
        "uncovered target pointer width; the source profile requires 64"
    );
    let fixture = fixture();
    let correction = fixture["corrections"]
        .as_array()
        .unwrap()
        .iter()
        .find(|row| row["id"] == entry["id"])
        .unwrap();
    assert_eq!(entry["source"], correction["source"]);
    assert_eq!(
        serde_json::json!({"kind":entry["kind"],"reason":entry["reason"],"span":entry["span"]}),
        correction["previousExpectation"]
    );
    let program = check_program(&parse_rust(entry["source"].as_str().unwrap()).unwrap()).unwrap();
    let Decl::Fn(function) = program
        .declarations
        .iter()
        .find(|declaration| declaration.name() == correction["functionName"].as_str().unwrap())
        .unwrap()
    else {
        panic!("the complete source function remains present")
    };
    assert_eq!(
        serde_json::to_value(&function.ret).unwrap(),
        correction["resultType"]
    );
    let cases = fixture["cases"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|row| row["legacyCorrection"]["id"] == entry["id"])
        .collect::<Vec<_>>();
    assert_eq!(cases.len(), correction["calls"].as_array().unwrap().len());
    for (index, row) in cases.iter().enumerate() {
        assert_eq!(
            row["legacyCorrection"]["call"].as_u64().unwrap(),
            u64::try_from(index).unwrap()
        );
        assert!(
            row["source"]
                .as_str()
                .unwrap()
                .starts_with(correction["source"].as_str().unwrap())
        );
    }
    let packet = encode_typed_program_links(&program).unwrap();
    let binary =
        encode_link_network_binary(&packet.network, BinaryLinksOptions::default()).unwrap();
    for target in ["JavaScript", "Rust", "Lean", "Rocq"] {
        let expected = emit_typed_program_links(&packet.network, packet.root, target).unwrap();
        for network in [
            LinkNetwork::from_lino(&packet.network.to_lino()).unwrap(),
            decode_link_network_binary(&binary, BinaryLinksOptions::default()).unwrap(),
        ] {
            let actual = emit_typed_program_links(&network, packet.root, target).unwrap();
            assert_eq!(actual.code, expected.code);
            assert_eq!(actual.mappings, expected.mappings);
            assert_eq!(actual.assumptions, expected.assumptions);
            assert_eq!(actual.encodings, expected.encodings);
            assert_eq!(actual.theorems, expected.theorems);
        }
    }
}
#[test]
fn ordinary_fresh_vector_results_keep_complete_original_rust_products() {
    super::translation_rust_records::assert_nominal_translation_product(
        "../experiments/rust-records/fresh-vector-results.mjs",
        "ordinary fresh Rust scalar vector results",
        || {
            meta_language::create_source_definition_program_links(
                "meta-language/source-program/rust-fresh-vector-result-program",
            )
            .map_err(|message| meta_language::TypedProgramLinksError { message })
        },
    );
}
#[test]
fn every_original_corrected_input_keeps_its_metadata_and_active_result_type() {
    for row in fixture()["corrections"].as_array().unwrap() {
        assert_original_array_result_case(row);
    }
}
#[test]
fn every_checked_call_decision_uses_the_actual_shipped_source_rules() {
    for row in fixture()["decisions"].as_array().unwrap() {
        assert_eq!(
            result_rules::accept_rust_known_fresh_vector_call(
                row["kind"].as_str().unwrap(),
                row["model"].as_str().unwrap(),
                row["wrappers"].as_f64().unwrap(),
                row["checked"].as_bool().unwrap()
            ),
            row["expected"].as_bool().unwrap()
        );
    }
}
