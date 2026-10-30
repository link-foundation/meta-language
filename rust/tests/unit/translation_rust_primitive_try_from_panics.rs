//! Deferred version-bounded originals, native products and shared decisions.
#[path = "../../src/translation/rust_panic_value_rules.rs"]
mod panic_rules;
#[path = "../../src/translation/rust_primitive_try_from_rules.rs"]
mod rules;
use serde_json::{Value, json};
fn text(args: &Value, index: usize) -> &str {
    args[index].as_str().unwrap()
}
#[test]
fn standard_integer_result_panics_retain_current_debug_values_and_operand_order() {
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
        version[0] == "1" && (97..100).contains(&version[1].parse::<u32>().unwrap()),
        "uncovered Rust Debug model {}; require >=1.97.0 <1.100.0",
        compiler.stdout
    );
    assert!(
        cfg!(target_pointer_width = "32") || cfg!(target_pointer_width = "64"),
        "uncovered pointer width"
    );
    super::translation_rust_records::assert_nominal_translation_product(
        "../experiments/rust-records/primitive-try-from-panics.mjs",
        "standard integer Result panic consumers",
        || {
            meta_language::create_source_definition_program_links(
                "meta-language/source-program/rust-primitive-try-from-program",
            )
            .map_err(|message| meta_language::TypedProgramLinksError { message })
        },
    );
    let data = meta_language::read_structured_data_lino(include_str!(
        "../../../parity/fixtures/rust-primitive-try-from-panics.lino"
    ))
    .unwrap();
    for row in data["premises"].as_array().unwrap() {
        let args = &row["args"];
        let actual = match row["operation"].as_str().unwrap() {
            "readRustPrimitiveTryFromConsumerArity" => json!(
                rules::read_rust_primitive_try_from_consumer_arity(text(args, 0))
            ),
            "readRustPrimitiveTryFromResultOwner" => {
                json!(rules::read_rust_primitive_try_from_result_owner(
                    text(args, 0),
                    text(args, 1),
                    text(args, 2)
                ))
            }
            "testRustPrimitiveTryFromPanicConsumer" => json!(
                rules::test_rust_primitive_try_from_panic_consumer(text(args, 0))
            ),
            "readRustPrimitiveTryFromFailureDebug" => json!(
                rules::read_rust_primitive_try_from_failure_debug(text(args, 0))
            ),
            "readRustPrimitiveTryFromPanicPrefix" => json!(
                rules::read_rust_primitive_try_from_panic_prefix(text(args, 0))
            ),
            "planRustPrimitiveTryFromFailureDebug" => json!(
                rules::plan_rust_primitive_try_from_failure_debug(args[0].as_f64().unwrap())
            ),
            "planRustPrimitiveTryFromConsumer" => {
                json!(rules::plan_rust_primitive_try_from_consumer(
                    text(args, 0),
                    args[1].as_f64().unwrap()
                ))
            }
            "readRustPrimitiveTryFromPanicVersionRange" => {
                json!(rules::read_rust_primitive_try_from_panic_version_range())
            }
            "readRustPanicMessageSourceError" => {
                json!(panic_rules::read_rust_panic_message_source_error(
                    text(args, 0),
                    text(args, 1),
                    text(args, 2),
                    args[3].as_f64().unwrap(),
                    args[4].as_bool().unwrap(),
                    args[5].as_bool().unwrap()
                ))
            }
            unknown => panic!("uncovered Result panic decision {unknown}"),
        };
        assert_eq!(actual, row["expected"], "{row}");
    }
}
