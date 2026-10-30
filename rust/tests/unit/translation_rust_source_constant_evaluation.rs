//! Deferred source constant originals, native products and shared source plans.
#[path = "../../src/translation/rust_source_constant_evaluation_rules.rs"]
mod rules;
use serde_json::Value;
fn text(args: &Value, index: usize) -> &str {
    args[index].as_str().unwrap()
}
#[test]
fn source_constants_retain_checked_values_and_declaration_owners() {
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
        "../experiments/rust-records/source-constant-evaluation.mjs",
        "Rust bounded source constant scalar evaluation",
        || {
            meta_language::create_source_definition_program_links(
                "meta-language/source-program/rust-source-constant-evaluation-program",
            )
            .map_err(|message| meta_language::TypedProgramLinksError { message })
        },
    );
    let data = meta_language::read_structured_data_lino(include_str!(
        "../../../parity/fixtures/rust-source-constant-evaluation.lino"
    ))
    .unwrap();
    for row in data["premises"].as_array().unwrap() {
        let args = &row["args"];
        match row["operation"].as_str().unwrap() {
            "readRustSourceConstantIntegerLiteral" => assert_eq!(
                rules::read_rust_source_constant_integer_literal(
                    text(args, 0),
                    args[1].as_bool().unwrap()
                ),
                row["expected"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|value| value.as_str().unwrap().to_owned())
                    .collect::<Vec<_>>()
            ),
            "readRustSourceConstantInitializerBoundary" => {
                let strings = |at: usize| {
                    args[at]
                        .as_array()
                        .unwrap()
                        .iter()
                        .map(|value| value.as_str().unwrap().to_owned())
                        .collect::<Vec<_>>()
                };
                assert_eq!(
                    rules::read_rust_source_constant_initializer_boundary(
                        &strings(0),
                        &strings(1),
                        args[2].as_f64().unwrap()
                    ),
                    row["expected"].as_f64().unwrap()
                );
            }
            unknown => panic!("uncovered source constant premise {unknown}"),
        }
    }
    for row in data["packets"].as_array().unwrap() {
        let facts = &row["facts"];
        let strings = |name: &str| {
            facts[name]
                .as_array()
                .unwrap()
                .iter()
                .map(|value| value.as_str().unwrap().to_owned())
                .collect::<Vec<_>>()
        };
        let numbers = |name: &str| {
            facts[name]
                .as_array()
                .unwrap()
                .iter()
                .map(|value| value.as_f64().unwrap())
                .collect::<Vec<_>>()
        };
        let expected = row["expected"]
            .as_array()
            .unwrap()
            .iter()
            .map(|value| value.as_str().unwrap().to_owned())
            .collect::<Vec<_>>();
        assert_eq!(
            rules::evaluate_rust_source_constant(
                &strings("texts"),
                &numbers("starts"),
                &numbers("counts"),
                &strings("labels"),
                &numbers("targets"),
                facts["root"].as_f64().unwrap(),
                row["owner"].as_str().unwrap()
            ),
            expected
        );
    }
    for row in data["refusals"].as_array().unwrap() {
        let source = row["source"].as_str().unwrap();
        let error = meta_language::translation::rust::parse_rust(source).unwrap_err();
        assert_eq!(error.kind.as_str(), row["kind"].as_str().unwrap());
        assert_eq!(error.reason, row["reason"].as_str().unwrap());
        let public = meta_language::create_rust_source_owner_program_links(source).unwrap_err();
        assert!(
            public.message.contains(row["reason"].as_str().unwrap()),
            "{}",
            public.message
        );
    }
}
