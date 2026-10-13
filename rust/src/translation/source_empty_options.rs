//! Host records for the independently selected empty primitive option proof.
//! Shared translated decisions own the model, source paths and value shape.
use super::diagnostics::{Result, type_error};
use super::ir::Program;
use super::rust::empty_option_rules::{
    accept_rust_empty_option_constructor_owner, accept_rust_empty_option_context,
    accept_rust_empty_option_expression, accept_rust_empty_option_function,
    accept_rust_empty_option_result_owner,
};
use serde_json::{Value, json};
use std::sync::LazyLock;

#[must_use]
pub(crate) fn policy() -> &'static Value {
    static POLICY: LazyLock<Value> = LazyLock::new(|| {
        crate::read_structured_data_lino(include_str!("../data/rust-empty-options.lino"))
            .expect("the shared empty option policy is valid")
    });
    &POLICY
}
fn fail(detail: &str) -> super::diagnostics::TranslationError {
    type_error(format!("Rust empty option proof: {detail}"), None)
}
fn fields(role: &str) -> Vec<&'static str> {
    policy()["fields"][role]
        .as_array()
        .expect("shared owned fields are ordered")
        .iter()
        .map(|field| field.as_str().expect("shared field names are strings"))
        .collect()
}
fn exact(value: &Value, role: &str) -> Result<()> {
    let fields = fields(role);
    if value.as_object().is_none_or(|record| {
        record.len() != fields.len() || fields.iter().any(|field| !record.contains_key(*field))
    }) {
        return Err(fail("unrecognized or missing owned fields"));
    }
    Ok(())
}
fn allowed(value: &Value, role: &str, detail: &str) -> Result<()> {
    let fields = fields(role);
    if value
        .as_object()
        .is_none_or(|record| record.keys().any(|field| !fields.contains(&field.as_str())))
    {
        return Err(fail(detail));
    }
    Ok(())
}
fn array<'a>(value: &'a Value, detail: &str) -> Result<&'a [Value]> {
    value
        .as_array()
        .map(Vec::as_slice)
        .ok_or_else(|| fail(detail))
}
fn span(value: &Value) -> Result<(u64, u64)> {
    if value.as_object().is_none_or(|record| {
        record.len() != 2 || !record.contains_key("start") || !record.contains_key("end")
    }) {
        return Err(fail("unrecognized or missing owned fields"));
    }
    let start = value["start"].as_u64();
    let end = value["end"].as_u64();
    match (start, end) {
        (Some(start), Some(end)) if end > start && end <= 9_007_199_254_740_991 => Ok((start, end)),
        _ => Err(fail("an actual source span is malformed")),
    }
}
const fn inside(inner: (u64, u64), outer: (u64, u64)) -> bool {
    inner.0 >= outer.0 && inner.1 <= outer.1
}
fn owner(value: &Value, construction: bool, names: &[Value]) -> Result<()> {
    exact(value, "owner")?;
    let source = array(
        &value["sourcePath"],
        "an actual standard owner path or its explicit type argument is invalid",
    )?;
    let path = source
        .iter()
        .map(|part| part.as_str())
        .collect::<Option<Vec<_>>>()
        .ok_or_else(|| {
            fail("an actual standard owner path or its explicit type argument is invalid")
        })?
        .join("::");
    let accepted = if construction {
        accept_rust_empty_option_constructor_owner(
            &path,
            source.len() == 1 && names.contains(&source[0]),
        )
    } else {
        accept_rust_empty_option_result_owner(&path, false)
    };
    let arguments = array(
        &value["arguments"],
        "an actual standard owner path or its explicit type argument is invalid",
    )?;
    if !accepted
        || value["sourcePath"] != value["resolvedPath"]
        || value["sourcePath"] != json!(path.split("::").collect::<Vec<_>>())
        || value["canonicalPath"]
            != policy()[if construction {
                "canonicalConstructor"
            } else {
                "canonicalResult"
            }]
        || arguments.len() != 1
    {
        return Err(fail(
            "an actual standard owner path or its explicit type argument is invalid",
        ));
    }
    let argument = &arguments[0];
    exact(argument, "argument")?;
    span(&argument["span"])?;
    if argument["type"] != policy()["elementType"]
        || ["sourcePath", "resolvedPath", "canonicalPath"]
            .iter()
            .any(|field| argument[*field] != policy()["elementPath"])
        || ["arguments", "wrappers", "lifetimes"]
            .iter()
            .any(|field| argument[*field] != json!([]))
    {
        return Err(fail(
            "the explicit element argument differs from direct primitive f64",
        ));
    }
    Ok(())
}
/// Validate actual raw records before serde can discard unknown or null fields.
pub(crate) fn validate_empty_option_records(value: &Value) -> Result<()> {
    let declarations = array(
        &value["declarations"],
        "the selected model requires a complete flat empty leaf program without imports or effects",
    )?;
    if !declarations
        .iter()
        .any(|entry| entry.get("sourceEmptyOption").is_some())
    {
        return Ok(());
    }
    let items = array(
        &value["items"],
        "the selected model requires a complete flat empty leaf program without imports or effects",
    )?;
    let main = &value["main"];
    let forbidden = declarations.is_empty()
        || declarations.len() != items.len()
        || main["effects"]
            .as_array()
            .is_none_or(|effects| !effects.is_empty())
        || items.iter().any(|item| item["k"] != "decl")
        || ["imports", "externals"].iter().any(|field| {
            value
                .get(*field)
                .is_some_and(|value| value.as_array().is_none_or(|entries| !entries.is_empty()))
        })
        || [
            "outputThreaded",
            "abortsThreaded",
            "referenceSemantics",
            "utf16Strings",
        ]
        .iter()
        .any(|field| {
            value
                .get(*field)
                .is_some_and(|value| value != &Value::Bool(false))
        });
    if !accept_rust_empty_option_context(
        value["sourceLanguage"].as_str().unwrap_or_default(),
        forbidden,
    ) {
        return Err(fail(
            "the selected model requires a complete flat empty leaf program without imports or effects",
        ));
    }
    allowed(
        value,
        "program",
        "the empty program has unrepresented own fields",
    )?;
    exact(main, "main")?;
    span(&main["span"])?;
    let names = items
        .iter()
        .map(|item| item["fullName"].clone())
        .collect::<Vec<_>>();
    for entry in declarations {
        allowed(
            entry,
            "declaration",
            "an empty function has unrepresented own fields",
        )?;
        let invalid = entry["params"]
            .as_array()
            .is_none_or(|params| !params.is_empty())
            || entry["modulePath"] != json!([])
            || entry["fullName"] != entry["name"]
            || entry["recursive"] != Value::Bool(false)
            || !entry["decreasing"].is_null()
            || ["mutual", "generated"].iter().any(|field| {
                entry
                    .get(*field)
                    .is_some_and(|value| value != &Value::Bool(false))
            });
        if !accept_rust_empty_option_function(entry["k"].as_str().unwrap_or_default(), invalid) {
            return Err(fail(
                "the actual declaration context contradicts its empty leaf ownership",
            ));
        }
        let proof = &entry["sourceEmptyOption"];
        exact(proof, "proof")?;
        if proof["model"] != policy()["model"]
            || proof["language"] != policy()["language"]
            || proof["sourceName"] != entry["name"]
            || proof["parameters"] != json!([])
        {
            return Err(fail(
                "the selected source name, interpretation or arity differs from its declaration",
            ));
        }
        exact(&proof["namespace"], "namespace")?;
        if proof["namespace"]["declaredTypeOwners"] != json!([])
            || proof["namespace"]["externalOptionRoots"] != policy()["externalOptionRoots"]
            || proof["namespace"]["imports"] != json!([])
            || proof["namespace"]["declarationNames"] != json!(names)
        {
            return Err(fail(
                "the namespace premise disagrees with the actual declarations",
            ));
        }
        let result = &proof["result"];
        exact(result, "position")?;
        let result_span = span(&result["span"])?;
        owner(&result["owner"], false, &names)?;
        if result["type"] != policy()["valueType"]
            || entry["ret"] != policy()["valueType"]
            || result["wrappers"] != json!([])
            || result["lifetimes"] != json!([])
            || !inside(span(&result["owner"]["arguments"][0]["span"])?, result_span)
        {
            return Err(fail(
                "the source result differs from the exact optional value interpretation",
            ));
        }
        let construction = &proof["construction"];
        exact(construction, "construction")?;
        let construction_span = span(&construction["span"])?;
        owner(&construction["owner"], true, &names)?;
        let body = &entry["body"];
        exact(body, "expression")?;
        span(&body["span"])?;
        let function_span = span(&entry["span"])?;
        if !accept_rust_empty_option_expression(
            body["k"].as_str().unwrap_or_default(),
            body["absence"].as_str().unwrap_or_default(),
        ) || body["type"] != policy()["valueType"]
            || construction["path"] != json!([])
            || construction["span"] != body["span"]
            || !inside(
                span(&construction["owner"]["arguments"][0]["span"])?,
                construction_span,
            )
            || !inside(result_span, function_span)
            || !inside(construction_span, function_span)
            || result_span.1 > construction_span.0
        {
            return Err(fail(
                "the actual empty constructor must own the complete checked body and its source site",
            ));
        }
    }
    Ok(())
}
/// Recheck the complete actual typed program before source-free emission.
pub(crate) fn validate_source_empty_options(program: &Program) -> Result<()> {
    let value = serde_json::to_value(program)
        .map_err(|_| fail("the empty program has unrepresented own fields"))?;
    validate_empty_option_records(&value)
}
