//! Future complete source, graph, native compiler and original Node controls.
use super::translation_rust_records::assert_nominal_translation_product;
use meta_language::create_reference_array_projection_program_links;

#[test]
fn shared_array_reads_preserve_receiver_key_order_aliases_and_native_values() {
    assert_nominal_translation_product(
        "../experiments/reference-array-projections/oracle.mjs",
        "shared array scalar projections",
        create_reference_array_projection_program_links,
    );
}
