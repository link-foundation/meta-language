//! Requirement I195-GRAMMAR-FEATURE-UNION: merging and lowering keep the
//! declarations and rule fields of the feature union. The interchange section
//! of parity/fixtures/grammar-feature-union.json records, from the JavaScript
//! runtime, the merged grammar, decisions and alternatives of each merge case
//! and the metadata steps and negative-control failures of the lowering; this
//! suite compares the Rust outcome with it byte for byte, as
//! js/tests/issue-195-grammar-feature-union.test.js does for JavaScript.

use meta_language::{
    GRAMMAR_LOWERING_FORMATS, Grammar, GrammarLoweringOptions, GrammarMergeOptions,
    GrammarMergeSource, check_grammar_lowering, lower_grammar, merge_grammars,
    parse_native_grammar, render_native_grammar,
};
use serde_json::Value;

use super::issue_195_observations::{Observation, record};

const FIXTURE: &str = include_str!("../../../parity/fixtures/grammar-feature-union.json");

fn text<'a>(value: &'a Value, key: &str) -> &'a str {
    value[key]
        .as_str()
        .unwrap_or_else(|| panic!("{key} is text"))
}

fn native(listing: &str) -> Grammar {
    parse_native_grammar(listing).expect("the listing parses")
}

fn to_json<T: serde::Serialize>(value: &T) -> Value {
    serde_json::to_value(value).expect("serializes")
}

fn unhonored(line: &str) -> bool {
    line.starts_with("(declarations ") || line.starts_with("(attributes ")
}

fn without_steps(metadata: &str) -> String {
    metadata
        .split('\n')
        .filter(|line| !unhonored(line))
        .collect::<Vec<_>>()
        .join("\n")
}

fn check_merge(item: &Value) {
    let id = text(item, "id");
    let sources: Vec<GrammarMergeSource> = item["sources"]
        .as_array()
        .expect("sources")
        .iter()
        .map(|source| {
            let precedence = source["precedence"].as_u64().expect("precedence");
            GrammarMergeSource::new(
                text(source, "id"),
                "quoted",
                native(text(source, "listing")),
            )
            .with_precedence(u32::try_from(precedence).expect("small precedence"))
        })
        .collect();
    let result = merge_grammars(&sources, &GrammarMergeOptions::default()).expect("merges");
    assert_eq!(result.groups.len(), 1, "{id}");
    let group = &result.groups[0];
    assert_eq!(
        render_native_grammar(&group.grammar),
        text(item, "merged"),
        "{id}: merged grammar"
    );
    assert_eq!(
        to_json(&group.decisions),
        item["decisions"],
        "{id}: decisions"
    );
    assert_eq!(
        to_json(&group.alternatives),
        item["alternatives"],
        "{id}: alternatives"
    );
}

fn check_lowering(lowering: &Value) {
    let formats: Vec<&str> = lowering["formats"]
        .as_array()
        .expect("formats")
        .iter()
        .map(|format| format.as_str().expect("format"))
        .collect();
    assert_eq!(formats, GRAMMAR_LOWERING_FORMATS);
    let steps: Vec<&str> = lowering["steps"]
        .as_array()
        .expect("steps")
        .iter()
        .map(|step| step.as_str().expect("step"))
        .collect();
    let dropped: Vec<&str> = lowering["dropped"]
        .as_array()
        .expect("dropped")
        .iter()
        .map(|detail| detail.as_str().expect("detail"))
        .collect();
    let grammar = native(text(lowering, "listing"));
    for format in formats {
        let lowered =
            lower_grammar(&grammar, format, &GrammarLoweringOptions::default()).expect("lowers");
        assert_eq!(
            lowered.status.as_str(),
            text(lowering, "status"),
            "{format}: status"
        );
        let lines: Vec<&str> = lowered
            .metadata
            .split('\n')
            .filter(|line| unhonored(line))
            .collect();
        assert_eq!(lines, steps, "{format}: steps");
        let report = check_grammar_lowering(&grammar, format, &GrammarLoweringOptions::default())
            .expect("checks");
        assert_eq!(report.failures, Vec::new(), "{format}: reconstructs");
        // Negative control: without the steps the declarations and fields are lost.
        let edit = |metadata: &str| without_steps(metadata);
        let options = GrammarLoweringOptions {
            edit_metadata: Some(&edit),
            ..GrammarLoweringOptions::default()
        };
        let report = check_grammar_lowering(&grammar, format, &options).expect("checks");
        let details: Vec<String> = report
            .failures
            .iter()
            .map(|failure| format!("{}: {}", failure.kind.as_str(), failure.detail))
            .collect();
        assert_eq!(details, dropped, "{format}: dropped");
    }
}

#[test]
fn merging_and_lowering_keep_the_feature_union_declarations_and_rule_fields() {
    let fixture: Value = serde_json::from_str(FIXTURE).expect("the feature union fixture is JSON");
    let interchange = &fixture["interchange"];
    let merge = interchange["merge"].as_array().expect("merge cases");
    assert_ne!(merge.as_slice(), &[] as &[Value]);
    for item in merge {
        check_merge(item);
    }
    check_lowering(&interchange["lowering"]);
    record(&Observation {
        requirement_id: "I195-GRAMMAR-FEATURE-UNION",
        suffix: "behavior",
        fixture_id: "planned:repository-directive:i195-grammar-feature-union",
        fixture_file: "parity/fixtures/grammar-feature-union.json",
        assertions: &["everyUnionFeatureRepresented", "negativeCasesRejected"],
        test_name: "merging_and_lowering_keep_the_feature_union_declarations_and_rule_fields",
    });
}
