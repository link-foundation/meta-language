//! Namespaced evidence frames share the JavaScript wire schema and data codec.

use meta_language::structured_data_records::{
    read_structured_data_records, read_structured_data_records_ordered,
    read_structured_data_records_with_limits, structured_data_record_to_lino,
    structured_data_record_to_lino_ordered_with_namespace,
    structured_data_record_to_lino_with_namespace,
};
use meta_language::{StructuredDataLinksLimits, StructuredDataValue};
use serde_json::json;

const FIRST: &str = "00000000-0000-4000-8000-000000000001";
const SECOND: &str = "00000000-0000-4000-8000-000000000002";

#[test]
fn javascript_frames_reload_and_serialize_with_identical_inspectable_data() {
    let source = include_str!("../../../parity/fixtures/structured-data-records.lino");
    let values = read_structured_data_records(source).unwrap();
    assert_eq!(values.len(), 2);
    assert_eq!(values[0]["runtime"], "javascript");
    assert_eq!(values[0]["commit"], "a".repeat(40));
    assert_eq!(values[0]["value"][0].as_u64(), Some(1));
    assert_eq!(values[0]["value"][1], "λ");
    assert_eq!(
        values[0]["value"][2].as_f64().unwrap().to_bits(),
        (-0.0_f64).to_bits()
    );
    assert_eq!(
        values[1],
        json!({"runtime": "rust", "value": [1, false, null]})
    );
    assert_eq!(
        structured_data_record_to_lino_with_namespace(&values[0], FIRST).unwrap()
            + &structured_data_record_to_lino_with_namespace(&values[1], SECOND).unwrap(),
        source,
    );
    let fresh = structured_data_record_to_lino(&values[0]).unwrap();
    assert_eq!(
        read_structured_data_records(&fresh).unwrap(),
        vec![values[0].clone()]
    );
    assert_eq!(fresh.as_bytes()[15], b'8');
    let second_fresh = structured_data_record_to_lino(&values[0]).unwrap();
    assert_eq!(
        read_structured_data_records(&(fresh + &second_fresh))
            .unwrap()
            .len(),
        2
    );
}

#[test]
fn eager_records_bound_aggregate_expansion_and_repeated_scalar_bytes() {
    let first = structured_data_record_to_lino_with_namespace(&json!([0, 0]), FIRST).unwrap();
    let second = structured_data_record_to_lino_with_namespace(&json!([0, 0]), SECOND).unwrap();
    let limits = StructuredDataLinksLimits {
        node_limit: 5,
        ..Default::default()
    };
    assert_eq!(
        read_structured_data_records_with_limits(&first, limits).unwrap(),
        vec![json!([0, 0])]
    );
    assert!(
        read_structured_data_records_with_limits(&(first + &second), limits)
            .unwrap_err()
            .contains("aggregate")
    );

    let value = json!(vec!["x".repeat(256); 64]);
    let first = structured_data_record_to_lino_with_namespace(&value, FIRST).unwrap();
    let second = structured_data_record_to_lino_with_namespace(&value, SECOND).unwrap();
    let limits = StructuredDataLinksLimits {
        byte_limit: 20_000,
        ..Default::default()
    };
    assert!(first.len() + second.len() < limits.byte_limit);
    assert_eq!(
        read_structured_data_records_with_limits(&first, limits).unwrap(),
        vec![value]
    );
    assert!(
        read_structured_data_records_with_limits(&(first + &second), limits)
            .unwrap_err()
            .contains("aggregate")
    );
}

#[test]
fn namespaced_records_preserve_ordered_keys_and_unpaired_code_units() {
    let value = StructuredDataValue::Object(vec![
        (
            "10".encode_utf16().collect(),
            StructuredDataValue::Number(-0.0),
        ),
        (
            "2".encode_utf16().collect(),
            StructuredDataValue::String(vec![0xd800]),
        ),
        (vec![0xdfff], StructuredDataValue::Null),
    ]);
    let source = structured_data_record_to_lino_ordered_with_namespace(&value, FIRST).unwrap();
    let decoded = read_structured_data_records_ordered(&source).unwrap();
    assert_eq!(decoded, vec![value]);
    assert!(
        read_structured_data_records(&source)
            .unwrap_err()
            .contains("unpaired UTF-16")
    );
}

#[test]
fn record_reader_rejects_foreign_ownership_and_malformed_graphs() {
    let first = structured_data_record_to_lino_with_namespace(&json!(1), FIRST).unwrap();
    let second = structured_data_record_to_lino_with_namespace(&json!(2), SECOND).unwrap();
    let malformed = [
        first.repeat(2),
        first.replace(&format!("{FIRST}/1:"), &format!("{SECOND}/1:")),
        first.replace(
            &format!("{FIRST}/2: {FIRST}/1"),
            &format!("{FIRST}/2: {SECOND}/1"),
        ),
        first.replace(
            &format!("{FIRST}/record: {FIRST}/2"),
            &format!("{FIRST}/record: {FIRST}/1"),
        ),
        first.replace(&format!("{FIRST}/2:"), &format!("{FIRST}/1:")),
        first.replace(
            "(term: structured%20data%20record)",
            "(term: unknown%20record)",
        ),
        first.replace(
            "(term: structured%20data%20record)",
            "(term: structured%20data%20record) (def: 1)",
        ),
        first.replace("(n: 1)", "(n: 0)"),
        first.replace("(term: number%20value)", "(term: unknown%20value)"),
        first.replace(
            &format!("{FIRST}/2: {FIRST}/1"),
            &format!("{FIRST}/2: {FIRST}/2"),
        ),
        first.replace(
            &format!("{FIRST}/2: {FIRST}/1"),
            &format!("{FIRST}/2: {FIRST}/99"),
        ),
        first.replace(FIRST, "00000000-0000-0000-8000-000000000001"),
        first.replace(FIRST, "00000000-0000-4000-0000-000000000001"),
    ];
    for source in malformed {
        assert!(
            read_structured_data_records(&source).is_err(),
            "accepted {source}"
        );
    }
    assert_eq!(
        read_structured_data_records(&(first.clone() + &second)).unwrap(),
        vec![json!(1), json!(2)]
    );
    assert!(structured_data_record_to_lino_with_namespace(&json!(1), "local").is_err());
    assert!(
        read_structured_data_records_with_limits(
            &first,
            StructuredDataLinksLimits {
                node_limit: 1,
                ..Default::default()
            }
        )
        .is_err()
    );
    assert!(
        read_structured_data_records_with_limits(
            &first,
            StructuredDataLinksLimits {
                byte_limit: 8,
                ..Default::default()
            }
        )
        .is_err()
    );
}
