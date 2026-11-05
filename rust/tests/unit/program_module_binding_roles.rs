//! Future paired CST, exact source, original Node and codec controls.
use super::issue_195_translation_pairs::{TemporaryDirectory, run};
use meta_language::{ProgramProjectContext, analyze_program, create_module_binding_program_links};
use std::{fs, process::Command};

#[test]
fn module_binding_renames_keep_imported_and_exported_names_and_actual_values() {
    let response = run(Command::new("node")
        .arg("--max-old-space-size=352")
        .arg("../experiments/module-bindings/oracle.mjs")
        .current_dir(env!("CARGO_MANIFEST_DIR")));
    assert!(response.ok, "{}", response.stderr);
    let oracle: serde_json::Value = serde_json::from_str(&response.stdout).unwrap();
    let directory = TemporaryDirectory::new("module-binding-native-source");
    let artifact = directory.0.join("renamed.mjs");
    for (index, row) in oracle["cases"].as_array().unwrap().iter().enumerate() {
        let source = row["source"].as_str().unwrap();
        let program =
            analyze_program(source, "JavaScript", ProgramProjectContext::default()).unwrap();
        assert_eq!(program.source(), source);
        assert_eq!(program.network().reconstruct_text(), source);
        let binding = program
            .bindings()
            .iter()
            .find(|binding| {
                binding.scope() == "scope:0" && binding.name() == row["binding"].as_str().unwrap()
            })
            .unwrap();
        let renamed = program
            .rename_binding(binding.id(), row["replacement"].as_str().unwrap())
            .unwrap();
        assert_eq!(renamed.source(), row["expectedSource"].as_str().unwrap());
        assert_eq!(renamed.source(), row["javascript"].as_str().unwrap());
        fs::write(&artifact, renamed.source()).unwrap();
        let observed = run(Command::new("node")
            .arg("../experiments/module-bindings/observe-source.mjs")
            .arg(&artifact)
            .arg(index.to_string())
            .current_dir(env!("CARGO_MANIFEST_DIR")));
        assert!(observed.ok, "{}", observed.stderr);
        let observed: serde_json::Value = serde_json::from_str(&observed.stdout).unwrap();
        let expected = row["observations"]
            .as_array()
            .unwrap()
            .iter()
            .map(|row| row["value"].clone())
            .collect::<Vec<_>>();
        assert_eq!(observed["values"], serde_json::json!(expected));
        assert_eq!(observed["values"], row["original"]["values"]);
        assert_eq!(observed["values"], row["renamed"]["values"]);
    }
    let definition = create_module_binding_program_links().unwrap();
    assert_eq!(
        definition.network.to_lino(),
        oracle["definition"]["network"].as_str().unwrap()
    );
    assert_eq!(
        definition.root.as_u64(),
        oracle["definition"]["root"].as_u64().unwrap()
    );
}
