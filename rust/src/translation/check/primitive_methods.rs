//! Checked node construction from the complete generated primitive method rules.
use super::{
    BOOL, Checker, Env, Expr, FLOAT, LitValue, Node, Result, SExpr, STRING, Span, Type, array,
    unsupported,
};
use crate::translation::frontend_rules::ml_array::number_index;
use crate::translation::primitive_method_rules as rules;

impl Checker {
    pub(super) fn copy_slice(
        &mut self,
        object: &SExpr,
        env: &Env,
        path: &[String],
        span: Option<Span>,
    ) -> Result<Expr> {
        let value = self.expr(object, env, path, None, false)?;
        match rules::read_primitive_copy_slice_form(value.ty.kind()).as_str() {
            "identity" => Ok(value),
            "method" => {
                let value = self.reference_value(value);
                self.checked_record_method_application(value, "slice", &[], env, path, span)
            }
            _ => Err(unsupported(
                "method call .slice()",
                "this value has no string or array slice method",
                span,
            )),
        }
    }

    pub(super) fn primitive_method(
        &mut self,
        object: &SExpr,
        operation: &str,
        args: &[Option<SExpr>],
        env: &Env,
        path: &[String],
        span: Option<Span>,
    ) -> Result<Expr> {
        let object = self.expr(object, env, path, None, false)?;
        let object = self.reference_value(object);
        if matches!(object.ty, Type::Record { .. }) {
            return self
                .checked_record_method_application(object, operation, args, env, path, span);
        }
        let receiver = object.ty.kind();
        let minimum = rules::read_primitive_method_minimum_arguments(receiver, operation);
        let maximum = rules::read_primitive_method_maximum_arguments(receiver, operation);
        if minimum < 0.0 {
            let reason = if receiver == "string" {
                format!(".{operation}() has no checked string operation")
            } else {
                format!(
                    ".{operation}() has no checked operation on {}",
                    object.ty.key()
                )
            };
            return Err(unsupported(
                &format!("method call .{operation}()"),
                &reason,
                span,
            ));
        }
        let minimum = number_index(minimum)
            .expect("the generated supported method has an integral minimum arity");
        let maximum = number_index(maximum)
            .expect("the generated supported method has an integral maximum arity");
        if args.len() < minimum || args.len() > maximum {
            let count = if minimum == maximum {
                minimum.to_string()
            } else {
                format!("{minimum} to {maximum}")
            };
            return Err(unsupported(
                &format!("method call .{operation}()"),
                &format!(".{operation}() expects {count} arguments"),
                span,
            ));
        }
        let element = if let Type::Array { element, .. } = &object.ty {
            Some(element.as_ref())
        } else {
            None
        };
        let base = element.map(|element| {
            if let Type::Option { element, .. } = element {
                element.as_ref()
            } else {
                element
            }
        });
        let obligation = rules::read_primitive_method_argument_obligation(
            receiver,
            operation,
            base.map_or("", Type::kind),
        );
        if !obligation.is_empty() {
            return Err(unsupported("array identity search", &obligation, span));
        }
        let count = number_index(rules::read_primitive_method_argument_count(
            receiver, operation,
        ))
        .expect("the generated supported method has an integral bounded arity");
        let mut values = Vec::with_capacity(count);
        for index in 0..count {
            #[allow(clippy::cast_precision_loss)]
            let position = index as f64;
            let kind =
                rules::read_primitive_method_checked_argument_kind(receiver, operation, position);
            let ty = match kind.as_str() {
                "float" => FLOAT,
                "element" => element
                    .expect("the generated array element position")
                    .clone(),
                _ => STRING,
            };
            if let Some(Some(argument)) = args.get(index) {
                let value = self.expr(argument, env, path, Some(&ty), false)?;
                values.push(Some(self.coerce(value, &ty, argument.span.or(span))?));
            } else {
                let form = rules::read_primitive_method_default_form(receiver, operation, position);
                if form == "absent" || form == "required" {
                    values.push(None);
                } else {
                    let ty = if form == "float" { FLOAT } else { STRING };
                    values.push(Some(Expr::new(
                        Node::Lit {
                            value: LitValue::Text(rules::read_primitive_method_default_value(
                                receiver, operation, position,
                            )),
                        },
                        ty,
                    )));
                }
            }
        }
        let ty =
            match rules::read_primitive_method_checked_result_form(receiver, operation).as_str() {
                "receiver" => object.ty.clone(),
                "bool" => BOOL,
                "float" => FLOAT,
                "array-string" => array(STRING),
                "optional-float" => Type::Option {
                    element: Box::new(FLOAT),
                    absence: "undefined".to_owned(),
                },
                _ => STRING,
            };
        Ok(Expr::new(
            Node::Method {
                object: Box::new(object),
                op: operation.to_owned(),
                args: values,
            },
            ty,
        ))
    }
}
