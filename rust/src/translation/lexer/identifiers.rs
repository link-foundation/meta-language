//! Unicode source/data adapters; decisions are automatically translated JavaScript.
use std::{cell::RefCell, collections::BTreeMap, panic::AssertUnwindSafe, rc::Rc};

use serde::Deserialize;

use super::{Language, Source, Span, Token, TokenKind};
use crate::translation::{
    diagnostics::{Result, TranslationError},
    unicode_scalar_rules,
};

/// Pinned Unicode version for portable source identifiers and canonical NFC.
pub const SOURCE_IDENTIFIER_UNICODE_VERSION: &str = "17.0.0";
type Numbers = Rc<RefCell<Vec<f64>>>;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct SourceTables {
    schema_version: u32,
    unicode: String,
    properties: BTreeMap<String, Vec<f64>>,
    classes: Vec<f64>,
    decompositions: Vec<f64>,
    values: Vec<f64>,
    pairs: Vec<f64>,
}
struct SharedTables {
    properties: BTreeMap<String, Numbers>,
    classes: Numbers,
    decompositions: Numbers,
    values: Numbers,
    pairs: Numbers,
}
fn shared(values: Vec<f64>) -> Numbers {
    Rc::new(RefCell::new(values))
}
fn read_tables() -> SharedTables {
    let value =
        crate::read_structured_data_lino(include_str!("../../data/unicode-source-tables.lino"))
            .expect("the shipped Unicode declarations are Links data");
    let data: SourceTables =
        serde_json::from_value(value).expect("the shipped Unicode table schema");
    assert_eq!(data.schema_version, 1, "uncovered Unicode table schema");
    assert_eq!(
        data.unicode, SOURCE_IDENTIFIER_UNICODE_VERSION,
        "uncovered source identifier Unicode data version"
    );
    SharedTables {
        properties: data
            .properties
            .into_iter()
            .map(|(name, values)| (name, shared(values)))
            .collect(),
        classes: shared(data.classes),
        decompositions: shared(data.decompositions),
        values: shared(data.values),
        pairs: shared(data.pairs),
    }
}
thread_local! {
    // Generated reference arrays are thread-local by construction. Neither
    // initialization nor declaration import invokes any programming lexer.
    static TABLES: SharedTables = read_tables();
    static EMPTY: Numbers = shared(Vec::new());
}
pub(super) fn scalar(source: &Source, index: usize) -> f64 {
    unicode_scalar_rules::decode_unicode_source_scalar(
        source.at(index).map_or(-1.0, f64::from),
        source.at(index + 1).map_or(-1.0, f64::from),
    )
}
pub(super) fn width(source: &Source, index: usize) -> usize {
    if unicode_scalar_rules::read_unicode_source_scalar_width(scalar(source, index)).to_bits()
        == 2f64.to_bits()
    {
        2
    } else {
        1
    }
}
pub(super) fn accepts(source: &Source, index: usize, language: Language, first: bool) -> bool {
    let point = scalar(source, index);
    accepts_point(point, language, first)
}
fn accepts_point(point: f64, language: Language, first: bool) -> bool {
    let name = if language == Language::Rust {
        "Rust"
    } else {
        "JavaScript"
    };
    if point < 128.0 {
        return EMPTY.with(|empty| {
            unicode_scalar_rules::accept_source_identifier_scalar(
                name,
                first,
                point,
                empty.clone(),
                empty.clone(),
            )
        });
    }
    TABLES.with(|data| {
        let prefix = if language == Language::Rust {
            "XID_"
        } else {
            "ID_"
        };
        unicode_scalar_rules::accept_source_identifier_scalar(
            name,
            first,
            point,
            data.properties[&(prefix.to_owned() + "Start")].clone(),
            data.properties[&(prefix.to_owned() + "Continue")].clone(),
        )
    })
}
pub(super) fn lifetime_prefix(source: &Source, index: usize) -> bool {
    if !source.is_char(index, '\'') {
        return false;
    }
    let first = accepts(source, index + 1, Language::Rust, true);
    let mut end = index + 1;
    if first {
        end += width(source, end);
        while end < source.len() && accepts(source, end, Language::Rust, false) {
            end += width(source, end);
        }
    }
    unicode_scalar_rules::accept_rust_source_lifetime_token(first, source.is_char(end, '\''))
}
pub(super) fn javascript_token(
    source: &Source,
    start: usize,
    units: &mut Option<Vec<f64>>,
) -> Result<Token> {
    let mut end = start;
    let mut segment = start;
    let mut value = String::new();
    while end < source.len() {
        if source.is_char(end, '\\') {
            let encoded = units.get_or_insert_with(|| {
                source.units().iter().map(|&unit| f64::from(unit)).collect()
            });
            let offset = serde_json::Number::from(end)
                .as_f64()
                .expect("a bounded UTF-16 source offset");
            let decoded = crate::translation::javascript_identifier_rules::decode_java_script_identifier_escape(encoded, offset);
            if decoded[0] < 0.0 || !accepts_point(decoded[1], Language::JavaScript, end == start) {
                let limit = if decoded[0] < 0.0 {
                    decoded[1]
                } else {
                    decoded[0]
                };
                let limit = crate::translation::frontend_rules::ml_array::number_index(limit)
                    .expect("a decoder source offset");
                return Err(TranslationError::syntax(
                    "invalid Unicode escape in a JavaScript identifier",
                    Some(Span {
                        start: end,
                        end: limit,
                    }),
                ));
            }
            // The shared property decision above admits only actual scalars.
            #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
            let point = decoded[1] as u32;
            value.push_str(&source.slice(segment, end));
            value.push(char::from_u32(point).expect("an admitted identifier scalar"));
            end = crate::translation::frontend_rules::ml_array::number_index(decoded[0])
                .expect("a decoder source offset");
            segment = end;
        } else {
            if !accepts(source, end, Language::JavaScript, end == start) {
                break;
            }
            end += width(source, end);
        }
    }
    value.push_str(&source.slice(segment, end));
    Ok(Token::new(
        TokenKind::Identifier,
        value,
        source.slice(start, end),
        start,
        end,
    ))
}
pub(crate) fn require_java_script_identifier_role(token: &Token, binding: bool) -> Result<()> {
    let message =
        crate::translation::javascript_identifier_rules::read_java_script_escaped_identifier_error(
            &token.value,
            &token.raw,
            binding,
        );
    if message.is_empty() {
        Ok(())
    } else {
        Err(TranslationError::syntax(message, Some(token.span())))
    }
}
pub(super) fn javascript_goal_value(token: Option<&Token>) -> String {
    token.map_or_else(String::new, |token| {
        crate::translation::javascript_identifier_rules::read_java_script_source_token_goal_value(
            if token.kind == TokenKind::Identifier {
                "identifier"
            } else {
                ""
            },
            &token.value,
            &token.raw,
        )
    })
}
pub(crate) fn accept_rust_source_identifier_name(value: &str) -> bool {
    let source = Source::new(value);
    if source.is_empty() {
        return false;
    }
    let mut index = 0;
    while index < source.len() {
        if !accepts(&source, index, Language::Rust, index == 0) {
            return false;
        }
        index += width(&source, index);
    }
    true
}
pub(super) fn normalize_rust(source: String, span: Span) -> Result<String> {
    if source.is_ascii() {
        return Ok(source);
    }
    let input = shared(
        source
            .chars()
            .map(|character| f64::from(u32::from(character)))
            .collect(),
    );
    let result = TABLES.with(|data| {
        std::panic::catch_unwind(AssertUnwindSafe(|| {
            unicode_scalar_rules::normalize_unicode_scalars(
                input,
                data.classes.clone(),
                data.decompositions.clone(),
                data.values.clone(),
                data.pairs.clone(),
                16_777_216.0,
            )
        }))
    });
    let output = result.map_err(|error| {
        let message = error
            .downcast_ref::<String>()
            .map(String::as_str)
            .or_else(|| error.downcast_ref::<&str>().copied())
            .unwrap_or("Unicode normalization failed");
        TranslationError::syntax(message, Some(span))
    })?;
    output
        .borrow()
        .iter()
        .map(|&point| {
            #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
            char::from_u32(point as u32).ok_or_else(|| {
                TranslationError::syntax("Unicode normalization requires scalar values", Some(span))
            })
        })
        .collect()
}
/// Canonical Unicode 17 NFC with the bounded, shared scalar definition.
///
/// # Errors
/// Returns a syntax error if the shared normalization resource bound is exceeded.
pub fn normalize_unicode_text(source: &str) -> Result<String> {
    normalize_rust(
        source.to_owned(),
        Span {
            start: 0,
            end: source.encode_utf16().count(),
        },
    )
}
