//! UTF16 source slices selected by the automatically translated type scan.

use super::{Result, Span, js_trim, unsupported};
use crate::translation::frontend_rules::ml_array::number_index;
use crate::translation::type_syntax_rules::read_type_separator_positions;

fn source_slice(units: &[u16], start: usize, end: usize, range: Span) -> Result<String> {
    units
        .get(start..end)
        .and_then(|slice| String::from_utf16(slice).ok())
        .ok_or_else(|| {
            unsupported(
                "JSDoc type source boundary",
                "a selected type range must preserve complete UTF16 characters",
                Some(range),
            )
        })
}

/// Retain empty parts and their source order, exactly as JavaScript `splitType`.
pub(super) fn split_type(text: &str, separator: u16, range: Span) -> Result<Vec<String>> {
    let units = text.encode_utf16().collect::<Vec<_>>();
    let numeric = units.iter().copied().map(f64::from).collect::<Vec<_>>();
    let positions = read_type_separator_positions(&numeric, f64::from(separator));
    let mut parts = Vec::new();
    let mut start = 0;
    for position in positions {
        let end = number_index(position)
            .filter(|&end| end < units.len())
            .ok_or_else(|| {
                unsupported(
                    "JSDoc type source boundary",
                    "a selected separator must identify an actual UTF16 unit",
                    Some(range),
                )
            })?;
        parts.push(js_trim(&source_slice(&units, start, end, range)?).to_owned());
        start = end + 1;
    }
    parts.push(js_trim(&source_slice(&units, start, units.len(), range)?).to_owned());
    Ok(parts)
}

/// Slice around an already selected single ASCII unit, preserving real text.
pub(super) fn split_type_unit(text: &str, position: f64, range: Span) -> Result<(String, String)> {
    let units = text.encode_utf16().collect::<Vec<_>>();
    let at = number_index(position)
        .filter(|&at| at < units.len())
        .ok_or_else(|| {
            unsupported(
                "JSDoc type source boundary",
                "a selected field delimiter must identify an actual UTF16 unit",
                Some(range),
            )
        })?;
    Ok((
        source_slice(&units, 0, at, range)?,
        source_slice(&units, at + 1, units.len(), range)?,
    ))
}
