//! `Math` and `Number` functions, whose arguments are Numbers.

use super::{
    BOOL, Checker, Env, Expr, FLOAT, Node, Result, SArrayItem, Span, array, type_error, unsupported,
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
                let arg = self.expr(&item.value, env, path, Some(&FLOAT), false)?;
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
        Ok(Expr::new(
            Node::Math {
                op: op.to_owned(),
                args,
            },
            ty,
        ))
    }
}
