//! Future complete Node/native/graph controls; compilation runs only in CI.
use super::translation_rust_records::assert_nominal_translation_product;
use meta_language::create_java_script_property_call_program_links;

#[test]
fn actual_javascript_property_owners_retain_all_values_graphs_and_emissions() {
    assert_nominal_translation_product(
        "../experiments/javascript-property-calls/oracle.mjs",
        "actual JavaScript property calls",
        create_java_script_property_call_program_links,
    );
}
