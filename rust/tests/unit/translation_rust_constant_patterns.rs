//! Deferred constant integer originals, native products and shared source plans.
#[path = "../../src/translation/rust_constant_integer_rules.rs"]
mod rules;
use serde_json::Value;
fn text(args: &Value, index: usize) -> &str {
    args[index].as_str().unwrap()
}
#[test]
fn primitive_constants_retain_actual_pattern_owners_and_exact_arithmetic() {
    super::translation_string_array_copy_sort::require_rust_profile();
    let compiler = super::issue_195_translation_pairs::run(
        std::process::Command::new("rustc").arg("--version"),
    );
    assert!(compiler.ok, "{}", compiler.stderr);
    let version = compiler
        .stdout
        .split_whitespace()
        .nth(1)
        .unwrap()
        .split('.')
        .collect::<Vec<_>>();
    assert!(
        version[0] == "1" && (90..100).contains(&version[1].parse::<u32>().unwrap()),
        "uncovered Rust {}; require >=1.90.0 <1.100.0",
        compiler.stdout
    );
    assert!(cfg!(target_pointer_width = "64"), "uncovered pointer width");
    super::translation_rust_records::assert_nominal_translation_product(
        "../experiments/rust-records/constant-patterns.mjs",
        "Rust primitive constant patterns and exact constant arithmetic",
        || {
            meta_language::create_source_definition_program_links(
                "meta-language/source-program/rust-constant-integer-program",
            )
            .map_err(|message| meta_language::TypedProgramLinksError { message })
        },
    );
    let data = meta_language::read_structured_data_lino(include_str!(
        "../../../parity/fixtures/rust-constant-patterns.lino"
    ))
    .unwrap();
    for row in data["premises"].as_array().unwrap() {
        let args = &row["args"];
        let strings = || {
            row["expected"]
                .as_array()
                .unwrap()
                .iter()
                .map(|value| value.as_str().unwrap().to_owned())
                .collect::<Vec<_>>()
        };
        match row["operation"].as_str().unwrap() {
            "readRustConstantPrimitiveAliasOwner" => assert_eq!(
                rules::read_rust_constant_primitive_alias_owner(
                    text(args, 0),
                    text(args, 1),
                    text(args, 2),
                    args[3].as_f64().unwrap(),
                    args[4].as_f64().unwrap(),
                    text(args, 5),
                    args[6].as_bool().unwrap()
                ),
                row["expected"].as_str().unwrap()
            ),

            "evaluateRustConstantIntegerOperation" => assert_eq!(
                rules::evaluate_rust_constant_integer_operation(
                    text(args, 0),
                    text(args, 1),
                    text(args, 2),
                    text(args, 3),
                    text(args, 4),
                    args[5].as_f64().unwrap()
                ),
                strings()
            ),
            "evaluateRustConstantBooleanOperation" => assert_eq!(
                rules::evaluate_rust_constant_boolean_operation(
                    text(args, 0),
                    args[1].as_bool().unwrap(),
                    args[2].as_bool().unwrap()
                ),
                strings()
            ),
            "requireRustConstantBooleanRight" => assert_eq!(
                rules::require_rust_constant_boolean_right(
                    text(args, 0),
                    args[1].as_bool().unwrap()
                ),
                row["expected"].as_bool().unwrap()
            ),
            "readRustConstantPatternEvaluationError" => assert_eq!(
                rules::read_rust_constant_pattern_evaluation_error(
                    args[0].as_f64().unwrap(),
                    args[1].as_f64().unwrap(),
                    args[2].as_bool().unwrap()
                ),
                row["expected"].as_str().unwrap()
            ),
            unknown => panic!("uncovered constant operation {unknown}"),
        }
    }
    for row in data["refusals"].as_array().unwrap() {
        let source = row["source"].as_str().unwrap();
        let error = meta_language::translation::rust::parse_rust(source).unwrap_err();
        assert_eq!(error.kind.as_str(), row["kind"].as_str().unwrap());
        assert_eq!(error.reason, row["reason"].as_str().unwrap());
        let public = meta_language::create_rust_source_owner_program_links(source).unwrap_err();
        assert!(
            public.message.contains(row["reason"].as_str().unwrap()),
            "{}",
            public.message
        );
    }
}
