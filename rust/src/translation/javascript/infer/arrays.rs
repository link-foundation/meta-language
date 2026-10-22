//! Writes inferred array element types back into the parsed program.

use super::{HashMap, SEffect, SExpr, SItem, SNode, SProgram, SProp, SPropNode, Type};
use crate::translation::surface::SurfaceBindingTypeAnnotation;

/// Writes the inferred element types of the array literals with no element of their own back.
pub(super) fn fill_arrays(
    program: &mut SProgram,
    arrays: &HashMap<*const SExpr, Type>,
    solved: &HashMap<*const SExpr, Type>,
) {
    fn walk_items(
        items: &mut [SItem],
        arrays: &HashMap<*const SExpr, Type>,
        solved: &HashMap<*const SExpr, Type>,
    ) {
        for item in items {
            match item {
                SItem::Fn(function) => {
                    for param in &mut function.params {
                        if let Some(default_value) = &mut param.default_value {
                            walk(default_value, arrays, solved);
                        }
                    }
                    walk(&mut function.body, arrays, solved);
                }
                SItem::Module(module) => walk_items(&mut module.items, arrays, solved),
                SItem::Data(_) | SItem::Theorem(_) => {}
            }
        }
    }
    fn walk_prop(
        prop: &mut SProp,
        arrays: &HashMap<*const SExpr, Type>,
        solved: &HashMap<*const SExpr, Type>,
    ) {
        match &mut prop.node {
            SPropNode::Eq(comparison)
            | SPropNode::Ne(comparison)
            | SPropNode::Lt(comparison)
            | SPropNode::Le(comparison)
            | SPropNode::Gt(comparison)
            | SPropNode::Ge(comparison) => {
                walk(&mut comparison.left, arrays, solved);
                walk(&mut comparison.right, arrays, solved);
            }
            SPropNode::And { left, right }
            | SPropNode::Or { left, right }
            | SPropNode::Implies { left, right } => {
                walk_prop(left, arrays, solved);
                walk_prop(right, arrays, solved);
            }
            SPropNode::Not { arg } | SPropNode::Forall { body: arg, .. } => {
                walk_prop(arg, arrays, solved)
            }
            SPropNode::Bool { expr: value } => walk(value, arrays, solved),
        }
    }
    fn walk(
        node: &mut SExpr,
        arrays: &HashMap<*const SExpr, Type>,
        solved: &HashMap<*const SExpr, Type>,
    ) {
        if let SNode::Let {
            ty: retained,
            value,
            ..
        } = &mut node.node
            && retained
                .as_ref()
                .and_then(SurfaceBindingTypeAnnotation::as_ref)
                .is_none()
            && let Some(ty @ Type::Option { element, .. }) =
                solved.get(&std::ptr::from_ref(value.as_ref()))
            && matches!(element.as_ref(), Type::Tuple { .. })
        {
            *retained = Some(ty.clone().into());
        }
        if let Some(ty) = arrays.get(&std::ptr::from_ref::<SExpr>(node)) {
            match (&mut node.node, ty) {
                (
                    SNode::Lambda { params, ret, .. },
                    Type::Function {
                        params: inferred,
                        ret: inferred_ret,
                    },
                ) => {
                    for (parameter, ty) in params.iter_mut().zip(inferred) {
                        if parameter.ty.is_none() {
                            parameter.ty = Some(ty.clone());
                        }
                    }
                    *ret = Some(inferred_ret.as_ref().clone());
                }
                (
                    SNode::ForOfIterable { ty: retained, .. }
                    | SNode::Set { ty: retained, .. }
                    | SNode::Map { ty: retained, .. }
                    | SNode::ArrayFrom { ty: retained, .. }
                    | SNode::Record { ty: retained, .. }
                    | SNode::CellAllocate { ty: retained, .. },
                    _,
                ) => *retained = Some(ty.clone()),
                _ => {}
            }
        }
        if let Some(ty) = arrays.get(&std::ptr::from_ref::<SExpr>(node))
            && let SNode::Array { element, .. } = &mut node.node
        {
            *element = Some(ty.clone());
        }
        if let Some(test) = &mut node.tag_test {
            walk(&mut test.object, arrays, solved);
        }
        match &mut node.node {
            SNode::ArrayBindingDefault {
                value, fallback, ..
            } => {
                walk(value, arrays, solved);
                walk(fallback, arrays, solved);
            }
            SNode::Record {
                entries,
                prototype_definition,
                ..
            } => {
                for entry in entries {
                    walk(&mut entry.value, arrays, solved);
                }
                if let Some(definition) = prototype_definition {
                    walk(&mut definition.value, arrays, solved);
                }
            }
            SNode::Method { object, args, .. } => {
                walk(object, arrays, solved);
                for arg in args.iter_mut().flatten() {
                    walk(arg, arrays, solved);
                }
            }
            SNode::ArrayFrom { args, .. } => {
                for argument in args {
                    walk(argument, arrays, solved);
                }
            }
            SNode::App { func, args } => {
                walk(func, arrays, solved);
                for arg in args {
                    walk(arg, arrays, solved);
                }
            }
            SNode::Lambda { params, body, .. } => {
                for parameter in params {
                    if let Some(value) = &mut parameter.default_value {
                        walk(value, arrays, solved);
                    }
                }
                walk(body, arrays, solved);
            }
            SNode::CellWrite { cell, value, body } => {
                walk(cell, arrays, solved);
                walk(value, arrays, solved);
                walk(body, arrays, solved);
            }
            SNode::ArrayBindingRest { object, .. }
            | SNode::Set { items: object, .. }
            | SNode::Map {
                entries: object, ..
            }
            | SNode::CopySlice { object }
            | SNode::Field { object, .. }
            | SNode::CellAllocate { value: object, .. }
            | SNode::ReferencePayload { value: object, .. }
            | SNode::CellRead { cell: object }
            | SNode::TypeOf { arg: object }
            | SNode::ArrayIsArray { arg: object }
            | SNode::Unary { arg: object, .. }
            | SNode::StringMap { object, .. }
            | SNode::ToString { arg: object }
            | SNode::Show { arg: object, .. }
            | SNode::Cast { arg: object, .. }
            | SNode::Length { object, .. }
            | SNode::ForOfIterable { object, .. }
            | SNode::StringPoints { object }
            | SNode::TupleIndex { object, .. } => walk(object, arrays, solved),
            SNode::ReferenceWrite {
                object,
                index,
                value,
                ..
            } => {
                walk(object, arrays, solved);
                if let Some(index) = index {
                    walk(index, arrays, solved);
                }
                if let Some(value) = value {
                    walk(value, arrays, solved);
                }
            }
            SNode::StringTest {
                object: left,
                search: right,
                ..
            }
            | SNode::ReferenceIdentity { left, right, .. }
            | SNode::Binary { left, right, .. }
            | SNode::Let {
                value: left,
                body: right,
                ..
            }
            | SNode::Print {
                expr: left,
                body: right,
                ..
            }
            | SNode::Cons {
                head: left,
                tail: right,
            }
            | SNode::Index {
                object: left,
                index: right,
                ..
            } => {
                walk(left, arrays, solved);
                walk(right, arrays, solved);
            }
            SNode::If {
                cond,
                then,
                otherwise,
            } => {
                walk(cond, arrays, solved);
                walk(then, arrays, solved);
                walk(otherwise, arrays, solved);
            }
            SNode::Match { scrutinees, rows } => {
                for scrutinee in scrutinees {
                    walk(scrutinee, arrays, solved);
                }
                for row in rows {
                    walk(&mut row.body, arrays, solved);
                }
            }
            SNode::Match1 { scrutinee, cases } => {
                walk(scrutinee, arrays, solved);
                for case in cases {
                    walk(&mut case.body, arrays, solved);
                }
            }
            SNode::CtorObject {
                fields,
                prototype_definition,
                ..
            } => {
                for (_, value) in fields {
                    walk(value, arrays, solved);
                }
                if let Some(definition) = prototype_definition {
                    walk(&mut definition.value, arrays, solved);
                }
            }
            SNode::List { items } | SNode::Tuple { items, .. } => {
                for item in items {
                    walk(item, arrays, solved);
                }
            }
            SNode::Array { items, .. } | SNode::Math { args: items, .. } => {
                for item in items {
                    walk(&mut item.value, arrays, solved);
                }
            }
            _ => {}
        }
    }
    walk_items(&mut program.items, arrays, solved);
    for effect in program.main.iter_mut().flat_map(|main| &mut main.effects) {
        match effect {
            SEffect::Print { expr: value, .. } | SEffect::Let { value, .. } => {
                walk(value, arrays, solved)
            }
            SEffect::Assert { prop: value, .. } => walk_prop(value, arrays, solved),
        }
    }
}
