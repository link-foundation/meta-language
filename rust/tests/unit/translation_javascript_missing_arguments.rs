//! Future native omitted-input values, full source, codecs and effect traces.
#[test]
fn missing_inferred_inputs_retain_actual_undefined() {
    super::translation_rust_records::assert_nominal_translation_product(
        "../experiments/javascript-program/missing-arguments.mjs",
        "JavaScript actual omitted inferred inputs",
        || {
            meta_language::create_source_definition_program_links(
                "meta-language/source-program/missing-argument-program",
            )
            .map_err(|message| meta_language::TypedProgramLinksError { message })
        },
    );
    let fixture = meta_language::read_structured_data_lino(include_str!(
        "../../../parity/fixtures/missing-arguments.lino"
    ))
    .expect("actual future fixture");
    for row in fixture["traces"].as_array().expect("actual source traces") {
        super::translation_javascript_array_bindings::require_declaration_list_equivalent(
            row["source"].as_str().expect("actual source"),
            &serde_json::json!({"lines":row["lines"],"error":row["error"]}),
        );
    }
}
