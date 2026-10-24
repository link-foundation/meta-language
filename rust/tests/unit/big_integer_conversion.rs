//! Future independent primitive conversion, message transport and graph controls.
use meta_language::{
    BinaryLinksOptions, LinkNetwork, create_big_integer_conversion_program_links,
    decode_link_network_binary, emit_typed_program_links, encode_link_network_binary,
    read_structured_data_lino,
};
use serde_json::json;

#[test]
fn source_big_integer_division_retains_signed_values_error_fields_and_argument_order() {
    super::translation_rust_records::assert_nominal_translation_product(
        "../experiments/big-integer-conversion/division-source-oracle.mjs",
        "JavaScript BigInt division and remainder",
        create_big_integer_conversion_program_links,
    );
}

#[test]
fn public_division_retains_exact_quotient_remainder_and_structured_source_error() {
    for (left, right, quotient, remainder) in [
        ("7", "3", "2", "1"),
        ("-7", "3", "-2", "-1"),
        ("7", "-3", "-2", "1"),
        ("-7", "-3", "2", "-1"),
        ("9007199254740993", "2", "4503599627370496", "1"),
        ("18446744073709551615", "3", "6148914691236517205", "0"),
    ] {
        let actual = meta_language::divide_javascript_big_integer(
            meta_language::convert_javascript_string_big_integer(left).unwrap(),
            meta_language::convert_javascript_string_big_integer(right).unwrap(),
        )
        .unwrap();
        assert_eq!(
            (actual.0.to_string(), actual.1.to_string()),
            (quotient.to_owned(), remainder.to_owned())
        );
    }
    let error = meta_language::divide_javascript_big_integer(
        meta_language::convert_javascript_boolean_big_integer(true),
        meta_language::convert_javascript_boolean_big_integer(false),
    )
    .unwrap_err();
    assert_eq!(error.class_name, "RangeError");
    assert_eq!(
        error.message_units,
        "Division by zero".encode_utf16().collect::<Vec<_>>()
    );
    assert!(error.message_present);
}

#[test]
fn exact_big_integer_conversion_matches_independent_node_and_complete_definition() {
    let version = std::process::Command::new("rustc")
        .arg("--version")
        .output()
        .unwrap();
    assert!(version.status.success());
    let version = String::from_utf8(version.stdout).unwrap();
    let active = version.split_whitespace().nth(1).unwrap();
    let parts = active
        .split('.')
        .take(2)
        .map(|part| part.parse::<u32>().unwrap())
        .collect::<Vec<_>>();
    assert!(
        parts[0] == 1 && (90..100).contains(&parts[1]),
        "uncovered Rust {active}; >=1.90 <1.100"
    );
    let output = std::process::Command::new("node")
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .args([
            "--max-old-space-size=352",
            "--max-semi-space-size=1",
            "../experiments/big-integer-conversion/oracle.mjs",
        ])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let fixture = read_structured_data_lino(std::str::from_utf8(&output.stdout).unwrap()).unwrap();
    for row in fixture["cases"].as_array().unwrap() {
        let result = if row["kind"] == "string" {
            let units = row["units"]
                .as_array()
                .unwrap()
                .iter()
                .map(|value| u16::try_from(value.as_u64().unwrap()).unwrap())
                .collect::<Vec<_>>();
            let result = meta_language::convert_javascript_string_big_integer_units(&units);
            if let Ok(text) = String::from_utf16(&units) {
                assert_eq!(
                    meta_language::convert_javascript_string_big_integer(&text),
                    result,
                    "{}",
                    row["name"]
                );
            }
            result
        } else {
            let bits = u64::from_str_radix(row["numberBits"].as_str().unwrap(), 16).unwrap();
            meta_language::convert_javascript_number_big_integer(f64::from_bits(bits))
        };
        let actual = match result {
            Ok(value) => json!({"kind":"value","value":value.to_string()}),
            Err(error) => {
                json!({"kind":"error","errorClass":error.class_name,"messageUnits":error.message_units})
            }
        };
        assert_eq!(row["original"], row["expected"], "{}", row["name"]);
        assert_eq!(row["javascript"], row["expected"], "{}", row["name"]);
        assert_eq!(actual, row["expected"], "{}", row["name"]);
    }
    for (value, expected) in [(false, "0"), (true, "1")] {
        assert_eq!(
            meta_language::convert_javascript_boolean_big_integer(value).to_string(),
            expected
        );
    }
    let packet = create_big_integer_conversion_program_links().unwrap();
    assert_eq!(packet.root.as_u64(), fixture["root"].as_u64().unwrap());
    assert_eq!(packet.network.to_lino(), fixture["links"].as_str().unwrap());
    let text = LinkNetwork::from_lino(&packet.network.to_lino()).unwrap();
    let binary = decode_link_network_binary(
        &encode_link_network_binary(&packet.network, BinaryLinksOptions::default()).unwrap(),
        BinaryLinksOptions::default(),
    )
    .unwrap();
    for language in ["JavaScript", "Rust", "Lean", "Rocq"] {
        let expected = emit_typed_program_links(&packet.network, packet.root, language).unwrap();
        assert_eq!(
            json!({"code":expected.code,"language":expected.language,"mappings":expected.mappings,"assumptions":expected.assumptions,"encodings":expected.encodings,"theorems":expected.theorems}),
            fixture["emissions"][language]
        );
        for network in [&text, &binary] {
            assert_eq!(
                emit_typed_program_links(network, packet.root, language).unwrap(),
                expected
            );
        }
    }
}
