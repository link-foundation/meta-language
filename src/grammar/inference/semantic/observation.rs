use std::collections::BTreeMap;

use crate::grammar::{Grammar, GrammarExpr, GrammarRule};

use super::{has_any, NonTerminalRef};

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(super) struct Observation {
    pub(super) values_by_rule: BTreeMap<String, Vec<ObservedValue>>,
}

impl Observation {
    pub(super) fn from_grammar(grammar: &Grammar, input: &str) -> Self {
        let mut values_by_rule = BTreeMap::new();
        for rule in grammar.rules() {
            let values = extract_rule_values(rule, input);
            if !values.is_empty() {
                values_by_rule.insert(rule.name().to_string(), values);
            }
        }
        Self { values_by_rule }
    }

    pub(super) fn values(&self, reference: &NonTerminalRef) -> &[ObservedValue] {
        self.values_by_rule
            .get(&reference.rule)
            .map_or(&[], Vec::as_slice)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct ObservedValue {
    pub(super) value: String,
    pub(super) position: usize,
}

impl ObservedValue {
    fn new(value: impl Into<String>, position: usize) -> Self {
        Self {
            value: value.into(),
            position,
        }
    }
}

fn extract_rule_values(rule: &GrammarRule, input: &str) -> Vec<ObservedValue> {
    let mut values = extract_by_rule_name(rule.name(), input);
    if values.is_empty() {
        values = extract_by_rule_expr(rule.expr(), rule.name(), input);
    }
    dedup_values(values)
}

fn extract_by_rule_name(rule_name: &str, input: &str) -> Vec<ObservedValue> {
    let lower = rule_name.to_ascii_lowercase();
    if has_any(&lower, &["def", "decl", "bind"]) {
        return extract_after_keywords(input, &["def", "define", "decl", "declare", "let", "var"]);
    }
    if has_any(&lower, &["use", "ref", "call"]) {
        return extract_after_keywords(input, &["use", "uses", "ref", "reference", "call"]);
    }
    if has_any(&lower, &["len", "length", "size"]) {
        return extract_numbers(input);
    }
    if has_any(&lower, &["body", "payload", "data", "content"]) {
        return extract_bodies(input);
    }
    if has_any(&lower, &["open", "left", "start", "begin"]) {
        return extract_characters(input, &['(', '[', '{', '<']);
    }
    if has_any(&lower, &["close", "right", "end"]) {
        return extract_characters(input, &[')', ']', '}', '>']);
    }
    if has_any(&lower, &["number", "index", "rank", "seq"]) {
        return extract_numbers(input);
    }
    if has_any(&lower, &["id", "name", "symbol", "item", "key"]) {
        return extract_identifiers(input);
    }
    Vec::new()
}

fn extract_by_rule_expr(expr: &GrammarExpr, rule_name: &str, input: &str) -> Vec<ObservedValue> {
    let mut terminals = Vec::new();
    collect_terminals(expr, &mut terminals);
    let mut values = terminals
        .iter()
        .flat_map(|terminal| extract_literal(input, terminal))
        .collect::<Vec<_>>();

    if values.is_empty() && expr_contains_digit_range(expr) {
        values = extract_numbers(input);
    }
    if values.is_empty() && expr_contains_alpha_range(expr) && has_any(rule_name, &["id", "name"]) {
        values = extract_identifiers(input);
    }

    values
}

fn collect_terminals(expr: &GrammarExpr, terminals: &mut Vec<String>) {
    match expr {
        GrammarExpr::Terminal(value) | GrammarExpr::TerminalInsensitive(value) => {
            if !value.is_empty() {
                terminals.push(value.clone());
            }
        }
        GrammarExpr::Choice { alternatives, .. } => {
            for alternative in alternatives {
                collect_terminals(alternative, terminals);
            }
        }
        GrammarExpr::Sequence(items) => {
            for item in items {
                collect_terminals(item, terminals);
            }
        }
        GrammarExpr::Optional(inner)
        | GrammarExpr::ZeroOrMore(inner)
        | GrammarExpr::OneOrMore(inner)
        | GrammarExpr::And(inner)
        | GrammarExpr::Not(inner)
        | GrammarExpr::Repeat { expr: inner, .. }
        | GrammarExpr::Capture { expr: inner, .. } => collect_terminals(inner, terminals),
        GrammarExpr::Empty
        | GrammarExpr::CharRange(_, _)
        | GrammarExpr::CharClass { .. }
        | GrammarExpr::AnyChar
        | GrammarExpr::NonTerminal(_) => {}
    }
}

fn expr_contains_digit_range(expr: &GrammarExpr) -> bool {
    match expr {
        GrammarExpr::CharRange(start, end) => *start <= '0' && *end >= '9',
        GrammarExpr::CharClass { items, .. } => items.iter().any(|item| {
            let text = item.to_string();
            text.contains('0') || text.contains('9')
        }),
        GrammarExpr::Choice { alternatives, .. } => {
            alternatives.iter().any(expr_contains_digit_range)
        }
        GrammarExpr::Sequence(items) => items.iter().any(expr_contains_digit_range),
        GrammarExpr::Optional(inner)
        | GrammarExpr::ZeroOrMore(inner)
        | GrammarExpr::OneOrMore(inner)
        | GrammarExpr::And(inner)
        | GrammarExpr::Not(inner)
        | GrammarExpr::Repeat { expr: inner, .. }
        | GrammarExpr::Capture { expr: inner, .. } => expr_contains_digit_range(inner),
        GrammarExpr::Empty
        | GrammarExpr::Terminal(_)
        | GrammarExpr::TerminalInsensitive(_)
        | GrammarExpr::AnyChar
        | GrammarExpr::NonTerminal(_) => false,
    }
}

fn expr_contains_alpha_range(expr: &GrammarExpr) -> bool {
    match expr {
        GrammarExpr::CharRange(start, end) => *start <= 'a' && *end >= 'z',
        GrammarExpr::CharClass { items, .. } => items.iter().any(|item| {
            let text = item.to_string();
            text.contains('a') || text.contains('z')
        }),
        GrammarExpr::Choice { alternatives, .. } => {
            alternatives.iter().any(expr_contains_alpha_range)
        }
        GrammarExpr::Sequence(items) => items.iter().any(expr_contains_alpha_range),
        GrammarExpr::Optional(inner)
        | GrammarExpr::ZeroOrMore(inner)
        | GrammarExpr::OneOrMore(inner)
        | GrammarExpr::And(inner)
        | GrammarExpr::Not(inner)
        | GrammarExpr::Repeat { expr: inner, .. }
        | GrammarExpr::Capture { expr: inner, .. } => expr_contains_alpha_range(inner),
        GrammarExpr::Empty
        | GrammarExpr::Terminal(_)
        | GrammarExpr::TerminalInsensitive(_)
        | GrammarExpr::AnyChar
        | GrammarExpr::NonTerminal(_) => false,
    }
}

fn extract_after_keywords(input: &str, keywords: &[&str]) -> Vec<ObservedValue> {
    let lower = input.to_ascii_lowercase();
    let mut values = Vec::new();

    for keyword in keywords {
        let mut search_start = 0;
        while let Some(relative) = lower[search_start..].find(keyword) {
            let position = search_start + relative;
            let after = position + keyword.len();
            if has_word_boundary_before(&lower, position) && has_word_boundary_after(&lower, after)
            {
                if let Some(value) = next_identifier(input, after) {
                    values.push(value);
                }
            }
            search_start = after;
        }
    }

    values
}

fn has_word_boundary_before(input: &str, position: usize) -> bool {
    position == 0
        || input[..position]
            .chars()
            .next_back()
            .map_or(true, |character| !is_identifier_character(character))
}

fn has_word_boundary_after(input: &str, position: usize) -> bool {
    position >= input.len()
        || input[position..]
            .chars()
            .next()
            .map_or(true, |character| !is_identifier_character(character))
}

fn next_identifier(input: &str, start: usize) -> Option<ObservedValue> {
    let mut token_start = None;
    let mut token_end = start;

    for (offset, character) in input[start..].char_indices() {
        let position = start + offset;
        if token_start.is_none() {
            if matches!(character, ';' | '\n' | '\r') {
                return None;
            }
            if is_identifier_character(character) {
                token_start = Some(position);
                token_end = position + character.len_utf8();
            }
            continue;
        }

        if is_identifier_character(character) {
            token_end = position + character.len_utf8();
        } else {
            break;
        }
    }

    let token_start = token_start?;
    Some(ObservedValue::new(
        &input[token_start..token_end],
        token_start,
    ))
}

fn extract_numbers(input: &str) -> Vec<ObservedValue> {
    extract_runs(input, |character| character.is_ascii_digit())
}

fn extract_identifiers(input: &str) -> Vec<ObservedValue> {
    extract_runs(input, is_identifier_character)
        .into_iter()
        .filter(|value| !is_keyword(&value.value))
        .collect()
}

fn extract_runs(input: &str, accepts: impl Fn(char) -> bool) -> Vec<ObservedValue> {
    let mut values = Vec::new();
    let mut start = None;
    let mut end = 0;

    for (position, character) in input.char_indices() {
        if accepts(character) {
            if start.is_none() {
                start = Some(position);
            }
            end = position + character.len_utf8();
        } else if let Some(token_start) = start.take() {
            values.push(ObservedValue::new(&input[token_start..end], token_start));
        }
    }

    if let Some(token_start) = start {
        values.push(ObservedValue::new(&input[token_start..end], token_start));
    }

    values
}

const fn is_identifier_character(character: char) -> bool {
    matches!(character, '_' | '0'..='9' | 'A'..='Z' | 'a'..='z')
}

fn is_keyword(value: &str) -> bool {
    matches!(
        value.to_ascii_lowercase().as_str(),
        "def"
            | "define"
            | "decl"
            | "declare"
            | "let"
            | "var"
            | "use"
            | "uses"
            | "ref"
            | "reference"
            | "call"
            | "len"
            | "length"
            | "body"
            | "payload"
            | "data"
            | "content"
            | "item"
    )
}

fn extract_bodies(input: &str) -> Vec<ObservedValue> {
    let mut values = Vec::new();

    for (position, character) in input.char_indices() {
        if character == ':' {
            if let Some(body) = body_after(input, position + character.len_utf8()) {
                values.push(body);
            }
        }
    }

    for marker in ["body=", "payload=", "data=", "content="] {
        let mut search_start = 0;
        while let Some(relative) = input[search_start..].find(marker) {
            let start = search_start + relative + marker.len();
            if let Some(body) = body_after(input, start) {
                values.push(body);
            }
            search_start = start;
        }
    }

    values
}

fn body_after(input: &str, start: usize) -> Option<ObservedValue> {
    let mut token_start = None;
    let mut token_end = start;

    for (offset, character) in input[start..].char_indices() {
        let position = start + offset;
        if token_start.is_none() {
            if character.is_whitespace() {
                continue;
            }
            if matches!(character, ';' | ')' | ']' | '}') {
                return None;
            }
            token_start = Some(position);
            token_end = position + character.len_utf8();
            continue;
        }

        if character.is_whitespace() || matches!(character, ';' | ')' | ']' | '}') {
            break;
        }
        token_end = position + character.len_utf8();
    }

    let token_start = token_start?;
    Some(ObservedValue::new(
        &input[token_start..token_end],
        token_start,
    ))
}

fn extract_characters(input: &str, characters: &[char]) -> Vec<ObservedValue> {
    input
        .char_indices()
        .filter(|(_position, character)| characters.contains(character))
        .map(|(position, character)| ObservedValue::new(character.to_string(), position))
        .collect()
}

fn extract_literal(input: &str, literal: &str) -> Vec<ObservedValue> {
    if literal.is_empty() {
        return Vec::new();
    }

    let mut values = Vec::new();
    let mut search_start = 0;
    while let Some(relative) = input[search_start..].find(literal) {
        let position = search_start + relative;
        values.push(ObservedValue::new(literal, position));
        search_start = position + literal.len();
    }
    values
}

fn dedup_values(mut values: Vec<ObservedValue>) -> Vec<ObservedValue> {
    values.sort_by(|left, right| {
        left.position
            .cmp(&right.position)
            .then_with(|| left.value.cmp(&right.value))
    });
    values.dedup();
    values
}
