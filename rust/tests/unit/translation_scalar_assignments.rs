//! Future complete source controls; compilation and execution belong to CI.
use super::translation_rust_records::assert_nominal_translation_product;
#[test]
fn scalar_expression_assignments_retain_order_cells_and_results() {
    assert_nominal_translation_product(
        "../experiments/scalar-assignments/oracle.mjs",
        "ordered scalar assignment expressions",
        meta_language::create_scalar_assignment_program_links,
    );
}

#[test]
fn assigned_expression_captures_keep_the_complete_lexical_definition() {
    assert_nominal_translation_product(
        "../experiments/scalar-assignments/callable-oracle.mjs",
        "lexical scalar assignment expression cells",
        meta_language::create_callable_source_program_links,
    );
}
