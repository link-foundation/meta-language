//! Actual standard iterator syntax/source facts into shared reduction plans.
use super::iterator_pipelines::{self, Pipeline};
use super::{
    BOOL, BinaryOp, FLOAT, Flavor, Options, Result, RustParser, SExpr, SNode, SourceOrigin, Span,
    Token, Type, unsupported,
};
use crate::translation::rust_array_index_rules::read_rust_scalar_array_index_receiver_error;
use crate::translation::rust_array_lookup_rules::read_rust_array_lookup_element_owner_error;
use crate::translation::rust_iterator_pipeline_rules as pipeline_rules;
use crate::translation::rust_iterator_reduction_rules as rules;
use crate::translation::source_signatures::SourceOwner;
use crate::translation::surface::SParam;
use std::collections::BTreeMap;
fn count(value: usize) -> f64 {
    value as f64
}
fn fail(reason: &str, range: Option<Span>) -> crate::translation::diagnostics::TranslationError {
    unsupported("Rust iterator reduction", reason, range)
}
fn boolean(path: &[String]) -> SourceOrigin {
    let mut origin = SourceOrigin::with_type(BOOL, path, Vec::new());
    origin.source_owner = Some(SourceOwner {
        kind: "primitive".to_owned(),
        path: vec!["bool".to_owned()],
        arguments: Vec::new(),
    });
    origin
}
impl RustParser {
    pub(super) fn iterator_reduction(
        &mut self,
        receiver: &SExpr,
        method: &Token,
        args: &[SExpr],
        range: Option<Span>,
        path: &[String],
    ) -> Result<Option<SExpr>> {
        if !rules::accept_rust_iterator_reduction_start(
            &method.value,
            count(args.len()),
            self.ordinary_completion_context(),
        ) {
            return Ok(None);
        }
        let actual = self.origins.origin(receiver, path);
        let result = self.iterator_reduction_value(receiver, method, args, range, path)?;
        if result.is_some() && method.value != "into_iter" {
            self.origins.check_owned_origin(&actual, range)?;
        }
        Ok(result)
    }
    fn iterator_reduction_value(
        &mut self,
        receiver: &SExpr,
        method: &Token,
        args: &[SExpr],
        mut range: Option<Span>,
        path: &[String],
    ) -> Result<Option<SExpr>> {
        let actual = self.origins.origin(receiver, path);
        let source = actual
            .static_byte_array_read_view()
            .unwrap_or_else(|| actual.clone());
        let element = match &source.ty {
            Some(Type::Array { element, .. }) => Some(element.as_ref()),
            _ => None,
        };
        let owner = super::array_lookups::source_owners::element_owner(
            source
                .expanded_source_owner
                .as_ref()
                .or(source.source_owner.as_ref()),
        );
        let known_receiver = read_rust_scalar_array_index_receiver_error(
            source.ty.as_ref().map_or("", Type::kind),
            element.map_or("", Type::kind),
            source.array_source_model.unwrap_or(""),
            source.proven,
            source
                .wrappers
                .iter()
                .all(|layer| layer == "reference" || layer == "box"),
            source.array_element_copy_evidence,
        )
        .is_empty();
        let known_owner = if let (Some(element), Some(owner)) = (element, owner) {
            let (bits, signed) = match element {
                Type::Fixed { bits, signed } => (bits.to_string(), *signed),
                _ => (String::new(), false),
            };
            owner.arguments.is_empty()
                && (owner.kind == "primitive" || owner.kind == "unit" && owner.path.is_empty())
                && read_rust_array_lookup_element_owner_error(
                    element.kind(),
                    &bits,
                    signed,
                    &owner.path.join("::"),
                )
                .is_empty()
        } else {
            false
        };
        let byte_error =
            crate::translation::rust_string_byte_view_rules::read_rust_string_byte_iterator_error(
                &method.value,
                source.source_byte_iterator,
            );
        if !byte_error.is_empty() {
            return Err(fail(&byte_error, range));
        }
        let owned = method.value == "into_iter";
        if owned {
            let error = rules::read_rust_iterator_owned_receiver_error(
                source.ty.as_ref().map_or("", Type::kind),
                source.array_source_model.unwrap_or(""),
                source.proven,
                count(source.wrappers.len()),
                self.origins.fresh_vector_argument(receiver, path, true)
                    || self.origins.available_owned_binding(&actual),
            );
            if !error.is_empty() {
                return Err(fail(&error, Some(method.span())));
            }
            self.origins
                .consume_owned_binding(&actual, Some(method.span()))?;
        }
        if self.cursor.eat(".").is_none() || !self.cursor.is_kind(super::TokenKind::Identifier) {
            return Err(fail(
                "an iterator requires its immediate copied or cloned reduction",
                range,
            ));
        }
        let mut consumer = self.cursor.advance();
        let mut state = rules::read_rust_iterator_reduction_initial_state(&method.value);
        let mut direction = "forward".to_owned();
        let mut stages = Vec::new();
        let mut current_item = element.map(|ty| {
            let mut item = SourceOrigin::with_type(ty.clone(), path, Vec::new());
            item.source_owner = owner.cloned();
            if state == "borrowed" {
                item.wrappers.push("reference".to_owned());
                item.source_lifetimes.push(None);
            }
            item
        });
        while rules::accept_rust_iterator_reduction_adapter(&consumer.value)
            || pipeline_rules::accept_rust_iterator_pipeline_stage(&consumer.value)
        {
            if pipeline_rules::accept_rust_iterator_pipeline_stage(&consumer.value) {
                let error =
                    rules::read_rust_iterator_reduction_receiver_error(known_receiver, known_owner);
                if !error.is_empty() {
                    return Err(fail(&error, Some(consumer.span())));
                }
                let stage = iterator_pipelines::read_stage(
                    self,
                    current_item.as_ref().expect("proved scalar item"),
                    &consumer,
                    path,
                )?;
                state = pipeline_rules::read_rust_iterator_pipeline_result_state(
                    &consumer.value,
                    &state,
                );
                current_item = Some(stage.output.clone());
                stages.push(stage);
            } else {
                let next = rules::read_rust_iterator_reduction_adapter_state(
                    &consumer.value,
                    &state,
                    &direction,
                );
                if next.len() != 2 {
                    return Err(fail(
                        "the iterator adapter needs its actual item indirection and direction",
                        Some(consumer.span()),
                    ));
                }
                state = next[0].clone();
                direction = next[1].clone();
                if let Some(item) = current_item.as_mut() {
                    item.wrappers = if state == "borrowed" {
                        vec!["reference".to_owned()]
                    } else {
                        Vec::new()
                    };
                    item.source_lifetimes = if state == "borrowed" {
                        vec![None]
                    } else {
                        Vec::new()
                    };
                }
                self.cursor.expect("(", Some("iterator adapter"))?;
                self.cursor.expect(")", Some("iterator adapter"))?;
            }
            if self.cursor.eat(".").is_none() || !self.cursor.is_kind(super::TokenKind::Identifier)
            {
                return Err(fail(
                    "an iterator requires its complete immediate reduction",
                    range,
                ));
            }
            consumer = self.cursor.advance();
        }

        let operation = rules::read_rust_iterator_reduction_operation(&consumer.value);
        if operation.is_empty() {
            return Err(fail(
                "the complete standard iterator reduction needs a registered operation",
                Some(consumer.span()),
            ));
        }
        let error = if operation == "count" {
            rules::read_rust_iterator_count_receiver_error(
                source.ty.as_ref().map_or("", Type::kind),
                source.array_source_model.unwrap_or(""),
                source.proven,
                source
                    .wrappers
                    .iter()
                    .all(|layer| layer == "reference" || layer == "box"),
                owned || state == "borrowed" || known_receiver && known_owner,
            )
        } else {
            rules::read_rust_iterator_reduction_receiver_error(known_receiver, known_owner)
        };
        if !error.is_empty() {
            return Err(fail(&error, range));
        }
        let element = element.cloned();
        let owner = owner.cloned();
        if crate::translation::rust_iterator_arithmetic_rules::accept_rust_iterator_arithmetic_consumer(&consumer.value){
            let (result,close)=super::iterator_arithmetic::read(self,current_item.as_ref().expect("proved numeric item"),path,&consumer)?;
            range=Some(Span::new(receiver.span.map_or(method.start,|span|span.start),close.end));
            return iterator_pipelines::construct(self,Pipeline{receiver,source:&source,stages:&stages,consumer:&consumer.value,direction:&direction,input_element:element.as_ref().unwrap(),item:current_item.as_ref().unwrap(),initial:None,callback:None,result,span:range}).map(Some);
        }
        if operation == "collect" {
            let item = current_item.as_ref().expect("proved collection item");
            let final_element = item.ty.clone().expect("proved collection element");
            let final_owner = item
                .expanded_source_owner
                .as_ref()
                .or(item.source_owner.as_ref())
                .expect("proved collection owner")
                .clone();
            let copied = state == "copied" || state == "owned";
            let early = if copied {
                String::new()
            } else {
                rules::read_rust_iterator_collection_error(
                    false, false, "", "", "", "", 0.0, false, 0.0,
                )
            };
            if !early.is_empty() {
                return Err(fail(&early, Some(consumer.span())));
            }
            self.cursor.expect("::", Some("iterator collection type"))?;
            self.cursor.expect("<", Some("iterator collection type"))?;
            let mut expected_item =
                SourceOrigin::with_type(final_element.clone(), path, Vec::new());
            expected_item.source_owner = Some(final_owner.clone());
            let previous = self
                .iterator_item_type_inference_origin
                .replace(expected_item);
            let parsed = self.parse_type();
            let mut origin = self.origins.current_type.clone();
            self.iterator_item_type_inference_origin = previous;
            let ty = parsed?;
            self.split_type_closing_token();
            self.cursor.expect(">", Some("iterator collection type"))?;
            self.cursor.expect("(", Some("iterator collect"))?;
            let close = self.cursor.expect(")", Some("iterator collect"))?;
            range = Some(Span::new(
                receiver.span.map_or(method.start, |span| span.start),
                close.end,
            ));
            let actual_owner = origin
                .expanded_source_owner
                .as_ref()
                .or(origin.source_owner.as_ref());
            let agrees = matches!(&ty,Type::Array{element:actual,..} if actual.as_ref()==&final_element)
                && actual_owner
                    .and_then(|owner| owner.arguments.first())
                    .is_some_and(|actual| {
                        actual.kind == final_owner.kind
                            && actual.path == final_owner.path
                            && actual.arguments.is_empty()
                    });
            let error = rules::read_rust_iterator_collection_error(
                copied,
                origin.proven,
                ty.kind(),
                origin.array_source_model.unwrap_or(""),
                actual_owner.map_or("", |owner| owner.kind.as_str()),
                &actual_owner
                    .map(|owner| owner.path.join("::"))
                    .unwrap_or_default(),
                actual_owner.map_or(0, |owner| owner.arguments.len()) as f64,
                agrees,
                count(origin.wrappers.len()),
            );
            if !error.is_empty() {
                return Err(fail(&error, Some(consumer.span())));
            }
            if !stages.is_empty() {
                origin.path = path.to_vec();
                origin.source_lifetimes.clear();
                origin.array_element_copy_evidence = true;
                origin.fresh_array_value = range;
                origin.fresh_vector_result_contract = false;
                return iterator_pipelines::construct(
                    self,
                    Pipeline {
                        receiver,
                        source: &source,
                        stages: &stages,
                        consumer: &consumer.value,
                        direction: &direction,
                        input_element: element.as_ref().expect("proved input element"),
                        item: current_item.as_ref().unwrap(),
                        initial: None,
                        callback: None,
                        result: origin,
                        span: range,
                    },
                )
                .map(Some);
            }
            let name = self.fresh_product_binding();
            let local = SExpr::new(
                SNode::Name {
                    path: vec![name.clone()],
                },
                range,
            );
            let object = construct_iterator_receiver(
                self,
                local,
                &name,
                element.as_ref().expect("proved input element"),
                &consumer.value,
                &direction,
                range,
            )?;
            origin.path = path.to_vec();
            origin.source_lifetimes.clear();
            origin.array_element_copy_evidence = true;
            origin.fresh_array_value = range;
            origin.fresh_vector_result_contract = false;
            let value = SExpr::new(
                SNode::Method {
                    object: Box::new(object),
                    op: "slice".to_owned(),
                    args: Vec::new(),
                },
                range,
            );
            self.origins.mark_vector_constructor(&value, origin.clone());
            let body = SExpr::new(
                SNode::Let {
                    name,
                    ty: Some(source.ty.clone().unwrap().into()),
                    value: Box::new(receiver.clone()),
                    body: Box::new(value),
                },
                range,
            );
            self.origins.mark(&body, origin);
            return Ok(Some(body));
        }
        if operation == "count" {
            self.cursor.expect("(", Some("iterator count"))?;
            let close = self.cursor.expect(")", Some("iterator count"))?;
            range = Some(Span::new(
                receiver.span.map_or(method.start, |span| span.start),
                close.end,
            ));
            if !stages.is_empty() {
                return iterator_pipelines::construct(
                    self,
                    Pipeline {
                        receiver,
                        source: &source,
                        stages: &stages,
                        consumer: &consumer.value,
                        direction: &direction,
                        input_element: element.as_ref().expect("proved input element"),
                        item: current_item.as_ref().unwrap(),
                        initial: None,
                        callback: None,
                        result: iterator_pipelines::count_origin(path),
                        span: range,
                    },
                )
                .map(Some);
            }
            let name = rules::read_rust_iterator_reduction_count_result_owner();
            let ty = super::rust_fixed_type(&name).expect("registered source count owner");
            let mut stack: Vec<SExpr> = Vec::new();
            for step in rules::plan_rust_iterator_reduction_count() {
                let value = match step.as_str() {
                    "receiver" => receiver.clone(),
                    "integer length" => SExpr::new(
                        SNode::Length {
                            object: Box::new(stack.pop().expect("registered count operand")),
                            integer: true,
                        },
                        range,
                    ),
                    "result cast" => SExpr::new(
                        SNode::Cast {
                            arg: Box::new(stack.pop().expect("registered count operand")),
                            to: ty.clone(),
                            from: None,
                            flavor: Flavor::Exact,
                        },
                        range,
                    ),
                    _ => {
                        return Err(fail(
                            "an iterator count plan needs its registered common operation",
                            range,
                        ));
                    }
                };
                stack.push(value);
            }
            if stack.len() != 1 {
                return Err(fail(
                    "an iterator count plan needs exactly one common term",
                    range,
                ));
            }
            let value = stack.pop().expect("complete count plan");
            let mut origin = SourceOrigin::with_type(ty, path, Vec::new());
            origin.source_owner = Some(SourceOwner {
                kind: "primitive".to_owned(),
                path: vec![name],
                arguments: Vec::new(),
            });
            self.origins.mark(&value, origin);
            return Ok(Some(value));
        }
        let element = element.expect("proved scalar element");
        let _owner = owner.expect("proved scalar owner");
        self.cursor.expect("(", Some("iterator reduction"))?;
        let mut initial = None;
        let mut initial_origin = None;
        if rules::read_rust_iterator_reduction_callback_arity(&consumer.value) == 2.0 {
            let value = self.expr(path, Options::default())?;
            let origin = self.origins.origin(&value, path);
            let error = rules::read_rust_iterator_reduction_accumulator_error(
                origin.ty.is_some() && origin.proven,
                origin.direct_copy_value(),
                count(origin.wrappers.len()),
            );
            if !error.is_empty() {
                return Err(fail(&error, value.span));
            }
            self.cursor.expect(",", Some("iterator fold"))?;
            initial = Some(value);
            initial_origin = Some(origin);
        }
        let item = current_item.clone().expect("proved scalar item");
        let result = initial_origin.clone().unwrap_or_else(|| boolean(path));
        let parameters = if let Some(accumulator) = &initial_origin {
            vec![accumulator.clone(), item]
        } else {
            vec![item]
        };
        let mut expected = SourceOrigin::with_type(
            Type::Function {
                params: parameters
                    .iter()
                    .map(|parameter| parameter.ty.clone().unwrap())
                    .collect(),
                ret: Box::new(result.ty.clone().unwrap()),
            },
            path,
            Vec::new(),
        );
        expected.copy_evidence = Some(());
        expected.callable_parameters = Some(parameters.clone());
        expected.callable_result = Some(Box::new(result.clone()));
        let callback = self.read_expected_copy_closure(path, Some(&expected))?;
        self.cursor.eat(",");
        let close = self.cursor.expect(")", Some("iterator reduction"))?;
        range = Some(Span::new(
            receiver.span.map_or(method.start, |span| span.start),
            close.end,
        ));
        let callable = self.origins.origin(&callback, path);
        let actual_parameters = callable.callable_parameters.as_deref().unwrap_or(&[]);
        let known = actual_parameters
            .iter()
            .map(SourceOrigin::copy_closure_parameter)
            .collect::<Vec<_>>();
        let count_parameters = match &callable.ty {
            Some(Type::Function { params, .. }) => count(params.len()),
            _ => -1.0,
        };
        let error = rules::read_rust_iterator_reduction_callback_error(
            &consumer.value,
            callable.ty.as_ref().map_or("", Type::kind),
            count_parameters,
            count(callable.wrappers.len()),
            &known,
            callable
                .callable_result
                .as_ref()
                .is_some_and(|result| result.proven && result.ty.is_some()),
        );
        if !error.is_empty() {
            return Err(fail(&error, callback.span));
        }
        for (actual, expected) in actual_parameters.iter().zip(&parameters) {
            self.origins.argument(actual, expected, callback.span)?;
        }
        self.origins.argument(
            callable.callable_result.as_deref().unwrap(),
            &result,
            callback.span,
        )?;
        if !stages.is_empty() {
            return iterator_pipelines::construct(
                self,
                Pipeline {
                    receiver,
                    source: &source,
                    stages: &stages,
                    consumer: &consumer.value,
                    direction: &direction,
                    input_element: &element,
                    item: current_item.as_ref().unwrap(),
                    initial: initial.zip(initial_origin),
                    callback: Some((callback, callable)),
                    result,
                    span: range,
                },
            )
            .map(Some);
        }
        let mut values = BTreeMap::from([
            (
                "receiver".to_owned(),
                (receiver.clone(), source.ty.clone().unwrap()),
            ),
            ("callback".to_owned(), (callback, callable.ty.unwrap())),
        ]);
        if let (Some(initial), Some(origin)) = (initial, initial_origin) {
            values.insert("initial".to_owned(), (initial, origin.ty.unwrap()));
        }
        let roles = rules::read_rust_iterator_reduction_capture_roles(&consumer.value);
        let mut captures = Vec::new();
        let mut names = BTreeMap::new();
        for role in &roles {
            let name = self.fresh_product_binding();
            let (value, ty) = values.remove(role).ok_or_else(|| {
                fail(
                    "a reduction capture plan lost an actual source operand",
                    range,
                )
            })?;
            names.insert(role.clone(), name.clone());
            captures.push((name, ty, value));
        }
        let local = |role: &str| {
            SExpr::new(
                SNode::Name {
                    path: vec![names[role].clone()],
                },
                range,
            )
        };
        let mut arguments = vec![Some(local("callback"))];
        if rules::read_rust_iterator_reduction_callback_arity(&consumer.value) == 2.0 {
            arguments.push(Some(local("initial")));
        }
        let object = construct_iterator_receiver(
            self,
            local("receiver"),
            &names["receiver"],
            &element,
            &consumer.value,
            &direction,
            range,
        )?;
        let mut body = SExpr::new(
            SNode::Method {
                object: Box::new(object),
                op: operation,
                args: arguments,
            },
            range,
        );
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
        self.origins.mark(&body, result);
        Ok(Some(body))
    }
}

pub(super) fn construct_iterator_receiver(
    parser: &mut RustParser,
    receiver: SExpr,
    receiver_name: &str,
    element: &Type,
    consumer: &str,
    direction: &str,
    range: Option<Span>,
) -> Result<SExpr> {
    let mut object = receiver.clone();
    for operation in rules::plan_rust_iterator_reduction_receiver(consumer, direction) {
        if operation != "map reversed indices" {
            return Err(fail(
                "an iterator receiver plan needs its registered common operation",
                range,
            ));
        }
        let unused = parser.fresh_product_binding();
        let index_name = parser.fresh_product_binding();
        let mut stack: Vec<SExpr> = Vec::new();
        for step in rules::plan_rust_iterator_reduction_reverse_index() {
            let value = match step.as_str() {
                "receiver" => receiver.clone(),
                "index" => SExpr::new(
                    SNode::Name {
                        path: vec![index_name.clone()],
                    },
                    range,
                ),
                "one" => SExpr::new(
                    SNode::Num {
                        value: "1".to_owned(),
                        ty: Some(FLOAT),
                        negative: false,
                        unit: false,
                    },
                    range,
                ),
                "length" => SExpr::new(
                    SNode::Length {
                        object: Box::new(stack.pop().expect("registered plan operand")),
                        integer: false,
                    },
                    range,
                ),
                "natural" => SExpr::new(
                    SNode::Cast {
                        arg: Box::new(stack.pop().expect("registered plan operand")),
                        to: Type::Nat,
                        from: None,
                        flavor: Flavor::Exact,
                    },
                    range,
                ),
                "sub" => {
                    let right = stack.pop().expect("registered plan operand");
                    let left = stack.pop().expect("registered plan operand");
                    SExpr::new(
                        SNode::Binary {
                            op: BinaryOp::Sub,
                            left: Box::new(left),
                            right: Box::new(right),
                            rounding: None,
                        },
                        range,
                    )
                }
                "lookup" => {
                    let index = stack.pop().expect("registered plan operand");
                    let object = stack.pop().expect("registered plan operand");
                    SExpr::new(
                        SNode::Index {
                            object: Box::new(object),
                            index: Box::new(index),
                            optional: None,
                            proven_in_bounds: None,
                        },
                        range,
                    )
                }
                _ => {
                    return Err(fail(
                        "an iterator index plan needs its registered common operation",
                        range,
                    ));
                }
            };
            stack.push(value);
        }
        if stack.len() != 1 {
            return Err(fail(
                "an iterator index plan needs exactly one common term",
                range,
            ));
        }
        let parameters = [(unused, element.clone()), (index_name, FLOAT)]
            .into_iter()
            .map(|(name, ty)| SParam {
                name,
                optional: false,
                default_value: None,
                ty: Some(ty),
                span: range,
                guard: None,
                rocq_type: None,
            })
            .collect();
        let callback = SExpr::new(
            SNode::Lambda {
                params: parameters,
                ret: Some(element.clone()),
                captures: vec![receiver_name.to_owned()],
                assigned_captures: None,
                body: Box::new(stack.pop().expect("complete plan")),
            },
            range,
        );
        object = SExpr::new(
            SNode::Method {
                object: Box::new(object),
                op: "map".to_owned(),
                args: vec![Some(callback)],
            },
            range,
        );
    }
    Ok(object)
}
