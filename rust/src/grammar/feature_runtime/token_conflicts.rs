//! The lexical conflict of two token leaves, as a tree-sitter lexer settles
//! it: `tokenRank`, `tokenConflict` and their helpers in
//! `js/src/grammar-runtime/executor.js`.

use std::cmp::Ordering;

use super::program::TokenRank;
use super::results::{TokenOrder, Tree, TreeType};

/// The rank of a token leaf, or None for another leaf or an unranked token.
/// A leaf matched under a lexical precedence ranks at that level, as the
/// token defined there (Rust's `//!` marker `!` outranks the comment text).
pub(super) fn token_rank(leaf: &Tree, tokens: TokenOrder<'_>) -> Option<TokenRank> {
    if leaf.ty != TreeType::Token {
        return None;
    }
    let rank = leaf.rank.or_else(|| {
        leaf.kind.as_ref().map_or_else(
            || {
                tokens
                    .bytes
                    .get(leaf.start..leaf.end)
                    .and_then(|text| tokens.ranks.literals.get(text))
                    .copied()
            },
            |kind| tokens.ranks.kinds.get(kind).copied(),
        )
    });
    let Some(priority) = leaf.priority else {
        return rank;
    };
    Some(TokenRank {
        priority,
        ..rank.unwrap_or(TokenRank {
            priority,
            specificity: 0,
            order: usize::MAX,
        })
    })
}

/// Two leaves that differ in end or kind: Greater when a lexer prefers `a`,
/// Less when it prefers `b`, Equal when it cannot tell. The ranks decide only
/// between two tokens at one offset; otherwise the longer leaf wins, but for
/// a token of separator text alone: a tree-sitter lexer that has lexed it,
/// its separators still going on, takes no transition of another token of
/// no higher precedence (`prefer_transition`), so Make's immediate blank
/// after `=` is a token of its own before the value's text, which could take
/// it.
pub(super) fn token_conflict(a: &Tree, b: &Tree, tokens: TokenOrder<'_>) -> Ordering {
    let ranks = if a.start == b.start {
        token_rank(a, tokens).zip(token_rank(b, tokens))
    } else {
        None
    };
    if let Some((first, second)) = ranks
        && first.priority != second.priority
    {
        return first.priority.cmp(&second.priority);
    }
    // Two ends of one token at one offset are two tokens (Make's blank and
    // text are both `unnamed_token`), as a token lexes its longest match; the
    // longer, of separators too, may be the same token gone on.
    if a.end != b.end && a.start == b.start && !(a.lexed.is_some() && a.lexed == b.lexed) {
        let (shorter, longer, sign) = if a.end < b.end {
            (a, b, Ordering::Greater)
        } else {
            (b, a, Ordering::Less)
        };
        if tokens.separator_text(shorter.start, shorter.end)
            && !tokens.separator_text(longer.start, longer.end)
        {
            return sign;
        }
    }
    if a.end != b.end {
        return a.end.cmp(&b.end);
    }
    match ranks {
        None => Ordering::Equal,
        Some((first, second)) if first == second => Ordering::Equal,
        Some((first, second)) if first.specificity != second.specificity => {
            first.specificity.cmp(&second.specificity)
        }
        Some((first, second)) => second.order.cmp(&first.order),
    }
}

/// Whether `leaf`, after separators `cover` takes, wins over it: a lexer
/// that goes on with a separator as the first of a valid token skips it no
/// more, so `leaf` starts there too (see `widen_tokens`), and wins at a
/// higher precedence (Make's `@` of a recipe line after `; `, over its shell
/// text). A cover of separator text alone the lexer completes first, and
/// takes no separator transition after it (see `token_conflict`): Make's
/// line breaks after `;` end the recipe's first line, before a `\t@` that
/// could go on.
pub(super) fn starts_at_separator(leaf: &Tree, cover: &Tree, tokens: TokenOrder<'_>) -> bool {
    if leaf.ty != TreeType::Token || cover.ty != TreeType::Token {
        return false;
    }
    if tokens.separator_text(cover.start, cover.end) {
        return false;
    }
    token_rank(leaf, tokens)
        .zip(token_rank(cover, tokens))
        .is_some_and(|(mine, theirs)| mine.priority > theirs.priority)
}

/// Whether two leaves of one span are one literal of one rank, whatever kind
/// each is aliased to.
pub(super) fn same_literal(a: &Tree, b: &Tree, tokens: TokenOrder<'_>) -> bool {
    if a.start != b.start || a.end != b.end || a.ty != TreeType::Token || b.ty != TreeType::Token {
        return false;
    }
    token_rank(a, tokens)
        .zip(token_rank(b, tokens))
        .is_some_and(|(first, second)| {
            first.specificity >= 2
                && first.priority == second.priority
                && first.specificity == second.specificity
        })
}

/// Whether two leaves of one alias name are tokens of other precedences or
/// specificities (Make's immediate blank after `=` and the text there, both
/// `unnamed_token`): a lexer tells them apart. Tokens that differ only in
/// order may be one token the grammar repeats.
pub(super) fn other_ranks(a: &Tree, b: &Tree) -> bool {
    a.rank.zip(b.rank).is_some_and(|(first, second)| {
        first.priority != second.priority || first.specificity != second.specificity
    })
}
