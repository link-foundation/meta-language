use std::fs;
use std::path::PathBuf;
use std::process::Command;

use meta_language::{analyze_program, ProgramProjectContext};
use serde_json::Value;

#[test]
fn binding_aware_rename_preserves_scopes_properties_unicode_and_observations() {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../parity/fixtures/four-language-conformance.json");
    let corpus: Value = serde_json::from_str(&fs::read_to_string(path).expect("shared corpus"))
        .expect("valid shared corpus");

    for fixture in corpus["renameCases"].as_array().expect("rename cases") {
        let language = fixture["language"].as_str().expect("language");
        let source = fixture["source"].as_str().expect("source");
        let program = analyze_program(source, language, ProgramProjectContext::default())
            .expect("semantic analysis");
        let occurrence = usize::try_from(
            fixture["declarationOccurrence"]
                .as_u64()
                .expect("declaration occurrence"),
        )
        .expect("occurrence fits usize");
        let binding = program
            .bindings()
            .iter()
            .filter(|binding| binding.name() == fixture["binding"].as_str().unwrap())
            .nth(occurrence)
            .expect("selected binding");
        let renamed = program
            .rename_binding(binding.id(), fixture["replacement"].as_str().unwrap())
            .expect("capture-safe rename");
        assert_eq!(
            renamed.emit(),
            fixture["expected"].as_str().expect("expected source"),
            "{language} binding rename"
        );
        assert!(renamed.network().verify_full_match(None).is_clean());
        if !fixture["expectedObservation"].is_null() {
            let renamed_source = renamed.emit();
            for source in [source, renamed_source.as_str()] {
                let output = Command::new("node")
                    .args([
                        "-e",
                        "eval(process.argv[1]); process.stdout.write(JSON.stringify(globalThis.result));",
                        source,
                    ])
                    .output()
                    .expect("Node executes JavaScript binding fixture");
                assert!(
                    output.status.success(),
                    "JavaScript fixture executes: {language}"
                );
                let observed: Value = serde_json::from_slice(&output.stdout)
                    .expect("fixture emits a JSON observation");
                assert_eq!(
                    observed, fixture["expectedObservation"],
                    "{language} binding observation"
                );
            }
        }
        let error = program
            .rename_binding(binding.id(), fixture["capture"].as_str().unwrap())
            .expect_err("capture must be rejected");
        assert!(
            error.to_string().contains("capture") || error.to_string().contains("conflict"),
            "{language} capture avoidance: {error}"
        );
    }
}

#[test]
fn binding_rename_rejects_capture_of_an_unresolved_reference() {
    let source = "globalThis.y = 10; const x = 1; globalThis.result = x + y;";
    let program = analyze_program(source, "JavaScript", ProgramProjectContext::default())
        .expect("JavaScript analysis");
    let binding = program
        .bindings()
        .iter()
        .find(|binding| binding.name() == "x")
        .expect("x binding");
    assert!(program
        .unresolved_references()
        .iter()
        .any(|reference| reference.name() == "y"));
    let error = program
        .rename_binding(binding.id(), "y")
        .expect_err("rename must reject capture of the global y reference");
    assert!(error.to_string().contains("capture"));

    let sibling_source = "function f() { const x = 1; return x; } function g() { return y; }";
    let sibling_program = analyze_program(
        sibling_source,
        "JavaScript",
        ProgramProjectContext::default(),
    )
    .expect("JavaScript sibling scopes");
    let sibling_binding = sibling_program
        .bindings()
        .iter()
        .find(|binding| binding.name() == "x")
        .expect("sibling x binding");
    assert_eq!(
        sibling_program
            .rename_binding(sibling_binding.id(), "y")
            .expect("unrelated sibling reference does not conflict")
            .emit(),
        "function f() { const y = 1; return y; } function g() { return y; }"
    );
}

#[test]
fn binding_rename_distinguishes_disjoint_nested_names_from_capture() {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../parity/fixtures/four-language-conformance.json");
    let corpus: Value = serde_json::from_str(&fs::read_to_string(path).expect("shared corpus"))
        .expect("valid shared corpus");

    for fixture in corpus["nestedRenameCases"]
        .as_array()
        .expect("nested rename cases")
    {
        let source = fixture["source"].as_str().expect("source");
        let program = analyze_program(source, "JavaScript", ProgramProjectContext::default())
            .expect("JavaScript analysis");
        let binding = program
            .bindings()
            .iter()
            .find(|binding| binding.name() == fixture["binding"].as_str().unwrap())
            .expect("selected binding");
        let original = javascript_observation(source);
        assert_eq!(original, fixture["expectedObservation"]);

        let result = program.rename_binding(binding.id(), fixture["replacement"].as_str().unwrap());
        if fixture["allowed"] == false {
            assert!(result
                .expect_err("capture must be rejected")
                .to_string()
                .contains("capture"));
            continue;
        }
        let renamed = result.expect("disjoint nested name is safe");
        assert_eq!(
            renamed.emit(),
            fixture["expected"].as_str().expect("expected source")
        );
        assert!(renamed.network().verify_full_match(None).is_clean());
        assert_eq!(
            javascript_observation(&renamed.emit()),
            fixture["expectedObservation"]
        );
    }
}

#[test]
fn javascript_var_bindings_use_function_scope_and_include_early_references() {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../parity/fixtures/four-language-conformance.json");
    let corpus: Value = serde_json::from_str(&fs::read_to_string(path).expect("shared corpus"))
        .expect("valid shared corpus");

    for fixture in corpus["varScopeCases"].as_array().expect("var scope cases") {
        let source = fixture["source"].as_str().expect("source");
        let program = analyze_program(source, "JavaScript", ProgramProjectContext::default())
            .expect("JavaScript analysis");
        let occurrence = usize::try_from(fixture["declarationOccurrence"].as_u64().unwrap())
            .expect("occurrence fits usize");
        let binding = program
            .bindings()
            .iter()
            .filter(|binding| binding.name() == fixture["binding"].as_str().unwrap())
            .nth(occurrence)
            .expect("selected var binding");
        assert_eq!(
            binding.references().len(),
            usize::try_from(fixture["expectedReferences"].as_u64().unwrap())
                .expect("reference count fits usize")
        );
        let renamed = program
            .rename_binding(binding.id(), fixture["replacement"].as_str().unwrap())
            .expect("rename scoped var");
        assert_eq!(renamed.emit(), fixture["expected"].as_str().unwrap());
        let renamed_source = renamed.emit();
        for source in [source, renamed_source.as_str()] {
            assert_eq!(
                javascript_observation(source),
                fixture["expectedObservation"]
            );
        }
    }
    let lexical = analyze_program(
        "{ let x = 1; } x;",
        "JavaScript",
        ProgramProjectContext::default(),
    )
    .expect("JavaScript lexical analysis");
    assert_eq!(
        lexical
            .bindings()
            .iter()
            .find(|binding| binding.name() == "x")
            .expect("x binding")
            .references()
            .len(),
        0
    );
    assert!(lexical
        .unresolved_references()
        .iter()
        .any(|reference| reference.name() == "x"));
}

fn javascript_observation(source: &str) -> Value {
    let output = Command::new("node")
        .args([
            "-e",
            "eval(process.argv[1]); process.stdout.write(JSON.stringify(globalThis.result));",
            source,
        ])
        .output()
        .expect("Node executes JavaScript fixture");
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    serde_json::from_slice(&output.stdout).expect("fixture emits JSON observation")
}
