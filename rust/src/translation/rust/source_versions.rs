//! Actual text into shared version-column grammar and source range rules.
use super::{Result, unsupported};
use crate::translation::rust_source_version_rules as rules;
use crate::translation::source_version_column_rules as columns;
pub(crate) fn canonical(version: &str) -> Result<String> {
    let units = u32::try_from(version.len()).map_or(f64::INFINITY, f64::from);
    if units > rules::read_rust_source_version_text_limit() {
        return Err(unsupported(
            "Rust source version",
            &rules::read_rust_source_version_error(-1.0, -1.0, -1.0, false),
            None,
        ));
    }
    let spelling = if version.is_empty() {
        rules::read_rust_source_version_default()
    } else {
        version.to_owned()
    };
    let values =
        columns::read_source_version_columns(&columns::ml_string::Text::from_text(&spelling));
    let fact = |index: usize| values.get(index).copied().unwrap_or(-1.0);
    let error = rules::read_rust_source_version_error(fact(0), fact(1), fact(2), values.len() == 3);
    if !error.is_empty() {
        return Err(unsupported("Rust source version", &error, None));
    }
    Ok(spelling)
}
pub(super) fn minor(version: &str) -> Result<f64> {
    let spelling = canonical(version)?;
    Ok(columns::read_source_version_columns(&columns::ml_string::Text::from_text(&spelling))[1])
}
