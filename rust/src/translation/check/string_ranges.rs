//! Transport actual children through the generated complete checked-operator plan.
use super::{
    BOOL, BinaryOp, Checker, Env, Expr, FLOAT, LitValue, Node, Result, SExpr, STRING, Span, Type,
    binary_node, type_error, unsupported,
};
use crate::translation::javascript_string_range_rules as rules;
fn number(value: &str) -> Expr {
    Expr::lit(FLOAT, LitValue::Text(value.to_owned()))
}
fn window(operation: &str, receiver: &Expr, start: &Expr, end: &Expr) -> Vec<Option<Expr>> {
    let mut stack: Vec<Expr> = Vec::new();
    for instruction in rules::plan_java_script_string_range_window(operation) {
        match instruction.as_str() {
            "start" => stack.push(start.clone()),
            "end" => stack.push(end.clone()),
            "length" => stack.push(Expr::new(
                Node::Length {
                    array: Box::new(receiver.clone()),
                },
                FLOAT,
            )),
            "0" | "Infinity" => stack.push(number(&instruction)),
            "trunc" => {
                let argument = stack.pop().expect("shared range unary operand");
                stack.push(Expr::new(
                    Node::Math {
                        op: "trunc".to_owned(),
                        args: vec![argument],
                    },
                    FLOAT,
                ));
            }
            "if" => {
                let otherwise = stack.pop().expect("shared range else operand");
                let then = stack.pop().expect("shared range then operand");
                let condition = stack.pop().expect("shared range condition");
                stack.push(Expr::new(
                    Node::If {
                        cond: Box::new(condition),
                        then: Box::new(then),
                        otherwise: Box::new(otherwise),
                    },
                    FLOAT,
                ));
            }
            instruction => {
                let right = stack.pop().expect("shared range right operand");
                let left = stack.pop().expect("shared range left operand");
                if instruction == "min" || instruction == "max" {
                    stack.push(Expr::new(
                        Node::Math {
                            op: instruction.to_owned(),
                            args: vec![left, right],
                        },
                        FLOAT,
                    ));
                } else {
                    let operation = match instruction {
                        "eq" => BinaryOp::Eq,
                        "lt" => BinaryOp::Lt,
                        "add" => BinaryOp::Add,
                        _ => unreachable!("shared range checked-operator instruction"),
                    };
                    stack.push(Expr::new(
                        binary_node(
                            operation,
                            Box::new(left),
                            Box::new(right),
                            Some(FLOAT),
                            None,
                            None,
                            None,
                        ),
                        if matches!(operation, BinaryOp::Eq | BinaryOp::Lt) {
                            BOOL
                        } else {
                            FLOAT
                        },
                    ));
                }
            }
        }
    }
    stack.into_iter().map(Some).collect()
}
impl Checker {
    pub(super) fn javascript_string_range(
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
                type_error("a string range must retain every actual argument", span)
            })?;
            values.push(self.expr(argument, env, path, None, false)?);
        }
        let names = values
            .iter()
            .map(|_| self.fresh_name("ml_string_range_operand"))
            .collect::<Vec<_>>();
        let bound = values
            .iter()
            .enumerate()
            .map(|(index, value)| Expr::var(names[index].clone(), value.ty.clone()))
            .collect::<Vec<_>>();
        let mut converted = Vec::new();
        for index in 0..2 {
            let fallback = number(if index == 0 { "0" } else { "Infinity" });
            let Some(value) = bound.get(index + 1) else {
                converted.push(fallback);
                continue;
            };
            if value.ty == Type::Unit {
                converted.push(fallback);
                continue;
            }
            if !super::number_conversions::accepts(&value.ty) {
                return Err(unsupported(
                    "string range position",
                    "the actual object or symbol needs its complete numeric conversion",
                    span,
                ));
            }
            let mut result =
                self.materialize_number_conversion(value.clone(), "coerceJavaScriptNumber");
            if matches!(&value.ty,Type::Option{absence,..}if absence!="null") {
                let condition = Expr::new(
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
                );
                result = Expr::new(
                    Node::If {
                        cond: Box::new(condition),
                        then: Box::new(fallback),
                        otherwise: Box::new(result),
                    },
                    FLOAT,
                );
            }
            converted.push(result);
        }
        let positions = converted
            .iter()
            .map(|_| self.fresh_name("ml_string_range_position"))
            .collect::<Vec<_>>();
        let start = Expr::var(positions[0].clone(), FLOAT);
        let end = Expr::var(positions[1].clone(), FLOAT);
        let mut body = Expr::new(
            Node::Method {
                op: "slice".to_owned(),
                object: Box::new(bound[0].clone()),
                args: window(operation, &bound[0], &start, &end),
            },
            STRING,
        );
        for index in (0..2).rev() {
            body = Expr::new(
                Node::Let {
                    name: positions[index].clone(),
                    value: Box::new(converted[index].clone()),
                    body: Box::new(body),
                },
                STRING,
            );
        }
        for index in (0..values.len()).rev() {
            body = Expr::new(
                Node::Let {
                    name: names[index].clone(),
                    value: Box::new(values[index].clone()),
                    body: Box::new(body),
                },
                STRING,
            );
        }
        Ok(body)
    }
}
