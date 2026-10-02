//! The ergonomic expression builder of [`Grammar::expr`](super::Grammar::expr).

use super::{CharClassItem, GrammarExpr};

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
