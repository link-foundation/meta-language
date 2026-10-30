//! Deferred closure inference originals, native products and shared source plans.
#[path = "../../src/translation/rust_pattern_match_rules.rs"]
mod rules;
use serde_json::Value;
fn text(args: &Value, index: usize) -> &str {
    args[index].as_str().unwrap()
}
#[test]
fn pattern_macros_reuse_actual_source_matches() {
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
        "../experiments/rust-records/pattern-matches.mjs",
        "Rust standard pattern macro contexts",
        || {
            meta_language::create_source_definition_program_links(
                "meta-language/source-program/rust-pattern-match-program",
            )
            .map_err(|message| meta_language::TypedProgramLinksError { message })
        },
    );
    let data = meta_language::read_structured_data_lino(include_str!(
        "../../../parity/fixtures/rust-pattern-matches.lino"
    ))
    .unwrap();
    for row in data["premises"].as_array().unwrap() {
        let args = &row["args"];
        match row["operation"].as_str().unwrap() {
            "acceptRustPatternMatchMacroOwner" => assert_eq!(
                rules::accept_rust_pattern_match_macro_owner(
                    text(args, 0),
                    args[1].as_bool().unwrap()
                ),
                row["expected"].as_bool().unwrap()
            ),
            "planRustPatternMatchRows" => {
                let kinds = args[0]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|value| value.as_str().unwrap().to_owned())
                    .collect::<Vec<_>>();
                let values = args[1]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|value| value.as_str().unwrap().to_owned())
                    .collect::<Vec<_>>();
                let expected = row["expected"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|value| value.as_f64().unwrap())
                    .collect::<Vec<_>>();
                assert_eq!(
                    rules::plan_rust_pattern_match_rows(
                        &kinds,
                        &values,
                        args[2].as_f64().unwrap(),
                        text(args, 3)
                    ),
                    expected
                );
            }
            "readRustPatternMatchTokenValue" => assert_eq!(
                rules::read_rust_pattern_match_token_value(args[0].as_f64().unwrap()),
                row["expected"].as_str().unwrap()
            ),
            "readRustPatternMatchTokenKind" => assert_eq!(
                rules::read_rust_pattern_match_token_kind(args[0].as_f64().unwrap()),
                row["expected"].as_str().unwrap()
            ),
            unknown => panic!("uncovered reference token decision {unknown}"),
        }
    }
}
