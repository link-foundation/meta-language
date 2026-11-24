//! Actual operands/type transport into the shared source unit-call decisions.
use super::{
    BOOL, BinaryOp, Checker, Env, Expr, FLOAT, LitValue, Node, Result, SExpr, STRING, Span, Type,
    array, binary_node, type_error, unsupported,
};
use crate::translation::javascript_property_call_rules as rules;
fn number(value: &str) -> Expr {
    Expr::lit(FLOAT, LitValue::Text(value.to_owned()))
}
fn expansion_predicate(plan: Vec<String>, count: &Expr, receiver: &Expr) -> Expr {
    let mut stack = Vec::new();
    for instruction in plan {
        match instruction.as_str() {
            "count" => stack.push(count.clone()),
            "length" => stack.push(Expr::new(
                Node::Length {
                    array: Box::new(receiver.clone()),
                },
                FLOAT,
            )),
            "true" => stack.push(Expr::lit(BOOL, LitValue::Bool(true))),
            "zero" => stack.push(number("0")),
            "infinity" => stack.push(number("Infinity")),
            "isNaN" | "trunc" => {
                let value = stack.pop().expect("shared padding unary operand");
                stack.push(Expr::new(
                    Node::Math {
                        op: instruction.clone(),
                        args: vec![value],
                    },
                    if instruction == "isNaN" { BOOL } else { FLOAT },
                ));
            }
            "le" | "lt" | "eq" => {
                let right = stack.pop().expect("shared padding right operand");
                let left = stack.pop().expect("shared padding left operand");
                stack.push(Expr::new(
                    binary_node(
                        match instruction.as_str() {
                            "le" => BinaryOp::Le,
                            "lt" => BinaryOp::Lt,
                            _ => BinaryOp::Eq,
                        },
                        Box::new(left),
                        Box::new(right),
                        Some(FLOAT),
                        None,
                        None,
                        None,
                    ),
                    BOOL,
                ));
            }
            "or" => {
                let right = stack.pop().expect("shared expansion right operand");
                let left = stack.pop().expect("shared expansion left operand");
                stack.push(Expr::new(
                    Node::If {
                        cond: Box::new(left),
                        then: Box::new(Expr::lit(BOOL, LitValue::Bool(true))),
                        otherwise: Box::new(right),
                    },
                    BOOL,
                ));
            }
            "if" => {
                let otherwise = stack.pop().expect("shared padding otherwise operand");
                let then = stack.pop().expect("shared padding then operand");
                let condition = stack.pop().expect("shared padding condition");
                stack.push(Expr::new(
                    Node::If {
                        cond: Box::new(condition),
                        then: Box::new(then),
                        otherwise: Box::new(otherwise),
                    },
                    BOOL,
                ));
            }
            _ => unreachable!("unregistered shared padding predicate instruction"),
        }
    }
    assert_eq!(stack.len(), 1, "complete shared padding predicate");
    stack.pop().expect("shared padding predicate result")
}
fn undefined_condition(value: &Expr) -> Option<Expr> {
    if value.ty == Type::Unit {
        return Some(Expr::lit(BOOL, LitValue::Bool(true)));
    }
    if matches!(&value.ty, Type::Option{absence,..} if absence!="null") {
        Some(Expr::new(
            Node::StrictEqual {
                left: Box::new(value.clone()),
                right: Box::new(Expr::new(
                    Node::None {
                        absence: "undefined".to_owned(),
                    },
                    value.ty.clone(),
                )),
                negate: false,
            },
            BOOL,
        ))
    } else {
        None
    }
}
impl Checker {
    fn string_unit_numeric_operand(
        &mut self,
        value: Option<&Expr>,
        fallback: Expr,
        span: Option<Span>,
    ) -> Result<Expr> {
        let Some(value) = value else {
            return Ok(fallback);
        };
        if !super::number_conversions::accepts(&value.ty) {
            return Err(unsupported(
                "string unit method numeric operand",
                "the actual object or symbol needs its complete numeric conversion",
                span,
            ));
        }
        let converted = self.materialize_number_conversion(value.clone(), "coerceJavaScriptNumber");
        Ok(if let Some(condition) = undefined_condition(value) {
            Expr::new(
                Node::If {
                    cond: Box::new(condition),
                    then: Box::new(fallback),
                    otherwise: Box::new(converted),
                },
                FLOAT,
            )
        } else {
            converted
        })
    }
    pub(super) fn javascript_string_unit_call(
        &mut self,
        operation: &str,
        receiver: Expr,
        args: &[Option<SExpr>],
        span: Option<Span>,
        env: &Env,
        path: &[String],
    ) -> Result<Expr> {
        let mut values = vec![receiver];
        for argument in args {
            let argument = argument.as_ref().ok_or_else(|| {
                type_error("a string unit call must retain every actual argument", span)
            })?;
            values.push(self.expr(argument, env, path, None, false)?);
        }
        let names = values
            .iter()
            .map(|_| self.fresh_name("ml_string_unit_operand"))
            .collect::<Vec<_>>();
        let bound = values
            .iter()
            .enumerate()
            .map(|(index, value)| Expr::var(names[index].clone(), value.ty.clone()))
            .collect::<Vec<_>>();
        let mut body = if rules::read_java_script_string_unit_call_form(operation) == "position" {
            let position = self.string_unit_numeric_operand(bound.get(1), number("0"), span)?;
            self.materialize_primitive_method(
                bound[0].clone(),
                operation,
                vec![Some(position)],
                path,
                span,
            )?
        } else if rules::read_java_script_string_unit_call_form(operation) == "repetition" {
            let count_name = self.fresh_name("ml_string_repetition_bound");
            let count = Expr::var(count_name.clone(), FLOAT);
            let call = self.materialize_primitive_method(
                bound[0].clone(),
                operation,
                vec![Some(count.clone())],
                path,
                span,
            )?;
            let original = bound
                .get(1)
                .cloned()
                .unwrap_or_else(|| Expr::new(Node::Unit, Type::Unit));
            let diagnostic = super::string_search::primitive_text(original, span)?;
            let message = Expr::new(
                binary_node(
                    BinaryOp::Concat,
                    Box::new(Expr::lit(
                        STRING,
                        LitValue::Text("Invalid count value: ".to_owned()),
                    )),
                    Box::new(diagnostic),
                    None,
                    None,
                    None,
                    None,
                ),
                STRING,
            );
            let failure = Expr::new(
                Node::Math {
                    op: "raiseJavaScriptRangeError".to_owned(),
                    args: vec![message, Expr::lit(BOOL, LitValue::Bool(true))],
                },
                STRING,
            );
            let conditional = Expr::new(
                Node::If {
                    cond: Box::new(expansion_predicate(
                        rules::plan_java_script_string_repetition_invalid(),
                        &count,
                        &bound[0],
                    )),
                    then: Box::new(failure),
                    otherwise: Box::new(call),
                },
                STRING,
            );
            let value = self.string_unit_numeric_operand(bound.get(1), number("0"), span)?;
            Expr::new(
                Node::Let {
                    name: count_name,
                    value: Box::new(value),
                    body: Box::new(conditional),
                },
                STRING,
            )
        } else if rules::read_java_script_string_unit_call_form(operation) == "padding" {
            let count_name = self.fresh_name("ml_string_padding_bound");
            let filler_name = self.fresh_name("ml_string_padding_filler");
            let count = Expr::var(count_name.clone(), FLOAT);
            let space = Expr::lit(STRING, LitValue::Text(" ".to_owned()));
            let mut converted = space.clone();
            if let Some(filler) = bound.get(2).filter(|value| value.ty != Type::Unit) {
                converted = super::string_search::primitive_text(filler.clone(), span)?;
                if let Some(condition) = undefined_condition(filler) {
                    converted = Expr::new(
                        Node::If {
                            cond: Box::new(condition),
                            then: Box::new(space),
                            otherwise: Box::new(converted),
                        },
                        STRING,
                    );
                }
            }
            let call = self.materialize_primitive_method(
                bound[0].clone(),
                operation,
                vec![
                    Some(count.clone()),
                    Some(Expr::var(filler_name.clone(), STRING)),
                ],
                path,
                span,
            )?;
            let present = Expr::new(
                Node::Let {
                    name: filler_name,
                    value: Box::new(converted),
                    body: Box::new(call),
                },
                STRING,
            );
            let conditional = Expr::new(
                Node::If {
                    cond: Box::new(expansion_predicate(
                        rules::plan_java_script_string_padding_unchanged(),
                        &count,
                        &bound[0],
                    )),
                    then: Box::new(bound[0].clone()),
                    otherwise: Box::new(present),
                },
                STRING,
            );
            let value = self.string_unit_numeric_operand(bound.get(1), number("0"), span)?;
            Expr::new(
                Node::Let {
                    name: count_name,
                    value: Box::new(value),
                    body: Box::new(conditional),
                },
                STRING,
            )
        } else if rules::read_java_script_string_unit_call_form(operation) == "replacement" {
            let search_name = self.fresh_name("ml_string_replace_search");
            let replacement_name = self.fresh_name("ml_string_replace_template");
            let missing = Expr::new(Node::Unit, Type::Unit);
            let search = super::string_search::primitive_text(
                bound.get(1).unwrap_or(&missing).clone(),
                span,
            )?;
            let replacement = super::string_search::primitive_text(
                bound.get(2).unwrap_or(&missing).clone(),
                span,
            )?;
            let call = self.materialize_primitive_method(
                bound[0].clone(),
                operation,
                vec![
                    Some(Expr::var(search_name.clone(), STRING)),
                    Some(Expr::var(replacement_name.clone(), STRING)),
                ],
                path,
                span,
            )?;
            let call = Expr::new(
                Node::Let {
                    name: replacement_name,
                    value: Box::new(replacement),
                    body: Box::new(call),
                },
                STRING,
            );
            Expr::new(
                Node::Let {
                    name: search_name,
                    value: Box::new(search),
                    body: Box::new(call),
                },
                STRING,
            )
        } else {
            let ty = array(STRING);
            let limit_name = self.fresh_name("ml_string_split_limit");
            let limit =
                self.string_unit_numeric_operand(bound.get(2), number("4294967295"), span)?;
            let limit_value = Expr::var(limit_name.clone(), FLOAT);
            let missing = self.materialize_primitive_method(
                bound[0].clone(),
                operation,
                vec![None, Some(limit_value.clone())],
                path,
                span,
            )?;
            let separator = bound.get(1);
            let mut split = if let Some(separator) =
                separator.filter(|value| value.ty != Type::Unit)
            {
                let separator_name = self.fresh_name("ml_string_split_separator");
                let separator_text = super::string_search::primitive_text(separator.clone(), span)?;
                let call = self.materialize_primitive_method(
                    bound[0].clone(),
                    operation,
                    vec![
                        Some(Expr::var(separator_name.clone(), STRING)),
                        Some(limit_value),
                    ],
                    path,
                    span,
                )?;
                let present = Expr::new(
                    Node::Let {
                        name: separator_name,
                        value: Box::new(separator_text),
                        body: Box::new(call),
                    },
                    ty.clone(),
                );
                if let Some(condition) = undefined_condition(separator) {
                    Expr::new(
                        Node::If {
                            cond: Box::new(condition),
                            then: Box::new(missing),
                            otherwise: Box::new(present),
                        },
                        ty.clone(),
                    )
                } else {
                    present
                }
            } else {
                missing
            };
            split = Expr::new(
                Node::Let {
                    name: limit_name,
                    value: Box::new(limit),
                    body: Box::new(split),
                },
                ty,
            );
            split
        };
        for index in (0..values.len()).rev() {
            let ty = body.ty.clone();
            body = Expr::new(
                Node::Let {
                    name: names[index].clone(),
                    value: Box::new(values[index].clone()),
                    body: Box::new(body),
                },
                ty,
            );
        }
        Ok(body)
    }
}
