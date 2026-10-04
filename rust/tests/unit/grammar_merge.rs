//! Multi-source grammar merge and binding-aware rename: equivalent rules are
//! merged only on a recursive structural proof, homonyms and lookalikes stay
//! distinct and explicit, merging is reproducible, idempotent, order
//! independent and incremental, and renaming follows references, recursion,
//! qualified names and reloads while keeping source aliases (requirements
//! I195-MERGE-DETERMINISM, I195-MERGE-MEANING-AWARE-DEDUPLICATION,
//! I195-MERGE-BINDING-AWARE-RENAME and I195-MERGE-UNCERTAINTY-PRESERVED). The
//! JavaScript twin is js/tests/grammar-merge.test.js.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::PathBuf;

use meta_language::{
    GRAMMAR_MERGE_METHOD, Grammar, GrammarExpr, GrammarFormat, GrammarMergeDecision,
    GrammarMergeDecisionKind, GrammarMergeFailureReason, GrammarMergeOptions, GrammarMergeResult,
    GrammarMergeSource, GrammarRenameErrorKind, GrammarRule, MergedGrammarGroup, RenamedGrammar,
    RuleAlias, RuleKind, assert_merge_complete, grammar_from_lino, grammar_to_lino, import_bnf,
    import_pest, merge_grammars, normalized_rule_definition, rename_grammar_rule,
    restore_source_names,
};
use serde_json::{Value, json};

use super::issue_195_observations::{Observation, record};

const FIXTURE: &str = "parity/fixtures/grammar-merge.json";

fn observe(requirement_id: &str, assertions: &[&str], test_name: &str) {
    let fixture_id = format!(
        "planned:repository-directive:{}",
        requirement_id.to_ascii_lowercase()
    );
    record(&Observation {
        requirement_id,
        suffix: "behavior",
        fixture_id: &fixture_id,
        fixture_file: FIXTURE,
        assertions,
        test_name,
    });
}

fn fixture() -> Value {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join(FIXTURE);
    serde_json::from_str(&fs::read_to_string(path).expect("grammar merge fixture"))
        .expect("valid grammar merge fixture")
}

fn text(value: &Value) -> &str {
    value.as_str().expect("fixture string")
}

fn source_from(entry: &Value, pest: &str) -> GrammarMergeSource {
    assert_eq!(entry["format"], "pest");
    GrammarMergeSource::new(
        text(&entry["id"]),
        text(&entry["language"]),
        import_pest(pest).expect("fixture pest grammar imports"),
    )
    .with_edition(text(&entry["edition"]))
    .with_precedence(
        u32::try_from(entry["precedence"].as_u64().expect("precedence")).expect("small precedence"),
    )
}

fn sources(fixture: &Value) -> Vec<GrammarMergeSource> {
    fixture["sources"]
        .as_array()
        .expect("sources")
        .iter()
        .map(|entry| source_from(entry, text(&entry["text"])))
        .collect()
}

fn changed_sources(fixture: &Value) -> Vec<GrammarMergeSource> {
    let change = &fixture["upstreamChange"];
    fixture["sources"]
        .as_array()
        .expect("sources")
        .iter()
        .map(|entry| {
            if entry["id"] == change["source"] {
                source_from(entry, text(&change["text"]))
            } else {
                source_from(entry, text(&entry["text"]))
            }
        })
        .collect()
}

fn samples(fixture: &Value) -> BTreeMap<String, Vec<String>> {
    serde_json::from_value(fixture["samples"].clone()).expect("samples")
}

fn pairs(value: &Value) -> Vec<(String, String)> {
    serde_json::from_value(value.clone()).expect("alias pairs")
}

fn options<'a>(fixture: &Value) -> GrammarMergeOptions<'a> {
    GrammarMergeOptions {
        samples: samples(fixture),
        ..GrammarMergeOptions::default()
    }
}

fn merge(fixture: &Value, list: &[GrammarMergeSource]) -> GrammarMergeResult {
    merge_grammars(list, &options(fixture)).expect("fixture merge")
}

fn group<'a>(result: &'a GrammarMergeResult, key: &str) -> &'a MergedGrammarGroup {
    result.group(key).expect("merged group")
}

fn decisions(
    entry: &MergedGrammarGroup,
    kind: GrammarMergeDecisionKind,
) -> Vec<&GrammarMergeDecision> {
    entry
        .decisions
        .iter()
        .filter(|decision| decision.kind == kind)
        .collect()
}

fn to_json<T: serde::Serialize>(value: &T) -> Value {
    serde_json::to_value(value).expect("serializable merge record")
}

fn rule<'a>(grammar: &'a Grammar, name: &str) -> &'a GrammarRule {
    grammar.rule(name).expect("rule exists")
}

#[test]
fn the_merge_decides_every_fixture_group_exactly_as_recorded() {
    let fixture = fixture();
    let result = merge(&fixture, &sources(&fixture));
    let expected = &fixture["expected"];
    assert_eq!(result.status(), "complete");
    assert_eq!(to_json(&result.alternatives), expected["alternatives"]);
    let keys: Vec<&str> = result
        .groups
        .iter()
        .map(|entry| entry.key.as_str())
        .collect();
    let expected_keys: Vec<&str> = expected["groups"]
        .as_array()
        .expect("groups")
        .iter()
        .map(|entry| text(&entry["key"]))
        .collect();
    assert_eq!(keys, expected_keys);
    for wanted in expected["groups"].as_array().expect("groups") {
        let key = text(&wanted["key"]);
        let actual = group(&result, key);
        assert_eq!(to_json(&actual.sources), wanted["sources"], "{key}");
        assert_eq!(
            actual
                .grammar
                .start()
                .map_or(Value::Null, |start| json!(start)),
            wanted["start"],
            "{key}"
        );
        assert_eq!(
            to_json(&actual.grammar.rule_names()),
            wanted["rules"],
            "{key}"
        );
        assert_eq!(to_json(&actual.identities), wanted["identities"], "{key}");
        assert_eq!(to_json(&actual.decisions), wanted["decisions"], "{key}");
        assert_eq!(to_json(&actual.nominations), wanted["nominations"], "{key}");
        assert_eq!(
            to_json(&actual.alternatives),
            wanted["alternatives"],
            "{key}"
        );
    }
}

#[test]
fn merging_is_reproducible_idempotent_and_independent_of_the_source_order() {
    let fixture = fixture();
    let first = merge(&fixture, &sources(&fixture));
    let second = merge(&fixture, &sources(&fixture));
    assert_eq!(second, first);
    assert_eq!(second.groups[0].fingerprint, first.groups[0].fingerprint);
    assert_eq!(first.groups[0].fingerprint.len(), 64);
    observe(
        "I195-MERGE-DETERMINISM",
        &["reproducibleOutput"],
        "merging is reproducible",
    );

    for entry in &first.groups {
        let again = merge(
            &fixture,
            &[
                GrammarMergeSource::new("merged", &entry.language, entry.grammar.clone())
                    .with_edition(&entry.edition),
            ],
        );
        assert_eq!(again.groups[0].grammar, entry.grammar, "{}", entry.key);
        assert_eq!(
            decisions(&again.groups[0], GrammarMergeDecisionKind::Merged),
            Vec::<&GrammarMergeDecision>::new(),
            "{}",
            entry.key
        );
    }
    let reused = merge_grammars(
        &sources(&fixture),
        &GrammarMergeOptions {
            previous: Some(&first),
            ..options(&fixture)
        },
    )
    .expect("re-merge");
    assert_eq!(reused.groups, first.groups);
    assert_eq!(reused.recomputed, Vec::<String>::new());
    observe(
        "I195-MERGE-DETERMINISM",
        &["idempotent"],
        "merging a merged grammar changes nothing",
    );

    let mut reversed_sources = sources(&fixture);
    reversed_sources.reverse();
    assert_eq!(merge(&fixture, &reversed_sources), first);
    let reordered: Vec<GrammarMergeSource> = sources(&fixture)
        .into_iter()
        .map(|mut source| {
            let start = source
                .grammar
                .start_rule()
                .expect("start rule")
                .name
                .clone();
            let mut grammar = Grammar::new();
            for rule in source.grammar.rules().iter().rev() {
                grammar.add_rule(rule.clone());
            }
            grammar.set_start(start);
            source.grammar = grammar.with_source_format(GrammarFormat::Peg);
            source
        })
        .collect();
    let reordered_result = merge(&fixture, &reordered);
    for entry in &first.groups {
        let other = group(&reordered_result, &entry.key);
        for (alias, name) in &other.identities {
            assert_eq!(Some(name), entry.identities.get(alias), "{alias}");
        }
        let names = |grammar: &Grammar| -> BTreeSet<String> {
            grammar
                .rule_names()
                .into_iter()
                .map(str::to_owned)
                .collect()
        };
        assert_eq!(names(&other.grammar), names(&entry.grammar));
        for name in entry.grammar.rule_names() {
            assert_eq!(
                rule(&other.grammar, name).expr,
                rule(&entry.grammar, name).expr,
                "{name}"
            );
        }
    }
    observe(
        "I195-MERGE-DETERMINISM",
        &["stableUnderReordering"],
        "merging ignores the order of sources and rules",
    );
}

#[test]
fn an_upstream_change_re_merges_only_its_group_and_keeps_canonical_identities() {
    let fixture = fixture();
    let previous = merge(&fixture, &sources(&fixture));
    let remerged = merge_grammars(
        &changed_sources(&fixture),
        &GrammarMergeOptions {
            previous: Some(&previous),
            ..options(&fixture)
        },
    )
    .expect("incremental merge");
    let expected = &fixture["expected"]["remerge"];
    assert_eq!(to_json(&remerged.reused), expected["reused"]);
    assert_eq!(to_json(&remerged.recomputed), expected["recomputed"]);
    for key in &remerged.reused {
        assert_eq!(group(&remerged, key), group(&previous, key));
    }
    let changed_key = text(&expected["recomputed"][0]);
    let changed = group(&remerged, changed_key);
    assert_ne!(
        changed.fingerprint,
        group(&previous, changed_key).fingerprint
    );
    assert_eq!(to_json(&changed.identities), expected["identities"]);
    assert_eq!(to_json(&changed.grammar.rule_names()), expected["rules"]);
    assert!(
        changed.grammar.rule("product").is_some(),
        "the new upstream rule is merged in"
    );
    observe(
        "I195-MERGE-DETERMINISM",
        &["incrementalRemerge"],
        "an upstream change re-merges only its group",
    );

    // Without the previous result the renamed upstream rule would found a new
    // canonical name; with it, the established equivalence keeps its identity.
    let fresh = merge(&fixture, &changed_sources(&fixture));
    assert_eq!(
        to_json(&group(&fresh, changed_key).identities),
        expected["identitiesWithoutPrevious"]
    );
    assert_eq!(
        changed.identities.get("upstream-a:numeral"),
        previous.groups[0].identities.get("upstream-a:number")
    );
    assert!(changed.grammar.rule("numeral").is_none());
    for (alias, name) in &previous.groups[0].identities {
        if let Some(current) = changed.identities.get(alias) {
            assert_eq!(current, name, "{alias}");
        }
    }
    for entry in &remerged.groups {
        let mut classes = BTreeSet::new();
        for decision in entry.decisions.iter().filter(|decision| {
            matches!(
                decision.kind,
                GrammarMergeDecisionKind::Merged | GrammarMergeDecisionKind::KeptUnique
            )
        }) {
            assert!(
                classes.insert(decision.name.clone()),
                "{} names one class",
                decision.name
            );
            for alias in &decision.members {
                assert_eq!(entry.identities[alias], decision.name, "{alias}");
            }
        }
        let rules: BTreeSet<String> = entry
            .grammar
            .rule_names()
            .into_iter()
            .map(str::to_owned)
            .collect();
        assert_eq!(classes, rules);
    }
    observe(
        "I195-MERGE-DETERMINISM",
        &["noDuplicateCanonicalIdentities"],
        "established equivalences keep one canonical identity",
    );
}

#[test]
fn language_editions_are_merged_separately_and_listed_as_explicit_alternatives() {
    let fixture = fixture();
    let result = merge(&fixture, &sources(&fixture));
    let editions: Vec<&str> = result
        .groups
        .iter()
        .filter(|entry| entry.language == "arithmetic")
        .map(|entry| entry.edition.as_str())
        .collect();
    assert_eq!(editions, ["2024", "2025"]);
    let older = group(&result, "arithmetic@2024");
    let newer = group(&result, "arithmetic@2025");
    assert!(
        older
            .identities
            .keys()
            .all(|alias| !alias.starts_with("upstream-c:"))
    );
    let newer_aliases: BTreeSet<&str> = newer.identities.keys().map(String::as_str).collect();
    assert_eq!(
        newer_aliases,
        BTreeSet::from([
            "upstream-c:expression",
            "upstream-c:term",
            "upstream-c:number"
        ])
    );
    assert_ne!(
        rule(&newer.grammar, "number").expr,
        rule(&older.grammar, "number").expr
    );
    assert_eq!(
        to_json(&result.alternatives),
        json!([{ "reason": "edition", "name": "arithmetic", "options": ["2024", "2025"] }])
    );
    observe(
        "I195-MERGE-DETERMINISM",
        &["editionBoundariesPreserved"],
        "language editions are merged separately",
    );
}

#[test]
fn differently_named_equivalent_rules_are_merged_recursively_and_after_normalization() {
    let fixture = fixture();
    let result = merge(&fixture, &sources(&fixture));
    let older = group(&result, "arithmetic@2024");
    let merged: Vec<(&str, Vec<&str>)> = decisions(older, GrammarMergeDecisionKind::Merged)
        .into_iter()
        .map(|decision| {
            (
                decision.name.as_str(),
                decision.members.iter().map(String::as_str).collect(),
            )
        })
        .collect();
    assert_eq!(
        merged,
        [
            (
                "expression",
                vec!["upstream-a:expression", "upstream-b:sum"]
            ),
            ("term", vec!["upstream-a:term", "upstream-b:operand"]),
            ("number", vec!["upstream-a:number", "upstream-b:integer"]),
        ]
    );
    // `ASCII_DIGIT ~ ASCII_DIGIT*` and `ASCII_DIGIT+` mean the same, and the
    // mutually recursive pair sum/operand matches expression/term.
    let list = sources(&fixture);
    let definition = |source: &GrammarMergeSource, name: &str| {
        normalized_rule_definition(rule(&source.grammar, name)).expect("normalizes")
    };
    assert_eq!(
        definition(&list[0], "number"),
        definition(&list[1], "integer")
    );
    assert_ne!(
        definition(&list[0], "expression"),
        definition(&list[1], "sum")
    );
    for name in ["sum", "operand", "integer"] {
        assert!(older.grammar.rule(name).is_none(), "{name}");
    }
    observe(
        "I195-MERGE-MEANING-AWARE-DEDUPLICATION",
        &["equivalentConceptsMerged"],
        "differently named equivalent rules are merged",
    );

    for decision in decisions(older, GrammarMergeDecisionKind::Merged) {
        assert_eq!(decision.basis, GRAMMAR_MERGE_METHOD);
        let definition = decision.definition.as_deref().expect("definition");
        assert!(
            definition.starts_with("normal:") || definition.starts_with("atomic:"),
            "{definition}"
        );
        assert_eq!(
            normalized_rule_definition(rule(&older.grammar, &decision.name)).expect("normalizes"),
            definition
        );
    }
    observe(
        "I195-MERGE-MEANING-AWARE-DEDUPLICATION",
        &["equivalenceJustified"],
        "every merge records its proof method and definition",
    );
}

#[test]
fn rules_with_the_same_name_and_different_meanings_are_kept_apart() {
    let fixture = fixture();
    let result = merge(&fixture, &sources(&fixture));
    let older = group(&result, "arithmetic@2024");
    assert_eq!(
        to_json(&decisions(
            older,
            GrammarMergeDecisionKind::HomonymKeptDistinct
        )),
        json!([{
            "kind": "homonym-kept-distinct",
            "name": "identifier",
            "members": ["identifier", "identifier_from_upstream_b"],
            "basis": "different-definitions",
            "definition": null,
        }])
    );
    assert_eq!(older.identities["upstream-a:identifier"], "identifier");
    assert_eq!(
        older.identities["upstream-b:identifier"],
        "identifier_from_upstream_b"
    );
    assert_ne!(
        rule(&older.grammar, "identifier").expr,
        rule(&older.grammar, "identifier_from_upstream_b").expr
    );
    let renamed: Vec<&Vec<String>> =
        decisions(older, GrammarMergeDecisionKind::RenamedForCollision)
            .into_iter()
            .map(|decision| &decision.members)
            .collect();
    assert_eq!(to_json(&renamed), json!([["upstream-b:identifier"]]));
    observe(
        "I195-MERGE-MEANING-AWARE-DEDUPLICATION",
        &["homonymsKeptDistinct"],
        "rules with the same name and different meanings are kept apart",
    );
}

#[test]
fn similar_names_and_identical_samples_nominate_candidates_but_never_decide_a_merge() {
    let fixture = fixture();
    let result = merge(&fixture, &sources(&fixture));
    let older = group(&result, "arithmetic@2024");
    let nominations: Vec<(&str, Value)> = older
        .nominations
        .iter()
        .map(|nomination| (nomination.basis.as_str(), to_json(&nomination.outcome)))
        .collect();
    assert_eq!(
        nominations,
        [
            ("name-similarity", json!("unproven")),
            ("identical-samples", json!("proven")),
            ("identical-samples", json!("unproven")),
        ]
    );
    // The proven nomination was merged by the structural proof, the unproven
    // ones stay separate rules.
    assert_eq!(
        older.identities["upstream-a:number"],
        older.identities["upstream-b:integer"]
    );
    assert_ne!(
        older.identities["upstream-a:sign"],
        older.identities["upstream-b:operator"]
    );
    let without = merge_grammars(&sources(&fixture), &GrammarMergeOptions::default())
        .expect("merge without samples");
    let without_samples = group(&without, "arithmetic@2024");
    assert_eq!(without_samples.identities, older.identities);
    assert_eq!(without_samples.grammar, older.grammar);
    observe(
        "I195-MERGE-MEANING-AWARE-DEDUPLICATION",
        &["samplesOnlyNominate"],
        "samples only nominate candidates",
    );
}

#[test]
fn uncertain_matches_stay_separate_and_every_alternative_is_explicit() {
    let fixture = fixture();
    let result = merge(&fixture, &sources(&fixture));
    let older = group(&result, "arithmetic@2024");
    let uncertain: Vec<(&str, Vec<&str>)> = decisions(older, GrammarMergeDecisionKind::Uncertain)
        .into_iter()
        .map(|decision| {
            (
                decision.basis.as_str(),
                decision.members.iter().map(String::as_str).collect(),
            )
        })
        .collect();
    assert_eq!(
        uncertain,
        [
            (
                "name-similarity",
                vec!["upstream-a:string_literal", "upstream-b:stringLiteral"]
            ),
            (
                "identical-samples",
                vec!["upstream-a:sign", "upstream-b:operator"]
            ),
        ]
    );
    for name in ["string_literal", "stringLiteral", "sign", "operator"] {
        assert!(older.grammar.rule(name).is_some(), "{name}");
    }
    observe(
        "I195-MERGE-UNCERTAINTY-PRESERVED",
        &["uncertainMatchesNotConflated"],
        "uncertain matches stay separate",
    );

    let alternatives: Vec<(Value, Vec<&str>)> = older
        .alternatives
        .iter()
        .map(|alternative| {
            (
                to_json(&alternative.reason),
                alternative.options.iter().map(String::as_str).collect(),
            )
        })
        .collect();
    assert_eq!(
        alternatives,
        [
            (json!("start-rule"), vec!["statement"]),
            (
                json!("distinct-meaning"),
                vec!["identifier", "identifier_from_upstream_b"]
            ),
            (
                json!("uncertain-match"),
                vec!["string_literal", "stringLiteral"]
            ),
            (json!("uncertain-match"), vec!["sign", "operator"]),
        ]
    );
    assert_eq!(older.grammar.start(), Some("expression"));
    assert!(
        older.grammar.rule("statement").is_some(),
        "the alternative start rule is kept"
    );
    assert_eq!(
        to_json(&result.alternatives),
        json!([{ "reason": "edition", "name": "arithmetic", "options": ["2024", "2025"] }])
    );
    observe(
        "I195-MERGE-UNCERTAINTY-PRESERVED",
        &["alternativesExplicit"],
        "every alternative is explicit",
    );
}

#[test]
fn an_unresolved_required_equivalence_fails_the_merge() {
    let fixture = fixture();
    let required = |equivalences: Vec<(String, String)>| {
        merge_grammars(
            &sources(&fixture),
            &GrammarMergeOptions {
                required_equivalences: equivalences,
                ..options(&fixture)
            },
        )
        .expect("merge with required equivalences")
    };
    let result = required(pairs(&fixture["requiredEquivalences"]));
    assert_eq!(result.status(), "incomplete");
    assert_eq!(to_json(&result.failures), fixture["expected"]["failures"]);
    let error = assert_merge_complete(&result).expect_err("incomplete merge fails");
    assert_eq!(error.failures(), result.failures.as_slice());
    assert!(error.to_string().contains("not-proven"), "{error}");

    let proven = required(vec![(
        "upstream-b:integer".to_owned(),
        "upstream-a:number".to_owned(),
    )]);
    assert_eq!(proven.status(), "complete");
    assert_eq!(assert_merge_complete(&proven), Ok(&proven));
    let unknown = required(vec![(
        "upstream-a:number".to_owned(),
        "upstream-b:missing".to_owned(),
    )]);
    let reasons: Vec<GrammarMergeFailureReason> = unknown
        .failures
        .iter()
        .map(|failure| failure.reason)
        .collect();
    assert_eq!(reasons, [GrammarMergeFailureReason::UnknownRule]);
    observe(
        "I195-MERGE-UNCERTAINTY-PRESERVED",
        &["unresolvedEquivalenceFails"],
        "an unresolved required equivalence fails the merge",
    );
}

// Builds a grammar from the normalized JSON grammar shape used by the
// JavaScript runtime (`serializeGrammar`).
fn grammar_from_json(value: &Value) -> Grammar {
    let mut grammar = Grammar::new();
    for entry in value["rules"].as_array().expect("rules") {
        let kind = match text(&entry["kind"]) {
            "normal" => RuleKind::Normal,
            "atomic" => RuleKind::Atomic,
            "silent" => RuleKind::Silent,
            "token" => RuleKind::Token,
            other => panic!("unknown rule kind {other}"),
        };
        grammar.add_rule(
            GrammarRule::new(text(&entry["name"]), expr_from_json(&entry["expression"]))
                .with_kind(kind),
        );
    }
    if let Some(start) = value["start"].as_str() {
        grammar.set_start(start);
    }
    assert_eq!(value["sourceFormat"], "peg");
    grammar.with_source_format(GrammarFormat::Peg)
}

fn expr_from_json(value: &Value) -> GrammarExpr {
    let items = || {
        value["items"]
            .as_array()
            .expect("items")
            .iter()
            .map(expr_from_json)
            .collect::<Vec<_>>()
    };
    let item = || expr_from_json(&value["item"]);
    let character = |key: &str| text(&value[key]).chars().next().expect("character");
    match text(&value["kind"]) {
        "empty" => GrammarExpr::Empty,
        "literal" => GrammarExpr::terminal(text(&value["value"])),
        "any" => GrammarExpr::AnyChar,
        "ref" => GrammarExpr::non_terminal(text(&value["name"])),
        "charRange" => GrammarExpr::CharRange(character("start"), character("end")),
        "seq" => GrammarExpr::Sequence(items()),
        "choice" => GrammarExpr::choice(value["ordered"] == true, items()),
        "optional" => GrammarExpr::optional(item()),
        "repeat0" => GrammarExpr::zero_or_more(item()),
        "repeat1" => GrammarExpr::one_or_more(item()),
        "and" => GrammarExpr::and(item()),
        "not" => GrammarExpr::not(item()),
        "capture" => value["label"].as_str().map_or_else(
            || GrammarExpr::capture_unlabeled(item()),
            |label| GrammarExpr::capture(label, item()),
        ),
        other => panic!("unsupported expression kind {other}"),
    }
}

fn rename_steps(
    fixture: &Value,
    grammar: Grammar,
    steps: &[Value],
    aliases: Vec<RuleAlias>,
) -> RenamedGrammar {
    let namespace = fixture["rename"]["namespace"].as_str();
    steps
        .iter()
        .fold(RenamedGrammar { grammar, aliases }, |state, step| {
            rename_grammar_rule(
                &state.grammar,
                text(&step["from"]),
                text(&step["to"]),
                namespace,
                &state.aliases,
            )
            .expect("fixture rename succeeds")
        })
}

fn steps(fixture: &Value) -> Vec<Value> {
    fixture["rename"]["steps"]
        .as_array()
        .expect("steps")
        .clone()
}

fn sequence(expr: &GrammarExpr) -> &[GrammarExpr] {
    match expr {
        GrammarExpr::Sequence(items) => items,
        other => panic!("expected a sequence, got {other:?}"),
    }
}

fn alternatives(expr: &GrammarExpr) -> &[GrammarExpr] {
    match expr {
        GrammarExpr::Choice { alternatives, .. } => alternatives,
        other => panic!("expected a choice, got {other:?}"),
    }
}

#[test]
fn a_merged_unordered_choice_keeps_the_source_order_of_its_alternatives() {
    // The comparison form of an unordered choice is order-free, but the
    // merged grammar is what parsers run: a PEG-style parser commits to the
    // first alternative that matches, so a sorted `letter | letter word`
    // would stop after one letter.
    let grammar = import_bnf("<word> ::= <letter> <word> | <letter>\n<letter> ::= \"b\" | \"a\"\n")
        .expect("BNF imports");
    let source = GrammarMergeSource::new("words", "words", grammar.clone());
    let result = merge_grammars(&[source], &GrammarMergeOptions::default()).expect("merges");
    let merged = &result.groups[0].grammar;
    for name in ["word", "letter"] {
        assert_eq!(rule(merged, name).expr, rule(&grammar, name).expr, "{name}");
    }
    let reversed =
        import_bnf("<word> ::= <letter> | <letter> <word>\n<letter> ::= \"a\" | \"b\"\n")
            .expect("BNF imports");
    assert_eq!(
        normalized_rule_definition(rule(&reversed, "word")).expect("normalizes"),
        normalized_rule_definition(rule(&grammar, "word")).expect("normalizes")
    );
}

#[test]
fn renaming_follows_recursive_references_qualified_names_and_captures() {
    let fixture = fixture();
    let original = grammar_from_json(&fixture["rename"]["grammar"]);
    let renamed = rename_steps(&fixture, original, &steps(&fixture), Vec::new());
    let grammar = &renamed.grammar;
    assert_eq!(grammar.start(), Some("sum"));
    let sum = sequence(&rule(grammar, "sum").expr);
    let GrammarExpr::Optional(tail) = &sum[1] else {
        panic!("sum tail is optional");
    };
    assert_eq!(sequence(tail)[1], GrammarExpr::non_terminal("sum"));
    let operand = alternatives(&rule(grammar, "operand").expr);
    assert_eq!(sequence(&operand[1])[1], GrammarExpr::non_terminal("sum"));
    assert_eq!(
        sequence(&rule(grammar, "spaced").expr)[1],
        GrammarExpr::non_terminal("operand")
    );
    observe(
        "I195-MERGE-BINDING-AWARE-RENAME",
        &["recursiveRulesRenamed"],
        "renaming follows recursive references",
    );

    assert_eq!(
        alternatives(&rule(grammar, "qualified").expr),
        [
            GrammarExpr::non_terminal("arithmetic.operand"),
            GrammarExpr::non_terminal("arithmetic::operand"),
            GrammarExpr::non_terminal("library.value"),
        ]
    );
    observe(
        "I195-MERGE-BINDING-AWARE-RENAME",
        &["qualifiedReferencesRenamed"],
        "renaming follows qualified references",
    );

    // The capture label `value` names a local binding, not the rule: it stays,
    // while the reference it wraps follows the rule.
    assert_eq!(
        sum[0],
        GrammarExpr::capture("value", GrammarExpr::non_terminal("operand"))
    );
    observe(
        "I195-MERGE-BINDING-AWARE-RENAME",
        &["shadowingHandled"],
        "capture labels shadowing a rule are not renamed",
    );
}

#[test]
fn renaming_refuses_collisions_and_unknown_rules() {
    let fixture = fixture();
    let case = &fixture["rename"];
    let original = grammar_from_json(&case["grammar"]);
    let kinds: Vec<&str> = case["collisions"]
        .as_array()
        .expect("collisions")
        .iter()
        .map(|collision| {
            rename_grammar_rule(
                &original,
                text(&collision["from"]),
                text(&collision["to"]),
                case["namespace"].as_str(),
                &[],
            )
            .map_or_else(|error| error.kind().as_str(), |_| "accepted")
        })
        .collect();
    assert_eq!(to_json(&kinds), case["expected"]["collisionKinds"]);
    assert_eq!(
        original,
        grammar_from_json(&case["grammar"]),
        "a refused rename leaves the grammar unchanged"
    );
    for invalid in ["", "two words"] {
        assert_eq!(
            rename_grammar_rule(&original, "value", invalid, None, &[])
                .expect_err("invalid name")
                .kind(),
            GrammarRenameErrorKind::InvalidName
        );
    }
    let restore = restore_source_names(
        &original,
        &[RuleAlias {
            canonical: "value".to_owned(),
            original: "number".to_owned(),
        }],
        None,
    );
    assert_eq!(
        restore.expect_err("restoring onto an existing rule").kind(),
        GrammarRenameErrorKind::Collision
    );
    observe(
        "I195-MERGE-BINDING-AWARE-RENAME",
        &["collisionsHandled"],
        "renaming refuses collisions",
    );
}

#[test]
fn renaming_works_on_a_reloaded_grammar_and_keeps_source_aliases_for_export() {
    let fixture = fixture();
    let case = &fixture["rename"];
    let original = grammar_from_json(&case["grammar"]);
    let renamed = rename_steps(&fixture, original.clone(), &steps(&fixture), Vec::new());
    let reloaded =
        grammar_from_lino(&grammar_to_lino(&renamed.grammar)).expect("renamed grammar reloads");
    assert_eq!(reloaded, renamed.grammar);
    let after_reload = [case["afterReload"].clone()];
    let last = rename_steps(&fixture, reloaded, &after_reload, renamed.aliases);
    assert_eq!(
        last.grammar,
        grammar_from_json(&case["expected"]["grammar"])
    );
    observe(
        "I195-MERGE-BINDING-AWARE-RENAME",
        &["referencesAfterReloadRenamed"],
        "renaming works on a reloaded grammar",
    );

    assert_eq!(to_json(&last.aliases), case["expected"]["aliases"]);
    let exported = restore_source_names(&last.grammar, &last.aliases, case["namespace"].as_str())
        .expect("source names restore");
    assert_eq!(exported, original);
    observe(
        "I195-MERGE-BINDING-AWARE-RENAME",
        &["sourceAliasesKept"],
        "source aliases restore the original names",
    );
}

#[test]
fn malformed_merge_input_is_rejected() {
    let fixture = fixture();
    let first = sources(&fixture).remove(0);
    let reject = |list: &[GrammarMergeSource]| {
        merge_grammars(list, &GrammarMergeOptions::default()).expect_err("malformed input")
    };
    reject(&[first.clone(), first.clone()]);
    let mut colon = first.clone();
    colon.id = "has:colon".to_owned();
    reject(&[colon]);
    let mut unnamed = first.clone();
    unnamed.language = String::new();
    reject(&[unnamed]);
    let mut duplicated = first.clone();
    duplicated
        .grammar
        .add_rule(rule(&first.grammar, "number").clone());
    reject(&[duplicated]);
    let mut bounds = first;
    bounds.grammar.add_rule(GrammarRule::new(
        "inverted",
        GrammarExpr::repeat(GrammarExpr::terminal("x"), 3, Some(1)),
    ));
    reject(&[bounds]);
}
