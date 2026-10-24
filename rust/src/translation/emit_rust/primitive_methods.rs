//! Host rendering of primitive operands through the shared generated functions.
use super::{Expr, Node, Prelude, Result, RustEmitter, Type, unsupported};
use crate::translation::primitive_method_rules as rules;

pub(super) fn program_requires_method_units(program: &crate::translation::ir::Program) -> bool {
    fn walk(value: &serde_json::Value) -> bool {
        match value {
            serde_json::Value::Array(values) => values.iter().any(walk),
            serde_json::Value::Object(fields) => {
                let expression = fields
                    .get("k")
                    .and_then(serde_json::Value::as_str)
                    .unwrap_or("");
                let receiver = fields
                    .get("object")
                    .and_then(|object| object.get("type"))
                    .and_then(|ty| ty.get("kind"))
                    .and_then(serde_json::Value::as_str)
                    .unwrap_or("");
                (crate::translation::string_construction_rules::accept_linked_java_script_string_construction(expression,fields.get("op").and_then(serde_json::Value::as_str).unwrap_or(""))&&crate::translation::string_construction_rules::read_java_script_string_construction_form(fields.get("op").and_then(serde_json::Value::as_str).unwrap_or(""))=="characters")
                    || crate::translation::javascript_string_unit_rules::accept_linked_java_script_string_unit_construction(expression,fields.get("op").and_then(serde_json::Value::as_str).unwrap_or(""))
                    || rules::require_primitive_method_utf16(expression, receiver)
                    || (crate::translation::big_integer_conversion_rules::accept_linked_java_script_big_integer_result(expression,fields.get("op").and_then(serde_json::Value::as_str).unwrap_or(""))&&matches!(crate::translation::big_integer_conversion_rules::read_java_script_big_integer_result_operation(fields.get("op").and_then(serde_json::Value::as_str).unwrap_or("")).as_str(),"string"|"string-compare"))
                    || crate::translation::string_number_rules::require_java_script_number_conversion_units(
                        expression,
                        fields.get("op").and_then(serde_json::Value::as_str).unwrap_or(""),
                        fields.get("args").and_then(serde_json::Value::as_array).and_then(|args| args.first()).and_then(|argument| argument.get("type")).and_then(|ty| ty.get("kind")).and_then(serde_json::Value::as_str).unwrap_or(""),
                    )
                    || crate::translation::string_order_rules::require_string_order_units(
                        expression,
                        fields
                            .get("op")
                            .and_then(serde_json::Value::as_str)
                            .unwrap_or(""),
                    )
                    || fields
                        .iter()
                        .any(|(key, value)| key != "type" && key != "span" && walk(value))
            }
            _ => false,
        }
    }
    walk(
        &serde_json::to_value(program).expect("checked programs retain their serde representation"),
    )
}

impl RustEmitter<'_> {
    fn primitive_array_join_element(&mut self, name: &str, ty: &Type) -> Result<String> {
        let form = rules::read_primitive_array_join_element_form(ty.kind());
        let empty = if matches!(form.as_str(), "empty" | "optional") {
            let value = super::LitValue::Text(String::new());
            self.literal(&Expr::lit(Type::String, value.clone()), &value)
        } else {
            String::new()
        };
        let (present, absence) = if form == "optional" {
            let Type::Option { element, absence } = ty else {
                unreachable!("the generated join element form selects the actual Option");
            };
            if absence == "both" {
                self.preludes.insert(Prelude::Optional);
            }
            (
                self.primitive_array_join_element("ml_present", element)?,
                absence.as_str(),
            )
        } else if form == "empty" {
            (String::new(), "")
        } else {
            let value = Expr::new(
                Node::ToString {
                    arg: Box::new(Expr::var(name, ty.clone())),
                    console: false,
                },
                Type::String,
            );
            (self.expr(&value)?, "")
        };
        Ok(rules::render_primitive_array_join_element(
            ty.kind(),
            absence,
            name,
            &present,
            &empty,
        ))
    }

    pub(super) fn primitive_method(&mut self, expression: &Expr) -> Result<String> {
        let Node::Method { object, op, args } = &expression.node else {
            unreachable!("primitive method dispatch")
        };
        if let Node::ReferenceRead { reference } = &object.node
            && reference.ty.is_reference()
            && !crate::translation::array_callback_rules::read_array_callback_result_form(op)
                .is_empty()
        {
            let safe = args
                .first()
                .and_then(Option::as_ref)
                .is_some_and(|callback| {
                    let Type::Function { params, .. } = &callback.ty else {
                        return false;
                    };
                    crate::translation::reference_method_rules::accept_snapshot_callback_parameters(
                        op,
                        serde_json::Number::from(params.len())
                            .as_f64()
                            .expect("an actual callback parameter count is a Number"),
                    ) && self.read_only_reference_callback(callback)
                });
            if !safe {
                return Err(unsupported(
                    "shared array callback",
                    "this callback needs live array slots, effects and the original receiver identity rather than a snapshot",
                    expression.span,
                ));
            }
        }
        let receiver = object.ty.kind();
        if receiver == "array" && op == "map" {
            self.preludes.insert(Prelude::Array);
            let callback = args
                .first()
                .and_then(Option::as_ref)
                .expect("the checked map has its actual callback");
            let Type::Function { params, .. } = &callback.ty else {
                unreachable!("the map checker retained its actual function type");
            };
            self.temporaries += 1;
            let suffix = self.temporaries.to_string();
            let parameter_count = serde_json::Number::from(params.len())
                .as_f64()
                .expect("a checked parameter count has a Number representation");
            let object = self.expr(object)?;
            let callback = self.expr(callback)?;
            return Ok(
                crate::translation::array_from_rules::render_array_from_mapping(
                    "Rust",
                    &object,
                    &callback,
                    parameter_count,
                    &suffix,
                ),
            );
        }
        if receiver == "array"
            && !crate::translation::array_callback_rules::read_array_callback_result_form(op)
                .is_empty()
        {
            self.preludes.insert(Prelude::Array);
            let callback = args.first().and_then(Option::as_ref).ok_or_else(|| {
                unsupported(
                    &format!(".{op}()"),
                    "the checked callback is absent",
                    expression.span,
                )
            })?;
            let Type::Function { params, ret } = &callback.ty else {
                return Err(unsupported(
                    &format!(".{op}()"),
                    "the checked callback has no function type",
                    expression.span,
                ));
            };
            self.temporaries += 1;
            let suffix = self.temporaries.to_string();
            #[allow(clippy::cast_precision_loss)]
            let parameter_count = params.len() as f64;
            let reference_result = matches!(
                ret.as_ref(),
                Type::Array {
                    reference: true,
                    ..
                }
            );
            let object = self.expr(object)?;
            let callback = self.expr(callback)?;
            let initial_position = crate::translation::array_callback_rules::read_array_callback_checked_argument_position(
                op, args.len() != 1, 0.0,
            );
            let has_initial =
                crate::translation::frontend_rules::ml_array::number_index(initial_position)
                    == Some(1);
            let initial = if has_initial {
                let value = args.get(1).and_then(Option::as_ref).ok_or_else(|| {
                    unsupported(
                        &format!(".{op}()"),
                        "the checked initial value is absent",
                        expression.span,
                    )
                })?;
                self.expr(value)?
            } else {
                String::new()
            };
            let referenced = crate::translation::array_callback_rules::render_array_callback_reference_expression(
                op, &object, &callback, parameter_count, &suffix, reference_result,
            );
            if !referenced.is_empty() {
                return Ok(referenced);
            }
            return Ok(
                crate::translation::array_callback_rules::render_array_callback_expression(
                    op,
                    &object,
                    &callback,
                    parameter_count,
                    &suffix,
                    &initial,
                    has_initial,
                ),
            );
        }
        if let Type::Array { element, .. } = &object.ty
            && op == "join"
        {
            self.preludes.insert(Prelude::Array);
            let string_element = element.kind() == "string";
            let separator = args[0]
                .as_ref()
                .expect("the checked join retains its separator");
            let object_text = if rules::read_primitive_array_join_operand_form(
                self.utf16_strings,
                string_element,
                "object",
            ) == "borrow"
            {
                self.borrow(object)?
            } else {
                self.receiver(object)?
            };
            let separator_text = if rules::read_primitive_array_join_operand_form(
                self.utf16_strings,
                string_element,
                "separator",
            ) == "borrow"
            {
                self.borrow(separator)?
            } else {
                self.receiver(separator)?
            };
            let element_text = if string_element {
                String::new()
            } else {
                self.primitive_array_join_element("ml_value", element)?
            };
            return Ok(rules::render_primitive_array_join_expression(
                self.utf16_strings,
                string_element,
                &object_text,
                &separator_text,
                &element_text,
            ));
        }
        if receiver == "string" {
            let object_text = self.receiver(object)?;
            let mut values = Vec::with_capacity(args.len());
            for (index, argument) in args.iter().enumerate() {
                let text = match argument {
                    Some(argument) if argument.ty == Type::String => self.borrow(argument)?,
                    Some(argument) => self.expr(argument)?,
                    None => "None".to_owned(),
                };
                #[allow(clippy::cast_precision_loss)]
                let position = index as f64;
                values.push(rules::render_primitive_method_argument(
                    "Rust",
                    receiver,
                    op,
                    position,
                    argument.is_none(),
                    &text,
                ));
            }
            if crate::translation::utf16_method_emission_rules::accept_utf16_text_method(
                "method", receiver, op,
            ) {
                self.preludes.insert(Prelude::Utf16Text);
                return Ok(
                    crate::translation::utf16_method_emission_rules::render_utf16_text_method(
                        "Rust",
                        op,
                        &object_text,
                        &values.join(", "),
                    ),
                );
            }
            return Ok(rules::render_primitive_method_expression(
                "Rust",
                receiver,
                op,
                &object_text,
                &(values.join(", ")),
            ));
        }
        if receiver == "array" && op == "slice" {
            self.standard_array = true;
            self.preludes.insert(Prelude::Array);
            let object = self.borrow(object)?;
            let mut values = Vec::with_capacity(args.len());
            for argument in args {
                values.push(
                    self.expr(
                        argument
                            .as_ref()
                            .expect("the checker retained every normalized slice bound"),
                    )?,
                );
            }
            return Ok(rules::render_primitive_method_expression(
                "Rust",
                receiver,
                op,
                &object,
                &(values.join(", ")),
            ));
        }
        if let Type::Array { element, .. } = &object.ty
            && matches!(op.as_str(), "includes" | "indexOf" | "lastIndexOf")
        {
            self.standard_array = true;
            self.preludes.insert(Prelude::Array);
            let (base, absence) = if let Type::Option { element, absence } = element.as_ref() {
                (element.as_ref(), absence.as_str())
            } else {
                (element.as_ref(), "")
            };
            let object = self.borrow(object)?;
            let search = self.expr(
                args[0]
                    .as_ref()
                    .expect("the checker retained the search operand"),
            )?;
            let from = self.expr(
                args[1]
                    .as_ref()
                    .expect("the checker retained the normalized start"),
            )?;
            return Ok(rules::render_primitive_array_search_expression(
                op,
                &object,
                &search,
                &from,
                base.kind(),
                absence,
            ));
        }
        Err(unsupported(
            &format!(".{op}()"),
            "the checked primitive method has no rendering rule",
            expression.span,
        ))
    }
}
