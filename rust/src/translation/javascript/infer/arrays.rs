//! Writes inferred array element types back into the parsed program.

use super::{HashMap, SEffect, SExpr, SItem, SNode, SProgram, SProp, SPropNode, Type};

/// Writes the inferred element types of the array literals with no element of their own back.
pub(super) fn fill_arrays(program: &mut SProgram, arrays: &HashMap<*const SExpr, Type>) {
    fn walk_items(items: &mut [SItem], arrays: &HashMap<*const SExpr, Type>) {
        for item in items {
            match item {
                SItem::Fn(function) => {
                    for param in &mut function.params {
                        if let Some(default_value) = &mut param.default_value {
                            walk(default_value, arrays);
                        }
                    }
                    walk(&mut function.body, arrays);
                }
                SItem::Module(module) => walk_items(&mut module.items, arrays),
                SItem::Data(_) | SItem::Theorem(_) => {}
            }
        }
    }
    fn walk_prop(prop: &mut SProp, arrays: &HashMap<*const SExpr, Type>) {
        match &mut prop.node {
            SPropNode::Eq(comparison)
            | SPropNode::Ne(comparison)
            | SPropNode::Lt(comparison)
            | SPropNode::Le(comparison)
            | SPropNode::Gt(comparison)
            | SPropNode::Ge(comparison) => {
                walk(&mut comparison.left, arrays);
                walk(&mut comparison.right, arrays);
            }
            SPropNode::And { left, right }
            | SPropNode::Or { left, right }
            | SPropNode::Implies { left, right } => {
                walk_prop(left, arrays);
                walk_prop(right, arrays);
            }
            SPropNode::Not { arg } | SPropNode::Forall { body: arg, .. } => walk_prop(arg, arrays),
            SPropNode::Bool { expr: value } => walk(value, arrays),
        }
    }
    fn walk(node: &mut SExpr, arrays: &HashMap<*const SExpr, Type>) {
        if let Some(ty) = arrays.get(&std::ptr::from_ref::<SExpr>(node))
            && let SNode::Array { element, .. } = &mut node.node
        {
            *element = Some(ty.clone());
        }
        if let Some(test) = &mut node.tag_test {
            walk(&mut test.object, arrays);
        }
        match &mut node.node {
            SNode::App { func, args } => {
                walk(func, arrays);
                for arg in args {
                    walk(arg, arrays);
                }
            }
            SNode::Field { object, .. }
            | SNode::TypeOf { arg: object }
            | SNode::Unary { arg: object, .. }
            | SNode::ToString { arg: object }
            | SNode::Show { arg: object, .. }
            | SNode::Cast { arg: object, .. }
            | SNode::Length { object, .. } => walk(object, arrays),
            SNode::Binary { left, right, .. }
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
            } => {
                walk(left, arrays);
                walk(right, arrays);
            }
            SNode::If {
                cond,
                then,
                otherwise,
            } => {
                walk(cond, arrays);
                walk(then, arrays);
                walk(otherwise, arrays);
            }
            SNode::Match { scrutinees, rows } => {
                for scrutinee in scrutinees {
                    walk(scrutinee, arrays);
                }
                for row in rows {
                    walk(&mut row.body, arrays);
                }
            }
            SNode::Match1 { scrutinee, cases } => {
                walk(scrutinee, arrays);
                for case in cases {
                    walk(&mut case.body, arrays);
                }
            }
            SNode::CtorObject { fields, .. } => {
                for (_, value) in fields {
                    walk(value, arrays);
                }
            }
            SNode::List { items } => {
                for item in items {
                    walk(item, arrays);
                }
            }
            SNode::Array { items, .. } | SNode::Math { args: items, .. } => {
                for item in items {
                    walk(&mut item.value, arrays);
                }
            }
            _ => {}
        }
    }
    walk_items(&mut program.items, arrays);
    for effect in program.main.iter_mut().flat_map(|main| &mut main.effects) {
        match effect {
            SEffect::Print { expr: value, .. } | SEffect::Let { value, .. } => walk(value, arrays),
            SEffect::Assert { prop: value, .. } => walk_prop(value, arrays),
        }
    }
}
