//! Normalize only named source spans at the raw structured-data boundary.
use super::source_span_rules::{accept_source_span_interval, accept_source_span_offset};
use super::{Result, error};
use serde_json::Value;

fn read_source_span_offset(value: &Value) -> Result<(u64, f64)> {
    let number = value
        .as_f64()
        .filter(|number| {
            accept_source_span_offset(*number, *number == 0.0 && number.is_sign_negative())
        })
        .ok_or_else(|| error("a linked source span is invalid"))?;
    // The shared rule proves the complete safe unsigned integral range.
    let integer = number
        .to_string()
        .parse()
        .map_err(|_| error("a linked source span is invalid"))?;
    Ok((integer, number))
}

/// Preserve record order and replace integral number aliases only inside spans.
pub fn normalize_source_spans(value: &mut Value) -> Result<()> {
    match value {
        Value::Object(record) => {
            if let Some(span) = record.get_mut("span").filter(|span| !span.is_null()) {
                let fields = span
                    .as_object_mut()
                    .filter(|fields| {
                        fields.len() == 2
                            && fields.contains_key("start")
                            && fields.contains_key("end")
                    })
                    .ok_or_else(|| error("a linked source span is invalid"))?;
                let (start, start_number) = read_source_span_offset(&fields["start"])?;
                let (end, end_number) = read_source_span_offset(&fields["end"])?;
                if !accept_source_span_interval(start_number, end_number) {
                    return Err(error("a linked source span is invalid"));
                }
                fields.insert("start".to_owned(), Value::from(start));
                fields.insert("end".to_owned(), Value::from(end));
            }
            for child in record.values_mut() {
                normalize_source_spans(child)?;
            }
        }
        Value::Array(items) => {
            for item in items {
                normalize_source_spans(item)?;
            }
        }
        _ => {}
    }
    Ok(())
}
