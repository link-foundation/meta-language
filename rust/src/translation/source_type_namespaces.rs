//! Inspectable source spellings rechecked against the actual scoped catalog.
use super::diagnostics::{Result, type_error};
use super::ir::{Decl, Program};
use super::rust::type_namespaces::{
    TypeNamespaceCatalog, accept_namespace_segment, require_anchor,
};
use super::source_signatures::{SourceTypeSignature, validate_source_signature};
use super::type_namespace_rules::{
    accept_rust_type_namespace_container, accept_rust_type_namespace_declaration,
    accept_rust_type_namespace_fields, accept_rust_type_namespace_model,
    accept_rust_type_namespace_position, accept_rust_type_namespace_record_value,
};
use serde_json::Value;
use std::sync::LazyLock;

pub(super) fn policy() -> &'static Value {
    static POLICY: LazyLock<Value> = LazyLock::new(|| {
        crate::read_structured_data_lino(include_str!("../data/rust-type-namespace.lino"))
            .expect("the shared type namespace policy is valid")
    });
    &POLICY
}

fn fail(detail: &str) -> super::diagnostics::TranslationError {
    type_error(format!("an owned Rust type namespace {detail}"), None)
}

fn unrepresented(value: &Value, fields: &str) -> bool {
    value.as_object().is_none_or(|record| {
        record.keys().any(|name| {
            !policy()[fields]
                .as_array()
                .unwrap()
                .iter()
                .any(|field| field.as_str() == Some(name.as_str()))
        })
    })
}

fn validate_namespace_value_shapes(value: &Value, context: &str) -> Result<()> {
    match value {
        Value::Array(items) => {
            for item in items {
                let context = item["k"].as_str().unwrap_or_default();
                validate_namespace_value_shapes(item, context)?;
            }
        }
        Value::Object(record) => {
            for (field, child) in record {
                let boolean_required = policy()["recordBooleanFields"][context]
                    .as_array()
                    .is_some_and(|fields| {
                        fields
                            .iter()
                            .any(|name| name.as_str() == Some(field.as_str()))
                    });
                if !accept_rust_type_namespace_record_value(
                    boolean_required,
                    child.is_boolean(),
                    true,
                ) {
                    if context == "fn" && field == "generated" {
                        return Err(fail(
                            "requires original declarations with absent or false generated facts",
                        ));
                    }
                    return Err(fail("contains a non-Boolean contextual flag"));
                }
                if field == "span" && !child.is_null() {
                    let valid = child.as_object().is_some_and(|span| {
                        span.len() == 2
                            && child["start"]
                                .as_u64()
                                .zip(child["end"].as_u64())
                                .is_some_and(|(start, end)| {
                                    start <= end
                                        && end <= policy()["sourceOffsetMaximum"].as_u64().unwrap()
                                })
                    });
                    if !valid {
                        return Err(fail("contains an invalid owned source span"));
                    }
                }
                let child_context = if context == "program" && field == "main" {
                    "main"
                } else {
                    child["k"].as_str().unwrap_or_default()
                };
                validate_namespace_value_shapes(child, child_context)?;
            }
        }
        _ => {}
    }
    Ok(())
}

/// Inspect present fields before serde can discard or normalize their values.
pub fn validate_type_namespace_records(value: &Value) -> Result<()> {
    let Some(declarations) = value["declarations"].as_array() else {
        return Ok(());
    };
    if !declarations.iter().any(|entry| {
        entry.get("sourceTypeNamespace").is_some()
            || entry.get("sourceUnitCloneDerivation").is_some()
    }) {
        return Ok(());
    }
    validate_namespace_value_shapes(value, "program")?;
    if value["sourceLanguage"] != "Rust"
        || value
            .get("imports")
            .is_some_and(|value| value.as_array().is_none_or(|items| !items.is_empty()))
        || value
            .get("externals")
            .is_some_and(|value| value.as_array().is_none_or(|items| !items.is_empty()))
    {
        return Err(fail(
            "requires a Rust program without imported aliases or externals",
        ));
    }
    for entry in declarations.iter().filter(|entry| entry["k"] == "fn") {
        validate_type_namespace_declaration_record(entry)?;
    }
    Ok(())
}

/// Preserve raw selected declaration facts before any declaration projection.
pub fn validate_type_namespace_declaration_record(entry: &Value) -> Result<()> {
    if !accept_rust_type_namespace_fields(unrepresented(entry, "declarationFields")) {
        return Err(fail("function contains unrepresented interface fields"));
    }
    if entry
        .get("generated")
        .is_some_and(|value| value != &Value::Bool(false))
    {
        return Err(fail(
            "requires original declarations with absent or false generated facts",
        ));
    }
    // Native arrays have dense positions and cannot carry host properties;
    // the shared decision still selects the actual raw parameter container.
    if !accept_rust_type_namespace_container("array", entry["params"].is_array(), false) {
        return Err(fail(
            "parameter contains unrepresented interface fields or defaults",
        ));
    }
    let parameters = entry["params"]
        .as_array()
        .ok_or_else(|| fail("parameter contains unrepresented interface fields or defaults"))?;
    for parameter in parameters {
        if !accept_rust_type_namespace_fields(unrepresented(parameter, "parameterFields"))
            || !parameter["guard"].is_null()
            || !parameter["default"].is_null()
        {
            return Err(fail(
                "parameter contains unrepresented interface fields or defaults",
            ));
        }
    }
    let proof = &entry["sourceTypeNamespace"];
    let fields = policy()["fields"].as_array().unwrap();
    if proof.as_object().is_none_or(|record| {
        record.len() != fields.len()
            || fields
                .iter()
                .any(|field| !record.contains_key(field.as_str().unwrap()))
    }) || proof["model"]
        .as_str()
        .is_none_or(|model| model.is_empty() || !accept_rust_type_namespace_model(model))
        || proof["language"] != "Rust"
    {
        return Err(fail("has malformed or missing selected facts"));
    }
    let signature = &entry["sourceSignature"];
    if !signature.is_object()
        || proof["parameters"].as_array().is_none_or(|positions| {
            signature["parameters"]
                .as_array()
                .is_none_or(|owned| positions.len() != owned.len())
        })
    {
        return Err(fail(
            "must retain its owned source signature and ordered parameter positions",
        ));
    }
    Ok(())
}

fn spelling(value: &Value) -> Result<Vec<String>> {
    let parts = value.as_array().filter(|parts| {
        !parts.is_empty() && parts.iter().all(|part| part.as_str().is_some_and(accept_namespace_segment))
    }).ok_or_else(|| fail("position needs an actual primitive or datatype spelling and a proved direct result"))?;
    Ok(parts
        .iter()
        .map(|part| part.as_str().unwrap().to_owned())
        .collect())
}

fn position(
    value: &Value,
    signature: &SourceTypeSignature,
    path: &[String],
    result: bool,
    catalog: &TypeNamespaceCatalog,
) -> Result<()> {
    let source = spelling(value)?;
    let invalid_wrappers = signature.wrappers.iter().any(|wrapper| {
        !policy()["parameterWrappers"]
            .as_array()
            .unwrap()
            .iter()
            .any(|allowed| allowed.as_str() == Some(wrapper.as_str()))
    });
    if !accept_rust_type_namespace_position(
        &signature.owner.kind,
        invalid_wrappers,
        result && !signature.wrappers.is_empty(),
    ) {
        return Err(fail(
            "position needs an actual primitive or datatype spelling and a proved direct result",
        ));
    }
    require_anchor(&source, path)?;
    if let Some((name, kind)) = catalog.lookup(&source, path) {
        if !accept_rust_type_namespace_declaration(kind)
            || signature.owner.kind != "declaration"
            || name != signature.owner.path.join(".")
        {
            return Err(fail(
                "spelling does not target its actual scoped datatype declaration",
            ));
        }
    } else if signature.owner.kind != "primitive"
        || source.len() != 1
        || signature.owner.path.len() != 1
        || source[0] != signature.owner.path[0]
    {
        return Err(fail("spelling has no actual primitive type binding"));
    }
    Ok(())
}

/// Recheck each selected spelling against the current source-free declarations.
pub fn validate_source_type_namespaces(program: &Program) -> Result<()> {
    if !program.declarations.iter().any(|entry| {
        matches!(entry, Decl::Fn(function) if function.source_type_namespace.is_some())
            || matches!(entry, Decl::Data(data) if data.source_unit_clone_derivation.is_some())
    }) {
        return Ok(());
    }
    super::source_unit_clone_derivations::validate_source_unit_clone_derivations(program)?;
    let raw = serde_json::to_value(program)
        .map_err(|_| fail("has malformed or missing selected facts"))?;
    validate_type_namespace_records(&raw)?;
    let catalog = TypeNamespaceCatalog::new(program)?;
    for declaration in &program.declarations {
        let Decl::Fn(function) = declaration else {
            continue;
        };
        let proof = function
            .source_type_namespace
            .as_ref()
            .ok_or_else(|| fail("has malformed or missing selected facts"))?;
        let signature = function.source_signature.as_ref().ok_or_else(|| {
            fail("must retain its owned source signature and ordered parameter positions")
        })?;
        validate_source_signature(
            signature,
            &function
                .params
                .iter()
                .map(|parameter| parameter.ty.clone())
                .collect::<Vec<_>>(),
            &function.ret,
            program.source_language.as_str(),
        )?;
        for (source, owned) in proof["parameters"]
            .as_array()
            .unwrap()
            .iter()
            .zip(&signature.parameters)
        {
            position(source, owned, &function.module_path, false, &catalog)?;
        }
        position(
            &proof["result"],
            &signature.result,
            &function.module_path,
            true,
            &catalog,
        )?;
    }
    Ok(())
}
