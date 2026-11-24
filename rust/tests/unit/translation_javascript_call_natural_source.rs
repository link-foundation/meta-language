//! Deferred full shared-rule parity, construction codecs and actual compilation.
#[path = "javascript_call_natural_source_codecs.rs"]
mod codecs;
#[path = "../../src/translation/javascript_call_natural_source.rs"]
mod native;
use super::issue_195_translation_pairs::run;
use codecs::*;
use meta_language::read_structured_data_lino;
use serde_json::Value as Data;
use std::process::Command;

fn text(value: &Data) -> native::ml_string::Text {
    native::ml_string::Text::from_text(value.as_str().unwrap())
}
fn texts(value: &Data) -> Vec<native::ml_string::Text> {
    value.as_array().unwrap().iter().map(text).collect()
}
fn numbers(value: &Data) -> Vec<f64> {
    value
        .as_array()
        .unwrap()
        .iter()
        .map(|value| value.as_f64().unwrap())
        .collect()
}

#[test]
fn every_shared_natural_rule_and_source_construction_matches() {
    let version = run(Command::new("rustc").arg("--version"));
    assert!(version.ok, "{}", version.stderr);
    let actual = version.stdout.split_whitespace().nth(1).unwrap();
    let parts = actual.split('.').collect::<Vec<_>>();
    assert!(
        parts[0] == "1" && (90..100).contains(&parts[1].parse::<u32>().unwrap()),
        "uncovered Rust {actual}; >=1.90 <1.100"
    );
    let producer = run(Command::new("node")
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .args([
            "--max-old-space-size=352",
            "--max-semi-space-size=1",
            "../experiments/javascript-properties/call-natural-source-oracle.mjs",
        ]));
    assert!(producer.ok, "{}", producer.stderr);
    let data = read_structured_data_lino(&producer.stdout).unwrap();
    for row in data["guardOrders"].as_array().unwrap() {
        assert_eq!(
            native::plan_java_script_natural_guard_order(
                &texts(&row["types"]),
                &numbers(&row["orders"]),
                row["oneRealm"].as_bool().unwrap()
            ),
            numbers(&row["expected"]),
            "{}",
            row["name"]
        );
    }
    for row in data["casts"].as_array().unwrap() {
        assert_eq!(
            native::read_java_script_natural_cast_plan(
                &text(&row["from"]),
                &text(&row["to"]),
                &text(&row["flavor"]),
                row["retainedMessage"].as_bool().unwrap(),
                row["order"].as_f64().unwrap(),
                row["oneRealm"].as_bool().unwrap()
            ),
            row["expected"].as_f64().unwrap(),
            "{}",
            row["name"]
        );
    }
    for row in data["patterns"].as_array().unwrap() {
        assert_eq!(
            native::plan_java_script_natural_pattern_branches(
                &text(&row["kind"]),
                &texts(&row["patterns"])
            ),
            numbers(&row["expected"]),
            "{}",
            row["name"]
        );
    }
    for row in data["bindings"].as_array().unwrap() {
        assert_eq!(
            native::read_java_script_natural_pattern_binding(&text(&row["pattern"])),
            row["expected"].as_f64().unwrap()
        );
    }
    for row in data["realms"].as_array().unwrap() {
        assert_eq!(
            native::test_java_script_natural_source_realm(&texts(&row["realms"])),
            row["expected"].as_bool().unwrap()
        );
    }
    for row in data["builders"].as_array().unwrap() {
        let message = native::ml_string::Text::from_units(
            row.get("messageUnits")
                .map(|units| {
                    units
                        .as_array()
                        .unwrap()
                        .iter()
                        .map(|unit| u16::try_from(unit.as_u64().unwrap()).unwrap())
                        .collect()
                })
                .unwrap_or_default(),
        );
        let source = match row["operation"].as_str().unwrap() {
            "error" => native::construct_java_script_literal_error_source(&message),
            "guard" => native::construct_java_script_natural_guard_source(
                row["identity"].as_f64().unwrap(),
                &message,
                decode_java_script_call_source(&row["body"]),
            ),
            "cast" => native::construct_java_script_natural_cast_source(
                row["identity"].as_f64().unwrap(),
                &message,
                row["clamp"].as_bool().unwrap(),
            ),
            "difference" => native::construct_java_script_natural_difference_source(
                row["left"].as_f64().unwrap(),
                row["right"].as_f64().unwrap(),
            ),
            "predecessor" => native::construct_java_script_natural_predecessor_source(
                row["identity"].as_f64().unwrap(),
            ),
            "match" => native::construct_java_script_natural_match_source(
                row["identity"].as_f64().unwrap(),
                decode_java_script_call_source(&row["zero"]),
                decode_java_script_call_source(&row["succ"]),
            ),
            _ => panic!("every declared builder requires its complete native control"),
        };
        assert_eq!(
            encode_java_script_call_source(&source),
            row["source"],
            "{}",
            row["name"]
        );
        assert_eq!(
            encode_java_script_call_source(&decode_java_script_call_source(&row["source"])),
            row["source"],
            "{}",
            row["name"]
        );
        let compiled =
            native::compile_java_script_natural_source(source, &numbers(&row["parameters"]));
        assert_eq!(
            encode_java_script_call_source_compilation(&compiled),
            row["compilation"],
            "{}",
            row["name"]
        );
    }
}
