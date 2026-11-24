//! Check a closed call's actual place selection, writeback tuple and rebuilding.
use super::{
    DataDecl, Expr, FnDecl, HashSet, Node, Reconstruction, ReplacementValue, Result,
    RustSourcePlaceTransform, SourcePlaceSlot, Type, box_projection_depth, check_position,
    check_slot, fields_for, plain, type_error, variable,
};

pub(super) fn check_call<'data, 'function>(
    facet: &RustSourcePlaceTransform,
    node: &Expr,
    language: &str,
    mut resolve_type: impl FnMut(&Type) -> Result<Type>,
    mut resolve_data: impl FnMut(&Type) -> Option<&'data DataDecl>,
    mut resolve_function: impl FnMut(&str) -> Option<&'function FnDecl>,
) -> Result<RustSourcePlaceTransform> {
    let fail = || {
        type_error(
            "a projected Vec call retains its actual complete call, selected source place and writeback",
            node.span,
        )
    };
    let RustSourcePlaceTransform::Call {
        slots,
        places,
        span,
    } = facet
    else {
        return Err(fail());
    };
    let ([root], [place]) = (slots.as_slice(), places.as_slice()) else {
        return Err(fail());
    };
    let Node::Let {
        name: packet,
        value: call,
        body,
    } = &node.node
    else {
        return Err(fail());
    };
    let Node::Call { func, args } = &call.node else {
        return Err(fail());
    };
    let Node::Let {
        name: rebuilt,
        value: state,
        body: output,
    } = &body.node
    else {
        return Err(fail());
    };
    let Node::Tuple { items, .. } = &output.node else {
        return Err(fail());
    };
    let [result, state_output] = items.as_slice() else {
        return Err(fail());
    };
    let function = resolve_function(func).ok_or_else(fail)?;
    let helper = function.source_vector_mutation.as_ref().ok_or_else(fail)?;
    let parameter = helper.parameters.first().ok_or_else(fail)?;
    let (receiver, operands) = args.split_first().ok_or_else(fail)?;
    let names = HashSet::from([root.input.as_str(), packet.as_str(), rebuilt.as_str()]);
    if language != "Rust"
        || span.start >= span.end
        || span.end as u64 > 9_007_199_254_740_991
        || !node
            .span
            .is_some_and(|actual| actual.start == span.start && actual.end == span.end)
        || node.source_position.is_some()
        || node.source_state_transition.is_some()
        || node.source_loop_fold.is_some()
        || node.source_record_place.is_some()
        || root.input.is_empty()
        || root.input.len() > 4096
        || names.len() != 3
        || place.parameter != 0
        || place.slot != 0
        || place.fields.len() > 128
        || args.len() != helper.parameters.len()
        || operands
            .iter()
            .zip(&helper.parameters[1..])
            .any(|(argument, expected)| {
                !plain(argument) || argument.var_name().is_none() || argument.ty != expected.ty
            })
        || !plain(call)
        || call.ty != function.ret
        || !plain(body)
        || !plain(output)
        || node.ty != body.ty
        || node.ty != output.ty
        || node.ty
            != (Type::Tuple {
                elements: vec![helper.result.ty.clone(), state.ty.clone()],
            })
        || !(ReplacementValue::PacketField {
            name: packet,
            packet_type: &call.ty,
            index: 0,
            field_type: &helper.result.ty,
        })
        .matches(result)
        || !variable(state_output, rebuilt, &state.ty)
    {
        return Err(fail());
    }
    let position = check_slot(
        &root.binder,
        &root.position,
        &state.ty,
        &mut resolve_type,
        &mut resolve_data,
    )?;
    let mut selected = &position;
    let mut work = 0;
    for index in &place.fields {
        let (_, fields) = fields_for(selected, &mut resolve_data).ok_or_else(fail)?;
        work += 1 + fields.len();
        if work > 100_000 {
            return Err(fail());
        }
        selected = fields.get(*index).ok_or_else(fail)?;
    }
    let depth = box_projection_depth(selected, parameter).ok_or_else(fail)?;
    if selected.ty != parameter.ty || (depth == 0 && place.fields.is_empty()) {
        return Err(fail());
    }
    if depth > 0 {
        let view = receiver.source_position.as_ref().ok_or_else(fail)?;
        let qualified = check_position(
            &view.position,
            &receiver.ty,
            language,
            &mut resolve_type,
            &mut resolve_data,
        )?;
        if qualified.ty != parameter.ty || box_projection_depth(&qualified, parameter) != Some(0) {
            return Err(fail());
        }
    }
    let mut reconstruction = Reconstruction {
        resolve_data: &mut resolve_data,
        replacement: ReplacementValue::PacketField {
            name: packet,
            packet_type: &call.ty,
            index: 1,
            field_type: &parameter.ty,
        },
        seen: names,
        work,
        lifetime: std::marker::PhantomData,
    };
    reconstruction
        .projection(receiver, &position, &place.fields, &root.input, depth > 0)
        .ok_or_else(fail)?;
    reconstruction.field(state, &position, &place.fields, &root.input)?;

    Ok(RustSourcePlaceTransform::Call {
        slots: vec![SourcePlaceSlot {
            position,
            ..root.clone()
        }],
        places: places.clone(),
        span: span.clone(),
    })
}
