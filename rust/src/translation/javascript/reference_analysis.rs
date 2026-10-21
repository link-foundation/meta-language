//! Source and surface transport for the checked JavaScript reference rules.

use serde::Serialize;
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::collections::HashMap;
use std::sync::OnceLock;

use super::{Result, SEffect, SExpr, SItem, SProgram, Span, Token, TokenKind, Type, type_error};
use crate::translation::surface_raw_metadata_rules::accept_shared_surface_reference_kind;
use crate::translation::surface_reference_analysis_rules::{
    accept_surface_reference_closure_token, accept_surface_reference_mutating_method,
    has_surface_structural_identity, require_surface_reference_token_after_elimination,
};
use crate::translation::surface_reference_facts::create_surface_reference_facts;

fn mutating_methods() -> &'static [String] {
    static METHODS: OnceLock<Vec<String>> = OnceLock::new();
    METHODS.get_or_init(|| {
        let policy: Value = crate::read_structured_data_lino(include_str!(
            "../../data/surface-reference-analysis.lino"
        ))
        .expect("the generated surface reference policy is valid project data");
        policy["mutatingMethods"]
            .as_array()
            .expect("the linked method aliases are an ordered array")
            .iter()
            .map(|method| {
                method
                    .as_str()
                    .expect("method aliases are strings")
                    .to_owned()
            })
            .collect()
    })
}

fn kind(token: Option<&Token>) -> &'static str {
    match token.map(|token| token.kind) {
        Some(TokenKind::Punct) => "punct",
        Some(TokenKind::Identifier) => "identifier",
        Some(TokenKind::Number) => "number",
        Some(TokenKind::String) => "string",
        Some(TokenKind::Character) => "char",
        Some(TokenKind::ByteCharacter) => "byteChar",
        Some(TokenKind::Eof) => "eof",
        Some(TokenKind::Macro) => "macro",
        Some(TokenKind::RegularExpression) => "regularExpression",
        Some(TokenKind::Interpolation) => "interpolation",
        Some(TokenKind::Template) => "template",
        None => "",
    }
}

fn text(token: Option<&Token>) -> &str {
    token.map_or("", |token| token.value.as_str())
}

/// Supply the exact eight token-window observations to generated decisions.
pub(super) fn requires_reference_semantics(
    tokens: &[Token],
    eliminated_operations: &[Span],
) -> bool {
    let closure = tokens.iter().enumerate().any(|(position, token)| {
        accept_surface_reference_closure_token(
            kind(Some(token)),
            &token.value,
            text(tokens.get(position + 1)),
        )
    });
    tokens.iter().enumerate().any(|(position, token)| {
        let previous = position.checked_sub(1).and_then(|index| tokens.get(index));
        let before_previous = position.checked_sub(2).and_then(|index| tokens.get(index));
        let following = tokens.get(position + 1);
        let eliminated_method = eliminated_operations
            .iter()
            .any(|operation| operation.start <= token.start && token.end <= operation.end);
        let window = [
            kind(Some(token)),
            &token.value,
            kind(previous),
            text(previous),
            text(before_previous),
            kind(following),
            text(following),
            text(tokens.get(position + 2)),
        ]
        .map(str::to_owned);
        require_surface_reference_token_after_elimination(
            &window,
            closure,
            !eliminated_method
                && accept_surface_reference_mutating_method(&token.value, mutating_methods()),
            eliminated_method,
        )
    })
}

/// Inspect the serialized native surface with the same recursive rule kernel.
pub(super) fn has_structural_identity(program: &SProgram) -> Result<bool> {
    let value = serde_json::to_value(program)
        .map_err(|error| type_error(format!("surface fact serialization: {error}"), None))?;
    let facts = create_surface_reference_facts(&value);
    Ok(has_surface_structural_identity(
        &facts.columns,
        &facts.rows,
        facts.size,
        facts.root,
    ))
}

/// A newly selected late identity model marks the actual inferred types through
/// the same shared metadata rule used by early selection.
pub(super) fn select_structural_reference_semantics(program: &mut SProgram) -> Result<()> {
    if crate::translation::surface_raw_metadata_rules::select_surface_reference_semantics(
        false,
        has_structural_identity(program)?,
        true,
    ) {
        let selected = !program
            .reference_semantics
            .as_ref()
            .and_then(super::super::surface::SurfaceRawMetadata::as_boolean)
            .unwrap_or(false);
        program.reference_semantics = Some(true.into());
        if selected {
            mark_shared_reference_types(program, &mut HashMap::new())?;
        }
    }
    Ok(())
}

fn mark_value(value: &mut Value) {
    match value {
        Value::Array(values) => values.iter_mut().for_each(mark_value),
        Value::Object(fields) => {
            fields.values_mut().for_each(mark_value);
            if accept_shared_surface_reference_kind(
                fields
                    .get("kind")
                    .and_then(Value::as_str)
                    .unwrap_or_default(),
            ) {
                fields.insert("reference".to_owned(), Value::Bool(true));
            }
        }
        _ => {}
    }
}

fn mark_types<T: Serialize + DeserializeOwned>(value: &mut T) -> Result<()> {
    let mut data = serde_json::to_value(&*value)
        .map_err(|error| type_error(format!("reference metadata serialization: {error}"), None))?;
    mark_value(&mut data);
    *value = serde_json::from_value(data)
        .map_err(|error| type_error(format!("reference metadata transport: {error}"), None))?;
    Ok(())
}

fn expression_positions(program: &SProgram) -> Vec<*const SExpr> {
    fn expression(value: &SExpr, into: &mut Vec<*const SExpr>) {
        into.push(std::ptr::from_ref(value));
        for child in super::products::children(value) {
            expression(child, into);
        }
    }
    fn items(values: &[SItem], into: &mut Vec<*const SExpr>) {
        for value in values {
            match value {
                SItem::Fn(function) => {
                    for parameter in &function.params {
                        if let Some(value) = &parameter.default_value {
                            expression(value, into);
                        }
                    }
                    expression(&function.body, into);
                }
                SItem::Module(module) => items(&module.items, into),
                SItem::Data(_) | SItem::Theorem(_) => {}
            }
        }
    }
    let mut positions = Vec::new();
    items(&program.items, &mut positions);
    for effect in program.main.iter().flat_map(|main| &main.effects) {
        match effect {
            SEffect::Let { value, .. } | SEffect::Print { expr: value, .. } => {
                expression(value, &mut positions);
            }
            SEffect::Assert { prop, .. } => {
                for value in super::products::prop_expressions(prop) {
                    expression(value, &mut positions);
                }
            }
        }
    }
    positions
}

/// Preserve inferred expression identity when the generic metadata copy moves
/// owned AST nodes. Only reference fields change; the expression order remains.
pub(super) fn mark_shared_reference_types(
    program: &mut SProgram,
    observed: &mut HashMap<*const SExpr, Type>,
) -> Result<()> {
    if !program
        .reference_semantics
        .as_ref()
        .and_then(super::super::surface::SurfaceRawMetadata::as_boolean)
        .unwrap_or(false)
    {
        return Ok(());
    }
    let before = expression_positions(program);
    mark_types(program)?;
    let after = expression_positions(program);
    if before.len() != after.len() {
        return Err(type_error(
            "reference metadata transport changed expression positions",
            None,
        ));
    }
    let positions: HashMap<_, _> = before.into_iter().zip(after).collect();
    let mut remapped = HashMap::with_capacity(observed.len());
    for (node, mut ty) in std::mem::take(observed) {
        if let Some(position) = positions.get(&node) {
            mark_types(&mut ty)?;
            remapped.insert(*position, ty);
        }
    }
    *observed = remapped;
    Ok(())
}
