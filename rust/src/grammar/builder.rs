//! The ergonomic expression builder of [`Grammar::expr`](super::Grammar::expr).

use std::fmt;

use super::{CharClassItem, Grammar, GrammarExpr, GrammarFormat, GrammarRule, RuleKind};

/// Largest integer a JavaScript number holds exactly (`Number.MAX_SAFE_INTEGER`).
const MAX_SAFE_INTEGER: u64 = (1 << 53) - 1;

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

/// Flattens nested sequences and drops empty items, as the JavaScript
/// `sequence` builder does: no items build [`GrammarExpr::Empty`] and one item
/// builds itself.
#[must_use]
pub fn sequence(items: Vec<GrammarExpr>) -> GrammarExpr {
    let mut flattened = Vec::with_capacity(items.len());
    for item in items {
        match item {
            GrammarExpr::Empty => {}
            GrammarExpr::Sequence(nested) => flattened.extend(nested),
            item => flattened.push(item),
        }
    }
    match flattened.len() {
        0 => GrammarExpr::Empty,
        1 => flattened.remove(0),
        _ => GrammarExpr::Sequence(flattened),
    }
}

/// Flattens nested choices of the same ordering, as the JavaScript `choice`
/// builder does: no alternatives build [`GrammarExpr::Empty`] and one
/// alternative builds itself.
#[must_use]
pub fn choice(alternatives: Vec<GrammarExpr>, ordered: bool) -> GrammarExpr {
    let mut flattened = Vec::with_capacity(alternatives.len());
    for alternative in alternatives {
        match alternative {
            GrammarExpr::Choice {
                ordered: nested_ordered,
                alternatives: nested,
            } if nested_ordered == ordered => flattened.extend(nested),
            alternative => flattened.push(alternative),
        }
    }
    match flattened.len() {
        0 => GrammarExpr::Empty,
        1 => flattened.remove(0),
        _ => GrammarExpr::Choice {
            ordered,
            alternatives: flattened,
        },
    }
}

/// Counted repetition bounds [`canonical_repeat`] rejects: an upper bound
/// below the lower one, or a bound past the JavaScript safe-integer range.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RepetitionBoundsError {
    /// The lower bound.
    pub min: u64,
    /// The upper bound, `None` when unbounded.
    pub max: Option<u64>,
}

impl fmt::Display for RepetitionBoundsError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let max = self.max.map_or_else(String::new, |max| max.to_string());
        write!(formatter, "invalid repetition bounds {}..{max}", self.min)
    }
}

impl std::error::Error for RepetitionBoundsError {}

/// Lowers counted repetition to its most specific expression variant.
///
/// As the JavaScript `canonicalRepeat` builder does, `0..` is zero-or-more,
/// `1..` is one-or-more, `0..1` is optional, and every other range is a
/// counted repeat.
///
/// # Errors
///
/// Returns [`RepetitionBoundsError`] when `max` is below `min` or either
/// bound exceeds the JavaScript safe-integer range.
pub fn canonical_repeat(
    expr: GrammarExpr,
    min: u64,
    max: Option<u64>,
) -> Result<GrammarExpr, RepetitionBoundsError> {
    let invalid = RepetitionBoundsError { min, max };
    if min > MAX_SAFE_INTEGER || max.is_some_and(|max| max > MAX_SAFE_INTEGER || max < min) {
        return Err(invalid);
    }
    let bound = |value: u64| usize::try_from(value).map_err(|_| invalid);
    Ok(match (min, max) {
        (0, None) => GrammarExpr::zero_or_more(expr),
        (1, None) => GrammarExpr::one_or_more(expr),
        (0, Some(1)) => GrammarExpr::optional(expr),
        (min, max) => GrammarExpr::repeat(expr, bound(min)?, max.map(bound).transpose()?),
    })
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
