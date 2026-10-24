//! `Math` and `Number` functions, whose arguments are Numbers.

use super::{
    BOOL, Checker, Env, Expr, FLOAT, LitValue, Node, Result, SArrayItem, Span, Type, array,
    type_error, unsupported,
};

/// The functions whose value is a boolean.
const PREDICATES: [&str; 4] = ["isInteger", "isSafeInteger", "isFinite", "isNaN"];

impl Checker {
    /// `Math.floor(x)`, `Math.max(a, ...xs)`, `Number.isInteger(x)`: the
    /// arguments are Numbers, which none of them converts, and a spread
    /// argument makes `max` and `min` a fold over one array.
    pub(super) fn math(
        &mut self,
        op: &str,
        name: &str,
        args: &[SArrayItem],
        span: Option<Span>,
        env: &Env,
        path: &[String],
    ) -> Result<Expr> {
        let bitwise_arity =
            crate::translation::number_bitwise_rules::read_java_script_number_bitwise_arity(op);
        if bitwise_arity > 0.0
            || crate::translation::big_integer_bitwise_rules::read_big_integer_bitwise_arity(op)
                > 0.0
        {
            return self.check_number_bitwise_operands(op, args, span, env, path);
        }
        let ty = if PREDICATES.contains(&op) {
            BOOL
        } else {
            FLOAT
        };
        let variadic = op == "max" || op == "min";
        if variadic && args.iter().any(|item| item.spread) {
            let values = self.array_literal(args, None, span, env, path, Some(&array(FLOAT)))?;
            return Ok(Expr::new(
                Node::Math {
                    op: format!("{op}Of"),
                    args: vec![values],
                },
                ty,
            ));
        }
        if let Some(spread) = args.iter().find(|item| item.spread) {
            return Err(unsupported(
                "spread argument",
                &format!("pass {name} its argument"),
                spread.value.span.or(span),
            ));
        }
        if !crate::translation::number_predicate_rules::read_number_predicate_argument_form(
            self.language.as_str(),
            op,
            "",
            "",
        )
        .is_empty()
        {
            return self.number_predicate(op, name, args, span, env, path);
        }
        if !variadic && args.len() != 1 {
            return Err(unsupported(
                &format!("{name} with {} arguments", args.len()),
                &format!("{name} takes one Number"),
                span,
            ));
        }
        let args = args
            .iter()
            .map(|item| {
                let mut arg = self.expr(&item.value, env, path, Some(&FLOAT), false)?;
                if matches!(arg.ty, Type::Option { .. }) {
                    if PREDICATES.contains(&op) {
                        return Ok(Expr::new(
                            Node::OptionalPredicate {
                                arg: Box::new(arg),
                                op: op.to_owned(),
                            },
                            BOOL,
                        ));
                    }
                    arg = self.number_value(arg, item.value.span.or(span))?;
                }
                if !arg.ty.is_float() {
                    return Err(type_error(
                        format!(
                            "{name} of {}; it takes Numbers, and converts or rejects anything else",
                            arg.ty.key()
                        ),
                        item.value.span.or(span),
                    ));
                }
                Ok(arg)
            })
            .collect::<Result<Vec<_>>>()?;
        if PREDICATES.contains(&op)
            && let Some(value) = args.first()
            && matches!(value.node, Node::OptionalPredicate { .. })
        {
            return Ok(value.clone());
        }
        Ok(Expr::new(
            Node::Math {
                op: op.to_owned(),
                args,
            },
            ty,
        ))
    }

    fn number_predicate(
        &mut self,
        op: &str,
        name: &str,
        arguments: &[SArrayItem],
        span: Option<Span>,
        env: &Env,
        path: &[String],
    ) -> Result<Expr> {
        let args = arguments
            .iter()
            .map(|item| self.expr(&item.value, env, path, None, false))
            .collect::<Result<Vec<_>>>()?;
        let Some(first) = args.first() else {
            return Ok(Expr::lit(BOOL, LitValue::Bool(false)));
        };
        let element = if let Type::Option { element, .. } = &first.ty {
            element.kind()
        } else {
            ""
        };
        let form = crate::translation::number_predicate_rules::read_number_predicate_argument_form(
            self.language.as_str(),
            op,
            first.ty.kind(),
            element,
        );
        if form == "unsupported" {
            return Err(type_error(
                format!(
                    "{name} of {}; it takes Numbers, and converts or rejects anything else",
                    first.ty.key()
                ),
                arguments[0].value.span.or(span),
            ));
        }
        if args.len() == 1 && form == "number" {
            return Ok(Expr::new(
                Node::Math {
                    op: op.to_owned(),
                    args,
                },
                BOOL,
            ));
        }
        if args.len() == 1 && form == "optional-number" {
            return Ok(Expr::new(
                Node::OptionalPredicate {
                    op: op.to_owned(),
                    arg: Box::new(first.clone()),
                },
                BOOL,
            ));
        }
        let binding = self.fresh_name("ml_number_predicate");
        let bound = Expr::var(binding.clone(), first.ty.clone());
        let mut body = match form.as_str() {
            "number" => Expr::new(
                Node::Math {
                    op: op.to_owned(),
                    args: vec![bound],
                },
                BOOL,
            ),
            "optional-number" => Expr::new(
                Node::OptionalPredicate {
                    op: op.to_owned(),
                    arg: Box::new(bound),
                },
                BOOL,
            ),
            _ => Expr::lit(BOOL, LitValue::Bool(false)),
        };
        for (index, value) in args.into_iter().enumerate().rev() {
            let name = if index == 0 {
                binding.clone()
            } else {
                self.fresh_name("ml_number_predicate_unused")
            };
            body = Expr::new(
                Node::Let {
                    name,
                    value: Box::new(value),
                    body: Box::new(body),
                },
                BOOL,
            );
        }
        Ok(body)
    }
}
