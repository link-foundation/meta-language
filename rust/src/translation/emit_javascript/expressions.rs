//! Complete expression emission, separated without changing its decisions.

use super::{
    Expr, Helper, JavaScriptEmitter, Node, Result, Type, UnaryOp, ctor_object, indent, json_string,
    literal, negate, unsupported,
};

impl JavaScriptEmitter<'_> {
    pub(super) fn expr(&mut self, e: &Expr) -> Result<String> {
        if e.ty.is_float() {
            self.uses_float = true;
        }
        match &e.node {
            Node::Record { entries } => self.record_value(entries),
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
                flavor,
                message,
                ..
            } => self.cast(arg, *flavor, message.as_deref()),
            Node::Abort { message } => {
                self.helpers.insert(Helper::Abort);
                Ok(format!("ml_abort({})", json_string(message)))
            }
            Node::Array { .. }
            | Node::Append { .. }
            | Node::Index { .. }
            | Node::Length { .. }
            | Node::OptionalIndex { .. }
            | Node::StringPoints { .. }
            | Node::CollectionItems { .. } => self.array_expr(e),
            Node::Math { .. } => self.math(e),
        }
    }

    pub(super) fn exprs(&mut self, list: &[Expr]) -> Result<Vec<String>> {
        list.iter().map(|arg| self.expr(arg)).collect()
    }
}
