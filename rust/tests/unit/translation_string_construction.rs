//! Future paired source constructor controls; native compilation is CI only.
use super::translation_rust_records::assert_nominal_translation_product;
#[test]
fn string_constructors_preserve_unit_values_errors_and_conversion_order() {
    assert_nominal_translation_product(
        "../experiments/javascript-program/string-construction.mjs",
        "JavaScript string construction",
        meta_language::create_string_construction_program_links,
    );
}
