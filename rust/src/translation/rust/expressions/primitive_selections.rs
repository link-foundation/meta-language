//! Actual primitive source facts and existing conditional AST projection.
use super::{
    BinaryOp, Options, Result, RustParser, SExpr, SNode, SourceOrigin, Span, Token, TokenKind,
    Type, UnaryOp, type_error, unsupported,
};
use crate::translation::rust_primitive_selection_rules as rules;
use crate::translation::source_signatures::SourceOwner;

struct Facts {
    origin: SourceOrigin,
    owner: Option<SourceOwner>,
    kind: String,
    bits: String,
    signed: bool,
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
        rules::accept_rust_primitive_selection_receiver(
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
impl RustParser {
    pub(in super::super) fn primitive_selection_import_role(
        &self,
        segments: &[String],
        path: &[String],
    ) -> String {
        let shadowed = self.origins.lookup(&["std".to_owned()], path).is_some()
            || self.origins.scope.contains_key("std")
            || self
                .aliases
                .values()
                .any(|aliases| aliases.contains_key("std"));
        rules::read_rust_primitive_selection_call_role(
            &segments.join("::"),
            self.ordinary_completion_context(),
            shadowed,
        )
    }
    pub(super) fn read_primitive_selection_call(
        &mut self,
        path: &[String],
    ) -> Result<Option<SExpr>> {
        let start = self.peek();
        let mut offset = usize::from(self.cursor.is("::"));
        let first = self.cursor.peek_at(offset);
        let aliases = self.aliases.get(&path.join("."));
        if first.kind != TokenKind::Identifier
            || self.cursor.is("::") && first.value != "std"
            || first.value != "std"
                && !aliases.is_some_and(|aliases| aliases.contains_key(&first.value))
            || self.origins.scope.contains_key(&first.value)
        {
            return Ok(None);
        }
        let mut segments = Vec::new();
        while self.cursor.peek_at(offset).kind == TokenKind::Identifier && segments.len() < 16 {
            segments.push(self.cursor.peek_at(offset).value.clone());
            offset += 1;
            if !self.cursor.is_at("::", offset) {
                break;
            }
            offset += 1;
        }
        let role = self.primitive_selection_import_role(&self.resolve(&segments, path), path);
        if role.is_empty() {
            return Ok(None);
        }
        if !self.cursor.is_at("(", offset) {
            return Err(unsupported(
                "Rust primitive selection call",
                "a standard comparison function value needs its own callable interpretation",
                Some(Span {
                    start: start.start,
                    end: self.cursor.peek_at(offset).end,
                }),
            ));
        }
        for _ in 0..=offset {
            self.cursor.advance();
        }
        let mut args = Vec::new();
        while !self.cursor.is(")") {
            args.push(self.expr(path, Options::default())?);
            if self.cursor.eat(",").is_none() {
                break;
            }
        }
        let close = self.cursor.expect(")", Some("primitive selection call"))?;
        let range = Some(Span {
            start: start.start,
            end: close.end,
        });
        if args.len() != 2 {
            return Err(type_error(
                "a standard primitive selection call needs two actual operands",
                range,
            ));
        }
        let receiver = args.remove(0);
        let mut method = start;
        method.value = role;
        self.read_primitive_selection(&receiver, &method, &args, range, path)
    }
    fn primitive_selection_facts(&self, node: &SExpr, path: &[String]) -> Facts {
        let origin = self.origins.origin(node, path);
        let owner = origin.source_owner.clone().or_else(|| {
            matches!(node.node, SNode::Bool { .. }).then(|| SourceOwner {
                kind: "primitive".to_owned(),
                path: vec!["bool".to_owned()],
                arguments: Vec::new(),
            })
        });
        let kind = origin.ty.as_ref().map_or("", Type::kind).to_owned();
        let (bits, signed) = match &origin.ty {
            Some(Type::Fixed { bits, signed }) => (bits.to_string(), *signed),
            _ => (String::new(), false),
        };
        Facts {
            origin,
            owner,
            kind,
            bits,
            signed,
        }
    }
    pub(super) fn read_primitive_selection(
        &mut self,
        receiver: &SExpr,
        method: &Token,
        args: &[SExpr],
        range: Option<Span>,
        path: &[String],
    ) -> Result<Option<SExpr>> {
        let comparison = rules::read_rust_primitive_selection_comparison(
            &method.value,
            self.ordinary_completion_context(),
        );
        if comparison.is_empty() {
            return Ok(None);
        }
        if args.len() != 1 {
            return Err(type_error(
                format!(
                    "{} expects 1 arguments but got {}",
                    method.value,
                    args.len()
                ),
                range,
            ));
        }
        if self.origins.collecting {
            return Ok(Some(SExpr::new(
                SNode::App {
                    func: Box::new(SExpr::new(
                        SNode::Name {
                            path: vec![method.value.clone()],
                        },
                        range,
                    )),
                    args: std::iter::once(receiver.clone())
                        .chain(args.iter().cloned())
                        .collect(),
                },
                range,
            )));
        }
        let left = self.primitive_selection_facts(receiver, path);
        let right = self.primitive_selection_facts(&args[0], path);
        if !left.admitted() {
            return Err(unsupported(
                "Rust primitive selection",
                "a proven direct standard fixed integer or Boolean receiver is required",
                range,
            ));
        }
        let literal = matches!(&args[0].node, SNode::Num { ty: None, .. })
            || matches!(&args[0].node,SNode::Unary{op:UnaryOp::Neg,arg}if matches!(&arg.node,SNode::Num{ty:None,..}));
        if !rules::accept_rust_primitive_selection_argument(
            &left.kind,
            &left.bits,
            left.signed,
            &left.path(),
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
                "Rust primitive selection",
                "the argument must retain the actual receiver type and primitive source owner",
                range,
            ));
        }
        let first = self.fresh_product_binding();
        let second = self.fresh_product_binding();
        let ty = left.origin.ty.clone();
        let mut origin = left.origin;
        origin.path = path.to_vec();
        origin.source_owner = left.owner;
        let local = |parser: &mut Self, name: &str| {
            let node = SExpr::new(
                SNode::Name {
                    path: vec![name.to_owned()],
                },
                range,
            );
            parser.origins.mark(&node, origin.clone());
            node
        };
        let one = local(self, &first);
        let two = local(self, &second);
        let condition = rules::read_rust_primitive_selection_condition(&method.value, &left.kind);
        let cond = match condition.as_str() {
            "identity" => one,
            "not" => SExpr::new(
                SNode::Unary {
                    op: UnaryOp::Not,
                    arg: Box::new(one),
                },
                range,
            ),
            "le" | "gt" => SExpr::new(
                SNode::Binary {
                    op: if condition == "le" {
                        BinaryOp::Le
                    } else {
                        BinaryOp::Gt
                    },
                    left: Box::new(one),
                    right: Box::new(two),
                    rounding: None,
                },
                range,
            ),
            _ => {
                return Err(unsupported(
                    "Rust primitive selection",
                    "the generated condition must retain its actual surface operation",
                    range,
                ));
            }
        };
        let body = SExpr::new(
            SNode::If {
                cond: Box::new(cond),
                then: Box::new(local(self, &first)),
                otherwise: Box::new(local(self, &second)),
            },
            range,
        );
        let second_body = SExpr::new(
            SNode::Let {
                name: second,
                ty: ty.clone().map(Into::into),
                value: Box::new(args[0].clone()),
                body: Box::new(body),
            },
            range,
        );
        let result = SExpr::new(
            SNode::Let {
                name: first,
                ty: ty.map(Into::into),
                value: Box::new(receiver.clone()),
                body: Box::new(second_body),
            },
            range,
        );
        self.origins.mark(&result, origin);
        Ok(Some(result))
    }
}
