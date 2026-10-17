//! Mechanically extracted original expression arms; no semantic replacement.
use super::*;

impl Checker {
    #[allow(clippy::too_many_lines)]
    pub(super) fn check_literal_expression(
        &mut self,
        node: &SExpr,
        env: &Env,
        path: &[String],
        context: ExpectedType<'_>,
        allow_literal: bool,
    ) -> Result<Expr> {
        let span = node.span;
        let expected = context.concrete();
        match &node.node {
            SNode::Num {
                value,
                ty,
                negative,
                unit,
            } => {
                // A JavaScript Number literal: its canonical text, `String(value)`, with the sign applied.
                if ty.as_ref().is_some_and(Type::is_float) {
                    let text = if *negative {
                        negate_number(value)
                    } else {
                        value.clone()
                    };
                    return Ok(text_lit(FLOAT, text));
                }
                // The 1 of `x++` takes the type of `x`, which may be a Number.
                if *unit && expected.is_some_and(Type::is_float) {
                    return Ok(text_lit(FLOAT, value.clone()));
                }
                let ty = ty
                    .clone()
                    .or_else(|| expected.filter(|ty| ty.is_numeric()).cloned());
                let Some(ty) = ty else {
                    if allow_literal {
                        return Ok(text_lit(Type::Literal, value.clone()));
                    }
                    return self.expr(node, env, path, Some(&self.default_number()), false);
                };
                if !ty.is_numeric() {
                    return Err(type_error(
                        format!("numeric literal where {} is expected", ty.key()),
                        span,
                    ));
                }
                let mut number = Decimal::parse(value)
                    .ok_or_else(|| type_error(format!("invalid numeric literal {value}"), span))?;
                if *negative {
                    number = number.negate();
                }
                if ty.is_natural() && number.is_negative() {
                    return Err(type_error("negative literal for a natural type", span));
                }
                if let Type::Fixed { bits, signed } = ty {
                    let (min, max) = fixed_bounds(bits, signed);
                    if number < Decimal::from_i128(min) || number > Decimal::from_u128(max) {
                        return Err(type_error(
                            format!("literal out of range for {}", ty.key()),
                            span,
                        ));
                    }
                }
                Ok(text_lit(ty, number.to_string()))
            }
            SNode::Bool { value } => Ok(Expr::lit(BOOL, LitValue::Bool(*value))),
            SNode::Str { value } => Ok(text_lit(STRING, value.clone())),
            SNode::Unit => Ok(Expr::new(Node::Unit, UNIT)),
            SNode::None { absence } => {
                if !crate::translation::optional_value_rules::accept_optional_literal_absence(
                    absence,
                ) {
                    return Err(type_error(
                        "an absent optional value requires its concrete null or undefined tag",
                        span,
                    ));
                }
                if expected.is_some_and(|ty| {
                    matches!(ty, Type::Option { .. }) && !ty.accepts_absence(absence)
                }) {
                    return Err(type_error(
                        "an absent optional value disagrees with its retained absence policy",
                        span,
                    ));
                }
                // The generic JS checker retains an expected optional type,
                // otherwise absence carries Unit. Scoped Number proofs are
                // checked by their own independently selected adapter.
                let ty = expected
                    .filter(|ty| matches!(ty, Type::Option { .. }))
                    .cloned()
                    .unwrap_or_else(|| crate::translation::types::option(UNIT, absence.clone()));
                Ok(Expr::new(
                    Node::None {
                        absence: absence.clone(),
                    },
                    ty,
                ))
            }
            _ => {
                unreachable!("the original dispatcher selects only this helper's retained variants")
            }
        }
    }

    pub(super) fn check_conditional_expression(
        &mut self,
        node: &SExpr,
        env: &Env,
        path: &[String],
        context: ExpectedType<'_>,
        allow_literal: bool,
    ) -> Result<Expr> {
        let span = node.span;
        let expected = context.concrete();
        match &node.node {
            SNode::If {
                cond,
                then,
                otherwise,
            } => {
                let checked = self.expr(cond, env, path, None, false)?;
                let cond = self.boolean_value(checked, span)?;
                let mut then_expr =
                    self.expr_with_context(then, env, path, context, allow_literal)?;
                let else_expected = if then_expr.ty.is_literal() {
                    expected.cloned()
                } else {
                    Some(then_expr.ty.clone())
                };
                let else_context = if then_expr.ty.is_literal() {
                    context
                } else {
                    ExpectedType::Concrete(else_expected.as_ref())
                };
                let mut else_expr =
                    self.expr_with_context(otherwise, env, path, else_context, allow_literal)?;
                if then_expr.ty.is_literal() && !else_expr.ty.is_literal() {
                    then_expr = self.expr(then, env, path, Some(&else_expr.ty), false)?;
                }
                if then_expr.ty.is_literal() {
                    then_expr = self.expr(then, env, path, Some(&self.default_number()), false)?;
                    else_expr = self.expr(otherwise, env, path, Some(&then_expr.ty), false)?;
                }
                let (then_expr, else_expr) =
                    self.unify_branches(then_expr, else_expr, expected, span)?;
                let ty = then_expr.ty.clone();
                Ok(Expr::new(
                    Node::If {
                        cond: Box::new(cond),
                        then: Box::new(then_expr),
                        otherwise: Box::new(else_expr),
                    },
                    ty,
                ))
            }
            _ => {
                unreachable!("the original dispatcher selects only this helper's retained variants")
            }
        }
    }

    pub(super) fn check_binding_expression(
        &mut self,
        node: &SExpr,
        env: &Env,
        path: &[String],
        context: ExpectedType<'_>,
        allow_literal: bool,
    ) -> Result<Expr> {
        let span = node.span;
        let _expected = context.concrete();
        match &node.node {
            SNode::Let {
                name,
                ty,
                value,
                body,
            } => {
                let declared = match ty.as_ref().and_then(SurfaceBindingTypeAnnotation::as_ref) {
                    Some(ty) => Some(self.resolve_type(Some(ty), path, span)?),
                    None => None,
                };
                let mut value = self.expr(value, env, path, declared.as_ref(), false)?;
                if let Some(declared) = &declared {
                    value = self.coerce(value, declared, span)?;
                }
                let mut inner = env.clone();
                inner.bind_local(name, value.ty.clone());
                if matches!(value.ty, Type::Tuple { .. })
                    && immutable_product_value(&value, &env.immutable_products)
                {
                    inner.immutable_products.insert(name.clone(), true);
                }
                let body = self.expr_with_context(body, &inner, path, context, allow_literal)?;
                let ty = body.ty.clone();
                Ok(Expr::new(
                    Node::Let {
                        name: name.clone(),
                        value: Box::new(value),
                        body: Box::new(body),
                    },
                    ty,
                ))
            }
            _ => {
                unreachable!("the original dispatcher selects only this helper's retained variants")
            }
        }
    }
}
