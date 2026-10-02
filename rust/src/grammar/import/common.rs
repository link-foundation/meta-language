//! Shared text scanning and expression building for the hand-written BNF,
//! EBNF, ABNF, and pest importers. Mirrors `js/src/grammar-importers/common.js`
//! so both runtimes accept the same language and build the same grammar IR.

use std::collections::BTreeSet;

use super::{GrammarImportError, parse_error};
use crate::grammar::{Grammar, GrammarExpr, GrammarFormat, GrammarRule};

/// Largest integer a JavaScript number holds exactly (`Number.MAX_SAFE_INTEGER`).
const MAX_SAFE_INTEGER: u64 = (1 << 53) - 1;

/// Matches one character of the JavaScript `\s` class, which is also the set
/// `String.prototype.trim` removes.
pub(super) const fn is_space(character: char) -> bool {
    matches!(
        character,
        '\t' | '\n' | '\u{b}' | '\u{c}' | '\r' | ' ' | '\u{a0}' | '\u{1680}' | '\u{2000}'
            ..='\u{200a}'
                | '\u{2028}'
                | '\u{2029}'
                | '\u{202f}'
                | '\u{205f}'
                | '\u{3000}'
                | '\u{feff}'
    )
}

/// Matches a character the JavaScript regular-expression `.` does not match.
pub(super) const fn is_line_terminator(character: char) -> bool {
    matches!(character, '\n' | '\r' | '\u{2028}' | '\u{2029}')
}

/// Trims JavaScript whitespace from both ends, as `String.prototype.trim` does.
pub(super) fn trim(text: &str) -> &str {
    text.trim_matches(is_space)
}

/// Splits source text on `\n` and `\r\n`, as `split(/\r?\n/)` does.
pub(super) fn split_lines(text: &str) -> impl Iterator<Item = &str> {
    text.split('\n')
        .map(|line| line.strip_suffix('\r').unwrap_or(line))
}

/// Quotes a string the way `JSON.stringify` does, for diagnostics.
pub(super) fn json_quote(value: &str) -> String {
    serde_json::Value::String(value.to_owned()).to_string()
}

/// Cuts `line` at the first `marker` outside a single- or double-quoted
/// literal. With `backslash_escapes`, a backslash inside a literal escapes the
/// next character.
pub(super) fn strip_line_comment(line: &str, marker: char, backslash_escapes: bool) -> &str {
    let mut quote = None;
    let mut escaped = false;
    for (index, character) in line.char_indices() {
        if escaped {
            escaped = false;
            continue;
        }
        if quote.is_some() && backslash_escapes && character == '\\' {
            escaped = true;
            continue;
        }
        if quote == Some(character) {
            quote = None;
        } else if quote.is_none() && matches!(character, '"' | '\'') {
            quote = Some(character);
        } else if quote.is_none() && character == marker {
            return &line[..index];
        }
    }
    line
}

/// Flattens nested sequences and drops empty items, as the JavaScript
/// `sequence` builder does.
pub(super) fn sequence(items: Vec<GrammarExpr>) -> GrammarExpr {
    let mut flattened = Vec::with_capacity(items.len());
    for item in items {
        match item {
            GrammarExpr::Empty => {}
            GrammarExpr::Sequence(nested) => flattened.extend(nested),
            item => flattened.push(item),
        }
    }
    match flattened.len() {
        0 => GrammarExpr::Empty,
        1 => flattened.remove(0),
        _ => GrammarExpr::Sequence(flattened),
    }
}

/// Flattens nested choices of the same ordering, as the JavaScript `choice`
/// builder does.
pub(super) fn choice(alternatives: Vec<GrammarExpr>, ordered: bool) -> GrammarExpr {
    let mut flattened = Vec::with_capacity(alternatives.len());
    for alternative in alternatives {
        match alternative {
            GrammarExpr::Choice {
                ordered: nested_ordered,
                alternatives: nested,
            } if nested_ordered == ordered => flattened.extend(nested),
            alternative => flattened.push(alternative),
        }
    }
    match flattened.len() {
        0 => GrammarExpr::Empty,
        1 => flattened.remove(0),
        _ => GrammarExpr::Choice {
            ordered,
            alternatives: flattened,
        },
    }
}

/// An unordered choice, or [`GrammarExpr::Empty`] when every alternative is
/// empty, as the JavaScript BNF and EBNF importers build alternatives.
pub(super) fn alternatives_choice(alternatives: Vec<GrammarExpr>) -> GrammarExpr {
    if alternatives
        .iter()
        .all(|alternative| *alternative == GrammarExpr::Empty)
    {
        GrammarExpr::Empty
    } else {
        choice(alternatives, false)
    }
}

/// A terminal, or [`GrammarExpr::Empty`] for the empty string.
pub(super) fn terminal_or_empty(value: String) -> GrammarExpr {
    if value.is_empty() {
        GrammarExpr::Empty
    } else {
        GrammarExpr::Terminal(value)
    }
}

/// Lowers counted repetition to its most specific expression variant, as the
/// JavaScript `canonicalRepeat` builder does, rejecting inverted or
/// out-of-range bounds.
pub(super) fn canonical_repeat(
    format: GrammarFormat,
    expr: GrammarExpr,
    min: u64,
    max: Option<u64>,
) -> Result<GrammarExpr, GrammarImportError> {
    let invalid = || {
        let max = max.map_or_else(String::new, |max| max.to_string());
        parse_error(format, format!("invalid repetition bounds {min}..{max}"))
    };
    if min > MAX_SAFE_INTEGER || max.is_some_and(|max| max > MAX_SAFE_INTEGER || max < min) {
        return Err(invalid());
    }
    let bound = |value: u64| usize::try_from(value).map_err(|_| invalid());
    Ok(match (min, max) {
        (0, None) => GrammarExpr::zero_or_more(expr),
        (1, None) => GrammarExpr::one_or_more(expr),
        (0, Some(1)) => GrammarExpr::optional(expr),
        (min, max) => GrammarExpr::repeat(expr, bound(min)?, max.map(bound).transpose()?),
    })
}

/// Parses ASCII decimal digits, saturating past the JavaScript safe-integer
/// range so [`canonical_repeat`] rejects the bound as JavaScript does.
pub(super) fn decimal(digits: &str) -> u64 {
    digits.bytes().fold(0, |value: u64, digit| {
        value
            .saturating_mul(10)
            .saturating_add(u64::from(digit - b'0'))
            .min(MAX_SAFE_INTEGER + 1)
    })
}

/// Builds the imported grammar: rejects an empty or duplicated rule list and
/// the alphabetically first reference that names neither a rule nor one of
/// the `allowed` built-ins, as the JavaScript `grammarFromRules` does.
pub(super) fn grammar_from_rules(
    format: GrammarFormat,
    rules: Vec<GrammarRule>,
    allowed: &[&str],
) -> Result<Grammar, GrammarImportError> {
    if rules.is_empty() {
        return Err(parse_error(format, "grammar contains no rules"));
    }
    let mut names = BTreeSet::new();
    for rule in &rules {
        if !names.insert(rule.name.as_str()) {
            return Err(parse_error(format, format!("duplicate rule {}", rule.name)));
        }
    }
    let mut grammar = Grammar::new().with_source_format(format);
    for rule in rules {
        grammar.add_rule(rule);
    }
    if let Some(missing) = grammar
        .undefined_nonterminals()
        .into_iter()
        .find(|name| !allowed.contains(&name.as_str()))
    {
        return Err(parse_error(
            format,
            format!("undefined non-terminal {missing}"),
        ));
    }
    Ok(grammar)
}

/// Collects referenced rule names once each, in pre-order.
pub(super) fn collect_references(expr: &GrammarExpr, names: &mut Vec<String>) {
    match expr {
        GrammarExpr::NonTerminal(name) => {
            if !names.contains(name) {
                names.push(name.clone());
            }
        }
        GrammarExpr::Choice {
            alternatives: items,
            ..
        }
        | GrammarExpr::Sequence(items) => {
            for item in items {
                collect_references(item, names);
            }
        }
        GrammarExpr::Optional(expr)
        | GrammarExpr::ZeroOrMore(expr)
        | GrammarExpr::OneOrMore(expr)
        | GrammarExpr::And(expr)
        | GrammarExpr::Not(expr)
        | GrammarExpr::Capture { expr, .. }
        | GrammarExpr::Repeat { expr, .. } => collect_references(expr, names),
        GrammarExpr::Feature(feature) => {
            feature.for_each_expression(&mut |inner| collect_references(inner, names));
        }
        GrammarExpr::Empty
        | GrammarExpr::Terminal(_)
        | GrammarExpr::TerminalInsensitive(_)
        | GrammarExpr::CharRange(_, _)
        | GrammarExpr::CharClass { .. }
        | GrammarExpr::AnyChar => {}
    }
}

/// Decodes the character after a backslash inside a quoted literal.
pub(super) type Escapes = fn(char, char, GrammarFormat) -> Result<String, GrammarImportError>;

/// The default escape set: `\n`, `\r`, and `\t` decode, any other character
/// stands for itself.
#[allow(clippy::unnecessary_wraps)]
pub(super) fn default_escapes(
    escaped: char,
    _quote: char,
    _format: GrammarFormat,
) -> Result<String, GrammarImportError> {
    Ok(match escaped {
        'n' => "\n".to_string(),
        'r' => "\r".to_string(),
        't' => "\t".to_string(),
        other => other.to_string(),
    })
}

/// A character cursor over one importer's source text. Offsets in
/// diagnostics count UTF-16 code units, as the JavaScript importers report.
#[derive(Clone, Debug)]
pub(super) struct Cursor<'source> {
    pub(super) source: &'source str,
    pub(super) format: GrammarFormat,
    /// Byte offset of the next character; may pass the end by one after
    /// [`Cursor::take`] reads past the end, as JavaScript offsets do.
    pub(super) offset: usize,
    /// Skips pest `//` line and `/* */` block comments as whitespace.
    comments: bool,
}

impl<'source> Cursor<'source> {
    pub(super) const fn new(source: &'source str, format: GrammarFormat) -> Self {
        Self {
            source,
            format,
            offset: 0,
            comments: false,
        }
    }

    /// A cursor that also treats pest `//` and `/* */` comments as whitespace.
    pub(super) const fn with_comments(source: &'source str, format: GrammarFormat) -> Self {
        Self {
            source,
            format,
            offset: 0,
            comments: true,
        }
    }

    pub(super) const fn eof(&self) -> bool {
        self.offset >= self.source.len()
    }

    /// The unread text.
    pub(super) fn rest(&self) -> &'source str {
        self.source.get(self.offset..).unwrap_or("")
    }

    pub(super) fn peek(&self) -> Option<char> {
        self.rest().chars().next()
    }

    pub(super) fn starts_with(&self, value: &str) -> bool {
        self.rest().starts_with(value)
    }

    /// Advances past `bytes` bytes of the unread text.
    pub(super) const fn advance(&mut self, bytes: usize) {
        self.offset += bytes;
    }

    /// Reads one character; at the end it still advances, as JavaScript does.
    pub(super) fn take(&mut self) -> Option<char> {
        let character = self.peek();
        self.offset += character.map_or(1, char::len_utf8);
        character
    }

    pub(super) fn skip_space(&mut self) -> Result<(), GrammarImportError> {
        loop {
            while self.peek().is_some_and(is_space) {
                self.take();
            }
            if !self.comments {
                return Ok(());
            }
            if self.starts_with("//") {
                self.offset = self.rest()[2..]
                    .find('\n')
                    .map_or(self.source.len(), |end| self.offset + 2 + end + 1);
            } else if self.starts_with("/*") {
                let Some(end) = self.rest()[2..].find("*/") else {
                    return Err(self.error("unterminated block comment"));
                };
                self.offset += 2 + end + 2;
            } else {
                return Ok(());
            }
        }
    }

    pub(super) fn consume(&mut self, value: &str) -> Result<(), GrammarImportError> {
        if self.try_consume(value)? {
            Ok(())
        } else {
            Err(parse_error(
                self.format,
                format!(
                    "expected {} at offset {}",
                    json_quote(value),
                    self.position()
                ),
            ))
        }
    }

    pub(super) fn try_consume(&mut self, value: &str) -> Result<bool, GrammarImportError> {
        self.skip_space()?;
        if !self.starts_with(value) {
            return Ok(false);
        }
        self.offset += value.len();
        Ok(true)
    }

    /// Reads an identifier of one `first` character and any `rest` characters.
    pub(super) fn identifier_with(
        &mut self,
        first: fn(char) -> bool,
        rest: fn(char) -> bool,
    ) -> Result<String, GrammarImportError> {
        self.skip_space()?;
        let text = self.rest();
        if !text.chars().next().is_some_and(first) {
            return Err(parse_error(
                self.format,
                format!("expected identifier at offset {}", self.position()),
            ));
        }
        let length = text
            .char_indices()
            .skip(1)
            .find(|&(_, character)| !rest(character))
            .map_or(text.len(), |(index, _)| index);
        self.offset += length;
        Ok(text[..length].to_string())
    }

    /// Reads an identifier matching `[A-Za-z_][A-Za-z0-9_-]*`.
    pub(super) fn identifier(&mut self) -> Result<String, GrammarImportError> {
        self.identifier_with(
            |character| character.is_ascii_alphabetic() || character == '_',
            |character| character.is_ascii_alphanumeric() || matches!(character, '_' | '-'),
        )
    }

    /// Reads a single- or double-quoted literal. `decode` decodes the
    /// character after a backslash; `None` keeps backslashes verbatim.
    pub(super) fn quoted(&mut self, decode: Option<Escapes>) -> Result<String, GrammarImportError> {
        self.skip_space()?;
        let start = self.position();
        let quote = self.take();
        let Some(quote @ ('"' | '\'')) = quote else {
            return Err(parse_error(
                self.format,
                format!("expected quoted literal at offset {start}"),
            ));
        };
        let mut result = String::new();
        while let Some(character) = self.take() {
            if character == quote {
                return Ok(result);
            }
            let Some(decode) = decode.filter(|_| character == '\\') else {
                result.push(character);
                continue;
            };
            let Some(escaped) = self.take() else {
                return Err(parse_error(self.format, "unterminated string escape"));
            };
            result.push_str(&decode(escaped, quote, self.format)?);
        }
        Err(parse_error(self.format, "unterminated string literal"))
    }

    /// The offset in UTF-16 code units, as the JavaScript importers report it.
    pub(super) fn position(&self) -> usize {
        let within = self.offset.min(self.source.len());
        self.source[..within].encode_utf16().count() + (self.offset - within)
    }

    pub(super) fn error(&self, message: &str) -> GrammarImportError {
        parse_error(
            self.format,
            format!("{message} at offset {}", self.position()),
        )
    }
}
