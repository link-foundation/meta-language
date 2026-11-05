//! Deferred paired syntax, source graphs, codecs and native products.
use super::translation_rust_records::assert_nominal_translation_product;
#[test]
fn formal_object_rest_reuses_the_complete_closed_record_copy_definition() {
    super::translation_javascript_object_binding_declarations::require_supported_rust_version();
    assert_nominal_translation_product(
        "../experiments/javascript-program/object-rest-formals.mjs",
        "JavaScript actual closed own-field object rest formal parameters",
        meta_language::create_java_script_record_parameter_program_links,
    );
}
