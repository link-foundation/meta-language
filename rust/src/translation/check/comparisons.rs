//! Actual typed comparison construction; shared rules own source decisions.
use super::{
    BOOL, BinaryOp, Checker, Env, Expr, FLOAT, INT, Language, LitValue, Node, Result, SArrayItem,
    Span, Type, UNIT, UnaryOp, binary_node, type_error, unsupported,
};
use crate::translation::javascript_comparison_rules as rules;

fn accepts(ty: &Type) -> bool {
    match ty {
        Type::Option { element, .. } => accepts(element),
        _ => rules::accept_java_script_comparison_primitive(ty.kind()),
    }
}
fn literal(ty: Type, value: &str) -> Expr {
    Expr::lit(ty, LitValue::Text(value.to_owned()))
}
fn strict_value(value: Expr) -> Expr {
    let absence = rules::read_java_script_comparison_unit_absence(value.ty.kind());
    if absence.is_empty() {
        value
    } else {
        Expr::new(
            Node::None {
                absence: absence.clone(),
            },
            crate::translation::types::option(UNIT, &absence),
        )
    }
}

impl Checker {
    pub(super) fn javascript_loose_comparison(
        &mut self,
        operation: &str,
        args: &[SArrayItem],
        span: Option<Span>,
        env: &Env,
        path: &[String],
    ) -> Result<Expr> {
        if self.language != Language::JavaScript
            || args.len() != 2
            || args.iter().any(|argument| argument.spread)
        {
            return Err(type_error(
                "loose equality needs two actual JavaScript operands",
                span,
            ));
        }
        let values = args
            .iter()
            .map(|argument| self.expr(&argument.value, env, path, None, false))
            .collect::<Result<Vec<_>>>()?;
        if values.iter().any(|value| !accepts(&value.ty)) {
            return Err(unsupported(
                "JavaScript loose equality operand",
                "the actual object or symbol needs its complete identity and primitive conversion",
                span,
            ));
        }
        let names = [
            self.fresh_name("ml_comparison_operand"),
            self.fresh_name("ml_comparison_operand"),
        ];
        let mut body = self.primitive_loose_equal(
            Expr::var(names[0].clone(), values[0].ty.clone()),
            Expr::var(names[1].clone(), values[1].ty.clone()),
        )?;
        if rules::require_java_script_loose_equality_negation(operation) {
            body = Expr::new(
                Node::Unary {
                    op: UnaryOp::Not,
                    arg: Box::new(body),
                    semantics: None,
                },
                BOOL,
            );
        }
        for index in (0..2).rev() {
            body = Expr::new(
                Node::Let {
                    name: names[index].clone(),
                    value: Box::new(values[index].clone()),
                    body: Box::new(body),
                },
                BOOL,
            );
        }
        Ok(body)
    }
    fn primitive_loose_equal(&mut self, left: Expr, right: Expr) -> Result<Expr> {
        let first =
            matches!(left.ty, Type::Option { .. }) && !matches!(left.node, Node::None { .. });
        let second =
            matches!(right.ty, Type::Option { .. }) && !matches!(right.node, Node::None { .. });
        if first || second {
            let value = if first { left.clone() } else { right.clone() };
            let Type::Option { element, absence } = &value.ty else {
                unreachable!("actual optional operand")
            };
            let present = Expr::new(
                Node::UnwrapOption {
                    arg: Box::new(value.clone()),
                },
                (**element).clone(),
            );
            let mut body = if first {
                self.primitive_loose_equal(present, right.clone())?
            } else {
                self.primitive_loose_equal(left.clone(), present)?
            };
            for absence in
                crate::translation::optional_value_rules::read_optional_output_absences(absence)
                    .into_iter()
                    .rev()
            {
                let empty = Expr::new(
                    Node::None {
                        absence: absence.clone(),
                    },
                    crate::translation::types::option(UNIT, &absence),
                );
                let condition = Expr::new(
                    Node::StrictEqual {
                        left: Box::new(value.clone()),
                        right: Box::new(Expr::new(Node::None { absence }, value.ty.clone())),
                        negate: false,
                    },
                    BOOL,
                );
                let selected = if first {
                    self.primitive_loose_equal(empty, right.clone())?
                } else {
                    self.primitive_loose_equal(left.clone(), empty)?
                };
                body = Expr::new(
                    Node::If {
                        cond: Box::new(condition),
                        then: Box::new(selected),
                        otherwise: Box::new(body),
                    },
                    BOOL,
                );
            }
            return Ok(body);
        }
        let a = left.ty == UNIT || matches!(left.node, Node::None { .. });
        let b = right.ty == UNIT || matches!(right.node, Node::None { .. });
        if a || b {
            return Ok(Expr::lit(BOOL, LitValue::Bool(a && b)));
        }
        let form =
            rules::read_java_script_primitive_loose_equality_form(left.ty.kind(), right.ty.kind());
        if form == "different" {
            return Ok(Expr::lit(BOOL, LitValue::Bool(false)));
        }
        if form == "strict" || form == "integer" {
            let left = if form == "integer" {
                self.coerce(left, &INT, None)?
            } else {
                left
            };
            let right = if form == "integer" {
                self.coerce(right, &INT, None)?
            } else {
                right
            };
            return Ok(Expr::new(
                Node::StrictEqual {
                    left: Box::new(left),
                    right: Box::new(right),
                    negate: false,
                },
                BOOL,
            ));
        }
        if form == "left-number" {
            let left = self.number_value(left, None)?;
            return self.primitive_loose_equal(left, right);
        }
        if form == "right-number" {
            let right = self.number_value(right, None)?;
            return self.primitive_loose_equal(left, right);
        }
        self.primitive_comparison_order(BinaryOp::Eq, left, right, &form)
    }
    pub(super) fn primitive_comparison(
        &mut self,
        operation: BinaryOp,
        left: &Expr,
        right: &Expr,
    ) -> Result<Option<Expr>> {
        if self.language != Language::JavaScript
            || !accepts(&left.ty)
            || !accepts(&right.ty)
            || !rules::require_java_script_primitive_comparison(left.ty.kind(), right.ty.kind())
        {
            return Ok(None);
        }
        let names = [
            self.fresh_name("ml_comparison_operand"),
            self.fresh_name("ml_comparison_operand"),
        ];
        let mut body = self.primitive_comparison_values(
            operation,
            Expr::var(names[0].clone(), left.ty.clone()),
            Expr::var(names[1].clone(), right.ty.clone()),
        )?;
        for (name, value) in [(names[1].clone(), right), (names[0].clone(), left)] {
            body = Expr::new(
                Node::Let {
                    name,
                    value: Box::new(value.clone()),
                    body: Box::new(body),
                },
                BOOL,
            );
        }
        Ok(Some(body))
    }
    fn primitive_comparison_values(
        &mut self,
        operation: BinaryOp,
        left: Expr,
        right: Expr,
    ) -> Result<Expr> {
        if matches!(operation, BinaryOp::Eq | BinaryOp::Ne) {
            return Ok(Expr::new(
                Node::StrictEqual {
                    left: Box::new(strict_value(left)),
                    right: Box::new(strict_value(right)),
                    negate: operation == BinaryOp::Ne,
                },
                BOOL,
            ));
        }
        if matches!(left.ty, Type::Option { .. }) || matches!(right.ty, Type::Option { .. }) {
            let first = matches!(left.ty, Type::Option { .. });
            let value = if first { left.clone() } else { right.clone() };
            let Type::Option { element, absence } = &value.ty else {
                unreachable!("actual optional operand")
            };
            let present = Expr::new(
                Node::UnwrapOption {
                    arg: Box::new(value.clone()),
                },
                (**element).clone(),
            );
            let mut body = if first {
                self.primitive_comparison_values(operation, present, right.clone())?
            } else {
                self.primitive_comparison_values(operation, left.clone(), present)?
            };
            for absence in
                crate::translation::optional_value_rules::read_optional_output_absences(absence)
                    .into_iter()
                    .rev()
            {
                let empty = literal(
                    FLOAT,
                    &rules::read_java_script_comparison_absent_number(&absence),
                );
                let condition = Expr::new(
                    Node::StrictEqual {
                        left: Box::new(value.clone()),
                        right: Box::new(Expr::new(Node::None { absence }, value.ty.clone())),
                        negate: false,
                    },
                    BOOL,
                );
                let selected = if first {
                    self.primitive_comparison_values(operation, empty, right.clone())?
                } else {
                    self.primitive_comparison_values(operation, left.clone(), empty)?
                };
                body = Expr::new(
                    Node::If {
                        cond: Box::new(condition),
                        then: Box::new(selected),
                        otherwise: Box::new(body),
                    },
                    BOOL,
                );
            }
            return Ok(body);
        }
        let form = rules::read_java_script_primitive_comparison_form(
            operation.name(),
            left.ty.kind(),
            right.ty.kind(),
        );
        if form == "string" {
            return Ok(Expr::new(
                Node::StringTest {
                    op: crate::translation::string_order_rules::read_string_order_operation(
                        "JavaScript",
                        operation.name(),
                        "string",
                    ),
                    string: Box::new(left),
                    search: Box::new(right),
                },
                BOOL,
            ));
        }
        if form == "number" || form == "integer" {
            let domain = if form == "number" { FLOAT } else { INT };
            let left = if form == "integer" {
                self.coerce(left, &INT, None)?
            } else {
                left
            };
            let right = if form == "integer" {
                self.coerce(right, &INT, None)?
            } else {
                right
            };
            return Ok(Expr::new(
                binary_node(
                    operation,
                    Box::new(left),
                    Box::new(right),
                    Some(domain),
                    None,
                    None,
                    None,
                ),
                BOOL,
            ));
        }
        if form == "coerce" {
            let left = self.number_value(left, None)?;
            let right = self.number_value(right, None)?;
            return self.primitive_comparison_values(operation, left, right);
        }
        self.primitive_comparison_order(operation, left, right, &form)
    }
    fn primitive_comparison_order(
        &mut self,
        operation: BinaryOp,
        left: Expr,
        right: Expr,
        form: &str,
    ) -> Result<Expr> {
        let reverse = form == "integer-number" || form == "integer-string";
        let text = form == "integer-string" || form == "string-integer";
        let (first, second) = if reverse {
            (right, left)
        } else {
            (left, right)
        };
        let second = self.coerce(second, &INT, None)?;
        let comparison = Expr::new(
            Node::Math {
                op: if text {
                    "compareJavaScriptStringBigIntegerValue"
                } else {
                    "compareJavaScriptNumberBigIntegerValue"
                }
                .to_owned(),
                args: vec![first, second],
            },
            FLOAT,
        );
        let name = self.fresh_name("ml_comparison_order");
        let order = Expr::var(name.clone(), FLOAT);
        let mut tests =
            rules::read_java_script_comparison_accepted_orders(operation.name(), reverse)
                .into_iter()
                .map(|value| {
                    Expr::new(
                        binary_node(
                            BinaryOp::Eq,
                            Box::new(order.clone()),
                            Box::new(literal(
                                FLOAT,
                                &crate::translation::js_number::js_number(value),
                            )),
                            Some(FLOAT),
                            None,
                            None,
                            None,
                        ),
                        BOOL,
                    )
                });
        let mut body = tests
            .next()
            .expect("actual comparison operation owns an order");
        for next in tests {
            body = Expr::new(
                binary_node(
                    BinaryOp::Or,
                    Box::new(body),
                    Box::new(next),
                    None,
                    None,
                    None,
                    None,
                ),
                BOOL,
            );
        }
        Ok(Expr::new(
            Node::Let {
                name,
                value: Box::new(comparison),
                body: Box::new(body),
            },
            BOOL,
        ))
    }
}
