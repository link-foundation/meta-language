//! Actual syntax/type/origin transport into shared primitive Result plans.
use super::{
    BOOL, BinaryOp, FLOAT, Flavor, Options, Result, RustParser, SExpr, SNode, SourceOrigin, Span,
    Token, TokenKind, Type, UnaryOp, rust_fixed_type, type_error, unsupported,
};
use crate::translation::{
    rust_primitive_from_rules as from_rules, rust_primitive_selection_rules as argument_rules,
    rust_primitive_try_from_rules as rules, rust_saturating_integer_rules as bounds,
    source_signatures::SourceOwner,
};
struct Head {
    start: Token,
    absolute: bool,
    offset: usize,
    resolved: Vec<String>,
    shadowed: bool,
}
struct Facts {
    origin: SourceOrigin,
    kind: String,
    bits: String,
    signed: bool,
    owner: Option<SourceOwner>,
}
fn count(value: usize) -> f64 {
    u32::try_from(value).map_or(f64::INFINITY, f64::from)
}
impl Facts {
    fn path(&self) -> String {
        self.owner
            .as_ref()
            .map_or_else(String::new, |owner| owner.path.join("::"))
    }
    fn admitted(&self) -> bool {
        from_rules::accept_rust_primitive_from_source(
            &self.kind,
            &self.bits,
            self.signed,
            self.owner.as_ref().map_or("", |owner| owner.kind.as_str()),
            &self.path(),
            self.owner
                .as_ref()
                .map_or(-1.0, |owner| count(owner.arguments.len())),
            self.origin.proven,
            count(self.origin.wrappers.len()),
        )
    }
}
fn origin(ty: Type, owner: &str, path: &[String]) -> SourceOrigin {
    let mut value = SourceOrigin::with_type(ty, path, Vec::new());
    value.source_owner = Some(SourceOwner {
        kind: "primitive".to_owned(),
        path: vec![owner.to_owned()],
        arguments: Vec::new(),
    });
    value
}
fn facts(parser: &RustParser, node: &SExpr, path: &[String]) -> Facts {
    let origin = parser.origins.origin(node, path);
    let owner = origin.source_owner.clone().or_else(|| {
        (origin.ty == Some(FLOAT) && origin.float_source_model == Some("binary64 scalar")).then(
            || SourceOwner {
                kind: "primitive".to_owned(),
                path: vec!["f64".to_owned()],
                arguments: Vec::new(),
            },
        )
    });
    let (bits, signed) = match &origin.ty {
        Some(Type::Fixed { bits, signed }) => (bits.to_string(), *signed),
        _ => (String::new(), false),
    };
    Facts {
        kind: origin.ty.as_ref().map_or("", Type::kind).to_owned(),
        origin,
        bits,
        signed,
        owner,
    }
}
fn pop(stack: &mut Vec<SExpr>, range: Option<Span>) -> Result<SExpr> {
    stack.pop().ok_or_else(|| {
        type_error(
            "the shared primitive Result plan lost an earlier source term",
            range,
        )
    })
}
impl RustParser {
    fn primitive_try_head(&self, path: &[String]) -> Option<Head> {
        let start = self.peek();
        let absolute = self.cursor.is("::");
        let mut offset = usize::from(absolute);
        let mut segments = Vec::new();
        while self.cursor.peek_at(offset).kind == TokenKind::Identifier && segments.len() < 16 {
            segments.push(self.cursor.peek_at(offset).value.clone());
            offset += 1;
            if !self.cursor.is_at("::", offset) {
                break;
            }
            if self.cursor.peek_at(offset + 1).kind != TokenKind::Identifier {
                return None;
            }
            offset += 1;
        }
        if segments.len() < 2 {
            return None;
        }
        let resolved = if absolute {
            segments.clone()
        } else {
            self.resolve(&segments, path)
        };
        let root = segments.first().cloned().unwrap_or_default();
        let target = resolved.first().cloned().unwrap_or_default();
        let shadowed = !absolute
            && (self.origins.lookup(&[root.clone()], path).is_some()
                || self.origins.scope.contains_key(&root)
                || self.origins.lookup(&[target.clone()], path).is_some()
                || self.origins.scope.contains_key(&target));
        Some(Head {
            start,
            absolute,
            offset,
            resolved,
            shadowed,
        })
    }
    pub(in super::super) fn primitive_try_from_import(
        &self,
        segments: &[String],
        path: &[String],
    ) -> bool {
        let root = segments.first().cloned().unwrap_or_default();
        let shadowed = self.origins.lookup(&[root.clone()], path).is_some()
            || self.origins.scope.contains_key(&root)
            || self
                .aliases
                .get(&path.join("."))
                .is_some_and(|aliases| aliases.contains_key(&root));
        rules::accept_rust_primitive_try_from_import(
            &segments.join("::"),
            self.ordinary_completion_context(),
            shadowed,
        )
    }
    pub(super) fn read_primitive_try_from(&mut self, path: &[String]) -> Result<Option<SExpr>> {
        let Some(head) = self.primitive_try_head(path) else {
            return Ok(None);
        };
        if head
            .resolved
            .last()
            .is_none_or(|method| method != "try_from")
        {
            return Ok(None);
        }
        let owner = rules::read_rust_primitive_try_from_owner(
            &head.resolved,
            self.ordinary_completion_context(),
            head.shadowed,
            head.absolute,
        );
        if owner.is_empty() {
            return Ok(None);
        }
        if !self.cursor.is_at("(", head.offset) {
            return Err(unsupported(
                "Rust primitive TryFrom",
                "an associated conversion function value needs its own callable interpretation",
                Some(Span {
                    start: head.start.start,
                    end: self.cursor.peek_at(head.offset).end,
                }),
            ));
        }
        for _ in 0..=head.offset {
            self.cursor.advance();
        }
        let mut inputs = self.call_arguments(path)?;
        self.cursor.expect(")", Some("primitive TryFrom call"))?;
        if inputs.len() != 1 {
            return Err(type_error(
                "a primitive TryFrom call needs one actual source argument",
                Some(Span {
                    start: head.start.start,
                    end: self.cursor.peek().end,
                }),
            ));
        }
        if self.cursor.eat(".").is_none() || self.cursor.peek().kind != TokenKind::Identifier {
            return Err(unsupported(
                "Rust primitive TryFrom",
                "the actual Result needs an immediate admitted standard consumer",
                Some(Span {
                    start: head.start.start,
                    end: self.cursor.peek().end,
                }),
            ));
        }
        let consumer = self.cursor.advance();
        let arity = rules::read_rust_primitive_try_from_consumer_arity(&consumer.value);
        if arity < 0.0 {
            return Err(unsupported(
                "Rust primitive TryFrom",
                "the actual Result needs an immediate admitted standard consumer",
                Some(Span {
                    start: head.start.start,
                    end: consumer.end,
                }),
            ));
        }
        self.cursor
            .expect("(", Some("primitive TryFrom consumer"))?;
        let mut defaults = Vec::new();
        let mut mapper_owner = String::new();
        if arity > 0.0 {
            defaults.push(self.expr(path, Options::default())?);
            if arity == 2.0 {
                self.cursor.expect(",", Some("primitive TryFrom mapper"))?;
                let mapper = self
                    .primitive_try_head(path)
                    .filter(|mapper| {
                        mapper
                            .resolved
                            .last()
                            .is_some_and(|method| method == "from")
                    })
                    .ok_or_else(|| {
                        unsupported(
                            "Rust primitive TryFrom mapper",
                            "an actual admitted standard primitive From function item is required",
                            Some(Span {
                                start: self.cursor.peek().start,
                                end: self.cursor.peek().end,
                            }),
                        )
                    })?;
                mapper_owner = from_rules::read_rust_primitive_from_owner(
                    &mapper.resolved,
                    self.ordinary_completion_context(),
                    mapper.shadowed,
                    mapper.absolute,
                );
                if mapper_owner.is_empty()
                    || from_rules::plan_rust_primitive_from(&mapper_owner, &owner).is_empty()
                {
                    return Err(unsupported(
                        "Rust primitive TryFrom mapper",
                        "the actual standard primitive From implementation must exist for the successful target",
                        Some(Span {
                            start: mapper.start.start,
                            end: self.cursor.peek_at(mapper.offset - 1).end,
                        }),
                    ));
                }
                for _ in 0..mapper.offset {
                    self.cursor.advance();
                }
            }
        }
        if arity > 0.0 {
            self.cursor.eat(",");
        }
        let close = self
            .cursor
            .expect(")", Some("primitive TryFrom consumer"))?;
        let range = Some(Span {
            start: head.start.start,
            end: close.end,
        });
        if self.origins.collecting {
            return Ok(Some(SExpr::new(
                SNode::App {
                    func: Box::new(SExpr::new(
                        SNode::Name {
                            path: head.resolved,
                        },
                        range,
                    )),
                    args: inputs.into_iter().chain(defaults).collect(),
                },
                range,
            )));
        }
        let input = self.prepare_primitive_from_argument(inputs.remove(0), range, path)?;
        let source = facts(self, &input, path);
        let target = rust_fixed_type(&owner).ok_or_else(|| {
            type_error(
                "the primitive TryFrom target lost its actual integer type",
                range,
            )
        })?;
        if !rules::accept_rust_primitive_try_from_source(
            &source.kind,
            &source.bits,
            source.signed,
            source
                .owner
                .as_ref()
                .map_or("", |owner| owner.kind.as_str()),
            &source.path(),
            source
                .owner
                .as_ref()
                .map_or(-1.0, |owner| count(owner.arguments.len())),
            source.origin.proven,
            count(source.origin.wrappers.len()),
        ) {
            return Err(unsupported(
                "Rust primitive TryFrom",
                "the actual input must retain a proven direct standard integer source owner",
                range,
            ));
        }
        let result_owner = rules::read_rust_primitive_try_from_result_owner(
            &consumer.value,
            &owner,
            &mapper_owner,
        );
        let result_type = match result_owner.as_str() {
            "bool" => BOOL,
            "f64" => FLOAT,
            _ => rust_fixed_type(&result_owner).ok_or_else(|| {
                type_error(
                    "the primitive TryFrom consumer lost its actual result type",
                    range,
                )
            })?,
        };
        let source_type = source.origin.ty.clone().ok_or_else(|| {
            type_error(
                "the primitive TryFrom input lost its actual source type",
                range,
            )
        })?;
        let first = self.fresh_product_binding();
        let input_local = SExpr::new(
            SNode::Name {
                path: vec![first.clone()],
            },
            input.span.or(range),
        );
        self.origins.mark(&input_local, source.origin.clone());
        let success = SExpr::new(
            SNode::Cast {
                arg: Box::new(input_local.clone()),
                to: target.clone(),
                from: None,
                flavor: Flavor::Exact,
            },
            range,
        );
        self.origins
            .mark(&success, origin(target.clone(), &owner, path));
        let converted = if consumer.value == "map_or" {
            self.project_primitive_from(success, &mapper_owner, range, path)?
        } else {
            success
        };
        let mut captures = vec![(first, source_type.clone(), input)];
        let mut fallback = None;
        if arity > 0.0 {
            let actual = facts(self, &defaults[0], path);
            let literal = matches!(defaults[0].node, SNode::Num { ty: None, .. })
                || matches!(&defaults[0].node,SNode::Unary{op:UnaryOp::Neg,arg}if matches!(arg.node,SNode::Num{ty:None,..}));
            let (bits, signed) = match &result_type {
                Type::Fixed { bits, signed } => (bits.to_string(), *signed),
                _ => (String::new(), false),
            };
            if !argument_rules::accept_rust_primitive_selection_argument(
                result_type.kind(),
                &bits,
                signed,
                &result_owner,
                &actual.kind,
                &actual.bits,
                actual.signed,
                &actual.path(),
                count(actual.origin.wrappers.len()),
                actual.origin.proven,
                actual.admitted(),
                literal,
            ) {
                return Err(unsupported(
                    "Rust primitive TryFrom default",
                    "the actual default must retain the selected result type and primitive source owner",
                    range,
                ));
            }
            let name = self.fresh_product_binding();
            let node = SExpr::new(
                SNode::Name {
                    path: vec![name.clone()],
                },
                defaults[0].span.or(range),
            );
            self.origins
                .mark(&node, origin(result_type.clone(), &result_owner, path));
            fallback = Some(node);
            captures.push((name, result_type.clone(), defaults.remove(0)));
        }
        let Type::Fixed {
            bits: source_bits, ..
        } = source_type
        else {
            return Err(type_error(
                "the primitive TryFrom source requires an actual fixed integer",
                range,
            ));
        };
        let Type::Fixed {
            bits: target_bits,
            signed: target_signed,
        } = target
        else {
            return Err(type_error(
                "the primitive TryFrom target requires an actual fixed integer",
                range,
            ));
        };
        let plan = rules::plan_rust_primitive_try_from_consumer(
            &consumer.value,
            rules::read_rust_primitive_try_from_guards(
                f64::from(source_bits),
                source.signed,
                f64::from(target_bits),
                target_signed,
            ),
        );
        let mut stack = Vec::new();
        for term in plan {
            match term.as_str() {
                "input" => stack.push(input_local.clone()),
                "minimum" | "maximum" => {
                    let magnitude = bounds::read_rust_saturating_integer_bound_magnitude(
                        &term,
                        f64::from(target_bits),
                        target_signed,
                    );
                    let number = SExpr::new(
                        SNode::Num {
                            value: magnitude,
                            ty: Some(source_type.clone()),
                            negative: false,
                            unit: false,
                        },
                        range,
                    );
                    stack.push(
                        if bounds::accept_rust_saturating_integer_bound_negative(
                            &term,
                            target_signed,
                        ) {
                            SExpr::new(
                                SNode::Unary {
                                    op: UnaryOp::Neg,
                                    arg: Box::new(number),
                                },
                                range,
                            )
                        } else {
                            number
                        },
                    );
                }
                "true" => stack.push(SExpr::new(SNode::Bool { value: true }, range)),
                "success" => stack.push(converted.clone()),
                "fallback" => stack.push(fallback.clone().ok_or_else(|| {
                    type_error(
                        "the shared primitive Result plan lost its actual default",
                        range,
                    )
                })?),
                "ge" | "le" | "and" => {
                    let right = pop(&mut stack, range)?;
                    let left = pop(&mut stack, range)?;
                    stack.push(SExpr::new(
                        SNode::Binary {
                            op: match term.as_str() {
                                "ge" => BinaryOp::Ge,
                                "le" => BinaryOp::Le,
                                _ => BinaryOp::And,
                            },
                            left: Box::new(left),
                            right: Box::new(right),
                            rounding: None,
                        },
                        range,
                    ));
                }
                "not" => {
                    let value = pop(&mut stack, range)?;
                    stack.push(SExpr::new(
                        SNode::Unary {
                            op: UnaryOp::Not,
                            arg: Box::new(value),
                        },
                        range,
                    ));
                }
                "if" => {
                    let otherwise = pop(&mut stack, range)?;
                    let yes = pop(&mut stack, range)?;
                    let cond = pop(&mut stack, range)?;
                    stack.push(SExpr::new(
                        SNode::If {
                            cond: Box::new(cond),
                            then: Box::new(yes),
                            otherwise: Box::new(otherwise),
                        },
                        range,
                    ));
                }
                _ => {
                    return Err(type_error(
                        "unknown shared primitive Result source term",
                        range,
                    ));
                }
            }
        }
        if stack.len() != 1 {
            return Err(type_error(
                "the shared primitive Result plan needs one complete result",
                range,
            ));
        }
        let mut body = pop(&mut stack, range)?;
        for (name, ty, value) in captures.into_iter().rev() {
            body = SExpr::new(
                SNode::Let {
                    name,
                    ty: Some(ty.into()),
                    value: Box::new(value),
                    body: Box::new(body),
                },
                range,
            );
        }
        self.origins
            .mark(&body, origin(result_type, &result_owner, path));
        Ok(Some(body))
    }
}
