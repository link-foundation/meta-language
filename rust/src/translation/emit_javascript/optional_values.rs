//! Optional syntax uses the complete shared value rendering rules.
use super::{Expr, Helper, JavaScriptEmitter, Node, Result, Type, json_string};
use crate::translation::optional_value_rules as rules;

fn policy(ty: &Type) -> &str {
    match ty {
        Type::Option { absence, .. } => absence,
        _ => "",
    }
}

impl JavaScriptEmitter<'_> {
    pub(super) fn optional_expr(&mut self, expr: &Expr) -> Result<String> {
        match &expr.node {
            Node::None { absence } => Ok(rules::render_optional_absent_value(
                "JavaScript",
                policy(&expr.ty),
                absence,
                "",
            )),
            Node::Some { value } => Ok(rules::render_optional_present_value(
                "JavaScript",
                policy(&expr.ty),
                &self.expr(value)?,
            )),
            Node::FlattenOption { value } => {
                let Type::Option { element, .. } = &value.ty else {
                    unreachable!("the checker retains the actual outer optional type");
                };
                Ok(rules::render_optional_flattening(
                    "JavaScript",
                    policy(&value.ty),
                    policy(element),
                    policy(&expr.ty),
                    &self.expr(value)?,
                ))
            }
            Node::Truthy { arg } => Ok(rules::render_optional_primitive_truth(
                "JavaScript",
                arg.ty.kind(),
                &self.expr(arg)?,
                "",
                false,
                false,
            )),
            Node::StrictEqual {
                left,
                right,
                negate,
            } => Ok(rules::render_optional_strict_equality(
                "JavaScript",
                left.ty.kind(),
                policy(&left.ty),
                right.ty.kind(),
                policy(&right.ty),
                "",
                &self.expr(left)?,
                &self.expr(right)?,
                *negate,
            )),
            Node::UnwrapOption { arg } => Ok(rules::render_optional_unwrapping(
                "JavaScript",
                policy(&arg.ty),
                &self.expr(arg)?,
                "",
                "",
            )),
            Node::Coalesce { left, right } => Ok(rules::render_optional_coalescing(
                "JavaScript",
                policy(&left.ty),
                &self.expr(left)?,
                "",
                &self.expr(right)?,
            )),
            Node::OptionalField { object, field, .. } | Node::OptionalProject { object, field } => {
                Ok(rules::render_optional_projection(
                    "JavaScript",
                    policy(&object.ty),
                    &self.expr(object)?,
                    "",
                    "",
                    "",
                    &json_string(field),
                ))
            }
            Node::NullishNumber { arg }
            | Node::OptionalIndexNumber { arg }
            | Node::OptionalTypeOf { arg, .. } => Ok(rules::render_optional_java_script_operation(
                expr.kind(),
                &self.expr(arg)?,
                "",
                false,
            )),
            Node::OptionalText { arg, console } => {
                if *console {
                    self.helpers.insert(Helper::ShowOptional);
                }
                Ok(rules::render_optional_java_script_operation(
                    expr.kind(),
                    &self.expr(arg)?,
                    "",
                    *console,
                ))
            }
            Node::OptionalPredicate { arg, op } => {
                Ok(rules::render_optional_java_script_operation(
                    expr.kind(),
                    &self.expr(arg)?,
                    op,
                    false,
                ))
            }
            _ => unreachable!("only optional checked nodes enter the shared renderer"),
        }
    }
}
