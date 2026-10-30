#[path = "../../src/translation/rust_callable_owner_rules.rs"]
mod callable_rules;
use super::issue_195_translation_pairs::{TemporaryDirectory, run};
use meta_language::create_rust_source_owner_program_links;
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
            .join("../parity/fixtures/rust-callable-owner.lino"),
    )
    .unwrap()
}
#[test]
fn complete_source_products_keep_independent_scalar_function_types_and_observations() {
    super::callable_profile_products::assert_products("rust-callable-owner", "callable-owner");
}

fn strings(value: &Value) -> Vec<String> {
    value
        .as_array()
        .unwrap()
        .iter()
        .map(|item| item.as_str().unwrap().to_owned())
        .collect()
}
#[test]
fn every_recursive_callable_owner_contract_join_equality_budget_and_refusal_stays_explicit() {
    let data = fixture();
    for row in data["contractErrors"].as_array().unwrap() {
        assert_eq!(
            callable_rules::read_rust_callable_owner_error(
                &strings(&row["actual"]),
                &strings(&row["expected"])
            ),
            row["expectedError"].as_str().unwrap()
        );
    }
    for row in data["equalities"].as_array().unwrap() {
        assert_eq!(
            callable_rules::accept_rust_callable_owner_descriptors_equal(
                &strings(&row["left"]),
                &strings(&row["right"])
            ),
            row["expected"].as_bool().unwrap()
        );
    }
    for row in data["joins"].as_array().unwrap() {
        assert_eq!(
            callable_rules::read_rust_callable_owner_join_error(&strings(&row["rows"])),
            row["expectedError"].as_str().unwrap()
        );
    }
    for row in data["limits"].as_array().unwrap() {
        assert_eq!(
            callable_rules::read_rust_callable_owner_projection_limit(
                row["name"].as_str().unwrap()
            ),
            row["expected"].as_f64().unwrap()
        );
    }
    for row in data["oversized"].as_array().unwrap() {
        let descriptor =
            vec![String::new(); usize::try_from(row["size"].as_u64().unwrap()).unwrap()];
        assert_eq!(
            callable_rules::read_rust_callable_owner_error(&descriptor, &[]),
            row["expectedError"].as_str().unwrap()
        );
        assert_eq!(
            callable_rules::read_rust_callable_owner_join_error(&descriptor),
            row["expectedError"].as_str().unwrap()
        );
        assert!(
            !callable_rules::accept_rust_callable_owner_descriptors_equal(&descriptor, &descriptor)
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
    for row in data["malformed"].as_array().unwrap() {
        assert!(
            create_rust_source_owner_program_links(row["source"].as_str().unwrap()).is_err(),
            "{}",
            row["name"]
        );
    }
}

#[test]
fn original_rust_rejects_each_retained_source_type_mismatch_and_malformed_input() {
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
        if observed.ok {
            failures.push(format!(
                "{}: original Rust accepted a retained invalid input",
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

#[test]
fn corrected_emitter_scope_retains_every_original_source_and_type_assertion() {
    use sha2::{Digest, Sha256};
    let data = fixture();
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..");
    for correction in data["assertionCorrections"].as_array().unwrap() {
        let path = root.join(correction["retainedDraft"].as_str().unwrap());
        let bytes = fs::read(&path).unwrap();
        assert_eq!(bytes.len() as u64, correction["bytes"].as_u64().unwrap());
        assert_eq!(
            format!("{:x}", Sha256::digest(&bytes)),
            correction["sha256"].as_str().unwrap()
        );
        let prior = meta_language::project_data::read_project_data_file(path).unwrap();
        assert!(
            prior["cases"].as_array().unwrap().len() <= data["cases"].as_array().unwrap().len()
        );
        for (index, before) in prior["cases"].as_array().unwrap().iter().enumerate() {
            let after = &data["cases"][index];
            for field in ["name", "module", "source", "expected", "resultType"] {
                assert_eq!(after[field], before[field]);
            }
            for check in before["checks"].as_array().unwrap() {
                assert!(after["checks"].as_array().unwrap().contains(check));
            }
        }
    }
}
