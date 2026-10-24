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

#[test]
fn complete_string_range_rule_graphs_keep_all_codecs_and_emissions() {
    use meta_language::{
        BinaryLinksOptions, LinkNetwork, create_java_script_string_range_program_links,
        decode_link_network_binary, emit_typed_program_links, encode_link_network_binary,
    };
    let packet = create_java_script_string_range_program_links().unwrap();
    let text = LinkNetwork::from_lino(&packet.network.to_lino()).unwrap();
    let binary = decode_link_network_binary(
        &encode_link_network_binary(&packet.network, BinaryLinksOptions::default()).unwrap(),
        BinaryLinksOptions::default(),
    )
    .unwrap();
    for target in ["JavaScript", "Rust", "Lean", "Rocq"] {
        let expected = emit_typed_program_links(&packet.network, packet.root, target).unwrap();
        for network in [&text, &binary] {
            assert_eq!(
                emit_typed_program_links(network, packet.root, target).unwrap(),
                expected
            );
        }
    }
}

#[test]
fn optional_and_boolean_source_definitions_keep_all_codecs_and_emissions() {
    use meta_language::{
        BinaryLinksOptions, LinkNetwork, create_optional_value_program_links,
        decode_link_network_binary, emit_typed_program_links, encode_link_network_binary,
    };
    let packet = create_optional_value_program_links().unwrap();
    let text = LinkNetwork::from_lino(&packet.network.to_lino()).unwrap();
    let binary = decode_link_network_binary(
        &encode_link_network_binary(&packet.network, BinaryLinksOptions::default()).unwrap(),
        BinaryLinksOptions::default(),
    )
    .unwrap();
    for target in ["JavaScript", "Rust", "Lean", "Rocq"] {
        let expected = emit_typed_program_links(&packet.network, packet.root, target).unwrap();
        for network in [&text, &binary] {
            assert_eq!(
                emit_typed_program_links(network, packet.root, target).unwrap(),
                expected
            );
        }
    }
}

#[test]
fn primitive_string_unit_calls_keep_actual_arity_coercions_absence_and_argument_order() {
    assert_nominal_translation_product(
        "../experiments/javascript-program/string-unit-calls.mjs",
        "JavaScript primitive string unit calls",
        create_java_script_property_call_program_links,
    );
}

#[test]
fn lexical_optional_calls_keep_callee_capture_lazy_arguments_and_result_absence() {
    assert_nominal_translation_product(
        "../experiments/javascript-program/optional-calls.mjs",
        "JavaScript lexical optional calls",
        meta_language::create_java_script_optional_call_program_links,
    );
}

#[test]
fn continuous_optional_chains_keep_scoped_suffixes_lazy_operands_and_graphs() {
    assert_nominal_translation_product(
        "../experiments/javascript-program/optional-chains.mjs",
        "continuous optional chains",
        meta_language::create_java_script_optional_chain_program_links,
    );
}

#[test]
fn ordinary_nullish_fields_keep_actual_errors_and_optional_chain_boundaries() {
    assert_nominal_translation_product(
        "../experiments/javascript-program/nullish-fields.mjs",
        "ordinary nullish fields",
        meta_language::create_java_script_nullish_field_program_links,
    );
}

#[test]
fn nullable_methods_keep_property_errors_argument_order_and_slice_identity() {
    assert_nominal_translation_product(
        "../experiments/javascript-program/nullish-methods.mjs",
        "nullable method property loads",
        meta_language::create_java_script_nullish_field_program_links,
    );
}
