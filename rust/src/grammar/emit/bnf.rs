use crate::grammar::{CharClassItem, Grammar, GrammarExpr, GrammarFormat};

use super::{
    expanded_chars, finish_lines, ordered_rules, render_rule_line, unsupported_error, EmitReport,
    GrammarEmitError, HelperRules, BNF_RULE_TEMPLATE,
};

const MAX_BNF_EXPANSION: u32 = 256;

/// Emits classic Backus-Naur Form text from the grammar IR.
///
/// BNF has no native grouping, optional, repetition, character range, or
/// character class operators, so this emitter synthesizes deterministic helper
/// productions when a faithful expansion is possible.
///
/// # Errors
///
/// Returns [`GrammarEmitError`] when the grammar contains a construct BNF cannot
/// represent, such as PEG predicates, negated character classes, `AnyChar`, or a
/// character range too large to expand safely.
pub fn emit_bnf(grammar: &Grammar) -> Result<(String, EmitReport), GrammarEmitError> {
    let mut emitter = BnfEmitter::new(grammar);
    let mut lines = Vec::new();

    for rule in ordered_rules(grammar) {
        let body = emitter.emit_expr(rule.expr(), BnfContext::Production)?;
        lines.push(render_rule_line(BNF_RULE_TEMPLATE, rule.name(), &body));
    }
    for helper in emitter.helpers.entries() {
        lines.push(render_rule_line(
            BNF_RULE_TEMPLATE,
            &helper.name,
            &helper.body,
        ));
    }

    Ok((finish_lines(&lines), emitter.report))
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum BnfContext {
    Production,
    SequenceItem,
}

#[derive(Clone, Debug)]
struct BnfEmitter {
    report: EmitReport,
    helpers: HelperRules,
}

impl BnfEmitter {
    fn new(grammar: &Grammar) -> Self {
        Self {
            report: EmitReport::default(),
            helpers: HelperRules::new(grammar),
        }
    }

    fn emit_expr(
        &mut self,
        expr: &GrammarExpr,
        context: BnfContext,
    ) -> Result<String, GrammarEmitError> {
        match expr {
            GrammarExpr::Empty => Ok(String::new()),
            GrammarExpr::Terminal(value) => Ok(quote_terminal(value)),
            GrammarExpr::TerminalInsensitive(value) => {
                self.report.add_lossy(format!(
                    "BNF cannot preserve case-insensitive terminal {value:?}"
                ));
                Ok(quote_terminal(value))
            }
            GrammarExpr::CharRange(start, end) => self.emit_range_helper(*start, *end),
            GrammarExpr::CharClass { negated, items } => {
                self.emit_char_class_helper(*negated, items)
            }
            GrammarExpr::AnyChar => Err(unsupported_error(GrammarFormat::Bnf, "AnyChar")),
            GrammarExpr::NonTerminal(name) => Ok(nonterminal(name)),
            GrammarExpr::Choice {
                ordered,
                alternatives,
            } => self.emit_choice(*ordered, alternatives, context),
            GrammarExpr::Sequence(items) => self.emit_sequence(items),
            GrammarExpr::Optional(inner) => self.emit_optional_helper(inner),
            GrammarExpr::ZeroOrMore(inner) => self.emit_star_helper(inner),
            GrammarExpr::OneOrMore(inner) => self.emit_plus_helper(inner),
            GrammarExpr::Repeat { expr, min, max } => self.emit_repeat(expr, *min, *max),
            GrammarExpr::And(_) => Err(unsupported_error(GrammarFormat::Bnf, "And")),
            GrammarExpr::Not(_) => Err(unsupported_error(GrammarFormat::Bnf, "Not")),
            GrammarExpr::Capture { label, expr } => {
                report_capture_loss(&mut self.report, GrammarFormat::Bnf, label.as_ref());
                self.emit_expr(expr, context)
            }
        }
    }

    fn emit_choice(
        &mut self,
        ordered: bool,
        alternatives: &[GrammarExpr],
        context: BnfContext,
    ) -> Result<String, GrammarEmitError> {
        if ordered {
            self.report
                .add_lossy("BNF treats ordered choice as unordered choice");
        }
        if context == BnfContext::SequenceItem {
            return self.emit_choice_helper(&GrammarExpr::Choice {
                ordered,
                alternatives: alternatives.to_vec(),
            });
        }

        alternatives
            .iter()
            .map(|alternative| self.emit_expr(alternative, BnfContext::Production))
            .collect::<Result<Vec<_>, _>>()
            .map(|items| items.join(" | "))
    }

    fn emit_sequence(&mut self, items: &[GrammarExpr]) -> Result<String, GrammarEmitError> {
        let mut emitted = Vec::new();
        for item in items {
            let text = self.emit_expr(item, BnfContext::SequenceItem)?;
            if !text.is_empty() {
                emitted.push(text);
            }
        }
        Ok(emitted.join(" "))
    }

    fn emit_repeat(
        &mut self,
        expr: &GrammarExpr,
        min: usize,
        max: Option<usize>,
    ) -> Result<String, GrammarEmitError> {
        if max.is_some_and(|max| max < min) {
            return Err(unsupported_error(
                GrammarFormat::Bnf,
                format!("Repeat with min {min} greater than max {max:?}"),
            ));
        }

        let mut parts = Vec::new();
        for _ in 0..min {
            parts.push(self.emit_expr(expr, BnfContext::SequenceItem)?);
        }

        match max {
            Some(max) => {
                for _ in min..max {
                    parts.push(self.emit_optional_helper(expr)?);
                }
            }
            None => parts.push(self.emit_star_helper(expr)?),
        }

        Ok(parts
            .into_iter()
            .filter(|part| !part.is_empty())
            .collect::<Vec<_>>()
            .join(" "))
    }

    fn emit_choice_helper(&mut self, expr: &GrammarExpr) -> Result<String, GrammarEmitError> {
        let (name, is_new) = self.helpers.reserve("choice", format!("{expr:?}"));
        if is_new {
            let body = self.emit_expr(expr, BnfContext::Production)?;
            self.helpers.push(name.clone(), body);
        }
        Ok(nonterminal(&name))
    }

    fn emit_optional_helper(&mut self, expr: &GrammarExpr) -> Result<String, GrammarEmitError> {
        if matches!(expr, GrammarExpr::Empty) {
            return Ok(String::new());
        }

        let (name, is_new) = self.helpers.reserve("opt", format!("{expr:?}"));
        if is_new {
            let body = format!("{} |", self.emit_expr(expr, BnfContext::Production)?);
            self.helpers.push(name.clone(), body);
        }
        Ok(nonterminal(&name))
    }

    fn emit_star_helper(&mut self, expr: &GrammarExpr) -> Result<String, GrammarEmitError> {
        if matches!(expr, GrammarExpr::Empty) {
            return Ok(String::new());
        }

        let (name, is_new) = self.helpers.reserve("star", format!("{expr:?}"));
        if is_new {
            let item = self.emit_expr(expr, BnfContext::SequenceItem)?;
            let body = if item.is_empty() {
                String::new()
            } else {
                format!("{item} {} |", nonterminal(&name))
            };
            self.helpers.push(name.clone(), body);
        }
        Ok(nonterminal(&name))
    }

    fn emit_plus_helper(&mut self, expr: &GrammarExpr) -> Result<String, GrammarEmitError> {
        if matches!(expr, GrammarExpr::Empty) {
            return Ok(String::new());
        }

        let (name, is_new) = self.helpers.reserve("plus", format!("{expr:?}"));
        if is_new {
            let item = self.emit_expr(expr, BnfContext::SequenceItem)?;
            let body = if item.is_empty() {
                String::new()
            } else {
                format!("{item} {} | {item}", nonterminal(&name))
            };
            self.helpers.push(name.clone(), body);
        }
        Ok(nonterminal(&name))
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
        Ok(nonterminal(&name))
    }

    fn emit_char_class_helper(
        &mut self,
        negated: bool,
        items: &[CharClassItem],
    ) -> Result<String, GrammarEmitError> {
        if negated {
            return Err(unsupported_error(GrammarFormat::Bnf, "negated CharClass"));
        }

        let (name, is_new) = self.helpers.reserve("class", format!("{items:?}"));
        if is_new {
            let chars = expand_class_items(items)?;
            if chars.is_empty() {
                return Err(unsupported_error(GrammarFormat::Bnf, "empty CharClass"));
            }
            let body = chars
                .into_iter()
                .map(|character| quote_terminal(&character.to_string()))
                .collect::<Vec<_>>()
                .join(" | ");
            self.helpers.push(name.clone(), body);
        }
        Ok(nonterminal(&name))
    }
}

fn expand_range(start: char, end: char) -> Result<Vec<char>, GrammarEmitError> {
    expanded_chars(
        GrammarFormat::Bnf,
        "CharRange",
        start,
        end,
        MAX_BNF_EXPANSION,
    )
}

fn expand_class_items(items: &[CharClassItem]) -> Result<Vec<char>, GrammarEmitError> {
    let mut chars = Vec::new();
    for item in items {
        match item {
            CharClassItem::Char(value) => chars.push(*value),
            CharClassItem::Range(start, end) => chars.extend(expand_range(*start, *end)?),
        }
        if chars.len() > MAX_BNF_EXPANSION as usize {
            return Err(unsupported_error(
                GrammarFormat::Bnf,
                format!("CharClass expands to more than {MAX_BNF_EXPANSION} characters"),
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

fn nonterminal(name: &str) -> String {
    format!("<{name}>")
}

fn quote_terminal(value: &str) -> String {
    let mut output = String::with_capacity(value.len() + 2);
    output.push('"');
    for character in value.chars() {
        match character {
            '"' => output.push_str("\\\""),
            '\\' => output.push_str("\\\\"),
            other => output.push(other),
        }
    }
    output.push('"');
    output
}
