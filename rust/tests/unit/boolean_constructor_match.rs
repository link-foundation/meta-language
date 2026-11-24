//! Deferred complete native rule cases; no compilation during drafting.
#[path = "../../src/translation/boolean_constructor_match_rules.rs"]
mod native;
use super::issue_195_translation_pairs::run;
use meta_language::read_structured_data_lino;
use meta_language::translation::{
    check::check_program, emit_javascript::emit_javascript, javascript::parse_javascript,
};
use std::process::Command;

#[test]
fn boolean_constructor_match_retains_every_plan_and_refusal() {
    let version = run(Command::new("rustc").arg("--version"));
    assert!(version.ok, "{}", version.stderr);
    let actual = version.stdout.split_whitespace().nth(1).unwrap();
    let parts = actual
        .split('.')
        .map(|value| value.parse::<u32>().unwrap())
        .collect::<Vec<_>>();
    assert!(
        parts[0] == 1 && (90..100).contains(&parts[1]),
        "uncovered Rust {actual}; >=1.90 <1.100"
    );
    let data = read_structured_data_lino(include_str!(
        "../../../parity/fixtures/boolean-constructor-match.lino"
    ))
    .unwrap();
    for row in data["rows"].as_array().unwrap() {
        let strings = |name: &str| {
            row[name]
                .as_array()
                .unwrap()
                .iter()
                .map(|value| value.as_str().unwrap().to_owned())
                .collect::<Vec<_>>()
        };
        let literals = row["literals"]
            .as_array()
            .unwrap()
            .iter()
            .map(|value| value.as_f64().unwrap())
            .collect::<Vec<_>>();
        let expected = row["expected"]
            .as_array()
            .unwrap()
            .iter()
            .map(|value| value.as_f64().unwrap())
            .collect::<Vec<_>>();
        assert_eq!(
            native::plan_boolean_constructor_match_sharing(
                row["owner"].as_str().unwrap(),
                &strings("declared"),
                &strings("owners"),
                &strings("constructors"),
                &literals
            ),
            expected,
            "{}",
            row["name"]
        );
    }
    for row in data["generalRows"].as_array().unwrap() {
        let strings = |name: &str| {
            row[name]
                .as_array()
                .unwrap()
                .iter()
                .map(|value| value.as_str().unwrap().to_owned())
                .collect::<Vec<_>>()
        };
        let classes = row["classes"]
            .as_array()
            .unwrap()
            .iter()
            .map(|value| value.as_f64().unwrap())
            .collect::<Vec<_>>();
        let expected = row["expected"]
            .as_array()
            .unwrap()
            .iter()
            .map(|value| value.as_f64().unwrap())
            .collect::<Vec<_>>();
        assert_eq!(
            native::plan_closed_constructor_match_sharing(
                row["owner"].as_str().unwrap(),
                &strings("declared"),
                &strings("owners"),
                &strings("constructors"),
                &classes
            ),
            expected,
            "{}",
            row["name"]
        );
    }
    for row in data["bindingRows"].as_array().unwrap() {
        let strings = |name: &str| {
            row[name]
                .as_array()
                .unwrap()
                .iter()
                .map(|value| value.as_str().unwrap().to_owned())
                .collect::<Vec<_>>()
        };
        let expected = row["expected"]
            .as_array()
            .unwrap()
            .iter()
            .map(|value| value.as_f64().unwrap())
            .collect::<Vec<_>>();
        assert_eq!(
            native::plan_unused_constructor_bindings(&strings("bindings"), &strings("references")),
            expected,
            "{}",
            row["name"]
        );
    }
    for row in data["aliasRows"].as_array().unwrap() {
        let strings = |name: &str| {
            row[name]
                .as_array()
                .unwrap()
                .iter()
                .map(|value| value.as_str().unwrap().to_owned())
                .collect::<Vec<_>>()
        };
        assert_eq!(
            native::test_unused_constructor_alias(
                row["nameOfBinding"].as_str().unwrap(),
                row["value"].as_str().unwrap(),
                &strings("available"),
                &strings("references")
            ),
            row["expected"].as_bool().unwrap(),
            "{}",
            row["name"]
        );
    }
    for row in data["projectionRows"].as_array().unwrap() {
        let counts = row["counts"]
            .as_array()
            .unwrap()
            .iter()
            .map(|value| value.as_f64().unwrap())
            .collect::<Vec<_>>();
        let expected = row["expected"]
            .as_array()
            .unwrap()
            .iter()
            .map(|value| value.as_f64().unwrap())
            .collect::<Vec<_>>();
        assert_eq!(
            native::plan_single_constructor_projections(
                row["constructors"].as_f64().unwrap(),
                &counts
            ),
            expected,
            "{}",
            row["name"]
        );
    }
}

#[test]
fn effectful_tag_receiver_keeps_single_evaluation_and_exact_text() {
    let fixture = read_structured_data_lino(include_str!(
        "../../../parity/fixtures/boolean-constructor-match.lino"
    ))
    .unwrap();
    for row in fixture["textPrograms"].as_array().unwrap() {
        let name = row["name"].as_str().unwrap();
        let source = row["source"].as_str().unwrap();
        let program = check_program(&parse_javascript(source).unwrap()).unwrap();
        let emitted = emit_javascript(&program).unwrap();
        let entry = &emitted
            .mappings
            .iter()
            .find(|mapping| mapping.kind == "function" && mapping.source == "compute")
            .unwrap()
            .target;
        for (body, function) in [(source, "compute"), (emitted.text.as_str(), entry.as_str())] {
            let script = format!(
                "{body}\nconst result = {function}();\nconsole.log(JSON.stringify(result.split('').map(unit => unit.charCodeAt(0))));"
            );
            let observation = run(Command::new("node").args(["--input-type=module", "-e", &script]));
            assert!(observation.ok, "{name}: {}", observation.stderr);
            let units: serde_json::Value = serde_json::from_str(&observation.stdout).unwrap();
            assert_eq!(units, row["expectedUnits"], "{name}");
        }
    }
}
