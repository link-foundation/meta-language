//! Deferred source-free definitions and independent original/native products.
use super::translation_rust_records::assert_nominal_translation_product;
#[test]
fn mixed_recursive_bindings_preserve_actual_owners_and_initialization_order() {
    super::translation_javascript_object_binding_declarations::require_supported_rust_version();
    assert_nominal_translation_product(
        "../experiments/javascript-program/recursive-bindings.mjs",
        "JavaScript mixed recursive bindings and formal contexts",
        || {
            meta_language::create_source_definition_program_links(
                "meta-language/source-program/recursive-binding-program",
            )
            .map_err(|message| meta_language::TypedProgramLinksError { message })
        },
    );
}
