//! Future controls; Rust compilation and execution belong to CI.
use super::translation_rust_records::assert_nominal_translation_product;
use meta_language::create_source_identifier_program_links;

#[test]
fn source_generated_and_runtime_identifier_names_remain_distinct() {
    assert_nominal_translation_product(
        "../experiments/source-identifiers/oracle.mjs",
        "capture free source identifier namespaces",
        create_source_identifier_program_links,
    );
}

#[test]
fn ordinary_patterns_retain_the_complete_record_parameter_definition() {
    assert_nominal_translation_product(
        "../experiments/source-identifiers/record-oracle.mjs",
        "ordinary and historical record parameter names",
        meta_language::create_javascript_record_parameter_program_links,
    );
}
