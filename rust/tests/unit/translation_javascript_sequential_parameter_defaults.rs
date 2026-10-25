//! Future complete source/native products and independent trace controls.
use super::translation_rust_records::assert_nominal_translation_product;
#[test]
fn sequential_defaults_preserve_actual_parameter_environments_and_argument_order() {
    assert_nominal_translation_product(
        "../experiments/javascript-program/sequential-parameter-defaults.mjs",
        "JavaScript sequential parameter defaults",
        || {
            meta_language::create_source_definition_program_links(
                "meta-language/source-program/sequential-parameter-default-program",
            )
            .map_err(|message| meta_language::TypedProgramLinksError { message })
        },
    );
    let fixture = meta_language::read_structured_data_lino(include_str!(
        "../../../parity/fixtures/sequential-parameter-defaults.lino"
    ))
    .expect("the actual shared fixture");
    for row in fixture["traces"].as_array().expect("trace controls") {
        super::translation_javascript_array_bindings::require_declaration_list_equivalent(
            row["source"].as_str().expect("actual source"),
            &serde_json::json!({"lines":row["lines"],"error":row["error"]}),
        );
    }
}
