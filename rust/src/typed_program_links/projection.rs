//! Host JSON boundary adapter for the shared mechanical projection instructions.
use super::{Result, error, typed_program_links_schema};
use crate::translation::ir::{Decl, Expr, Program};
use crate::translation::surface::SExpr;
use crate::translation::typed_program_link_rules::read_program_expression_projection;
use crate::translation::types::Type;
use serde_json::{Map, Value, json};

fn expression(value: &Value, program: &Program) -> Result<Value> {
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
            "expression" => expression(child, program)?,
            "expressions" => Value::Array(
                child
                    .as_array()
                    .ok_or_else(|| error("a projection expression sequence is not an array"))?
                    .iter()
                    .map(|item| expression(item, program))
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
                            Ok(json!({"value":expression(item, program)?,"spread":spread}))
                        })
                        .collect::<Result<_>>()?,
                )
            }
            "spread-pair" => Value::Array(
                [child, &value[rule[3].as_str().unwrap()]]
                    .into_iter()
                    .map(|item| -> Result<Value> {
                        Ok(json!({"value":expression(item, program)?,"spread":true}))
                    })
                    .collect::<Result<_>>()?,
            ),
            "element-type" => child["element"].clone(),
            "binding-type" => child["type"].clone(),
            "constructor-path" => {
                let mut path: Vec<_> = value["data"]
                    .as_str()
                    .unwrap_or_default()
                    .split('.')
                    .map(str::to_owned)
                    .collect();
                path.push(child.as_str().unwrap_or_default().to_owned());
                json!({"k":"name", "path":path})
            }
            "constructor-fields" => {
                let Some(Decl::Data(declaration)) =
                    program.declaration(value["data"].as_str().unwrap_or_default())
                else {
                    return Err(error("the linked constructor has no data declaration"));
                };
                let constructor = declaration
                    .ctors
                    .iter()
                    .find(|constructor| Some(constructor.name.as_str()) == value["ctor"].as_str())
                    .ok_or_else(|| error("the linked constructor is not declared"))?;
                let arguments = child
                    .as_array()
                    .ok_or_else(|| error("the linked constructor arguments are not an array"))?;
                if arguments.len() != constructor.fields.len() {
                    return Err(error(
                        "the linked constructor projection differs from its declaration",
                    ));
                }
                let fields = constructor
                    .fields
                    .iter()
                    .zip(arguments)
                    .map(|(field, argument)| {
                        Ok((field.name.clone(), expression(argument, program)?))
                    })
                    .collect::<Result<Map<String, Value>>>()?;
                Value::Object(fields)
            }
            "match-cases" => Value::Array(
                child
                    .as_array()
                    .ok_or_else(|| error("the linked match cases are not an array"))?
                    .iter()
                    .map(|case| -> Result<Value> {
                        let mut pattern = case["pattern"].clone();
                        if pattern["k"] == "ctor" {
                            let mut path: Vec<_> = pattern["data"]
                                .as_str()
                                .unwrap_or_default()
                                .split('.')
                                .map(str::to_owned)
                                .collect();
                            path.push(pattern["ctor"].as_str().unwrap_or_default().to_owned());
                            pattern = json!({"k":"ctor", "path":path, "binds":pattern["binds"]});
                        }
                        let mut projected = Map::new();
                        projected.insert("pattern".to_owned(), pattern);
                        projected.insert("body".to_owned(), expression(&case["body"], program)?);
                        if let Some(span) = case.get("span") {
                            projected.insert("span".to_owned(), span.clone());
                        }
                        Ok(Value::Object(projected))
                    })
                    .collect::<Result<_>>()?,
            ),
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

pub(super) fn project(value: &Expr, program: &Program) -> Result<SExpr> {
    let value = serde_json::to_value(value).map_err(|failure| error(failure.to_string()))?;
    serde_json::from_value(expression(&value, program)?)
        .map_err(|failure| error(failure.to_string()))
}

/// Restore a resolved type name for the existing checker to look up again.
pub(super) fn project_type(value: &Type) -> Result<Type> {
    if let Type::Data { name } = value {
        return Ok(Type::Named {
            path: name.split('.').map(str::to_owned).collect(),
            span: None,
        });
    }
    value.map_children(project_type)
}
