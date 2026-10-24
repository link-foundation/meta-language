//! `Math` and `Number` functions stay the JavaScript built-ins.

use super::{Expr, JavaScriptEmitter, Node, Result};

impl JavaScriptEmitter<'_> {
    pub(super) fn math(&mut self, e: &Expr) -> Result<String> {
        let Node::Math { op, args } = &e.node else {
            unreachable!("not a Math expression")
        };
        let args = self.exprs(args)?;
        if crate::translation::javascript_string_unit_rules::accept_linked_java_script_string_unit_construction("math",op){
            return Ok(crate::translation::javascript_string_unit_rules::render_java_script_string_unit_construction("JavaScript",&args[0]));
        }
        if crate::translation::big_integer_conversion_rules::accept_linked_java_script_big_integer_result("math",op) {
            self.helpers.insert(super::Helper::BigIntegerConversion);
            let form=crate::translation::big_integer_conversion_rules::read_java_script_big_integer_result_operation(op);
            return Ok(crate::translation::big_integer_conversion_rules::render_java_script_big_integer_result_expression("JavaScript",&form,&args[0],false));
        }
        if crate::translation::source_exception_rules::accept_linked_java_script_source_exception(
            "math", op,
        ) {
            return Ok(crate::translation::source_exception_rules::render_java_script_source_exception_expression("JavaScript",op,&args[0],&args[1],false));
        }
        if crate::translation::string_number_rules::accept_linked_java_script_number_conversion(
            "math", op,
        ) {
            return Ok(crate::translation::string_number_rules::render_java_script_number_scalar_conversion("JavaScript", op, "float", &args[0], false));
        }
        if crate::translation::rust_shift_rules::accept_linked_rust_checked_shift("math", op) {
            return Ok(crate::translation::rust_shifts::emit_rust_checked_shift(
                "JavaScript",
                e,
                &args,
            ));
        }
        if crate::translation::rust_primitive_bitwise_rules::read_rust_primitive_bitwise_arity(op)
            > 0.0
        {
            return Ok(
                crate::translation::rust_primitive_bitwise::emit_rust_primitive_bitwise(
                    "JavaScript",
                    e,
                    &args,
                ),
            );
        }
        if crate::translation::big_integer_bitwise_rules::read_big_integer_bitwise_arity(op) > 0.0 {
            let text = crate::translation::big_integer_bitwise_rules::render_big_integer_bitwise_expression("JavaScript", op, &args);
            if text.is_empty() {
                return Err(super::unsupported(
                    "BigInt bitwise arity",
                    "the operation requires its exact checked operand inventory",
                    e.span,
                ));
            }
            return Ok(text);
        }
        let symbol =
            crate::translation::number_bitwise_rules::read_java_script_number_bitwise_symbol(op);
        if !symbol.is_empty() {
            let count = match args.len() {
                1 => 1.0,
                2 => 2.0,
                _ => 0.0,
            };
            if !crate::translation::number_bitwise_rules::accept_java_script_number_bitwise_operand_count(op, count) {
                return Err(super::unsupported("Number bitwise arity", "the operation requires its exact checked operand inventory", e.span));
            }
            return Ok(if args.len() == 1 {
                format!("({symbol}{})", args[0])
            } else {
                format!("({} {symbol} {})", args[0], args[1])
            });
        }
        if op == "maxOf" || op == "minOf" {
            return Ok(format!("Math.{}(...{})", &op[..3], args[0]));
        }
        let namespace = if e.ty == super::Type::Bool {
            "Number"
        } else {
            "Math"
        };
        Ok(format!("{namespace}.{op}({})", args.join(", ")))
    }
}
