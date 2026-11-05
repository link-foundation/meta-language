//! Deferred actual owner facets, complete originals, codecs and meaning checks.
#[path = "../../src/translation/rust_source_owner_retention_rules.rs"]
mod owner_rules;
use meta_language::translation::ir::{Decl, FnDecl};
use meta_language::{
    BinaryLinksOptions, LinkNetwork, TypedProgramLinks, create_rust_source_owner_program_links,
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
            .join("../parity/fixtures/rust-source-owner-retention.lino"),
    )
    .unwrap()
}
fn function<'a>(program: &'a meta_language::translation::ir::Program, name: &str) -> &'a FnDecl {
    program
        .declarations
        .iter()
        .find_map(|entry| match entry {
            Decl::Fn(function) if function.full_name == name => Some(function),
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
fn every_retained_header_and_original_product_uses_actual_source_owners() {
    super::translation_rust_records::assert_nominal_translation_product(
        "../experiments/rust-records/source-owner-retention.mjs",
        "retained Rust source signature owners",
        || {
            meta_language::create_source_definition_program_links(
                "meta-language/source-program/rust-source-owner-retention-program",
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
                    &function(&program, row["functionName"].as_str().unwrap()).source_signature
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
fn actual_retention_and_function_owner_decisions_keep_their_version_scope() {
    let fixture = fixture();
    for row in fixture["frames"].as_array().unwrap() {
        assert_eq!(
            owner_rules::accept_rust_retained_function_owner(
                row["model"].as_str().unwrap(),
                row["owner"].as_str().unwrap(),
                row["type"].as_str().unwrap(),
                row["paths"].as_f64().unwrap(),
                row["children"].as_f64().unwrap(),
                row["parameters"].as_f64().unwrap(),
                row["reference"].as_bool().unwrap()
            ),
            row["expected"].as_bool().unwrap()
        );
    }
    for row in fixture["selections"].as_array().unwrap() {
        assert_eq!(
            owner_rules::select_rust_retained_source_signature_model(
                row["model"].as_str().unwrap(),
                row["retain"].as_bool().unwrap()
            ),
            row["expected"].as_str().unwrap()
        );
    }
    let row = fixture["cases"]
        .as_array()
        .unwrap()
        .iter()
        .find(|row| {
            row["functionName"] == "invoke"
                && row["signature"]["parameters"][0]["type"]["params"]
                    .as_array()
                    .unwrap()
                    .len()
                    == 1
        })
        .unwrap();
    let mut signature: meta_language::translation::source_signatures::SourceSignature =
        serde_json::from_value(row["signature"].clone()).unwrap();
    signature.model = "Rust source function signature 1".to_owned();
    let parameters = signature
        .parameters
        .iter()
        .map(|position| position.ty.clone())
        .collect::<Vec<_>>();
    let error = meta_language::translation::source_signatures::validate_source_signature(
        &signature,
        &parameters,
        &signature.result.ty,
        "Rust",
    )
    .unwrap_err();
    assert_eq!(
        error.reason,
        "an owned Rust source type has an unsupported value type"
    );
}
#[test]
fn equal_width_does_not_certify_equal_source_type_meanings() {
    use meta_language::{
        FormalConceptDefinitionInput, encode_formal_concept_definition,
        prove_formal_concept_equivalence,
    };
    let fixture = fixture();
    let packets = fixture["identities"]
        .as_array()
        .unwrap()
        .iter()
        .map(|row| create_rust_source_owner_program_links(row["source"].as_str().unwrap()).unwrap())
        .collect::<Vec<_>>();
    let programs = packets
        .iter()
        .map(|packet| decode_typed_program_links(&packet.network, packet.root).unwrap())
        .collect::<Vec<_>>();
    assert_eq!(
        function(&programs[0], "identity").params[0].ty,
        function(&programs[1], "identity").params[0].ty
    );
    assert_ne!(
        function(&programs[0], "identity").source_signature,
        function(&programs[1], "identity").source_signature
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
#[test]
fn full_source_project_and_cli_options_keep_headers_and_original_concrete_graphs() {
    use meta_language::{
        create_rust_source_project_program_links, read_rust_source_project_concrete_graphs,
        run_rust_source_project_command, structured_data_to_lino,
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
        for name in ["identity", "values.identity"] {
            assert_eq!(
                function(&program, name)
                    .source_signature
                    .as_ref()
                    .unwrap()
                    .model,
                "Rust source function signature 2"
            );
        }
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
    let inventory = structured_data_to_lino(
        &serde_json::json!({"inputs":row["inputs"],"options":row["options"]}),
    )
    .unwrap();
    for binary in [false, true] {
        let mut args = vec![
            "inventory.lino".to_owned(),
            "--entry".to_owned(),
            row["entry"].as_str().unwrap().to_owned(),
        ];
        if binary {
            args.push("--binary".to_owned());
        }
        let output = run_rust_source_project_command(&args, |path| {
            assert_eq!(path, "inventory.lino");
            Ok(inventory.clone())
        });
        assert_eq!(output.exit_code, 0, "{}", output.stderr);
        assert!(output.stderr.is_empty());
        let network = if binary {
            decode_link_network_binary(&output.stdout, BinaryLinksOptions::default()).unwrap()
        } else {
            LinkNetwork::from_lino(std::str::from_utf8(&output.stdout).unwrap()).unwrap()
        };
        assert_eq!(network.to_lino(), packet.network.to_lino());
    }
}
