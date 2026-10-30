//! Deferred dependency witnesses and public complete-project transport.
use meta_language::{
    BinaryLinksOptions, LinkNetwork, StructuredDataLinksLimits, decode_link_network_binary,
    decode_structured_data_links, draft_project_translation, encode_link_network_binary,
    project_translation_draft_to_links,
};
use serde_json::{Value, json};
#[path = "../../src/project_translation_rules.rs"]
mod rules;
fn fixture() -> Value {
    meta_language::read_structured_data_lino(include_str!(
        "../../../parity/fixtures/project-dependency-gaps.lino"
    ))
    .unwrap()
}
fn numbers(value: &Value) -> Vec<f64> {
    value
        .as_array()
        .unwrap()
        .iter()
        .map(|value| value.as_f64().unwrap())
        .collect()
}
fn inputs(data: &Value) -> Value {
    json!([
     {"identifier":"a.js","language":"JavaScript","source":data["source"],"sourceIssues":[data["sourceIssue"]]},
     {"identifier":"b.js","language":"JavaScript","source":data["source"],"imports":{"./a.js":"a.js"}},
     {"identifier":"c.js","language":"JavaScript","source":data["source"],"imports":{"./b.js":"b.js"}},
     {"identifier":"d.js","language":"JavaScript","source":data["source"]}
    ])
}
#[test]
fn complete_dependency_witness_decisions() {
    super::translation_string_array_copy_sort::require_rust_profile();
    let data = fixture();
    for row in data["cases"].as_array().unwrap() {
        assert_eq!(
            rules::plan_project_translation_dependency_gaps(
                row["count"].as_f64().unwrap(),
                &numbers(&row["failed"]),
                &numbers(&row["importers"]),
                &numbers(&row["providers"]),
                row["limit"].as_f64().unwrap()
            ),
            numbers(&row["expected"]),
            "{}",
            row["name"]
        );
    }
}
#[test]
fn public_project_output_retains_gaps_and_both_codecs() {
    super::translation_string_array_copy_sort::require_rust_profile();
    let data = fixture();
    let draft =
        draft_project_translation(&inputs(&data), &json!({"targetLanguage":"JavaScript"})).unwrap();
    let modules = draft["modules"].as_array().unwrap();
    let module = |name: &str| {
        modules
            .iter()
            .find(|module| module["identifier"] == name)
            .unwrap()
    };
    assert_eq!(module("a.js")["sourceIssues"], json!([data["sourceIssue"]]));
    assert!(
        module("a.js")["gaps"]
            .as_array()
            .unwrap()
            .iter()
            .any(|gap| gap["diagnostic"]["name"] == "SourceInventoryGap"
                && gap["diagnostic"]["message"] == data["sourceIssue"])
    );
    for (name, provider) in [("b.js", "a.js"), ("c.js", "b.js")] {
        let value = module(name);
        assert!(value["emittedCode"].is_null());
        assert!(value["draftCode"].is_string());
        assert!(
            value["gaps"]
                .as_array()
                .unwrap()
                .iter()
                .any(|gap| gap["diagnostic"]["name"] == "DependencySourceGap"
                    && gap["diagnostic"]["provider"] == provider
                    && gap["diagnostic"]["root"] == "a.js")
        );
    }
    assert!(module("d.js")["emittedCode"].is_string());
    let packet =
        project_translation_draft_to_links(&draft, StructuredDataLinksLimits::default()).unwrap();
    let text = LinkNetwork::from_lino(&packet.network.to_lino()).unwrap();
    let binary = decode_link_network_binary(
        &encode_link_network_binary(&packet.network, BinaryLinksOptions::default()).unwrap(),
        BinaryLinksOptions::default(),
    )
    .unwrap();
    for network in [&packet.network, &text, &binary] {
        assert_eq!(
            decode_structured_data_links(network, packet.root).unwrap(),
            draft
        );
    }
}
#[test]
fn explicit_budget_and_issue_domain_refusals() {
    super::translation_string_array_copy_sort::require_rust_profile();
    let data = fixture();
    let sources = inputs(&data);
    let draft = draft_project_translation(
        &sources,
        &json!({"targetLanguage":"JavaScript","dependencyWorkLimit":1}),
    )
    .unwrap();
    for module in draft["modules"].as_array().unwrap() {
        assert!(module["emittedCode"].is_null());
        assert!(
            module["gaps"]
                .as_array()
                .unwrap()
                .iter()
                .any(|gap| gap["diagnostic"]["name"] == "DependencyWorkLimitGap")
        );
    }
    for invalid in [json!([""]), json!([1]), json!("issue")] {
        let mut inputs = sources.clone();
        inputs[0]["sourceIssues"] = invalid;
        assert!(
            draft_project_translation(&inputs, &json!({"targetLanguage":"JavaScript"}))
                .unwrap_err()
                .contains("invalid source inventory issues")
        );
    }
    assert!(
        draft_project_translation(
            &sources,
            &json!({"targetLanguage":"JavaScript","dependencyWorkLimit":0})
        )
        .unwrap_err()
        .contains("invalid drafting resource limit")
    );
}
#[test]
fn actual_provider_failure_and_duplicate_issues_keep_their_origins() {
    super::translation_string_array_copy_sort::require_rust_profile();
    let data = fixture();
    let mut sources = inputs(&data);
    sources[0] = json!({"identifier":"a.js","language":"Rust","source":"pub fn value()->u8{unknown_operation()}"});
    let draft =
        draft_project_translation(&sources, &json!({"targetLanguage":"JavaScript"})).unwrap();
    let modules = draft["modules"].as_array().unwrap();
    let module = |name: &str| {
        modules
            .iter()
            .find(|module| module["identifier"] == name)
            .unwrap()
    };
    assert!(!module("a.js")["gaps"].as_array().unwrap().is_empty());
    assert_eq!(module("a.js")["sourceIssues"], json!([]));
    for identifier in ["b.js", "c.js"] {
        assert!(module(identifier)["emittedCode"].is_null());
        assert!(
            module(identifier)["gaps"]
                .as_array()
                .unwrap()
                .iter()
                .any(|gap| gap["diagnostic"]["name"] == "DependencySourceGap"
                    && gap["diagnostic"]["root"] == "a.js")
        );
    }
    assert!(module("d.js")["emittedCode"].is_string());
    let repeated = json!([{"identifier":"d.js","language":"JavaScript","source":data["source"],"sourceIssues":[data["sourceIssue"],data["sourceIssue"]]}]);
    let draft =
        draft_project_translation(&repeated, &json!({"targetLanguage":"JavaScript"})).unwrap();
    assert_eq!(
        draft["modules"][0]["sourceIssues"],
        json!([data["sourceIssue"], data["sourceIssue"]])
    );
    assert_eq!(
        draft["modules"][0]["gaps"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|gap| gap["diagnostic"]["name"] == "SourceInventoryGap")
            .count(),
        2
    );
    let oversized = json!([{"identifier":"d.js","language":"JavaScript","source":data["source"],"sourceIssues":["é".repeat(40)]}]);
    assert!(
        draft_project_translation(
            &oversized,
            &json!({"targetLanguage":"JavaScript","byteLimit":64})
        )
        .unwrap_err()
        .contains("source issue byte resource limit exceeded")
    );
}
