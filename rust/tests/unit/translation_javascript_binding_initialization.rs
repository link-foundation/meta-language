//! Future native values, complete definition, codecs, emissions and traces.
#[test]
fn nameless_patterns_retain_source_initialization_boundaries() {
    super::translation_rust_records::assert_nominal_translation_product(
        "../experiments/javascript-program/binding-initialization.mjs",
        "JavaScript nameless binding initialization",
        || {
            meta_language::create_source_definition_program_links(
                "meta-language/source-program/binding-initialization-program",
            )
            .map_err(|message| meta_language::TypedProgramLinksError { message })
        },
    );
    let fixture = meta_language::read_structured_data_lino(include_str!(
        "../../../parity/fixtures/binding-initialization.lino"
    ))
    .expect("actual future fixture");
    for row in fixture["traces"].as_array().expect("actual source traces") {
        super::translation_javascript_array_bindings::require_declaration_list_equivalent(
            row["source"].as_str().expect("actual source"),
            &serde_json::json!({"lines":row["lines"],"error":row["error"]}),
        );
    }
}
