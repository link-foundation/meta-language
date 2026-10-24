//! Mechanical materialization of shared primitive numeric conversion decisions.
use super::{
    BOOL, Checker, Env, Expr, FLOAT, Language, LitValue, Node, Result, SArrayItem, Span, Type,
    type_error, unsupported,
};
use crate::translation::string_number_rules as rules;

fn accepts(ty: &Type) -> bool {
    match rules::read_java_script_number_conversion_form(ty.kind()).as_str() {
        "option" => matches!(ty, Type::Option { element, .. } if accepts(element)),
        "unsupported" => false,
        _ => true,
    }
}

impl Checker {
    fn materialize_number_conversion(&mut self, value: Expr, operation: &str) -> Expr {
        let Type::Option {
            element,
            absence: policy,
        } = &value.ty
        else {
            return Expr::new(
                Node::Math {
                    op: operation.to_owned(),
                    args: vec![value],
                },
                FLOAT,
            );
        };
        let element = (**element).clone();
        let absences =
            crate::translation::optional_value_rules::read_optional_output_absences(policy);
        let name = self.fresh_name("ml_numeric_optional");
        let variable = Expr::var(name.clone(), value.ty.clone());
        let mut body = self.materialize_number_conversion(
            Expr::new(
                Node::UnwrapOption {
                    arg: Box::new(variable.clone()),
                },
                element,
            ),
            operation,
        );
        for absence in absences.into_iter().rev() {
            let empty = Expr::new(
                Node::None {
                    absence: absence.clone(),
                },
                value.ty.clone(),
            );
            let condition = Expr::new(
                Node::StrictEqual {
                    left: Box::new(variable.clone()),
                    right: Box::new(empty),
                    negate: false,
                },
                BOOL,
            );
            let number = Expr::lit(
                FLOAT,
                LitValue::Text(if absence == "null" { "0" } else { "NaN" }.to_owned()),
            );
            body = Expr::new(
                Node::If {
                    cond: Box::new(condition),
                    then: Box::new(number),
                    otherwise: Box::new(body),
                },
                FLOAT,
            );
        }
        Expr::new(
            Node::Let {
                name,
                value: Box::new(value),
                body: Box::new(body),
            },
            FLOAT,
        )
    }

    pub(super) fn javascript_number_builtin(
        &mut self,
        operation: &str,
        args: &[SArrayItem],
        span: Option<Span>,
        env: &Env,
        path: &[String],
    ) -> Result<Expr> {
        if self.language != Language::JavaScript {
            return Err(unsupported(
                "JavaScript Number conversion",
                "the operation needs its actual JavaScript source owner",
                span,
            ));
        }
        if args.iter().any(|item| item.spread) {
            return Err(unsupported(
                "spread numeric conversion arguments",
                "the actual spread iterator and argument evaluations need to be represented",
                span,
            ));
        }
        if operation == "coerceJavaScriptNumber" && args.len() != 1 {
            return Err(type_error(
                "ToNumber needs exactly one represented operand",
                span,
            ));
        }
        let values = args
            .iter()
            .map(|item| self.expr(&item.value, env, path, None, false))
            .collect::<Result<Vec<_>>>()?;
        let coercing = operation != "convertJavaScriptNumber";
        let binding = if values.len() > 1 {
            self.fresh_name("ml_numeric_argument")
        } else {
            String::new()
        };
        let first = values.first().map(|value| {
            if values.len() > 1 {
                Expr::var(binding.clone(), value.ty.clone())
            } else {
                value.clone()
            }
        });
        let mut body = if let Some(first) = first {
            if !accepts(&first.ty) {
                return Err(unsupported(
                    "numeric conversion operand",
                    "the actual object primitive conversion, union or symbol must have its own represented behavior",
                    span,
                ));
            }
            self.materialize_number_conversion(
                first,
                if coercing {
                    "coerceJavaScriptNumber"
                } else {
                    "convertJavaScriptNumber"
                },
            )
        } else {
            Expr::lit(
                FLOAT,
                LitValue::Text(if coercing { "NaN" } else { "0" }.to_owned()),
            )
        };
        let predicate = match operation {
            "testCoercedJavaScriptNumberFinite" => "isFinite",
            "testCoercedJavaScriptNumberNaN" => "isNaN",
            _ => "",
        };
        if !predicate.is_empty() {
            body = Expr::new(
                Node::Math {
                    op: predicate.to_owned(),
                    args: vec![body],
                },
                BOOL,
            );
        }
        if values.len() > 1 {
            for (index, value) in values.into_iter().enumerate().rev() {
                let name = if index == 0 {
                    binding.clone()
                } else {
                    self.fresh_name("ml_numeric_argument_unused")
                };
                let ty = body.ty.clone();
                body = Expr::new(
                    Node::Let {
                        name,
                        value: Box::new(value),
                        body: Box::new(body),
                    },
                    ty,
                );
            }
        }
        Ok(body)
    }
}
