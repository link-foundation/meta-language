use std::collections::{BTreeMap, BTreeSet};

use crate::grammar::{Grammar, GrammarExpr, GrammarRule, RuleKind};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Candidate {
    Merge { winner: usize, loser: usize },
    Inline { rule: usize },
    Factor { rule: usize },
    Prune,
}

impl Candidate {
    pub(super) const fn kind(self) -> CandidateKind {
        match self {
            Self::Merge { .. } => CandidateKind::Merge,
            Self::Inline { .. } => CandidateKind::Inline,
            Self::Factor { .. } => CandidateKind::Factor,
            Self::Prune => CandidateKind::Prune,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum CandidateKind {
    Merge,
    Inline,
    Factor,
    Prune,
}

pub(super) fn enumerate_candidates(grammar: &Grammar) -> Vec<Candidate> {
    let mut candidates = Vec::new();
    let rules = grammar.rules();

    for winner in 0..rules.len() {
        for loser in winner.saturating_add(1)..rules.len() {
            if can_merge(grammar, winner, loser) {
                candidates.push(Candidate::Merge { winner, loser });
            }
        }

        if can_inline(grammar, winner) {
            candidates.push(Candidate::Inline { rule: winner });
        }

        if factor_expr(rules[winner].expr()) != canonicalize_expr(rules[winner].expr()) {
            candidates.push(Candidate::Factor { rule: winner });
        }
    }

    if prune_unreachable(grammar) != *grammar {
        candidates.push(Candidate::Prune);
    }

    candidates
}

pub(super) fn apply_candidate(grammar: &Grammar, candidate: Candidate) -> Grammar {
    match candidate {
        Candidate::Merge { winner, loser } => merge_rules(grammar, winner, loser),
        Candidate::Inline { rule } => inline_rule(grammar, rule),
        Candidate::Factor { rule } => factor_rule(grammar, rule),
        Candidate::Prune => prune_unreachable(grammar),
    }
}

fn can_merge(grammar: &Grammar, winner: usize, loser: usize) -> bool {
    let rules = grammar.rules();
    let Some(winner_rule) = rules.get(winner) else {
        return false;
    };
    let Some(loser_rule) = rules.get(loser) else {
        return false;
    };

    if grammar.start() == Some(loser_rule.name()) {
        return false;
    }
    if winner_rule.kind() != loser_rule.kind()
        || winner_rule.concept() != loser_rule.concept()
        || winner_rule.doc() != loser_rule.doc()
    {
        return false;
    }
    if expr_references(winner_rule.expr(), loser_rule.name())
        || expr_references(loser_rule.expr(), winner_rule.name())
        || expr_references(winner_rule.expr(), winner_rule.name())
        || expr_references(loser_rule.expr(), loser_rule.name())
    {
        return false;
    }

    expr_shape_compatible(winner_rule.expr(), loser_rule.expr())
}

fn can_inline(grammar: &Grammar, rule_index: usize) -> bool {
    let Some(rule) = grammar.rules().get(rule_index) else {
        return false;
    };
    if grammar.start() == Some(rule.name())
        || rule.kind() != RuleKind::Normal
        || rule.concept().is_some()
        || rule.doc().is_some()
        || expr_references(rule.expr(), rule.name())
    {
        return false;
    }

    reference_counts(grammar)
        .get(rule.name())
        .copied()
        .unwrap_or_default()
        == 1
}

fn merge_rules(grammar: &Grammar, winner_index: usize, loser_index: usize) -> Grammar {
    let rules = grammar.rules();
    let Some(winner_rule) = rules.get(winner_index) else {
        return grammar.clone();
    };
    let Some(loser_rule) = rules.get(loser_index) else {
        return grammar.clone();
    };

    let winner_name = winner_rule.name().to_string();
    let loser_name = loser_rule.name().to_string();
    let replacement = GrammarExpr::non_terminal(&winner_name);
    let merged_expr = merge_exprs(winner_rule.expr(), loser_rule.expr());

    let next_rules = rules
        .iter()
        .enumerate()
        .filter_map(|(index, rule)| {
            if index == loser_index {
                return None;
            }

            let mut next = rule.clone();
            if index == winner_index {
                next.expr = merged_expr.clone();
            }
            next.expr = rewrite_nonterminal_refs(&next.expr, &loser_name, &replacement);
            Some(next)
        })
        .collect::<Vec<_>>();

    canonicalize_grammar(&rebuild_like(grammar, next_rules))
}

fn inline_rule(grammar: &Grammar, rule_index: usize) -> Grammar {
    let Some(target_rule) = grammar.rules().get(rule_index) else {
        return grammar.clone();
    };
    let target_name = target_rule.name().to_string();
    let replacement = target_rule.expr().clone();

    let next_rules = grammar
        .rules()
        .iter()
        .enumerate()
        .filter_map(|(index, rule)| {
            if index == rule_index {
                return None;
            }

            let mut next = rule.clone();
            next.expr = rewrite_nonterminal_refs(&next.expr, &target_name, &replacement);
            Some(next)
        })
        .collect::<Vec<_>>();

    canonicalize_grammar(&rebuild_like(grammar, next_rules))
}

fn factor_rule(grammar: &Grammar, rule_index: usize) -> Grammar {
    let mut next_rules = grammar.rules().to_vec();
    let Some(rule) = next_rules.get_mut(rule_index) else {
        return grammar.clone();
    };
    rule.expr = factor_expr(rule.expr());
    canonicalize_grammar(&rebuild_like(grammar, next_rules))
}

fn prune_unreachable(grammar: &Grammar) -> Grammar {
    let reachable = reachable_rule_names(grammar);
    if reachable.is_empty() {
        return grammar.clone();
    }

    let next_rules = grammar
        .rules()
        .iter()
        .filter(|rule| reachable.contains(rule.name()))
        .cloned()
        .collect::<Vec<_>>();
    rebuild_like(grammar, next_rules)
}

fn reachable_rule_names(grammar: &Grammar) -> BTreeSet<String> {
    let Some(start) = grammar.start_rule() else {
        return BTreeSet::new();
    };

    let mut reachable = BTreeSet::new();
    let mut stack = vec![start.name().to_string()];
    while let Some(name) = stack.pop() {
        if !reachable.insert(name.clone()) {
            continue;
        }
        let Some(rule) = grammar.rule(&name) else {
            continue;
        };
        for referenced in expr_nonterminals(rule.expr()) {
            if !reachable.contains(&referenced) {
                stack.push(referenced);
            }
        }
    }
    reachable
}

fn rebuild_like(source: &Grammar, rules: Vec<GrammarRule>) -> Grammar {
    let mut next = Grammar::new();
    if let Some(source_format) = source.source_format() {
        next.set_source_format(source_format);
    }
    for rule in rules {
        next.add_rule(rule);
    }
    if let Some(start) = source.start() {
        next.set_start(start);
    }
    next
}

fn canonicalize_grammar(grammar: &Grammar) -> Grammar {
    let rules = grammar
        .rules()
        .iter()
        .cloned()
        .map(|mut rule| {
            rule.expr = canonicalize_expr(rule.expr());
            rule
        })
        .collect::<Vec<_>>();
    rebuild_like(grammar, rules)
}

fn canonicalize_expr(expr: &GrammarExpr) -> GrammarExpr {
    match expr {
        GrammarExpr::Choice {
            ordered,
            alternatives,
        } => choice_expr(
            *ordered,
            alternatives.iter().map(canonicalize_expr).collect(),
        ),
        GrammarExpr::Sequence(items) => seq_expr(items.iter().map(canonicalize_expr).collect()),
        GrammarExpr::Optional(inner) => {
            let inner = canonicalize_expr(inner);
            if inner == GrammarExpr::Empty {
                GrammarExpr::Empty
            } else {
                GrammarExpr::optional(inner)
            }
        }
        GrammarExpr::ZeroOrMore(inner) => {
            let inner = canonicalize_expr(inner);
            if inner == GrammarExpr::Empty {
                GrammarExpr::Empty
            } else {
                GrammarExpr::zero_or_more(inner)
            }
        }
        GrammarExpr::OneOrMore(inner) => {
            let inner = canonicalize_expr(inner);
            if inner == GrammarExpr::Empty {
                GrammarExpr::Empty
            } else {
                GrammarExpr::one_or_more(inner)
            }
        }
        GrammarExpr::Repeat { expr, min, max } => {
            let expr = canonicalize_expr(expr);
            if expr == GrammarExpr::Empty {
                GrammarExpr::Empty
            } else {
                GrammarExpr::repeat(expr, *min, *max)
            }
        }
        GrammarExpr::And(inner) => GrammarExpr::and(canonicalize_expr(inner)),
        GrammarExpr::Not(inner) => GrammarExpr::not(canonicalize_expr(inner)),
        GrammarExpr::Capture { label, expr } => {
            capture_with_label(label.as_deref(), canonicalize_expr(expr))
        }
        GrammarExpr::Empty
        | GrammarExpr::Terminal(_)
        | GrammarExpr::TerminalInsensitive(_)
        | GrammarExpr::CharRange(_, _)
        | GrammarExpr::CharClass { .. }
        | GrammarExpr::AnyChar
        | GrammarExpr::NonTerminal(_) => expr.clone(),
    }
}

fn factor_expr(expr: &GrammarExpr) -> GrammarExpr {
    match canonicalize_expr(expr) {
        GrammarExpr::Choice {
            ordered,
            alternatives,
        } => {
            let factored_alternatives = alternatives
                .iter()
                .map(factor_expr)
                .collect::<Vec<GrammarExpr>>();
            let choice = choice_expr(ordered, factored_alternatives);
            let GrammarExpr::Choice {
                ordered,
                alternatives,
            } = choice
            else {
                return choice;
            };

            factor_common_prefix(ordered, &alternatives)
                .or_else(|| factor_common_suffix(ordered, &alternatives))
                .unwrap_or_else(|| GrammarExpr::choice(ordered, alternatives))
        }
        GrammarExpr::Sequence(items) => seq_expr(items.iter().map(factor_expr).collect()),
        GrammarExpr::Optional(inner) => GrammarExpr::optional(factor_expr(&inner)),
        GrammarExpr::ZeroOrMore(inner) => GrammarExpr::zero_or_more(factor_expr(&inner)),
        GrammarExpr::OneOrMore(inner) => GrammarExpr::one_or_more(factor_expr(&inner)),
        GrammarExpr::Repeat { expr, min, max } => GrammarExpr::repeat(factor_expr(&expr), min, max),
        GrammarExpr::And(inner) => GrammarExpr::and(factor_expr(&inner)),
        GrammarExpr::Not(inner) => GrammarExpr::not(factor_expr(&inner)),
        GrammarExpr::Capture { label, expr } => {
            capture_with_label(label.as_deref(), factor_expr(&expr))
        }
        other => other,
    }
}

fn factor_common_prefix(ordered: bool, alternatives: &[GrammarExpr]) -> Option<GrammarExpr> {
    if alternatives.len() < 2 {
        return None;
    }
    let sequences = alternatives.iter().map(sequence_items).collect::<Vec<_>>();
    let mut prefix = Vec::new();

    loop {
        let position = prefix.len();
        let first = sequences.first()?.get(position)?.clone();
        if sequences
            .iter()
            .all(|sequence| sequence.get(position) == Some(&first))
        {
            prefix.push(first);
        } else {
            break;
        }
    }

    if prefix.is_empty() {
        return None;
    }

    let suffixes = sequences
        .into_iter()
        .map(|sequence| seq_expr(sequence[prefix.len()..].to_vec()))
        .collect::<Vec<_>>();
    let mut items = prefix;
    items.push(choice_expr(ordered, suffixes));
    Some(canonicalize_expr(&seq_expr(items)))
}

fn factor_common_suffix(ordered: bool, alternatives: &[GrammarExpr]) -> Option<GrammarExpr> {
    if alternatives.len() < 2 {
        return None;
    }
    let sequences = alternatives.iter().map(sequence_items).collect::<Vec<_>>();
    let mut suffix = Vec::new();

    loop {
        let position = suffix.len();
        let first_sequence = sequences.first()?;
        if position >= first_sequence.len() {
            break;
        }
        let first = first_sequence[first_sequence.len() - position - 1].clone();
        if sequences.iter().all(|sequence| {
            position < sequence.len() && sequence[sequence.len() - position - 1] == first
        }) {
            suffix.push(first);
        } else {
            break;
        }
    }

    if suffix.is_empty() {
        return None;
    }

    let suffix_len = suffix.len();
    let prefixes = sequences
        .into_iter()
        .map(|sequence| seq_expr(sequence[..sequence.len() - suffix_len].to_vec()))
        .collect::<Vec<_>>();
    suffix.reverse();

    let mut items = vec![choice_expr(ordered, prefixes)];
    items.extend(suffix);
    Some(canonicalize_expr(&seq_expr(items)))
}

fn sequence_items(expr: &GrammarExpr) -> Vec<GrammarExpr> {
    match expr {
        GrammarExpr::Empty => Vec::new(),
        GrammarExpr::Sequence(items) => items.clone(),
        other => vec![other.clone()],
    }
}

fn choice_expr(ordered: bool, alternatives: Vec<GrammarExpr>) -> GrammarExpr {
    let mut unique = BTreeMap::<String, GrammarExpr>::new();
    for alternative in alternatives {
        match alternative {
            GrammarExpr::Choice {
                ordered: inner_ordered,
                alternatives,
            } if inner_ordered == ordered => {
                for alternative in alternatives {
                    unique.entry(expr_key(&alternative)).or_insert(alternative);
                }
            }
            other => {
                unique.entry(expr_key(&other)).or_insert(other);
            }
        }
    }

    match unique.len() {
        0 => GrammarExpr::Empty,
        1 => unique
            .into_values()
            .next()
            .expect("one alternative must be present"),
        _ => GrammarExpr::choice(ordered, unique.into_values()),
    }
}

fn seq_expr(items: Vec<GrammarExpr>) -> GrammarExpr {
    let mut flattened = Vec::new();
    for item in items {
        match item {
            GrammarExpr::Empty => {}
            GrammarExpr::Sequence(items) => flattened.extend(items),
            other => flattened.push(other),
        }
    }

    match flattened.len() {
        0 => GrammarExpr::Empty,
        1 => flattened
            .into_iter()
            .next()
            .expect("one sequence item must be present"),
        _ => GrammarExpr::sequence(flattened),
    }
}

fn rewrite_nonterminal_refs(
    expr: &GrammarExpr,
    target: &str,
    replacement: &GrammarExpr,
) -> GrammarExpr {
    match expr {
        GrammarExpr::NonTerminal(name) if name == target => replacement.clone(),
        GrammarExpr::Choice {
            ordered,
            alternatives,
        } => GrammarExpr::choice(
            *ordered,
            alternatives
                .iter()
                .map(|alternative| rewrite_nonterminal_refs(alternative, target, replacement)),
        ),
        GrammarExpr::Sequence(items) => GrammarExpr::sequence(
            items
                .iter()
                .map(|item| rewrite_nonterminal_refs(item, target, replacement)),
        ),
        GrammarExpr::Optional(inner) => {
            GrammarExpr::optional(rewrite_nonterminal_refs(inner, target, replacement))
        }
        GrammarExpr::ZeroOrMore(inner) => {
            GrammarExpr::zero_or_more(rewrite_nonterminal_refs(inner, target, replacement))
        }
        GrammarExpr::OneOrMore(inner) => {
            GrammarExpr::one_or_more(rewrite_nonterminal_refs(inner, target, replacement))
        }
        GrammarExpr::Repeat { expr, min, max } => GrammarExpr::repeat(
            rewrite_nonterminal_refs(expr, target, replacement),
            *min,
            *max,
        ),
        GrammarExpr::And(inner) => {
            GrammarExpr::and(rewrite_nonterminal_refs(inner, target, replacement))
        }
        GrammarExpr::Not(inner) => {
            GrammarExpr::not(rewrite_nonterminal_refs(inner, target, replacement))
        }
        GrammarExpr::Capture { label, expr } => capture_with_label(
            label.as_deref(),
            rewrite_nonterminal_refs(expr, target, replacement),
        ),
        GrammarExpr::Empty
        | GrammarExpr::Terminal(_)
        | GrammarExpr::TerminalInsensitive(_)
        | GrammarExpr::CharRange(_, _)
        | GrammarExpr::CharClass { .. }
        | GrammarExpr::AnyChar
        | GrammarExpr::NonTerminal(_) => expr.clone(),
    }
}

fn merge_exprs(left: &GrammarExpr, right: &GrammarExpr) -> GrammarExpr {
    if left == right {
        return left.clone();
    }

    match (left, right) {
        (GrammarExpr::Sequence(left_items), GrammarExpr::Sequence(right_items))
            if left_items.len() == right_items.len() =>
        {
            seq_expr(
                left_items
                    .iter()
                    .zip(right_items)
                    .map(|(left, right)| merge_exprs(left, right))
                    .collect(),
            )
        }
        (GrammarExpr::Optional(left_inner), GrammarExpr::Optional(right_inner)) => {
            GrammarExpr::optional(merge_exprs(left_inner, right_inner))
        }
        (GrammarExpr::ZeroOrMore(left_inner), GrammarExpr::ZeroOrMore(right_inner)) => {
            GrammarExpr::zero_or_more(merge_exprs(left_inner, right_inner))
        }
        (GrammarExpr::OneOrMore(left_inner), GrammarExpr::OneOrMore(right_inner)) => {
            GrammarExpr::one_or_more(merge_exprs(left_inner, right_inner))
        }
        (
            GrammarExpr::Repeat {
                expr: left_inner,
                min,
                max,
            },
            GrammarExpr::Repeat {
                expr: right_inner,
                min: right_min,
                max: right_max,
            },
        ) if min == right_min && max == right_max => {
            GrammarExpr::repeat(merge_exprs(left_inner, right_inner), *min, *max)
        }
        (
            GrammarExpr::Capture {
                label,
                expr: left_inner,
            },
            GrammarExpr::Capture {
                label: right_label,
                expr: right_inner,
            },
        ) if label == right_label => {
            capture_with_label(label.as_deref(), merge_exprs(left_inner, right_inner))
        }
        _ => choice_expr(false, vec![left.clone(), right.clone()]),
    }
}

#[allow(clippy::option_if_let_else)]
fn capture_with_label(label: Option<&str>, expr: GrammarExpr) -> GrammarExpr {
    if let Some(label) = label {
        GrammarExpr::capture(label, expr)
    } else {
        GrammarExpr::capture_unlabeled(expr)
    }
}

fn expr_shape_compatible(left: &GrammarExpr, right: &GrammarExpr) -> bool {
    match (left, right) {
        (GrammarExpr::Empty, GrammarExpr::Empty)
        | (GrammarExpr::Terminal(_), GrammarExpr::Terminal(_))
        | (GrammarExpr::TerminalInsensitive(_), GrammarExpr::TerminalInsensitive(_))
        | (GrammarExpr::CharRange(_, _), GrammarExpr::CharRange(_, _))
        | (GrammarExpr::AnyChar, GrammarExpr::AnyChar)
        | (GrammarExpr::NonTerminal(_), GrammarExpr::NonTerminal(_)) => true,
        (
            GrammarExpr::CharClass {
                negated: left_negated,
                items: left_items,
            },
            GrammarExpr::CharClass {
                negated: right_negated,
                items: right_items,
            },
        ) => left_negated == right_negated && left_items.len() == right_items.len(),
        (
            GrammarExpr::Choice {
                ordered: left_ordered,
                alternatives: left_alternatives,
            },
            GrammarExpr::Choice {
                ordered: right_ordered,
                alternatives: right_alternatives,
            },
        ) => {
            left_ordered == right_ordered
                && left_alternatives.len() == right_alternatives.len()
                && left_alternatives
                    .iter()
                    .zip(right_alternatives)
                    .all(|(left, right)| expr_shape_compatible(left, right))
        }
        (GrammarExpr::Sequence(left_items), GrammarExpr::Sequence(right_items)) => {
            left_items.len() == right_items.len()
                && left_items
                    .iter()
                    .zip(right_items)
                    .all(|(left, right)| expr_shape_compatible(left, right))
        }
        (GrammarExpr::Optional(left), GrammarExpr::Optional(right))
        | (GrammarExpr::ZeroOrMore(left), GrammarExpr::ZeroOrMore(right))
        | (GrammarExpr::OneOrMore(left), GrammarExpr::OneOrMore(right))
        | (GrammarExpr::And(left), GrammarExpr::And(right))
        | (GrammarExpr::Not(left), GrammarExpr::Not(right)) => expr_shape_compatible(left, right),
        (
            GrammarExpr::Repeat {
                expr: left,
                min: left_min,
                max: left_max,
            },
            GrammarExpr::Repeat {
                expr: right,
                min: right_min,
                max: right_max,
            },
        ) => left_min == right_min && left_max == right_max && expr_shape_compatible(left, right),
        (
            GrammarExpr::Capture {
                label: left_label,
                expr: left,
            },
            GrammarExpr::Capture {
                label: right_label,
                expr: right,
            },
        ) => left_label == right_label && expr_shape_compatible(left, right),
        _ => false,
    }
}

fn expr_references(expr: &GrammarExpr, expected: &str) -> bool {
    match expr {
        GrammarExpr::NonTerminal(name) => name == expected,
        GrammarExpr::Choice { alternatives, .. } => alternatives
            .iter()
            .any(|alternative| expr_references(alternative, expected)),
        GrammarExpr::Sequence(items) => items.iter().any(|item| expr_references(item, expected)),
        GrammarExpr::Optional(inner)
        | GrammarExpr::ZeroOrMore(inner)
        | GrammarExpr::OneOrMore(inner)
        | GrammarExpr::And(inner)
        | GrammarExpr::Not(inner)
        | GrammarExpr::Repeat { expr: inner, .. }
        | GrammarExpr::Capture { expr: inner, .. } => expr_references(inner, expected),
        GrammarExpr::Empty
        | GrammarExpr::Terminal(_)
        | GrammarExpr::TerminalInsensitive(_)
        | GrammarExpr::CharRange(_, _)
        | GrammarExpr::CharClass { .. }
        | GrammarExpr::AnyChar => false,
    }
}

fn expr_nonterminals(expr: &GrammarExpr) -> BTreeSet<String> {
    let mut names = BTreeSet::new();
    collect_nonterminals(expr, &mut names);
    names
}

fn collect_nonterminals(expr: &GrammarExpr, names: &mut BTreeSet<String>) {
    match expr {
        GrammarExpr::NonTerminal(name) => {
            names.insert(name.clone());
        }
        GrammarExpr::Choice { alternatives, .. } => {
            for alternative in alternatives {
                collect_nonterminals(alternative, names);
            }
        }
        GrammarExpr::Sequence(items) => {
            for item in items {
                collect_nonterminals(item, names);
            }
        }
        GrammarExpr::Optional(inner)
        | GrammarExpr::ZeroOrMore(inner)
        | GrammarExpr::OneOrMore(inner)
        | GrammarExpr::And(inner)
        | GrammarExpr::Not(inner)
        | GrammarExpr::Repeat { expr: inner, .. }
        | GrammarExpr::Capture { expr: inner, .. } => collect_nonterminals(inner, names),
        GrammarExpr::Empty
        | GrammarExpr::Terminal(_)
        | GrammarExpr::TerminalInsensitive(_)
        | GrammarExpr::CharRange(_, _)
        | GrammarExpr::CharClass { .. }
        | GrammarExpr::AnyChar => {}
    }
}

fn reference_counts(grammar: &Grammar) -> BTreeMap<String, usize> {
    let mut counts = BTreeMap::new();
    for rule in grammar.rules() {
        count_references(rule.expr(), &mut counts);
    }
    counts
}

fn count_references(expr: &GrammarExpr, counts: &mut BTreeMap<String, usize>) {
    match expr {
        GrammarExpr::NonTerminal(name) => {
            *counts.entry(name.clone()).or_default() += 1;
        }
        GrammarExpr::Choice { alternatives, .. } => {
            for alternative in alternatives {
                count_references(alternative, counts);
            }
        }
        GrammarExpr::Sequence(items) => {
            for item in items {
                count_references(item, counts);
            }
        }
        GrammarExpr::Optional(inner)
        | GrammarExpr::ZeroOrMore(inner)
        | GrammarExpr::OneOrMore(inner)
        | GrammarExpr::And(inner)
        | GrammarExpr::Not(inner)
        | GrammarExpr::Repeat { expr: inner, .. }
        | GrammarExpr::Capture { expr: inner, .. } => count_references(inner, counts),
        GrammarExpr::Empty
        | GrammarExpr::Terminal(_)
        | GrammarExpr::TerminalInsensitive(_)
        | GrammarExpr::CharRange(_, _)
        | GrammarExpr::CharClass { .. }
        | GrammarExpr::AnyChar => {}
    }
}

fn expr_key(expr: &GrammarExpr) -> String {
    format!("{expr:?}")
}
