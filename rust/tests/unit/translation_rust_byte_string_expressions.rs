//! Deferred paired products plus every actual byte/owner/view decision.
#[path = "../../src/translation/rust_byte_string_expression_rules.rs"]
mod rules;
fn text(value: &serde_json::Value, key: &str) -> rules::ml_string::Text {
    rules::ml_string::Text::from_text(value[key].as_str().unwrap())
}
#[test]
fn static_byte_arrays_keep_original_products_and_all_shared_premises() {
    super::translation_string_array_copy_sort::require_rust_profile();
    assert!(
        cfg!(target_pointer_width = "32") || cfg!(target_pointer_width = "64"),
        "uncovered source pointer width"
    );
    super::translation_rust_records::assert_nominal_translation_product(
        "../experiments/rust-records/byte-string-expressions.mjs",
        "ordinary Rust static byte array expressions",
        || {
            meta_language::create_source_definition_program_links(
                "meta-language/source-program/rust-byte-string-expression-program",
            )
            .map_err(|message| meta_language::TypedProgramLinksError { message })
        },
    );
    let data = meta_language::read_structured_data_lino(include_str!(
        "../../../parity/fixtures/rust-byte-string-expressions.lino"
    ))
    .unwrap();
    for row in data["payloads"].as_array().unwrap() {
        assert_eq!(
            rules::read_rust_byte_string_expression_bytes(
                &text(row, "payload"),
                &text(row, "suffix")
            ),
            row["expected"]
                .as_array()
                .unwrap()
                .iter()
                .map(|value| value.as_f64().unwrap())
                .collect::<Vec<_>>()
        );
    }
    for row in data["coercions"].as_array().unwrap() {
        assert_eq!(
            rules::accept_rust_static_byte_array_coercion(
                &text(row, "actual"),
                &text(row, "expected"),
                row["borrowed"].as_bool().unwrap()
            ),
            row["accepted"].as_bool().unwrap()
        );
    }
    for row in data["owners"].as_array().unwrap() {
        assert_eq!(
            rules::accept_rust_static_byte_array_owner_equality(
                &text(row, "model"),
                &text(row, "leftKind"),
                &text(row, "leftPath"),
                &text(row, "rightKind"),
                &text(row, "rightPath")
            ),
            row["expected"].as_bool().unwrap()
        );
    }
    for row in data["views"].as_array().unwrap() {
        assert_eq!(
            rules::accept_rust_static_byte_array_slice_view(
                &text(row, "method"),
                row["count"].as_f64().unwrap(),
                &text(row, "model"),
                row["proven"].as_bool().unwrap(),
                row["layers"].as_f64().unwrap(),
                row["references"].as_bool().unwrap()
            ),
            row["expected"].as_bool().unwrap()
        );
    }
    for row in data["reads"].as_array().unwrap() {
        assert_eq!(
            rules::read_rust_static_byte_array_read_model(&text(row, "model")).to_string(),
            row["expected"].as_str().unwrap()
        );
    }
}

#[test]
fn every_actual_static_byte_read_view_premise_is_retained() {
    super::translation_string_array_copy_sort::require_rust_profile();
    let data = meta_language::read_structured_data_lino(include_str!(
        "../../../parity/fixtures/rust-byte-string-expressions.lino"
    ))
    .unwrap();
    for row in data["readViews"].as_array().unwrap() {
        assert_eq!(
            rules::accept_rust_static_byte_array_read_view(
                &text(row, "model"),
                &text(row, "kind"),
                &text(row, "element"),
                row["bits"].as_f64().unwrap(),
                row["signed"].as_bool().unwrap(),
                row["proven"].as_bool().unwrap(),
                row["layers"].as_f64().unwrap(),
                row["references"].as_bool().unwrap(),
                &text(row, "owner"),
                row["children"].as_f64().unwrap(),
                &text(row, "childKind"),
                &text(row, "childPath")
            ),
            row["expected"].as_bool().unwrap()
        );
    }
}
