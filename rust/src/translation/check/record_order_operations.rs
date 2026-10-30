//! Actual typed operands for the complete ordinary source order library.
use super::{Checker, Env, Expr, Node, Result, SArrayItem, Span, Type, array, type_error};
use crate::translation::javascript_enumeration_rules as rules;

pub(in super::super) fn role_type(role: &str) -> Result<Type> {
    match role {
        "number" => Ok(Type::Float),
        "text" => Ok(Type::String),
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
    pub(super) fn record_value_order_operation(
        &mut self,
        args: &[SArrayItem],
        env: &Env,
        path: &[String],
        span: Option<Span>,
        expected: Option<&Type>,
    ) -> Result<Expr> {
        if self.language != super::Language::JavaScript
            || args.len() != 2
            || args.iter().any(|argument| argument.spread)
        {
            return Err(type_error(
                "an ordinary record order operation needs every owned source operand",
                span,
            ));
        }
        let expected = expected.filter(|ty| matches!(ty, Type::Array { .. }));
        let values = self.expr(&args[0].value, env, path, expected, false)?;
        let values = Self::reference_value(values);
        if !matches!(
            values.ty,
            Type::Array {
                reference: false,
                ..
            }
        ) {
            return Err(type_error(
                "record value ordering needs its actual owned value array",
                span,
            ));
        }
        let positions = array(Type::Float);
        let order = self.expr(&args[1].value, env, path, Some(&positions), false)?;
        let order = self.coerce(Self::reference_value(order), &positions, span)?;
        let ty = values.ty.clone();
        Ok(Expr::new(
            Node::Math {
                op: "reorderJavaScriptRecordPropertyValues".to_owned(),
                args: vec![values, order],
            },
            ty,
        ))
    }
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
        if operation == rules::read_java_script_object_entry_pairing_operation() {
            let keys_type = array(Type::String);
            let keys = self.expr(&args[0].value, env, path, Some(&keys_type), false)?;
            let keys = self.coerce(Self::reference_value(keys), &keys_type, span)?;
            let values = self.expr(&args[1].value, env, path, None, false)?;
            let values = Self::reference_value(values);
            let Type::Array {
                element,
                reference: false,
            } = &values.ty
            else {
                return Err(type_error(
                    "own entry pairing needs its actual owned value array",
                    span,
                ));
            };
            let ty = array(Type::Tuple {
                elements: vec![Type::String, element.as_ref().clone()],
            });
            return Ok(Expr::new(
                Node::Math {
                    op: operation.to_owned(),
                    args: vec![keys, values],
                },
                ty,
            )
            .with_span(span));
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
