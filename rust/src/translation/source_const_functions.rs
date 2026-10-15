//! Bind actual owned records to the automatically translated const decisions.
use super::diagnostics::{Result, type_error};
use super::ir::Program;
use super::rust::const_function_rules::{
    accept_rust_const_function_container, accept_rust_const_function_interval,
    accept_rust_const_function_model, accept_rust_const_function_name,
    accept_rust_const_function_name_character, accept_rust_const_function_offset,
    accept_rust_const_function_type,
};
use super::source_signatures::{SourceSignature, validate_source_signature};
use super::types::Type;
use serde_json::{Value, json};
use std::collections::HashMap;
use std::sync::LazyLock;

mod expression;

/// The same inspectable policy consumed by the JavaScript const adapter.
#[must_use]
pub fn policy() -> &'static Value {
    static POLICY: LazyLock<Value> = LazyLock::new(|| {
        crate::read_structured_data_lino(include_str!("../data/rust-const-functions.lino"))
            .expect("the shared const function policy is valid")
    });
    &POLICY
}

fn fail(detail: &str) -> super::diagnostics::TranslationError {
    type_error(format!("Rust const function proof: {detail}"), None)
}

fn fields(value: &Value, allowed: &Value, exact: bool) -> Result<()> {
    let represented = value.as_object().is_some_and(|record| {
        allowed.as_array().is_some_and(|allowed| {
            (!exact || record.len() == allowed.len())
                && record
                    .keys()
                    .all(|name| allowed.contains(&Value::String(name.clone())))
        })
    });
    if !accept_rust_const_function_container("record", true, !represented) {
        return Err(fail("an owned record has unrepresented or missing fields"));
    }
    Ok(())
}

fn array<'a>(value: &'a Value, detail: &str) -> Result<&'a [Value]> {
    if !accept_rust_const_function_container("array", value.is_array(), false) {
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

fn name_accepted(value: &Value) -> bool {
    value.as_str().is_some_and(|name| {
        accept_rust_const_function_name(name)
            && name.chars().enumerate().all(|(index, character)| {
                accept_rust_const_function_name_character(
                    f64::from(u32::from(character)),
                    index == 0,
                )
            })
    })
}

fn offset(value: &Value) -> Result<(u64, f64)> {
    let number = value
        .as_f64()
        .filter(|number| {
            accept_rust_const_function_offset(*number, *number == 0.0 && number.is_sign_negative())
        })
        .ok_or_else(|| fail("a source interval has an invalid shape or offset"))?;
    let integer = number
        .to_string()
        .parse()
        .map_err(|_| fail("a source interval has an invalid shape or offset"))?;
    Ok((integer, number))
}

fn interval(value: &Value, nonempty: bool) -> Result<(u64, u64)> {
    fields(value, &json!(["start", "end"]), true)?;
    let (start, start_number) = offset(&value["start"])?;
    let (end, end_number) = offset(&value["end"])?;
    if !accept_rust_const_function_interval(start_number, end_number, nonempty) {
        return Err(fail("a source interval has an invalid shape or offset"));
    }
    Ok((start, end))
}

fn same_order(left: &Value, right: &Value) -> Result<bool> {
    let left = serde_json::to_string(left).map_err(|failure| fail(&failure.to_string()))?;
    let right = serde_json::to_string(right).map_err(|failure| fail(&failure.to_string()))?;
    Ok(left == right)
}

fn selected(value: &Value) -> Result<bool> {
    let mut pending = vec![(value, 0)];
    let mut visits = 0;
    let mut selected = false;
    let mut limited = false;
    while let Some((value, depth)) = pending.pop() {
        if !value.is_object() && !value.is_array() {
            continue;
        }
        visits += 1;
        if depth > 256 || visits > 100_000 {
            limited = true;
            continue;
        }
        if let Some(record) = value.as_object() {
            selected |= record.contains_key("sourceConstFunction");
            pending.extend(record.values().map(|child| (child, depth + 1)));
        } else if let Some(items) = value.as_array() {
            pending.extend(items.iter().map(|child| (child, depth + 1)));
        }
    }
    if selected && limited {
        return Err(fail("the predicate exceeds its semantic traversal limit"));
    }
    Ok(selected)
}

fn validate_interface(entry: &Value) -> Result<HashMap<String, Value>> {
    let parameters = array(
        &entry["params"],
        "the predicate needs actual direct primitive parameters and a Boolean result",
    )?;
    if entry["ret"]["kind"] != "bool" {
        return Err(fail(
            "the predicate needs actual direct primitive parameters and a Boolean result",
        ));
    }
    fields(&entry["ret"], &json!(["kind"]), true)?;
    let mut types = Vec::with_capacity(parameters.len());
    for parameter in parameters {
        fields(&parameter["type"], &json!(["kind"]), true)?;
        let ty: Type = serde_json::from_value(parameter["type"].clone())
            .map_err(|failure| fail(&failure.to_string()))?;
        types.push(ty);
    }
    let raw = &entry["sourceSignature"];
    let positions = array(
        &raw["parameters"],
        "the interface needs actual unwrapped primitive bool or f64 owners",
    )?;
    for position in positions.iter().chain(std::iter::once(&raw["result"])) {
        fields(&position["type"], &json!(["kind"]), true)?;
    }
    let signature: SourceSignature =
        serde_json::from_value(raw.clone()).map_err(|failure| fail(&failure.to_string()))?;
    validate_source_signature(&signature, &types, &Type::Bool, "Rust")?;
    if !signature.lifetime_bindings.is_empty() {
        return Err(fail(
            "lifetime binders require an independent source interpretation",
        ));
    }
    for position in signature
        .parameters
        .iter()
        .chain(std::iter::once(&signature.result))
    {
        if !accept_rust_const_function_type(position.ty.kind())
            || !position.wrappers.is_empty()
            || !position.lifetimes.is_empty()
            || position.owner.kind != "primitive"
            || position.owner.path.len() != 1
            || policy()["primitiveOwners"][position.ty.kind()].as_str()
                != Some(position.owner.path[0].as_str())
            || !position.owner.arguments.is_empty()
        {
            return Err(fail(
                "the interface needs actual unwrapped primitive bool or f64 owners",
            ));
        }
    }
    let mut environment = HashMap::new();
    for parameter in parameters {
        fields(parameter, &policy()["fields"]["parameter"], false)?;
        let name = parameter["name"].as_str().unwrap_or_default();
        if !name_accepted(&parameter["name"])
            || !parameter["guard"].is_null()
            || !parameter["default"].is_null()
            || environment.contains_key(name)
            || contains(&policy()["reservedOwners"], &parameter["name"])
        {
            return Err(fail("a parameter has an unproved source owner or default"));
        }
        environment.insert(name.to_owned(), parameter["type"].clone());
    }
    Ok(environment)
}

fn validate_declaration(entry: &Value) -> Result<()> {
    fields(entry, &policy()["fields"]["declaration"], false)?;
    let proof = &entry["sourceConstFunction"];
    fields(proof, &policy()["fields"]["proof"], true)?;
    if entry["k"] != "fn"
        || !name_accepted(&entry["name"])
        || entry["modulePath"] != json!([])
        || entry["recursive"] != false
        || !entry["decreasing"].is_null()
        || ["generated", "mutual"].iter().any(|field| {
            entry
                .get(*field)
                .is_some_and(|value| value != &Value::Bool(false))
        })
        || contains(&policy()["reservedOwners"], &entry["name"])
        || entry["fullName"] != entry["name"]
    {
        return Err(fail(
            "the qualification needs an original closed flat predicate declaration",
        ));
    }
    let model = proof["model"].as_str().unwrap_or_default();
    if !accept_rust_const_function_model(model)
        || proof["model"] != policy()["model"]
        || proof["language"] != "Rust"
        || proof["sourceName"] != entry["name"]
        || proof["keyword"] != policy()["keyword"]
        || !same_order(&proof["interpretation"], &policy()["interpretation"])?
    {
        return Err(fail(
            "the actual const source qualification or observation boundary changed",
        ));
    }
    let (site_start, site_end) = interval(&entry["span"], true)?;
    let (keyword_start, keyword_end) = interval(&proof["keywordSpan"], true)?;
    let (function_start, function_end) = interval(&proof["functionKeywordSpan"], true)?;
    let (header_start, header_end) = interval(&proof["headerSpan"], true)?;
    let body = interval(&proof["bodySpan"], true)?;
    let (body_start, body_end) = body;
    let keyword_width = u64::try_from(policy()["keyword"].as_str().unwrap().len()).unwrap();
    let function_width =
        u64::try_from(policy()["functionKeyword"].as_str().unwrap().len()).unwrap();
    if keyword_end - keyword_start != keyword_width
        || function_end - function_start != function_width
        || keyword_end > function_start
        || header_start != keyword_start
        || site_start != keyword_start
        || header_end != body_start
        || function_end >= header_end
        || body_end > site_end
    {
        return Err(fail(
            "the keyword, header and body intervals have contradictory ownership",
        ));
    }
    let environment = validate_interface(entry)?;
    let mut visits = 0;
    expression::validate(&entry["body"], &environment, body, 0, &mut visits)?;
    if entry["body"]["type"]["kind"] != "bool" {
        return Err(fail("the actual checked body does not return Boolean"));
    }
    Ok(())
}

/// Check raw selected values before serde can discard unknown fields or flags.
pub fn validate_const_function_records(value: &Value) -> Result<()> {
    if !selected(value)? {
        return Ok(());
    }
    let entries = array(
        &value["declarations"],
        "the model requires a complete flat Rust predicate program and an empty main",
    )?;
    fields(value, &policy()["fields"]["program"], false)?;
    let items = array(
        &value["items"],
        "the model requires a complete flat Rust predicate program and an empty main",
    )?;
    if value["sourceLanguage"] != "Rust"
        || items.is_empty()
        || value["main"].is_null()
        || value["main"]["effects"]
            .as_array()
            .is_none_or(|effects| !effects.is_empty())
        || ["imports", "externals"].iter().any(|field| {
            value
                .get(*field)
                .is_some_and(|value| value.as_array().is_none_or(|items| !items.is_empty()))
        })
    {
        return Err(fail(
            "the model requires a complete flat Rust predicate program and an empty main",
        ));
    }
    if [
        "referenceSemantics",
        "utf16Strings",
        "outputThreaded",
        "abortsThreaded",
    ]
    .iter()
    .any(|field| {
        value
            .get(*field)
            .is_some_and(|value| value != &Value::Bool(false))
    }) {
        return Err(fail(
            "another runtime interpretation needs an independent composition proof",
        ));
    }
    fields(
        &value["main"],
        &json!(["effects", "span", "sequentialAsync"]),
        false,
    )?;
    if value["main"]
        .get("sequentialAsync")
        .is_some_and(|value| value != &Value::Bool(false))
    {
        return Err(fail("the main cannot be asynchronous"));
    }
    if !value["main"]["span"].is_null() {
        interval(&value["main"]["span"], false)?;
    }
    if items.len() != entries.len()
        || items
            .iter()
            .zip(entries)
            .any(|(item, entry)| item["k"] != "decl" || item["fullName"] != entry["fullName"])
    {
        return Err(fail(
            "the actual ordered declarations differ from the program items",
        ));
    }
    for entry in entries {
        validate_declaration(entry)?;
    }
    Ok(())
}

/// Recheck native typed programs without changing any represented proof order.
pub fn validate_const_functions(program: &Program) -> Result<()> {
    validate_const_function_records(
        &serde_json::to_value(program).map_err(|failure| fail(&failure.to_string()))?,
    )
}
