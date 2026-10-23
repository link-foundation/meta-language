//! Lossless publication markers and source provenance for checked items.
use super::*;

pub(super) fn record_translated_definitions(
    context: (&str, &str, &str, &str, &DecoratorSet),
    emitted: crate::translation::emit_common::Emitted,
    signatures: Vec<SelfTranslationSignature>,
    exported_override: Option<bool>,
) -> Translated {
    let (text, term, from, to, decorators) = context;
    if emitted.definitions.is_empty() {
        return carry(text, term, from, "no definition");
    }
    let exported = exported_override.unwrap_or_else(|| {
        family(to) == "JavaScript"
            && if family(from) == "Rust" {
                RUST_EXPORT.is_match(text)
            } else {
                JAVASCRIPT_EXPORT.is_match(text)
            }
    });
    let generic = emitted
        .definitions
        .iter()
        .map(|definition| {
            if exported {
                format!("export {definition}")
            } else {
                definition.clone()
            }
        })
        .collect::<Vec<_>>()
        .join("\n\n");
    let code = decorate_emitted(to, &generic, decorators);
    if code.trim().is_empty() {
        return carry(text, term, from, "dropped by a decorator");
    }
    let count = emitted.definitions.len();
    let marker = format!(
        "{TRANSLATED}{from} {term} items={} sha256={}",
        count,
        sha256(&code)
    );
    let mut lines = vec![marker];
    lines.extend(LINE_BREAK.split(text).map(source_line));
    lines.push(code);
    Translated {
        code: Some(lines.join("\n")),
        preludes: emitted.preludes,
        status: "translated",
        reason: None,
        signatures,
    }
}

pub(super) fn carry(text: &str, term: &str, from: &str, reason: &str) -> Translated {
    let mut lines = vec![format!("{CARRIED}{from} {term} ({reason})")];
    lines.extend(LINE_BREAK.split(text).map(source_line));
    Translated {
        code: Some(lines.join("\n")),
        preludes: Vec::new(),
        status: "carried",
        reason: Some(reason.to_owned()),
        signatures: Vec::new(),
    }
}

pub(super) fn source_line(line: &str) -> String {
    if line.is_empty() {
        SOURCE_LINE.to_owned()
    } else {
        format!("{SOURCE_LINE} {line}")
    }
}

// A comment keeps its text when the target reads it as an ordinary comment:
// a Rust doc comment needs an item after it, and Rust block comments nest.
pub(super) fn comment_fits(text: &str, to: &str) -> bool {
    if text.starts_with("/*") {
        let inner = text
            .get(2..text.len().saturating_sub(2))
            .unwrap_or_default();
        if inner.contains("/*") || inner.contains("*/") {
            return false;
        }
    }
    family(to) != "Rust"
        || !["///", "//!", "/**", "/*!"]
            .iter()
            .any(|prefix| text.starts_with(prefix))
}
