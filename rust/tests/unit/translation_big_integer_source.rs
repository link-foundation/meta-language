//! Complete future source BigInt controls; native execution belongs to CI.
use super::translation_rust_records::assert_nominal_translation_product;
#[test]
fn bigint_source_lowering_keeps_exact_values_errors_and_argument_order() {
    assert_nominal_translation_product(
        "../experiments/big-integer-conversion/source-oracle.mjs",
        "JavaScript source BigInt conversions",
        meta_language::create_big_integer_conversion_program_links,
    );
}
