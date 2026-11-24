//! Expression precedence, immutable reads, primary values and qualified paths.

mod array_index;
mod array_lookup_options;
mod array_lookups;
mod ascii_text_predicates;
mod byte_ascii;
mod byte_strings;
mod checked_integer;
pub(super) mod control;
mod integer_bits;
mod integer_float_plan;
mod integer_magnitudes;
mod iterator_arithmetic;
mod iterator_pipelines;
mod iterator_reductions;
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
mod string_byte_views;
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

pub(in super::super) fn source_completion_kind(node: &SExpr) -> String {
    let operation = if let SNode::Math { op, .. } = &node.node {
        op.as_str()
    } else {
        ""
    };
    crate::translation::rust_panic_value_rules::read_rust_panic_completion_kind(
        node.node.kind(),
        operation,
    )
}

impl RustParser {
    pub(super) fn expr(&mut self, path: &[String], options: Options) -> Result<SExpr> {
        let mut value = if let Some(value) = self.read_owned_record_effect_assignment(path)? {
            value
        } else {
            self.or(path, options)?
        };
        self.origins.record_expression_type(&value, path)?;
        self.note_borrowed_expression_source_position(&mut value, path)?;
        Ok(value)
    }

    pub(super) fn or(&mut self, path: &[String], options: Options) -> Result<SExpr> {
        let mut left = self.and(path, options)?;
        while self.cursor.is("||") {
            let token = self.cursor.advance();
            let before = self.begin_owned_record_short_circuit();
            let right = self.and(path, Options::default());
            self.end_owned_record_short_circuit(before);
            let right = right?;
            left = binary(BinaryOp::Or, left, right, &token);
            self.origins.pointer_integer_expression(&left, path)?;
        }
        Ok(left)
    }

    pub(super) fn and(&mut self, path: &[String], options: Options) -> Result<SExpr> {
        let mut left = self.comparison(path, options)?;
        while self.cursor.is("&&") {
            let token = self.cursor.advance();
            let before = self.begin_owned_record_short_circuit();
            let right = self.comparison(path, Options::default());
            self.end_owned_record_short_circuit(before);
            let right = right?;
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
            self.guard_borrowed_vector_observation(
                &left,
                path,
                "comparison",
                joined(left.span, right.span, &token),
            )?;
            self.guard_borrowed_vector_observation(
                &right,
                path,
                "comparison",
                joined(left.span, right.span, &token),
            )?;
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
            if let Some(result) = self.standard_collection_comparison(
                &left,
                &right,
                op,
                false,
                path,
                joined(left.span, right.span, &token),
            )? {
                return Ok(result);
            }
            self.refuse_collection_observation(&left, path, "comparison", false)?;
            self.refuse_collection_observation(&right, path, "comparison", false)?;
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
                args: vec![SArrayItem::new(left, false), SArrayItem::new(right, false)],
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
                    args: vec![SArrayItem::new(left, false), SArrayItem::new(right, false)],
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
                self.origins.wrap(&arg, path, "reference")?;
            }
            self.origins.wrap(&arg, path, "reference")?;
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
        let shadowed = self
            .origins
            .lookup_type_owner(&source_owner, &origin.path)
            .is_some()
            || self
                .origins
                .lookup_type_owner(&owner, &origin.path)
                .is_some();
        super::float_predicate_rules::accept_rust_float_predicate_receiver(
            origin.ty.as_ref().map_or("", Type::kind),
            origin.float_source_model.unwrap_or(""),
            origin.proven,
            !origin.wrappers.is_empty(),
            shadowed,
            &arguments_count.to_string(),
        )
    }
}

include!("expression_selection.rs");
