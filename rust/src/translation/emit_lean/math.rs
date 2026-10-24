//! The `Math` and `Number` functions: Lean's `Float` functions where they agree
//! with JavaScript, and the generated math helpers where they do not.

use super::{Expr, LeanEmitter, Node, Result, lean_float};

impl LeanEmitter<'_> {
    pub(super) fn math(&mut self, e: &Expr, depth: usize) -> Result<String> {
        let Node::Math { op, args } = &e.node else {
            unreachable!("not a Math expression")
        };
        if crate::translation::string_construction_rules::accept_linked_java_script_string_construction("math",op){
            self.helpers.insert("utf16Text");
            let value=self.expr(&args[0],depth)?;
            return Ok(crate::translation::string_construction_rules::render_java_script_string_construction("Lean",op,&value));
        }
        if crate::translation::big_integer_conversion_rules::accept_linked_java_script_big_integer_result("math",op) {
            self.helpers.insert("bigIntegerConversion");
            let form=crate::translation::big_integer_conversion_rules::read_java_script_big_integer_result_operation(op);
            let value=self.expr(&args[0],depth)?;
            return Ok(crate::translation::big_integer_conversion_rules::render_java_script_big_integer_result_expression("Lean",&form,&value,false));
        }
        if crate::translation::string_number_rules::accept_linked_java_script_number_conversion(
            "math", op,
        ) {
            if matches!(args[0].ty.kind(), "string" | "int" | "nat") {
                self.helpers.insert("stringNumber");
            }
            let value = self.expr(&args[0], depth)?;
            let text = crate::translation::string_number_rules::render_java_script_number_scalar_conversion("Lean", op, args[0].ty.kind(), &value, false);
            if text.is_empty() {
                return Err(super::unsupported(
                    "numeric conversion operand",
                    "the checked conversion must retain its total value or threaded abort",
                    e.span,
                ));
            }
            return Ok(text);
        }
        if crate::translation::rust_shift_rules::accept_linked_rust_checked_shift("math", op) {
            let values = args
                .iter()
                .map(|argument| self.expr(argument, depth))
                .collect::<Result<Vec<_>>>()?;
            return Ok(crate::translation::rust_shifts::emit_rust_checked_shift(
                "Lean", e, &values,
            ));
        }
        if crate::translation::rust_primitive_bitwise_rules::read_rust_primitive_bitwise_arity(op)
            > 0.0
        {
            let values = args
                .iter()
                .map(|argument| self.expr(argument, depth))
                .collect::<Result<Vec<_>>>()?;
            return Ok(
                crate::translation::rust_primitive_bitwise::emit_rust_primitive_bitwise(
                    "Lean", e, &values,
                ),
            );
        }
        if crate::translation::big_integer_bitwise_rules::read_big_integer_bitwise_arity(op) > 0.0 {
            let values = args
                .iter()
                .map(|argument| self.expr(argument, depth))
                .collect::<Result<Vec<_>>>()?;
            return Ok(crate::translation::big_integer_bitwise_rules::render_big_integer_bitwise_expression("Lean", op, &values));
        }
        let mut texts = Vec::with_capacity(args.len());
        for arg in args {
            texts.push(self.expr(arg, depth)?);
        }
        let native = match op.as_str() {
            "abs" => Some("Float.abs"),
            "floor" => Some("Float.floor"),
            "ceil" => Some("Float.ceil"),
            "sqrt" => Some("Float.sqrt"),
            "isFinite" => Some("Float.isFinite"),
            "isNaN" => Some("Float.isNaN"),
            _ => None,
        };
        if let Some(native) = native {
            return Ok(format!("({native} {})", texts[0]));
        }
        self.helpers.insert("math");
        let bound = |max: bool| lean_float(if max { "-Infinity" } else { "Infinity" });
        Ok(match op.as_str() {
            "max" | "min" => texts
                .into_iter()
                .reduce(|left, right| format!("(ml_{op} {left} {right})"))
                .unwrap_or_else(|| bound(op == "max")),
            "maxOf" | "minOf" => format!(
                "({}.foldl ml_{} {})",
                texts[0],
                &op[..3],
                bound(op == "maxOf")
            ),
            _ => {
                let name = match op.as_str() {
                    "isInteger" => "is_integer",
                    "isSafeInteger" => "is_safe_integer",
                    other => other,
                };
                format!("(ml_{name} {})", texts[0])
            }
        })
    }
}
