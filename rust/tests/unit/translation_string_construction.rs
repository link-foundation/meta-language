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

#[test]
fn literal_string_split_keeps_utf16_fields_missing_separator_and_unsigned_limits() {
    super::translation_rust_records::assert_nominal_translation_product(
        "../experiments/javascript-program/string-split.mjs",
        "JavaScript literal UTF16 string split",
        meta_language::create_utf16_text_program_links,
    );
}
