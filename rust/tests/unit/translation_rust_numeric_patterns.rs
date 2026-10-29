//! Deferred original/native products and every shared literal/owner premise.
#[path = "../../src/translation/rust_numeric_pattern_rules.rs"]
mod rules;

#[test]
fn complete_rust_numeric_pattern_products_preserve_originals_and_every_premise() {
    super::translation_string_array_copy_sort::require_rust_profile();
    assert!(
        cfg!(target_pointer_width = "64"),
        "uncovered source pointer width"
    );
    super::translation_rust_records::assert_nominal_translation_product(
        "../experiments/rust-records/numeric-patterns.mjs",
        "ordinary Rust suffixed numeric literal patterns",
        || {
            meta_language::create_source_definition_program_links(
                "meta-language/source-program/rust-numeric-pattern-program",
            )
            .map_err(|message| meta_language::TypedProgramLinksError { message })
        },
    );
    let fixture = meta_language::read_structured_data_lino(include_str!(
        "../../../parity/fixtures/rust-numeric-patterns.lino"
    ))
    .unwrap();
    for row in fixture["owners"].as_array().unwrap() {
        assert_eq!(
            rules::read_rust_source_numeric_pattern_owner(
                row["suffix"].as_str().unwrap(),
                row["floating"].as_bool().unwrap()
            ),
            row["expected"].as_str().unwrap()
        );
    }
    for row in fixture["lexical"].as_array().unwrap() {
        assert_eq!(
            rules::read_rust_source_numeric_pattern_lexical_error(
                row["suffix"].as_str().unwrap(),
                row["floating"].as_bool().unwrap()
            ),
            row["expected"].as_str().unwrap()
        );
    }
    for row in fixture["ownerPremises"].as_array().unwrap() {
        assert_eq!(
            rules::read_rust_source_numeric_pattern_owner_error(
                row["actual"].as_str().unwrap(),
                row["expectedOwner"].as_str().unwrap(),
                row["typed"].as_bool().unwrap(),
                row["collecting"].as_bool().unwrap()
            ),
            row["expected"].as_str().unwrap()
        );
    }
    for row in fixture["types"].as_array().unwrap() {
        assert_eq!(
            rules::read_rust_source_numeric_pattern_type_error(
                row["owner"].as_str().unwrap(),
                row["kind"].as_str().unwrap(),
                row["bits"].as_str().unwrap(),
                row["signed"].as_bool().unwrap()
            ),
            row["expected"].as_str().unwrap()
        );
    }
    for row in fixture["negations"].as_array().unwrap() {
        assert_eq!(
            rules::read_rust_source_numeric_pattern_negation_error(
                row["owner"].as_str().unwrap(),
                row["negative"].as_bool().unwrap()
            ),
            row["expected"].as_str().unwrap()
        );
    }
    for row in fixture["inference"].as_array().unwrap() {
        let owners = row["owners"]
            .as_array()
            .unwrap()
            .iter()
            .map(|value| value.as_str().unwrap().to_owned())
            .collect::<Vec<_>>();
        assert_eq!(
            rules::read_rust_source_numeric_pattern_inference(
                row["language"].as_str().unwrap(),
                row["kind"].as_str().unwrap(),
                row["typed"].as_bool().unwrap(),
                &owners
            ),
            row["expected"].as_str().unwrap()
        );
    }
    for row in fixture["contexts"].as_array().unwrap() {
        assert_eq!(
            rules::read_rust_source_integer_pattern_context_error(
                row["language"].as_str().unwrap(),
                row["kind"].as_str().unwrap()
            ),
            row["expected"].as_str().unwrap()
        );
    }
    for row in fixture["inferenceErrors"].as_array().unwrap() {
        let owners = row["owners"]
            .as_array()
            .unwrap()
            .iter()
            .map(|value| value.as_str().unwrap().to_owned())
            .collect::<Vec<_>>();
        assert_eq!(
            rules::read_rust_source_numeric_pattern_inference_error(
                row["language"].as_str().unwrap(),
                row["kind"].as_str().unwrap(),
                row["typed"].as_bool().unwrap(),
                &owners
            ),
            row["expected"].as_str().unwrap()
        );
    }
    for row in fixture["aliases"].as_array().unwrap() {
        assert_eq!(
            rules::accept_rust_source_numeric_pattern_alias(row["form"].as_str().unwrap()),
            row["expected"].as_bool().unwrap()
        );
    }
}
