//! Deferred complete original/product oracle and independent shared decisions.
#[path = "../../src/translation/abrupt_result_rules.rs"]
mod abrupt;
#[path = "../../src/translation/rust_panic_value_rules.rs"]
mod panic_rules;
#[path = "../../src/translation/rust_array_lookup_rules.rs"]
mod rules;
use serde_json::{Value, json};
fn text(args: &Value, index: usize) -> &str {
    args[index].as_str().unwrap()
}
#[test]
fn copied_lookup_panics_retain_bounds_message_and_source_order() {
    super::translation_string_array_copy_sort::require_rust_profile();
    assert!(
        cfg!(target_pointer_width = "32") || cfg!(target_pointer_width = "64"),
        "uncovered pointer width"
    );
    super::translation_rust_records::assert_nominal_translation_product(
        "../experiments/rust-records/array-lookup-panics.mjs",
        "ordinary Rust copied lookup panic consumers",
        || {
            meta_language::create_source_definition_program_links(
                "meta-language/source-program/rust-array-lookup-program",
            )
            .map_err(|message| meta_language::TypedProgramLinksError { message })
        },
    );
    let data = meta_language::read_structured_data_lino(include_str!(
        "../../../parity/fixtures/rust-array-lookup-panics.lino"
    ))
    .unwrap();
    for row in data["premises"].as_array().unwrap() {
        let args = &row["args"];
        let actual = match row["operation"].as_str().unwrap() {
            "readRustArrayLookupPanicValueError" => {
                json!(rules::read_rust_array_lookup_panic_value_error(
                    text(args, 0),
                    text(args, 1),
                    text(args, 2),
                    args[3].as_f64().unwrap(),
                    args[4].as_bool().unwrap(),
                    args[5].as_bool().unwrap()
                ))
            }
            "readRustPanicValueOperation" => json!(panic_rules::read_rust_panic_value_operation()),
            "acceptLinkedRustPanicValue" => json!(panic_rules::accept_linked_rust_panic_value(
                text(args, 0),
                text(args, 1)
            )),
            "readRustPanicValueArgumentError" => {
                json!(panic_rules::read_rust_panic_value_argument_error(
                    text(args, 0),
                    args[1].as_f64().unwrap(),
                    args[2].as_bool().unwrap(),
                    text(args, 3)
                ))
            }
            "renderRustPanicValueExpression" => json!(
                panic_rules::render_rust_panic_value_expression(text(args, 0), text(args, 1))
            ),
            "readAbruptResultCoercionForm" => json!(abrupt::read_abrupt_result_coercion_form(
                text(args, 0),
                text(args, 1)
            )),
            "testRustArrayLookupPanicConsumer" => {
                json!(rules::test_rust_array_lookup_panic_consumer(text(args, 0)))
            }
            "readRustArrayLookupPanicMessageError" => json!(
                rules::read_rust_array_lookup_panic_message_error(text(args, 0), text(args, 1))
            ),
            "readRustArrayLookupPanicMessage" => json!(
                rules::read_rust_array_lookup_panic_message(text(args, 0), text(args, 1))
            ),
            "readRustArrayLookupConsumerArity" => {
                json!(rules::read_rust_array_lookup_consumer_arity(
                    text(args, 0),
                    args[1].as_bool().unwrap()
                ))
            }
            "planRustArrayLookupConsumer" => {
                json!(rules::plan_rust_array_lookup_consumer(text(args, 0)))
            }
            unknown => panic!("uncovered lookup panic decision {unknown}"),
        };
        assert_eq!(actual, row["expected"], "{row}");
    }
}
