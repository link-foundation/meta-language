//! Deferred actual native decisions and independent original Rust type errors.
#[path = "../../src/translation/rust_pointer_integer_owner_rules.rs"]
mod owner_rules;
use serde_json::Value;
fn fixture() -> Value {
    meta_language::project_data::read_project_data_file(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../parity/fixtures/rust-pointer-integer-owners.lino"),
    )
    .unwrap()
}
#[test]
fn pointer_integer_owners_keep_complete_original_products() {
    super::translation_rust_records::assert_nominal_translation_product(
        "../experiments/rust-records/pointer-integer-owners.mjs",
        "distinct Rust pointer integer source owners",
        || {
            meta_language::create_source_definition_program_links(
                "meta-language/source-program/rust-pointer-integer-owner-program",
            )
            .map_err(|message| meta_language::TypedProgramLinksError { message })
        },
    );
}
#[test]
fn all_declared_contracts_use_actual_generated_rules() {
    super::translation_string_array_copy_sort::require_rust_profile();
    let fixture = fixture();
    for row in fixture["decisions"].as_array().unwrap() {
        assert_eq!(
            owner_rules::read_rust_pointer_integer_owner_error(
                row["actual"].as_str().unwrap(),
                row["expected"].as_str().unwrap()
            ),
            row["error"].as_str().unwrap()
        );
    }
    for row in fixture["joins"].as_array().unwrap() {
        let owners = row["owners"]
            .as_array()
            .unwrap()
            .iter()
            .map(|value| value.as_str().unwrap().to_owned())
            .collect::<Vec<_>>();
        assert_eq!(
            owner_rules::read_rust_pointer_integer_owner_join_error(&owners),
            row["error"].as_str().unwrap()
        );
        assert_eq!(
            owner_rules::select_rust_pointer_integer_result_owner(&owners),
            row["selected"].as_str().unwrap()
        );
    }
    for row in fixture["containers"].as_array().unwrap() {
        assert_eq!(
            owner_rules::accept_rust_pointer_integer_owner_child(
                row["kind"].as_str().unwrap(),
                row["path"].as_str().unwrap(),
                row["count"].as_f64().unwrap()
            ),
            row["expected"].as_bool().unwrap()
        );
    }
}
#[test]
fn every_original_refusal_has_an_independent_compiler_type_error() {
    use super::issue_195_translation_pairs::{TemporaryDirectory, run};
    use std::{fs, process::Command};
    super::translation_string_array_copy_sort::require_rust_profile();
    assert!(
        cfg!(target_pointer_width = "64"),
        "uncovered target pointer width"
    );
    let fixture = fixture();
    let refusals = fixture["refusals"].as_array().unwrap();
    assert!(
        !refusals.is_empty() && refusals.len() <= 64,
        "bounded complete original compiler batch"
    );
    let mut source = String::new();
    let mut intervals = Vec::new();
    for (index, row) in refusals.iter().enumerate() {
        source.push_str(&format!("\npub mod source_{index} {{\n"));
        let first = source.len();
        source.push_str(row["source"].as_str().unwrap());
        intervals.push((first, source.len()));
        source.push_str("\n}\n");
    }
    let directory = TemporaryDirectory::new("pointer-integer-original-refusals");
    let input = directory.0.join("original.rs");
    fs::write(&input, source).unwrap();
    let compiler = run(Command::new("rustc")
        .args([
            "--edition",
            "2024",
            "--crate-type",
            "lib",
            "--emit",
            "metadata",
            "--error-format",
            "json",
        ])
        .arg(&input)
        .arg("-o")
        .arg(directory.0.join("original.rmeta")));
    assert!(
        !compiler.ok,
        "the complete original invalid Rust inventory cannot compile"
    );
    let diagnostics = compiler
        .stderr
        .lines()
        .filter_map(|line| serde_json::from_str::<Value>(line).ok())
        .collect::<Vec<_>>();
    for (row, (first, last)) in refusals.iter().zip(intervals) {
        assert!(
            diagnostics
                .iter()
                .any(|diagnostic| diagnostic["level"] == "error"
                    && row["rustCompilerCodes"]
                        .as_array()
                        .unwrap()
                        .contains(&diagnostic["code"]["code"])
                    && diagnostic["spans"]
                        .as_array()
                        .is_some_and(|spans| spans.iter().any(|span| span["is_primary"] == true
                            && span["byte_start"]
                                .as_u64()
                                .is_some_and(|start| usize::try_from(start)
                                    .is_ok_and(|start| start >= first))
                            && span["byte_end"].as_u64().is_some_and(
                                |end| usize::try_from(end).is_ok_and(|end| end <= last)
                            )))),
            "original input {} has no independent primary type-error span:\n{}",
            row["name"],
            compiler.stderr
        );
    }
}
