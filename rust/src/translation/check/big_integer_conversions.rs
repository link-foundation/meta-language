//! Actual checked type and shared intrinsic result-column adapters.
use super::{
    BOOL, BinaryOp, Checker, Env, Expr, FLOAT, INT, Language, LitValue, Node, Result, SArrayItem,
    STRING, Span, Type, array, binary_node, plain_binary, type_error, unsupported,
};
use crate::translation::big_integer_conversion_rules as rules;
use crate::translation::types::option;

fn text(value: impl Into<String>) -> Expr {
    Expr::lit(STRING, LitValue::Text(value.into()))
}
fn failure(form: &str, value: Expr) -> Expr {
    let pieces = rules::read_java_script_big_integer_conversion_message_pieces(form);
    let first = plain_binary(BinaryOp::Concat, text(&pieces[0]), value, STRING);
    let message = plain_binary(BinaryOp::Concat, first, text(&pieces[1]), STRING);
    let class = rules::read_java_script_big_integer_conversion_error_class(form);
    Expr::new(Node::Math {op:crate::translation::source_exception_rules::read_java_script_source_exception_operation(&class),args:vec![message,Expr::lit(BOOL,LitValue::Bool(true))]},INT)
}

impl Checker {
    fn materialize_big_integer(&mut self, value: Expr, span: Option<Span>) -> Result<Expr> {
        let form = rules::read_java_script_big_integer_conversion_form(value.ty.kind());
        match form.as_str() {
            "identity" => return self.coerce(value, &INT, span),
            "boolean" => {
                return Ok(Expr::new(
                    Node::Math {
                        op: "convertJavaScriptBooleanBigIntegerValue".to_owned(),
                        args: vec![value],
                    },
                    INT,
                ));
            }
            "undefined" => {
                let name = self.fresh_name("ml_integer_undefined");
                return Ok(Expr::new(
                    Node::Let {
                        name,
                        value: Box::new(value),
                        body: Box::new(failure("undefined", text("undefined"))),
                    },
                    INT,
                ));
            }
            "optional" => {
                let Type::Option { element, absence } = &value.ty else {
                    unreachable!("actual optional primitive")
                };
                let element = (**element).clone();
                let absences =
                    crate::translation::optional_value_rules::read_optional_output_absences(
                        absence,
                    );
                let name = self.fresh_name("ml_integer_optional");
                let variable = Expr::var(name.clone(), value.ty.clone());
                let mut body = self.materialize_big_integer(
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
                    body = Expr::new(
                        Node::If {
                            cond: Box::new(condition),
                            then: Box::new(failure(&absence, text(&absence))),
                            otherwise: Box::new(body),
                        },
                        INT,
                    );
                }
                return Ok(Expr::new(
                    Node::Let {
                        name,
                        value: Box::new(value),
                        body: Box::new(body),
                    },
                    INT,
                ));
            }
            "string" | "number" => {}
            _ => {
                return Err(unsupported(
                    "BigInt primitive conversion",
                    "the actual object primitive conversion or symbol must have its complete source behavior",
                    span,
                ));
            }
        }
        let name = self.fresh_name("ml_integer_operand");
        let bound = Expr::var(name.clone(), value.ty.clone());
        let result_name = self.fresh_name("ml_integer_result");
        let result = Expr::var(result_name.clone(), array(INT));
        let conversion = Expr::new(
            Node::Math {
                op: if form == "string" {
                    "parseJavaScriptStringBigInteger"
                } else {
                    "convertJavaScriptNumberBigIntegerResult"
                }
                .to_owned(),
                args: vec![bound.clone()],
            },
            array(INT),
        );
        let length = Expr::new(
            Node::Length {
                array: Box::new(result.clone()),
            },
            FLOAT,
        );
        let condition = Expr::new(
            binary_node(
                BinaryOp::Eq,
                Box::new(length),
                Box::new(Expr::lit(FLOAT, LitValue::Text("1".to_owned()))),
                Some(FLOAT),
                None,
                None,
                None,
            ),
            BOOL,
        );
        let selected = Expr::new(
            Node::OptionalIndex {
                array: Box::new(result),
                index: Box::new(Expr::lit(FLOAT, LitValue::Text("0".to_owned()))),
            },
            option(INT, "undefined"),
        );
        let success = Expr::new(
            Node::Coalesce {
                left: Box::new(selected),
                right: Box::new(Expr::lit(INT, LitValue::Text("0".to_owned()))),
            },
            INT,
        );
        let display = if form == "string" {
            bound
        } else {
            Expr::new(
                Node::ToString {
                    arg: Box::new(bound),
                    console: false,
                },
                STRING,
            )
        };
        let body = Expr::new(
            Node::If {
                cond: Box::new(condition),
                then: Box::new(success),
                otherwise: Box::new(failure(&form, display)),
            },
            INT,
        );
        let body = Expr::new(
            Node::Let {
                name: result_name,
                value: Box::new(conversion),
                body: Box::new(body),
            },
            INT,
        );
        Ok(Expr::new(
            Node::Let {
                name,
                value: Box::new(value),
                body: Box::new(body),
            },
            INT,
        ))
    }

    pub(super) fn javascript_big_integer_builtin(
        &mut self,
        arguments: &[SArrayItem],
        span: Option<Span>,
        env: &Env,
        path: &[String],
    ) -> Result<Expr> {
        if self.language != Language::JavaScript {
            return Err(unsupported(
                "JavaScript BigInt conversion",
                "the operation needs its actual JavaScript source owner",
                span,
            ));
        }
        if arguments.iter().any(|item| item.spread) {
            return Err(unsupported(
                "spread BigInt arguments",
                "the actual iterator and complete argument effects need their represented operations",
                span,
            ));
        }
        let values = arguments
            .iter()
            .map(|item| self.expr(&item.value, env, path, None, false))
            .collect::<Result<Vec<_>>>()?;
        let names = if values.len() > 1 {
            values
                .iter()
                .map(|_| self.fresh_name("ml_integer_argument"))
                .collect::<Vec<_>>()
        } else {
            Vec::new()
        };
        let first = values.first().map(|value| {
            if values.len() > 1 {
                Expr::var(names[0].clone(), value.ty.clone())
            } else {
                value.clone()
            }
        });
        let mut body = if let Some(first) = first {
            self.materialize_big_integer(first, span)?
        } else {
            failure("undefined", text("undefined"))
        };
        if values.len() > 1 {
            for (index, value) in values.into_iter().enumerate().rev() {
                body = Expr::new(
                    Node::Let {
                        name: names[index].clone(),
                        value: Box::new(value),
                        body: Box::new(body),
                    },
                    INT,
                );
            }
        }
        Ok(body)
    }

    pub(super) fn javascript_big_integer_result(
        &mut self,
        operation: &str,
        arguments: &[SArrayItem],
        span: Option<Span>,
        env: &Env,
        path: &[String],
    ) -> Result<Expr> {
        let kinds = rules::read_java_script_big_integer_intrinsic_operand_kinds(operation);
        if self.language != Language::JavaScript
            || arguments.len() != kinds.len()
            || arguments.iter().any(|argument| argument.spread)
        {
            return Err(type_error(
                "a JavaScript numeric intrinsic needs its complete owned operands",
                span,
            ));
        }
        let mut args = Vec::new();
        for (argument, kind) in arguments.iter().zip(&kinds) {
            let ty = match kind.as_str() {
                "float" => FLOAT,
                "int" => INT,
                "string" => STRING,
                "bool" => BOOL,
                _ => unreachable!("registered intrinsic operand"),
            };
            let value = self.expr(&argument.value, env, path, Some(&ty), false)?;
            if value.ty.kind() != kind.as_str() {
                return Err(type_error(
                    "a JavaScript numeric intrinsic needs its actual operand types",
                    span,
                ));
            }
            args.push(value);
        }
        let result = rules::read_java_script_big_integer_intrinsic_result_kind(operation);
        let ty = match result.as_str() {
            "float" => FLOAT,
            "int" => INT,
            "array-integer" => array(INT),
            _ => unreachable!("registered intrinsic result"),
        };
        Ok(Expr::new(
            Node::Math {
                op: operation.to_owned(),
                args,
            },
            ty,
        ))
    }

    /// Actual IR adapter; complete division is generated from the shared source.
    pub(super) fn materialize_big_integer_division(
        &mut self,
        operation: BinaryOp,
        left: &Expr,
        right: &Expr,
    ) -> Option<Expr> {
        let literal = match &right.node {
            Node::Lit { value } => value.text(),
            _ => String::new(),
        };
        if !rules::require_java_script_big_integer_division(
            self.language.as_str(),
            operation.name(),
            left.ty.kind(),
            &literal,
        ) {
            return None;
        }
        Some(self.big_integer_arithmetic_result(
            "divideJavaScriptBigIntegerResult",
            vec![left.clone(), right.clone()],
            "2",
            if operation == BinaryOp::Div { "0" } else { "1" },
            rules::read_java_script_big_integer_division_message(),
        ))
    }

    pub(super) fn big_integer_arithmetic_result(
        &mut self,
        operation: &str,
        arguments: Vec<Expr>,
        count: &str,
        index: &str,
        message: String,
    ) -> Expr {
        let stem = if operation == "powerJavaScriptBigIntegerResult" {
            "ml_integer_power"
        } else {
            "ml_integer_division"
        };
        let name = self.fresh_name(stem);
        let result = Expr::var(name.clone(), array(INT));
        let conversion = Expr::new(
            Node::Math {
                op: operation.to_owned(),
                args: arguments,
            },
            array(INT),
        );
        let length = Expr::new(
            Node::Length {
                array: Box::new(result.clone()),
            },
            FLOAT,
        );
        let condition = Expr::new(
            binary_node(
                BinaryOp::Eq,
                Box::new(length),
                Box::new(Expr::lit(FLOAT, LitValue::Text(count.to_owned()))),
                Some(FLOAT),
                None,
                None,
                None,
            ),
            BOOL,
        );
        let index = Expr::lit(FLOAT, LitValue::Text(index.to_owned()));
        let optional = Expr::new(
            Node::OptionalIndex {
                array: Box::new(result),
                index: Box::new(index),
            },
            option(INT, "undefined"),
        );
        let value = Expr::new(
            Node::Coalesce {
                left: Box::new(optional),
                right: Box::new(Expr::lit(INT, LitValue::Text("0".to_owned()))),
            },
            INT,
        );
        let failure = Expr::new(
            Node::Math {
                op: "raiseJavaScriptRangeError".to_owned(),
                args: vec![text(message), Expr::lit(BOOL, LitValue::Bool(true))],
            },
            INT,
        );
        let body = Expr::new(
            Node::If {
                cond: Box::new(condition),
                then: Box::new(value),
                otherwise: Box::new(failure),
            },
            INT,
        );
        Expr::new(
            Node::Let {
                name,
                value: Box::new(conversion),
                body: Box::new(body),
            },
            INT,
        )
    }
}
