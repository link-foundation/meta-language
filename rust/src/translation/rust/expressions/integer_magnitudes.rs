//! Actual source facts and generic AST projection into shared magnitude plans.
use super::primitive_try_from::{facts, origin};
use super::{
    BinaryOp, Flavor, Result, RustParser, SExpr, SNode, Span, Token, Type, UnaryOp,
    rust_fixed_type, type_error, unsupported,
};
use crate::translation::{
    rust_integer_magnitude_rules as rules, rust_primitive_selection_rules as arguments,
    rust_saturating_integer_rules as bounds,
};
fn count(value: usize) -> f64 {
    u32::try_from(value).map_or(f64::INFINITY, f64::from)
}
fn pop(stack: &mut Vec<SExpr>, range: Option<Span>) -> Result<SExpr> {
    stack.pop().ok_or_else(|| {
        type_error(
            "the integer magnitude source plan lost an earlier term",
            range,
        )
    })
}
impl RustParser {
    pub(super) fn read_integer_magnitudes(
        &mut self,
        receiver: &SExpr,
        method: &Token,
        args: &[SExpr],
        range: Option<Span>,
        path: &[String],
    ) -> Result<Option<SExpr>> {
        let arity = rules::read_rust_integer_magnitude_arity(
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
        let mut source = facts(self, receiver, path);
        let owner = source.path();
        if crate::translation::rust_primitive_inherent_copy_rules::accept_rust_primitive_inherent_copy(&method.value,&source.kind,&source.bits,source.signed,source.owner.as_ref().map_or("",|owner|owner.kind.as_str()),&owner,source.owner.as_ref().map_or(-1.0,|owner|count(owner.arguments.len())),source.origin.proven,count(source.origin.wrappers.len()),source.origin.wrappers.iter().all(|layer|layer=="reference"),source.origin.copy_evidence.is_some()){
   source.origin.wrappers.clear();source.origin.source_lifetimes.clear();source.origin.copy_evidence=Some(());
  }
        if !rules::accept_rust_integer_magnitude_receiver(
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
                "Rust integer magnitude",
                "a proven standard integer receiver with the actual signed method is required",
                range,
            ));
        }
        if arity > 0.0 {
            let right = facts(self, &args[0], path);
            let literal = matches!(&args[0].node, SNode::Num { ty: None, .. })
                || matches!(&args[0].node,SNode::Unary{op:UnaryOp::Neg,arg}if matches!(&arg.node,SNode::Num{ty:None,..}));
            if !arguments::accept_rust_primitive_selection_argument(
                &source.kind,
                &source.bits,
                source.signed,
                &owner,
                &right.kind,
                &right.bits,
                right.signed,
                &right.path(),
                count(right.origin.wrappers.len()),
                right.origin.proven,
                right.admitted(),
                literal,
            ) {
                return Err(unsupported(
                    "Rust integer magnitude argument",
                    "the actual argument must retain the receiver type and source owner",
                    range,
                ));
            }
        }
        let result_owner = rules::read_rust_integer_magnitude_result_owner(&owner);
        let unsigned = rust_fixed_type(&result_owner)
            .ok_or_else(|| type_error("the integer magnitude lost its result type", range))?;
        let source_type = source.origin.ty.clone().ok_or_else(|| {
            type_error("the integer magnitude receiver lost its source type", range)
        })?;
        let Type::Fixed { bits, .. } = &unsigned else {
            return Err(type_error(
                "the integer magnitude needs its actual unsigned type",
                range,
            ));
        };
        let mut captures = Vec::new();
        let mut locals = Vec::new();
        for value in std::iter::once(receiver).chain(args) {
            let name = self.fresh_product_binding();
            let node = SExpr::new(
                SNode::Name {
                    path: vec![name.clone()],
                },
                value.span.or(range),
            );
            self.origins
                .mark(&node, origin(source_type.clone(), &owner, path));
            locals.push(node);
            captures.push((name, source_type.clone(), value.clone()));
        }
        let cast = |arg: SExpr| {
            SExpr::new(
                SNode::Cast {
                    arg: Box::new(arg),
                    to: unsigned.clone(),
                    from: None,
                    flavor: Flavor::Wrapping,
                },
                range,
            )
        };
        let mut stack = Vec::new();
        for term in rules::plan_rust_integer_magnitude(&method.value) {
            match term.as_str() {
                "operand" | "right" => stack.push(locals[usize::from(term == "right")].clone()),
                "unsigned operand" | "unsigned right" => {
                    stack.push(cast(locals[usize::from(term == "unsigned right")].clone()))
                }
                "unsigned cast" => {
                    let value = pop(&mut stack, range)?;
                    stack.push(cast(value));
                }
                "zero" | "positive unit" | "unsigned unit" | "unsigned maximum" => {
                    let value = if term == "zero" {
                        "0".to_owned()
                    } else if term == "unsigned maximum" {
                        bounds::read_rust_saturating_integer_bound_magnitude(
                            "maximum",
                            f64::from(*bits),
                            false,
                        )
                    } else {
                        "1".to_owned()
                    };
                    let ty = if term.starts_with("unsigned ") {
                        unsigned.clone()
                    } else {
                        source_type.clone()
                    };
                    stack.push(SExpr::new(
                        SNode::Num {
                            value,
                            ty: Some(ty),
                            negative: false,
                            unit: false,
                        },
                        range,
                    ));
                }
                "lt" | "ge" | "add" | "sub" => {
                    let right = pop(&mut stack, range)?;
                    let left = pop(&mut stack, range)?;
                    let op = match term.as_str() {
                        "lt" => BinaryOp::Lt,
                        "ge" => BinaryOp::Ge,
                        "add" => BinaryOp::Add,
                        _ => BinaryOp::Sub,
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
                "neg" => {
                    let arg = pop(&mut stack, range)?;
                    stack.push(SExpr::new(
                        SNode::Unary {
                            op: UnaryOp::Neg,
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
                _ => return Err(type_error("unknown integer magnitude source term", range)),
            }
        }
        if stack.len() != 1 {
            return Err(type_error(
                "the integer magnitude source plan needs one complete result",
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
            .mark(&body, origin(unsigned, &result_owner, path));
        Ok(Some(body))
    }
}
