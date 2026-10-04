//! The metadata of a grammar that keeps the names its rules and node kinds
//! have in the grammars it was imported or merged from, for the reverse
//! conversion and for the default trees that keep the upstream node kinds.

/// A name a rule has in a grammar it was merged from, such as its tree-sitter
/// or ANTLR name, kept for the reverse conversion.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GrammarSourceName {
    /// The grammar source, such as `tree-sitter` or `antlr`.
    pub source: String,
    /// The rule name in that source.
    pub name: String,
}

/// A node kind no rule defines, with its names in the grammars it came from.
///
/// Such a kind is one an alias names: tree-sitter's
/// `alias($._line_doc_content, $.doc_comment)` imports as the kind
/// `documentation_comment` with the tree-sitter name `doc_comment`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GrammarKind {
    /// The native kind name.
    pub name: String,
    /// The names of the kind in its sources.
    pub source_names: Vec<GrammarSourceName>,
}
