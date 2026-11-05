//! Deferred native source/codec/four-target controls; native execution is CI only.
use meta_language::translation::{check::check_program, surface::SProgram};
use meta_language::{
    BinaryLinksOptions, LinkNetwork, decode_link_network_binary, decode_typed_program_links,
    emit_typed_program_links, encode_link_network_binary, encode_typed_program_links,
    read_structured_data_lino,
};
use serde_json::json;

#[test]
fn dynamic_order_values_preserve_actual_inputs_errors_and_all_emissions() {
    let fixture = read_structured_data_lino(include_str!(
        "../../../parity/fixtures/ordered-record-values.lino"
    ))
    .unwrap();
    let surface: SProgram = serde_json::from_value(fixture["surface"].clone()).unwrap();
    let program = check_program(&surface).unwrap();
    let packet = encode_typed_program_links(&program).unwrap();
    let output = std::process::Command::new("node")
        .arg("../experiments/ordered-record-values/produce.mjs")
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let observed = read_structured_data_lino(std::str::from_utf8(&output.stdout).unwrap()).unwrap();
    assert_eq!(
        observed["cases"].as_array().unwrap().len(),
        fixture["cases"].as_array().unwrap().len()
    );
    for (index, row) in fixture["cases"].as_array().unwrap().iter().enumerate() {
        let expected = if row["error"].is_null() {
            json!({"kind":"value","keys":row["keys"],"values":row["values"]})
        } else {
            json!({"kind":"error","className":row["error"]["className"],"message":row["error"]["message"]})
        };
        assert_eq!(observed["cases"][index]["name"], row["name"]);
        assert_eq!(
            observed["cases"][index]["observed"], expected,
            "{}",
            row["name"]
        );
        if row["error"].is_null() {
            assert_eq!(
                observed["cases"][index]["original"], expected,
                "{}",
                row["name"]
            );
        }
    }
    assert_eq!(observed["links"], packet.network.to_lino());
    let text = LinkNetwork::from_lino(&packet.network.to_lino()).unwrap();
    let binary = decode_link_network_binary(
        &encode_link_network_binary(&packet.network, BinaryLinksOptions::default()).unwrap(),
        BinaryLinksOptions::default(),
    )
    .unwrap();
    for network in [&packet.network, &text, &binary] {
        assert_eq!(
            decode_typed_program_links(network, packet.root).unwrap(),
            program
        );
        for target in ["JavaScript", "Rust", "Lean", "Rocq"] {
            let native = emit_typed_program_links(network, packet.root, target).unwrap();
            assert_eq!(
                serde_json::to_value(native).unwrap(),
                observed["emissions"][target]
            );
        }
    }
}
