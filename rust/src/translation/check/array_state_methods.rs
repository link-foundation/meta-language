//! Actual checked operands for source-generated dense shared-array operations.
use super::{Checker, Env, Expr, Node, Result, SExpr, Span, Type, unsupported};
use crate::translation::array_state_method_rules as rules;

impl Checker {
    pub(super) fn check_array_state_method(
        &mut self,
        object: Expr,
        operation: &str,
        arguments: &[Option<SExpr>],
        env: &Env,
        path: &[String],
        span: Option<Span>,
    ) -> Result<Expr> {
        let count = serde_json::Number::from(arguments.len())
            .as_f64()
            .expect("actual arity is a Number");
        if count < rules::read_array_state_method_required_arguments(operation) {
            return Err(unsupported(
                "shared array range arguments",
                "the missing value needs an explicit undefined element domain",
                span,
            ));
        }
        let Type::Array { element, .. } = &object.ty else {
            unreachable!("the actual shared array owner")
        };
        let element = element.as_ref().clone();
        let mut args = Vec::new();
        for (index, argument) in arguments.iter().enumerate() {
            let argument = argument.as_ref().ok_or_else(|| {
                unsupported(
                    "shared array range argument",
                    "each actual source argument needs its expression",
                    span,
                )
            })?;
            let role = rules::read_array_state_method_argument_role(
                operation,
                serde_json::Number::from(index)
                    .as_f64()
                    .expect("an actual position is a Number"),
            );
            let value = self.expr(
                argument,
                env,
                path,
                if role == "element" {
                    Some(&element)
                } else {
                    None
                },
                false,
            )?;
            let value = if role == "element" {
                self.coerce(value, &element, argument.span.or(span))?
            } else {
                let (element, absence) = if let Type::Option { element, absence } = &value.ty {
                    (element.kind(), absence.as_str())
                } else {
                    ("", "")
                };
                if !role.is_empty()
                    && rules::read_array_state_method_argument_profile(
                        value.ty.kind(),
                        element,
                        absence,
                    )
                    .is_empty()
                {
                    return Err(unsupported(
                        "shared array range index",
                        "this index needs its complete primitive conversion and error domain",
                        argument.span.or(span),
                    ));
                }
                value
            };
            args.push(Some(value));
        }
        let ty = object.ty.clone();
        Ok(Expr::new(
            Node::Method {
                object: Box::new(object),
                op: operation.to_owned(),
                args,
            },
            ty,
        ))
    }
}
