use meta_language::transform::{
    JsonRecoveryIssue, JsonRecoveryOptions, JsonRepairKind, recover_json,
};
use serde_json::Value;

use crate::issue_195_observations::{Observation, record};

const FIXTURE_FILE: &str = "parity/fixtures/json-recovery.lino";

#[test]
fn json_recovery_matches_the_shared_reports_and_replays_every_edit() {
    let fixtures: Value = crate::project_data::read_project_data(include_str!(
        "../../../parity/fixtures/json-recovery.lino"
    ))
    .expect("JSON recovery fixtures");
    let cases = fixtures["cases"].as_array().expect("recovery cases");
    let mut groups = std::collections::BTreeSet::new();
    for case in cases {
        let id = case["id"].as_str().expect("case name");
        let input = case["input"].as_str().expect("input source");
        let options: JsonRecoveryOptions =
            serde_json::from_value(case["options"].clone()).expect("recovery options");
        let result = recover_json(input, &options);
        assert_eq!(
            serde_json::to_value(&result).expect("serializable report"),
            case["expected"],
            "{id}"
        );

        let mut replayed = String::new();
        let mut offset = 0;
        for edit in &result.edits {
            assert!(
                offset <= edit.span.start && edit.span.start <= edit.span.end,
                "{id}"
            );
            assert_eq!(
                input.get(edit.span.start..edit.span.end),
                Some(edit.removed.as_str()),
                "{id}"
            );
            assert!(options.allow.contains(&edit.kind), "{id}");
            replayed.push_str(&input[offset..edit.span.start]);
            replayed.push_str(&edit.inserted);
            offset = edit.span.end;
        }
        replayed.push_str(&input[offset..]);
        assert_eq!(replayed, result.text, "{id}");
        if !result
            .issues
            .iter()
            .any(|issue| matches!(issue, JsonRecoveryIssue::UnrecoveredSyntax { .. }))
        {
            assert_eq!(
                result.value,
                Some(serde_json::from_str::<Value>(&result.text).expect(id)),
                "{id}"
            );
        }
        groups.insert(case["group"].as_str().expect("requirement group"));
    }

    for group in groups {
        record(&Observation {
            requirement_id: &format!("I195-JSON-RECOVERY-{}", group.to_ascii_uppercase()),
            suffix: "behavior",
            fixture_id: &format!("json-recovery:{group}"),
            fixture_file: FIXTURE_FILE,
            assertions: &["jsonRecoveryReportMatches", "jsonRecoveryEditsReplay"],
            test_name: "json_recovery_matches_the_shared_reports_and_replays_every_edit",
        });
    }
}

#[test]
fn json_recovery_closes_deeply_nested_objects_within_the_native_rule_limit() {
    let depth = 60;
    let input = format!("{}0", r#"{"value":"#.repeat(depth));
    let options = JsonRecoveryOptions {
        allow: vec![JsonRepairKind::CloseOpenContainers],
        schema: None,
    };
    let result = recover_json(&input, &options);
    assert_eq!(result.edits.len(), depth);
    assert_eq!(result.text, input + &"}".repeat(depth));
    assert!(!result.complete);
}
