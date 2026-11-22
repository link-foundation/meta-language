fn method_call(
    receiver: SExpr,
    method: &Token,
    args: Vec<SExpr>,
    range: Option<Span>,
) -> Result<SExpr> {
    let expect = |count: usize| {
        if args.len() == count {
            Ok(())
        } else {
            Err(type_error(
                format!(
                    "{} expects {count} arguments but got {}",
                    method.value,
                    args.len()
                ),
                range,
            ))
        }
    };
    let mapping =
        crate::translation::frontend_rules::read_string_map_operation("Rust", &method.value);
    if !mapping.is_empty() {
        expect(0)?;
        return Ok(SExpr::new(
            SNode::StringMap {
                op: mapping,
                object: Box::new(receiver),
            },
            range,
        ));
    }
    let operation =
        crate::translation::frontend_rules::read_string_test_operation("Rust", &method.value);
    if !operation.is_empty() {
        expect(1)?;
        return Ok(SExpr::new(
            SNode::StringTest {
                op: operation,
                object: Box::new(receiver),
                search: Box::new(args.into_iter().next().expect("one search argument")),
            },
            range,
        ));
    }
    match method.value.as_str() {
        "clone" | "to_owned" | "as_str" => {
            expect(0)?;
            Ok(receiver)
        }
        "to_string" => {
            expect(0)?;
            Ok(SExpr::new(
                SNode::ToString {
                    arg: Box::new(receiver),
                },
                range,
            ))
        }
        "div_euclid" | "rem_euclid" => {
            expect(1)?;
            let op = if method.value == "div_euclid" {
                BinaryOp::Div
            } else {
                BinaryOp::Rem
            };
            let right = args
                .into_iter()
                .next()
                .unwrap_or_else(|| unreachable!("one argument"));
            Ok(SExpr::new(
                SNode::Binary {
                    op,
                    left: Box::new(receiver),
                    right: Box::new(right),
                    rounding: Some(Rounding::Euclid),
                },
                range,
            ))
        }
        _ => Err(unsupported(
            &format!("method {}", method.value),
            "outside the portable core",
            range,
        )),
    }
}

/// An imported path from the crate root, so it means the same wherever it is used.
fn absolute(segments: &[String], path: &[String], range: Option<Span>) -> Result<Vec<String>> {
    let crate_root = || vec!["crate".to_owned()];
    if segments[0] == "crate" {
        return Ok(segments.to_vec());
    }
    if segments[0] == "self" {
        let mut result = crate_root();
        result.extend_from_slice(path);
        result.extend_from_slice(&segments[1..]);
        return Ok(result);
    }
    let up = segments
        .iter()
        .take_while(|segment| *segment == "super")
        .count();
    if up > path.len() {
        return Err(type_error("super beyond crate root", range));
    }
    let mut result = crate_root();
    result.extend_from_slice(&path[..path.len() - up]);
    result.extend_from_slice(&segments[up..]);
    Ok(result)
}

fn prop_of(expr: SExpr) -> SProp {
    let node = match expr.node {
        SNode::Binary {
            op:
                op @ (BinaryOp::Eq
                | BinaryOp::Ne
                | BinaryOp::Lt
                | BinaryOp::Le
                | BinaryOp::Gt
                | BinaryOp::Ge),
            left,
            right,
            ..
        } => {
            let comparison = SComparison {
                left: *left,
                right: *right,
                reference: false,
                same_value: false,
            };
            match op {
                BinaryOp::Eq => SPropNode::Eq(comparison),
                BinaryOp::Ne => SPropNode::Ne(comparison),
                BinaryOp::Lt => SPropNode::Lt(comparison),
                BinaryOp::Le => SPropNode::Le(comparison),
                BinaryOp::Gt => SPropNode::Gt(comparison),
                _ => SPropNode::Ge(comparison),
            }
        }
        SNode::Binary {
            op: BinaryOp::And,
            left,
            right,
            ..
        } => SPropNode::And {
            left: Box::new(prop_of(*left)),
            right: Box::new(prop_of(*right)),
        },
        SNode::Binary {
            op: BinaryOp::Or,
            left,
            right,
            ..
        } => SPropNode::Or {
            left: Box::new(prop_of(*left)),
            right: Box::new(prop_of(*right)),
        },
        SNode::Unary {
            op: UnaryOp::Not,
            arg,
        } => SPropNode::Not {
            arg: Box::new(prop_of(*arg)),
        },
        node => SPropNode::Bool {
            expr: SExpr { node, ..expr },
        },
    };
    SProp { node, span: None }
}

/// The value a pattern matched, for `name @ p`.
fn pattern_value(pattern: &SPattern, range: Option<Span>) -> Result<SExpr> {
    let node = match &pattern.node {
        SPatternNode::BindOrCtor { name } => SNode::Name {
            path: vec![name.clone()],
        },
        SPatternNode::NumLit { value, negative } => SNode::Num {
            value: value.clone(),
            ty: None,
            negative: *negative,
            unit: false,
        },
        SPatternNode::BoolLit { value, .. } => SNode::Bool { value: *value },
        SPatternNode::TypedNumLit {
            value,
            ty,
            negative,
            ..
        } => SNode::Num {
            value: value.clone(),
            ty: Some(ty.clone()),
            negative: *negative,
            unit: false,
        },
        SPatternNode::ByteLit { value } => SNode::Num {
            value: value.clone(),
            ty: Some(Type::Fixed {
                bits: 8,
                signed: false,
            }),
            negative: false,
            unit: false,
        },
        SPatternNode::StrLit { value } => SNode::Str {
            value: value.clone(),
        },
        SPatternNode::Ctor { path, args } if args.is_empty() => SNode::Name { path: path.clone() },
        SPatternNode::Ctor { path, args } => SNode::App {
            func: Box::new(SExpr::new(SNode::Name { path: path.clone() }, range)),
            args: args
                .iter()
                .map(|arg| pattern_value(arg, range))
                .collect::<Result<_>>()?,
        },
        _ => {
            return Err(unsupported(
                "@ binding",
                "the aliased pattern must bind every field",
                range,
            ));
        }
    };
    Ok(SExpr::new(node, range))
}

const fn is_if_or_match(expr: &SExpr) -> bool {
    expr.block || matches!(expr.node, SNode::If { .. } | SNode::Match { .. })
}

fn operator(token: &Token, table: fn(&str) -> Option<BinaryOp>) -> Option<BinaryOp> {
    if token.kind == TokenKind::Punct {
        table(&token.value)
    } else {
        None
    }
}

fn binary(op: BinaryOp, left: SExpr, right: SExpr, token: &Token) -> SExpr {
    let range = joined(left.span, right.span, token);
    SExpr::new(
        SNode::Binary {
            op,
            left: Box::new(left),
            right: Box::new(right),
            rounding: None,
        },
        range,
    )
}

#[allow(clippy::unnecessary_wraps)] // nodes and errors take an optional span
fn joined(left: Option<Span>, right: Option<Span>, token: &Token) -> Option<Span> {
    Some(Span::new(
        left.map_or(token.start, |left| left.start),
        right.map_or(token.end, |right| right.end),
    ))
}

#[allow(clippy::unnecessary_wraps)] // nodes and errors take an optional span
fn span(from: &Token, to: &Token) -> Option<Span> {
    let to_start = if to.kind == TokenKind::Eof {
        from.end
    } else {
        to.start
    };
    Some(Span::new(from.start, from.end.max(to_start)))
}
