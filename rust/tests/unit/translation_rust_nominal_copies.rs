//! Deferred nominal Copy originals, native products and shared source plans.
#[path = "../../src/translation/rust_nominal_copy_rules.rs"]
mod rules;
use serde_json::Value;
fn text(args: &Value, index: usize) -> &str {
    args[index].as_str().unwrap()
}
#[test]
fn nominal_copy_uses_actual_declarations_and_all_fields() {
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
        "../experiments/rust-records/nominal-copies.mjs",
        "Rust actual nominal Copy declaration graph",
        || {
            meta_language::create_source_definition_program_links(
                "meta-language/source-program/rust-nominal-copy-program",
            )
            .map_err(|message| meta_language::TypedProgramLinksError { message })
        },
    );
    let data = meta_language::read_structured_data_lino(include_str!(
        "../../../parity/fixtures/rust-nominal-copies.lino"
    ))
    .unwrap();
    for row in data["premises"].as_array().unwrap() {
        let args = &row["args"];
        match row["operation"].as_str().unwrap() {
            "acceptRustNominalCopyFieldLeaf" => {
                let wrappers = args[1]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|value| value.as_str().unwrap().to_owned())
                    .collect::<Vec<_>>();
                assert_eq!(
                    rules::accept_rust_nominal_copy_field_leaf(
                        text(args, 0),
                        &wrappers,
                        args[2].as_bool().unwrap()
                    ),
                    row["expected"].as_bool().unwrap()
                );
            }
            "planRustNominalCopyOwners" => {
                let strings = |at: usize| {
                    args[at]
                        .as_array()
                        .unwrap()
                        .iter()
                        .map(|value| value.as_str().unwrap().to_owned())
                        .collect::<Vec<_>>()
                };
                let numbers = |at: usize| {
                    args[at]
                        .as_array()
                        .unwrap()
                        .iter()
                        .map(|value| value.as_f64().unwrap())
                        .collect::<Vec<_>>()
                };
                let flags = |at: usize| {
                    args[at]
                        .as_array()
                        .unwrap()
                        .iter()
                        .map(|value| value.as_bool().unwrap())
                        .collect::<Vec<_>>()
                };
                let expected = row["expected"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|value| value.as_f64().unwrap())
                    .collect::<Vec<_>>();
                assert_eq!(
                    rules::plan_rust_nominal_copy_owners(
                        &strings(0),
                        &flags(1),
                        &flags(2),
                        &numbers(3),
                        &numbers(4),
                        &numbers(5),
                        &flags(6)
                    ),
                    expected
                );
            }
            unknown => panic!("uncovered reference token decision {unknown}"),
        }
    }
    for row in data["refusals"].as_array().unwrap() {
        let source = row["source"].as_str().unwrap();
        let error = meta_language::translation::rust::parse_rust(source).unwrap_err();
        assert_eq!(error.kind.as_str(), row["kind"].as_str().unwrap());
        assert_eq!(error.reason, row["reason"].as_str().unwrap());
        let expected: meta_language::translation::Span =
            serde_json::from_value(row["span"].clone()).unwrap();
        assert_eq!(error.span, Some(expected));
        let public = meta_language::create_rust_source_owner_program_links(source).unwrap_err();
        assert!(
            public.message.contains(row["reason"].as_str().unwrap()),
            "{}",
            public.message
        );
    }
}
