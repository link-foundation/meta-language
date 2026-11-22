use std::{collections::BTreeMap, sync::LazyLock};

use super::{document_projection_rule, document_text_policy::document_scalar_is_valid};

static HTML_REFERENCES: LazyLock<BTreeMap<String, String>> = LazyLock::new(|| {
    include_str!("../../data/document-character-references.lino")
        .lines()
        .map(|line| {
            let mut words = line[1..line.len() - 1].split_ascii_whitespace();
            assert_eq!(words.next(), Some("namedCharacterReference"));
            let name = words
                .next()
                .expect("character reference name")
                .replace("%3B", ";");
            let value = words
                .map(|word| {
                    let scalar = word.parse().expect("character reference scalar");
                    char::from_u32(scalar).expect("Unicode scalar value")
                })
                .collect();
            (name, value)
        })
        .collect()
});

#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum ReferenceSyntax {
    Html,
    Markdown,
    Xml,
}

pub(super) fn character_reference(
    source: &str,
    syntax: ReferenceSyntax,
) -> Option<(usize, String)> {
    let rest = source.strip_prefix('&')?;
    if let Some(number) = rest.strip_prefix('#') {
        return numeric_reference(number, syntax);
    }
    let mut end = rest
        .bytes()
        .take(32)
        .take_while(u8::is_ascii_alphanumeric)
        .count();
    if rest.as_bytes().get(end) == Some(&b';') {
        end += 1;
    }
    while end > 0 {
        let name = &rest[..end];
        let value = if syntax == ReferenceSyntax::Xml {
            match name {
                "amp;" => Some("&"),
                "lt;" => Some("<"),
                "gt;" => Some(">"),
                "quot;" => Some("\""),
                "apos;" => Some("'"),
                _ => None,
            }
        } else if syntax == ReferenceSyntax::Html || name.ends_with(';') {
            HTML_REFERENCES.get(name).map(String::as_str)
        } else {
            None
        };
        if let Some(value) = value {
            return Some((end + 1, value.to_owned()));
        }
        end -= 1;
    }
    None
}

fn numeric_reference(source: &str, syntax: ReferenceSyntax) -> Option<(usize, String)> {
    let hexadecimal = source.starts_with(['x', 'X']);
    let prefix = usize::from(hexadecimal);
    let radix = if hexadecimal { 16 } else { 10 };
    let mut end = prefix;
    let mut value: u32 = 0;
    for byte in source.bytes().skip(prefix) {
        let Some(digit) = char::from(byte).to_digit(radix) else {
            break;
        };
        value = (value * radix + digit).min(0x11_0000);
        end += 1;
    }
    let semicolon = source.as_bytes().get(end) == Some(&b';');
    if end == prefix || (!semicolon && syntax != ReferenceSyntax::Html) {
        return None;
    }
    let valid = document_scalar_is_valid(f64::from(value));
    if !valid && syntax == ReferenceSyntax::Xml {
        return None;
    }
    let scalar = if !valid {
        0xfffd
    } else if syntax != ReferenceSyntax::Xml && (0x80..0xa0).contains(&value) {
        document_projection_rule("numericReferenceReplacement")
            .nth((value - 0x80) as usize)
            .and_then(|word| word.parse().ok())
            .unwrap_or(value)
    } else {
        value
    };
    Some((
        2 + end + usize::from(semicolon),
        char::from_u32(scalar)?.to_string(),
    ))
}
