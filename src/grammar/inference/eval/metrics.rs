use std::collections::BTreeSet;

use crate::grammar::{CharClassItem, Grammar, GrammarExpr};

use super::GrammarOracle;

const OPERATOR_SYMBOLS: usize = 16;

pub(super) fn size_symbols(grammar: &Grammar) -> usize {
    grammar
        .rules()
        .iter()
        .map(|rule| 1usize.saturating_add(expr_symbol_count(rule.expr())))
        .sum()
}

pub(super) fn mdl(grammar: &Grammar, data: &[&str]) -> f64 {
    grammar_description_bits(grammar) + data_description_bits(grammar, data)
}

pub(super) fn ratio(numerator: usize, denominator: usize) -> f64 {
    debug_assert!(denominator > 0);
    usize_to_f64(numerator) / usize_to_f64(denominator)
}

fn expr_symbol_count(expr: &GrammarExpr) -> usize {
    match expr {
        GrammarExpr::Empty
        | GrammarExpr::Terminal(_)
        | GrammarExpr::TerminalInsensitive(_)
        | GrammarExpr::CharRange(_, _)
        | GrammarExpr::AnyChar
        | GrammarExpr::NonTerminal(_) => 1,
        GrammarExpr::CharClass { items, .. } => 1usize.saturating_add(items.len()),
        GrammarExpr::Choice { alternatives, .. } => {
            1usize.saturating_add(alternatives.iter().map(expr_symbol_count).sum::<usize>())
        }
        GrammarExpr::Sequence(items) => {
            1usize.saturating_add(items.iter().map(expr_symbol_count).sum::<usize>())
        }
        GrammarExpr::Optional(expr)
        | GrammarExpr::ZeroOrMore(expr)
        | GrammarExpr::OneOrMore(expr)
        | GrammarExpr::And(expr)
        | GrammarExpr::Not(expr)
        | GrammarExpr::Capture { expr, .. }
        | GrammarExpr::Repeat { expr, .. } => 1usize.saturating_add(expr_symbol_count(expr)),
    }
}

fn grammar_description_bits(grammar: &Grammar) -> f64 {
    let alphabet = distinct_symbol_count(grammar).max(2);
    let bits_per_symbol = ceil_log2(alphabet).max(1);
    usize_to_f64(size_symbols(grammar).saturating_mul(bits_per_symbol))
}

fn data_description_bits(grammar: &Grammar, data: &[&str]) -> f64 {
    let oracle = GrammarOracle(grammar);
    data.iter()
        .map(|text| {
            if oracle.accepts(text) {
                usize_to_f64(text.chars().count().saturating_add(1))
            } else {
                usize_to_f64(text.len().saturating_mul(8).saturating_add(64))
            }
        })
        .sum()
}

fn distinct_symbol_count(grammar: &Grammar) -> usize {
    let mut symbols = BTreeSet::new();
    for operator in 0..OPERATOR_SYMBOLS {
        symbols.insert(format!("op:{operator}"));
    }
    for rule in grammar.rules() {
        symbols.insert(format!("rule:{}", rule.name()));
        collect_expr_symbols(rule.expr(), &mut symbols);
    }
    symbols.len()
}

fn collect_expr_symbols(expr: &GrammarExpr, symbols: &mut BTreeSet<String>) {
    match expr {
        GrammarExpr::Empty => {
            symbols.insert("empty".to_string());
        }
        GrammarExpr::Terminal(value) => {
            symbols.insert(format!("terminal:{value}"));
        }
        GrammarExpr::TerminalInsensitive(value) => {
            symbols.insert(format!("terminal-insensitive:{value}"));
        }
        GrammarExpr::CharRange(start, end) => {
            symbols.insert(format!("range:{start}-{end}"));
        }
        GrammarExpr::CharClass { negated, items } => {
            symbols.insert(format!("class:{negated}"));
            for item in items {
                match item {
                    CharClassItem::Char(value) => {
                        symbols.insert(format!("char:{value}"));
                    }
                    CharClassItem::Range(start, end) => {
                        symbols.insert(format!("class-range:{start}-{end}"));
                    }
                }
            }
        }
        GrammarExpr::AnyChar => {
            symbols.insert("any".to_string());
        }
        GrammarExpr::NonTerminal(name) => {
            symbols.insert(format!("nonterminal:{name}"));
        }
        GrammarExpr::Choice { alternatives, .. } => {
            for alternative in alternatives {
                collect_expr_symbols(alternative, symbols);
            }
        }
        GrammarExpr::Sequence(items) => {
            for item in items {
                collect_expr_symbols(item, symbols);
            }
        }
        GrammarExpr::Optional(expr)
        | GrammarExpr::ZeroOrMore(expr)
        | GrammarExpr::OneOrMore(expr)
        | GrammarExpr::And(expr)
        | GrammarExpr::Not(expr)
        | GrammarExpr::Capture { expr, .. }
        | GrammarExpr::Repeat { expr, .. } => collect_expr_symbols(expr, symbols),
    }
}

fn ceil_log2(value: usize) -> usize {
    if value <= 1 {
        0
    } else {
        usize::try_from(usize::BITS - (value - 1).leading_zeros()).unwrap_or(usize::MAX)
    }
}

#[allow(clippy::cast_precision_loss)]
const fn usize_to_f64(value: usize) -> f64 {
    value as f64
}
