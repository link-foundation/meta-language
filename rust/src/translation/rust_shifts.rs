//! Actual operand/type ownership for shared checked-shift construction.
use super::{fixed_integer_cast_rules, rust_shift_rules as rules};
use super::{
    ir::{Expr, Node},
    types::Type,
};

pub(crate) fn emit_rust_checked_shift(
    target: &str,
    expression: &Expr,
    values: &[String],
) -> String {
    let Node::Math { op, args } = &expression.node else {
        unreachable!("a selected checked shift")
    };
    let Type::Option { element, .. } = &expression.ty else {
        unreachable!("the actual checked optional result")
    };
    let Type::Fixed { bits, signed } = element.as_ref() else {
        unreachable!("the actual shifted integer owner")
    };
    let Type::Fixed {
        signed: count_signed,
        ..
    } = args[1].ty
    else {
        unreachable!("the actual count owner")
    };
    let arguments = values
        .iter()
        .enumerate()
        .map(|(index, value)| {
            rules::render_rust_shift_integer_argument(
                target,
                value,
                if index == 0 { *signed } else { count_signed },
            )
        })
        .collect::<Vec<_>>();
    rules::render_rust_checked_shift(
        target,
        op,
        &arguments,
        f64::from(*bits),
        *signed,
        count_signed,
        &fixed_integer_cast_rules::read_fixed_integer_cast_modulus(f64::from(*bits)),
    )
}
