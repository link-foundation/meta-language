//! Source handle/value transport into existing ordered lets, reads and appends.
use super::{
    BinaryOp, Checker, Env, Expr, Node, Result, SExpr, STRING, Span, Type, array, plain_binary,
    type_error, unsupported,
};
use crate::translation::javascript_property_call_rules as rules;

impl Checker {
    pub(super) fn source_concatenation(
        &mut self,
        source: &SExpr,
        args: &[Option<SExpr>],
        env: &Env,
        path: &[String],
        span: Option<Span>,
    ) -> Result<Option<Expr>> {
        let receiver = self.expr(source, env, path, None, false)?;
        let kind = receiver.ty.kind();
        if kind != "array" && kind != "string" {
            return Ok(None);
        }
        let element = if let Type::Array { element, .. } = &receiver.ty {
            Some(element.as_ref().clone())
        } else {
            None
        };
        let ty = element
            .as_ref()
            .map_or(STRING, |element| array(element.clone()));
        let mut values = vec![receiver];
        for argument in args {
            let argument = argument.as_ref().ok_or_else(|| {
                type_error("a concat call must retain every actual argument", span)
            })?;
            values.push(self.expr(argument, env, path, None, false)?);
        }
        let mut inner = env.clone();
        let mut bindings = Vec::new();
        for value in &values {
            let name = loop {
                let name = self.fresh_name("ml_concat_value");
                if !inner.has(&name) {
                    break name;
                }
            };
            inner.bind_local(&name, value.ty.clone());
            bindings.push(name);
        }
        let variable = |index: usize| {
            Expr::new(
                Node::Var {
                    name: bindings[index].clone(),
                },
                values[index].ty.clone(),
            )
        };
        let mut body = if element.is_some() {
            Expr::new(
                Node::Append {
                    left: Box::new(Expr::new(Node::Array { items: Vec::new() }, ty.clone())),
                    right: Box::new(Self::reference_value(variable(0))),
                },
                ty.clone(),
            )
        } else {
            variable(0)
        };
        for index in 1..values.len() {
            let role = rules::read_java_script_concatenation_argument_role(
                kind,
                values[index].ty.kind(),
                true,
            );
            if role == "unsupported" {
                return Err(unsupported(
                    "String concat coercion",
                    "this argument needs its complete ToPrimitive interpretation",
                    span,
                ));
            }
            if let Some(element) = &element {
                let part = if role == "spread" {
                    self.coerce(Self::reference_value(variable(index)), &ty, span)?
                } else {
                    Expr::new(
                        Node::Array {
                            items: vec![self.coerce(variable(index), element, span)?],
                        },
                        ty.clone(),
                    )
                };
                body = Expr::new(
                    Node::Append {
                        left: Box::new(body),
                        right: Box::new(part),
                    },
                    ty.clone(),
                );
            } else {
                let text = super::string_search::primitive_text(variable(index), span)?;
                body = plain_binary(BinaryOp::Concat, body, text, STRING);
            }
        }
        for (name, value) in bindings.into_iter().zip(values).rev() {
            body = Expr::new(
                Node::Let {
                    name,
                    value: Box::new(value),
                    body: Box::new(body),
                },
                ty.clone(),
            );
        }
        Ok(Some(body))
    }
}
