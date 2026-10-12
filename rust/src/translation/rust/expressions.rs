//! Expressions from operator precedence down to paths, macros, `if` and `match`.

use super::indirection_rules::accept_rust_source_box_owner;
use super::{
    Alias, BinaryOp, Flavor, Options, Result, RustParser, SExpr, SNode, SPattern, SPatternNode,
    SRow, STRING, SourceOrigin, Span, Token, TokenKind, UnaryOp, additive_op, binary,
    comparison_op, describe, format, is_if_or_match, joined, method_call, multiplicative_op,
    operator, pattern_value, rust_fixed_type, rust_macro_message, span, type_error, unsupported,
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
                return Ok(SExpr::new(
                    SNode::Unary {
                        op,
                        arg: Box::new(arg),
                    },
                    span(&token, self.cursor.peek()),
                ));
            }
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
                let mut origin = self.origins.origin(&value, path);
                let method_name = method.value.as_str();
                if ["div_euclid", "rem_euclid"].contains(&method_name) {
                    if let Some(argument) = args.first() {
                        self.origins.direct(argument, path, argument.span)?;
                    }
                }
                if ["clone", "to_owned"].contains(&method_name)
                    && origin
                        .wrappers
                        .first()
                        .is_some_and(|layer| layer == "reference")
                {
                    origin.wrappers.remove(0);
                }
                if method_name == "as_str" {
                    origin.ty = Some(STRING);
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
            first_token.value = first.to_owned();
            first_token.raw = first.to_owned();
            first_token.suffix.clear();
            first_token.floating = false;
            first_token.end = middle;
            let mut separator = token.clone();
            separator.kind = TokenKind::Punct;
            separator.value = ".".to_owned();
            separator.raw = ".".to_owned();
            separator.suffix.clear();
            separator.floating = false;
            separator.start = middle;
            separator.end = middle + 1;
            let mut second_token = token.clone();
            second_token.value = second.to_owned();
            second_token.raw = second.to_owned();
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
            return Ok(SExpr::new(
                SNode::Num {
                    value: token.value.clone(),
                    ty,
                    negative: false,
                    unit: false,
                },
                span(&token, &token),
            ));
        }
        if token.kind == TokenKind::String {
            self.cursor.advance();
            let result = SExpr::new(
                SNode::Str {
                    value: token.value.clone(),
                },
                span(&token, &token),
            );
            self.origins.mark(
                &result,
                SourceOrigin::with_type(STRING, path, vec!["reference".to_owned()]),
            );
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

    pub(super) fn expression_macro(&mut self, token: &Token, path: &[String]) -> Result<SExpr> {
        let args = self.macro_arguments(token, path)?;
        let range = span(token, self.cursor.peek());
        match token.value.as_str() {
            "format" => {
                let mut formatted = format(args, token)?;
                formatted.span = range;
                Ok(formatted)
            }
            "panic" | "unreachable" | "unimplemented" | "todo" => {
                if args.len() > 1 {
                    return Err(unsupported(
                        &format!("{}! arguments", token.value),
                        "only a literal message is portable",
                        range,
                    ));
                }
                let text = match args.into_iter().next() {
                    None => None,
                    Some(SExpr {
                        node: SNode::Str { value },
                        span,
                        ..
                    }) => Some(panic_text(&value, span)?),
                    Some(_) => {
                        return Err(unsupported(
                            &format!("{}! message", token.value),
                            "the message must be a literal",
                            range,
                        ));
                    }
                };
                let message = rust_macro_message(&token.value, text.as_deref());
                Ok(SExpr::new(SNode::Abort { message }, range))
            }
            _ => Err(unsupported(
                &format!("{}!", token.value),
                "outside the portable core",
                range,
            )),
        }
    }

    pub(super) fn if_expr(&mut self, path: &[String]) -> Result<SExpr> {
        let token = self.cursor.advance();
        if self.cursor.is("let") {
            return Err(unsupported(
                "if let",
                "write a match",
                span(&token, self.cursor.peek()),
            ));
        }
        let cond = self.expr(
            path,
            Options {
                statement: false,
                no_struct: true,
            },
        )?;
        self.origins.direct(&cond, path, cond.span)?;
        let then = self.block(path)?;
        if self.cursor.eat("else").is_none() {
            return Err(unsupported(
                "if without else",
                "the value of an if must be defined on both branches",
                span(&token, self.cursor.peek()),
            ));
        }
        let otherwise = if self.cursor.is("if") {
            self.if_expr(path)?
        } else {
            self.block(path)?
        };
        Ok(SExpr::new(
            SNode::If {
                cond: Box::new(cond),
                then: Box::new(then),
                otherwise: Box::new(otherwise),
            },
            span(&token, self.cursor.peek()),
        ))
    }

    pub(super) fn match_expr(&mut self, path: &[String]) -> Result<SExpr> {
        let token = self.cursor.advance();
        let scrutinee = self.expr(
            path,
            Options {
                statement: false,
                no_struct: true,
            },
        )?;
        self.cursor.expect("{", Some("match"))?;
        let mut rows = Vec::new();
        while !self.cursor.is("}") {
            let arm_start = self.peek();
            let mut aliases = Vec::new();
            self.cursor.eat("|");
            let mut alternatives = vec![self.pattern(path, &mut aliases)?];
            while self.cursor.eat("|").is_some() {
                alternatives.push(self.pattern(path, &mut aliases)?);
            }
            if self.cursor.is("if") {
                return Err(unsupported(
                    "match guard",
                    "guards are outside the portable pattern language",
                    span(&arm_start, self.cursor.peek()),
                ));
            }
            self.cursor.expect("=>", Some("match arm"))?;
            let scope = self.origins.cases(&alternatives, &scrutinee, path)?;
            let previous = self.origins.enter(Some(scope));
            for alias in &aliases {
                self.origins.bind(&alias.name, &alias.value, path, None)?;
            }
            let mut body = self.expr(path, Options::default())?;
            let body_origin = self.origins.origin(&body, path);
            self.origins.scope = previous;
            if body.block || is_if_or_match(&body) {
                self.cursor.eat(",");
            } else if !self.cursor.is("}") {
                self.cursor.expect(",", Some("match arm"))?;
            }
            if alternatives.len() > 1 && !aliases.is_empty() {
                return Err(unsupported(
                    "binding in an or-pattern",
                    "bind the value in each alternative separately",
                    span(&arm_start, self.cursor.peek()),
                ));
            }
            for alias in aliases.into_iter().rev() {
                let body_span = body.span;
                body = SExpr::new(
                    SNode::Let {
                        name: alias.name,
                        ty: None,
                        value: Box::new(alias.value),
                        body: Box::new(body),
                    },
                    body_span,
                );
            }
            self.origins.mark(&body, body_origin);
            // `A | B => e` is one row per alternative with the same body.
            let row_span = span(&arm_start, self.cursor.peek());
            for pattern in alternatives {
                rows.push(SRow {
                    patterns: vec![pattern],
                    body: body.clone(),
                    span: row_span,
                });
            }
        }
        self.cursor.expect("}", Some("match"))?;
        if rows.is_empty() {
            return Err(unsupported(
                "empty match",
                "uninhabited types are outside the portable core",
                span(&token, self.cursor.peek()),
            ));
        }
        let origins = rows
            .iter()
            .map(|row| self.origins.origin(&row.body, path))
            .collect::<Vec<_>>();
        let origin = self.origins.join(&origins);
        let result = SExpr::new(
            SNode::Match {
                scrutinees: vec![scrutinee],
                rows,
            },
            span(&token, self.cursor.peek()),
        );
        self.origins.mark(&result, origin);
        Ok(result)
    }

    /// Patterns: `_`, bindings, literals, `Enum::Variant(p, …)`, `&p` and `name @ p`.
    #[allow(clippy::too_many_lines)] // one branch per pattern form, as in rust.js
    pub(super) fn pattern(
        &mut self,
        path: &[String],
        aliases: &mut Vec<Alias>,
    ) -> Result<SPattern> {
        let token = self.peek();
        if self.cursor.eat("&").is_some() {
            let pattern = self.pattern(path, aliases)?;
            self.origins.reference_pattern(&pattern);
            return Ok(pattern);
        }
        if self.cursor.eat("_").is_some() {
            self.origins.wildcard_pattern();
            return Ok(SPattern {
                node: SPatternNode::Wild,
                span: None,
            });
        }
        if self.cursor.is("ref") || self.cursor.is("mut") {
            return Err(unsupported(
                &format!("{} binding", token.value),
                "bindings are by value in the portable core",
                span(&token, &token),
            ));
        }
        if self.cursor.is("-") && self.cursor.is_kind_at(TokenKind::Number, 1) {
            self.cursor.advance();
            let number = self.cursor.advance();
            return Ok(SPattern {
                node: SPatternNode::NumLit {
                    value: number.value.clone(),
                    negative: true,
                },
                span: span(&token, &number),
            });
        }
        if token.kind == TokenKind::Number {
            self.cursor.advance();
            if self.cursor.is("..") || self.cursor.is("..=") {
                return Err(unsupported(
                    "range pattern",
                    "ranges are outside the portable pattern language",
                    span(&token, self.cursor.peek()),
                ));
            }
            return Ok(SPattern {
                node: SPatternNode::NumLit {
                    value: token.value.clone(),
                    negative: false,
                },
                span: span(&token, &token),
            });
        }
        if token.kind == TokenKind::String {
            return Err(unsupported(
                "string pattern",
                "match on &str is outside the Rust frontend",
                span(&token, &token),
            ));
        }
        if self.cursor.is("true") || self.cursor.is("false") {
            self.cursor.advance();
            return Ok(SPattern {
                node: SPatternNode::BoolLit {
                    value: token.value == "true",
                    negative: false,
                },
                span: span(&token, &token),
            });
        }
        if self.cursor.is("(") {
            return Err(unsupported(
                "tuple match pattern",
                "tuple match coverage is not represented; use an immutable destructuring let",
                span(&token, &token),
            ));
        }
        if token.kind != TokenKind::Identifier {
            return Err(unsupported(
                "pattern",
                &format!(
                    "{} is outside the portable pattern language",
                    describe(&token)
                ),
                span(&token, &token),
            ));
        }
        let segments = self.path_segments("pattern")?;
        if self.cursor.eat("@").is_some() {
            if segments.len() != 1 {
                return Err(Self::fail("expected a binding before @", &token));
            }
            let inner = self.pattern(path, aliases)?;
            let value = pattern_value(&inner, span(&token, self.cursor.peek()))?;
            aliases.push(Alias {
                name: segments[0].clone(),
                value,
            });
            return Ok(inner);
        }
        if self.cursor.is("{") {
            return Err(unsupported(
                "struct pattern",
                "named fields are outside the Rust frontend",
                span(&token, self.cursor.peek()),
            ));
        }
        let resolved = self.resolve(&segments, path);
        if self.cursor.eat("(").is_some() {
            let mut args = Vec::new();
            while !self.cursor.is(")") {
                if self.cursor.is("..") {
                    let rest = self.cursor.peek();
                    return Err(unsupported(
                        "rest pattern",
                        "name every field",
                        span(rest, rest),
                    ));
                }
                args.push(self.pattern(path, aliases)?);
                if self.cursor.eat(",").is_none() {
                    break;
                }
            }
            self.cursor.expect(")", Some("pattern"))?;
            return Ok(SPattern {
                node: SPatternNode::Ctor {
                    path: resolved,
                    args,
                },
                span: span(&token, self.cursor.peek()),
            });
        }
        if segments.len() == 1 && resolved.len() == 1 {
            return Ok(SPattern {
                node: SPatternNode::BindOrCtor {
                    name: segments[0].clone(),
                },
                span: span(&token, &token),
            });
        }
        Ok(SPattern {
            node: SPatternNode::Ctor {
                path: resolved,
                args: Vec::new(),
            },
            span: span(&token, &token),
        })
    }
}

/// The text a `panic!` message literal prints: `{{` and `}}` are braces; a lone brace captures a variable.
fn panic_text(template: &str, span: Option<Span>) -> Result<String> {
    let mut text = String::new();
    let mut chars = template.chars().peekable();
    while let Some(char) = chars.next() {
        if (char == '{' || char == '}') && chars.next_if_eq(&char).is_none() {
            return Err(unsupported(
                "formatted panic message",
                "only a literal message is portable",
                span,
            ));
        }
        text.push(char);
    }
    Ok(text)
}
