//! Actual typed operands for the complete ordinary source order library.
use super::{Checker, Env, Expr, Node, Result, SArrayItem, Span, Type, array, type_error};
use crate::translation::javascript_enumeration_rules as rules;

pub(in super::super) fn role_type(role: &str) -> Result<Type> {
    match role {
        "number" => Ok(Type::Float),
        "boolean" => Ok(Type::Bool),
        "positions" => Ok(array(Type::Float)),
        "name rows" => Ok(array(array(Type::Float))),
        "key texts" => Ok(array(Type::String)),
        _ => Err(type_error(
            "an ordinary record order library operand has an unknown role",
            None,
        )),
    }
}
impl Checker {
    pub(super) fn record_order_operation(
        &mut self,
        operation: &str,
        args: &[SArrayItem],
        env: &Env,
        path: &[String],
        span: Option<Span>,
    ) -> Result<Expr> {
        let arity = rules::read_java_script_record_order_kernel_arity(operation);
        if self.language != super::Language::JavaScript
            || serde_json::Number::from(args.len()).as_f64() != Some(arity)
            || args.iter().any(|argument| argument.spread)
        {
            return Err(type_error(
                "an ordinary record order operation needs every owned source operand",
                span,
            ));
        }
        let mut values = Vec::new();
        for (index, argument) in args.iter().enumerate() {
            let index = serde_json::Number::from(index)
                .as_f64()
                .expect("a bounded source operand count is a Number");
            let ty = role_type(&rules::read_java_script_record_order_kernel_parameter(
                operation, index,
            ))?;
            let value = self.expr(&argument.value, env, path, Some(&ty), false)?;
            values.push(self.coerce(Self::reference_value(value), &ty, span)?);
        }
        Ok(Expr::new(
            Node::Math {
                op: operation.to_owned(),
                args: values,
            },
            role_type(&rules::read_java_script_record_order_kernel_result(
                operation,
            ))?,
        ))
    }
    pub(super) fn ordered_record_order_read(
        object: Expr,
        operation: &str,
        count: usize,
        span: Option<Span>,
    ) -> Result<Expr> {
        if !rules::accept_linked_ordered_record_order_read(
            "method",
            operation,
            object.ty.kind(),
            matches!(&object.ty, Type::Record { ordered: true, .. }),
            object.ty.is_reference(),
        ) || count != 0
        {
            return Err(type_error(
                "own-field order projection needs one actual owned ordered record",
                span,
            ));
        }
        Ok(Expr::new(
            Node::Method {
                object: Box::new(object),
                op: operation.to_owned(),
                args: Vec::new(),
            },
            array(Type::Float),
        ))
    }
}
