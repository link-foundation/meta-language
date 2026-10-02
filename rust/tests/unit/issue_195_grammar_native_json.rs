//! Requirement I195-GRAMMAR-NATIVE-JSON: the native merged JSON grammar,
//! parity/grammars/native/json.lino, builds the concrete syntax trees of the
//! tree-sitter-json oracle that still backs the default JSON parse.
//! parity/fixtures/native-grammars/json.json holds the corpus with the oracle
//! rows, which js/scripts/generate-native-grammar-fixtures.mjs generates; this
//! suite projects the Rust executor's trees the way
//! js/scripts/native-grammar-rows.mjs projects the JavaScript ones and
//! compares them with the fixture, as
//! js/tests/issue-195-grammar-native-json.test.js does for JavaScript.

use std::collections::HashSet;

use meta_language::{
    FeatureGrammarParser, FeatureParseOptions, LeafText, SyntaxTree, compile_feature_grammar,
    parse_grammar_links, percent_decode_links_text, render_grammar_links,
};
use serde_json::{Value, json};

use super::issue_195_observations::{Observation, record};

const FIXTURE_FILE: &str = "parity/fixtures/native-grammars/json.json";
const FIXTURE: &str = include_str!("../../../parity/fixtures/native-grammars/json.json");
const GRAMMAR: &str = include_str!("../../../parity/grammars/native/json.lino");

fn observe(assertions: &[&str], test_name: &str) {
    record(&Observation {
        requirement_id: "I195-GRAMMAR-NATIVE-JSON",
        suffix: "behavior",
        fixture_id: "planned:repository-directive:i195-grammar-native-json",
        fixture_file: FIXTURE_FILE,
        assertions,
        test_name,
    });
}

fn fixture() -> Value {
    serde_json::from_str(FIXTURE).expect("the native JSON fixture is JSON")
}

fn parser() -> FeatureGrammarParser {
    let grammar = parse_grammar_links(GRAMMAR).expect("the native JSON grammar reads");
    compile_feature_grammar(&grammar, None, FeatureParseOptions::default())
        .expect("the native JSON grammar compiles")
}

fn cases<'a>(fixture: &'a Value, key: &str) -> &'a [Value] {
    fixture[key].as_array().expect("fixture cases")
}

fn source(case: &Value) -> &str {
    case["source"].as_str().expect("case source")
}

fn parse(parser: &FeatureGrammarParser, source: &str) -> Option<SyntaxTree> {
    let outcome = parser
        .parse_tree(source.as_bytes(), &FeatureParseOptions::default())
        .expect("the parse runs");
    if outcome.ok { outcome.tree } else { None }
}

fn text(text: &LeafText) -> &str {
    match text {
        LeafText::Text(text) => text,
        LeafText::Hex(hex) => panic!("JSON leaves are UTF-8, not <{hex}>"),
    }
}

fn leaves(tree: &SyntaxTree) -> Vec<&SyntaxTree> {
    match tree {
        SyntaxTree::Node { children, .. } => children.iter().flat_map(leaves).collect(),
        leaf => vec![leaf],
    }
}

/// Projects a native tree to the rows of the tree-sitter oracle: see
/// js/scripts/native-grammar-rows.mjs.
struct Rows<'a> {
    hidden: HashSet<&'a str>,
}

impl Rows<'_> {
    /// Whitespace and hidden leaves, which are not rows.
    fn invisible(&self, tree: &SyntaxTree) -> bool {
        match tree {
            SyntaxTree::Token {
                kind: None, trivia, ..
            } => *trivia,
            SyntaxTree::Token {
                kind: Some(kind), ..
            } => self.hidden.contains(kind.as_str()),
            _ => false,
        }
    }

    fn trivia(&self, tree: &SyntaxTree) -> bool {
        self.invisible(tree) || matches!(tree, SyntaxTree::Token { trivia: true, .. })
    }

    /// The node with its leading trivia moved before it.
    fn hoist(&self, tree: &SyntaxTree) -> Vec<SyntaxTree> {
        let SyntaxTree::Node {
            kind,
            field,
            start,
            end,
            children,
            attributes,
        } = tree
        else {
            return vec![tree.clone()];
        };
        let mut children: Vec<SyntaxTree> = children
            .iter()
            .flat_map(|child| self.hoist(child))
            .collect();
        let first = children
            .iter()
            .take_while(|child| self.trivia(child))
            .count();
        let rest = children.split_off(first);
        children.push(SyntaxTree::Node {
            kind: kind.clone(),
            field: field.clone(),
            start: *start,
            end: *end,
            children: rest,
            attributes: attributes.clone(),
        });
        children
    }

    fn span(&self, tree: &SyntaxTree) -> (usize, usize) {
        let SyntaxTree::Node {
            start, children, ..
        } = tree
        else {
            return (tree.start(), tree.end());
        };
        let inner: Vec<_> = children
            .iter()
            .filter(|child| !self.trivia(child))
            .map(|child| self.span(child))
            .collect();
        match (inner.first(), inner.last()) {
            (Some(first), Some(last)) => (first.0, last.1),
            _ => (*start, *start),
        }
    }

    fn visit(&self, tree: &SyntaxTree, depth: usize, rows: &mut Vec<Value>) {
        if self.invisible(tree) {
            return;
        }
        let (start, end) = self.span(tree);
        match tree {
            SyntaxTree::Node {
                kind,
                field,
                children,
                ..
            } => {
                rows.push(json!([depth, field, kind, 1, start, end, ""]));
                for child in children.iter().flat_map(|child| self.hoist(child)) {
                    self.visit(&child, depth + 1, rows);
                }
            }
            SyntaxTree::Token {
                kind,
                field,
                trivia,
                text: leaf,
                ..
            } => {
                let (kind, named) = kind
                    .as_deref()
                    .map_or_else(|| (text(leaf), 0), |kind| (kind, 1));
                rows.push(json!([
                    depth,
                    field,
                    kind,
                    named,
                    start,
                    end,
                    if *trivia { "X" } else { "" }
                ]));
            }
            other => panic!("an accepted JSON tree has no {other:?}"),
        }
    }

    fn rows(&self, tree: &SyntaxTree, source: &str) -> Vec<Value> {
        let SyntaxTree::Node { kind, children, .. } = tree else {
            panic!("the JSON root is a node");
        };
        let first = leaves(tree).into_iter().find(|leaf| !self.invisible(leaf));
        let start = first.map_or(source.len(), SyntaxTree::start);
        let mut rows = vec![json!([0, null, kind, 1, start, source.len(), ""])];
        for child in children.iter().flat_map(|child| self.hoist(child)) {
            self.visit(&child, 1, &mut rows);
        }
        rows
    }
}

fn projection(fixture: &Value) -> Rows<'_> {
    let hidden = fixture["hidden"].as_array().expect("hidden kinds");
    Rows {
        hidden: hidden
            .iter()
            .map(|kind| kind.as_str().expect("kind"))
            .collect(),
    }
}

#[test]
fn native_json_grammar_is_canonical_links_notation() {
    let grammar = parse_grammar_links(GRAMMAR).expect("the native JSON grammar reads");
    assert_eq!(render_grammar_links(&grammar), GRAMMAR);
    assert_eq!(grammar.start(), Some("document"));
    for rule in [
        "document", "object", "pair", "array", "string", "number", "true", "false", "null",
        "comment",
    ] {
        assert!(grammar.rule(rule).is_some(), "{rule}");
    }
    assert!(GRAMMAR.trim_end().lines().all(|line| {
        ["(grammar ", "(extra ", "(rule "]
            .iter()
            .any(|prefix| line.starts_with(prefix))
    }));
    assert!(!GRAMMAR.contains("tree-sitter") && !GRAMMAR.contains("grammar.js"));
    observe(
        &["nativeJsonGrammarIsCanonicalLinks"],
        "native JSON grammar is canonical Links Notation",
    );
}

#[test]
fn native_json_grammar_builds_the_oracle_rows() {
    let fixture = fixture();
    let parser = parser();
    let rows = projection(&fixture);
    let matches = cases(&fixture, "matches");
    assert!(matches.len() >= 30);
    for case in matches {
        let tree = parse(&parser, source(case)).unwrap_or_else(|| panic!("{:?}", source(case)));
        assert_eq!(
            Value::Array(rows.rows(&tree, source(case))),
            case["rows"],
            "{:?}",
            source(case)
        );
    }
    observe(
        &["nativeJsonTreesMatchOracle"],
        "native JSON grammar builds the oracle rows",
    );
}

#[test]
fn native_json_grammar_accepts_merged_source_extensions() {
    let fixture = fixture();
    let parser = parser();
    let rows = projection(&fixture);
    let divergences = cases(&fixture, "divergences");
    assert!(divergences.len() >= 2);
    for case in divergences {
        assert!(
            case["reason"]
                .as_str()
                .expect("reason")
                .contains("RFC 8259")
        );
        let tree = parse(&parser, source(case)).unwrap_or_else(|| panic!("{:?}", source(case)));
        assert_eq!(
            Value::Array(rows.rows(&tree, source(case))),
            case["rows"],
            "{:?}",
            source(case)
        );
    }
    assert_eq!(
        divergences[0]["rows"],
        json!([
            [0, null, "document", 1, 0, 4, ""],
            [1, null, "number", 1, 0, 4, ""]
        ])
    );
    observe(
        &["nativeJsonAcceptsMergedSourceExtensions"],
        "native JSON grammar accepts merged source extensions",
    );
}

#[test]
fn native_json_grammar_rejects_invalid_json() {
    let fixture = fixture();
    let parser = parser();
    let rejections = cases(&fixture, "rejections");
    assert!(rejections.len() >= 10);
    for case in rejections {
        let outcome = parser
            .parse_tree(source(case).as_bytes(), &FeatureParseOptions::default())
            .expect("the parse runs");
        assert!(!outcome.ok, "{:?}", source(case));
        assert!(outcome.rejection.is_some(), "{:?}", source(case));
    }
    observe(
        &["nativeJsonRejectsInvalidInput"],
        "native JSON grammar rejects invalid JSON",
    );
}

#[test]
fn native_json_trees_keep_every_byte() {
    let fixture = fixture();
    let parser = parser();
    for case in cases(&fixture, "matches")
        .iter()
        .chain(cases(&fixture, "divergences"))
    {
        let tree = parse(&parser, source(case)).expect("accepted");
        let rebuilt: String = leaves(&tree)
            .into_iter()
            .map(|leaf| match leaf {
                SyntaxTree::Token { text: leaf, .. } => text(leaf),
                other => panic!("an accepted JSON tree has no {other:?}"),
            })
            .collect();
        assert_eq!(rebuilt, source(case));
    }
    let tree = parse(&parser, "\u{feff}[]").expect("a leading byte order mark is accepted");
    let SyntaxTree::Token {
        kind,
        text: leaf,
        start,
        end,
        ..
    } = leaves(&tree)[0]
    else {
        panic!("the byte order mark is a leaf");
    };
    assert_eq!(
        (kind.as_deref(), text(leaf), *start, *end),
        (Some("byte_order_mark"), "\u{feff}", 0, 3)
    );
    assert_eq!(
        percent_decode_links_text("%EF%BB%BF").expect("decodes"),
        "\u{feff}"
    );
    observe(
        &["nativeJsonTreesLossless"],
        "native JSON trees keep every byte",
    );
}
