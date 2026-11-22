//! Build source expressions over the complete, checked collection packet interface.
use std::{collections::HashMap, convert::Infallible, rc::Rc};

use serde_json::{Value, json};

use super::{SourceCarrier, scalar_domain};
use crate::translation::{
    Span,
    diagnostics::{Result, unsupported},
    ir::{Decl, FnDecl, Program},
    rust::{Options, RustParser, SourceOrigin, Token, span},
    rust_standard_collection_carriers::CollectionSourceDeclarations,
    rust_standard_collection_value_packet::{
        ml_string::Text, read_rust_standard_collection_argument_origin_error,
        read_rust_standard_collection_method_arity, read_rust_standard_collection_method_result,
        test_rust_standard_collection_mutation, test_rust_standard_collection_ordered,
    },
    source_signatures::SourceOwner,
    surface::{
        BinaryOp, Flavor, SArrayItem, SExpr, SNode, SParam, SPattern, SPatternNode, SRecordEntry,
        SRow, UnaryOp,
    },
    types::Type,
};

type Fields = HashMap<String, (SExpr, Type)>;

fn fail(reason: &str, location: Option<Span>) -> crate::translation::diagnostics::TranslationError {
    unsupported("Rust standard collection source call", reason, location)
}

fn library_path(name: &str) -> Vec<String> {
    std::iter::once("crate".to_owned())
        .chain(name.split('.').map(str::to_owned))
        .collect()
}

fn source_type(ty: &Type) -> Type {
    if let Type::Data { name } = ty {
        return Type::Named {
            path: library_path(name),
            span: None,
        };
    }
    ty.map_children(|child| Ok::<_, Infallible>(source_type(child)))
        .expect("infallible type projection")
}

fn parameter(binding: String, ty: Type, span: Option<Span>) -> SParam {
    SParam {
        name: binding,
        optional: false,
        default_value: None,
        ty: Some(ty),
        span,
        guard: None,
        rocq_type: None,
    }
}

fn infer_integer_argument(node: &mut SExpr, expected: &SourceOrigin, actual: &mut SourceOrigin) {
    let Some(expected_ty @ Type::Fixed { .. }) = expected.ty.as_ref() else {
        return;
    };
    let literal = match &mut node.node {
        SNode::Unary {
            op: UnaryOp::Neg,
            arg,
        } => &mut arg.node,
        other => other,
    };
    if let SNode::Num { ty, .. } = literal
        && ty.is_none()
    {
        *ty = Some(expected_ty.clone());
        actual.ty.clone_from(&expected.ty);
        actual.source_owner.clone_from(&expected.source_owner);
        actual
            .expanded_source_owner
            .clone_from(&expected.expanded_source_owner);
        actual.proven = true;
        actual.copy_evidence = expected.copy_evidence;
    }
}

fn name(path: Vec<String>, location: Option<Span>) -> SExpr {
    SExpr::new(SNode::Name { path }, location)
}

fn text(value: &str, location: Option<Span>) -> SExpr {
    SExpr::new(
        SNode::Str {
            value: value.to_owned(),
        },
        location,
    )
}

fn apply(path: Vec<String>, args: Vec<SExpr>, location: Option<Span>) -> SExpr {
    SExpr::new(
        SNode::App {
            func: Box::new(name(path, location)),
            args,
        },
        location,
    )
}

fn callable<'a>(
    program: &'a Program,
    qualified: &str,
    arity: usize,
    location: Option<Span>,
) -> Result<&'a FnDecl> {
    match program.declaration(qualified) {
        Some(Decl::Fn(function)) if function.params.len() == arity => Ok(function),
        _ => Err(fail(
            "the complete checked library callable is unavailable",
            location,
        )),
    }
}

fn encode(
    program: &Program,
    value: Option<&Value>,
    ty: &Type,
    depth: usize,
    location: Option<Span>,
) -> Result<SExpr> {
    encode_metadata(program, value, ty, depth, location, None)
}

fn encode_metadata(
    program: &Program,
    value: Option<&Value>,
    ty: &Type,
    depth: usize,
    location: Option<Span>,
    fields_override: Option<&HashMap<&str, SExpr>>,
) -> Result<SExpr> {
    if depth >= 128 {
        return Err(fail(
            "source metadata exceeds its finite checked type depth",
            location,
        ));
    }
    let child = |value, ty| encode(program, value, ty, depth + 1, location);
    let encode_field = |field: &str, value, ty| {
        if let Some(expression) = fields_override.and_then(|fields| fields.get(field)) {
            return Ok(expression.clone());
        }
        child(value, ty)
    };
    let node = match (ty, value) {
        (Type::Bool, Some(Value::Bool(value))) => SNode::Bool { value: *value },
        (Type::String, Some(Value::String(value))) => SNode::Str {
            value: value.clone(),
        },
        (Type::Float | Type::Fixed { .. } | Type::Int | Type::Nat, Some(Value::Number(value))) => {
            SNode::Num {
                value: value.to_string(),
                ty: Some(source_type(ty)),
                negative: false,
                unit: false,
            }
        }
        (Type::Unit, None) => SNode::Unit,
        (Type::Option { absence, .. }, None) => SNode::None {
            absence: absence.clone(),
        },
        (Type::Option { element, .. }, Some(value)) => SNode::Some {
            value: Box::new(child(Some(value), element)?),
            ty: source_type(ty),
        },
        (Type::Array { element, .. }, Some(Value::Array(values))) => SNode::Array {
            element: Some(source_type(element)),
            items: values
                .iter()
                .map(|value| Ok(SArrayItem::new(child(Some(value), element)?, false)))
                .collect::<Result<_>>()?,
        },
        (Type::Tuple { elements }, Some(Value::Array(values)))
            if elements.len() == values.len() =>
        {
            SNode::Tuple {
                items: values
                    .iter()
                    .zip(elements)
                    .map(|(value, ty)| child(Some(value), ty))
                    .collect::<Result<_>>()?,
                frozen: false,
                product_model: None,
            }
        }
        (Type::Record { fields, .. }, Some(Value::Object(fields_value))) => SNode::Record {
            entries: fields
                .iter()
                .map(|field| {
                    Ok(SRecordEntry {
                        name: Some(field.name.clone()),
                        spread: false,
                        value: encode_field(&field.name, fields_value.get(&field.name), &field.ty)?,
                    })
                })
                .collect::<Result<_>>()?,
            record_projection: None,
            ty: None,
            prototype_definition: None,
            property_order: None,
        },
        (Type::Data { name: datatype }, Some(Value::Object(fields))) => {
            let Some(Decl::Data(data)) = program.declaration(datatype) else {
                return Err(fail(
                    "a collection constant needs its actual checked datatype",
                    location,
                ));
            };
            let constructor = data
                .ctors
                .iter()
                .find(|ctor| Some(ctor.name.as_str()) == fields.get("$").and_then(Value::as_str))
                .ok_or_else(|| {
                    fail(
                        "collection metadata names an unknown checked constructor",
                        location,
                    )
                })?;
            let args = constructor
                .fields
                .iter()
                .map(|member| encode_field(&member.name, fields.get(&member.name), &member.ty))
                .collect::<Result<_>>()?;
            let path = library_path(&format!("{datatype}.{}", constructor.name));
            return Ok(if args.is_empty() {
                name(path, location)
            } else {
                apply(path, args, location)
            });
        }
        _ => {
            return Err(fail(
                "collection metadata does not match its checked library type",
                location,
            ));
        }
    };
    Ok(SExpr::new(node, location))
}

fn cast(expression: SExpr, ty: Type, location: Option<Span>) -> SExpr {
    SExpr::new(
        SNode::Cast {
            arg: Box::new(expression),
            to: ty,
            from: None,
            flavor: Flavor::Exact,
        },
        location,
    )
}

fn tuple_index(expression: SExpr, index: usize, location: Option<Span>) -> SExpr {
    SExpr::new(
        SNode::TupleIndex {
            object: Box::new(expression),
            index,
        },
        location,
    )
}

fn packet_leaf(
    program: &Program,
    ty: &Type,
    domain: &str,
    node: SExpr,
    location: Option<Span>,
) -> Result<SExpr> {
    if domain == "String" {
        let helper = callable(program, "encodeRustStandardCollectionText", 1, location)?;
        return Ok(apply(library_path(&helper.full_name), vec![node], location));
    }
    let tag = if domain == "bool" {
        "rustCollectionBoolean"
    } else {
        "rustCollectionInteger"
    };
    let Type::Data { name } = ty else {
        return Err(fail(
            "a scalar packet requires its actual checked datatype",
            location,
        ));
    };
    let constructor = match program.declaration(name) {
        Some(Decl::Data(data)) => data
            .ctors
            .iter()
            .find(|constructor| constructor.name == tag),
        _ => None,
    }
    .filter(|constructor| constructor.fields.len() == 1 && constructor.fields[0].name == "value")
    .ok_or_else(|| fail("a scalar packet requires its actual value field", location))?;
    let value = if domain == "bool" {
        node
    } else {
        cast(node, Type::Int, location)
    };
    Ok(apply(
        library_path(&format!("{name}.{}", constructor.name)),
        vec![value],
        location,
    ))
}

fn required_field<'a>(
    fields: &'a Fields,
    field: &str,
    location: Option<Span>,
) -> Result<&'a (SExpr, Type)> {
    fields.get(field).ok_or_else(|| {
        fail(
            "the collection packet lost a required checked field",
            location,
        )
    })
}

fn carrier_path(carrier: &SourceCarrier) -> Vec<String> {
    library_path(&format!("{}.CollectionState", carrier.data.name))
}

fn invalid(
    library: &dyn CollectionSourceDeclarations,
    reason: &str,
    location: Option<Span>,
) -> SExpr {
    library.invalid(text(reason, location))
}

impl RustParser {
    pub(in super::super) fn standard_collection_equality_call(
        &mut self,
        path: &[String],
        arguments: &[SExpr],
        module: &[String],
        location: Option<Span>,
    ) -> Result<Option<SExpr>> {
        if self.standard_collections.is_none() {
            return Ok(None);
        }
        let Some((method, owner)) = path.split_last().filter(|(_, owner)| !owner.is_empty()) else {
            return Ok(None);
        };
        let operation = match method.as_str() {
            "eq" => BinaryOp::Eq,
            "ne" => BinaryOp::Ne,
            _ => return Ok(None),
        };
        if !self.standard_collection_trait(
            owner,
            module,
            ["std::cmp::PartialEq", "core::cmp::PartialEq"],
        ) {
            return Ok(None);
        }
        let [left, right] = arguments else {
            return Err(fail(
                "standard PartialEq requires both complete reference arguments",
                location,
            ));
        };
        let origins = [
            self.origins.origin(left, module),
            self.origins.origin(right, module),
        ];
        if origins
            .iter()
            .all(|origin| self.standard_collection_origin(origin).is_none())
        {
            return Ok(None);
        }
        if origins.iter().any(|origin| {
            origin.wrappers.is_empty() || origin.wrappers.iter().any(|layer| layer != "reference")
        }) {
            return Err(fail(
                "standard PartialEq requires both actual shared reference operands",
                location,
            ));
        }
        self.standard_collection_comparison(left, right, operation, true, module, location)
    }

    pub(in super::super) fn standard_collection_comparison(
        &mut self,
        left: &SExpr,
        right: &SExpr,
        operation: BinaryOp,
        borrowed_call: bool,
        module: &[String],
        location: Option<Span>,
    ) -> Result<Option<SExpr>> {
        let left_origin = self.origins.origin(left, module);
        let right_origin = self.origins.origin(right, module);
        let first = self.standard_collection_origin(&left_origin);
        let second = self.standard_collection_origin(&right_origin);
        if first.is_none() && second.is_none() {
            return Ok(None);
        }
        let Some(first) = first
            .filter(|first| second.is_some_and(|second| first.data.name == second.data.name))
            .cloned()
        else {
            return Err(fail(
                "comparison requires the same actual concrete collection owner and domains",
                location,
            ));
        };
        if borrowed_call && right_origin.wrappers.is_empty()
            || !borrowed_call && left_origin.wrappers.len() != right_origin.wrappers.len()
        {
            return Err(fail(
                "comparison requires the actual reference positions of its source operands",
                location,
            ));
        }
        let ordered = !matches!(operation, BinaryOp::Eq | BinaryOp::Ne);
        if ordered
            && !test_rust_standard_collection_ordered(&Text::from_text(
                &first.owner.path.join("::"),
            ))
        {
            return Err(fail(
                "Hash collection ordering has no standard Ord implementation",
                location,
            ));
        }
        self.origins.check_owned_origin(&left_origin, location)?;
        self.origins.check_owned_origin(&right_origin, location)?;
        self.check_collection_operand_effects(left, right)?;
        let library = Rc::clone(
            &self
                .standard_collections
                .as_ref()
                .expect("selected source collection profile")
                .library,
        );
        let program = library.program();
        let qualified = if ordered {
            library.state_and_ordering_names().1
        } else {
            library.callable_names()[2]
        };
        let comparison = callable(program, qualified, 4, location)?;
        let type_identity = &first
            .data
            .source_standard_collection_carrier
            .as_ref()
            .expect("retained concrete collection facet")
            .concrete_identity;
        let origin = if ordered {
            json!({"$":"rustStandardCollectionOrderingOrigin","owner":"core::cmp::PartialOrd","typeIdentity":type_identity,"sourceVersionMinor":self.source_version_minor,"proven":true,"leftAvailable":true,"rightAvailable":true,"customImplementation":false})
        } else {
            let operation = if operation == BinaryOp::Ne {
                "not equal"
            } else {
                "equal"
            };
            json!({"$":"rustStandardCollectionEqualityOrigin","owner":"core::cmp::PartialEq","typeIdentity":type_identity,"operation":operation,"sourceVersionMinor":self.source_version_minor,"proven":true,"leftAvailable":true,"rightAvailable":true,"customImplementation":false})
        };
        let operands = [self.fresh_product_binding(), self.fresh_product_binding()];
        let args = vec![
            name(vec![operands[0].clone()], location),
            name(vec![operands[1].clone()], location),
            encode(
                program,
                Some(&origin),
                &comparison.params[2].ty,
                0,
                location,
            )?,
            encode(
                program,
                Some(&json!(16_777_216)),
                &comparison.params[3].ty,
                0,
                location,
            )?,
        ];
        let compared = apply(library_path(qualified), args, location);
        let completion = self.match_collection_packet(
            program,
            &comparison.ret,
            compared,
            location,
            |_, tag, fields| match (ordered, tag) {
                (false, "rustCollectionEqualityCompleted") => {
                    Ok(required_field(&fields, "value", location)?.0.clone())
                }
                (true, "rustCollectionOrderingCompleted") => {
                    let (value, ty) = required_field(&fields, "value", location)?;
                    let zero = encode(program, Some(&json!(0)), ty, 0, location)?;
                    Ok(SExpr::new(
                        SNode::Binary {
                            op: operation,
                            left: Box::new(value.clone()),
                            right: Box::new(zero),
                            rounding: None,
                        },
                        location,
                    ))
                }
                (
                    _,
                    "rustCollectionEqualityResourceLimit" | "rustCollectionOrderingResourceLimit",
                ) => Ok(library.resource(
                    required_field(&fields, "state", location)?.0.clone(),
                    required_field(&fields, "reason", location)?.0.clone(),
                )),
                (_, "rustCollectionEqualityInvalid" | "rustCollectionOrderingInvalid") => {
                    Ok(library.invalid(required_field(&fields, "reason", location)?.0.clone()))
                }
                _ => Ok(invalid(
                    library.as_ref(),
                    "unexpected collection comparison continuation",
                    location,
                )),
            },
        )?;
        let patterns = operands
            .into_iter()
            .map(|binding| SPattern {
                node: SPatternNode::Ctor {
                    path: carrier_path(&first),
                    args: vec![SPattern {
                        node: SPatternNode::Bind { name: binding },
                        span: location,
                    }],
                },
                span: location,
            })
            .collect();
        let result = SExpr::new(
            SNode::Match {
                scrutinees: vec![left.clone(), right.clone()],
                rows: vec![SRow {
                    patterns,
                    body: completion,
                    span: location,
                }],
            },
            location,
        );
        self.origins.mark(
            &result,
            SourceOrigin::with_type(Type::Bool, module, Vec::new()),
        );
        Ok(Some(result))
    }

    fn match_collection_packet(
        &mut self,
        program: &Program,
        ty: &Type,
        expression: SExpr,
        location: Option<Span>,
        mut build: impl FnMut(&mut Self, &str, Fields) -> Result<SExpr>,
    ) -> Result<SExpr> {
        let Type::Data { name: qualified } = ty else {
            return Err(fail("the packet requires a checked nominal type", location));
        };
        let Some(Decl::Data(data)) = program.declaration(qualified) else {
            return Err(fail(
                "the packet has no complete checked constructor inventory",
                location,
            ));
        };
        let mut rows = Vec::new();
        for constructor in &data.ctors {
            let mut fields = Fields::new();
            let mut args = Vec::new();
            for field in &constructor.fields {
                let binding = self.fresh_product_binding();
                fields.insert(
                    field.name.clone(),
                    (name(vec![binding.clone()], location), field.ty.clone()),
                );
                args.push(SPattern {
                    node: SPatternNode::Bind { name: binding },
                    span: location,
                });
            }
            let pattern = SPattern {
                node: SPatternNode::Ctor {
                    path: library_path(&format!("{qualified}.{}", constructor.name)),
                    args,
                },
                span: location,
            };
            rows.push(SRow {
                patterns: vec![pattern],
                body: build(self, &constructor.name, fields)?,
                span: location,
            });
        }
        Ok(SExpr::new(
            SNode::Match {
                scrutinees: vec![expression],
                rows,
            },
            location,
        ))
    }

    pub(in super::super) fn standard_collection_constructor(
        &mut self,
        path: &[String],
        module: &[String],
        token: &Token,
    ) -> Result<Option<SExpr>> {
        if self.cursor.is("::")
            && self.cursor.is_at("<", 1)
            && self.standard_collection_owner(path, module).is_some()
        {
            self.cursor.advance();
            self.standard_collection_type(path, token)?;
            let explicit = self.origins.current_type.clone();
            let carrier = self
                .standard_collection_origin(&explicit)
                .cloned()
                .expect("parsed concrete collection type");
            self.cursor
                .expect("::", Some("concrete standard collection constructor"))?;
            let method = self
                .cursor
                .identifier(Some("concrete standard collection constructor"))?;
            if !matches!(method.value.as_str(), "new" | "default") {
                return Err(fail(
                    "the concrete type requires its actual supported constructor",
                    Some(method.span()),
                ));
            }
            return self
                .build_collection_constructor(carrier, &method.value, module, token)
                .map(Some);
        }
        let Some((method, receiver)) = path.split_last().filter(|(_, owner)| !owner.is_empty())
        else {
            return Ok(None);
        };
        let owner = self.standard_collection_owner(receiver, module);
        let trait_path = self.resolve(receiver, module);
        let standard_default = matches!(
            trait_path.join("::").as_str(),
            "std::default::Default" | "core::default::Default"
        );
        if self.standard_collections.is_none() || owner.is_none() && !standard_default {
            return Ok(None);
        }
        let location = span(token, self.cursor.peek());
        let Some(carrier) = self
            .standard_sum_source
            .expected_origin
            .as_ref()
            .and_then(|origin| self.standard_collection_origin(origin))
            .cloned()
        else {
            if owner.is_none() {
                return Ok(None);
            }
            return Err(fail(
                "collection constructors require an explicit matching concrete type",
                location,
            ));
        };
        if (standard_default
            && (method != "default"
                || self.origins.lookup_type_owner(receiver, module).is_some()
                || self
                    .origins
                    .lookup_type_owner(&trait_path[..1], module)
                    .is_some()
                || self
                    .origins
                    .lookup_value_owner(&trait_path[..1], module)
                    .is_some()
                || self.origins.scope.contains_key(&receiver[0])))
            || !matches!(method.as_str(), "new" | "default")
            || owner
                .as_ref()
                .is_some_and(|owner| *owner != carrier.owner.path.join("::"))
        {
            return Err(fail(
                "the constructor requires its actual unshadowed collection or Default owner",
                location,
            ));
        }
        self.build_collection_constructor(carrier, method, module, token)
            .map(Some)
    }

    fn build_collection_constructor(
        &mut self,
        carrier: SourceCarrier,
        method: &str,
        module: &[String],
        token: &Token,
    ) -> Result<SExpr> {
        self.cursor
            .expect("(", Some("standard collection constructor"))?;
        let close = self
            .cursor
            .expect(")", Some("zero-argument standard collection constructor"))?;
        let location = span(token, &close);
        let library = Rc::clone(
            &self
                .standard_collections
                .as_ref()
                .expect("selected source collection profile")
                .library,
        );
        let program = library.program();
        let callable = callable(program, "createRustStandardCollectionState", 6, location)?;
        let facet = carrier
            .data
            .source_standard_collection_carrier
            .as_ref()
            .expect("concrete carrier");
        let root = self.fresh_product_binding();
        let source_owner = carrier.owner.path.join("::");
        let dispatch_owner = if method == "default" {
            "core::default::Default"
        } else {
            &source_owner
        };
        let call = json!({
            "$": "rustStandardCollectionCallOrigin", "owner": source_owner,
            "typeIdentity": facet.concrete_identity, "receiverRoot": root, "method": method,
            "dispatchOwner": dispatch_owner, "proven": true, "shadowed": false,
            "mutable": false, "available": false, "overlappingLoan": false, "arguments": [],
        });
        let constants = [
            facet.origin.clone(),
            json!(carrier.domains[0]),
            json!(carrier.domains.get(1).map_or("unit", String::as_str)),
            json!(64),
            json!(root),
            call,
        ];
        let args = constants
            .iter()
            .zip(&callable.params)
            .map(|(value, parameter)| encode(program, Some(value), &parameter.ty, 0, location))
            .collect::<Result<_>>()?;
        let application = apply(library_path(&callable.full_name), args, location);
        let expression = self.match_collection_packet(
            program,
            &callable.ret,
            application,
            location,
            |_, tag, fields| {
                Ok(match tag {
                    "rustCollectionCreated" => apply(
                        carrier_path(&carrier),
                        vec![required_field(&fields, "state", location)?.0.clone()],
                        location,
                    ),
                    "rustCollectionResourceLimit" => library.resource(
                        required_field(&fields, "state", location)?.0.clone(),
                        required_field(&fields, "reason", location)?.0.clone(),
                    ),
                    "rustCollectionInvalid" => {
                        library.invalid(required_field(&fields, "reason", location)?.0.clone())
                    }
                    _ => invalid(
                        library.as_ref(),
                        "the collection constructor returned a non-construction packet",
                        location,
                    ),
                })
            },
        )?;
        let ty = Type::Named {
            path: library_path(&carrier.data.name),
            span: None,
        };
        let mut origin = SourceOrigin::with_type(ty, module, Vec::new());
        origin.source_owner = Some(carrier.owner.clone());
        origin.expanded_source_owner = Some(carrier.owner);
        origin.source_owned_result_proof = true;
        self.origins.mark(&expression, origin);
        Ok(expression)
    }

    fn decode_collection_scalar(
        &mut self,
        library: &dyn CollectionSourceDeclarations,
        value: SExpr,
        ty: &Type,
        domain: &str,
        result: &Type,
        location: Option<Span>,
    ) -> Result<SExpr> {
        self.match_collection_packet(library.program(), ty, value, location, |_, tag, fields| {
            Ok(match (domain, tag) {
                ("String", "rustCollectionString") => {
                    let decoder = callable(
                        library.program(),
                        "decodeRustStandardCollectionText",
                        1,
                        location,
                    )?;
                    apply(
                        library_path(&decoder.full_name),
                        vec![required_field(&fields, "units", location)?.0.clone()],
                        location,
                    )
                }
                ("bool", "rustCollectionBoolean") => {
                    required_field(&fields, "value", location)?.0.clone()
                }
                (_, "rustCollectionInteger") if matches!(result, Type::Fixed { .. }) => cast(
                    required_field(&fields, "value", location)?.0.clone(),
                    result.clone(),
                    location,
                ),
                _ => invalid(
                    library,
                    "collection result lost its actual scalar variant",
                    location,
                ),
            })
        })
    }

    pub(in super::super) fn standard_collection_clone_call(
        &mut self,
        segments: &[String],
        arguments: &[SExpr],
        module: &[String],
        location: Option<Span>,
    ) -> Result<Option<SExpr>> {
        if self.standard_collections.is_none() || segments.last().is_none_or(|name| name != "clone")
        {
            return Ok(None);
        }
        let source_owner = &segments[..segments.len() - 1];
        if !self.standard_collection_trait(
            source_owner,
            module,
            ["std::clone::Clone", "core::clone::Clone"],
        ) {
            return Ok(None);
        }
        let Some(receiver) = arguments.first() else {
            return Ok(None);
        };
        let actual = self.origins.origin(receiver, module);
        if self.standard_collection_origin(&actual).is_none() {
            return Ok(None);
        }
        if arguments.len() != 1 || actual.wrappers.as_slice() != ["reference"] {
            return Err(fail(
                "standard collection Clone requires its actual single shared receiver",
                location,
            ));
        }
        self.lower_standard_collection_method(receiver, "clone", module, location, false, None)
    }

    pub(in super::super) fn standard_collection_method(
        &mut self,
        receiver: &SExpr,
        method: &Token,
        module: &[String],
    ) -> Result<Option<SExpr>> {
        let actual = self.origins.origin(receiver, module);
        if self.standard_collection_origin(&actual).is_none() {
            return Ok(None);
        }
        if matches!(
            method.value.as_str(),
            "iter" | "keys" | "values" | "into_iter"
        ) && self.ordered_iteration_head.is_some()
        {
            self.cursor
                .expect("(", Some("ordered collection iterator"))?;
            let end = self
                .cursor
                .expect(")", Some("ordered collection iterator"))?;
            let (mut node, item) = self
                .standard_collection_iteration(receiver, &method.value, module)?
                .ok_or_else(|| {
                    fail(
                        "iteration requires its actual standard BTree owner",
                        Some(method.span()),
                    )
                })?;
            let origin = self.origins.origin(&node, module);
            node.span = receiver.span.map(|start| Span::new(start.start, end.end));
            self.origins.mark(&node, origin);
            self.ordered_iteration_head
                .as_mut()
                .expect("actual ordered iteration head reader")
                .push(super::OrderedIterationHead {
                    expression: node.clone(),
                    item,
                    owned_vector: None,
                    end_token: self.cursor.index,
                });
            return Ok(Some(node));
        }
        if matches!(method.value.as_str(), "eq" | "ne") {
            self.cursor
                .expect("(", Some("standard collection PartialEq method"))?;
            let argument = self.expr(module, Options::default())?;
            self.cursor.eat(",");
            self.cursor
                .expect(")", Some("standard collection PartialEq method"))?;
            let operation = if method.value == "eq" {
                BinaryOp::Eq
            } else {
                BinaryOp::Ne
            };
            return self.standard_collection_comparison(
                receiver,
                &argument,
                operation,
                true,
                module,
                Some(method.span()),
            );
        }
        self.lower_standard_collection_method(
            receiver,
            &method.value,
            module,
            Some(method.span()),
            true,
            None,
        )
    }

    pub(in super::super) fn standard_collection_iteration(
        &mut self,
        receiver: &SExpr,
        method: &str,
        module: &[String],
    ) -> Result<Option<(SExpr, SourceOrigin)>> {
        let actual = self.origins.origin(receiver, module);
        let Some(carrier) = self.standard_collection_origin(&actual) else {
            return Ok(None);
        };
        let location = receiver.span;
        if !test_rust_standard_collection_ordered(&Text::from_text(&carrier.owner.path.join("::")))
        {
            return Err(fail(
                "ordered iteration requires its actual standard BTree owner",
                location,
            ));
        }
        let carrier = carrier.clone();
        let owned = method == "into_iter";
        let loan = if owned {
            None
        } else {
            if !(actual.wrappers.is_empty() || actual.wrappers.as_slice() == ["reference"])
                || !matches!(
                    (carrier.arguments.len(), method),
                    (1, "iter") | (2, "iter" | "keys" | "values")
                )
            {
                return Err(fail(
                    "a shared iterator needs its actual standard BTree method and receiver",
                    location,
                ));
            }
            Some(self.borrow_ordered_collection_receiver(receiver, actual.clone(), location)?)
        };
        let regions = loan.as_ref().map_or_else(Vec::new, |origin| {
            origin.source_value_borrow_bindings.clone()
        });
        let item_origin = |origin: &SourceOrigin| {
            let mut selected = origin.clone();
            if !owned {
                selected.wrappers = vec!["reference".to_owned()];
                selected.source_lifetimes =
                    vec![actual.source_lifetimes.first().cloned().flatten()];
                selected.source_borrowed_place = true;
                selected.copy_evidence = Some(());
                selected.source_owned_result_proof = false;
                selected.source_value_borrow_bindings = regions.clone();
            }
            selected
        };
        let (collection_module, item) = match (carrier.arguments.as_slice(), method) {
            ([key], _) => ("btree_set", item_origin(key)),
            ([key, _], "keys") => ("btree_map", item_origin(key)),
            ([_, value], "values") => ("btree_map", item_origin(value)),
            ([key, value], _) => {
                let key = item_origin(key);
                let value = item_origin(value);
                let (Some(key_type), Some(value_type)) = (&key.ty, &value.ty) else {
                    return Err(fail(
                        "the ordered map needs its actual key and value types",
                        location,
                    ));
                };
                let mut pair = SourceOrigin::with_type(
                    Type::Tuple {
                        elements: vec![key_type.clone(), value_type.clone()],
                    },
                    module,
                    Vec::new(),
                );
                pair.copy_evidence = key.copy_evidence.and(value.copy_evidence);
                pair.fields = Some(vec![key, value]);
                pair.source_value_borrow_bindings = regions.clone();
                ("btree_map", pair)
            }
            _ => {
                return Err(fail(
                    "the ordered receiver lost its actual scalar domains",
                    location,
                ));
            }
        };
        let element = item
            .ty
            .clone()
            .ok_or_else(|| fail("the ordered receiver lost its actual item type", location))?;
        let mut iteration = SourceOrigin::with_type(
            Type::Array {
                element: Box::new(element),
                reference: false,
            },
            module,
            Vec::new(),
        );
        iteration.source_owner = Some(SourceOwner {
            kind: "library".to_owned(),
            path: [
                "std",
                "collections",
                collection_module,
                match method {
                    "into_iter" => "IntoIter",
                    "keys" => "Keys",
                    "values" => "Values",
                    _ => "Iter",
                },
            ]
            .map(str::to_owned)
            .to_vec(),
            arguments: carrier.owner.arguments.clone(),
        });
        iteration.array_source_model = None;
        iteration.copy_evidence = None;
        iteration.source_borrowed_place = !owned;
        iteration.source_value_borrow_bindings = regions;
        let lowered = self.lower_standard_collection_method(
            receiver,
            method,
            module,
            location,
            false,
            Some(iteration),
        )?;
        Ok(lowered.map(|expression| (expression, item)))
    }

    fn borrow_ordered_collection_receiver(
        &mut self,
        receiver: &SExpr,
        mut actual: SourceOrigin,
        location: Option<Span>,
    ) -> Result<SourceOrigin> {
        self.origins.check_owned_origin(&actual, location)?;
        let identity = actual
            .source_place_binding_identity
            .as_ref()
            .or(actual.source_binding_identity.as_ref());
        let root = identity.map(|id| {
            let ultimate =
                super::super::owned_record_policy::fields::read_rust_owned_record_ultimate_root(id);
            if ultimate.is_empty() {
                id.clone()
            } else {
                ultimate
            }
        });
        let owning = super::super::owned_record_loops::place(self, receiver)
            .map(|place| place.slot.origin.clone())
            .or_else(|| {
                self.origins
                    .scope
                    .values()
                    .find(|origin| {
                        root.is_some()
                            && origin.source_binding_identity == root
                            && origin.wrappers.is_empty()
                    })
                    .cloned()
            });
        if let Some(owning) = owning {
            self.origins
                .register_mutable_record_loan_root(&owning, location, false)?;
        }
        let regions = super::super::borrowed_iterators::argument_regions(&actual)?;
        if !self.origins.collecting && regions.is_empty() {
            return Err(fail(
                "a shared iterator needs actual live receiver-derived source regions",
                location,
            ));
        }
        actual.wrappers = vec!["reference".to_owned()];
        actual.source_borrowed_place = true;
        actual.copy_evidence = Some(());
        actual.source_value_borrow_bindings = regions;
        self.origins
            .capture_mutable_record_value_loans(actual, location)
    }

    fn lower_standard_collection_method(
        &mut self,
        receiver: &SExpr,
        method: &str,
        module: &[String],
        location: Option<Span>,
        read_arguments: bool,
        iteration: Option<SourceOrigin>,
    ) -> Result<Option<SExpr>> {
        let actual = self.origins.origin(receiver, module);
        let Some(carrier) = self.standard_collection_origin(&actual).cloned() else {
            return Ok(None);
        };
        let owner = carrier.owner.path.join("::");
        let owner_text = Text::from_text(&owner);
        let method_text = Text::from_text(method);
        let arity = read_rust_standard_collection_method_arity(&owner_text, &method_text);
        let result_kind =
            read_rust_standard_collection_method_result(&owner_text, &method_text).to_string();
        if matches!(method, "new" | "default") {
            return Err(fail(
                "there is no admitted source method for this collection owner",
                location,
            ));
        }
        let arity = match arity {
            0.0 => 0,
            1.0 => 1,
            2.0 => 2,
            _ => {
                return Err(fail(
                    "there is no admitted source method for this collection owner",
                    location,
                ));
            }
        };
        let mutation = test_rust_standard_collection_mutation(&owner_text, &method_text);
        if mutation && !actual.wrappers.is_empty() {
            return Err(fail(
                "a shared collection reference cannot authorize mutation",
                location,
            ));
        }
        if actual.wrappers.len() > 1 && method == "clone" {
            return Err(fail(
                "nested reference Clone dispatch needs its actual reference-result interpretation",
                location,
            ));
        }
        self.origins.check_owned_name(receiver, module)?;
        if read_arguments {
            self.cursor
                .expect("(", Some("standard collection method"))?;
        }
        let mut arguments = Vec::new();
        let mut argument_facts = Vec::new();
        for index in 0..arity {
            if index != 0 {
                self.cursor
                    .expect(",", Some("collection method arguments"))?;
            }
            let expected = &carrier.arguments[index];
            let mut argument = self
                .with_standard_sum_source_context(Some(expected.clone()), |parser| {
                    parser.expr(module, Options::default())
                })?;
            let source_borrowed = method != "insert";
            let mut source = self.origins.origin(&argument, module);
            infer_integer_argument(&mut argument, expected, &mut source);
            let direct = source.wrappers.is_empty();
            if !source_borrowed && !direct {
                return Err(fail(
                    "collection insertion requires a direct owned source value",
                    argument.span,
                ));
            }
            let borrowed = source.wrappers.as_slice() == ["reference"];
            self.origins.check_owned_origin(&source, argument.span)?;
            let expected_domain = &carrier.domains[index];
            let string_slice = source_borrowed
                && expected_domain == "String"
                && source.ty.as_ref() == Some(&Type::String)
                && source.borrowed_string_slice()
                && (borrowed || matches!(&argument.node, SNode::Str { .. }));
            source.wrappers.clear();
            let domain = if string_slice {
                source.source_owner = Some(SourceOwner {
                    kind: "primitive".to_owned(),
                    path: vec!["str".to_owned()],
                    arguments: Vec::new(),
                });
                source.expanded_source_owner = None;
                "str".to_owned()
            } else {
                if source.expanded_source_owner.is_none()
                    && source.source_owner.is_none()
                    && source.ty.as_ref() == Some(&Type::String)
                    && source.text_source_model == Some("owned string")
                {
                    source.source_owner = Some(SourceOwner {
                        kind: "library".to_owned(),
                        path: ["std", "string", "String"].map(str::to_owned).to_vec(),
                        arguments: Vec::new(),
                    });
                }
                scalar_domain(&source).unwrap_or_default()
            };
            let argument_owner = source
                .expanded_source_owner
                .as_ref()
                .or(source.source_owner.as_ref())
                .filter(|owner| owner.arguments.is_empty())
                .ok_or_else(|| {
                    fail(
                        "collection arguments retain their actual closed scalar owner",
                        argument.span,
                    )
                })?;
            let native_owner = (
                Vec::new(),
                Text::from_text(&argument_owner.kind),
                argument_owner
                    .path
                    .iter()
                    .map(|segment| Text::from_text(segment))
                    .collect(),
            );
            let collecting_local = self.origins.collecting
                && matches!(&argument.node, SNode::Name { path }
                    if path.len() == 1 && self.origins.scope.contains_key(&path[0]));
            let owned_transfer = !source_borrowed
                && direct
                && domain == "String"
                && (self.origins.available_owned_binding(&source)
                    || source.source_owned_result_proof
                    || collecting_local);
            let reason = read_rust_standard_collection_argument_origin_error(
                Some((
                    true,
                    borrowed || string_slice,
                    Text::from_text(&domain),
                    owned_transfer,
                    source.proven,
                    native_owner,
                )),
                &Text::from_text(expected_domain),
                source_borrowed,
            );
            if !reason.0.is_empty() {
                return Err(fail(&reason.to_string(), argument.span));
            }
            argument_facts.push(json!({
                "domain": domain, "proven": source.proven, "available": true,
                "borrowed": borrowed || string_slice, "ownedTransfer": owned_transfer, "sourceOwner": argument_owner,
            }));
            if owned_transfer {
                self.origins
                    .prepare_owned_argument(&argument, Some(expected), module)?;
            }
            self.check_collection_operand_effects(receiver, &argument)?;
            arguments.push(argument);
        }
        if !arguments.is_empty() {
            self.cursor.eat(",");
        }
        if read_arguments {
            self.cursor
                .expect(")", Some("standard collection method"))?;
        }
        let owned_iteration = result_kind == "owned ordered entries" && iteration.is_some();
        let shared_iteration = result_kind == "owned ordered values" && iteration.is_some();
        if owned_iteration {
            if !actual.wrappers.is_empty() || actual.source_borrowed_place {
                return Err(fail(
                    "ordered IntoIterator requires a direct owned BTree receiver",
                    location,
                ));
            }
            self.origins.check_owned_origin(&actual, location)?;
            if !self.origins.collecting {
                let transferred = self.origins.transfer_owned_result(receiver, module)?;
                if !self.origins.available_owned_binding(&transferred)
                    || !transferred.source_owned_result_proof
                    || !self.origins.consume_owned_binding(&transferred, location)?
                {
                    return Err(fail(
                        "ordered IntoIterator requires one actual available owned collection transfer",
                        location,
                    ));
                }
            }
        }
        let place = if mutation {
            Some(self.collection_mutable_state_place(receiver, location)?)
        } else {
            None
        };
        let payload_index = usize::from(carrier.domains.len() == 2);
        let ordered_values = result_kind == "owned ordered values" && !shared_iteration;
        if ordered_values && method == "iter" && payload_index == 1 {
            return Err(fail(
                "ordered map pair iteration requires the actual pair consumer",
                location,
            ));
        }
        let selected_index = if ordered_values || shared_iteration {
            usize::from(method == "values")
        } else {
            payload_index
        };
        let projection = if owned_iteration {
            "owned".to_owned()
        } else if shared_iteration {
            "borrowed".to_owned()
        } else if method == "get" || ordered_values {
            self.cursor
                .expect(".", Some("collection reference projection"))?;
            let selection = self
                .cursor
                .identifier(Some("collection reference projection"))?;
            if !matches!(selection.value.as_str(), "copied" | "cloned")
                || selection.value == "copied" && carrier.domains[selected_index] == "String"
            {
                return Err(fail(
                    "collection borrows require immediate copied or cloned with the actual scalar capability",
                    Some(selection.span()),
                ));
            }
            self.cursor
                .expect("(", Some("collection reference projection"))?;
            self.cursor
                .expect(")", Some("collection reference projection"))?;
            selection.value
        } else {
            String::new()
        };
        let mut result = match result_kind.as_str() {
            "owned collection" => {
                let mut origin = SourceOrigin::with_type(
                    Type::Named {
                        path: library_path(&carrier.data.name),
                        span: None,
                    },
                    module,
                    Vec::new(),
                );
                origin.source_owner = Some(carrier.owner.clone());
                origin.expanded_source_owner = origin.source_owner.clone();
                origin
            }
            "bool" => SourceOrigin::with_type(Type::Bool, module, Vec::new()),
            "unit" => SourceOrigin::with_type(Type::Unit, module, Vec::new()),
            "usize" => SourceOrigin::with_type(
                super::super::rust_fixed_type("usize").expect("usize is represented"),
                module,
                Vec::new(),
            ),
            "borrowed Option" | "owned Option" => self.instantiate_standard_sum_source(
                "std::option::Option",
                vec![carrier.arguments[payload_index].clone()],
                location,
            )?,
            "owned ordered values" if !shared_iteration => {
                self.read_ordered_collection_target(&carrier.arguments[selected_index], location)?
            }
            _ if owned_iteration || shared_iteration => {
                iteration.expect("admitted ordered consumer")
            }
            _ => {
                return Err(fail(
                    "the method requires its actual distinct result interpretation",
                    location,
                ));
            }
        };
        result.source_owned_result_proof = !(owned_iteration || shared_iteration);
        let decoded_kind = if shared_iteration {
            "borrowed ordered entries"
        } else {
            &result_kind
        };
        let library = Rc::clone(
            &self
                .standard_collections
                .as_ref()
                .expect("selected source collection profile")
                .library,
        );
        let program = library.program();
        let [packet_name, _, _] = library.callable_names();
        let packet = callable(program, packet_name, 5, location)?;
        let Type::Array { element, .. } = &packet.params[2].ty else {
            return Err(fail("the packet needs its actual array argument", location));
        };
        let argument_list = SExpr::new(
            SNode::Array {
                element: Some(source_type(element)),
                items: arguments
                    .into_iter()
                    .zip(&carrier.domains)
                    .map(|(argument, domain)| {
                        Ok(SArrayItem::new(
                            packet_leaf(program, element, domain, argument, location)?,
                            false,
                        ))
                    })
                    .collect::<Result<_>>()?,
            },
            location,
        );
        let evaluated_receiver = self.fresh_product_binding();
        let evaluated_arguments = self.fresh_product_binding();
        let facet = carrier
            .data
            .source_standard_collection_carrier
            .as_ref()
            .expect("retained collection facet");
        let clone_method = method == "clone";
        let dispatch = if clone_method {
            "core::clone::Clone"
        } else if owned_iteration {
            "core::iter::IntoIterator"
        } else {
            &owner
        };
        let suffix = format!("/{}", self.fresh_product_binding());
        let mut metadata = json!({
            "$": "rustStandardCollectionCallOrigin", "owner": owner,
            "typeIdentity": facet.concrete_identity, "receiverRoot": "", "method": method,
            "dispatchOwner": dispatch, "proven": true, "shadowed": false,
            "mutable": mutation, "available": true, "overlappingLoan": false, "arguments": argument_facts,
        });
        if owned_iteration {
            metadata["ownedReceiverTransfer"] = json!(true);
        }
        let held = self.fresh_product_binding();
        let state = name(vec![held.clone()], location);
        let state_match =
            self.match_collection_packet(
                program,
                &packet.params[0].ty,
                state.clone(),
                location,
                |parser, tag, fields| {
                    if tag != "rustStandardCollectionState" {
                        return Ok(invalid(
                            library.as_ref(),
                            "the collection carrier lost its actual state variant",
                            location,
                        ));
                    }
                    let root = required_field(&fields, "root", location)?.0.clone();
                    let mut overrides = HashMap::from([("receiverRoot", root.clone())]);
                    if clone_method {
                        let output = SExpr::new(
                            SNode::Binary {
                                op: BinaryOp::Concat,
                                left: Box::new(root),
                                right: Box::new(text(&suffix, location)),
                                rounding: None,
                            },
                            location,
                        );
                        overrides.insert(
                            "outputRoot",
                            SExpr::new(
                                SNode::Some {
                                    value: Box::new(output),
                                    ty: Type::Option {
                                        element: Box::new(Type::String),
                                        absence: "undefined".to_owned(),
                                    },
                                },
                                location,
                            ),
                        );
                    }
                    let call = encode_metadata(
                        program,
                        Some(&metadata),
                        &packet.params[3].ty,
                        0,
                        location,
                        Some(&overrides),
                    )?;
                    let expression = apply(
                        library_path(packet_name),
                        vec![
                            state.clone(),
                            text(method, location),
                            name(vec![evaluated_arguments.clone()], location),
                            call,
                            text(&projection, location),
                        ],
                        location,
                    );
                    parser.match_collection_packet(
                        program,
                        &packet.ret,
                        expression,
                        location,
                        |parser, tag, fields| match tag {
                            "rustCollectionInvalid" => Ok(library
                                .invalid(required_field(&fields, "reason", location)?.0.clone())),
                            "rustCollectionResourceLimit" => Ok(library.resource(
                                required_field(&fields, "state", location)?.0.clone(),
                                required_field(&fields, "reason", location)?.0.clone(),
                            )),
                            "rustCollectionPacketNormal" => {
                                let (values, ty) = required_field(&fields, "values", location)?;
                                let Type::Tuple { elements } = ty else {
                                    return Err(fail(
                                        "the actual packet has no result/state product",
                                        location,
                                    ));
                                };
                                let [value_type, _] = elements.as_slice() else {
                                    return Err(fail(
                                        "the actual packet has no result/state pair",
                                        location,
                                    ));
                                };
                                let value = tuple_index(values.clone(), 0, location);
                                let decoded = parser.decode_collection_result(
                                    library.as_ref(),
                                    &carrier,
                                    value_type,
                                    value,
                                    &result,
                                    decoded_kind,
                                    selected_index,
                                    location,
                                )?;
                                let writeback =
                                    required_field(&fields, "writeback", location)?.0.clone();
                                let correct_writeback = if mutation {
                                    writeback
                                } else {
                                    SExpr::new(
                                        SNode::Unary {
                                            op: UnaryOp::Not,
                                            arg: Box::new(writeback),
                                        },
                                        location,
                                    )
                                };
                                let success = if mutation {
                                    let updated = apply(
                                        carrier_path(&carrier),
                                        vec![tuple_index(values.clone(), 1, location)],
                                        location,
                                    );
                                    SExpr::new(
                                        SNode::Tuple {
                                            items: vec![decoded, updated],
                                            frozen: false,
                                            product_model: None,
                                        },
                                        location,
                                    )
                                } else {
                                    decoded
                                };
                                Ok(SExpr::new(
                                    SNode::If {
                                        cond: Box::new(correct_writeback),
                                        then: Box::new(success),
                                        otherwise: Box::new(invalid(
                                            library.as_ref(),
                                            "collection writeback must match its actual method",
                                            location,
                                        )),
                                    },
                                    location,
                                ))
                            }
                            _ => Err(fail(
                                "the projected packet returned an unsupported constructor",
                                location,
                            )),
                        },
                    )
                },
            )?;
        let expression = SExpr::new(
            SNode::Match {
                scrutinees: vec![name(vec![evaluated_receiver.clone()], location)],
                rows: vec![SRow {
                    patterns: vec![SPattern {
                        node: SPatternNode::Ctor {
                            path: carrier_path(&carrier),
                            args: vec![SPattern {
                                node: SPatternNode::Bind { name: held },
                                span: location,
                            }],
                        },
                        span: location,
                    }],
                    body: state_match,
                    span: location,
                }],
            },
            location,
        );
        let expression = SExpr::new(
            SNode::Let {
                name: evaluated_arguments,
                ty: Some(source_type(&packet.params[2].ty).into()),
                value: Box::new(argument_list),
                body: Box::new(expression),
            },
            location,
        );
        let expression = SExpr::new(
            SNode::Let {
                name: evaluated_receiver,
                ty: Some(
                    actual
                        .ty
                        .clone()
                        .ok_or_else(|| {
                            fail("the receiver needs its actual collection type", location)
                        })?
                        .into(),
                ),
                value: Box::new(receiver.clone()),
                body: Box::new(expression),
            },
            location,
        );
        if let Some(place) = place {
            let ty = Type::Tuple {
                elements: vec![
                    result.ty.clone().expect("actual method result"),
                    actual.ty.expect("actual collection receiver type"),
                ],
            };
            return self
                .owned_record_effect_collection_packet(place, ty, expression, result)
                .map(Some);
        }
        if ordered_values {
            self.origins.mark_vector_constructor(&expression, result)?;
        } else {
            self.origins.mark(&expression, result);
        }
        Ok(Some(expression))
    }

    fn read_ordered_collection_target(
        &mut self,
        element: &SourceOrigin,
        location: Option<Span>,
    ) -> Result<SourceOrigin> {
        self.cursor
            .expect(".", Some("immediate ordered collection"))?;
        self.cursor
            .expect("collect", Some("immediate ordered collection"))?;
        self.cursor
            .expect("::", Some("explicit ordered Vec target"))?;
        self.cursor
            .expect("<", Some("explicit ordered Vec target"))?;
        let vector = self.parse_type()?;
        let mut origin = self.origins.current_type.clone();
        self.split_type_closing_token();
        self.cursor
            .expect(">", Some("explicit ordered Vec target"))?;
        self.cursor
            .expect("(", Some("immediate ordered collection"))?;
        self.cursor
            .expect(")", Some("immediate ordered collection"))?;
        let expected_owner = element
            .expanded_source_owner
            .as_ref()
            .or(element.source_owner.as_ref());
        let Type::Array {
            element: vector_element,
            reference: false,
        } = vector
        else {
            return Err(fail(
                "ordered collection needs its actual owned Vec target",
                location,
            ));
        };
        let owner = origin
            .expanded_source_owner
            .as_ref()
            .or(origin.source_owner.as_ref())
            .filter(|owner| {
                owner.kind == "library"
                    && matches!(
                        owner.path.join("::").as_str(),
                        "std::vec::Vec" | "alloc::vec::Vec"
                    )
            });
        if !origin.wrappers.is_empty()
            || element.ty.as_ref() != Some(vector_element.as_ref())
            || !owner.is_some_and(|owner| {
                owner.arguments.len() == 1 && owner.arguments.first() == expected_owner
            })
        {
            return Err(fail(
                "ordered collection needs its actual same-domain owned Vec target",
                location,
            ));
        }
        origin.array_source_model = Some("owned vector");
        origin.array_element_copy_evidence = element.copy_evidence.is_some();
        Ok(origin)
    }

    fn decode_ordered_collection_entry(
        &mut self,
        library: &dyn CollectionSourceDeclarations,
        carrier: &SourceCarrier,
        input: SExpr,
        packet_element: &Type,
        output: &Type,
        selected_index: usize,
        location: Option<Span>,
    ) -> Result<SExpr> {
        let Type::Record { fields, .. } = packet_element else {
            return Err(fail(
                "ordered entries need their actual checked record layout",
                location,
            ));
        };
        let fields = fields
            .iter()
            .map(|field| {
                let ty = if field.optional {
                    crate::translation::javascript_constructor_presence::field_value_type(&field.ty)
                        .ok_or_else(|| {
                            fail(
                                "an optional entry field lost its actual presence carrier",
                                location,
                            )
                        })?
                } else {
                    &field.ty
                };
                let expression = SExpr::new(
                    SNode::Field {
                        object: Box::new(input.clone()),
                        field: field.name.clone(),
                    },
                    location,
                );
                Ok((field.name.clone(), (expression, ty.clone())))
            })
            .collect::<Result<Fields>>()?;
        let mut decode_field = |field: &str, index: usize, output: &Type| {
            let (expression, ty) = required_field(&fields, field, location)?;
            let (selected, scalar_type) = if let Type::Option { element, .. } = ty {
                (
                    SExpr::new(
                        SNode::Coalesce {
                            left: Box::new(expression.clone()),
                            right: Box::new(invalid(
                                library,
                                "the ordered collection entry lost its selected scalar",
                                location,
                            )),
                        },
                        location,
                    ),
                    element.as_ref(),
                )
            } else {
                (expression.clone(), ty)
            };
            self.decode_collection_scalar(
                library,
                selected,
                scalar_type,
                &carrier.domains[index],
                output,
                location,
            )
        };
        match (carrier.domains.as_slice(), output) {
            ([_], _) => decode_field("key", 0, output),
            ([_, _], Type::Tuple { elements }) => {
                let [key, value] = elements.as_slice() else {
                    return Err(fail(
                        "an ordered map needs its actual key/value pair",
                        location,
                    ));
                };
                Ok(SExpr::new(
                    SNode::Tuple {
                        items: vec![
                            decode_field("key", 0, key)?,
                            decode_field("value", 1, value)?,
                        ],
                        frozen: false,
                        product_model: None,
                    },
                    location,
                ))
            }
            ([_, _], _) => {
                let field = if selected_index == 0 { "key" } else { "value" };
                decode_field(field, selected_index, output)
            }
            _ => Err(fail(
                "the ordered entry lost its actual collection item type",
                location,
            )),
        }
    }

    fn decode_collection_result(
        &mut self,
        library: &dyn CollectionSourceDeclarations,
        carrier: &SourceCarrier,
        ty: &Type,
        value: SExpr,
        result: &SourceOrigin,
        kind: &str,
        selected_index: usize,
        location: Option<Span>,
    ) -> Result<SExpr> {
        let result_type = result.ty.as_ref().ok_or_else(|| {
            fail(
                "the collection result has no concrete source type",
                location,
            )
        })?;
        match kind {
            "bool" | "usize" => {
                self.decode_collection_scalar(library, value, ty, kind, result_type, location)
            }
            "unit" => {
                self.match_collection_packet(library.program(), ty, value, location, |_, tag, _| {
                    Ok(if tag == "rustCollectionUnit" {
                        SExpr::new(SNode::Unit, location)
                    } else {
                        invalid(
                            library,
                            "the collection clear needs its actual Unit result",
                            location,
                        )
                    })
                })
            }
            "owned ordered values" | "owned ordered entries" | "borrowed ordered entries" => self
                .match_collection_packet(
                    library.program(),
                    ty,
                    value,
                    location,
                    |parser, tag, fields| {
                        let field = match (kind, tag) {
                            ("owned ordered values", "rustCollectionOrderedValues") => "values",
                            (
                                "owned ordered entries" | "borrowed ordered entries",
                                "rustCollectionOrderedEntries",
                            ) => "entries",
                            _ => "",
                        };
                        if field.is_empty() {
                            return Ok(invalid(
                                library,
                                "the ordered collection needs its actual complete sequence",
                                location,
                            ));
                        }
                        let (values, values_type) = required_field(&fields, field, location)?;
                        let Type::Array {
                            element: packet_element,
                            ..
                        } = values_type
                        else {
                            return Err(fail(
                                "the ordered collection lost its actual sequence type",
                                location,
                            ));
                        };
                        let Type::Array { element, .. } = result_type else {
                            return Err(fail(
                                "the ordered result needs its actual owned Vec target",
                                location,
                            ));
                        };
                        let accumulator = parser.fresh_product_binding();
                        let item = parser.fresh_product_binding();
                        let input = name(vec![item.clone()], location);
                        let decoded = if field == "values" {
                            parser.decode_collection_scalar(
                                library,
                                input,
                                packet_element,
                                &carrier.domains[selected_index],
                                element,
                                location,
                            )?
                        } else {
                            parser.decode_ordered_collection_entry(
                                library,
                                carrier,
                                input,
                                packet_element,
                                element,
                                selected_index,
                                location,
                            )?
                        };
                        let body = SExpr::new(
                            SNode::Array {
                                element: Some(element.as_ref().clone()),
                                items: vec![
                                    SArrayItem::new(
                                        name(vec![accumulator.clone()], location),
                                        true,
                                    ),
                                    SArrayItem::new(decoded, false),
                                ],
                            },
                            location,
                        );
                        let reduce = SExpr::new(
                            SNode::Lambda {
                                params: vec![
                                    parameter(accumulator, result_type.clone(), location),
                                    parameter(item, source_type(packet_element), location),
                                ],
                                ret: Some(result_type.clone()),
                                captures: Vec::new(),
                                assigned_captures: None,
                                body: Box::new(body),
                            },
                            location,
                        );
                        let initial = SExpr::new(
                            SNode::Array {
                                element: Some(element.as_ref().clone()),
                                items: Vec::new(),
                            },
                            location,
                        );
                        Ok(SExpr::new(
                            SNode::Method {
                                object: Box::new(values.clone()),
                                op: "reduce".to_owned(),
                                args: vec![Some(reduce), Some(initial)],
                            },
                            location,
                        ))
                    },
                ),
            "owned collection" => self.match_collection_packet(
                library.program(),
                ty,
                value,
                location,
                |_, tag, fields| {
                    if tag != "rustStandardCollectionState" {
                        return Ok(invalid(
                            library,
                            "the cloned collection needs its actual state result",
                            location,
                        ));
                    }
                    let Type::Data { name } = ty else {
                        return Err(fail(
                            "the cloned collection state needs its complete datatype",
                            location,
                        ));
                    };
                    let Some(Decl::Data(data)) = library.program().declaration(name) else {
                        return Err(fail(
                            "the cloned collection state has no actual declaration",
                            location,
                        ));
                    };
                    let ctor = data
                        .ctors
                        .iter()
                        .find(|constructor| constructor.name == tag)
                        .ok_or_else(|| {
                            fail("the cloned state constructor is unavailable", location)
                        })?;
                    let arguments = ctor
                        .fields
                        .iter()
                        .map(|field| {
                            required_field(&fields, &field.name, location)
                                .map(|value| value.0.clone())
                        })
                        .collect::<Result<_>>()?;
                    let (state, _) = library.state_and_ordering_names();
                    let projected =
                        apply(library_path(&format!("{state}.{tag}")), arguments, location);
                    Ok(apply(carrier_path(carrier), vec![projected], location))
                },
            ),
            "borrowed Option" | "owned Option" => {
                let sum = result.standard_sum_source.as_deref().ok_or_else(|| {
                    fail(
                        "collection get retains its actual standard Option owner",
                        location,
                    )
                })?;
                let some = sum.constructor_paths.get("Some").ok_or_else(|| {
                    fail(
                        "collection get requires the actual Some constructor",
                        location,
                    )
                })?;
                let none = sum.constructor_paths.get("None").ok_or_else(|| {
                    fail(
                        "collection get requires the actual None constructor",
                        location,
                    )
                })?;
                let index = usize::from(carrier.domains.len() == 2);
                let domain = &carrier.domains[index];
                let scalar = carrier.arguments[index]
                    .ty
                    .as_ref()
                    .expect("actual scalar argument");
                self.match_collection_packet(
                    library.program(),
                    ty,
                    value,
                    location,
                    |parser, tag, fields| {
                        if tag != "rustCollectionOption" {
                            return Ok(invalid(
                                library,
                                "collection get lost its actual Option result",
                                location,
                            ));
                        }
                        let (optional, optional_type) = required_field(&fields, "value", location)?;
                        let Type::Option { element, .. } = optional_type else {
                            return Err(fail(
                                "collection Option payload has no actual presence representation",
                                location,
                            ));
                        };
                        let selected = SExpr::new(
                            SNode::Coalesce {
                                left: Box::new(optional.clone()),
                                right: Box::new(invalid(
                                    library,
                                    "present collection Option lost its actual payload",
                                    location,
                                )),
                            },
                            location,
                        );
                        let decoded = parser.decode_collection_scalar(
                            library, selected, element, domain, scalar, location,
                        )?;
                        Ok(SExpr::new(
                            SNode::If {
                                cond: Box::new(
                                    required_field(&fields, "present", location)?.0.clone(),
                                ),
                                then: Box::new(apply(some.clone(), vec![decoded], location)),
                                otherwise: Box::new(name(none.clone(), location)),
                            },
                            location,
                        ))
                    },
                )
            }
            _ => Err(fail(
                "collection decoding requires its actual source result interpretation",
                location,
            )),
        }
    }
}
