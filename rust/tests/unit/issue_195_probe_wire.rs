//! Exercises the actual example wire writer with a tiny independent payload.

#[path = "../../examples/support/issue_195_probe_canonical.rs"]
mod probe_canonical;
#[path = "../../examples/support/issue_195_probe_wire.rs"]
mod probe_wire;

use serde_json::{Value, json};

#[test]
fn actual_probe_wire_writer_emits_complete_ordered_entries_without_source_corpora() {
    let sections = [
        (
            "positive",
            vec![
                json!({"text": "λ\nsecond line", "clean": true}),
                json!(null),
            ],
        ),
        ("empty", vec![]),
        (
            "negative",
            vec![json!([false, 7, {"diagnostic": "retained"}])],
        ),
    ];
    let mut output = Vec::new();
    probe_wire::write_runtime_probe_wire_records(&mut output, "fixture-digest", &sections).unwrap();
    let text = String::from_utf8(output).unwrap();
    assert!(text.ends_with('\n'));
    let records: Vec<Value> = text
        .lines()
        .map(|line| serde_json::from_str(line).unwrap())
        .collect();
    assert_eq!(
        records,
        vec![
            json!({"section": "schemaVersion", "index": null, "value": 1}),
            json!({"section": "fixtureDigest", "index": null, "value": "fixture-digest"}),
            json!({"section": "positive", "index": 0, "value": {"text": "λ\nsecond line", "clean": true}}),
            json!({"section": "positive", "index": 1, "value": null}),
            json!({"section": "negative", "index": 0, "value": [false, 7, {"diagnostic": "retained"}]}),
        ]
    );
}

#[test]
fn canonical_probe_signatures_preserve_ordered_arrays_and_original_values() {
    let original = json!({
        "z": [{"b": 2, "a": 1}, {"b": 4, "a": 3}],
        "a": {"z": "λ\n\"", "a": [false, null, true]},
    });
    let original_wire = serde_json::to_string(&original).unwrap();
    assert_eq!(
        probe_canonical::serialize_canonical_observation(&original),
        r#"{"a":{"a":[false,null,true],"z":"λ\n\""},"z":[{"a":1,"b":2},{"a":3,"b":4}]}"#,
    );
    assert_eq!(serde_json::to_string(&original).unwrap(), original_wire);
    assert_eq!(original["z"][0]["a"], 1);
    assert_eq!(original["z"][1]["a"], 3);
}

#[test]
fn canonical_probe_fact_sort_matches_javascript_range_and_unicode_order() {
    let values = vec![
        json!({"kind": "same", "term": "a", "range": {"start": 20, "end": 21}}),
        json!({"kind": "same", "term": "z", "range": {"start": 1, "end": 2}}),
    ];
    assert_eq!(
        probe_canonical::sort_observation_values(values.clone()),
        vec![values[1].clone(), values[0].clone()],
    );
    let unicode = vec![
        json!({"name": "\u{e000}"}),
        json!({"name": "\u{1f600}"}),
        json!({"name": "\u{1f600}"}),
    ];
    assert_eq!(
        probe_canonical::sort_observation_values(unicode.clone()),
        vec![unicode[1].clone(), unicode[2].clone(), unicode[0].clone()],
    );
}
