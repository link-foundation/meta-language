//! Future native product, immutable source and trace controls.
#[test]
fn closed_record_absence_retains_its_receiver_and_actual_fallback() {
    super::translation_rust_records::assert_nominal_translation_product(
        "../experiments/javascript-program/absent-record-properties.mjs",
        "JavaScript absent record properties",
        || {
            meta_language::create_source_definition_program_links(
                "meta-language/source-program/absence-only-program",
            )
            .map_err(|message| meta_language::TypedProgramLinksError { message })
        },
    );
    let fixture = meta_language::read_structured_data_lino(include_str!(
        "../../../parity/fixtures/absent-record-properties.lino"
    ))
    .expect("actual future source fixture");
    for row in fixture["traces"].as_array().expect("actual source traces") {
        super::translation_javascript_array_bindings::require_declaration_list_equivalent(
            row["source"].as_str().expect("actual source"),
            &serde_json::json!({"lines":row["lines"],"error":row["error"]}),
        );
    }
}
