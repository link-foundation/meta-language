use meta_language::program_translation::translate_program;
use meta_language::translation::{
    check::check_program, emit_javascript::emit_javascript, javascript::parse_javascript,
};
use std::process::Command;

#[test]
fn string_maps_match_javascript_emission_and_preserve_unicode_and_whitespace() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!(
        "../../../parity/fixtures/self-translation-bindings.json"
    ))
    .unwrap();
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap();
    let script = "import {translateProgram} from './js/src/program-translation.js'; console.log(JSON.stringify(translateProgram(process.argv[1], 'JavaScript', 'Rust')));";
    for entry in fixture["cases"].as_array().unwrap().iter().filter(|entry| {
        matches!(
            entry["name"].as_str().unwrap(),
            "unicode-case-mapping"
                | "javascript-whitespace-trimming"
                | "non-javascript-whitespace-retained"
        )
    }) {
        let source = format!(
            "{}\nconsole.log('%s', {});\n",
            entry["source"].as_str().unwrap().replace("export ", ""),
            entry["call"].as_str().unwrap()
        );
        let expected = Command::new("node")
            .current_dir(root)
            .args(["--input-type=module", "-e", script, &source])
            .output()
            .unwrap();
        assert!(
            expected.status.success(),
            "{}",
            String::from_utf8_lossy(&expected.stderr)
        );
        let expected: serde_json::Value = serde_json::from_slice(&expected.stdout).unwrap();
        let translated = translate_program(&source, "JavaScript", "Rust").unwrap();
        assert_eq!(translated.diagnostic(), None);
        assert_eq!(translated.code(), expected["code"].as_str().unwrap());
        let program = check_program(&parse_javascript(&source).unwrap()).unwrap();
        let emitted = emit_javascript(&program).unwrap();
        let executed = Command::new("node")
            .args(["--input-type=module", "-e", &emitted.text])
            .output()
            .unwrap();
        assert!(
            executed.status.success(),
            "{}",
            String::from_utf8_lossy(&executed.stderr)
        );
        assert_eq!(
            String::from_utf8(executed.stdout).unwrap(),
            entry["expected"].as_str().unwrap()
        );
        for target in ["Lean", "Rocq"] {
            let refused = translate_program(&source, "JavaScript", target).unwrap();
            assert!(refused.diagnostic().unwrap().message.contains("ASCII"));
        }
    }
}

#[test]
fn string_maps_read_rust_case_methods_and_refuse_invalid_arguments() {
    let source = "pub fn shout(value: String) -> String { value.to_lowercase().to_uppercase() } fn main() { println!(\"{}\", shout(String::from(\"Grüße\"))); }";
    let result = translate_program(source, "Rust", "JavaScript").unwrap();
    assert_eq!(result.diagnostic(), None);
    let output = Command::new("node")
        .args(["--input-type=module", "-e", result.code()])
        .output()
        .unwrap();
    assert!(output.status.success());
    assert_eq!(String::from_utf8(output.stdout).unwrap(), "GRÜSSE\n");
    for source in [
        "console.log('a'.trim(1));",
        "console.log(7 .toLowerCase());",
    ] {
        assert!(
            translate_program(source, "JavaScript", "Rust")
                .unwrap()
                .diagnostic()
                .is_some()
        );
    }
}
