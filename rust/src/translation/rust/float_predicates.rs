//! Instantiate the same linked expression template used by the JS frontend.
//! Source receiver admission is performed by the automatically generated policy.

use super::{Result, SExpr, Span, unsupported};
use serde_json::Value;
use std::sync::OnceLock;

fn policy() -> &'static Value {
    static POLICY: OnceLock<Value> = OnceLock::new();
    POLICY.get_or_init(|| {
        crate::read_structured_data_lino(include_str!("float-predicates.lino"))
            .expect("the packaged float predicate policy is valid")
    })
}
fn instantiate(template: &Value, receiver: &Value, span: Option<Span>) -> Value {
    if template.get("slot").and_then(Value::as_str) == Some("receiver") {
        return receiver.clone();
    }
    match template {
        Value::Array(values) => Value::Array(
            values
                .iter()
                .map(|value| instantiate(value, receiver, span))
                .collect(),
        ),
        Value::Object(fields) => {
            let mut value = fields
                .iter()
                .map(|(key, value)| (key.clone(), instantiate(value, receiver, span)))
                .collect::<serde_json::Map<_, _>>();
            if value.contains_key("k") {
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
pub(super) fn expression(method: &str, receiver: &SExpr, range: Option<Span>) -> Result<SExpr> {
    selected_expression(
        &policy()["templates"],
        "predicate",
        method,
        Some(receiver),
        range,
    )
}

pub(super) fn operation_expression(
    method: &str,
    receiver: &SExpr,
    range: Option<Span>,
) -> Result<SExpr> {
    selected_expression(
        &policy()["operationTemplates"],
        "operation",
        method,
        Some(receiver),
        range,
    )
}

pub(super) fn constant_expression(name: &str, range: Option<Span>) -> Result<SExpr> {
    selected_expression(
        &policy()["constantTemplates"],
        "constant",
        name,
        None,
        range,
    )
}

fn selected_expression(
    templates: &Value,
    kind: &str,
    method: &str,
    receiver: Option<&SExpr>,
    range: Option<Span>,
) -> Result<SExpr> {
    let template = templates.get(method).ok_or_else(|| {
        unsupported(
            &format!("Rust binary64 {kind}"),
            &format!("the shared {kind} template is unknown"),
            range,
        )
    })?;
    let receiver = serde_json::to_value(receiver).expect("a parsed surface expression serializes");
    serde_json::from_value(instantiate(template, &receiver, range)).map_err(|_| {
        unsupported(
            &format!("Rust binary64 {kind}"),
            &format!("the shared {kind} template is not a representable expression"),
            range,
        )
    })
}
