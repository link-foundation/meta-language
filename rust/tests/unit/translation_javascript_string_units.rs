//! Future full literal graph and source controls; CI executes native compilation.
use super::translation_rust_records::assert_nominal_translation_product;
#[test]
fn isolated_source_units_retain_values_errors_and_complete_linked_programs() {
    assert_nominal_translation_product(
        "../experiments/javascript-program/string-units.mjs",
        "JavaScript UTF16 source literals",
        meta_language::create_java_script_string_unit_program_links,
    );
}
