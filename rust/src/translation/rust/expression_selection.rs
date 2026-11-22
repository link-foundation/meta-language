impl RustParser {
    pub(super) fn postfix(&mut self, path: &[String], options: Options) -> Result<SExpr> {
        let mut value = self.primary(path, options)?;
        loop {
            let token = self.peek();
            if self.cursor.is("(")
                && matches!(
                    self.origins.origin(&value, path).ty,
                    Some(Type::Function { .. })
                )
            {
                self.cursor.advance();
                let args = self.contextual_call_arguments(path, &value)?;
                let close = self.cursor.expect(")", Some("closure call"))?;
                let span = Some(super::Span::new(
                    value.span.map_or(token.start, |span| span.start),
                    close.end,
                ));
                value = self
                    .read_copy_closure_call(&value, &args, path, span)?
                    .expect("actual callable source type");
                continue;
            }
            if self.cursor.is(".") {
                self.cursor.advance();
                if self.cursor.is_kind(TokenKind::Number) {
                    self.refuse_collection_observation(&value, path, "field projection", true)?;
                    let field = self.product_index_token()?;
                    let index = field.raw.parse::<usize>().map_err(|_| {
                        unsupported(
                            "tuple index",
                            "the positional index exceeds the exact supported integer range",
                            Some(field.span()),
                        )
                    })?;
                    if index as u64 > 9_007_199_254_740_991 {
                        return Err(unsupported(
                            "tuple index",
                            "the positional index exceeds the exact supported integer range",
                            Some(field.span()),
                        ));
                    }
                    let range = Some(super::Span::new(
                        value.span.map_or(token.start, |range| range.start),
                        field.end,
                    ));
                    if let Some(record) = self.record_projection(&value, &field.raw, path, range)? {
                        value = record;
                        continue;
                    }
                    value = SExpr::new(
                        SNode::TupleIndex {
                            object: Box::new(value),
                            index,
                        },
                        range,
                    );
                    self.origins.product_projection(&value, path, range)?;
                    continue;
                }
                let method = self.cursor.identifier(Some("method"))?;
                if let Some(result) =
                    self.read_exclusive_owned_string_mutation(&value, &method, path)?
                {
                    value = result;
                    continue;
                }
                if let Some(result) =
                    self.read_exclusive_owned_vector_append(&value, &method, path)?
                {
                    value = result;
                    continue;
                }
                if let Some(result) =
                    self.read_exclusive_owned_vector_mutation(&value, &method, path)?
                {
                    value = result;
                    continue;
                }
                if let Some(result) =
                    self.read_owned_vector_iteration_method(&value, &method, path)?
                {
                    value = result;
                    continue;
                }
                if let Some(result) = self.standard_collection_method(&value, &method, path)? {
                    value = result;
                    continue;
                }
                self.guard_borrowed_vector_observation(
                    &value,
                    path,
                    &method.value,
                    Some(Span::new(
                        value.span.map_or(token.start, |span| span.start),
                        method.end,
                    )),
                )?;
                if let Some(boxed) = self.read_owned_box_clone(&value, &method, path)? {
                    value = boxed;
                    continue;
                }
                if let Some(vector) = self.read_owned_vector_clone(&value, &method, path)? {
                    value = vector;
                    continue;
                }
                if let Some(sum) = self.read_owned_standard_sum_observer(&value, &method, path)? {
                    value = sum;
                    continue;
                }
                if let Some(sum) = self.read_standard_sum_clone(&value, &method, path)? {
                    value = sum;
                    continue;
                }
                if let Some(nominal) = self.read_nominal_clone(&value, &method, path)? {
                    value = nominal;
                    continue;
                }
                if let Some(sum) = self.read_owned_standard_sum_method(&value, &method, path)? {
                    value = sum;
                    continue;
                }
                if let Some(sum) = self.read_owned_standard_sum_callback(&value, &method, path)? {
                    value = sum;
                    continue;
                }
                if let Some(sum) =
                    self.read_standard_sum_source_combinator(&value, &method, path)?
                {
                    value = sum;
                    continue;
                }
                if let Some(sum) = self.read_standard_sum_source_nested(&value, &method, path)? {
                    value = sum;
                    continue;
                }
                if let Some(sum) =
                    self.read_standard_sum_source_conversion(&value, &method, path)?
                {
                    value = sum;
                    continue;
                }
                if let Some(sum) = self.read_owned_standard_sum_predicate(&value, &method, path)? {
                    value = sum;
                    continue;
                }
                if let Some(sum) = self.read_standard_sum_source_predicate(&value, &method, path)? {
                    value = sum;
                    continue;
                }
                if let Some(sum) = self.read_standard_sum_source_method(&value, &method, path)? {
                    value = sum;
                    continue;
                }
                if let Some(ascii) = self.read_ascii_text_predicate(&value, &method, path)? {
                    value = ascii;
                    continue;
                }
                if !self.cursor.is("(") {
                    self.refuse_collection_observation(&value, path, "field projection", true)?;
                    if let Some(record) =
                        self.record_projection(&value, &method.value, path, span(&token, &method))?
                    {
                        value = record;
                        continue;
                    }
                    return Err(unsupported(
                        "field access",
                        &format!("field {} is outside the portable core", method.value),
                        span(&token, &method),
                    ));
                }
                self.capture_inherent_receiver_value_loan(&value, &method, path)?;
                self.cursor.advance();
                let args = self.call_arguments(path)?;
                self.cursor.expect(")", Some(&method.value))?;
                if let Some(shared) = self.shared_array_method(
                    &value,
                    &method,
                    &args,
                    span(&token, self.cursor.peek()),
                )? {
                    value = shared;
                    continue;
                }
                if let Some(iterator) = self.read_borrowed_iterator(
                    &value,
                    &method,
                    &args,
                    span(&token, self.cursor.peek()),
                    path,
                )? {
                    value = iterator;
                    continue;
                }
                if let Some(inherent) = self.inherent_method(
                    &value,
                    &method,
                    &args,
                    span(&token, self.cursor.peek()),
                    path,
                )? {
                    value = inherent;
                    continue;
                }
                if let Some(view) = self.byte_string_slice_view(
                    &value,
                    &method,
                    args.len(),
                    span(&token, self.cursor.peek()),
                    path,
                ) {
                    value = view;
                    continue;
                }
                if let Some(bytes) = self.string_byte_view(
                    &value,
                    &method,
                    &args,
                    span(&token, self.cursor.peek()),
                    path,
                )? {
                    value = bytes;
                    continue;
                }
                self.require_type_namespace_body_method(
                    &value,
                    &method.value,
                    span(&token, self.cursor.peek()),
                )?;
                if let Some(reduction) = self.iterator_reduction(
                    &value,
                    &method,
                    &args,
                    span(&token, self.cursor.peek()),
                    path,
                )? {
                    value = reduction;
                    continue;
                }
                if let Some(copy) = self.read_scalar_vector_copy(
                    &value,
                    &method,
                    args.len(),
                    span(&token, self.cursor.peek()),
                    path,
                )? {
                    value = copy;
                    continue;
                }
                if let Some(lookup) = self.read_array_lookup_option(
                    &value,
                    &method,
                    &args,
                    span(&token, self.cursor.peek()),
                    path,
                )? {
                    value = lookup;
                    continue;
                }
                if let Some(lookup) = self.read_array_lookup(
                    &value,
                    &method,
                    &args,
                    span(&token, self.cursor.peek()),
                    path,
                )? {
                    value = lookup;
                    continue;
                }
                if let Some(ascii) = self.read_byte_ascii(
                    &value,
                    &method,
                    &args,
                    span(&token, self.cursor.peek()),
                    path,
                )? {
                    value = ascii;
                    continue;
                }
                if let Some(selected) = self.read_primitive_selection(
                    &value,
                    &method,
                    &args,
                    span(&token, self.cursor.peek()),
                    path,
                )? {
                    value = selected;
                    continue;
                }
                if let Some(property) = self.read_signed_integer_property(
                    &value,
                    &method,
                    &args,
                    span(&token, self.cursor.peek()),
                    path,
                )? {
                    value = property;
                    continue;
                }
                if let Some(saturated) = self.read_saturating_integer(
                    &value,
                    &method,
                    &args,
                    span(&token, self.cursor.peek()),
                    path,
                )? {
                    value = saturated;
                    continue;
                }
                if let Some(product) = self.read_saturating_multiplication(
                    &value,
                    &method,
                    &args,
                    span(&token, self.cursor.peek()),
                    path,
                )? {
                    value = product;
                    continue;
                }
                if let Some(magnitude) = self.read_integer_magnitudes(
                    &value,
                    &method,
                    &args,
                    span(&token, self.cursor.peek()),
                    path,
                )? {
                    value = magnitude;
                    continue;
                }
                if let Some(bits) = self.read_integer_bits(
                    &value,
                    &method,
                    &args,
                    span(&token, self.cursor.peek()),
                    path,
                )? {
                    value = bits;
                    continue;
                }
                if let Some(shifted) = self.read_wrapping_shifts(
                    &value,
                    &method,
                    &args,
                    span(&token, self.cursor.peek()),
                    path,
                )? {
                    value = shifted;
                    continue;
                }
                if let Some(checked) = self.read_checked_integer(
                    &value,
                    &method,
                    &args,
                    span(&token, self.cursor.peek()),
                    path,
                )? {
                    value = checked;
                    continue;
                }
                if let Some(wrapped) = self.read_wrapping_integer(
                    &value,
                    &method,
                    &args,
                    span(&token, self.cursor.peek()),
                    path,
                )? {
                    value = wrapped;
                    continue;
                }
                let mut origin = self.origins.origin(&value, path);
                let method_name = method.value.as_str();
                if let Some(copy) = self.read_owned_local_copy(
                    &value,
                    &method,
                    args.len(),
                    &origin,
                    span(&token, self.cursor.peek()),
                )? {
                    value = copy;
                    continue;
                }
                let float_operation =
                    super::float_predicate_rules::accept_rust_float_operation_method(method_name);
                if float_operation
                    || super::float_predicate_rules::accept_rust_float_predicate_method(method_name)
                {
                    let range = span(&token, self.cursor.peek());
                    if !args.is_empty() {
                        return Err(type_error(
                            format!(
                                "{} expects 0 arguments but got {}",
                                method.value,
                                args.len()
                            ),
                            range,
                        ));
                    }
                    if !self.origins.collecting
                        && !self.accept_float_receiver_origin(&origin, args.len())
                    {
                        return Err(unsupported(
                            if float_operation {
                                "Rust binary64 operation"
                            } else {
                                "Rust binary64 predicate"
                            },
                            "the receiver needs a proven direct unshadowed primitive f64 source type",
                            range,
                        ));
                    }
                    if float_operation {
                        value = super::float_predicates::operation_expression(
                            method_name,
                            &value,
                            range,
                        )?;
                        // Preserve the admitted receiver's actual source owner/model;
                        // an inferred FLOAT alone cannot establish this evidence.
                        origin.ty = Some(FLOAT);
                        self.origins.mark(&value, origin);
                    } else {
                        value = super::float_predicates::expression(method_name, &value, range)?;
                        self.origins
                            .mark(&value, primitive_try_from::origin(BOOL, "bool", path));
                    }
                    continue;
                }
                if method_name == "is_empty"
                    && let Some(query) = self.read_string_empty_query(
                        &value,
                        args.len(),
                        path,
                        span(&token, self.cursor.peek()),
                    )?
                {
                    value = query;
                    continue;
                }
                if ["len", "is_empty"].contains(&method_name) {
                    if method_name == "len"
                        && let Some(bytes) = self.read_string_byte_length(
                            &value,
                            args.len(),
                            span(&token, self.cursor.peek()),
                            path,
                        )?
                    {
                        value = bytes;
                        continue;
                    }
                    value = self.array_read(
                        value,
                        &method,
                        args.len(),
                        path,
                        span(&token, self.cursor.peek()),
                    )?;
                    continue;
                }
                if ["clone", "to_owned"].contains(&method_name)
                    && origin.array_source_model.is_some()
                {
                    return Err(unsupported(
                        "Rust array clone",
                        "array cloning needs a separate source allocation and ownership proof",
                        span(&token, self.cursor.peek()),
                    ));
                }
                if ["div_euclid", "rem_euclid"].contains(&method_name)
                    && let Some(argument) = args.first()
                {
                    self.origins.direct(argument, path, argument.span)?;
                }
                let owned_text = crate::translation::rust_standard_owned_call_rules::accept_rust_standard_string_owned_method_result(
                    &method_name,
                    origin.ty.as_ref().map_or("", Type::kind),
                    origin.text_source_model.unwrap_or(""),
                    u32::try_from(origin.wrappers.len()).map_or(f64::INFINITY, f64::from),
                    origin.proven,
                    origin.wrappers.iter().all(|layer| layer == "reference"),
                    origin.source_owner.as_ref().map_or("", |owner| owner.kind.as_str()),
                    &origin.source_owner.as_ref().map_or_else(String::new, |owner| owner.path.join("::")),
                    origin.source_owner.as_ref().map_or(-1.0, |owner| u32::try_from(owner.arguments.len()).map_or(f64::INFINITY, f64::from)),
                );
                if ["clone", "to_owned"].contains(&method_name)
                    && !crate::translation::rust_owned_binding_rules::accept_rust_string_slice_clone_reference(&method_name,origin.ty.as_ref().map_or("",Type::kind),origin.text_source_model.unwrap_or(""),origin.wrappers.len() as f64)
                    && origin
                        .wrappers
                        .first()
                        .is_some_and(|layer| layer == "reference")
                {
                    origin.wrappers.remove(0);
                }
                if method_name == "to_owned" && origin.ty == Some(STRING) {
                    origin.text_source_model = Some("owned string");
                }
                if owned_text {
                    origin.source_owner =
                        Some(crate::translation::source_signatures::SourceOwner {
                            kind: "library".to_owned(),
                            path: vec!["std".to_owned(), "string".to_owned(), "String".to_owned()],
                            arguments: Vec::new(),
                        });
                    origin.source_lifetimes.clear();
                    origin.copy_evidence = None;
                }
                if method_name == "as_str" {
                    if origin.array_source_model.is_some() {
                        return Err(unsupported(
                            "method as_str",
                            "outside the portable core",
                            span(&token, self.cursor.peek()),
                        ));
                    }
                    origin = self.origins.capture_owned_borrow(origin);
                    origin.source_extending_temporary_bindings.clear();
                    origin.ty = Some(STRING);
                    origin.text_source_model = Some("string slice");
                    origin.source_owner =
                        Some(crate::translation::source_signatures::SourceOwner {
                            kind: "primitive".to_owned(),
                            path: vec!["str".to_owned()],
                            arguments: Vec::new(),
                        });
                    origin.path = path.to_vec();
                    origin.wrappers = vec!["reference".to_owned()];
                }
                value = method_call(value, &method, args, span(&token, self.cursor.peek()))?;
                if ["div_euclid", "rem_euclid"].contains(&method_name) {
                    origin.wrappers.clear();
                }
                if ["clone", "to_owned", "as_str", "div_euclid", "rem_euclid"]
                    .contains(&method_name)
                {
                    if owned_text {
                        self.origins.mark_owned_string_constructor(&value, origin);
                    } else {
                        self.origins.mark(&value, origin);
                    }
                }
                continue;
            }
            if self.cursor.is("?") {
                value = self.standard_sum_try_expression(value, path, &token)?;
                continue;
            }
            if self.cursor.is("[") {
                if self.ordinary_completion_context() {
                    value = self.scalar_array_index(value, path)?;
                    continue;
                }
                return Err(unsupported(
                    "indexing",
                    "collections are outside the portable core",
                    span(&token, &token),
                ));
            }
            return Ok(value);
        }
    }

    /// In postfix context `0.1` is two positional fields, with exact spans.
    fn product_index_token(&mut self) -> Result<Token> {
        use super::product_rules::accept_rust_tuple_index_spelling;

        let token = self.peek();
        if let Some((first, second)) = token.raw.split_once('.')
            && accept_rust_tuple_index_spelling(first)
            && accept_rust_tuple_index_spelling(second)
        {
            let middle = token.start + first.len();
            let mut first_token = token.clone();
            first.clone_into(&mut first_token.value);
            first.clone_into(&mut first_token.raw);
            first_token.suffix.clear();
            first_token.floating = false;
            first_token.end = middle;
            let mut separator = token.clone();
            separator.kind = TokenKind::Punct;
            ".".clone_into(&mut separator.value);
            ".".clone_into(&mut separator.raw);
            separator.suffix.clear();
            separator.floating = false;
            separator.start = middle;
            separator.end = middle + 1;
            let mut second_token = token.clone();
            second.clone_into(&mut second_token.value);
            second.clone_into(&mut second_token.raw);
            second_token.suffix.clear();
            second_token.floating = false;
            second_token.start = middle + 1;
            self.cursor.tokens.splice(
                self.cursor.index..=self.cursor.index,
                [first_token, separator, second_token],
            );
        }
        let field = self.cursor.advance();
        if !accept_rust_tuple_index_spelling(&field.raw) {
            return Err(unsupported(
                "tuple index spelling",
                "Rust tuple indices are decimal digits without leading zeros, separators or suffixes",
                Some(field.span()),
            ));
        }
        Ok(field)
    }

    fn array_read(
        &mut self,
        receiver: SExpr,
        method: &Token,
        count: usize,
        path: &[String],
        range: Option<Span>,
    ) -> Result<SExpr> {
        let origin = self.origins.origin(&receiver, path);
        if origin.array_source_model.is_none() {
            return Err(unsupported(
                &format!("method {}", method.value),
                "outside the portable core",
                range,
            ));
        }
        if count != 0 {
            return Err(type_error(
                format!("{} expects 0 arguments but got {count}", method.value),
                range,
            ));
        }
        if !origin.proven || !matches!(origin.ty, Some(Type::Array { .. })) {
            return Err(unsupported(
                "Rust array read",
                "read-only length needs a proven vector or immutable slice source",
                range,
            ));
        }
        if crate::translation::rust_array_length_rules::accept_rust_exact_array_length(
            &method.value,
            self.cursor.is("as") && self.cursor.is_at("f64", 1),
            self.ordinary_completion_context(),
        ) {
            let ty = rust_fixed_type("usize").expect("the existing exact 64-bit source width");
            let length = SExpr::new(
                SNode::Length {
                    object: Box::new(receiver),
                    integer: true,
                },
                range,
            );
            let result = SExpr::new(
                SNode::Cast {
                    arg: Box::new(length),
                    to: ty.clone(),
                    from: None,
                    flavor: Flavor::Exact,
                },
                range,
            );
            let mut result_origin = SourceOrigin::with_type(ty, path, Vec::new());
            result_origin.source_owner = Some(crate::translation::source_signatures::SourceOwner {
                kind: "primitive".to_owned(),
                path: vec!["usize".to_owned()],
                arguments: Vec::new(),
            });
            self.origins.mark(&result, result_origin);
            return Ok(result);
        }
        let length = SExpr::new(
            SNode::Length {
                object: Box::new(receiver),
                integer: false,
            },
            range,
        );
        let result = if method.value == "len" {
            if !self.cursor.is("as") || !self.cursor.is_at("f64", 1) {
                return Err(unsupported(
                    "Rust array length",
                    "the selected Number length model requires an explicit as f64 conversion",
                    range,
                ));
            }
            self.cursor.advance();
            self.cursor.advance();
            self.require_cast_following_token(false)?;
            length
        } else {
            let zero = SExpr::new(
                SNode::Num {
                    value: "0".to_owned(),
                    ty: Some(FLOAT),
                    negative: false,
                    unit: false,
                },
                range,
            );
            SExpr::new(
                SNode::Binary {
                    op: BinaryOp::Eq,
                    left: Box::new(length),
                    right: Box::new(zero),
                    rounding: None,
                },
                range,
            )
        };
        self.origins.mark(
            &result,
            SourceOrigin::with_type(
                if method.value == "len" { FLOAT } else { BOOL },
                path,
                Vec::new(),
            ),
        );
        Ok(result)
    }

    /// Comma-separated expressions up to (not including) the closing `)`.
    pub(super) fn call_arguments(&mut self, path: &[String]) -> Result<Vec<SExpr>> {
        let mut args = Vec::new();
        while !self.cursor.is(")") {
            args.push(self.with_standard_sum_source_context(None, |parser| {
                parser.expr(path, Options::default())
            })?);
            if self.cursor.eat(",").is_none() {
                break;
            }
        }
        Ok(args)
    }

    #[allow(clippy::too_many_lines)] // one branch per primary form, as in rust.js
    pub(super) fn primary(&mut self, path: &[String], options: Options) -> Result<SExpr> {
        self.prepare_default_type_alias_access_head(path)?;
        if let Some(value) = self.read_primitive_function_value(path, None)? {
            return Ok(value);
        }
        if let Some(value) = self.read_primitive_default(path)? {
            return Ok(value);
        }
        if let Some(value) = self.read_primitive_try_from(path)? {
            return Ok(value);
        }
        if let Some(value) = self.read_primitive_integer_constant(path)? {
            return Ok(value);
        }
        if let Some(value) = self.read_primitive_associated_call(path)? {
            return Ok(value);
        }
        if let Some(value) = self.read_primitive_selection_call(path)? {
            return Ok(value);
        }
        if let Some(value) = self.read_standard_owned_call(path)? {
            return Ok(value);
        }
        if let Some(value) = self.read_standard_string_view(path)? {
            return Ok(value);
        }
        let token = self.peek();
        if token.kind == TokenKind::Number {
            self.cursor.advance();
            let ty = if token.suffix == "f64" || (token.suffix.is_empty() && token.floating) {
                Some(super::FLOAT)
            } else if token.suffix.is_empty() {
                None
            } else {
                let Some(fixed) = rust_fixed_type(&token.suffix) else {
                    return Err(unsupported(
                        &format!("literal suffix {}", token.suffix),
                        "portable numeric literals are integers and IEEE-754 binary64 values",
                        span(&token, &token),
                    ));
                };
                Some(fixed)
            };
            let result = SExpr::new(
                SNode::Num {
                    value: token.value.clone(),
                    ty,
                    negative: false,
                    unit: false,
                },
                span(&token, &token),
            );
            if token.suffix == "f64" || (token.suffix.is_empty() && token.floating) {
                self.origins
                    .mark(&result, primitive_try_from::origin(FLOAT, "f64", path));
            }
            if !token.suffix.is_empty() && rust_fixed_type(&token.suffix).is_some() {
                let mut origin = self.origins.origin(&result, path);
                origin.source_owner = Some(crate::translation::source_signatures::SourceOwner {
                    kind: "primitive".to_owned(),
                    path: vec![token.suffix.clone()],
                    arguments: Vec::new(),
                });
                self.origins.mark(&result, origin);
            }
            return Ok(result);
        }
        if token.kind == TokenKind::String {
            self.cursor.advance();
            let result = SExpr::new(
                SNode::Str {
                    value: token.value.clone(),
                },
                span(&token, &token),
            );
            let mut origin = SourceOrigin::with_type(STRING, path, vec!["reference".to_owned()]);
            origin.text_source_model = Some("string slice");
            origin.source_owner = Some(crate::translation::source_signatures::SourceOwner {
                kind: "primitive".to_owned(),
                path: vec!["str".to_owned()],
                arguments: Vec::new(),
            });
            self.origins.mark(&result, origin);
            return Ok(result);
        }
        if token.kind == TokenKind::Macro {
            self.cursor.advance();
            return self.expression_macro(&token, path);
        }
        let character_type = read_rust_character_expression_type(token_kind(&token));
        if !character_type.is_empty() {
            let ty = rust_fixed_type(&character_type).ok_or_else(|| {
                unsupported(
                    &format!("character literal type {character_type}"),
                    "a character expression needs its represented fixed type",
                    span(&token, &token),
                )
            })?;
            self.cursor.advance();
            let origin = primitive_try_from::origin(ty.clone(), &character_type, path);
            let result = SExpr::new(
                SNode::Num {
                    value: token.value.clone(),
                    ty: Some(ty),
                    negative: false,
                    unit: false,
                },
                span(&token, &token),
            );
            self.origins.mark(&result, origin);
            return Ok(result);
        }
        let c = &self.cursor;
        if token.kind == TokenKind::ByteString {
            return self.byte_string_expression(&token, path);
        }
        if matches!(token.kind, TokenKind::Character | TokenKind::ByteCharacter) || c.is("'") {
            return Err(unsupported(
                "character literal",
                "characters are outside the portable core",
                span(&token, &token),
            ));
        }
        if c.is("(") {
            self.cursor.advance();
            if self.cursor.eat(")").is_some() {
                return Ok(SExpr::new(SNode::Unit, span(&token, &token)));
            }
            let inner = self.expr(path, Options::default())?;
            if self.cursor.is("..=") {
                return self.read_inclusive_range_query(&token, path, inner);
            }
            if self.cursor.eat(",").is_some() {
                let mut items = vec![inner];
                while !self.cursor.is(")") {
                    items.push(self.expr(path, Options::default())?);
                    if self.cursor.eat(",").is_none() {
                        break;
                    }
                }
                let close = self.cursor.expect(")", Some("tuple expression"))?;
                let range = Some(super::Span::new(token.start, close.end));
                let result = SExpr::new(
                    SNode::Tuple {
                        items,
                        frozen: self.local_value_state_model.is_empty(),
                        product_model: self
                            .local_value_state_model
                            .is_empty()
                            .then(|| "immutable value product".to_owned()),
                    },
                    range,
                );
                self.origins.product(&result, path, range)?;
                return Ok(result);
            }
            self.cursor.expect(")", Some("parenthesised expression"))?;
            return Ok(inner);
        }
        if c.is("{") {
            let mut block = self.block(path)?;
            block.block = true;
            return Ok(block);
        }
        if c.is("if") {
            return self.if_expr(path);
        }
        if c.is("match") {
            return self.match_expr(path);
        }
        if c.is("|") || c.is("||") || c.is("move") {
            return self.read_copy_closure(path);
        }
        if c.is("true") || c.is("false") {
            self.cursor.advance();
            let result = SExpr::new(
                SNode::Bool {
                    value: token.value == "true",
                },
                span(&token, &token),
            );
            self.origins
                .mark(&result, primitive_try_from::origin(BOOL, "bool", path));
            return Ok(result);
        }
        if ["unsafe", "loop", "while", "for", "return", "break"]
            .iter()
            .any(|keyword| c.is(keyword))
        {
            return Err(unsupported(
                &format!("{} expression", token.value),
                "outside the portable core",
                span(&token, &token),
            ));
        }
        if token.kind == TokenKind::Identifier {
            return self.path_expression(&token, path, options);
        }
        Err(Self::fail("expected an expression", self.cursor.peek()))
    }
}
