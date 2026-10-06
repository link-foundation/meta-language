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

mod builder;
pub mod concepts;
pub mod decorators;
pub mod emit;
pub mod feature;
pub mod feature_runtime;
pub mod fidelity;
pub mod import;
pub mod inference;
pub mod interchange;
mod links;
pub mod merge;
mod metadata;
pub mod reverse;
pub mod round_trip;
pub mod runtime;
pub mod surface;
pub mod translate;
pub mod validate;

pub use builder::{
    ExprBuilder, GrammarBuilder, RepetitionBoundsError, canonical_repeat, choice, sequence,
};
pub use concepts::{
    GRAMMAR_CONCEPTS, GrammarConcept, annotate_grammar_concepts, grammar_expr_concept_id,
    rule_concept_id,
};
pub use decorators::{decorate_emitted, decorate_grammar, decorate_syntax_tree};
pub use emit::{
    EmitReport, GrammarEmitError, JsParserArtifacts, RustParserArtifacts, emit_abnf, emit_antlr,
    emit_bnf, emit_ebnf, emit_gbnf, emit_javascript_parser, emit_lark, emit_peggy, emit_pest,
    emit_rust_parser, emit_tree_sitter_grammar_js, emit_tree_sitter_grammar_js_with_report,
    emit_tree_sitter_json, render_rust_type,
};
pub use feature::{
    ByteClassItem, FEATURE_EXPRESSION_FORMS, FeatureExpr, FeatureForm, FieldType, FieldValue,
    GrammarDeclarations, GrammarMacro, GrammarScanner, MATCHING_MODES, OPERATION_FORMS, Operation,
    OperationCategory, PrecedenceEntry, RuleAttributes, SETTLING_STEPS, UnicodeClassItem,
    default_settling, settling_problem,
};
pub use fidelity::{
    FORMER_GRAMMAR_CONSTRUCTS, GRAMMAR_CONSTRUCTS, GRAMMAR_FORMATS, GrammarFidelityLevel,
    GrammarFormatProfile, canonical_grammar_format, current_grammar_construct,
    grammar_format_profile,
};
pub use import::{
    GrammarImportError, import_abnf, import_antlr, import_bnf, import_ebnf, import_gbnf,
    import_lark, import_pest, import_tree_sitter_json,
};
pub use inference::active::{
    ActiveLearningConfig, ActiveLearningError, Dfa, GrammarAcceptorOracle,
    Oracle as ActiveLearningOracle, ParserAcceptancePredicate, ParserMembershipOracle,
    SamplingEquivalenceOracle, Symbol as ActiveSymbol, clean_structural_acceptance, learn_dfa,
    learn_grammar,
};
pub use inference::advisor::{
    AdviceDecision, AdviceDecisionKind, AdviceSource, ConceptNamingAdvisor, FallbackAdvisor,
    MdlMergeAdvisor, MergeAdvisor, MergeCandidate, MergeRequest, MergeScore, NameCandidate,
    NamingAdvisor, NamingRequest,
};
#[cfg(feature = "llm-assist")]
pub use inference::advisor::{LlmClient, LlmError, LlmMergeAdvisor, LlmNamingAdvisor};
pub use inference::cfg::{
    InferenceOptions, InferenceReport, InferenceResult, Oracle, PositiveOnlyOracle, infer_cfg,
    infer_cfg_with_advisors,
};
pub use inference::eval::{
    BenchmarkReport, EvalError, GOLDEN_CORPORA, GoldenCorpus, GrammarOracle, MembershipOracle,
    MetricScores, SampleConfig, ScoringMode, evaluate, mdl, run_corpus, run_named_corpus, sample,
    size_symbols,
};
pub use inference::lexical::{
    CharCategory, LexicalConfig, LexicalModel, Token, categorise, infer_lexical_classes,
};
pub use inference::minimize::{
    Mdl, MinimizeOptions, MinimizeReport, MinimizeResult, mdl_cost, minimize,
};
pub use inference::prior::{
    ByteSpan, Delimiter, LeafKind, PriorOptions, SeedNode, SeedTree, StructuralPrior,
    WhitespacePolicy, build_structural_prior,
};
pub use inference::semantic::{
    ConstraintAtom, ConstraintClause, ConstraintPattern, LengthUnit, NonTerminalRef,
    SemanticConstraint, SemanticInferenceConfig, default_pattern_catalog, evaluate_atom,
    evaluate_clause, evaluate_constraint, evaluate_probabilistic, mine_semantic_constraints,
};
pub use inference::sequitur::{Symbol, run_sequitur};
pub use inference::state_merging::{InferredAutomaton, MergeStrategy, Sample, infer_dfa};
pub use interchange::{
    GRAMMAR_COMMAND_USAGE, GRAMMAR_EXPORT_FORMATS, GRAMMAR_IMPORT_FORMATS,
    GRAMMAR_LOSSLESS_FORMATS, GrammarCommandOutput, GrammarEmitter, GrammarFileReader,
    GrammarImporter, GrammarLayout, GrammarLayoutDefinition, GrammarLayoutImplicit,
    GrammarLosslessError, GrammarSourceDefinition, GrammarSourceSplit, capture_grammar_layout,
    deserialize_grammar, emit_grammar_lossless, grammar_emitter, grammar_importer,
    import_grammar_lossless, parse_grammar_layout_links, parse_grammar_links,
    parse_links_expression, parse_native_grammar, percent_decode_links_text,
    percent_encode_links_text, render_declaration_links, render_grammar_layout_links,
    render_grammar_links, render_links_expression, render_native_expression, render_native_feature,
    render_native_grammar, render_rule_fields, render_rule_link, run_grammar_command,
    serialize_grammar, split_grammar_source,
};
pub use merge::{
    GRAMMAR_MERGE_METHOD, GrammarMergeAlternative, GrammarMergeAlternativeReason,
    GrammarMergeDecision, GrammarMergeDecisionKind, GrammarMergeError, GrammarMergeFailure,
    GrammarMergeFailureKind, GrammarMergeFailureReason, GrammarMergeNomination,
    GrammarMergeNominationBasis, GrammarMergeNominationOutcome, GrammarMergeOptions,
    GrammarMergeResult, GrammarMergeSource, GrammarRenameError, GrammarRenameErrorKind,
    MergedGrammarGroup, RenamedGrammar, RuleAlias, assert_merge_complete, assert_merge_shares,
    merge_grammars, normalized_rule_definition, rename_grammar_rule, restore_source_names,
    shared_rule_decisions,
};
pub use metadata::{GrammarFormat, GrammarKind, GrammarSourceName};
pub use reverse::{
    GrammarReverseConversion, GrammarReverseFailure, GrammarReverseFailureKind,
    GrammarReverseReport, GrammarReverseStage, GrammarReverseStatus,
    check_grammar_reverse_conversion,
};
pub use round_trip::{
    GRAMMAR_ROUND_TRIP_MARKER, GrammarEmitFn, GrammarImportFn, GrammarRoundTrip,
    GrammarRoundTripError, GrammarRoundTripFailure, GrammarRoundTripFailureKind,
    GrammarRoundTripReport, GrammarRoundTripStage, GrammarRoundTripStatus, accepts_text,
    canonical_rule_definition, check_grammar_round_trip, mutate_grammar_start_rule,
};
pub use runtime::{GrammarParser, register_grammar, with_grammar};
pub use surface::{
    GrammarSurfaceError, grammar_from_lino, grammar_to_lino, parse_grammar_surface,
    write_grammar_surface,
};
pub use translate::{
    GrammarTranslateError, grammar_concept_translation_rules, translate_grammar_surface,
};
pub use validate::{
    DiagnosticKind, GRAMMAR_DIAGNOSTIC_KINDS, GrammarDiagnostic, RuleSpan, Severity,
    display_grammar_expression, validate,
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
    /// A form of the grammar feature union (`docs/grammar/feature-union.md`).
    Feature(Box<FeatureExpr>),
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

    /// Builds a feature union expression.
    #[must_use]
    pub fn feature(feature: FeatureExpr) -> Self {
        Self::Feature(Box::new(feature))
    }

    /// Copies a feature form with every nested expression rewritten by `map`.
    pub(crate) fn rewrite_feature(
        feature: &FeatureExpr,
        mut map: impl FnMut(&Self) -> Self,
    ) -> Self {
        let mut copy = feature.clone();
        copy.map_expressions(&mut |expr| *expr = map(expr));
        Self::Feature(Box::new(copy))
    }

    /// The first feature union form in this expression, in source order.
    #[must_use]
    pub fn first_feature(&self) -> Option<&FeatureExpr> {
        match self {
            Self::Feature(feature) => Some(feature),
            Self::Choice { alternatives, .. } | Self::Sequence(alternatives) => {
                alternatives.iter().find_map(Self::first_feature)
            }
            Self::Optional(inner)
            | Self::ZeroOrMore(inner)
            | Self::OneOrMore(inner)
            | Self::And(inner)
            | Self::Not(inner)
            | Self::Repeat { expr: inner, .. }
            | Self::Capture { expr: inner, .. } => inner.first_feature(),
            Self::Empty
            | Self::Terminal(_)
            | Self::TerminalInsensitive(_)
            | Self::CharRange(_, _)
            | Self::CharClass { .. }
            | Self::AnyChar
            | Self::NonTerminal(_) => None,
        }
    }

    pub(crate) fn collect_nonterminals(&self, names: &mut BTreeSet<String>) {
        match self {
            Self::Feature(feature) => feature.collect_references(names),
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
            Self::Feature(feature) => {
                formatter.write_str(&interchange::render_native_feature(feature))
            }
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
    /// The names the rule has in the grammars it was merged from.
    pub source_names: Vec<GrammarSourceName>,
    /// Optional free-text documentation or comment.
    pub doc: Option<String>,
    /// Parameters, channel, modes and action of the feature union.
    pub attributes: RuleAttributes,
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
            source_names: Vec::new(),
            doc: None,
            attributes: RuleAttributes::default(),
        }
    }

    /// Returns this rule with feature union attributes.
    #[must_use]
    pub fn with_attributes(mut self, attributes: RuleAttributes) -> Self {
        self.attributes = attributes;
        self
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

    /// Returns this rule with the names it has in the grammars it was merged
    /// from.
    #[must_use]
    pub fn with_source_names(mut self, source_names: Vec<GrammarSourceName>) -> Self {
        self.source_names = source_names;
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

    /// The names the rule has in the grammars it was merged from.
    #[must_use]
    pub fn source_names(&self) -> &[GrammarSourceName] {
        &self.source_names
    }

    /// Rule documentation, when present.
    #[must_use]
    pub fn doc(&self) -> Option<&str> {
        self.doc.as_deref()
    }
}

/// Order-preserving grammar.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Grammar {
    rules: Vec<GrammarRule>,
    start: Option<String>,
    source_format: Option<GrammarFormat>,
    declarations: GrammarDeclarations,
    kinds: Vec<GrammarKind>,
}

impl Grammar {
    /// Builds an empty grammar.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            rules: Vec::new(),
            start: None,
            source_format: None,
            declarations: GrammarDeclarations {
                matching: None,
                settling: None,
                imports: Vec::new(),
                modes: Vec::new(),
                extras: Vec::new(),
                conflicts: Vec::new(),
                precedences: Vec::new(),
                macros: Vec::new(),
                scanners: Vec::new(),
            },
            kinds: Vec::new(),
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

    /// Returns this grammar with feature union declarations.
    #[must_use]
    pub fn with_declarations(mut self, declarations: GrammarDeclarations) -> Self {
        self.declarations = declarations;
        self
    }

    /// The feature union declarations.
    #[must_use]
    pub const fn declarations(&self) -> &GrammarDeclarations {
        &self.declarations
    }

    /// Replaces the feature union declarations.
    pub fn set_declarations(&mut self, declarations: GrammarDeclarations) {
        self.declarations = declarations;
    }

    /// The node kinds no rule defines that keep their source names.
    #[must_use]
    pub fn kinds(&self) -> &[GrammarKind] {
        &self.kinds
    }

    /// Replaces the node kinds no rule defines.
    pub fn set_kinds(&mut self, kinds: Vec<GrammarKind>) {
        self.kinds = kinds;
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
        for extra in &self.declarations.extras {
            extra.collect_nonterminals(&mut names);
        }
        for declared in &self.declarations.macros {
            declared.expression.collect_nonterminals(&mut names);
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
            .chain(
                self.declarations
                    .external_tokens()
                    .into_iter()
                    .map(str::to_owned),
            )
            .collect::<BTreeSet<_>>();
        self.referenced_nonterminals()
            .difference(&defined)
            .cloned()
            .collect()
    }
}

/// Copies the documentation of every rule of `source` onto the rule of
/// `target` that `rename` names it.
///
/// As the JavaScript `carryRuleDocs` does, a rule of `source` without
/// documentation, or one `target` lacks, changes nothing.
#[must_use]
pub fn carry_rule_docs(
    mut target: Grammar,
    source: &Grammar,
    rename: &dyn Fn(&str) -> String,
) -> Grammar {
    for rule in source.rules() {
        let Some(doc) = &rule.doc else {
            continue;
        };
        let name = rename(&rule.name);
        for carried in target
            .rules
            .iter_mut()
            .filter(|carried| carried.name == name)
        {
            carried.doc = Some(doc.clone());
        }
    }
    target
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
