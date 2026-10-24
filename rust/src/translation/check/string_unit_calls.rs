//! Actual operands/type transport into the shared source unit-call decisions.
use super::{
    BOOL, Checker, Env, Expr, FLOAT, LitValue, Node, Result, SExpr, STRING, Span, Type, array,
    type_error, unsupported,
};
use crate::translation::javascript_property_call_rules as rules;
fn number(value: &str) -> Expr {
    Expr::lit(FLOAT, LitValue::Text(value.to_owned()))
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
