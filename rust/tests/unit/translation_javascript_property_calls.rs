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
