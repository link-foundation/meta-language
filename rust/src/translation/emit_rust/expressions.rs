//! Expressions, arithmetic, matches, casts and `main`.

use super::{
    BinaryOp, ByZero, Decimal, Effect, Expr, Flavor, LitValue, Node, Pattern, Prelude, Result,
    Rounding, RustEmitter, Semantics, Type, UnaryOp, block, comparison_operator, indent, is_copy,
    own, rename_main, rounding_text, rust_string, snake, unsupported,
};
use crate::translation::aborts::{cast_message, overflow_message, zero_divisor_message};
use crate::translation::rust_record_rules::render_rust_constructor_fields;

impl RustEmitter<'_> {
    pub(super) fn expr(&mut self, expr: &Expr) -> Result<String> {
        if let Some(projection) = self.reference_array_projection(expr)? {
            return Ok(projection);
        }
        if let Node::Method { object, op, .. } = &expr.node
            && crate::translation::array_state_method_rules::accept_linked_array_state_method(
                "method",
                op,
                object.ty.kind(),
                object.ty.is_reference(),
            )
        {
            return self.array_state_method(expr);
        }
        if let Node::Method { object, op, .. } = &expr.node
            && crate::translation::record_write_rules::accept_linked_record_field_write(
                "method",
                op,
                object.ty.kind(),
                object.ty.is_reference(),
            )
        {
            return self.record_field_write(expr);
        }
        if let Node::Method { object, op, .. } = &expr.node
            && object.ty.is_reference()
            && !crate::translation::reference_method_rules::read_reference_method_result(
                object.ty.kind(),
                op,
            )
            .is_empty()
        {
            return self.reference_method(expr);
        }
        if self.program.source_language == crate::translation::Language::JavaScript
            && matches!(&expr.node, Node::Ctor { frozen: true, .. })
        {
            return Err(unsupported(
                "explicit data object freeze",
                "this target needs a JavaScript property descriptor model to retain explicit constructor freezing",
                expr.span,
            ));
        }
        let stateful_receiver = match &expr.node {
            Node::Index { array, .. }
            | Node::OptionalIndex { array, .. }
            | Node::Length { array } => array.ty.is_reference(),
            Node::RecordField { object, .. }
            | Node::TupleIndex { object, .. }
            | Node::Method { object, .. }
            | Node::CollectionSize { object }
            | Node::CollectionRead { object, .. }
            | Node::CollectionItems { object, .. } => object.ty.is_reference(),
            _ => false,
        };
        let reference_operation =
            crate::translation::surface_reference_analysis_rules::read_surface_reference_emission_operation(
                expr.kind(), stateful_receiver, expr.ty.is_reference(),
            );
        if !reference_operation.is_empty() {
            return Err(unsupported(
                &reference_operation,
                &crate::translation::surface_reference_analysis_rules::read_surface_reference_emission_refusal(
                    &reference_operation,
                ),
                expr.span,
            ));
        }
        match &expr.node {
            Node::Record { entries, order } => self.record_value(
                entries,
                order.as_deref(),
                matches!(&expr.ty, Type::Record { ordered: true, .. }),
            ),
            Node::RecordField { object, index, .. } => self.record_field(object, *index),
            Node::Lit { value } => Ok(self.literal(expr, value)),
            Node::Unit => Ok("()".to_owned()),
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
            | Node::OptionalPredicate { .. } => self.optional_expr(expr),

            Node::Tuple {
                items,
                frozen,
                product_model,
            } => {
                if product_model.as_ref().is_some_and(|model| !crate::translation::typed_program_link_rules::accept_program_tuple_product_model(model, *frozen)) {
                    return Err(unsupported("immutable tuple product model", "the declared immutable value product requires its retained frozen facet", expr.span));
                }
                let mut texts = Vec::with_capacity(items.len());
                for item in items {
                    texts.push(self.expr(item)?);
                }
                Ok(
                    crate::translation::product_emission_rules::render_tuple_value(
                        "Rust", &texts, *frozen,
                    ),
                )
            }
            Node::TupleIndex { .. } => {
                let text = self.tuple_place(expr)?;
                Ok(if is_copy(&expr.ty, None) {
                    text
                } else {
                    format!("({text}).clone()")
                })
            }
            Node::Var { name } => Ok(
                if self.array_final_uses.contains(&std::ptr::from_ref(expr)) {
                    name.clone()
                } else {
                    own(name, &expr.ty)
                },
            ),
            Node::FunctionRef { func } => Ok(
                crate::translation::callable_source_rules::render_callable_function_reference(
                    "Rust",
                    &(format!("crate::{}", self.state.reference(func, "::"))),
                ),
            ),
            Node::Apply { func, args } => {
                let function = self.expr(func)?;
                let arguments = args
                    .iter()
                    .map(|argument| self.expr(argument))
                    .collect::<Result<Vec<_>>>()?;
                Ok(
                    crate::translation::callable_source_rules::render_callable_application(
                        "Rust", &function, &arguments,
                    ),
                )
            }
            Node::Lambda { params, body } => self.callable_lambda(expr, params, body),
            Node::CellAllocate { value } => {
                let value = self.expr(value)?;
                Ok(
                    crate::translation::callable_source_rules::render_callable_cell_operation(
                        "Rust",
                        "cellAllocate",
                        "",
                        &value,
                        "",
                        "",
                        "",
                    ),
                )
            }
            Node::CellRead { cell } => {
                let cell = self.receiver(cell)?;
                Ok(
                    crate::translation::callable_source_rules::render_callable_cell_operation(
                        "Rust", "cellRead", &cell, "", "", "", "",
                    ),
                )
            }
            Node::CellWrite { cell, value, body } => {
                self.temporaries += 1;
                let cell_name = format!("ml_cell{}", self.temporaries);
                let value_name = format!("ml_value{}", self.temporaries);
                let cell = self.expr(cell)?;
                let value = self.expr(value)?;
                let body = self.expr(body)?;
                Ok(
                    crate::translation::callable_source_rules::render_callable_cell_operation(
                        "Rust",
                        "cellWrite",
                        &cell,
                        &value,
                        &body,
                        &cell_name,
                        &value_name,
                    ),
                )
            }
            Node::ReferenceAllocate { value } => {
                let value = self.expr(value)?;
                Ok(crate::translation::surface_reference_analysis_rules::render_surface_reference_operation(
                    "Rust", "referenceAllocate", value, "",
                ))
            }
            Node::ReferenceRead { reference } => {
                self.temporaries += 1;
                let temporary = format!("ml_reference_value{}", self.temporaries);
                let value = self.receiver(reference)?;
                Ok(crate::translation::surface_reference_analysis_rules::render_surface_reference_operation(
                    "Rust", "referenceRead", value, &temporary,
                ))
            }
            Node::Print { text, body } => {
                let text = self.expr(text)?;
                let body = self.expr(body)?;
                Ok(format!(
                    "{{\n{}\n}}",
                    indent(&format!("println!(\"{{}}\", {text});\n{body}"), 1)
                ))
            }
            Node::OutNil | Node::OutCons { .. } => {
                unreachable!("only the Lean and Rocq emitters thread output")
            }
            Node::Call { func, args } if self.borrowed_call_targets.is_some() => {
                let signature = self
                    .borrowed_call_targets
                    .as_ref()
                    .unwrap()
                    .get(func)
                    .cloned()
                    .expect("the selected interface validates every actual callee");
                let mut values = Vec::with_capacity(args.len());
                for argument in args {
                    values.push(self.expr(argument)?);
                }
                let names = values
                    .iter()
                    .map(|_| {
                        self.borrowed_call_scope
                            .as_mut()
                            .expect("a direct call owns its hygienic function scope")
                            .fresh(
                                crate::translation::borrowed_call_interface::borrowed_call_policy()
                                    ["argumentName"]
                                    .as_str()
                                    .unwrap(),
                            )
                    })
                    .collect::<Vec<_>>();
                let head = format!("crate::{}", self.state.reference(func, "::"));
                Ok(
                    crate::translation::borrowed_call_interface::render_rust_borrowed_call(
                        &signature, &head, &values, &names,
                    ),
                )
            }
            Node::Call { func, args }
                if self
                    .loop_params
                    .as_ref()
                    .is_some_and(|(name, _)| name == func) =>
            {
                let params = self
                    .loop_params
                    .as_ref()
                    .map(|(_, names)| names.clone())
                    .unwrap_or_default();
                // A variable the next iteration keeps as it is, an array most of all, is not copied.
                let mut texts = Vec::with_capacity(args.len());
                let mut names = Vec::with_capacity(args.len());
                for (arg, param) in args.iter().zip(&params) {
                    let text = self.expr(arg)?;
                    if arg.var_name() != Some(param.as_str()) {
                        texts.push(text);
                        names.push(param.clone());
                    }
                }
                let assign = match names.as_slice() {
                    [] => String::new(),
                    [name] => format!("{name} = {};\n", texts[0]),
                    _ => format!("({}) = ({});\n", names.join(", "), texts.join(", ")),
                };
                Ok(block(&format!("{assign}continue;")))
            }
            Node::Call { func, args } if self.program.external(func).is_some() => {
                self.external_call(func, args, &expr.ty)
            }
            Node::Call { func, args } => {
                let head = format!("crate::{}", self.state.reference(func, "::"));
                let mut texts = Vec::with_capacity(args.len());
                for arg in args {
                    texts.push(self.expr(arg)?);
                }
                let fields = declared
                    .fields
                    .iter()
                    .map(|field| field.name.clone().unwrap_or_default())
                    .collect::<Vec<_>>();
                Ok(render_rust_constructor_fields(
                    self.program.source_language.as_str(),
                    &head,
                    &fields,
                    &texts,
                ))
            }
            Node::Ctor {
                data, ctor, args, ..
            } => {
                let program = self.program;
                let entry = program.data(data);
                let declared = entry
                    .ctors
                    .iter()
                    .find(|candidate| candidate.name == *ctor)
                    .expect("the checker resolves every constructor");
                let head = format!("crate::{}", self.state.ctor_ref(data, ctor, "::"));
                if args.is_empty() {
                    return Ok(head);
                }
                let mut texts = Vec::with_capacity(args.len());
                for (arg, field) in args.iter().zip(&declared.fields) {
                    let text = self.expr(arg)?;
                    texts.push(if matches!(field.ty, Type::Data { .. }) {
                        format!("Box::new({text})")
                    } else {
                        text
                    });
                }
                Ok(format!("{head}({})", texts.join(", ")))
            }
            Node::Unary { op, arg, .. } => {
                if *op == UnaryOp::Not {
                    return Ok(format!("!{}", self.receiver(arg)?));
                }
                if matches!(expr.ty, Type::Fixed { .. }) {
                    let text = format!("{}.checked_neg()", self.receiver(arg)?);
                    return Ok(self.checked(&text, &expr.ty, "neg"));
                }
                if expr.ty.is_float() {
                    return Ok(format!("(-{})", self.receiver(arg)?));
                }
                Ok(format!("{}.neg()", self.receiver(arg)?))
            }
            Node::Binary { .. } => self.binary(expr),
            Node::If {
                cond,
                then,
                otherwise,
            } => {
                let cond = self.expr(cond)?;
                let then = self.expr(then)?;
                let otherwise = self.expr(otherwise)?;
                Ok(format!(
                    "if {cond} {{\n{}\n}} else {{\n{}\n}}",
                    indent(&then, 1),
                    indent(&otherwise, 1)
                ))
            }
            Node::Let { name, value, body } => {
                let value = self.expr(value)?;
                let body = self.expr(body)?;
                Ok(block(&format!("let {name} = {value};\n{body}")))
            }
            Node::Match { .. } => self.match_expr(expr),
            Node::Method { .. } => self.primitive_method(expr),
            Node::StringMap { op, string } => {
                if self.program.source_language == crate::translation::Language::JavaScript
                    && crate::translation::unicode_case_emission_rules::accept_unicode_case_mapping_operation(op) {
                    self.preludes.insert(Prelude::UnicodeCase);
                    self.state.encode("unicode-default-case-16",&crate::translation::unicode_case_emission_rules::read_unicode_case_encoding_statement());
                    let value = self.expr(string)?;
                    return Ok(crate::translation::unicode_case_emission_rules::render_unicode_case_string_map("Rust",op,self.utf16_strings,&value));
                }
                let object = self.receiver(string)?;
                if self.utf16_strings {
                    return Ok(crate::translation::primitive_method_rules::render_primitive_string_representation(true, "map", &object, "", op));
                }
                Ok(
                    crate::translation::frontend_rules::render_string_map_expression(
                        "Rust", op, &object,
                    ),
                )
            }
            Node::StringTest { op, string, search } => {
                if crate::translation::string_order_rules::require_string_order_units(
                    "stringTest",
                    op,
                ) {
                    let left = self.borrow(string)?;
                    let right = self.borrow(search)?;
                    return Ok(
                        crate::translation::string_order_rules::render_string_order_expression(
                            "Rust", op, &left, &right,
                        ),
                    );
                }
                let object = self.receiver(string)?;
                if self.utf16_strings {
                    let search = self.borrow(search)?;
                    return Ok(crate::translation::primitive_method_rules::render_primitive_string_representation(true, "test", &object, &search, op));
                }
                let search = match &search.node {
                    Node::Lit { value } => rust_string(&value.text()),
                    _ => format!("{}.as_str()", self.receiver(search)?),
                };
                Ok(
                    crate::translation::frontend_rules::render_string_test_expression(
                        "Rust", op, &object, &search,
                    ),
                )
            }
            Node::ToString { arg, console } => {
                match arg.ty {
                    Type::String => return self.expr(arg),
                    Type::Data { .. } | Type::Unit => {
                        return Err(unsupported(
                            "output of structured values",
                            &format!("a {} value has no portable textual form", arg.ty.kind()),
                            arg.span,
                        ));
                    }
                    Type::Float => {
                        self.preludes.insert(Prelude::Number);
                        let function =
                            crate::translation::primitive_value_rules::read_number_display_function(
                                arg.ty.kind(),
                                *console,
                            );
                        let text = format!("crate::ml_number::{function}({})", self.expr(arg)?);
                        return Ok(crate::translation::primitive_method_rules::render_primitive_string_representation(self.utf16_strings, "text", &text, "", ""));
                    }
                    _ => {}
                }
                let text = format!("{}.to_string()", self.receiver(arg)?);
                Ok(crate::translation::primitive_method_rules::render_primitive_string_representation(self.utf16_strings, "text", &text, "", ""))
            }
            Node::Cast { .. } => self.cast(expr),
            Node::Abort { message } => Ok(format!("panic!(\"{{}}\", {})", rust_string(message))),
            Node::Array { .. }
            | Node::Set { .. }
            | Node::Map { .. }
            | Node::Append { .. }
            | Node::Index { .. }
            | Node::Length { .. }
            | Node::OptionalIndex { .. }
            | Node::StringPoints { .. }
            | Node::CollectionItems { .. } => self.array_expr(expr),
            Node::CollectionSize { .. } | Node::CollectionRead { .. } => self.array_expr(expr),
            Node::Math { .. } => self.math(expr),
        }
    }

    fn tuple_place(&mut self, expr: &Expr) -> Result<String> {
        let Node::TupleIndex { object, index } = &expr.node else {
            unreachable!("a tuple projection place")
        };
        let text = match &object.node {
            Node::TupleIndex { .. } => self.tuple_place(object)?,
            Node::Var { name } => name.clone(),
            _ => self.expr(object)?,
        };
        Ok(
            crate::translation::product_emission_rules::render_tuple_projection(
                "Rust",
                &text,
                &index.to_string(),
            ),
        )
    }

    pub(super) fn literal(&mut self, expr: &Expr, value: &LitValue) -> String {
        let text = value.text();
        match &expr.ty {
            Type::Nat | Type::Int => {
                self.preludes.insert(Prelude::Big);
                let bound = 1i128 << 126;
                let small = Decimal::parse(&text)
                    .and_then(|value| value.to_i128())
                    .is_some_and(|value| value >= -bound && value < bound);
                if small {
                    format!("crate::ml::Big::from_i128({text})")
                } else {
                    format!("crate::ml::Big::parse(\"{text}\")")
                }
            }
            Type::Fixed { .. } => {
                if text.starts_with('-') {
                    format!("({text}{})", expr.ty.key())
                } else {
                    format!("{text}{}", expr.ty.key())
                }
            }
            Type::Float => {
                self.floats();
                rust_float(&text)
            }
            Type::Bool => text,
            Type::String => {
                crate::translation::primitive_method_rules::render_primitive_string_representation(
                    self.utf16_strings,
                    "literal",
                    &(if self.utf16_strings {
                        text.encode_utf16()
                            .map(|unit| unit.to_string())
                            .collect::<Vec<_>>()
                            .join(", ")
                    } else {
                        rust_string(&text)
                    }),
                    "",
                    "",
                )
            }
            other => unreachable!("no Rust literal for {}", other.kind()),
        }
    }

    /// A machine-integer result out of range panics, with Rust's own message.
    pub(super) fn checked(&mut self, text: &str, ty: &Type, op: &str) -> String {
        self.machine_integer(ty);
        format!("{text}.expect({})", rust_string(overflow_message(op)))
    }

    fn machine_integer(&mut self, ty: &Type) {
        let key = ty.key();
        self.state.encode(
            &format!("machine-integer:{key}"),
            &format!("{key} stays a Rust {key}; its arithmetic is checked and panics where the source aborts, with the source's message"),
        );
    }

    pub(super) fn binary(&mut self, expr: &Expr) -> Result<String> {
        let Node::Binary {
            op,
            left,
            right,
            semantics,
            domain,
            ..
        } = &expr.node
        else {
            unreachable!("a binary expression")
        };
        let reference_error =
            crate::translation::reference_identity_rules::read_reference_identity_comparison_error(
                op.name(),
                left.ty.kind(),
                right.ty.kind(),
                left.ty.is_reference(),
                right.ty.is_reference(),
                left.ty == right.ty,
                domain.is_some(),
                domain.as_ref().is_some_and(Type::is_reference),
                domain.as_ref() == Some(&left.ty),
            );
        if !reference_error.is_empty() {
            return Err(unsupported(
                "reference comparison",
                &reference_error,
                expr.span,
            ));
        }
        if crate::translation::reference_identity_rules::accept_reference_identity_comparison(
            op.name(),
            left.ty.kind(),
            right.ty.kind(),
            left.ty.is_reference(),
            right.ty.is_reference(),
            left.ty == right.ty,
        ) {
            let left = self.borrow(left)?;
            let right = self.borrow(right)?;
            return Ok(
                crate::translation::reference_identity_rules::render_reference_identity_comparison(
                    op.name(),
                    &left,
                    &right,
                ),
            );
        }
        match op {
            BinaryOp::And => Ok(format!("({} && {})", self.expr(left)?, self.expr(right)?)),
            BinaryOp::Or => Ok(format!("({} || {})", self.expr(left)?, self.expr(right)?)),
            BinaryOp::Concat => {
                let left = self.receiver(left)?;
                let right = if self.utf16_strings {
                    self.borrow(right)?
                } else {
                    self.receiver(right)?
                };
                Ok(crate::translation::primitive_method_rules::render_primitive_string_representation(self.utf16_strings, "concat", &left, &right, ""))
            }
            BinaryOp::Eq
            | BinaryOp::Ne
            | BinaryOp::Lt
            | BinaryOp::Le
            | BinaryOp::Gt
            | BinaryOp::Ge => Ok(format!(
                "({} {} {})",
                self.compared(left)?,
                comparison_operator(*op),
                self.compared(right)?
            )),
            _ if *semantics == Some(Semantics::Ieee) => self.float_arithmetic(*op, left, right),
            _ if matches!(expr.ty, Type::Fixed { .. }) => self.fixed_arithmetic(expr),
            _ => self.big_arithmetic(expr),
        }
    }

    pub(super) fn floats(&mut self) {
        self.state.encode(
            "floats",
            "a JavaScript Number is an f64 with the same IEEE-754 arithmetic; % on f64 is the same truncated remainder, and ml_number::js_number prints a value as JavaScript does",
        );
    }

    /// f64 arithmetic is IEEE-754 binary64 arithmetic, as JavaScript's; Rust's
    /// `%` is fmod, as JavaScript's.
    fn float_arithmetic(&mut self, op: BinaryOp, left: &Expr, right: &Expr) -> Result<String> {
        let operator = match op {
            BinaryOp::Add => "+",
            BinaryOp::Sub => "-",
            BinaryOp::Mul => "*",
            BinaryOp::Div => "/",
            _ => "%",
        };
        Ok(format!(
            "({} {operator} {})",
            self.expr(left)?,
            self.expr(right)?
        ))
    }

    pub(super) fn fixed_arithmetic(&mut self, expr: &Expr) -> Result<String> {
        let Node::Binary {
            op,
            left,
            right,
            rounding,
            ..
        } = &expr.node
        else {
            unreachable!("a binary expression")
        };
        let left = self.receiver(left)?;
        let right = self.expr(right)?;
        if matches!(op, BinaryOp::Div | BinaryOp::Rem) {
            let signed = matches!(expr.ty, Type::Fixed { signed: true, .. });
            if !matches!(rounding, Some(Rounding::Trunc | Rounding::Euclid)) && signed {
                return Err(unsupported(
                    &format!("{} division on {}", rounding_text(*rounding), expr.ty.key()),
                    "Rust machine integers divide with truncating or Euclidean rounding only",
                    expr.span,
                ));
            }
            // Rust integer division always panics by zero and on overflow, with the source's messages.
            self.machine_integer(&expr.ty);
            if *rounding == Some(Rounding::Euclid) {
                return Ok(format!("{left}.{}_euclid({right})", op.name()));
            }
            let operator = if *op == BinaryOp::Div { "/" } else { "%" };
            return Ok(format!("({left} {operator} {right})"));
        }
        Ok(self.checked(
            &format!("{left}.checked_{}({right})", op.name()),
            &expr.ty,
            op.name(),
        ))
    }

    pub(super) fn big_arithmetic(&mut self, expr: &Expr) -> Result<String> {
        let Node::Binary {
            op,
            left: left_expr,
            right: right_expr,
            semantics,
            rounding,
            by_zero,
            ..
        } = &expr.node
        else {
            unreachable!("a binary expression")
        };
        let left = self.receiver(left_expr)?;
        let right = self.borrow(right_expr)?;
        match op {
            BinaryOp::Add => Ok(format!("{left}.add({right})")),
            BinaryOp::Mul => Ok(format!("{left}.mul({right})")),
            BinaryOp::Sub => {
                if *semantics == Some(Semantics::Truncated) {
                    return Ok(format!(
                        "crate::ml::nat_sub({}, {right})",
                        self.borrow(left_expr)?
                    ));
                }
                Ok(format!("{left}.sub({right})"))
            }
            BinaryOp::Div | BinaryOp::Rem => {
                let zero = if *by_zero == Some(ByZero::Abort) {
                    format!(
                        "Some({})",
                        rust_string(zero_divisor_message(op.name(), &expr.ty))
                    )
                } else {
                    "None".to_owned()
                };
                Ok(format!(
                    "crate::ml::divide({}, {right}, \"{}\", {zero}, {})",
                    self.borrow(left_expr)?,
                    rounding_text(*rounding),
                    *op == BinaryOp::Rem
                ))
            }
            other => unreachable!("no Rust operator {other:?}"),
        }
    }

    pub(super) fn match_expr(&mut self, expr: &Expr) -> Result<String> {
        let Node::Match { scrutinee, cases } = &expr.node else {
            unreachable!("a match expression")
        };
        let fallback = cases
            .iter()
            .find(|kase| matches!(kase.pattern, Pattern::Wild | Pattern::Bind { .. }));
        if let Type::Data { name } = &scrutinee.ty {
            let program = self.program;
            let entry = program.data(name);
            let mut arms = Vec::with_capacity(cases.len());
            for kase in cases {
                let arm = match &kase.pattern {
                    Pattern::Wild => format!("_ => {}", block(&self.expr(&kase.body)?)),
                    Pattern::Bind { name } => {
                        format!("{name} => {}", block(&self.expr(&kase.body)?))
                    }
                    Pattern::Ctor { data, ctor, binds } => {
                        let declared = entry
                            .ctors
                            .iter()
                            .find(|candidate| candidate.name == *ctor)
                            .expect("the checker resolves every constructor");
                        let head = format!("crate::{}", self.state.ctor_ref(data, ctor, "::"));
                        let names: Vec<String> = binds
                            .iter()
                            .map(|bind| bind.clone().unwrap_or_else(|| "_".to_owned()))
                            .collect();
                        let mut lines: Vec<String> = binds
                            .iter()
                            .zip(&declared.fields)
                            .filter_map(|(bind, field)| match bind {
                                Some(bind) if matches!(field.ty, Type::Data { .. }) => {
                                    Some(format!("let {bind} = *{bind};"))
                                }
                                _ => None,
                            })
                            .collect();
                        lines.push(self.expr(&kase.body)?);
                        let fields = declared
                            .fields
                            .iter()
                            .map(|field| field.name.clone().unwrap_or_default())
                            .collect::<Vec<_>>();
                        let text = render_rust_constructor_fields(
                            self.program.source_language.as_str(),
                            &head,
                            &fields,
                            &names,
                        );
                        format!("{text} => {}", block(&lines.join("\n")))
                    }
                    Pattern::NatZero | Pattern::NatSucc { .. } => {
                        unreachable!("a natural pattern on a data type")
                    }
                };
                arms.push(arm);
            }
            return Ok(format!(
                "match {} {{\n{}\n}}",
                self.expr(scrutinee)?,
                indent(&arms.join("\n"), 1)
            ));
        }
        let natural = cases
            .iter()
            .any(|kase| matches!(kase.pattern, Pattern::NatZero | Pattern::NatSucc { .. }));
        if scrutinee.ty != Type::Nat && !natural {
            let kase = fallback.expect("a match without constructors has a fallback case");
            let pattern = match &kase.pattern {
                Pattern::Bind { name } => name.as_str(),
                _ => "_",
            };
            let subject = self.expr(scrutinee)?;
            let body = block(&self.expr(&kase.body)?);
            return Ok(format!(
                "match {subject} {{\n{}\n}}",
                indent(&format!("{pattern} => {body}"), 1)
            ));
        }
        let mut lines = Vec::new();
        let subject = if let Some(name) = scrutinee.var_name() {
            name.to_owned()
        } else {
            self.temporaries += 1;
            let subject = format!("__subject{}", self.temporaries);
            let value = self.expr(scrutinee)?;
            lines.push(format!("let {subject} = {value};"));
            subject
        };
        let zero = cases
            .iter()
            .find(|kase| matches!(kase.pattern, Pattern::NatZero));
        let succ = cases.iter().find_map(|kase| match &kase.pattern {
            Pattern::NatSucc { name } => Some((name, &kase.body)),
            _ => None,
        });
        let zero_text = match zero {
            Some(zero) => self.expr(&zero.body)?,
            None => self.fallback_body(fallback, &subject)?,
        };
        let machine = matches!(scrutinee.ty, Type::Fixed { .. });
        // On an unsigned machine integer the successor case holds a value of at least 1, so `- 1` cannot wrap.
        let predecessor = if machine {
            format!("{subject} - 1")
        } else {
            format!("crate::ml::pred(&{subject})")
        };
        let succ_text = match succ {
            Some((name, body)) => format!("let {name} = {predecessor};\n{}", self.expr(body)?),
            None => self.fallback_body(fallback, &subject)?,
        };
        let test = if machine {
            format!("{subject} == 0")
        } else {
            format!("{subject}.is_zero()")
        };
        lines.push(format!(
            "if {test} {{\n{}\n}} else {{\n{}\n}}",
            indent(&zero_text, 1),
            indent(&succ_text, 1)
        ));
        if lines.len() == 1 {
            return Ok(lines.remove(0));
        }
        Ok(block(&lines.join("\n")))
    }

    pub(super) fn fallback_body(
        &mut self,
        fallback: Option<&crate::translation::ir::Case>,
        subject: &str,
    ) -> Result<String> {
        let kase = fallback.expect("a match missing a natural case has a fallback case");
        let mut lines = Vec::new();
        if let Pattern::Bind { name } = &kase.pattern {
            lines.push(format!("let {name} = {subject}.clone();"));
        }
        lines.push(self.expr(&kase.body)?);
        Ok(lines.join("\n"))
    }

    pub(super) fn cast(&mut self, expr: &Expr) -> Result<String> {
        let Node::Cast {
            arg,
            from,
            to,
            flavor,
            message,
            ..
        } = &expr.node
        else {
            unreachable!("a cast")
        };
        let arg = self.expr(arg)?;
        if *flavor == Flavor::Wrapping {
            let (
                Type::Fixed {
                    signed: source_signed,
                    ..
                },
                Type::Fixed { bits, signed },
            ) = (from, to)
            else {
                return Err(unsupported(
                    "wrapping integer cast",
                    "requires actual fixed integer source and target types",
                    expr.span,
                ));
            };
            let code =
                crate::translation::fixed_integer_cast_rules::render_wrapping_fixed_integer_cast(
                    "Rust",
                    &arg,
                    f64::from(*bits),
                    *signed,
                    *source_signed,
                );
            if code.is_empty() {
                return Err(unsupported(
                    "wrapping integer cast",
                    "the integer width needs an exact target encoding",
                    expr.span,
                ));
            }
            self.state.encode(
                "fixed integer casts",
                &crate::translation::fixed_integer_cast_rules::read_fixed_integer_cast_statement(),
            );
            return Ok(code);
        }
        if matches!(from, Type::Fixed { .. }) && matches!(to, Type::Fixed { .. }) {
            return Ok(format!("{}::from({arg})", to.key()));
        }
        match flavor {
            Flavor::Exact => Ok(arg),
            Flavor::Clamp => Ok(format!("crate::ml::clamp_nat({arg})")),
            Flavor::Checked => Ok(format!(
                "crate::ml::to_nat_checked({arg}, {})",
                rust_string(cast_message(message.as_deref()))
            )),
            Flavor::Wrapping => {
                unreachable!("wrapping casts return their checked fixed representation above")
            }
        }
    }

    pub(super) fn main(&mut self, main: &crate::translation::ir::Main) -> Result<String> {
        let effects = rename_main(main, &snake, &self.state.local_reserved());
        let mut lines = Vec::with_capacity(effects.len());
        let mut assertion = 0;
        for effect in &effects {
            match effect {
                Effect::Print { expr, .. } => {
                    lines.push(format!("println!(\"{{}}\", {});", self.expr(expr)?));
                }
                Effect::Let { name, value, .. } => {
                    lines.push(format!("let {name} = {};", self.expr(value)?));
                }
                Effect::Assert { prop, .. } => {
                    assertion += 1;
                    lines.push(format!(
                        "assert!({}, \"assertion {assertion}\");",
                        self.prop(prop)?
                    ));
                    self.state
                        .assertion_theorem(&format!("assertion {assertion}"), effect);
                }
                Effect::Output { .. } | Effect::Unwrap { .. } => {
                    unreachable!("only the Lean and Rocq emitters thread output")
                }
            }
        }
        if main.sequential_async {
            self.state.encode(
                "sequential-async",
                "an async function is the function its body computes and await is its call: every call of one is awaited where it is made, so nothing runs concurrently and the output is the same, in the same order",
            );
        }
        self.state.encode(
            "program-output",
            "main prints the lines the source program prints, in order, with println!",
        );
        Ok(format!(
            "fn ml_main() {{\n{}\n}}",
            indent(&lines.join("\n"), 1)
        ))
    }
}

/// A Number's canonical JavaScript text as an `f64` expression.
fn rust_float(value: &str) -> String {
    match value {
        "NaN" => "f64::NAN".to_owned(),
        "Infinity" => "f64::INFINITY".to_owned(),
        "-Infinity" => "f64::NEG_INFINITY".to_owned(),
        _ if value.starts_with('-') => format!("({value}f64)"),
        _ => format!("{value}f64"),
    }
}
