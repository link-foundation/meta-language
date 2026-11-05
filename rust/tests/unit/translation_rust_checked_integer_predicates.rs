//! Deferred Option predicate originals, native products and shared source plans.
#[path = "../../src/translation/rust_checked_integer_rules.rs"]
mod rules;
use serde_json::{Value, json};
fn text(args: &Value, index: usize) -> &str {
    args[index].as_str().unwrap()
}
#[test]
fn checked_integer_predicates_integer_results_preserve_values_and_source_order() {
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
        "../experiments/rust-records/checked-integer-predicates.mjs",
        "checked Option predicate integer operations",
        || {
            meta_language::create_source_definition_program_links(
                "meta-language/source-program/rust-checked-integer-program",
            )
            .map_err(|message| meta_language::TypedProgramLinksError { message })
        },
    );
    let data = meta_language::read_structured_data_lino(include_str!(
        "../../../parity/fixtures/rust-checked-integer-predicates.lino"
    ))
    .unwrap();
    for row in data["premises"].as_array().unwrap() {
        let args = &row["args"];
        match row["operation"].as_str().unwrap() {
            "testRustCheckedIntegerPredicateConsumer" => assert_eq!(
                rules::test_rust_checked_integer_predicate_consumer(text(args, 0)),
                row["expected"].as_bool().unwrap()
            ),
            "readRustCheckedIntegerConsumerArity" => assert_eq!(
                rules::read_rust_checked_integer_consumer_arity(text(args, 0)),
                row["expected"].as_f64().unwrap()
            ),
            "readRustCheckedIntegerResultOwner" => assert_eq!(
                rules::read_rust_checked_integer_result_owner(
                    text(args, 0),
                    text(args, 1),
                    text(args, 2)
                ),
                row["expected"].as_str().unwrap()
            ),
            "planRustCheckedIntegerConsumer" => assert_eq!(
                json!(rules::plan_rust_checked_integer_consumer(text(args, 0))),
                row["expected"]
            ),
            "readRustCheckedIntegerPredicateError" => assert_eq!(
                rules::read_rust_checked_integer_predicate_error(
                    text(args, 0),
                    args[1].as_f64().unwrap(),
                    args[2].as_f64().unwrap(),
                    args[3].as_bool().unwrap(),
                    args[4].as_bool().unwrap()
                ),
                row["expected"].as_str().unwrap()
            ),
            unknown => panic!("uncovered Option predicate decision {unknown}"),
        }
    }
}
