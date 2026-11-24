//! `Math` and `Number` functions, whose arguments are Numbers.

use super::{
    BOOL, Checker, Env, Expr, FLOAT, LitValue, Node, Result, SArrayItem, Span, Type, array,
    type_error, unsupported,
};

/// The functions whose value is a boolean.
const PREDICATES: [&str; 4] = ["isInteger", "isSafeInteger", "isFinite", "isNaN"];

impl Checker {
    /// `Math.floor(x)`, `Math.max(a, ...xs)`, `Number.isInteger(x)`: the
    /// arguments are Numbers, which none of them converts, and a spread
    /// argument makes `max` and `min` a fold over one array.
    pub(super) fn math(
        &mut self,
        op: &str,
        name: &str,
        args: &[SArrayItem],
        span: Option<Span>,
        env: &Env,
        path: &[String],
    ) -> Result<Expr> {
        use crate::translation::javascript_nominal_spread_rules::{
            accept_java_script_projected_typed_absence,
            read_java_script_guarded_optional_projection_operation,
        };
        if self.checked_projection
            && self.language == super::Language::JavaScript
            && op == read_java_script_guarded_optional_projection_operation()
        {
            let diagnostic = || {
                type_error(
                    "a checked optional extraction needs its actual dominating absence guards",
                    span,
                )
            };
            let [argument] = args else {
                return Err(diagnostic());
            };
            if argument.spread {
                return Err(diagnostic());
            }
            let value = self.expr(&argument.value, env, path, None, false)?;
            let original_type = value.ty.clone();
            let value = self.guarded_nominal_optional(&argument.value, value, env);
            if value.ty == original_type {
                return Err(diagnostic());
            }
            return Ok(value);
        }
        if self.checked_projection && matches!(op, "strictEqual:false" | "strictEqual:true") {
            let [left, right] = args else {
                return Err(type_error(
                    "checked strict equality requires two actual operands",
                    span,
                ));
            };
            if left.spread || right.spread {
                return Err(type_error(
                    "checked strict equality requires two actual operands",
                    span,
                ));
            }
            let mut left = self.expr(&left.value, env, path, None, false)?;
            let mut right = self.expr(&right.value, env, path, None, false)?;
            if self.language == super::Language::JavaScript && matches!(name, "left" | "right") {
                let (literal, peer) = if name == "left" {
                    (&mut left, &right)
                } else {
                    (&mut right, &left)
                };
                let diagnostic = || {
                    type_error(
                        "checked strict equality needs an absence in its actual optional operand domain",
                        span,
                    )
                };
                let (Node::None { absence }, Type::Option { absence: policy, .. }) =
                    (&literal.node, &peer.ty)
                else {
                    return Err(diagnostic());
                };
                if matches!(&peer.node, Node::None { .. })
                    || !accept_java_script_projected_typed_absence(
                        "none",
                        peer.ty.kind(),
                        absence,
                        policy,
                        true,
                    )
                {
                    return Err(diagnostic());
                }
                literal.ty = peer.ty.clone();
            }
            use crate::translation::optional_value_rules::{
                accept_optional_primitive_kind, read_optional_comparison_operation,
            };
            let element = |ty: &Type| match ty {
                Type::Option { element, .. } => element.as_ref().clone(),
                _ => ty.clone(),
            };
            let a = element(&left.ty);
            let b = element(&right.ty);
            let primitive = accept_optional_primitive_kind(a.kind());
            let mode = read_optional_comparison_operation(
                self.language.as_str(),
                left.ty.kind(),
                right.ty.kind(),
                "eq",
                a.same(&b),
                primitive,
            );
            if !(primitive && accept_optional_primitive_kind(b.kind()))
                && mode != "strict"
                && !matches!(left.node, Node::None { .. })
                && !matches!(right.node, Node::None { .. })
            {
                return Err(type_error(
                    "checked strict equality requires portable primitive operands or an actual absence",
                    span,
                ));
            }
            return Ok(Expr::new(
                Node::StrictEqual {
                    left: Box::new(left),
                    right: Box::new(right),
                    negate: op == "strictEqual:true",
                },
                BOOL,
            ));
        }
        if crate::translation::rust_string_byte_length_rules::accept_rust_string_byte_length_operation("math",op) {
            let value = if args.len()==1 && !args[0].spread {Some(self.expr(&args[0].value,env,path,Some(&super::STRING),false)?)} else {None};
            let error = crate::translation::rust_string_byte_length_rules::read_rust_string_byte_length_argument_error(self.language.as_str(),false,serde_json::Number::from(args.len()).as_f64().expect("actual arity"),args.iter().any(|arg|arg.spread),value.as_ref().map_or("",|value|value.ty.kind()));
            if !error.is_empty() {return Err(type_error(error,span));}
            return Ok(Expr::new(Node::Math {op:op.to_owned(),args:vec![value.expect("actual String operand")]},Type::Fixed {bits:64,signed:false}));
        }
        if op==crate::translation::recursive_binding_rules::read_java_script_recursive_binding_iterable_operation().to_string(){return self.recursive_binding_iterable(args,span,env,path);}
        if op
            == crate::translation::object_rest_rules::read_java_script_object_rest_operation()
                .to_string()
        {
            return self.object_rest(args, span, env, path);
        }
        if op==crate::translation::javascript_record_parameter_rules::read_java_script_record_formal_default_operation().to_string(){
            return self.record_formal_default(args,span,env,path);
        }
        if op==crate::translation::array_formal_parameter_rules::read_java_script_array_formal_iterable_operation().to_string(){return self.array_formal_iterable(args,span,env,path);}
        if crate::translation::javascript_enumeration_rules::read_java_script_record_order_kernel_arity(op)>0.0{return self.record_order_operation(op,args,env,path,span);}
        if op == "testJavaScriptOwnedProperty" {
            return self.javascript_own_property(args, span, env, path);
        }
        if op==crate::translation::javascript_optional_chain_rules::read_java_script_optional_chain_operation(){return self.javascript_optional_chain(name,args,span,env,path);}
        if op==crate::translation::javascript_optional_call_rules::read_java_script_optional_call_operation(){return self.javascript_optional_call(args,span,env,path);}
        if op == "testJavaScriptArrayValue" {
            return self.javascript_array_builtin(args, span, env, path);
        }
        if op == "convertJavaScriptBooleanValue" {
            return self.javascript_boolean_builtin(args, span, env, path);
        }
        if op == "convertJavaScriptPrimitiveText" {
            return self.javascript_primitive_text_builtin(args, span, env, path);
        }
        if crate::translation::javascript_comparison_rules::accept_java_script_same_value_operation(
            op,
        ) {
            return self.javascript_same_value(args, span, env, path);
        }
        if crate::translation::javascript_comparison_rules::accept_java_script_loose_equality_operation(op){return self.javascript_loose_comparison(op,args,span,env,path);}
        if op == "constructJavaScriptStringFromCharacterCodes"
            || op == "constructJavaScriptStringFromCodePoints"
        {
            return self.javascript_string_construction(op, args, span, env, path);
        }
        if crate::translation::string_construction_rules::accept_linked_java_script_string_construction("math",op){return self.javascript_string_construction_value(op,args,span,env,path);}
        if crate::translation::javascript_string_unit_rules::accept_linked_java_script_string_unit_construction("math",op){
            if self.language!=super::Language::JavaScript||args.len()!=1||args[0].spread{return Err(type_error("a JavaScript UTF16 literal needs one owned unit column",span));}
            let units=self.expr(&args[0].value,env,path,Some(&array(FLOAT)),false)?;
            let Node::Array{items}=&units.node else{return Err(type_error("a JavaScript UTF16 literal needs explicit checked integral code units",span))};
            let values=items.iter().map(|item|match &item.node{Node::Lit{value:LitValue::Text(value)} if item.ty==FLOAT=>value.parse::<f64>().ok(),_=>None}).collect::<Option<Vec<_>>>();
            if !values.as_ref().is_some_and(|values|crate::translation::javascript_string_unit_rules::accept_java_script_string_units(values)){return Err(type_error("a JavaScript UTF16 literal needs explicit checked integral code units",span));}
            return Ok(Expr::new(Node::Math{op:op.to_owned(),args:vec![units]},Type::String));
        }
        if op == "convertJavaScriptBigInteger" {
            return self.javascript_big_integer_builtin(args, span, env, path);
        }
        if crate::translation::big_integer_conversion_rules::accept_linked_java_script_big_integer_result("math",op){return self.javascript_big_integer_result(op,args,span,env,path);}
        if !crate::translation::string_number_rules::read_java_script_number_conversion_result(op)
            .is_empty()
        {
            return self.javascript_number_builtin(op, args, span, env, path);
        }
        let ty = if PREDICATES.contains(&op) {
            BOOL
        } else {
            FLOAT
        };
        let variadic = op == "max" || op == "min";
        if variadic && args.iter().any(|item| item.spread) {
            let values = self.array_literal(args, None, span, env, path, Some(&array(FLOAT)))?;
            return Ok(Expr::new(
                Node::Math {
                    op: format!("{op}Of"),
                    args: vec![values],
                },
                ty,
            ));
        }
        if let Some(spread) = args.iter().find(|item| item.spread) {
            return Err(unsupported(
                "spread argument",
                &format!("pass {name} its argument"),
                spread.value.span.or(span),
            ));
        }
        if !crate::translation::number_predicate_rules::read_number_predicate_argument_form(
            self.language.as_str(),
            op,
            "",
            "",
        )
        .is_empty()
        {
            return self.number_predicate(op, name, args, span, env, path);
        }
        let mathematical =
            crate::translation::string_number_rules::read_java_script_mathematical_conversion_form(
                self.language.as_str(),
                op,
            );
        if !mathematical.is_empty() {
            return self.javascript_mathematical_arguments(
                op,
                args,
                span,
                env,
                path,
                &mathematical,
            );
        }
        if !variadic && args.len() != 1 {
            return Err(unsupported(
                &format!("{name} with {} arguments", args.len()),
                &format!("{name} takes one Number"),
                span,
            ));
        }
        let args = args
            .iter()
            .map(|item| {
                let mut arg = self.expr(&item.value, env, path, Some(&FLOAT), false)?;
                if matches!(arg.ty, Type::Option { .. }) {
                    if PREDICATES.contains(&op) {
                        return Ok(Expr::new(
                            Node::OptionalPredicate {
                                arg: Box::new(arg),
                                op: op.to_owned(),
                            },
                            BOOL,
                        ));
                    }
                    arg = self.number_value(arg, item.value.span.or(span))?;
                }
                if !arg.ty.is_float() {
                    return Err(type_error(
                        format!(
                            "{name} of {}; it takes Numbers, and converts or rejects anything else",
                            arg.ty.key()
                        ),
                        item.value.span.or(span),
                    ));
                }
                Ok(arg)
            })
            .collect::<Result<Vec<_>>>()?;
        if PREDICATES.contains(&op)
            && let Some(value) = args.first()
            && matches!(value.node, Node::OptionalPredicate { .. })
        {
            return Ok(value.clone());
        }
        Ok(Expr::new(
            Node::Math {
                op: op.to_owned(),
                args,
            },
            ty,
        ))
    }

    fn number_predicate(
        &mut self,
        op: &str,
        name: &str,
        arguments: &[SArrayItem],
        span: Option<Span>,
        env: &Env,
        path: &[String],
    ) -> Result<Expr> {
        let args = arguments
            .iter()
            .map(|item| self.expr(&item.value, env, path, None, false))
            .collect::<Result<Vec<_>>>()?;
        let Some(first) = args.first() else {
            return Ok(Expr::lit(BOOL, LitValue::Bool(false)));
        };
        let element = if let Type::Option { element, .. } = &first.ty {
            element.kind()
        } else {
            ""
        };
        let form = crate::translation::number_predicate_rules::read_number_predicate_argument_form(
            self.language.as_str(),
            op,
            first.ty.kind(),
            element,
        );
        if form == "unsupported" {
            return Err(type_error(
                format!(
                    "{name} of {}; it takes Numbers, and converts or rejects anything else",
                    first.ty.key()
                ),
                arguments[0].value.span.or(span),
            ));
        }
        if args.len() == 1 && form == "number" {
            return Ok(Expr::new(
                Node::Math {
                    op: op.to_owned(),
                    args,
                },
                BOOL,
            ));
        }
        if args.len() == 1 && form == "optional-number" {
            return Ok(Expr::new(
                Node::OptionalPredicate {
                    op: op.to_owned(),
                    arg: Box::new(first.clone()),
                },
                BOOL,
            ));
        }
        let binding = self.fresh_name("ml_number_predicate");
        let bound = Expr::var(binding.clone(), first.ty.clone());
        let mut body = match form.as_str() {
            "number" => Expr::new(
                Node::Math {
                    op: op.to_owned(),
                    args: vec![bound],
                },
                BOOL,
            ),
            "optional-number" => Expr::new(
                Node::OptionalPredicate {
                    op: op.to_owned(),
                    arg: Box::new(bound),
                },
                BOOL,
            ),
            _ => Expr::lit(BOOL, LitValue::Bool(false)),
        };
        for (index, value) in args.into_iter().enumerate().rev() {
            let name = if index == 0 {
                binding.clone()
            } else {
                self.fresh_name("ml_number_predicate_unused")
            };
            body = Expr::new(
                Node::Let {
                    name,
                    value: Box::new(value),
                    body: Box::new(body),
                },
                BOOL,
            );
        }
        Ok(body)
    }
}
