//! Unicode source/data adapters; decisions are automatically translated JavaScript.
use std::{cell::RefCell, collections::BTreeMap, panic::AssertUnwindSafe, rc::Rc};

use serde::Deserialize;

use super::{Language, Source, Span};
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
    if unicode_scalar_rules::read_unicode_source_scalar_width(scalar(source, index)) == 2.0 {
        2
    } else {
        1
    }
}
pub(super) fn accepts(source: &Source, index: usize, language: Language, first: bool) -> bool {
    let point = scalar(source, index);
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
