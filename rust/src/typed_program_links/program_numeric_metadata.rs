//! Bind dynamic metadata records to the shared JavaScript Number decision.
use super::{Result, error};
use serde_json::{Map, Value};

/// Normalize only the supported fixed type width before reading an integer.
pub fn normalize_program_integral_metadata_record(record: &mut Map<String, Value>) -> Result<()> {
    if record.get("kind").and_then(Value::as_str) == Some("fixed")
        && let Some(number) = record.get("bits").and_then(Value::as_f64)
        && super::program_numeric_metadata_rules::accept_program_integral_metadata(
            "fixed",
            "bits",
            number,
            number == 0.0 && number.is_sign_negative(),
        )
    {
        let width = number
            .to_string()
            .parse::<u64>()
            .map_err(|_| error("a linked machine integer width is invalid"))?;
        record.insert("bits".to_owned(), Value::from(width));
    }
    Ok(())
}

/// Prepare a bounded comparison view without rebuilding any owned records.
pub fn normalize_program_integral_metadata(value: &mut Value) -> Result<()> {
    inspect(value, 0, &mut 100_000)
}

fn inspect(value: &mut Value, depth: usize, remaining: &mut usize) -> Result<()> {
    *remaining = remaining
        .checked_sub(1)
        .ok_or_else(|| error("integral program metadata exceeds its traversal limit"))?;
    if depth > 256 {
        return Err(error(
            "integral program metadata exceeds its traversal limit",
        ));
    }
    if let Some(record) = value.as_object_mut() {
        normalize_program_integral_metadata_record(record)?;
        for child in record.values_mut() {
            inspect(child, depth + 1, remaining)?;
        }
    } else if let Some(values) = value.as_array_mut() {
        for child in values {
            inspect(child, depth + 1, remaining)?;
        }
    }
    Ok(())
}
