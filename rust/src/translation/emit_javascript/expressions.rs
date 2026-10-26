//! Complete expression emission, separated without changing its decisions.

use crate::translation::external_emission_rules::{
    ml_string::Text, render_java_script_external_call,
};

use super::{
    Expr, Helper, JavaScriptEmitter, Node, Result, Type, UnaryOp, ctor_object, indent, json_string,
    literal, negate, unsupported,
};

impl JavaScriptEmitter<'_> {
    pub(super) fn expr(&mut self, e: &Expr) -> Result<String> {
        if let Some(copy) =
            crate::translation::scalar_vector_copy::read_rust_scalar_vector_copy_value(
                self.program.source_language.as_str(),
                e,
            )
        {
            self.state.encode(&crate::translation::rust_scalar_vector_copy_rules::read_rust_scalar_vector_copy_encoding(),&crate::translation::rust_scalar_vector_copy_rules::read_rust_scalar_vector_copy_statement());
            let value = self.expr(copy)?;
            return Ok(crate::translation::rust_scalar_vector_copy_rules::render_rust_scalar_vector_copy_value("JavaScript",&value));
        }
        if e.ty.is_float() {
            self.uses_float = true;
        }
        match &e.node {
            Node::Record { entries, order } => self.record_value(
                entries,
                order.as_deref(),
                matches!(&e.ty, Type::Record { ordered: true, .. }),
            ),
            Node::RecordField { object, field, .. } => self.record_field(object, field),
            Node::Lit { value } => literal(&e.ty, value),
            Node::None { .. }
            | Node::Some { .. }
            | Node::FlattenOption { .. }
            | Node::Coalesce { .. }
            | Node::OptionalField { .. }
            | Node::OptionalProject { .. }
            | Node::NullishNumber { .. }
            | Node::OptionalIndexNumber { .. }
            | Node::OptionalText { .. }
            | Node::OptionalTypeOf { .. }
            | Node::Truthy { .. }
            | Node::StrictEqual { .. }
            | Node::UnwrapOption { .. }
            | Node::OptionalPredicate { .. } => self.optional_expr(e),

            Node::Unit => Ok(
                crate::translation::product_emission_rules::render_java_script_unit(
                    self.program.source_language.as_str(),
                ),
            ),
            Node::Tuple {
                items,
                frozen,
                product_model,
            } => {
                if product_model.as_ref().is_some_and(|model| {
                    !crate::translation::typed_program_link_rules::accept_program_tuple_product_model(
                        model, *frozen,
                    )
                }) {
                    return Err(unsupported(
                        "immutable tuple product model",
                        "the declared immutable value product requires its retained frozen facet",
                        e.span,
                    ));
                }
                let texts = self.exprs(items)?;
                Ok(
                    crate::translation::product_emission_rules::render_tuple_value(
                        "JavaScript",
                        &texts,
                        *frozen,
                    ),
                )
            }
            Node::TupleIndex { object, index } => {
                let text = self.expr(object)?;
                Ok(
                    crate::translation::product_emission_rules::render_tuple_projection(
                        "JavaScript",
                        &text,
                        &index.to_string(),
                    ),
                )
            }
            Node::Var { name } => Ok(name.clone()),
            Node::FunctionRef { .. }
            | Node::Apply { .. }
            | Node::Lambda { .. }
            | Node::CellAllocate { .. }
            | Node::CellRead { .. }
            | Node::CellWrite { .. } => self.callable_expression(e),
            Node::ReferenceAllocate { value } => {
                let value = self.expr(value)?;
                Ok(
                    crate::translation::surface_reference_analysis_rules::render_surface_reference_operation(
                        "JavaScript",
                        "referenceAllocate",
                        value,
                        "",
                    ),
                )
            }
            Node::ReferenceRead { reference } => {
                let value = self.expr(reference)?;
                Ok(
                    crate::translation::surface_reference_analysis_rules::render_surface_reference_operation(
                        "JavaScript",
                        "referenceRead",
                        value,
                        "",
                    ),
                )
            }
            Node::Print { text, body } => Ok(format!(
                "(console.log({}), {})",
                self.expr(text)?,
                self.expr(body)?
            )),
            Node::OutNil | Node::OutCons { .. } => {
                unreachable!("only the Lean and Rocq emitters thread output")
            }
            Node::Call { func, args } => {
                if let Some(external) = self.program.external(func) {
                    let kind = match external {
                        crate::translation::surface::External::Function { .. } => "fn",
                        crate::translation::surface::External::Constant { .. } => "const",
                    };
                    let args = self.exprs(args)?.join(", ");
                    let rendered = render_java_script_external_call(
                        &Text::from_text(kind),
                        &Text::from_text(func),
                        &Text::from_text(&args),
                    )
                    .to_string();
                    if rendered.is_empty() {
                        return Err(super::unsupported(
                            "JavaScript external call",
                            "this checked external binding has no supported target operation",
                            None,
                        ));
                    }
                    return Ok(rendered);
                }
                let target = self.state.reference(func, ".");
                let args = self.exprs(args)?;
                Ok(format!("{target}({})", args.join(", ")))
            }
            Node::Ctor {
                data,
                ctor,
                args,
                frozen,
            } => {
                let entry = self.program.data(data);
                let ctor = entry
                    .ctors
                    .iter()
                    .find(|candidate| candidate.name == *ctor)
                    .unwrap_or_else(|| panic!("{data} has no constructor {ctor}"));
                let args = self.exprs(args)?;
                Ok(ctor_object(
                    ctor,
                    &args,
                    self.program.source_language,
                    *frozen,
                ))
            }
            Node::Unary { op, arg, .. } => {
                let arg = self.expr(arg)?;
                Ok(match op {
                    UnaryOp::Not => format!("!{arg}"),
                    UnaryOp::Neg => self.checked(negate(&arg), &e.ty, "neg"),
                })
            }
            Node::Binary { .. } => self.binary(e),
            Node::If {
                cond,
                then,
                otherwise,
            } => Ok(format!(
                "({} ? {} : {})",
                self.expr(cond)?,
                self.expr(then)?,
                self.expr(otherwise)?
            )),
            Node::Let { .. } | Node::Match { .. } => Ok(format!(
                "(() => {{\n{}\n}})()",
                indent(&self.statements(e)?.join("\n"), 1)
            )),
            Node::Method { object, op, args } => {
                if op == "toSorted" {
                    let receiver = self.expr(object)?;
                    return Ok(
                        crate::translation::string_array_sort_rules::render_string_array_copy_sort(
                            "JavaScript",
                            &receiver,
                            false,
                        ),
                    );
                }
                if op == "readJavaScriptRecordPropertyOrder" {
                    let rendered = self.expr(object)?;
                    return crate::translation::ordered_record_values::order_read(
                        "JavaScript",
                        e,
                        &rendered,
                    );
                }
                if crate::translation::array_state_method_rules::accept_linked_array_state_method(
                    "method",
                    op,
                    object.ty.kind(),
                    object.ty.is_reference(),
                ) {
                    let receiver = self.expr(object)?;
                    let mut arguments = Vec::new();
                    for argument in args {
                        let value = argument.as_ref().ok_or_else(|| {
                            unsupported(
                                "shared array range argument",
                                "each actual source argument needs its expression",
                                e.span,
                            )
                        })?;
                        arguments.push(self.expr(value)?);
                    }
                    self.state.encode(&crate::translation::array_state_method_rules::read_array_state_method_identity(op),&crate::translation::array_state_method_rules::read_array_state_method_statement());
                    return Ok(crate::translation::array_state_method_rules::render_array_state_method_expression("JavaScript",op,&receiver,&arguments,&[],&[],""));
                }
                if crate::translation::record_write_rules::accept_linked_record_field_write(
                    "method",
                    op,
                    object.ty.kind(),
                    object.ty.is_reference(),
                ) {
                    let receiver = self.expr(object)?;
                    let mut arguments = Vec::new();
                    for argument in args {
                        let value = argument.as_ref().ok_or_else(|| {
                            unsupported(
                                "shared record field assignment",
                                "the actual operand is missing",
                                e.span,
                            )
                        })?;
                        arguments.push(self.expr(value)?);
                    }
                    self.state.encode(
                        "write-record-field",
                        &crate::translation::record_write_rules::read_record_field_write_statement(
                        ),
                    );
                    return Ok(crate::translation::record_write_rules::render_record_field_write("JavaScript",
                        &crate::translation::record_write_rules::read_record_field_write_operator(op), &receiver,
                        &json_string(&crate::translation::record_write_rules::read_record_field_write_name(op)), "", &arguments, ""));
                }
                if object.ty.is_reference()
                    && !crate::translation::reference_method_rules::read_reference_method_result(
                        object.ty.kind(),
                        op,
                    )
                    .is_empty()
                {
                    let receiver = self.expr(object)?;
                    let mut arguments = Vec::new();
                    for argument in args {
                        let value = argument.as_ref().ok_or_else(|| {
                            unsupported(
                                "shared collection method argument",
                                "every source argument needs its actual expression",
                                e.span,
                            )
                        })?;
                        arguments.push(self.expr(value)?);
                    }
                    self.state.encode(&crate::translation::reference_method_rules::read_reference_method_identity(object.ty.kind(), op),
                        &crate::translation::reference_method_rules::read_reference_method_statement());
                    return Ok(crate::translation::reference_method_rules::render_reference_method_expression("JavaScript", object.ty.kind(), op, &receiver, &arguments, false, ""));
                }
                if op == "map"
                    && let [Some(callback)] = args.as_slice()
                {
                    let object = self.expr(object)?;
                    let callback = self.expr(callback)?;
                    return Ok(
                        crate::translation::array_from_rules::render_array_from_mapping(
                            "JavaScript",
                            &object,
                            &callback,
                            0.0,
                            "",
                        ),
                    );
                }
                let receiver_kind = object.ty.kind();
                let object = self.expr(object)?;
                let mut values = Vec::with_capacity(args.len());
                for (index, argument) in args.iter().enumerate() {
                    let text = if let Some(argument) = argument {
                        self.expr(argument)?
                    } else {
                        String::new()
                    };
                    #[allow(clippy::cast_precision_loss)]
                    let position = index as f64;
                    values.push(
                        crate::translation::primitive_method_rules::render_primitive_method_argument(
                            "JavaScript",
                            "",
                            op,
                            position,
                            argument.is_none(),
                            &text,
                        ),
                    );
                }
                if crate::translation::utf16_method_emission_rules::accept_utf16_text_method(
                    "method",
                    receiver_kind,
                    op,
                ) {
                    self.helpers.insert(Helper::Utf16Text);
                    return Ok(
                        crate::translation::utf16_method_emission_rules::render_utf16_text_method(
                            "JavaScript",
                            op,
                            &object,
                            &values.join(", "),
                        ),
                    );
                }
                Ok(
                    crate::translation::primitive_method_rules::render_primitive_method_expression(
                        "JavaScript",
                        "",
                        op,
                        &object,
                        &(values.join(", ")),
                    ),
                )
            }
            Node::StringMap { op, string } => {
                if self.program.source_language == crate::translation::Language::JavaScript
                    && crate::translation::unicode_case_emission_rules::accept_unicode_case_mapping_operation(op) {
                    self.helpers.insert(Helper::UnicodeCase);
                    self.state.encode("unicode-default-case-16",&crate::translation::unicode_case_emission_rules::read_unicode_case_encoding_statement());
                    let value = self.expr(string)?;
                    return Ok(crate::translation::unicode_case_emission_rules::render_unicode_case_string_map("JavaScript",op,false,&value));
                }
                let value = self.expr(string)?;
                let object = if matches!(string.node, Node::Var { .. } | Node::Lit { .. }) {
                    value
                } else {
                    format!("({value})")
                };
                Ok(
                    crate::translation::frontend_rules::render_string_map_expression(
                        "JavaScript",
                        op,
                        &object,
                    ),
                )
            }
            Node::StringTest { op, string, search } => {
                if crate::translation::string_order_rules::require_string_order_units(
                    "stringTest",
                    op,
                ) {
                    let left = self.expr(string)?;
                    let right = self.expr(search)?;
                    return Ok(
                        crate::translation::string_order_rules::render_string_order_expression(
                            "JavaScript",
                            op,
                            &left,
                            &right,
                        ),
                    );
                }
                let value = self.expr(string)?;
                let object = if matches!(string.node, Node::Var { .. } | Node::Lit { .. }) {
                    value
                } else {
                    format!("({value})")
                };
                let search = self.expr(search)?;
                Ok(
                    crate::translation::frontend_rules::render_string_test_expression(
                        "JavaScript",
                        op,
                        &object,
                        &search,
                    ),
                )
            }
            Node::ToString { arg, console } => {
                if *console && arg.ty.is_float() {
                    self.helpers.insert(Helper::ShowNumber);
                    Ok(format!("ml_showNumber({})", self.expr(arg)?))
                } else if matches!(arg.ty, Type::String) {
                    self.expr(arg)
                } else {
                    self.text_of(arg)
                }
            }
            Node::Cast {
                arg,
                from,
                to,
                flavor,
                message,
                ..
            } => self.cast(arg, from, to, *flavor, message.as_deref()),
            Node::Abort { message } => {
                self.helpers.insert(Helper::Abort);
                Ok(format!("ml_abort({})", json_string(message)))
            }
            Node::Array { .. }
            | Node::Set { .. }
            | Node::Map { .. }
            | Node::Append { .. }
            | Node::Index { .. }
            | Node::Length { .. }
            | Node::OptionalIndex { .. }
            | Node::StringPoints { .. }
            | Node::CollectionItems { .. } => self.array_expr(e),
            Node::CollectionSize { .. } | Node::CollectionRead { .. } => self.array_expr(e),
            Node::Math { .. } => self.math(e),
        }
    }

    pub(super) fn exprs(&mut self, list: &[Expr]) -> Result<Vec<String>> {
        list.iter().map(|arg| self.expr(arg)).collect()
    }
}
