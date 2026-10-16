//! Optional operations are materialized from the complete checked shared rules.
use super::{Expr, LitValue, Node, Prelude, Result, RustEmitter, Type};
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

impl RustEmitter<'_> {
    fn optional_match(
        &mut self,
        arg: &Expr,
        binder: &str,
        present: &str,
        null_case: &str,
        undefined_case: &str,
    ) -> Result<String> {
        if policy(&arg.ty) == "both" {
            self.preludes.insert(Prelude::Optional);
        }
        Ok(rules::render_optional_match(
            "Rust",
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
        let value = self.receiver(arg)?;
        let evaluated = if matches!(
            arg.ty,
            Type::Int | Type::Nat | Type::Fixed { .. } | Type::String
        ) {
            String::new()
        } else {
            self.expr(arg)?
        };
        Ok(rules::render_optional_primitive_truth(
            "Rust",
            arg.ty.kind(),
            &evaluated,
            &value,
            self.utf16_strings,
            matches!(arg.ty, Type::Fixed { signed: true, .. }),
        ))
    }

    fn optional_text_literal(&mut self, text: &str) -> String {
        let value = LitValue::Text(text.to_owned());
        self.literal(&Expr::lit(Type::String, value.clone()), &value)
    }

    pub(super) fn optional_expr(&mut self, expr: &Expr) -> Result<String> {
        if policy(&expr.ty) == "both" {
            self.preludes.insert(Prelude::Optional);
        }
        match &expr.node {
            Node::None { absence } => {
                let ty = if policy(&expr.ty) == "both" {
                    String::new()
                } else {
                    self.ty(element(&expr.ty))?
                };
                Ok(rules::render_optional_absent_value(
                    "Rust",
                    policy(&expr.ty),
                    absence,
                    &ty,
                ))
            }
            Node::Some { value } => Ok(rules::render_optional_present_value(
                "Rust",
                policy(&expr.ty),
                &self.expr(value)?,
            )),
            Node::FlattenOption { value } => Ok(rules::render_optional_flattening(
                "Rust",
                policy(&value.ty),
                policy(element(&value.ty)),
                policy(&expr.ty),
                &self.expr(value)?,
            )),
            Node::Coalesce { left, right } => {
                if policy(&left.ty) == "both" {
                    self.preludes.insert(Prelude::Optional);
                }
                let value = if policy(&left.ty) == "both" {
                    self.expr(left)?
                } else {
                    String::new()
                };
                let receiver = if policy(&left.ty) == "both" {
                    String::new()
                } else {
                    self.receiver(left)?
                };
                Ok(rules::render_optional_coalescing(
                    "Rust",
                    policy(&left.ty),
                    &value,
                    &receiver,
                    &self.expr(right)?,
                ))
            }
            Node::OptionalField { object, .. } | Node::OptionalProject { object, .. } => {
                if policy(&object.ty) == "both" {
                    self.preludes.insert(Prelude::Optional);
                }
                let (binder, present) = match &expr.node {
                    Node::OptionalField { index, .. } => {
                        ("ml_record", format!("ml_record.{index}"))
                    }
                    _ => {
                        let source = Expr::var("ml_value", element(&object.ty).clone());
                        let length = Expr::new(
                            Node::Length {
                                array: Box::new(source),
                            },
                            Type::Float,
                        );
                        ("ml_value", self.expr(&length)?)
                    }
                };
                let value = if policy(&object.ty) == "both" {
                    self.expr(object)?
                } else {
                    String::new()
                };
                let receiver = if policy(&object.ty) == "both" {
                    String::new()
                } else {
                    self.receiver(object)?
                };
                Ok(rules::render_optional_projection(
                    "Rust",
                    policy(&object.ty),
                    &value,
                    &receiver,
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
                    "Rust",
                    element(&arg.ty).kind(),
                    operation,
                );
                self.optional_match(
                    arg,
                    "ml_present",
                    &present,
                    &render_absent_number_expression("Rust", operation, "null"),
                    &render_absent_number_expression("Rust", operation, "undefined"),
                )
            }
            Node::Truthy { arg } => self.optional_truth(arg),
            Node::StrictEqual {
                left,
                right,
                negate,
            } => {
                if policy(&left.ty) == "both" || policy(&right.ty) == "both" {
                    self.preludes.insert(Prelude::Optional);
                }
                let compare = if element(&left.ty).key() == element(&right.ty).key() {
                    "ml_left == ml_right"
                } else {
                    "false"
                };
                Ok(rules::render_optional_strict_equality(
                    "Rust",
                    left.ty.kind(),
                    policy(&left.ty),
                    right.ty.kind(),
                    policy(&right.ty),
                    compare,
                    &self.expr(left)?,
                    &self.expr(right)?,
                    *negate,
                ))
            }
            Node::OptionalText { arg, console } => {
                let element = element(&arg.ty);
                let present = if *element == Type::Unit {
                    self.optional_text_literal("undefined")
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
                    "Rust",
                    &present,
                    element.is_numeric(),
                    *console,
                    self.utf16_strings,
                );
                let null_case = self.optional_text_literal("null");
                let undefined_case = self.optional_text_literal("undefined");
                self.optional_match(arg, "ml_present", &present, &null_case, &undefined_case)
            }
            Node::OptionalTypeOf { arg, present } => {
                let literal = self.optional_text_literal(present);
                let present = rules::render_optional_type_query_present("Rust", &literal);
                let null_case = self.optional_text_literal("object");
                let undefined_case = self.optional_text_literal("undefined");
                self.optional_match(arg, "ml_present", &present, &null_case, &undefined_case)
            }
            Node::UnwrapOption { arg } => {
                if policy(&arg.ty) == "both" {
                    self.preludes.insert(Prelude::Optional);
                }
                let value = if policy(&arg.ty) == "both" {
                    self.expr(arg)?
                } else {
                    String::new()
                };
                let receiver = if policy(&arg.ty) == "both" {
                    String::new()
                } else {
                    self.receiver(arg)?
                };
                Ok(rules::render_optional_unwrapping(
                    "Rust",
                    policy(&arg.ty),
                    &value,
                    &receiver,
                    "",
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
