//! Deferred full field positions, strict boundaries and independent meanings.
#[path = "../../src/translation/rust_data_type_field_signature_rules.rs"]
mod field_rules;
use meta_language::translation::ir::{Decl, Program};
use meta_language::{
    BinaryLinksOptions, LinkNetwork, TypedProgramLinks, create_rust_source_owner_program_links,
    decode_link_network_binary, decode_typed_program_links, emit_typed_program_links,
    encode_link_network_binary, encode_typed_program_links,
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
            .join("../parity/fixtures/rust-data-type-field-signatures.lino"),
    )
    .unwrap()
}
fn data<'a>(program: &'a Program, name: &str) -> &'a meta_language::translation::ir::DataDecl {
    program
        .declarations
        .iter()
        .find_map(|entry| match entry {
            Decl::Data(data) if data.full_name == name => Some(data),
            _ => None,
        })
        .unwrap()
}
fn views(packet: &TypedProgramLinks) -> Vec<LinkNetwork> {
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
fn all_original_sources_owners_codecs_and_four_emissions_remain_complete() {
    super::translation_rust_records::assert_nominal_translation_product(
        "../experiments/rust-records/data-type-field-signatures.mjs",
        "Rust datatype source fields",
        || {
            meta_language::create_source_definition_program_links(
                "meta-language/source-program/rust-data-type-field-signature-program",
            )
            .map_err(|message| meta_language::TypedProgramLinksError { message })
        },
    );
    let fixture = fixture();
    for row in fixture["cases"].as_array().unwrap() {
        let packet =
            create_rust_source_owner_program_links(row["source"].as_str().unwrap()).unwrap();
        for network in std::iter::once(&packet.network).chain(views(&packet).iter()) {
            let program = decode_typed_program_links(network, packet.root).unwrap();
            assert_eq!(
                serde_json::to_value(
                    &data(&program, row["dataTypeName"].as_str().unwrap()).source_field_signatures
                )
                .unwrap(),
                row["signature"]
            );
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
fn each_framing_field_budget_and_container_decision_keeps_its_full_premise() {
    let fixture = fixture();
    for row in fixture["frames"].as_array().unwrap() {
        assert_eq!(
            field_rules::accept_rust_data_type_field_signature(
                row["model"].as_str().unwrap(),
                row["language"].as_str().unwrap(),
                row["declared"].as_str().unwrap(),
                row["actual"].as_str().unwrap(),
                row["constructors"].as_f64().unwrap(),
                row["expectedCount"].as_f64().unwrap()
            ),
            row["expected"].as_bool().unwrap()
        );
    }
    for row in fixture["fields"].as_array().unwrap() {
        assert_eq!(
            field_rules::accept_rust_constructor_field_signature(
                row["name"].as_str().unwrap(),
                row["actual"].as_str().unwrap(),
                row["fields"].as_f64().unwrap(),
                row["expectedCount"].as_f64().unwrap()
            ),
            row["expected"].as_bool().unwrap()
        );
    }
    for row in fixture["budgets"].as_array().unwrap() {
        assert_eq!(
            field_rules::accept_rust_data_type_field_signature_budget(
                row["visits"].as_f64().unwrap(),
                row["depth"].as_f64().unwrap()
            ),
            row["expected"].as_bool().unwrap()
        );
    }
    for row in fixture["containers"].as_array().unwrap() {
        assert_eq!(
            field_rules::accept_rust_data_type_field_signature_container(
                row["kind"].as_str().unwrap(),
                row["dense"].as_bool().unwrap(),
                row["unrepresented"].as_bool().unwrap()
            ),
            row["expected"].as_bool().unwrap()
        );
        assert_eq!(
            field_rules::accept_rust_data_type_field_signature_program_container(
                row["kind"].as_str().unwrap(),
                row["dense"].as_bool().unwrap(),
                row["unrepresented"].as_bool().unwrap()
            ),
            row["programExpected"].as_bool().unwrap()
        );
    }
}
#[test]
fn equal_portable_layouts_and_function_headers_keep_distinct_field_meanings() {
    use meta_language::{
        FormalConceptDefinitionInput, encode_formal_concept_definition,
        prove_formal_concept_equivalence,
    };
    let fixture = fixture();
    for row in fixture["identities"].as_array().unwrap() {
        let packets = row["sources"]
            .as_array()
            .unwrap()
            .iter()
            .map(|source| create_rust_source_owner_program_links(source.as_str().unwrap()).unwrap())
            .collect::<Vec<_>>();
        let programs = packets
            .iter()
            .map(|packet| decode_typed_program_links(&packet.network, packet.root).unwrap())
            .collect::<Vec<_>>();
        assert_eq!(
            data(&programs[0], "Value").ctors,
            data(&programs[1], "Value").ctors
        );
        let signatures = programs
            .iter()
            .map(|program| {
                program
                    .declarations
                    .iter()
                    .find_map(|entry| match entry {
                        Decl::Fn(function) if function.full_name == "identity" => {
                            Some(&function.source_signature)
                        }
                        _ => None,
                    })
                    .unwrap()
            })
            .collect::<Vec<_>>();
        assert_eq!(signatures[0], signatures[1]);
        assert_ne!(
            data(&programs[0], "Value").source_field_signatures,
            data(&programs[1], "Value").source_field_signatures
        );
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
        assert!(prove_formal_concept_equivalence(&definitions[0], &definitions[1]).is_none());
    }
}
fn change(record: &mut Value, path: &[Value], value: Value) {
    let mut target = record;
    for field in &path[..path.len() - 1] {
        target = if let Some(index) = field.as_u64() {
            &mut target[index as usize]
        } else {
            &mut target[field.as_str().unwrap()]
        };
    }
    let field = path.last().unwrap();
    if let Some(index) = field.as_u64() {
        target[index as usize] = value;
    } else {
        target[field.as_str().unwrap()] = value;
    }
}
#[test]
fn every_changed_field_contract_refuses_at_surface_and_checked_graph_boundaries() {
    use meta_language::translation::{
        check::check_program, rust::parse_rust_with_source_type_owners,
    };
    let fixture = fixture();
    let source = fixture["controlSource"].as_str().unwrap();
    for row in fixture["malformed"].as_array().unwrap() {
        let surface = parse_rust_with_source_type_owners(source).unwrap();
        let mut value = serde_json::to_value(surface).unwrap();
        let declaration = value["items"]
            .as_array_mut()
            .unwrap()
            .iter_mut()
            .find(|item| item["k"] == "data")
            .unwrap();
        change(
            &mut declaration["sourceFieldSignatures"],
            row["path"].as_array().unwrap(),
            row["value"].clone(),
        );
        let accepted = serde_json::from_value(value)
            .ok()
            .is_some_and(|surface| check_program(&surface).is_ok());
        assert!(!accepted, "{row}");
        let packet = create_rust_source_owner_program_links(source).unwrap();
        let program = decode_typed_program_links(&packet.network, packet.root).unwrap();
        let mut value = serde_json::to_value(program).unwrap();
        let declaration = value["declarations"]
            .as_array_mut()
            .unwrap()
            .iter_mut()
            .find(|item| item["k"] == "data")
            .unwrap();
        change(
            &mut declaration["sourceFieldSignatures"],
            row["path"].as_array().unwrap(),
            row["value"].clone(),
        );
        let accepted = serde_json::from_value::<Program>(value)
            .ok()
            .is_some_and(|program| encode_typed_program_links(&program).is_ok());
        assert!(!accepted, "{row}");
    }
    let mut value =
        serde_json::to_value(parse_rust_with_source_type_owners(source).unwrap()).unwrap();
    value["items"][0]["sourceFieldSignatures"] = Value::Null;
    assert!(
        serde_json::from_value::<meta_language::translation::surface::SProgram>(value).is_err()
    );
}
#[test]
fn plain_parsing_and_existing_compact_reference_splitting_remain_independent() {
    use meta_language::translation::{check::check_program, rust::parse_rust};
    let fixture = fixture();
    let plain =
        check_program(&parse_rust(fixture["controlSource"].as_str().unwrap()).unwrap()).unwrap();
    assert!(data(&plain, "Value").source_field_signatures.is_none());
    let programs = fixture["compact"]["sources"]
        .as_array()
        .unwrap()
        .iter()
        .map(|source| {
            let packet = create_rust_source_owner_program_links(source.as_str().unwrap()).unwrap();
            decode_typed_program_links(&packet.network, packet.root).unwrap()
        })
        .collect::<Vec<_>>();
    let signatures = programs
        .iter()
        .map(|program| {
            program
                .declarations
                .iter()
                .find_map(|entry| match entry {
                    Decl::Fn(function) if function.full_name == "length" => {
                        Some(&function.source_signature)
                    }
                    _ => None,
                })
                .unwrap()
        })
        .collect::<Vec<_>>();
    assert_eq!(signatures[0], signatures[1]);
}
#[test]
fn complete_source_projects_keep_field_owners_and_each_original_concrete_graph() {
    use meta_language::{
        create_rust_source_project_program_links, read_rust_source_project_concrete_graphs,
    };
    let fixture = fixture();
    let row = &fixture["project"];
    let packet = create_rust_source_project_program_links(
        &row["inputs"],
        row["entry"].as_str().unwrap(),
        &row["options"],
    )
    .unwrap();
    for network in std::iter::once(&packet.network).chain(views(&packet).iter()) {
        let program = decode_typed_program_links(network, packet.root).unwrap();
        let signature = data(&program, "values.Value")
            .source_field_signatures
            .as_ref()
            .unwrap();
        assert_eq!(signature.data_type_name, "values.Value");
        assert_eq!(signature.ctors[0].fields[0].owner.path[0], "usize");
        let concrete =
            read_rust_source_project_concrete_graphs(network, packet.root, &serde_json::json!({}))
                .unwrap();
        assert_eq!(
            serde_json::to_value(
                concrete
                    .iter()
                    .map(|source| serde_json::json!({"path":source.path,"source":source.source}))
                    .collect::<Vec<_>>()
            )
            .unwrap(),
            row["inputs"]
        );
    }
}
