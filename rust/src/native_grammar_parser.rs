//! The default parse of a language whose catalog grammar is a native Links
//! Notation grammar (the catalog's `nativeGrammars`): the grammar runs on the
//! native executor with automatic error recovery, and its tree is projected as
//! the language's tree-sitter oracle (the catalog's `oracleGrammars`) places
//! nodes, so the default concrete syntax tree keeps its shape when the native
//! grammar replaces the oracle. This is the port of
//! `js/src/native-grammar-parser.js`.
//!
//! Whitespace trivia is not a node, named trivia (a comment token) is an extra
//! node, leading trivia belongs before the node it precedes, a node spans its
//! first to last non-trivia leaf, and the root starts at its first leaf that is
//! not whitespace and ends at the end of the input. Kinds in `hidden` are
//! leaves the oracle drops, such as a byte order mark, and kinds in
//! `anonymous` are leaves the oracle keeps inside a node without a node of
//! their own; the text of both is the gap text between nodes. Node kinds in
//! `extras` are extra nodes, as the oracle marks a comment node it parses as an
//! extra. A rule renamed from its tree-sitter name keeps that name as its
//! node kind (`oracle_kinds`, from the rule's `(source-names (tree-sitter
//! NAME))`). A kind `'TEXT`, an imported anonymous alias, is an anonymous
//! node TEXT. An ERROR leaf is a named `ERROR` node and a MISSING leaf an
//! empty MISSING node, named unless it stands for a literal.

use std::collections::{BTreeMap, BTreeSet};
use std::sync::OnceLock;

use crate::{
    FeatureGrammarParser, FeatureParseOptions, LeafText, SyntaxTree, compile_feature_grammar,
    native_grammar, parse_grammar_links,
};

/// The Links Notation text of every native grammar the catalog ships, by
/// catalog file; `tests/unit/default_cst_expectations.rs` checks that the
/// catalog's native grammars are exactly the shipped files.
const NATIVE_GRAMMAR_TEXTS: &[(&str, &str)] = &[
    (
        "native-grammars/c.lino",
        include_str!("data/native-grammars/c.lino"),
    ),
    (
        "native-grammars/csv.lino",
        include_str!("data/native-grammars/csv.lino"),
    ),
    (
        "native-grammars/diff.lino",
        include_str!("data/native-grammars/diff.lino"),
    ),
    (
        "native-grammars/ini.lino",
        include_str!("data/native-grammars/ini.lino"),
    ),
    (
        "native-grammars/javascript.lino",
        include_str!("data/native-grammars/javascript.lino"),
    ),
    (
        "native-grammars/json.lino",
        include_str!("data/native-grammars/json.lino"),
    ),
    (
        "native-grammars/json5.lino",
        include_str!("data/native-grammars/json5.lino"),
    ),
    (
        "native-grammars/lean.lino",
        include_str!("data/native-grammars/lean.lino"),
    ),
    (
        "native-grammars/racket.lino",
        include_str!("data/native-grammars/racket.lino"),
    ),
    (
        "native-grammars/rocq.lino",
        include_str!("data/native-grammars/rocq.lino"),
    ),
    (
        "native-grammars/rust.lino",
        include_str!("data/native-grammars/rust.lino"),
    ),
    (
        "native-grammars/scheme.lino",
        include_str!("data/native-grammars/scheme.lino"),
    ),
    (
        "native-grammars/tsx.lino",
        include_str!("data/native-grammars/tsx.lino"),
    ),
    (
        "native-grammars/typescript.lino",
        include_str!("data/native-grammars/typescript.lino"),
    ),
];

/// One node of a projected native tree; offsets are UTF-8 byte offsets. The
/// flags mirror tree-sitter's node flags.
#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NativeNode {
    pub term: String,
    pub named: bool,
    pub start: usize,
    pub end: usize,
    pub is_error: bool,
    pub is_missing: bool,
    pub is_extra: bool,
    pub has_error: bool,
    /// The children with the field each is captured under.
    pub children: Vec<(Self, Option<String>)>,
}

/// Whether the grammar id `id` names a native grammar of the catalog.
pub fn is_native_grammar(id: &str) -> bool {
    native_grammar(id).is_some()
}

fn native_grammar_index(id: &str) -> Option<usize> {
    let file = &native_grammar(id)?.file;
    NATIVE_GRAMMAR_TEXTS
        .iter()
        .position(|(path, _)| path == file)
}

/// The Links Notation text of the native grammar `id`.
pub fn native_grammar_text(id: &str) -> Option<&'static str> {
    native_grammar_index(id).map(|index| NATIVE_GRAMMAR_TEXTS[index].1)
}

// Each native grammar is compiled on its first use and kept for the process.
pub fn native_parser(id: &str) -> Option<&'static FeatureGrammarParser> {
    static PARSERS: [OnceLock<Option<FeatureGrammarParser>>; NATIVE_GRAMMAR_TEXTS.len()] =
        [const { OnceLock::new() }; NATIVE_GRAMMAR_TEXTS.len()];
    let index = native_grammar_index(id)?;
    PARSERS[index]
        .get_or_init(|| {
            let grammar = parse_grammar_links(NATIVE_GRAMMAR_TEXTS[index].1).ok()?;
            compile_feature_grammar(&grammar, None, FeatureParseOptions::default()).ok()
        })
        .as_ref()
}

/// Parses `source` with the native grammar `id` and returns the projected
/// root. Input the executor cannot finish within its resource limits is one
/// ERROR root.
pub fn parse_native(id: &str, source: &str) -> NativeNode {
    let length = source.len();
    let error_root = || NativeNode {
        term: "ERROR".to_owned(),
        named: true,
        start: 0,
        end: length,
        is_error: true,
        is_missing: false,
        is_extra: false,
        has_error: true,
        children: Vec::new(),
    };
    let (Some(entry), Some(parser)) = (native_grammar(id), native_parser(id)) else {
        return error_root();
    };
    let options = FeatureParseOptions {
        error_recovery: Some(true),
        accept_recovery: Some(true),
        ..FeatureParseOptions::default()
    };
    match parser.parse_tree(source.as_bytes(), &options) {
        Ok(outcome) => match outcome.tree {
            Some(tree @ SyntaxTree::Node { .. }) => Projection {
                hidden: entry.hidden.iter().map(String::as_str).collect(),
                anonymous: entry.anonymous.iter().map(String::as_str).collect(),
                extras: entry.extras.iter().map(String::as_str).collect(),
                oracle_kinds: &entry.oracle_kinds,
            }
            .root(&tree, length),
            _ => error_root(),
        },
        Err(_) => error_root(),
    }
}

struct Projection<'a> {
    hidden: BTreeSet<&'a str>,
    anonymous: BTreeSet<&'a str>,
    extras: BTreeSet<&'a str>,
    oracle_kinds: &'a BTreeMap<String, String>,
}

impl Projection<'_> {
    /// The tree-sitter kind of a native kind: the kind itself unless its rule
    /// was renamed from it, and the text of an anonymous alias `'TEXT`.
    fn oracle_kind(&self, kind: &str) -> String {
        kind.strip_prefix('\'').map_or_else(
            || {
                self.oracle_kinds
                    .get(kind)
                    .map_or_else(|| kind.to_owned(), Clone::clone)
            },
            str::to_owned,
        )
    }

    fn invisible(&self, node: &SyntaxTree) -> bool {
        matches!(node, SyntaxTree::Token { kind, trivia, .. }
            if (*trivia && kind.is_none())
                || kind.as_deref().is_some_and(|kind| self.hidden.contains(kind)))
    }

    fn trivia(&self, node: &SyntaxTree) -> bool {
        self.invisible(node)
            || matches!(node, SyntaxTree::Token { trivia: true, .. })
            || matches!(node, SyntaxTree::Node { kind, .. } if self.extras.contains(kind.as_str()))
    }

    fn anonymous(&self, node: &SyntaxTree) -> bool {
        matches!(node, SyntaxTree::Token { kind: Some(kind), .. } if self.anonymous.contains(kind.as_str()))
    }

    /// Moves the leading trivia of a node before it.
    fn hoist(&self, node: &SyntaxTree) -> Vec<SyntaxTree> {
        let SyntaxTree::Node {
            kind,
            field,
            trivia,
            start,
            end,
            children,
            attributes,
        } = node
        else {
            return vec![node.clone()];
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

    fn span(&self, node: &SyntaxTree) -> (usize, usize) {
        match node {
            SyntaxTree::Node {
                start, children, ..
            } => {
                let inner: Vec<(usize, usize)> = children
                    .iter()
                    .filter(|child| !self.trivia(child))
                    .map(|child| self.span(child))
                    .collect();
                match (inner.first(), inner.last()) {
                    (Some(first), Some(last)) => (first.0, last.1),
                    _ => (*start, *start),
                }
            }
            SyntaxTree::Token { start, end, .. }
            | SyntaxTree::Error { start, end, .. }
            | SyntaxTree::Missing { start, end, .. }
            | SyntaxTree::Embed { start, end, .. } => (*start, *end),
        }
    }

    fn project(&self, children: &[SyntaxTree]) -> Vec<(NativeNode, Option<String>)> {
        children
            .iter()
            .flat_map(|child| self.hoist(child))
            .filter(|child| !self.invisible(child) && !self.anonymous(child))
            .map(|child| {
                let field = match &child {
                    SyntaxTree::Node { field, .. } | SyntaxTree::Token { field, .. } => {
                        field.clone()
                    }
                    _ => None,
                };
                (self.node(&child), field)
            })
            .collect()
    }

    fn node(&self, node: &SyntaxTree) -> NativeNode {
        let (start, end) = self.span(node);
        let leaf = |term: String, named: bool| NativeNode {
            term,
            named,
            start,
            end,
            is_error: false,
            is_missing: false,
            is_extra: false,
            has_error: false,
            children: Vec::new(),
        };
        match node {
            SyntaxTree::Node { kind, children, .. } => {
                let children = self.project(children);
                NativeNode {
                    has_error: children.iter().any(|(child, _)| child.has_error),
                    is_extra: self.extras.contains(kind.as_str()),
                    children,
                    ..leaf(self.oracle_kind(kind), !anonymous_alias(kind))
                }
            }
            SyntaxTree::Error { .. } => NativeNode {
                is_error: true,
                has_error: true,
                ..leaf("ERROR".to_owned(), true)
            },
            SyntaxTree::Missing { kind, literal, .. } => NativeNode {
                is_missing: true,
                has_error: true,
                ..leaf(
                    kind.as_deref()
                        .map_or_else(|| "MISSING".to_owned(), |kind| self.oracle_kind(kind)),
                    !literal && !kind.as_deref().is_some_and(anonymous_alias),
                )
            },
            SyntaxTree::Token {
                kind, trivia, text, ..
            } => NativeNode {
                is_extra: *trivia,
                ..leaf(
                    kind.as_deref().map_or_else(
                        || match text {
                            LeafText::Text(text) | LeafText::Hex(text) => text.clone(),
                        },
                        |kind| self.oracle_kind(kind),
                    ),
                    kind.as_deref().is_some_and(|kind| !anonymous_alias(kind)),
                )
            },
            SyntaxTree::Embed { language, .. } => leaf(language.clone(), true),
        }
    }

    fn root(&self, tree: &SyntaxTree, length: usize) -> NativeNode {
        let SyntaxTree::Node { kind, children, .. } = tree else {
            unreachable!("the root of a native tree is a node");
        };
        let mut leaves = Vec::new();
        collect_leaves(tree, &mut leaves);
        let start = leaves
            .iter()
            .find(|leaf| !self.invisible(leaf))
            .map_or(length, |leaf| self.span(leaf).0);
        let children = self.project(children);
        NativeNode {
            term: self.oracle_kind(kind),
            named: true,
            start,
            end: length,
            is_error: false,
            is_missing: false,
            is_extra: false,
            has_error: children.iter().any(|(child, _)| child.has_error),
            children,
        }
    }
}

/// Whether a kind is an imported anonymous alias `'TEXT`.
fn anonymous_alias(kind: &str) -> bool {
    kind.starts_with('\'')
}

fn collect_leaves<'a>(node: &'a SyntaxTree, leaves: &mut Vec<&'a SyntaxTree>) {
    if let SyntaxTree::Node { children, .. } = node {
        for child in children {
            collect_leaves(child, leaves);
        }
    } else {
        leaves.push(node);
    }
}
