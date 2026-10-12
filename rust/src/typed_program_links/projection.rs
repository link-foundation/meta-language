//! Host JSON boundary adapter for the shared mechanical projection instructions.
use super::{Result, error, typed_program_links_schema};
use crate::translation::ir::Expr;
use crate::translation::surface::SExpr;
use crate::translation::typed_program_link_rules::read_program_expression_projection;
use serde_json::{Map, Value, json};

fn expression(value: &Value) -> Result<Value> {
    let plans = &typed_program_links_schema()["expressionProjections"];
    let mut key = value["k"].as_str().unwrap_or_default().to_owned();
    if let Some(path) = plans[&key]["variant"].as_array() {
        let discriminator = path.iter().fold(value, |value, field| {
            &value[field.as_str().unwrap_or_default()]
        });
        let variant = format!(
            "{key}:{}",
            discriminator
                .as_str()
                .map(str::to_owned)
                .unwrap_or_else(|| discriminator.to_string())
        );
        if !read_program_expression_projection(&variant).is_empty() {
            key = variant;
        }
    }
    let construct = plans[&key]["construct"].as_str().unwrap_or_default();
    if read_program_expression_projection(&key).is_empty() || construct.is_empty() {
        return Err(error(format!(
            "the linked expression {} has no surface projection",
            value["k"].as_str().unwrap_or_default()
        )));
    }
    let plan = &plans[&key];
    let mut result = Map::new();
    result.insert("k".to_owned(), Value::String(construct.to_owned()));
    if let Some(span) = value.get("span") {
        result.insert("span".to_owned(), span.clone());
    }
    let operation = value["op"].as_str().unwrap_or_default();
    let operation = plan["renames"][operation].as_str().unwrap_or(operation);
    for rule in plan["fields"].as_array().unwrap() {
        let target = rule[0].as_str().unwrap();
        let mode = rule[1].as_str().unwrap();
        let source = rule[2].as_str().unwrap();
        let child = &value[source];
        let projected = match mode {
            "copy" => {
                if value.get(source).is_none() {
                    continue;
                }
                child.clone()
            }
            "expression" => expression(child)?,
            "expressions" => Value::Array(
                child
                    .as_array()
                    .ok_or_else(|| error("a projection expression sequence is not an array"))?
                    .iter()
                    .map(expression)
                    .collect::<Result<_>>()?,
            ),
            "name-path" => json!([child]),
            "function-path" => json!({"k":"name", "path":[child]}),
            "array-items" => {
                let spread = plan["spreadOperations"]
                    .as_array()
                    .is_some_and(|operations| operations.contains(&value["op"]));
                Value::Array(
                    child
                        .as_array()
                        .ok_or_else(|| error("projection array items are not an array"))?
                        .iter()
                        .map(|item| -> Result<Value> {
                            Ok(json!({"value":expression(item)?,"spread":spread}))
                        })
                        .collect::<Result<_>>()?,
                )
            }
            "spread-pair" => Value::Array(
                [child, &value[rule[3].as_str().unwrap()]]
                    .into_iter()
                    .map(|item| -> Result<Value> {
                        Ok(json!({"value":expression(item)?,"spread":true}))
                    })
                    .collect::<Result<_>>()?,
            ),
            "element-type" => child["element"].clone(),
            "binding-type" => child["type"].clone(),
            "integer-type" => Value::Bool(child["kind"] != "float"),
            "operation" => Value::String(operation.to_owned()),
            "operation-name" => Value::String(format!("Math.{operation}")),
            "constant" => rule[2].clone(),
            _ => {
                return Err(error(format!(
                    "unknown shared projection instruction {mode}"
                )));
            }
        };
        result.insert(target.to_owned(), projected);
    }
    Ok(Value::Object(result))
}

pub(super) fn project(value: &Expr) -> Result<SExpr> {
    let value = serde_json::to_value(value).map_err(|failure| error(failure.to_string()))?;
    serde_json::from_value(expression(&value)?).map_err(|failure| error(failure.to_string()))
}
