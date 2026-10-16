//! Apply the complete checked optional syntax to actual target operands.
use super::{Expr, LeanEmitter, LitValue, Node, Result, Type, literal};
use crate::translation::optional_value_rules as rules;
use crate::translation::primitive_value_rules::render_absent_number_expression;

fn policy(ty: &Type) -> &str {
    match ty {
        Type::Option { absence, .. } => absence,
        _ => "",
    }
}
fn element(ty: &Type) -> &Type {
    match ty {
        Type::Option { element, .. } => element,
        _ => ty,
    }
}
fn number(value: usize) -> f64 {
    serde_json::Number::from(value)
        .as_f64()
        .expect("an actual field position has a Number representation")
}

fn text_literal(text: &str) -> Result<String> {
    literal(&Type::String, &LitValue::Text(text.to_owned()))
}

impl LeanEmitter<'_> {
    fn optional_match(
        &mut self,
        arg: &Expr,
        binder: &str,
        present: &str,
        null_case: &str,
        undefined_case: &str,
        depth: usize,
    ) -> Result<String> {
        Ok(rules::render_optional_match(
            "Lean",
            policy(&arg.ty),
            &self.expr(arg, depth)?,
            binder,
            present,
            null_case,
            undefined_case,
        ))
    }

    fn optional_truth(&mut self, arg: &Expr, depth: usize) -> Result<String> {
        if let Type::Option { element, .. } = &arg.ty {
            let present =
                self.optional_truth(&Expr::var("ml_present", element.as_ref().clone()), depth)?;
            return self.optional_match(arg, "ml_present", &present, "false", "false", depth);
        }

        Ok(rules::render_optional_primitive_truth(
            "Lean",
            arg.ty.kind(),
            &self.expr(arg, depth)?,
            "",
            false,
            matches!(arg.ty, Type::Fixed { signed: true, .. }),
        ))
    }

    pub(super) fn optional_expr(&mut self, expr: &Expr, depth: usize) -> Result<String> {
        match &expr.node {
            Node::None { absence } => {
                let ty = String::new();
                Ok(rules::render_optional_absent_value(
                    "Lean",
                    policy(&expr.ty),
                    absence,
                    &ty,
                ))
            }
            Node::Some { value } => Ok(rules::render_optional_present_value(
                "Lean",
                policy(&expr.ty),
                &self.expr(value, depth)?,
            )),
            Node::FlattenOption { value } => Ok(rules::render_optional_flattening(
                "Lean",
                policy(&value.ty),
                policy(element(&value.ty)),
                policy(&expr.ty),
                &self.expr(value, depth)?,
            )),
            Node::Coalesce { left, right } => Ok(rules::render_optional_coalescing(
                "Lean",
                policy(&left.ty),
                &self.expr(left, depth)?,
                "",
                &self.expr(right, depth)?,
            )),
            Node::OptionalField { object, .. } | Node::OptionalProject { object, .. } => {
                let (binder, present) = match &expr.node {
                    Node::OptionalField { index, .. } => {
                        let Type::Record { fields, .. } = element(&object.ty) else {
                            unreachable!(
                                "a selected optional field retains its actual record layout"
                            );
                        };
                        (
                            "ml_record",
                            crate::translation::record_emission_rules::render_record_product_field(
                                "Lean",
                                "ml_record",
                                number(*index),
                                number(fields.len()),
                            ),
                        )
                    }
                    _ => {
                        let source = Expr::var("ml_value", element(&object.ty).clone());
                        let length = Expr::new(
                            Node::Length {
                                array: Box::new(source),
                            },
                            Type::Float,
                        );
                        ("ml_value", self.expr(&length, depth)?)
                    }
                };
                Ok(rules::render_optional_projection(
                    "Lean",
                    policy(&object.ty),
                    &self.expr(object, depth)?,
                    "",
                    binder,
                    &present,
                    "",
                ))
            }
            Node::NullishNumber { arg } | Node::OptionalIndexNumber { arg } => {
                let operation = if matches!(expr.node, Node::OptionalIndexNumber { .. }) {
                    "array-index"
                } else {
                    "arithmetic"
                };
                let present = rules::render_optional_numeric_present(
                    "Lean",
                    element(&arg.ty).kind(),
                    operation,
                );
                self.optional_match(
                    arg,
                    "ml_present",
                    &present,
                    &render_absent_number_expression("Lean", operation, "null"),
                    &render_absent_number_expression("Lean", operation, "undefined"),
                    depth,
                )
            }
            Node::Truthy { arg } => self.optional_truth(arg, depth),
            Node::StrictEqual {
                left,
                right,
                negate,
            } => {
                let first = element(&left.ty);
                let second = element(&right.ty);
                let compare = if first == second {
                    "(ml_left == ml_right)".to_owned()
                } else {
                    "false".to_owned()
                };
                Ok(rules::render_optional_strict_equality(
                    "Lean",
                    left.ty.kind(),
                    policy(&left.ty),
                    right.ty.kind(),
                    policy(&right.ty),
                    &compare,
                    &self.expr(left, depth)?,
                    &self.expr(right, depth)?,
                    *negate,
                ))
            }
            Node::OptionalText { arg, console } => {
                let element = element(&arg.ty);
                let present = if *element == Type::Unit {
                    text_literal("undefined")?
                } else if *element == Type::String {
                    "ml_present".to_owned()
                } else {
                    self.expr(
                        &Expr::new(
                            Node::ToString {
                                arg: Box::new(Expr::var("ml_present", element.clone())),
                                console: *console,
                            },
                            Type::String,
                        ),
                        depth,
                    )?
                };
                let present = rules::render_optional_console_text(
                    "Lean",
                    &present,
                    element.is_numeric(),
                    *console,
                    false,
                );
                let null_case = text_literal("null")?;
                let undefined_case = text_literal("undefined")?;
                self.optional_match(
                    arg,
                    "ml_present",
                    &present,
                    &null_case,
                    &undefined_case,
                    depth,
                )
            }
            Node::OptionalTypeOf { arg, present } => {
                let literal = text_literal(present)?;
                let present = rules::render_optional_type_query_present("Lean", &literal);
                let null_case = text_literal("object")?;
                let undefined_case = text_literal("undefined")?;
                self.optional_match(
                    arg,
                    "ml_present",
                    &present,
                    &null_case,
                    &undefined_case,
                    depth,
                )
            }
            Node::UnwrapOption { arg } => {
                let fallback = String::new();
                Ok(rules::render_optional_unwrapping(
                    "Lean",
                    policy(&arg.ty),
                    &self.expr(arg, depth)?,
                    "",
                    &fallback,
                ))
            }
            Node::OptionalPredicate { arg, op } => {
                let present = if rules::require_optional_numeric_predicate(element(&arg.ty).kind())
                {
                    self.math(
                        &Expr::new(
                            Node::Math {
                                op: op.clone(),
                                args: vec![Expr::var("ml_present", Type::Float)],
                            },
                            Type::Bool,
                        ),
                        depth,
                    )?
                } else {
                    "false".to_owned()
                };
                self.optional_match(arg, "ml_present", &present, "false", "false", depth)
            }
            _ => unreachable!("the optional renderer receives only actual optional nodes"),
        }
    }
}
