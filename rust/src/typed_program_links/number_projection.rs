//! Borrowed JSON shape adapter for the generated numeric operand decision.
use super::{Result, error};
use crate::translation::optional_value_rules::read_optional_number_operation;
use crate::translation::program_number_projection_rules::accept_program_derived_number_operand;
use serde_json::Value;

fn float_type(value: &Value) -> bool {
    value.as_object().is_some_and(|fields| {
        fields.len() == 1 && fields.get("kind").and_then(Value::as_str) == Some("float")
    })
}

pub(super) fn read_linked_program_number_operand<'a>(
    parent: &Value,
    field: &str,
    value: &'a Value,
) -> Result<&'a Value> {
    if value["k"].as_str() != Some("nullishNumber") {
        return Ok(value);
    }
    if !accept_program_derived_number_operand(
        parent["k"].as_str().unwrap_or_default(),
        parent["op"].as_str().unwrap_or_default(),
        field,
    ) {
        return Ok(value);
    }
    let failure = || {
        error(
            "a linked derived number operand has inconsistent fields, types or arithmetic context",
        )
    };
    if !float_type(&parent["type"]) || !float_type(&parent["domain"]) {
        return Err(failure());
    }
    let fields = value.as_object().ok_or_else(failure)?;
    if fields.len() != 3
        || fields
            .keys()
            .any(|key| !matches!(key.as_str(), "k" | "arg" | "type"))
        || !float_type(&value["type"])
    {
        return Err(failure());
    }
    let child = &value["arg"];
    if !child.is_object()
        || read_optional_number_operation(
            "JavaScript",
            child["type"]["kind"].as_str().unwrap_or_default(),
            child["type"]["element"]["kind"]
                .as_str()
                .unwrap_or_default(),
        ) != "number"
    {
        return Err(failure());
    }
    Ok(child)
}
