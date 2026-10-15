//! Primitive string projection into the shared bounded Unicode scalar model.
//! The generated numeric module owns every UTF-8 decision. This adapter supplies
//! actual UTF-16 units and converts validated offsets into host indices.

use super::diagnostics::{Result, type_error};
use super::frontend_rules::ml_array::number_index;
use super::source_text_byte_rules::{
    accept_source_text_byte_slice, accept_source_text_scalar_units, accept_source_text_unit_count,
    read_source_text_byte as numeric_byte, read_source_text_byte_length as numeric_length,
};

/// Actual UTF-16 units of scalar-valid text under the shared finite vector bound.
pub fn read_source_text_units(text: &str) -> Result<Vec<f64>> {
    let count = serde_json::Number::from(text.encode_utf16().count())
        .as_f64()
        .ok_or_else(|| {
            type_error(
                "source text exceeds the bounded Unicode code unit model",
                None,
            )
        })?;
    if !accept_source_text_unit_count(count) {
        return Err(type_error(
            "source text exceeds the bounded Unicode code unit model",
            None,
        ));
    }
    let units: Vec<f64> = text.encode_utf16().map(f64::from).collect();
    if !accept_source_text_scalar_units(&units) {
        return Err(type_error(
            "source text exceeds the bounded Unicode code unit model",
            None,
        ));
    }
    Ok(units)
}

/// Exact UTF-8 length, without replacement or normalization.
pub fn read_source_text_byte_length(text: &str) -> Result<usize> {
    let units = read_source_text_units(text)?;
    number_index(numeric_length(&units))
        .ok_or_else(|| type_error("source text exceeds the bounded byte index model", None))
}

/// Exact UTF-8 byte, or `None` for a nonintegral or out-of-bounds position.
pub fn read_source_text_byte(text: &str, offset: f64) -> Result<Option<u8>> {
    let units = read_source_text_units(text)?;
    Ok(number_index(numeric_byte(&units, offset)).and_then(|byte| u8::try_from(byte).ok()))
}

/// Unchanged substring at ordered UTF-8 byte boundaries, or `None`.
pub fn slice_source_text_bytes(text: &str, start: f64, end: f64) -> Result<Option<&str>> {
    let units = read_source_text_units(text)?;
    if !accept_source_text_byte_slice(&units, start, end) {
        return Ok(None);
    }
    let Some(start) = number_index(start) else {
        return Ok(None);
    };
    let Some(end) = number_index(end) else {
        return Ok(None);
    };
    Ok(text.get(start..end))
}
