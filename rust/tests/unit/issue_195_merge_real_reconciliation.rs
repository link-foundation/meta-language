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
    FeatureParseOptions, GrammarMergeDecisionKind, GrammarMergeOptions, GrammarMergeResult,
    GrammarMergeSource, assert_merge_shares, import_antlr, import_pest, merge_grammars,
    parse_grammar_links, parse_with_grammar, shared_rule_decisions,
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
    assert_eq!(strict.reused, [] as [String; 0]);
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
    assert_eq!(
        shared_rule_decisions(&merged("xml").groups[0]),
        [] as [&meta_language::GrammarMergeDecision; 0]
    );
    observe(
        &["sharedRulesNonZero"],
        "the name tier strips a leading language name",
    );
}

#[test]
fn shared_source_production_union_preserves_syntax_and_distinct_meanings() {
    let fixture: Value = serde_json::from_str(&read(
        "parity/fixtures/grammar-source-production-union.json",
    ))
    .expect("source union fixture");
    for case in fixture["cases"].as_array().expect("cases") {
        let list: Vec<GrammarMergeSource> = case["sources"]
            .as_array()
            .expect("sources")
            .iter()
            .enumerate()
            .map(|(precedence, source)| {
                let grammar = if text(source, "format") == "pest" {
                    import_pest(text(source, "text")).expect("pest source imports")
                } else {
                    parse_grammar_links(text(source, "text")).expect("links source imports")
                };
                GrammarMergeSource::new(text(source, "id"), "Example", grammar)
                    .with_precedence(u32::try_from(precedence).expect("small precedence"))
            })
            .collect();
        let result = merge(&list, true);
        let group = &result.groups[0];
        assert_eq!(
            group.grammar.rules().len(),
            usize::try_from(case["rules"].as_u64().expect("rule count")).expect("small count")
        );
        for source in case["accepted"].as_array().expect("accepted") {
            assert!(
                parse_with_grammar(
                    &group.grammar,
                    source.as_str().expect("source").as_bytes(),
                    &FeatureParseOptions::default()
                )
                .is_ok(),
                "{}: {source}",
                text(case, "name")
            );
        }
        for source in case["rejected"].as_array().expect("rejected") {
            assert!(
                parse_with_grammar(
                    &group.grammar,
                    source.as_str().expect("source").as_bytes(),
                    &FeatureParseOptions::default()
                )
                .is_err(),
                "{}: {source}",
                text(case, "name")
            );
        }
        for pair in case["shared"].as_array().expect("shared") {
            assert_eq!(
                group.identities[pair[0].as_str().expect("alias")],
                group.identities[pair[1].as_str().expect("alias")]
            );
        }
        if let Some(pairs) = case["distinct"].as_array() {
            for pair in pairs {
                assert_ne!(
                    group.identities[pair[0].as_str().expect("alias")],
                    group.identities[pair[1].as_str().expect("alias")]
                );
            }
        }
        assert_eq!(
            group.source_productions.len(),
            list.iter()
                .map(|source| source.grammar.rules().len())
                .sum::<usize>()
        );
    }
}

#[test]
fn the_merge_forwards_formal_operational_premises_and_retains_unknown_origins() {
    let fixture: Value = serde_json::from_str(&read("parity/fixtures/global-concept-space.json"))
        .expect("formal premise fixture");
    let list: Vec<GrammarMergeSource> = fixture["sources"]
        .as_array()
        .expect("sources")
        .iter()
        .map(|entry| {
            let source = meta_language::GlobalConceptSource::from_json(entry)
                .expect("fixture grammar source");
            GrammarMergeSource::new(source.id, source.language, source.grammar)
                .with_edition(source.edition)
        })
        .collect();
    let mut options = GrammarMergeOptions {
        concept_records: Some(fixture["records"].clone()),
        formal_definitions: Some(fixture["formalDefinitions"].clone()),
        global_equivalences: fixture["equivalences"]
            .as_array()
            .expect("equivalences")
            .iter()
            .map(|pair| {
                [
                    pair[0].as_str().expect("origin").to_owned(),
                    pair[1].as_str().expect("origin").to_owned(),
                ]
            })
            .collect(),
        ..GrammarMergeOptions::default()
    };
    let grounded = merge_grammars(&list, &options).expect("the formally grounded merge runs");
    assert_eq!(grounded.status(), "complete");
    assert_eq!(grounded.global_concept_space, fixture["expectedSpace"]);
    options.formal_definitions = None;
    let unknown = merge_grammars(&list, &options).expect("unknown concepts remain copied");
    assert_eq!(unknown.status(), "incomplete");
    assert!(
        unknown.global_concept_space["proofs"]
            .as_array()
            .expect("proofs")
            .is_empty()
    );
    assert_eq!(
        unknown.global_concept_space["formalDefinitions"],
        serde_json::json!({})
    );
    assert_eq!(
        unknown.global_concept_space["concepts"]
            .as_array()
            .expect("concepts")
            .len(),
        6
    );
    assert_eq!(
        unknown.global_concept_space["mappings"]
            .as_object()
            .expect("origins")
            .len(),
        6
    );
}
