//! Future values, source definitions, serialization and effect traces.
#[test]
fn every_actual_argument_precedes_the_callee_entry() {
    super::translation_rust_records::assert_nominal_translation_product(
        "../experiments/javascript-program/excess-arguments.mjs",
        "JavaScript excess arguments",
        || {
            meta_language::create_source_definition_program_links(
                "meta-language/source-program/excess-argument-program",
            )
            .map_err(|message| meta_language::TypedProgramLinksError { message })
        },
    );
    let fixture = meta_language::read_structured_data_lino(include_str!(
        "../../../parity/fixtures/excess-arguments.lino"
    ))
    .expect("actual future source fixture");
    for row in fixture["traces"].as_array().expect("actual source traces") {
        super::translation_javascript_array_bindings::require_declaration_list_equivalent(
            row["source"].as_str().expect("actual source"),
            &serde_json::json!({"lines":row["lines"],"error":row["error"]}),
        );
    }
}
