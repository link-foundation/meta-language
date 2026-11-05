//! Deferred original products and every declared shared macro decision.
#[path = "../../src/translation/rust_panic_value_rules.rs"]
mod rules;
use serde_json::{Value, json};
fn text(args: &Value, index: usize) -> &str {
    args[index].as_str().unwrap()
}
#[test]
fn formatted_panics_preserve_original_values_order_and_complete_messages() {
    super::translation_string_array_copy_sort::require_rust_profile();
    assert!(
        cfg!(target_pointer_width = "32") || cfg!(target_pointer_width = "64"),
        "uncovered pointer width"
    );
    super::translation_rust_records::assert_nominal_translation_product(
        "../experiments/rust-records/formatted-panics.mjs",
        "ordinary Rust formatted panic macros",
        || {
            meta_language::create_source_definition_program_links(
                "meta-language/source-program/rust-panic-value-program",
            )
            .map_err(|message| meta_language::TypedProgramLinksError { message })
        },
    );
    let data = meta_language::read_structured_data_lino(include_str!(
        "../../../parity/fixtures/rust-formatted-panics.lino"
    ))
    .unwrap();
    for row in data["premises"].as_array().unwrap() {
        let args = &row["args"];
        let actual = match row["operation"].as_str().unwrap() {
            "acceptRustPanicFormattingMacro" => json!(rules::accept_rust_panic_formatting_macro(
                text(args, 0),
                args[1].as_bool().unwrap()
            )),
            "acceptRustPanicFormattingValue" => json!(rules::accept_rust_panic_formatting_value(
                text(args, 0),
                args[1].as_bool().unwrap(),
                text(args, 2),
                args[3].as_f64().unwrap(),
                args[4].as_bool().unwrap()
            )),
            "readRustPanicFormattingPrefix" => {
                json!(rules::read_rust_panic_formatting_prefix(text(args, 0)))
            }
            "readRustPanicCompletionKind" => json!(rules::read_rust_panic_completion_kind(
                text(args, 0),
                text(args, 1)
            )),
            unknown => panic!("uncovered formatted panic decision {unknown}"),
        };
        assert_eq!(actual, row["expected"], "{row}");
    }
}
