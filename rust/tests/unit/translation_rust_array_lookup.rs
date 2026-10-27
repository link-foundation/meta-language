#[path = "../../src/translation/rust_array_lookup_rules.rs"]
mod rules;
use super::issue_195_translation_pairs::{TemporaryDirectory, run};
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
            .join("../parity/fixtures/rust-array-lookup.lino"),
    )
    .unwrap()
}
fn strings(value: &Value) -> Vec<String> {
    value
        .as_array()
        .unwrap()
        .iter()
        .map(|value| value.as_str().unwrap().to_owned())
        .collect()
}
#[test]
fn every_array_lookup_source_and_native_product_retains_complete_independent_observations() {
    super::callable_profile_products::assert_products("rust-array-lookup", "array-lookup");
}
#[test]
fn every_lookup_arity_owner_consumer_receiver_and_lazy_plan_remains_explicit() {
    let data = fixture();
    for row in data["arities"].as_array().unwrap() {
        assert_eq!(
            rules::read_rust_array_lookup_arity(
                row["method"].as_str().unwrap(),
                row["ordinary"].as_bool().unwrap()
            ),
            row["expected"].as_f64().unwrap()
        );
    }
    for row in data["consumers"].as_array().unwrap() {
        assert_eq!(
            rules::accept_rust_array_lookup_presence_consumer(row["consumer"].as_str().unwrap()),
            ["is_some", "is_none"].contains(&row["consumer"].as_str().unwrap())
        );
        assert_eq!(
            rules::read_rust_array_lookup_consumer_arity(
                row["consumer"].as_str().unwrap(),
                row["copied"].as_bool().unwrap()
            ),
            row["expected"].as_f64().unwrap()
        );
    }
    for row in data["receivers"].as_array().unwrap() {
        assert_eq!(
            rules::read_rust_array_lookup_receiver_error(
                row["kind"].as_str().unwrap(),
                row["element"].as_str().unwrap(),
                row["model"].as_str().unwrap(),
                row["proven"].as_bool().unwrap(),
                row["immutable"].as_bool().unwrap(),
                row["copy"].as_bool().unwrap(),
                row["copied"].as_bool().unwrap()
            ),
            row["expected"].as_str().unwrap()
        );
    }
    for row in data["elementOwners"].as_array().unwrap() {
        assert_eq!(
            rules::read_rust_array_lookup_element_owner_error(
                row["kind"].as_str().unwrap(),
                row["bits"].as_str().unwrap(),
                row["signed"].as_bool().unwrap(),
                row["owner"].as_str().unwrap()
            ),
            row["expected"].as_str().unwrap()
        );
    }
    for row in data["plans"].as_array().unwrap() {
        assert_eq!(
            rules::plan_rust_array_lookup_condition(row["method"].as_str().unwrap()),
            strings(&row["condition"])
        );
        assert_eq!(
            rules::plan_rust_array_lookup_index(row["method"].as_str().unwrap()),
            strings(&row["index"])
        );
    }
    for row in data["consumerPlans"].as_array().unwrap() {
        assert_eq!(
            rules::plan_rust_array_lookup_consumer(row["consumer"].as_str().unwrap()),
            strings(&row["expected"])
        );
    }
}
#[test]
fn every_unsupported_and_malformed_source_keeps_its_actual_frontend_diagnostic() {
    let data = fixture();
    for row in data["refusals"].as_array().unwrap() {
        let error =
            meta_language::create_rust_source_owner_program_links(row["source"].as_str().unwrap())
                .unwrap_err();
        assert!(
            error.message.contains(row["reason"].as_str().unwrap()),
            "{}",
            error.message
        );
    }
    for row in data["malformed"].as_array().unwrap() {
        assert!(
            meta_language::create_rust_source_owner_program_links(row["source"].as_str().unwrap())
                .is_err()
        );
    }
}

#[test]
fn original_rust_rejects_each_retained_callable_type_mismatch_and_malformed_input() {
    let data = fixture();
    let directory = TemporaryDirectory::new("callable-owner-original-refusals");
    let mut failures = Vec::new();
    for (index, row) in data["refusals"]
        .as_array()
        .unwrap()
        .iter()
        .chain(data["malformed"].as_array().unwrap())
        .enumerate()
    {
        let input = directory.0.join(format!("refusal-{index}.rs"));
        let output = directory.0.join(format!("refusal-{index}.rmeta"));
        fs::write(&input, row["source"].as_str().unwrap()).unwrap();
        let observed = run(Command::new("rustc")
            .args([
                "--edition",
                "2024",
                "--crate-type",
                "lib",
                "--emit",
                "metadata",
                "--color",
                "never",
            ])
            .arg(&input)
            .arg("-o")
            .arg(&output));
        if observed.ok != row["originalRustAccepted"].as_bool().unwrap_or(false) {
            failures.push(format!(
                "{}: original Rust acceptance differs from the retained input contract",
                row["name"]
            ));
        }
        if let Some(codes) = row["originalRustErrorCodes"].as_array() {
            for code in codes {
                if !observed
                    .stderr
                    .contains(&format!("error[{}]", code.as_str().unwrap()))
                {
                    failures.push(format!(
                        "{}: original Rust omitted {}\n{}",
                        row["name"], code, observed.stderr
                    ));
                }
            }
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}
