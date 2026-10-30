//! Deferred source-owned Result predicate translations and shared policy controls.
#[path = "../../src/translation/rust_primitive_try_from_rules.rs"]
mod rules;
use serde_json::{Value, json};
fn text(args: &Value, index: usize) -> &str {
    args[index].as_str().unwrap()
}
#[test]
fn standard_integer_result_predicates_preserve_types_and_evaluation_order() {
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
    assert!(
        cfg!(target_pointer_width = "32") || cfg!(target_pointer_width = "64"),
        "uncovered pointer width"
    );
    super::translation_rust_records::assert_nominal_translation_product(
        "../experiments/rust-records/primitive-try-from-predicates.mjs",
        "standard integer Result predicates",
        || {
            meta_language::create_source_definition_program_links(
                "meta-language/source-program/rust-primitive-try-from-program",
            )
            .map_err(|message| meta_language::TypedProgramLinksError { message })
        },
    );
    let data = meta_language::read_structured_data_lino(include_str!(
        "../../../parity/fixtures/rust-primitive-try-from-predicates.lino"
    ))
    .unwrap();
    for row in data["premises"].as_array().unwrap() {
        let args = &row["args"];
        match row["operation"].as_str().unwrap() {
            "readRustPrimitiveTryFromConsumerArity" => assert_eq!(
                rules::read_rust_primitive_try_from_consumer_arity(text(args, 0)),
                row["expected"].as_f64().unwrap()
            ),
            "readRustPrimitiveTryFromResultOwner" => assert_eq!(
                rules::read_rust_primitive_try_from_result_owner(
                    text(args, 0),
                    text(args, 1),
                    text(args, 2)
                ),
                row["expected"].as_str().unwrap()
            ),
            "planRustPrimitiveTryFromConsumer" => assert_eq!(
                json!(rules::plan_rust_primitive_try_from_consumer(
                    text(args, 0),
                    args[1].as_f64().unwrap()
                )),
                row["expected"]
            ),
            "readRustPrimitiveTryFromPredicateError" => assert_eq!(
                rules::read_rust_primitive_try_from_predicate_error(
                    text(args, 0),
                    args[1].as_f64().unwrap(),
                    args[2].as_f64().unwrap(),
                    args[3].as_bool().unwrap(),
                    args[4].as_bool().unwrap()
                ),
                row["expected"].as_str().unwrap()
            ),
            unknown => panic!("uncovered predicate decision {unknown}"),
        }
    }
}
