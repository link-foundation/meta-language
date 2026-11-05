//! Deferred actual string borrows, all shared premises and textual source replay.
#[path = "../../src/translation/rust_standard_string_view_rules.rs"]
mod rules;
use super::issue_195_translation_pairs::{TemporaryDirectory, run};
use meta_language::translation::emit_rust::{RustEmissionOptions, emit_rust_with_options};
use meta_language::{
    BinaryLinksOptions, LinkNetwork, create_rust_source_owner_program_links,
    decode_link_network_binary, decode_typed_program_links, emit_typed_program_links,
    encode_link_network_binary,
};
use serde_json::Value;
use std::{fs, process::Command};
fn fixture() -> Value {
    super::translation_string_array_copy_sort::require_rust_profile();
    assert!(
        cfg!(target_pointer_width = "64"),
        "uncovered source pointer width"
    );
    meta_language::project_data::read_project_data_file(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../parity/fixtures/rust-standard-string-views.lino"),
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
fn emission_value(value: &meta_language::TypedProgramLinksEmission) -> Value {
    serde_json::json!({"code":value.code,"language":value.language.as_str(),"mappings":value.mappings,"assumptions":value.assumptions,"encodings":value.encodings,"theorems":value.theorems})
}
#[test]
fn complete_original_and_generated_products_keep_each_actual_nonstatic_view() {
    super::translation_rust_records::assert_nominal_translation_product(
        "../experiments/rust-records/standard-string-views.mjs",
        "immediate standard Rust string views",
        || {
            meta_language::create_source_definition_program_links(
                "meta-language/source-program/rust-standard-string-view-program",
            )
            .map_err(|message| meta_language::TypedProgramLinksError { message })
        },
    );
    let data = fixture();
    for row in data["cases"].as_array().unwrap() {
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
            for target in ["JavaScript", "Rust", "Lean", "Rocq"] {
                assert_eq!(
                    emission_value(
                        &emit_typed_program_links(&network, packet.root, target).unwrap()
                    ),
                    emission_value(
                        &emit_typed_program_links(&packet.network, packet.root, target).unwrap()
                    )
                );
            }
        }
    }
}
#[test]
fn every_path_layer_consumer_receiver_and_argument_premise_is_exercised() {
    let data = fixture();
    for row in data["paths"].as_array().unwrap() {
        assert_eq!(
            rules::accept_rust_standard_string_view_path(
                row["path"].as_str().unwrap(),
                row["ordinary"].as_bool().unwrap(),
                row["shadowed"].as_bool().unwrap()
            ),
            row["expected"].as_bool().unwrap()
        );
    }
    for row in data["layers"].as_array().unwrap() {
        assert_eq!(
            rules::accept_rust_standard_string_view_layers(row["layers"].as_f64().unwrap()),
            row["expected"].as_bool().unwrap()
        );
    }
    for row in data["consumers"].as_array().unwrap() {
        assert_eq!(
            rules::accept_rust_standard_string_view_consumer(
                row["parameter"].as_str().unwrap(),
                row["layers"].as_f64().unwrap(),
                row["knownSlice"].as_bool().unwrap(),
                row["staticLifetime"].as_bool().unwrap(),
                row["result"].as_str().unwrap(),
                row["resultLayers"].as_f64().unwrap(),
                row["knownResult"].as_bool().unwrap()
            ),
            row["expected"].as_bool().unwrap()
        );
    }
    for row in data["receivers"].as_array().unwrap() {
        assert_eq!(
            rules::accept_rust_standard_string_view_receiver(
                row["kind"].as_str().unwrap(),
                row["model"].as_str().unwrap(),
                row["layers"].as_f64().unwrap(),
                row["proven"].as_bool().unwrap(),
                row["references"].as_bool().unwrap(),
                row["known"].as_bool().unwrap(),
                row["argumentsCount"].as_f64().unwrap()
            ),
            row["expected"].as_bool().unwrap()
        );
    }
    for row in data["arguments"].as_array().unwrap() {
        assert_eq!(
            rules::accept_rust_standard_string_view_argument(
                row["sites"].as_f64().unwrap(),
                row["root"].as_bool().unwrap(),
                row["kind"].as_str().unwrap(),
                row["layers"].as_f64().unwrap(),
                row["expectedLayers"].as_f64().unwrap(),
                row["references"].as_bool().unwrap()
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
fn actual_emitted_textual_definitions_reparse_with_independent_native_signatures() {
    let data = fixture();
    let produced = run(Command::new("node")
        .arg("../experiments/rust-records/standard-string-view-reparse.mjs")
        .current_dir(env!("CARGO_MANIFEST_DIR")));
    assert!(produced.ok, "{}\n{}", produced.stdout, produced.stderr);
    let javascript = meta_language::read_structured_data_lino(&produced.stdout).unwrap();
    assert_eq!(javascript["value"], data["roundTripValue"]);
    let original =
        create_rust_source_owner_program_links(data["roundTripSource"].as_str().unwrap()).unwrap();
    let program = decode_typed_program_links(&original.network, original.root).unwrap();
    let emission = emit_rust_with_options(
        &program,
        RustEmissionOptions {
            source_header_model: data["headerModel"].as_str(),
        },
    )
    .unwrap();
    let source = format!("{}\nfn main(){{}}", emission.definitions.join("\n\n"));
    assert!(source.contains("::std::borrow::ToOwned::to_owned"));
    assert!(source.contains("::std::string::String::as_str"));
    assert_eq!(source, javascript["source"]);
    let restored = create_rust_source_owner_program_links(&source).unwrap();
    let reparsed = decode_typed_program_links(&restored.network, restored.root).unwrap();
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
    assert_eq!(signatures(&program), signatures(&reparsed));
    for network in views(&restored) {
        for target in ["JavaScript", "Rust", "Lean", "Rocq"] {
            assert_eq!(
                emission_value(&emit_typed_program_links(&network, restored.root, target).unwrap()),
                javascript["emissions"][target]
            );
        }
    }
    let selected = emit_rust_with_options(
        &reparsed,
        RustEmissionOptions {
            source_header_model: data["headerModel"].as_str(),
        },
    )
    .unwrap();
    let native_library = format!(
        "{}\n{}",
        selected.preludes.join("\n"),
        selected.definitions.join("\n")
    );
    assert_reparsed_native_products(&data, &javascript, &native_library);
}

fn assert_reparsed_native_products(data: &Value, javascript: &Value, native_library: &str) {
    let directory = TemporaryDirectory::new("standard-string-view-reparse");
    for (product, library) in [
        ("javascript", javascript["nativeLibrary"].as_str().unwrap()),
        ("native", native_library),
    ] {
        let mut code = format!(
            "{library}\nmod original {{\n{}\n}}\n",
            data["roundTripSource"].as_str().unwrap()
        );
        for prefix in ["original", "crate"] {
            for check in data["roundTripChecks"].as_array().unwrap() {
                code.push_str(&check.as_str().unwrap().replace("CASE", prefix));
                code.push('\n');
            }
        }
        code.push_str(&format!("#[test]fn actual_reparsed_observation(){{assert_eq!(original::result().to_string(),{:?});assert_eq!(crate::result().to_string(),{:?});}}",data["roundTripValue"].as_str().unwrap(),data["roundTripValue"].as_str().unwrap()));
        let input = directory.0.join(format!("{product}.rs"));
        let binary = directory
            .0
            .join(format!("{product}{}", std::env::consts::EXE_SUFFIX));
        fs::write(&input, code).unwrap();
        let mut command = Command::new("rustc");
        command.args(["--edition", "2024", "--test"]);
        #[cfg(windows)]
        if let Ok(linker) = std::env::var("CARGO_TARGET_X86_64_PC_WINDOWS_MSVC_LINKER") {
            command.arg("-C").arg(format!("linker={linker}"));
        }
        let compiled = run(command.arg(&input).arg("-o").arg(&binary));
        assert!(
            compiled.ok,
            "{product}\n{}\n{}",
            compiled.stdout, compiled.stderr
        );
        let observed = run(Command::new(&binary).args(["--exact", "actual_reparsed_observation"]));
        assert!(
            observed.ok,
            "{product}\n{}\n{}",
            observed.stdout, observed.stderr
        );
        assert!(observed.stdout.contains("1 passed"));
    }
}
