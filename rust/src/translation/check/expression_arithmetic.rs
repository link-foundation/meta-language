impl Checker {
    #[allow(clippy::too_many_arguments)]
    pub(super) fn binary(
        &mut self,
        op: BinaryOp,
        left: &SExpr,
        right: &SExpr,
        rounding: Option<Rounding>,
        span: Option<Span>,
        env: &Env,
        path: &[String],
        expected: Option<&Type>,
        allow_literal: bool,
    ) -> Result<Expr> {
        match op {
            BinaryOp::And | BinaryOp::Or => {
                let (mut then_env, mut else_env) = self.own_property_branch_environments(left, env);
                let outcome = op == BinaryOp::And;
                let right_env = if outcome {
                    &mut then_env
                } else {
                    &mut else_env
                };
                self.apply_javascript_optional_guard(left, right_env, outcome, 0);
                let left = self.expr(left, env, path, None, false)?;
                let right = self.expr(right, right_env, path, None, false)?;
                self.logical_values(op, left, right, span)
            }
            BinaryOp::Concat => {
                let left = self.expr(left, env, path, Some(&STRING), false)?;
                let left = self.coerce(left, &STRING, span)?;
                let right = self.expr(right, env, path, Some(&STRING), false)?;
                let right = self.coerce(right, &STRING, span)?;
                Ok(plain_binary(op, left, right, STRING))
            }
            _ if comparison(op) => {
                if self.language == Language::JavaScript {
                    let mut checked_left = self.expr(left, env, path, None, true)?;
                    let mut checked_right = self.expr(right, env, path, None, true)?;
                    if checked_left.ty.is_literal() {
                        checked_left =
                            self.expr(left, env, path, Some(&self.default_number()), false)?;
                    }
                    if checked_right.ty.is_literal() {
                        checked_right =
                            self.expr(right, env, path, Some(&self.default_number()), false)?;
                    }
                    self.refuse_callable_data_observation(&checked_left.ty, "comparison", span)?;
                    self.refuse_callable_data_observation(&checked_right.ty, "comparison", span)?;
                    if let Some(value) =
                        self.primitive_comparison(op, &checked_left, &checked_right)?
                    {
                        return Ok(value);
                    }
                    if let Some(value) =
                        self.optional_comparison(op, checked_left, checked_right, span)?
                    {
                        return Ok(value);
                    }
                }
                let (left, right) = self.operands(left, right, env, path, span)?;
                self.refuse_callable_data_observation(&left.ty, "comparison", span)?;
                if self.language == Language::JavaScript && crate::translation::reference_identity_rules::accept_reference_identity_comparison(
                    op_name(op), left.ty.kind(), right.ty.kind(),
                    left.ty.is_reference(), right.ty.is_reference(), left.ty == right.ty,
                ) {
                    let domain = left.ty.clone();
                    return Ok(Expr::new(binary_node(op, Box::new(left), Box::new(right), Some(domain), None, None, None), BOOL));
                }
                if matches!(left.ty, Type::Tuple { .. }) {
                    return Err(unsupported(
                        "comparison of tuples",
                        "tuple value equality and JavaScript reference identity need an explicit operational comparison model; compare positional elements",
                        span,
                    ));
                }
                if matches!(left.ty, Type::Data { .. } | Type::Unit) {
                    return Err(unsupported(
                        "structural equality of data values",
                        "comparisons of data values are not in the portable core",
                        span,
                    ));
                }
                if left.ty.element().is_some() {
                    return Err(unsupported(
                        "comparison of arrays",
                        "=== of two arrays compares which array each is, which a value translation does not keep; compare their elements",
                        span,
                    ));
                }
                let string_order =
                    crate::translation::string_order_rules::read_string_order_operation(
                        self.language.as_str(),
                        op_name(op),
                        if left.ty == STRING { "string" } else { "" },
                    );
                if !string_order.is_empty() {
                    return Ok(Expr::new(
                        Node::StringTest {
                            op: string_order,
                            string: Box::new(left),
                            search: Box::new(right),
                        },
                        BOOL,
                    ));
                }
                if !matches!(op, BinaryOp::Eq | BinaryOp::Ne) && !left.ty.is_ordered() {
                    return Err(type_error(format!("ordering on {}", left.ty.key()), span));
                }
                let domain = left.ty.clone();
                Ok(Expr::new(
                    binary_node(
                        op,
                        Box::new(left),
                        Box::new(right),
                        Some(domain),
                        None,
                        None,
                        None,
                    ),
                    BOOL,
                ))
            }
            BinaryOp::Plus => self.plus(
                left,
                right,
                rounding,
                span,
                env,
                path,
                expected,
                allow_literal,
            ),
            _ => self.arithmetic(
                op,
                left,
                right,
                rounding,
                span,
                env,
                path,
                expected,
                allow_literal,
                None,
            ),
        }
    }

    /// JavaScript's `+` concatenates when either operand is a string, reading
    /// the other as `String(value)`, and adds numbers otherwise.
    #[allow(clippy::too_many_arguments)]
    pub(super) fn plus(
        &mut self,
        left_surface: &SExpr,
        right_surface: &SExpr,
        rounding: Option<Rounding>,
        span: Option<Span>,
        env: &Env,
        path: &[String],
        expected: Option<&Type>,
        allow_literal: bool,
    ) -> Result<Expr> {
        let left = self.expr(left_surface, env, path, None, true)?;
        let right = self.expr(right_surface, env, path, None, true)?;
        if left.ty != Type::String && right.ty != Type::String {
            return self.arithmetic(
                BinaryOp::Add,
                left_surface,
                right_surface,
                rounding,
                span,
                env,
                path,
                expected,
                allow_literal,
                Some((left, right)),
            );
        }
        let left = self.text(left, left_surface, env, path)?;
        let right = self.text(right, right_surface, env, path)?;
        Ok(plain_binary(BinaryOp::Concat, left, right, STRING))
    }

    pub(super) fn text(
        &mut self,
        value: Expr,
        surface: &SExpr,
        env: &Env,
        path: &[String],
    ) -> Result<Expr> {
        if value.ty == Type::String {
            return Ok(value);
        }
        let checked = if value.ty.is_literal() {
            self.expr(surface, env, path, Some(&self.default_number()), false)?
        } else {
            value
        };
        if matches!(checked.ty, Type::Data { .. } | Type::Unit) {
            return Err(unsupported(
                "string conversion of structured values",
                &format!("String({}) has no portable textual form", checked.ty.key()),
                surface.span,
            ));
        }
        if let Type::Option { element, .. } = &checked.ty {
            if !crate::translation::optional_value_rules::accept_optional_primitive_kind(
                element.kind(),
            ) {
                return Err(unsupported(
                    "string conversion of nullable structured values",
                    "this conversion needs its source structured-value formatter",
                    surface.span,
                ));
            }
            return Ok(Expr::new(
                Node::OptionalText {
                    arg: Box::new(checked),
                    console: false,
                },
                STRING,
            ));
        }
        if checked.ty.element().is_some() {
            return Err(array_text(surface.span));
        }
        Ok(Expr::new(
            Node::ToString {
                arg: Box::new(checked),
                console: false,
            },
            STRING,
        ))
    }

    /// `checked` holds operands already checked without an expected type, as `plus` does.
    #[allow(clippy::too_many_arguments)]
    pub(super) fn arithmetic(
        &mut self,
        op: BinaryOp,
        left_surface: &SExpr,
        right_surface: &SExpr,
        rounding: Option<Rounding>,
        span: Option<Span>,
        env: &Env,
        path: &[String],
        expected: Option<&Type>,
        allow_literal: bool,
        checked: Option<(Expr, Expr)>,
    ) -> Result<Expr> {
        if !matches!(
            op,
            BinaryOp::Add | BinaryOp::Sub | BinaryOp::Mul | BinaryOp::Div | BinaryOp::Rem
        ) {
            return Err(type_error(
                format!("unknown operator {}", op_name(op)),
                span,
            ));
        }
        let numeric_expected = expected.filter(|ty| ty.is_numeric()).cloned();
        let (checked_left, checked_right) = match checked {
            Some((left, right)) => (Some(left), Some(right)),
            None => (None, None),
        };
        let mut left = match checked_left {
            Some(left) if !left.ty.is_literal() => left,
            _ => self.expr(left_surface, env, path, numeric_expected.as_ref(), true)?,
        };
        let mut right = match checked_right {
            Some(right) if !right.ty.is_literal() => right,
            _ => {
                let right_expected = if left.ty.is_literal() {
                    numeric_expected.clone()
                } else {
                    Some(left.ty.clone())
                };
                self.expr(right_surface, env, path, right_expected.as_ref(), true)?
            }
        };
        if left.ty.is_literal() {
            let left_expected = if right.ty.is_literal() {
                numeric_expected.clone()
            } else {
                Some(right.ty.clone())
            };
            left = self.expr(left_surface, env, path, left_expected.as_ref(), true)?;
        }
        if left.ty.is_literal() && right.ty.is_literal() {
            if allow_literal {
                return Ok(text_lit(Type::Literal, "0"));
            }
            left = self.expr(left_surface, env, path, Some(&self.default_number()), false)?;
            right = self.expr(right_surface, env, path, Some(&left.ty), false)?;
        }
        if right.ty.is_literal() {
            right = self.expr(right_surface, env, path, Some(&left.ty), false)?;
        }
        if left.ty.is_literal() {
            left = self.expr(left_surface, env, path, Some(&right.ty), false)?;
        }
        if self.language == Language::JavaScript {
            left = self.number_value(left, span)?;
            right = self.number_value(right, span)?;
        }
        if self.language == Language::JavaScript && (left.ty == Type::Nat || right.ty == Type::Nat)
        {
            // BigInt arithmetic is integer arithmetic even on guarded naturals: `n - 1n` is -1n for n = 0n.
            let widen = |value: Expr| match &value.node {
                Node::Lit { value } => Ok(Expr::lit(INT, value.clone())),
                _ => coerce(value, &INT, Language::JavaScript, span, None),
            };
            left = widen(left)?;
            right = widen(right)?;
        }
        if let Some(failure) =
            self.materialize_numeric_failure(op.name(), &[left.clone(), right.clone()], expected)
        {
            return Ok(failure);
        }
        let (left, right) = self.unify(left, right, span)?;
        if !left.ty.is_ordered() {
            return Err(type_error(format!("arithmetic on {}", left.ty.key()), span));
        }
        if let Some(division) = self.materialize_big_integer_division(op, &left, &right) {
            return Ok(division);
        }
        let ty = left.ty.clone();
        let (semantics, rounding, by_zero) = arithmetic_semantics(self.language, op, &ty, rounding);
        Ok(Expr::new(
            binary_node(
                op,
                Box::new(left),
                Box::new(right),
                Some(ty.clone()),
                Some(semantics),
                rounding,
                by_zero,
            ),
            ty,
        ))
    }
}
