//! Actual callback/source facts into shared lazy common-term plans.
use super::super::UNIT;
use super::{
    BOOL, BinaryOp, Result, RustParser, SArrayItem, SExpr, SNode, SourceOrigin, Span, Token, Type,
    unsupported,
};
use crate::translation::rust_iterator_pipeline_rules as rules;
use crate::translation::source_signatures::SourceOwner;
use crate::translation::surface::SParam;
use std::collections::BTreeMap;
fn fail(reason: &str, span: Option<Span>) -> crate::translation::diagnostics::TranslationError {
    unsupported("Rust iterator pipeline", reason, span)
}
fn owner_known(origin: &SourceOrigin) -> bool {
    let Some(ty) = origin.ty.as_ref() else {
        return false;
    };
    let Some(owner) = origin
        .expanded_source_owner
        .as_ref()
        .or(origin.source_owner.as_ref())
    else {
        return false;
    };
    let (bits, signed) = match ty {
        Type::Fixed { bits, signed } => (bits.to_string(), *signed),
        _ => (String::new(), false),
    };
    owner.arguments.is_empty()
        && (owner.kind == "primitive" || owner.kind == "unit" && owner.path.is_empty())
        && crate::translation::rust_array_lookup_rules::read_rust_array_lookup_element_owner_error(
            ty.kind(),
            &bits,
            signed,
            &owner.path.join("::"),
        )
        .is_empty()
}
pub(super) struct Stage {
    pub operation: String,
    pub callback: SExpr,
    pub ty: Type,
    pub output: SourceOrigin,
}
pub(super) fn read_stage(
    parser: &mut RustParser,
    item: &SourceOrigin,
    method: &Token,
    path: &[String],
) -> Result<Stage> {
    let layers = rules::read_rust_iterator_pipeline_parameter_layers(
        &method.value,
        item.wrappers.len() as f64,
    ) as usize;
    let mut parameter = item.clone();
    parameter.wrappers = vec!["reference".to_owned(); layers];
    parameter.source_lifetimes = vec![None; layers];
    let context = rules::read_rust_iterator_pipeline_result_context(&method.value);
    let infer = context == "infer";
    let result = if infer {
        None
    } else {
        let mut value = SourceOrigin::with_type(
            if context == "unit" { UNIT } else { BOOL },
            path,
            Vec::new(),
        );
        value.source_owner = Some(SourceOwner {
            kind: if context == "unit" {
                "unit"
            } else {
                "primitive"
            }
            .to_owned(),
            path: if context == "unit" {
                Vec::new()
            } else {
                vec!["bool".to_owned()]
            },
            arguments: Vec::new(),
        });
        Some(value)
    };
    let mut expected = SourceOrigin::with_type(
        Type::Function {
            params: vec![parameter.ty.clone().unwrap()],
            ret: Box::new(
                result
                    .as_ref()
                    .and_then(|value| value.ty.clone())
                    .unwrap_or(UNIT),
            ),
        },
        path,
        Vec::new(),
    );
    expected.copy_evidence = Some(());
    expected.callable_parameters = Some(vec![parameter.clone()]);
    expected.callable_result = result.clone().map(Box::new);
    parser.cursor.expect("(", Some("iterator stage"))?;
    let callback = parser.read_expected_copy_closure_result_mode(path, Some(&expected), infer)?;
    parser.cursor.eat(",");
    parser.cursor.expect(")", Some("iterator stage"))?;
    let callable = parser.origins.origin(&callback, path);
    let actual = callable
        .callable_parameters
        .as_ref()
        .and_then(|values| values.first());
    let output = callable.callable_result.as_deref();
    let parameters = match &callable.ty {
        Some(Type::Function { params, .. }) => params.len() as f64,
        _ => -1.0,
    };
    let error = rules::read_rust_iterator_pipeline_callable_error(
        callable.ty.as_ref().map_or("", Type::kind),
        parameters,
        callable.wrappers.len() as f64,
        actual.is_some_and(SourceOrigin::copy_closure_parameter),
        output.is_some_and(|value| value.proven),
        output.is_some_and(SourceOrigin::direct_copy_value),
        output.is_some_and(owner_known),
    );
    if !error.is_empty() {
        return Err(fail(&error, callback.span));
    }
    parser
        .origins
        .argument(actual.unwrap(), &parameter, callback.span)?;
    if let Some(expected) = result.as_ref() {
        parser
            .origins
            .argument(output.unwrap(), expected, callback.span)?;
    }
    Ok(Stage {
        operation: method.value.clone(),
        callback,
        ty: callable.ty.unwrap(),
        output: if infer {
            output.unwrap().clone()
        } else {
            item.clone()
        },
    })
}
pub(super) fn count_origin(path: &[String]) -> SourceOrigin {
    let owner=crate::translation::rust_iterator_reduction_rules::read_rust_iterator_reduction_count_result_owner();
    let mut origin =
        SourceOrigin::with_type(super::rust_fixed_type(&owner).unwrap(), path, Vec::new());
    origin.source_owner = Some(SourceOwner {
        kind: "primitive".to_owned(),
        path: vec![owner],
        arguments: Vec::new(),
    });
    origin
}
pub(super) struct Pipeline<'a> {
    pub receiver: &'a SExpr,
    pub source: &'a SourceOrigin,
    pub stages: &'a [Stage],
    pub consumer: &'a str,
    pub direction: &'a str,
    pub input_element: &'a Type,
    pub item: &'a SourceOrigin,
    pub initial: Option<(SExpr, SourceOrigin)>,
    pub callback: Option<(SExpr, SourceOrigin)>,
    pub result: SourceOrigin,
    pub span: Option<Span>,
}
struct Capture {
    name: String,
    value: SExpr,
    ty: Type,
}
fn name(value: &str, range: Option<Span>) -> SExpr {
    SExpr::new(
        SNode::Name {
            path: vec![value.to_owned()],
        },
        range,
    )
}
fn parameter(name: String, ty: Type, range: Option<Span>) -> SParam {
    SParam {
        name,
        ty: Some(ty),
        optional: false,
        default_value: None,
        span: range,
        guard: None,
        rocq_type: None,
    }
}
struct Builder<'a> {
    stages: &'a [Stage],
    stage_names: &'a [String],
    consumer: &'a str,
    accumulator: Option<SExpr>,
    callback: Option<SExpr>,
    result_type: Type,
    item_type: Type,
    range: Option<Span>,
}
impl Builder<'_> {
    fn inactive(&self) -> Result<SExpr> {
        match rules::read_rust_iterator_pipeline_inactive(self.consumer).as_str() {
            "true" => Ok(SExpr::new(SNode::Bool { value: true }, self.range)),
            "false" => Ok(SExpr::new(SNode::Bool { value: false }, self.range)),
            "accumulator" => self.accumulator.clone().ok_or_else(|| {
                fail(
                    "a pipeline terminal needs its registered inactive term",
                    self.range,
                )
            }),
            _ => Err(fail(
                "a pipeline terminal needs its registered inactive term",
                self.range,
            )),
        }
    }
    fn project(
        &self,
        plan: Vec<String>,
        terms: BTreeMap<String, SExpr>,
        binding: Option<(String, Type)>,
    ) -> Result<SExpr> {
        let mut stack: Vec<SExpr> = Vec::new();
        for step in plan {
            if let Some(value) = terms.get(&step) {
                stack.push(value.clone());
                continue;
            }
            let value = match step.as_str() {
                "call one" => {
                    let value = stack.pop().expect("registered term operand");
                    let function = stack.pop().expect("registered term operand");
                    SExpr::new(
                        SNode::App {
                            func: Box::new(function),
                            args: vec![value],
                        },
                        self.range,
                    )
                }
                "call two" => {
                    let value = stack.pop().expect("registered term operand");
                    let acc = stack.pop().expect("registered term operand");
                    let function = stack.pop().expect("registered term operand");
                    SExpr::new(
                        SNode::App {
                            func: Box::new(function),
                            args: vec![acc, value],
                        },
                        self.range,
                    )
                }
                "add" | "multiply" => {
                    let right = stack.pop().expect("registered term operand");
                    let left = stack.pop().expect("registered term operand");
                    SExpr::new(
                        SNode::Binary {
                            op: if step == "add" {
                                BinaryOp::Add
                            } else {
                                BinaryOp::Mul
                            },
                            left: Box::new(left),
                            right: Box::new(right),
                            rounding: None,
                        },
                        self.range,
                    )
                }
                "append" => {
                    let value = stack.pop().expect("registered term operand");
                    let acc = stack.pop().expect("registered term operand");
                    SExpr::new(
                        SNode::Array {
                            items: vec![SArrayItem::new(acc, true), SArrayItem::new(value, false)],
                            element: Some(self.item_type.clone()),
                        },
                        self.range,
                    )
                }
                "bind" => {
                    let body = stack.pop().expect("registered term operand");
                    let value = stack.pop().expect("registered term operand");
                    let (name, ty) = binding.clone().expect("registered mapping binding");
                    SExpr::new(
                        SNode::Let {
                            name,
                            ty: Some(ty.into()),
                            value: Box::new(value),
                            body: Box::new(body),
                        },
                        self.range,
                    )
                }
                "if" => {
                    let otherwise = stack.pop().expect("registered term operand");
                    let then = stack.pop().expect("registered term operand");
                    let cond = stack.pop().expect("registered term operand");
                    SExpr::new(
                        SNode::If {
                            cond: Box::new(cond),
                            then: Box::new(then),
                            otherwise: Box::new(otherwise),
                        },
                        self.range,
                    )
                }
                _ => {
                    return Err(fail(
                        "a pipeline plan needs its registered common operation",
                        self.range,
                    ));
                }
            };
            stack.push(value);
        }
        if stack.len() != 1 {
            return Err(fail(
                "a pipeline plan needs exactly one common term",
                self.range,
            ));
        }
        Ok(stack.pop().unwrap())
    }
    fn process(&self, parser: &mut RustParser, index: usize, value: SExpr) -> Result<SExpr> {
        if index == self.stages.len() {
            let mut terms = BTreeMap::from([
                ("item".to_owned(), value),
                (
                    "one".to_owned(),
                    SExpr::new(
                        SNode::Num {
                            value: "1".to_owned(),
                            ty: Some(self.result_type.clone()),
                            negative: false,
                            unit: false,
                        },
                        self.range,
                    ),
                ),
            ]);
            if let Some(value) = self.callback.as_ref() {
                terms.insert("callback".to_owned(), value.clone());
            }
            if let Some(value) = self.accumulator.as_ref() {
                terms.insert("accumulator".to_owned(), value.clone());
            }
            return self.project(
                rules::plan_rust_iterator_pipeline_terminal(self.consumer),
                terms,
                None,
            );
        }
        let stage = &self.stages[index];
        let application = SExpr::new(
            SNode::App {
                func: Box::new(name(&self.stage_names[index], self.range)),
                args: vec![value.clone()],
            },
            self.range,
        );
        let binding = parser.fresh_product_binding();
        let continued = self.process(
            parser,
            index + 1,
            if rules::read_rust_iterator_pipeline_continuation_item(&stage.operation) == "mapped" {
                name(&binding, self.range)
            } else {
                value
            },
        )?;
        self.project(
            rules::plan_rust_iterator_pipeline_stage(&stage.operation),
            BTreeMap::from([
                ("application".to_owned(), application),
                ("continuation".to_owned(), continued),
                ("inactive".to_owned(), self.inactive()?),
            ]),
            Some((
                binding,
                match &stage.ty {
                    Type::Function { ret, .. } => ret.as_ref().clone(),
                    _ => {
                        return Err(fail(
                            "a pipeline stage needs its actual callable return type",
                            self.range,
                        ));
                    }
                },
            )),
        )
    }
}
pub(super) fn construct(parser: &mut RustParser, pipeline: Pipeline<'_>) -> Result<SExpr> {
    let mut captures = Vec::new();
    let mut stage_names = Vec::new();
    let mut names = BTreeMap::new();
    let mut stage_index = 0usize;
    for role in
        rules::plan_rust_iterator_pipeline_captures(pipeline.consumer, pipeline.stages.len() as f64)
    {
        let (value, ty) = match role.as_str() {
            "receiver" => (
                pipeline.receiver.clone(),
                pipeline.source.ty.clone().unwrap(),
            ),
            "stage" => {
                let stage = &pipeline.stages[stage_index];
                stage_index += 1;
                (stage.callback.clone(), stage.ty.clone())
            }
            "initial" => {
                let (value, origin) = pipeline.initial.as_ref().ok_or_else(|| {
                    fail(
                        "a pipeline capture needs its complete actual source operand",
                        pipeline.span,
                    )
                })?;
                (value.clone(), origin.ty.clone().unwrap())
            }
            "callback" => {
                let (value, origin) = pipeline.callback.as_ref().ok_or_else(|| {
                    fail(
                        "a pipeline capture needs its complete actual source operand",
                        pipeline.span,
                    )
                })?;
                (value.clone(), origin.ty.clone().unwrap())
            }
            _ => {
                return Err(fail(
                    "a pipeline capture needs its complete actual source operand",
                    pipeline.span,
                ));
            }
        };
        let name = parser.fresh_product_binding();
        if role == "stage" {
            stage_names.push(name.clone());
        } else {
            names.insert(role, name.clone());
        }
        captures.push(Capture { name, value, ty });
    }
    let operation = rules::read_rust_iterator_pipeline_consumer_operation(pipeline.consumer);
    let reducing = operation == "reduce";
    let element = parser.fresh_product_binding();
    let accumulator = if reducing {
        Some(parser.fresh_product_binding())
    } else {
        None
    };
    let ty = pipeline.result.ty.clone().unwrap();
    let terminal = names.get("callback").cloned();
    let builder = Builder {
        stages: pipeline.stages,
        stage_names: &stage_names,
        consumer: pipeline.consumer,
        accumulator: accumulator.as_ref().map(|value| name(value, pipeline.span)),
        callback: terminal.as_ref().map(|value| name(value, pipeline.span)),
        result_type: ty.clone(),
        item_type: pipeline.item.ty.clone().unwrap(),
        range: pipeline.span,
    };
    let body = builder.process(parser, 0, name(&element, pipeline.span))?;
    let mut parameters = Vec::new();
    if let Some(name) = accumulator {
        parameters.push(parameter(name, ty.clone(), pipeline.span));
    }
    parameters.push(parameter(
        element,
        pipeline.input_element.clone(),
        pipeline.span,
    ));
    let mut closure_captures = stage_names;
    if let Some(name) = terminal {
        closure_captures.push(name);
    }
    let callback = SExpr::new(
        SNode::Lambda {
            params: parameters,
            ret: Some(ty.clone()),
            captures: closure_captures,
            assigned_captures: None,
            body: Box::new(body),
        },
        pipeline.span,
    );
    let object = super::iterator_reductions::construct_iterator_receiver(
        parser,
        name(&names["receiver"], pipeline.span),
        &names["receiver"],
        pipeline.input_element,
        pipeline.consumer,
        pipeline.direction,
        pipeline.span,
    )?;
    let mut arguments = vec![Some(callback)];
    if reducing {
        let seed_role = rules::read_rust_iterator_pipeline_seed(pipeline.consumer);
        let seed = if seed_role == "unit" {
            SExpr::new(SNode::Unit, pipeline.span)
        } else if seed_role == "numeric identity" {
            let literal=crate::translation::rust_iterator_arithmetic_rules::plan_rust_iterator_arithmetic_seed(pipeline.consumer,ty.kind());
            SExpr::new(
                SNode::Num {
                    value: literal[0].clone(),
                    ty: Some(ty),
                    negative: literal[1] == "negative",
                    unit: false,
                },
                pipeline.span,
            )
        } else if seed_role == "zero" {
            SExpr::new(
                SNode::Num {
                    value: "0".to_owned(),
                    ty: Some(ty),
                    negative: false,
                    unit: false,
                },
                pipeline.span,
            )
        } else if seed_role == "empty array" {
            SExpr::new(
                SNode::Array {
                    items: Vec::new(),
                    element: pipeline.item.ty.clone(),
                },
                pipeline.span,
            )
        } else {
            name(&names["initial"], pipeline.span)
        };
        arguments.push(Some(seed));
    }
    let mut body = SExpr::new(
        SNode::Method {
            object: Box::new(object),
            op: operation,
            args: arguments,
        },
        pipeline.span,
    );
    if pipeline.consumer == "collect" {
        parser
            .origins
            .mark_vector_constructor(&body, pipeline.result.clone());
    }
    for capture in captures.into_iter().rev() {
        body = SExpr::new(
            SNode::Let {
                name: capture.name,
                ty: Some(capture.ty.into()),
                value: Box::new(capture.value),
                body: Box::new(body),
            },
            pipeline.span,
        );
    }
    parser.origins.mark(&body, pipeline.result);
    Ok(body)
}
