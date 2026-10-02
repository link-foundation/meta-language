//! The native grammar listing written and read by the `meta-language grammar`
//! commands.
//!
//! An optional `format TAG` line, a `start NAME` line for the start rule, then
//! one `rule NAME = KIND EXPRESSION` line per rule in grammar order, with
//! expressions spelled as the shared grammar parity fixtures spell them
//! (`seq(ref(a), literal("b"))`). Mirrors the native listing of
//! `js/src/grammar-interchange.js`.

use super::super::{
    CharClassItem, Grammar, GrammarExpr, GrammarFormat, GrammarImportError, GrammarRule, RuleKind,
};

/// The longest repetition bound a listing may spell, in decimal digits.
const MAX_BOUND_DIGITS: usize = 9;

fn native_error(line: Option<usize>, detail: impl Into<String>) -> GrammarImportError {
    let detail = detail.into();
    GrammarImportError::Parse {
        format: GrammarFormat::MetaLanguage,
        message: line.map_or_else(|| detail.clone(), |line| format!("line {line}: {detail}")),
    }
}

/// Renders the native grammar listing of `grammar`.
#[must_use]
pub fn render_native_grammar(grammar: &Grammar) -> String {
    let mut lines = Vec::new();
    if let Some(format) = grammar.source_format() {
        lines.push(format!("format {}\n", format.as_str()));
    }
    if let Some(start) = grammar.start_rule() {
        lines.push(format!("start {}\n", render_name(&start.name)));
    }
    lines.extend(grammar.rules().iter().map(|rule| {
        format!(
            "rule {} = {} {}\n",
            render_name(&rule.name),
            rule.kind().as_str(),
            render_native_expression(rule.expr())
        )
    }));
    lines.concat()
}

/// Renders one expression with the native listing spelling.
#[must_use]
pub fn render_native_expression(expr: &GrammarExpr) -> String {
    let list = |items: &[GrammarExpr]| {
        items
            .iter()
            .map(render_native_expression)
            .collect::<Vec<_>>()
            .join(", ")
    };
    match expr {
        GrammarExpr::Empty => "empty".to_owned(),
        GrammarExpr::AnyChar => "any".to_owned(),
        GrammarExpr::Terminal(value) => format!("literal({})", quote(value)),
        GrammarExpr::TerminalInsensitive(value) => format!("literalInsensitive({})", quote(value)),
        GrammarExpr::CharRange(start, end) => render_range(*start, *end),
        GrammarExpr::CharClass { negated, items } => {
            let items = items
                .iter()
                .map(|item| match item {
                    CharClassItem::Range(start, end) => render_range(*start, *end),
                    CharClassItem::Char(value) => format!("char({})", quote_char(*value)),
                })
                .collect::<Vec<_>>()
                .join(", ");
            format!("{}({items})", if *negated { "notClass" } else { "class" })
        }
        GrammarExpr::NonTerminal(name) => format!("ref({})", render_name(name)),
        GrammarExpr::Choice {
            ordered,
            alternatives,
        } => format!(
            "{}({})",
            if *ordered { "orderedChoice" } else { "choice" },
            list(alternatives)
        ),
        GrammarExpr::Sequence(items) => format!("seq({})", list(items)),
        GrammarExpr::Optional(inner) => format!("optional({})", render_native_expression(inner)),
        GrammarExpr::ZeroOrMore(inner) => format!("repeat0({})", render_native_expression(inner)),
        GrammarExpr::OneOrMore(inner) => format!("repeat1({})", render_native_expression(inner)),
        GrammarExpr::And(inner) => format!("and({})", render_native_expression(inner)),
        GrammarExpr::Not(inner) => format!("not({})", render_native_expression(inner)),
        GrammarExpr::Repeat { expr, min, max } => format!(
            "repeat({}, {min}, {})",
            render_native_expression(expr),
            max.map_or_else(|| "unbounded".to_owned(), |max| max.to_string())
        ),
        GrammarExpr::Capture { label, expr } => format!(
            "capture({}, {})",
            label.as_deref().map_or_else(|| "null".to_owned(), quote),
            render_native_expression(expr)
        ),
    }
}

fn render_range(start: char, end: char) -> String {
    format!("range({}, {})", quote_char(start), quote_char(end))
}

fn quote(value: &str) -> String {
    serde_json::to_string(value).unwrap_or_default()
}

fn quote_char(value: char) -> String {
    quote(&value.to_string())
}

/// Whether `value` ends a bare rule name.
const fn is_name_stop(value: char) -> bool {
    value.is_whitespace() || matches!(value, '(' | ')' | ',' | '"' | '=')
}

fn render_name(name: &str) -> String {
    if !name.is_empty() && !name.chars().any(is_name_stop) {
        name.to_owned()
    } else {
        quote(name)
    }
}

/// Parses a native grammar listing written by [`render_native_grammar`].
///
/// # Errors
///
/// Returns a [`GrammarImportError`] of the meta-language format naming the
/// first line that does not parse.
pub fn parse_native_grammar(source: &str) -> Result<Grammar, GrammarImportError> {
    let mut rules: Vec<GrammarRule> = Vec::new();
    let mut start: Option<(String, usize)> = None;
    let mut source_format: Option<GrammarFormat> = None;
    for (index, text) in source.split('\n').enumerate() {
        let line = index + 1;
        let text = text.strip_suffix('\r').unwrap_or(text);
        let mut cursor = Cursor {
            chars: text.chars().collect(),
            line,
            position: 0,
        };
        cursor.skip_spaces();
        if cursor.done() || cursor.peek() == Some('#') {
            continue;
        }
        let directive = cursor.word()?;
        cursor.require_space()?;
        match directive.as_str() {
            "format" => {
                if source_format.is_some() {
                    return Err(cursor.fail("the format is given twice"));
                }
                let tag = cursor.word()?;
                source_format = Some(
                    GrammarFormat::from_tag(&tag)
                        .ok_or_else(|| cursor.fail(format!("unknown source format {tag}")))?,
                );
            }
            "start" => {
                if start.is_some() {
                    return Err(cursor.fail("the start rule is given twice"));
                }
                start = Some((cursor.name()?, line));
            }
            "rule" => {
                let name = cursor.name()?;
                cursor.skip_spaces();
                cursor.expect('=')?;
                cursor.skip_spaces();
                let kind = cursor.word()?;
                let kind = RuleKind::from_tag(&kind)
                    .ok_or_else(|| cursor.fail(format!("unknown rule kind {kind}")))?;
                cursor.require_space()?;
                let expr = cursor.expression()?;
                if rules.iter().any(|rule| rule.name == name) {
                    return Err(cursor.fail(format!("rule {name} is defined twice")));
                }
                rules.push(GrammarRule::new(name, expr).with_kind(kind));
            }
            other => return Err(cursor.fail(format!("unknown directive {other}"))),
        }
        cursor.skip_spaces();
        if !cursor.done() {
            return Err(cursor.fail("unexpected text at the end of the line"));
        }
    }
    let Some(first) = rules.first().map(|rule| rule.name.clone()) else {
        return Err(native_error(None, "the listing defines no rules"));
    };
    let start = match start {
        Some((name, line)) => {
            if !rules.iter().any(|rule| rule.name == name) {
                return Err(native_error(
                    Some(line),
                    format!("start rule {name} is not defined"),
                ));
            }
            name
        }
        None => first,
    };
    let mut grammar = Grammar::new();
    for rule in rules {
        grammar.add_rule(rule);
    }
    grammar.set_start(start);
    if let Some(format) = source_format {
        grammar.set_source_format(format);
    }
    Ok(grammar)
}

struct Cursor {
    chars: Vec<char>,
    line: usize,
    position: usize,
}

impl Cursor {
    fn fail(&self, detail: impl Into<String>) -> GrammarImportError {
        native_error(Some(self.line), detail)
    }

    const fn done(&self) -> bool {
        self.position >= self.chars.len()
    }

    fn peek(&self) -> Option<char> {
        self.chars.get(self.position).copied()
    }

    fn text(&self, begin: usize) -> String {
        self.chars[begin..self.position].iter().collect()
    }

    fn skip_spaces(&mut self) {
        while matches!(self.peek(), Some(' ' | '\t')) {
            self.position += 1;
        }
    }

    fn require_space(&mut self) -> Result<(), GrammarImportError> {
        if !matches!(self.peek(), Some(' ' | '\t')) {
            return Err(self.fail("expected a space"));
        }
        self.skip_spaces();
        Ok(())
    }

    fn expect(&mut self, expected: char) -> Result<(), GrammarImportError> {
        if self.peek() != Some(expected) {
            return Err(self.fail(format!("expected {expected}")));
        }
        self.position += 1;
        Ok(())
    }

    fn word(&mut self) -> Result<String, GrammarImportError> {
        let begin = self.position;
        while self
            .peek()
            .is_some_and(|value| value.is_ascii_alphanumeric() || matches!(value, '_' | '-'))
        {
            self.position += 1;
        }
        if begin == self.position {
            return Err(self.fail("expected a word"));
        }
        Ok(self.text(begin))
    }

    fn name(&mut self) -> Result<String, GrammarImportError> {
        if self.peek() == Some('"') {
            return self.string();
        }
        let begin = self.position;
        while self.peek().is_some_and(|value| !is_name_stop(value)) {
            self.position += 1;
        }
        if begin == self.position {
            return Err(self.fail("expected a name"));
        }
        Ok(self.text(begin))
    }

    fn string(&mut self) -> Result<String, GrammarImportError> {
        if self.peek() != Some('"') {
            return Err(self.fail("expected a string"));
        }
        let begin = self.position;
        self.position += 1;
        while let Some(value) = self.peek() {
            if value == '"' {
                break;
            }
            self.position += if value == '\\' { 2 } else { 1 };
        }
        if self.done() {
            return Err(self.fail("unterminated string"));
        }
        self.position += 1;
        serde_json::from_str::<String>(&self.text(begin)).map_err(|_| self.fail("invalid string"))
    }

    fn character(&mut self) -> Result<char, GrammarImportError> {
        let value = self.string()?;
        let mut chars = value.chars();
        match (chars.next(), chars.next()) {
            (Some(value), None) => Ok(value),
            _ => Err(self.fail("a character must be one code point")),
        }
    }

    fn bound(&mut self, allow_unbounded: bool) -> Result<Option<usize>, GrammarImportError> {
        if allow_unbounded && self.peek() == Some('u') {
            if self.word()? != "unbounded" {
                return Err(self.fail("expected a repetition bound"));
            }
            return Ok(None);
        }
        let begin = self.position;
        while self.peek().is_some_and(|value| value.is_ascii_digit()) {
            self.position += 1;
        }
        let digits = self.text(begin);
        if digits.is_empty() {
            return Err(self.fail("expected a repetition bound"));
        }
        if digits.len() > MAX_BOUND_DIGITS {
            return Err(self.fail(format!("repetition bound {digits} is too large")));
        }
        digits
            .parse()
            .map(Some)
            .map_err(|_| self.fail("expected a repetition bound"))
    }

    fn separator(&mut self) -> Result<(), GrammarImportError> {
        self.skip_spaces();
        self.expect(',')?;
        self.skip_spaces();
        Ok(())
    }

    fn open(&mut self) -> Result<(), GrammarImportError> {
        self.expect('(')?;
        self.skip_spaces();
        Ok(())
    }

    fn close(&mut self) -> Result<(), GrammarImportError> {
        self.skip_spaces();
        self.expect(')')
    }

    fn list<T>(
        &mut self,
        item: fn(&mut Self) -> Result<T, GrammarImportError>,
    ) -> Result<Vec<T>, GrammarImportError> {
        self.open()?;
        let mut items = Vec::new();
        self.skip_spaces();
        while self.peek() != Some(')') {
            if !items.is_empty() {
                self.separator()?;
            }
            items.push(item(self)?);
            self.skip_spaces();
            if self.done() {
                return Err(self.fail("expected )"));
            }
        }
        self.close()?;
        Ok(items)
    }

    fn inner(&mut self) -> Result<Box<GrammarExpr>, GrammarImportError> {
        self.open()?;
        let expr = self.expression()?;
        self.close()?;
        Ok(Box::new(expr))
    }

    fn expression(&mut self) -> Result<GrammarExpr, GrammarImportError> {
        let kind = self.word()?;
        Ok(match kind.as_str() {
            "empty" => GrammarExpr::Empty,
            "any" => GrammarExpr::AnyChar,
            "literal" | "literalInsensitive" => {
                self.open()?;
                let value = self.string()?;
                self.close()?;
                if kind == "literal" {
                    GrammarExpr::Terminal(value)
                } else {
                    GrammarExpr::TerminalInsensitive(value)
                }
            }
            "range" => {
                self.open()?;
                let start = self.character()?;
                self.separator()?;
                let end = self.character()?;
                self.close()?;
                GrammarExpr::CharRange(start, end)
            }
            "class" | "notClass" => GrammarExpr::CharClass {
                negated: kind == "notClass",
                items: self.list(Self::class_item)?,
            },
            "ref" => {
                self.open()?;
                let name = self.name()?;
                self.close()?;
                GrammarExpr::NonTerminal(name)
            }
            "choice" | "orderedChoice" => GrammarExpr::Choice {
                ordered: kind == "orderedChoice",
                alternatives: self.list(Self::expression)?,
            },
            "seq" => GrammarExpr::Sequence(self.list(Self::expression)?),
            "repeat" => {
                self.open()?;
                let expr = Box::new(self.expression()?);
                self.separator()?;
                let min = self.bound(false)?.unwrap_or_default();
                self.separator()?;
                let max = self.bound(true)?;
                self.close()?;
                if max.is_some_and(|max| max < min) {
                    let max = max.unwrap_or_default();
                    return Err(self.fail(format!("repetition bounds {min}, {max} are reversed")));
                }
                GrammarExpr::Repeat { expr, min, max }
            }
            "capture" => {
                self.open()?;
                let label = if self.peek() == Some('"') {
                    Some(self.string()?)
                } else if self.word()? == "null" {
                    None
                } else {
                    return Err(self.fail("expected a capture label or null"));
                };
                self.separator()?;
                let expr = Box::new(self.expression()?);
                self.close()?;
                GrammarExpr::Capture { label, expr }
            }
            "optional" => GrammarExpr::Optional(self.inner()?),
            "repeat0" => GrammarExpr::ZeroOrMore(self.inner()?),
            "repeat1" => GrammarExpr::OneOrMore(self.inner()?),
            "and" => GrammarExpr::And(self.inner()?),
            "not" => GrammarExpr::Not(self.inner()?),
            other => return Err(self.fail(format!("unknown expression {other}"))),
        })
    }

    fn class_item(&mut self) -> Result<CharClassItem, GrammarImportError> {
        let kind = self.word()?;
        self.open()?;
        let item = match kind.as_str() {
            "char" => CharClassItem::Char(self.character()?),
            "range" => {
                let start = self.character()?;
                self.separator()?;
                CharClassItem::Range(start, self.character()?)
            }
            other => return Err(self.fail(format!("unknown class item {other}"))),
        };
        self.close()?;
        Ok(item)
    }
}
