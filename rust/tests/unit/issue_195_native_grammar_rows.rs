//! Shared helpers of the native merged grammar suites
//! (`issue_195_grammar_native_json.rs`, `issue_195_grammar_native_ini.rs`,
//! `issue_195_grammar_native_diff.rs`, `issue_195_grammar_native_csv.rs`,
//! `issue_195_grammar_native_json5.rs`, `issue_195_grammar_native_racket.rs`,
//! `issue_195_grammar_native_scheme.rs`, `issue_195_grammar_native_c.rs`,
//! `issue_195_grammar_native_rust.rs`,
//! `issue_195_grammar_native_javascript.rs`,
//! `issue_195_grammar_native_typescript.rs`,
//! `issue_195_grammar_native_tsx.rs`, `issue_195_grammar_native_lean.rs`,
//! `issue_195_grammar_native_rocq.rs`, `issue_195_grammar_native_java.rs`,
//! `issue_195_grammar_native_go.rs`, `issue_195_grammar_native_regex.rs`,
//! `issue_195_grammar_native_graphql.rs`,
//! `issue_195_grammar_native_proto.rs`,
//! `issue_195_grammar_native_make.rs`,
//! `issue_195_grammar_native_solidity.rs`):
//! they read a fixture of
//! parity/fixtures/native-grammars/, parse with the Rust
//! executor and project its trees to the rows of the tree-sitter oracle the
//! way js/scripts/native-grammar-rows.mjs projects the JavaScript ones.
//! `oracle_agrees` parses the corpus with the pinned oracle crate, a
//! development dependency since the native grammar replaced it as the default
//! parse; the CSV oracle is a vendored parser that only the JavaScript suite
//! loads.

use std::collections::{HashMap, HashSet};

use meta_language::{
    FeatureGrammarParser, FeatureParseOptions, LeafText, SyntaxTree, compile_feature_grammar,
    parse_grammar_links,
};
use serde_json::{Value, json};

pub fn parser(grammar: &str) -> FeatureGrammarParser {
    let grammar = parse_grammar_links(grammar).expect("the native grammar reads");
    compile_feature_grammar(&grammar, None, FeatureParseOptions::default())
        .expect("the native grammar compiles")
}

pub fn cases<'a>(fixture: &'a Value, key: &str) -> &'a [Value] {
    fixture[key].as_array().expect("fixture cases")
}

pub fn source(case: &Value) -> &str {
    case["source"].as_str().expect("case source")
}

/// The tree of an accepted parse with no ambiguity, or `None`.
pub fn parse(parser: &FeatureGrammarParser, source: &str) -> Option<SyntaxTree> {
    let outcome = parser
        .parse_tree(source.as_bytes(), &FeatureParseOptions::default())
        .expect("the parse runs");
    assert!(outcome.ambiguities.is_empty(), "{source:?} is ambiguous");
    if outcome.ok { outcome.tree } else { None }
}

pub fn text(text: &LeafText) -> &str {
    match text {
        LeafText::Text(text) => text,
        LeafText::Hex(hex) => panic!("the corpus leaves are UTF-8, not <{hex}>"),
    }
}

pub fn leaves(tree: &SyntaxTree) -> Vec<&SyntaxTree> {
    match tree {
        SyntaxTree::Node { children, .. } => children.iter().flat_map(leaves).collect(),
        leaf => vec![leaf],
    }
}

/// The source rebuilt from the leaves of an accepted tree.
pub fn rebuilt(tree: &SyntaxTree) -> String {
    leaves(tree)
        .into_iter()
        .map(|leaf| match leaf {
            SyntaxTree::Token { text: leaf, .. } => text(leaf),
            other => panic!("an accepted tree has no {other:?}"),
        })
        .collect()
}

/// Rebuilds recovered UTF-8 source, retaining error text and empty missing leaves.
pub fn rebuilt_recovery(tree: &SyntaxTree) -> String {
    leaves(tree)
        .into_iter()
        .map(|leaf| match leaf {
            SyntaxTree::Token { text: leaf, .. } | SyntaxTree::Error { text: leaf, .. } => {
                text(leaf)
            }
            SyntaxTree::Missing { start, end, .. } => {
                assert_eq!(start, end, "a missing leaf must be zero-width");
                ""
            }
            other => panic!("a recovered source has no {other:?}"),
        })
        .collect()
}

fn kinds<'a>(fixture: &'a Value, key: &str) -> HashSet<&'a str> {
    fixture[key]
        .as_array()
        .expect("fixture kinds")
        .iter()
        .map(|kind| kind.as_str().expect("kind"))
        .collect()
}

/// Projects a native tree to the rows of the tree-sitter oracle: see
/// js/scripts/native-grammar-rows.mjs. `hidden` leaves are projected like
/// whitespace, `anonymous` leaves are not rows but count in the spans when
/// they are significant syntax rather than trivia, and
/// `extras` nodes are rows with flag X. A rule renamed from its tree-sitter
/// name is a row of that name (`oracleKinds`). Recovered trees retain ERROR
/// and visible MISSING rows with their original spans and flags.
pub struct Rows<'a> {
    root_includes_leading_trivia: bool,
    hidden: HashSet<&'a str>,
    anonymous: HashSet<&'a str>,
    extras: HashSet<&'a str>,
    oracle_kinds: HashMap<&'a str, &'a str>,
}

impl<'a> Rows<'a> {
    pub fn new(fixture: &'a Value) -> Self {
        Self {
            root_includes_leading_trivia: fixture["rootIncludesLeadingTrivia"]
                .as_bool()
                .unwrap_or(false),
            hidden: kinds(fixture, "hidden"),
            anonymous: kinds(fixture, "anonymous"),
            extras: kinds(fixture, "extras"),
            oracle_kinds: fixture["oracleKinds"]
                .as_object()
                .expect("fixture oracle kinds")
                .iter()
                .map(|(name, kind)| (name.as_str(), kind.as_str().expect("oracle kind")))
                .collect(),
        }
    }

    /// The oracle kind of a native kind; an anonymous alias `'TEXT` is TEXT.
    fn oracle_kind<'b>(&'b self, kind: &'b str) -> &'b str {
        kind.strip_prefix('\'')
            .unwrap_or_else(|| self.oracle_kinds.get(kind).copied().unwrap_or(kind))
    }

    /// Whitespace, anonymous trivia and hidden leaves, which are not rows.
    fn invisible(&self, tree: &SyntaxTree) -> bool {
        match tree {
            SyntaxTree::Token {
                kind: None, trivia, ..
            } => *trivia,
            SyntaxTree::Token {
                kind: Some(kind),
                trivia,
                ..
            } => {
                self.hidden.contains(kind.as_str())
                    || (*trivia && self.anonymous.contains(kind.as_str()))
            }
            _ => false,
        }
    }

    fn anonymous(&self, tree: &SyntaxTree) -> bool {
        matches!(tree, SyntaxTree::Token { kind: Some(kind), .. } if self.anonymous.contains(kind.as_str()))
    }

    /// A missing hidden or anonymous token has no oracle row, as in JavaScript.
    fn hidden_missing(&self, tree: &SyntaxTree) -> bool {
        matches!(tree, SyntaxTree::Missing { kind: Some(kind), .. }
            if self.hidden.contains(kind.as_str()) || self.anonymous.contains(kind.as_str()))
    }

    fn trivia(&self, tree: &SyntaxTree) -> bool {
        self.invisible(tree)
            || matches!(tree, SyntaxTree::Token { trivia: true, .. })
            || matches!(tree, SyntaxTree::Node { trivia: true, .. })
            || matches!(tree, SyntaxTree::Node { kind, .. } if self.extras.contains(kind.as_str()))
    }

    /// The node with its leading trivia moved before it.
    fn hoist(&self, tree: &SyntaxTree) -> Vec<SyntaxTree> {
        let SyntaxTree::Node {
            kind,
            field,
            trivia,
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
            trivia: *trivia,
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
        if self.invisible(tree) || self.anonymous(tree) || self.hidden_missing(tree) {
            return;
        }
        let (start, end) = self.span(tree);
        match tree {
            SyntaxTree::Node {
                kind,
                field,
                children,
                trivia,
                ..
            } => {
                let flags = if *trivia || self.extras.contains(kind.as_str()) {
                    "X"
                } else {
                    ""
                };
                rows.push(json!([
                    depth,
                    field,
                    self.oracle_kind(kind),
                    u8::from(!kind.starts_with('\'')),
                    start,
                    end,
                    flags
                ]));
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
                let (kind, named) = kind.as_deref().map_or_else(
                    || (text(leaf), 0),
                    |kind| (self.oracle_kind(kind), u8::from(!kind.starts_with('\''))),
                );
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
            SyntaxTree::Error { .. } => {
                rows.push(json!([depth, null, "ERROR", 1, start, end, "E"]));
            }
            SyntaxTree::Missing { kind, literal, .. } => {
                let named =
                    !*literal && !kind.as_deref().is_some_and(|kind| kind.starts_with('\''));
                rows.push(json!([
                    depth,
                    null,
                    kind.as_deref().map(|kind| self.oracle_kind(kind)),
                    u8::from(named),
                    start,
                    end,
                    "M"
                ]));
            }
            other @ SyntaxTree::Embed { .. } => {
                panic!("the oracle projection does not handle {other:?}")
            }
        }
    }

    pub fn rows(&self, tree: &SyntaxTree, source: &str) -> Vec<Value> {
        let SyntaxTree::Node { kind, children, .. } = tree else {
            panic!("the root is a node");
        };
        let first = leaves(tree).into_iter().find(|leaf| !self.invisible(leaf));
        let start = if self.root_includes_leading_trivia {
            tree.start()
        } else {
            first.map_or(source.len(), SyntaxTree::start)
        };
        let mut rows = vec![json!([
            0,
            null,
            self.oracle_kind(kind),
            1,
            start,
            source.len(),
            ""
        ])];
        for child in children.iter().flat_map(|child| self.hoist(child)) {
            self.visit(&child, 1, &mut rows);
        }
        rows
    }
}

/// The rows of the pinned tree-sitter oracle's tree of `source`, and whether
/// the oracle recovers from an error in it.
pub fn oracle_rows(language: &tree_sitter::Language, source: &str) -> (Vec<Value>, bool) {
    fn visit(
        node: tree_sitter::Node<'_>,
        depth: usize,
        field: Option<&str>,
        rows: &mut Vec<Value>,
    ) {
        let flags = [
            (node.is_error(), "E"),
            (node.is_missing(), "M"),
            (node.is_extra(), "X"),
        ]
        .iter()
        .filter(|(set, _)| *set)
        .map(|(_, flag)| *flag)
        .collect::<String>();
        rows.push(json!([
            depth,
            field,
            node.kind(),
            u8::from(node.is_named()),
            node.start_byte(),
            node.end_byte(),
            flags
        ]));
        for index in 0..node.child_count() {
            let child = node.child(index).expect("a child");
            visit(child, depth + 1, node.field_name_for_child(index), rows);
        }
    }
    let mut parser = tree_sitter::Parser::new();
    parser.set_language(language).expect("the oracle loads");
    let tree = parser.parse(source, None).expect("the oracle parses");
    let mut rows = Vec::new();
    visit(tree.root_node(), 0, None, &mut rows);
    (rows, tree.root_node().has_error())
}

/// Checks that the pinned oracle still gives the fixture: the rows of every
/// match, and a recovery from every divergence and rejection.
pub fn oracle_agrees(language: &tree_sitter::Language, fixture: &Value) {
    for case in cases(fixture, "matches") {
        let (rows, recovers) = oracle_rows(language, source(case));
        assert!(!recovers, "{:?}", source(case));
        assert_eq!(Value::Array(rows), case["rows"], "{:?}", source(case));
    }
    let divergences = fixture.get("divergences").map_or(&[][..], |value| {
        value
            .as_array()
            .expect("fixture divergence cases")
            .as_slice()
    });
    for case in divergences.iter().chain(cases(fixture, "rejections")) {
        assert!(oracle_rows(language, source(case)).1, "{:?}", source(case));
    }
}

// Matches the tiny controls in js/tests/grammar-native-row-recovery.test.js.
// These trees isolate projection and reconstruction from grammar execution.
#[cfg(test)]
mod recovery_projection_tests {
    use super::{LeafText, Rows, SyntaxTree, Value, json, rebuilt, rebuilt_recovery};

    fn projection() -> Value {
        json!({
            "hidden": ["hidden_missing"], "anonymous": ["line_break"], "extras": [],
            "oracleKinds": {
                "native_document": "document", "native_group": "group",
                "native_name": "identifier", "native_identifier": "identifier"
            }
        })
    }

    fn token(
        text: &str,
        start: usize,
        end: usize,
        kind: Option<&str>,
        trivia: bool,
        field: Option<&str>,
    ) -> SyntaxTree {
        SyntaxTree::Token {
            kind: kind.map(str::to_owned),
            field: field.map(str::to_owned),
            trivia,
            start,
            end,
            attributes: None,
            text: LeafText::Text(text.to_owned()),
            lexical: None,
        }
    }

    fn missing(kind: Option<&str>, position: usize, literal: bool) -> SyntaxTree {
        SyntaxTree::Missing {
            kind: kind.map(str::to_owned),
            start: position,
            end: position,
            literal,
        }
    }

    fn node(kind: &str, start: usize, end: usize, children: Vec<SyntaxTree>) -> SyntaxTree {
        SyntaxTree::Node {
            kind: kind.to_owned(),
            field: None,
            trivia: false,
            start,
            end,
            children,
            attributes: None,
        }
    }

    #[test]
    fn recovery_rows_preserve_error_bytes_named_and_literal_missing_leaves_and_hidden_missing() {
        let source = " α?β\n";
        let tree = node(
            "native_document",
            0,
            7,
            vec![
                token(" ", 0, 1, None, true, None),
                node(
                    "native_group",
                    1,
                    6,
                    vec![
                        token("α", 1, 3, Some("native_name"), false, Some("word")),
                        SyntaxTree::Error {
                            start: 3,
                            end: 4,
                            text: LeafText::Text("?".to_owned()),
                            reason: None,
                        },
                        missing(Some("native_identifier"), 4, false),
                        missing(Some(";"), 4, true),
                        missing(Some("';"), 4, false),
                        missing(None, 4, false),
                        missing(Some("hidden_missing"), 4, false),
                        missing(Some("line_break"), 4, false),
                        token("β", 4, 6, Some("native_name"), false, None),
                    ],
                ),
                token("\n", 6, 7, None, true, None),
            ],
        );
        assert_eq!(
            Value::Array(Rows::new(&projection()).rows(&tree, source)),
            json!([
                [0, null, "document", 1, 1, 7, ""],
                [1, null, "group", 1, 1, 6, ""],
                [2, "word", "identifier", 1, 1, 3, ""],
                [2, null, "ERROR", 1, 3, 4, "E"],
                [2, null, "identifier", 1, 4, 4, "M"],
                [2, null, ";", 0, 4, 4, "M"],
                [2, null, ";", 0, 4, 4, "M"],
                [2, null, null, 1, 4, 4, "M"],
                [2, null, "identifier", 1, 4, 6, ""]
            ])
        );
        assert_eq!(rebuilt_recovery(&tree), source);
    }

    #[test]
    fn missing_only_recovery_keeps_empty_source_and_zero_width_rows() {
        let tree = node(
            "native_document",
            0,
            0,
            vec![
                missing(Some("native_identifier"), 0, false),
                missing(Some("]"), 0, true),
                missing(Some("hidden_missing"), 0, false),
            ],
        );
        assert_eq!(
            Value::Array(Rows::new(&projection()).rows(&tree, "")),
            json!([
                [0, null, "document", 1, 0, 0, ""],
                [1, null, "identifier", 1, 0, 0, "M"],
                [1, null, "]", 0, 0, 0, "M"]
            ])
        );
        assert_eq!(rebuilt_recovery(&tree), "");
    }

    #[test]
    fn error_only_recovery_retains_nul_unicode_and_crlf_source_bytes() {
        let source = "\0é\r\n";
        let tree = node(
            "native_document",
            0,
            5,
            vec![SyntaxTree::Error {
                start: 0,
                end: 5,
                text: LeafText::Text(source.to_owned()),
                reason: None,
            }],
        );
        assert_eq!(
            Value::Array(Rows::new(&projection()).rows(&tree, source)),
            json!([
                [0, null, "document", 1, 0, 5, ""],
                [1, null, "ERROR", 1, 0, 5, "E"]
            ])
        );
        assert_eq!(rebuilt_recovery(&tree), source);
    }

    #[test]
    #[should_panic(expected = "an accepted tree has no")]
    fn accepted_reconstruction_remains_strict_about_error_leaves() {
        let _ = rebuilt(&SyntaxTree::Error {
            start: 0,
            end: 1,
            text: LeafText::Text("?".to_owned()),
            reason: None,
        });
    }

    #[test]
    #[should_panic(expected = "an accepted tree has no")]
    fn accepted_reconstruction_remains_strict_about_missing_leaves() {
        let _ = rebuilt(&missing(Some("identifier"), 0, false));
    }
}
