//! Checked operand owners selected by the complete generated callback rules.
use super::expectation::ExpectedType;
use super::{
    BOOL, Checker, Env, Expr, FLOAT, Node, Result, SExpr, SNode, Type, UNIT, array, type_error,
    unsupported,
};
use crate::translation::array_callback_rules as rules;
use crate::translation::frontend_rules::ml_array::number_index;

#[allow(clippy::cast_precision_loss)]
const fn number(value: usize) -> f64 {
    value as f64
}

fn signature_error(
    operation: &str,
    actual: &Type,
    supplied: &[Type],
    expected: Option<&Type>,
) -> String {
    let (declared, parameters_match, result_matches) =
        if let Type::Function { params, ret } = actual {
            (
                params.len(),
                params
                    .iter()
                    .zip(supplied)
                    .all(|(left, right)| left.same(right)),
                expected.is_none_or(|expected| ret.same(expected)),
            )
        } else {
            (0, true, true)
        };
    rules::read_array_callback_signature_error(
        operation,
        actual.kind(),
        number(declared),
        number(supplied.len()),
        parameters_match,
        result_matches,
        &expected.map_or_else(String::new, Type::key),
    )
}

impl Checker {
    pub(super) fn array_callback_method(
        &mut self,
        node: &SExpr,
        env: &Env,
        path: &[String],
        expected: ExpectedType<'_>,
        form: &str,
    ) -> Result<Expr> {
        let SNode::Method { object, op, args } = &node.node else {
            return Err(type_error(
                "an array callback needs an actual method",
                node.span,
            ));
        };
        let object = self.expr(object, env, path, None, false)?;
        let object = Self::reference_value(object);
        let Type::Array { element, .. } = &object.ty else {
            return self
                .primitive_method_on_checked_receiver(object, op, args, env, path, node.span);
        };
        let error = rules::read_array_callback_argument_count_error(op, number(args.len()));
        if !error.is_empty() {
            return Err(unsupported(
                &format!("method call .{op}()"),
                &error,
                node.span,
            ));
        }
        let has_initial = args.get(1).and_then(Option::as_ref).is_some();
        let initial_position =
            rules::read_array_callback_checked_argument_position(op, has_initial, 0.0);
        let initial = if number_index(initial_position) == Some(1) {
            args.get(1)
                .and_then(Option::as_ref)
                .map(|value| self.expr_with_context(value, env, path, expected, false))
                .transpose()?
        } else {
            None
        };
        let accumulator = initial.as_ref().map_or(element.as_ref(), |value| &value.ty);
        let mut params = Vec::new();
        loop {
            let role = rules::read_array_callback_parameter_role(op, number(params.len()));
            let ty = match role.as_str() {
                "accumulator" => accumulator,
                "element" => element.as_ref(),
                "index" => &FLOAT,
                "receiver" => &object.ty,
                _ => break,
            };
            params.push(ty.clone());
        }
        let context = rules::read_array_callback_checked_result_context(op);
        let ret = match context.as_str() {
            "bool" => Some(&BOOL),
            "accumulator" => Some(accumulator),
            _ => None,
        };
        let callback = args
            .first()
            .and_then(Option::as_ref)
            .ok_or_else(|| type_error(signature_error(op, &UNIT, &params, ret), node.span))?;
        let callback = self.expr_with_callable_expectation(callback, env, path, &params, ret)?;
        let error = signature_error(op, &callback.ty, &params, ret);
        if !error.is_empty() {
            return Err(type_error(error, node.span));
        }
        let Type::Function { ret, .. } = &callback.ty else {
            return Err(type_error(
                signature_error(op, &callback.ty, &params, None),
                node.span,
            ));
        };
        let error = rules::read_array_callback_result_error(op, ret.kind());
        if !error.is_empty() {
            return Err(type_error(error, node.span));
        }
        let ty = match form {
            "receiver" => object.ty.clone(),
            "optional element" => Type::Option {
                element: element.clone(),
                absence: "undefined".to_owned(),
            },
            "float" => FLOAT,
            "bool" => BOOL,
            "unit" => UNIT,
            "accumulator" => accumulator.clone(),
            "mapped array" => array(ret.as_ref().clone()),
            _ => ret.as_ref().clone(),
        };
        let mut values = vec![Some(callback)];
        if let Some(initial) = initial {
            values.push(Some(initial));
        }
        Ok(Expr::new(
            Node::Method {
                object: Box::new(object),
                op: op.clone(),
                args: values,
            },
            ty,
        ))
    }
}
