//! Deferred original/native products and every shared literal/owner premise.
#[path = "../../src/translation/rust_byte_pattern_rules.rs"]
mod rules;

#[test]
fn complete_rust_byte_pattern_products_preserve_originals_and_every_premise() {
    super::translation_string_array_copy_sort::require_rust_profile();
    assert!(
        cfg!(target_pointer_width = "32") || cfg!(target_pointer_width = "64"),
        "uncovered source pointer width"
    );
    super::translation_rust_records::assert_nominal_translation_product(
        "../experiments/rust-records/byte-patterns.mjs",
        "ordinary Rust unsigned byte literal patterns",
        || {
            meta_language::create_source_definition_program_links(
                "meta-language/source-program/rust-byte-pattern-program",
            )
            .map_err(|message| meta_language::TypedProgramLinksError { message })
        },
    );
    let fixture = meta_language::read_structured_data_lino(include_str!(
        "../../../parity/fixtures/rust-byte-patterns.lino"
    ))
    .unwrap();
    for row in fixture["tokens"].as_array().unwrap() {
        assert_eq!(
            rules::accept_rust_source_byte_pattern_token(row["token"].as_str().unwrap()),
            row["expected"].as_bool().unwrap()
        );
    }
    for row in fixture["types"].as_array().unwrap() {
        assert_eq!(
            rules::read_rust_source_byte_pattern_type_error(
                row["kind"].as_str().unwrap(),
                row["bits"].as_f64().unwrap(),
                row["signed"].as_bool().unwrap()
            ),
            row["expected"].as_str().unwrap()
        );
    }
    for row in fixture["aliases"].as_array().unwrap() {
        assert_eq!(
            rules::accept_rust_source_literal_pattern_alias(row["form"].as_str().unwrap()),
            row["expected"].as_bool().unwrap()
        );
    }
    for row in fixture["inference"].as_array().unwrap() {
        let patterns = row["patterns"]
            .as_array()
            .unwrap()
            .iter()
            .map(|value| value.as_str().unwrap().to_owned())
            .collect::<Vec<_>>();
        assert_eq!(
            rules::accept_rust_source_byte_literal_inference(
                row["language"].as_str().unwrap(),
                row["kind"].as_str().unwrap(),
                row["typed"].as_bool().unwrap(),
                &patterns
            ),
            row["expected"].as_bool().unwrap()
        );
    }
}
