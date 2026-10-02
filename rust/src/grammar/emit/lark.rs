//! Lark grammar emitter.

use std::collections::BTreeSet;
use std::fmt::Write as _;

use super::structural::{
    NamePlan, Precedence, Rendered, check_repeat_bounds, collect_references, doc_lines,
    join_choice, join_sequence, negated_lookahead_set, snake_case, wrap,
};
use super::{EmitReport, GrammarEmitError, finish_lines, unsupported_error};
use crate::grammar::{
    CharClassItem, Grammar, GrammarExpr, GrammarFormat, GrammarRule, RuleKind, import_lark,
};

const FORMAT: GrammarFormat = GrammarFormat::Lark;
const LABEL: &str = "Lark";

/// Documentation `import_lark` attaches to rules created from `%ignore`.
const IGNORE_DOC: &str = "%ignore";

/// Emits a Lark grammar.
///
/// Normal rules become lower-case Lark rules, silent rules become inlined
/// `?rule`s, and token and atomic rules become upper-case terminals. Rule
/// names are adjusted deterministically to Lark's case conventions; every
/// rename is recorded in [`EmitReport::lossy`] as
/// `Lark renamed rule "a" to "b"`. Silent rules documented exactly as
/// `%ignore`, which is how `import_lark` represents `%ignore` directives, are
/// written back as `%ignore` directives; no other `%ignore` is emitted.
///
/// Character classes, ranges and any-character are written as regex
/// terminals, case-insensitive literals as `(?i:...)` regexes, counted
/// repetition as `~ n..m`, and captures labelled `regex` over a literal as
/// the regex they were imported from. Ordered choice is written as Lark
/// alternation; lookahead and other captures have no Lark form and are
/// dropped, except that a negative lookahead over a character set followed by
/// any character becomes a complemented character class. Every lowering and
/// drop is recorded in the report, so `import_lark` of the output reproduces
/// the grammar modulo the recorded notes, and emitting the re-imported
/// grammar reproduces the same text.
///
/// Rule documentation is written as `//` comments above each rule; trailing
/// `inline` and `priority N` documentation parts, as produced by
/// `import_lark`, are written back as the `?` prefix and `.N` priority.
///
/// # Errors
///
/// Returns [`GrammarEmitError::Unsupported`] for empty non-negated character
/// classes, descending character ranges, repetitions whose maximum is below
/// their minimum, and regexes that Lark cannot delimit.
pub fn emit_lark(grammar: &Grammar) -> Result<(String, EmitReport), GrammarEmitError> {
    LarkEmitter::new(grammar).emit()
}

#[derive(Debug)]
struct LarkEmitter<'grammar> {
    grammar: &'grammar Grammar,
    names: NamePlan,
    ignored: BTreeSet<usize>,
    report: EmitReport,
    foreign: bool,
    rule: String,
}

impl<'grammar> LarkEmitter<'grammar> {
    fn new(grammar: &'grammar Grammar) -> Self {
        let rules = grammar.rules();
        let mut ignored = rules
            .iter()
            .enumerate()
            .filter(|(_index, rule)| {
                rule.kind == RuleKind::Silent && rule.doc() == Some(IGNORE_DOC)
            })
            .map(|(index, _rule)| index)
            .collect::<BTreeSet<_>>();
        if ignored.len() == rules.len() {
            ignored.clear();
        }
        let foreign = grammar.source_format() != Some(FORMAT);
        let mut report = EmitReport::default();
        let mut candidates = rules
            .iter()
            .map(|rule| rule_candidate(rule, foreign, &mut report))
            .collect::<Vec<_>>();
        for (position, &index) in ignored.iter().enumerate() {
            candidates[index] = if position == 0 {
                "_ignore".to_string()
            } else {
                format!("_ignore_{}", position + 1)
            };
        }
        let names = NamePlan::new(
            grammar,
            LABEL,
            &candidates,
            &ignored,
            reference_candidate,
            &mut report,
        );
        Self {
            grammar,
            names,
            ignored,
            report,
            foreign,
            rule: String::new(),
        }
    }

    fn emit(mut self) -> Result<(String, EmitReport), GrammarEmitError> {
        let (order, effective) = self.emission_order();
        self.note_start(effective);
        let mut lines = Vec::new();
        for index in order {
            self.emit_rule(index, &mut lines)?;
        }
        Ok((finish_lines(&lines), self.report))
    }

    /// Orders rules so that `import_lark` picks the same start rule and
    /// returns that rule. The importer starts at `start` when it exists,
    /// wherever it sits, so source order is kept then; otherwise the start
    /// rule moves first. `%ignore` directives come last because the importer
    /// appends them.
    fn emission_order(&self) -> (Vec<usize>, Option<usize>) {
        let rules = self.grammar.rules();
        let regular = (0..rules.len())
            .filter(|index| !self.ignored.contains(index))
            .collect::<Vec<_>>();
        let named_start = regular
            .iter()
            .copied()
            .find(|&index| self.names.rule_name(index) == "start");
        let mut order = Vec::with_capacity(rules.len());
        let effective = if named_start.is_some() {
            order.extend(regular.iter().copied());
            named_start
        } else {
            let effective = self
                .grammar
                .start()
                .and_then(|start| {
                    regular
                        .iter()
                        .copied()
                        .find(|&index| rules[index].name == start)
                })
                .or_else(|| regular.first().copied());
            order.extend(effective);
            order.extend(
                regular
                    .iter()
                    .copied()
                    .filter(|&index| Some(index) != effective),
            );
            effective
        };
        order.extend(self.ignored.iter().copied());
        (order, effective)
    }

    fn note_start(&mut self, effective: Option<usize>) {
        let (Some(start), Some(effective)) = (self.grammar.start(), effective) else {
            return;
        };
        let expected = self.names.resolve(start).to_string();
        let actual = self.names.rule_name(effective).to_string();
        if expected != actual {
            self.report.add_lossy(format!(
                "Lark re-imports start rule as {actual:?} instead of {expected:?}, \
                 because Lark starts at `start` or the first rule"
            ));
        }
    }

    fn emit_rule(&mut self, index: usize, lines: &mut Vec<String>) -> Result<(), GrammarEmitError> {
        let rule = &self.grammar.rules()[index];
        let name = self.names.rule_name(index).to_string();
        self.rule.clone_from(&name);
        let body = self.render(rule.expr())?.0;

        if self.ignored.contains(&index) {
            if body.is_empty() {
                return Err(unsupported_error(FORMAT, "empty %ignore expression"));
            }
            lines.push(format!("%ignore {body}"));
            return Ok(());
        }

        let silent = rule.kind == RuleKind::Silent;
        let doc = plan_doc(rule.doc(), silent);
        if doc.reimported.as_deref() != rule.doc() {
            self.report.add_lossy(format!(
                "Lark re-imports the documentation of rule {name:?} as {:?}",
                doc.reimported
            ));
        }
        lines.extend(doc.comments);

        let mut line = String::new();
        if silent {
            line.push('?');
        }
        line.push_str(&name);
        if let Some(priority) = doc.priority {
            let _ = write!(line, ".{priority}");
        }
        line.push(':');
        if !body.is_empty() {
            line.push(' ');
            line.push_str(&body);
        }
        lines.push(line);

        if rule.kind == RuleKind::Atomic {
            self.report.add_lossy(format!(
                "Lark emitted atomic rule {:?} as terminal {name:?}, which re-imports as a token rule",
                rule.name
            ));
        }
        if self.foreign && is_terminal_name(&name) {
            self.note_terminal_references(rule, &name);
        }
        Ok(())
    }

    fn note_terminal_references(&mut self, rule: &GrammarRule, name: &str) {
        let mut references = Vec::new();
        collect_references(rule.expr(), &mut references);
        for reference in references {
            let emitted = self.names.resolve(reference).to_string();
            if self.grammar.rule(reference).is_some() && !is_terminal_name(&emitted) {
                self.report.add_lossy(format!(
                    "Lark terminal {name:?} references rule {emitted:?}, which Lark rejects"
                ));
            }
        }
    }

    fn note(&mut self, message: &str) {
        let note = format!("Lark {message} in rule {:?}", self.rule);
        self.report.add_lossy(note);
    }

    fn render(&mut self, expr: &GrammarExpr) -> Result<Rendered, GrammarEmitError> {
        Ok(match expr {
            GrammarExpr::Empty => (String::new(), Precedence::Sequence),
            GrammarExpr::Terminal(value) => {
                if value.is_empty() {
                    self.note("keeps an empty literal, which Lark rejects,");
                }
                (quote(value), Precedence::Atom)
            }
            GrammarExpr::TerminalInsensitive(value) => {
                self.note(&format!(
                    "emitted case-insensitive literal {value:?} as a (?i:...) regex"
                ));
                (
                    format!("/(?i:{})/", regex_literal_text(value)),
                    Precedence::Atom,
                )
            }
            GrammarExpr::CharRange(start, end) => {
                if start > end {
                    return Err(unsupported_error(
                        FORMAT,
                        format!("descending character range {start:?}..{end:?}"),
                    ));
                }
                self.note("emitted character range as a regex character class");
                render_class(false, &[CharClassItem::Range(*start, *end)])
            }
            GrammarExpr::CharClass { negated, items } => self.render_class(*negated, items)?,
            GrammarExpr::AnyChar => {
                self.note("emitted any character as a regex character class");
                render_class(false, &[any_char_range()])
            }
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
            GrammarExpr::Optional(inner) => {
                let (text, _precedence) = self.render(inner)?;
                (format!("[{text}]"), Precedence::Atom)
            }
            GrammarExpr::ZeroOrMore(inner) => self.render_postfix(inner, "*")?,
            GrammarExpr::OneOrMore(inner) => self.render_postfix(inner, "+")?,
            GrammarExpr::Repeat { expr, min, max } => self.render_repeat(expr, *min, *max)?,
            GrammarExpr::And(_) => {
                self.note("dropped positive lookahead");
                (String::new(), Precedence::Sequence)
            }
            GrammarExpr::Not(_) => {
                self.note("dropped negative lookahead");
                (String::new(), Precedence::Sequence)
            }
            GrammarExpr::Capture { label, expr } => self.render_capture(label.as_deref(), expr)?,
            GrammarExpr::Feature(feature) => {
                return Err(unsupported_error(GrammarFormat::Lark, feature.head()));
            }
        })
    }

    fn render_sequence(&mut self, items: &[GrammarExpr]) -> Result<Rendered, GrammarEmitError> {
        let mut rendered = Vec::with_capacity(items.len());
        let mut index = 0;
        while index < items.len() {
            if let Some((negated, class)) = negated_lookahead_set(items, index) {
                self.note(
                    "lowered negative lookahead followed by any character to a complemented character class",
                );
                rendered.push(render_class(negated, &class));
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
            format!("{}{operator}", wrap(operand, Precedence::Atom)),
            Precedence::Postfix,
        ))
    }

    fn render_repeat(
        &mut self,
        inner: &GrammarExpr,
        min: usize,
        max: Option<usize>,
    ) -> Result<Rendered, GrammarEmitError> {
        check_repeat_bounds(FORMAT, min, max)?;
        let operand = wrap(self.render(inner)?, Precedence::Atom);
        Ok(match (min, max) {
            (_, Some(max)) if max == min => (format!("{operand} ~ {min}"), Precedence::Postfix),
            (_, Some(max)) => (format!("{operand} ~ {min}..{max}"), Precedence::Postfix),
            (0, None) => {
                self.note("emitted unbounded repetition {0,} as `*`");
                (format!("{operand}*"), Precedence::Postfix)
            }
            (1, None) => {
                self.note("emitted unbounded repetition {1,} as `+`");
                (format!("{operand}+"), Precedence::Postfix)
            }
            (min, None) => {
                self.note(&format!(
                    "emitted unbounded repetition {{{min},}} as `~ {min}` followed by `*`"
                ));
                (
                    format!("{operand} ~ {min} {operand}*"),
                    Precedence::Sequence,
                )
            }
        })
    }

    fn render_capture(
        &mut self,
        label: Option<&str>,
        inner: &GrammarExpr,
    ) -> Result<Rendered, GrammarEmitError> {
        if label == Some("regex")
            && let GrammarExpr::Terminal(pattern) = inner
        {
            return self.render_regex(pattern);
        }
        match label {
            Some(label) => self.note(&format!("dropped capture label {label:?}")),
            None => self.note("dropped anonymous capture"),
        }
        self.render(inner)
    }

    fn render_regex(&mut self, pattern: &str) -> Result<Rendered, GrammarEmitError> {
        if pattern.is_empty() {
            self.note("emitted an empty regex as /(?:)/");
            return Ok(("/(?:)/".to_string(), Precedence::Atom));
        }
        let body = delimit_regex(pattern)?;
        if body != pattern {
            self.note(&format!("escaped regex /{pattern}/ as /{body}/"));
        }
        if is_class_shaped(&body) {
            let probe = format!("probe: /{body}/\n");
            if let Err(error) = import_lark(&probe) {
                return Err(unsupported_error(
                    FORMAT,
                    format!("regex /{body}/ does not re-import as a character class: {error}"),
                ));
            }
            self.note(&format!("regex /{body}/ re-imports as a character class"));
        }
        Ok((format!("/{body}/"), Precedence::Atom))
    }

    fn render_class(
        &mut self,
        negated: bool,
        items: &[CharClassItem],
    ) -> Result<Rendered, GrammarEmitError> {
        if items.is_empty() {
            if negated {
                self.note("emitted an empty negated character class as any character");
                return Ok(render_class(false, &[any_char_range()]));
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
        Ok(render_class(negated, items))
    }
}

const fn any_char_range() -> CharClassItem {
    CharClassItem::Range('\0', char::MAX)
}

fn is_terminal_name(name: &str) -> bool {
    name.starts_with(|c: char| c.is_ascii_uppercase())
}

/// Returns Lark's conventional spelling of a rule or terminal name.
fn rule_candidate(rule: &GrammarRule, foreign: bool, report: &mut EmitReport) -> String {
    match rule.kind {
        RuleKind::Token | RuleKind::Atomic => terminal_candidate(&rule.name),
        RuleKind::Normal if is_filtered_terminal_name(&rule.name) => {
            if foreign {
                report.add_lossy(format!(
                    "Lark keeps rule {:?} as a filtered terminal name, which re-imports as a normal rule",
                    rule.name
                ));
            }
            rule.name.clone()
        }
        RuleKind::Normal | RuleKind::Silent => lower_candidate(&rule.name),
    }
}

/// Returns a valid spelling for a reference without a rule, keeping its case.
fn reference_candidate(name: &str) -> String {
    if is_filtered_terminal_name(name) {
        name.to_string()
    } else if is_terminal_name(name) {
        terminal_candidate(name)
    } else {
        lower_candidate(name)
    }
}

/// Recognizes `_NAME`, the spelling Lark uses for filtered terminals.
fn is_filtered_terminal_name(name: &str) -> bool {
    let mut chars = name.chars();
    chars.next() == Some('_')
        && chars.next().is_some_and(|c| c.is_ascii_uppercase())
        && chars.all(|c| c.is_ascii_uppercase() || c.is_ascii_digit() || c == '_')
}

fn lower_candidate(name: &str) -> String {
    let output = snake_case(name);
    let mut chars = output.chars();
    let valid = match chars.next() {
        Some('_') => chars.next().is_some_and(|c| c.is_ascii_lowercase()),
        Some(first) => first.is_ascii_lowercase(),
        None => false,
    };
    if valid {
        output
    } else if output.starts_with('_') {
        format!("r{output}")
    } else {
        format!("r_{output}")
    }
}

fn terminal_candidate(name: &str) -> String {
    let mut chars = name.chars();
    if chars.next().is_some_and(|c| c.is_ascii_uppercase())
        && chars.all(|c| c.is_ascii_uppercase() || c.is_ascii_digit() || c == '_')
    {
        return name.to_string();
    }
    let output = snake_case(name).to_ascii_uppercase();
    if output.starts_with(|c: char| c.is_ascii_uppercase()) {
        output
    } else if output.starts_with('_') {
        format!("T{output}")
    } else {
        format!("T_{output}")
    }
}

/// Quotes a Lark string literal.
fn quote(value: &str) -> String {
    let mut output = String::from("\"");
    for character in value.chars() {
        match character {
            '"' => output.push_str("\\\""),
            '\\' => output.push_str("\\\\"),
            character => push_plain_char(&mut output, character),
        }
    }
    output.push('"');
    output
}

/// Escapes regex metacharacters so a regex matches `value` literally.
fn regex_literal_text(value: &str) -> String {
    let mut output = String::new();
    for character in value.chars() {
        if character.is_ascii_punctuation() {
            output.push('\\');
            output.push(character);
        } else {
            push_plain_char(&mut output, character);
        }
    }
    output
}

/// Renders a regex character class terminal.
fn render_class(negated: bool, items: &[CharClassItem]) -> Rendered {
    let mut output = String::from(if negated { "/[^" } else { "/[" });
    for item in items {
        match item {
            CharClassItem::Char(character) => push_class_char(&mut output, *character),
            CharClassItem::Range(start, end) => {
                push_class_char(&mut output, *start);
                output.push('-');
                push_class_char(&mut output, *end);
            }
        }
    }
    output.push_str("]/");
    (output, Precedence::Atom)
}

fn push_class_char(output: &mut String, character: char) {
    match character {
        '\\' | ']' | '[' | '-' | '^' | '/' => {
            output.push('\\');
            output.push(character);
        }
        character => push_plain_char(output, character),
    }
}

/// Pushes a character, escaping line breaks, tabs, controls, whitespace
/// other than space, and characters outside the Basic Multilingual Plane.
fn push_plain_char(output: &mut String, character: char) {
    let code = u32::from(character);
    match character {
        '\n' => output.push_str("\\n"),
        '\r' => output.push_str("\\r"),
        '\t' => output.push_str("\\t"),
        character
            if character.is_control()
                || (character.is_whitespace() && character != ' ')
                || code > 0xFFFF =>
        {
            let _ = match code {
                0..=0xFF => write!(output, "\\x{code:02X}"),
                0x100..=0xFFFF => write!(output, "\\u{code:04X}"),
                _ => write!(output, "\\U{code:08X}"),
            };
        }
        character => output.push(character),
    }
}

/// Escapes unescaped `/` and raw line breaks so the pattern fits in `/.../`.
fn delimit_regex(pattern: &str) -> Result<String, GrammarEmitError> {
    let mut output = String::new();
    let mut chars = pattern.chars();
    while let Some(character) = chars.next() {
        match character {
            '\\' => match chars.next() {
                Some('\n') => output.push_str("\\n"),
                Some('\r') => output.push_str("\\r"),
                Some(next) => {
                    output.push('\\');
                    output.push(next);
                }
                None => {
                    return Err(unsupported_error(
                        FORMAT,
                        format!("regex /{pattern}/ ends with a lone backslash"),
                    ));
                }
            },
            '/' => output.push_str("\\/"),
            '\n' => output.push_str("\\n"),
            '\r' => output.push_str("\\r"),
            character => output.push(character),
        }
    }
    Ok(output)
}

/// Mirrors the importer's test for regexes it lowers to character classes.
fn is_class_shaped(body: &str) -> bool {
    if body.len() < 2 || !body.starts_with('[') || !body.ends_with(']') {
        return false;
    }
    let inner = &body[1..body.len() - 1];
    let mut escaped = false;
    for character in inner.chars() {
        if escaped {
            escaped = false;
        } else if character == '\\' {
            escaped = true;
        } else if character == ']' {
            return false;
        }
    }
    true
}

/// Comments, priority and re-imported documentation for one rule.
#[derive(Debug)]
struct DocPlan {
    comments: Vec<String>,
    priority: Option<usize>,
    reimported: Option<String>,
}

/// Plans how rule documentation is written so that `import_lark` reads the
/// same documentation back whenever it consists of comments, `inline` and
/// `priority N`, as `import_lark` produces it.
fn plan_doc(doc: Option<&str>, silent: bool) -> DocPlan {
    let mut text = doc.unwrap_or("");
    let mut priority = None;
    if let Some((rest, part)) = split_last_part(text)
        && let Some(digits) = part.strip_prefix("priority ")
        && let Ok(value) = digits.parse::<usize>()
        && value.to_string() == digits
    {
        priority = Some(value);
        text = rest;
    }
    if silent && let Some((rest, "inline")) = split_last_part(text) {
        text = rest;
    }

    let trimmed = text.trim();
    let comments = if trimmed.starts_with("//") && !trimmed.contains(['\n', '\r']) {
        vec![trimmed.to_string()]
    } else {
        doc_lines(text)
            .map(|line| {
                if line.starts_with("//") {
                    line.to_string()
                } else {
                    format!("// {line}")
                }
            })
            .collect()
    };

    let mut parts = comments.clone();
    if silent {
        parts.push("inline".to_string());
    }
    if let Some(priority) = priority {
        parts.push(format!("priority {priority}"));
    }
    DocPlan {
        comments,
        priority,
        reimported: (!parts.is_empty()).then(|| parts.join("; ")),
    }
}

/// Splits the last `; `-separated part off documentation.
fn split_last_part(text: &str) -> Option<(&str, &str)> {
    if text.is_empty() {
        return None;
    }
    Some(text.rfind("; ").map_or(("", text), |position| {
        (&text[..position], &text[position + 2..])
    }))
}
