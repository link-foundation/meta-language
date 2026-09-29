//! ANTLR v4 grammar emitter.

use std::collections::BTreeSet;
use std::fmt::Write as _;

use super::structural::{
    NamePlan, Precedence, Rendered, ascii_identifier, case_variants, collect_references,
    join_choice, join_sequence, lower_repeat, negated_lookahead_set, repeat_bounds, wrap,
};
use super::{EmitReport, GrammarEmitError, finish_lines, unsupported_error};
use crate::grammar::{
    CharClassItem, Grammar, GrammarExpr, GrammarFormat, GrammarRule, RuleKind, import_antlr,
};

const FORMAT: GrammarFormat = GrammarFormat::Antlr;
const LABEL: &str = "ANTLR";

/// Lower-case words ANTLR reserves, which cannot name parser rules or labels.
const RESERVED_WORDS: &[&str] = &[
    "catch", "channels", "finally", "fragment", "grammar", "import", "lexer", "locals", "mode",
    "options", "parser", "returns", "throws", "tokens",
];

/// Token names ANTLR predefines, which cannot name lexer rules.
const RESERVED_TOKENS: &[&str] = &["EOF"];

/// Emits a combined ANTLR v4 grammar.
///
/// Normal rules become parser rules (lower-case first letter), token and
/// atomic rules become lexer rules (upper-case first letter) and silent rules
/// become `fragment` lexer rules. Rule names are adjusted deterministically to
/// ANTLR's case conventions and reserved words; every rename is recorded in
/// [`EmitReport::lossy`] as `ANTLR renamed rule "a" to "b"`.
///
/// Ordered choice is written as ANTLR alternation, negative lookahead as the
/// `~` complement (a lookahead over a set followed by any character becomes a
/// complemented set), case-insensitive literals as per-letter sets, counted
/// repetition as copies with nested optionals, and captures as `label=`
/// element labels. Positive lookahead and anonymous captures have no ANTLR
/// form and are dropped. Every lowering and drop is recorded in the report,
/// so `import_antlr` of the output reproduces the grammar modulo the recorded
/// notes, and emitting the re-imported grammar reproduces the same text.
///
/// Rule documentation is written as comments above each rule; a trailing
/// `-> command` documentation part of a lexer rule, as produced by
/// `import_antlr`, is written back as the lexer command.
///
/// # Errors
///
/// Returns [`GrammarEmitError::Unsupported`] for empty non-negated character
/// classes, descending character ranges, repetitions whose maximum is below
/// their minimum, and counted repetitions that would expand to more than 256
/// copies of their operand.
pub fn emit_antlr(grammar: &Grammar) -> Result<(String, EmitReport), GrammarEmitError> {
    AntlrEmitter::new(grammar).emit()
}

#[derive(Debug)]
struct AntlrEmitter<'grammar> {
    grammar: &'grammar Grammar,
    names: NamePlan,
    report: EmitReport,
    foreign: bool,
    rule: String,
}

impl<'grammar> AntlrEmitter<'grammar> {
    fn new(grammar: &'grammar Grammar) -> Self {
        let mut report = EmitReport::default();
        let candidates = grammar
            .rules()
            .iter()
            .map(|rule| rule_candidate(&rule.name, is_lexer_kind(rule.kind)))
            .collect::<Vec<_>>();
        let names = NamePlan::new(
            grammar,
            LABEL,
            &candidates,
            &BTreeSet::new(),
            reference_candidate,
            &mut report,
        );
        Self {
            grammar,
            names,
            report,
            foreign: grammar.source_format() != Some(FORMAT),
            rule: String::new(),
        }
    }

    fn emit(mut self) -> Result<(String, EmitReport), GrammarEmitError> {
        let rules = self.grammar.rules();
        let order = self.emission_order();
        self.note_start(&order);

        let has_parser_rules = rules.iter().any(|rule| rule.kind == RuleKind::Normal);
        let grammar_name = order.first().map_or_else(String::new, |&index| {
            pascal_case(self.names.rule_name(index))
        });
        let grammar_name = if grammar_name.starts_with(|c: char| c.is_ascii_alphabetic()) {
            grammar_name
        } else {
            "Grammar".to_string()
        };
        let mut lines = vec![if has_parser_rules {
            format!("grammar {grammar_name};")
        } else {
            format!("lexer grammar {grammar_name};")
        }];
        if !order.is_empty() {
            lines.push(String::new());
        }
        for index in order {
            self.emit_rule(index, &mut lines)?;
        }
        Ok((finish_lines(&lines), self.report))
    }

    /// Orders rules so that `import_antlr` picks the same start rule: the
    /// importer starts at the first parser rule, or the first rule when there
    /// are no parser rules.
    fn emission_order(&self) -> Vec<usize> {
        let rules = self.grammar.rules();
        let start_index = self
            .grammar
            .start()
            .and_then(|start| rules.iter().position(|rule| rule.name == start));
        let effective = match start_index {
            Some(index) if rules[index].kind == RuleKind::Normal => Some(index),
            _ => rules
                .iter()
                .position(|rule| rule.kind == RuleKind::Normal)
                .or(start_index),
        };
        let mut order = effective.into_iter().collect::<Vec<_>>();
        order.extend((0..rules.len()).filter(|&index| Some(index) != effective));
        order
    }

    fn note_start(&mut self, order: &[usize]) {
        let Some(start) = self.grammar.start() else {
            return;
        };
        let Some(&first) = order.first() else {
            return;
        };
        let expected = self.names.resolve(start).to_string();
        let actual = self.names.rule_name(first).to_string();
        if expected != actual {
            self.report.add_lossy(format!(
                "ANTLR re-imports start rule as {actual:?} instead of {expected:?}, \
                 because ANTLR starts at the first parser rule"
            ));
        }
    }

    fn emit_rule(&mut self, index: usize, lines: &mut Vec<String>) -> Result<(), GrammarEmitError> {
        let rule = &self.grammar.rules()[index];
        let name = self.names.rule_name(index).to_string();
        self.rule.clone_from(&name);
        let lexer = is_lexer_kind(rule.kind);
        self.note_kind(rule, &name);

        let doc = plan_doc(rule.doc(), lexer);
        if doc.reimported.as_deref() != rule.doc() {
            self.report.add_lossy(format!(
                "ANTLR re-imports the documentation of rule {name:?} as {:?}",
                doc.reimported
            ));
        }
        lines.extend(doc.comments);

        let body = self.render(rule.expr())?.0;
        let mut line = if rule.kind == RuleKind::Silent {
            format!("fragment {name} :")
        } else {
            format!("{name} :")
        };
        if !body.is_empty() {
            line.push(' ');
            line.push_str(&body);
        }
        if let Some(command) = doc.command {
            line.push(' ');
            line.push_str(&command);
        }
        line.push_str(" ;");
        lines.push(line);

        if self.foreign {
            self.note_rule_placement(rule, &name, lexer);
        }
        Ok(())
    }

    fn note_kind(&mut self, rule: &GrammarRule, name: &str) {
        match rule.kind {
            RuleKind::Atomic => self.report.add_lossy(format!(
                "ANTLR emitted atomic rule {:?} as lexer rule {name:?}, which re-imports as a token rule",
                rule.name
            )),
            RuleKind::Silent if self.foreign => self.report.add_lossy(format!(
                "ANTLR emitted silent rule {:?} as lexer fragment {name:?}",
                rule.name
            )),
            RuleKind::Normal | RuleKind::Silent | RuleKind::Token => {}
        }
    }

    /// Records rules that real ANTLR would reject: lexer rules referring to
    /// parser rules, parser rules referring to fragments, and parser rules
    /// using character-level constructs.
    fn note_rule_placement(&mut self, rule: &GrammarRule, name: &str, lexer: bool) {
        let mut references = Vec::new();
        collect_references(rule.expr(), &mut references);
        for reference in references {
            let Some(target) = self.grammar.rule(reference) else {
                continue;
            };
            let emitted = self.names.resolve(reference).to_string();
            if lexer && target.kind == RuleKind::Normal {
                self.report.add_lossy(format!(
                    "ANTLR lexer rule {name:?} references parser rule {emitted:?}"
                ));
            } else if !lexer && target.kind == RuleKind::Silent {
                self.report.add_lossy(format!(
                    "ANTLR parser rule {name:?} references lexer fragment {emitted:?}"
                ));
            }
        }
        if !lexer && uses_character_constructs(rule.expr()) {
            self.report.add_lossy(format!(
                "ANTLR parser rule {name:?} uses character-level constructs that ANTLR matches only in lexer rules"
            ));
        }
    }

    fn note(&mut self, message: &str) {
        let note = format!("ANTLR {message} in rule {:?}", self.rule);
        self.report.add_lossy(note);
    }

    fn render(&mut self, expr: &GrammarExpr) -> Result<Rendered, GrammarEmitError> {
        Ok(match expr {
            GrammarExpr::Empty => (String::new(), Precedence::Sequence),
            GrammarExpr::Terminal(value) => {
                if value.is_empty() {
                    self.note("keeps an empty literal, which the ANTLR tool rejects,");
                }
                (quote(value), Precedence::Atom)
            }
            GrammarExpr::TerminalInsensitive(value) => self.render_insensitive(value)?,
            GrammarExpr::CharRange(start, end) => {
                if start > end {
                    return Err(unsupported_error(
                        FORMAT,
                        format!("descending character range {start:?}..{end:?}"),
                    ));
                }
                (
                    format!("{}..{}", quote(&start.to_string()), quote(&end.to_string())),
                    Precedence::Atom,
                )
            }
            GrammarExpr::CharClass { negated, items } => self.render_class(*negated, items)?,
            GrammarExpr::AnyChar => (".".to_string(), Precedence::Atom),
            GrammarExpr::NonTerminal(name) => {
                (self.names.resolve(name).to_string(), Precedence::Atom)
            }
            GrammarExpr::Choice {
                ordered,
                alternatives,
            } => {
                if *ordered && alternatives.len() > 1 {
                    self.note("treats ordered choice as unordered choice");
                }
                let rendered = alternatives
                    .iter()
                    .map(|alternative| self.render(alternative))
                    .collect::<Result<Vec<_>, _>>()?;
                join_choice(rendered)
            }
            GrammarExpr::Sequence(items) => self.render_sequence(items)?,
            GrammarExpr::Optional(inner) => self.render_postfix(inner, "?")?,
            GrammarExpr::ZeroOrMore(inner) => self.render_postfix(inner, "*")?,
            GrammarExpr::OneOrMore(inner) => self.render_postfix(inner, "+")?,
            GrammarExpr::Repeat { expr, min, max } => {
                let lowered = lower_repeat(FORMAT, expr, *min, *max)?;
                self.note(&format!(
                    "expanded counted repetition {}",
                    repeat_bounds(*min, *max)
                ));
                self.render(&lowered)?
            }
            GrammarExpr::And(_) => {
                self.note("dropped positive lookahead");
                (String::new(), Precedence::Sequence)
            }
            GrammarExpr::Not(inner) => self.render_not(inner)?,
            GrammarExpr::Capture { label, expr } => self.render_capture(label.as_deref(), expr)?,
        })
    }

    fn render_sequence(&mut self, items: &[GrammarExpr]) -> Result<Rendered, GrammarEmitError> {
        let mut rendered = Vec::with_capacity(items.len());
        let mut index = 0;
        while index < items.len() {
            if self.foreign
                && let Some((negated, class)) = negated_lookahead_set(items, index)
            {
                self.note(
                    "lowered negative lookahead followed by any character to a complemented set",
                );
                rendered.push(render_set(negated, &class));
                index += 2;
                continue;
            }
            rendered.push(self.render(&items[index])?);
            index += 1;
        }
        Ok(join_sequence(rendered))
    }

    fn render_postfix(
        &mut self,
        inner: &GrammarExpr,
        operator: &str,
    ) -> Result<Rendered, GrammarEmitError> {
        let operand = self.render(inner)?;
        Ok((
            format!("{}{operator}", wrap(operand, Precedence::Prefix)),
            Precedence::Postfix,
        ))
    }

    fn render_not(&mut self, inner: &GrammarExpr) -> Result<Rendered, GrammarEmitError> {
        if matches!(inner, GrammarExpr::CharClass { negated: false, .. }) {
            self.note("emitted negative lookahead over a character set as a complemented set");
        } else if self.foreign {
            self.note(
                "emitted negative lookahead as the `~` complement, which consumes one symbol",
            );
        }
        let operand = self.render(inner)?;
        Ok((
            format!("~{}", wrap(operand, Precedence::Atom)),
            Precedence::Prefix,
        ))
    }

    fn render_capture(
        &mut self,
        label: Option<&str>,
        inner: &GrammarExpr,
    ) -> Result<Rendered, GrammarEmitError> {
        let Some(label) = label else {
            self.note("dropped anonymous capture");
            return self.render(inner);
        };
        if label == "non_greedy"
            && matches!(
                inner,
                GrammarExpr::Optional(_) | GrammarExpr::ZeroOrMore(_) | GrammarExpr::OneOrMore(_)
            )
        {
            let (text, _precedence) = self.render(inner)?;
            return Ok((format!("{text}?"), Precedence::Postfix));
        }
        let emitted = label_candidate(label);
        if emitted != label {
            self.note(&format!("renamed capture label {label:?} to {emitted:?}"));
        }
        if label == "regex"
            && self.grammar.source_format() == Some(GrammarFormat::Lark)
            && let GrammarExpr::Terminal(pattern) = inner
        {
            self.note(&format!(
                "emitted regex /{pattern}/ as a literal labelled `regex`; regex semantics are not translated"
            ));
        }
        let operand = self.render(inner)?;
        Ok((
            format!("{emitted}={}", wrap(operand, Precedence::Postfix)),
            Precedence::Labeled,
        ))
    }

    fn render_insensitive(&mut self, value: &str) -> Result<Rendered, GrammarEmitError> {
        if value.is_empty() {
            return self.render(&GrammarExpr::Terminal(String::new()));
        }
        let mut items = Vec::new();
        let mut literal = String::new();
        for character in value.chars() {
            if let Some(variants) = case_variants(character) {
                if !literal.is_empty() {
                    items.push(GrammarExpr::Terminal(std::mem::take(&mut literal)));
                }
                items.push(GrammarExpr::CharClass {
                    negated: false,
                    items: variants.into_iter().map(CharClassItem::Char).collect(),
                });
            } else {
                literal.push(character);
            }
        }
        if !literal.is_empty() {
            items.push(GrammarExpr::Terminal(literal));
        }
        self.note(&format!(
            "expanded case-insensitive literal {value:?} to per-letter character sets"
        ));
        let lowered = if items.len() == 1 {
            items.remove(0)
        } else {
            GrammarExpr::Sequence(items)
        };
        self.render(&lowered)
    }

    fn render_class(
        &mut self,
        negated: bool,
        items: &[CharClassItem],
    ) -> Result<Rendered, GrammarEmitError> {
        if items.is_empty() {
            if negated {
                self.note("emitted an empty negated character class as `.`");
                return Ok((".".to_string(), Precedence::Atom));
            }
            return Err(unsupported_error(FORMAT, "empty character class"));
        }
        if let Some(CharClassItem::Range(start, end)) = items
            .iter()
            .find(|item| matches!(item, CharClassItem::Range(start, end) if start > end))
        {
            return Err(unsupported_error(
                FORMAT,
                format!("descending character class range {start:?}-{end:?}"),
            ));
        }
        Ok(render_set(negated, items))
    }
}

const fn is_lexer_kind(kind: RuleKind) -> bool {
    !matches!(kind, RuleKind::Normal)
}

/// Returns ANTLR's conventional spelling of a rule name.
fn rule_candidate(name: &str, lexer: bool) -> String {
    let mut output = ascii_identifier(name);
    match output.chars().next() {
        Some(first) if first.is_ascii_alphabetic() => {
            let fixed = if lexer {
                first.to_ascii_uppercase()
            } else {
                first.to_ascii_lowercase()
            };
            output.replace_range(..1, fixed.encode_utf8(&mut [0; 4]));
        }
        _ => output.insert(0, if lexer { 'T' } else { 'r' }),
    }
    let reserved = if lexer {
        RESERVED_TOKENS
    } else {
        RESERVED_WORDS
    };
    if reserved.contains(&output.as_str()) {
        output.push('_');
    }
    output
}

/// Returns a valid spelling for a reference without a rule, keeping its case.
fn reference_candidate(name: &str) -> String {
    let lexer = name.starts_with(|c: char| c.is_ascii_uppercase());
    let mut output = ascii_identifier(name);
    if !output.starts_with(|c: char| c.is_ascii_alphabetic()) {
        output.insert(0, if lexer { 'T' } else { 'r' });
    }
    if RESERVED_WORDS.contains(&output.as_str()) {
        output.push('_');
    }
    output
}

fn label_candidate(label: &str) -> String {
    let mut output = ascii_identifier(label);
    if !output.starts_with(|c: char| c.is_ascii_alphabetic() || c == '_') {
        output.insert(0, 'l');
    }
    if RESERVED_WORDS.contains(&output.as_str()) {
        output.push('_');
    }
    output
}

fn pascal_case(name: &str) -> String {
    name.split('_')
        .filter(|part| !part.is_empty())
        .map(|part| {
            let mut chars = part.chars();
            chars.next().map_or_else(String::new, |first| {
                first.to_ascii_uppercase().to_string() + chars.as_str()
            })
        })
        .collect()
}

fn uses_character_constructs(expr: &GrammarExpr) -> bool {
    match expr {
        GrammarExpr::TerminalInsensitive(_)
        | GrammarExpr::CharRange(_, _)
        | GrammarExpr::CharClass { .. }
        | GrammarExpr::AnyChar => true,
        GrammarExpr::Choice { alternatives, .. } => {
            alternatives.iter().any(uses_character_constructs)
        }
        GrammarExpr::Sequence(items) => items.iter().any(uses_character_constructs),
        GrammarExpr::Optional(inner)
        | GrammarExpr::ZeroOrMore(inner)
        | GrammarExpr::OneOrMore(inner)
        | GrammarExpr::And(inner)
        | GrammarExpr::Not(inner)
        | GrammarExpr::Repeat { expr: inner, .. }
        | GrammarExpr::Capture { expr: inner, .. } => uses_character_constructs(inner),
        GrammarExpr::Empty | GrammarExpr::Terminal(_) | GrammarExpr::NonTerminal(_) => false,
    }
}

/// Quotes an ANTLR string literal.
fn quote(value: &str) -> String {
    let mut output = String::from("'");
    for character in value.chars() {
        match character {
            '\'' => output.push_str("\\'"),
            '\\' => output.push_str("\\\\"),
            '\n' => output.push_str("\\n"),
            '\r' => output.push_str("\\r"),
            '\t' => output.push_str("\\t"),
            '\u{08}' => output.push_str("\\b"),
            '\u{0c}' => output.push_str("\\f"),
            character if character.is_control() => {
                let _ = write!(output, "\\u{:04X}", u32::from(character));
            }
            character => output.push(character),
        }
    }
    output.push('\'');
    output
}

/// Renders a lexer set as `[...]`, or `~[...]` when negated.
fn render_set(negated: bool, items: &[CharClassItem]) -> Rendered {
    let mut body = String::new();
    for item in items {
        match item {
            CharClassItem::Char(character) => push_set_char(&mut body, *character),
            CharClassItem::Range(start, end) => {
                push_set_char(&mut body, *start);
                body.push('-');
                push_set_char(&mut body, *end);
            }
        }
    }
    if negated {
        (format!("~[{body}]"), Precedence::Prefix)
    } else {
        (format!("[{body}]"), Precedence::Atom)
    }
}

fn push_set_char(body: &mut String, character: char) {
    match character {
        '\\' | ']' | '-' => {
            body.push('\\');
            body.push(character);
        }
        '^' if body.is_empty() => body.push_str("\\^"),
        '\n' => body.push_str("\\n"),
        '\r' => body.push_str("\\r"),
        '\t' => body.push_str("\\t"),
        '\u{08}' => body.push_str("\\b"),
        '\u{0c}' => body.push_str("\\f"),
        character => body.push(character),
    }
}

/// Comments, lexer command and re-imported documentation for one rule.
#[derive(Debug)]
struct DocPlan {
    comments: Vec<String>,
    command: Option<String>,
    reimported: Option<String>,
}

/// Plans how rule documentation is written so that `import_antlr` reads the
/// same documentation back whenever it consists of comments and a lexer
/// command, as `import_antlr` produces it.
fn plan_doc(doc: Option<&str>, lexer: bool) -> DocPlan {
    let Some(doc) = doc else {
        return DocPlan {
            comments: Vec::new(),
            command: None,
            reimported: None,
        };
    };
    let (text, command) = if lexer {
        split_command(doc)
    } else {
        (doc, None)
    };
    let comments = comment_lines(text);
    let mut parts = comments
        .iter()
        .map(|comment| comment.trim().to_string())
        .collect::<Vec<_>>();
    parts.extend(command.clone());
    DocPlan {
        comments,
        command,
        reimported: (!parts.is_empty()).then(|| parts.join("; ")),
    }
}

/// Splits a trailing `-> command` part off lexer rule documentation when the
/// command re-imports verbatim.
fn split_command(doc: &str) -> (&str, Option<String>) {
    let (text, command) = if doc.starts_with("->") {
        ("", doc)
    } else if let Some(position) = doc.rfind("; ->") {
        (&doc[..position], &doc[position + 2..])
    } else {
        return (doc, None);
    };
    let probe = format!("A : 'a' {command} ;");
    let round_trips = !command.contains(['\n', '\r'])
        && import_antlr(&probe)
            .ok()
            .and_then(|grammar| {
                grammar
                    .rule("A")
                    .and_then(GrammarRule::doc)
                    .map(str::to_string)
            })
            .is_some_and(|reimported| reimported == command);
    if round_trips {
        (text, Some(command.to_string()))
    } else {
        (doc, None)
    }
}

/// Writes documentation as ANTLR comments. Parts that already are comments
/// are written verbatim; other text becomes `//` line comments.
fn comment_lines(text: &str) -> Vec<String> {
    let mut lines = Vec::new();
    for part in split_comment_parts(text) {
        let block = part.len() >= 4
            && part.starts_with("/*")
            && part[2..].find("*/") == Some(part.len() - 4);
        let line = part.starts_with("//") && !part.contains(['\n', '\r']);
        if block || line {
            lines.push(part.to_string());
        } else {
            lines.extend(super::structural::doc_lines(part).map(|line| {
                if line.starts_with("//") {
                    line.to_string()
                } else {
                    format!("// {line}")
                }
            }));
        }
    }
    lines
}

/// Splits documentation at `; ` separators that precede a comment.
fn split_comment_parts(text: &str) -> Vec<&str> {
    let mut parts = Vec::new();
    let mut rest = text;
    loop {
        let split = ["; //", "; /*"]
            .iter()
            .filter_map(|separator| rest.find(separator))
            .min();
        let Some(position) = split else {
            parts.push(rest);
            break;
        };
        parts.push(&rest[..position]);
        rest = &rest[position + 2..];
    }
    parts.retain(|part| !part.trim().is_empty());
    parts
}
