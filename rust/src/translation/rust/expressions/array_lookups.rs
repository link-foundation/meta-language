//! Original tokens/types and generic shared source-plan term transport only.
use super::{
    BOOL, BinaryOp, Flavor, Options, Result, RustParser, SExpr, SNode, SourceOrigin, Span, Token,
    Type, UnaryOp, rust_fixed_type, type_error, unsupported,
};
use crate::translation::rust_array_lookup_rules as rules;
use crate::translation::source_signatures::SourceOwner;
use std::collections::HashMap;
mod source_owners;
fn count(value: usize) -> f64 {
    u32::try_from(value).map_or(f64::INFINITY, f64::from)
}
fn pop(stack: &mut Vec<SExpr>, span: Option<Span>) -> Result<SExpr> {
    stack
        .pop()
        .ok_or_else(|| type_error("an array lookup source plan lost an earlier term", span))
}
fn project(plan: &[String], values: &HashMap<String, SExpr>, span: Option<Span>) -> Result<SExpr> {
    let mut stack = Vec::new();
    for term in plan {
        if let Some(value) = values.get(term) {
            stack.push(value.clone());
            continue;
        }
        let node = match term.as_str() {
            "lt" | "gt" | "sub" => {
                let right = pop(&mut stack, span)?;
                let left = pop(&mut stack, span)?;
                SNode::Binary {
                    op: match term.as_str() {
                        "lt" => BinaryOp::Lt,
                        "gt" => BinaryOp::Gt,
                        _ => BinaryOp::Sub,
                    },
                    left: Box::new(left),
                    right: Box::new(right),
                    rounding: None,
                }
            }
            "not" => SNode::Unary {
                op: UnaryOp::Not,
                arg: Box::new(pop(&mut stack, span)?),
            },
            "if" => {
                let otherwise = pop(&mut stack, span)?;
                let then = pop(&mut stack, span)?;
                let cond = pop(&mut stack, span)?;
                SNode::If {
                    cond: Box::new(cond),
                    then: Box::new(then),
                    otherwise: Box::new(otherwise),
                }
            }
            _ => return Err(type_error("unknown array lookup source term", span)),
        };
        stack.push(SExpr::new(node, span));
    }
    if stack.len() != 1 {
        return Err(type_error(
            "an array lookup source plan needs one complete result",
            span,
        ));
    }
    pop(&mut stack, span)
}
struct LookupInput<'a> {
    receiver: &'a SExpr,
    method: &'a Token,
    args: &'a [SExpr],
    range: Option<Span>,
    path: &'a [String],
    consumer: Token,
    copied: bool,
    cloned: bool,
    defaults: Vec<SExpr>,
}
struct LookupFacts {
    source: SourceOrigin,
    element: Option<Type>,
    presence: bool,
    owner: String,
    result_type: Type,
    result_origin: SourceOrigin,
    element_owner: Option<SourceOwner>,
}
fn selected_lookup(object: SExpr, index: SExpr, range: Option<Span>) -> SExpr {
    SExpr::new(
        SNode::Index {
            object: Box::new(object),
            index: Box::new(SExpr::new(
                SNode::Cast {
                    arg: Box::new(index),
                    to: Type::Nat,
                    from: None,
                    flavor: Flavor::Exact,
                },
                range,
            )),
            optional: None,
            proven_in_bounds: None,
        },
        range,
    )
}
fn lookup_result_origin(
    result_type: &Type,
    owner: &str,
    presence: bool,
    path: &[String],
) -> SourceOrigin {
    let mut result_origin = SourceOrigin::with_type(result_type.clone(), path, Vec::new());
    result_origin.source_owner = Some(SourceOwner {
        kind: if *result_type == Type::Unit {
            "unit"
        } else {
            "primitive"
        }
        .to_owned(),
        path: if *result_type == Type::Unit {
            Vec::new()
        } else {
            vec![if presence {
                "bool".to_owned()
            } else {
                owner.to_owned()
            }]
        },
        arguments: Vec::new(),
    });
    result_origin
}
fn lookup_dimensions(
    object: SExpr,
    usize_type: &Type,
    range: Option<Span>,
) -> HashMap<String, SExpr> {
    let length = SExpr::new(
        SNode::Cast {
            arg: Box::new(SExpr::new(
                SNode::Length {
                    object: Box::new(object),
                    integer: true,
                },
                range,
            )),
            to: usize_type.clone(),
            from: None,
            flavor: Flavor::Exact,
        },
        range,
    );
    let number = |value: &str| {
        SExpr::new(
            SNode::Num {
                value: value.to_owned(),
                ty: Some(usize_type.clone()),
                negative: false,
                unit: false,
            },
            range,
        )
    };
    HashMap::from([
        ("length".to_owned(), length),
        ("zero".to_owned(), number("0")),
        ("one".to_owned(), number("1")),
    ])
}
impl RustParser {
    fn array_lookup_callable_context(
        &self,
        receiver: &SExpr,
        consumer: &str,
        path: &[String],
    ) -> Option<SourceOrigin> {
        let source = self.origins.origin(receiver, path);
        let Some(Type::Array { element, .. }) = source.ty.as_ref() else {
            return None;
        };
        let owner = source_owners::element_owner(
            source
                .expanded_source_owner
                .as_ref()
                .or(source.source_owner.as_ref()),
        )?;
        let (bits, signed) = match element.as_ref() {
            Type::Fixed { bits, signed } => (bits.to_string(), *signed),
            _ => (String::new(), false),
        };
        let known_element = owner.arguments.is_empty()
            && (owner.kind == "primitive" || owner.kind == "unit" && owner.path.is_empty())
            && rules::read_rust_array_lookup_element_owner_error(
                element.kind(),
                &bits,
                signed,
                &owner.path.join("::"),
            )
            .is_empty();
        let known_receiver = rules::read_rust_array_lookup_receiver_error(
            "array",
            element.kind(),
            source.array_source_model.unwrap_or(""),
            source.proven,
            source
                .wrappers
                .iter()
                .all(|layer| layer == "reference" || layer == "box"),
            source.array_element_copy_evidence,
            true,
        )
        .is_empty();
        let arity = rules::read_rust_array_lookup_callable_context_arity(
            consumer,
            known_receiver,
            known_element,
        );
        if arity < 0.0 {
            return None;
        }
        let mut selected = SourceOrigin::with_type(element.as_ref().clone(), path, Vec::new());
        selected.source_owner = Some(owner.clone());
        let result = if arity == 0.0 {
            selected.clone()
        } else {
            lookup_result_origin(&BOOL, "", true, path)
        };
        let parameters = if arity == 0.0 {
            Vec::new()
        } else {
            vec![selected]
        };
        let mut context = SourceOrigin::with_type(
            Type::Function {
                params: if arity == 0.0 {
                    Vec::new()
                } else {
                    vec![element.as_ref().clone()]
                },
                ret: Box::new(if arity == 0.0 {
                    element.as_ref().clone()
                } else {
                    BOOL
                }),
            },
            path,
            Vec::new(),
        );
        context.copy_evidence = Some(());
        context.callable_parameters = Some(parameters);
        context.callable_result = Some(Box::new(result));
        Some(context)
    }
    pub(super) fn read_array_lookup(
        &mut self,
        receiver: &SExpr,
        method: &Token,
        args: &[SExpr],
        range: Option<Span>,
        path: &[String],
    ) -> Result<Option<SExpr>> {
        let arity =
            rules::read_rust_array_lookup_arity(&method.value, self.ordinary_completion_context());
        if arity < 0.0 {
            return Ok(None);
        }
        if count(args.len()) != arity {
            return Err(type_error(
                format!(
                    "{} needs {arity} actual array lookup arguments",
                    method.value
                ),
                range,
            ));
        }
        if self.cursor.eat(".").is_none() || self.peek().kind != super::TokenKind::Identifier {
            return Err(unsupported(
                "Rust array lookup Option",
                "an immediate standard presence or copied-value consumer is required",
                range,
            ));
        }
        let mut consumer = self.cursor.advance();
        let cloned = consumer.value == "cloned";
        let copied = consumer.value == "copied" || cloned;
        if copied {
            self.cursor.expect("(", Some("copied lookup"))?;
            self.cursor.expect(")", Some("copied lookup"))?;
            if self.cursor.eat(".").is_none() || self.peek().kind != super::TokenKind::Identifier {
                return Err(unsupported(
                    "Rust array lookup Option",
                    "an immediate standard presence or copied-value consumer is required",
                    range,
                ));
            }
            consumer = self.cursor.identifier(Some("copied lookup consumer"))?;
        }
        let consumer_arity = rules::read_rust_array_lookup_consumer_arity(&consumer.value, copied);
        if consumer_arity < 0.0 {
            return Err(unsupported(
                "Rust array lookup Option",
                "an immediate standard presence or copied-value consumer is required",
                range,
            ));
        }
        self.cursor.expect("(", Some("array lookup consumer"))?;
        let mut defaults = Vec::new();
        if consumer_arity > 0.0 {
            let context = self.array_lookup_callable_context(receiver, &consumer.value, path);
            defaults.push(if let Some(context) = context {
                self.read_expected_copy_closure(path, Some(&context))?
            } else {
                self.expr(path, Options::default())?
            });
            self.cursor.eat(",");
        }
        let close = self.cursor.expect(")", Some("array lookup consumer"))?;
        let range = Some(Span::new(
            receiver
                .span
                .or(range)
                .map_or(method.start, |span| span.start),
            close.end,
        ));
        let input = LookupInput {
            receiver,
            method,
            args,
            range,
            path,
            consumer,
            copied,
            cloned,
            defaults,
        };
        let facts = self.array_lookup_facts(&input)?;
        self.project_array_lookup(&input, facts).map(Some)
    }
    fn array_lookup_facts(&self, input: &LookupInput<'_>) -> Result<LookupFacts> {
        let LookupInput {
            receiver,
            method: _,
            args,
            range,
            path,
            consumer,
            copied,
            cloned,
            defaults,
        } = input;
        let range = *range;
        let copied = *copied;
        let cloned = *cloned;
        let arity = count(args.len());
        let source = self.origins.origin(receiver, path);
        let element = match &source.ty {
            Some(Type::Array { element, .. }) => Some(element.as_ref().clone()),
            _ => None,
        };
        let element_owner = source_owners::element_owner(
            source
                .expanded_source_owner
                .as_ref()
                .or(source.source_owner.as_ref()),
        )
        .cloned();
        let (clone_error, string_clone) =
            source_owners::clone_facts(element.as_ref(), element_owner.as_ref(), cloned);
        let error = rules::read_rust_array_lookup_receiver_error(
            source.ty.as_ref().map_or("", Type::kind),
            element.as_ref().map_or("", Type::kind),
            source.array_source_model.unwrap_or(""),
            source.proven,
            source
                .wrappers
                .iter()
                .all(|layer| layer == "reference" || layer == "box"),
            source.array_element_copy_evidence,
            copied && !string_clone,
        );
        if !error.is_empty() {
            return Err(unsupported("Rust array lookup receiver", &error, range));
        }
        if !clone_error.is_empty() {
            return Err(unsupported("Rust array lookup clone", &clone_error, range));
        }
        if arity > 0.0 && !self.usize_index_expression(&args[0], path)?.is_empty() {
            return Err(unsupported(
                "Rust array lookup index",
                "the actual index needs its complete direct usize source meaning",
                args[0].span.or(range),
            ));
        }
        let presence = rules::accept_rust_array_lookup_presence_consumer(&consumer.value);
        if !presence && element.is_none() {
            return Err(unsupported(
                "Rust array lookup result",
                "a copied value needs its complete actual scalar element type",
                range,
            ));
        }
        let owner = source_owners::scalar_owner(&source);
        if !presence && !cloned {
            let actual = element.as_ref().expect("complete copied element");
            let (bits, signed) = match actual {
                Type::Fixed { bits, signed } => (bits.to_string(), *signed),
                _ => (String::new(), false),
            };
            let error = rules::read_rust_array_lookup_element_owner_error(
                actual.kind(),
                &bits,
                signed,
                &owner,
            );
            if !error.is_empty() {
                return Err(unsupported("Rust array lookup result", &error, range));
            }
        }
        let result_type = if presence {
            BOOL
        } else {
            element.clone().expect("complete copied element")
        };
        let mut result_origin = lookup_result_origin(&result_type, &owner, presence, path);
        if !presence && string_clone {
            result_origin.source_owner = element_owner.clone();
        }
        if let Some(default) = defaults.first() {
            let lazy = rules::test_rust_array_lookup_lazy_fallback(&consumer.value);
            let fallback_origin = self.origins.origin(default, path);
            if rules::test_rust_array_lookup_predicate_consumer(&consumer.value) {
                let parameter = fallback_origin
                    .callable_parameters
                    .as_ref()
                    .and_then(|parameters| parameters.first());
                let result = fallback_origin.callable_result.as_deref();
                let parameters = match &fallback_origin.ty {
                    Some(Type::Function { params, .. }) => count(params.len()),
                    _ => -1.0,
                };
                let error = rules::read_rust_array_lookup_predicate_error(
                    fallback_origin.ty.as_ref().map_or("", Type::kind),
                    parameters,
                    count(fallback_origin.wrappers.len()),
                    parameter.is_some_and(|value| value.proven),
                    result
                        .and_then(|value| value.ty.as_ref())
                        .map_or("", Type::kind),
                    result.is_some_and(|value| value.proven),
                );
                if !error.is_empty() {
                    return Err(unsupported("Rust array lookup predicate", &error, range));
                }
                self.origins.argument(
                    &result_origin,
                    parameter.expect("shared predicate parameter admission"),
                    default.span.or(range),
                )?;
                self.origins.argument(
                    result.expect("shared predicate result admission"),
                    &lookup_result_origin(&BOOL, "", true, path),
                    default.span.or(range),
                )?;
            } else {
                let fallback_result = if lazy {
                    let parameters = match &fallback_origin.ty {
                        Some(Type::Function { params, .. }) => count(params.len()),
                        _ => -1.0,
                    };
                    let error = rules::read_rust_array_lookup_lazy_fallback_error(
                        fallback_origin.ty.as_ref().map_or("", Type::kind),
                        parameters,
                        count(fallback_origin.wrappers.len()),
                        fallback_origin
                            .callable_result
                            .as_ref()
                            .is_some_and(|result| result.proven),
                    );
                    if !error.is_empty() {
                        return Err(unsupported(
                            "Rust array lookup lazy fallback",
                            &error,
                            range,
                        ));
                    }
                    fallback_origin
                        .callable_result
                        .as_deref()
                        .expect("shared lazy result admission")
                } else {
                    &fallback_origin
                };
                self.origins
                    .argument(fallback_result, &result_origin, default.span.or(range))?;
            }
        }
        Ok(LookupFacts {
            source,
            element,
            presence,
            owner,
            result_type,
            result_origin,
            element_owner,
        })
    }
    fn project_array_lookup(
        &mut self,
        input: &LookupInput<'_>,
        facts: LookupFacts,
    ) -> Result<SExpr> {
        let LookupInput {
            receiver,
            method,
            args,
            range,
            path,
            consumer,
            defaults,
            ..
        } = input;
        let range = *range;
        let arity = count(args.len());
        let predicate = rules::test_rust_array_lookup_predicate_consumer(&consumer.value);
        let LookupFacts {
            source,
            element,
            presence,
            owner,
            result_type,
            result_origin,
            element_owner,
        } = facts;
        let usize_type = rust_fixed_type("usize").expect("the existing exact usize source profile");
        let mut captures = vec![(self.fresh_product_binding(), source.ty, (*receiver).clone())];
        if arity > 0.0 {
            captures.push((
                self.fresh_product_binding(),
                Some(usize_type.clone()),
                args[0].clone(),
            ));
        }
        if let Some(default) = defaults.first() {
            let default_type =
                if predicate || rules::test_rust_array_lookup_lazy_fallback(&consumer.value) {
                    self.origins.origin(default, path).ty
                } else {
                    element.clone()
                };
            captures.push((self.fresh_product_binding(), default_type, default.clone()));
        }
        let local = |index: usize| {
            SExpr::new(
                SNode::Name {
                    path: vec![captures[index].0.clone()],
                },
                captures[index].2.span.or(range),
            )
        };
        let mut values = lookup_dimensions(local(0), &usize_type, range);
        if arity > 0.0 {
            values.insert("index".to_owned(), local(1));
        }
        let condition = project(
            &rules::plan_rust_array_lookup_condition(&method.value),
            &values,
            range,
        )?;
        values.insert("condition".to_owned(), condition);
        if !presence {
            let index = project(
                &rules::plan_rust_array_lookup_index(&method.value),
                &values,
                range,
            )?;
            values.insert(
                "selected".to_owned(),
                selected_lookup(local(0), index, range),
            );
            let default_term = rules::read_rust_array_lookup_default_term(
                result_type.kind(),
                result_type == super::super::STRING && element_owner.is_some(),
            );
            let fallback = if !defaults.is_empty() {
                let value = local(captures.len() - 1);
                if rules::test_rust_array_lookup_lazy_fallback(&consumer.value) {
                    SExpr::new(
                        SNode::App {
                            func: Box::new(value),
                            args: Vec::new(),
                        },
                        range,
                    )
                } else {
                    value
                }
            } else if default_term == "unit" {
                SExpr::new(SNode::Unit, range)
            } else if default_term == "empty string" {
                SExpr::new(
                    SNode::Str {
                        value: String::new(),
                    },
                    range,
                )
            } else {
                self.project_primitive_default(&owner, path, range)?
            };
            values.insert("fallback".to_owned(), fallback);
        }
        if predicate {
            let selected = values
                .get("selected")
                .expect("shared predicate selected value")
                .clone();
            values.insert(
                "predicate".to_owned(),
                SExpr::new(
                    SNode::App {
                        func: Box::new(local(captures.len() - 1)),
                        args: vec![selected],
                    },
                    range,
                ),
            );
            values.insert(
                "true".to_owned(),
                SExpr::new(SNode::Bool { value: true }, range),
            );
            values.insert(
                "false".to_owned(),
                SExpr::new(SNode::Bool { value: false }, range),
            );
        }
        let mut body = project(
            &rules::plan_rust_array_lookup_consumer(&consumer.value),
            &values,
            range,
        )?;
        for (name, ty, value) in captures.into_iter().rev() {
            body = SExpr::new(
                SNode::Let {
                    name,
                    ty: ty.map(Into::into),
                    value: Box::new(value),
                    body: Box::new(body),
                },
                range,
            );
        }
        self.origins.mark(
            &body,
            if predicate {
                lookup_result_origin(&BOOL, "", true, path)
            } else {
                result_origin
            },
        );
        Ok(body)
    }
}
