//! Future paired inherent-method controls; product compilation runs only in CI.
use super::translation_rust_records::assert_nominal_translation_product;
use meta_language::create_rust_inherent_program_links;

#[test]
fn inherent_methods_preserve_complete_graphs_owners_order_and_original_values() {
    assert_nominal_translation_product(
        "../experiments/rust-inherent/oracle.mjs",
        "Rust inherent methods",
        create_rust_inherent_program_links,
    );
}
