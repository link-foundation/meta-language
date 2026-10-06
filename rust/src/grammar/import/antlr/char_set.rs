//! ANTLR character sets: `[...]` classes with `\uXXXX`, `\u{X...}` and
//! `\p{NAME}` items, `caseInsensitive` case variants and `~` complements.

use super::{FORMAT, error_at};
use crate::grammar::feature::{FeatureExpr, class_expression};
use crate::grammar::import::{GrammarImportError, unsupported_error};
use crate::grammar::{CharClassItem, GrammarExpr, UnicodeClassItem};

pub(super) fn lower_char_set(
    content: &str,
    offset: usize,
    case_insensitive: bool,
) -> Result<GrammarExpr, GrammarImportError> {
    let mut scanner = ClassScanner::new(content, offset);
    let negated = scanner.try_consume('^');
    let mut items = Vec::new();
    let mut read = false;
    while !scanner.is_end() {
        read = true;
        if let Some(property) = scanner.try_read_property()? {
            items.push(property);
            continue;
        }
        let start = scanner.read_char()?;
        let end = if scanner.try_consume_range_separator() {
            scanner.read_char()?
        } else {
            start
        };
        if start > end {
            return Err(error_at(offset, "character class range start exceeds end"));
        }
        push_scalar_range(&mut items, start, end);
    }
    if !read {
        return Err(error_at(offset, "character class must not be empty"));
    }
    if case_insensitive {
        items = with_case_variants(&items);
    }
    Ok(class_expression(negated, items))
}

pub(super) fn has_case(text: &str) -> bool {
    text.to_lowercase() != text.to_uppercase()
}

// Under `caseInsensitive`, ANTLR matches each character in either case. The
// items gain the other case of each character, and of the ASCII letters in
// each range.
pub(super) fn with_case_variants(items: &[UnicodeClassItem]) -> Vec<UnicodeClassItem> {
    let mut result = items.to_vec();
    let mut add = |item: UnicodeClassItem| {
        if !result.contains(&item) {
            result.push(item);
        }
    };
    for item in items {
        match *item {
            UnicodeClassItem::Char(value) => {
                for variant in [
                    value.to_lowercase().to_string(),
                    value.to_uppercase().to_string(),
                ] {
                    let mut chars = variant.chars();
                    if let (Some(variant), None) = (chars.next(), chars.next())
                        && variant != value
                    {
                        add(UnicodeClassItem::Char(variant));
                    }
                }
            }
            UnicodeClassItem::Range(from, to) => {
                for (low, high, upper) in [('a', 'z', false), ('A', 'Z', true)] {
                    let (start, end) = (from.max(low), to.min(high));
                    if start > end {
                        continue;
                    }
                    let swap = |character: char| {
                        if upper {
                            character.to_ascii_lowercase()
                        } else {
                            character.to_ascii_uppercase()
                        }
                    };
                    add(if start == end {
                        UnicodeClassItem::Char(swap(start))
                    } else {
                        UnicodeClassItem::Range(swap(start), swap(end))
                    });
                }
            }
            UnicodeClassItem::Category(_) | UnicodeClassItem::Script(_) => {}
        }
    }
    result
}

// ANTLR reads `\uD800`-`\uDFFF` in a set as UTF-16 surrogates, which Java
// grammars name to match the halves of a pair. Text here is code points, so
// no surrogate is ever matched and a set keeps the scalars it names.
pub(super) fn push_scalar_range(items: &mut Vec<UnicodeClassItem>, start: u32, end: u32) {
    let mut push = |from: u32, to: u32| {
        if let (Some(from), Some(to)) = (char::from_u32(from), char::from_u32(to)) {
            items.push(if from == to {
                UnicodeClassItem::Char(from)
            } else {
                UnicodeClassItem::Range(from, to)
            });
        }
    };
    if start < 0xD800 {
        push(start, end.min(0xD7FF));
    }
    if end > 0xDFFF {
        push(start.max(0xE000), end);
    }
}

pub(super) const UNICODE_SCRIPTS: [&str; 14] = [
    "Arabic",
    "Armenian",
    "Bengali",
    "Cyrillic",
    "Devanagari",
    "Georgian",
    "Greek",
    "Han",
    "Hangul",
    "Hebrew",
    "Hiragana",
    "Katakana",
    "Latin",
    "Thai",
];

/// The character class item `\p{NAME}` names: a general category such as `L`
/// or `Nd`, or a script such as `Greek` or `Script=Greek`.
pub(super) fn unicode_property_item(name: &str) -> Result<UnicodeClassItem, GrammarImportError> {
    let property = name
        .strip_prefix("General_Category=")
        .or_else(|| name.strip_prefix("gc="))
        .unwrap_or(name);
    let mut chars = property.chars();
    let is_category = chars.next().is_some_and(|first| "LMNPSZC".contains(first))
        && chars
            .next()
            .is_none_or(|second| second.is_ascii_lowercase())
        && chars.next().is_none();
    if is_category {
        return Ok(UnicodeClassItem::Category(property.to_owned()));
    }
    let script = property
        .strip_prefix("Script=")
        .or_else(|| property.strip_prefix("sc="))
        .unwrap_or(property);
    if UNICODE_SCRIPTS.contains(&script) {
        return Ok(UnicodeClassItem::Script(script.to_owned()));
    }
    Err(unsupported_error(
        FORMAT,
        format!("Unicode property {name}"),
    ))
}

#[derive(Clone, Debug)]
pub(super) struct ClassScanner<'text> {
    text: &'text str,
    cursor: usize,
    offset: usize,
}

impl<'text> ClassScanner<'text> {
    const fn new(text: &'text str, offset: usize) -> Self {
        Self {
            text,
            cursor: 0,
            offset,
        }
    }

    /// The code point of the next item, which may be a surrogate.
    fn read_char(&mut self) -> Result<u32, GrammarImportError> {
        let Some(character) = self.advance_char() else {
            return Err(error_at(self.offset, "unexpected end of character class"));
        };
        if character == '\\' {
            self.read_escape()
        } else {
            Ok(u32::from(character))
        }
    }

    fn read_escape(&mut self) -> Result<u32, GrammarImportError> {
        let Some(character) = self.advance_char() else {
            return Err(error_at(self.offset, "unterminated character class escape"));
        };
        let escaped = match character {
            'n' => '\n',
            'r' => '\r',
            't' => '\t',
            'b' => '\u{08}',
            'f' => '\u{0c}',
            'u' => return self.read_code_point(),
            character => character,
        };
        Ok(u32::from(escaped))
    }

    /// The code point of `\uXXXX` or `\u{X...}`, after the `u`.
    fn read_code_point(&mut self) -> Result<u32, GrammarImportError> {
        let mut digits = String::new();
        if self.try_consume('{') {
            while let Some(character) = self.peek_char().filter(|&character| character != '}') {
                digits.push(character);
                self.advance_char();
            }
            if !self.try_consume('}') {
                return Err(error_at(self.offset, "unterminated unicode escape"));
            }
        } else {
            for _ in 0..4 {
                let Some(character) = self.advance_char() else {
                    break;
                };
                digits.push(character);
            }
        }
        let valid = (1..=6).contains(&digits.len())
            && digits.chars().all(|digit| digit.is_ascii_hexdigit());
        valid
            .then(|| u32::from_str_radix(&digits, 16).ok())
            .flatten()
            .filter(|&value| value <= 0x10_FFFF)
            .ok_or_else(|| error_at(self.offset, "invalid unicode escape"))
    }

    /// A `\p{NAME}` item at the cursor, if one is there.
    fn try_read_property(&mut self) -> Result<Option<UnicodeClassItem>, GrammarImportError> {
        let rest = &self.text[self.cursor..];
        if rest.starts_with("\\P") {
            return Err(unsupported_error(
                FORMAT,
                "negated Unicode property in character set",
            ));
        }
        let Some(after) = rest.strip_prefix("\\p") else {
            return Ok(None);
        };
        let Some(body) = after.strip_prefix('{') else {
            return Err(error_at(self.offset, "Unicode property needs a {NAME}"));
        };
        let Some(close) = body.find('}') else {
            return Err(error_at(self.offset, "unterminated Unicode property"));
        };
        let name = body[..close].to_owned();
        self.cursor += "\\p{".len() + close + 1;
        unicode_property_item(&name).map(Some)
    }

    fn try_consume(&mut self, expected: char) -> bool {
        if self.peek_char() == Some(expected) {
            self.advance_char();
            true
        } else {
            false
        }
    }

    fn try_consume_range_separator(&mut self) -> bool {
        if self.peek_char() == Some('-') && self.has_char_after_current() {
            self.advance_char();
            true
        } else {
            false
        }
    }

    fn has_char_after_current(&self) -> bool {
        let mut chars = self.text[self.cursor..].chars();
        chars.next();
        chars.next().is_some()
    }

    const fn is_end(&self) -> bool {
        self.cursor >= self.text.len()
    }

    fn peek_char(&self) -> Option<char> {
        self.text[self.cursor..].chars().next()
    }

    fn advance_char(&mut self) -> Option<char> {
        let character = self.peek_char()?;
        self.cursor += character.len_utf8();
        Some(character)
    }
}

// `~` complements a set (a character class, a one-character literal, a range,
// or an alternation of those) and matches one character outside it. Any
// other operand stays a negative lookahead.
pub(super) fn negate_expr(expr: GrammarExpr) -> GrammarExpr {
    set_items(&expr).map_or_else(
        || GrammarExpr::not(expr),
        |items| class_expression(true, items),
    )
}

pub(super) fn set_items(expr: &GrammarExpr) -> Option<Vec<UnicodeClassItem>> {
    match expr {
        GrammarExpr::CharClass {
            negated: false,
            items,
        } => Some(
            items
                .iter()
                .map(|item| match item {
                    CharClassItem::Char(value) => UnicodeClassItem::Char(*value),
                    CharClassItem::Range(start, end) => UnicodeClassItem::Range(*start, *end),
                })
                .collect(),
        ),
        GrammarExpr::Terminal(value) => {
            let mut characters = value.chars();
            match (characters.next(), characters.next()) {
                (Some(character), None) => Some(vec![UnicodeClassItem::Char(character)]),
                _ => None,
            }
        }
        GrammarExpr::CharRange(start, end) => Some(vec![UnicodeClassItem::Range(*start, *end)]),
        GrammarExpr::Choice { alternatives, .. } => {
            let parts = alternatives
                .iter()
                .map(set_items)
                .collect::<Option<Vec<_>>>()?;
            Some(parts.into_iter().flatten().collect())
        }
        GrammarExpr::Feature(feature) => match feature.as_ref() {
            FeatureExpr::UnicodeClass {
                negated: false,
                items,
            } => Some(items.clone()),
            _ => None,
        },
        _ => None,
    }
}
