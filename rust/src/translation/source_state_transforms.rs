//! Checked interface of an automatically lowered finite Rust loop chunk.
use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use super::{
    Span,
    diagnostics::{Result, type_error},
    ir::{DataDecl, FnDecl, Param},
    rust_owned_record_loop_rules as rules,
    source_expression_positions::check_position,
    source_loop_folds::{RustSourceFoldSlot, SourceFoldSpan, check_slot},
    source_signatures::SourceTypeSignature,
    surface::SFn,
    types::Type,
};

pub(super) const MODEL: &str = "Rust finite loop state transformation 1";

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RustFiniteLoopStateTransform {
    pub model: String,
    pub slots: Vec<RustSourceFoldSlot>,
    #[serde(deserialize_with = "read_control")]
    pub control: Option<Type>,
    pub result: SourceTypeSignature,
    pub span: SourceFoldSpan,
}

fn read_control<'de, D: serde::Deserializer<'de>>(
    deserializer: D,
) -> std::result::Result<Option<Type>, D::Error> {
    Option::deserialize(deserializer)
}

struct StateFunction<'a> {
    name: &'a str,
    span: Option<Span>,
    generated: bool,
    source_signature: bool,
}

pub(super) fn check_state_transform<'data>(
    facet: &RustFiniteLoopStateTransform,
    function: &SFn,
    params: &[Param],
    ret: &Type,
    language: &str,
    resolve_type: impl FnMut(&Type) -> Result<Type>,
    resolve_data: impl FnMut(&Type) -> Option<&'data DataDecl>,
) -> Result<RustFiniteLoopStateTransform> {
    check_transform(
        facet,
        StateFunction {
            name: &function.name,
            span: function.span,
            generated: function.generated,
            source_signature: function.source_signature.is_some(),
        },
        params,
        ret,
        language,
        resolve_type,
        resolve_data,
    )
}

pub(crate) fn validate_checked_state_transform<'data>(
    facet: &RustFiniteLoopStateTransform,
    function: &FnDecl,
    language: &str,
    resolve_data: impl FnMut(&Type) -> Option<&'data DataDecl>,
) -> Result<()> {
    let checked = check_transform(
        facet,
        StateFunction {
            name: &function.name,
            span: function.span,
            generated: function.generated,
            source_signature: function.source_signature.is_some(),
        },
        &function.params,
        &function.ret,
        language,
        |ty| Ok(ty.clone()),
        resolve_data,
    )?;
    if &checked != facet {
        return Err(type_error(
            "the linked state transformation retains its actual fully checked positions",
            function.span,
        ));
    }
    Ok(())
}

fn check_transform<'data>(
    facet: &RustFiniteLoopStateTransform,
    function: StateFunction<'_>,
    params: &[Param],
    ret: &Type,
    language: &str,
    mut resolve_type: impl FnMut(&Type) -> Result<Type>,
    mut resolve_data: impl FnMut(&Type) -> Option<&'data DataDecl>,
) -> Result<RustFiniteLoopStateTransform> {
    let fail = |reason: &str| type_error(reason, function.span);
    let Type::Tuple { elements } = ret else {
        return Err(fail(
            "a compiler state function retains its actual result and state tuple",
        ));
    };
    if !rules::accept_rust_finite_loop_state_transform(
        language,
        &facet.model,
        function.name,
        function.generated,
        facet.slots.len() as f64,
        facet.span.start as f64,
        facet.span.end as f64,
    ) || function.source_signature
        || !function
            .span
            .is_some_and(|span| span.start == facet.span.start && span.end == facet.span.end)
        || elements.len() != facet.slots.len() + 1
        || params.len() != facet.slots.len()
    {
        return Err(fail(
            "a finite state transformation retains its actual generated function, bounded state slots and original loop span",
        ));
    }

    let mut identities = HashSet::new();
    let mut slots = Vec::with_capacity(params.len());
    for ((slot, param), output) in facet.slots.iter().zip(params).zip(&elements[1..]) {
        if &param.ty != output || !identities.insert(slot.binder.identity.as_str()) {
            return Err(fail(
                "a compiler state function retains the actual distinct ordered input and output state slots",
            ));
        }
        slots.push(RustSourceFoldSlot {
            binder: slot.binder.clone(),
            position: check_slot(
                &slot.binder,
                &slot.position,
                &param.ty,
                &mut resolve_type,
                &mut resolve_data,
            )?,
        });
    }

    let control_error =
        || fail("a compiler state completion retains its actual complete declaration and payload");
    if facet
        .control
        .as_ref()
        .is_some_and(|ty| !matches!(ty, Type::Named { .. } | Type::Data { .. }))
    {
        return Err(control_error());
    }
    let control = facet.control.as_ref().map(&mut resolve_type).transpose()?;
    let expected = match control.as_ref() {
        Some(control @ Type::Data { .. }) if control == &elements[0] => {
            let data = resolve_data(control).ok_or_else(control_error)?;
            let variants = rules::read_rust_owned_vector_source_fold_control_variants();
            if data.ctors.len() != variants.len()
                || data.ctors.iter().zip(&variants).enumerate().any(
                    |(index, (constructor, name))| {
                        &constructor.name != name
                            || constructor.fields.len() != usize::from(index == 0)
                    },
                )
            {
                return Err(control_error());
            }
            data.ctors[0].fields[0].ty.clone()
        }
        None if elements[0] == Type::Unit => Type::Unit,
        _ => return Err(control_error()),
    };
    let result = check_position(
        &facet.result,
        &expected,
        language,
        &mut resolve_type,
        &mut resolve_data,
    )?;
    if control.is_none()
        && (!result.wrappers.is_empty()
            || !result.lifetimes.is_empty()
            || result.owner.kind != "unit")
    {
        return Err(control_error());
    }
    Ok(RustFiniteLoopStateTransform {
        model: facet.model.clone(),
        slots,
        control,
        result,
        span: facet.span.clone(),
    })
}
