use std::collections::{BTreeSet, HashMap};
use std::sync::LazyLock;

use regex::Regex;

use super::document_container::binary_string;

const REPLACEMENT: &str = "\u{fffd}";
const MAXIMUM_CMAP_MAPPINGS: usize = 65_536;

static GLYPHS: LazyLock<HashMap<&str, String>> = LazyLock::new(|| {
    table(include_str!("../../data/pdf-glyph-map.lino"))
        .map(|(glyph, values)| (glyph, values.filter_map(scalar).collect()))
        .collect()
});

static ENCODINGS: LazyLock<HashMap<&str, Vec<String>>> = LazyLock::new(|| {
    table(include_str!("../../data/pdf-font-encodings.lino"))
        .map(|(encoding, values)| {
            (
                encoding,
                values
                    .map(|value| {
                        if value == "-" {
                            REPLACEMENT.into()
                        } else {
                            value.split('-').filter_map(scalar).collect()
                        }
                    })
                    .collect(),
            )
        })
        .collect()
});

fn table(source: &str) -> impl Iterator<Item = (&str, std::str::SplitAsciiWhitespace<'_>)> {
    source.lines().filter_map(|line| {
        let mut values = line
            .strip_prefix('(')?
            .strip_suffix(')')?
            .split_ascii_whitespace();
        Some((values.next()?, values))
    })
}

fn scalar(value: &str) -> Option<char> {
    char::from_u32(u32::from_str_radix(value, 16).ok()?)
}

pub(super) fn glyph_name(name: &str) -> String {
    let name = name.split('.').next().unwrap_or_default();
    let mut output = String::new();
    for component in name.split('_') {
        if let Some(value) = GLYPHS.get(component) {
            output.push_str(value);
        } else if let Some(hex) = component
            .strip_prefix("uni")
            .filter(|hex| is_upper_hex(hex) && hex.len().is_multiple_of(4))
        {
            output.push_str(&utf16(&hex_bytes(hex)));
        } else if let Some(character) = component
            .strip_prefix('u')
            .filter(|hex| (4..=6).contains(&hex.len()) && is_upper_hex(hex))
            .and_then(scalar)
        {
            output.push(character);
        } else {
            output.push_str(REPLACEMENT);
        }
    }
    output
}

fn is_upper_hex(text: &str) -> bool {
    !text.is_empty()
        && text
            .bytes()
            .all(|byte| matches!(byte, b'0'..=b'9' | b'A'..=b'F'))
}

pub(super) fn predefined_encoding(name: &str) -> &'static [String] {
    ENCODINGS
        .get(name)
        .unwrap_or(&ENCODINGS["StandardEncoding"])
}

pub(super) const fn pdf_whitespace(byte: u8) -> bool {
    matches!(byte, 0 | b'\t' | b'\n' | 12 | b'\r' | b' ')
}

fn hex_digit(byte: u8) -> Option<u8> {
    char::from(byte).to_digit(16).map(|digit| digit as u8)
}

pub(super) fn hex_bytes(text: &str) -> Vec<u8> {
    let mut digits = text.bytes().filter(|byte| !pdf_whitespace(*byte));
    let mut result = Vec::new();
    while let Some(high) = digits.next().and_then(hex_digit) {
        let low = match digits.next() {
            Some(low) => match hex_digit(low) {
                Some(low) => low,
                None => break,
            },
            None => 0,
        };
        result.push(high * 16 + low);
    }
    result
}

pub(super) fn string_bytes(spelling: &str) -> Vec<u8> {
    if let Some(hex) = spelling
        .strip_prefix('<')
        .and_then(|text| text.strip_suffix('>'))
    {
        return hex_bytes(hex);
    }
    let Some(literal) = spelling
        .strip_prefix('(')
        .and_then(|text| text.strip_suffix(')'))
    else {
        return Vec::new();
    };
    let mut characters = literal.chars().peekable();
    let mut result = Vec::new();
    while let Some(mut character) = characters.next() {
        if character == '\\' {
            let Some(escaped) = characters.next() else {
                break;
            };
            match escaped {
                'n' => character = '\n',
                'r' => character = '\r',
                't' => character = '\t',
                'b' => character = '\u{8}',
                'f' => character = '\u{c}',
                '\n' | '\r' => {
                    if escaped == '\r' && characters.peek() == Some(&'\n') {
                        characters.next();
                    }
                    continue;
                }
                '0'..='7' => {
                    let mut value = escaped as u8 - b'0';
                    for _ in 0..2 {
                        if !matches!(characters.peek(), Some('0'..='7')) {
                            break;
                        }
                        let digit = characters.next().expect("octal digit") as u8 - b'0';
                        value = value.wrapping_mul(8).wrapping_add(digit);
                    }
                    result.push(value);
                    continue;
                }
                _ => character = escaped,
            }
        } else if character == '\r' {
            if characters.peek() == Some(&'\n') {
                characters.next();
            }
            character = '\n';
        }
        if let Ok(byte) = u8::try_from(u32::from(character)) {
            result.push(byte);
        } else {
            result.extend_from_slice(character.encode_utf8(&mut [0; 4]).as_bytes());
        }
    }
    result
}

fn utf16(bytes: &[u8]) -> String {
    let words = bytes
        .chunks_exact(2)
        .map(|pair| u16::from_be_bytes([pair[0], pair[1]]));
    char::decode_utf16(words)
        .map(|value| value.unwrap_or(char::REPLACEMENT_CHARACTER))
        .collect()
}

#[derive(Default)]
pub(super) struct UnicodeMap {
    mapping: HashMap<(usize, u32), String>,
    spaces: HashMap<usize, Vec<(u32, u32)>>,
    widths: BTreeSet<usize>,
}

static CMAP_TOKENS: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
    r"(?i)%[^\r\n]*|<([0-9a-f\x00\t\n\x0c\r ]+)>|[\[\]]|(?:begin|end)(?:codespacerange|bfchar|bfrange)",
).expect("PDF CMap tokens")
});

enum CmapToken<'source> {
    Text(&'source str),
    Hex(String),
}

fn code(token: Option<&CmapToken<'_>>) -> Option<(usize, u32)> {
    let CmapToken::Hex(hex) = token? else {
        return None;
    };
    if hex.is_empty() || hex.len() > 8 || !hex.len().is_multiple_of(2) {
        return None;
    }
    Some((hex.len() / 2, u32::from_str_radix(hex, 16).ok()?))
}

fn unicode_bytes<'token>(token: Option<&'token CmapToken<'_>>) -> Option<&'token str> {
    let CmapToken::Hex(hex) = token? else {
        return None;
    };
    (!hex.is_empty() && hex.len().is_multiple_of(4)).then_some(hex)
}

pub(super) fn read_unicode_map<'map>(sources: impl IntoIterator<Item = &'map str>) -> UnicodeMap {
    let mut result = UnicodeMap::default();
    let mut remaining_range_work = MAXIMUM_CMAP_MAPPINGS;
    // The source order is from the derived map to its earliest base.
    for source in sources {
        let capacity = MAXIMUM_CMAP_MAPPINGS - result.mapping.len();
        if capacity == 0 {
            break;
        }
        let layer = parse_unicode_map(source, capacity, &mut remaining_range_work);
        if result.spaces.is_empty() {
            result.spaces = layer.spaces;
        }
        for (code, value) in layer.mapping {
            result.mapping.entry(code).or_insert(value);
        }
    }
    result.widths.extend(result.spaces.keys().copied());
    result
        .widths
        .extend(result.mapping.keys().map(|&(width, _)| width));
    result
}

fn parse_unicode_map(
    source: &str,
    capacity: usize,
    remaining_range_work: &mut usize,
) -> UnicodeMap {
    let mut tokens = CMAP_TOKENS.captures_iter(source).filter_map(|capture| {
        let text = capture.get(0)?.as_str();
        if text.starts_with('%') {
            return None;
        }
        Some(capture.get(1).map_or(CmapToken::Text(text), |value| {
            CmapToken::Hex(
                value
                    .as_str()
                    .bytes()
                    .filter(|byte| !pdf_whitespace(*byte))
                    .map(char::from)
                    .collect(),
            )
        }))
    });
    let mut result = UnicodeMap::default();
    let mut token = tokens.next();
    while let Some(current) = token.take() {
        if result.mapping.len() >= capacity {
            break;
        }
        token = tokens.next();
        let CmapToken::Text(kind) = current else {
            continue;
        };
        let kind = kind.to_ascii_lowercase();
        while let Some((width, first)) = code(token.as_ref()) {
            let datum = tokens.next();
            match kind.as_str() {
                "begincodespacerange" => {
                    let Some((_, last)) =
                        code(datum.as_ref()).filter(|(upper_width, _)| *upper_width == width)
                    else {
                        token = datum;
                        break;
                    };
                    let ranges = result.spaces.entry(width).or_default();
                    if ranges.len() < 256 {
                        ranges.push((first, last));
                    }
                }
                "beginbfchar" => {
                    let Some(hex) = unicode_bytes(datum.as_ref()) else {
                        token = datum;
                        break;
                    };
                    if hex.len() <= 256 {
                        result
                            .mapping
                            .insert((width, first), utf16(&hex_bytes(hex)));
                    }
                }
                "beginbfrange" => {
                    let Some((_, last)) =
                        code(datum.as_ref()).filter(|(upper_width, _)| *upper_width == width)
                    else {
                        token = datum;
                        break;
                    };
                    let available = (*remaining_range_work).min(capacity - result.mapping.len());
                    let count = if last >= first {
                        ((u64::from(last - first) + 1).min(available as u64)) as usize
                    } else {
                        0
                    };
                    *remaining_range_work -= count;
                    let target = tokens.next();
                    if matches!(target, Some(CmapToken::Text("["))) {
                        let mut index = 0;
                        while let Some(element) = tokens.next() {
                            let Some(hex) = unicode_bytes(Some(&element)) else {
                                break;
                            };
                            if index < count && hex.len() <= 256 {
                                result
                                    .mapping
                                    .insert((width, first + index as u32), utf16(&hex_bytes(hex)));
                            }
                            index += 1;
                        }
                    } else if let Some(hex) =
                        unicode_bytes(target.as_ref()).filter(|hex| hex.len() <= 256)
                    {
                        let mut bytes = hex_bytes(hex);
                        for index in 0..count {
                            result
                                .mapping
                                .insert((width, first + index as u32), utf16(&bytes));
                            if !increment(&mut bytes) {
                                break;
                            }
                        }
                    }
                }
                _ => {
                    token = datum;
                    break;
                }
            }
            token = tokens.next();
            if result.mapping.len() >= capacity {
                break;
            }
        }
    }
    result
}

fn increment(bytes: &mut [u8]) -> bool {
    for byte in bytes.iter_mut().rev() {
        *byte = byte.wrapping_add(1);
        if *byte != 0 {
            return true;
        }
    }
    false
}

pub(super) struct PdfFont {
    pub(super) encoding: &'static [String],
    pub(super) differences: HashMap<u32, String>,
    pub(super) composite: bool,
    pub(super) cmap: Option<UnicodeMap>,
}

fn code_in_space(bytes: &[u8], first: u32, last: u32) -> bool {
    let first = first.to_be_bytes();
    let last = last.to_be_bytes();
    let offset = first.len() - bytes.len();
    bytes
        .iter()
        .enumerate()
        .all(|(index, &byte)| first[offset + index] <= byte && byte <= last[offset + index])
}

pub(super) fn decode_font_text(mut bytes: &[u8], font: Option<&PdfFont>) -> String {
    let Some(font) = font else {
        return binary_string(bytes);
    };
    let mut result = String::new();
    while !bytes.is_empty() {
        let code = |width| {
            bytes[..width]
                .iter()
                .fold(0u32, |value, byte| value * 256 + u32::from(*byte))
        };
        let code_width = font.cmap.as_ref().and_then(|cmap| {
            let widths = cmap
                .widths
                .iter()
                .copied()
                .filter(|&width| width <= bytes.len());
            // Check shorter code spaces first. Recover known codes if none match.
            widths
                .clone()
                .find(|width| {
                    cmap.spaces.get(width).is_some_and(|ranges| {
                        ranges
                            .iter()
                            .any(|&(first, last)| code_in_space(&bytes[..*width], first, last))
                    })
                })
                .or_else(|| {
                    widths
                        .rev()
                        .find(|&width| cmap.mapping.contains_key(&(width, code(width))))
                })
        });
        let width = code_width
            .unwrap_or(if font.composite { 2 } else { 1 })
            .min(bytes.len());
        let value = code(width);
        if let Some(text) = font
            .cmap
            .as_ref()
            .and_then(|cmap| cmap.mapping.get(&(width, value)))
        {
            result.push_str(text);
            bytes = &bytes[width..];
            continue;
        }
        result.push_str(if font.composite {
            REPLACEMENT
        } else {
            font.differences
                .get(&value)
                .or_else(|| font.encoding.get(value as usize))
                .map_or(REPLACEMENT, String::as_str)
        });
        bytes = &bytes[width..];
    }
    result
}
