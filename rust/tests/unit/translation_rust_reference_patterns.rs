//! Deferred closure inference originals, native products and shared source plans.
#[path = "../../src/translation/rust_type_token_rules.rs"]
mod rules;
use serde_json::Value;
fn text(args: &Value, index: usize) -> &str {
    args[index].as_str().unwrap()
}
#[test]
fn reference_patterns_preserve_actual_layers_and_matching() {
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
        "../experiments/rust-records/reference-patterns.mjs",
        "immutable Rust reference pattern contexts",
        || {
            meta_language::create_source_definition_program_links(
                "meta-language/source-program/rust-type-token-program",
            )
            .map_err(|message| meta_language::TypedProgramLinksError { message })
        },
    );
    let data = meta_language::read_structured_data_lino(include_str!(
        "../../../parity/fixtures/rust-reference-patterns.lino"
    ))
    .unwrap();
    for row in data["premises"].as_array().unwrap() {
        let args = &row["args"];
        match row["operation"].as_str().unwrap() {
            "readRustReferenceTokenParts" => {
                let expected = row["expected"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|value| value.as_str().unwrap().to_owned())
                    .collect::<Vec<_>>();
                assert_eq!(
                    rules::read_rust_reference_token_parts(text(args, 0)),
                    expected
                );
            }
            "readRustTypeClosingTokenParts" => {
                let expected = row["expected"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|value| value.as_str().unwrap().to_owned())
                    .collect::<Vec<_>>();
                assert_eq!(
                    rules::read_rust_type_closing_token_parts(text(args, 0)),
                    expected
                );
            }
            unknown => panic!("uncovered reference token decision {unknown}"),
        }
    }
}
