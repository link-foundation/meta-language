//! Future original/native values, source definitions, codecs, emissions and traces.
use super::translation_rust_records::assert_nominal_translation_product;
#[test]
fn optional_callables_preserve_actual_arguments_and_callback_bridges() {
    assert_nominal_translation_product(
        "../experiments/javascript-program/optional-callables.mjs",
        "JavaScript optional callables",
        || {
            meta_language::create_source_definition_program_links(
                "meta-language/source-program/optional-callable-program",
            )
            .map_err(|message| meta_language::TypedProgramLinksError { message })
        },
    );
    let fixture = meta_language::read_structured_data_lino(include_str!(
        "../../../parity/fixtures/optional-callables.lino"
    ))
    .expect("the actual source fixture");
    for row in fixture["traces"]
        .as_array()
        .expect("actual trace declarations")
    {
        super::translation_javascript_array_bindings::require_declaration_list_equivalent(
            row["source"].as_str().expect("actual source"),
            &serde_json::json!({"lines":row["lines"],"error":row["error"]}),
        );
    }
}
