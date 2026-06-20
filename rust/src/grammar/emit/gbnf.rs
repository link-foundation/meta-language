use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;

use crate::grammar::{CharClassItem, Grammar, GrammarExpr, GrammarFormat};

use super::{
    finish_lines, render_rule_line, unsupported_error, EmitReport, GrammarEmitError,
    GBNF_RULE_TEMPLATE,
};

const ANY_CHAR_CLASS: &str = r"[\x00-\U0010FFFF]";

/// Emits GGML BNF text for grammar-constrained LLM decoding.
///
/// The output is deterministic, starts with the mandatory GBNF `root` rule, and
/// uses GBNF's native alternation, grouping, character classes, and counted
/// repetition operators. The IR's `AnyChar` wildcard is emitted as the documented
/// full Unicode scalar range class `[\x00-\U0010FFFF]`.
///
/// # Errors
///
/// Returns [`GrammarEmitError`] when the grammar contains a construct GBNF
/// cannot faithfully represent, such as general PEG lookahead predicates, empty
/// character classes, descending ranges, invalid repeat bounds, or a configured
/// start rule that is not present in the grammar.
pub fn emit_gbnf(grammar: &Grammar) -> Result<(String, EmitReport), GrammarEmitError> {
    if grammar.rules().is_empty() {
        return Ok((String::new(), EmitReport::default()));
    }

    let start_index = start_rule_index(grammar).ok_or_else(|| {
        unsupported_error(
            GrammarFormat::Gbnf,
            "configured start rule is not present in the grammar",
        )
    })?;
    let start_name = grammar.rules()[start_index].name();

    let mut report = EmitReport::default();
    let names = NamePlan::new(grammar, start_name, &mut report);
    let mut emitter = GbnfEmitter { report, names };
    let mut lines = Vec::new();

    let start_body = emitter.emit_expr(grammar.rules()[start_index].expr(), Precedence::Choice)?;
    lines.push(render_rule_line(GBNF_RULE_TEMPLATE, "root", &start_body));

    for (index, rule) in grammar.rules().iter().enumerate() {
        if index == start_index {
            continue;
        }
        let name = emitter.names.name_for(rule.name());
        let body = emitter.emit_expr(rule.expr(), Precedence::Choice)?;
        lines.push(render_rule_line(GBNF_RULE_TEMPLATE, &name, &body));
    }

    Ok((finish_lines(&lines), emitter.report))
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Precedence {
    Choice = 0,
    Sequence = 1,
    Postfix = 2,
    Atom = 3,
}

#[derive(Clone, Debug)]
struct GbnfEmitter {
    report: EmitReport,
    names: NamePlan,
}

impl GbnfEmitter {
    fn emit_expr(
        &mut self,
        expr: &GrammarExpr,
        parent: Precedence,
    ) -> Result<String, GrammarEmitError> {
        let (text, precedence) = match expr {
            GrammarExpr::Empty => (quote_terminal(""), Precedence::Atom),
            GrammarExpr::Terminal(value) => (quote_terminal(value), Precedence::Atom),
            GrammarExpr::TerminalInsensitive(value) => {
                self.report.add_lossy(format!(
                    "GBNF expands case-insensitive terminal {value:?} to character classes"
                ));
                (emit_case_insensitive_terminal(value), Precedence::Sequence)
            }
            GrammarExpr::CharRange(start, end) => {
                (emit_char_range(*start, *end)?, Precedence::Atom)
            }
            GrammarExpr::CharClass { negated, items } => {
                (emit_char_class(*negated, items)?, Precedence::Atom)
            }
            GrammarExpr::AnyChar => (ANY_CHAR_CLASS.to_string(), Precedence::Atom),
            GrammarExpr::NonTerminal(name) => (self.names.name_for(name), Precedence::Atom),
            GrammarExpr::Choice {
                ordered,
                alternatives,
            } => (
                self.emit_choice(*ordered, alternatives)?,
                Precedence::Choice,
            ),
            GrammarExpr::Sequence(items) => (self.emit_sequence(items)?, Precedence::Sequence),
            GrammarExpr::Optional(inner) => {
                let inner = self.emit_expr(inner, Precedence::Choice)?;
                (format!("({inner})?"), Precedence::Postfix)
            }
            GrammarExpr::ZeroOrMore(inner) => {
                let inner = self.emit_expr(inner, Precedence::Choice)?;
                (format!("({inner})*"), Precedence::Postfix)
            }
            GrammarExpr::OneOrMore(inner) => {
                let inner = self.emit_expr(inner, Precedence::Choice)?;
                (format!("({inner})+"), Precedence::Postfix)
            }
            GrammarExpr::Repeat { expr, min, max } => {
                (self.emit_repeat(expr, *min, *max)?, Precedence::Postfix)
            }
            GrammarExpr::And(_) => {
                return Err(unsupported_error(GrammarFormat::Gbnf, "and-predicate"));
            }
            GrammarExpr::Not(_) => {
                return Err(unsupported_error(GrammarFormat::Gbnf, "not-predicate"));
            }
            GrammarExpr::Capture { label, expr } => {
                report_capture_loss(&mut self.report, label.as_ref());
                return self.emit_expr(expr, parent);
            }
        };

        if precedence < parent {
            Ok(format!("({text})"))
        } else {
            Ok(text)
        }
    }

    fn emit_choice(
        &mut self,
        ordered: bool,
        alternatives: &[GrammarExpr],
    ) -> Result<String, GrammarEmitError> {
        if alternatives.is_empty() {
            return Err(unsupported_error(GrammarFormat::Gbnf, "empty Choice"));
        }
        if ordered {
            self.report
                .add_lossy("GBNF treats ordered choice as unordered choice");
        }

        alternatives
            .iter()
            .map(|alternative| self.emit_expr(alternative, Precedence::Choice))
            .collect::<Result<Vec<_>, _>>()
            .map(|items| items.join(" | "))
    }

    fn emit_sequence(&mut self, items: &[GrammarExpr]) -> Result<String, GrammarEmitError> {
        let mut emitted = Vec::new();
        let mut index = 0;
        while index < items.len() {
            if let Some(class_items) = negated_class_peephole(items, index) {
                emitted.push(emit_char_class(true, &class_items)?);
                index += 2;
                continue;
            }

            let text = self.emit_expr(&items[index], Precedence::Sequence)?;
            if !text.is_empty() {
                emitted.push(text);
            }
            index += 1;
        }

        if emitted.is_empty() {
            Ok(quote_terminal(""))
        } else {
            Ok(emitted.join(" "))
        }
    }

    fn emit_repeat(
        &mut self,
        expr: &GrammarExpr,
        min: usize,
        max: Option<usize>,
    ) -> Result<String, GrammarEmitError> {
        if max.is_some_and(|max| max < min) {
            return Err(unsupported_error(
                GrammarFormat::Gbnf,
                format!("Repeat with min {min} greater than max {max:?}"),
            ));
        }

        let inner = self.emit_expr(expr, Precedence::Choice)?;
        let suffix = match max {
            Some(max) if min == max => format!("{{{min}}}"),
            Some(max) => format!("{{{min},{max}}}"),
            None => format!("{{{min},}}"),
        };
        Ok(format!("({inner}){suffix}"))
    }
}

#[derive(Clone, Debug)]
struct NamePlan {
    names: BTreeMap<String, String>,
}

impl NamePlan {
    fn new(grammar: &Grammar, start_name: &str, report: &mut EmitReport) -> Self {
        let defined_names = grammar
            .rules()
            .iter()
            .map(|rule| rule.name().to_string())
            .collect::<BTreeSet<_>>();
        let mut symbols = Vec::new();
        let mut seen = BTreeSet::new();
        for rule in grammar.rules() {
            push_unique_symbol(&mut symbols, &mut seen, rule.name());
        }
        for reference in grammar.referenced_nonterminals() {
            push_unique_symbol(&mut symbols, &mut seen, &reference);
        }

        let mut used = BTreeSet::from(["root".to_string()]);
        let mut names = BTreeMap::new();
        names.insert(start_name.to_string(), "root".to_string());

        for symbol in symbols {
            if symbol == start_name {
                continue;
            }
            let base = sanitize_identifier(&symbol);
            let emitted = unique_identifier(&base, &mut used);
            if emitted != symbol {
                report_name_change(report, &defined_names, &symbol, &emitted);
            }
            names.insert(symbol, emitted);
        }

        Self { names }
    }

    fn name_for(&self, source: &str) -> String {
        self.names
            .get(source)
            .map_or_else(|| source.to_string(), Clone::clone)
    }
}

fn push_unique_symbol(symbols: &mut Vec<String>, seen: &mut BTreeSet<String>, symbol: &str) {
    if seen.insert(symbol.to_string()) {
        symbols.push(symbol.to_string());
    }
}

fn report_name_change(
    report: &mut EmitReport,
    defined_names: &BTreeSet<String>,
    source: &str,
    emitted: &str,
) {
    let kind = if defined_names.contains(source) {
        "rule"
    } else {
        "non-terminal reference"
    };
    report.add_lossy(format!("GBNF renamed {kind} {source:?} to {emitted:?}"));
}

fn unique_identifier(base: &str, used: &mut BTreeSet<String>) -> String {
    if used.insert(base.to_string()) {
        return base.to_string();
    }

    for suffix in 1_usize.. {
        let candidate = format!("{base}-{suffix}");
        if used.insert(candidate.clone()) {
            return candidate;
        }
    }
    unreachable!("unbounded suffix search must eventually find a free identifier")
}

fn sanitize_identifier(source: &str) -> String {
    let mut output = String::new();
    let mut previous_hyphen = false;
    for character in source.chars() {
        let sanitized = if character.is_ascii_alphanumeric() {
            character
        } else {
            '-'
        };
        if sanitized == '-' {
            if !previous_hyphen {
                output.push(sanitized);
                previous_hyphen = true;
            }
        } else {
            output.push(sanitized);
            previous_hyphen = false;
        }
    }

    let mut output = output.trim_matches('-').to_string();
    if output.is_empty() {
        output.push_str("ml");
    }
    if !output.starts_with(|character: char| character.is_ascii_alphabetic()) {
        output.insert_str(0, "ml-");
    }
    output
}

fn start_rule_index(grammar: &Grammar) -> Option<usize> {
    grammar.start().map_or(Some(0), |start| {
        grammar.rules().iter().position(|rule| rule.name() == start)
    })
}

fn negated_class_peephole(items: &[GrammarExpr], index: usize) -> Option<Vec<CharClassItem>> {
    let GrammarExpr::Not(inner) = items.get(index)? else {
        return None;
    };
    if !matches!(items.get(index + 1), Some(GrammarExpr::AnyChar)) {
        return None;
    }
    predicate_negated_class_items(inner)
}

fn predicate_negated_class_items(expr: &GrammarExpr) -> Option<Vec<CharClassItem>> {
    match expr {
        GrammarExpr::CharClass {
            negated: false,
            items,
        } => Some(items.clone()),
        GrammarExpr::Terminal(value) => {
            let mut chars = value.chars();
            let character = chars.next()?;
            chars
                .next()
                .is_none()
                .then_some(vec![CharClassItem::Char(character)])
        }
        _ => None,
    }
}

fn emit_case_insensitive_terminal(value: &str) -> String {
    if value.is_empty() {
        return quote_terminal("");
    }

    value
        .chars()
        .map(emit_case_insensitive_char)
        .collect::<String>()
}

fn emit_case_insensitive_char(character: char) -> String {
    if character.is_ascii_alphabetic() {
        let upper = character.to_ascii_uppercase();
        let lower = character.to_ascii_lowercase();
        let mut output = String::new();
        output.push('[');
        push_escaped_class_char(&mut output, upper);
        push_escaped_class_char(&mut output, lower);
        output.push(']');
        output
    } else {
        quote_terminal(&character.to_string())
    }
}

fn emit_char_range(start: char, end: char) -> Result<String, GrammarEmitError> {
    validate_range("CharRange", start, end)?;
    Ok(format!(
        "[{}-{}]",
        escaped_class_char(start),
        escaped_class_char(end)
    ))
}

fn emit_char_class(negated: bool, items: &[CharClassItem]) -> Result<String, GrammarEmitError> {
    if items.is_empty() {
        return Err(unsupported_error(GrammarFormat::Gbnf, "empty CharClass"));
    }

    let mut output = String::new();
    output.push('[');
    if negated {
        output.push('^');
    }
    for item in items {
        output.push_str(&emit_char_class_item(item)?);
    }
    output.push(']');
    Ok(output)
}

fn emit_char_class_item(item: &CharClassItem) -> Result<String, GrammarEmitError> {
    match item {
        CharClassItem::Char(value) => Ok(escaped_class_char(*value)),
        CharClassItem::Range(start, end) => {
            validate_range("CharClass range", *start, *end)?;
            Ok(format!(
                "{}-{}",
                escaped_class_char(*start),
                escaped_class_char(*end)
            ))
        }
    }
}

fn validate_range(construct: &str, start: char, end: char) -> Result<(), GrammarEmitError> {
    if start > end {
        return Err(unsupported_error(
            GrammarFormat::Gbnf,
            format!(
                "{construct} has descending bounds U+{:04X}..=U+{:04X}",
                start as u32, end as u32
            ),
        ));
    }
    Ok(())
}

fn report_capture_loss(report: &mut EmitReport, label: Option<&String>) {
    if let Some(label) = label {
        report.add_lossy(format!("GBNF dropped capture label {label:?}"));
    } else {
        report.add_lossy("GBNF dropped anonymous capture");
    }
}

fn quote_terminal(value: &str) -> String {
    let mut output = String::with_capacity(value.len() + 2);
    output.push('"');
    for character in value.chars() {
        push_escaped_string_char(&mut output, character);
    }
    output.push('"');
    output
}

fn escaped_class_char(character: char) -> String {
    let mut output = String::new();
    push_escaped_class_char(&mut output, character);
    output
}

fn push_escaped_string_char(output: &mut String, character: char) {
    match character {
        '"' => output.push_str("\\\""),
        '\\' => output.push_str("\\\\"),
        '\n' => output.push_str("\\n"),
        '\r' => output.push_str("\\r"),
        '\t' => output.push_str("\\t"),
        '\u{08}' => output.push_str("\\b"),
        '\u{0c}' => output.push_str("\\f"),
        character if character.is_control() => push_hex_escape(output, character),
        character => output.push(character),
    }
}

fn push_escaped_class_char(output: &mut String, character: char) {
    match character {
        '\\' => output.push_str("\\\\"),
        '[' => output.push_str("\\["),
        ']' => output.push_str("\\]"),
        '-' => output.push_str("\\-"),
        '^' => output.push_str("\\^"),
        '\n' => output.push_str("\\n"),
        '\r' => output.push_str("\\r"),
        '\t' => output.push_str("\\t"),
        '\u{08}' => output.push_str("\\b"),
        '\u{0c}' => output.push_str("\\f"),
        character if character.is_control() => push_hex_escape(output, character),
        character => output.push(character),
    }
}

fn push_hex_escape(output: &mut String, character: char) {
    let code = character as u32;
    if code <= 0xff {
        let _ = write!(output, "\\x{code:02X}");
    } else if code <= 0xffff {
        let _ = write!(output, "\\u{code:04X}");
    } else {
        let _ = write!(output, "\\U{code:08X}");
    }
}
