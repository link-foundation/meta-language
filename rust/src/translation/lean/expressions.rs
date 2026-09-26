//! Expressions, `match` alternatives, and patterns of the Lean frontend.

use super::{
    annotate_layout, bind_or_ctor, column, expr, is_item_keyword, operator_name, pattern,
    plain_application, span, syntax_at, LeanParser, PatternHead, BINARY,
};
use crate::translation::diagnostics::{type_error, unsupported, Result};
use crate::translation::lexer::{describe, tokenize_source, Source, Token, TokenKind};
use crate::translation::surface::{
    BinaryOp, Flavor, SExpr, SNode, SPattern, SPatternNode, SRow, ShowStyle, UnaryOp,
};
use crate::translation::types::{INT, NAT};
use crate::translation::{Language, Span};

impl LeanParser<'_> {
    pub(super) fn expr(&mut self) -> Result<SExpr> {
        let token = self.peek();
        if self.cursor.is("if") {
            self.cursor.advance();
            if self.cursor.is_kind(TokenKind::Identifier) && self.cursor.is_at(":", 1) {
                return Err(unsupported(
                    "dependent if",
                    "if h : … is outside the portable core",
                    Some(span(&token, &token)),
                ));
            }
            let cond = self.expr()?;
            self.cursor.expect("then", Some("if"))?;
            let then = self.expr()?;
            self.cursor.expect("else", Some("if"))?;
            let otherwise = self.expr()?;
            return Ok(expr(
                SNode::If {
                    cond: Box::new(cond),
                    then: Box::new(then),
                    otherwise: Box::new(otherwise),
                },
                self.span_to_next(&token),
            ));
        }
        if self.cursor.is("let") {
            self.cursor.advance();
            let name = self.cursor.identifier(Some("let"))?.value;
            let ty = if self.cursor.eat(":").is_some() {
                Some(self.ty()?)
            } else {
                None
            };
            self.cursor.expect(":=", Some("let"))?;
            let value = self.with_bound(column(&token), Self::expr)?;
            self.cursor.eat(";");
            let body = self.expr()?;
            return Ok(expr(
                SNode::Let {
                    name,
                    ty,
                    value: Box::new(value),
                    body: Box::new(body),
                },
                self.span_to_next(&token),
            ));
        }
        if self.cursor.is("match") {
            self.cursor.advance();
            let mut scrutinees = vec![self.expr()?];
            while self.cursor.eat(",").is_some() {
                scrutinees.push(self.expr()?);
            }
            self.cursor.expect("with", Some("match"))?;
            let rows = self.alternatives(scrutinees.len())?;
            return Ok(expr(
                SNode::Match { scrutinees, rows },
                self.span_to_next(&token),
            ));
        }
        if self.cursor.is("fun") || self.cursor.is("λ") {
            return Err(unsupported(
                "lambda",
                "higher-order values are outside the portable core",
                Some(span(&token, &token)),
            ));
        }
        if self.cursor.is("do") {
            return Err(unsupported(
                "do block",
                "monadic code outside main is outside the portable core",
                Some(span(&token, &token)),
            ));
        }
        self.binary(0)
    }

    pub(super) fn alternatives(&mut self, arity: usize) -> Result<Vec<SRow>> {
        let mut rows = Vec::new();
        while self.cursor.is("|") && !self.outdented(self.bound()) {
            let bar = self.cursor.advance();
            let mut patterns = vec![self.pattern()?];
            while self.cursor.eat(",").is_some() {
                patterns.push(self.pattern()?);
            }
            if patterns.len() != arity {
                return Err(syntax_at(
                    format!(
                        "alternative has {} patterns, expected {arity}",
                        patterns.len()
                    ),
                    &bar,
                ));
            }
            self.cursor.expect("=>", Some("alternative"))?;
            let body = self.expr()?;
            rows.push(SRow {
                patterns,
                body,
                span: Some(self.span_to_next(&bar)),
            });
        }
        if rows.is_empty() {
            return Err(self.fail("expected match alternatives"));
        }
        Ok(rows)
    }

    pub(super) fn pattern(&mut self) -> Result<SPattern> {
        let token = self.peek();
        let mut result = self.pattern_app()?;
        while self.cursor.is("+") {
            self.cursor.advance();
            let amount = self.cursor.advance();
            if amount.kind != TokenKind::Number || !amount.suffix.is_empty() {
                return Err(Self::fail_at(
                    "expected a numeral in an n + k pattern",
                    &amount,
                ));
            }
            result = pattern(
                SPatternNode::NatAdd {
                    inner: Box::new(result),
                    add: amount.value.clone(),
                },
                Some(self.span_to_next(&token)),
            );
        }
        Ok(result)
    }

    pub(super) fn pattern_app(&mut self) -> Result<SPattern> {
        let token = self.peek();
        if token.kind == TokenKind::Identifier || self.cursor.is(".") {
            let head = self.pattern_head()?;
            let mut args = Vec::new();
            while self.pattern_argument_start() {
                args.push(self.pattern_atom()?);
            }
            if args.is_empty() && head.path.len() == 1 && !head.dot {
                return Ok(bind_or_ctor(head, &token));
            }
            return Ok(self.ctor_pattern(head, args, &token));
        }
        self.pattern_atom()
    }

    pub(super) fn pattern_argument_start(&self) -> bool {
        let token = self.cursor.peek();
        if self.blocked(token) {
            return false;
        }
        token.kind == TokenKind::Identifier
            || token.kind == TokenKind::Number
            || self.cursor.is("(")
            || self.cursor.is("_")
            || self.cursor.is(".")
    }

    pub(super) fn pattern_head(&mut self) -> Result<PatternHead> {
        let dot = self.cursor.eat(".").is_some();
        let path = self
            .cursor
            .identifier(Some("pattern"))?
            .value
            .split('.')
            .map(str::to_owned)
            .collect();
        Ok(PatternHead { path, dot })
    }

    pub(super) fn ctor_pattern(
        &self,
        head: PatternHead,
        mut args: Vec<SPattern>,
        token: &Token,
    ) -> SPattern {
        let joined = head.path.join(".");
        let name = head.path.last().map_or("", String::as_str);
        if (joined == "Nat.succ" || (head.dot && name == "succ")) && args.len() == 1 {
            return pattern(
                SPatternNode::NatAdd {
                    inner: Box::new(args.remove(0)),
                    add: "1".to_owned(),
                },
                Some(self.span_to_next(token)),
            );
        }
        if (joined == "Nat.zero" || (head.dot && name == "zero")) && args.is_empty() {
            return pattern(
                SPatternNode::NumLit {
                    value: "0".to_owned(),
                    negative: false,
                },
                Some(span(token, token)),
            );
        }
        pattern(
            SPatternNode::Ctor {
                path: head.path,
                args,
            },
            Some(self.span_to_next(token)),
        )
    }

    pub(super) fn pattern_atom(&mut self) -> Result<SPattern> {
        let token = self.peek();
        if self.cursor.eat("_").is_some() {
            return Ok(pattern(SPatternNode::Wild, None));
        }
        if token.kind == TokenKind::Number {
            self.cursor.advance();
            return Ok(pattern(
                SPatternNode::NumLit {
                    value: token.value.clone(),
                    negative: false,
                },
                Some(span(&token, &token)),
            ));
        }
        if self.cursor.eat("(").is_some() {
            let inner = self.pattern()?;
            self.cursor.expect(")", Some("pattern"))?;
            return Ok(inner);
        }
        if token.kind == TokenKind::Identifier || self.cursor.is(".") {
            let head = self.pattern_head()?;
            if head.path.len() == 1 && !head.dot {
                return Ok(bind_or_ctor(head, &token));
            }
            return Ok(self.ctor_pattern(head, Vec::new(), &token));
        }
        Err(unsupported(
            "pattern",
            &format!(
                "{} is outside the portable pattern language",
                describe(&token)
            ),
            Some(span(&token, &token)),
        ))
    }

    pub(super) fn binary(&mut self, min_prec: u32) -> Result<SExpr> {
        let mut left = self.unary()?;
        loop {
            let token = self.cursor.peek();
            if self.blocked(token) || token.kind != TokenKind::Punct {
                return Ok(left);
            }
            let Some(level) = BINARY
                .iter()
                .find(|level| level.ops.contains(&token.value.as_str()))
            else {
                return Ok(left);
            };
            if level.prec < min_prec {
                return Ok(left);
            }
            let token = self.cursor.advance();
            let right = self.binary(if level.right {
                level.prec
            } else {
                level.prec + 1
            })?;
            let op = level
                .op
                .or_else(|| operator_name(&token.value))
                .unwrap_or(BinaryOp::Eq);
            let at = Span::new(
                left.span.map_or(token.start, |at| at.start),
                right.span.map_or(token.end, |at| at.end),
            );
            left = expr(
                SNode::Binary {
                    op,
                    left: Box::new(left),
                    right: Box::new(right),
                    rounding: None,
                },
                at,
            );
        }
    }

    pub(super) fn unary(&mut self) -> Result<SExpr> {
        let token = self.peek();
        let op = if self.cursor.is("-") {
            UnaryOp::Neg
        } else if self.cursor.is("!") || self.cursor.is("not") {
            UnaryOp::Not
        } else {
            return self.application();
        };
        self.cursor.advance();
        let arg = self.unary()?;
        Ok(expr(
            SNode::Unary {
                op,
                arg: Box::new(arg),
            },
            self.span_to_next(&token),
        ))
    }

    pub(super) fn argument_start(&self) -> bool {
        let token = self.cursor.peek();
        if self.blocked(token) {
            return false;
        }
        match token.kind {
            TokenKind::Identifier => {
                ![
                    "then", "else", "with", "do", "at", "if", "let", "match", "fun", "by", "from",
                ]
                .contains(&token.value.as_str())
                    && !is_item_keyword(&token.value)
            }
            TokenKind::Number | TokenKind::String | TokenKind::Interpolation => true,
            _ => ["(", ".", "↑"].contains(&token.value.as_str()),
        }
    }

    pub(super) fn application(&mut self) -> Result<SExpr> {
        let token = self.peek();
        let head = self.atom()?;
        let mut args = Vec::new();
        while self.argument_start() {
            args.push(self.atom()?);
        }
        self.apply_head(head, args, &token)
    }

    pub(super) fn apply_head(&self, head: SExpr, args: Vec<SExpr>, token: &Token) -> Result<SExpr> {
        let range = self.span_to_next(token);
        let name = match &head.node {
            SNode::Name { path } => path.join("."),
            _ => return plain_application(head, args, range),
        };
        let one = |args: Vec<SExpr>| -> Result<Box<SExpr>> {
            let mut args = args.into_iter();
            match (args.next(), args.next()) {
                (Some(arg), None) => Ok(Box::new(arg)),
                _ => Err(type_error(
                    format!("{name} expects one argument"),
                    Some(range),
                )),
            }
        };
        let node = match name.as_str() {
            "toString" => SNode::ToString { arg: one(args)? },
            "Int.toNat" => SNode::Cast {
                arg: one(args)?,
                to: NAT,
                from: Some(INT),
                flavor: Flavor::Clamp,
            },
            "Int.ofNat" => SNode::Cast {
                arg: one(args)?,
                to: INT,
                from: Some(NAT),
                flavor: Flavor::Exact,
            },
            "Nat.succ" => SNode::Binary {
                op: BinaryOp::Add,
                left: one(args)?,
                right: Box::new(SExpr::new(
                    SNode::Num {
                        value: "1".to_owned(),
                        ty: None,
                        negative: false,
                    },
                    None,
                )),
                rounding: None,
            },
            "Nat.zero" => {
                if !args.is_empty() {
                    return Err(type_error("Nat.zero takes no arguments", Some(range)));
                }
                SNode::Num {
                    value: "0".to_owned(),
                    ty: Some(NAT),
                    negative: false,
                }
            }
            _ => {
                if ["Nat.", "Int.", "String.", "Bool."]
                    .iter()
                    .any(|prefix| name.starts_with(prefix))
                {
                    return Err(unsupported(
                        &format!("Lean library function {name}"),
                        "outside the portable core",
                        Some(range),
                    ));
                }
                return plain_application(head, args, range);
            }
        };
        Ok(expr(node, range))
    }

    pub(super) fn atom(&mut self) -> Result<SExpr> {
        let token = self.peek();
        let at = span(&token, &token);
        match token.kind {
            TokenKind::Number => {
                self.cursor.advance();
                if !token.suffix.is_empty() {
                    return Err(unsupported("numeric literal suffix", &token.raw, Some(at)));
                }
                return Ok(expr(
                    SNode::Num {
                        value: token.value,
                        ty: None,
                        negative: false,
                    },
                    at,
                ));
            }
            TokenKind::String => {
                self.cursor.advance();
                return Ok(expr(SNode::Str { value: token.value }, at));
            }
            TokenKind::Interpolation => {
                self.cursor.advance();
                return self.interpolation(&token);
            }
            _ => {}
        }
        if self.cursor.eat("↑").is_some() {
            let arg = self.atom()?;
            return Ok(expr(
                SNode::Cast {
                    arg: Box::new(arg),
                    to: INT,
                    from: Some(NAT),
                    flavor: Flavor::Exact,
                },
                self.span_to_next(&token),
            ));
        }
        if self.cursor.eat(".").is_some() {
            let name = self.cursor.identifier(Some("constructor"))?.value;
            return Ok(expr(SNode::DotCtor { name }, at));
        }
        if self.cursor.eat("(").is_some() {
            if self.cursor.eat(")").is_some() {
                return Ok(expr(SNode::Unit, at));
            }
            let inner = self.with_bound(-1, Self::expr)?;
            if self.cursor.eat(":").is_some() {
                let ty = self.ty()?;
                self.cursor.expect(")", Some("type ascription"))?;
                let ascribed = self.span_to_next(&token);
                if let SNode::Num {
                    value, negative, ..
                } = &inner.node
                {
                    return Ok(expr(
                        SNode::Num {
                            value: value.clone(),
                            ty: Some(ty),
                            negative: *negative,
                        },
                        ascribed,
                    ));
                }
                if let SNode::Unary {
                    op: UnaryOp::Neg,
                    arg,
                } = &inner.node
                {
                    if let SNode::Num { value, .. } = &arg.node {
                        return Ok(expr(
                            SNode::Num {
                                value: value.clone(),
                                ty: Some(ty),
                                negative: true,
                            },
                            ascribed,
                        ));
                    }
                }
                return Ok(expr(
                    SNode::Cast {
                        arg: Box::new(inner),
                        to: ty,
                        from: None,
                        flavor: Flavor::Exact,
                    },
                    ascribed,
                ));
            }
            if self.cursor.is(",") {
                return Err(unsupported(
                    "tuple",
                    "product values are outside the portable core",
                    Some(self.span_to_next(&token)),
                ));
            }
            self.cursor.expect(")", Some("parenthesised expression"))?;
            return Ok(inner);
        }
        if token.kind == TokenKind::Identifier {
            self.cursor.advance();
            if token.value == "true" || token.value == "false" {
                return Ok(expr(
                    SNode::Bool {
                        value: token.value == "true",
                    },
                    at,
                ));
            }
            if token.value == "sorry" {
                return Err(unsupported(
                    "sorry",
                    "incomplete definitions cannot be translated",
                    Some(at),
                ));
            }
            let next = self.cursor.peek();
            if next.value == "." && next.start == token.end {
                let member = self.cursor.peek_at(1);
                return Err(unsupported(
                    "method call",
                    &format!(
                        "{}.{} is outside the portable core",
                        token.value, member.value
                    ),
                    Some(span(&token, member)),
                ));
            }
            return Ok(expr(
                SNode::Name {
                    path: token.value.split('.').map(str::to_owned).collect(),
                },
                at,
            ));
        }
        Err(self.fail("expected an expression"))
    }

    pub(super) fn interpolation(&self, token: &Token) -> Result<SExpr> {
        // `token.raw.slice(3, -1)`: the text between `s!"` and the closing quote.
        let raw: Vec<u16> = token.raw.encode_utf16().collect();
        let raw = if raw.len() >= 4 {
            &raw[3..raw.len() - 1]
        } else {
            &[][..]
        };
        let mut parts = Vec::new();
        let mut text: Vec<u16> = Vec::new();
        let mut index = 0;
        let offset = token.start + 3;
        let unit = |ch: u8| u16::from(ch);
        while index < raw.len() {
            if raw[index] == unit(b'\\') {
                let escaped = raw.get(index + 1).copied();
                let replacement = match escaped.and_then(|unit| u8::try_from(unit).ok()) {
                    Some(b'n') => b'\n',
                    Some(b't') => b'\t',
                    Some(b'\\') => b'\\',
                    Some(b'"') => b'"',
                    Some(b'{') => b'{',
                    _ => {
                        let shown = escaped.map_or_else(
                            || "undefined".to_owned(),
                            |unit| String::from_utf16_lossy(&[unit]),
                        );
                        return Err(syntax_at(format!("unsupported escape \\{shown}"), token));
                    }
                };
                text.push(u16::from(replacement));
                index += 2;
                continue;
            }
            if raw[index] == unit(b'{') {
                let Some(close) = (index..raw.len()).find(|&at| raw[at] == unit(b'}')) else {
                    return Err(syntax_at("unterminated interpolation".to_owned(), token));
                };
                if !text.is_empty() {
                    parts.push(SExpr::new(
                        SNode::Str {
                            value: String::from_utf16_lossy(&text),
                        },
                        None,
                    ));
                }
                text.clear();
                let inner = &raw[index + 1..close];
                let shift = offset + index + 1;
                let mut tokens = tokenize_source(
                    &Source::new(&String::from_utf16_lossy(inner)),
                    Language::Lean,
                )?
                .tokens;
                for inner_token in &mut tokens {
                    inner_token.start += shift;
                    inner_token.end += shift;
                }
                let mut padded = vec![unit(b' '); shift];
                padded.extend_from_slice(inner);
                annotate_layout(&mut tokens, &padded);
                let mut parser = LeanParser::new(tokens, self.source);
                let arg = parser.expr()?;
                if !parser.cursor.at_end() {
                    return Err(parser.fail("unexpected token in interpolation"));
                }
                parts.push(SExpr::new(
                    SNode::Show {
                        arg: Box::new(arg),
                        style: ShowStyle::Lean,
                    },
                    None,
                ));
                index = close + 1;
                continue;
            }
            text.push(raw[index]);
            index += 1;
        }
        if !text.is_empty() || parts.is_empty() {
            parts.push(SExpr::new(
                SNode::Str {
                    value: String::from_utf16_lossy(&text),
                },
                None,
            ));
        }
        let mut parts = parts.into_iter();
        let first = parts
            .next()
            .unwrap_or_else(|| SExpr::new(SNode::Unit, None));
        Ok(parts.fold(first, |left, right| {
            expr(
                SNode::Binary {
                    op: BinaryOp::Concat,
                    left: Box::new(left),
                    right: Box::new(right),
                    rounding: None,
                },
                span(token, token),
            )
        }))
    }
}
