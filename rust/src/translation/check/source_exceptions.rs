//! Actual typed expression adapters for shared JavaScript exception operations.
use super::{
    BOOL, Checker, Env, Expr, Language, LitValue, Node, Result, SArrayItem, STRING, Span, Type,
    type_error, unsupported,
};
use crate::translation::source_exception_rules as rules;

fn accepts_options(ty: &Type) -> bool {
    if let Type::Option { element, .. } = ty {
        return accepts_options(element);
    }
    matches!(
        ty.kind(),
        "string" | "bool" | "float" | "int" | "nat" | "unit"
    )
}

impl Checker {
    fn exception_message(&mut self, value: Expr, span: Option<Span>) -> Result<Expr> {
        match rules::read_java_script_source_exception_message_form(value.ty.kind()).as_str() {
            "identity" => Ok(value),
            "text" => Ok(Expr::new(
                Node::ToString {
                    arg: Box::new(value),
                    console: false,
                },
                STRING,
            )),
            "empty" => {
                let name = self.fresh_name("ml_exception_message_unused");
                Ok(Expr::new(
                    Node::Let {
                        name,
                        value: Box::new(value),
                        body: Box::new(Expr::lit(STRING, LitValue::Text(String::new()))),
                    },
                    STRING,
                ))
            }
            "optional" => {
                let Type::Option { element, absence } = &value.ty else {
                    unreachable!("actual optional message")
                };
                let element = (**element).clone();
                let absences =
                    crate::translation::optional_value_rules::read_optional_output_absences(
                        absence,
                    );
                let name = self.fresh_name("ml_exception_message");
                let variable = Expr::var(name.clone(), value.ty.clone());
                let mut body = self.exception_message(
                    Expr::new(
                        Node::UnwrapOption {
                            arg: Box::new(variable.clone()),
                        },
                        element,
                    ),
                    span,
                )?;
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
                    let message = Expr::lit(
                        STRING,
                        LitValue::Text(rules::read_java_script_source_exception_absent_message(
                            &absence,
                        )),
                    );
                    body = Expr::new(
                        Node::If {
                            cond: Box::new(condition),
                            then: Box::new(message),
                            otherwise: Box::new(body),
                        },
                        STRING,
                    );
                }
                Ok(Expr::new(
                    Node::Let {
                        name,
                        value: Box::new(value),
                        body: Box::new(body),
                    },
                    STRING,
                ))
            }
            _ => Err(unsupported(
                "exception message primitive conversion",
                "the actual object or symbol conversion needs its complete source operation",
                span,
            )),
        }
    }

    pub(super) fn javascript_source_exception(
        &mut self,
        operation: &str,
        arguments: &[SArrayItem],
        span: Option<Span>,
        env: &Env,
        path: &[String],
        expected: Option<&Type>,
    ) -> Result<Expr> {
        if self.language != Language::JavaScript {
            return Err(unsupported(
                "JavaScript exception operation",
                "the operation needs its actual JavaScript source owner",
                span,
            ));
        }
        let ty = expected.cloned().unwrap_or(Type::Unit);
        if arguments.iter().any(|item| item.spread) {
            return Err(unsupported(
                "spread exception arguments",
                "the source iterator and complete argument effects need their represented operations",
                span,
            ));
        }
        let values = arguments
            .iter()
            .map(|item| self.expr(&item.value, env, path, None, false))
            .collect::<Result<Vec<_>>>()?;
        if values.len() > 1 && !accepts_options(&values[1].ty) {
            return Err(unsupported(
                "exception cause options",
                "the options object and cause property need their actual property effects",
                span,
            ));
        }
        let capture = values.len() > 1
            || values
                .first()
                .is_some_and(|value| matches!(value.ty, Type::Option { .. }));
        let names = if capture {
            values
                .iter()
                .map(|_| self.fresh_name("ml_exception_argument"))
                .collect::<Vec<_>>()
        } else {
            Vec::new()
        };
        let first = values.first().map(|value| {
            if capture {
                Expr::var(names[0].clone(), value.ty.clone())
            } else {
                value.clone()
            }
        });
        let present = if let Some(value) = first.as_ref() {
            if let Type::Option { absence, .. } = &value.ty {
                if absence != "null" {
                    Expr::new(
                        Node::StrictEqual {
                            left: Box::new(value.clone()),
                            right: Box::new(Expr::new(
                                Node::None {
                                    absence: "undefined".to_owned(),
                                },
                                value.ty.clone(),
                            )),
                            negate: true,
                        },
                        BOOL,
                    )
                } else {
                    Expr::lit(BOOL, LitValue::Bool(true))
                }
            } else {
                Expr::lit(BOOL, LitValue::Bool(value.ty != Type::Unit))
            }
        } else {
            Expr::lit(BOOL, LitValue::Bool(false))
        };
        let message = if let Some(first) = first {
            self.exception_message(first, span)?
        } else {
            Expr::lit(STRING, LitValue::Text(String::new()))
        };
        let mut body = Expr::new(
            Node::Math {
                op: rules::read_java_script_source_exception_operation(
                    &rules::read_java_script_exception_constructor_class(operation),
                ),
                args: vec![message, present],
            },
            ty.clone(),
        );
        if capture {
            for (index, value) in values.into_iter().enumerate().rev() {
                body = Expr::new(
                    Node::Let {
                        name: names[index].clone(),
                        value: Box::new(value),
                        body: Box::new(body),
                    },
                    ty.clone(),
                );
            }
        }
        Ok(body)
    }

    pub(super) fn javascript_source_exception_value(
        &mut self,
        operation: &str,
        arguments: &[SArrayItem],
        span: Option<Span>,
        env: &Env,
        path: &[String],
        expected: Option<&Type>,
    ) -> Result<Expr> {
        if self.language != Language::JavaScript
            || arguments.len() != 2
            || arguments.iter().any(|item| item.spread)
        {
            return Err(type_error(
                "a source exception needs its checked message and property presence",
                span,
            ));
        }
        let args = arguments
            .iter()
            .map(|item| self.expr(&item.value, env, path, None, false))
            .collect::<Result<Vec<_>>>()?;
        if args[0].ty != STRING || args[1].ty != BOOL {
            return Err(type_error(
                "a source exception needs a String message and Boolean property presence",
                span,
            ));
        }
        Ok(Expr::new(
            Node::Math {
                op: operation.to_owned(),
                args,
            },
            expected.cloned().unwrap_or(Type::Unit),
        ))
    }
}
