use std::collections::{BTreeSet, HashMap, HashSet};

use crate::grammar::{CharClassItem, Grammar, GrammarExpr};

pub(super) fn accepts(grammar: &Grammar, text: &str) -> bool {
    Recognizer::new(grammar, text).accepts()
}

struct Recognizer<'g, 't> {
    grammar: &'g Grammar,
    text: &'t str,
    memo: HashMap<(String, usize), BTreeSet<usize>>,
}

impl<'g, 't> Recognizer<'g, 't> {
    fn new(grammar: &'g Grammar, text: &'t str) -> Self {
        Self {
            grammar,
            text,
            memo: HashMap::new(),
        }
    }

    fn accepts(&mut self) -> bool {
        let Some(start) = self.grammar.start_rule() else {
            return false;
        };
        self.match_expr(start.expr(), 0, &mut HashSet::new())
            .contains(&self.text.len())
    }

    fn match_rule(
        &mut self,
        name: &str,
        position: usize,
        stack: &mut HashSet<(String, usize)>,
    ) -> BTreeSet<usize> {
        let key = (name.to_string(), position);
        if let Some(cached) = self.memo.get(&key) {
            return cached.clone();
        }
        if stack.contains(&key) {
            return BTreeSet::new();
        }
        let Some(rule) = self.grammar.rule(name) else {
            return BTreeSet::new();
        };

        stack.insert(key.clone());
        let matches = self.match_expr(rule.expr(), position, stack);
        stack.remove(&key);
        self.memo.insert(key, matches.clone());
        matches
    }

    fn match_expr(
        &mut self,
        expr: &GrammarExpr,
        position: usize,
        stack: &mut HashSet<(String, usize)>,
    ) -> BTreeSet<usize> {
        if position > self.text.len() || !self.text.is_char_boundary(position) {
            return BTreeSet::new();
        }

        match expr {
            GrammarExpr::Empty => singleton(position),
            GrammarExpr::Terminal(value) => {
                if self.text[position..].starts_with(value) {
                    singleton(position + value.len())
                } else {
                    BTreeSet::new()
                }
            }
            GrammarExpr::TerminalInsensitive(value) => {
                if starts_with_ascii_insensitive(&self.text[position..], value) {
                    singleton(position + value.len())
                } else {
                    BTreeSet::new()
                }
            }
            GrammarExpr::CharRange(start, end) => self
                .char_at(position)
                .filter(|(value, _)| start <= value && value <= end)
                .map_or_else(BTreeSet::new, |(_, next)| singleton(next)),
            GrammarExpr::CharClass { negated, items } => self
                .char_at(position)
                .filter(|(value, _)| class_accepts(*value, *negated, items))
                .map_or_else(BTreeSet::new, |(_, next)| singleton(next)),
            GrammarExpr::AnyChar => self
                .char_at(position)
                .map_or_else(BTreeSet::new, |(_, next)| singleton(next)),
            GrammarExpr::NonTerminal(name) => self.match_rule(name, position, stack),
            GrammarExpr::Choice {
                ordered,
                alternatives,
            } => self.match_choice(*ordered, alternatives, position, stack),
            GrammarExpr::Sequence(items) => self.match_sequence(items, position, stack),
            GrammarExpr::Optional(inner) => {
                let mut matches = singleton(position);
                matches.extend(self.match_expr(inner, position, stack));
                matches
            }
            GrammarExpr::ZeroOrMore(inner) => {
                self.repeat_closure(inner, singleton(position), stack)
            }
            GrammarExpr::OneOrMore(inner) => {
                let first = self.match_expr(inner, position, stack);
                self.repeat_closure(inner, first, stack)
            }
            GrammarExpr::Repeat { expr, min, max } => {
                self.repeat_bounds(expr, position, *min, *max, stack)
            }
            GrammarExpr::And(inner) => {
                if self.match_expr(inner, position, stack).is_empty() {
                    BTreeSet::new()
                } else {
                    singleton(position)
                }
            }
            GrammarExpr::Not(inner) => {
                if self.match_expr(inner, position, stack).is_empty() {
                    singleton(position)
                } else {
                    BTreeSet::new()
                }
            }
            GrammarExpr::Capture { expr, .. } => self.match_expr(expr, position, stack),
        }
    }

    fn match_choice(
        &mut self,
        ordered: bool,
        alternatives: &[GrammarExpr],
        position: usize,
        stack: &mut HashSet<(String, usize)>,
    ) -> BTreeSet<usize> {
        if ordered {
            for alternative in alternatives {
                let matches = self.match_expr(alternative, position, stack);
                if !matches.is_empty() {
                    return matches;
                }
            }
            BTreeSet::new()
        } else {
            let mut matches = BTreeSet::new();
            for alternative in alternatives {
                matches.extend(self.match_expr(alternative, position, stack));
            }
            matches
        }
    }

    fn match_sequence(
        &mut self,
        items: &[GrammarExpr],
        position: usize,
        stack: &mut HashSet<(String, usize)>,
    ) -> BTreeSet<usize> {
        let mut positions = singleton(position);
        for item in items {
            let mut next_positions = BTreeSet::new();
            for current in positions {
                next_positions.extend(self.match_expr(item, current, stack));
            }
            if next_positions.is_empty() {
                return BTreeSet::new();
            }
            positions = next_positions;
        }
        positions
    }

    fn repeat_closure(
        &mut self,
        inner: &GrammarExpr,
        starts: BTreeSet<usize>,
        stack: &mut HashSet<(String, usize)>,
    ) -> BTreeSet<usize> {
        let mut results = starts.clone();
        let mut frontier = starts;

        while !frontier.is_empty() {
            let mut next_frontier = BTreeSet::new();
            for position in frontier {
                for next in self.match_expr(inner, position, stack) {
                    if next != position && results.insert(next) {
                        next_frontier.insert(next);
                    }
                }
            }
            frontier = next_frontier;
        }

        results
    }

    fn repeat_bounds(
        &mut self,
        inner: &GrammarExpr,
        position: usize,
        min: usize,
        max: Option<usize>,
        stack: &mut HashSet<(String, usize)>,
    ) -> BTreeSet<usize> {
        let max = max.unwrap_or_else(|| self.remaining_chars(position).saturating_add(min));
        if max < min {
            return BTreeSet::new();
        }

        let mut matches = BTreeSet::new();
        let mut positions = singleton(position);
        for count in 0..=max {
            if count >= min {
                matches.extend(positions.iter().copied());
            }
            if count == max {
                break;
            }
            let mut next_positions = BTreeSet::new();
            for current in positions {
                next_positions.extend(self.match_expr(inner, current, stack));
            }
            if next_positions.is_empty() {
                break;
            }
            positions = next_positions;
        }
        matches
    }

    fn char_at(&self, position: usize) -> Option<(char, usize)> {
        self.text[position..]
            .chars()
            .next()
            .map(|value| (value, position + value.len_utf8()))
    }

    fn remaining_chars(&self, position: usize) -> usize {
        self.text[position..].chars().count()
    }
}

fn starts_with_ascii_insensitive(text: &str, value: &str) -> bool {
    text.get(..value.len())
        .is_some_and(|prefix| prefix.eq_ignore_ascii_case(value))
}

fn class_accepts(value: char, negated: bool, items: &[CharClassItem]) -> bool {
    let contains = items.iter().any(|item| match item {
        CharClassItem::Char(item) => *item == value,
        CharClassItem::Range(start, end) => *start <= value && value <= *end,
    });
    contains != negated
}

fn singleton(value: usize) -> BTreeSet<usize> {
    BTreeSet::from([value])
}
