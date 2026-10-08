//! Expression checking: operators, arithmetic, applications and constructors.

use super::arrays::array_text;
use super::{
    BOOL, BinaryOp, Checker, Ctor, Decimal, Decl, Entry, Env, Expr, FLOAT, Flavor, INT, Language,
    LitValue, Node, Param, Result, Rounding, SExpr, SNode, STRING, Semantics, Span, Type, UNIT,
    UnaryOp, arithmetic_semantics, binary_node, cast_to, coerce, comparison, data, fixed_bounds,
    negate_number, op_name, plain_binary, text_lit, type_error, unsupported,
};
use crate::translation::frontend_rules::accept_argument_count;

impl Checker {
    pub(super) fn coerce(&self, value: Expr, ty: &Type, span: Option<Span>) -> Result<Expr> {
        coerce(value, ty, self.language, span, None)
    }

    pub(super) fn expr(
        &mut self,
        node: &SExpr,
        env: &Env,
        path: &[String],
        expected: Option<&Type>,
        allow_literal: bool,
    ) -> Result<Expr> {
        let mut result = self.expr_inner(node, env, path, expected, allow_literal)?;
        if node.span.is_some() && result.span.is_none() {
            result.span = node.span;
        }
        Ok(result)
    }

    #[allow(clippy::too_many_lines)]
    pub(super) fn expr_inner(
        &mut self,
        node: &SExpr,
        env: &Env,
        path: &[String],
        expected: Option<&Type>,
        allow_literal: bool,
    ) -> Result<Expr> {
        let span = node.span;
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
            SNode::Name { path: segments } => {
                if let Some(name) = env.local(node) {
                    return Ok(Expr::var(name, env.vars[name].clone()));
                }
                let _ = segments;
                self.application(node, &[], env, path, expected, span)
            }
            SNode::DotCtor { .. } => self.application(node, &[], env, path, expected, span),
            SNode::App { func, args } => self.application(func, args, env, path, expected, span),
            SNode::Field { field, object } => {
                if field == "length"
                    && let Some(length) = self.length_field(object, span, env, path)?
                {
                    return Ok(length);
                }
                Err(unsupported(
                    "field access",
                    &format!("field {field} is only portable inside a constructor match"),
                    span,
                ))
            }
            SNode::TypeOf { arg } => self.type_query(arg, env, path, span),
            SNode::Unary {
                op: UnaryOp::Not,
                arg,
            } => {
                let arg = self.expr(arg, env, path, Some(&BOOL), false)?;
                Ok(Expr::new(
                    Node::Unary {
                        op: UnaryOp::Not,
                        arg: Box::new(self.coerce(arg, &BOOL, span)?),
                        semantics: None,
                    },
                    BOOL,
                ))
            }
            SNode::Unary {
                op: UnaryOp::Neg,
                arg,
            } => {
                if let SNode::Num {
                    value,
                    ty,
                    negative,
                    ..
                } = &arg.node
                {
                    let flipped = SExpr {
                        node: SNode::Num {
                            value: value.clone(),
                            ty: ty.clone(),
                            negative: !negative,
                            unit: false,
                        },
                        span,
                        block: arg.block,
                        tag_test: None,
                    };
                    return self.expr(&flipped, env, path, expected, allow_literal);
                }
                let numeric = expected.filter(|ty| ty.is_numeric());
                let arg = self.expr(arg, env, path, numeric, false)?;
                if arg.ty.is_float() {
                    return Ok(Expr::new(
                        Node::Unary {
                            op: UnaryOp::Neg,
                            arg: Box::new(arg),
                            semantics: Some(Semantics::Ieee),
                        },
                        FLOAT,
                    ));
                }
                if arg.ty == Type::Nat && self.language == Language::JavaScript {
                    // A guarded natural is still a BigInt, and its negation an integer.
                    return Ok(Expr::new(
                        Node::Unary {
                            op: UnaryOp::Neg,
                            arg: Box::new(self.coerce(arg, &INT, span)?),
                            semantics: Some(Semantics::Exact),
                        },
                        INT,
                    ));
                }
                if !matches!(arg.ty, Type::Int | Type::Fixed { signed: true, .. }) {
                    return Err(type_error(format!("negation of {}", arg.ty.key()), span));
                }
                let ty = arg.ty.clone();
                let semantics = if matches!(ty, Type::Fixed { .. }) {
                    Semantics::Checked
                } else {
                    Semantics::Exact
                };
                Ok(Expr::new(
                    Node::Unary {
                        op: UnaryOp::Neg,
                        arg: Box::new(arg),
                        semantics: Some(semantics),
                    },
                    ty,
                ))
            }
            SNode::Binary {
                op,
                left,
                right,
                rounding,
            } => self.binary(
                *op,
                left,
                right,
                *rounding,
                span,
                env,
                path,
                expected,
                allow_literal,
            ),
            SNode::If {
                cond,
                then,
                otherwise,
            } => {
                let checked = self.expr(cond, env, path, Some(&BOOL), false)?;
                let cond = self.coerce(checked, &BOOL, span)?;
                let mut then_expr = self.expr(then, env, path, expected, allow_literal)?;
                let else_expected = if then_expr.ty.is_literal() {
                    expected.cloned()
                } else {
                    Some(then_expr.ty.clone())
                };
                let mut else_expr =
                    self.expr(otherwise, env, path, else_expected.as_ref(), allow_literal)?;
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
            SNode::Let {
                name,
                ty,
                value,
                body,
            } => {
                let declared = match ty {
                    Some(ty) => Some(self.resolve_type(Some(ty), path, span)?),
                    None => None,
                };
                let mut value = self.expr(value, env, path, declared.as_ref(), false)?;
                if let Some(declared) = &declared {
                    value = self.coerce(value, declared, span)?;
                }
                let mut inner = env.clone();
                inner.bind_local(name, value.ty.clone());
                let body = self.expr(body, &inner, path, expected, allow_literal)?;
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
            SNode::Match { scrutinees, rows } => {
                let compiled = self.compile_match(scrutinees, rows, span, env, path)?;
                self.expr(&compiled, env, path, expected, allow_literal)
            }
            SNode::Match1 { scrutinee, cases } => {
                self.match_cases(scrutinee, cases, span, env, path, expected)
            }
            SNode::StringMap { .. } => self.string_map(node, env, path),
            SNode::StringTest { .. } => self.string_test(node, env, path),
            SNode::ToString { arg } => {
                let arg = self.expr(arg, env, path, None, false)?;
                if arg.ty == Type::String {
                    return Ok(arg);
                }
                if arg.ty.element().is_some() {
                    return Err(array_text(span));
                }
                if !arg.ty.is_ordered() && arg.ty != Type::Bool {
                    return Err(type_error(format!("toString of {}", arg.ty.key()), span));
                }
                Ok(Expr::new(
                    Node::ToString {
                        arg: Box::new(arg),
                        console: false,
                    },
                    STRING,
                ))
            }
            SNode::Show { arg, style } => self.show(arg, env, path, *style),
            SNode::Cast {
                arg,
                to,
                from,
                flavor,
            } => {
                let to = self.resolve_type(Some(to), path, span)?;
                let arg = self.expr(arg, env, path, from.as_ref(), false)?;
                cast_to(arg, &to, *flavor, span)
            }
            SNode::Abort { message } => {
                let Some(expected) = expected else {
                    return Err(type_error("abort needs a known result type", span));
                };
                Ok(Expr::new(
                    Node::Abort {
                        message: message.clone(),
                    },
                    expected.clone(),
                ))
            }
            SNode::CtorObject { tag, fields } => {
                self.ctor_object(tag, fields, span, env, path, expected)
            }
            SNode::Print { expr, style, body } => {
                let text = self.show(expr, env, path, *style)?;
                let body = self.expr(body, env, path, expected, allow_literal)?;
                let ty = body.ty.clone();
                Ok(Expr::new(
                    Node::Print {
                        text: Box::new(text),
                        body: Box::new(body),
                    },
                    ty,
                ))
            }
            SNode::Length { object, integer } => self.length(object, *integer, span, env, path),
            SNode::Array { items, element } => {
                self.array_literal(items, element.as_ref(), span, env, path, expected)
            }
            SNode::Index { object, index } => self.index(object, index, span, env, path),
            SNode::Math { op, name, args } => self.math(op, name, args, span, env, path),
            other => Err(type_error(
                format!("unknown expression {}", other.kind()),
                span,
            )),
        }
    }

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
            BinaryOp::And | BinaryOp::Or | BinaryOp::Concat => {
                let ty = if op == BinaryOp::Concat { STRING } else { BOOL };
                let checked = self.expr(left, env, path, Some(&ty), false)?;
                let left = self.coerce(checked, &ty, span)?;
                let checked = self.expr(right, env, path, Some(&ty), false)?;
                let right = self.coerce(checked, &ty, span)?;
                Ok(plain_binary(op, left, right, ty))
            }
            _ if comparison(op) => {
                let (left, right) = self.operands(left, right, env, path, span)?;
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
        let (left, right) = self.unify(left, right, span)?;
        if !left.ty.is_ordered() {
            return Err(type_error(format!("arithmetic on {}", left.ty.key()), span));
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

    pub(super) fn application(
        &mut self,
        head: &SExpr,
        args: &[SExpr],
        env: &Env,
        path: &[String],
        expected: Option<&Type>,
        span: Option<Span>,
    ) -> Result<Expr> {
        let segments = match &head.node {
            SNode::DotCtor { name } => {
                let Some(Type::Data { name: data_name }) = expected else {
                    return Err(type_error(
                        format!("cannot infer the type of .{name}"),
                        span,
                    ));
                };
                let decl = self.data_decl(data_name)?;
                let full_name = decl.full_name.clone();
                let Some(ctor) = decl.ctors.iter().find(|ctor| ctor.name == *name).cloned() else {
                    return Err(type_error(
                        format!("{data_name} has no constructor {name}"),
                        span,
                    ));
                };
                return self.construct(&full_name, &ctor, args, env, path, span);
            }
            SNode::Name { path } => path,
            _ => {
                return Err(unsupported(
                    "higher-order application",
                    "only named functions and constructors can be applied",
                    span,
                ));
            }
        };
        if let Some(name) = env.local(head) {
            return Err(unsupported(
                "higher-order application",
                &format!("local {name} is not a function in the portable core"),
                span,
            ));
        }
        let Some(entry) = self.lookup(segments, path, span)? else {
            return Err(type_error(
                format!("unknown name {}", segments.join(".")),
                span,
            ));
        };
        let decl_name = match entry {
            Entry::Ctor { data, name } => {
                let decl = self.data_decl(&data)?;
                let Some(ctor) = decl.ctors.iter().find(|ctor| ctor.name == name).cloned() else {
                    return Err(type_error(
                        format!("{data} has no constructor {name}"),
                        span,
                    ));
                };
                return self.construct(&data, &ctor, args, env, path, span);
            }
            Entry::External(name) => {
                return self.external_application(&name, segments, args, env, path, span);
            }
            Entry::Decl(name) => name,
        };
        let Some(Decl::Fn(entry)) = self.decls.get(&decl_name) else {
            return Err(type_error(
                format!("{} is not a function", segments.join(".")),
                span,
            ));
        };
        if !self.signatures.contains(&decl_name) {
            return Err(type_error(
                format!("{} is used before its signature is known", entry.full_name),
                span,
            ));
        }
        let (full_name, params, ret, module_path) = (
            entry.full_name.clone(),
            entry.params.clone(),
            entry.ret.clone(),
            entry.module_path.clone(),
        );
        self.checked_call(full_name, &params, ret, &module_path, args, env, path, span)
    }

    /// A call of the function `full_name` with its checked arguments: an
    /// omitted argument takes its parameter's default, checked in `module_path`.
    #[allow(clippy::too_many_arguments)]
    pub(super) fn checked_call(
        &mut self,
        full_name: String,
        params: &[Param],
        ret: Type,
        module_path: &[String],
        args: &[SExpr],
        env: &Env,
        path: &[String],
        span: Option<Span>,
    ) -> Result<Expr> {
        let mut arguments: Vec<(&SExpr, bool)> = args.iter().map(|arg| (arg, false)).collect();
        #[allow(clippy::cast_precision_loss)]
        let argument_count = args.len() as f64;
        if args.len() < params.len()
            && accept_argument_count(
                params
                    .iter()
                    .map(|param| param.default_value.is_some())
                    .collect(),
                argument_count,
            )
        {
            arguments.extend(params[args.len()..].iter().map(|param| {
                (
                    param
                        .default_value
                        .as_ref()
                        .expect("omitted parameter has a default"),
                    true,
                )
            }));
        }
        if arguments.len() != params.len() {
            if args.len() < params.len() {
                return Err(unsupported(
                    "partial application",
                    &format!("{full_name} expects {} arguments", params.len()),
                    span,
                ));
            }
            return Err(type_error(
                format!(
                    "{full_name} expects {} arguments but got {}",
                    params.len(),
                    args.len()
                ),
                span,
            ));
        }
        let mut checked = Vec::new();
        let closed_env = Env::default();
        for ((arg, closed_default), param) in arguments.iter().zip(params) {
            // A guarded argument is an integer the guard checks, wherever in it a negative value arises.
            let expected = if param.guard.is_some() && !matches!(arg.node, SNode::Num { .. }) {
                &INT
            } else {
                &param.ty
            };
            let value = self.expr(
                arg,
                if *closed_default { &closed_env } else { env },
                if *closed_default { module_path } else { path },
                Some(expected),
                false,
            )?;
            let flavor = param.guard.is_some().then_some(Flavor::Checked);
            let mut value = coerce(value, &param.ty, self.language, arg.span.or(span), flavor)?;
            if let (
                Node::Cast {
                    flavor: Flavor::Checked,
                    message,
                    order,
                    ..
                },
                Some(guard),
            ) = (&mut value.node, &param.guard)
            {
                *message = Some(guard.message.clone());
                *order = Some(guard.order);
            }
            checked.push(value);
        }
        Ok(self.guard_order(full_name, checked, ret))
    }

    /// A JavaScript call evaluates every argument, then the callee's guards run
    /// in their order. Checking each argument where it is passed is the same
    /// unless a later argument could print or abort, or the guards run in
    /// another order; then the arguments are bound first and checked after.
    fn guard_order(&mut self, func: String, args: Vec<Expr>, ret: Type) -> Expr {
        let guarded: Vec<(usize, usize)> = args
            .iter()
            .enumerate()
            .filter_map(|(index, arg)| guard_of(arg).map(|(order, _)| (index, order)))
            .collect();
        let Some(&(first, _)) = guarded.first() else {
            return Expr::new(Node::Call { func, args }, ret);
        };
        let in_order = guarded.windows(2).all(|pair| pair[0].1 < pair[1].1);
        let later = args[first + 1..]
            .iter()
            .all(|arg| simple(guard_of(arg).map_or(arg, |(_, inner)| inner)));
        if in_order && later {
            return Expr::new(Node::Call { func, args }, ret);
        }
        let bound: Vec<(String, Expr)> = args
            .iter()
            .map(|arg| {
                let value = guard_of(arg).map_or(arg, |(_, inner)| inner).clone();
                (self.fresh_name("ml_a"), value)
            })
            .collect();
        let mut sorted = guarded;
        sorted.sort_by_key(|&(_, order)| order);
        let checks: Vec<(usize, String, Expr)> = sorted
            .into_iter()
            .map(|(index, _)| {
                let mut value = args[index].clone();
                if let Node::Cast { arg, .. } = &mut value.node {
                    let ty = arg.ty.clone();
                    **arg = Expr::var(bound[index].0.clone(), ty);
                }
                (index, self.fresh_name("ml_a"), value)
            })
            .collect();
        let call_args = args
            .iter()
            .enumerate()
            .map(|(index, arg)| {
                let name = checks
                    .iter()
                    .find(|(candidate, ..)| *candidate == index)
                    .map_or(&bound[index].0, |(_, name, _)| name);
                Expr::var(name.clone(), arg.ty.clone())
            })
            .collect();
        let call = Expr::new(
            Node::Call {
                func,
                args: call_args,
            },
            ret,
        );
        let bindings = bound
            .into_iter()
            .chain(checks.into_iter().map(|(_, name, value)| (name, value)));
        bindings.rev().fold(call, |body, (name, value)| {
            let ty = body.ty.clone();
            Expr::new(
                Node::Let {
                    name,
                    value: Box::new(value),
                    body: Box::new(body),
                },
                ty,
            )
        })
    }

    pub(super) fn construct(
        &mut self,
        data_name: &str,
        ctor: &Ctor,
        args: &[SExpr],
        env: &Env,
        path: &[String],
        span: Option<Span>,
    ) -> Result<Expr> {
        if args.len() != ctor.fields.len() {
            return Err(type_error(
                format!(
                    "{data_name}.{} expects {} fields but got {}",
                    ctor.name,
                    ctor.fields.len(),
                    args.len()
                ),
                span,
            ));
        }
        let mut checked = Vec::new();
        for (arg, field) in args.iter().zip(&ctor.fields) {
            let value = self.expr(arg, env, path, Some(&field.ty), false)?;
            checked.push(self.coerce(value, &field.ty, arg.span.or(span))?);
        }
        Ok(Expr::new(
            Node::Ctor {
                data: data_name.to_owned(),
                ctor: ctor.name.clone(),
                args: checked,
            },
            data(data_name),
        ))
    }

    pub(super) fn ctor_object(
        &mut self,
        tag: &str,
        fields: &[(String, SExpr)],
        span: Option<Span>,
        env: &Env,
        path: &[String],
        expected: Option<&Type>,
    ) -> Result<Expr> {
        let mut candidates = Vec::new();
        for name in &self.order {
            let Some(Decl::Data(entry)) = self.decls.get(name) else {
                continue;
            };
            if let Some(Type::Data { name: expected }) = expected
                && entry.full_name != *expected
            {
                continue;
            }
            if let Some(ctor) = entry.ctors.iter().find(|ctor| ctor.name == tag) {
                candidates.push((entry.full_name.clone(), ctor.clone()));
            }
        }
        if candidates.len() != 1 {
            return Err(type_error(
                if candidates.is_empty() {
                    format!("no data type has tag {tag}")
                } else {
                    format!("tag {tag} is ambiguous")
                },
                span,
            ));
        }
        let (data_name, ctor) = candidates.remove(0);
        let expected_names: Vec<&str> = ctor
            .fields
            .iter()
            .map(|field| field.name.as_str())
            .collect();
        if fields.len() != expected_names.len()
            || fields
                .iter()
                .any(|(name, _)| !expected_names.contains(&name.as_str()))
        {
            return Err(type_error(
                format!("{tag} expects fields {}", expected_names.join(", ")),
                span,
            ));
        }
        let args: Vec<SExpr> = expected_names
            .iter()
            .filter_map(|name| {
                fields
                    .iter()
                    .rev()
                    .find(|(field, _)| field == name)
                    .map(|(_, value)| value.clone())
            })
            .collect();
        self.construct(&data_name, &ctor, &args, env, path, span)
    }
}

/// The order of the guard a checked argument is, and the argument it checks.
fn guard_of(arg: &Expr) -> Option<(usize, &Expr)> {
    match &arg.node {
        Node::Cast {
            arg: inner,
            order: Some(order),
            ..
        } => Some((*order, inner)),
        _ => None,
    }
}

/// A value whose evaluation neither prints nor aborts.
fn simple(e: &Expr) -> bool {
    match &e.node {
        Node::Var { .. } | Node::Lit { .. } => true,
        Node::Cast {
            flavor: Flavor::Exact,
            arg,
            ..
        } => simple(arg),
        _ => false,
    }
}
