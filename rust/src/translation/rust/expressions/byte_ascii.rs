//! Original source facts and generic shared predicate-term projection.
use super::{
    BOOL, BinaryOp, Result, RustParser, SExpr, SNode, Span, Token, Type, rust_fixed_type,
    type_error, unsupported,
};
use crate::translation::{
    rust_byte_ascii_rules as rules,
    rust_pointer_integer_owner_rules::accept_rust_pointer_integer_owner_child,
    source_signatures::SourceOwner,
};
fn count(value: usize) -> f64 {
    u32::try_from(value).map_or(f64::INFINITY, f64::from)
}
fn leaf(mut owner: Option<&SourceOwner>) -> Option<&SourceOwner> {
    for _ in 0..128 {
        let actual = owner?;
        if !accept_rust_pointer_integer_owner_child(
            &actual.kind,
            &actual.path.join("::"),
            count(actual.arguments.len()),
        ) {
            return owner;
        }
        owner = actual.arguments.first();
    }
    None
}
fn pop(stack: &mut Vec<SExpr>, span: Option<Span>) -> Result<SExpr> {
    stack
        .pop()
        .ok_or_else(|| type_error("a byte predicate source plan lost an earlier term", span))
}
fn project(
    plan: Vec<String>,
    name: &str,
    operand_span: Option<Span>,
    range: Option<Span>,
    ty: &Type,
) -> Result<SExpr> {
    let mut stack = Vec::new();
    for term in plan {
        let node = match term.as_str() {
            "operand" => {
                stack.push(SExpr::new(
                    SNode::Name {
                        path: vec![name.to_owned()],
                    },
                    operand_span,
                ));
                continue;
            }
            "ge" | "le" | "and" | "or" => {
                let right = pop(&mut stack, range)?;
                let left = pop(&mut stack, range)?;
                SNode::Binary {
                    op: match term.as_str() {
                        "ge" => BinaryOp::Ge,
                        "le" => BinaryOp::Le,
                        "and" => BinaryOp::And,
                        _ => BinaryOp::Or,
                    },
                    left: Box::new(left),
                    right: Box::new(right),
                    rounding: None,
                }
            }
            _ if !term.is_empty() && term.bytes().all(|byte| byte.is_ascii_digit()) => SNode::Num {
                value: term,
                ty: Some(ty.clone()),
                negative: false,
                unit: false,
            },
            _ => return Err(type_error("unknown byte predicate source term", range)),
        };
        stack.push(SExpr::new(node, range));
    }
    if stack.len() != 1 {
        return Err(type_error(
            "a byte predicate source plan needs one complete result",
            range,
        ));
    }
    pop(&mut stack, range)
}
impl RustParser {
    pub(super) fn read_byte_ascii(
        &mut self,
        receiver: &SExpr,
        method: &Token,
        args: &[SExpr],
        range: Option<Span>,
        path: &[String],
    ) -> Result<Option<SExpr>> {
        let arity =
            rules::read_rust_byte_ascii_arity(&method.value, self.ordinary_completion_context());
        if arity < 0.0 {
            return Ok(None);
        }
        let source = self.origins.origin(receiver, path);
        if source.ty == Some(super::super::STRING) {
            return Ok(None);
        }
        if count(args.len()) != arity {
            return Err(type_error(
                format!("{} needs no actual byte predicate arguments", method.value),
                range,
            ));
        }
        let owner = leaf(
            source
                .expanded_source_owner
                .as_ref()
                .or(source.source_owner.as_ref()),
        );
        let (bits, signed) = match &source.ty {
            Some(Type::Fixed { bits, signed }) => (bits.to_string(), *signed),
            _ => (String::new(), false),
        };
        let error = rules::read_rust_byte_ascii_receiver_error(
            source.ty.as_ref().map_or("", Type::kind),
            &bits,
            signed,
            owner.map_or("", |owner| owner.kind.as_str()),
            &owner.map_or_else(String::new, |owner| owner.path.join("::")),
            owner.map_or(-1.0, |owner| count(owner.arguments.len())),
            source.proven,
            source
                .wrappers
                .iter()
                .all(|layer| layer == "reference" || layer == "box"),
        );
        if !error.is_empty() {
            return Err(unsupported("Rust byte ASCII receiver", &error, range));
        }
        let ty = rust_fixed_type("u8").expect("the actual source byte type");
        let name = self.fresh_product_binding();
        let body = project(
            rules::plan_rust_byte_ascii(&method.value),
            &name,
            receiver.span.or(range),
            range,
            &ty,
        )?;
        let node = SExpr::new(
            SNode::Let {
                name,
                ty: Some(ty.into()),
                value: Box::new(receiver.clone()),
                body: Box::new(body),
            },
            range,
        );
        let mut actual = super::SourceOrigin::with_type(BOOL, path, Vec::new());
        actual.source_owner = Some(SourceOwner {
            kind: "primitive".to_owned(),
            path: vec!["bool".to_owned()],
            arguments: Vec::new(),
        });
        self.origins.mark(&node, actual);
        Ok(Some(node))
    }
}
