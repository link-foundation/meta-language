//! Deferred integer range originals, native products and shared source plans.
#[path = "../../src/translation/rust_integer_range_rules.rs"]
mod rules;
use serde_json::Value;
fn text(args: &Value, index: usize) -> &str {
    args[index].as_str().unwrap()
}
#[test]
fn integer_ranges_retain_exact_domains_and_source_order() {
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
        "../experiments/rust-records/integer-ranges.mjs",
        "Rust exact integer range pattern domains",
        || {
            meta_language::create_source_definition_program_links(
                "meta-language/source-program/rust-integer-range-program",
            )
            .map_err(|message| meta_language::TypedProgramLinksError { message })
        },
    );
    let data = meta_language::read_structured_data_lino(include_str!(
        "../../../parity/fixtures/rust-integer-ranges.lino"
    ))
    .unwrap();
    for row in data["premises"].as_array().unwrap() {
        let args = &row["args"];
        let strings = |at: usize| {
            args[at]
                .as_array()
                .unwrap()
                .iter()
                .map(|value| value.as_str().unwrap().to_owned())
                .collect::<Vec<_>>()
        };
        let expected_numbers = || {
            row["expected"]
                .as_array()
                .unwrap()
                .iter()
                .map(|value| value.as_f64().unwrap())
                .collect::<Vec<_>>()
        };
        match row["operation"].as_str().unwrap() {
            "testRustIntegerRangeRestFollower" => assert_eq!(
                rules::test_rust_integer_range_rest_follower(text(args, 0)),
                row["expected"].as_bool().unwrap()
            ),
            "testRustIntegerRangeCanonicalDecimal" => assert_eq!(
                rules::test_rust_integer_range_canonical_decimal(text(args, 0)),
                row["expected"].as_bool().unwrap()
            ),
            "compareRustIntegerRangeDecimals" => assert_eq!(
                rules::compare_rust_integer_range_decimals(text(args, 0), text(args, 1)),
                row["expected"].as_f64().unwrap()
            ),
            "readRustIntegerRangeSuccessor" => assert_eq!(
                rules::read_rust_integer_range_successor(text(args, 0)),
                row["expected"].as_str().unwrap()
            ),
            "readRustIntegerRangeCuts" => assert_eq!(
                rules::read_rust_integer_range_cuts(
                    text(args, 0),
                    text(args, 1),
                    &strings(2),
                    &strings(3)
                ),
                row["expected"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|value| value.as_str().unwrap().to_owned())
                    .collect::<Vec<_>>()
            ),
            "planRustIntegerRangeRows" => assert_eq!(
                rules::plan_rust_integer_range_rows(
                    &strings(0),
                    &strings(1),
                    &strings(2),
                    &strings(3)
                ),
                expected_numbers()
            ),
            "readRustIntegerRangeSourceError" => assert_eq!(
                rules::read_rust_integer_range_source_error(
                    args[0].as_bool().unwrap(),
                    text(args, 1),
                    args[2].as_bool().unwrap(),
                    args[3].as_bool().unwrap()
                ),
                row["expected"].as_str().unwrap()
            ),
            unknown => panic!("uncovered integer range premise {unknown}"),
        }
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
