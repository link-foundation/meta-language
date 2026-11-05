//! Checked operand construction for the complete shared reference method rules.
use super::{BOOL, Checker, Env, Expr, FLOAT, Node, Result, SExpr, Span, Type, UNIT, unsupported};
use crate::translation::reference_method_rules as rules;

impl Checker {
    pub(super) fn check_reference_method(
        &mut self,
        object: Expr,
        operation: &str,
        arguments: &[Option<SExpr>],
        env: &Env,
        path: &[String],
        span: Option<Span>,
    ) -> Result<Expr> {
        let kind = object.ty.kind();
        let result = rules::read_reference_method_result(kind, operation);
        let key = match &object.ty {
            Type::Map { key, .. }
            | Type::Set { element: key, .. }
            | Type::Array { element: key, .. } => key.as_ref().clone(),
            _ => {
                return Err(unsupported(
                    "shared collection receiver",
                    "the actual receiver needs a supported reference type",
                    span,
                ));
            }
        };
        if (kind == "map" || kind == "set") && !rules::accept_reference_method_key_kind(key.kind())
        {
            return Err(unsupported(
                "shared collection key",
                "this key needs its explicit SameValueZero and identity encoding",
                span,
            ));
        }
        let count = serde_json::Number::from(arguments.len())
            .as_f64()
            .expect("an actual argument count is a Number");
        if count < rules::read_reference_method_required_arguments(kind, operation) {
            return Err(unsupported(
                "shared collection method arguments",
                "missing arguments need explicit undefined key and value domains",
                span,
            ));
        }
        let element = match &object.ty {
            Type::Map { value, .. } => value.as_ref().clone(),
            _ => key.clone(),
        };
        let mut args = Vec::new();
        for (index, argument) in arguments.iter().enumerate() {
            let argument = argument.as_ref().ok_or_else(|| {
                unsupported(
                    "shared collection method argument",
                    "every source argument needs its actual expression",
                    span,
                )
            })?;
            let role = rules::read_reference_method_argument_role(
                kind,
                operation,
                serde_json::Number::from(index)
                    .as_f64()
                    .expect("an argument position is a Number"),
            );
            let expected = match role.as_str() {
                "key" => Some(&key),
                "value" | "element" => Some(&element),
                _ => None,
            };
            let value = self.expr(argument, env, path, expected, false)?;
            args.push(Some(if let Some(expected) = expected {
                self.coerce(value, expected, argument.span.or(span))?
            } else {
                value
            }));
        }
        let ty = match result.as_str() {
            "receiver" => object.ty.clone(),
            "length" => FLOAT,
            "boolean" => BOOL,
            "unit" => UNIT,
            "optional element" | "optional value" => {
                crate::translation::types::option(element, "undefined".to_owned())
            }
            _ => {
                return Err(unsupported(
                    "shared collection method",
                    "the selected operation needs an actual shared result rule",
                    span,
                ));
            }
        };
        Ok(super::optional_values::flatten_option(Expr::new(
            Node::Method {
                object: Box::new(object),
                op: operation.to_owned(),
                args,
            },
            ty,
        )))
    }
}
