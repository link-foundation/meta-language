//! Deferred complete source contracts, unchanged inputs and native allocations.
#[path = "../../src/translation/rust_fresh_vector_call_result_rules.rs"]
mod rules;
use super::issue_195_translation_pairs::{TemporaryDirectory, run};
use meta_language::translation::{check::check_program, emit_rust::emit_rust, rust::parse_rust};
use meta_language::{
    BinaryLinksOptions, LinkNetwork, decode_link_network_binary, decode_typed_program_links,
    emit_typed_program_links, encode_link_network_binary, encode_typed_program_links,
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
            .join("../parity/fixtures/rust-fresh-vector-call-results.lino"),
    )
    .unwrap()
}
pub(super) fn assert_original_forwarded_result_case(entry: &Value) {
    let data = fixture();
    let correction = data["corrections"]
        .as_array()
        .unwrap()
        .iter()
        .find(|row| row["original"]["name"] == entry["name"])
        .unwrap();
    assert_eq!(*entry, correction["original"]);
    let program = check_program(&parse_rust(entry["source"].as_str().unwrap()).unwrap()).unwrap();
    let declaration = program
        .declaration(correction["functionName"].as_str().unwrap())
        .unwrap();
    let meta_language::translation::ir::Decl::Fn(function) = declaration else {
        panic!("actual owned function required")
    };
    assert_eq!(
        serde_json::to_value(&function.ret).unwrap(),
        correction["resultType"]
    );
    let packet = encode_typed_program_links(&program).unwrap();
    let views = [
        LinkNetwork::from_lino(&packet.network.to_lino()).unwrap(),
        decode_link_network_binary(
            &encode_link_network_binary(&packet.network, BinaryLinksOptions::default()).unwrap(),
            BinaryLinksOptions::default(),
        )
        .unwrap(),
    ];
    for network in &views {
        assert_eq!(
            decode_typed_program_links(network, packet.root).unwrap(),
            program
        );
        for target in ["JavaScript", "Rust", "Lean", "Rocq"] {
            assert_eq!(
                emit_typed_program_links(network, packet.root, target).unwrap(),
                emit_typed_program_links(&packet.network, packet.root, target).unwrap()
            );
        }
    }
}
#[test]
fn every_declared_source_contract_and_call_decision_uses_actual_generated_definitions() {
    let data = fixture();
    for row in data["declarations"].as_array().unwrap() {
        assert_eq!(
            rules::accept_rust_fresh_vector_result_declaration(
                row["ordinary"].as_bool().unwrap(),
                row["model"].as_str().unwrap(),
                row["wrappers"].as_f64().unwrap(),
                row["element"].as_str().unwrap(),
                row["copy"].as_bool().unwrap()
            ),
            row["expected"].as_bool().unwrap()
        );
    }
    for row in data["calls"].as_array().unwrap() {
        assert_eq!(
            rules::accept_rust_fresh_vector_result_call(
                row["kind"].as_str().unwrap(),
                row["model"].as_str().unwrap(),
                row["wrappers"].as_f64().unwrap(),
                row["constructed"].as_bool().unwrap(),
                row["declared"].as_bool().unwrap(),
                row["called"].as_bool().unwrap()
            ),
            row["expected"].as_bool().unwrap()
        );
    }
}
#[test]
fn complete_original_products_validate_every_forwarding_body_in_both_source_orders() {
    fixture();
    super::translation_rust_records::assert_nominal_translation_product(
        "../experiments/rust-records/fresh-vector-call-results.mjs",
        "closed fresh Rust vector result forwarding",
        || {
            meta_language::create_source_definition_program_links(
                "meta-language/source-program/rust-fresh-vector-call-result-program",
            )
            .map_err(|message| meta_language::TypedProgramLinksError { message })
        },
    );
}
#[test]
fn the_exact_original_call_tail_has_native_value_and_distinct_allocation_controls() {
    let data = fixture();
    for correction in data["corrections"].as_array().unwrap() {
        let original = &correction["original"];
        assert_original_forwarded_result_case(original);
        let program =
            check_program(&parse_rust(original["source"].as_str().unwrap()).unwrap()).unwrap();
        let emitted = emit_rust(&program).unwrap();
        let header = emitted
            .text
            .lines()
            .find(|line| line.starts_with("#![allow("))
            .unwrap();
        let source = format!(
            "{header}\n{}\n{}\nmod original{{\n{}\n}}\n#[test]fn forwarded_allocation_observations(){{for operation in [original::bad as fn()->Vec<i64>,bad as fn()->Vec<i64>]{{let first=operation();let second=operation();assert_eq!(first,vec![7i64]);assert_eq!(second,vec![7i64]);assert_ne!(first.as_ptr(),second.as_ptr());}}}}\n",
            emitted.preludes.join("\n"),
            emitted.definitions.join("\n"),
            original["source"].as_str().unwrap()
        );
        let directory = TemporaryDirectory::new("forwarded-vector-allocation");
        let input = directory.0.join("witness.rs");
        let binary = directory
            .0
            .join(format!("witness{}", std::env::consts::EXE_SUFFIX));
        fs::write(&input, source).unwrap();
        let mut command = Command::new("rustc");
        command.args(["--edition", "2024", "--test"]);
        #[cfg(windows)]
        if let Ok(linker) = std::env::var("CARGO_TARGET_X86_64_PC_WINDOWS_MSVC_LINKER") {
            command.arg("-C").arg(format!("linker={linker}"));
        }
        let compiled = run(command.arg(&input).arg("-o").arg(&binary));
        assert!(compiled.ok, "{}\n{}", compiled.stdout, compiled.stderr);
        let observed =
            run(Command::new(&binary).args(["--exact", "forwarded_allocation_observations"]));
        assert!(observed.ok, "{}\n{}", observed.stdout, observed.stderr);
        assert!(observed.stdout.contains("1 passed"));
    }
}
