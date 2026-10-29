//! Expression precedence, immutable reads, primary values and qualified paths.

mod array_index;
mod array_lookups;
mod ascii_text_predicates;
mod byte_ascii;
mod byte_strings;
mod checked_integer;
pub(super) mod control;
mod integer_bits;
mod integer_float_plan;
mod integer_magnitudes;
mod primitive_associated_calls;
mod primitive_default;
mod primitive_from;
mod primitive_function_values;
mod primitive_integer_constants;
mod primitive_selections;
mod primitive_try_from;
mod read_only_queries;
mod scalar_vector_copy;
mod standard_owned_calls;
pub(super) mod standard_string_views;
mod string_addition;
mod usize_index_expressions;
mod wrapping_shifts;

use super::array_rules::{accept_rust_source_array_element, accept_rust_source_array_owner};
use super::cast_syntax_rules::{accept_rust_cast_following_token, read_rust_cast_syntax_message};
use super::{
    Alias, BOOL, BinaryOp, FLOAT, Flavor, Options, Result, RustParser, SArrayItem, SExpr, SNode,
    SPattern, SPatternNode, SRow, STRING, SourceOrigin, Span, Token, TokenKind, TranslationError,
    Type, UnaryOp, additive_op, binary, comparison_op, describe, is_if_or_match, joined,
    method_call, multiplicative_op, operator, pattern_value, rust_fixed_type, rust_macro_message,
    span, type_error, unsupported,
};
use crate::translation::rust_character_rules::read_rust_character_expression_type;
use crate::translation::source_diagnostic_attributes::token_kind;

impl RustParser {
    pub(super) fn expr(&mut self, path: &[String], options: Options) -> Result<SExpr> {
        self.or(path, options)
    }

    pub(super) fn or(&mut self, path: &[String], options: Options) -> Result<SExpr> {
        let mut left = self.and(path, options)?;
        while self.cursor.is("||") {
            let token = self.cursor.advance();
            let right = self.and(path, Options::default())?;
            left = binary(BinaryOp::Or, left, right, &token);
            self.origins.pointer_integer_expression(&left, path)?;
        }
        Ok(left)
    }

    pub(super) fn and(&mut self, path: &[String], options: Options) -> Result<SExpr> {
        let mut left = self.comparison(path, options)?;
        while self.cursor.is("&&") {
            let token = self.cursor.advance();
            let right = self.comparison(path, Options::default())?;
            left = binary(BinaryOp::And, left, right, &token);
            self.origins.pointer_integer_expression(&left, path)?;
        }
        Ok(left)
    }

    pub(super) fn comparison(&mut self, path: &[String], options: Options) -> Result<SExpr> {
        let left = self.primitive_bitwise(path, options, 0)?;
        let token = self.peek();
        let c = &self.cursor;
        let shift = (token.value == "<" || token.value == ">")
            && c.is_at(&token.value, 1)
            && c.peek_at(1).start == token.end;
        let block_statement = options.statement && is_if_or_match(&left);
        if token.kind == TokenKind::Punct
            && (shift || ["|", "^", "&"].contains(&token.value.as_str()))
            && !block_statement
        {
            let operator = if shift {
                token.value.repeat(2)
            } else {
                token.value.clone()
            };
            return Err(unsupported(
                &format!("operator {operator}"),
                "bitwise operators are outside the portable core",
                span(&token, c.peek_at(usize::from(shift))),
            ));
        }
        if token.kind == TokenKind::Punct
            && let Some(op) = comparison_op(&token.value)
        {
            self.cursor.advance();
            let right = self.primitive_bitwise(path, Options::default(), 0)?;
            self.refuse_callable_observation(&left, path, joined(left.span, right.span, &token))?;
            self.refuse_callable_observation(&right, path, joined(left.span, right.span, &token))?;
            self.refuse_shared_value_observation(
                &left,
                Some(&right),
                path,
                joined(left.span, right.span, &token),
            )?;
            let after = self.cursor.peek();
            if after.kind == TokenKind::Punct && comparison_op(&after.value).is_some() {
                return Err(Self::fail("comparison operators cannot be chained", after));
            }
            let result = binary(op, left, right, &token);
            self.origins.pointer_integer_expression(&result, path)?;
            return Ok(result);
        }
        Ok(left)
    }

    fn primitive_bitwise(&mut self, path: &[String], options: Options, level: u8) -> Result<SExpr> {
        if level == 3 {
            return self.shift(path, options);
        }
        let mut left = self.primitive_bitwise(path, options, level + 1)?;
        loop {
            let token = self.peek();
            if token.kind != TokenKind::Punct || !crate::translation::rust_primitive_bitwise_rules::accept_rust_primitive_bitwise_precedence(&token.value, f64::from(level)) {
                return Ok(left);
            }
            if options.statement && is_if_or_match(&left) {
                return Ok(left);
            }
            self.cursor.advance();
            let right = self.primitive_bitwise(path, Options::default(), level + 1)?;
            let range = joined(left.span, right.span, &token);
            left = SExpr::new(SNode::Math {
                op: crate::translation::rust_primitive_bitwise_rules::read_rust_primitive_bitwise_operation(&token.value),
                name: "Rust primitive bitwise operation".to_owned(),
                args: vec![SArrayItem { value: left, spread: false }, SArrayItem { value: right, spread: false }],
            }, range);
        }
    }

    fn shift(&mut self, path: &[String], options: Options) -> Result<SExpr> {
        let mut left = self.additive(path, options)?;
        loop {
            let token = self.peek();
            if !(token.value == "<" || token.value == ">")
                || !self.cursor.is_at(&token.value, 1)
                || self.cursor.peek_at(1).start != token.end
            {
                return Ok(left);
            }
            if options.statement && is_if_or_match(&left) {
                return Ok(left);
            }
            self.cursor.advance();
            self.cursor.advance();
            let right = self.additive(path, Options::default())?;
            let range = joined(left.span, right.span, &token);
            left = SExpr::new(
                SNode::Math {
                    op: crate::translation::rust_shift_rules::read_rust_shift_source_operation(
                        &token.value.repeat(2),
                    ),
                    name: "Rust primitive shift".to_owned(),
                    args: vec![
                        SArrayItem {
                            value: left,
                            spread: false,
                        },
                        SArrayItem {
                            value: right,
                            spread: false,
                        },
                    ],
                },
                range,
            );
        }
    }

    pub(super) fn additive(&mut self, path: &[String], options: Options) -> Result<SExpr> {
        let mut left = self.multiplicative(path, options)?;
        loop {
            let token = self.peek();
            let Some(op) = operator(&token, additive_op) else {
                return Ok(left);
            };
            if options.statement && is_if_or_match(&left) {
                return Ok(left);
            }
            self.cursor.advance();
            let right = self.multiplicative(path, Options::default())?;
            let addition = binary(op, left, right, &token);
            left = self
                .read_string_addition(&addition, path)?
                .unwrap_or(addition);
            self.origins.pointer_integer_expression(&left, path)?;
            self.scalar_loop_arithmetic(&left, path);
        }
    }

    pub(super) fn multiplicative(&mut self, path: &[String], options: Options) -> Result<SExpr> {
        let mut left = self.cast(path, options)?;
        loop {
            let token = self.peek();
            let Some(op) = operator(&token, multiplicative_op) else {
                return Ok(left);
            };
            if options.statement && is_if_or_match(&left) {
                return Ok(left);
            }
            self.cursor.advance();
            let right = self.cast(path, Options::default())?;
            left = binary(op, left, right, &token);
            self.origins.pointer_integer_expression(&left, path)?;
            self.scalar_loop_arithmetic(&left, path);
        }
    }

    /// `e as T` converts between integer types when every value fits.
    pub(super) fn cast(&mut self, path: &[String], options: Options) -> Result<SExpr> {
        let mut value = self.unary(path, options)?;
        while self.cursor.is("as") {
            let token = self.cursor.advance();
            self.refuse_callable_observation(&value, path, span(&token, self.cursor.peek()))?;
            let parenthesized_type = self.cursor.is("(");
            let to = self.parse_type()?;
            let target_owner = self.origins.current_type.source_owner.clone();
            if !self.origins.collecting && !self.origins.current_type.wrappers.is_empty() {
                return Err(unsupported(
                    "source cast",
                    "numeric casts require a direct source and target value type",
                    span(&token, self.cursor.peek()),
                ));
            }
            self.require_cast_following_token(parenthesized_type)?;
            let range = joined(value.span, span(&token, self.cursor.peek()), &token);
            value = SExpr::new(
                SNode::Cast {
                    arg: Box::new(value),
                    to,
                    from: None,
                    flavor: Flavor::Exact,
                },
                range,
            );
            let mut origin = self.origins.origin(&value, path);
            if origin.proven {
                origin.source_owner = target_owner;
                self.origins.mark(&value, origin);
            }
        }
        Ok(value)
    }

    fn require_cast_following_token(&self, parenthesized_type: bool) -> Result<()> {
        let token = self.cursor.peek();
        if accept_rust_cast_following_token(&token.value, parenthesized_type) {
            Ok(())
        } else {
            Err(TranslationError::syntax(
                read_rust_cast_syntax_message(),
                Some(token.span()),
            ))
        }
    }

    pub(super) fn unary(&mut self, path: &[String], options: Options) -> Result<SExpr> {
        let token = self.peek();
        for (symbol, op) in [("-", UnaryOp::Neg), ("!", UnaryOp::Not)] {
            if self.cursor.eat(symbol).is_some() {
                let arg = self.unary(path, Options::default())?;
                let origin = self.origins.origin(&arg, path);
                let result = SExpr::new(
                    SNode::Unary {
                        op,
                        arg: Box::new(arg),
                    },
                    span(&token, self.cursor.peek()),
                );
                if op == UnaryOp::Neg
                    && (self.accept_float_receiver_origin(&origin, 0)
                        || origin.proven
                            && matches!(origin.ty, Some(Type::Fixed { signed: true, .. }))
                            && origin.wrappers.is_empty())
                {
                    self.origins.mark(&result, origin);
                }
                return Ok(result);
            }
        }
        if self.scalar_loop_active && ["&", "&&", "*"].contains(&token.value.as_str()) {
            return Err(unsupported(
                "Rust scalar loop",
                "loop expressions exclude references and dereferences",
                Some(token.span()),
            ));
        }
        self.require_owned_local_indirection(&token)?;
        // Source evidence proves immutable indirection before projecting its value.
        if self.cursor.is("&") || self.cursor.is("&&") {
            self.cursor.advance();
            if self.cursor.is("mut") {
                return Err(unsupported(
                    "mutable reference",
                    "mutation is outside the portable core",
                    span(&token, self.cursor.peek()),
                ));
            }
            let arg = self.unary(path, Options::default())?;
            if token.value == "&&" {
                self.origins.wrap(&arg, path, "reference");
            }
            self.origins.wrap(&arg, path, "reference");
            self.shared_array_borrow(&arg, path)?;
            return Ok(arg);
        }
        if self.cursor.eat("*").is_some() {
            let arg = self.unary(path, Options::default())?;
            self.origins
                .dereference(&arg, path, span(&token, self.cursor.peek()))?;
            return Ok(arg);
        }
        self.postfix(path, options)
    }

    fn scalar_loop_arithmetic(&mut self, node: &SExpr, path: &[String]) {
        if !self.scalar_loop_active {
            return;
        }
        if let SNode::Binary { left, right, .. } = &node.node {
            let origin = self.origins.origin(left, path);
            if self.accept_float_receiver_origin(&origin, 0)
                && self.accept_float_receiver_origin(&self.origins.origin(right, path), 0)
            {
                self.origins.mark(node, origin);
            }
        }
    }

    fn accept_float_receiver_origin(&self, origin: &SourceOrigin, arguments_count: usize) -> bool {
        if !self.primitive_owner_model.is_empty()
            && let Some(position) = &origin.primitive_owner_position
            && position["resolvedPath"]
                .as_array()
                .is_some_and(|path| path.len() > 1)
        {
            let source = position["sourcePath"]
                .as_array()
                .expect("source owner path")
                .iter()
                .map(|value| value.as_str().expect("owner path segment").to_owned())
                .collect::<Vec<_>>();
            return self
                .read_standard_primitive_position(&source, &origin.path)
                .is_ok_and(|owner| owner.is_some())
                && super::float_predicate_rules::accept_rust_float_predicate_receiver(
                    origin.ty.as_ref().map_or("", Type::kind),
                    origin.float_source_model.unwrap_or(""),
                    origin.proven,
                    !origin.wrappers.is_empty(),
                    false,
                    &arguments_count.to_string(),
                );
        }
        let source_owner = vec!["f64".to_owned()];
        let owner = self.resolve(&source_owner, &origin.path);
        let shadowed = self.origins.lookup(&source_owner, &origin.path).is_some()
            || self.origins.lookup(&owner, &origin.path).is_some();
        super::float_predicate_rules::accept_rust_float_predicate_receiver(
            origin.ty.as_ref().map_or("", Type::kind),
            origin.float_source_model.unwrap_or(""),
            origin.proven,
            !origin.wrappers.is_empty(),
            shadowed,
            &arguments_count.to_string(),
        )
    }

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
                if let Some(ascii) = self.read_ascii_text_predicate(&value, &method, path)? {
                    value = ascii;
                    continue;
                }
                if !self.cursor.is("(") {
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
                self.require_type_namespace_body_method(
                    &value,
                    &method.value,
                    span(&token, self.cursor.peek()),
                )?;
                if let Some(copy) = self.read_scalar_vector_copy(
                    &value,
                    &method,
                    args.len(),
                    span(&token, self.cursor.peek()),
                    path,
                ) {
                    value = copy;
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
                    origin.ty = Some(STRING);
                    origin.text_source_model = Some("string slice");
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
                    self.origins.mark(&value, origin);
                }
                continue;
            }
            if self.cursor.is("?") {
                return Err(unsupported(
                    "? operator",
                    "Result and Option are outside the portable core",
                    span(&token, &token),
                ));
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
            args.push(self.expr(path, Options::default())?);
            if self.cursor.eat(",").is_none() {
                break;
            }
        }
        Ok(args)
    }

    #[allow(clippy::too_many_lines)] // one branch per primary form, as in rust.js
    pub(super) fn primary(&mut self, path: &[String], options: Options) -> Result<SExpr> {
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
