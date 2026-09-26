//! Tokenizer shared by the portable-core translation frontends.
//!
//! Every token keeps its UTF-16 offsets so diagnostics and source mappings agree with the
//! JavaScript runtime; comments are retained separately because `JSDoc` types
//! are part of the JavaScript frontend's input.
//!
//! Mirrors `js/src/translation/lexer.js`.

use std::sync::OnceLock;

use regex::Regex;

use super::diagnostics::{Result, TranslationError};
use super::{Language, Span};

/// Source text addressed by UTF-16 code units, as JavaScript strings are.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Source {
    units: Vec<u16>,
}

impl Source {
    #[must_use]
    pub fn new(text: &str) -> Self {
        Self {
            units: text.encode_utf16().collect(),
        }
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.units.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.units.is_empty()
    }

    #[must_use]
    pub fn units(&self) -> &[u16] {
        &self.units
    }

    /// The code unit at `index`, if any.
    #[must_use]
    pub fn at(&self, index: usize) -> Option<u16> {
        self.units.get(index).copied()
    }

    /// The code unit at `index` as a character; lone surrogates are `None`.
    #[must_use]
    pub fn char_at(&self, index: usize) -> Option<char> {
        self.at(index)
            .and_then(|unit| char::from_u32(u32::from(unit)))
    }

    /// True when the unit at `index` is the ASCII or BMP character `ch`.
    #[must_use]
    pub fn is_char(&self, index: usize, ch: char) -> bool {
        self.char_at(index) == Some(ch)
    }

    #[must_use]
    pub fn starts_with(&self, pattern: &str, index: usize) -> bool {
        pattern
            .encode_utf16()
            .enumerate()
            .all(|(offset, unit)| self.at(index + offset) == Some(unit))
    }

    /// `source.slice(start, end)`, clamped like JavaScript's.
    #[must_use]
    pub fn slice(&self, start: usize, end: usize) -> String {
        let end = end.min(self.units.len());
        let start = start.min(end);
        String::from_utf16_lossy(&self.units[start..end])
    }

    /// `source.indexOf(pattern, from)`.
    #[must_use]
    pub fn index_of(&self, pattern: &str, from: usize) -> Option<usize> {
        (from..self.units.len()).find(|&index| self.starts_with(pattern, index))
    }

    /// `String.fromCodePoint(source.codePointAt(index))`: the whole character
    /// when `index` starts a surrogate pair.
    #[must_use]
    pub fn code_point_text(&self, index: usize) -> String {
        let high = self.units[index];
        if (0xD800..0xDC00).contains(&high) {
            if let Some(low) = self.at(index + 1) {
                if (0xDC00..0xE000).contains(&low) {
                    return String::from_utf16_lossy(&self.units[index..index + 2]);
                }
            }
        }
        String::from_utf16_lossy(&self.units[index..=index])
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TokenKind {
    Number,
    Identifier,
    Macro,
    Punct,
    String,
    Interpolation,
    Template,
    Eof,
}

/// A `${…}` substitution in a JavaScript template literal.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TemplateExpression {
    pub source: String,
    pub offset: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TemplatePart {
    pub text: String,
    pub expression: Option<TemplateExpression>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Token {
    pub kind: TokenKind,
    pub value: String,
    /// The letters after a number's digits (`n`, `u32`, `x1fn`).
    pub suffix: String,
    pub raw: String,
    pub start: usize,
    pub end: usize,
    pub parts: Vec<TemplatePart>,
    /// Layout positions, filled in by the Lean frontend.
    pub line: usize,
    pub col: usize,
    pub first: bool,
}

impl Token {
    const fn new(kind: TokenKind, value: String, raw: String, start: usize, end: usize) -> Self {
        Self {
            kind,
            value,
            suffix: String::new(),
            raw,
            start,
            end,
            parts: Vec::new(),
            line: 0,
            col: 0,
            first: false,
        }
    }

    #[must_use]
    pub const fn span(&self) -> Span {
        Span {
            start: self.start,
            end: self.end,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Comment {
    pub text: String,
    pub start: usize,
    pub end: usize,
}

#[derive(Clone, Debug)]
pub struct Tokens {
    pub tokens: Vec<Token>,
    pub comments: Vec<Comment>,
}

fn operators(language: Language) -> Vec<&'static str> {
    let mut list: Vec<&'static str> = match language {
        Language::JavaScript => vec![
            ">>>=", "===", "!==", "...", "**=", "<<=", ">>=", ">>>", "&&=", "||=", "??=", "**",
            "&&", "||", "==", "!=", "<=", ">=", "=>", "+=", "-=", "*=", "/=", "%=", "&=", "|=",
            "^=", "++", "--", "??", "?.", "<<", ">>",
        ],
        Language::Rust => vec![
            "..=", "::", "->", "=>", "==", "!=", "<=", ">=", "&&", "||", "+=", "-=", "*=", "/=",
            "%=", "..",
        ],
        Language::Lean => vec![
            ":=", "++", "->", "=>", "==", "!=", "<=", ">=", "&&", "||", "<;>", "<|", "|>", "::",
            "..", "←", "→", "∀", "∧", "≤", "≥", "≠", "×",
        ],
        Language::Rocq => vec![
            ":=", "=>", "->", "<->", "<=?", "<?", "=?", "<=", ">=", "<>", "/\\", "\\/", "&&", "||",
            "++", "::", "%",
        ],
    };
    // Longest first, keeping the listed order among equal lengths (a stable
    // sort by UTF-16 length, like the JavaScript runtime's).
    list.sort_by_key(|operator| std::cmp::Reverse(operator.encode_utf16().count()));
    list
}

struct CommentSyntax {
    line: Option<&'static str>,
    open: &'static str,
    close: &'static str,
    nested: bool,
}

const fn comment_syntax(language: Language) -> CommentSyntax {
    match language {
        Language::JavaScript => CommentSyntax {
            line: Some("//"),
            open: "/*",
            close: "*/",
            nested: false,
        },
        Language::Rust => CommentSyntax {
            line: Some("//"),
            open: "/*",
            close: "*/",
            nested: true,
        },
        Language::Lean => CommentSyntax {
            line: Some("--"),
            open: "/-",
            close: "-/",
            nested: true,
        },
        Language::Rocq => CommentSyntax {
            line: None,
            open: "(*",
            close: "*)",
            nested: true,
        },
    }
}

/// JavaScript's `/\s/u` on one code unit.
#[must_use]
pub const fn is_js_space(unit: u16) -> bool {
    matches!(
        unit,
        0x09..=0x0D
            | 0x20
            | 0xA0
            | 0x1680
            | 0x2000..=0x200A
            | 0x2028
            | 0x2029
            | 0x202F
            | 0x205F
            | 0x3000
            | 0xFEFF
    )
}

fn letter_regex() -> &'static Regex {
    static LETTER: OnceLock<Regex> = OnceLock::new();
    LETTER.get_or_init(|| Regex::new(r"^\p{L}$").expect("letter class"))
}

fn letter_or_number_regex() -> &'static Regex {
    static LETTER: OnceLock<Regex> = OnceLock::new();
    LETTER.get_or_init(|| Regex::new(r"^[\p{L}\p{N}]$").expect("letter or number class"))
}

const LEAN_SYMBOLS: &str = "∀→∧≤≥≠λ×";

fn unicode_test(unit: Option<u16>, regex: &Regex) -> bool {
    unit.and_then(|unit| char::from_u32(u32::from(unit)))
        .is_some_and(|ch| !LEAN_SYMBOLS.contains(ch) && regex.is_match(ch.encode_utf8(&mut [0; 4])))
}

fn is_identifier_start(unit: Option<u16>, language: Language) -> bool {
    let Some(value) = unit else { return false };
    if u8::try_from(value).is_ok_and(|byte| byte.is_ascii_alphabetic() || byte == b'_') {
        return true;
    }
    if language == Language::JavaScript && value == u16::from(b'$') {
        return true;
    }
    matches!(language, Language::Lean | Language::Rocq) && unicode_test(unit, letter_regex())
}

fn is_identifier_part(unit: Option<u16>, language: Language) -> bool {
    let Some(value) = unit else { return false };
    if u8::try_from(value).is_ok_and(|byte| byte.is_ascii_alphanumeric() || byte == b'_') {
        return true;
    }
    let is = |ch: u8| value == u16::from(ch);
    if language == Language::JavaScript && is(b'$') {
        return true;
    }
    if language == Language::Rocq && is(b'\'') {
        return true;
    }
    if language == Language::Lean && (is(b'\'') || is(b'!') || is(b'?')) {
        return true;
    }
    matches!(language, Language::Lean | Language::Rocq)
        && unicode_test(unit, letter_or_number_regex())
}

const fn span(start: usize, end: usize) -> Span {
    Span { start, end }
}

/// Splits `text` into tokens and comments.
///
/// # Errors
///
/// Returns a syntax error for malformed numbers, unsupported escapes and
/// unterminated strings, template literals or block comments.
pub fn tokenize(text: &str, language: Language) -> Result<Tokens> {
    tokenize_source(&Source::new(text), language)
}

/// [`tokenize`] over an already encoded source.
///
/// # Errors
///
/// See [`tokenize`].
pub fn tokenize_source(source: &Source, language: Language) -> Result<Tokens> {
    let operators = operators(language);
    let comments = comment_syntax(language);
    let mut tokens: Vec<Token> = Vec::new();
    let mut comment_list = Vec::new();
    let mut index = 0;
    while index < source.len() {
        let unit = source.units[index];
        if is_js_space(unit) {
            index += 1;
            continue;
        }
        if let Some(line) = comments.line {
            if source.starts_with(line, index) {
                let stop = source.index_of("\n", index).unwrap_or(source.len());
                comment_list.push(Comment {
                    text: source.slice(index, stop),
                    start: index,
                    end: stop,
                });
                index = stop;
                continue;
            }
        }
        if source.starts_with(comments.open, index) {
            let stop = block_comment_end(source, index, &comments)?;
            comment_list.push(Comment {
                text: source.slice(index, stop),
                start: index,
                end: stop,
            });
            index = stop;
            continue;
        }
        let start = index;
        let ch = source.char_at(index);
        if language == Language::JavaScript && ch == Some('`') {
            let token = template_token(source, index)?;
            index = token.end;
            tokens.push(token);
            continue;
        }
        if ch == Some('"') || (language == Language::JavaScript && ch == Some('\'')) {
            let token = string_token(source, index, language)?;
            index = token.end;
            tokens.push(token);
            continue;
        }
        if language == Language::Lean && source.starts_with("s!\"", index) {
            let mut token = string_token(source, index + 2, language)?;
            token.kind = TokenKind::Interpolation;
            token.start = start;
            token.raw = source.slice(start, token.end);
            index = token.end;
            tokens.push(token);
            continue;
        }
        if ch.is_some_and(|ch| ch.is_ascii_digit()) {
            let mut stop = index;
            while source
                .char_at(stop)
                .is_some_and(|ch| ch.is_ascii_digit() || ch == '_')
            {
                stop += 1;
            }
            let digits_end = stop;
            if source
                .char_at(stop)
                .is_some_and(|ch| ch.is_ascii_lowercase())
            {
                stop += 1;
                while source
                    .char_at(stop)
                    .is_some_and(|ch| ch.is_ascii_lowercase() || ch.is_ascii_digit())
                {
                    stop += 1;
                }
            }
            let raw = source.slice(start, stop);
            let digits = source.slice(start, digits_end);
            if digits.ends_with('_') {
                return Err(TranslationError::syntax(
                    format!("malformed number {raw}"),
                    Some(span(start, stop)),
                ));
            }
            let mut token =
                Token::new(TokenKind::Number, digits.replace('_', ""), raw, start, stop);
            token.suffix = source.slice(digits_end, stop);
            tokens.push(token);
            index = stop;
            continue;
        }
        if is_identifier_start(Some(unit), language) {
            let mut stop = index + 1;
            loop {
                while stop < source.len() && is_identifier_part(source.at(stop), language) {
                    stop += 1;
                }
                // Lean and Rocq qualified names (`Tree.node`, `Nat.add`) are single tokens.
                let qualified = matches!(language, Language::Lean | Language::Rocq)
                    && source.is_char(stop, '.')
                    && stop + 1 < source.len()
                    && is_identifier_start(source.at(stop + 1), language);
                if !qualified {
                    break;
                }
                stop += 2;
            }
            if language == Language::Rust
                && source.is_char(stop, '!')
                && !source.is_char(stop + 1, '=')
            {
                tokens.push(Token::new(
                    TokenKind::Macro,
                    source.slice(index, stop),
                    source.slice(index, stop + 1),
                    start,
                    stop + 1,
                ));
                index = stop + 1;
                continue;
            }
            let value = source.slice(index, stop);
            tokens.push(Token::new(
                TokenKind::Identifier,
                value.clone(),
                value,
                start,
                stop,
            ));
            index = stop;
            continue;
        }
        let text = operators
            .iter()
            .find(|operator| source.starts_with(operator, index))
            .map_or_else(
                || source.code_point_text(index),
                |operator| (*operator).to_owned(),
            );
        let length = text.encode_utf16().count();
        tokens.push(Token::new(
            TokenKind::Punct,
            text.clone(),
            text,
            start,
            start + length,
        ));
        index += length;
    }
    tokens.push(Token::new(
        TokenKind::Eof,
        String::new(),
        String::new(),
        source.len(),
        source.len(),
    ));
    Ok(Tokens {
        tokens,
        comments: comment_list,
    })
}

fn block_comment_end(source: &Source, index: usize, comments: &CommentSyntax) -> Result<usize> {
    let open = comments.open.encode_utf16().count();
    let close = comments.close.encode_utf16().count();
    let mut depth = 0usize;
    let mut cursor = index;
    while cursor < source.len() {
        if source.starts_with(comments.open, cursor) {
            depth += 1;
            cursor += open;
            if !comments.nested && depth > 1 {
                depth = 1;
            }
            continue;
        }
        if source.starts_with(comments.close, cursor) {
            // A close before any open (impossible from `tokenize`) would go negative in JavaScript.
            depth = depth.saturating_sub(1);
            cursor += close;
            if depth == 0 {
                return Ok(cursor);
            }
            continue;
        }
        cursor += 1;
    }
    Err(TranslationError::syntax(
        "unterminated block comment",
        Some(span(index, source.len())),
    ))
}

fn string_token(source: &Source, index: usize, language: Language) -> Result<Token> {
    let quote = source.units[index];
    let mut cursor = index + 1;
    let mut value: Vec<u16> = Vec::new();
    while cursor < source.len() {
        let unit = source.units[cursor];
        if unit == quote {
            if language == Language::Rocq && source.is_char(cursor + 1, '"') {
                value.push(u16::from(b'"'));
                cursor += 2;
                continue;
            }
            let mut token = Token::new(
                TokenKind::String,
                String::from_utf16_lossy(&value),
                source.slice(index, cursor + 1),
                index,
                cursor + 1,
            );
            token.raw = source.slice(index, cursor + 1);
            return Ok(token);
        }
        if unit == u16::from(b'\\') && language != Language::Rocq {
            let escaped = source.char_at(cursor + 1);
            let replacement = match escaped {
                Some('n') => "\n",
                Some('t') => "\t",
                Some('r') => "\r",
                Some('\\') => "\\",
                Some('"') => "\"",
                Some('\'') => "'",
                Some('0') => "\0",
                Some('{') => "\\{",
                _ => {
                    return Err(TranslationError::syntax(
                        format!(
                            "unsupported string escape \\{}",
                            escaped_text(source, cursor + 1)
                        ),
                        Some(span(cursor, cursor + 2)),
                    ));
                }
            };
            value.extend(replacement.encode_utf16());
            cursor += 2;
            continue;
        }
        if unit == u16::from(b'\n') && language == Language::JavaScript {
            break;
        }
        value.push(unit);
        cursor += 1;
    }
    Err(TranslationError::syntax(
        "unterminated string literal",
        Some(span(index, source.len())),
    ))
}

fn template_token(source: &Source, index: usize) -> Result<Token> {
    let mut parts = Vec::new();
    let mut cursor = index + 1;
    let mut text: Vec<u16> = Vec::new();
    while cursor < source.len() {
        let unit = source.units[cursor];
        if unit == u16::from(b'`') {
            parts.push(TemplatePart {
                text: String::from_utf16_lossy(&text),
                expression: None,
            });
            let raw = source.slice(index, cursor + 1);
            let mut token = Token::new(TokenKind::Template, String::new(), raw, index, cursor + 1);
            token.parts = parts;
            return Ok(token);
        }
        if unit == u16::from(b'\\') {
            let replacement = match source.char_at(cursor + 1) {
                Some('n') => '\n',
                Some('t') => '\t',
                Some('\\') => '\\',
                Some('`') => '`',
                Some('$') => '$',
                _ => {
                    return Err(TranslationError::syntax(
                        format!(
                            "unsupported template escape \\{}",
                            escaped_text(source, cursor + 1)
                        ),
                        Some(span(cursor, cursor + 2)),
                    ));
                }
            };
            text.push(replacement as u16);
            cursor += 2;
            continue;
        }
        if source.starts_with("${", cursor) {
            let close = matching_brace(source, cursor + 1)?;
            parts.push(TemplatePart {
                text: String::from_utf16_lossy(&text),
                expression: Some(TemplateExpression {
                    source: source.slice(cursor + 2, close),
                    offset: cursor + 2,
                }),
            });
            text.clear();
            cursor = close + 1;
            continue;
        }
        text.push(unit);
        cursor += 1;
    }
    Err(TranslationError::syntax(
        "unterminated template literal",
        Some(span(index, source.len())),
    ))
}

/// The escaped unit as JavaScript interpolates it (`undefined` past the end).
fn escaped_text(source: &Source, index: usize) -> String {
    if index < source.len() {
        source.slice(index, index + 1)
    } else {
        "undefined".to_owned()
    }
}

/// The index of the `}` closing the `{` at `open`.
///
/// # Errors
///
/// Returns a syntax error when the braces are unbalanced.
pub fn matching_brace(source: &Source, open: usize) -> Result<usize> {
    let mut depth = 0i64;
    for cursor in open..source.len() {
        if source.is_char(cursor, '{') {
            depth += 1;
        }
        if source.is_char(cursor, '}') {
            depth -= 1;
            if depth == 0 {
                return Ok(cursor);
            }
        }
    }
    Err(TranslationError::syntax(
        "unbalanced braces",
        Some(span(open, source.len())),
    ))
}

/// Cursor over a token list with the helpers every frontend needs.
#[derive(Clone, Debug)]
pub struct TokenCursor {
    pub tokens: Vec<Token>,
    pub language: Language,
    pub index: usize,
}

impl TokenCursor {
    #[must_use]
    pub const fn new(tokens: Vec<Token>, language: Language) -> Self {
        Self {
            tokens,
            language,
            index: 0,
        }
    }

    #[must_use]
    pub fn peek(&self) -> &Token {
        self.peek_at(0)
    }

    #[must_use]
    pub fn peek_at(&self, offset: usize) -> &Token {
        &self.tokens[(self.index + offset).min(self.tokens.len() - 1)]
    }

    pub fn advance(&mut self) -> Token {
        let token = self.peek().clone();
        if token.kind != TokenKind::Eof {
            self.index += 1;
        }
        token
    }

    #[must_use]
    pub fn is(&self, value: &str) -> bool {
        self.is_at(value, 0)
    }

    #[must_use]
    pub fn is_at(&self, value: &str, offset: usize) -> bool {
        let token = self.peek_at(offset);
        matches!(token.kind, TokenKind::Punct | TokenKind::Identifier) && token.value == value
    }

    #[must_use]
    pub fn is_kind(&self, kind: TokenKind) -> bool {
        self.peek().kind == kind
    }

    #[must_use]
    pub fn is_kind_at(&self, kind: TokenKind, offset: usize) -> bool {
        self.peek_at(offset).kind == kind
    }

    pub fn eat(&mut self, value: &str) -> Option<Token> {
        if self.is(value) {
            Some(self.advance())
        } else {
            None
        }
    }

    /// Consumes `value`.
    ///
    /// # Errors
    ///
    /// Returns a syntax error naming the token found instead.
    pub fn expect(&mut self, value: &str, context: Option<&str>) -> Result<Token> {
        if self.is(value) {
            return Ok(self.advance());
        }
        let token = self.peek();
        Err(TranslationError::syntax(
            format!(
                "expected {value}{} but found {}",
                context.map_or_else(String::new, |context| format!(" in {context}")),
                describe(token)
            ),
            Some(token.span()),
        ))
    }

    /// Consumes an identifier.
    ///
    /// # Errors
    ///
    /// Returns a syntax error naming the token found instead.
    pub fn identifier(&mut self, context: Option<&str>) -> Result<Token> {
        let token = self.peek();
        if token.kind != TokenKind::Identifier {
            return Err(TranslationError::syntax(
                format!(
                    "expected identifier{} but found {}",
                    context.map_or_else(String::new, |context| format!(" in {context}")),
                    describe(token)
                ),
                Some(token.span()),
            ));
        }
        Ok(self.advance())
    }

    #[must_use]
    pub fn at_end(&self) -> bool {
        self.peek().kind == TokenKind::Eof
    }
}

/// `JSON.stringify(token.raw)`, or `end of input`.
#[must_use]
pub fn describe(token: &Token) -> String {
    if token.kind == TokenKind::Eof {
        "end of input".to_owned()
    } else {
        json_string(&token.raw)
    }
}

/// `JSON.stringify` of a string.
#[must_use]
pub fn json_string(text: &str) -> String {
    serde_json::to_string(text).unwrap_or_default()
}
