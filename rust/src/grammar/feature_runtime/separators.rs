//! Separator text: a run of the grammar's extras that are no rule, as
//! `separatorText` in `js/src/grammar-runtime/executor.js` reads it.

use super::program::{Expr, Trivia};

/// Whether the text of `[start, end)` is separators alone, each the longest
/// match of an extra of no kind that ends within it. An extra the matcher
/// cannot read without the executor (a rule reference) matches nothing.
pub(super) fn separator_text(trivia: &[Trivia], bytes: &[u8], start: usize, end: usize) -> bool {
    if start >= end {
        return false;
    }
    let mut at = start;
    while at < end {
        let mut next = at;
        for item in trivia.iter().filter(|item| item.kind.is_none()) {
            if let Some(reach) = ends(&item.expression, bytes, at).into_iter().max()
                && reach > next
                && reach <= end
            {
                next = reach;
            }
        }
        if next == at {
            return false;
        }
        at = next;
    }
    true
}

/// The ends of the matches of the lexical `expr` at `position`.
fn ends(expr: &Expr, bytes: &[u8], position: usize) -> Vec<usize> {
    match expr {
        Expr::Empty => vec![position],
        Expr::Terminal { matcher, .. } => matcher
            .matches(bytes, position, bytes.len())
            .into_iter()
            .collect(),
        Expr::Seq(items) => items.iter().fold(vec![position], |starts, item| {
            let mut found = Vec::new();
            for start in starts {
                for reach in ends(item, bytes, start) {
                    if !found.contains(&reach) {
                        found.push(reach);
                    }
                }
            }
            found
        }),
        Expr::Choice { items, .. } | Expr::Longest(items) => {
            let mut found = Vec::new();
            for item in items {
                for reach in ends(item, bytes, position) {
                    if !found.contains(&reach) {
                        found.push(reach);
                    }
                }
            }
            found
        }
        Expr::Repeat { item, min, max } => {
            let mut found = Vec::new();
            let mut frontier = vec![position];
            let mut count = 0;
            while !frontier.is_empty() {
                if count >= *min {
                    for &reach in &frontier {
                        if !found.contains(&reach) {
                            found.push(reach);
                        }
                    }
                }
                if max.is_some_and(|max| count >= max) {
                    break;
                }
                let mut next = Vec::new();
                for start in frontier {
                    for reach in ends(item, bytes, start) {
                        // An iteration of no width ends the repetition.
                        if reach > start && !next.contains(&reach) {
                            next.push(reach);
                        }
                    }
                }
                frontier = next;
                count += 1;
            }
            found
        }
        Expr::And(item) => {
            if ends(item, bytes, position).is_empty() {
                Vec::new()
            } else {
                vec![position]
            }
        }
        Expr::Not(item) => {
            if ends(item, bytes, position).is_empty() {
                vec![position]
            } else {
                Vec::new()
            }
        }
        Expr::Capture { item, .. }
        | Expr::Alias { item, .. }
        | Expr::Precedence { item, .. }
        | Expr::DynamicPrecedence { item, .. }
        | Expr::LexicalPrecedence { item, .. }
        | Expr::Token(item)
        | Expr::ImmediateToken(item) => ends(item, bytes, position),
        _ => Vec::new(),
    }
}
