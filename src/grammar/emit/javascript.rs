use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;

use crate::grammar::{CharClassItem, Grammar, GrammarExpr, GrammarFormat, RuleKind};

use super::{finish_lines, ordered_rules, unsupported_error, EmitReport, GrammarEmitError};

/// Bundled JavaScript parser codegen output.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct JsParserArtifacts {
    /// Peggy grammar source.
    pub peggy_grammar: String,
    /// ESM wrapper module that compiles and exports a Peggy parser.
    pub module: String,
}

/// Emits Peggy PEG grammar text from the grammar IR.
///
/// Peggy maps directly to the IR's PEG-oriented expression algebra: ordered
/// choice uses `/`, sequences are whitespace-separated, predicates use `&` and
/// `!`, labels use `label:expr`, and counted repetitions use Peggy's
/// `expr|min..max|` form.
///
/// # Errors
///
/// Returns [`GrammarEmitError`] when the IR contains an internal invariant that
/// cannot form valid Peggy text, such as an empty choice, an empty character
/// class, a descending character range, or counted repetition with `max < min`.
pub fn emit_peggy(grammar: &Grammar) -> Result<(String, EmitReport), GrammarEmitError> {
    let mut report = EmitReport::default();
    let names = NamePlan::new(grammar, &mut report);
    let mut emitter = PeggyEmitter { report, names };
    let mut lines = Vec::new();

    for rule in ordered_rules(grammar) {
        if let Some(doc) = rule.doc() {
            push_doc_lines(&mut lines, doc);
        }
        if let Some(comment) = emitter.names.rule_rename_comment(rule.name()) {
            lines.push(comment);
        }
        if contains_unordered_choice(rule.expr()) {
            lines.push(
                "// NOTE: unordered choice in source is emitted as ordered Peggy choice."
                    .to_string(),
            );
        }
        if let Some(comment) = rule_kind_comment(rule.kind()) {
            lines.push(comment.to_string());
        }

        let body = emitter.emit_expr(rule.expr(), Precedence::Choice)?;
        let body = apply_rule_kind(rule.kind(), body, &mut emitter.report);
        let name = emitter.names.name_for(rule.name());
        lines.push(format!("{name} = {body}"));
    }

    Ok((finish_lines(&lines), emitter.report))
}

/// Emits a runnable JavaScript parser bundle from the grammar IR.
///
/// The generated ESM module imports `peggy`, embeds the emitted grammar as a
/// JavaScript string literal, and exports `parser = peggy.generate(GRAMMAR)`.
///
/// # Errors
///
/// Returns [`GrammarEmitError`] when the underlying Peggy grammar emitter cannot
/// represent an invalid IR construct.
pub fn emit_javascript_parser(
    grammar: &Grammar,
) -> Result<(JsParserArtifacts, EmitReport), GrammarEmitError> {
    let (peggy_grammar, report) = emit_peggy(grammar)?;
    let module = render_javascript_module(&peggy_grammar);

    Ok((
        JsParserArtifacts {
            peggy_grammar,
            module,
        },
        report,
    ))
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Precedence {
    Choice = 0,
    Sequence = 1,
    Prefix = 2,
    Postfix = 3,
    Atom = 4,
}

#[derive(Clone, Debug)]
struct PeggyEmitter {
    report: EmitReport,
    names: NamePlan,
}

impl PeggyEmitter {
    fn emit_expr(
        &mut self,
        expr: &GrammarExpr,
        parent: Precedence,
    ) -> Result<String, GrammarEmitError> {
        let (text, precedence) = match expr {
            GrammarExpr::Empty => (quote_js_string(""), Precedence::Atom),
            GrammarExpr::Terminal(value) => (quote_js_string(value), Precedence::Atom),
            GrammarExpr::TerminalInsensitive(value) => {
                (format!("{}i", quote_js_string(value)), Precedence::Atom)
            }
            GrammarExpr::CharRange(start, end) => {
                (emit_char_range(*start, *end)?, Precedence::Atom)
            }
            GrammarExpr::CharClass { negated, items } => {
                (emit_char_class(*negated, items)?, Precedence::Atom)
            }
            GrammarExpr::AnyChar => (".".to_string(), Precedence::Atom),
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
                let Some(label) = label else {
                    return self.emit_expr(expr, parent);
                };
                let label = self.emit_label(label);
                let inner = self.emit_expr(expr, Precedence::Atom)?;
                (format!("{label}:{inner}"), Precedence::Prefix)
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
                .add_lossy("Peggy treats unordered choice as ordered choice");
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
            Ok(quote_js_string(""))
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
                GrammarFormat::Peg,
                format!("Repeat with min {min} greater than max {max:?}"),
            ));
        }

        let inner = self.emit_expr(expr, Precedence::Postfix)?;
        let suffix = match max {
            Some(max) if min == max => format!("|{min}|"),
            Some(max) => format!("|{min}..{max}|"),
            None => format!("|{min}..|"),
        };
        Ok(format!("{inner}{suffix}"))
    }

    fn emit_label(&mut self, label: &str) -> String {
        let emitted = sanitize_identifier(label);
        if emitted != label {
            self.report.add_lossy(format!(
                "Peggy renamed capture label {label:?} to {emitted:?}"
            ));
        }
        emitted
    }
}

#[derive(Clone, Debug)]
struct NamePlan {
    names: BTreeMap<String, String>,
}

impl NamePlan {
    fn new(grammar: &Grammar, report: &mut EmitReport) -> Self {
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

        let mut used = BTreeSet::new();
        let mut names = BTreeMap::new();
        for symbol in symbols {
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

    fn rule_rename_comment(&self, source: &str) -> Option<String> {
        let emitted = self.names.get(source)?;
        (emitted != source).then(|| {
            format!(
                "// NOTE: rule {source:?} is emitted as {emitted:?} for Peggy identifier syntax."
            )
        })
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
    report.add_lossy(format!("Peggy renamed {kind} {source:?} to {emitted:?}"));
}

fn unique_identifier(base: &str, used: &mut BTreeSet<String>) -> String {
    if used.insert(base.to_string()) {
        return base.to_string();
    }

    let mut suffix = 1_usize;
    loop {
        let candidate = format!("{base}_{suffix}");
        if used.insert(candidate.clone()) {
            return candidate;
        }
        suffix = suffix.saturating_add(1);
    }
}

fn sanitize_identifier(source: &str) -> String {
    let mut output = String::new();
    for character in source.chars() {
        if output.is_empty() {
            if is_identifier_start(character) {
                output.push(character);
            } else if is_identifier_continue(character) {
                output.push('_');
                output.push(character);
            } else {
                output.push('_');
            }
        } else if is_identifier_continue(character) {
            output.push(character);
        } else {
            output.push('_');
        }
    }

    if output.is_empty() {
        output.push_str("ml");
    }
    if is_reserved_identifier(&output) {
        output.insert_str(0, "ml_");
    }
    output
}

const fn is_identifier_start(character: char) -> bool {
    character.is_ascii_alphabetic() || character == '_'
}

const fn is_identifier_continue(character: char) -> bool {
    character.is_ascii_alphanumeric() || character == '_' || character == '$'
}

fn is_reserved_identifier(value: &str) -> bool {
    matches!(
        value,
        "arguments"
            | "await"
            | "break"
            | "case"
            | "catch"
            | "class"
            | "const"
            | "continue"
            | "debugger"
            | "default"
            | "delete"
            | "do"
            | "else"
            | "enum"
            | "eval"
            | "export"
            | "extends"
            | "false"
            | "finally"
            | "for"
            | "function"
            | "if"
            | "implements"
            | "import"
            | "in"
            | "instanceof"
            | "interface"
            | "let"
            | "new"
            | "null"
            | "package"
            | "private"
            | "protected"
            | "public"
            | "return"
            | "static"
            | "super"
            | "switch"
            | "this"
            | "throw"
            | "true"
            | "try"
            | "typeof"
            | "var"
            | "void"
            | "while"
            | "with"
            | "yield"
    )
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
        return Err(unsupported_error(GrammarFormat::Peg, "empty CharClass"));
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
            GrammarFormat::Peg,
            format!(
                "{construct} has descending bounds U+{:04X}..=U+{:04X}",
                start as u32, end as u32
            ),
        ));
    }
    Ok(())
}

fn apply_rule_kind(kind: RuleKind, body: String, report: &mut EmitReport) -> String {
    match kind {
        RuleKind::Normal => body,
        RuleKind::Atomic => {
            report.add_lossy("Peggy has no RuleKind::Atomic modifier; emitted a text() action");
            format!("({body}) {{ return text(); }}")
        }
        RuleKind::Silent => {
            report.add_lossy("Peggy has no RuleKind::Silent modifier; emitted a normal rule");
            body
        }
        RuleKind::Token => {
            report.add_lossy("Peggy has no RuleKind::Token modifier; emitted a text() action");
            format!("({body}) {{ return text(); }}")
        }
    }
}

const fn rule_kind_comment(kind: RuleKind) -> Option<&'static str> {
    match kind {
        RuleKind::Normal => None,
        RuleKind::Atomic => {
            Some("// NOTE: Peggy has no atomic rule modifier; this rule returns its matched text.")
        }
        RuleKind::Silent => Some(
            "// NOTE: Peggy has no silent rule modifier; this rule is emitted as a normal rule.",
        ),
        RuleKind::Token => {
            Some("// NOTE: Peggy has no token rule modifier; this rule returns its matched text.")
        }
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

fn render_javascript_module(peggy_grammar: &str) -> String {
    format!(
        "import peggy from \"peggy\";\n\nconst GRAMMAR = {};\nexport const parser = peggy.generate(GRAMMAR);\n",
        quote_js_string(peggy_grammar)
    )
}

fn quote_js_string(value: &str) -> String {
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
        '\u{2028}' => output.push_str("\\u2028"),
        '\u{2029}' => output.push_str("\\u2029"),
        character if character.is_ascii_control() => push_hex_escape(output, character),
        character if !character.is_ascii() => push_unicode_escape(output, character),
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
        character if character.is_ascii_control() => push_hex_escape(output, character),
        character if !character.is_ascii() => push_unicode_escape(output, character),
        character => output.push(character),
    }
}

fn push_hex_escape(output: &mut String, character: char) {
    let code = character as u32;
    let _ = write!(output, "\\x{code:02X}");
}

fn push_unicode_escape(output: &mut String, character: char) {
    let code = character as u32;
    if code <= 0xffff {
        let _ = write!(output, "\\u{code:04X}");
    } else {
        let _ = write!(output, "\\u{{{code:X}}}");
    }
}
