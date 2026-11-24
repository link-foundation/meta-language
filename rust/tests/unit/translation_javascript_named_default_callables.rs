//! Future native values, full definitions, codecs, emissions and effect traces.
#[test]
fn named_default_references_retain_initialization_and_policy_domains() {
    super::translation_rust_records::assert_nominal_translation_product(
        "../experiments/javascript-program/named-default-callables.mjs",
        "JavaScript named default callables",
        || {
            meta_language::create_source_definition_program_links(
                "meta-language/source-program/named-default-callable-program",
            )
            .map_err(|message| meta_language::TypedProgramLinksError { message })
        },
    );
    let fixture = meta_language::read_structured_data_lino(include_str!(
        "../../../parity/fixtures/named-default-callables.lino"
    ))
    .expect("actual future fixture");
    for row in fixture["traces"].as_array().expect("actual source traces") {
        super::translation_javascript_array_bindings::require_declaration_list_equivalent(
            row["source"].as_str().expect("actual source"),
            &serde_json::json!({"lines":row["lines"],"error":row["error"]}),
        );
    }
}
