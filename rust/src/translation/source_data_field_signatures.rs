//! Owned datatype-field positions checked by the existing source type model.
use super::diagnostics::{Result, type_error};
use super::ir::Ctor;
use super::rust_data_type_field_signature_rules::{
    accept_rust_constructor_field_signature, accept_rust_data_type_field_signature,
    accept_rust_data_type_field_signature_budget,
};
use super::source_signatures::{
    SourceOwner, SourceSignature, SourceTypeSignature, validate_source_signature,
};
use super::types::Type;
use serde::{Deserialize, Deserializer, Serialize};

/// Actual source type positions owned by one actual datatype constructor.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RustConstructorFieldSignatures {
    /// Actual portable constructor identity within its datatype.
    pub name: String,
    /// Complete source positions in declaration order.
    pub fields: Vec<SourceTypeSignature>,
}
/// Complete optional Rust datatype field-type interpretation.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RustDataTypeFieldSignatures {
    /// Selected field interpretation, independent of value type erasure.
    pub model: String,
    /// Original source language.
    pub language: String,
    /// Actual fully qualified source datatype identity.
    pub data_type_name: String,
    /// Every constructor and every original source field position.
    pub ctors: Vec<RustConstructorFieldSignatures>,
}
/// Absence is optional; a represented null is not an omitted source qualifier.
pub(crate) fn read_present<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> std::result::Result<Option<Box<RustDataTypeFieldSignatures>>, D::Error> {
    // Type's general serde adapter tolerates unknown keys. A represented source
    // position cannot silently discard them before the actual type check.
    let original = serde_json::Value::deserialize(deserializer)?;
    let mut visits = 0;
    inspect(&original, &mut visits, 0).map_err(<D::Error as serde::de::Error>::custom)?;
    let value: RustDataTypeFieldSignatures =
        serde_json::from_value(original.clone()).map_err(<D::Error as serde::de::Error>::custom)?;
    let normalized =
        serde_json::to_value(&value).map_err(<D::Error as serde::de::Error>::custom)?;
    represented_fields(&original, &normalized).map_err(<D::Error as serde::de::Error>::custom)?;
    Ok(Some(Box::new(value)))
}
// This is a serde transport check, not a second interpretation of Rust types.
fn represented_fields(original: &serde_json::Value, normalized: &serde_json::Value) -> Result<()> {
    match (original, normalized) {
        (serde_json::Value::Object(fields), serde_json::Value::Object(output)) => {
            for (field, value) in fields {
                if let Some(normalized) = output.get(field) {
                    represented_fields(value, normalized)?;
                } else if !(field == "span" && original["kind"] == "named" && value.is_null()) {
                    return Err(fail(
                        "an owned field-signature record has missing or unrepresented fields",
                    ));
                }
            }
        }
        (serde_json::Value::Array(values), serde_json::Value::Array(output)) => {
            if values.len() != output.len() {
                return Err(fail(
                    "an ordered field-signature container changed during transport",
                ));
            }
            for (value, normalized) in values.iter().zip(output) {
                represented_fields(value, normalized)?;
            }
        }
        _ => {}
    }
    Ok(())
}
fn fail(reason: &str) -> super::diagnostics::TranslationError {
    type_error(format!("Rust data type field signatures: {reason}"), None)
}
fn count(length: usize) -> f64 {
    u32::try_from(length).map_or(f64::INFINITY, f64::from)
}
fn inspect(value: &serde_json::Value, visits: &mut usize, depth: usize) -> Result<()> {
    *visits = visits.saturating_add(1);
    if !accept_rust_data_type_field_signature_budget(count(*visits), count(depth)) {
        return Err(fail(
            "the complete field-signature tree exceeds its resource limit",
        ));
    }
    match value {
        serde_json::Value::Array(values) => {
            for child in values {
                inspect(child, visits, depth + 1)?;
            }
        }
        serde_json::Value::Object(values) => {
            for child in values.values() {
                inspect(child, visits, depth + 1)?;
            }
        }
        _ => {}
    }
    Ok(())
}
/// Resolve declared positions, then check every actual constructor and field.
pub(crate) fn check(
    signature: &RustDataTypeFieldSignatures,
    constructors: &[Ctor],
    language: &str,
    data_type_name: &str,
    mut resolve_type: impl FnMut(&Type) -> Result<Type>,
) -> Result<RustDataTypeFieldSignatures> {
    let value = serde_json::to_value(signature)
        .map_err(|_| fail("an owned field-signature record has missing or unrepresented fields"))?;
    let mut visits = 0;
    inspect(&value, &mut visits, 0)?;
    if !accept_rust_data_type_field_signature(
        &signature.model,
        language,
        &signature.data_type_name,
        data_type_name,
        count(signature.ctors.len()),
        count(constructors.len()),
    ) || signature.language != language
    {
        return Err(fail(
            "the model, source language or actual datatype/constructor inventory differs",
        ));
    }
    let mut result = signature.clone();
    for (fields, actual) in result.ctors.iter_mut().zip(constructors) {
        if !accept_rust_constructor_field_signature(
            &fields.name,
            &actual.name,
            count(fields.fields.len()),
            count(actual.fields.len()),
        ) {
            return Err(fail(
                "the actual constructor name or complete field inventory differs",
            ));
        }
        for position in &mut fields.fields {
            position.ty = resolve_type(&position.ty)?;
        }
        let unit = SourceTypeSignature {
            ty: Type::Unit,
            wrappers: Vec::new(),
            lifetimes: Vec::new(),
            owner: SourceOwner {
                kind: "unit".into(),
                path: Vec::new(),
                arguments: Vec::new(),
            },
        };
        let function = SourceSignature {
            model: "Rust source function signature 2".into(),
            language: language.to_owned(),
            parameters: fields.fields.clone(),
            result: unit,
            lifetime_bindings: Vec::new(),
        };
        validate_source_signature(
            &function,
            &actual
                .fields
                .iter()
                .map(|field| field.ty.clone())
                .collect::<Vec<_>>(),
            &Type::Unit,
            language,
        )?;
    }
    Ok(result)
}
