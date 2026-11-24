//! Apply the complete checked optional syntax to actual target operands.
use super::{BinaryOp, Expr, LitValue, Node, Result, RocqEmitter, Type};
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

impl RocqEmitter<'_> {
    fn optional_match(
        &mut self,
        arg: &Expr,
        binder: &str,
        present: &str,
        null_case: &str,
        undefined_case: &str,
    ) -> Result<String> {
        Ok(rules::render_optional_match(
            "Rocq",
            policy(&arg.ty),
            &self.expr(arg)?,
            binder,
            present,
            null_case,
            undefined_case,
        ))
    }

    fn optional_truth(&mut self, arg: &Expr) -> Result<String> {
        if let Type::Option { element, .. } = &arg.ty {
            let present =
                self.optional_truth(&Expr::var("ml_present", element.as_ref().clone()))?;
            return self.optional_match(arg, "ml_present", &present, "false", "false");
        }
        if arg.ty.is_float() {
            self.floats();
        }
        if self.utf16_strings && arg.ty == Type::String {
            return Ok(crate::translation::formal_unit_strings::truth(
                "Rocq",
                &self.expr(arg)?,
            ));
        }
        Ok(rules::render_optional_primitive_truth(
            "Rocq",
            arg.ty.kind(),
            &self.expr(arg)?,
            "",
            false,
            matches!(arg.ty, Type::Fixed { signed: true, .. }),
        ))
    }

    fn optional_text_literal(&mut self, text: &str) -> Result<String> {
        self.literal(&Type::String, &LitValue::Text(text.to_owned()))
    }

    pub(super) fn optional_expr(&mut self, expr: &Expr) -> Result<String> {
        match &expr.node {
            Node::None { absence } => {
                let ty = self.ty(element(&expr.ty))?;
                Ok(rules::render_optional_absent_value(
                    "Rocq",
                    policy(&expr.ty),
                    absence,
                    &ty,
                ))
            }
            Node::Some { value } => Ok(rules::render_optional_present_value(
                "Rocq",
                policy(&expr.ty),
                &self.expr(value)?,
            )),
            Node::FlattenOption { value } => Ok(rules::render_optional_flattening(
                "Rocq",
                policy(&value.ty),
                policy(element(&value.ty)),
                policy(&expr.ty),
                &self.expr(value)?,
            )),
            Node::Coalesce { left, right } => Ok(rules::render_optional_coalescing(
                "Rocq",
                policy(&left.ty),
                &self.expr(left)?,
                "",
                &self.expr(right)?,
            )),
            Node::OptionalField { object, .. } | Node::OptionalProject { object, .. } => {
                let (binder, present) = if let Node::OptionalField { index, .. } = &expr.node {
                    let Type::Record { fields, .. } = element(&object.ty) else {
                        unreachable!("a selected optional field retains its actual record layout");
                    };
                    (
                        "ml_record",
                        crate::translation::record_emission_rules::render_record_product_field(
                            "Rocq",
                            &if matches!(element(&object.ty), Type::Record { ordered: true, .. }) {
                                crate::translation::ordered_record_layout_rules::render_ordered_record_payload("Rocq","ml_record")
                            } else {
                                "ml_record".to_owned()
                            },
                            number(*index),
                            number(fields.len()),
                        ),
                    )
                } else {
                    let source = Expr::var("ml_value", element(&object.ty).clone());
                    let length = Expr::new(
                        Node::Length {
                            array: Box::new(source),
                        },
                        Type::Float,
                    );
                    ("ml_value", self.expr(&length)?)
                };
                Ok(rules::render_optional_projection(
                    "Rocq",
                    policy(&object.ty),
                    &self.expr(object)?,
                    "",
                    binder,
                    &present,
                    "",
                ))
            }
            Node::NullishNumber { arg } | Node::OptionalIndexNumber { arg } => {
                self.floats();
                let operation = if matches!(expr.node, Node::OptionalIndexNumber { .. }) {
                    "array-index"
                } else {
                    "arithmetic"
                };
                let present = rules::render_optional_numeric_present(
                    "Rocq",
                    element(&arg.ty).kind(),
                    operation,
                );
                self.optional_match(
                    arg,
                    "ml_present",
                    &present,
                    &render_absent_number_expression("Rocq", operation, "null"),
                    &render_absent_number_expression("Rocq", operation, "undefined"),
                )
            }
            Node::Truthy { arg } => self.optional_truth(arg),
            Node::StrictEqual {
                left,
                right,
                negate,
            } => {
                let first = element(&left.ty);
                let second = element(&right.ty);
                let compare = if first == second {
                    if *first == Type::Unit {
                        "true".to_owned()
                    } else {
                        self.comparison(BinaryOp::Eq, first, "ml_left", "ml_right")?
                    }
                } else {
                    "false".to_owned()
                };
                Ok(rules::render_optional_strict_equality(
                    "Rocq",
                    left.ty.kind(),
                    policy(&left.ty),
                    right.ty.kind(),
                    policy(&right.ty),
                    &compare,
                    &self.expr(left)?,
                    &self.expr(right)?,
                    *negate,
                ))
            }
            Node::OptionalText { arg, console } => {
                let element = element(&arg.ty);
                let present = if *element == Type::Unit {
                    self.optional_text_literal("undefined")?
                } else if *element == Type::String {
                    "ml_present".to_owned()
                } else {
                    self.expr(&Expr::new(
                        Node::ToString {
                            arg: Box::new(Expr::var("ml_present", element.clone())),
                            console: *console,
                        },
                        Type::String,
                    ))?
                };
                let present = rules::render_optional_console_text(
                    "Rocq",
                    &present,
                    element.is_numeric(),
                    *console,
                    false,
                );
                let null_case = self.optional_text_literal("null")?;
                let undefined_case = self.optional_text_literal("undefined")?;
                self.optional_match(arg, "ml_present", &present, &null_case, &undefined_case)
            }
            Node::OptionalTypeOf { arg, present } => {
                let literal = self.optional_text_literal(present)?;
                let present = rules::render_optional_type_query_present("Rocq", &literal);
                let null_case = self.optional_text_literal("object")?;
                let undefined_case = self.optional_text_literal("undefined")?;
                self.optional_match(arg, "ml_present", &present, &null_case, &undefined_case)
            }
            Node::UnwrapOption { arg } => {
                let fallback = self.inhabitant(&expr.ty)?;
                Ok(rules::render_optional_unwrapping(
                    "Rocq",
                    policy(&arg.ty),
                    &self.expr(arg)?,
                    "",
                    &fallback,
                ))
            }
            Node::OptionalPredicate { arg, op } => {
                let present = if rules::require_optional_numeric_predicate(element(&arg.ty).kind())
                {
                    self.math(&Expr::new(
                        Node::Math {
                            op: op.clone(),
                            args: vec![Expr::var("ml_present", Type::Float)],
                        },
                        Type::Bool,
                    ))?
                } else {
                    "false".to_owned()
                };
                self.optional_match(arg, "ml_present", &present, "false", "false")
            }
            _ => unreachable!("the optional renderer receives only actual optional nodes"),
        }
    }
}
