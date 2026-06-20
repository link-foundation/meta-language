use crate::grammar::{CharClassItem, Grammar, GrammarExpr, GrammarFormat};

use super::{
    finish_lines, ordered_rules, render_rule_line, unsupported_error, EmitReport, GrammarEmitError,
    ABNF_RULE_TEMPLATE,
};

/// Emits Augmented Backus-Naur Form text from the grammar IR.
///
/// ABNF has native alternation, grouping, optional expressions, counted
/// repetition, numeric value ranges, and RFC 7405 case-sensitive string
/// prefixes.
///
/// # Errors
///
/// Returns [`GrammarEmitError`] when the grammar contains a construct ABNF
/// cannot represent, such as PEG predicates or negated character classes.
pub fn emit_abnf(grammar: &Grammar) -> Result<(String, EmitReport), GrammarEmitError> {
    let mut emitter = AbnfEmitter::default();
    let mut lines = Vec::new();

    for rule in ordered_rules(grammar) {
        let body = emitter.emit_expr(rule.expr(), Precedence::Choice)?;
        lines.push(render_rule_line(ABNF_RULE_TEMPLATE, rule.name(), &body));
    }

    Ok((finish_lines(&lines), emitter.report))
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Precedence {
    Choice = 0,
    Sequence = 1,
    Atom = 2,
}

#[derive(Clone, Debug, Default)]
struct AbnfEmitter {
    report: EmitReport,
}

impl AbnfEmitter {
    fn emit_expr(
        &mut self,
        expr: &GrammarExpr,
        parent: Precedence,
    ) -> Result<String, GrammarEmitError> {
        let (text, precedence) = match expr {
            GrammarExpr::Empty => ("\"\"".to_string(), Precedence::Atom),
            GrammarExpr::Terminal(value) => (quote_terminal("%s", value), Precedence::Atom),
            GrammarExpr::TerminalInsensitive(value) => {
                (quote_terminal("%i", value), Precedence::Atom)
            }
            GrammarExpr::CharRange(start, end) => {
                (emit_char_range(*start, *end)?, Precedence::Atom)
            }
            GrammarExpr::CharClass { negated, items } => {
                (emit_char_class(*negated, items)?, Precedence::Atom)
            }
            GrammarExpr::AnyChar => ("%x00-10FFFF".to_string(), Precedence::Atom),
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
                let inner = self.emit_expr(inner, Precedence::Choice)?;
                (format!("[ {inner} ]"), Precedence::Atom)
            }
            GrammarExpr::ZeroOrMore(inner) => {
                let inner = self.emit_expr(inner, Precedence::Choice)?;
                (format!("*( {inner} )"), Precedence::Atom)
            }
            GrammarExpr::OneOrMore(inner) => {
                let inner = self.emit_expr(inner, Precedence::Choice)?;
                (format!("1*( {inner} )"), Precedence::Atom)
            }
            GrammarExpr::Repeat { expr, min, max } => {
                (self.emit_repeat(expr, *min, *max)?, Precedence::Atom)
            }
            GrammarExpr::And(_) => return Err(unsupported_error(GrammarFormat::Abnf, "And")),
            GrammarExpr::Not(_) => return Err(unsupported_error(GrammarFormat::Abnf, "Not")),
            GrammarExpr::Capture { label, expr } => {
                report_capture_loss(&mut self.report, GrammarFormat::Abnf, label.as_ref());
                return self.emit_expr(expr, parent);
            }
        };

        if precedence < parent {
            Ok(format!("( {text} )"))
        } else {
            Ok(text)
        }
    }

    fn emit_choice(
        &mut self,
        ordered: bool,
        alternatives: &[GrammarExpr],
    ) -> Result<String, GrammarEmitError> {
        if ordered {
            self.report
                .add_lossy("ABNF treats ordered choice as unordered choice");
        }
        alternatives
            .iter()
            .map(|alternative| self.emit_expr(alternative, Precedence::Choice))
            .collect::<Result<Vec<_>, _>>()
            .map(|items| items.join(" / "))
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
            Ok("\"\"".to_string())
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
                GrammarFormat::Abnf,
                format!("Repeat with min {min} greater than max {max:?}"),
            ));
        }

        let inner = self.emit_expr(expr, Precedence::Choice)?;
        let prefix = max.map_or_else(
            || {
                if min == 0 {
                    "*".to_string()
                } else {
                    format!("{min}*")
                }
            },
            |max| {
                if min == 0 {
                    format!("*{max}")
                } else {
                    format!("{min}*{max}")
                }
            },
        );
        Ok(format!("{prefix}( {inner} )"))
    }
}

fn emit_char_range(start: char, end: char) -> Result<String, GrammarEmitError> {
    if start > end {
        return Err(unsupported_error(
            GrammarFormat::Abnf,
            format!(
                "CharRange has descending bounds U+{:04X}..=U+{:04X}",
                start as u32, end as u32
            ),
        ));
    }
    Ok(format!("%x{}-{}", hex_char(start), hex_char(end)))
}

fn emit_char_class(negated: bool, items: &[CharClassItem]) -> Result<String, GrammarEmitError> {
    if negated {
        return Err(unsupported_error(GrammarFormat::Abnf, "negated CharClass"));
    }
    if items.is_empty() {
        return Err(unsupported_error(GrammarFormat::Abnf, "empty CharClass"));
    }

    let items = items
        .iter()
        .map(emit_char_class_item)
        .collect::<Result<Vec<_>, _>>()?
        .join(" / ");
    Ok(format!("( {items} )"))
}

fn emit_char_class_item(item: &CharClassItem) -> Result<String, GrammarEmitError> {
    match item {
        CharClassItem::Char(value) => Ok(format!("%x{}", hex_char(*value))),
        CharClassItem::Range(start, end) => emit_char_range(*start, *end),
    }
}

fn report_capture_loss(report: &mut EmitReport, format: GrammarFormat, label: Option<&String>) {
    if let Some(label) = label {
        report.add_lossy(format!("{format} dropped capture label {label:?}"));
    } else {
        report.add_lossy(format!("{format} dropped anonymous capture"));
    }
}

fn quote_terminal(prefix: &str, value: &str) -> String {
    if value.is_empty() {
        return "\"\"".to_string();
    }
    if abnf_char_value_safe(value) {
        return format!("{prefix}\"{value}\"");
    }
    numeric_terminal(value)
}

fn abnf_char_value_safe(value: &str) -> bool {
    value
        .chars()
        .all(|character| matches!(character as u32, 0x20 | 0x21 | 0x23..=0x7e))
}

fn numeric_terminal(value: &str) -> String {
    let values = value.chars().map(hex_char).collect::<Vec<_>>().join(".");
    format!("%x{values}")
}

fn hex_char(value: char) -> String {
    let code = value as u32;
    if code <= 0xff {
        format!("{code:02X}")
    } else {
        format!("{code:X}")
    }
}
