//! Exact source positions and checked body of a generated standard Vec helper.
use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use super::{
    Span,
    diagnostics::{Result, type_error},
    ir::{DataDecl, Expr, FnDecl, LitValue, Node, Param, Pattern},
    rust_exclusive_source_vector_rules as rules,
    source_expression_positions::check_position,
    source_loop_folds::SourceFoldSpan,
    source_signatures::SourceTypeSignature,
    surface::{BinaryOp, SFn},
    types::Type,
};

pub(super) const MODEL: &str = "Rust vector source mutation helper 1";

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RustVectorMutation {
    pub model: String,
    pub operation: String,
    pub parameters: Vec<SourceTypeSignature>,
    pub element: SourceTypeSignature,
    pub result: SourceTypeSignature,
    pub span: SourceFoldSpan,
}

pub(super) fn check_surface<'data>(
    facet: &RustVectorMutation,
    function: &SFn,
    params: &[Param],
    ret: &Type,
    language: &str,
    resolve_type: impl FnMut(&Type) -> Result<Type>,
    resolve_data: impl FnMut(&Type) -> Option<&'data DataDecl>,
) -> Result<RustVectorMutation> {
    check_interface(
        facet,
        params,
        ret,
        function.span,
        function.generated
            && function.source_signature.is_none()
            && function.source_state_transform.is_none(),
        language,
        resolve_type,
        resolve_data,
    )
}

pub(crate) fn validate_checked<'data>(
    facet: &RustVectorMutation,
    function: &FnDecl,
    language: &str,
    resolve_data: impl FnMut(&Type) -> Option<&'data DataDecl>,
) -> Result<()> {
    let checked = check_interface(
        facet,
        &function.params,
        &function.ret,
        function.span,
        function.generated
            && function.source_signature.is_none()
            && function.source_state_transform.is_none(),
        language,
        |ty| Ok(ty.clone()),
        resolve_data,
    )?;
    if &checked != facet {
        return Err(type_error(
            "the linked Vec helper retains its exact checked source positions",
            function.span,
        ));
    }
    check_body(function, facet)
}

fn direct(position: &SourceTypeSignature) -> bool {
    position.wrappers.is_empty()
        && position.lifetimes.is_empty()
        && position.array_element_position.is_none()
}

#[allow(clippy::too_many_arguments)]
fn check_interface<'data>(
    facet: &RustVectorMutation,
    params: &[Param],
    ret: &Type,
    span: Option<Span>,
    generated: bool,
    language: &str,
    mut resolve_type: impl FnMut(&Type) -> Result<Type>,
    mut resolve_data: impl FnMut(&Type) -> Option<&'data DataDecl>,
) -> Result<RustVectorMutation> {
    let fail = |reason: &str| type_error(reason, span);
    let Type::Tuple { elements } = ret else {
        return Err(fail(
            "a Vec mutation helper retains its exact result and vector state packet",
        ));
    };
    let arity = rules::read_rust_exclusive_source_vector_method_arity(&facet.operation);
    if language != "Rust"
        || facet.model != MODEL
        || arity < 0.0
        || !generated
        || params.len() != 1 + arity as usize
        || params.len() != facet.parameters.len()
        || params
            .iter()
            .map(|param| &param.name)
            .collect::<HashSet<_>>()
            .len()
            != params.len()
        || elements.len() != 2
        || facet.span.end <= facet.span.start
        || !span.is_some_and(|site| site.start == facet.span.start && site.end == facet.span.end)
    {
        return Err(fail(
            "a Vec mutation helper retains its exact generated operation, span and complete interface",
        ));
    }
    let mut position = |value: &SourceTypeSignature, ty: &Type| {
        check_position(value, ty, language, &mut resolve_type, &mut resolve_data)
    };
    let parameters = facet
        .parameters
        .iter()
        .zip(params)
        .map(|(value, param)| position(value, &param.ty))
        .collect::<Result<Vec<_>>>()?;
    let vector = &parameters[0];
    let Type::Array {
        element: element_type,
        reference: false,
    } = &vector.ty
    else {
        return Err(fail(
            "a Vec mutation helper retains its actual array parameter",
        ));
    };
    let element = position(&facet.element, element_type)?;
    let result = position(&facet.result, &elements[0])?;
    if !direct(vector)
        || vector.owner.kind != "library"
        || vector.owner.path != ["std", "vec", "Vec"]
        || !matches!(vector.owner.arguments.as_slice(), [argument] if argument == &element.owner)
        || vector.ty != elements[1]
        || !direct(&element)
        || !direct(&result)
    {
        return Err(fail(
            "a Vec mutation helper retains the actual standard vector and complete closed element and result",
        ));
    }
    if facet.operation == "pop" {
        let data = resolve_data(&result.ty)
            .ok_or_else(|| fail("Vec pop requires its actual standard Option declaration"))?;
        let none = data.ctors.iter().find(|ctor| ctor.name == "None");
        let some = data.ctors.iter().find(|ctor| ctor.name == "Some");
        if result.owner.kind != "library"
            || result.owner.path != ["std", "option", "Option"]
            || !matches!(result.owner.arguments.as_slice(), [argument] if argument == &element.owner)
            || data.ctors.len() != 2
            || none.is_none_or(|ctor| !ctor.fields.is_empty())
            || some.is_none_or(|ctor| ctor.fields.len() != 1 || ctor.fields[0].ty != element.ty)
        {
            return Err(fail(
                "Vec pop retains its actual complete standard Option element result",
            ));
        }
    } else if result.ty != Type::Unit || result.owner.kind != "unit" {
        return Err(fail(
            "Vec clear and truncate retain their actual Unit completion",
        ));
    }
    if let Some(bound) = parameters.get(1)
        && (!direct(bound)
            || bound.ty
                != (Type::Fixed {
                    signed: false,
                    bits: 64,
                })
            || bound.owner.kind != "primitive"
            || bound.owner.path != ["usize"]
            || !bound.owner.arguments.is_empty())
    {
        return Err(fail(
            "Vec truncate retains its direct usize bound for the supported 64-bit source target",
        ));
    }
    Ok(RustVectorMutation {
        parameters,
        element,
        result,
        ..facet.clone()
    })
}

fn variable(node: &Expr, name: &str) -> bool {
    matches!(&node.node, Node::Var { name: actual } if actual == name)
}

fn projection(node: &Expr, name: &str, expected: usize) -> bool {
    matches!(&node.node, Node::TupleIndex { object, index } if *index == expected && variable(object, name))
}

fn pair(node: &Expr) -> Option<(&Expr, &Expr)> {
    let Node::Tuple { items, .. } = &node.node else {
        return None;
    };
    let [first, second] = items.as_slice() else {
        return None;
    };
    Some((first, second))
}

fn empty(node: &Expr, ty: &Type) -> bool {
    &node.ty == ty && matches!(&node.node, Node::Array { items } if items.is_empty())
}

fn appended(node: &Expr, accumulator: &str, item: &str) -> bool {
    matches!(&node.node, Node::Append { left, right }
        if projection(left, accumulator, 1)
        && matches!(&right.node, Node::Array { items } if matches!(items.as_slice(), [value] if variable(value, item))))
}

fn option(node: &Expr, data_name: &str, label: &str, payload: Option<&str>) -> bool {
    matches!(&node.node, Node::Ctor { data, ctor, args, .. } if data == data_name && ctor == label
    && match (args.as_slice(), payload) {
        ([], None) => true,
        ([value], Some(name)) => variable(value, name),
        _ => false,
    })
}

fn fixed_count(
    node: &Expr,
    operation: BinaryOp,
    accumulator: &str,
    literal: &str,
    ty: &Type,
) -> bool {
    matches!(&node.node, Node::Binary { op, left, right, .. } if *op == operation
        && projection(left, accumulator, 0) && &right.ty == ty
        && matches!(&right.node, Node::Lit { value: LitValue::Text(value) } if value == literal))
}

pub(super) fn check_body(function: &FnDecl, facet: &RustVectorMutation) -> Result<()> {
    let mut pending = vec![&function.body];
    let mut count = 0;
    while let Some(node) = pending.pop() {
        count += 1;
        if count > 100_000
            || node.source_position.is_some()
            || node.source_state_transition.is_some()
            || node.source_loop_fold.is_some()
            || node.source_record_place.is_some()
            || node.source_place_transform.is_some()
        {
            return Err(type_error(
                "a Vec helper uses bounded pure nodes without competing source qualifiers",
                function.span,
            ));
        }
        pending.extend(node.children());
    }
    verify_body(function, facet).ok_or_else(|| type_error(
        "the Vec mutation helper retains its exact complete constructor, fold arguments and branch behavior", function.span))
}

fn verify_body(function: &FnDecl, facet: &RustVectorMutation) -> Option<()> {
    let vector = &facet.parameters.first()?.ty;
    if facet.operation == "clear" {
        let (result, state) = pair(&function.body)?;
        return (matches!(result.node, Node::Unit) && empty(state, vector)).then_some(());
    }
    let (fold, binding) = if facet.operation == "truncate" {
        let Node::Let { name, value, body } = &function.body.node else {
            return None;
        };
        let (result, state) = pair(body)?;
        if !matches!(result.node, Node::Unit) || !projection(state, name, 1) {
            return None;
        }
        (value.as_ref(), Some(name.as_str()))
    } else {
        (&function.body, None)
    };
    let Node::Method { object, op, args } = &fold.node else {
        return None;
    };
    let [Some(callback), Some(initial)] = args.as_slice() else {
        return None;
    };
    let Node::Lambda { params, body } = &callback.node else {
        return None;
    };
    let [accumulator, item] = params.as_slice() else {
        return None;
    };
    let names = function
        .params
        .iter()
        .map(|param| param.name.as_str())
        .chain([accumulator.name.as_str(), item.name.as_str()])
        .chain(binding)
        .collect::<Vec<_>>();
    if op != "reduce"
        || !variable(object, &function.params.first()?.name)
        || names.iter().copied().collect::<HashSet<_>>().len() != names.len()
        || accumulator.ty != fold.ty
        || item.ty != facet.element.ty
        || initial.ty != fold.ty
        || body.ty != fold.ty
    {
        return None;
    }
    let (initial_value, initial_vector) = pair(initial)?;
    if !empty(initial_vector, vector) {
        return None;
    }
    let accumulator = accumulator.name.as_str();
    let item = item.name.as_str();
    if facet.operation == "pop" {
        let Type::Data { name } = &facet.result.ty else {
            return None;
        };
        let Node::Match { scrutinee, cases } = &body.node else {
            return None;
        };
        if fold.ty != function.ret
            || !option(initial_value, name, "None", None)
            || cases.len() != 2
            || !projection(scrutinee, accumulator, 0)
        {
            return None;
        }
        let none = cases.iter().find(|case| {
            matches!(&case.pattern, Pattern::Ctor { data, ctor, binds }
            if data == name && ctor == "None" && binds.is_empty())
        })?;
        let some = cases.iter().find(|case| {
            matches!(&case.pattern, Pattern::Ctor { data, ctor, .. }
            if data == name && ctor == "Some")
        })?;
        let Pattern::Ctor { binds, .. } = &some.pattern else {
            return None;
        };
        let [Some(prior)] = binds.as_slice() else {
            return None;
        };
        let (none_value, none_state) = pair(&none.body)?;
        let (some_value, some_state) = pair(&some.body)?;
        return (!names.contains(&prior.as_str())
            && option(none_value, name, "Some", Some(item))
            && projection(none_state, accumulator, 1)
            && option(some_value, name, "Some", Some(item))
            && appended(some_state, accumulator, prior))
        .then_some(());
    }
    let bound = &facet.parameters.get(1)?.ty;
    let Type::Tuple { elements } = &fold.ty else {
        return None;
    };
    let [count, state] = elements.as_slice() else {
        return None;
    };
    let Node::If {
        cond,
        then,
        otherwise,
    } = &body.node
    else {
        return None;
    };
    let (remaining, kept) = pair(then)?;
    (count == bound
        && state == vector
        && variable(initial_value, &function.params.get(1)?.name)
        && fixed_count(cond, BinaryOp::Gt, accumulator, "0", bound)
        && variable(otherwise, accumulator)
        && fixed_count(remaining, BinaryOp::Sub, accumulator, "1", bound)
        && appended(kept, accumulator, item))
    .then_some(())
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::MODEL;
    use crate::{
        translation::{check::check_program, surface::SProgram},
        typed_program_links::{decode_typed_program_links, encode_typed_program_links},
    };

    #[test]
    fn clear_keeps_the_same_checked_facet_through_links() {
        let vector =
            json!({"kind": "array", "element": {"kind": "fixed", "bits": 32, "signed": false}});
        let unit = json!({"kind": "unit"});
        let span = json!({"start": 3, "end": 13});
        let element = json!({"kind": "primitive", "path": ["u32"], "arguments": []});
        let position =
            |ty, owner| json!({"type": ty, "owner": owner, "wrappers": [], "lifetimes": []});
        let helper = json!({
            "k": "fn", "name": "clear_vector", "generated": true, "span": span,
            "params": [{"name": "input", "type": vector}],
            "ret": {"kind": "tuple", "elements": [unit, vector]},
            "body": {"k": "tuple", "items": [{"k": "unit"}, {"k": "array", "items": []}]},
            "sourceVectorMutation": {
                "model": MODEL, "operation": "clear", "span": span,
                "parameters": [position(vector.clone(), json!({
                    "kind": "library", "path": ["std", "vec", "Vec"], "arguments": [element]
                }))],
                "element": position(vector["element"].clone(), element),
                "result": position(unit, json!({"kind": "unit", "path": [], "arguments": []}))
            }
        });
        let surface = |helper| {
            serde_json::from_value::<SProgram>(json!({"language": "Rust", "items": [helper]}))
                .expect("complete generated Vec helper surface")
        };
        let checked = check_program(&surface(helper.clone())).expect("actual clear");
        let links = encode_typed_program_links(&checked).expect("complete Vec facet");
        let replay =
            decode_typed_program_links(&links.network, links.root).expect("rechecked Vec facet");
        assert_eq!(replay, checked);

        let mut changed = helper;
        changed["body"]["items"][1] = json!({"k": "name", "path": ["input"]});
        assert!(check_program(&surface(changed)).is_err());
    }
}
