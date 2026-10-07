//! Machine-integer operations and checked conversions that cannot abort, as
//! total operations on their Lean and Rocq representation.
//!
//! Mirrors the totalization half of `js/src/translation/output.js`.

use crate::translation::ir::{
    ByZero, Comparison, Decl, Effect, Expr, FnDecl, LitValue, Main, Node, Program, Prop, Semantics,
    TheoremDecl,
};
use crate::translation::surface::{BinaryOp, Flavor};
use crate::translation::types::{INT, NAT, Type, fixed_bounds};

/// The unbounded type a machine integer is represented by in Lean and Rocq.
pub(super) fn representation(ty: &Type) -> Type {
    match ty {
        Type::Fixed { signed: true, .. } => INT,
        Type::Fixed { signed: false, .. } => NAT,
        other => other.clone(),
    }
}

/// The value of an integer literal; one beyond 128 bits is the nearest bound,
/// which compares as it does with every value tested here.
fn integer_literal(e: &Expr) -> Option<i128> {
    let Node::Lit {
        value: LitValue::Text(text),
    } = &e.node
    else {
        return None;
    };
    let digits = text.strip_prefix('-').unwrap_or(text);
    if digits.is_empty() || !digits.bytes().all(|byte| byte.is_ascii_digit()) {
        return None;
    }
    Some(text.parse().unwrap_or_else(|_| {
        if text.starts_with('-') {
            i128::MIN + 1
        } else {
            i128::MAX
        }
    }))
}

/// The least value of a machine-integer type.
pub(super) fn fixed_min(ty: &Type) -> i128 {
    match ty {
        Type::Fixed { bits, signed } => fixed_bounds(*bits, *signed).0,
        _ => 0,
    }
}

/// The greatest value of a machine-integer type.
pub(super) fn fixed_max(ty: &Type) -> u128 {
    match ty {
        Type::Fixed { bits, signed } => fixed_bounds(*bits, *signed).1,
        _ => 0,
    }
}

pub(super) const fn division(op: BinaryOp) -> bool {
    matches!(op, BinaryOp::Div | BinaryOp::Rem)
}

/// True when the node itself, apart from its operands, may abort.
#[must_use]
pub(super) fn aborts(e: &Expr) -> bool {
    match &e.node {
        Node::Abort { .. } => true,
        Node::Cast {
            flavor: Flavor::Checked,
            arg,
            ..
        } => integer_literal(arg).is_none_or(|value| value < 0),
        Node::Unary {
            semantics: Some(Semantics::Checked),
            arg,
            ..
        } => integer_literal(arg).is_none_or(|value| value == fixed_min(&e.ty)),
        Node::Binary {
            op,
            right,
            semantics,
            by_zero,
            ..
        } => {
            if !division(*op) {
                return *semantics == Some(Semantics::Checked);
            }
            if *by_zero != Some(ByZero::Abort) {
                return false;
            }
            // A literal divisor aborts only when it is zero, or -1 of a signed machine integer (MIN / -1 overflows).
            match integer_literal(right) {
                None | Some(0) => true,
                Some(divisor) => matches!(e.ty, Type::Fixed { signed: true, .. }) && divisor == -1,
            }
        }
        _ => false,
    }
}

/// A machine-integer operation or a checked conversion that cannot abort,
/// computed on its representation. The result keeps its source type, whose
/// Lean and Rocq representation is the same.
pub(super) fn total(e: Expr) -> Expr {
    let Expr { node, ty, span } = e;
    let (node, ty) = match node {
        Node::Cast {
            arg,
            from,
            to,
            message,
            order,
            ..
        } => (
            Node::Cast {
                arg,
                from,
                to,
                flavor: Flavor::Clamp,
                message,
                order,
            },
            ty,
        ),
        Node::Unary { op, arg, .. } => (
            Node::Unary {
                op,
                arg,
                semantics: Some(Semantics::Exact),
            },
            representation(&ty),
        ),
        Node::Binary {
            op,
            left,
            right,
            rounding,
            by_zero,
            ..
        } => {
            let ty = representation(&ty);
            let semantics = if op == BinaryOp::Sub && ty == NAT {
                Semantics::Truncated
            } else {
                Semantics::Exact
            };
            (
                Node::Binary {
                    op,
                    left,
                    right,
                    domain: Some(ty.clone()),
                    semantics: Some(semantics),
                    rounding,
                    by_zero: if division(op) {
                        Some(ByZero::Total)
                    } else {
                        by_zero
                    },
                },
                ty,
            )
        }
        other => (other, ty),
    };
    Expr { node, ty, span }
}

/// True when the node is an operation `total` rewrites.
fn partial(e: &Expr) -> bool {
    match &e.node {
        Node::Cast { flavor, .. } => *flavor == Flavor::Checked,
        Node::Unary { semantics, .. } => *semantics == Some(Semantics::Checked),
        Node::Binary {
            semantics, by_zero, ..
        } => *semantics == Some(Semantics::Checked) || *by_zero == Some(ByZero::Abort),
        _ => false,
    }
}

/// Every operation that may abort but cannot here, as a total one.
fn settle(e: &Expr) -> Expr {
    let copy = e.map_children(&mut |child| settle(child));
    if partial(&copy) && !aborts(&copy) {
        total(copy)
    } else {
        copy
    }
}

fn settle_comparison(comparison: &Comparison) -> Comparison {
    Comparison {
        left: settle(&comparison.left),
        right: settle(&comparison.right),
        ..comparison.clone()
    }
}

fn settle_prop(prop: &Prop) -> Prop {
    match prop {
        Prop::Forall { binders, body } => Prop::Forall {
            binders: binders.clone(),
            body: Box::new(settle_prop(body)),
        },
        Prop::And { left, right } => Prop::And {
            left: Box::new(settle_prop(left)),
            right: Box::new(settle_prop(right)),
        },
        Prop::Or { left, right } => Prop::Or {
            left: Box::new(settle_prop(left)),
            right: Box::new(settle_prop(right)),
        },
        Prop::Implies { left, right } => Prop::Implies {
            left: Box::new(settle_prop(left)),
            right: Box::new(settle_prop(right)),
        },
        Prop::Not { arg } => Prop::Not {
            arg: Box::new(settle_prop(arg)),
        },
        Prop::Bool { expr } => Prop::Bool { expr: settle(expr) },
        other => {
            let (op, comparison) = other.comparison().expect("a comparison");
            Prop::compare(op, settle_comparison(comparison))
        }
    }
}

fn settle_effect(effect: &Effect) -> Effect {
    match effect {
        Effect::Print { expr, span } => Effect::Print {
            expr: settle(expr),
            span: *span,
        },
        Effect::Let {
            name,
            value,
            span,
            constant,
        } => Effect::Let {
            constant: *constant,
            name: name.clone(),
            value: settle(value),
            span: *span,
        },
        Effect::Assert { prop, span } => Effect::Assert {
            prop: settle_prop(prop),
            span: *span,
        },
        Effect::Output { expr, span } => Effect::Output {
            expr: settle(expr),
            span: *span,
        },
        other @ Effect::Unwrap { .. } => other.clone(),
    }
}

pub(super) fn settle_program(program: &Program) -> Program {
    let declarations = program
        .declarations
        .iter()
        .map(|entry| match entry {
            Decl::Fn(function) => Decl::Fn(FnDecl {
                body: settle(&function.body),
                ..function.clone()
            }),
            Decl::Theorem(theorem) => Decl::Theorem(TheoremDecl {
                prop: settle_prop(&theorem.prop),
                ..theorem.clone()
            }),
            Decl::Data(_) => entry.clone(),
        })
        .collect();
    let main = program.main.as_ref().map(|main| Main {
        effects: main.effects.iter().map(settle_effect).collect(),
        ..main.clone()
    });
    Program {
        main,
        declarations,
        ..program.clone()
    }
}
