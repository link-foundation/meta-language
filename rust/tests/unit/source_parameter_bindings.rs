//! Deferred shared-column, original Node, CST binding and rename controls.
#[path = "../../src/translation/source_parameter_binding_rules.rs"]
mod rules;
use super::issue_195_translation_pairs::{TemporaryDirectory, run};
use meta_language::{ProgramProjectContext, ProgramRepresentation, analyze_program};
use std::{fs, path::Path, process::Command};
fn texts(value: &serde_json::Value) -> Vec<String> {
    value
        .as_array()
        .unwrap()
        .iter()
        .map(|value| value.as_str().unwrap().to_string())
        .collect()
}
fn numbers(value: &serde_json::Value) -> Vec<f64> {
    value
        .as_array()
        .unwrap()
        .iter()
        .map(|value| value.as_f64().unwrap())
        .collect()
}
fn observe(index: usize, source: Option<&Path>) -> String {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
    let mut command = Command::new("node");
    command
        .arg("--max-old-space-size=352")
        .arg(root.join("experiments/source-parameter-bindings/oracle.mjs"))
        .arg(index.to_string())
        .current_dir(root);
    if let Some(source) = source {
        command.arg(source);
    }
    let response = run(&mut command);
    assert!(response.ok, "{}", response.stderr);
    let value = meta_language::read_structured_data_lino(&response.stdout).unwrap();
    value["value"].as_str().unwrap().to_string()
}
#[test]
fn actual_cst_parameter_bindings_keep_complete_originals_and_source_owners() {
    super::translation_string_array_copy_sort::require_rust_profile();
    let fixture = meta_language::read_structured_data_lino(include_str!(
        "../../../parity/fixtures/source-parameter-bindings.lino"
    ))
    .unwrap();
    for row in fixture["terms"].as_array().unwrap() {
        assert_eq!(
            rules::accept_java_script_source_binding_root_term(row["term"].as_str().unwrap()),
            row["root"].as_bool().unwrap()
        );
        assert_eq!(
            rules::accept_java_script_source_parameter_identifier_term(
                row["term"].as_str().unwrap()
            ),
            row["expected"].as_bool().unwrap()
        );
    }
    for row in fixture["plans"].as_array().unwrap() {
        let candidates = row["candidates"]
            .as_array()
            .unwrap()
            .iter()
            .map(|value| value.as_bool().unwrap())
            .collect::<Vec<_>>();
        assert_eq!(
            rules::plan_java_script_source_parameter_bindings(
                &texts(&row["kinds"]),
                &texts(&row["values"]),
                &candidates,
                &numbers(&row["defaultFirsts"]),
                &numbers(&row["defaultLasts"]),
                &numbers(&row["computedFirsts"]),
                &numbers(&row["computedLasts"])
            ),
            numbers(&row["expected"])
        );
    }
    let directory = TemporaryDirectory::new("source-parameter-bindings");
    for (index, row) in fixture["cases"].as_array().unwrap().iter().enumerate() {
        let source = row["source"].as_str().unwrap();
        let program =
            analyze_program(source, "JavaScript", ProgramProjectContext::default()).unwrap();
        assert_eq!(program.network().reconstruct_text(), source);
        let expected = row["bindings"].as_array().unwrap();
        assert_eq!(program.bindings().len(), expected.len());
        for (binding, expected) in program.bindings().iter().zip(expected) {
            assert_eq!(binding.name(), expected[0].as_str().unwrap());
            assert_eq!(binding.kind(), expected[1].as_str().unwrap());
            assert_eq!(
                binding.references().len(),
                expected[2].as_u64().unwrap() as usize
            );
        }
        assert_eq!(
            program
                .unresolved_references()
                .iter()
                .map(|reference| reference.name().to_string())
                .collect::<Vec<_>>(),
            texts(&row["unresolved"])
        );
        assert_eq!(observe(index, None), row["value"].as_str().unwrap());
        let rename = &row["rename"];
        let binding = program
            .bindings()
            .iter()
            .find(|binding| {
                binding.kind()
                    == rename
                        .get("kind")
                        .and_then(|kind| kind.as_str())
                        .unwrap_or("parameter")
                    && binding.name() == rename["name"].as_str().unwrap()
            })
            .unwrap();
        let renamed = program
            .rename_binding(binding.id(), rename["replacement"].as_str().unwrap())
            .unwrap();
        assert_eq!(renamed.source(), rename["source"].as_str().unwrap());
        let path = directory.0.join(format!("renamed-{index}.js"));
        fs::write(&path, renamed.source()).unwrap();
        assert_eq!(observe(index, Some(&path)), row["value"].as_str().unwrap());
        let restored = ProgramRepresentation::from_snapshot(&renamed.serialize_snapshot()).unwrap();
        assert_eq!(restored.source(), renamed.source());
        assert_eq!(restored.bindings(), renamed.bindings());
    }
}
