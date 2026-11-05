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

#[test]
fn named_receiver_region_retains_its_original_input_and_complete_products() {
    assert_nominal_translation_product(
        "../experiments/rust-inherent/receiver-0.mjs",
        "named immutable receiver region",
        create_rust_inherent_program_links,
    );
}

#[test]
fn explicit_receiver_type_retains_its_original_input_and_complete_products() {
    assert_nominal_translation_product(
        "../experiments/rust-inherent/receiver-1.mjs",
        "explicit nominal receiver type",
        create_rust_inherent_program_links,
    );
}

#[test]
fn boxed_receiver_retains_its_original_input_and_complete_products() {
    assert_nominal_translation_product(
        "../experiments/rust-inherent/receiver-2.mjs",
        "boxed receiver dereference",
        create_rust_inherent_program_links,
    );
}
