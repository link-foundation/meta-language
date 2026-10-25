//! Future values, linked source definitions, codecs, emissions and traces.
#[test]
fn named_omission_uses_actual_optional_parameter_domains() {
    super::translation_rust_records::assert_nominal_translation_product(
        "../experiments/javascript-program/typed-omitted-arguments.mjs",
        "JavaScript typed omitted arguments",
        || {
            meta_language::create_source_definition_program_links(
                "meta-language/source-program/optional-callable-program",
            )
            .map_err(|message| meta_language::TypedProgramLinksError { message })
        },
    );
    let fixture = meta_language::read_structured_data_lino(include_str!(
        "../../../parity/fixtures/typed-omitted-arguments.lino"
    ))
    .expect("actual future fixture");
    for row in fixture["traces"].as_array().expect("actual source traces") {
        super::translation_javascript_array_bindings::require_declaration_list_equivalent(
            row["source"].as_str().expect("actual source"),
            &serde_json::json!({"lines":row["lines"],"error":row["error"]}),
        );
    }
}
