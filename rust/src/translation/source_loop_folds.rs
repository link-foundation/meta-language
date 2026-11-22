//! Checked wire records for retained Rust places and ordered fold interfaces.
use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use super::diagnostics::{Result, type_error};
use super::ir::{DataDecl, Expr, LitValue, Node};
use super::rust_owned_record_loop_rules as rules;
use super::source_signatures::{
    SourceSignature, SourceTypeSignature, resolve_source_position, source_policy_column,
    validate_source_signature_with_data,
};
use super::types::Type;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
/// Identity and UTF-16 token coordinates of the original mutable declaration.
pub struct SourceStateBinder {
    pub name: String,
    pub identity: String,
    pub start: usize,
    pub end: usize,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
/// Source binding or assignment metadata on its lowered value expression.
pub struct RustSourceStateTransition {
    pub model: String,
    pub operation: String,
    pub binder: SourceStateBinder,
    pub position: SourceTypeSignature,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
/// The complete source position of a carried mutable binding.
pub struct RustSourceFoldSlot {
    pub binder: SourceStateBinder,
    pub position: SourceTypeSignature,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
/// UTF-16 coordinates of the complete original loop.
pub struct SourceFoldSpan {
    pub start: usize,
    pub end: usize,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
/// A source Vec loop's checked iterator, completion and carried state interface.
pub struct RustSourceLoopFold {
    pub model: String,
    pub input: SourceTypeSignature,
    pub item: SourceTypeSignature,
    pub result: SourceTypeSignature,
    pub control: Type,
    pub slots: Vec<RustSourceFoldSlot>,
    pub span: SourceFoldSpan,
}

fn fail(message: &str) -> super::diagnostics::TranslationError {
    type_error(message, None)
}

fn check_slot<'data>(
    binder: &SourceStateBinder,
    position: &SourceTypeSignature,
    expected: &Type,
    resolve_type: &mut impl FnMut(&Type) -> Result<Type>,
    resolve_data: &mut impl FnMut(&Type) -> Option<&'data DataDecl>,
) -> Result<SourceTypeSignature> {
    if !rules::accept_rust_source_state_binder(
        binder.name.encode_utf16().count() as f64,
        binder.identity.encode_utf16().count() as f64,
        binder.start as f64,
        binder.end as f64,
    ) {
        return Err(fail(
            "a source state position needs its actual source binder",
        ));
    }
    super::rust::measure_source_position_metadata(position).map_err(|error| fail(&error))?;
    let signature = SourceSignature {
        model: "Rust source function signature 2".into(),
        language: "Rust".into(),
        parameters: Vec::new(),
        result: position.clone(),
        lifetime_bindings: Vec::new(),
    };
    validate_source_signature_with_data(
        &signature,
        &[],
        expected,
        "Rust",
        &mut *resolve_type,
        resolve_data,
    )?;
    let mut position = signature.result;
    let mut owner = &position.owner;
    let is_box = |owner: &super::source_signatures::SourceOwner| {
        super::rust_owned_recursive_datatype_rules::accept_rust_owned_box_type_node(
            source_policy_column(&["box".into()]),
            &owner.kind,
            &owner.path.join("::"),
            owner.arguments.len() as f64,
        )
    };
    for wrapper in &position.wrappers {
        if wrapper != "box" || !is_box(owner) {
            return Err(fail(
                "a source state position needs every actual canonical Box layer",
            ));
        }
        owner = &owner.arguments[0];
    }
    if is_box(owner) || !position.lifetimes.is_empty() || position.array_element_position.is_some()
    {
        return Err(fail(
            "a source state position needs its actual closed owned referent",
        ));
    }
    resolve_source_position(&mut position, resolve_type)?;
    Ok(position)
}

pub(crate) fn check_state_transition<'data>(
    facet: &RustSourceStateTransition,
    expected: &Type,
    language: &str,
    mut resolve_type: impl FnMut(&Type) -> Result<Type>,
    mut resolve_data: impl FnMut(&Type) -> Option<&'data DataDecl>,
) -> Result<RustSourceStateTransition> {
    if !rules::accept_rust_source_state_transition_model(language, &facet.model, &facet.operation) {
        return Err(fail(
            "a source state transition retains its exact Rust model and operation",
        ));
    }
    let position = check_slot(
        &facet.binder,
        &facet.position,
        expected,
        &mut resolve_type,
        &mut resolve_data,
    )?;
    Ok(RustSourceStateTransition {
        model: facet.model.clone(),
        operation: facet.operation.clone(),
        binder: facet.binder.clone(),
        position,
    })
}

pub(crate) fn check_loop_fold<'data>(
    facet: &RustSourceLoopFold,
    node: &Expr,
    language: &str,
    mut resolve_type: impl FnMut(&Type) -> Result<Type>,
    mut resolve_data: impl FnMut(&Type) -> Option<&'data DataDecl>,
) -> Result<RustSourceLoopFold> {
    if !rules::accept_rust_owned_vector_source_fold_model(
        language,
        &facet.model,
        facet.slots.len() as f64,
        facet.span.start as f64,
        facet.span.end as f64,
    ) || !node
        .span
        .is_some_and(|span| span.start == facet.span.start && span.end == facet.span.end)
    {
        return Err(fail(
            "a source fold needs its exact bounded Rust interface and original source span",
        ));
    }
    if !matches!(&facet.control, Type::Named { .. } | Type::Data { .. }) {
        return Err(fail(
            "a source fold control needs its actual nominal type identity",
        ));
    }
    let packet_error = || {
        fail(
            "a source fold interface belongs to its actual two-parameter initialized reduce packet",
        )
    };
    let Node::Method { object, op, args } = &node.node else {
        return Err(packet_error());
    };
    let (
        Type::Tuple { elements },
        Type::Array {
            element,
            reference: false,
        },
        [Some(callback), Some(initial)],
    ) = (&node.ty, &object.ty, args.as_slice())
    else {
        return Err(packet_error());
    };
    let Node::Lambda { params, body } = &callback.node else {
        return Err(packet_error());
    };
    let [state, parameter] = params.as_slice() else {
        return Err(packet_error());
    };
    if op != "reduce" || elements.len() != facet.slots.len() + 2 || elements[0] != Type::Bool {
        return Err(packet_error());
    }

    let mut checked_position = |position: &SourceTypeSignature, expected: &Type| {
        // These fold values are checked in the parser's enclosing region scope.
        super::source_expression_positions::check_position(
            position,
            expected,
            language,
            &mut resolve_type,
            &mut resolve_data,
        )
    };
    let input = checked_position(&facet.input, &object.ty)?;
    let item = checked_position(&facet.item, element)?;
    let result = checked_position(&facet.result, &facet.result.ty)?;
    if !input.wrappers.is_empty()
        || !input.lifetimes.is_empty()
        || input.array_element_position.is_some()
        || input.owner.kind != "library"
        || input.owner.path != ["std", "vec", "Vec"]
        || input.owner.arguments.as_slice() != std::slice::from_ref(&item.owner)
        || !item.wrappers.is_empty()
        || !item.lifetimes.is_empty()
        || item.array_element_position.is_some()
    {
        return Err(fail(
            "an owned source fold retains its actual direct standard Vec and closed item positions",
        ));
    }
    let control = resolve_type(&facet.control)?;
    let Type::Data { name } = &control else {
        return Err(packet_error());
    };
    if control != elements[1]
        || state.ty != node.ty
        || parameter.ty != item.ty
        || !matches!(&callback.ty, Type::Function { ret, .. } if ret.as_ref() == &node.ty)
        || initial.ty != node.ty
    {
        return Err(packet_error());
    }
    let Node::If { cond, then, .. } = &body.node else {
        return Err(packet_error());
    };
    let Node::Tuple { items, .. } = &initial.node else {
        return Err(packet_error());
    };
    let [active, continuation, ..] = items.as_slice() else {
        return Err(packet_error());
    };
    if !matches!(&cond.node, Node::TupleIndex { object, index: 0 } if object.var_name() == Some(state.name.as_str()))
        || then.var_name() != Some(state.name.as_str())
        || !matches!(
            &active.node,
            Node::Lit {
                value: LitValue::Bool(false)
            }
        )
        || !matches!(&continuation.node, Node::Ctor { data, ctor, args, .. } if data == name && ctor == "StatementContinuation" && args.is_empty())
    {
        return Err(fail(
            "a source fold starts active and passes through its exact accumulator after completion",
        ));
    }
    let variants = rules::read_rust_owned_vector_source_fold_control_variants();
    let declaration = resolve_data(&control).ok_or_else(packet_error)?;
    if declaration.full_name != *name
        || declaration.ctors.len() != variants.len()
        || declaration
            .ctors
            .iter()
            .zip(&variants)
            .enumerate()
            .any(|(index, (ctor, variant))| {
                ctor.name != *variant || ctor.fields.len() != usize::from(index == 0)
            })
        || resolve_type(&declaration.ctors[0].fields[0].ty)? != result.ty
    {
        return Err(fail(
            "a source fold retains its actual complete compiler control layout and result type",
        ));
    }

    let mut identities = HashSet::new();
    let slots = facet
        .slots
        .iter()
        .zip(&elements[2..])
        .map(|(slot, ty)| {
            let position = check_slot(
                &slot.binder,
                &slot.position,
                ty,
                &mut resolve_type,
                &mut resolve_data,
            )?;
            if !identities.insert(&slot.binder.identity) {
                return Err(fail("source fold slots retain distinct actual binders"));
            }
            Ok(RustSourceFoldSlot {
                binder: slot.binder.clone(),
                position,
            })
        })
        .collect::<Result<Vec<_>>>()?;
    Ok(RustSourceLoopFold {
        model: facet.model.clone(),
        input,
        item,
        result,
        control,
        slots,
        span: facet.span.clone(),
    })
}
