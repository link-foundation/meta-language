//! Complete original inference expression dispatcher; host term adaptation only.
use super::{
    Arithmetic, BOOL, Env, FLOAT, FieldOf, IndexOf, Inference, IterableOf, Result, SExpr, SNode,
    STRING, Term, UNIT, UnaryOp,
};
impl Inference {
    pub(super) fn expr_inner(&mut self, expr: &SExpr, env: &Env) -> Result<Term> {
        match &expr.node {
            // The 1 of `x++` has the type of `x`, a Number or a BigInt.
            SNode::Num { unit: true, .. } => Ok(self.fresh(false)),
            SNode::Num { ty, .. } => Ok(match ty {
                Some(ty) => self.declared(Some(ty)),
                None => self.fresh(true),
            }),
            SNode::ArrayBindingRest { .. } | SNode::ArrayBindingDefault { .. } => {
                self.array_binding_expr(expr, env, None)
            }
            SNode::Bool { .. } => Ok(Term::Known(BOOL)),
            SNode::Str { .. } => Ok(Term::Known(STRING)),
            SNode::Unit => Ok(Term::Known(UNIT)),
            SNode::None { .. }
            | SNode::Some { .. }
            | SNode::FlattenOption { .. }
            | SNode::Coalesce { .. }
            | SNode::OptionalField { .. } => self.optional_expr(expr, env, None),
            SNode::Name { path } => {
                if let [name] = path.as_slice()
                    && let Some(term) = env.get(name)
                {
                    return Ok(term.clone());
                }
                self.call(path, &[], env, expr.span)
            }
            SNode::App { func, args } => self.callable_application(func, args, env, expr.span),
            SNode::Record { entries, .. } => self.record_expr(expr, entries, env, None),
            SNode::FunctionValue { .. }
            | SNode::Lambda { .. }
            | SNode::CellAllocate { .. }
            | SNode::CellRead { .. }
            | SNode::CellWrite { .. } => self.callable_expr(expr, env, None),
            SNode::Field { object, field } => {
                let object = self.expr(object, env)?;
                let result = self.fresh(false);
                self.fields.push(FieldOf {
                    object,
                    field: field.clone(),
                    result: result.clone(),
                    place: expr.span,
                });
                Ok(result)
            }
            SNode::Unary { op, arg } => {
                let arg = self.expr(arg, env)?;
                if *op == UnaryOp::Not {
                    self.product_condition(&arg, expr.span);
                    return Ok(Term::Known(BOOL));
                }
                let result = self.fresh(false);
                self.arithmetic.push(Arithmetic {
                    left: arg,
                    operation: String::new(),
                    right: None,
                    result: result.clone(),
                    place: expr.span,
                });
                Ok(result)
            }
            SNode::Binary {
                op, left, right, ..
            } => self.binary(*op, left, right, expr.span, env),
            SNode::If { .. } => self.infer_conditional_expression(expr, env),
            SNode::Let { .. } => self.infer_binding_expression(expr, env),
            SNode::Match { .. } => self.infer_match_expression(expr, env),
            SNode::StringMap { object, .. } => {
                let value = self.expr(object, env)?;
                self.unify(&value, &Term::Known(STRING), object.span)?;
                Ok(Term::Known(STRING))
            }
            SNode::CopySlice { object } => self.method(None, object, "slice", &[], env, expr.span),
            SNode::ArrayFrom { args, .. } => self.array_from(expr, args, env, None),
            SNode::Set { items, .. } | SNode::Map { entries: items, .. } => {
                self.collection_construction(expr, items, env, None)
            }
            SNode::Method { object, op, args } => self.method(
                Some(std::ptr::from_ref(expr)),
                object,
                op,
                args,
                env,
                expr.span,
            ),
            SNode::StringTest { op, object, search } if op == "includes" => {
                self.includes(expr, object, search, env)
            }
            SNode::StringTest { object, search, .. } => {
                let object_type = self.expr(object, env)?;
                self.unify(&object_type, &Term::Known(STRING), expr.span)?;
                let search_type = self.expr(search, env)?;
                self.unify(&search_type, &Term::Known(STRING), expr.span)?;
                Ok(Term::Known(BOOL))
            }
            SNode::ArrayIsArray { arg } => {
                self.expr(arg, env)?;
                Ok(Term::Known(BOOL))
            }
            SNode::TypeOf { arg } | SNode::ToString { arg } | SNode::Show { arg, .. } => {
                self.expr(arg, env)?;
                Ok(Term::Known(STRING))
            }
            SNode::CtorObject {
                tag,
                fields,
                prototype_definition,
                ..
            } => {
                if let Some(definition) = prototype_definition {
                    self.expr(&definition.value, env)?;
                }
                self.ctor_object(tag, fields, expr.span, env)
            }
            SNode::Print { expr, body, .. } => {
                self.expr(expr, env)?;
                self.expr(body, env)
            }
            SNode::Array { items, .. } => {
                let element = self.fresh(false);
                self.empty_arrays.insert(
                    std::ptr::from_ref::<SExpr>(expr),
                    (element.clone(), expr.span),
                );
                for item in items {
                    let value = self.expr(&item.value, env)?;
                    let want = if item.spread {
                        Term::Array(Box::new(element.clone()))
                    } else {
                        element.clone()
                    };
                    self.unify(&value, &want, item.value.span.or(expr.span))?;
                    // Match the existing expected-array optional owner update.
                    let contained = if item.spread {
                        match self.resolve(&value) {
                            Term::Array(element) => Some(*element),
                            _ => None,
                        }
                    } else {
                        Some(self.resolve(&value))
                    };
                    if let Some(contained) = contained
                        && matches!(self.resolve(&contained), Term::Option { .. })
                        && let Term::Var(variable) = &element
                    {
                        self.bindings[*variable] = Some(self.resolve(&contained));
                    }
                }
                Ok(Term::Array(Box::new(element)))
            }
            SNode::Tuple { items, .. } => Ok(Term::Tuple(
                items
                    .iter()
                    .map(|item| self.expr(item, env))
                    .collect::<Result<Vec<_>>>()?,
            )),
            SNode::Math { op, args, .. } => {
                if op==&crate::translation::javascript_optional_chain_rules::read_java_script_optional_chain_operation(){return self.optional_chain_term(args,env,expr.span);}
                if op==&crate::translation::javascript_optional_call_rules::read_java_script_optional_call_operation(){return self.optional_call_term(args,env,expr.span);}
                if op=="convertJavaScriptPrimitiveText"||op=="convertJavaScriptBooleanValue"||op=="testJavaScriptArrayValue"||op=="testJavaScriptOwnedProperty"||crate::translation::javascript_comparison_rules::accept_java_script_same_value_operation(op){return self.primitive_observation_term(op,args,env);}
                if crate::translation::javascript_comparison_rules::accept_java_script_loose_equality_operation(op){return self.loose_comparison_term(args,env,expr.span);}
                if op=="constructJavaScriptStringFromCharacterCodes"||op=="constructJavaScriptStringFromCodePoints"||crate::translation::string_construction_rules::accept_linked_java_script_string_construction("math",op){
                    for item in args{self.expr(&item.value,env)?;}
                    return Ok(if crate::translation::string_construction_rules::read_java_script_string_construction_form(op)=="points"{Term::Array(Box::new(Term::Known(FLOAT)))}else{Term::Known(STRING)});
                }
                if op == "createJavaScriptStringFromUnits" {
                    for item in args {
                        self.expr(&item.value, env)?;
                    }
                    return Ok(Term::Known(STRING));
                }
                if op=="convertJavaScriptBigInteger"||crate::translation::big_integer_conversion_rules::accept_linked_java_script_big_integer_result("math",op) {
                    return self.numeric_intrinsic_term(op,args,env,expr.span);
                }
                if crate::translation::source_exception_rules::accept_linked_java_script_source_exception("math", op)||!crate::translation::source_exception_rules::read_java_script_exception_constructor_class(op).is_empty() {
                    for item in args { self.expr(&item.value, env)?; }
                    return Ok(self.fresh(false));
                }
                let conversion = crate::translation::string_number_rules::read_java_script_number_conversion_result(op);
                if !conversion.is_empty() {
                    for item in args {
                        self.expr(&item.value, env)?;
                    }
                    return Ok(Term::Known(if conversion == "bool" { BOOL } else { FLOAT }));
                }
                if (crate::translation::number_bitwise_rules::read_java_script_number_bitwise_arity(
                    op,
                ) > 0.0
                    || crate::translation::big_integer_bitwise_rules::read_big_integer_bitwise_arity(
                        op,
                    ) > 0.0
                    || op == "exponentiateJavaScriptNumericValue")
                    && !args.is_empty()
                {
                    let values = args
                        .iter()
                        .map(|argument| self.expr(&argument.value, env))
                        .collect::<Result<Vec<_>>>()?;
                    let result = self.fresh(false);
                    self.arithmetic.push(Arithmetic {
                        left: values[0].clone(),
                        operation: op.clone(),
                        right: values.get(1).cloned(),
                        result: result.clone(),
                        place: expr.span,
                    });
                    return Ok(result);
                }
                // Math takes Numbers: an argument of no known type is one, and the checker refuses any other.
                for item in args {
                    let ty = self.expr(&item.value, env)?;
                    let ty = self.resolve(&ty);
                    let unknown = match &ty {
                        Term::Var(_) => true,
                        Term::Array(element) => {
                            item.spread && matches!(self.resolve(element), Term::Var(_))
                        }
                        Term::Known(_)
                        | Term::Tuple(_)
                        | Term::Record(_)
                        | Term::Union(_)
                        | Term::Map { .. }
                        | Term::Set { .. }
                        | Term::Option { .. }
                        | Term::Function { .. }
                        | Term::Cell(_) => false,
                    };
                    let mathematical = crate::translation::string_number_rules::read_java_script_mathematical_conversion_form("JavaScript", op);
                    if unknown && !mathematical.is_empty() && item.spread {
                        let element = self.fresh(false);
                        self.unify(
                            &ty,
                            &Term::Array(Box::new(element)),
                            item.value.span.or(expr.span),
                        )?;
                    }
                    if unknown && mathematical.is_empty() && crate::translation::number_predicate_rules::read_number_predicate_argument_form("JavaScript", op, "", "").is_empty() {
                        let want = if item.spread {
                            Term::Array(Box::new(Term::Known(FLOAT)))
                        } else {
                            Term::Known(FLOAT)
                        };
                        self.unify(&ty, &want, item.value.span.or(expr.span))?;
                    }
                }
                let predicate = ["isInteger", "isSafeInteger", "isFinite", "isNaN"];
                Ok(Term::Known(if predicate.contains(&op.as_str()) {
                    BOOL
                } else {
                    FLOAT
                }))
            }
            SNode::ForOfIterable { object, .. } => {
                let object = self.expr(object, env)?;
                let actual = self.resolve(&object);
                if self.array_from_iterable_is_array(&actual) {
                    self.collections
                        .insert(std::ptr::from_ref(expr), (object.clone(), expr.span));
                    return Ok(object);
                }
                let result = Term::Array(Box::new(self.fresh(false)));
                self.iterables.push(IterableOf {
                    object,
                    mapper: None,
                    result: result.clone(),
                    place: expr.span,
                });
                self.collections
                    .insert(std::ptr::from_ref(expr), (result.clone(), expr.span));
                Ok(result)
            }
            SNode::StringPoints { object } => {
                let value = self.expr(object, env)?;
                self.unify(&value, &Term::Known(STRING), expr.span)?;
                Ok(Term::Array(Box::new(Term::Known(STRING))))
            }
            SNode::ReferenceWrite {
                object,
                index,
                value,
                ..
            } => {
                self.expr(object, env)?;
                if let Some(index) = index {
                    self.expr(index, env)?;
                }
                if let Some(value) = value {
                    self.expr(value, env)?;
                }
                Ok(Term::Known(UNIT))
            }
            SNode::Index {
                object,
                index,
                optional,
                proven_in_bounds,
            } => {
                let element = self.fresh(false);
                let object_type = self.expr(object, env)?;
                self.expr(index, env)?;
                let position = super::products::read_javascript_product_index(index);
                let literal = matches!(index.node, SNode::Num { .. })
                    || matches!(&index.node, SNode::Unary { op: UnaryOp::Neg, arg } if matches!(arg.node, SNode::Num { .. }));
                self.indices.push(IndexOf {
                    object: object_type,
                    property: match &index.node {
                        SNode::Str { value } => Some(value.clone()),
                        _ => None,
                    },
                    index: position,
                    literal,
                    optional: optional
                        .as_ref()
                        .and_then(super::super::surface::SurfaceRawMetadata::as_boolean)
                        == Some(true),
                    proven_in_bounds: proven_in_bounds
                        .as_ref()
                        .and_then(super::super::surface::SurfaceRawMetadata::as_boolean)
                        == Some(true),
                    result: element.clone(),
                    place: object.span.or(expr.span),
                });
                Ok(element)
            }
            SNode::TupleIndex { object, index } => {
                let element = self.fresh(false);
                let object_type = self.expr(object, env)?;
                #[allow(clippy::cast_precision_loss)]
                let position = *index as f64;
                self.indices.push(IndexOf {
                    object: object_type,
                    property: None,
                    index: position,
                    literal: true,
                    optional: false,
                    proven_in_bounds: true,
                    result: element.clone(),
                    place: object.span.or(expr.span),
                });
                Ok(element)
            }
            SNode::Length { object, integer } => {
                let array = self.expr(object, env)?;
                if matches!(self.resolve(&array), Term::Tuple(_)) {
                    return Ok(if *integer {
                        self.fresh(true)
                    } else {
                        Term::Known(FLOAT)
                    });
                }
                let element = self.fresh(false);
                self.unify(&array, &Term::Array(Box::new(element)), expr.span)?;
                Ok(if *integer {
                    self.fresh(true)
                } else {
                    Term::Known(FLOAT)
                })
            }
            _ => Ok(self.fresh(false)),
        }
    }
}
