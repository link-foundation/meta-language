//! The native grammar listing written and read by the `meta-language grammar`
//! commands.
//!
//! An optional `format TAG` line, a `start NAME` line for the start rule, then
//! one `rule NAME = KIND EXPRESSION` line per rule in grammar order, with
//! expressions spelled as the shared grammar parity fixtures spell them
//! (`seq(ref(a), literal("b"))`). Mirrors the native listing of
//! `js/src/grammar-interchange.js`.

use super::super::feature::class_expression;
use super::super::{
    CharClassItem, FeatureExpr, Grammar, GrammarDeclarations, GrammarExpr, GrammarFormat,
    GrammarImportError, GrammarMacro, GrammarRule, GrammarScanner, MATCHING_MODES, Operation,
    RuleAttributes, RuleKind, UnicodeClassItem,
};
use super::feature_forms::{FormCodec, render_feature_form, render_operation};

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
///
/// The listing holds the `format` and `start` lines, the feature union declarations (`matching`, `import`, `mode`,
/// `extra`, `conflict`, `macro` and `scanner` lines, in that order), then one
/// rule line per rule, with its parameters and its `channel(...)`,
/// `modes(...)` and `action(...)` attributes.
#[must_use]
pub fn render_native_grammar(grammar: &Grammar) -> String {
    let mut lines = Vec::new();
    if let Some(format) = grammar.source_format() {
        lines.push(format!("format {}", format.as_str()));
    }
    if let Some(start) = grammar.start_rule() {
        lines.push(format!("start {}", render_name(&start.name)));
    }
    let declarations = grammar.declarations();
    let names = |names: &[String], separator: &str| {
        names
            .iter()
            .map(|name| render_name(name))
            .collect::<Vec<_>>()
            .join(separator)
    };
    let parameters = |parameters: &[String]| {
        if parameters.is_empty() {
            String::new()
        } else {
            format!("({})", names(parameters, ", "))
        }
    };
    let operations = |operations: &[Operation]| {
        operations
            .iter()
            .map(|operation| render_operation(operation, &LineCodec))
            .collect::<Vec<_>>()
            .join(", ")
    };
    if let Some(matching) = &declarations.matching {
        lines.push(format!("matching {matching}"));
    }
    for name in &declarations.imports {
        lines.push(format!("import {}", render_name(name)));
    }
    for name in &declarations.modes {
        lines.push(format!("mode {}", render_name(name)));
    }
    for extra in &declarations.extras {
        lines.push(format!("extra {}", render_native_expression(extra)));
    }
    for group in &declarations.conflicts {
        lines.push(format!("conflict {}", names(group, " ")));
    }
    for declared in &declarations.macros {
        lines.push(format!(
            "macro {}{} = {}",
            render_name(&declared.name),
            parameters(&declared.parameters),
            render_native_expression(&declared.expression)
        ));
    }
    for scanner in &declarations.scanners {
        lines.push(format!(
            "scanner {} tokens({}) operations({})",
            render_name(&scanner.name),
            names(&scanner.tokens, ", "),
            operations(&scanner.operations)
        ));
    }
    for rule in grammar.rules() {
        let attributes = &rule.attributes;
        let mut line = format!(
            "rule {}{} = {} {}",
            render_name(&rule.name),
            parameters(&attributes.parameters),
            rule.kind().as_str(),
            render_native_expression(rule.expr())
        );
        let mut attribute = |head: &str, body: String| {
            line.push(' ');
            line.push_str(head);
            line.push('(');
            line.push_str(&body);
            line.push(')');
        };
        if let Some(channel) = &attributes.channel {
            attribute("channel", render_name(channel));
        }
        if let Some(modes) = &attributes.modes {
            attribute("modes", names(modes, ", "));
        }
        if let Some(action) = &attributes.action {
            attribute("action", operations(action));
        }
        lines.push(line);
    }
    lines.into_iter().map(|line| line + "\n").collect()
}

/// The native listing spelling of the feature union forms.
struct LineCodec;

impl FormCodec for LineCodec {
    fn name(&self, value: &str) -> String {
        render_name(value)
    }

    fn text(&self, value: &str) -> String {
        quote(value)
    }

    fn expression(&self, expr: &GrammarExpr) -> String {
        render_native_expression(expr)
    }

    fn call(&self, head: &str, parts: Vec<String>) -> String {
        if parts.is_empty() {
            head.to_owned()
        } else {
            format!("{head}({})", parts.join(", "))
        }
    }

    fn block(&self, word: &str, parts: Vec<String>) -> String {
        format!("{word}({})", parts.join(", "))
    }

    fn byte_class(&self, negated: bool, items: Vec<String>) -> String {
        format!(
            "{}({})",
            if negated { "notByteClass" } else { "byteClass" },
            items.join(", ")
        )
    }
}

/// Renders one feature union expression with the native listing spelling.
#[must_use]
pub fn render_native_feature(feature: &FeatureExpr) -> String {
    match feature {
        FeatureExpr::UnicodeClass { negated, items } => {
            let items = items
                .iter()
                .map(|item| match item {
                    UnicodeClassItem::Char(value) => format!("char({})", quote_char(*value)),
                    UnicodeClassItem::Range(start, end) => render_range(*start, *end),
                    UnicodeClassItem::Category(value) => format!("category({})", quote(value)),
                    UnicodeClassItem::Script(value) => format!("script({})", quote(value)),
                })
                .collect::<Vec<_>>()
                .join(", ");
            format!("{}({items})", if *negated { "notClass" } else { "class" })
        }
        FeatureExpr::Call { name, arguments } => format!(
            "ref({}, {})",
            render_name(name),
            arguments
                .iter()
                .map(render_native_expression)
                .collect::<Vec<_>>()
                .join(", ")
        ),
        FeatureExpr::Form(_) | FeatureExpr::ByteClass { .. } => {
            render_feature_form(feature, &LineCodec).unwrap_or_default()
        }
    }
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
        GrammarExpr::Feature(feature) => render_native_feature(feature),
    }
}

fn render_range(start: char, end: char) -> String {
    format!("range({}, {})", quote_char(start), quote_char(end))
}

pub(super) fn quote(value: &str) -> String {
    serde_json::to_string(value).unwrap_or_default()
}

fn quote_char(value: char) -> String {
    quote(&value.to_string())
}

/// Whether `value` ends a bare rule name.
const fn is_name_stop(value: char) -> bool {
    value.is_whitespace() || matches!(value, '(' | ')' | ',' | '"' | '=')
}

pub(super) fn render_name(name: &str) -> String {
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
    let mut declarations = GrammarDeclarations::default();
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
                let parameters = cursor.parameters()?;
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
                let mut attributes = cursor.rule_attributes()?;
                attributes.parameters = parameters;
                rules.push(
                    GrammarRule::new(name, expr)
                        .with_kind(kind)
                        .with_attributes(attributes),
                );
            }
            "matching" => {
                if declarations.matching.is_some() {
                    return Err(cursor.fail("the matching is given twice"));
                }
                let matching = cursor.word()?;
                if !MATCHING_MODES.contains(&matching.as_str()) {
                    return Err(cursor.fail(format!("unknown matching {matching}")));
                }
                declarations.matching = Some(matching);
            }
            "import" => declarations.imports.push(cursor.name()?),
            "mode" => declarations.modes.push(cursor.name()?),
            "extra" => declarations.extras.push(cursor.expression()?),
            "conflict" => {
                let mut group = vec![cursor.name()?];
                cursor.skip_spaces();
                while !cursor.done() {
                    group.push(cursor.name()?);
                    cursor.skip_spaces();
                }
                declarations.conflicts.push(group);
            }
            "macro" => {
                let name = cursor.name()?;
                let parameters = cursor.parameters()?;
                cursor.skip_spaces();
                cursor.expect('=')?;
                cursor.skip_spaces();
                let expression = cursor.expression()?;
                declarations.macros.push(GrammarMacro {
                    name,
                    parameters,
                    expression,
                });
            }
            "scanner" => {
                let name = cursor.name()?;
                cursor.require_space()?;
                if cursor.word()? != "tokens" {
                    return Err(cursor.fail("expected tokens(...)"));
                }
                let tokens = cursor.list(Cursor::name)?;
                cursor.require_space()?;
                if cursor.word()? != "operations" {
                    return Err(cursor.fail("expected operations(...)"));
                }
                let operations = cursor.statements()?;
                declarations.scanners.push(GrammarScanner {
                    name,
                    tokens,
                    operations,
                });
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
    grammar.set_declarations(declarations);
    Ok(grammar)
}

/// Parses one expression written by [`render_native_expression`].
///
/// # Errors
///
/// Returns a [`GrammarImportError`] of the meta-language format when the text
/// is not exactly one expression.
pub fn parse_native_expression(text: &str) -> Result<GrammarExpr, GrammarImportError> {
    let mut cursor = Cursor {
        chars: text.chars().collect(),
        line: 1,
        position: 0,
    };
    cursor.skip_spaces();
    let expr = cursor.expression()?;
    cursor.skip_spaces();
    if cursor.done() {
        Ok(expr)
    } else {
        Err(cursor.fail("unexpected text after the expression"))
    }
}

pub(super) struct Cursor {
    pub(super) chars: Vec<char>,
    pub(super) line: usize,
    pub(super) position: usize,
}

impl Cursor {
    pub(super) fn fail(&self, detail: impl Into<String>) -> GrammarImportError {
        native_error(Some(self.line), detail)
    }

    pub(super) const fn done(&self) -> bool {
        self.position >= self.chars.len()
    }

    pub(super) fn peek(&self) -> Option<char> {
        self.chars.get(self.position).copied()
    }

    pub(super) fn text(&self, begin: usize) -> String {
        self.chars[begin..self.position].iter().collect()
    }

    pub(super) fn skip_spaces(&mut self) {
        while matches!(self.peek(), Some(' ' | '\t')) {
            self.position += 1;
        }
    }

    pub(super) fn require_space(&mut self) -> Result<(), GrammarImportError> {
        if !matches!(self.peek(), Some(' ' | '\t')) {
            return Err(self.fail("expected a space"));
        }
        self.skip_spaces();
        Ok(())
    }

    pub(super) fn expect(&mut self, expected: char) -> Result<(), GrammarImportError> {
        if self.peek() != Some(expected) {
            return Err(self.fail(format!("expected {expected}")));
        }
        self.position += 1;
        Ok(())
    }

    pub(super) fn word(&mut self) -> Result<String, GrammarImportError> {
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

    pub(super) fn name(&mut self) -> Result<String, GrammarImportError> {
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

    pub(super) fn string(&mut self) -> Result<String, GrammarImportError> {
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

    pub(super) fn character(&mut self) -> Result<char, GrammarImportError> {
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

    pub(super) fn separator(&mut self) -> Result<(), GrammarImportError> {
        self.skip_spaces();
        self.expect(',')?;
        self.skip_spaces();
        Ok(())
    }

    pub(super) fn open(&mut self) -> Result<(), GrammarImportError> {
        self.expect('(')?;
        self.skip_spaces();
        Ok(())
    }

    pub(super) fn close(&mut self) -> Result<(), GrammarImportError> {
        self.skip_spaces();
        self.expect(')')
    }

    pub(super) fn list<T>(
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

    pub(super) fn expression(&mut self) -> Result<GrammarExpr, GrammarImportError> {
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
            "class" | "notClass" => {
                let negated = kind == "notClass";
                class_expression(negated, self.list(Self::class_item)?)
            }
            "ref" => {
                self.open()?;
                let name = self.name()?;
                let mut arguments = Vec::new();
                self.skip_spaces();
                while self.peek() == Some(',') {
                    self.separator()?;
                    arguments.push(self.expression()?);
                    self.skip_spaces();
                }
                self.close()?;
                if arguments.is_empty() {
                    GrammarExpr::NonTerminal(name)
                } else {
                    GrammarExpr::feature(FeatureExpr::Call { name, arguments })
                }
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
            other => {
                if let Some(feature) = self.feature(other)? {
                    return Ok(feature);
                }
                match other {
                    "optional" => GrammarExpr::Optional(self.inner()?),
                    "repeat0" => GrammarExpr::ZeroOrMore(self.inner()?),
                    "repeat1" => GrammarExpr::OneOrMore(self.inner()?),
                    "and" => GrammarExpr::And(self.inner()?),
                    "not" => GrammarExpr::Not(self.inner()?),
                    _ => return Err(self.fail(format!("unknown expression {other}"))),
                }
            }
        })
    }

    fn class_item(&mut self) -> Result<UnicodeClassItem, GrammarImportError> {
        let kind = self.word()?;
        self.open()?;
        let item = match kind.as_str() {
            "char" => UnicodeClassItem::Char(self.character()?),
            "range" => {
                let start = self.character()?;
                self.separator()?;
                UnicodeClassItem::Range(start, self.character()?)
            }
            "category" => UnicodeClassItem::Category(self.string()?),
            "script" => UnicodeClassItem::Script(self.string()?),
            other => return Err(self.fail(format!("unknown class item {other}"))),
        };
        self.close()?;
        Ok(item)
    }

    /// The optional `(NAME, ...)` parameter list after a rule or macro name.
    fn parameters(&mut self) -> Result<Vec<String>, GrammarImportError> {
        if self.peek() != Some('(') {
            return Ok(Vec::new());
        }
        let names = self.list(Self::name)?;
        if names.is_empty() {
            return Err(self.fail("a parameter list names at least one parameter"));
        }
        let mut seen = std::collections::BTreeSet::new();
        if !names.iter().all(|name| seen.insert(name)) {
            return Err(self.fail("a parameter is named twice"));
        }
        Ok(names)
    }

    /// The `channel(NAME)`, `modes(MODE, ...)` and `action(OPERATION, ...)`
    /// attributes after a rule expression; other text is trailing text.
    fn rule_attributes(&mut self) -> Result<RuleAttributes, GrammarImportError> {
        const ORDER: [&str; 3] = ["channel", "modes", "action"];
        let mut attributes = RuleAttributes::default();
        let mut next = 0;
        self.skip_spaces();
        while !self.done() {
            let begin = self.position;
            while self.peek().is_some_and(|value| value.is_ascii_alphabetic()) {
                self.position += 1;
            }
            let attribute = self.text(begin);
            let position = ORDER.iter().position(|word| *word == attribute);
            let Some(position) = position.filter(|_| self.peek() == Some('(')) else {
                self.position = begin;
                return Err(self.fail("unexpected text at the end of the line"));
            };
            if position < next {
                return Err(self.fail(format!("rule attribute {attribute} is out of order")));
            }
            next = position + 1;
            match position {
                0 => {
                    self.open()?;
                    attributes.channel = Some(self.name()?);
                    self.close()?;
                }
                1 => attributes.modes = Some(self.list(Self::name)?),
                _ => attributes.action = Some(self.statements()?),
            }
            self.skip_spaces();
        }
        Ok(attributes)
    }
}
