//! Future complete source exception controls; no local compilation or execution.
use super::translation_rust_records::assert_nominal_translation_product;
#[test]
fn computed_exception_messages_retain_class_units_presence_and_operand_order() {
    assert_nominal_translation_product(
        "../experiments/source-exceptions/oracle.mjs",
        "JavaScript source exceptions",
        meta_language::create_source_exception_program_links,
    );
}
