//! Actual primitive argument/IR transport into the shared string search rules.
use super::{
    BOOL, Checker, Env, Expr, FLOAT, Language, LitValue, Node, Result, SArrayItem, SExpr, STRING,
    Span, Type, UNIT, type_error, unsupported,
};
use crate::translation::javascript_property_call_rules as rules;
fn number(value: &str) -> Expr {
    Expr::lit(FLOAT, LitValue::Text(value.to_owned()))
}
fn accepts(ty: &Type) -> bool {
    match rules::read_java_script_primitive_text_form(ty.kind()).as_str() {
        "optional" => matches!(ty,Type::Option{element,..}if accepts(element)),
        "refuse" => false,
        _ => true,
    }
}
pub(super) fn primitive_text(value: Expr, span: Option<Span>) -> Result<Expr> {
    if !accepts(&value.ty) {
        return Err(unsupported(
            "primitive text conversion",
            "the actual object or symbol needs its complete primitive conversion",
            span,
        ));
    }
    let form = rules::read_java_script_primitive_text_form(value.ty.kind());
    Ok(match form.as_str() {
        "identity" => value,
        "undefined" => Expr::lit(STRING, LitValue::Text("undefined".to_owned())),
        "optional" => Expr::new(
            Node::OptionalText {
                arg: Box::new(value),
                console: false,
            },
            STRING,
        ),
        _ => Expr::new(
            Node::ToString {
                arg: Box::new(value),
                console: false,
            },
            STRING,
        ),
    })
}
impl Checker {
    pub(super) fn javascript_string_map_call(
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
                type_error("a string map must retain every actual argument", span)
            })?;
            values.push(self.expr(argument, env, path, None, false)?);
        }
        let names = values
            .iter()
            .map(|_| self.fresh_name("ml_string_map_operand"))
            .collect::<Vec<_>>();
        let mut body = Expr::new(
            Node::StringMap {
                op: operation.to_owned(),
                string: Box::new(Expr::var(names[0].clone(), values[0].ty.clone())),
            },
            STRING,
        );
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

    pub(super) fn javascript_primitive_text_builtin(
        &mut self,
        args: &[SArrayItem],
        span: Option<Span>,
        env: &Env,
        path: &[String],
    ) -> Result<Expr> {
        if self.language != Language::JavaScript || args.iter().any(|argument| argument.spread) {
            return Err(unsupported(
                "String arguments",
                "the complete actual iterator effects must be represented",
                span,
            ));
        }
        let values = args
            .iter()
            .map(|argument| self.expr(&argument.value, env, path, None, false))
            .collect::<Result<Vec<_>>>()?;
        let names = values
            .iter()
            .map(|_| self.fresh_name("ml_string_argument"))
            .collect::<Vec<_>>();
        let mut body = if let Some(first) = values.first() {
            primitive_text(Expr::var(names[0].clone(), first.ty.clone()), span)?
        } else {
            Expr::lit(STRING, LitValue::Text(String::new()))
        };
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
    pub(super) fn javascript_string_search(
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
                type_error("a string search must retain every actual argument", span)
            })?;
            values.push(self.expr(argument, env, path, None, false)?);
        }
        let names = values
            .iter()
            .map(|_| self.fresh_name("ml_string_search_operand"))
            .collect::<Vec<_>>();
        let bound = values
            .iter()
            .enumerate()
            .map(|(index, value)| Expr::var(names[index].clone(), value.ty.clone()))
            .collect::<Vec<_>>();
        let undefined = Expr::new(
            Node::None {
                absence: "undefined".to_owned(),
            },
            crate::translation::types::option(UNIT, "undefined"),
        );
        let search = primitive_text(bound.get(1).cloned().unwrap_or(undefined), span)?;
        let search_name = self.fresh_name("ml_string_search_text");
        let default_value = number(&rules::read_java_script_string_search_default_position(
            operation,
        ));
        let mut position = default_value.clone();
        if let Some(value) = bound.get(2) {
            if !super::number_conversions::accepts(&value.ty) {
                return Err(unsupported(
                    "string search position",
                    "the actual object or symbol needs its complete numeric conversion",
                    span,
                ));
            }
            position = if value.ty == Type::Unit {
                default_value.clone()
            } else {
                self.materialize_number_conversion(value.clone(), "coerceJavaScriptNumber")
            };
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
                position = Expr::new(
                    Node::If {
                        cond: Box::new(condition),
                        then: Box::new(default_value),
                        otherwise: Box::new(position),
                    },
                    FLOAT,
                );
            }
        }
        let position_name = self.fresh_name("ml_string_search_position");
        let at = Expr::var(position_name.clone(), FLOAT);
        let nonnegative = Expr::new(
            Node::Math {
                op: "max".to_owned(),
                args: vec![at, number("0")],
            },
            FLOAT,
        );
        let window = rules::plan_java_script_string_search_window(operation)
            .iter()
            .map(|value| {
                Some(if value == "position" {
                    nonnegative.clone()
                } else {
                    number(value)
                })
            })
            .collect();
        let sliced = Expr::new(
            Node::Method {
                op: "slice".to_owned(),
                object: Box::new(bound[0].clone()),
                args: window,
            },
            STRING,
        );
        let ty = if rules::read_java_script_string_search_result_kind(operation) == "float" {
            FLOAT
        } else {
            BOOL
        };
        let mut body = if ty == FLOAT {
            Expr::new(
                Node::Method {
                    op: operation.to_owned(),
                    object: Box::new(bound[0].clone()),
                    args: vec![
                        Some(Expr::var(search_name.clone(), STRING)),
                        Some(Expr::var(position_name.clone(), FLOAT)),
                    ],
                },
                FLOAT,
            )
        } else {
            Expr::new(
                Node::StringTest {
                    op: operation.to_owned(),
                    string: Box::new(sliced),
                    search: Box::new(Expr::var(search_name.clone(), STRING)),
                },
                BOOL,
            )
        };
        body = Expr::new(
            Node::Let {
                name: position_name,
                value: Box::new(position),
                body: Box::new(body),
            },
            ty.clone(),
        );
        body = Expr::new(
            Node::Let {
                name: search_name,
                value: Box::new(search),
                body: Box::new(body),
            },
            ty.clone(),
        );
        for index in (0..values.len()).rev() {
            body = Expr::new(
                Node::Let {
                    name: names[index].clone(),
                    value: Box::new(values[index].clone()),
                    body: Box::new(body),
                },
                ty.clone(),
            );
        }
        Ok(body)
    }
}
