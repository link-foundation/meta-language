#[path = "../../src/translation/rust_primitive_function_value_rules.rs"]
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
            .join("../parity/fixtures/rust-contextual-from-function-value.lino"),
    )
    .unwrap()
}
#[test]
fn complete_source_products_keep_original_and_canonical_callable_signatures() {
    super::callable_profile_products::assert_products(
        "rust-contextual-from-function-value",
        "contextual-from-function-value",
    );
}
#[test]
fn every_contextual_from_source_fact_retains_its_complete_expected_signature() {
    let data = fixture();
    for row in data["sourceControls"].as_array().unwrap() {
        assert_eq!(
            rules::read_rust_primitive_from_function_source(
                row["target"].as_str().unwrap(),
                row["parameters"].as_f64().unwrap(),
                row["kind"].as_str().unwrap(),
                row["bits"].as_str().unwrap(),
                row["signed"].as_bool().unwrap(),
                row["ownerKind"].as_str().unwrap(),
                row["source"].as_str().unwrap(),
                row["children"].as_f64().unwrap(),
                row["proven"].as_bool().unwrap(),
                row["layers"].as_f64().unwrap()
            ),
            row["expected"].as_str().unwrap()
        );
    }
}
#[test]
fn corrected_refusals_keep_every_preceding_source_scope_and_unaffected_assertion() {
    use sha2::{Digest, Sha256};
    let data = fixture();
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..");
    for retained in data["retained"].as_array().unwrap() {
        let path = root.join(retained["path"].as_str().unwrap());
        let bytes = std::fs::read(&path).unwrap();
        assert_eq!(bytes.len() as u64, retained["bytes"].as_u64().unwrap());
        assert_eq!(
            format!("{:x}", Sha256::digest(&bytes)),
            retained["sha256"].as_str().unwrap()
        );
        let before = meta_language::project_data::read_project_data_file(&path).unwrap();
        let after = meta_language::project_data::read_project_data_file(root.join(format!(
            "parity/fixtures/{}.lino",
            retained["fixture"].as_str().unwrap()
        )))
        .unwrap();
        for (field, value) in before.as_object().unwrap() {
            if !["refusals", "assertionCorrections"].contains(&field.as_str()) {
                assert_eq!(&after[field], value);
            }
        }
        if let Some(prior) = before["assertionCorrections"].as_array() {
            for correction in prior {
                assert!(
                    after["assertionCorrections"]
                        .as_array()
                        .unwrap()
                        .contains(correction)
                );
            }
        }
        for prior in before["refusals"].as_array().unwrap() {
            if let Some(kept) = after["refusals"]
                .as_array()
                .unwrap()
                .iter()
                .find(|row| row["name"] == prior["name"])
            {
                assert_eq!(kept, prior);
                continue;
            }
            let positive = data["cases"]
                .as_array()
                .unwrap()
                .iter()
                .find(|row| {
                    row["retainedRefusal"]["fixture"] == retained["fixture"]
                        && row["retainedRefusal"]["source"] == prior["source"]
                })
                .unwrap();
            assert!(
                positive["source"]
                    .as_str()
                    .unwrap()
                    .starts_with(prior["source"].as_str().unwrap())
            );
            assert_eq!(positive["retainedRefusal"]["kind"], prior["kind"]);
            assert_eq!(positive["retainedRefusal"]["reason"], prior["reason"]);
            assert!(
                after["assertionCorrections"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .any(|row| row["input"] == prior["source"]
                        && row["priorReason"] == prior["reason"]
                        && row["retainedFixture"] == retained["path"])
            );
        }
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
fn every_refused_and_malformed_source_remains_a_real_frontend_obligation() {
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
fn complete_preceding_generation_recipes_remain_authenticated_history() {
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
        let prior = meta_language::project_data::read_project_data_file(&path).unwrap();
        assert!(
            !prior["sourceClosure"]["inputs"]
                .as_array()
                .unwrap()
                .is_empty()
        );
        for source in prior["sourceClosure"]["inputs"].as_array().unwrap() {
            let text = source["source"].as_str().unwrap();
            assert_eq!(text.len() as u64, source["bytes"].as_u64().unwrap());
            assert_eq!(
                format!("{:x}", Sha256::digest(text.as_bytes())),
                source["sourceSha256"].as_str().unwrap()
            );
        }
    }
}
