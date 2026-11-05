//! Actual collection operand owners for the shared immutable access rules.
use super::{BOOL, Checker, Env, Expr, Node, Result, SExpr, Span, Type, array, unsupported};
use crate::translation::collection_access_rules as rules;

impl Checker {
    pub(super) fn checked_collection_access(
        &mut self,
        object: Expr,
        operation: &str,
        arguments: &[Option<SExpr>],
        env: &Env,
        path: &[String],
        span: Option<Span>,
    ) -> Result<Expr> {
        let result = rules::read_collection_access_result(object.ty.kind(), operation);
        let (key, value) = match &object.ty {
            Type::Map { key, value, .. } => (key.as_ref().clone(), value.as_ref().clone()),
            Type::Set { element, .. } => (element.as_ref().clone(), element.as_ref().clone()),
            _ => {
                return Err(unsupported(
                    "collection receiver",
                    "an actual map or set is required",
                    span,
                ));
            }
        };
        if result.is_empty() {
            return Err(unsupported(
                &format!("mutable collection .{operation}()"),
                "mutation requires shared reference identities and cells so aliases observe every change",
                span,
            ));
        }
        let count = rules::read_collection_access_argument_count(operation);
        if serde_json::Number::from(arguments.len()).as_f64() != Some(count) {
            return Err(unsupported(
                &format!("method call .{operation}()"),
                &format!(".{operation}() expects {count} arguments"),
                span,
            ));
        }
        if result == "boolean" || result == "optional value" {
            let argument = arguments.first().and_then(Option::as_ref).ok_or_else(|| {
                unsupported(
                    "collection key",
                    "the lookup needs its actual key expression",
                    span,
                )
            })?;
            let checked = self.expr(argument, env, path, Some(&key), false)?;
            let key = self.coerce(checked, &key, argument.span.or(span))?;
            let ty = if result == "boolean" {
                BOOL
            } else {
                crate::translation::types::option(value, "undefined")
            };
            return Ok(super::optional_values::flatten_option(Expr::new(
                Node::CollectionRead {
                    object: Box::new(object),
                    op: operation.to_owned(),
                    key: Box::new(key),
                },
                ty,
            )));
        }
        let element = match result.as_str() {
            "keys" => key,
            "values" => value,
            _ => Type::Tuple {
                elements: vec![key, value],
            },
        };
        Ok(Expr::new(
            Node::CollectionItems {
                object: Box::new(object),
                op: operation.to_owned(),
            },
            array(element),
        ))
    }
}
