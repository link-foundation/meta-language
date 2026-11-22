//! Check original source places against their actual extraction and rebuild terms.
use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use super::{
    diagnostics::{Result, type_error},
    ir::{DataDecl, Expr, LitValue, Node, Pattern},
    rust_exclusive_source_string_rules as string_rules,
    source_expression_positions::check_position,
    source_loop_folds::{SourceFoldSpan, SourceStateBinder, check_slot},
    source_record_places::{box_projection_depth, owner_view},
    source_signatures::SourceTypeSignature,
    surface::BinaryOp,
    types::Type,
};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "model", deny_unknown_fields)]
pub enum RustSourcePlaceTransform {
    #[serde(rename = "Rust source place replacement 1")]
    Replacement {
        operation: SourcePlaceOperation,
        slots: Vec<SourcePlaceSlot>,
        places: Vec<SourcePlacePath>,
        operands: Vec<SourcePlaceOperand>,
        span: SourceFoldSpan,
    },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum SourcePlaceOperation {
    #[serde(rename = "assign field")]
    AssignField,
    #[serde(rename = "clear string")]
    ClearString,
    #[serde(rename = "append string text")]
    AppendStringText,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SourcePlaceSlot {
    pub binder: SourceStateBinder,
    pub position: SourceTypeSignature,
    pub input: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SourcePlacePath {
    pub slot: usize,
    pub fields: Vec<usize>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SourcePlaceOperand {
    pub input: String,
    pub position: SourceTypeSignature,
}

pub(super) fn plain(node: &Expr) -> bool {
    node.source_position.is_none()
        && node.source_state_transition.is_none()
        && node.source_loop_fold.is_none()
        && node.source_record_place.is_none()
        && node.source_place_transform.is_none()
}

fn variable(node: &Expr, name: &str, ty: &Type) -> bool {
    plain(node) && node.var_name() == Some(name) && &node.ty == ty
}

pub(super) fn fields_for<'data>(
    position: &SourceTypeSignature,
    resolve_data: &mut impl FnMut(&Type) -> Option<&'data DataDecl>,
) -> Option<(&'data DataDecl, &'data [SourceTypeSignature])> {
    let (owner, _) = owner_view(position, false)?;
    let Type::Data { name } = &position.ty else {
        return None;
    };
    if owner.kind != "declaration"
        || !owner.path.iter().map(String::as_str).eq(name.split('.'))
        || position.array_element_position.is_some()
    {
        return None;
    }
    let data = resolve_data(&position.ty)?;
    let [constructor] = data.ctors.as_slice() else {
        return None;
    };
    let signature = data
        .source_field_signatures
        .as_ref()
        .filter(|view| view.model == "Rust data type field signatures 1")?;
    let [fields] = signature.ctors.as_slice() else {
        return None;
    };
    (fields.fields.len() == constructor.fields.len()).then_some((data, &fields.fields[..]))
}

struct Reconstruction<'a, 'data, D> {
    resolve_data: &'a mut D,
    replacement: &'a str,
    replacement_type: &'a Type,
    seen: HashSet<&'a str>,
    work: usize,
    lifetime: std::marker::PhantomData<&'data DataDecl>,
}

impl<'a, 'data, D: FnMut(&Type) -> Option<&'data DataDecl>> Reconstruction<'a, 'data, D> {
    fn projection(
        &mut self,
        mut node: &Expr,
        root: &SourceTypeSignature,
        selected: &[usize],
        input: &str,
    ) -> Option<()> {
        let mut position = root;
        let mut fields = Vec::with_capacity(selected.len());
        for &index in selected {
            let (data, positions) = fields_for(position, self.resolve_data)?;
            self.work += 1 + positions.len();
            if self.work > 100_000 {
                return None;
            }
            position = positions.get(index)?;
            fields.push((index, data, positions));
        }
        for (index, data, positions) in fields.into_iter().rev() {
            let Node::Match { scrutinee, cases } = &node.node else {
                return None;
            };
            let [case] = cases.as_slice() else {
                return None;
            };
            let Pattern::Ctor {
                data: owner,
                ctor,
                binds,
            } = &case.pattern
            else {
                return None;
            };
            let chosen = binds.get(index)?.as_deref()?;
            if !plain(node)
                || owner != &data.full_name
                || ctor != &data.ctors[0].name
                || binds.len() != positions.len()
                || chosen.is_empty()
                || binds
                    .iter()
                    .enumerate()
                    .any(|(at, bind)| at != index && bind.is_some())
                || node.ty != positions[index].ty
                || !variable(&case.body, chosen, &node.ty)
            {
                return None;
            }
            node = scrutinee;
        }
        variable(node, input, &root.ty).then_some(())
    }

    fn field(
        &mut self,
        node: &'a Expr,
        root: &SourceTypeSignature,
        selected: &[usize],
        input: &str,
    ) -> Result<SourceTypeSignature> {
        let fail = || {
            type_error(
                "a field assignment retains its actual selected declaration path and every unchanged sibling",
                node.span,
            )
        };
        let Some((&index, rest)) = selected.split_first() else {
            return variable(node, self.replacement, self.replacement_type)
                .then(|| root.clone())
                .ok_or_else(fail);
        };
        let (data, positions) = fields_for(root, self.resolve_data).ok_or_else(fail)?;
        self.work += 1 + positions.len();
        if self.work > 100_000 || !plain(node) || node.ty != root.ty || index >= positions.len() {
            return Err(fail());
        }
        let Node::Match { scrutinee, cases } = &node.node else {
            return Err(fail());
        };
        let [case] = cases.as_slice() else {
            return Err(fail());
        };
        let Pattern::Ctor {
            data: owner,
            ctor,
            binds,
        } = &case.pattern
        else {
            return Err(fail());
        };
        let Node::Ctor {
            data: rebuilt_owner,
            ctor: rebuilt,
            args,
            ..
        } = &case.body.node
        else {
            return Err(fail());
        };
        if !variable(scrutinee, input, &root.ty)
            || !plain(&case.body)
            || case.body.ty != root.ty
            || owner != &data.full_name
            || rebuilt_owner != owner
            || ctor != &data.ctors[0].name
            || rebuilt != ctor
            || binds.len() != positions.len()
            || args.len() != positions.len()
        {
            return Err(fail());
        }
        for name in binds.iter().flatten() {
            if name.is_empty() || !self.seen.insert(name.as_str()) {
                return Err(fail());
            }
        }
        for (at, (arg, position)) in args.iter().zip(positions).enumerate() {
            if at != index
                && !binds[at]
                    .as_deref()
                    .is_some_and(|binding| variable(arg, binding, &position.ty))
            {
                return Err(fail());
            }
        }
        if rest.is_empty() {
            if binds[index].is_some()
                || !variable(&args[index], self.replacement, self.replacement_type)
            {
                return Err(fail());
            }
            Ok(positions[index].clone())
        } else {
            let binding = binds[index].as_deref().ok_or_else(fail)?;
            self.field(&args[index], &positions[index], rest, binding)
        }
    }
}

pub(super) fn check_place_transform<'data>(
    facet: &RustSourcePlaceTransform,
    node: &Expr,
    language: &str,
    mut resolve_type: impl FnMut(&Type) -> Result<Type>,
    mut resolve_data: impl FnMut(&Type) -> Option<&'data DataDecl>,
) -> Result<RustSourcePlaceTransform> {
    let fail = || {
        type_error(
            "a field replacement retains its exact operation, complete source place and Unit writeback packet",
            node.span,
        )
    };
    let RustSourcePlaceTransform::Replacement {
        operation,
        slots,
        places,
        operands,
        span,
    } = facet;
    let [root] = slots.as_slice() else {
        return Err(fail());
    };
    let [selected] = places.as_slice() else {
        return Err(fail());
    };
    let operand = match (operation, operands.as_slice()) {
        (SourcePlaceOperation::ClearString, []) => None,
        (SourcePlaceOperation::AssignField | SourcePlaceOperation::AppendStringText, [operand]) => {
            Some(operand)
        }
        _ => return Err(fail()),
    };
    let Node::Let { name, value, body } = &node.node else {
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
    let [unit, result] = items.as_slice() else {
        return Err(fail());
    };
    let expected = Type::Tuple {
        elements: vec![Type::Unit, state.ty.clone()],
    };
    let mut names = HashSet::from([root.input.as_str(), name.as_str(), rebuilt.as_str()]);
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
        || selected.slot != 0
        || selected.fields.is_empty()
        || selected.fields.len() > 128
        || root.input.is_empty()
        || root.input.len() > 4096
        || names.len() != 3
        || operand.is_some_and(|operand| {
            operand.input.is_empty()
                || operand.input.len() > 4096
                || !names.insert(operand.input.as_str())
        })
        || !plain(body)
        || !plain(output)
        || !plain(unit)
        || !matches!(unit.node, Node::Unit)
        || unit.ty != Type::Unit
        || node.ty != expected
        || body.ty != expected
        || output.ty != expected
        || !variable(result, rebuilt, &state.ty)
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
    let (argument, projection) = match (operation, &value.node) {
        (SourcePlaceOperation::AssignField, Node::Var { .. }) => (Some(value.as_ref()), None),
        (
            SourcePlaceOperation::ClearString,
            Node::Lit {
                value: LitValue::Text(text),
            },
        ) if text.is_empty() && plain(value) => (None, None),
        (
            SourcePlaceOperation::AppendStringText,
            Node::Binary {
                op: BinaryOp::Concat,
                left,
                right,
                domain: None,
                semantics: None,
                rounding: None,
                by_zero: None,
            },
        ) if plain(value) => (Some(right.as_ref()), Some(left.as_ref())),
        _ => return Err(fail()),
    };
    let checked_operand = match (operand, argument) {
        (Some(operand), Some(argument)) => {
            let position = check_position(
                &operand.position,
                &argument.ty,
                language,
                &mut resolve_type,
                &mut resolve_data,
            )?;
            if argument.var_name() != Some(operand.input.as_str())
                || argument.source_state_transition.is_some()
                || argument.source_loop_fold.is_some()
                || argument.source_record_place.is_some()
                || argument.source_place_transform.is_some()
            {
                return Err(fail());
            }
            if *operation == SourcePlaceOperation::AssignField {
                if argument
                    .source_position
                    .as_ref()
                    .is_none_or(|qualifier| qualifier.position != position)
                {
                    return Err(fail());
                }
            } else if !plain(argument)
                || !string_rules::accept_rust_exclusive_source_string_suffix(
                    position.ty.kind(),
                    if position.owner.kind == "primitive" {
                        "string slice"
                    } else {
                        "owned string"
                    },
                    position.wrappers.len() as f64,
                    true,
                    position
                        .wrappers
                        .iter()
                        .all(|wrapper| wrapper == "reference"),
                    &position.owner.kind,
                    &position.owner.path.join("::"),
                    position.owner.arguments.len() as f64,
                )
            {
                return Err(fail());
            }
            Some(SourcePlaceOperand {
                input: operand.input.clone(),
                position,
            })
        }
        (None, None) => None,
        _ => return Err(fail()),
    };
    let mut reconstruction = Reconstruction {
        resolve_data: &mut resolve_data,
        replacement: name,
        replacement_type: &value.ty,
        work: 0,
        seen: names,
        lifetime: std::marker::PhantomData,
    };
    let target = reconstruction.field(state, &position, &selected.fields, &root.input)?;
    if target.ty != value.ty
        || !target.lifetimes.is_empty()
        || target.array_element_position.is_some()
    {
        return Err(fail());
    }
    if *operation == SourcePlaceOperation::AssignField {
        if checked_operand
            .as_ref()
            .is_none_or(|operand| box_projection_depth(&operand.position, &target) != Some(0))
        {
            return Err(fail());
        }
    } else if !string_rules::accept_rust_exclusive_source_string_receiver(
        target.ty.kind(),
        "owned string",
        target.wrappers.len() as f64,
        true,
        &target.owner.kind,
        &target.owner.path.join("::"),
        target.owner.arguments.len() as f64,
    ) {
        return Err(fail());
    }
    if let Some(projection) = projection {
        reconstruction
            .projection(projection, &position, &selected.fields, &root.input)
            .ok_or_else(fail)?;
    }
    Ok(RustSourcePlaceTransform::Replacement {
        operation: *operation,
        slots: vec![SourcePlaceSlot {
            binder: root.binder.clone(),
            position,
            input: root.input.clone(),
        }],
        places: places.clone(),
        operands: checked_operand.into_iter().collect(),
        span: span.clone(),
    })
}
