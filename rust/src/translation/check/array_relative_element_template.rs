//! Instantiate the complete shared array lambda; the ordinary checker owns behavior.
use std::sync::OnceLock;

use serde_json::Value;

use super::{Result, SExpr, Span, Type, type_error};

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

pub(super) fn expression(receiver: &Type, span: Option<Span>) -> Result<SExpr> {
    static POLICY: OnceLock<Value> = OnceLock::new();
    let policy = POLICY.get_or_init(|| {
        crate::read_structured_data_lino(include_str!(
            "../../data/array-relative-element-template.lino"
        ))
        .expect("the packaged relative array element template is valid Links data")
    });
    let Type::Array { element, .. } = receiver else {
        return Err(type_error(
            "the relative array element template requires an array",
            span,
        ));
    };
    if policy.get("schemaVersion").and_then(Value::as_u64) != Some(1)
        || policy.get("model").and_then(Value::as_str)
            != Some("portable dense array relative element lambda 1")
        || policy.pointer("/template/k").and_then(Value::as_str) != Some("lambda")
    {
        return Err(type_error(
            "the generated relative array element template is malformed",
            span,
        ));
    }
    let mut template = instantiate(&policy["template"], span);
    template["params"][0]["type"] =
        serde_json::to_value(receiver).expect("a checked receiver type serializes");
    template["ret"]["element"] =
        serde_json::to_value(element).expect("a checked element type serializes");
    serde_json::from_value(template).map_err(|_| {
        type_error(
            "the relative array element template is not a representable expression",
            span,
        )
    })
}
