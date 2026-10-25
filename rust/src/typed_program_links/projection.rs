//! Host JSON boundary adapter for the shared mechanical projection instructions.
use super::{Result, error, typed_program_links_schema};
use crate::translation::ir::{Decl, Expr, Program};
use crate::translation::record_value_rules::{
    read_record_projection, read_record_projection_error,
};
use crate::translation::surface::SExpr;
use crate::translation::surface_reference_analysis_rules::{
    read_surface_reference_payload_operand, read_surface_reference_projection_form,
};
use crate::translation::typed_program_link_rules::{
    accept_program_expression_field, read_program_expression_projection,
};
use crate::translation::types::Type;
use serde_json::{Map, Value, json};

#[path = "number_projection.rs"]
mod number_projection;

fn expression(value: &Value, program: &Program) -> Result<Value> {
    if crate::translation::rust_array_length_rules::accept_rust_exact_array_length_result(
        value["k"].as_str().unwrap_or_default(),
        value["type"]["kind"].as_str().unwrap_or_default(),
        value["type"]["bits"].as_f64().unwrap_or_default(),
        value["type"]["signed"] == true,
        value["array"]["type"]["kind"].as_str().unwrap_or_default(),
    ) {
        let mut result = json!({"k":"cast","arg":{"k":"length","object":expression(&value["array"],program)?,"integer":true},"to":value["type"],"flavor":"exact"});
        if let Some(span) = value.get("span") {
            result["span"] = span.clone();
        }
        return Ok(result);
    }
    if value["k"] == "binary"
        && crate::translation::rust_shared_array_handle_rules::read_rust_shared_array_handle_comparison_projection(
            program.source_language.as_str(), value["op"].as_str().unwrap_or_default(),
            value["domain"]["kind"].as_str().unwrap_or_default(), value["domain"]["reference"] == true,
        ) == "referenceIdentity"
    {
        let mut result = Map::new();
        result.insert("k".to_owned(), Value::String("referenceIdentity".to_owned()));
        result.insert("op".to_owned(), value["op"].clone());
        result.insert("left".to_owned(), expression(&value["left"], program)?);
        result.insert("right".to_owned(), expression(&value["right"], program)?);
        if let Some(span) = value.get("span") { result.insert("span".to_owned(), span.clone()); }
        return Ok(Value::Object(result));
    }
    if value["k"].as_str() == Some("record")
        && value.as_object().is_some_and(|fields| {
            fields
                .keys()
                .any(|field| !accept_program_expression_field("record", field))
        })
    {
        return Err(error(
            "a linked record expression has an unrecognized projection field",
        ));
    }
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
    let reference_form = read_surface_reference_projection_form(
        value["k"].as_str().unwrap_or_default(),
        value["type"]["kind"].as_str().unwrap_or_default(),
        value["type"]["reference"].as_bool().unwrap_or(false),
        program.reference_semantics,
    );
    if construct == "operand" {
        let mut projected = expression(
            &value[plan["operand"].as_str().unwrap_or_default()],
            program,
        )?;
        let payload =
            read_surface_reference_payload_operand(projected["k"].as_str().unwrap_or_default());
        if reference_form == "allocation" && !payload.is_empty() {
            projected = projected
                .get_mut(&payload)
                .ok_or_else(|| error("a projected reference payload has no owned value"))?
                .take();
        }
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
            "expression" => expression(
                number_projection::read_linked_program_number_operand(
                    value,
                    source.unwrap_or_default(),
                    child,
                )?,
                program,
            )?,
            "expressions" => Value::Array(
                child
                    .as_array()
                    .ok_or_else(|| error("a projection expression sequence is not an array"))?
                    .iter()
                    .map(|item| expression(item, program))
                    .collect::<Result<_>>()?,
            ),
            "optional-expression" => {
                if child.is_null() {
                    continue;
                }
                expression(child, program)?
            }
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
            "declaration-path" => json!(project_declaration_path(
                child.as_str().unwrap_or_default(),
                program
            )?),
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
                } else if let Some(Decl::Fn(function)) =
                    child.as_str().and_then(|name| program.declaration(name))
                {
                    let name = child.as_str().unwrap();
                    let actual = function
                        .module_path
                        .iter()
                        .cloned()
                        .chain(std::iter::once(function.name.clone()))
                        .collect::<Vec<_>>();
                    if function.full_name != name || actual.join(".") != name {
                        return Err(error(
                            "a projected function must target its actual declaration identity",
                        ));
                    }
                    std::iter::once(Value::String("crate".to_owned()))
                        .chain(actual.into_iter().map(Value::String))
                        .collect()
                } else if owns_type_namespace(program) {
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
            "record-entries" => {
                let entries = child.as_array().ok_or_else(|| {
                    error("checked record projection needs an ordered field array")
                })?;
                let field_names = entries
                    .iter()
                    .map(|entry| {
                        entry["name"]
                            .as_str()
                            .unwrap_or_default()
                            .encode_utf16()
                            .map(f64::from)
                            .collect()
                    })
                    .collect::<Vec<Vec<f64>>>();
                let admitted = entries
                    .iter()
                    .map(|entry| {
                        entry.as_object().is_some_and(|fields| {
                            fields.get("name").is_some_and(Value::is_string)
                                && fields.get("value").is_some_and(Value::is_object)
                                && fields
                                    .keys()
                                    .all(|field| field == "name" || field == "value")
                        })
                    })
                    .collect::<Vec<_>>();
                let projection = read_record_projection();
                let refusal = read_record_projection_error(&projection, &field_names, &admitted);
                if !refusal.is_empty() {
                    return Err(error(refusal));
                }
                result.insert("recordProjection".to_owned(), Value::String(projection));
                Value::Array(entries.iter().map(|entry| -> Result<Value> {
                    Ok(json!({"name":entry["name"],"spread":false,"value":expression(&entry["value"], program)?}))
                }).collect::<Result<_>>()?)
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
    if read_surface_reference_payload_operand(&reference_form).is_empty() {
        Ok(Value::Object(result))
    } else {
        let mut payload = Map::new();
        payload.insert("k".to_owned(), Value::String(reference_form));
        payload.insert("value".to_owned(), Value::Object(result));
        if let Some(span) = value.get("span") {
            payload.insert("span".to_owned(), span.clone());
        }
        Ok(Value::Object(payload))
    }
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
