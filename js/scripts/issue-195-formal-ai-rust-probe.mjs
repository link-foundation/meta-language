// The Rust probe js/scripts/run-formal-ai-workloads.mjs copies into the pinned
// link-assistant/formal-ai checkout as rust/tests/issue_195_meta_language_probe.rs.
// It compiles in the same patched cargo invocation as formal-ai's workload tests,
// so formal-ai's own functions (grammar_kinds::parse_network and
// named_kind_histogram, agentic_coding::apply_link_edit,
// rust_projection::{project, projection_from}) run on the unpacked
// meta-language crate. It reads the inputs file the runner writes from
// formal-ai's data (js/scripts/issue-195-formal-ai-workload-probes.mjs) and
// writes the same outputs the JavaScript consumer writes, plus the
// distinctionsPreserved checks.

/** The test target name of the Rust probe inside formal-ai's rust/tests. */
export const RUST_PROBE_TARGET = 'issue_195_meta_language_probe';

/** The Rust test of each probe output. */
export const RUST_PROBE_TESTS = Object.freeze({
  outputs: 'issue_195_probe_outputs',
  distinctionsPreserved: 'issue_195_probe_distinctions',
});

/** The environment variables the probe reads. */
export const RUST_PROBE_ENVIRONMENT = Object.freeze({
  inputs: 'ISSUE_195_FORMAL_AI_INPUTS',
  directory: 'ISSUE_195_FORMAL_AI_PROBE_DIRECTORY',
});

/**
 * The Rust probe. `issue_195_probe_outputs` writes rust-outputs.json;
 * `issue_195_probe_distinctions` writes distinctionsPreserved.json and then
 * fails when any of its checks does not hold.
 */
export const RUST_PROBE = String.raw`//! Issue 195 consumer probe, copied into link-assistant/formal-ai's rust/tests
//! by meta-language's js/scripts/run-formal-ai-workloads.mjs and compiled
//! against the unpacked meta-language crate the runner patches in.

use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;

use formal_ai::agentic_coding::{LinkEditRule, apply_link_edit};
use formal_ai::grammar_kinds::{named_kind_histogram, parse_network};
use formal_ai::rust_projection::{GrammarProjection, ProjectionOutcome, project, projection_from};
use meta_language::{LinkNetwork, LinkQuery, LinkType, QueryMatch, TranslationRuleSet};
use serde_json::{Map, Value, json};

const REFUSAL_LANGUAGE: &str = "grammar-projection-refusal";
const NOFORM_LANGUAGE: &str = "grammar-projection-noform";

fn inputs() -> Value {
    let path = std::env::var_os("ISSUE_195_FORMAL_AI_INPUTS").expect("the runner names the inputs file");
    let text = std::fs::read_to_string(path).expect("the inputs file is readable");
    serde_json::from_str(&text).expect("the inputs file is JSON")
}

fn record(name: &str, evidence: &Value) {
    if let Some(directory) = std::env::var_os("ISSUE_195_FORMAL_AI_PROBE_DIRECTORY") {
        let path = PathBuf::from(directory).join(format!("{name}.json"));
        let text = serde_json::to_string_pretty(evidence).expect("the evidence serializes");
        std::fs::write(path, text).expect("the evidence is written");
    }
}

fn text<'a>(value: &'a Value, key: &str) -> &'a str {
    value[key].as_str().unwrap_or_default()
}

fn strings(value: &Value) -> Vec<String> {
    value.as_array().map_or_else(Vec::new, |items| {
        items.iter().filter_map(Value::as_str).map(str::to_owned).collect()
    })
}

fn syntax_links(network: &LinkNetwork) -> impl Iterator<Item = &meta_language::Link> {
    network.links().filter(|link| link.metadata().link_type() == Some(LinkType::Syntax))
}

fn node_label(network: &LinkNetwork, id: meta_language::LinkId) -> String {
    network.link(id).map_or_else(
        || "? ?".to_owned(),
        |link| {
            let metadata = link.metadata();
            let term = metadata.term().unwrap_or("undefined");
            metadata.span().map_or_else(
                || format!("{term} ?"),
                |span| format!("{term} {}..{}", span.byte_range().start(), span.byte_range().end()),
            )
        },
    )
}

fn named_nodes(network: &LinkNetwork) -> Vec<String> {
    let mut nodes: Vec<String> = syntax_links(network)
        .filter(|link| link.metadata().is_named() && link.metadata().span().is_some())
        .map(|link| node_label(network, link.id()))
        .collect();
    nodes.sort();
    nodes
}

fn document_output(document: &Value) -> Value {
    let language = text(document, "language");
    let source = text(document, "text");
    // formal-ai's own parse: the binding compiles only when it hands out the patched crate's network.
    let network: LinkNetwork = parse_network(language, source);
    let nodes = named_nodes(&network);
    let kinds: Map<String, Value> = named_kind_histogram(&network)
        .into_iter()
        .map(|(kind, count)| (kind, json!(count)))
        .collect();
    let reload = match LinkNetwork::from_lino(&network.to_lino()) {
        Ok(reloaded) => {
            let again = named_nodes(&reloaded);
            json!({
                "namedNodes": again.len(),
                "sameNamedNodes": again == nodes,
                "roundTrip": reloaded.reconstruct_text() == source,
            })
        }
        Err(error) => json!({ "error": error.to_string() }),
    };
    json!({
        "language": language,
        "roundTrip": network.reconstruct_text() == source,
        "clean": network.verify_full_match(None).is_clean(),
        "namedKinds": kinds,
        "namedNodes": nodes,
        "reload": reload,
    })
}

fn rule_set_output(source: &str) -> Value {
    match TranslationRuleSet::from_lino(source) {
        Ok(set) => {
            let rules: Vec<&str> = set.rules().iter().map(meta_language::TranslationRule::name).collect();
            let reload = TranslationRuleSet::from_lino(&set.to_lino()).map_or_else(
                |error| json!({ "error": error.to_string() }),
                |again| json!(again.rules().iter().map(meta_language::TranslationRule::name).collect::<Vec<_>>()),
            );
            json!({ "rules": rules, "reloadRules": reload })
        }
        Err(error) => json!({ "error": error.to_string() }),
    }
}

fn seed_network_output(source: &str) -> Value {
    match LinkNetwork::from_lino(source) {
        Ok(network) => {
            let count = |accept: &dyn Fn(&meta_language::LinkMetadata) -> bool| {
                network.links().filter(|link| accept(link.metadata())).count()
            };
            json!({
                "translationRules": count(&|metadata| metadata.term() == Some("translation-rule")),
                "refusalRows": count(&|metadata| metadata.language() == Some(REFUSAL_LANGUAGE)),
                "noformRows": count(&|metadata| metadata.language() == Some(NOFORM_LANGUAGE)),
            })
        }
        Err(error) => json!({ "error": error.to_string() }),
    }
}

fn match_label(network: &LinkNetwork, found: &QueryMatch) -> String {
    let mut captures: Vec<String> = found
        .captures()
        .iter()
        .filter(|capture| capture.name() != "match")
        .map(|capture| format!("{}={}", capture.name(), node_label(network, capture.link_id())))
        .collect();
    captures.sort();
    format!("{} {{{}}}", node_label(network, found.link_id()), captures.join(", "))
}

fn query_outputs(rules: &[Value], documents: &[Value]) -> Value {
    let mut networks: BTreeMap<String, LinkNetwork> = BTreeMap::new();
    let mut outputs = Map::new();
    for rule in rules {
        let sources = strings(&rule["sources"]);
        let mut output = json!({ "asWritten": null, "adapted": null, "documents": {} });
        match LinkQuery::from_sexpression(text(rule, "sexpression")) {
            Ok(query) => {
                for document in documents.iter().filter(|document| sources.iter().any(|source| source == text(document, "language"))) {
                    let name = text(document, "name").to_owned();
                    let network = networks
                        .entry(name.clone())
                        .or_insert_with(|| parse_network(text(document, "language"), text(document, "text")));
                    let mut labels: Vec<String> = network.find(&query).iter().map(|found| match_label(network, found)).collect();
                    labels.sort();
                    output["documents"][name] = json!(labels);
                }
            }
            Err(error) => output["asWritten"] = json!(error.to_string()),
        }
        outputs.insert(text(rule, "name").to_owned(), output);
    }
    Value::Object(outputs)
}

fn link_edit_rule(rule: &Value) -> LinkEditRule {
    let field = |key: &str| text(rule, key).to_owned();
    match text(rule, "shape") {
        "insert_member" => LinkEditRule::InsertMember { list: field("list"), member: field("member") },
        "replace_literal" => LinkEditRule::ReplaceLiteral { old: field("old"), new: field("new") },
        _ => LinkEditRule::RenameIdentifier { old: field("old"), new: field("new") },
    }
}

fn link_edit_outputs(cases: &[Value]) -> Value {
    let mut outputs = Map::new();
    for case in cases {
        let language = text(case, "language");
        let mut current = text(case, "source").to_owned();
        let mut steps = Vec::new();
        for step in case["steps"].as_array().map_or(&[][..], Vec::as_slice) {
            match apply_link_edit(&current, language, &link_edit_rule(&step["rule"])) {
                Ok((edited, report)) => {
                    let reparsed = parse_network(language, &edited);
                    steps.push(json!({
                        "outcome": "edited",
                        "source": edited,
                        "edits": report.edits,
                        "roundTripBefore": report.round_trip_before,
                        "cleanAfter": report.clean_after,
                        "reparsed": {
                            "roundTrip": reparsed.reconstruct_text() == edited,
                            "namedNodes": named_nodes(&reparsed),
                        },
                    }));
                    current = edited;
                }
                Err(error) => steps.push(json!({ "outcome": "refused", "error": error.to_string() })),
            }
        }
        outputs.insert(text(case, "name").to_owned(), json!({ "steps": steps }));
    }
    Value::Object(outputs)
}

fn projection_outputs(cases: &[Value]) -> Value {
    let mut outputs = Map::new();
    for case in cases {
        let output = match project(text(case, "from"), text(case, "target"), text(case, "source")) {
            ProjectionOutcome::Rendered { source, .. } => json!({ "outcome": "rendered", "source": source }),
            ProjectionOutcome::Refused { refusals } => json!({
                "outcome": "refused",
                "refusals": refusals.iter().map(|refusal| refusal.construct.clone()).collect::<Vec<_>>(),
            }),
        };
        outputs.insert(text(case, "name").to_owned(), output);
    }
    Value::Object(outputs)
}

fn array(value: &Value) -> &[Value] {
    value.as_array().map_or(&[][..], Vec::as_slice)
}

#[test]
fn issue_195_probe_outputs() {
    let inputs = inputs();
    let documents: Map<String, Value> = array(&inputs["documents"])
        .iter()
        .map(|document| (text(document, "name").to_owned(), document_output(document)))
        .collect();
    let outputs = json!({
        "documents": documents,
        "ruleSet": rule_set_output(text(&inputs, "ruleSetText")),
        "seedNetwork": seed_network_output(text(&inputs, "ruleSetText")),
        "queries": query_outputs(array(&inputs["rules"]), array(&inputs["documents"])),
        "linkEdits": link_edit_outputs(array(&inputs["linkEdits"])),
        "projections": projection_outputs(array(&inputs["projections"])),
    });
    record("rust-outputs", &outputs);
}

fn check(name: &str, failures: &[String], detail: Value) -> Value {
    json!({
        "name": name,
        "holds": failures.is_empty(),
        "detail": if failures.is_empty() { detail } else { json!(failures.iter().take(8).collect::<Vec<_>>()) },
    })
}

fn kinds_of(documents: &[Value], label: &str) -> BTreeSet<String> {
    documents
        .iter()
        .filter(|document| text(document, "language") == label && text(document, "name").starts_with("template_"))
        .flat_map(|document| named_kind_histogram(&parse_network(label, text(document, "text"))).into_iter().map(|(kind, _)| kind))
        .collect()
}

fn classification_failures(projection: &GrammarProjection, inputs: &Value) -> Vec<String> {
    let labels = strings(&inputs["projectionLabels"]);
    let classification = &inputs["classification"];
    let ruled: BTreeSet<(String, String)> = array(&classification["ruled"])
        .iter()
        .map(|row| (text(row, "kind").to_owned(), text(row, "target").to_owned()))
        .collect();
    let ruled_kinds: BTreeSet<&str> = ruled.iter().map(|(kind, _)| kind.as_str()).collect();
    let mut failures = Vec::new();
    for (kind, target) in &ruled {
        if !projection.is_ruled(kind, target) {
            failures.push(format!("{kind} is not ruled for {target}"));
        }
    }
    for row in array(&classification["refused"]) {
        let kind = text(row, "kind");
        let targets = if text(row, "target") == "any" { labels.clone() } else { vec![text(row, "target").to_owned()] };
        for target in &targets {
            if !projection.is_refused(kind, target) {
                failures.push(format!("{kind} is not refused for {target}"));
            }
            if !ruled_kinds.contains(kind) && projection.is_ruled(kind, target) {
                failures.push(format!("the refused-only {kind} is ruled for {target}"));
            }
        }
    }
    for row in array(&classification["noform"]) {
        let (kind, target) = (text(row, "kind"), text(row, "target"));
        if !projection.is_noform(kind, target) {
            failures.push(format!("{kind} is not no-form for {target}"));
        }
        if projection.is_ruled(kind, target) {
            failures.push(format!("the no-form {kind} is ruled for {target}"));
        }
    }
    let counts = [
        ("rules", projection.rule_count(), array(&inputs["rules"]).len()),
        ("refusals", projection.refusal_count(), array(&classification["refused"]).len()),
        ("no-form rows", projection.noform_count(), array(&classification["noform"]).len()),
    ];
    for (name, actual, expected) in counts {
        if actual != expected {
            failures.push(format!("{actual} {name}, the seed declares {expected}"));
        }
    }
    failures
}

fn projection_failures(cases: &[Value]) -> Vec<String> {
    let mut failures = Vec::new();
    for case in cases {
        let expect = &case["expect"];
        let outcome = project(text(case, "from"), text(case, "target"), text(case, "source"));
        let holds = match (&outcome, text(expect, "outcome")) {
            (ProjectionOutcome::Rendered { source, .. }, "rendered") => source == text(expect, "source"),
            (ProjectionOutcome::Refused { refusals }, "refused") => {
                refusals.iter().any(|refusal| refusal.construct == text(expect, "construct"))
                    && expect["refusals"].as_u64().is_none_or(|count| refusals.len() as u64 == count)
            }
            _ => false,
        };
        if !holds {
            failures.push(format!("{}: {outcome:?}", text(case, "name")));
        }
    }
    failures
}

fn link_edit_failures(cases: &[Value]) -> Vec<String> {
    let mut failures = Vec::new();
    for case in cases {
        let language = text(case, "language");
        let mut current = text(case, "source").to_owned();
        for step in array(&case["steps"]) {
            let expect = &step["expect"];
            let outcome = apply_link_edit(&current, language, &link_edit_rule(&step["rule"]));
            let holds = match (&outcome, text(expect, "outcome")) {
                (Ok((edited, report)), "edited") => {
                    expect["edits"].as_u64() == Some(report.edits as u64) && edited.contains(text(expect, "contains"))
                }
                (Err(error), "refused") => error.to_string() == text(expect, "error"),
                _ => false,
            };
            if !holds {
                failures.push(format!("{}: {outcome:?}", text(case, "name")));
            }
            if let Ok((edited, _)) = outcome {
                current = edited;
            }
        }
    }
    failures
}

#[test]
fn issue_195_probe_distinctions() {
    let inputs = inputs();
    let documents = array(&inputs["documents"]);
    let mut language_failures = Vec::new();
    for document in documents {
        let language = text(document, "language");
        let network = parse_network(language, text(document, "text"));
        let foreign = syntax_links(&network).filter(|link| link.metadata().language() != Some(language)).count();
        if foreign > 0 {
            language_failures.push(format!("{}: {foreign} syntax links carry another language", text(document, "name")));
        }
    }
    let kinds: BTreeMap<String, BTreeSet<String>> = ["rust", "javascript", "typescript"]
        .into_iter()
        .map(|label| (label.to_owned(), kinds_of(documents, label)))
        .collect();
    let mut kind_failures = Vec::new();
    let mut kind_detail = Map::new();
    for (left, right) in [("typescript", "javascript"), ("rust", "javascript"), ("rust", "typescript")] {
        let only: Vec<&String> = kinds[left].difference(&kinds[right]).collect();
        if only.is_empty() {
            kind_failures.push(format!("no {left} kind is missing from {right}"));
        }
        kind_detail.insert(format!("{left} without {right}"), json!(only.len()));
    }
    let classification = projection_from(text(&inputs, "ruleSetText"))
        .map_or_else(|error| vec![format!("the seed does not load: {error}")], |projection| {
            classification_failures(&projection, &inputs)
        });
    let checks = vec![
        check("every syntax link keeps its document's language label", &language_failures, json!({ "documents": documents.len() })),
        check("rust, javascript and typescript keep kinds of their own", &kind_failures, Value::Object(kind_detail)),
        check("ruled, refused and no-form kinds stay distinct per target", &classification, json!({
            "ruled": array(&inputs["classification"]["ruled"]).len(),
            "refused": array(&inputs["classification"]["refused"]).len(),
            "noform": array(&inputs["classification"]["noform"]).len(),
        })),
        check("projections render ruled constructs and refuse the rest by name", &projection_failures(array(&inputs["projections"])), json!({
            "cases": array(&inputs["projections"]).len(),
        })),
        check("link edits stay distinct from their refusals", &link_edit_failures(array(&inputs["linkEdits"])), json!({
            "cases": array(&inputs["linkEdits"]).len(),
        })),
    ];
    let failed: Vec<&str> = checks
        .iter()
        .filter(|check| check["holds"] != json!(true))
        .filter_map(|check| check["name"].as_str())
        .collect();
    record("distinctionsPreserved", &json!({ "holds": failed.is_empty(), "checks": checks }));
    assert_eq!(failed, Vec::<&str>::new());
}
`;
