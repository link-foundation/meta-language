//! Merging real grammars of one language reconciles their corresponding rules
//! instead of concatenating them (requirement I195-MERGE-REAL-RECONCILIATION).
//! Each pair of parity/grammars/reconcile/pairs.json is a shipped native
//! grammar and the grammars-v4 ANTLR grammar of its language. With
//! `reconcile: true` the merge unites the listed rules under one canonical
//! name and one concept record, so the pair shares rules; the strict merge of
//! the same pair shares none, and `assert_merge_shares` rejects it as a
//! concatenation. The JavaScript twin is
//! js/tests/issue-195-merge-real-reconciliation.test.js.

use std::collections::BTreeSet;
use std::fs;
use std::path::PathBuf;

use meta_language::{
    GrammarMergeDecisionKind, GrammarMergeOptions, GrammarMergeResult, GrammarMergeSource,
    assert_merge_shares, import_antlr, import_pest, merge_grammars, parse_grammar_links,
    shared_rule_decisions,
};
use serde_json::Value;

use super::issue_195_observations::{Observation, record};

const FIXTURE: &str = "parity/grammars/reconcile/pairs.json";

fn observe(assertions: &[&str], test_name: &str) {
    record(&Observation {
        requirement_id: "I195-MERGE-REAL-RECONCILIATION",
        suffix: "behavior",
        fixture_id: "planned:repository-directive:i195-merge-real-reconciliation",
        fixture_file: FIXTURE,
        assertions,
        test_name,
    });
}

fn read(file: &str) -> String {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join(file);
    fs::read_to_string(&path).unwrap_or_else(|error| panic!("{}: {error}", path.display()))
}

fn pairs() -> Vec<Value> {
    let fixture: Value = serde_json::from_str(&read(FIXTURE)).expect("the pairs are JSON");
    fixture["pairs"].as_array().expect("pairs").clone()
}

fn text<'a>(value: &'a Value, key: &str) -> &'a str {
    value[key]
        .as_str()
        .unwrap_or_else(|| panic!("{key} is a string"))
}

fn sources(pair: &Value) -> Vec<GrammarMergeSource> {
    let language = text(pair, "language");
    let native =
        parse_grammar_links(&read(text(pair, "native"))).expect("the native grammar reads");
    let mut antlr =
        import_antlr(&read(text(pair, "grammarsV4"))).expect("the ANTLR grammar imports");
    antlr.set_start(text(pair, "entryPoint"));
    vec![
        GrammarMergeSource::new("native", language, native),
        GrammarMergeSource::new("grammars-v4", language, antlr).with_precedence(1),
    ]
}

fn merge(sources: &[GrammarMergeSource], reconcile: bool) -> GrammarMergeResult {
    merge_grammars(
        sources,
        &GrammarMergeOptions {
            reconcile,
            ..GrammarMergeOptions::default()
        },
    )
    .expect("the pair merges")
}

fn rule_count(sources: &[GrammarMergeSource]) -> usize {
    sources
        .iter()
        .map(|source| source.grammar.rules().len())
        .sum()
}

fn corresponding(pair: &Value) -> Vec<[&str; 3]> {
    pair["corresponding"]
        .as_array()
        .expect("corresponding")
        .iter()
        .map(|entry| {
            let entry = entry.as_array().expect("a correspondence");
            [0, 1, 2].map(|index| entry[index].as_str().expect("a correspondence field"))
        })
        .collect()
}

#[test]
fn corresponding_rules_of_the_native_and_grammars_v4_grammars_are_reconciled() {
    for pair in pairs() {
        let language = text(&pair, "language");
        let list = sources(&pair);
        let result = merge(&list, true);
        assert_eq!(result.status(), "complete", "{language}");
        let group = &result.groups[0];
        let rows = corresponding(&pair);
        for [native, antlr, basis] in &rows {
            let name = &group.identities[&format!("native:{native}")];
            assert_eq!(
                &group.identities[&format!("grammars-v4:{antlr}")],
                name,
                "{language}: {antlr} is reconciled with {native}"
            );
            let decision = group
                .decisions
                .iter()
                .find(|entry| {
                    &entry.name == name && entry.kind == GrammarMergeDecisionKind::Reconciled
                })
                .unwrap_or_else(|| panic!("{language}: {name} is a reconciled decision"));
            assert_eq!(decision.basis, *basis, "{language}: {name}");
            assert_eq!(
                decision.members,
                [format!("native:{native}"), format!("grammars-v4:{antlr}")],
                "{language}: {name}"
            );
            // The merged rule is the native rule, and both sources' rules now
            // stand for one concept record.
            assert_eq!(
                group.grammar.rule(name).expect("the merged rule").concept,
                list[0]
                    .grammar
                    .rule(native)
                    .expect("the native rule")
                    .concept,
                "{language}: {name}"
            );
        }
        for (antlr, concept) in pair["concepts"].as_object().expect("concepts") {
            let name = &group.identities[&format!("grammars-v4:{antlr}")];
            assert_eq!(
                group
                    .grammar
                    .rule(name)
                    .expect("the merged rule")
                    .concept
                    .as_deref(),
                concept.as_str(),
                "{language}: {antlr}"
            );
        }
        // A reconciled pair is no evidence about the other rules of the
        // sources: every other rule of a source stays its own.
        let reconciled: BTreeSet<String> = rows
            .iter()
            .flat_map(|[native, antlr, _]| {
                [format!("native:{native}"), format!("grammars-v4:{antlr}")]
            })
            .collect();
        for decision in group
            .decisions
            .iter()
            .filter(|decision| decision.kind == GrammarMergeDecisionKind::Reconciled)
        {
            for member in &decision.members {
                assert!(
                    reconciled.contains(member),
                    "{language}: {member} is reconciled only as listed"
                );
            }
        }
        assert_eq!(
            group.grammar.rules().len(),
            rule_count(&list) - rows.len(),
            "{language}"
        );
        observe(
            &["correspondingRulesReconciled"],
            &format!("{language} corresponding rules are reconciled"),
        );
    }
}

#[test]
fn the_reconciled_merge_shares_rules_and_the_strict_merge_is_rejected_as_a_concatenation() {
    for pair in pairs() {
        let language = text(&pair, "language");
        let list = sources(&pair);
        let reconciled = merge(&list, true);
        assert_eq!(
            shared_rule_decisions(&reconciled.groups[0]).len(),
            corresponding(&pair).len(),
            "{language}"
        );
        assert!(assert_merge_shares(&reconciled).is_ok(), "{language}");
        observe(
            &["sharedRulesNonZero"],
            &format!("{language} reconciled merge shares rules"),
        );

        let strict = merge(&list, false);
        assert!(
            shared_rule_decisions(&strict.groups[0]).is_empty(),
            "{language}"
        );
        assert_eq!(
            strict.groups[0].grammar.rules().len(),
            rule_count(&list),
            "{language}"
        );
        let error = assert_merge_shares(&strict).expect_err("a concatenation is rejected");
        assert!(
            error.to_string().contains("only concatenates its sources"),
            "{error}"
        );
        assert_eq!(error.concatenated().len(), 1, "{language}");
        observe(
            &["concatenationRejected"],
            &format!("{language} strict merge is rejected as a concatenation"),
        );
    }
}

#[test]
fn reconciliation_is_deterministic_and_keeps_a_reconciled_group_reusable() {
    let pair = &pairs()[0];
    let first = merge(&sources(pair), true);
    let mut reversed = sources(pair);
    reversed.reverse();
    let second = merge(&reversed, true);
    assert_eq!(second.groups[0].identities, first.groups[0].identities);
    let again = merge_grammars(
        &sources(pair),
        &GrammarMergeOptions {
            reconcile: true,
            previous: Some(&first),
            ..GrammarMergeOptions::default()
        },
    )
    .expect("the pair merges again");
    assert_eq!(again.reused, [first.groups[0].key.clone()]);
    // The fingerprint covers the mode: a strict merge never reuses a
    // reconciled group.
    let strict = merge_grammars(
        &sources(pair),
        &GrammarMergeOptions {
            previous: Some(&first),
            ..GrammarMergeOptions::default()
        },
    )
    .expect("the pair merges strictly");
    assert!(strict.reused.is_empty());
    observe(
        &["correspondingRulesReconciled"],
        "reconciliation is deterministic",
    );
}

#[test]
fn the_name_tier_compares_rule_names_without_a_leading_name_of_their_language() {
    let native = import_pest(
        "document = { element* }\n\
         element = { \"<\" ~ name ~ attribute* ~ \">\" ~ document ~ \"</\" ~ name ~ \">\" }\n\
         attribute = { name ~ \"=\" ~ name }\n\
         name = @{ ASCII_ALPHA+ }\n",
    )
    .expect("the native grammar imports");
    // grammars-v4's HTML grammar names its rules `htmlDocument`, `htmlElement`
    // and `htmlAttribute`; they decompose the constructs differently, so only
    // their names correspond.
    let prefixed = |prefix: &str| {
        import_pest(&format!(
            "{prefix}Document = {{ {prefix}Element+ ~ EOI }}\n\
             {prefix}Element = {{ \"<\" ~ tag ~ {prefix}Attribute* ~ \"/>\" | \"<\" ~ tag ~ \">\" ~ {prefix}Element* ~ \"</\" ~ tag ~ \">\" }}\n\
             {prefix}Attribute = {{ tag ~ (\"=\" ~ tag)? }}\n\
             tag = @{{ ASCII_ALPHA ~ ASCII_ALPHANUMERIC* }}\n"
        ))
        .expect("the prefixed grammar imports")
    };
    let merged = |prefix: &str| {
        merge(
            &[
                GrammarMergeSource::new("native", "HTML", native.clone()),
                GrammarMergeSource::new("grammars-v4", "HTML", prefixed(prefix)).with_precedence(1),
            ],
            true,
        )
    };
    let html = merged("html");
    let shared: Vec<(String, Vec<String>, String)> = shared_rule_decisions(&html.groups[0])
        .into_iter()
        .map(|decision| {
            (
                decision.name.clone(),
                decision.members.clone(),
                decision.basis.clone(),
            )
        })
        .collect();
    let expected: Vec<(String, Vec<String>, String)> = ["document", "element", "attribute"]
        .into_iter()
        .map(|name| {
            let capitalized = format!("{}{}", name[..1].to_uppercase(), &name[1..]);
            (
                name.to_owned(),
                vec![
                    format!("native:{name}"),
                    format!("grammars-v4:html{capitalized}"),
                ],
                "name-correspondence".to_owned(),
            )
        })
        .collect();
    assert_eq!(shared, expected);
    // Another prefix is part of the name, so nothing corresponds.
    assert!(shared_rule_decisions(&merged("xml").groups[0]).is_empty());
    observe(
        &["sharedRulesNonZero"],
        "the name tier strips a leading language name",
    );
}
