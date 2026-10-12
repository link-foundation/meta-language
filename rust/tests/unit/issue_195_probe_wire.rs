//! Exercises the actual example wire writer with a tiny independent payload.

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
