//! Deferred standard qualified calls, complete original products and value replay.
#[path = "../../src/translation/rust_string_addition_rules.rs"]
mod rules;
use meta_language::{
    BinaryLinksOptions, LinkNetwork, create_rust_source_owner_program_links,
    decode_link_network_binary, decode_typed_program_links, emit_typed_program_links,
    encode_link_network_binary,
};
use serde_json::Value;
fn fixture() -> Value {
    super::translation_string_array_copy_sort::require_rust_profile();
    assert!(
        cfg!(target_pointer_width = "64"),
        "uncovered source pointer width"
    );
    meta_language::project_data::read_project_data_file(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../parity/fixtures/rust-string-addition.lino"),
    )
    .unwrap()
}
fn views(packet: &meta_language::TypedProgramLinks) -> Vec<LinkNetwork> {
    vec![
        LinkNetwork::from_lino(&packet.network.to_lino()).unwrap(),
        decode_link_network_binary(
            &encode_link_network_binary(&packet.network, BinaryLinksOptions::default()).unwrap(),
            BinaryLinksOptions::default(),
        )
        .unwrap(),
    ]
}

#[test]
fn every_candidate_owner_operand_and_refusal_has_its_actual_premises() {
    let data = fixture();
    for row in data["candidates"].as_array().unwrap() {
        assert_eq!(
            rules::accept_rust_string_addition_candidate(
                row["operation"].as_str().unwrap(),
                row["ordinary"].as_bool().unwrap(),
                row["left"].as_str().unwrap(),
                row["right"].as_str().unwrap()
            ),
            row["expected"].as_bool().unwrap()
        );
    }
    for row in data["owners"].as_array().unwrap() {
        assert_eq!(
            rules::accept_rust_string_addition_owner(
                row["side"].as_str().unwrap(),
                row["kind"].as_str().unwrap(),
                row["path"].as_str().unwrap(),
                row["children"].as_f64().unwrap()
            ),
            row["expected"].as_bool().unwrap()
        );
    }
    for row in data["operands"].as_array().unwrap() {
        assert_eq!(
            rules::accept_rust_string_addition_operand(
                row["side"].as_str().unwrap(),
                row["kind"].as_str().unwrap(),
                row["model"].as_str().unwrap(),
                row["layers"].as_f64().unwrap(),
                row["proven"].as_bool().unwrap(),
                row["references"].as_bool().unwrap(),
                row["known"].as_bool().unwrap()
            ),
            row["expected"].as_bool().unwrap()
        );
    }
    for row in data["refusals"].as_array().unwrap() {
        let error =
            create_rust_source_owner_program_links(row["source"].as_str().unwrap()).unwrap_err();
        assert!(
            error.message.contains(row["reason"].as_str().unwrap()),
            "{}: {}",
            row["name"],
            error.message
        );
    }
}
#[test]
fn complete_original_and_generated_products_retain_actual_standard_copies() {
    super::translation_rust_records::assert_nominal_translation_product(
        "../experiments/rust-records/string-addition.mjs",
        "ordinary standard Rust String addition",
        || {
            meta_language::create_source_definition_program_links(
                "meta-language/source-program/rust-string-addition-program",
            )
            .map_err(|message| meta_language::TypedProgramLinksError { message })
        },
    );
    let fixture = fixture();
    for row in fixture["cases"].as_array().unwrap() {
        let source = format!("{}fn main(){{}}", row["source"].as_str().unwrap());
        let packet = create_rust_source_owner_program_links(&source).unwrap();
        let program = decode_typed_program_links(&packet.network, packet.root).unwrap();
        let options = meta_language::TypedProgramLinksEmissionOptions {
            rust_source_header_emission_model: Some(
                "retained constructed Rust string function interface 1".to_owned(),
            ),
            ..meta_language::TypedProgramLinksEmissionOptions::default()
        };
        let retained = meta_language::emit_typed_program_links_with_options(
            &packet.network,
            packet.root,
            "Rust",
            &options,
        )
        .unwrap();
        for network in views(&packet) {
            let restored = decode_typed_program_links(&network, packet.root).unwrap();
            let signatures = |program: &meta_language::translation::ir::Program| {
                program
                    .declarations
                    .iter()
                    .filter_map(|entry| match entry {
                        meta_language::translation::ir::Decl::Fn(function) => Some((
                            function.full_name.clone(),
                            function.source_signature.clone(),
                        )),
                        _ => None,
                    })
                    .collect::<Vec<_>>()
            };
            assert_eq!(signatures(&restored), signatures(&program));
            let actual = meta_language::emit_typed_program_links_with_options(
                &network,
                packet.root,
                "Rust",
                &options,
            )
            .unwrap();
            assert_eq!(actual.code, retained.code);
            assert_eq!(actual.encodings, retained.encodings);
            assert_eq!(actual.mappings, retained.mappings);
            assert_eq!(actual.assumptions, retained.assumptions);
            assert_eq!(actual.theorems, retained.theorems);
        }
        for target in ["JavaScript", "Rust", "Lean", "Rocq"] {
            let expected = emit_typed_program_links(&packet.network, packet.root, target).unwrap();
            for network in views(&packet) {
                let actual = emit_typed_program_links(&network, packet.root, target).unwrap();
                assert_eq!(actual.code, expected.code);
                assert_eq!(actual.mappings, expected.mappings);
                assert_eq!(actual.encodings, expected.encodings);
                assert_eq!(actual.assumptions, expected.assumptions);
                assert_eq!(actual.theorems, expected.theorems);
            }
        }
    }
}
