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

fn property_plan_error(range: Option<Span>) -> super::TranslationError {
    unsupported(
        "Rust signed integer property source plan",
        "the generated terms must retain one complete source result",
        range,
    )
}
fn property_pop(stack: &mut Vec<SExpr>, range: Option<Span>) -> Result<SExpr> {
    stack.pop().ok_or_else(|| property_plan_error(range))
}
fn property_literal(ty: &Type, one: bool, range: Option<Span>) -> SExpr {
    SExpr::new(
        SNode::Num {
            value: if one { "1" } else { "0" }.to_owned(),
            ty: Some(ty.clone()),
            negative: false,
            unit: false,
        },
        range,
    )
}
impl RustParser {
    pub(super) fn read_signed_integer_property(
        &mut self,
        receiver: &SExpr,
        method: &Token,
        args: &[SExpr],
        range: Option<Span>,
        path: &[String],
    ) -> Result<Option<SExpr>> {
        use crate::translation::rust_signed_integer_property_rules as properties;
        if !properties::accept_rust_signed_integer_property_method(
            &method.value,
            self.ordinary_completion_context(),
        ) {
            return Ok(None);
        }
        if !args.is_empty() {
            return Err(type_error(
                format!(
                    "{} expects 0 arguments but got {}",
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
                    args: vec![receiver.clone()],
                },
                range,
            )));
        }
        let value = self.primitive_selection_facts(receiver, path);
        if !properties::accept_rust_signed_integer_property_receiver(
            &value.kind,
            &value.bits,
            value.signed,
            value.owner.as_ref().map_or("", |owner| owner.kind.as_str()),
            &value.path(),
            value
                .owner
                .as_ref()
                .map_or(-1.0, |owner| count(owner.arguments.len())),
            value.origin.proven,
            count(value.origin.wrappers.len()),
        ) {
            return Err(unsupported(
                "Rust signed integer property",
                "a proven direct standard signed integer receiver is required",
                range,
            ));
        }
        let result = self.project_primitive_integer_plan(
            receiver,
            &[],
            &properties::plan_rust_signed_integer_property(&method.value),
            &value,
            range,
            path,
        )?;
        let mut origin = value.origin;
        origin.path = path.to_vec();
        origin.source_owner = value.owner;
        if properties::read_rust_signed_integer_property_result(&method.value) != "receiver" {
            origin = SourceOrigin::with_type(super::BOOL, path, Vec::new());
            origin.source_owner = Some(SourceOwner {
                kind: "primitive".to_owned(),
                path: vec!["bool".to_owned()],
                arguments: Vec::new(),
            });
        }
        self.origins.mark(&result, origin);
        Ok(Some(result))
    }
}

impl RustParser {
    fn project_primitive_integer_plan(
        &mut self,
        receiver: &SExpr,
        args: &[SExpr],
        plan: &[String],
        value: &Facts,
        range: Option<Span>,
        path: &[String],
    ) -> Result<SExpr> {
        use crate::translation::rust_saturating_integer_rules as saturating;
        let Some(ty) = value.origin.ty.clone() else {
            return Err(property_plan_error(range));
        };
        let mut origin = value.origin.clone();
        origin.path = path.to_vec();
        origin.source_owner = value.owner.clone();
        let captures = std::iter::once(receiver)
            .chain(args.iter())
            .map(|input| (self.fresh_product_binding(), input.clone()))
            .collect::<Vec<_>>();
        let mut stack = Vec::new();
        for term in plan {
            match term.as_str() {
                "operand" | "right" => {
                    let index = usize::from(term == "right");
                    let Some((name, input)) = captures.get(index) else {
                        return Err(property_plan_error(range));
                    };
                    let node = SExpr::new(
                        SNode::Name {
                            path: vec![name.clone()],
                        },
                        input.span.or(range),
                    );
                    self.origins.mark(&node, origin.clone());
                    stack.push(node);
                }
                "zero" | "positive unit" => {
                    stack.push(property_literal(&ty, term == "positive unit", range))
                }
                "negative unit" => stack.push(SExpr::new(
                    SNode::Unary {
                        op: UnaryOp::Neg,
                        arg: Box::new(property_literal(&ty, true, range)),
                    },
                    range,
                )),
                "minimum" | "maximum" => {
                    let Type::Fixed { bits, signed } = &ty else {
                        return Err(property_plan_error(range));
                    };
                    let magnitude = saturating::read_rust_saturating_integer_bound_magnitude(
                        term,
                        f64::from(*bits),
                        *signed,
                    );
                    if magnitude.is_empty() {
                        return Err(property_plan_error(range));
                    }
                    let node = SExpr::new(
                        SNode::Num {
                            value: magnitude,
                            ty: Some(ty.clone()),
                            negative: false,
                            unit: false,
                        },
                        range,
                    );
                    stack.push(
                        if saturating::accept_rust_saturating_integer_bound_negative(term, *signed)
                        {
                            SExpr::new(
                                SNode::Unary {
                                    op: UnaryOp::Neg,
                                    arg: Box::new(node),
                                },
                                range,
                            )
                        } else {
                            node
                        },
                    );
                }
                "lt" | "gt" | "eq" | "add" | "sub" | "mul" | "div" => {
                    let op = match term.as_str() {
                        "lt" => BinaryOp::Lt,
                        "gt" => BinaryOp::Gt,
                        "eq" => BinaryOp::Eq,
                        "add" => BinaryOp::Add,
                        "sub" => BinaryOp::Sub,
                        "mul" => BinaryOp::Mul,
                        "div" => BinaryOp::Div,
                        _ => return Err(property_plan_error(range)),
                    };
                    let right = property_pop(&mut stack, range)?;
                    let left = property_pop(&mut stack, range)?;
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
                    let arg = property_pop(&mut stack, range)?;
                    stack.push(SExpr::new(
                        SNode::Unary {
                            op: UnaryOp::Neg,
                            arg: Box::new(arg),
                        },
                        range,
                    ));
                }
                "if" => {
                    let otherwise = property_pop(&mut stack, range)?;
                    let then = property_pop(&mut stack, range)?;
                    let cond = property_pop(&mut stack, range)?;
                    stack.push(SExpr::new(
                        SNode::If {
                            cond: Box::new(cond),
                            then: Box::new(then),
                            otherwise: Box::new(otherwise),
                        },
                        range,
                    ));
                }
                _ => return Err(property_plan_error(range)),
            }
        }
        if stack.len() != 1 {
            return Err(property_plan_error(range));
        }
        let mut body = property_pop(&mut stack, range)?;
        for (name, input) in captures.into_iter().rev() {
            body = SExpr::new(
                SNode::Let {
                    name,
                    ty: Some(ty.clone().into()),
                    value: Box::new(input),
                    body: Box::new(body),
                },
                range,
            );
        }
        Ok(body)
    }
    pub(super) fn read_saturating_integer(
        &mut self,
        receiver: &SExpr,
        method: &Token,
        args: &[SExpr],
        range: Option<Span>,
        path: &[String],
    ) -> Result<Option<SExpr>> {
        use crate::translation::rust_saturating_integer_rules as saturating;
        let arity = saturating::read_rust_saturating_integer_arity(
            &method.value,
            self.ordinary_completion_context(),
        );
        if arity < 0.0 {
            return Ok(None);
        }
        if count(args.len()) != arity {
            return Err(type_error(
                format!(
                    "{} expects {} arguments but got {}",
                    method.value,
                    arity,
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
        let value = self.primitive_selection_facts(receiver, path);
        if !saturating::accept_rust_saturating_integer_receiver(
            &method.value,
            &value.kind,
            &value.bits,
            value.signed,
            value.owner.as_ref().map_or("", |owner| owner.kind.as_str()),
            &value.path(),
            value
                .owner
                .as_ref()
                .map_or(-1.0, |owner| count(owner.arguments.len())),
            value.origin.proven,
            count(value.origin.wrappers.len()),
        ) {
            return Err(unsupported(
                "Rust saturating integer arithmetic",
                "a proven direct standard fixed integer receiver with the actual signed operation is required",
                range,
            ));
        }
        if arity == 1.0 {
            let right = self.primitive_selection_facts(&args[0], path);
            let literal = matches!(&args[0].node, SNode::Num { ty: None, .. })
                || matches!(&args[0].node,SNode::Unary{op:UnaryOp::Neg,arg}if matches!(&arg.node,SNode::Num{ty:None,..}));
            if !rules::accept_rust_primitive_selection_argument(
                &value.kind,
                &value.bits,
                value.signed,
                &value.path(),
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
                    "Rust saturating integer arithmetic",
                    "the argument must retain the actual receiver type and primitive source owner",
                    range,
                ));
            }
        }
        let result = self.project_primitive_integer_plan(
            receiver,
            args,
            &saturating::plan_rust_saturating_integer(&method.value, value.signed),
            &value,
            range,
            path,
        )?;
        let mut origin = value.origin;
        origin.path = path.to_vec();
        origin.source_owner = value.owner;
        self.origins.mark(&result, origin);
        Ok(Some(result))
    }
}

impl RustParser {
    pub(super) fn read_saturating_multiplication(
        &mut self,
        receiver: &SExpr,
        method: &Token,
        args: &[SExpr],
        range: Option<Span>,
        path: &[String],
    ) -> Result<Option<SExpr>> {
        use crate::translation::rust_saturating_multiplication_rules as multiplication;
        if !multiplication::accept_rust_saturating_multiplication_method(
            &method.value,
            self.ordinary_completion_context(),
        ) {
            return Ok(None);
        }
        if args.len() != 1 {
            return Err(type_error(
                format!("saturating_mul expects 1 arguments but got {}", args.len()),
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
        let value = self.primitive_selection_facts(receiver, path);
        let right = self.primitive_selection_facts(&args[0], path);
        if !multiplication::accept_rust_saturating_multiplication_receiver(
            &value.kind,
            &value.bits,
            value.signed,
            value.owner.as_ref().map_or("", |owner| owner.kind.as_str()),
            &value.path(),
            value
                .owner
                .as_ref()
                .map_or(-1.0, |owner| count(owner.arguments.len())),
            value.origin.proven,
            count(value.origin.wrappers.len()),
        ) {
            return Err(unsupported(
                "Rust saturating multiplication",
                "a proven direct standard fixed integer receiver is required",
                range,
            ));
        }
        let literal = matches!(&args[0].node, SNode::Num { ty: None, .. })
            || matches!(&args[0].node,SNode::Unary{op:UnaryOp::Neg,arg}if matches!(&arg.node,SNode::Num{ty:None,..}));
        if !rules::accept_rust_primitive_selection_argument(
            &value.kind,
            &value.bits,
            value.signed,
            &value.path(),
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
                "Rust saturating multiplication",
                "the argument must retain the actual receiver type and primitive source owner",
                range,
            ));
        }
        let result = self.project_primitive_integer_plan(
            receiver,
            args,
            &multiplication::plan_rust_saturating_multiplication(value.signed),
            &value,
            range,
            path,
        )?;
        let mut origin = value.origin;
        origin.path = path.to_vec();
        origin.source_owner = value.owner;
        self.origins.mark(&result, origin);
        Ok(Some(result))
    }
}

impl RustParser {
    pub(super) fn read_wrapping_integer(
        &mut self,
        receiver: &SExpr,
        method: &Token,
        args: &[SExpr],
        range: Option<Span>,
        path: &[String],
    ) -> Result<Option<SExpr>> {
        use crate::translation::rust_wrapping_integer_rules as wrapping;
        let arity = wrapping::read_rust_wrapping_integer_arity(
            &method.value,
            self.ordinary_completion_context(),
        );
        if arity < 0.0 {
            return Ok(None);
        }
        if count(args.len()) != arity {
            return Err(type_error(
                format!(
                    "{} expects {} arguments but got {}",
                    method.value,
                    arity,
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
        let value = self.primitive_selection_facts(receiver, path);
        if !wrapping::accept_rust_wrapping_integer_receiver(
            &method.value,
            &value.kind,
            &value.bits,
            value.signed,
            value.owner.as_ref().map_or("", |owner| owner.kind.as_str()),
            &value.path(),
            value
                .owner
                .as_ref()
                .map_or(-1.0, |owner| count(owner.arguments.len())),
            value.origin.proven,
            count(value.origin.wrappers.len()),
        ) {
            return Err(unsupported(
                "Rust wrapping integer arithmetic",
                "a proven direct standard fixed integer receiver with the actual signed operation is required",
                range,
            ));
        }
        if arity == 1.0 {
            let right = self.primitive_selection_facts(&args[0], path);
            let literal = matches!(&args[0].node, SNode::Num { ty: None, .. })
                || matches!(&args[0].node,SNode::Unary{op:UnaryOp::Neg,arg}if matches!(&arg.node,SNode::Num{ty:None,..}));
            if !rules::accept_rust_primitive_selection_argument(
                &value.kind,
                &value.bits,
                value.signed,
                &value.path(),
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
                    "Rust wrapping integer arithmetic",
                    "the argument must retain the actual receiver type and primitive source owner",
                    range,
                ));
            }
        }
        let result = self.project_primitive_integer_plan(
            receiver,
            args,
            &wrapping::plan_rust_wrapping_integer(&method.value, value.signed),
            &value,
            range,
            path,
        )?;
        let mut origin = value.origin;
        origin.path = path.to_vec();
        origin.source_owner = value.owner;
        self.origins.mark(&result, origin);
        Ok(Some(result))
    }
}
