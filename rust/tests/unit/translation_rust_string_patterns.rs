//! Deferred original/native products and every shared literal/owner premise.
#[path = "../../src/translation/rust_string_pattern_rules.rs"]
mod rules;

#[test]
fn complete_rust_string_pattern_products_preserve_originals_and_every_premise() {
    super::translation_string_array_copy_sort::require_rust_profile();
    assert!(
        cfg!(target_pointer_width = "64"),
        "uncovered source pointer width"
    );
    super::translation_rust_records::assert_nominal_translation_product(
        "../experiments/rust-records/string-patterns.mjs",
        "ordinary Rust borrowed string literal patterns",
        || {
            meta_language::create_source_definition_program_links(
                "meta-language/source-program/rust-string-pattern-program",
            )
            .map_err(|message| meta_language::TypedProgramLinksError { message })
        },
    );
    let fixture = meta_language::read_structured_data_lino(include_str!(
        "../../../parity/fixtures/rust-string-patterns.lino"
    ))
    .unwrap();
    for row in fixture["tokens"].as_array().unwrap() {
        assert_eq!(
            rules::accept_rust_source_string_pattern_token(row["token"].as_str().unwrap()),
            row["expected"].as_bool().unwrap()
        );
    }
    for row in fixture["premises"].as_array().unwrap() {
        assert_eq!(
            rules::read_rust_source_string_pattern_error(
                row["kind"].as_str().unwrap(),
                row["model"].as_str().unwrap(),
                row["proven"].as_bool().unwrap(),
                row["layers"].as_f64().unwrap(),
                row["references"].as_bool().unwrap(),
                row["ownerKind"].as_str().unwrap(),
                row["ownerPath"].as_str().unwrap(),
                row["argumentsCount"].as_f64().unwrap(),
                row["collecting"].as_bool().unwrap()
            ),
            row["expected"].as_str().unwrap()
        );
    }
}
