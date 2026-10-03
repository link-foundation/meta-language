//! Requirement I195-GRAMMAR-NATIVE-RECOVERY: automatic error recovery in the
//! native executor. With `error_recovery` a parse the grammar rejects is
//! repaired into a tree of ERROR and MISSING leaves, with no recovery rules in
//! the grammar. Every rejection of parity/fixtures/native-grammars/*.json
//! records its `recovered` tree, which js/scripts/generate-native-grammar-fixtures.mjs
//! generates with the JavaScript executor; this suite checks that the Rust
//! executor builds the same trees, as
//! js/tests/issue-195-grammar-native-recovery.test.js does for JavaScript.

use meta_language::{FeatureGrammarParser, FeatureParseOptions, ParseOutcome, SyntaxTree};
use serde_json::Value;

use super::issue_195_native_grammar_rows::{cases, parser, source};
use super::issue_195_observations::{Observation, record};

const GRAMMARS: [(&str, &str, &str); 9] = [
    (
        "json",
        include_str!("../../../parity/grammars/native/json.lino"),
        include_str!("../../../parity/fixtures/native-grammars/json.json"),
    ),
    (
        "ini",
        include_str!("../../../parity/grammars/native/ini.lino"),
        include_str!("../../../parity/fixtures/native-grammars/ini.json"),
    ),
    (
        "diff",
        include_str!("../../../parity/grammars/native/diff.lino"),
        include_str!("../../../parity/fixtures/native-grammars/diff.json"),
    ),
    (
        "csv",
        include_str!("../../../parity/grammars/native/csv.lino"),
        include_str!("../../../parity/fixtures/native-grammars/csv.json"),
    ),
    (
        "json5",
        include_str!("../../../parity/grammars/native/json5.lino"),
        include_str!("../../../parity/fixtures/native-grammars/json5.json"),
    ),
    (
        "scheme",
        include_str!("../../../parity/grammars/native/scheme.lino"),
        include_str!("../../../parity/fixtures/native-grammars/scheme.json"),
    ),
    (
        "racket",
        include_str!("../../../parity/grammars/native/racket.lino"),
        include_str!("../../../parity/fixtures/native-grammars/racket.json"),
    ),
    (
        "c",
        include_str!("../../../parity/grammars/native/c.lino"),
        include_str!("../../../parity/fixtures/native-grammars/c.json"),
    ),
    (
        "rust",
        include_str!("../../../parity/grammars/native/rust.lino"),
        include_str!("../../../parity/fixtures/native-grammars/rust.json"),
    ),
];

fn observe(assertions: &[&str], test_name: &str) {
    record(&Observation {
        requirement_id: "I195-GRAMMAR-NATIVE-RECOVERY",
        suffix: "behavior",
        fixture_id: "planned:repository-directive:i195-grammar-native-recovery",
        fixture_file: "parity/fixtures/native-grammars/json.json",
        assertions,
        test_name,
    });
}

fn grammars() -> Vec<(&'static str, Value, FeatureGrammarParser)> {
    GRAMMARS
        .iter()
        .map(|(id, grammar, fixture)| {
            let fixture = serde_json::from_str(fixture).expect("the fixture is JSON");
            (*id, fixture, parser(grammar))
        })
        .collect()
}

fn recover() -> FeatureParseOptions {
    FeatureParseOptions {
        error_recovery: Some(true),
        ..FeatureParseOptions::default()
    }
}

fn parse(
    parser: &FeatureGrammarParser,
    source: &str,
    options: &FeatureParseOptions,
) -> ParseOutcome {
    parser
        .parse_tree(source.as_bytes(), options)
        .expect("the parse runs")
}

fn rendered(outcome: &ParseOutcome) -> String {
    outcome.tree.as_ref().expect("a tree").render()
}

// The bytes of the leaves in source order; a MISSING leaf covers no bytes.
fn leaf_bytes(tree: &SyntaxTree, source: &[u8], out: &mut Vec<u8>) {
    match tree {
        SyntaxTree::Node { children, .. } => {
            for child in children {
                leaf_bytes(child, source, out);
            }
        }
        SyntaxTree::Embed { root, .. } => leaf_bytes(root, source, out),
        SyntaxTree::Missing { start, end, .. } => assert_eq!(start, end),
        SyntaxTree::Token { start, end, .. } | SyntaxTree::Error { start, end, .. } => {
            out.extend_from_slice(&source[*start..*end]);
        }
    }
}

#[test]
fn native_grammars_repair_each_fixture_rejection_into_its_recorded_tree() {
    let mut repaired = 0;
    for (id, fixture, parser) in grammars() {
        let rejections = cases(&fixture, "rejections");
        assert!(!rejections.is_empty(), "{id}");
        for case in rejections {
            let source = source(case);
            let label = format!("{id} {source:?}");
            let recorded = case["recovered"].as_str().expect("recovered tree");
            let plain = parse(&parser, source, &FeatureParseOptions::default());
            assert!(!plain.ok, "{label}");
            assert!(plain.tree.is_none(), "{label}");
            let outcome = parse(&parser, source, &recover());
            assert!(!outcome.ok, "{label}");
            assert_eq!(
                outcome.rejection.as_ref().map(|rejection| rejection.reason),
                Some("recovered"),
                "{label}"
            );
            assert_eq!(rendered(&outcome), recorded, "{label}");
            let mut bytes = Vec::new();
            leaf_bytes(
                outcome.tree.as_ref().expect("a tree"),
                source.as_bytes(),
                &mut bytes,
            );
            assert_eq!(bytes, source.as_bytes(), "{label}");
            let accepted = parse(
                &parser,
                source,
                &FeatureParseOptions {
                    accept_recovery: Some(true),
                    ..recover()
                },
            );
            assert!(accepted.ok, "{label}");
            assert_eq!(rendered(&accepted), recorded, "{label}");
            repaired += 1;
        }
    }
    assert!(repaired >= 200, "{repaired} repaired rejections");
    observe(
        &[
            "nativeRecoveryTreesMatchFixtures",
            "nativeRecoveryTreesLossless",
            "nativeRecoveryReportedAsRecovered",
        ],
        "native_grammars_repair_each_fixture_rejection_into_its_recorded_tree",
    );
}

#[test]
fn automatic_recovery_leaves_the_trees_of_accepted_input_unchanged() {
    for (id, fixture, parser) in grammars() {
        for case in cases(&fixture, "matches")
            .iter()
            .chain(cases(&fixture, "divergences"))
        {
            let source = source(case);
            let plain = parse(&parser, source, &FeatureParseOptions::default());
            let outcome = parse(&parser, source, &recover());
            assert!(outcome.ok, "{id} {source:?}");
            assert_eq!(rendered(&outcome), rendered(&plain), "{id} {source:?}");
        }
    }
    observe(
        &["nativeRecoveryKeepsAcceptedTrees"],
        "automatic_recovery_leaves_the_trees_of_accepted_input_unchanged",
    );
}

#[test]
fn a_repair_inserts_a_missing_leaf_or_skips_input_as_an_error_leaf() {
    let json = parser(GRAMMARS[0].1);
    let tree =
        |source: &str, options: &FeatureParseOptions| rendered(&parse(&json, source, options));
    // A missing separator and a missing closing bracket are inserted; as in
    // tree-sitter, a MISSING leaf comes before the white space after the
    // token it follows.
    assert_eq!(
        tree(r#"{"a" 1}"#, &recover()),
        r#"(document (object "{" (pair key:(string "\"" (string_content "a") "\"") (MISSING@4 ":") ~" " value:(number "1")) "}"))"#
    );
    assert_eq!(
        tree("[1, 2", &recover()),
        r#"(document (array "[" (number "1") "," ~" " (number "2") (MISSING@5 "]")))"#
    );
    // A stray value is skipped: deleting one byte costs less than inserting a comma.
    assert_eq!(
        tree("[1, 2 3]", &recover()),
        r#"(document (array "[" (number "1") "," ~" " (number "2") ~" " (ERROR@6..7 "3") "]"))"#
    );
    // With no repair point left, the rest of the input is one ERROR leaf.
    let none = FeatureParseOptions {
        max_repairs: Some(0),
        ..recover()
    };
    assert_eq!(
        tree("[1, 2 3]", &none),
        r#"(document (ERROR@0..8 "[1, 2 3]"))"#
    );
    // Without the option the parse is rejected with no tree.
    assert!(
        parse(&json, "[1, 2 3]", &FeatureParseOptions::default())
            .tree
            .is_none()
    );
}

#[test]
fn a_long_repetition_repaired_near_its_end_keeps_every_item_in_order() {
    // A join links its parts instead of copying the children before it, so a
    // repetition of n items costs O(n) and not O(n²); the tree is unchanged.
    let json = parser(GRAMMARS[0].1);
    let count = 10_000;
    let items: Vec<String> = (0..count).map(|index| (index % 10).to_string()).collect();
    let source = format!("[{} 7]", items.join(","));
    let outcome = parse(&json, &source, &recover());
    assert_eq!(
        outcome.rejection.as_ref().map(|rejection| rejection.reason),
        Some("recovered")
    );
    let tree = outcome.tree.as_ref().expect("a tree");
    let SyntaxTree::Node { children, .. } = tree else {
        panic!("a document node");
    };
    let Some(SyntaxTree::Node {
        children: array, ..
    }) = children.first()
    else {
        panic!("an array node");
    };
    let numbers: Vec<(usize, usize)> = array
        .iter()
        .filter_map(|child| match child {
            SyntaxTree::Node {
                kind, start, end, ..
            }
            | SyntaxTree::Token {
                kind: Some(kind),
                start,
                end,
                ..
            } if kind == "number" => Some((*start, *end)),
            _ => None,
        })
        .collect();
    assert_eq!(numbers.len(), count);
    for (index, (start, end)) in numbers.iter().enumerate() {
        assert_eq!(&source[*start..*end], (index % 10).to_string());
    }
    assert!(matches!(array[array.len() - 2], SyntaxTree::Error { .. }));
    let mut bytes = Vec::new();
    leaf_bytes(tree, source.as_bytes(), &mut bytes);
    assert_eq!(bytes, source.as_bytes());
}
