//! Mechanical instantiation of the same complete callable template as JavaScript.
//! The ordinary checker owns all operations, binders, argument types and effects.

use std::sync::OnceLock;

use serde_json::Value;

use super::{Result, SExpr, Span, type_error};

fn policy() -> &'static Value {
    static POLICY: OnceLock<Value> = OnceLock::new();
    POLICY.get_or_init(|| {
        crate::read_structured_data_lino(include_str!("../../data/string-code-unit-template.lino"))
            .expect("the packaged code-unit callable template is valid Links data")
    })
}

fn instantiate(template: &Value, span: Option<Span>) -> Value {
    match template {
        Value::Array(values) => Value::Array(
            values
                .iter()
                .map(|value| instantiate(value, span))
                .collect(),
        ),
        Value::Object(fields) => {
            let mut value = fields
                .iter()
                .map(|(key, value)| (key.clone(), instantiate(value, span)))
                .collect::<serde_json::Map<_, _>>();
            if value.contains_key("k") || value.contains_key("span") {
                value.insert(
                    "span".to_owned(),
                    serde_json::to_value(span).expect("a source span serializes"),
                );
            }
            Value::Object(value)
        }
        value => value.clone(),
    }
}

pub(super) fn expression(span: Option<Span>) -> Result<SExpr> {
    let policy = policy();
    if policy.get("schemaVersion").and_then(Value::as_u64) != Some(1)
        || policy.get("model").and_then(Value::as_str) != Some("portable UTF16 code unit lambda 1")
        || policy.pointer("/template/k").and_then(Value::as_str) != Some("lambda")
    {
        return Err(type_error(
            "the generated UTF-16 code unit template is malformed",
            span,
        ));
    }
    serde_json::from_value(instantiate(&policy["template"], span)).map_err(|_| {
        type_error(
            "the generated UTF-16 code unit template is not a representable expression",
            span,
        )
    })
}

pub(super) fn character_expression(span: Option<Span>) -> Result<SExpr> {
    let policy = &policy()["character"];
    if policy.get("schemaVersion").and_then(Value::as_u64) != Some(1)
        || policy.get("model").and_then(Value::as_str) != Some("portable UTF16 character lambda 1")
        || policy.pointer("/template/k").and_then(Value::as_str) != Some("lambda")
    {
        return Err(type_error(
            "the generated UTF-16 character template is malformed",
            span,
        ));
    }
    serde_json::from_value(instantiate(&policy["template"], span)).map_err(|_| {
        type_error(
            "the generated UTF-16 character template is not a representable expression",
            span,
        )
    })
}

pub(super) fn relative_character_expression(span: Option<Span>) -> Result<SExpr> {
    static POLICY: OnceLock<Value> = OnceLock::new();
    let policy = POLICY.get_or_init(|| {
        crate::read_structured_data_lino(include_str!(
            "../../data/string-relative-character-template.lino"
        ))
        .expect("the packaged relative character callable template is valid Links data")
    });
    if policy.get("schemaVersion").and_then(Value::as_u64) != Some(1)
        || policy.get("model").and_then(Value::as_str)
            != Some("portable UTF16 relative character lambda 1")
        || policy.pointer("/template/k").and_then(Value::as_str) != Some("lambda")
    {
        return Err(type_error(
            "the generated UTF-16 relative character template is malformed",
            span,
        ));
    }
    serde_json::from_value(instantiate(&policy["template"], span)).map_err(|_| {
        type_error(
            "the generated UTF-16 relative character template is not a representable expression",
            span,
        )
    })
}
