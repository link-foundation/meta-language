//! Deferred checked arithmetic Option panics and shared source policies.
#[path = "../../src/translation/rust_panic_value_rules.rs"]
mod panic_rules;
#[path = "../../src/translation/rust_checked_integer_rules.rs"]
mod rules;
use serde_json::{Value, json};
fn text(args: &Value, index: usize) -> &str {
    args[index].as_str().unwrap()
}
#[test]
fn checked_integer_option_panics_preserve_types_and_evaluation_order() {
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
        "../experiments/rust-records/checked-integer-panics.mjs",
        "standard integer Result checked Option panics",
        || {
            meta_language::create_source_definition_program_links(
                "meta-language/source-program/rust-checked-integer-program",
            )
            .map_err(|message| meta_language::TypedProgramLinksError { message })
        },
    );
    let data = meta_language::read_structured_data_lino(include_str!(
        "../../../parity/fixtures/rust-checked-integer-panics.lino"
    ))
    .unwrap();
    for row in data["premises"].as_array().unwrap() {
        let args = &row["args"];
        match row["operation"].as_str().unwrap() {
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
            "testRustOptionPanicConsumer" => assert_eq!(
                panic_rules::test_rust_option_panic_consumer(text(args, 0)),
                row["expected"].as_bool().unwrap()
            ),
            "readRustOptionPanicMessage" => assert_eq!(
                panic_rules::read_rust_option_panic_message(text(args, 0), text(args, 1)),
                row["expected"].as_str().unwrap()
            ),
            unknown => panic!("uncovered checked Option panic decision {unknown}"),
        }
    }
}
