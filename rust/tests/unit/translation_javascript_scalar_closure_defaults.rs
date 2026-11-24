//! Future values, complete source definitions, codecs, emissions and traces.
#[test]
fn scalar_closure_defaults_use_ordered_existing_parameter_entries() {
    super::translation_rust_records::assert_nominal_translation_product(
        "../experiments/javascript-program/scalar-closure-defaults.mjs",
        "JavaScript scalar closure defaults",
        || {
            meta_language::create_source_definition_program_links(
                "meta-language/source-program/scalar-closure-default-program",
            )
            .map_err(|message| meta_language::TypedProgramLinksError { message })
        },
    );
    let fixture = meta_language::read_structured_data_lino(include_str!(
        "../../../parity/fixtures/scalar-closure-defaults.lino"
    ))
    .expect("actual future fixture");
    for row in fixture["traces"].as_array().expect("actual source traces") {
        super::translation_javascript_array_bindings::require_declaration_list_equivalent(
            row["source"].as_str().expect("actual source"),
            &serde_json::json!({"lines":row["lines"],"error":row["error"]}),
        );
    }
}
