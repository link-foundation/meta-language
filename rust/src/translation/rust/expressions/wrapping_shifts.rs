//! Actual source facts and generic existing AST projection for shared plans.
use super::primitive_try_from::{facts, origin};
use super::{
    BinaryOp, Flavor, Result, RustParser, SArrayItem, SExpr, SNode, Span, Token, Type, UnaryOp,
    type_error, unsupported,
};
use crate::translation::{
    rust_primitive_selection_rules as arguments, rust_wrapping_shift_rules as rules,
};
fn count(value: usize) -> f64 {
    u32::try_from(value).map_or(f64::INFINITY, f64::from)
}
fn pop(stack: &mut Vec<SExpr>, range: Option<Span>) -> Result<SExpr> {
    stack
        .pop()
        .ok_or_else(|| type_error("the wrapping shift source plan lost an earlier term", range))
}
impl RustParser {
    pub(super) fn read_wrapping_shifts(
        &mut self,
        receiver: &SExpr,
        method: &Token,
        args: &[SExpr],
        range: Option<Span>,
        path: &[String],
    ) -> Result<Option<SExpr>> {
        if !self.ordinary_completion_context()
            || !rules::accept_rust_wrapping_shift_method(&method.value)
        {
            return Ok(None);
        }
        if args.len() != 1 {
            return Err(type_error(
                format!("{} needs one actual u32 count argument", method.value),
                range,
            ));
        }
        let mut source = facts(self, receiver, path);
        let owner = source.path();
        if crate::translation::rust_primitive_inherent_copy_rules::accept_rust_primitive_inherent_copy(&method.value,&source.kind,&source.bits,source.signed,source.owner.as_ref().map_or("",|owner|owner.kind.as_str()),&owner,source.owner.as_ref().map_or(-1.0,|owner|count(owner.arguments.len())),source.origin.proven,count(source.origin.wrappers.len()),source.origin.wrappers.iter().all(|layer|layer=="reference"),source.origin.copy_evidence.is_some()){
   source.origin.wrappers.clear();source.origin.source_lifetimes.clear();source.origin.copy_evidence=Some(());
  }
        if !rules::accept_rust_wrapping_shift_receiver(
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
                "Rust wrapping shift",
                "a proven standard fixed integer receiver with its actual source owner is required",
                range,
            ));
        }
        let actual = facts(self, &args[0], path);
        let literal = matches!(&args[0].node, SNode::Num { ty: None, .. })
            || matches!(&args[0].node,SNode::Unary{op:UnaryOp::Neg,arg}if matches!(&arg.node,SNode::Num{ty:None,..}));
        if !arguments::accept_rust_primitive_selection_argument(
            "fixed",
            "32",
            false,
            "u32",
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
                "Rust wrapping shift count",
                "the actual count must retain standard u32 ownership",
                range,
            ));
        }
        let ty =
            source.origin.ty.clone().ok_or_else(|| {
                type_error("the wrapping shift receiver lost its source type", range)
            })?;
        let Type::Fixed { bits, .. } = &ty else {
            return Err(type_error(
                "the wrapping shift receiver needs its actual fixed type",
                range,
            ));
        };
        let unsigned = Type::Fixed {
            bits: *bits,
            signed: false,
        };
        let count_type = Type::Fixed {
            bits: 32,
            signed: false,
        };
        let mut captures = Vec::new();
        let mut locals = Vec::new();
        for (index, value) in [receiver, &args[0]].into_iter().enumerate() {
            let name = self.fresh_product_binding();
            let actual_type = if index == 0 {
                ty.clone()
            } else {
                count_type.clone()
            };
            let actual_owner = if index == 0 { owner.as_str() } else { "u32" };
            let node = SExpr::new(
                SNode::Name {
                    path: vec![name.clone()],
                },
                value.span.or(range),
            );
            self.origins
                .mark(&node, origin(actual_type.clone(), actual_owner, path));
            locals.push(node);
            captures.push((name, actual_type, value.clone()));
        }
        let mut stack = Vec::new();
        for term in rules::plan_rust_wrapping_shift(&method.value) {
            match term.as_str() {
                "operand" => stack.push(locals[0].clone()),
                "unsigned operand" => stack.push(SExpr::new(
                    SNode::Cast {
                        arg: Box::new(locals[0].clone()),
                        to: unsigned.clone(),
                        from: None,
                        flavor: Flavor::Wrapping,
                    },
                    range,
                )),
                "count" => stack.push(locals[1].clone()),
                "width" => stack.push(SExpr::new(
                    SNode::Num {
                        value: bits.to_string(),
                        ty: Some(count_type.clone()),
                        negative: false,
                        unit: false,
                    },
                    range,
                )),
                "rem" | "sub" => {
                    let right = pop(&mut stack, range)?;
                    let left = pop(&mut stack, range)?;
                    stack.push(SExpr::new(
                        SNode::Binary {
                            op: if term == "rem" {
                                BinaryOp::Rem
                            } else {
                                BinaryOp::Sub
                            },
                            left: Box::new(left),
                            right: Box::new(right),
                            rounding: None,
                        },
                        range,
                    ));
                }
                "source cast" => {
                    let value = pop(&mut stack, range)?;
                    stack.push(SExpr::new(
                        SNode::Cast {
                            arg: Box::new(value),
                            to: ty.clone(),
                            from: None,
                            flavor: Flavor::Wrapping,
                        },
                        range,
                    ));
                }
                _ => {
                    let operation = rules::read_rust_wrapping_shift_operation(&term);
                    if operation.is_empty() {
                        return Err(type_error("unknown wrapping shift source term", range));
                    }
                    let right = pop(&mut stack, range)?;
                    let left = pop(&mut stack, range)?;
                    stack.push(SExpr::new(
                        SNode::Math {
                            op: operation,
                            name: "Rust primitive wrapping shift".to_owned(),
                            args: vec![SArrayItem::new(left, false), SArrayItem::new(right, false)],
                        },
                        range,
                    ));
                }
            }
        }
        if stack.len() != 1 {
            return Err(type_error(
                "the wrapping shift source plan needs one complete result",
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
        self.origins.mark(&body, origin(ty, &owner, path));
        Ok(Some(body))
    }
}
