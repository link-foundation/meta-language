//! Actual source-name census and UTF-16/ordering adapters for shared decisions.
use crate::translation::source_identifier_rules::{self as rules, ml_string::Text};
use std::{borrow::Cow, collections::HashSet};

pub(crate) fn read_source_identifier_supply<T: AsRef<str>>(
    names: impl IntoIterator<Item = T>,
) -> String {
    let mut private_names = false;
    let mut ordinals = Vec::new();
    for name in names {
        let name = Text::from_text(name.as_ref());
        private_names |= rules::accept_source_private_identifier(&name);
        let ordinal = rules::read_source_identifier_prefix_ordinal(&name);
        if ordinal >= 1.0 {
            ordinals.push(ordinal);
        }
    }
    ordinals.sort_by(f64::total_cmp);
    rules::read_source_identifier_prefix(private_names, &ordinals).to_string()
}
pub(crate) fn read_source_parser_identifier_supply(
    source: &str,
    tokens: &[crate::translation::lexer::Token],
) -> String {
    let identifiers = tokens
        .iter()
        .filter(|token| token.kind == crate::translation::lexer::TokenKind::Identifier)
        .map(|token| Cow::Borrowed(token.value.as_str()));
    let snippets = source
        .match_indices("ml_")
        .map(|(offset, _)| Cow::Owned(source[offset..].chars().take(32).collect::<String>()));
    read_source_identifier_supply(identifiers.chain(snippets))
}
pub(crate) fn render_source_target_identifier(name: &str) -> String {
    // Native source names are valid UTF-8; the shared result adds ASCII only.
    rules::render_source_target_identifier(&Text::from_text(name)).to_string()
}
pub(crate) fn render_source_generated_identifier(
    prefix: &str,
    role: &str,
    ordinal: usize,
) -> String {
    let ordinal = serde_json::Number::from(ordinal)
        .as_f64()
        .expect("a bounded generation ordinal is a Number");
    rules::render_source_generated_identifier(
        &Text::from_text(prefix),
        &Text::from_text(role),
        ordinal,
    )
    .to_string()
}
pub(crate) fn render_source_cell_parameter(prefix: &str, name: &str, reference: bool) -> String {
    rules::render_source_cell_parameter(&Text::from_text(prefix), &Text::from_text(name), reference)
        .to_string()
}
pub(crate) fn collect_source_identifier_texts(value: &serde_json::Value) -> HashSet<&str> {
    let mut result = HashSet::new();
    let mut pending = vec![value];
    while let Some(value) = pending.pop() {
        match value {
            serde_json::Value::String(text) => {
                result.insert(text.as_str());
            }
            serde_json::Value::Array(values) => pending.extend(values),
            serde_json::Value::Object(fields) => {
                result.extend(fields.keys().map(String::as_str));
                pending.extend(fields.values());
            }
            _ => {}
        }
    }
    result
}
