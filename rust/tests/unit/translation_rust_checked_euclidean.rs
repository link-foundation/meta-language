//! Deferred Euclidean originals, native products and shared source plans.
#[path = "../../src/translation/rust_checked_integer_rules.rs"]
mod rules;
use serde_json::{Value, json};
fn text(args: &Value, index: usize) -> &str {
    args[index].as_str().unwrap()
}
#[test]
fn checked_euclidean_integer_results_preserve_values_and_source_order() {
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
        "../experiments/rust-records/checked-euclidean.mjs",
        "checked Euclidean integer operations",
        || {
            meta_language::create_source_definition_program_links(
                "meta-language/source-program/rust-checked-integer-program",
            )
            .map_err(|message| meta_language::TypedProgramLinksError { message })
        },
    );
    let data = meta_language::read_structured_data_lino(include_str!(
        "../../../parity/fixtures/rust-checked-euclidean.lino"
    ))
    .unwrap();
    for row in data["premises"].as_array().unwrap() {
        let args = &row["args"];
        match row["operation"].as_str().unwrap() {
            "readRustCheckedIntegerArity" => assert_eq!(
                rules::read_rust_checked_integer_arity(text(args, 0), args[1].as_bool().unwrap()),
                row["expected"].as_f64().unwrap()
            ),
            "planRustCheckedIntegerCondition" => assert_eq!(
                json!(rules::plan_rust_checked_integer_condition(
                    text(args, 0),
                    args[1].as_bool().unwrap()
                )),
                row["expected"]
            ),
            "planRustCheckedIntegerSuccess" => assert_eq!(
                json!(rules::plan_rust_checked_integer_success(
                    text(args, 0),
                    args[1].as_bool().unwrap()
                )),
                row["expected"]
            ),
            unknown => panic!("uncovered Euclidean decision {unknown}"),
        }
    }
}
