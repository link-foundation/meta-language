//! The FIRST and FOLLOW sets of a program's rules, tree-sitter's sets of the
//! tokens a rule can begin with and the tokens that can come after it (see
//! `Lead`).

use std::collections::{HashMap, HashSet};

use super::super::precedence::nullable;
use super::super::program::{Expr, Matcher, Rule, Target};
use super::Lead;
use crate::grammar::RuleKind;

/// The tokens each of the `rules` can begin with, by rule index, as
/// tree-sitter's FIRST sets (see `Lead`). It mirrors firstSets in
/// js/src/grammar-runtime/executor.js.
pub(super) fn first_sets(rules: &[Rule]) -> Vec<HashSet<Lead>> {
    let mut sets = vec![HashSet::new(); rules.len()];
    let mut changed = true;
    while changed {
        changed = false;
        for (index, rule) in rules.iter().enumerate() {
            if matches!(rule.kind, RuleKind::Token | RuleKind::Atomic) {
                continue;
            }
            let mut found = HashSet::new();
            first_of(&rule.expression, rules, &sets, &mut found);
            let size = sets[index].len();
            sets[index].extend(found);
            changed |= sets[index].len() != size;
        }
    }
    sets
}

/// Adds to `found` the tokens `expr` can begin with, by the FIRST sets of
/// the rules known so far (see `first_sets`). It mirrors firstOf in
/// js/src/grammar-runtime/executor.js.
pub(super) fn first_of(
    expr: &Expr,
    rules: &[Rule],
    sets: &[HashSet<Lead>],
    found: &mut HashSet<Lead>,
) {
    match expr {
        Expr::Terminal {
            matcher: Matcher::Literal(text),
            ..
        } => {
            found.insert(Lead::Literal(text.clone()));
        }
        Expr::Ref(Target::External(name)) => {
            found.insert(Lead::Ref(name.clone()));
        }
        Expr::Ref(Target::Rule(index)) => {
            if matches!(rules[*index].kind, RuleKind::Token | RuleKind::Atomic) {
                found.insert(Lead::Ref(rules[*index].name.clone()));
            } else {
                found.extend(sets[*index].iter().cloned());
            }
        }
        Expr::Seq(items) => {
            for item in items {
                first_of(item, rules, sets, found);
                if !nullable(item) {
                    break;
                }
            }
        }
        Expr::Choice { items, .. } => {
            for item in items {
                first_of(item, rules, sets, found);
            }
        }
        Expr::Alias { name, item } => {
            found.insert(Lead::Ref(name.clone()));
            first_of(item, rules, sets, found);
        }
        Expr::Token(item)
        | Expr::ImmediateToken(item)
        | Expr::LexicalPrecedence { item, .. }
        | Expr::Capture { item, .. }
        | Expr::Precedence { item, .. }
        | Expr::DynamicPrecedence { item, .. }
        | Expr::Repeat { item, .. } => first_of(item, rules, sets, found),
        _ => {}
    }
}

/// Adds to `found` the tokens the sequence `items` can begin with, by the
/// FIRST sets `sets` (see `first_of`).
pub(super) fn first_of_items(
    items: &[Expr],
    rules: &[Rule],
    sets: &[HashSet<Lead>],
    found: &mut HashSet<Lead>,
) {
    for item in items {
        first_of(item, rules, sets, found);
        if !nullable(item) {
            break;
        }
    }
}

/// The tokens that may follow each of the `rules`, by rule index, as the
/// FOLLOW sets of an LR parser's lookaheads, by the FIRST sets `first` (see
/// `first_sets`). It mirrors followSets in
/// js/src/grammar-runtime/executor.js.
pub(super) fn follow_sets(rules: &[Rule], first: &[HashSet<Lead>]) -> Vec<HashSet<Lead>> {
    let mut sets = vec![HashSet::new(); rules.len()];
    let mut changed = true;
    while changed {
        changed = false;
        for (index, rule) in rules.iter().enumerate() {
            if matches!(rule.kind, RuleKind::Token | RuleKind::Atomic) {
                continue;
            }
            let after = sets[index].clone();
            changed |= follow_walk(&rule.expression, &after, rules, first, &mut sets);
        }
    }
    sets
}

/// Adds `after`, the tokens that may follow `expr`, to the FOLLOW sets of
/// the rules within it (see `follow_sets`); true when any set grew.
pub(super) fn follow_walk(
    expr: &Expr,
    after: &HashSet<Lead>,
    rules: &[Rule],
    first: &[HashSet<Lead>],
    sets: &mut [HashSet<Lead>],
) -> bool {
    match expr {
        Expr::Ref(Target::Rule(index)) => {
            if matches!(rules[*index].kind, RuleKind::Token | RuleKind::Atomic) {
                return false;
            }
            let size = sets[*index].len();
            sets[*index].extend(after.iter().cloned());
            sets[*index].len() != size
        }
        Expr::Seq(items) => {
            let mut changed = false;
            let mut rest = after.clone();
            for item in items.iter().rev() {
                changed |= follow_walk(item, &rest, rules, first, sets);
                let mut own = HashSet::new();
                first_of(item, rules, first, &mut own);
                if nullable(item) {
                    own.extend(rest);
                }
                rest = own;
            }
            changed
        }
        Expr::Repeat {
            item, max: Some(1), ..
        }
        | Expr::Alias { item, .. }
        | Expr::Capture { item, .. }
        | Expr::Precedence { item, .. }
        | Expr::DynamicPrecedence { item, .. } => follow_walk(item, after, rules, first, sets),
        Expr::Repeat { item, .. } => {
            let mut next = HashSet::new();
            first_of(item, rules, first, &mut next);
            next.extend(after.iter().cloned());
            follow_walk(item, &next, rules, first, sets)
        }
        Expr::Choice { items, .. } => items.iter().fold(false, |changed, item| {
            follow_walk(item, after, rules, first, sets) | changed
        }),
        _ => false,
    }
}

/// The rules `expr` is a unit chain over (a choice, a precedence, a field or
/// an alias over a rule), by rule index (see `left_corner_follow`).
pub(super) fn units(expr: &Expr, found: &mut Vec<usize>) {
    match expr {
        Expr::Ref(Target::Rule(index)) => found.push(*index),
        Expr::Choice { items, .. } => {
            for item in items {
                units(item, found);
            }
        }
        Expr::Capture { item, .. }
        | Expr::Precedence { item, .. }
        | Expr::DynamicPrecedence { item, .. }
        | Expr::Alias { item, .. } => units(item, found),
        _ => {}
    }
}

/// The tokens that follow each of the `rules`, by rule index, where it is
/// the first part of a production, through its unit chains (a choice, a
/// precedence, a field or an alias over it): the lookaheads its reduction
/// has in every LR state it begins in, whatever the context (Rust's `-`,
/// `(` or `[` after `break_expression`, the left operand of a binary, call
/// or index expression), by the FIRST sets `first`. It mirrors
/// leftCornerFollow in js/src/grammar-runtime/executor.js.
pub(super) fn left_corner_follow(rules: &[Rule], first: &[HashSet<Lead>]) -> Vec<HashSet<Lead>> {
    let mut chains: HashMap<usize, HashSet<usize>> = HashMap::new();
    let mut chain = |start: usize| -> HashSet<usize> {
        chains
            .entry(start)
            .or_insert_with(|| {
                let mut found = HashSet::from([start]);
                let mut pending = vec![start];
                while let Some(index) = pending.pop() {
                    let rule = &rules[index];
                    if matches!(rule.kind, RuleKind::Token | RuleKind::Atomic) {
                        continue;
                    }
                    let mut next = Vec::new();
                    units(&rule.expression, &mut next);
                    for unit in next {
                        if found.insert(unit) {
                            pending.push(unit);
                        }
                    }
                }
                found
            })
            .clone()
    };
    let mut sets = vec![HashSet::new(); rules.len()];
    let mut pending: Vec<&Expr> = rules
        .iter()
        .filter(|rule| !matches!(rule.kind, RuleKind::Token | RuleKind::Atomic))
        .map(|rule| &rule.expression)
        .collect();
    while let Some(expr) = pending.pop() {
        match expr {
            Expr::Seq(items) => {
                for (index, item) in items.iter().enumerate() {
                    let mut rest = HashSet::new();
                    first_of_items(&items[index + 1..], rules, first, &mut rest);
                    let mut heads = Vec::new();
                    units(item, &mut heads);
                    for unit in heads {
                        for name in chain(unit) {
                            sets[name].extend(rest.iter().cloned());
                        }
                    }
                    if !nullable(item) {
                        break;
                    }
                }
                pending.extend(items);
            }
            Expr::Choice { items, .. } => pending.extend(items),
            Expr::Repeat { item, .. }
            | Expr::Capture { item, .. }
            | Expr::Precedence { item, .. }
            | Expr::DynamicPrecedence { item, .. }
            | Expr::Alias { item, .. } => pending.push(item),
            _ => {}
        }
    }
    sets
}
