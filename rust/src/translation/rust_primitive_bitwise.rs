//! Target construction over actual primitive type owners, using shared rules.
use super::ir::Expr;
use super::types::Type;
use super::{
    big_integer_bitwise_rules, fixed_integer_cast_rules, rust_primitive_bitwise_rules as rules,
};

pub(crate) fn emit_rust_primitive_bitwise(
    target: &str,
    expression: &Expr,
    values: &[String],
) -> String {
    let super::ir::Node::Math { op, .. } = &expression.node else {
        unreachable!("a selected bit operation")
    };
    let (bits, signed) = match expression.ty {
        Type::Fixed { bits, signed } => (f64::from(bits), signed),
        _ => (0.0, false),
    };
    let integer =
        if matches!(expression.ty, Type::Fixed { .. }) && (target == "Lean" || target == "Rocq") {
            let arguments = values
                .iter()
                .map(|argument| {
                    rules::render_rust_primitive_bitwise_integer_argument(target, signed, argument)
                })
                .collect::<Vec<_>>();
            big_integer_bitwise_rules::render_big_integer_bitwise_expression(
                target,
                &rules::read_rust_primitive_bitwise_integer_operation(op),
                &arguments,
            )
        } else {
            String::new()
        };
    let modulus = if matches!(expression.ty, Type::Fixed { .. }) {
        fixed_integer_cast_rules::read_fixed_integer_cast_modulus(bits)
    } else {
        String::new()
    };
    rules::render_rust_primitive_bitwise_expression(
        target,
        op,
        expression.ty.kind(),
        bits,
        signed,
        values,
        &integer,
        &modulus,
    )
}
