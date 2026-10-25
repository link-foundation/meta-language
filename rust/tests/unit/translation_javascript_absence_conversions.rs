//! Future native values, source definitions, codecs, emissions and traces.
#[test]
fn absence_only_conversion_preserves_tags_and_receiver_effects() {
    super::translation_rust_records::assert_nominal_translation_product(
        "../experiments/javascript-program/absence-conversions.mjs",
        "JavaScript absence-only conversions",
        || {
            meta_language::create_source_definition_program_links(
                "meta-language/source-program/absence-conversion-program",
            )
            .map_err(|message| meta_language::TypedProgramLinksError { message })
        },
    );
    let fixture = meta_language::read_structured_data_lino(include_str!(
        "../../../parity/fixtures/absence-conversions.lino"
    ))
    .expect("actual future source fixture");
    for row in fixture["traces"].as_array().expect("actual source traces") {
        super::translation_javascript_array_bindings::require_declaration_list_equivalent(
            row["source"].as_str().expect("actual source"),
            &serde_json::json!({"lines":row["lines"],"error":row["error"]}),
        );
    }
}
