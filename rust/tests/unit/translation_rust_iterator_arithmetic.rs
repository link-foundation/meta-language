//! Deferred standard numeric originals and every source-owned arithmetic decision.
#[path = "../../src/translation/rust_iterator_arithmetic_rules.rs"]
mod rules;
use serde_json::{Value, json};
fn text(args: &Value, index: usize) -> &str {
    args[index].as_str().unwrap()
}
fn number(args: &Value, index: usize) -> f64 {
    args[index].as_f64().unwrap()
}
fn boolean(args: &Value, index: usize) -> bool {
    args[index].as_bool().unwrap()
}
#[test]
fn numeric_iterator_arithmetic_preserves_originals_owners_and_ieee_identities() {
    super::translation_string_array_copy_sort::require_rust_profile();
    assert!(
        cfg!(target_pointer_width = "64"),
        "uncovered numeric source pointer profile: expected64; additional profiles remain requirements"
    );
    super::translation_rust_records::assert_nominal_translation_product(
        "../experiments/rust-records/iterator-arithmetic.mjs",
        "standard Rust numeric iterator arithmetic",
        || {
            meta_language::create_source_definition_program_links(
                "meta-language/source-program/rust-iterator-arithmetic-program",
            )
            .map_err(|message| meta_language::TypedProgramLinksError { message })
        },
    );
    let data = meta_language::read_structured_data_lino(include_str!(
        "../../../parity/fixtures/rust-iterator-arithmetic.lino"
    ))
    .unwrap();
    for row in data["premises"].as_array().unwrap() {
        let args = &row["args"];
        let actual = match row["operation"].as_str().unwrap() {
            "acceptRustIteratorArithmeticConsumer" => json!(
                rules::accept_rust_iterator_arithmetic_consumer(text(args, 0))
            ),
            "planRustIteratorArithmeticSeed" => json!(rules::plan_rust_iterator_arithmetic_seed(
                text(args, 0),
                text(args, 1)
            )),
            "planRustIteratorArithmeticTerminal" => {
                json!(rules::plan_rust_iterator_arithmetic_terminal(text(args, 0)))
            }
            "readRustIteratorArithmeticError" => json!(rules::read_rust_iterator_arithmetic_error(
                text(args, 0),
                boolean(args, 1),
                boolean(args, 2),
                number(args, 3),
                boolean(args, 4),
                number(args, 5),
                boolean(args, 6),
                boolean(args, 7)
            )),
            unknown => panic!("uncovered arithmetic decision {unknown}"),
        };
        assert_eq!(actual, row["expected"], "{row}");
    }
}
