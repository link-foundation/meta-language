use std::fmt::Write as _;

use crate::grammar::{CharClassItem, Grammar, GrammarExpr, GrammarFormat, RuleKind};

use super::{
    finish_lines, ordered_rules, render_rule_line_with_modifier, unsupported_error, EmitReport,
    GrammarEmitError, PEST_RULE_TEMPLATE,
};

/// Emits pest PEG grammar text from the grammar IR.
///
/// pest maps closely to the grammar IR: ordered choice uses `|`, sequences use
/// `~`, predicates use `&`/`!`, and repetition forms are native. `RuleKind::Token`
/// is emitted as pest's compound-atomic `$` rule modifier, the closest pest
/// analogue to a token-level rule and the syntax a future round-trip importer
/// should invert. Rust `peg::parser!` and `winnow` combinator emission follow the
/// same PEG algebra but are intentionally left to the Rust codegen path.
///
/// # Errors
///
/// Returns [`GrammarEmitError`] when the IR contains an internal invariant that
/// cannot form valid pest text, such as an empty choice, an empty character
/// class, a descending character range, or counted repetition with `max < min`.
pub fn emit_pest(grammar: &Grammar) -> Result<(String, EmitReport), GrammarEmitError> {
    let mut emitter = PestEmitter::default();
    let mut lines = Vec::new();

    for rule in ordered_rules(grammar) {
        if let Some(doc) = rule.doc() {
            push_doc_lines(&mut lines, doc);
        }
        if contains_unordered_choice(rule.expr()) {
            lines.push(
                "// NOTE: unordered choice in source is emitted as ordered pest choice."
                    .to_string(),
            );
        }
        let body = emitter.emit_expr(rule.expr(), Precedence::Choice)?;
        lines.push(render_rule_line_with_modifier(
            PEST_RULE_TEMPLATE,
            rule.name(),
            rule_modifier(rule.kind()),
            &body,
        ));
    }

    Ok((finish_lines(&lines), emitter.report))
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Precedence {
    Choice = 0,
    Sequence = 1,
    Prefix = 2,
    Postfix = 3,
    Atom = 4,
}

#[derive(Clone, Debug, Default)]
struct PestEmitter {
    report: EmitReport,
}

impl PestEmitter {
    fn emit_expr(
        &mut self,
        expr: &GrammarExpr,
        parent: Precedence,
    ) -> Result<String, GrammarEmitError> {
        let (text, precedence) = match expr {
            GrammarExpr::Empty => (quote_terminal(""), Precedence::Atom),
            GrammarExpr::Terminal(value) => (quote_terminal(value), Precedence::Atom),
            GrammarExpr::TerminalInsensitive(value) => {
                (format!("^{}", quote_terminal(value)), Precedence::Atom)
            }
            GrammarExpr::CharRange(start, end) => {
                (emit_char_range(*start, *end)?, Precedence::Atom)
            }
            GrammarExpr::CharClass { negated, items } => {
                (emit_char_class(*negated, items)?, Precedence::Atom)
            }
            GrammarExpr::AnyChar => ("ANY".to_string(), Precedence::Atom),
            GrammarExpr::NonTerminal(name) => (name.clone(), Precedence::Atom),
            GrammarExpr::Choice {
                ordered,
                alternatives,
            } => (
                self.emit_choice(*ordered, alternatives)?,
                Precedence::Choice,
            ),
            GrammarExpr::Sequence(items) => (self.emit_sequence(items)?, Precedence::Sequence),
            GrammarExpr::Optional(inner) => {
                let inner = self.emit_expr(inner, Precedence::Postfix)?;
                (format!("{inner}?"), Precedence::Postfix)
            }
            GrammarExpr::ZeroOrMore(inner) => {
                let inner = self.emit_expr(inner, Precedence::Postfix)?;
                (format!("{inner}*"), Precedence::Postfix)
            }
            GrammarExpr::OneOrMore(inner) => {
                let inner = self.emit_expr(inner, Precedence::Postfix)?;
                (format!("{inner}+"), Precedence::Postfix)
            }
            GrammarExpr::Repeat { expr, min, max } => {
                (self.emit_repeat(expr, *min, *max)?, Precedence::Postfix)
            }
            GrammarExpr::And(inner) => {
                let inner = self.emit_expr(inner, Precedence::Prefix)?;
                (format!("&{inner}"), Precedence::Prefix)
            }
            GrammarExpr::Not(inner) => {
                let inner = self.emit_expr(inner, Precedence::Prefix)?;
                (format!("!{inner}"), Precedence::Prefix)
            }
            GrammarExpr::Capture { label, expr } => {
                if let Some(label) = label {
                    self.report
                        .add_lossy(format!("PEG dropped capture label {label:?}"));
                }
                let inner = self.emit_expr(expr, Precedence::Choice)?;
                (format!("({inner})"), Precedence::Atom)
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
            return Err(unsupported_error(GrammarFormat::Peg, "empty Choice"));
        }
        if !ordered {
            self.report
                .add_lossy("PEG treats unordered choice as ordered choice");
        }

        alternatives
            .iter()
            .map(|alternative| self.emit_expr(alternative, Precedence::Choice))
            .collect::<Result<Vec<_>, _>>()
            .map(|items| items.join(" | "))
    }

    fn emit_sequence(&mut self, items: &[GrammarExpr]) -> Result<String, GrammarEmitError> {
        let mut emitted = Vec::new();
        for item in items {
            if matches!(item, GrammarExpr::Empty) {
                continue;
            }
            let text = self.emit_expr(item, Precedence::Sequence)?;
            if !text.is_empty() {
                emitted.push(text);
            }
        }
        if emitted.is_empty() {
            Ok(quote_terminal(""))
        } else {
            Ok(emitted.join(" ~ "))
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
                GrammarFormat::Peg,
                format!("Repeat with min {min} greater than max {max:?}"),
            ));
        }

        let inner = self.emit_expr(expr, Precedence::Postfix)?;
        let suffix = match max {
            Some(max) if min == max => format!("{{{min}}}"),
            Some(max) => format!("{{{min},{max}}}"),
            None => format!("{{{min},}}"),
        };
        Ok(format!("{inner}{suffix}"))
    }
}

fn emit_char_class(negated: bool, items: &[CharClassItem]) -> Result<String, GrammarEmitError> {
    if items.is_empty() {
        return Err(unsupported_error(GrammarFormat::Peg, "empty CharClass"));
    }

    let inner = items
        .iter()
        .map(emit_char_class_item)
        .collect::<Result<Vec<_>, _>>()?
        .join(" | ");
    if negated {
        Ok(format!("(!({inner}) ~ ANY)"))
    } else {
        Ok(format!("({inner})"))
    }
}

fn emit_char_class_item(item: &CharClassItem) -> Result<String, GrammarEmitError> {
    match item {
        CharClassItem::Char(value) => Ok(quote_terminal(&value.to_string())),
        CharClassItem::Range(start, end) => emit_char_range(*start, *end),
    }
}

fn emit_char_range(start: char, end: char) -> Result<String, GrammarEmitError> {
    if start > end {
        return Err(unsupported_error(
            GrammarFormat::Peg,
            format!(
                "CharRange has descending bounds U+{:04X}..=U+{:04X}",
                start as u32, end as u32
            ),
        ));
    }
    Ok(format!(
        "{}..{}",
        quote_char_literal(start),
        quote_char_literal(end)
    ))
}

const fn rule_modifier(kind: RuleKind) -> &'static str {
    match kind {
        RuleKind::Normal => "",
        RuleKind::Atomic => "@",
        RuleKind::Silent => "_",
        RuleKind::Token => "$",
    }
}

fn push_doc_lines(lines: &mut Vec<String>, doc: &str) {
    if doc.is_empty() {
        lines.push("//".to_string());
        return;
    }
    for line in doc.lines() {
        if line.is_empty() {
            lines.push("//".to_string());
        } else {
            lines.push(format!("// {line}"));
        }
    }
}

fn contains_unordered_choice(expr: &GrammarExpr) -> bool {
    match expr {
        GrammarExpr::Choice { ordered: false, .. } => true,
        GrammarExpr::Choice { alternatives, .. } | GrammarExpr::Sequence(alternatives) => {
            alternatives.iter().any(contains_unordered_choice)
        }
        GrammarExpr::Optional(expr)
        | GrammarExpr::ZeroOrMore(expr)
        | GrammarExpr::OneOrMore(expr)
        | GrammarExpr::And(expr)
        | GrammarExpr::Not(expr)
        | GrammarExpr::Capture { expr, .. }
        | GrammarExpr::Repeat { expr, .. } => contains_unordered_choice(expr),
        GrammarExpr::Empty
        | GrammarExpr::Terminal(_)
        | GrammarExpr::TerminalInsensitive(_)
        | GrammarExpr::CharRange(_, _)
        | GrammarExpr::CharClass { .. }
        | GrammarExpr::AnyChar
        | GrammarExpr::NonTerminal(_) => false,
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

fn quote_char_literal(value: char) -> String {
    let mut output = String::new();
    output.push('\'');
    push_escaped_char_literal_char(&mut output, value);
    output.push('\'');
    output
}

fn push_escaped_string_char(output: &mut String, character: char) {
    match character {
        '"' => output.push_str("\\\""),
        '\\' => output.push_str("\\\\"),
        '\n' => output.push_str("\\n"),
        '\r' => output.push_str("\\r"),
        '\t' => output.push_str("\\t"),
        character if character.is_control() => {
            let _ = write!(output, "\\u{{{:X}}}", character as u32);
        }
        character => output.push(character),
    }
}

fn push_escaped_char_literal_char(output: &mut String, character: char) {
    match character {
        '\'' => output.push_str("\\'"),
        '\\' => output.push_str("\\\\"),
        '\n' => output.push_str("\\n"),
        '\r' => output.push_str("\\r"),
        '\t' => output.push_str("\\t"),
        character if character.is_control() => {
            let _ = write!(output, "\\u{{{:X}}}", character as u32);
        }
        character => output.push(character),
    }
}
