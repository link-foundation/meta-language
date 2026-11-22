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
        self.check_reserved_place_operand_effects(left, right)?;
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
}

include!("packet_operations.rs");
