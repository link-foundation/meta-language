//! Bind actual checked records to the shared owned local text decisions.
use super::diagnostics::{Result, type_error};
use super::ir::{Expr, Program};
use super::rust::owned_local_state_rules::{
    accept_rust_owned_local_container, accept_rust_owned_local_value_type,
};
use super::source_signatures::SourceTypeSignature;
use super::types::Type;
use serde_json::{Value, json};
use std::sync::LazyLock;

mod expression;
mod records;
mod shapes;
mod validation;

/// The same inspectable policy consumed by the JavaScript host.
#[must_use]
pub fn policy() -> &'static Value {
    static POLICY: LazyLock<Value> = LazyLock::new(|| {
        crate::read_structured_data_lino(include_str!("../data/rust-owned-local-state.lino"))
            .expect("the shared owned local state policy is valid")
    });
    &POLICY
}

fn fail(detail: &str) -> super::diagnostics::TranslationError {
    type_error(format!("Rust owned local state proof: {detail}"), None)
}

fn array<'a>(value: &'a Value, detail: &str) -> Result<&'a [Value]> {
    if !accept_rust_owned_local_container("array", value.is_array(), false) {
        return Err(fail(detail));
    }
    value
        .as_array()
        .map(Vec::as_slice)
        .ok_or_else(|| fail(detail))
}

fn contains(values: &Value, value: &Value) -> bool {
    values
        .as_array()
        .is_some_and(|values| values.contains(value))
}

fn exact(value: &Value, fields: &Value) -> Result<()> {
    let unrepresented = value.as_object().is_none_or(|record| {
        fields.as_array().is_none_or(|fields| {
            record.len() != fields.len()
                || fields.iter().any(|field| {
                    field
                        .as_str()
                        .is_none_or(|field| !record.contains_key(field))
                })
        })
    });
    if !accept_rust_owned_local_container("record", true, unrepresented) {
        return Err(fail("unrecognized or missing owned fields"));
    }
    Ok(())
}

fn allowed(value: &Value, fields: &Value) -> bool {
    let represented = value.as_object().is_some_and(|record| {
        record
            .keys()
            .all(|field| contains(fields, &Value::String(field.clone())))
    });
    accept_rust_owned_local_container("record", true, !represented)
}

fn same_order(left: &Value, right: &Value, ordered: bool) -> bool {
    if !ordered {
        return left == right;
    }
    match (left, right) {
        (Value::Object(left), Value::Object(right)) => {
            left.len() == right.len()
                && left
                    .iter()
                    .zip(right)
                    .all(|((name, value), (other, next))| {
                        name == other && same_order(value, next, true)
                    })
        }
        (Value::Array(left), Value::Array(right)) => {
            left.len() == right.len()
                && left
                    .iter()
                    .zip(right)
                    .all(|(value, next)| same_order(value, next, true))
        }
        _ => left == right,
    }
}

fn interval(value: &Value) -> Result<(u64, u64)> {
    exact(value, &json!(["start", "end"]))?;
    match (value["start"].as_u64(), value["end"].as_u64()) {
        (Some(start), Some(end)) if start < end && end <= 9_007_199_254_740_991 => Ok((start, end)),
        _ => Err(fail(
            "the function must retain its actual nonempty source-owner interval",
        )),
    }
}

fn value_type(value: &Value) -> bool {
    let kind = value["kind"].as_str().unwrap_or_default();
    let bits = value["bits"]
        .as_u64()
        .map_or_else(String::new, |bits| bits.to_string());
    accept_rust_owned_local_value_type(kind, &bits, value["signed"] == true)
        && exact(
            value,
            &if kind == "fixed" {
                json!(["kind", "bits", "signed"])
            } else {
                json!(["kind"])
            },
        )
        .is_ok()
}

fn pair_type(value: &Value) -> bool {
    value["kind"] == "tuple"
        && exact(value, &json!(["kind", "elements"])).is_ok()
        && value["elements"].as_array().is_some_and(|elements| {
            elements.len() == 2
                && elements[0]["kind"] == "string"
                && value_type(&elements[0])
                && elements[1]["kind"] == "fixed"
                && value_type(&elements[1])
        })
}

/// Read the translated classifier for an actual checked value type.
#[must_use]
pub fn owned_local_type(ty: &Type) -> bool {
    serde_json::to_value(ty).is_ok_and(|value| value_type(&value))
}

/// Check the policy's owned String and signed i32 tuple layout.
#[must_use]
pub fn owned_local_pair(ty: &Type) -> bool {
    serde_json::to_value(ty).is_ok_and(|value| pair_type(&value))
}

/// Bind the actual source owner to the exact unwrapped value interpretation.
#[must_use]
pub fn owned_local_position(position: &SourceTypeSignature) -> bool {
    let expected = match position.ty {
        Type::String => ("library", "std::string::String"),
        Type::Fixed { .. } => ("primitive", "i32"),
        _ => ("primitive", "bool"),
    };
    owned_local_type(&position.ty)
        && position.wrappers.is_empty()
        && position.lifetimes.is_empty()
        && position.owner.arguments.is_empty()
        && position.owner.kind == expected.0
        && position.owner.path.join("::") == expected.1
}

/// Resolve source facts to the actual checked lexical occurrence paths.
pub fn prepare_owned_local_state_proof(proof: &Value, body: &Expr) -> Result<Value> {
    let body =
        serde_json::to_value(body).map_err(|_| fail("a checked body cannot be represented"))?;
    records::prepare(proof, &body)
}

/// Validate raw selected records before serde can erase unknown or null fields.
pub fn validate_owned_local_state_records(value: &Value) -> Result<()> {
    validation::validate(value, false)
}

/// Recheck actual checked IR before source-free emission.
pub fn validate_owned_local_state(program: &Program) -> Result<()> {
    let value = serde_json::to_value(program)
        .map_err(|_| fail("a checked program cannot be represented"))?;
    validation::validate(&value, true)
}
