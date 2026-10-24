//! Checked node construction from the complete generated primitive method rules.
use super::{
    BOOL, Checker, Env, Expr, FLOAT, LitValue, Node, Result, SExpr, STRING, Span, Type, array,
    unsupported,
};
use crate::translation::frontend_rules::ml_array::number_index;
use crate::translation::primitive_method_rules as rules;

impl Checker {
    pub(super) fn copy_slice(
        &mut self,
        subject: (&SExpr, Option<&Type>),
        env: &Env,
        path: &[String],
        span: Option<Span>,
    ) -> Result<Expr> {
        let (object, expected) = subject;
        let value = self.expr(object, env, path, None, false)?;
        if !self.checked_projection
            && let Some(result) = self.javascript_nullish_access(
                (&value, object),
                super::javascript_nullish_fields::JavaScriptPropertyAccess::Method("slice", &[]),
                env,
                path,
                expected,
                span,
            )?
        {
            return Ok(result);
        }
        if crate::translation::javascript_nullish_field_rules::require_java_script_slice_reference_copy(self.language.as_str(),value.ty.kind(),value.ty.is_reference()){
            return self.primitive_method_on_checked_receiver(Self::reference_value(value),"slice",&[],env,path,span);
        }
        match rules::read_primitive_copy_slice_form(value.ty.kind()).as_str() {
            "identity" => Ok(value),
            "method" => {
                let value = Self::reference_value(value);
                self.checked_record_method_application(value, "slice", &[], env, path, span)
            }
            _ => Err(unsupported(
                "method call .slice()",
                "this value has no string or array slice method",
                span,
            )),
        }
    }

    pub(super) fn primitive_method(
        &mut self,
        object: &SExpr,
        operation: &str,
        args: &[Option<SExpr>],
        env: &Env,
        path: &[String],
        span: Option<Span>,
    ) -> Result<Expr> {
        self.primitive_method_with_context((object, None), operation, args, env, path, span)
    }

    pub(super) fn primitive_method_with_context(
        &mut self,
        subject: (&SExpr, Option<&Type>),
        operation: &str,
        args: &[Option<SExpr>],
        env: &Env,
        path: &[String],
        span: Option<Span>,
    ) -> Result<Expr> {
        let (source, expected) = subject;
        let object = self.expr(source, env, path, None, false)?;
        if !self.checked_projection
            && let Some(value) = self.javascript_nullish_access(
                (&object, source),
                super::javascript_nullish_fields::JavaScriptPropertyAccess::Method(operation, args),
                env,
                path,
                expected,
                span,
            )?
        {
            return Ok(value);
        }
        if crate::translation::array_state_method_rules::accept_linked_array_state_method(
            "method",
            operation,
            object.ty.kind(),
            object.ty.is_reference(),
        ) {
            return self.check_array_state_method(object, operation, args, env, path, span);
        }
        if !crate::translation::record_write_rules::read_record_field_write_operator(operation)
            .is_empty()
        {
            return self.check_record_field_write(object, operation, args, env, path, span);
        }
        if object.ty.is_reference() {
            let refusal = crate::translation::reference_method_rules::read_reference_method_refusal(
                object.ty.kind(),
                operation,
            );
            if !refusal.is_empty() {
                return Err(unsupported("mutable collection iteration", &refusal, span));
            }
        }
        if object.ty.is_reference()
            && !crate::translation::reference_method_rules::read_reference_method_result(
                object.ty.kind(),
                operation,
            )
            .is_empty()
        {
            return self.check_reference_method(object, operation, args, env, path, span);
        }
        let object = Self::reference_value(object);
        self.primitive_method_on_checked_receiver(object, operation, args, env, path, span)
    }

    pub(super) fn primitive_method_on_checked_receiver(
        &mut self,
        object: Expr,
        operation: &str,
        args: &[Option<SExpr>],
        env: &Env,
        path: &[String],
        span: Option<Span>,
    ) -> Result<Expr> {
        if operation == "readJavaScriptRecordPropertyOrder" {
            return Self::ordered_record_order_read(object, operation, args.len(), span);
        }
        if matches!(object.ty, Type::Record { .. }) {
            return self
                .checked_record_method_application(object, operation, args, env, path, span);
        }
        if matches!(object.ty, Type::Map { .. } | Type::Set { .. }) {
            return self.checked_collection_access(object, operation, args, env, path, span);
        }
        if matches!(object.ty, Type::Array { .. }) && operation == "map" {
            return self.array_map_on_checked_receiver(object, args, env, path, span);
        }
        let receiver = object.ty.kind();
        let minimum = rules::read_primitive_method_minimum_arguments(receiver, operation);
        let maximum = rules::read_primitive_method_maximum_arguments(receiver, operation);
        if minimum < 0.0 {
            let reason = if receiver == "string" {
                format!(".{operation}() has no checked string operation")
            } else {
                format!(
                    ".{operation}() has no checked operation on {}",
                    object.ty.key()
                )
            };
            return Err(unsupported(
                &format!("method call .{operation}()"),
                &reason,
                span,
            ));
        }
        let minimum = number_index(minimum)
            .expect("the generated supported method has an integral minimum arity");
        let maximum = number_index(maximum)
            .expect("the generated supported method has an integral maximum arity");
        if args.len() < minimum || args.len() > maximum {
            let count = if minimum == maximum {
                minimum.to_string()
            } else {
                format!("{minimum} to {maximum}")
            };
            return Err(unsupported(
                &format!("method call .{operation}()"),
                &format!(".{operation}() expects {count} arguments"),
                span,
            ));
        }
        let element = if let Type::Array { element, .. } = &object.ty {
            Some(element.as_ref())
        } else {
            None
        };
        let base = element.map(|element| {
            if let Type::Option { element, .. } = element {
                element.as_ref()
            } else {
                element
            }
        });
        let obligation = rules::read_primitive_method_argument_obligation(
            receiver,
            operation,
            base.map_or("", Type::kind),
        );
        if !obligation.is_empty() {
            return Err(unsupported(
                &rules::read_primitive_method_argument_obligation_feature(operation),
                &obligation,
                span,
            ));
        }
        let count = number_index(rules::read_primitive_method_argument_count(
            receiver, operation,
        ))
        .expect("the generated supported method has an integral bounded arity");
        let mut values = Vec::with_capacity(count);
        for index in 0..count {
            #[allow(clippy::cast_precision_loss)]
            let position = index as f64;
            let kind =
                rules::read_primitive_method_checked_argument_kind(receiver, operation, position);
            let ty = match kind.as_str() {
                "float" | "number-value" => FLOAT,
                "element" => element
                    .expect("the generated array element position")
                    .clone(),
                _ => STRING,
            };
            if let Some(Some(argument)) = args.get(index) {
                let value = self.expr(argument, env, path, Some(&ty), false)?;
                let value = if kind == "number-value" {
                    self.number_value(value, argument.span.or(span))?
                } else {
                    value
                };
                values.push(Some(self.coerce(value, &ty, argument.span.or(span))?));
            } else {
                let form = rules::read_primitive_method_default_form(receiver, operation, position);
                if form == "absent" || form == "required" {
                    values.push(None);
                } else {
                    let ty = if form == "float" { FLOAT } else { STRING };
                    values.push(Some(Expr::new(
                        Node::Lit {
                            value: LitValue::Text(rules::read_primitive_method_default_value(
                                receiver, operation, position,
                            )),
                        },
                        ty,
                    )));
                }
            }
        }
        self.materialize_primitive_method(object, operation, values, path, span)
    }

    pub(super) fn materialize_primitive_method(
        &mut self,
        object: Expr,
        operation: &str,
        values: Vec<Option<Expr>>,
        path: &[String],
        span: Option<Span>,
    ) -> Result<Expr> {
        let receiver = object.ty.kind();
        let lowering = rules::read_primitive_method_lowering_form(receiver, operation);
        if lowering == "relative-element-template" {
            let Type::Array { element, .. } = &object.ty else {
                unreachable!("the generated relative element lowering has an array receiver");
            };
            let result = Type::Option {
                element: element.clone(),
                absence: "undefined".to_owned(),
            };
            let signature = Type::Function {
                params: vec![object.ty.clone(), FLOAT],
                ret: Box::new(result.clone()),
            };
            let template = super::array_relative_element_template::expression(&object.ty, span)?;
            let function = self.expr(&template, &Env::default(), path, Some(&signature), false)?;
            let function = self.coerce(function, &signature, span)?;
            let arguments = std::iter::once(object)
                .chain(values.into_iter().map(|value| {
                    value.expect(
                        "the generated relative element lowering supplies its numeric argument",
                    )
                }))
                .collect();
            return Ok(super::optional_values::flatten_option(Expr::new(
                Node::Apply {
                    func: Box::new(function),
                    args: arguments,
                },
                result,
            )));
        }
        if lowering == "relative-character-template" {
            let result = Type::Option {
                element: Box::new(STRING),
                absence: "undefined".to_owned(),
            };
            let signature = Type::Function {
                params: vec![STRING, FLOAT],
                ret: Box::new(result.clone()),
            };
            let template = super::string_code_unit_template::relative_character_expression(span)?;
            let function = self.expr(&template, &Env::default(), path, Some(&signature), false)?;
            let function = self.coerce(function, &signature, span)?;
            let arguments = std::iter::once(object)
                .chain(values.into_iter().map(|value| {
                    value.expect(
                        "the generated relative character lowering supplies its numeric argument",
                    )
                }))
                .collect();
            return Ok(Expr::new(
                Node::Apply {
                    func: Box::new(function),
                    args: arguments,
                },
                result,
            ));
        }
        if lowering == "code-unit-template" || lowering == "character-template" {
            let character = lowering == "character-template";
            let result = if character { STRING } else { FLOAT };
            let template = if character {
                super::string_code_unit_template::character_expression(span)?
            } else {
                super::string_code_unit_template::expression(span)?
            };
            let signature = Type::Function {
                params: vec![STRING, FLOAT],
                ret: Box::new(result.clone()),
            };
            let function = self.expr(&template, &Env::default(), path, Some(&signature), false)?;
            let function = self.coerce(function, &signature, span)?;
            let arguments = std::iter::once(object)
                .chain(values.into_iter().map(|value| {
                    value.expect("the generated code-unit lowering supplies its numeric argument")
                }))
                .collect();
            return Ok(Expr::new(
                Node::Apply {
                    func: Box::new(function),
                    args: arguments,
                },
                result,
            ));
        }
        let ty =
            match rules::read_primitive_method_checked_result_form(receiver, operation).as_str() {
                "receiver" => object.ty.clone(),
                "bool" => BOOL,
                "float" => FLOAT,
                "array-string" => array(STRING),
                "optional-float" => Type::Option {
                    element: Box::new(FLOAT),
                    absence: "undefined".to_owned(),
                },
                _ => STRING,
            };
        Ok(Expr::new(
            Node::Method {
                object: Box::new(object),
                op: operation.to_owned(),
                args: values,
            },
            ty,
        ))
    }
}
