//! Tokenizer shared by the portable-core translation frontends.
//!
//! Every token keeps its UTF-16 offsets so diagnostics and source mappings agree with the
//! JavaScript runtime; comments are retained separately because `JSDoc` types
//! are part of the JavaScript frontend's input.
//!
//! Mirrors `js/src/translation/lexer.js`.

use std::sync::OnceLock;

use regex::Regex;

use super::decimal::Decimal;
use super::diagnostics::{ErrorKind, Result, TranslationError};
use super::frontend_rules::{
    UnicodeEscape, accept_control_parenthesis, accept_statement_brace, decode_unicode_escape,
    read_rust_number_bounds, regular_expression_end, start_regular_expression,
};
use super::js_number::js_number_literal;
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
    pub const fn len(&self) -> usize {
        self.units.len()
    }

    #[must_use]
    pub const fn is_empty(&self) -> bool {
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
        if (0xD800..0xDC00).contains(&high)
            && let Some(low) = self.at(index + 1)
            && (0xDC00..0xE000).contains(&low)
        {
            return String::from_utf16_lossy(&self.units[index..index + 2]);
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
    RegularExpression,
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
    /// A decimal or explicitly suffixed binary floating-point literal.
    pub floating: bool,
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
            floating: false,
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
    tokenize_region(source, language, 0, false, &mut None)
}

// Use ordinary lexical decisions for template substitutions: opaque tokens
// cannot close their surrounding expression. Offsets stay in the full source.
fn tokenize_region(
    source: &Source,
    language: Language,
    start: usize,
    braced: bool,
    units: &mut Option<Vec<f64>>,
) -> Result<Tokens> {
    let operators = operators(language);
    let comments = comment_syntax(language);
    let mut tokens: Vec<Token> = Vec::new();
    let mut comment_list = Vec::new();
    // Encode lazily once; copying each suffix is quadratic in modules with
    // many regular-expression literals. The shared rule uses absolute offsets.
    let mut scopes = Vec::new();
    let mut braces = Vec::new();
    let mut closed = false;
    let mut declaration_body = false;
    let mut depth = 0usize;
    let mut index = start;
    while index < source.len() {
        let unit = source.units[index];
        if braced && source.is_char(index, '}') && depth == 0 {
            break;
        }
        if is_js_space(unit) {
            index += 1;
            continue;
        }
        if let Some(line) = comments.line
            && source.starts_with(line, index)
        {
            let stop = source.index_of("\n", index).unwrap_or(source.len());
            comment_list.push(Comment {
                text: source.slice(index, stop),
                start: index,
                end: stop,
            });
            index = stop;
            continue;
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
        if language == Language::JavaScript && ch == Some('/') {
            let previous = tokens.last();
            let kind = previous.map_or("", |token| match token.kind {
                TokenKind::Identifier => "identifier",
                TokenKind::Punct => "punct",
                _ => "other",
            });
            let kind = if tokens
                .iter()
                .rev()
                .nth(1)
                .is_some_and(|token| token.value == "." || token.value == "?.")
            {
                "member"
            } else {
                kind
            };
            let value = previous.map_or("", |token| token.value.as_str());
            if closed || start_regular_expression(kind, value) {
                let units = units.get_or_insert_with(|| {
                    source.units().iter().map(|unit| f64::from(*unit)).collect()
                });
                #[allow(clippy::cast_precision_loss)]
                let end = regular_expression_end(units, index as f64);
                if end < 0.0 {
                    return Err(TranslationError::syntax(
                        "unterminated regular expression literal",
                        Some(span(start, source.len())),
                    ));
                }
                #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
                let end = end as usize;
                tokens.push(Token::new(
                    TokenKind::RegularExpression,
                    "/".to_owned(),
                    source.slice(index, end),
                    start,
                    end,
                ));
                index = end;
                closed = false;
                continue;
            }
        }
        let control_closed = closed;
        closed = false;
        if language == Language::JavaScript && ch == Some('`') {
            let token = template_token(source, index, units)?;
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
        if language == Language::JavaScript
            && (ch.is_some_and(|ch| ch.is_ascii_digit())
                || (ch == Some('.')
                    && source
                        .char_at(index + 1)
                        .is_some_and(|ch| ch.is_ascii_digit())))
        {
            let token = javascript_number(source, index)?;
            index = token.end;
            tokens.push(token);
            continue;
        }
        if ch.is_some_and(|ch| ch.is_ascii_digit()) {
            if language == Language::Rust {
                let token = rust_number(source, index, units)?;
                index = token.end;
                tokens.push(token);
                continue;
            }
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
        if braced && text == "{" {
            depth += 1;
        }
        if braced && text == "}" {
            depth -= 1;
        }
        if language == Language::JavaScript {
            if text == "(" {
                let qualified = tokens
                    .iter()
                    .rev()
                    .nth(1)
                    .is_some_and(|token| token.value == "." || token.value == "?.");
                let declaration = tokens
                    .iter()
                    .rev()
                    .nth(1)
                    .is_some_and(|token| token.value == "function")
                    && (tokens.len() == 2
                        || tokens.iter().rev().nth(2).is_some_and(|token| {
                            matches!(token.value.as_str(), ";" | "{" | "}" | "export" | "async")
                        }));
                let control = tokens.last().is_some_and(|token| {
                    let kind = if token.kind == TokenKind::Identifier {
                        "identifier"
                    } else {
                        "other"
                    };
                    accept_control_parenthesis(kind, &token.value, qualified)
                });
                scopes.push((control, declaration));
            }
            if text == ")" {
                let (control, declaration) = scopes.pop().unwrap_or((false, false));
                closed = control;
                declaration_body = declaration;
            } else if text == "{" {
                let kind =
                    tokens.last().map_or(
                        if braced { "expression" } else { "" },
                        |token| match token.kind {
                            TokenKind::Identifier => "identifier",
                            TokenKind::Punct => "punct",
                            _ => "other",
                        },
                    );
                let value = tokens.last().map_or("", |token| token.value.as_str());
                braces.push(accept_statement_brace(
                    kind,
                    value,
                    control_closed,
                    declaration_body,
                ));
                declaration_body = false;
            } else if text == "}" {
                closed = braces.pop().unwrap_or(false);
            } else {
                declaration_body = false;
            }
        }
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
        index,
        index,
    ));
    Ok(Tokens {
        tokens,
        comments: comment_list,
    })
}

include!("lexer/literals.rs");

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
