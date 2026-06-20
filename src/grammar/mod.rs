//! Grammar intermediate representation and links encoding.
//!
//! The grammar IR is a small expression algebra that can hold PEG, BNF, EBNF,
//! ABNF, and inferred grammars without committing to one textual surface
//! syntax. Values can be encoded as first-class grammar links in a
//! [`LinkNetwork`](crate::LinkNetwork).
//!
//! # Example
//!
//! ```
//! use meta_language::{
//!     FromLinks, Grammar, LinkType, LinksDecoder, LinksEncoder, ToLinks,
//! };
//!
//! let expr = Grammar::expr();
//! let grammar = Grammar::builder().start("word").rule("word", expr.rep1(expr.char_range('a', 'z'))).build();
//!
//! let mut encoder = LinksEncoder::new();
//! let root = grammar.to_links(&mut encoder);
//! let network = encoder.into_network();
//! assert!(network.links().any(|link| link.metadata().link_type() == Some(LinkType::Grammar)));
//! let mut decoder = LinksDecoder::new(&network);
//! assert_eq!(Grammar::from_links(&mut decoder, root).expect("grammar decodes"), grammar);
//! ```

pub mod concepts;
pub mod emit;
pub mod import;
pub mod inference;
mod links;
pub mod surface;
pub mod translate;

pub use concepts::{
    annotate_grammar_concepts, grammar_expr_concept_id, rule_concept_id, GrammarConcept,
    GRAMMAR_CONCEPTS,
};
pub use emit::{
    emit_abnf, emit_bnf, emit_ebnf, emit_gbnf, emit_javascript_parser, emit_peggy, emit_pest,
    emit_rust_parser, emit_tree_sitter_grammar_js, emit_tree_sitter_grammar_js_with_report,
    render_rust_type, EmitReport, GrammarEmitError, JsParserArtifacts, RustParserArtifacts,
};
pub use import::{
    import_abnf, import_antlr, import_bnf, import_ebnf, import_gbnf, import_lark, import_pest,
    import_tree_sitter_json, GrammarImportError,
};
pub use inference::eval::{
    evaluate, mdl, run_corpus, run_named_corpus, sample, size_symbols, BenchmarkReport, EvalError,
    GoldenCorpus, GrammarOracle, MembershipOracle, MetricScores, SampleConfig, ScoringMode,
    GOLDEN_CORPORA,
};
pub use inference::lexical::{
    categorise, infer_lexical_classes, CharCategory, LexicalConfig, LexicalModel, Token,
};
pub use inference::prior::{
    build_structural_prior, ByteSpan, Delimiter, LeafKind, PriorOptions, SeedNode, SeedTree,
    StructuralPrior, WhitespacePolicy,
};
pub use inference::sequitur::{run_sequitur, Symbol};
pub use surface::{
    grammar_from_lino, grammar_to_lino, parse_grammar_surface, write_grammar_surface,
    GrammarSurfaceError,
};
pub use translate::{
    grammar_concept_translation_rules, translate_grammar_surface, GrammarTranslateError,
};

use std::collections::BTreeSet;
use std::fmt;

/// One node of the grammar expression algebra.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GrammarExpr {
    /// Matches the empty string.
    Empty,
    /// Literal string terminal, for example `"fn"`.
    Terminal(String),
    /// Case-insensitive literal string terminal.
    TerminalInsensitive(String),
    /// Inclusive character range, for example `'a'..='z'`.
    CharRange(char, char),
    /// Explicit set of characters or ranges.
    CharClass {
        /// Whether the class is negated.
        negated: bool,
        /// Characters and ranges accepted by the class.
        items: Vec<CharClassItem>,
    },
    /// The any-character wildcard.
    AnyChar,
    /// Reference to another grammar rule by name.
    NonTerminal(String),
    /// Alternation between expressions.
    Choice {
        /// Whether alternatives are ordered, as in PEG choice.
        ordered: bool,
        /// Alternative expressions.
        alternatives: Vec<Self>,
    },
    /// Concatenation of expressions.
    Sequence(Vec<Self>),
    /// Optional expression.
    Optional(Box<Self>),
    /// Zero-or-more repetition.
    ZeroOrMore(Box<Self>),
    /// One-or-more repetition.
    OneOrMore(Box<Self>),
    /// Counted repetition.
    Repeat {
        /// Repeated expression.
        expr: Box<Self>,
        /// Minimum number of repetitions.
        min: usize,
        /// Maximum number of repetitions, or `None` for unbounded.
        max: Option<usize>,
    },
    /// Positive lookahead predicate.
    And(Box<Self>),
    /// Negative lookahead predicate.
    Not(Box<Self>),
    /// Labelled or anonymous capture.
    Capture {
        /// Optional capture label.
        label: Option<String>,
        /// Captured expression.
        expr: Box<Self>,
    },
}

impl GrammarExpr {
    /// Builds an empty-string expression.
    #[must_use]
    pub const fn empty() -> Self {
        Self::Empty
    }

    /// Builds a literal terminal expression.
    #[must_use]
    pub fn terminal(value: impl Into<String>) -> Self {
        Self::Terminal(value.into())
    }

    /// Builds a case-insensitive literal terminal expression.
    #[must_use]
    pub fn terminal_insensitive(value: impl Into<String>) -> Self {
        Self::TerminalInsensitive(value.into())
    }

    /// Builds an inclusive character range expression.
    #[must_use]
    pub const fn char_range(start: char, end: char) -> Self {
        Self::CharRange(start, end)
    }

    /// Builds a character class expression.
    #[must_use]
    pub fn char_class<I>(negated: bool, items: I) -> Self
    where
        I: IntoIterator<Item = CharClassItem>,
    {
        Self::CharClass {
            negated,
            items: items.into_iter().collect(),
        }
    }

    /// Builds an any-character wildcard expression.
    #[must_use]
    pub const fn any_char() -> Self {
        Self::AnyChar
    }

    /// Builds a non-terminal reference expression.
    #[must_use]
    pub fn non_terminal(value: impl Into<String>) -> Self {
        Self::NonTerminal(value.into())
    }

    /// Builds a choice expression.
    #[must_use]
    pub fn choice<I>(ordered: bool, alternatives: I) -> Self
    where
        I: IntoIterator<Item = Self>,
    {
        Self::Choice {
            ordered,
            alternatives: alternatives.into_iter().collect(),
        }
    }

    /// Builds a sequence expression.
    #[must_use]
    pub fn sequence<I>(items: I) -> Self
    where
        I: IntoIterator<Item = Self>,
    {
        Self::Sequence(items.into_iter().collect())
    }

    /// Builds an optional expression.
    #[must_use]
    pub fn optional(expr: Self) -> Self {
        Self::Optional(Box::new(expr))
    }

    /// Builds a zero-or-more repetition expression.
    #[must_use]
    pub fn zero_or_more(expr: Self) -> Self {
        Self::ZeroOrMore(Box::new(expr))
    }

    /// Builds a one-or-more repetition expression.
    #[must_use]
    pub fn one_or_more(expr: Self) -> Self {
        Self::OneOrMore(Box::new(expr))
    }

    /// Builds a counted repetition expression.
    #[must_use]
    pub fn repeat(expr: Self, min: usize, max: Option<usize>) -> Self {
        Self::Repeat {
            expr: Box::new(expr),
            min,
            max,
        }
    }

    /// Builds a positive lookahead expression.
    #[must_use]
    pub fn and(expr: Self) -> Self {
        Self::And(Box::new(expr))
    }

    /// Builds a negative lookahead expression.
    #[must_use]
    #[allow(clippy::should_implement_trait)]
    pub fn not(expr: Self) -> Self {
        Self::Not(Box::new(expr))
    }

    /// Builds a labelled capture expression.
    #[must_use]
    pub fn capture(label: impl Into<String>, expr: Self) -> Self {
        Self::Capture {
            label: Some(label.into()),
            expr: Box::new(expr),
        }
    }

    /// Builds an anonymous capture expression.
    #[must_use]
    pub fn capture_unlabeled(expr: Self) -> Self {
        Self::Capture {
            label: None,
            expr: Box::new(expr),
        }
    }

    fn collect_nonterminals(&self, names: &mut BTreeSet<String>) {
        match self {
            Self::NonTerminal(name) => {
                names.insert(name.clone());
            }
            Self::Choice { alternatives, .. } => {
                for alternative in alternatives {
                    alternative.collect_nonterminals(names);
                }
            }
            Self::Sequence(items) => {
                for item in items {
                    item.collect_nonterminals(names);
                }
            }
            Self::Optional(expr)
            | Self::ZeroOrMore(expr)
            | Self::OneOrMore(expr)
            | Self::And(expr)
            | Self::Not(expr)
            | Self::Capture { expr, .. }
            | Self::Repeat { expr, .. } => expr.collect_nonterminals(names),
            Self::Empty
            | Self::Terminal(_)
            | Self::TerminalInsensitive(_)
            | Self::CharRange(_, _)
            | Self::CharClass { .. }
            | Self::AnyChar => {}
        }
    }
}

impl fmt::Display for GrammarExpr {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("empty"),
            Self::Terminal(value) => write!(formatter, "{value:?}"),
            Self::TerminalInsensitive(value) => write!(formatter, "i{value:?}"),
            Self::CharRange(start, end) => write!(formatter, "{start:?}..={end:?}"),
            Self::CharClass { negated, items } => {
                let marker = if *negated { "^" } else { "" };
                write!(formatter, "[{marker}")?;
                for item in items {
                    write!(formatter, "{item}")?;
                }
                formatter.write_str("]")
            }
            Self::AnyChar => formatter.write_str("."),
            Self::NonTerminal(name) => formatter.write_str(name),
            Self::Choice {
                ordered,
                alternatives,
            } => {
                let separator = if *ordered { " / " } else { " | " };
                write_joined(formatter, alternatives, separator)
            }
            Self::Sequence(items) => write_joined(formatter, items, " "),
            Self::Optional(expr) => write!(formatter, "({expr})?"),
            Self::ZeroOrMore(expr) => write!(formatter, "({expr})*"),
            Self::OneOrMore(expr) => write!(formatter, "({expr})+"),
            Self::Repeat { expr, min, max } => match max {
                Some(max) => write!(formatter, "({expr}){{{min},{max}}}"),
                None => write!(formatter, "({expr}){{{min},}}"),
            },
            Self::And(expr) => write!(formatter, "&({expr})"),
            Self::Not(expr) => write!(formatter, "!({expr})"),
            Self::Capture { label, expr } => match label {
                Some(label) => write!(formatter, "{label}:({expr})"),
                None => write!(formatter, "capture({expr})"),
            },
        }
    }
}

/// One item inside a character class.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CharClassItem {
    /// A single character.
    Char(char),
    /// An inclusive character range.
    Range(char, char),
}

impl CharClassItem {
    /// Builds a single-character class item.
    #[must_use]
    pub const fn char(value: char) -> Self {
        Self::Char(value)
    }

    /// Builds an inclusive character range class item.
    #[must_use]
    pub const fn range(start: char, end: char) -> Self {
        Self::Range(start, end)
    }
}

impl fmt::Display for CharClassItem {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Char(value) => write!(formatter, "{}", value.escape_default()),
            Self::Range(start, end) => {
                write!(
                    formatter,
                    "{}-{}",
                    start.escape_default(),
                    end.escape_default()
                )
            }
        }
    }
}

/// How a rule participates in parsing.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RuleKind {
    /// A normal rule that participates in the parse tree.
    Normal,
    /// An atomic rule whose inner expression is treated as an indivisible token.
    Atomic,
    /// A silent rule that can be omitted from visible parse output.
    Silent,
    /// A token-level rule.
    Token,
}

impl RuleKind {
    /// Stable tag used in links encoding and display output.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Normal => "normal",
            Self::Atomic => "atomic",
            Self::Silent => "silent",
            Self::Token => "token",
        }
    }

    pub(crate) fn from_tag(value: &str) -> Option<Self> {
        match value {
            "normal" => Some(Self::Normal),
            "atomic" => Some(Self::Atomic),
            "silent" => Some(Self::Silent),
            "token" => Some(Self::Token),
            _ => None,
        }
    }
}

impl fmt::Display for RuleKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// A named grammar rule.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GrammarRule {
    /// Rule name.
    pub name: String,
    /// Rule expression.
    pub expr: GrammarExpr,
    /// Rule participation kind.
    pub kind: RuleKind,
    /// Optional concept-ontology alignment.
    pub concept: Option<String>,
    /// Optional free-text documentation or comment.
    pub doc: Option<String>,
}

impl GrammarRule {
    /// Builds a normal grammar rule.
    #[must_use]
    pub fn new(name: impl Into<String>, expr: GrammarExpr) -> Self {
        Self {
            name: name.into(),
            expr,
            kind: RuleKind::Normal,
            concept: None,
            doc: None,
        }
    }

    /// Returns this rule with a different rule kind.
    #[must_use]
    pub const fn with_kind(mut self, kind: RuleKind) -> Self {
        self.kind = kind;
        self
    }

    /// Returns this rule with concept-ontology alignment.
    #[must_use]
    pub fn with_concept(mut self, concept: impl Into<String>) -> Self {
        self.concept = Some(concept.into());
        self
    }

    /// Returns this rule with documentation text.
    #[must_use]
    pub fn with_doc(mut self, doc: impl Into<String>) -> Self {
        self.doc = Some(doc.into());
        self
    }

    /// Rule name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Rule expression.
    #[must_use]
    pub const fn expr(&self) -> &GrammarExpr {
        &self.expr
    }

    /// Rule participation kind.
    #[must_use]
    pub const fn kind(&self) -> RuleKind {
        self.kind
    }

    /// Concept-ontology alignment, when present.
    #[must_use]
    pub fn concept(&self) -> Option<&str> {
        self.concept.as_deref()
    }

    /// Rule documentation, when present.
    #[must_use]
    pub fn doc(&self) -> Option<&str> {
        self.doc.as_deref()
    }
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

/// Order-preserving grammar.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Grammar {
    rules: Vec<GrammarRule>,
    start: Option<String>,
    source_format: Option<GrammarFormat>,
}

impl Grammar {
    /// Builds an empty grammar.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            rules: Vec::new(),
            start: None,
            source_format: None,
        }
    }

    /// Builds a fluent grammar builder.
    #[must_use]
    pub const fn builder() -> GrammarBuilder {
        GrammarBuilder::new()
    }

    /// Builds an expression builder.
    #[must_use]
    pub const fn expr() -> ExprBuilder {
        ExprBuilder
    }

    /// Returns this grammar with an additional rule.
    #[must_use]
    pub fn with_rule(mut self, rule: GrammarRule) -> Self {
        self.rules.push(rule);
        self
    }

    /// Returns this grammar with a start rule name.
    #[must_use]
    pub fn with_start(mut self, start: impl Into<String>) -> Self {
        self.start = Some(start.into());
        self
    }

    /// Returns this grammar with a source format.
    #[must_use]
    pub const fn with_source_format(mut self, source_format: GrammarFormat) -> Self {
        self.source_format = Some(source_format);
        self
    }

    /// Adds a rule to the grammar.
    pub fn add_rule(&mut self, rule: GrammarRule) {
        self.rules.push(rule);
    }

    /// Sets the grammar start rule name.
    pub fn set_start(&mut self, start: impl Into<String>) {
        self.start = Some(start.into());
    }

    /// Clears the explicit grammar start rule.
    pub fn clear_start(&mut self) {
        self.start = None;
    }

    /// Sets the grammar source format.
    pub const fn set_source_format(&mut self, source_format: GrammarFormat) {
        self.source_format = Some(source_format);
    }

    /// Returns all rules in source order.
    #[must_use]
    pub fn rules(&self) -> &[GrammarRule] {
        &self.rules
    }

    /// Returns the rule with `name`, when present.
    #[must_use]
    pub fn rule(&self, name: &str) -> Option<&GrammarRule> {
        self.rules.iter().find(|rule| rule.name == name)
    }

    /// Returns the explicitly configured start symbol, if present.
    #[must_use]
    pub fn start(&self) -> Option<&str> {
        self.start.as_deref()
    }

    /// Returns the start rule, defaulting to the first rule when unset.
    #[must_use]
    pub fn start_rule(&self) -> Option<&GrammarRule> {
        self.start
            .as_deref()
            .map_or_else(|| self.rules.first(), |start| self.rule(start))
    }

    /// Returns the source format, if known.
    #[must_use]
    pub const fn source_format(&self) -> Option<GrammarFormat> {
        self.source_format
    }

    /// Returns rule names in source order.
    #[must_use]
    pub fn rule_names(&self) -> Vec<&str> {
        self.rules.iter().map(GrammarRule::name).collect()
    }

    /// Returns non-terminal names referenced from every rule expression.
    #[must_use]
    pub fn referenced_nonterminals(&self) -> BTreeSet<String> {
        let mut names = BTreeSet::new();
        for rule in &self.rules {
            rule.expr.collect_nonterminals(&mut names);
        }
        names
    }

    /// Returns referenced non-terminals that do not have a local rule.
    #[must_use]
    pub fn undefined_nonterminals(&self) -> BTreeSet<String> {
        let defined = self
            .rules
            .iter()
            .map(|rule| rule.name.clone())
            .collect::<BTreeSet<_>>();
        self.referenced_nonterminals()
            .difference(&defined)
            .cloned()
            .collect()
    }
}

/// Fluent builder for order-preserving grammars.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct GrammarBuilder {
    grammar: Grammar,
}

impl GrammarBuilder {
    /// Builds an empty grammar builder.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            grammar: Grammar::new(),
        }
    }

    /// Returns this builder with a source format.
    #[must_use]
    pub const fn source_format(mut self, source_format: GrammarFormat) -> Self {
        self.grammar.source_format = Some(source_format);
        self
    }

    /// Returns this builder with a start rule name.
    #[must_use]
    pub fn start(mut self, start: impl Into<String>) -> Self {
        self.grammar.start = Some(start.into());
        self
    }

    /// Adds a normal rule from a name and expression.
    #[must_use]
    pub fn rule(mut self, name: impl Into<String>, expr: GrammarExpr) -> Self {
        self.grammar.rules.push(GrammarRule::new(name, expr));
        self
    }

    /// Adds a complete rule.
    #[must_use]
    pub fn grammar_rule(mut self, rule: GrammarRule) -> Self {
        self.grammar.rules.push(rule);
        self
    }

    /// Adds a rule with an explicit kind.
    #[must_use]
    pub fn rule_with_kind(
        mut self,
        name: impl Into<String>,
        expr: GrammarExpr,
        kind: RuleKind,
    ) -> Self {
        self.grammar
            .rules
            .push(GrammarRule::new(name, expr).with_kind(kind));
        self
    }

    /// Finishes the builder.
    #[must_use]
    pub fn build(self) -> Grammar {
        self.grammar
    }
}

/// Ergonomic constructor for grammar expressions.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ExprBuilder;

impl ExprBuilder {
    /// Builds an empty-string expression.
    #[must_use]
    pub const fn empty(self) -> GrammarExpr {
        GrammarExpr::Empty
    }

    /// Builds a literal terminal.
    #[must_use]
    pub fn term(self, value: impl Into<String>) -> GrammarExpr {
        GrammarExpr::terminal(value)
    }

    /// Builds a literal terminal.
    #[must_use]
    pub fn terminal(self, value: impl Into<String>) -> GrammarExpr {
        GrammarExpr::terminal(value)
    }

    /// Builds a case-insensitive literal terminal.
    #[must_use]
    pub fn terminal_insensitive(self, value: impl Into<String>) -> GrammarExpr {
        GrammarExpr::terminal_insensitive(value)
    }

    /// Builds a single-character range expression.
    #[must_use]
    pub const fn char(self, value: char) -> GrammarExpr {
        GrammarExpr::CharRange(value, value)
    }

    /// Builds an inclusive character range expression.
    #[must_use]
    pub const fn char_range(self, start: char, end: char) -> GrammarExpr {
        GrammarExpr::CharRange(start, end)
    }

    /// Builds a character class.
    #[must_use]
    pub fn char_class<I>(self, negated: bool, items: I) -> GrammarExpr
    where
        I: IntoIterator<Item = CharClassItem>,
    {
        GrammarExpr::char_class(negated, items)
    }

    /// Builds an any-character wildcard.
    #[must_use]
    pub const fn any(self) -> GrammarExpr {
        GrammarExpr::AnyChar
    }

    /// Builds a non-terminal reference.
    #[must_use]
    pub fn nt(self, value: impl Into<String>) -> GrammarExpr {
        GrammarExpr::non_terminal(value)
    }

    /// Builds a non-terminal reference.
    #[must_use]
    pub fn non_terminal(self, value: impl Into<String>) -> GrammarExpr {
        GrammarExpr::non_terminal(value)
    }

    /// Builds a choice expression.
    #[must_use]
    pub fn choice<I>(self, ordered: bool, alternatives: I) -> GrammarExpr
    where
        I: IntoIterator<Item = GrammarExpr>,
    {
        GrammarExpr::choice(ordered, alternatives)
    }

    /// Builds an ordered choice expression.
    #[must_use]
    pub fn choice_ordered<I>(self, alternatives: I) -> GrammarExpr
    where
        I: IntoIterator<Item = GrammarExpr>,
    {
        GrammarExpr::choice(true, alternatives)
    }

    /// Builds an unordered choice expression.
    #[must_use]
    pub fn choice_unordered<I>(self, alternatives: I) -> GrammarExpr
    where
        I: IntoIterator<Item = GrammarExpr>,
    {
        GrammarExpr::choice(false, alternatives)
    }

    /// Builds a sequence expression.
    #[must_use]
    pub fn seq<I>(self, items: I) -> GrammarExpr
    where
        I: IntoIterator<Item = GrammarExpr>,
    {
        GrammarExpr::sequence(items)
    }

    /// Builds an optional expression.
    #[must_use]
    pub fn opt(self, expr: GrammarExpr) -> GrammarExpr {
        GrammarExpr::optional(expr)
    }

    /// Builds a zero-or-more repetition expression.
    #[must_use]
    pub fn rep0(self, expr: GrammarExpr) -> GrammarExpr {
        GrammarExpr::zero_or_more(expr)
    }

    /// Builds a one-or-more repetition expression.
    #[must_use]
    pub fn rep1(self, expr: GrammarExpr) -> GrammarExpr {
        GrammarExpr::one_or_more(expr)
    }

    /// Builds a counted repetition expression.
    #[must_use]
    pub fn repeat(self, expr: GrammarExpr, min: usize, max: Option<usize>) -> GrammarExpr {
        GrammarExpr::repeat(expr, min, max)
    }

    /// Builds a positive lookahead expression.
    #[must_use]
    pub fn and(self, expr: GrammarExpr) -> GrammarExpr {
        GrammarExpr::and(expr)
    }

    /// Builds a negative lookahead expression.
    #[must_use]
    pub fn not(self, expr: GrammarExpr) -> GrammarExpr {
        GrammarExpr::not(expr)
    }

    /// Builds a labelled capture expression.
    #[must_use]
    pub fn capture(self, label: Option<impl Into<String>>, expr: GrammarExpr) -> GrammarExpr {
        match label {
            Some(label) => GrammarExpr::capture(label, expr),
            None => GrammarExpr::capture_unlabeled(expr),
        }
    }

    /// Builds an anonymous capture expression.
    #[must_use]
    pub fn capture_unlabeled(self, expr: GrammarExpr) -> GrammarExpr {
        GrammarExpr::capture_unlabeled(expr)
    }
}

fn write_joined(
    formatter: &mut fmt::Formatter<'_>,
    expressions: &[GrammarExpr],
    separator: &str,
) -> fmt::Result {
    if let Some((first, rest)) = expressions.split_first() {
        write!(formatter, "{first}")?;
        for expression in rest {
            formatter.write_str(separator)?;
            write!(formatter, "{expression}")?;
        }
    }
    Ok(())
}
