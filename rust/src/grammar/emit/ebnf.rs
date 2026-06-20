use crate::grammar::{CharClassItem, Grammar, GrammarExpr, GrammarFormat};

use super::{
    expanded_chars, finish_lines, ordered_rules, render_rule_line, unsupported_error, EmitReport,
    GrammarEmitError, HelperRules, EBNF_RULE_TEMPLATE,
};

const MAX_EBNF_EXPANSION: u32 = 256;

/// Emits ISO/IEC 14977-style Extended Backus-Naur Form text from the grammar IR.
///
/// EBNF has native optional and repetition operators, but ISO EBNF does not have
/// character ranges or character classes, so those constructs are expanded
/// through deterministic helper productions.
///
/// # Errors
///
/// Returns [`GrammarEmitError`] when the grammar contains a construct ISO EBNF
/// cannot represent, such as PEG predicates, negated character classes, or a
/// character range too large to expand safely.
pub fn emit_ebnf(grammar: &Grammar) -> Result<(String, EmitReport), GrammarEmitError> {
    let mut emitter = EbnfEmitter::new(grammar);
    let mut lines = Vec::new();

    for rule in ordered_rules(grammar) {
        let body = emitter.emit_expr(rule.expr(), Precedence::Choice)?;
        lines.push(render_rule_line(EBNF_RULE_TEMPLATE, rule.name(), &body));
    }
    for helper in emitter.helpers.entries() {
        lines.push(render_rule_line(
            EBNF_RULE_TEMPLATE,
            &helper.name,
            &helper.body,
        ));
    }

    Ok((finish_lines(&lines), emitter.report))
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Precedence {
    Choice = 0,
    Sequence = 1,
    Atom = 2,
}

#[derive(Clone, Debug)]
struct EbnfEmitter {
    report: EmitReport,
    helpers: HelperRules,
}

impl EbnfEmitter {
    fn new(grammar: &Grammar) -> Self {
        Self {
            report: EmitReport::default(),
            helpers: HelperRules::new(grammar),
        }
    }

    fn emit_expr(
        &mut self,
        expr: &GrammarExpr,
        parent: Precedence,
    ) -> Result<String, GrammarEmitError> {
        let (text, precedence) = match expr {
            GrammarExpr::Empty => (String::new(), Precedence::Atom),
            GrammarExpr::Terminal(value) => (quote_terminal(value), Precedence::Atom),
            GrammarExpr::TerminalInsensitive(value) => {
                self.report.add_lossy(format!(
                    "EBNF cannot preserve case-insensitive terminal {value:?}"
                ));
                (quote_terminal(value), Precedence::Atom)
            }
            GrammarExpr::CharRange(start, end) => {
                (self.emit_range_helper(*start, *end)?, Precedence::Atom)
            }
            GrammarExpr::CharClass { negated, items } => (
                self.emit_char_class_helper(*negated, items)?,
                Precedence::Atom,
            ),
            GrammarExpr::AnyChar => ("? any character ?".to_string(), Precedence::Atom),
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
                (format!("{{ {inner} }}"), Precedence::Atom)
            }
            GrammarExpr::OneOrMore(inner) => (
                self.emit_repeat_sequence(inner, 1, None)?,
                Precedence::Sequence,
            ),
            GrammarExpr::Repeat { expr, min, max } => (
                self.emit_repeat_sequence(expr, *min, *max)?,
                Precedence::Sequence,
            ),
            GrammarExpr::And(_) => return Err(unsupported_error(GrammarFormat::Ebnf, "And")),
            GrammarExpr::Not(_) => return Err(unsupported_error(GrammarFormat::Ebnf, "Not")),
            GrammarExpr::Capture { label, expr } => {
                report_capture_loss(&mut self.report, GrammarFormat::Ebnf, label.as_ref());
                return self.emit_expr(expr, parent);
            }
        };

        if precedence < parent && !text.is_empty() {
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
        if ordered {
            self.report
                .add_lossy("EBNF treats ordered choice as unordered choice");
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
            let text = self.emit_expr(item, Precedence::Sequence)?;
            if !text.is_empty() {
                emitted.push(text);
            }
        }
        Ok(emitted.join(" , "))
    }

    fn emit_repeat_sequence(
        &mut self,
        expr: &GrammarExpr,
        min: usize,
        max: Option<usize>,
    ) -> Result<String, GrammarEmitError> {
        if max.is_some_and(|max| max < min) {
            return Err(unsupported_error(
                GrammarFormat::Ebnf,
                format!("Repeat with min {min} greater than max {max:?}"),
            ));
        }

        let mut parts = Vec::new();
        for _ in 0..min {
            let part = self.emit_expr(expr, Precedence::Sequence)?;
            if !part.is_empty() {
                parts.push(part);
            }
        }

        if let Some(max) = max {
            for _ in min..max {
                let inner = self.emit_expr(expr, Precedence::Choice)?;
                if !inner.is_empty() {
                    parts.push(format!("[ {inner} ]"));
                }
            }
        } else {
            let inner = self.emit_expr(expr, Precedence::Choice)?;
            if !inner.is_empty() {
                parts.push(format!("{{ {inner} }}"));
            }
        }

        Ok(parts.join(" , "))
    }

    fn emit_range_helper(&mut self, start: char, end: char) -> Result<String, GrammarEmitError> {
        let (name, is_new) = self
            .helpers
            .reserve("range", format!("{}:{}", start as u32, end as u32));
        if is_new {
            let body = expand_range(start, end)?
                .into_iter()
                .map(|character| quote_terminal(&character.to_string()))
                .collect::<Vec<_>>()
                .join(" | ");
            self.helpers.push(name.clone(), body);
        }
        Ok(name)
    }

    fn emit_char_class_helper(
        &mut self,
        negated: bool,
        items: &[CharClassItem],
    ) -> Result<String, GrammarEmitError> {
        if negated {
            return Err(unsupported_error(GrammarFormat::Ebnf, "negated CharClass"));
        }

        let (name, is_new) = self.helpers.reserve("class", format!("{items:?}"));
        if is_new {
            let chars = expand_class_items(items)?;
            if chars.is_empty() {
                return Err(unsupported_error(GrammarFormat::Ebnf, "empty CharClass"));
            }
            let body = chars
                .into_iter()
                .map(|character| quote_terminal(&character.to_string()))
                .collect::<Vec<_>>()
                .join(" | ");
            self.helpers.push(name.clone(), body);
        }
        Ok(name)
    }
}

fn expand_range(start: char, end: char) -> Result<Vec<char>, GrammarEmitError> {
    expanded_chars(
        GrammarFormat::Ebnf,
        "CharRange",
        start,
        end,
        MAX_EBNF_EXPANSION,
    )
}

fn expand_class_items(items: &[CharClassItem]) -> Result<Vec<char>, GrammarEmitError> {
    let mut chars = Vec::new();
    for item in items {
        match item {
            CharClassItem::Char(value) => chars.push(*value),
            CharClassItem::Range(start, end) => chars.extend(expand_range(*start, *end)?),
        }
        if chars.len() > MAX_EBNF_EXPANSION as usize {
            return Err(unsupported_error(
                GrammarFormat::Ebnf,
                format!("CharClass expands to more than {MAX_EBNF_EXPANSION} characters"),
            ));
        }
    }
    Ok(chars)
}

fn report_capture_loss(report: &mut EmitReport, format: GrammarFormat, label: Option<&String>) {
    if let Some(label) = label {
        report.add_lossy(format!("{format} dropped capture label {label:?}"));
    } else {
        report.add_lossy(format!("{format} dropped anonymous capture"));
    }
}

fn quote_terminal(value: &str) -> String {
    let quote = if value.contains('"') && !value.contains('\'') {
        '\''
    } else {
        '"'
    };
    let escaped = value.replace(quote, &quote.to_string().repeat(2));
    format!("{quote}{escaped}{quote}")
}
