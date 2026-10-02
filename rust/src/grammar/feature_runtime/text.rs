//! The text layer of the executor, as `js/src/grammar-runtime/text.js`: the
//! input is a byte string, every offset is a byte offset, and a code point is
//! decoded on demand.
//!
//! A byte that does not start a well-formed UTF-8 sequence is one unit of its
//! own with no code point, so byte classes and `any` still see it.

use std::fmt::Write as _;

/// The code point at `position` and its byte length; a malformed or truncated
/// sequence yields `None` and length 1.
pub(super) fn decode_at(bytes: &[u8], position: usize, end: usize) -> (Option<u32>, usize) {
    let first = bytes[position];
    if first < 0x80 {
        return (Some(u32::from(first)), 1);
    }
    let (length, mut code_point, minimum) = match first {
        0xc2..=0xdf => (2, u32::from(first & 0x1f), 0x80),
        0xe0..=0xef => (3, u32::from(first & 0x0f), 0x800),
        0xf0..=0xf4 => (4, u32::from(first & 0x07), 0x10000),
        _ => return (None, 1),
    };
    if position + length > end {
        return (None, 1);
    }
    for index in 1..length {
        let next = bytes[position + index];
        if next & 0xc0 != 0x80 {
            return (None, 1);
        }
        code_point = (code_point << 6) | u32::from(next & 0x3f);
    }
    if code_point < minimum || code_point > 0x0010_ffff || (0xd800..=0xdfff).contains(&code_point) {
        return (None, 1);
    }
    (Some(code_point), length)
}

/// The byte length of the unit at `position`.
pub(super) fn unit_length(bytes: &[u8], position: usize, end: usize) -> usize {
    decode_at(bytes, position, end).1
}

/// The text of a byte range, or `None` when it is not well-formed UTF-8.
pub(super) fn text_of(bytes: &[u8], start: usize, end: usize) -> Option<String> {
    std::str::from_utf8(&bytes[start..end])
        .ok()
        .map(str::to_owned)
}

/// The lower-case hexadecimal spelling of a byte range.
pub(super) fn hex_of(bytes: &[u8], start: usize, end: usize) -> String {
    let mut hex = String::with_capacity((end - start) * 2);
    for byte in &bytes[start..end] {
        let _ = write!(hex, "{byte:02x}");
    }
    hex
}

/// The number of code points (or stray bytes) in a byte range.
pub(super) fn unit_count(bytes: &[u8], start: usize, end: usize) -> usize {
    let mut count = 0;
    let mut position = start;
    while position < end {
        position += unit_length(bytes, position, end);
        count += 1;
    }
    count
}

/// The 1-based line and column of `offset`, counting columns in code points.
pub(super) fn line_and_column(bytes: &[u8], offset: usize) -> (usize, usize) {
    let mut line = 1;
    let mut line_start = 0;
    for (position, byte) in bytes.iter().enumerate().take(offset) {
        if *byte == b'\n' {
            line += 1;
            line_start = position + 1;
        }
    }
    (line, unit_count(bytes, line_start, offset) + 1)
}

/// The 0-based column of `offset`: the code points since the last line feed (or `begin`).
pub(super) fn column_of(bytes: &[u8], offset: usize, begin: usize) -> usize {
    let mut line_start = offset;
    while line_start > begin && bytes[line_start - 1] != b'\n' {
        line_start -= 1;
    }
    unit_count(bytes, line_start, offset)
}

/// Folds a text for `literalInsensitive`: the lower case of every code point.
pub(super) fn fold_case(text: &str) -> String {
    text.chars().flat_map(char::to_lowercase).collect()
}

/// The matcher of a Unicode general category (`Lu`, `L`, `Nd`, ...) or
/// script (`Greek`, `Latin`, ...), or `None` when the name is not one.
pub(super) fn unicode_property_matcher(script: bool, value: &str) -> Option<regex::Regex> {
    if value.is_empty() || !value.chars().all(|c| c.is_ascii_alphabetic() || c == '_') {
        return None;
    }
    let property = if script { "Script" } else { "General_Category" };
    regex::Regex::new(&format!("^\\p{{{property}={value}}}$")).ok()
}

/// A JSON string literal with the fixed escaping of `quoteText`: `"` and
/// `\`, the short escapes, every other control character as a lower-case
/// `\u00xx`, and everything else literal.
pub(super) fn quote_text(text: &str) -> String {
    let mut quoted = String::with_capacity(text.len() + 2);
    quoted.push('"');
    for c in text.chars() {
        match c {
            '"' => quoted.push_str("\\\""),
            '\\' => quoted.push_str("\\\\"),
            '\u{8}' => quoted.push_str("\\b"),
            '\u{c}' => quoted.push_str("\\f"),
            '\n' => quoted.push_str("\\n"),
            '\r' => quoted.push_str("\\r"),
            '\t' => quoted.push_str("\\t"),
            c if (c as u32) < 0x20 || c as u32 == 0x7f => {
                let _ = write!(quoted, "\\u{:04x}", c as u32);
            }
            c => quoted.push(c),
        }
    }
    quoted.push('"');
    quoted
}
