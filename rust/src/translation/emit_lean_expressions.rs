impl LeanEmitter<'_> {
    fn expr(&mut self, e: &Expr, depth: usize) -> Result<String> {
        if let Some(copy) =
            crate::translation::scalar_vector_copy::read_rust_scalar_vector_copy_value(
                self.program.source_language.as_str(),
                e,
            )
        {
            self.state.encode(&crate::translation::rust_scalar_vector_copy_rules::read_rust_scalar_vector_copy_encoding(),&crate::translation::rust_scalar_vector_copy_rules::read_rust_scalar_vector_copy_statement());
            let value = self.expr(copy, depth)?;
            return Ok(crate::translation::rust_scalar_vector_copy_rules::render_rust_scalar_vector_copy_value("Lean",&value));
        }
        if let Some(text) = super::formal_unit_strings::expression(self, e, depth)? {
            return Ok(text);
        }
        if self.program.source_language == crate::translation::Language::JavaScript
            && matches!(&e.node, Node::Ctor { frozen: true, .. })
        {
            return Err(unsupported(
                "explicit data object freeze",
                "this target needs a JavaScript property descriptor model to retain explicit constructor freezing",
                e.span,
            ));
        }
        if let Node::Tuple {
            frozen,
            product_model: Some(model),
            ..
        } = &e.node
        {
            if !accept_program_tuple_product_model(model, *frozen) {
                return Err(unsupported(
                    "immutable tuple product model",
                    "the tuple model must be immutable value product with explicit frozen evidence",
                    e.span,
                ));
            }
            let mut pending = vec![&e.ty];
            while let Some(ty) = pending.pop() {
                if let Type::Tuple { elements } = ty {
                    if elements.len() < 2 {
                        return Err(unsupported(
                            "immutable tuple arity",
                            "this target needs a nominal product representation to distinguish unit and single-element tuple arities",
                            e.span,
                        ));
                    }
                    pending.extend(elements);
                }
            }
        }
        Ok(match &e.node {
            Node::Record { entries, order } => self.record_value(
                entries,
                order.as_deref(),
                matches!(&e.ty, Type::Record { ordered: true, .. }),
                depth,
            )?,
            Node::RecordField { object, index, .. } => self.record_field(object, *index, depth)?,
            Node::ReferenceAllocate { .. }
            | Node::ReferenceRead { .. }
            | Node::CellAllocate { .. }
            | Node::CellRead { .. }
            | Node::CellWrite { .. } => {
                return Err(unsupported(
                    "mutable structural value",
                    &super::surface_reference_analysis_rules::read_surface_reference_refusal(),
                    e.span,
                ));
            }
            Node::Lit { value } => {
                if e.ty.is_float() {
                    self.ty(&e.ty)?;
                }
                literal(&e.ty, value)?
            }
            Node::Unit => "()".to_owned(),
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
            | Node::OptionalPredicate { .. } => self.optional_expr(e, depth)?,
            Node::OutNil => "([] : List String)".to_owned(),
            Node::OutCons { head, tail } => {
                format!(
                    "({} :: {})",
                    self.expr(head, depth)?,
                    self.expr(tail, depth)?
                )
            }
            Node::Tuple { items, .. } => {
                let mut parts = Vec::new();
                for item in items {
                    parts.push(self.expr(item, depth)?);
                }
                format!("({})", parts.join(", "))
            }
            Node::TupleIndex { object, index } => {
                let Type::Tuple { elements } = &object.ty else {
                    return Err(unsupported(
                        "tuple projection",
                        "the projection needs a checked tuple layout",
                        e.span,
                    ));
                };
                let value = self.expr(object, depth)?;
                if elements.len() == 1 {
                    value
                } else {
                    format!(
                        "({value}){}{}",
                        ".2".repeat(*index),
                        if *index < elements.len() - 1 {
                            ".1"
                        } else {
                            ""
                        }
                    )
                }
            }
            Node::Var { name } => self.successors.get(name).unwrap_or(name).clone(),
            Node::FunctionRef { .. } | Node::Apply { .. } | Node::Lambda { .. } => {
                self.callable_expression(e, depth)?
            }
            Node::Call { func, args } => {
                let head = self.state.reference(func, ".");
                self.application(head, args, depth)?
            }
            Node::Ctor {
                data, ctor, args, ..
            } => {
                let head = self.state.ctor_ref(data, ctor, ".");
                self.application(head, args, depth)?
            }
            Node::Unary { op, arg, semantics } => {
                if *op == UnaryOp::Not {
                    format!("(!{})", self.expr(arg, depth)?)
                } else {
                    if *semantics == Some(Semantics::Checked) {
                        unthreaded("a machine-integer negation");
                    }
                    format!("(-{})", self.expr(arg, depth)?)
                }
            }
            Node::Binary { .. } => self.binary(e, depth)?,
            Node::If {
                cond,
                then,
                otherwise,
            } => {
                let cond = self.expr(cond, depth)?;
                let then = self.expr(then, depth)?;
                format!(
                    "(if {cond} then {then} else {})",
                    self.expr(otherwise, depth)?
                )
            }
            Node::Let { name, value, body } => {
                let value = self.expr(value, depth)?;
                format!(
                    "(let {name} := {value};\n{}{})",
                    pad(depth + 1),
                    self.expr(body, depth + 1)?
                )
            }
            Node::Match { scrutinee, cases } => self.match_expr(scrutinee, cases, depth)?,
            Node::Method { object, op, args } => {
                if op == "toSorted" {
                    return Err(unsupported(
                        "array copying sort",
                        "this target needs an explicit UTF-16 string ordering interpretation",
                        e.span,
                    ));
                }
                if op == "readJavaScriptRecordPropertyOrder" {
                    let rendered = self.expr(object, depth)?;
                    return crate::translation::ordered_record_values::order_read(
                        "Lean", e, &rendered,
                    );
                }
                if matches!(object.ty, Type::Array { .. }) && op == "map" {
                    let callback = args
                        .first()
                        .and_then(Option::as_ref)
                        .expect("the checked map has its actual callback");
                    return self.array_map_expression(object, callback, depth);
                }
                if object.ty.kind() == "array"
                    && crate::translation::array_from_rules::accept_formal_array_callback_operation(op)
                {
                    let callback = args[0].as_ref().expect("the checked array callback");
                    let Type::Function { params, .. } = &callback.ty else {
                        unreachable!("the checked callback retains its function type");
                    };
                    let count = serde_json::Number::from(params.len()).as_f64()
                        .expect("the checked callback arity has a Number representation");
                    let object = self.expr(object, depth)?;
                    let callback = self.expr(callback, depth)?;
                    let mut initial = String::new();
                    if op == "reduce" {
                        let Some(value) = args.get(1).and_then(Option::as_ref) else {
                            return Err(unsupported("array reduce without an initial value",
                                "the empty-array TypeError needs an operational abort encoding in Lean", e.span));
                        };
                        initial = self.expr(value, depth)?;
                    }
                    return Ok(crate::translation::array_from_rules::render_formal_array_callback_operation(
                        "Lean", op, &object, &callback, count, &initial, "",
                    ));
                }
                return Err(unsupported(
                    &format!(".{op}()"),
                    &super::primitive_method_rules::read_primitive_method_rendering_refusal(
                        "Lean",
                        object.ty.kind(),
                        op,
                    ),
                    e.span,
                ));
            }
            Node::StringMap { op, .. } => {
                return Err(crate::translation::diagnostics::unsupported(
                    &format!(".{op}()"),
                    &crate::translation::frontend_rules::read_string_map_refusal("Lean", op),
                    e.span,
                ));
            }
            Node::StringTest { op, string, search } => {
                let refusal =
                    crate::translation::string_order_rules::read_string_order_refusal("Lean", op);
                if !refusal.is_empty() {
                    return Err(crate::translation::diagnostics::unsupported(
                        "string comparison",
                        &refusal,
                        e.span,
                    ));
                }
                let helper =
                    crate::translation::frontend_rules::read_string_test_helper("Lean", op);
                if helper == "stringIncludes" {
                    self.helpers.insert("stringIncludes");
                }
                let object = self.expr(string, depth)?;
                let search = self.expr(search, depth)?;
                crate::translation::frontend_rules::render_string_test_expression(
                    "Lean", op, &object, &search,
                )
            }
            Node::PropositionValue { .. } => {
                self.check(&e.proposition_value_atom().ok_or_else(|| {
                    unsupported(
                        "proposition value",
                        "an actual checked comparison is required",
                        e.span,
                    )
                })?)?
            }
            Node::ToString { arg, console } => {
                if arg.ty == Type::String {
                    self.expr(arg, depth)?
                } else {
                    self.text_of(arg, depth, *console)?
                }
            }
            Node::Cast {
                arg,
                from,
                to,
                flavor,
                ..
            } => self.cast(arg, from, to, *flavor, depth)?,
            Node::Abort { .. } => unthreaded("an abort"),
            Node::Array { .. }
            | Node::Set { .. }
            | Node::Map { .. }
            | Node::Append { .. }
            | Node::Index { .. }
            | Node::Length { .. }
            | Node::OptionalIndex { .. }
            | Node::StringPoints { .. }
            | Node::CollectionItems { .. } => self.array_expr(e, depth)?,
            Node::CollectionSize { .. } | Node::CollectionRead { .. } => {
                self.array_expr(e, depth)?
            }
            Node::Math { .. } => self.math(e, depth)?,
            // Output threading leaves no `print` node.
            Node::Print { .. } => {
                return Err(type_error(
                    "print with its output not threaded".to_owned(),
                    e.span,
                ));
            }
        })
    }

    /// A call or constructor application: the bare head without arguments.
    fn application(&mut self, head: String, args: &[Expr], depth: usize) -> Result<String> {
        if args.is_empty() {
            return Ok(head);
        }
        let mut parts = vec![head];
        for arg in args {
            parts.push(self.expr(arg, depth)?);
        }
        Ok(format!("({})", parts.join(" ")))
    }

    fn binary(&mut self, e: &Expr, depth: usize) -> Result<String> {
        let Node::Binary {
            op, left, right, ..
        } = &e.node
        else {
            unreachable!("binary expressions only");
        };
        let left_text = self.expr(left, depth)?;
        let right_text = self.expr(right, depth)?;
        let comparison = match op {
            BinaryOp::And => return Ok(format!("({left_text} && {right_text})")),
            BinaryOp::Or => return Ok(format!("({left_text} || {right_text})")),
            BinaryOp::Concat => return Ok(format!("({left_text} ++ {right_text})")),
            BinaryOp::Eq => return Ok(format!("({left_text} == {right_text})")),
            BinaryOp::Ne => return Ok(format!("({left_text} != {right_text})")),
            BinaryOp::Lt => "<",
            BinaryOp::Le => "≤",
            BinaryOp::Gt => ">",
            BinaryOp::Ge => "≥",
            _ => return self.arithmetic(e, &left_text, &right_text),
        };
        Ok(format!("(decide ({left_text} {comparison} {right_text}))"))
    }

    fn arithmetic(&mut self, e: &Expr, left_text: &str, right_text: &str) -> Result<String> {
        let Node::Binary {
            op,
            domain,
            rounding,
            by_zero,
            semantics,
            ..
        } = &e.node
        else {
            unreachable!("binary expressions only");
        };
        if *semantics == Some(Semantics::Ieee) {
            let operator = match op {
                BinaryOp::Add => "+",
                BinaryOp::Sub => "-",
                BinaryOp::Mul => "*",
                BinaryOp::Div => "/",
                _ => {
                    self.helpers.insert("floatRem");
                    return Ok(format!("(ml_float_rem {left_text} {right_text})"));
                }
            };
            return Ok(format!("({left_text} {operator} {right_text})"));
        }
        // A machine-integer operation, one that may abort, reaches here as its abort test and its value computed on the representation (output.rs).
        if *semantics == Some(Semantics::Checked)
            || *by_zero == Some(ByZero::Abort)
            || matches!(e.ty, Type::Fixed { .. })
        {
            unthreaded(&format!("a machine-integer {}", op.name()));
        }
        match op {
            BinaryOp::Add => Ok(format!("({left_text} + {right_text})")),
            BinaryOp::Mul => Ok(format!("({left_text} * {right_text})")),
            BinaryOp::Sub => Ok(format!("({left_text} - {right_text})")),
            BinaryOp::Div | BinaryOp::Rem => {
                let division = *op == BinaryOp::Div;
                if matches!(domain, Some(Type::Nat)) {
                    return Ok(format!(
                        "({left_text} {} {right_text})",
                        if division { "/" } else { "%" }
                    ));
                }
                if *rounding == Some(Rounding::Trunc) {
                    return Ok(format!(
                        "(Int.{} {left_text} {right_text})",
                        if division { "tdiv" } else { "tmod" }
                    ));
                }
                if *rounding == Some(Rounding::Floor) {
                    return Ok(format!(
                        "(Int.{} {left_text} {right_text})",
                        if division { "fdiv" } else { "fmod" }
                    ));
                }
                Ok(format!(
                    "({left_text} {} {right_text})",
                    if division { "/" } else { "%" }
                ))
            }
            other => Err(type_error(
                format!("no Lean operator {}", operator_name(*other)),
                None,
            )),
        }
    }

    fn match_expr(&mut self, scrutinee: &Expr, cases: &[Case], depth: usize) -> Result<String> {
        let subject = self.expr(scrutinee, depth)?;
        let mut arms = Vec::new();
        for kase in cases {
            let text = match &kase.pattern {
                Pattern::NatZero => "0".to_owned(),
                Pattern::NatSucc { name } => format!("{name} + 1"),
                Pattern::Wild => "_".to_owned(),
                Pattern::Bind { name } => name.clone(),
                Pattern::Ctor { data, ctor, binds } => {
                    let mut parts = vec![self.state.ctor_ref(data, ctor, ".")];
                    parts.extend(
                        binds
                            .iter()
                            .map(|bind| bind.clone().unwrap_or_else(|| "_".to_owned())),
                    );
                    parts.join(" ")
                }
            };
            let saved = self.successors.clone();
            if let (Pattern::NatSucc { name }, Some(variable)) =
                (&kase.pattern, scrutinee.var_name())
            {
                self.successors
                    .insert(variable.to_owned(), format!("({name} + 1)"));
            }
            let body = self.expr(&kase.body, depth + 2);
            self.successors = saved;
            arms.push(format!("{}| {text} => {}", pad(depth + 1), body?));
        }
        Ok(format!("(match {subject} with\n{})", arms.join("\n")))
    }
}

fn literal(ty: &Type, value: &LitValue) -> Result<String> {
    Ok(match ty {
        Type::Float => lean_float(&value.text()),
        Type::Nat => format!("({} : Nat)", value.text()),
        Type::Int => format!("({} : Int)", value.text()),
        Type::Fixed { signed, .. } => {
            format!(
                "({} : {})",
                value.text(),
                if *signed { "Int" } else { "Nat" }
            )
        }
        Type::Bool => value.text(),
        Type::String => json_string(&value.text()),
        other => {
            return Err(type_error(
                format!("no Lean literal for {}", other.kind()),
                None,
            ));
        }
    })
}

/// A Number's canonical JavaScript text as a Float term; Lean reads decimal
/// literals correctly rounded.
fn lean_float(value: &str) -> String {
    let text = match value {
        "NaN" => "0.0 / 0.0".to_owned(),
        "Infinity" => "1.0 / 0.0".to_owned(),
        "-Infinity" => "-1.0 / 0.0".to_owned(),
        _ => value.replacen("e+", "e", 1),
    };
    format!("({text} : Float)")
}

/// A string literal as JavaScript's `JSON.stringify` writes it.
fn json_string(text: &str) -> String {
    serde_json::to_string(text).unwrap_or_default()
}

/// The operator's name in the checked program's JSON form.
fn operator_name(op: BinaryOp) -> String {
    serde_json::to_value(op)
        .ok()
        .and_then(|value| value.as_str().map(str::to_owned))
        .unwrap_or_default()
}

fn collect_unfold(plan: &Plan) -> Vec<&String> {
    match plan {
        Plan::Close { hints } => hints.unfold.iter().collect(),
        Plan::Induction(split) | Plan::Cases(split) => split
            .cases
            .iter()
            .flat_map(|kase| collect_unfold(&kase.plan))
            .collect(),
    }
}

fn pad(depth: usize) -> String {
    "  ".repeat(depth)
}
