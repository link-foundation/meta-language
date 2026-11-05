//! Deferred original-source, native and complete shared definition products.
use super::translation_rust_records::assert_nominal_translation_product;
#[test]
fn recursive_object_patterns_keep_actual_owners_defaults_and_rest_copies() {
    super::translation_javascript_object_binding_declarations::require_supported_rust_version();
    assert_nominal_translation_product(
        "../experiments/javascript-program/recursive-object-bindings.mjs",
        "JavaScript recursive object bindings and formal contexts",
        || {
            meta_language::create_source_definition_program_links(
                "meta-language/source-program/recursive-object-binding-program",
            )
            .map_err(|message| meta_language::TypedProgramLinksError { message })
        },
    );
}
