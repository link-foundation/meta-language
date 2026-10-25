//! Future original-source/native products and complete source-definition parity.
use super::translation_rust_records::assert_nominal_translation_product;
#[test]
fn own_property_branch_facts_preserve_actual_runtime_record_key_reads() {
    assert_nominal_translation_product(
        "../experiments/javascript-program/javascript-guarded-record-indices.mjs",
        "JavaScript guarded record indices",
        meta_language::create_java_script_record_order_program_links,
    );
}
