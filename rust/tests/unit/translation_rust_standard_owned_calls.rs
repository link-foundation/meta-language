//! Deferred standard qualified calls, complete original products and value replay.
#[path = "../../src/translation/rust_standard_owned_call_rules.rs"]
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
            .join("../parity/fixtures/rust-standard-owned-calls.lino"),
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
fn complete_original_and_generated_products_retain_actual_standard_copies() {
    super::translation_rust_records::assert_nominal_translation_product(
        "../experiments/rust-records/standard-owned-calls.mjs",
        "qualified standard Rust owned calls",
        || {
            meta_language::create_source_definition_program_links(
                "meta-language/source-program/rust-standard-owned-call-program",
            )
            .map_err(|message| meta_language::TypedProgramLinksError { message })
        },
    );
    let fixture = fixture();
    for row in fixture["cases"].as_array().unwrap() {
        let source = format!("{}fn main(){{}}", row["source"].as_str().unwrap());
        let packet = create_rust_source_owner_program_links(&source).unwrap();
        let program = decode_typed_program_links(&packet.network, packet.root).unwrap();
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
#[test]
fn every_qualified_path_text_and_array_decision_keeps_the_actual_receiver() {
    let fixture = fixture();
    for row in fixture["paths"].as_array().unwrap() {
        assert_eq!(
            rules::read_rust_standard_owned_call_role(
                row["path"].as_str().unwrap(),
                row["ordinary"].as_bool().unwrap(),
                row["shadowed"].as_bool().unwrap()
            ),
            row["expected"].as_str().unwrap()
        );
    }
    for row in fixture["texts"].as_array().unwrap() {
        assert_eq!(
            rules::accept_rust_standard_owned_text_call(
                row["role"].as_str().unwrap(),
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
    for row in fixture["scalars"].as_array().unwrap() {
        assert_eq!(
            rules::accept_rust_standard_owned_scalar_call(
                row["role"].as_str().unwrap(),
                row["kind"].as_str().unwrap(),
                row["layers"].as_f64().unwrap(),
                row["proven"].as_bool().unwrap(),
                row["references"].as_bool().unwrap(),
                row["known"].as_bool().unwrap()
            ),
            row["expected"].as_bool().unwrap()
        );
    }
    for row in fixture["arrays"].as_array().unwrap() {
        assert_eq!(
            rules::accept_rust_standard_owned_array_call(
                row["role"].as_str().unwrap(),
                row["model"].as_str().unwrap(),
                row["layers"].as_f64().unwrap(),
                row["known"].as_bool().unwrap()
            ),
            row["expected"].as_bool().unwrap()
        );
    }
    for row in fixture["refusals"].as_array().unwrap() {
        let error =
            create_rust_source_owner_program_links(row["source"].as_str().unwrap()).unwrap_err();
        assert!(error.message.contains(row["reason"].as_str().unwrap()));
    }
}
#[test]
fn a_portable_copy_definition_reparses_from_its_actual_emitted_rust() {
    use meta_language::translation::{
        check::check_program, emit_rust::emit_rust, rust::parse_rust,
    };
    use meta_language::{
        FormalConceptDefinitionInput, encode_formal_concept_definition, encode_typed_program_links,
        prove_formal_concept_equivalence,
    };
    let fixture = fixture();
    let original =
        check_program(&parse_rust(fixture["roundTripSource"].as_str().unwrap()).unwrap()).unwrap();
    let emission = emit_rust(&original).unwrap();
    let definition = emission
        .definitions
        .iter()
        .find(|text| text.starts_with("pub fn copied("))
        .unwrap();
    assert!(definition.contains("::std::clone::Clone::clone"));
    let restored =
        check_program(&parse_rust(&format!("{definition}\nfn main(){{}}")).unwrap()).unwrap();
    let packets = [original, restored]
        .iter()
        .map(|program| encode_typed_program_links(program).unwrap())
        .collect::<Vec<_>>();
    let definitions = packets
        .iter()
        .map(|packet| {
            encode_formal_concept_definition(
                FormalConceptDefinitionInput::LinkedProgram {
                    network: &packet.network,
                    root: packet.root,
                },
                "checked typed program",
            )
            .unwrap()
        })
        .collect::<Vec<_>>();
    assert!(prove_formal_concept_equivalence(&definitions[0], &definitions[0]).is_some());
    assert!(prove_formal_concept_equivalence(&definitions[0], &definitions[1]).is_some());
}
