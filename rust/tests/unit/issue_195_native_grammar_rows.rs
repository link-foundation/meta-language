//! Shared helpers of the native merged grammar suites
//! (`issue_195_grammar_native_json.rs`, `issue_195_grammar_native_ini.rs`,
//! `issue_195_grammar_native_diff.rs`, `issue_195_grammar_native_csv.rs`,
//! `issue_195_grammar_native_json5.rs`, `issue_195_grammar_native_scheme.rs`): they read a fixture of
//! parity/fixtures/native-grammars/, parse with the Rust
//! executor and project its trees to the rows of the tree-sitter oracle the
//! way js/scripts/native-grammar-rows.mjs projects the JavaScript ones.

use std::collections::HashSet;

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
/// whitespace, `anonymous` leaves are not rows but count in the spans, and
/// `extras` nodes are rows with flag X.
pub struct Rows<'a> {
    hidden: HashSet<&'a str>,
    anonymous: HashSet<&'a str>,
    extras: HashSet<&'a str>,
}

impl<'a> Rows<'a> {
    pub fn new(fixture: &'a Value) -> Self {
        Self {
            hidden: kinds(fixture, "hidden"),
            anonymous: kinds(fixture, "anonymous"),
            extras: kinds(fixture, "extras"),
        }
    }

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

    fn anonymous(&self, tree: &SyntaxTree) -> bool {
        matches!(tree, SyntaxTree::Token { kind: Some(kind), .. } if self.anonymous.contains(kind.as_str()))
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
        if self.invisible(tree) || self.anonymous(tree) {
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
                let flags = if self.extras.contains(kind.as_str()) {
                    "X"
                } else {
                    ""
                };
                rows.push(json!([depth, field, kind, 1, start, end, flags]));
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
            other => panic!("an accepted tree has no {other:?}"),
        }
    }

    pub fn rows(&self, tree: &SyntaxTree, source: &str) -> Vec<Value> {
        let SyntaxTree::Node { kind, children, .. } = tree else {
            panic!("the root is a node");
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
