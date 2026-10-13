//! Expression precedence, immutable reads, primary values and qualified paths.

mod control;
mod read_only_queries;

use super::array_rules::{accept_rust_source_array_element, accept_rust_source_array_owner};
use super::indirection_rules::accept_rust_source_box_owner;
use super::{
    Alias, BOOL, BinaryOp, FLOAT, Flavor, Options, Result, RustParser, SArrayItem, SExpr, SNode,
    SPattern, SPatternNode, SRow, STRING, SourceOrigin, Span, Token, TokenKind, Type, UnaryOp,
    additive_op, binary, comparison_op, describe, format, is_if_or_match, joined, method_call,
    multiplicative_op, operator, pattern_value, rust_fixed_type, rust_macro_message, span,
    type_error, unsupported,
};

impl RustParser {
    /// Expressions, by Rust precedence.
    pub(super) fn expr(&mut self, path: &[String], options: Options) -> Result<SExpr> {
        self.or(path, options)
    }

    pub(super) fn or(&mut self, path: &[String], options: Options) -> Result<SExpr> {
        let mut left = self.and(path, options)?;
        while self.cursor.is("||") {
            let token = self.cursor.advance();
            let right = self.and(path, Options::default())?;
            left = binary(BinaryOp::Or, left, right, &token);
        }
        Ok(left)
    }

    pub(super) fn and(&mut self, path: &[String], options: Options) -> Result<SExpr> {
        let mut left = self.comparison(path, options)?;
        while self.cursor.is("&&") {
            let token = self.cursor.advance();
            let right = self.comparison(path, Options::default())?;
            left = binary(BinaryOp::And, left, right, &token);
        }
        Ok(left)
    }

    pub(super) fn comparison(&mut self, path: &[String], options: Options) -> Result<SExpr> {
        let left = self.additive(path, options)?;
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
            let right = self.additive(path, Options::default())?;
            let after = self.cursor.peek();
            if after.kind == TokenKind::Punct && comparison_op(&after.value).is_some() {
                return Err(Self::fail("comparison operators cannot be chained", after));
            }
            return Ok(binary(op, left, right, &token));
        }
        Ok(left)
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
            left = binary(op, left, right, &token);
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
            self.scalar_loop_arithmetic(&left, path);
        }
    }

    /// `e as T` converts between integer types when every value fits.
    pub(super) fn cast(&mut self, path: &[String], options: Options) -> Result<SExpr> {
        let mut value = self.unary(path, options)?;
        while self.cursor.is("as") {
            let token = self.cursor.advance();
            let to = self.parse_type()?;
            if !self.origins.collecting && !self.origins.current_type.wrappers.is_empty() {
                return Err(unsupported(
                    "source cast",
                    "numeric casts require a direct source and target value type",
                    span(&token, self.cursor.peek()),
                ));
            }
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
        }
        Ok(value)
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
                if op == UnaryOp::Neg && self.accept_float_receiver_origin(&origin, 0) {
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
                if !self.cursor.is("(") {
                    return Err(unsupported(
                        "field access",
                        &format!("field {} is outside the portable core", method.value),
                        span(&token, &method),
                    ));
                }
                self.cursor.advance();
                let args = self.call_arguments(path)?;
                self.cursor.expect(")", Some(&method.value))?;
                self.require_type_namespace_body_method(
                    &value,
                    &method.value,
                    span(&token, self.cursor.peek()),
                )?;
                let mut origin = self.origins.origin(&value, path);
                let method_name = method.value.as_str();
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
                            .mark(&value, SourceOrigin::with_type(BOOL, path, Vec::new()));
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
            if token.suffix == "f64" {
                self.origins
                    .mark(&result, SourceOrigin::with_type(FLOAT, path, Vec::new()));
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
        let c = &self.cursor;
        if c.is("'") {
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
                        frozen: true,
                        product_model: Some("immutable value product".to_owned()),
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
            return Err(unsupported(
                "closure",
                "higher-order values are outside the portable core",
                span(&token, &token),
            ));
        }
        if c.is("true") || c.is("false") {
            self.cursor.advance();
            return Ok(SExpr::new(
                SNode::Bool {
                    value: token.value == "true",
                },
                span(&token, &token),
            ));
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

    /// A name, a call of a function or constructor, or a conversion (`u64::from(x)`).
    pub(super) fn path_expression(
        &mut self,
        token: &Token,
        path: &[String],
        options: Options,
    ) -> Result<SExpr> {
        let segments = self.path_segments("expression")?;
        let name = segments.join("::");
        let resolved = self.resolve(&segments, path);
        if !self.primitive_owner_model.is_empty()
            && segments.len() > 1
            && super::float_predicate_rules::accept_rust_float_constant_name(
                segments.last().unwrap(),
            )
            && let Some(owner) =
                self.read_standard_primitive_position(&segments[..segments.len() - 1], path)?
        {
            let range = Some(Span::new(
                token.start,
                self.cursor.tokens[self.cursor.index - 1].end,
            ));
            if self.cursor.is("(") || self.origins.scope.contains_key(&segments[0]) {
                return Err(unsupported(
                    "Rust primitive constant owner",
                    "an associated constant needs its actual unshadowed type path and value syntax",
                    range,
                ));
            }
            let name = segments.last().unwrap();
            let result = super::float_predicates::constant_expression(name, range)?;
            self.primitive_constant_sites
                .push(serde_json::json!({"owner":owner,"name":name,"span":range}));
            let mut origin = SourceOrigin::with_type(FLOAT, path, Vec::new());
            origin.source_owner = Some(crate::translation::source_signatures::SourceOwner {
                kind: "primitive".to_owned(),
                path: vec!["f64".to_owned()],
                arguments: Vec::new(),
            });
            origin.primitive_owner_position = Some(owner);
            self.origins.mark(&result, origin);
            return Ok(result);
        }
        if segments.len() == 2
            && segments[0] == "f64"
            && super::float_predicate_rules::accept_rust_float_constant_name(&segments[1])
        {
            let range = Some(Span::new(
                token.start,
                self.cursor.tokens[self.cursor.index - 1].end,
            ));
            let source_owner = vec!["f64".to_owned()];
            let owner = self.resolve(&source_owner, path);
            let shadowed = self.origins.lookup(&source_owner, path).is_some()
                || self.origins.lookup(&owner, path).is_some();
            if !super::float_predicate_rules::accept_rust_float_constant_owner(
                &owner.join("::"),
                shadowed,
            ) || self.cursor.is("(")
            {
                return Err(unsupported(
                    "Rust binary64 constant",
                    "associated constants require the exact unshadowed primitive f64 owner and value syntax",
                    range,
                ));
            }
            let result = super::float_predicates::constant_expression(&segments[1], range)?;
            self.origins
                .mark(&result, SourceOrigin::with_type(FLOAT, path, Vec::new()));
            return Ok(result);
        }
        if self.cursor.is("::")
            && self.cursor.is_at("<", 1)
            && (accept_rust_source_array_owner(&name, false)
                || accept_rust_source_array_owner(&resolved.join("::"), false))
        {
            self.origins.vector_owner(
                &segments,
                &resolved,
                path,
                span(token, self.cursor.peek()),
            )?;
            self.cursor.advance();
            self.cursor.advance();
            let element_token = self.peek();
            let element = self.parse_type()?;
            self.array_element(token, &element_token)?;
            self.cursor.expect(">", Some("Vec constructor"))?;
            self.cursor.expect("::", Some("Vec constructor"))?;
            self.cursor.expect("new", Some("Vec constructor"))?;
            self.cursor.expect("(", Some("Vec constructor"))?;
            self.cursor.expect(")", Some("Vec constructor"))?;
            let result = SExpr::new(
                SNode::Array {
                    items: Vec::new(),
                    element: Some(element.clone()),
                },
                span(token, self.cursor.peek()),
            );
            self.origins.mark(
                &result,
                SourceOrigin::with_type(
                    Type::Array {
                        element: Box::new(element),
                        reference: false,
                    },
                    path,
                    Vec::new(),
                ),
            );
            return Ok(result);
        }
        let c = &self.cursor;
        if c.is("::") && c.is_at("<", 1) {
            return Err(unsupported(
                "turbofish",
                "generic functions are outside the portable core",
                span(token, c.peek()),
            ));
        }
        let last_is_upper = segments
            .last()
            .and_then(|segment| segment.chars().next())
            .is_some_and(|first| first.is_ascii_uppercase());
        if c.is("{") && last_is_upper && !options.no_struct {
            return Err(unsupported(
                "struct literal",
                "named fields are outside the Rust frontend",
                span(token, c.peek()),
            ));
        }
        if !c.is("(") {
            return Ok(SExpr::new(
                SNode::Name {
                    path: self.resolve(&segments, path),
                },
                span(token, token),
            ));
        }
        self.cursor.advance();
        let mut args = self.call_arguments(path)?;
        self.cursor.expect(")", Some("call"))?;
        let range = span(token, self.cursor.peek());
        if let Some(empty) =
            self.read_empty_string_constructor(&segments, args.len(), path, range)?
        {
            return Ok(empty);
        }
        let owner = &segments[..segments.len() - 1];
        let resolved_owner = if owner.is_empty() {
            Vec::new()
        } else {
            self.resolve(owner, path)
        };
        if segments.last().is_some_and(|segment| segment == "new")
            && (accept_rust_source_box_owner(&owner.join("::"), false)
                || accept_rust_source_box_owner(&resolved_owner.join("::"), false))
        {
            if args.len() != 1 {
                return Err(type_error("Box::new expects 1 argument", range));
            }
            self.origins
                .box_owner(owner, &resolved_owner, path, range)?;
            let result = args.remove(0);
            self.origins.wrap(&result, path, "box");
            return Ok(result);
        }
        if name == "String::from" {
            self.require_type_namespace_body_constructor(&name, range)?;
            if args.len() != 1 {
                return Err(type_error("String::from expects 1 argument", range));
            }
            let result = SExpr::new(
                SNode::ToString {
                    arg: Box::new(args.remove(0)),
                },
                range,
            );
            self.origins.conversion(&result, path);
            return Ok(result);
        }
        let fixed = rust_fixed_type(&segments[0]);
        if segments.len() == 2
            && segments[1] == "from"
            && let Some(to) = fixed.clone()
        {
            self.require_type_namespace_body_constructor(&name, range)?;
            if args.len() != 1 {
                return Err(type_error(format!("{name} expects 1 argument"), range));
            }
            return Ok(SExpr::new(
                SNode::Cast {
                    arg: Box::new(args.remove(0)),
                    to,
                    from: None,
                    flavor: Flavor::Exact,
                },
                range,
            ));
        }
        if segments.len() == 2 && fixed.is_some() {
            return Err(unsupported(&name, "outside the portable core", range));
        }
        if ["std", "core", "alloc"].contains(&segments[0].as_str()) {
            return Err(unsupported(
                &name,
                "the standard library is outside the portable core",
                range,
            ));
        }
        let callee = self.resolve(&segments, path);
        self.origins.call(&callee, &args, path, range)?;
        if args.is_empty() {
            return Ok(SExpr::new(SNode::Name { path: callee }, range));
        }
        Ok(SExpr::new(
            SNode::App {
                func: Box::new(SExpr::new(SNode::Name { path: callee }, span(token, token))),
                args,
            },
            range,
        ))
    }
}
