//! Deferred original-source, shallow identity, order and complete graph products.
use super::translation_rust_records::assert_nominal_translation_product;
#[test]
fn lexical_object_rest_keeps_actual_fields_order_and_fresh_shallow_identity() {
    super::translation_javascript_object_binding_declarations::require_supported_rust_version();
    assert_nominal_translation_product(
        "../experiments/javascript-program/object-rest-bindings.mjs",
        "JavaScript actual closed own-field lexical object rest",
        || {
            meta_language::create_source_definition_program_links(
                "meta-language/source-program/object-rest-program",
            )
            .map_err(|message| meta_language::TypedProgramLinksError { message })
        },
    );
}
