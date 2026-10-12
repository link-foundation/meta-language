use super::issue_195_observations::{Observation, record};
use meta_language::program_translation::translate_program;
use std::process::Command;

#[test]
fn trim_predicates_preserve_each_languages_unicode_boundaries() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!(
        "../../../parity/fixtures/string-trim-semantics.json"
    ))
    .unwrap();
    let source = "pub fn trimmed(s: &str) -> String { s.trim().to_string() }\npub fn start(s: &str) -> String { s.trim_start().to_string() }\npub fn end(s: &str) -> String { s.trim_end().to_string() }\nfn main() {}\n";
    let javascript = translate_program(source, "Rust", "JavaScript").unwrap();
    assert_eq!(javascript.diagnostic(), None);
    assert!(!javascript.code().contains("portable-source-envelope"));
    for case in fixture["cases"].as_array().unwrap() {
        let input = case["input"].as_str().unwrap();
        assert_eq!(
            serde_json::json!([input.trim(), input.trim_start(), input.trim_end()]),
            case["rust"],
            "{}",
            case["name"]
        );
        // Build the expression separately so the source never participates in format syntax.
        let literal = serde_json::to_string(input).unwrap();
        let program = format!(
            "{}\nconsole.log(JSON.stringify([trimmed({literal}),start({literal}),end({literal})]));",
            javascript.code()
        );
        let output = Command::new("node")
            .args(["--input-type=module", "-e", &program])
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        assert_eq!(
            serde_json::from_slice::<serde_json::Value>(&output.stdout).unwrap(),
            case["rust"]
        );
        let js_source = format!(
            "/** @param {{string}} s @returns {{string}} */ function trimmed(s) {{ return s.trim(); }}\nconsole.log('%s', trimmed({literal}));"
        );
        let rust = translate_program(&js_source, "JavaScript", "Rust").unwrap();
        assert_eq!(rust.diagnostic(), None);
        assert!(!rust.code().contains("portable-source-envelope"));
        assert!(rust.code().contains("c != '\\u{85}'"));
        assert!(rust.code().contains("c == '\\u{feff}'"));
        let original = Command::new("node")
            .args(["--input-type=module", "-e", &js_source])
            .output()
            .unwrap();
        assert!(original.status.success());
        assert_eq!(
            String::from_utf8(original.stdout).unwrap(),
            format!("{}\n", case["javascript"][0].as_str().unwrap())
        );
    }
    record(&Observation {
        requirement_id: "I195-STRING-TRIM-SEMANTICS",
        suffix: "behavior",
        fixture_id: "string-trim-semantics",
        fixture_file: "parity/fixtures/string-trim-semantics.json",
        assertions: &[
            "rustWhitespacePreserved",
            "ecmascriptWhitespacePreserved",
            "bidirectionalUnicodeTrimFixtures",
            "trimTranslationsHaveNoSourceEnvelope",
        ],
        test_name: "trim_predicates_preserve_each_languages_unicode_boundaries",
    });
}
