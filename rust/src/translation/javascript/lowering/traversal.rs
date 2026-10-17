//! Existing generic surface expression child traversal, moved without semantic changes.

use super::{SExpr, SNode};

/// The subexpressions the JavaScript frontend's generic traversals visit.
/// They skip keys named `span` and `type`, so constructor-object fields with
/// those names are skipped too.
pub(in crate::translation::javascript) fn children(expr: &SExpr) -> Vec<&SExpr> {
    let mut out: Vec<&SExpr> = Vec::new();
    match &expr.node {
        SNode::Record {
            entries,
            prototype_definition,
            ..
        } => {
            out.extend(entries.iter().map(|entry| &entry.value));
            if let Some(definition) = prototype_definition {
                out.push(&definition.value);
            }
        }
        SNode::Lambda { params, body, .. } => {
            out.extend(
                params
                    .iter()
                    .filter_map(|parameter| parameter.default_value.as_ref()),
            );
            out.push(body);
        }
        SNode::CellWrite { cell, value, body } => {
            out.push(cell);
            out.push(value);
            out.push(body);
        }
        SNode::Print { expr, body, .. } => {
            out.push(expr);
            out.push(body);
        }
        SNode::Method { object, args, .. } => {
            out.push(object);
            out.extend(args.iter().flatten());
        }
        SNode::ArrayFrom { args, .. } => out.extend(args),
        SNode::App { func, args } => {
            out.push(func);
            out.extend(args);
        }
        SNode::ArrayBindingRest { object, .. }
        | SNode::Set { items: object, .. }
        | SNode::Map {
            entries: object, ..
        }
        | SNode::CopySlice { object }
        | SNode::Some { value: object, .. }
        | SNode::FlattenOption { value: object }
        | SNode::OptionalField { object, .. }
        | SNode::CellAllocate { value: object, .. }
        | SNode::CellRead { cell: object }
        | SNode::TypeOf { arg: object }
        | SNode::Unary { arg: object, .. }
        | SNode::StringMap { object, .. }
        | SNode::ToString { arg: object }
        | SNode::Show { arg: object, .. }
        | SNode::Field { object, .. }
        | SNode::Length { object, .. }
        | SNode::ForOfIterable { object, .. }
        | SNode::StringPoints { object } => out.push(object),
        SNode::ReferenceWrite {
            object,
            index,
            value,
            ..
        } => {
            out.push(object);
            out.extend(index.iter().map(Box::as_ref));
            out.extend(value.iter().map(Box::as_ref));
        }
        SNode::Array { items, .. } | SNode::Math { args: items, .. } => {
            out.extend(items.iter().map(|item| &item.value));
        }
        SNode::ArrayBindingDefault {
            value: left,
            fallback: right,
            ..
        }
        | SNode::Coalesce { left, right }
        | SNode::Index {
            object: left,
            index: right,
            ..
        }
        | SNode::Let {
            value: left,
            body: right,
            ..
        }
        | SNode::StringTest {
            object: left,
            search: right,
            ..
        }
        | SNode::Binary { left, right, .. } => {
            out.push(left);
            out.push(right);
        }
        SNode::If {
            cond,
            then,
            otherwise,
        } => {
            out.push(cond);
            out.push(then);
            out.push(otherwise);
        }
        SNode::Match { scrutinees, rows } => {
            out.extend(scrutinees);
            out.extend(rows.iter().map(|row| &row.body));
        }
        SNode::CtorObject {
            fields,
            prototype_definition,
            ..
        } => {
            out.extend(
                fields
                    .iter()
                    .filter(|(name, _)| name != "span" && name != "type")
                    .map(|(_, value)| value),
            );
            if let Some(definition) = prototype_definition {
                out.push(&definition.value);
            }
        }
        _ => {}
    }
    if let Some(test) = &expr.tag_test {
        out.push(&test.object);
    }
    out
}

/// The subexpressions of `children`, to rewrite.
pub(in crate::translation::javascript) fn children_mut(expr: &mut SExpr) -> Vec<&mut SExpr> {
    let mut out: Vec<&mut SExpr> = Vec::new();
    match &mut expr.node {
        SNode::Record {
            entries,
            prototype_definition,
            ..
        } => {
            out.extend(entries.iter_mut().map(|entry| &mut entry.value));
            if let Some(definition) = prototype_definition {
                out.push(&mut definition.value);
            }
        }
        SNode::Lambda { params, body, .. } => {
            out.extend(
                params
                    .iter_mut()
                    .filter_map(|parameter| parameter.default_value.as_mut()),
            );
            out.push(body);
        }
        SNode::CellWrite { cell, value, body } => {
            out.push(cell);
            out.push(value);
            out.push(body);
        }
        SNode::Print { expr, body, .. } => {
            out.push(expr);
            out.push(body);
        }
        SNode::Method { object, args, .. } => {
            out.push(object);
            out.extend(args.iter_mut().flatten());
        }
        SNode::ArrayFrom { args, .. } => out.extend(args),
        SNode::App { func, args } => {
            out.push(func);
            out.extend(args);
        }
        SNode::ArrayBindingRest { object, .. }
        | SNode::Set { items: object, .. }
        | SNode::Map {
            entries: object, ..
        }
        | SNode::CopySlice { object }
        | SNode::Some { value: object, .. }
        | SNode::FlattenOption { value: object }
        | SNode::OptionalField { object, .. }
        | SNode::CellAllocate { value: object, .. }
        | SNode::CellRead { cell: object }
        | SNode::TypeOf { arg: object }
        | SNode::Unary { arg: object, .. }
        | SNode::StringMap { object, .. }
        | SNode::ToString { arg: object }
        | SNode::Show { arg: object, .. }
        | SNode::Field { object, .. }
        | SNode::Length { object, .. }
        | SNode::ForOfIterable { object, .. }
        | SNode::StringPoints { object } => out.push(object),
        SNode::ReferenceWrite {
            object,
            index,
            value,
            ..
        } => {
            out.push(object);
            out.extend(index.iter_mut().map(Box::as_mut));
            out.extend(value.iter_mut().map(Box::as_mut));
        }
        SNode::Array { items, .. } | SNode::Math { args: items, .. } => {
            out.extend(items.iter_mut().map(|item| &mut item.value));
        }
        SNode::ArrayBindingDefault {
            value: left,
            fallback: right,
            ..
        }
        | SNode::Coalesce { left, right }
        | SNode::Index {
            object: left,
            index: right,
            ..
        }
        | SNode::Let {
            value: left,
            body: right,
            ..
        }
        | SNode::StringTest {
            object: left,
            search: right,
            ..
        }
        | SNode::Binary { left, right, .. } => {
            out.push(left);
            out.push(right);
        }
        SNode::If {
            cond,
            then,
            otherwise,
        } => {
            out.push(cond);
            out.push(then);
            out.push(otherwise);
        }
        SNode::Match { scrutinees, rows } => {
            out.extend(scrutinees);
            out.extend(rows.iter_mut().map(|row| &mut row.body));
        }
        SNode::CtorObject {
            fields,
            prototype_definition,
            ..
        } => {
            out.extend(
                fields
                    .iter_mut()
                    .filter(|(name, _)| name != "span" && name != "type")
                    .map(|(_, value)| value),
            );
            if let Some(definition) = prototype_definition {
                out.push(&mut definition.value);
            }
        }
        _ => {}
    }
    if let Some(test) = &mut expr.tag_test {
        out.push(&mut test.object);
    }
    out
}
