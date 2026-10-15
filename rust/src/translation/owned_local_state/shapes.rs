//! Bind actual host records to shared contextual flag and source span rules.
use super::{Result, Value, contains, fail, policy};
use crate::translation::rust::owned_local_state_rules::{
    accept_rust_owned_local_record_value, accept_rust_owned_local_source_span,
};

pub(super) fn validate(program: &Value) -> Result<()> {
    inspect(program, Some("program"), 0, &mut 0)
}

fn inspect(value: &Value, context: Option<&str>, depth: usize, visits: &mut usize) -> Result<()> {
    *visits += 1;
    if *visits > 100_000 || depth > 256 {
        return Err(fail("record shapes exceed their traversal limit"));
    }
    if let Some(values) = value.as_array() {
        for child in values {
            inspect(child, None, depth + 1, visits)?;
        }
    }
    let Some(record) = value.as_object() else {
        return Ok(());
    };
    let context = context.unwrap_or_else(|| value["k"].as_str().unwrap_or_default());
    for (field, child) in record {
        let required = contains(
            &policy()["recordBooleanFields"][context],
            &Value::String(field.clone()),
        );
        if !accept_rust_owned_local_record_value(required, child.is_boolean()) {
            return Err(fail("a contextual flag has a non-Boolean value"));
        }
        if field == "span" {
            let shape = child.as_object().is_some_and(|span| {
                span.len() == 2 && span.contains_key("start") && span.contains_key("end")
            });
            let offsets = match (
                child["start"].as_u64(),
                child["end"].as_u64(),
                policy()["sourceOffsetMaximum"].as_u64(),
            ) {
                (Some(start), Some(end), Some(maximum)) => start <= end && end <= maximum,
                _ => false,
            };
            if !accept_rust_owned_local_source_span(!child.is_null(), shape, offsets) {
                return Err(fail("a source span has an invalid owned shape or offset"));
            }
        }
        inspect(
            child,
            if context == "program" && field == "main" {
                Some("main")
            } else {
                None
            },
            depth + 1,
            visits,
        )?;
    }
    Ok(())
}
