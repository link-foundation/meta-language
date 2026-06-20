use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;

use crate::grammar::{CharClassItem, Grammar, GrammarExpr, GrammarFormat, RuleKind};

use super::{ordered_rules, unsupported_error, EmitReport, GrammarEmitError};

const FORMAT: GrammarFormat = GrammarFormat::TreeSitter;

/// Emits a tree-sitter `grammar.js` module from the grammar IR.
///
/// This convenience API returns only the JavaScript source. Use
/// [`emit_tree_sitter_grammar_js_with_report`] when callers need fidelity notes
/// for lossy IR-to-tree-sitter mappings.
pub fn emit_tree_sitter_grammar_js(grammar: &Grammar) -> Result<String, GrammarEmitError> {
    emit_tree_sitter_grammar_js_with_report(grammar).map(|(source, _report)| source)
}

/// Emits a tree-sitter `grammar.js` module and a fidelity report.
///
/// tree-sitter maps CFG-shaped IR constructs directly with `seq`, `choice`,
/// `repeat`, `repeat1`, `optional`, `token`, `field`, `alias`, and `prec`.
/// Ordered choices are emitted as tree-sitter's unordered GLR `choice`, counted
/// repetition is desugared, and syntactic predicates return
/// [`GrammarEmitError::Unsupported`] because tree-sitter has no predicate DSL.
pub fn emit_tree_sitter_grammar_js_with_report(
    grammar: &Grammar,
) -> Result<(String, EmitReport), GrammarEmitError> {
    let mut report = EmitReport::default();
    let names = NamePlan::new(grammar, &mut report);
    let mut emitter = TreeSitterEmitter { report, names };

    let mut output = String::new();
    output.push_str("module.exports = grammar({\n");
    let _ = writeln!(
        output,
        "  name: {},",
        quote_js_string(&emitter.names.grammar_name)
    );

    let inline_rules = inline_rules(grammar, &emitter.names, &mut emitter.report);
    if !inline_rules.is_empty() {
        output.push_str("  inline: $ => [\n");
        for name in inline_rules {
            let _ = writeln!(output, "    $.{name},");
        }
        output.push_str("  ],\n");
    }

    output.push_str("  rules: {\n");
    for rule in ordered_rules(grammar) {
        if let Some(doc) = rule.doc() {
            push_doc_lines(&mut output, doc);
        }
        let body = emitter.emit_expr(rule.expr())?;
        let body = apply_rule_kind(rule.kind(), body);
        let name = emitter.names.name_for(rule.name());
        let _ = writeln!(output, "    {name}: $ => {body},");
    }
    output.push_str("  },\n");
    output.push_str("});\n");

    Ok((output, emitter.report))
}

#[derive(Clone, Debug)]
struct TreeSitterEmitter {
    report: EmitReport,
    names: NamePlan,
}

impl TreeSitterEmitter {
    fn emit_expr(&mut self, expr: &GrammarExpr) -> Result<String, GrammarEmitError> {
        match expr {
            GrammarExpr::Empty => Ok("blank()".to_string()),
            GrammarExpr::Terminal(value) => Ok(quote_js_string(value)),
            GrammarExpr::TerminalInsensitive(value) => {
                self.report.add_lossy(format!(
                    "tree-sitter expands case-insensitive terminal {value:?} to a regex"
                ));
                Ok(emit_case_insensitive_terminal(value))
            }
            GrammarExpr::CharRange(start, end) => emit_char_range(*start, *end),
            GrammarExpr::CharClass { negated, items } => emit_char_class(*negated, items),
            GrammarExpr::AnyChar => Ok("/./".to_string()),
            GrammarExpr::NonTerminal(name) => Ok(format!("$.{}", self.names.name_for(name))),
            GrammarExpr::Choice {
                ordered,
                alternatives,
            } => self.emit_choice(*ordered, alternatives),
            GrammarExpr::Sequence(items) => self.emit_sequence(items),
            GrammarExpr::Optional(inner) => {
                let inner = self.emit_expr(inner)?;
                Ok(format!("optional({inner})"))
            }
            GrammarExpr::ZeroOrMore(inner) => {
                let inner = self.emit_expr(inner)?;
                Ok(format!("repeat({inner})"))
            }
            GrammarExpr::OneOrMore(inner) => {
                let inner = self.emit_expr(inner)?;
                Ok(format!("repeat1({inner})"))
            }
            GrammarExpr::Repeat { expr, min, max } => self.emit_repeat(expr, *min, *max),
            GrammarExpr::And(_) | GrammarExpr::Not(_) => {
                Err(unsupported_error(FORMAT, "predicate"))
            }
            GrammarExpr::Capture { label, expr } => self.emit_capture(label.as_deref(), expr),
        }
    }

    fn emit_choice(
        &mut self,
        ordered: bool,
        alternatives: &[GrammarExpr],
    ) -> Result<String, GrammarEmitError> {
        if alternatives.is_empty() {
            return Err(unsupported_error(FORMAT, "empty Choice"));
        }
        if ordered {
            self.report
                .add_lossy("tree-sitter treats ordered choice as unordered choice");
        }

        alternatives
            .iter()
            .map(|alternative| self.emit_expr(alternative))
            .collect::<Result<Vec<_>, _>>()
            .map(|items| format!("choice({})", items.join(", ")))
    }

    fn emit_sequence(&mut self, items: &[GrammarExpr]) -> Result<String, GrammarEmitError> {
        let mut emitted = Vec::new();
        for item in items {
            if matches!(item, GrammarExpr::Empty) {
                continue;
            }
            emitted.push(self.emit_expr(item)?);
        }

        Ok(match emitted.len() {
            0 => "blank()".to_string(),
            _ => format!("seq({})", emitted.join(", ")),
        })
    }

    fn emit_repeat(
        &mut self,
        expr: &GrammarExpr,
        min: usize,
        max: Option<usize>,
    ) -> Result<String, GrammarEmitError> {
        if max.is_some_and(|max| max < min) {
            return Err(unsupported_error(
                FORMAT,
                format!("Repeat with min {min} greater than max {max:?}"),
            ));
        }

        let inner = self.emit_expr(expr)?;
        match (min, max) {
            (0, None) => Ok(format!("repeat({inner})")),
            (1, None) => Ok(format!("repeat1({inner})")),
            _ => {
                self.report.add_lossy(format!(
                    "tree-sitter desugared Repeat with min {min} and max {max:?}"
                ));
                Ok(desugar_repeat(&inner, min, max))
            }
        }
    }

    fn emit_capture(
        &mut self,
        label: Option<&str>,
        expr: &GrammarExpr,
    ) -> Result<String, GrammarEmitError> {
        let Some(label) = label else {
            self.report
                .add_lossy("tree-sitter dropped anonymous capture");
            return self.emit_expr(expr);
        };

        let inner = self.emit_expr(expr)?;
        if let Some(value) = label.strip_prefix("prec=") {
            return Ok(format!("prec({}, {inner})", emit_precedence_value(value)));
        }
        if let Some(value) = label.strip_prefix("prec_left=") {
            return Ok(format!(
                "prec.left({}, {inner})",
                emit_precedence_value(value)
            ));
        }
        if let Some(value) = label.strip_prefix("prec_right=") {
            return Ok(format!(
                "prec.right({}, {inner})",
                emit_precedence_value(value)
            ));
        }
        if let Some(value) = label.strip_prefix("prec_dynamic=") {
            return Ok(format!(
                "prec.dynamic({}, {inner})",
                emit_precedence_value(value)
            ));
        }
        if let Some(value) = label.strip_prefix("alias:") {
            return Ok(format!("alias({inner}, {})", emit_alias_target(value)));
        }

        match label {
            "token" => Ok(format!("token({inner})")),
            "immediate_token" => Ok(format!("token.immediate({inner})")),
            _ => Ok(format!("field({}, {inner})", quote_js_string(label))),
        }
    }
}

fn desugar_repeat(inner: &str, min: usize, max: Option<usize>) -> String {
    let mut parts = vec![inner.to_string(); min];
    match max {
        Some(max) => {
            parts.extend((min..max).map(|_| format!("optional({inner})")));
        }
        None => parts.push(format!("repeat({inner})")),
    }

    match parts.len() {
        0 => "blank()".to_string(),
        1 => parts.remove(0),
        _ => format!("seq({})", parts.join(", ")),
    }
}

fn apply_rule_kind(kind: RuleKind, body: String) -> String {
    match kind {
        RuleKind::Normal | RuleKind::Silent => body,
        RuleKind::Atomic | RuleKind::Token => format!("token({body})"),
    }
}

fn inline_rules(grammar: &Grammar, names: &NamePlan, report: &mut EmitReport) -> Vec<String> {
    let rules = ordered_rules(grammar)
        .into_iter()
        .filter(|rule| rule.kind() == RuleKind::Silent)
        .map(|rule| names.name_for(rule.name()))
        .collect::<Vec<_>>();

    for name in &rules {
        report.add_lossy(format!(
            "tree-sitter emits RuleKind::Silent rule {name:?} in the inline list"
        ));
    }

    rules
}

#[derive(Clone, Debug)]
struct NamePlan {
    names: BTreeMap<String, String>,
    grammar_name: String,
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

        let grammar_name = grammar
            .start_rule()
            .or_else(|| grammar.rules().first())
            .map_or_else(|| "grammar".to_string(), |rule| rule.name().to_string());
        let grammar_name = names
            .get(&grammar_name)
            .cloned()
            .unwrap_or_else(|| sanitize_identifier(&grammar_name));

        Self {
            names,
            grammar_name,
        }
    }

    fn name_for(&self, source: &str) -> String {
        self.names
            .get(source)
            .map_or_else(|| sanitize_identifier(source), Clone::clone)
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
    report.add_lossy(format!(
        "tree-sitter renamed {kind} {source:?} to {emitted:?}"
    ));
}

fn unique_identifier(base: &str, used: &mut BTreeSet<String>) -> String {
    if used.insert(base.to_string()) {
        return base.to_string();
    }

    for suffix in 1_usize.. {
        let candidate = format!("{base}_{suffix}");
        if used.insert(candidate.clone()) {
            return candidate;
        }
    }
    unreachable!("unbounded suffix search must eventually find a free identifier")
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
        output.push_str("grammar");
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
    character.is_ascii_alphanumeric() || character == '_'
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

fn emit_case_insensitive_terminal(value: &str) -> String {
    if value.is_empty() {
        return quote_js_string("");
    }

    let mut pattern = String::new();
    for character in value.chars() {
        if character.is_ascii_alphabetic() {
            pattern.push('[');
            push_escaped_class_char(&mut pattern, character.to_ascii_uppercase());
            push_escaped_class_char(&mut pattern, character.to_ascii_lowercase());
            pattern.push(']');
        } else {
            push_escaped_regex_char(&mut pattern, character);
        }
    }
    regex_literal(&pattern)
}

fn emit_char_range(start: char, end: char) -> Result<String, GrammarEmitError> {
    validate_range("CharRange", start, end)?;
    let mut pattern = String::new();
    pattern.push('[');
    push_escaped_class_char(&mut pattern, start);
    pattern.push('-');
    push_escaped_class_char(&mut pattern, end);
    pattern.push(']');
    Ok(regex_literal(&pattern))
}

fn emit_char_class(negated: bool, items: &[CharClassItem]) -> Result<String, GrammarEmitError> {
    if items.is_empty() {
        return Err(unsupported_error(FORMAT, "empty CharClass"));
    }

    let mut pattern = String::new();
    pattern.push('[');
    if negated {
        pattern.push('^');
    }
    for item in items {
        push_char_class_item(&mut pattern, item)?;
    }
    pattern.push(']');
    Ok(regex_literal(&pattern))
}

fn push_char_class_item(output: &mut String, item: &CharClassItem) -> Result<(), GrammarEmitError> {
    match item {
        CharClassItem::Char(value) => push_escaped_class_char(output, *value),
        CharClassItem::Range(start, end) => {
            validate_range("CharClass range", *start, *end)?;
            push_escaped_class_char(output, *start);
            output.push('-');
            push_escaped_class_char(output, *end);
        }
    }
    Ok(())
}

fn validate_range(construct: &str, start: char, end: char) -> Result<(), GrammarEmitError> {
    if start > end {
        return Err(unsupported_error(
            FORMAT,
            format!(
                "{construct} has descending bounds U+{:04X}..=U+{:04X}",
                start as u32, end as u32
            ),
        ));
    }
    Ok(())
}

fn emit_precedence_value(value: &str) -> String {
    if value.parse::<i64>().is_ok() {
        value.to_string()
    } else {
        quote_js_string(value)
    }
}

fn emit_alias_target(value: &str) -> String {
    let identifier = sanitize_identifier(value);
    if identifier == value {
        format!("$.{identifier}")
    } else {
        quote_js_string(value)
    }
}

fn regex_literal(pattern: &str) -> String {
    format!("/{pattern}/")
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

fn push_escaped_regex_char(output: &mut String, character: char) {
    match character {
        '/' => output.push_str("\\/"),
        '\\' => output.push_str("\\\\"),
        '^' | '$' | '.' | '*' | '+' | '?' | '(' | ')' | '[' | ']' | '{' | '}' | '|' => {
            output.push('\\');
            output.push(character);
        }
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
        '/' => output.push_str("\\/"),
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
        '\u{2028}' => output.push_str("\\u2028"),
        '\u{2029}' => output.push_str("\\u2029"),
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
        for unit in character.encode_utf16(&mut [0; 2]) {
            let _ = write!(output, "\\u{unit:04X}");
        }
    }
}

fn push_doc_lines(output: &mut String, doc: &str) {
    if doc.is_empty() {
        output.push_str("    //\n");
        return;
    }
    for line in doc.lines() {
        if line.is_empty() {
            output.push_str("    //\n");
        } else {
            let _ = writeln!(output, "    // {line}");
        }
    }
}
