//! Deferred native compiler comparison on every actual checked source body.
#[path = "javascript_call_source_codecs.rs"]
mod codecs;
#[path = "../../src/translation/javascript_call_source.rs"]
mod native;
use super::issue_195_translation_pairs::run;
use codecs::*;
use meta_language::read_structured_data_lino;
use std::process::Command;
#[test]
fn all_actual_source_functions_compile_to_the_same_complete_instructions() {
    let version = run(Command::new("rustc").arg("--version"));
    assert!(version.ok, "{}", version.stderr);
    let actual = version.stdout.split_whitespace().nth(1).unwrap();
    let pieces = actual
        .split('.')
        .map(|value| value.parse::<u32>().unwrap())
        .collect::<Vec<_>>();
    assert!(
        pieces[0] == 1 && (90..100).contains(&pieces[1]),
        "uncovered Rust {actual}; >=1.90 <1.100"
    );
    let producer = run(Command::new("node")
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .args([
            "--max-old-space-size=352",
            "--max-semi-space-size=1",
            "../experiments/javascript-properties/call-source-oracle.mjs",
        ]));
    assert!(producer.ok, "{}", producer.stderr);
    let data = read_structured_data_lino(&producer.stdout).unwrap();
    for row in data["rows"].as_array().unwrap() {
        for unit in row["sourceUnits"].as_array().unwrap() {
            let source = decode_java_script_call_source(&unit["source"]);
            assert_eq!(encode_java_script_call_source(&source), unit["source"]);
            let parameters = unit["parameters"]
                .as_array()
                .unwrap()
                .iter()
                .map(|value| value.as_f64().unwrap())
                .collect::<Vec<_>>();
            let actual = native::compile_java_script_call_source(source, &parameters);
            assert_eq!(
                encode_java_script_call_source_compilation(&actual),
                unit["compilation"],
                "{} {}",
                row["name"],
                unit["name"]
            );
            assert_eq!(
                encode_java_script_call_source_compilation(
                    &decode_java_script_call_source_compilation(&unit["compilation"])
                ),
                unit["compilation"]
            );
        }
    }
    for row in data["sourceRefusals"].as_array().unwrap() {
        let source = decode_java_script_call_source(&row["expression"]);
        assert_eq!(encode_java_script_call_source(&source), row["expression"]);
        let parameters = row["parameters"]
            .as_array()
            .unwrap()
            .iter()
            .map(|value| value.as_f64().unwrap())
            .collect::<Vec<_>>();
        let actual = native::compile_java_script_call_source(source, &parameters);
        assert_eq!(
            encode_java_script_call_source_compilation(&actual),
            row["compilation"],
            "{}",
            row["name"]
        );
    }
}
