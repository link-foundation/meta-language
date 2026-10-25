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
        if op==crate::translation::javascript_record_parameter_rules::read_java_script_record_formal_default_operation().to_string(){
            return self.record_formal_default(args,span,env,path);
        }
        if op==crate::translation::array_formal_parameter_rules::read_java_script_array_formal_iterable_operation().to_string(){return self.array_formal_iterable(args,span,env,path);}
        if crate::translation::javascript_enumeration_rules::read_java_script_record_order_kernel_arity(op)>0.0{return self.record_order_operation(op,args,env,path,span);}
        if op == "testJavaScriptOwnedProperty" {
            return self.javascript_own_property(args, span, env, path);
        }
        if op==crate::translation::javascript_optional_chain_rules::read_java_script_optional_chain_operation(){return self.javascript_optional_chain(args,span,env,path);}
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
