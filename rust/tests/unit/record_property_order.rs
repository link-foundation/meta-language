//! Future source/native/original controls; compilation and execution stay in CI.
use meta_language::{
    RecordPropertyOrderError, create_java_script_record_property_order,
    merge_java_script_record_property_order, read_java_script_record_property_keys,
    rebase_java_script_record_property_order,
};
use serde_json::{Value, json};

fn float_numbers(value: &Value) -> Value {
    match value {
        Value::Number(number) => json!(number.as_f64().unwrap()),
        Value::Array(values) => Value::Array(values.iter().map(float_numbers).collect()),
        Value::Object(fields) => Value::Object(
            fields
                .iter()
                .map(|(name, value)| (name.clone(), float_numbers(value)))
                .collect(),
        ),
        value => value.clone(),
    }
}
#[test]
fn all_actual_orders_errors_definitions_and_codecs_match_node_and_ordinary_objects() {
    use meta_language::{
        BinaryLinksOptions, LinkNetwork, create_java_script_record_order_program_links,
        decode_link_network_binary, emit_typed_program_links, encode_link_network_binary,
        read_structured_data_lino,
    };
    let output = std::process::Command::new("node")
        .arg("../experiments/record-property-order/produce.mjs")
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let observed = read_structured_data_lino(std::str::from_utf8(&output.stdout).unwrap()).unwrap();
    let fixture = read_structured_data_lino(include_str!(
        "../../../parity/fixtures/javascript-record-order.lino"
    ))
    .unwrap();
    assert_eq!(
        observed["cases"].as_array().unwrap().len(),
        fixture["cases"].as_array().unwrap().len()
    );
    for (index, row) in fixture["cases"].as_array().unwrap().iter().enumerate() {
        let names: Vec<Vec<f64>> = serde_json::from_value(row["names"].clone()).unwrap();
        let actual: Result<Value, RecordPropertyOrderError> =
            match row["operation"].as_str().unwrap() {
                "create" => create_java_script_record_property_order(
                    &names,
                    &serde_json::from_value::<Vec<f64>>(row["input"].clone()).unwrap(),
                )
                .map(|value| json!(value)),
                "merge" => merge_java_script_record_property_order(
                    &names,
                    &serde_json::from_value::<Vec<Vec<f64>>>(row["input"].clone()).unwrap(),
                )
                .map(|value| json!(value)),
                "rebase" => rebase_java_script_record_property_order(
                    &names,
                    &serde_json::from_value::<Vec<Vec<f64>>>(row["sourceNames"].clone()).unwrap(),
                    &serde_json::from_value::<Vec<f64>>(row["input"].clone()).unwrap(),
                )
                .map(|value| json!(value)),
                "keys" => read_java_script_record_property_keys(
                    &names,
                    &serde_json::from_value::<Vec<f64>>(row["input"].clone()).unwrap(),
                )
                .map(|value| json!(value)),
                operation => panic!("unknown actual fixture operation {operation}"),
            };
        let expected = if let Some(message) = row["error"].as_str() {
            json!({"kind":"error","className":"TypeError","message":message})
        } else {
            json!({"kind":"value","value":row["expected"]})
        };
        let native = match actual {
            Ok(value) => json!({"kind":"value","value":value}),
            Err(error) => {
                json!({"kind":"error","className":error.class_name(),"message":error.message()})
            }
        };
        assert_eq!(
            float_numbers(&native),
            float_numbers(&expected),
            "{}",
            row["name"]
        );
        assert_eq!(
            float_numbers(&observed["cases"][index]["observed"]),
            float_numbers(&expected),
            "{}",
            row["name"]
        );
        if row["error"].is_null() {
            assert_eq!(
                float_numbers(&observed["cases"][index]["originalObserved"]),
                float_numbers(&expected),
                "{}",
                row["name"]
            );
        }
    }
    let packet = create_java_script_record_order_program_links().unwrap();
    assert_eq!(observed["links"], packet.network.to_lino());
    let text = LinkNetwork::from_lino(&packet.network.to_lino()).unwrap();
    let binary = decode_link_network_binary(
        &encode_link_network_binary(&packet.network, BinaryLinksOptions::default()).unwrap(),
        BinaryLinksOptions::default(),
    )
    .unwrap();
    for target in ["JavaScript", "Rust", "Lean", "Rocq"] {
        let expected = emit_typed_program_links(&packet.network, packet.root, target).unwrap();
        assert_eq!(
            observed["emissions"][target],
            serde_json::to_value(&expected).unwrap()
        );
        for network in [&text, &binary] {
            assert_eq!(
                emit_typed_program_links(network, packet.root, target).unwrap(),
                expected
            );
        }
    }
}

#[test]
fn canonical_positions_refuse_nonfinite_numbers_and_negative_zero() {
    use meta_language::{
        accept_java_script_record_order_permutation, accept_java_script_record_property_order,
    };
    for value in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY, -0.0] {
        assert!(!accept_java_script_record_order_permutation(1.0, &[value]));
        assert!(!accept_java_script_record_property_order(
            &[vec![97.0]],
            &[value]
        ));
        let error = create_java_script_record_property_order(&[vec![97.0]], &[value]).unwrap_err();
        assert_eq!(error.class_name(), "TypeError");
        assert_eq!(error.message(), "invalid ordered record field sequence");
        let error =
            merge_java_script_record_property_order(&[vec![97.0]], &[vec![value]]).unwrap_err();
        assert_eq!(error.class_name(), "TypeError");
        assert_eq!(error.message(), "invalid ordered record source field");
    }
    assert!(!accept_java_script_record_order_permutation(-0.0, &[]));
    assert!(accept_java_script_record_order_permutation(0.0, &[]));
}
