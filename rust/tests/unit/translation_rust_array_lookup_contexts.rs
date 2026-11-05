//! Deferred closure inference originals, native products and shared source plans.
#[path = "../../src/translation/rust_copy_closure_rules.rs"]
mod rules;
use serde_json::Value;
fn text(args: &Value, index: usize) -> &str {
    args[index].as_str().unwrap()
}
#[test]
fn lookup_contexts_preserve_actual_result_types_and_source_order() {
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
        "../experiments/rust-records/array-lookup-contexts.mjs",
        "checked deferred closure integer operations",
        || {
            meta_language::create_source_definition_program_links(
                "meta-language/source-program/rust-array-lookup-program",
            )
            .map_err(|message| meta_language::TypedProgramLinksError { message })
        },
    );
    let data = meta_language::read_structured_data_lino(include_str!(
        "../../../parity/fixtures/rust-array-lookup-contexts.lino"
    ))
    .unwrap();
    for row in data["premises"].as_array().unwrap() {
        let args = &row["args"];
        match row["operation"].as_str().unwrap() {
            "readRustCopyClosureParameterHeaderError" => {
                let names = args[1]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|value| value.as_str().unwrap().to_owned())
                    .collect::<Vec<_>>();
                let copies = args[2]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|value| value.as_bool().unwrap())
                    .collect::<Vec<_>>();
                assert_eq!(
                    rules::read_rust_copy_closure_parameter_header_error(
                        args[0].as_bool().unwrap(),
                        &names,
                        &copies
                    ),
                    row["expected"].as_str().unwrap()
                );
            }
            "testRustCopyClosureDeferredResult" => assert_eq!(
                rules::test_rust_copy_closure_deferred_result(
                    args[0].as_bool().unwrap(),
                    args[1].as_bool().unwrap(),
                    args[2].as_bool().unwrap()
                ),
                row["expected"].as_bool().unwrap()
            ),
            "readRustCopyClosureDeferredSourceError" => assert_eq!(
                rules::read_rust_copy_closure_deferred_source_error(
                    text(args, 0),
                    args[1].as_f64().unwrap(),
                    args[2].as_f64().unwrap(),
                    args[3].as_bool().unwrap()
                ),
                row["expected"].as_str().unwrap()
            ),
            "readRustCopyClosureDeferredResultError" => assert_eq!(
                rules::read_rust_copy_closure_deferred_result_error(
                    text(args, 0),
                    args[1].as_bool().unwrap(),
                    args[2].as_bool().unwrap(),
                    args[3].as_bool().unwrap()
                ),
                row["expected"].as_str().unwrap()
            ),
            "readRustCopyClosureResultContextRole" => assert_eq!(
                rules::read_rust_copy_closure_result_context_role(text(args, 0), text(args, 1)),
                row["expected"].as_str().unwrap()
            ),
            "testRustCopyClosureResultBinding" => {
                let path = args[0]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|value| value.as_str().unwrap().to_owned())
                    .collect::<Vec<_>>();
                assert_eq!(
                    rules::test_rust_copy_closure_result_binding(&path, text(args, 1)),
                    row["expected"].as_bool().unwrap()
                );
            }
            "readRustCopyClosureLiteralResultError" => assert_eq!(
                rules::read_rust_copy_closure_literal_result_error(
                    text(args, 0),
                    args[1].as_bool().unwrap(),
                    args[2].as_bool().unwrap(),
                    text(args, 3),
                    args[4].as_bool().unwrap()
                ),
                row["expected"].as_str().unwrap()
            ),
            unknown => panic!("uncovered deferred closure decision {unknown}"),
        }
    }
}
