//! Mechanical whole-surface transport; no declaration order replaces value data.
use super::diagnostics::{Result, type_error, unsupported};
use super::surface::SProgram;
use serde_json::Value;
use std::borrow::Cow;
fn selected(value: &Value) -> bool {
    if value["k"] == "math"
        && (!super::javascript_enumeration_rules::read_java_script_enumeration_form(
            value["op"].as_str().unwrap_or_default(),
        )
        .is_empty()
            || value["op"].as_str()
                == Some(
                    super::object_rest_rules::read_java_script_object_rest_operation()
                        .to_string()
                        .as_str(),
                ))
    {
        return true;
    }
    match value {
        Value::Array(items) => items.iter().any(selected),
        Value::Object(fields) => fields
            .iter()
            .any(|(key, value)| key != "span" && !key.starts_with("source") && selected(value)),
        _ => false,
    }
}
fn external(value: &Value) -> Result<()> {
    if value["kind"] == "record" && value["ordered"] != true {
        return Err(unsupported(
            "ordered record external interface",
            "an actual foreign record signature must already retain its own-field order",
            None,
        ));
    }
    match value {
        Value::Array(items) => {
            for value in items {
                external(value)?;
            }
        }
        Value::Object(fields) => {
            for value in fields.values() {
                external(value)?;
            }
        }
        _ => {}
    }
    Ok(())
}
fn mark(value: &mut Value) {
    match value {
        Value::Array(items) => {
            for value in items {
                mark(value);
            }
        }
        Value::Object(fields) => {
            let ordered = fields.get("kind").and_then(Value::as_str) == Some("record");
            for value in fields.values_mut() {
                mark(value);
            }
            if ordered {
                fields.insert("ordered".to_owned(), Value::Bool(true));
            }
        }
        _ => {}
    }
}
pub(super) fn prepare(surface: &SProgram) -> Result<(Cow<'_, SProgram>, bool)> {
    if surface.language != super::Language::JavaScript {
        return Ok((Cow::Borrowed(surface), false));
    }
    let mut value =
        serde_json::to_value(surface).map_err(|error| type_error(error.to_string(), None))?;
    if !selected(&value["items"]) && !selected(&value["main"]) {
        return Ok((Cow::Borrowed(surface), false));
    }
    external(&value["externals"])?;
    mark(&mut value);
    Ok((
        Cow::Owned(
            serde_json::from_value(value).map_err(|error| type_error(error.to_string(), None))?,
        ),
        true,
    ))
}
