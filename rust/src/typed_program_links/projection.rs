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
    if let Some(alternatives) = typed_program_links_schema()["expressionProjectionAlternatives"]
        .get(&key)
        .and_then(Value::as_array)
    {
        for alternative in alternatives {
            let selected = alternative["path"]
                .as_array()
                .ok_or_else(|| error("a projection alternative has no field path"))?
                .iter()
                .fold(value, |value, field| {
                    &value[field.as_str().unwrap_or_default()]
                });
            if *selected == alternative["value"] {
                key = alternative["projection"]
                    .as_str()
                    .unwrap_or_default()
                    .to_owned();
                break;
            }
        }
    }
    if let Some(path) = plans[&key]["variant"].as_array() {
        let discriminator = path.iter().fold(value, |value, field| {
            &value[field.as_str().unwrap_or_default()]
        });
        let variant = format!(
            "{key}:{}",
            discriminator
                .as_str()
                .map_or_else(|| discriminator.to_string(), str::to_owned)
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
    if construct == "operand" {
        let mut projected = expression(
            &value[plan["operand"].as_str().unwrap_or_default()],
            program,
        )?;
        if let Some(span) = value.get("span")
            && projected.get("span").is_none()
        {
            projected
                .as_object_mut()
                .ok_or_else(|| error("a projected operand is not an expression record"))?
                .insert("span".to_owned(), span.clone());
        }
        return Ok(projected);
    }
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
        // Constant instructions contain their actual literal in position two;
        // only field instructions use that position as a source field name.
        let source = rule[2].as_str();
        let child = source.map_or(&Value::Null, |source| &value[source]);
        let projected = match mode {
            "copy" => {
                if source.and_then(|source| value.get(source)).is_none() {
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
            "nullable-expressions" => Value::Array(
                child
                    .as_array()
                    .ok_or_else(|| error("a projection expression sequence is not an array"))?
                    .iter()
                    .map(|item| {
                        if item.is_null() {
                            Ok(Value::Null)
                        } else {
                            expression(item, program)
                        }
                    })
                    .collect::<Result<_>>()?,
            ),
            "name-path" => json!([child]),
            "declaration-path" => json!(project_declaration_path(child.as_str().unwrap_or_default(), program)?),
            "return-type" => child["ret"].clone(),
            "function-path" => {
                let path = if let Some(Decl::Fn(function)) =
                    child.as_str().and_then(|name| program.declaration(name))
                    && let Some(continuation) = function
                        .source_loop
                        .as_ref()
                        .and_then(|proof| proof.continuation.as_ref())
                {
                    crate::translation::source_loops::validate_source_loop_at(
                        function,
                        program.source_language.as_str(),
                        child.as_str().unwrap(),
                    )
                    .map_err(|failure| error(failure.to_string()))?;
                    std::iter::once(Value::String(continuation.root.clone()))
                        .chain(continuation.path.iter().cloned().map(Value::String))
                        .collect()
                } else if owns_type_namespace(program)
                    || child.as_str().and_then(|name| program.declaration(name)).is_some_and(
                        |declaration| matches!(declaration, Decl::Fn(function) if !function.module_path.is_empty()),
                    )
                {
                    project_declaration_path(child.as_str().unwrap_or_default(), program)?
                        .into_iter()
                        .map(Value::String)
                        .collect()
                } else {
                    vec![child.clone()]
                };
                json!({"k":"name", "path":path})
            }
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
            "record-entries" => Value::Array(
                child.as_array().ok_or_else(|| error("projection record entries are not an array"))?
                    .iter().map(|entry| -> Result<Value> {
                        Ok(json!({"name":entry["name"],"spread":false,"value":expression(&entry["value"], program)?}))
                    }).collect::<Result<_>>()?,
            ),
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
                let mut path =
                    project_declaration_path(value["data"].as_str().unwrap_or_default(), program)?;
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
                            let mut path = project_declaration_path(
                                pattern["data"].as_str().unwrap_or_default(),
                                program,
                            )?;
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
pub(super) fn project_type(value: &Type, program: &Program) -> Result<Type> {
    if let Type::Data { name } = value {
        return Ok(Type::Named {
            path: project_declaration_path(name, program)?,
            span: None,
        });
    }
    value.map_children(|child| project_type(child, program))
}

// The caller independently rechecks the actual namespace facts before projection.
fn owns_type_namespace(program: &Program) -> bool {
    program.declarations.iter().any(|declaration| matches!(declaration, Decl::Fn(function) if function.source_type_namespace.is_some()) || matches!(declaration, Decl::Data(data) if data.source_unit_clone_derivation.is_some()))
}
fn project_declaration_path(name: &str, program: &Program) -> Result<Vec<String>> {
    if !owns_type_namespace(program) {
        return Ok(name.split('.').map(str::to_owned).collect());
    }
    let declaration = program
        .declaration(name)
        .ok_or_else(|| error("an owned namespace projection has no actual declaration target"))?;
    let actual = declaration
        .module_path()
        .iter()
        .cloned()
        .chain(std::iter::once(declaration.name().to_owned()))
        .collect::<Vec<_>>();
    if actual.join(".") != name {
        return Err(error(
            "an owned namespace projection disagrees with its actual declaration binding",
        ));
    }
    Ok(std::iter::once("crate".to_owned()).chain(actual).collect())
}
