//! Shared concepts of the native grammars: every rule of every native grammar
//! names a concept record, a construct that corresponds one to one across
//! native grammars resolves to one concept identity, lookalike constructs stay
//! distinct (I195-GRAMMAR-SHARED-CONCEPTS); both runtimes compute the same
//! per-language reuse report (I195-GRAMMAR-CONCEPT-REUSE-REPORT); a shared
//! construct translates between two native grammars through its concept with
//! no rule for the pair of languages (I195-GRAMMAR-CONCEPT-TRANSLATION); and
//! native rule names are readable English with their tree-sitter names kept
//! as aliases (I195-NAMING-NATIVE-GRAMMARS). The JavaScript twin is
//! js/tests/issue-195-grammar-shared-concepts.test.js.

use std::collections::BTreeMap;
use std::path::PathBuf;

use meta_language::{
    ConceptRecord, ConstructTranslationRelation, ConstructTree, CorrespondenceRelation,
    NativeRuleConcept, SourceAlias, check_native_grammar_concepts,
    check_native_grammar_concepts_in, concept_correspondence, concept_records,
    native_construct_tree, native_grammar, native_grammar_concept_reuse, native_grammar_ids,
    native_grammar_rule_concepts, native_grammar_source, parse_grammar_links,
    translate_native_construct, translate_native_construct_in, translate_native_construct_tree,
};
use serde_json::{Value, json};

use super::issue_195_observations::{Observation, record};

const FIXTURE: &str = "parity/fixtures/grammar-shared-concepts.json";
const REUSE_FIXTURE: &str = "parity/fixtures/native-grammar-concept-reuse.json";
const REUSE_DOCUMENT: &str = "docs/grammar/native-grammar-concept-reuse.md";
const NAMING_FIXTURE: &str = "parity/naming/canonical-concepts.json";

fn observe(requirement_id: &str, fixture_file: &'static str, assertions: &[&str], test_name: &str) {
    let fixture_id = format!(
        "planned:repository-directive:{}",
        requirement_id.to_lowercase()
    );
    record(&Observation {
        requirement_id,
        suffix: "behavior",
        fixture_id: &fixture_id,
        fixture_file,
        assertions,
        test_name,
    });
}

fn read(file: &str) -> String {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join(file);
    std::fs::read_to_string(path).expect("the fixture reads")
}

fn fixture() -> Value {
    serde_json::from_str(&read(FIXTURE)).expect("the fixture is JSON")
}

fn text(value: &Value) -> &str {
    value.as_str().expect("a string")
}

fn rule_of(value: &Value) -> (&str, &str) {
    (text(&value[0]), text(&value[1]))
}

fn alias((grammar, name): (&str, &str)) -> SourceAlias {
    SourceAlias {
        source: native_grammar_source(grammar).expect("a native grammar"),
        name: name.to_owned(),
    }
}

fn concept_of(grammar: &str, rule: &str) -> Option<String> {
    native_grammar_rule_concepts(grammar)
        .into_iter()
        .find(|entry| entry.rule == rule)
        .and_then(|entry| entry.concept)
}

fn construct_tree(grammar: &str, source: &Value) -> ConstructTree {
    native_construct_tree(grammar, text(source)).expect("the native grammar accepts the source")
}

fn problems(
    grammars: &[&str],
    change: &dyn Fn(Vec<NativeRuleConcept>) -> Vec<NativeRuleConcept>,
) -> Vec<String> {
    let rule_concepts = |grammar: &str| {
        let rules = native_grammar_rule_concepts(grammar);
        if grammar == grammars[0] {
            change(rules)
        } else {
            rules
        }
    };
    check_native_grammar_concepts_in(concept_records(), grammars, &rule_concepts)
        .into_iter()
        .map(|problem| format!("{} {}", problem.kind.as_str(), problem.rule))
        .collect()
}

fn with_concept(
    rules: Vec<NativeRuleConcept>,
    rule: &str,
    concept: Option<&str>,
) -> Vec<NativeRuleConcept> {
    rules
        .into_iter()
        .map(|entry| {
            if entry.rule == rule {
                NativeRuleConcept {
                    rule: entry.rule,
                    concept: concept.map(str::to_owned),
                }
            } else {
                entry
            }
        })
        .collect()
}

#[test]
fn every_native_rule_names_a_concept_record_whose_native_aliases_are_its_rules() {
    assert_eq!(check_native_grammar_concepts(), Vec::new());
    let mut rules = 0;
    for grammar in native_grammar_ids() {
        for entry in native_grammar_rule_concepts(grammar) {
            rules += 1;
            let concept = entry.concept.expect("every rule names a concept");
            assert!(
                concept_records().iter().any(|record| record.id == concept),
                "{grammar} {} names {concept}",
                entry.rule
            );
        }
    }
    assert!(rules >= 127, "{rules} native rules");

    let grammars = ["native-json", "native-diff"];
    assert_eq!(
        problems(&grammars, &|rules| with_concept(rules, "pair", None)),
        ["rule-without-concept pair", "alias-without-rule pair"]
    );
    assert_eq!(
        problems(&grammars, &|rules| with_concept(
            rules,
            "pair",
            Some("grammar.pair-of-things")
        )),
        ["concept-without-record pair", "alias-without-rule pair"]
    );
    assert_eq!(
        problems(&grammars, &|mut rules| {
            rules.push(NativeRuleConcept {
                rule: "extra".to_owned(),
                concept: Some("grammar.diff.hunk".to_owned()),
            });
            rules
        }),
        ["rule-without-alias extra", "namespace-mismatch extra"]
    );
    observe(
        "I195-GRAMMAR-SHARED-CONCEPTS",
        FIXTURE,
        &["everyRuleHasConceptRecord", "recordAliasesMatchRules"],
        "every native rule names a concept record",
    );
}

#[test]
fn language_specific_concepts_are_named_by_their_language_alone() {
    let report = native_grammar_concept_reuse();
    for grammar in &report.grammars {
        assert_eq!(
            grammar.rules,
            native_grammar_rule_concepts(&grammar.grammar).len()
        );
        assert_eq!(grammar.shared.len() + grammar.specific.len(), grammar.rules);
        for entry in &grammar.shared {
            assert!(entry.languages.len() > 1 && entry.languages.contains(&grammar.language));
        }
        for entry in &grammar.specific {
            let named = report
                .concepts
                .iter()
                .find(|concept| concept.concept == entry.concept)
                .expect("the concept is listed");
            assert_eq!(
                named.languages.as_slice(),
                std::slice::from_ref(&grammar.language),
                "{} {}",
                grammar.grammar,
                entry.rule
            );
        }
    }
    observe(
        "I195-GRAMMAR-SHARED-CONCEPTS",
        FIXTURE,
        &["languageSpecificConceptsStayInTheirLanguage"],
        "shared and language-specific rules",
    );
}

#[test]
fn constructs_that_correspond_one_to_one_resolve_to_one_concept_identity() {
    let fixture = fixture();
    for construct in fixture["sharedConstructs"]
        .as_array()
        .expect("sharedConstructs")
    {
        let concept = text(&construct["concept"]);
        let rules: Vec<(&str, &str)> = construct["rules"]
            .as_array()
            .expect("rules")
            .iter()
            .map(rule_of)
            .collect();
        for &(grammar, rule) in &rules {
            assert_eq!(
                concept_of(grammar, rule).as_deref(),
                Some(concept),
                "{grammar} {rule}"
            );
        }
        for &first in &rules {
            for &second in &rules {
                let relation = concept_correspondence(&alias(first), &alias(second), None);
                assert_eq!(
                    relation.relation,
                    CorrespondenceRelation::Shared,
                    "{first:?} / {second:?}"
                );
                assert_eq!(relation.shared.as_deref(), Some(concept));
            }
        }
    }
    observe(
        "I195-GRAMMAR-SHARED-CONCEPTS",
        FIXTURE,
        &["sharedConstructsResolveToOneIdentity"],
        "shared constructs resolve to one identity",
    );
}

#[test]
fn lookalike_constructs_stay_distinct_concepts() {
    let fixture = fixture();
    for lookalike in fixture["lookalikes"].as_array().expect("lookalikes") {
        let first = rule_of(&lookalike["first"]);
        let second = rule_of(&lookalike["second"]);
        assert_eq!(
            concept_of(first.0, first.1).as_deref(),
            Some(text(&lookalike["concepts"][0]))
        );
        assert_eq!(
            concept_of(second.0, second.1).as_deref(),
            Some(text(&lookalike["concepts"][1]))
        );
        let relation = concept_correspondence(&alias(first), &alias(second), None);
        assert_eq!(
            relation.relation,
            CorrespondenceRelation::Distinct,
            "{first:?} / {second:?}"
        );
        assert!(
            relation
                .justification
                .is_some_and(|reason| reason.ends_with('.'))
        );
        let translation = translate_native_construct(first.0, first.1, second.0);
        assert!(
            !translation.rules.iter().any(|rule| rule == second.1),
            "{first:?} / {second:?}"
        );
    }
    let merged = problems(&["native-scheme"], &|rules| {
        with_concept(rules, "list", Some("grammar.list"))
    });
    assert_eq!(
        merged,
        ["rule-without-alias list", "alias-without-rule list"]
    );
    observe(
        "I195-GRAMMAR-SHARED-CONCEPTS",
        FIXTURE,
        &["lookalikeConstructsStayDistinct"],
        "lookalike constructs stay distinct",
    );
}

fn renamed(tree: &ConstructTree, out: &mut Vec<String>) {
    out.push(tree.kind.clone());
    for child in &tree.children {
        renamed(child, out);
    }
}

#[test]
fn shared_constructs_translate_through_their_concept_with_no_rule_for_the_pair() {
    let fixture = fixture();
    for translation in fixture["translations"].as_array().expect("translations") {
        let from = text(&translation["from"]);
        let to = text(&translation["to"]);
        let tree = construct_tree(from, &translation["source"]);
        let result = translate_native_construct_tree(&tree, from, to);
        assert_eq!(result.problems, Vec::new(), "{from} → {to}");
        assert_eq!(
            result.tree,
            Some(construct_tree(to, &translation["target"])),
            "{from} → {to}"
        );
        // Without the target's source aliases, no construct translates.
        let target = native_grammar_source(to).expect("a native grammar");
        let blind: Vec<ConceptRecord> = concept_records()
            .iter()
            .cloned()
            .map(|mut record| {
                record.source_aliases.retain(|alias| alias.source != target);
                record
            })
            .collect();
        let mut kinds = Vec::new();
        renamed(&tree, &mut kinds);
        for kind in &kinds {
            assert_eq!(
                translate_native_construct_in(&blind, from, kind, to).relation,
                ConstructTranslationRelation::Untranslatable,
                "{from} {kind}"
            );
        }
        for pair in translation["renamed"].as_array().expect("renamed") {
            let (kind, target_kind) = rule_of(pair);
            let translated = translate_native_construct(from, kind, to);
            assert_eq!(
                translated.relation,
                ConstructTranslationRelation::Translated
            );
            assert_eq!(translated.rules, [target_kind]);
            assert_eq!(translated.concept, concept_of(to, target_kind));
        }
    }
    for case in fixture["untranslatable"]
        .as_array()
        .expect("untranslatable")
    {
        let from = text(&case["from"]);
        let to = text(&case["to"]);
        let result =
            translate_native_construct_tree(&construct_tree(from, &case["source"]), from, to);
        assert_eq!(result.tree, None);
        let found: Vec<Value> = result
            .problems
            .iter()
            .map(|problem| {
                assert_eq!(
                    problem.relation,
                    ConstructTranslationRelation::Untranslatable
                );
                json!([problem.kind, problem.concept])
            })
            .collect();
        assert_eq!(Value::Array(found), case["problems"]);
    }
    observe(
        "I195-GRAMMAR-CONCEPT-TRANSLATION",
        FIXTURE,
        &[
            "constructsTranslateThroughConcepts",
            "noPairwiseRuleUsed",
            "untranslatableConstructsReported",
            "translatedTreesMatchTargetParses",
        ],
        "shared constructs translate through concepts",
    );
}

#[test]
fn the_rust_reuse_report_equals_the_published_report() {
    let report = native_grammar_concept_reuse();
    let grammars: Vec<Value> = report
        .grammars
        .iter()
        .map(|grammar| {
            json!({
                "grammar": grammar.grammar,
                "language": grammar.language,
                "rules": grammar.rules,
                "shared": grammar.shared.iter().map(|entry| json!({
                    "rule": entry.rule, "concept": entry.concept, "languages": entry.languages,
                })).collect::<Vec<_>>(),
                "specific": grammar.specific.iter().map(|entry| json!({
                    "rule": entry.rule, "concept": entry.concept,
                })).collect::<Vec<_>>(),
            })
        })
        .collect();
    let concepts: Vec<Value> = report
        .concepts
        .iter()
        .map(|entry| json!({ "concept": entry.concept, "languages": entry.languages, "shared": entry.shared }))
        .collect();
    let mut published: Value =
        serde_json::from_str(&read(REUSE_FIXTURE)).expect("the report is JSON");
    published
        .as_object_mut()
        .expect("an object")
        .remove("generatedBy");
    assert_eq!(
        json!({ "grammars": grammars, "concepts": concepts }),
        published
    );
    for grammar in &report.grammars {
        let mut listed: Vec<&str> = grammar
            .shared
            .iter()
            .map(|entry| entry.rule.as_str())
            .chain(grammar.specific.iter().map(|entry| entry.rule.as_str()))
            .collect();
        let mut rules: Vec<String> = native_grammar_rule_concepts(&grammar.grammar)
            .into_iter()
            .map(|entry| entry.rule)
            .collect();
        listed.sort_unstable();
        rules.sort();
        assert_eq!(listed, rules, "{} lists every rule once", grammar.grammar);
    }
    let document = read(REUSE_DOCUMENT);
    assert!(document.contains("I195-MERGE-QUALITY-EVIDENCE"));
    for grammar in &report.grammars {
        let row = format!(
            "| `{}` | {} | {} | {} |",
            grammar.grammar,
            grammar.rules,
            grammar.shared.len(),
            grammar.specific.len()
        );
        assert!(document.contains(&row), "the report lists {row}");
    }
    observe(
        "I195-GRAMMAR-CONCEPT-REUSE-REPORT",
        REUSE_FIXTURE,
        &[
            "reuseReportCurrent",
            "sharedAndSpecificRulesListed",
            "reportPublishedWithMergeQualityEvidence",
            "runtimesAgreeOnReport",
        ],
        "the Rust reuse report equals the published report",
    );
}

#[test]
fn native_rule_names_are_english_and_keep_their_tree_sitter_names() {
    for grammar in native_grammar_ids() {
        let entry = native_grammar(grammar).expect("a catalog native grammar");
        let parsed = parse_grammar_links(&read(&format!(
            "parity/grammars/native/{}",
            entry.file.rsplit('/').next().unwrap_or_default()
        )))
        .expect("the native grammar parses");
        let mut renamed = BTreeMap::new();
        for rule in parsed.rules() {
            let name = &rule.name;
            assert!(
                name.split('_')
                    .all(|word| !word.is_empty() && word.chars().all(|c| c.is_ascii_lowercase())),
                "{grammar} {name}"
            );
            // What check:naming reads of the grammar: the rule name and the
            // concept reference, which must name a record.
            let concept = rule.concept.as_deref().expect("the rule names a concept");
            assert!(
                concept_records().iter().any(|record| record.id == concept),
                "{grammar} {name} names {concept}"
            );
            if let Some(original) = rule
                .source_names
                .iter()
                .find(|alias| alias.source == "tree-sitter")
            {
                renamed.insert(name.clone(), original.name.clone());
            }
        }
        // The catalog maps every renamed rule back to its tree-sitter kind.
        assert_eq!(entry.oracle_kinds, renamed, "{grammar}");
    }
    let json = native_grammar("native-json").expect("native-json");
    assert_eq!(
        json.oracle_kinds.get("value").map(String::as_str),
        Some("_value")
    );
    observe(
        "I195-NAMING-NATIVE-GRAMMARS",
        NAMING_FIXTURE,
        &[
            "nativeRuleNamesAreEnglish",
            "sourceNamesKeptAsAliases",
            "oracleKindsPreserved",
            "namingCheckCoversNativeGrammars",
        ],
        "native rule names are readable English",
    );
}
