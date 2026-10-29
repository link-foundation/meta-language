//! Token/type/origin projection into shared checked arithmetic and Option plans.
use super::primitive_try_from::{facts, origin};
use super::{
    BOOL, BinaryOp, FLOAT, Options, Result, RustParser, SExpr, SNode, Span, Token, TokenKind, Type,
    UnaryOp, rust_fixed_type, type_error, unsupported,
};
use crate::translation::{
    rust_checked_integer_rules as rules, rust_primitive_from_rules as from_rules,
    rust_primitive_selection_rules as arguments, rust_saturating_integer_rules as bounds,
};
fn count(value: usize) -> f64 {
    u32::try_from(value).map_or(f64::INFINITY, f64::from)
}
fn literal(node: &SExpr) -> bool {
    matches!(&node.node, SNode::Num { ty: None, .. })
        || matches!(&node.node,SNode::Unary{op:UnaryOp::Neg,arg}if matches!(&arg.node,SNode::Num{ty:None,..}))
}
fn pop(stack: &mut Vec<SExpr>, range: Option<Span>) -> Result<SExpr> {
    stack
        .pop()
        .ok_or_else(|| type_error("the checked source plan lost an earlier term", range))
}
struct Terms {
    operand: SExpr,
    right: Option<SExpr>,
    condition: Option<SExpr>,
    success: Option<SExpr>,
    fallback: Option<SExpr>,
    ty: Type,
    signed: bool,
    bits: u32,
}
impl Terms {
    fn project(&self, plan: &[String], range: Option<Span>) -> Result<SExpr> {
        let mut stack = Vec::new();
        let number = |value: String| {
            SExpr::new(
                SNode::Num {
                    value,
                    ty: Some(self.ty.clone()),
                    negative: false,
                    unit: false,
                },
                range,
            )
        };
        for term in plan {
            match term.as_str() {
                "operand" => stack.push(self.operand.clone()),
                "right" | "condition" | "success" | "fallback" => stack.push(
                    match term.as_str() {
                        "right" => &self.right,
                        "condition" => &self.condition,
                        "success" => &self.success,
                        _ => &self.fallback,
                    }
                    .clone()
                    .ok_or_else(|| {
                        type_error(
                            "the checked source plan lost its actual consumer value",
                            range,
                        )
                    })?,
                ),
                "zero" | "default" => stack.push(number("0".to_owned())),
                "width" => stack.push(SExpr::new(
                    SNode::Num {
                        value: self.bits.to_string(),
                        ty: Some(Type::Fixed {
                            bits: 32,
                            signed: false,
                        }),
                        negative: false,
                        unit: false,
                    },
                    range,
                )),
                "shift left" | "shift right" => {
                    let right = pop(&mut stack, range)?;
                    let left = pop(&mut stack, range)?;
                    stack.push(SExpr::new(
                        SNode::Math {
                            op: rules::read_rust_checked_integer_shift_operation(term),
                            name: "Rust primitive shift".to_owned(),
                            args: vec![
                                super::SArrayItem::new(left, false),
                                super::SArrayItem::new(right, false),
                            ],
                        },
                        range,
                    ));
                }
                "negative unit" => stack.push(SExpr::new(
                    SNode::Unary {
                        op: UnaryOp::Neg,
                        arg: Box::new(number("1".to_owned())),
                    },
                    range,
                )),
                "minimum" | "maximum" => {
                    let value = number(bounds::read_rust_saturating_integer_bound_magnitude(
                        term,
                        f64::from(self.bits),
                        self.signed,
                    ));
                    stack.push(
                        if bounds::accept_rust_saturating_integer_bound_negative(term, self.signed)
                        {
                            SExpr::new(
                                SNode::Unary {
                                    op: UnaryOp::Neg,
                                    arg: Box::new(value),
                                },
                                range,
                            )
                        } else {
                            value
                        },
                    );
                }
                "true" | "false" => stack.push(SExpr::new(
                    SNode::Bool {
                        value: term == "true",
                    },
                    range,
                )),
                "lt" | "gt" | "eq" | "ne" | "le" | "ge" | "add" | "sub" | "mul" | "div" | "rem" => {
                    let right = pop(&mut stack, range)?;
                    let left = pop(&mut stack, range)?;
                    let op = match term.as_str() {
                        "lt" => BinaryOp::Lt,
                        "gt" => BinaryOp::Gt,
                        "eq" => BinaryOp::Eq,
                        "ne" => BinaryOp::Ne,
                        "le" => BinaryOp::Le,
                        "ge" => BinaryOp::Ge,
                        "add" => BinaryOp::Add,
                        "sub" => BinaryOp::Sub,
                        "mul" => BinaryOp::Mul,
                        "div" => BinaryOp::Div,
                        _ => BinaryOp::Rem,
                    };
                    stack.push(SExpr::new(
                        SNode::Binary {
                            op,
                            left: Box::new(left),
                            right: Box::new(right),
                            rounding: None,
                        },
                        range,
                    ));
                }
                "neg" | "not" => {
                    let arg = pop(&mut stack, range)?;
                    stack.push(SExpr::new(
                        SNode::Unary {
                            op: if term == "neg" {
                                UnaryOp::Neg
                            } else {
                                UnaryOp::Not
                            },
                            arg: Box::new(arg),
                        },
                        range,
                    ));
                }
                "if" => {
                    let otherwise = pop(&mut stack, range)?;
                    let then = pop(&mut stack, range)?;
                    let cond = pop(&mut stack, range)?;
                    stack.push(SExpr::new(
                        SNode::If {
                            cond: Box::new(cond),
                            then: Box::new(then),
                            otherwise: Box::new(otherwise),
                        },
                        range,
                    ));
                }
                _ => return Err(type_error("unknown checked source plan term", range)),
            }
        }
        if stack.len() != 1 {
            return Err(type_error(
                "the checked source plan needs one complete result",
                range,
            ));
        }
        pop(&mut stack, range)
    }
}
impl RustParser {
    pub(super) fn read_checked_integer(
        &mut self,
        receiver: &SExpr,
        method: &Token,
        args: &[SExpr],
        range: Option<Span>,
        path: &[String],
    ) -> Result<Option<SExpr>> {
        let arity = rules::read_rust_checked_integer_arity(
            &method.value,
            self.ordinary_completion_context(),
        );
        if arity < 0.0 {
            return Ok(None);
        }
        if count(args.len()) != arity {
            return Err(type_error(
                format!("{} needs {} actual source arguments", method.value, arity),
                range,
            ));
        }
        if self.cursor.eat(".").is_none() || self.cursor.peek().kind != TokenKind::Identifier {
            return Err(unsupported(
                "Rust checked integer Option",
                "the actual Option needs an immediate admitted standard consumer",
                range,
            ));
        }
        let consumer = self.cursor.advance();
        let consumer_arity = rules::read_rust_checked_integer_consumer_arity(&consumer.value);
        if consumer_arity < 0.0 {
            return Err(unsupported(
                "Rust checked integer Option",
                "the actual Option needs an immediate admitted standard consumer",
                range,
            ));
        }
        self.cursor.expect("(", Some("checked integer consumer"))?;
        let mut defaults = Vec::new();
        let mut mapper_owner = String::new();
        if consumer_arity > 0.0 {
            defaults.push(self.expr(path, Options::default())?);
            if consumer_arity == 2.0 {
                self.cursor.expect(",", Some("checked integer mapper"))?;
                let mapper = self
                    .primitive_try_head(path)
                    .filter(|head| head.resolved.last().is_some_and(|method| method == "from"))
                    .ok_or_else(|| {
                        unsupported(
                            "Rust checked integer mapper",
                            "an actual admitted standard primitive From function item is required",
                            range,
                        )
                    })?;
                mapper_owner = from_rules::read_rust_primitive_from_owner(
                    &mapper.resolved,
                    self.ordinary_completion_context(),
                    mapper.shadowed,
                    mapper.absolute,
                );
                if mapper_owner.is_empty() {
                    return Err(unsupported(
                        "Rust checked integer mapper",
                        "an actual admitted standard primitive From function item is required",
                        range,
                    ));
                }
                for _ in 0..mapper.offset {
                    self.cursor.advance();
                }
            }
        }
        if consumer_arity > 0.0 {
            self.cursor.eat(",");
        }
        let close = self.cursor.expect(")", Some("checked integer consumer"))?;
        let range = Some(Span {
            start: receiver
                .span
                .or(range)
                .map_or(consumer.start, |span| span.start),
            end: close.end,
        });
        let mut source = facts(self, receiver, path);
        let references_only = source
            .origin
            .wrappers
            .iter()
            .all(|layer| layer == "reference");
        if crate::translation::rust_primitive_inherent_copy_rules::accept_rust_primitive_inherent_copy(&method.value,&source.kind,&source.bits,source.signed,source.owner.as_ref().map_or("",|owner|owner.kind.as_str()),&source.path(),source.owner.as_ref().map_or(-1.0,|owner|count(owner.arguments.len())),source.origin.proven,count(source.origin.wrappers.len()),references_only,source.origin.copy_evidence.is_some()){
   source.origin.wrappers.clear();source.origin.source_lifetimes.clear();source.origin.copy_evidence=Some(());
  }
        let owner = source.path();
        if !rules::accept_rust_checked_integer_receiver(
            &method.value,
            &source.kind,
            &source.bits,
            source.signed,
            source
                .owner
                .as_ref()
                .map_or("", |owner| owner.kind.as_str()),
            &owner,
            source
                .owner
                .as_ref()
                .map_or(-1.0, |owner| count(owner.arguments.len())),
            source.origin.proven,
            count(source.origin.wrappers.len()),
        ) {
            return Err(unsupported(
                "Rust checked integer arithmetic",
                "a proven standard integer receiver with the actual signed operation is required",
                range,
            ));
        }
        let argument_owner = rules::read_rust_checked_integer_argument_owner(&method.value, &owner);
        let argument_type = rust_fixed_type(&argument_owner)
            .ok_or_else(|| type_error("the checked argument lost its source type", range))?;
        let Type::Fixed {
            bits: argument_bits,
            signed: argument_signed,
        } = &argument_type
        else {
            return Err(type_error(
                "the checked argument needs its actual fixed type",
                range,
            ));
        };
        if arity > 0.0 {
            let right = facts(self, &args[0], path);
            if !arguments::accept_rust_primitive_selection_argument(
                argument_type.kind(),
                &argument_bits.to_string(),
                *argument_signed,
                &argument_owner,
                &right.kind,
                &right.bits,
                right.signed,
                &right.path(),
                count(right.origin.wrappers.len()),
                right.origin.proven,
                right.admitted(),
                literal(&args[0]),
            ) {
                return Err(unsupported(
                    "Rust checked integer argument",
                    &rules::read_rust_checked_integer_argument_reason(&method.value),
                    range,
                ));
            }
        }
        if consumer.value == "map_or"
            && from_rules::plan_rust_primitive_from(&mapper_owner, &owner).is_empty()
        {
            return Err(unsupported(
                "Rust checked integer mapper",
                "the actual standard primitive From implementation must exist for the successful value",
                range,
            ));
        }
        let result_owner =
            rules::read_rust_checked_integer_result_owner(&consumer.value, &owner, &mapper_owner);
        let result_type = match result_owner.as_str() {
            "bool" => BOOL,
            "f64" => FLOAT,
            _ => rust_fixed_type(&result_owner)
                .ok_or_else(|| type_error("the checked consumer lost its result type", range))?,
        };
        let ty = source
            .origin
            .ty
            .clone()
            .ok_or_else(|| type_error("the checked receiver lost its source type", range))?;
        let Type::Fixed { bits, signed } = &ty else {
            return Err(type_error(
                "the checked receiver needs its actual fixed type",
                range,
            ));
        };
        let mut captures = Vec::new();
        let mut locals = Vec::new();
        for (index, value) in std::iter::once(receiver).chain(args).enumerate() {
            let (actual_type, actual_owner) = if index == 0 {
                (&ty, owner.as_str())
            } else {
                (&argument_type, argument_owner.as_str())
            };
            let name = self.fresh_product_binding();
            let local = SExpr::new(
                SNode::Name {
                    path: vec![name.clone()],
                },
                value.span.or(range),
            );
            self.origins
                .mark(&local, origin(actual_type.clone(), actual_owner, path));
            locals.push(local);
            captures.push((name, actual_type.clone(), value.clone()));
        }
        let mut terms = Terms {
            operand: locals[0].clone(),
            right: locals.get(1).cloned(),
            condition: None,
            success: None,
            fallback: None,
            ty: ty.clone(),
            bits: *bits,
            signed: *signed,
        };
        if consumer_arity > 0.0 {
            let actual = facts(self, &defaults[0], path);
            let (result_bits, result_signed) = match &result_type {
                Type::Fixed { bits, signed } => (bits.to_string(), *signed),
                _ => (String::new(), false),
            };
            if !arguments::accept_rust_primitive_selection_argument(
                result_type.kind(),
                &result_bits,
                result_signed,
                &result_owner,
                &actual.kind,
                &actual.bits,
                actual.signed,
                &actual.path(),
                count(actual.origin.wrappers.len()),
                actual.origin.proven,
                actual.admitted(),
                literal(&defaults[0]),
            ) {
                return Err(unsupported(
                    "Rust checked integer default",
                    "the actual default must retain the selected result type and source owner",
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
            terms.fallback = Some(node);
            captures.push((name, result_type.clone(), defaults.remove(0)));
        }
        terms.condition = Some(terms.project(
            &rules::plan_rust_checked_integer_condition(&method.value, source.signed),
            range,
        )?);
        let success = terms.project(
            &rules::plan_rust_checked_integer_success(&method.value, source.signed),
            range,
        )?;
        self.origins.mark(&success, origin(ty, &owner, path));
        terms.success = Some(if consumer.value == "map_or" {
            self.project_primitive_from(success, &mapper_owner, range, path)?
        } else {
            success
        });
        let mut body = terms.project(
            &rules::plan_rust_checked_integer_consumer(&consumer.value),
            range,
        )?;
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
