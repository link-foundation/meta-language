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
            .join("../parity/fixtures/rust-array-cloned-lookup.lino"),
    )
    .unwrap()
}
#[test]
fn every_cloned_lookup_source_and_native_product_keeps_complete_independent_values() {
    super::callable_profile_products::assert_products(
        "rust-array-cloned-lookup",
        "array-cloned-lookup",
    );
}
#[test]
fn every_actual_standard_clone_owner_and_default_term_remains_explicit() {
    let data = fixture();
    for row in data["clonedElements"].as_array().unwrap() {
        assert_eq!(
            rules::read_rust_array_lookup_cloned_element_error(
                row["kind"].as_str().unwrap(),
                row["bits"].as_str().unwrap(),
                row["signed"].as_bool().unwrap(),
                row["ownerKind"].as_str().unwrap(),
                row["owner"].as_str().unwrap(),
                row["children"].as_f64().unwrap()
            ),
            row["expected"].as_str().unwrap()
        );
    }
    for row in data["defaultTerms"].as_array().unwrap() {
        assert_eq!(
            rules::read_rust_array_lookup_default_term(
                row["kind"].as_str().unwrap(),
                row["standardString"].as_bool().unwrap()
            ),
            row["expected"].as_str().unwrap()
        );
    }
}
#[test]
fn every_preceding_copied_scalar_input_keeps_its_independent_cloned_expectation() {
    let data = fixture();
    let prior = meta_language::project_data::read_project_data_file(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../parity/fixtures/rust-array-lookup.lino"),
    )
    .unwrap();
    for row in prior["cases"].as_array().unwrap() {
        if !row["source"].as_str().unwrap().contains(".copied()") {
            continue;
        }
        let current = data["cases"]
            .as_array()
            .unwrap()
            .iter()
            .find(|value| value["predecessorCase"]["name"] == row["name"])
            .unwrap();
        assert_eq!(
            current["source"].as_str().unwrap(),
            row["source"]
                .as_str()
                .unwrap()
                .replace(".copied()", ".cloned()")
        );
        assert_eq!(current["expected"], row["expected"]);
        assert_eq!(current["checks"], row["checks"]);
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

#[test]
fn every_refused_or_malformed_cloned_source_keeps_its_actual_frontend_obligation() {
    let data = fixture();
    for row in data["refusals"].as_array().unwrap() {
        let error =
            meta_language::create_rust_source_owner_program_links(row["source"].as_str().unwrap())
                .unwrap_err();
        if let Some(reason) = row["reason"].as_str() {
            assert!(error.message.contains(reason), "{}", error.message);
        } else {
            assert!(!error.message.is_empty());
        }
    }
    for row in data["malformed"].as_array().unwrap() {
        assert!(
            meta_language::create_rust_source_owner_program_links(row["source"].as_str().unwrap())
                .is_err()
        );
    }
}
#[test]
fn full_preceding_generation_recipes_remain_authenticated_source_definitions() {
    use sha2::{Digest, Sha256};
    let data = fixture();
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..");
    for row in data["recipes"].as_array().unwrap() {
        let path = root.join(row["path"].as_str().unwrap());
        let bytes = std::fs::read(&path).unwrap();
        assert_eq!(bytes.len() as u64, row["bytes"].as_u64().unwrap());
        assert_eq!(
            format!("{:x}", Sha256::digest(&bytes)),
            row["sha256"].as_str().unwrap()
        );
        let saved = meta_language::project_data::read_project_data_file(&path).unwrap();
        for source in saved["sourceClosure"]["inputs"].as_array().unwrap() {
            let text = source["source"].as_str().unwrap();
            assert_eq!(text.len() as u64, source["bytes"].as_u64().unwrap());
            assert_eq!(
                format!("{:x}", Sha256::digest(text.as_bytes())),
                source["sourceSha256"].as_str().unwrap()
            );
        }
    }
}
