//! Requirement I195-PARITY-FEATURE-COMPLETENESS: the shared corpus
//! `parity/fixtures/export-parity/cases.lino` gives the observable output both
//! runtimes produce for the exports it names. `js/tests/export-parity.test.js`
//! runs the same cases through the JavaScript exports, so a difference between
//! the runtimes fails one of the two suites.

use std::fs;
use std::path::{Path, PathBuf};

use links_notation::{LiNo, parse_lino_to_links};
use meta_language::{
    AccessMode, FeatureParseOptions, GRAMMAR_DIAGNOSTIC_KINDS, Grammar, GrammarExpr, LinkId,
    RegionDetectionPolicy, SourceAlias, accepts_text, canonical_repeat, canonical_rule_definition,
    carry_rule_docs, choice, compile_feature_grammar, concept_records, detect_embedded_regions,
    display_grammar_expression, foundation_register, import_ebnf, parse_with_grammar, sequence,
    sniff_language, source_meanings_in, validate,
};
use serde_json::Value;

use super::issue_195_observations::{Observation, record};

const CASES_FILE: &str = "parity/fixtures/export-parity/cases.lino";

fn language_features() -> (PathBuf, Value) {
    let mut dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    while !dir.join("parity/language-features.json").is_file() {
        assert!(dir.pop(), "parity/language-features.json should exist");
    }
    let text = fs::read_to_string(dir.join("parity/language-features.json"))
        .expect("parity/language-features.json should be readable");
    let features = serde_json::from_str(&text).expect("the features should be JSON");
    (dir, features)
}

/// A link read as `[id, ...values]` and a reference as its id, as the
/// JavaScript test reads the corpus.
fn strings(link: &LiNo<String>) -> Vec<String> {
    match link {
        LiNo::Ref(id) => vec![id.clone()],
        LiNo::Link { id, values } if values.is_empty() => id.iter().cloned().collect(),
        LiNo::Link { values, .. } => values
            .iter()
            .map(|value| match value {
                LiNo::Ref(id) | LiNo::Link { id: Some(id), .. } => id.clone(),
                LiNo::Link { id: None, .. } => panic!("a case value is a reference"),
            })
            .collect(),
    }
}

struct Case {
    name: String,
    input: Vec<String>,
    expected: Vec<String>,
}

fn cases(root: &Path) -> Vec<Case> {
    let text = fs::read_to_string(root.join(CASES_FILE)).expect("the corpus should be readable");
    let links = parse_lino_to_links(&text).expect("the corpus should be Links Notation");
    links
        .iter()
        .map(|link| {
            let LiNo::Link { values, .. } = link else {
                panic!("a case is a link");
            };
            let [name, input, expected] = values.as_slice() else {
                panic!("a case is (name (input ...) (expected ...))");
            };
            let part = |part: &LiNo<String>, label: &str| {
                let mut items = strings(part);
                assert_eq!(items.first().map(String::as_str), Some(label));
                items.remove(0);
                items
            };
            Case {
                name: strings(name).concat(),
                input: part(input, "input"),
                expected: part(expected, "expected"),
            }
        })
        .collect()
}

fn grammar(source: &str) -> Grammar {
    import_ebnf(source).expect("the case grammar should import")
}

fn rule_expression(grammar: &Grammar, rule: &str) -> GrammarExpr {
    grammar
        .rule(rule)
        .expect("the case rule exists")
        .expr
        .clone()
}

fn all_expressions(grammar: &Grammar) -> Vec<GrammarExpr> {
    grammar
        .rules()
        .iter()
        .map(|rule| rule.expr.clone())
        .collect()
}

fn parsed(source: &str, text: &str) -> String {
    parse_with_grammar(
        &grammar(source),
        text.as_bytes(),
        &FeatureParseOptions::default(),
    )
    .map_or_else(|_| "error".to_string(), |tree| tree.render())
}

fn compiled(source: &str, text: &str) -> String {
    let parser = compile_feature_grammar(&grammar(source), None, FeatureParseOptions::default())
        .expect("the case grammar should compile");
    parser
        .parse(text.as_bytes(), &FeatureParseOptions::default())
        .map_or_else(|_| "error".to_string(), |tree| tree.render())
}

fn access_mode(label: &str) -> AccessMode {
    match label {
        "mutable" => AccessMode::Mutable,
        "read-only" => AccessMode::ReadOnly,
        other => panic!("unknown access mode {other}"),
    }
}

fn one(value: &impl ToString) -> Vec<String> {
    vec![value.to_string()]
}

fn carried_doc(source: &str, rule: &str, doc: &str) -> String {
    let original = grammar(source);
    let mut documented = Grammar::new();
    if let Some(start) = original.start() {
        documented = documented.with_start(start);
    }
    for candidate in original.rules() {
        let mut candidate = candidate.clone();
        if candidate.name == rule {
            candidate = candidate.with_doc(doc);
        }
        documented = documented.with_rule(candidate);
    }
    let carried = carry_rule_docs(grammar(source), &documented, &str::to_string);
    carried
        .rule(rule)
        .and_then(|rule| rule.doc().map(str::to_string))
        .unwrap_or_else(|| "none".to_string())
}

/// The observable output of one corpus case, as strings.
fn run(name: &str, input: &[String]) -> Vec<String> {
    let arg = |index: usize| input[index].as_str();
    match name {
        "sniffLanguage" => one(&sniff_language(arg(0)).unwrap_or("none")),
        "GRAMMAR_DIAGNOSTIC_KINDS" => GRAMMAR_DIAGNOSTIC_KINDS.map(str::to_string).to_vec(),
        "displayGrammarExpression" => one(&display_grammar_expression(&rule_expression(
            &grammar(arg(0)),
            arg(1),
        ))),
        "sequence" => one(&sequence(all_expressions(&grammar(arg(0))))),
        "choice" => one(&choice(all_expressions(&grammar(arg(0))), arg(1) == "true")),
        "canonicalRepeat" => {
            let item = rule_expression(&grammar(arg(0)), arg(1));
            let min = arg(2).parse().expect("a minimum");
            let max = (arg(3) != "none").then(|| arg(3).parse().expect("a maximum"));
            one(&canonical_repeat(item, min, max)
                .map_or_else(|error| format!("error: {error}"), |expr| expr.to_string()))
        }
        "canonicalRuleDefinition" => one(&canonical_rule_definition(
            grammar(arg(0)).rule(arg(1)).expect("the case rule exists"),
        )
        .expect("the rule has a canonical definition")),
        "acceptsText" => one(&accepts_text(&grammar(arg(0)), arg(1))),
        "parseWithGrammar" | "renderSyntaxTree" => one(&parsed(arg(0), arg(1))),
        "compileGrammar" | "createGrammarParser" => one(&compiled(arg(0), arg(1))),
        "validateGrammar" => validate(&grammar(arg(0)))
            .iter()
            .map(|diagnostic| diagnostic.kind.as_str().to_string())
            .collect(),
        "carryRuleDocs" => one(&carried_doc(arg(0), arg(1), arg(2))),
        "detectEmbeddedRegions" => {
            detect_embedded_regions(arg(0), arg(1), RegionDetectionPolicy::Both)
                .iter()
                .map(|region| {
                    let range = region.span().byte_range();
                    format!("{} {}..{}", region.language(), range.start(), range.end())
                })
                .collect()
        }
        "sourceMeanings" => source_meanings_in(
            concept_records(),
            foundation_register(),
            &SourceAlias {
                source: arg(0).to_string(),
                name: arg(1).to_string(),
            },
            None,
        ),
        "idKey" => one(&LinkId::from_u64(arg(0).parse().expect("an id")).as_u64()),
        "accessModeLabel" => one(&access_mode(arg(0)).label()),
        "accessModeIsMutable" => one(&access_mode(arg(0)).is_mutable()),
        "accessModeIsReadOnly" => one(&access_mode(arg(0)).is_read_only()),
        other => panic!("{other} has no corpus runner"),
    }
}

#[test]
fn export_parity_corpus_gives_the_expected_output_in_rust() {
    let (root, features) = language_features();
    let listed = &features["javascriptExports"];
    let cases = cases(&root);
    assert!(!cases.is_empty(), "the corpus has cases");
    for case in &cases {
        assert!(
            listed[&case.name].is_object(),
            "{} is a listed export",
            case.name
        );
        assert_eq!(
            run(&case.name, &case.input),
            case.expected,
            "{}({})",
            case.name,
            case.input.join(", ")
        );
    }
    record(&Observation {
        requirement_id: "I195-PARITY-FEATURE-COMPLETENESS",
        suffix: "behavior",
        fixture_id: "planned:repository-directive:i195-parity-feature-completeness",
        fixture_file: "docs/vision.md",
        assertions: &["observableOutputEqual"],
        test_name: "export_parity_corpus::export_parity_corpus_gives_the_expected_output_in_rust",
    });
}
