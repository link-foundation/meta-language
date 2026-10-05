//! The metadata of a grammar that keeps the names its rules and node kinds
//! have in the grammars it was imported or merged from, for the reverse
//! conversion and for the default trees that keep the upstream node kinds,
//! and the format a grammar comes from.

use std::fmt;

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

/// Origin grammar format.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GrammarFormat {
    /// The meta-language's own grammar notation.
    MetaLanguage,
    /// Backus-Naur Form.
    Bnf,
    /// Extended Backus-Naur Form.
    Ebnf,
    /// Augmented Backus-Naur Form.
    Abnf,
    /// Parsing Expression Grammar.
    Peg,
    /// ANTLR grammar.
    Antlr,
    /// Lark grammar.
    Lark,
    /// GBNF grammar.
    Gbnf,
    /// Tree-sitter grammar.
    TreeSitter,
    /// Grammar inferred from examples or observations.
    Inferred,
}

impl GrammarFormat {
    /// Stable tag used in links encoding and display output.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MetaLanguage => "meta-language",
            Self::Bnf => "bnf",
            Self::Ebnf => "ebnf",
            Self::Abnf => "abnf",
            Self::Peg => "peg",
            Self::Antlr => "antlr",
            Self::Lark => "lark",
            Self::Gbnf => "gbnf",
            Self::TreeSitter => "tree-sitter",
            Self::Inferred => "inferred",
        }
    }

    pub(crate) fn from_tag(value: &str) -> Option<Self> {
        match value {
            "meta-language" => Some(Self::MetaLanguage),
            "bnf" => Some(Self::Bnf),
            "ebnf" => Some(Self::Ebnf),
            "abnf" => Some(Self::Abnf),
            "peg" => Some(Self::Peg),
            "antlr" => Some(Self::Antlr),
            "lark" => Some(Self::Lark),
            "gbnf" => Some(Self::Gbnf),
            "tree-sitter" => Some(Self::TreeSitter),
            "inferred" => Some(Self::Inferred),
            _ => None,
        }
    }
}

impl fmt::Display for GrammarFormat {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}
