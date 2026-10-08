//! Portability checks, effects and notation-scope resolution.

use super::{
    BinaryOp, ProofTypes, Result, SComparison, SEffect, SExpr, SNode, SPattern, SPatternNode,
    SProp, SPropNode, SRow, ShowStyle, Span, Type, scope_type, unsupported,
};

/// Lists only appear as the program output: the first list node, in the
/// JavaScript runtime's depth-first key order, is rejected.
pub(super) fn check_portable(node: &SExpr) -> Result<()> {
    let children: Vec<&SExpr> = match &node.node {
        SNode::Cons { .. } | SNode::Nil | SNode::List { .. } => {
            return Err(unsupported(
                "list value",
                "lists are outside the portable core except as the program output",
                node.span,
            ));
        }
        SNode::Binary { left, right, .. } => vec![left, right],
        SNode::If {
            cond,
            then,
            otherwise,
        } => vec![cond, then, otherwise],
        SNode::StringTest { object, search, .. } => vec![object, search],
        SNode::Let { value, body, .. } => vec![value, body],
        SNode::Match { scrutinees, rows } => scrutinees
            .iter()
            .chain(rows.iter().map(|row| &row.body))
            .collect(),
        SNode::App { func, args } => std::iter::once(func.as_ref()).chain(args).collect(),
        SNode::TypeOf { arg }
        | SNode::Unary { arg, .. }
        | SNode::Cast { arg, .. }
        | SNode::ToString { arg } => {
            vec![arg]
        }
        _ => Vec::new(),
    };
    children.into_iter().try_for_each(check_portable)
}

/// The program output: a list of lines built with `::`, `[ ; ]` and `let`.
pub(super) fn walk_output(node: SExpr, effects: &mut Vec<SEffect>) -> Result<()> {
    let node_span = node.span;
    match node.node {
        SNode::Nil => Ok(()),
        SNode::List { items } => {
            for item in items {
                effects.push(print_effect(item)?);
            }
            Ok(())
        }
        SNode::Cons { head, tail } => {
            effects.push(print_effect(*head)?);
            walk_output(*tail, effects)
        }
        SNode::Let {
            name,
            ty,
            value,
            body,
        } => {
            check_portable(&value)?;
            effects.push(SEffect::Let {
                name,
                ty,
                value: *value,
                constant: false,
                span: node_span,
            });
            walk_output(*body, effects)
        }
        _ => Err(unsupported(
            "program output",
            "main must be a list of lines built with ::, [ ; ] and let",
            node_span,
        )),
    }
}

pub(super) fn print_effect(expr: SExpr) -> Result<SEffect> {
    check_portable(&expr)?;
    let span = expr.span;
    Ok(SEffect::Print {
        expr,
        style: ShowStyle::Rocq,
        span,
    })
}

/// Numerals in notation positions take the delimited scope; arguments of
/// applications keep the scope of their parameter type, which the checker
/// infers from the signature, exactly as Rocq's argument scopes do.
pub(super) fn with_scope(node: SExpr, scope: &str) -> SExpr {
    match scope_type(scope) {
        Some(ty) => scope_expr(node, &ty),
        None => node,
    }
}

pub(super) fn scope_expr(node: SExpr, ty: &Type) -> SExpr {
    let visit = |inner: Box<SExpr>| Box::new(scope_expr(*inner, ty));
    let scoped = match node.node {
        SNode::Num {
            value,
            ty: None,
            negative,
            ..
        } => SNode::Num {
            value,
            ty: Some(ty.clone()),
            negative,
            unit: false,
        },
        SNode::StringTest { op, object, search } => SNode::StringTest {
            op,
            object: visit(object),
            search: visit(search),
        },
        SNode::TypeOf { arg } => SNode::TypeOf { arg: visit(arg) },
        SNode::Unary { op, arg } => SNode::Unary {
            op,
            arg: visit(arg),
        },
        SNode::Binary {
            op:
                op @ (BinaryOp::Eq
                | BinaryOp::Lt
                | BinaryOp::Le
                | BinaryOp::Gt
                | BinaryOp::Ge
                | BinaryOp::Ne
                | BinaryOp::Add
                | BinaryOp::Sub
                | BinaryOp::Mul
                | BinaryOp::Div
                | BinaryOp::Rem),
            left,
            right,
            rounding,
        } => SNode::Binary {
            op,
            left: visit(left),
            right: visit(right),
            rounding,
        },
        SNode::If {
            cond,
            then,
            otherwise,
        } => SNode::If {
            cond,
            then: visit(then),
            otherwise: visit(otherwise),
        },
        SNode::Let {
            name,
            ty: let_type,
            value,
            body,
        } => SNode::Let {
            name,
            ty: let_type,
            value,
            body: visit(body),
        },
        SNode::Match { scrutinees, rows } => SNode::Match {
            scrutinees,
            rows: rows
                .into_iter()
                .map(|row| SRow {
                    body: scope_expr(row.body, ty),
                    ..row
                })
                .collect(),
        },
        other => other,
    };
    SExpr {
        node: scoped,
        ..node
    }
}

/// The literals of a whole proposition take the delimited scope.
pub(super) fn scope_prop(prop: SProp, scope: &str) -> SProp {
    let visit = |inner: Box<SProp>| Box::new(scope_prop(*inner, scope));
    let compare = |comparison: SComparison| SComparison {
        left: with_scope(comparison.left, scope),
        right: with_scope(comparison.right, scope),
        reference: comparison.reference,
        same_value: comparison.same_value,
    };
    let node = match prop.node {
        SPropNode::Forall { binders, body } => SPropNode::Forall {
            binders,
            body: visit(body),
        },
        SPropNode::Not { arg } => SPropNode::Not { arg: visit(arg) },
        SPropNode::Bool { expr } => SPropNode::Bool { expr },
        SPropNode::And { left, right } => SPropNode::And {
            left: visit(left),
            right: visit(right),
        },
        SPropNode::Or { left, right } => SPropNode::Or {
            left: visit(left),
            right: visit(right),
        },
        SPropNode::Implies { left, right } => SPropNode::Implies {
            left: visit(left),
            right: visit(right),
        },
        SPropNode::Eq(comparison) => SPropNode::Eq(compare(comparison)),
        SPropNode::Ne(comparison) => SPropNode::Ne(compare(comparison)),
        SPropNode::Lt(comparison) => SPropNode::Lt(compare(comparison)),
        SPropNode::Le(comparison) => SPropNode::Le(compare(comparison)),
        SPropNode::Gt(comparison) => SPropNode::Gt(compare(comparison)),
        SPropNode::Ge(comparison) => SPropNode::Ge(compare(comparison)),
    };
    SProp { node, ..prop }
}

/// The value a fully bound pattern matched, for `p as x`.
pub(super) fn pattern_value(pattern: &SPattern, range: Span) -> Result<SExpr> {
    let node = |node: SNode| SExpr::new(node, Some(range));
    match &pattern.node {
        SPatternNode::BindOrCtor { name } => Ok(node(SNode::Name {
            path: vec![name.clone()],
        })),
        SPatternNode::NumLit { value, .. } => Ok(node(SNode::Num {
            value: value.clone(),
            ty: None,
            negative: false,
            unit: false,
        })),
        SPatternNode::Ctor { path, args } => {
            let name = path.join(".");
            if (name == "S" || name == "Nat.succ") && args.len() == 1 {
                let left = pattern_value(&args[0], range)?;
                let one = node(SNode::Num {
                    value: "1".to_owned(),
                    ty: None,
                    negative: false,
                    unit: false,
                });
                return Ok(node(SNode::Binary {
                    op: BinaryOp::Add,
                    left: Box::new(left),
                    right: Box::new(one),
                    rounding: None,
                }));
            }
            let args = args
                .iter()
                .map(|arg| pattern_value(arg, range))
                .collect::<Result<Vec<_>>>()?;
            Ok(node(SNode::App {
                func: Box::new(node(SNode::Name { path: path.clone() })),
                args,
            }))
        }
        _ => Err(unsupported(
            "as-pattern",
            "an aliased pattern must bind every field",
            Some(range),
        )),
    }
}

pub(super) fn collect_forall_types(prop: &SProp, types: &mut ProofTypes) {
    if let SPropNode::Forall { binders, body } = &prop.node {
        for binder in binders {
            if let Some(rocq_type) = &binder.rocq_type {
                types.insert(binder.name.clone(), rocq_type.clone());
            }
        }
        collect_forall_types(body, types);
    }
}
