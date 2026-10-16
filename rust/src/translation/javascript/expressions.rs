//! Expressions from operator precedence down to literals and references.

use super::{
    ASSIGNMENTS, BinaryOp, External, FLOAT, GLOBALS, JavaScriptParser, Language, ROOT, Result,
    SArrayItem, SData, SExpr, SNode, SPattern, SPatternNode, SRow, STagTest, ShowStyle, Span,
    Token, TokenCursor, TokenKind, UnaryOp, binary, has_ctor, is_tag_field, joined, node,
    number_literal, span, tokenize, type_error, unsupported, wild,
};

mod values;

impl JavaScriptParser {
    /// Expressions, by JavaScript precedence.
    pub(super) fn expr(&mut self) -> Result<SExpr> {
        let start = self.peek();
        if start.kind == TokenKind::Identifier && self.cursor.is_at("=>", 1) {
            return self.read_closure(false);
        }
        let cond = self.or()?;
        let next = self.cursor.peek();
        if next.kind == TokenKind::Punct && ASSIGNMENTS.contains(&next.value.as_str()) {
            return Err(unsupported(
                "assignment",
                "mutation is outside the portable core",
                Some(self.to_here(&start)),
            ));
        }
        if self.cursor.eat("?").is_none() {
            return Ok(cond);
        }
        let then = self.expr()?;
        self.cursor.expect(":", Some("conditional expression"))?;
        let otherwise = self.expr()?;
        let place = joined(&cond, &otherwise, &start);
        Ok(node(
            SNode::If {
                cond: Box::new(cond),
                then: Box::new(then),
                otherwise: Box::new(otherwise),
            },
            place,
        ))
    }

    pub(super) fn or(&mut self) -> Result<SExpr> {
        let mut left = self.and()?;
        let mut coalesced = false;
        loop {
            let token = self.peek();
            if self.cursor.eat("??").is_some() {
                let operation = match &left.node {
                    SNode::Binary { op, .. } => op.name(),
                    _ => "",
                };
                let parenthesized = left
                    .parenthesized
                    .as_ref()
                    .and_then(|value| value.as_boolean())
                    .unwrap_or(false);
                let error =
                    crate::translation::optional_value_rules::read_optional_logical_mixing_error(
                        "left",
                        coalesced,
                        parenthesized,
                        left.kind(),
                        operation,
                    );
                if !error.is_empty() {
                    return Err(Self::fail(&error, &token));
                }
                let right = self.and()?;
                let operation = match &right.node {
                    SNode::Binary { op, .. } => op.name(),
                    _ => "",
                };
                let parenthesized = right
                    .parenthesized
                    .as_ref()
                    .and_then(|value| value.as_boolean())
                    .unwrap_or(false);
                let error =
                    crate::translation::optional_value_rules::read_optional_logical_mixing_error(
                        "right",
                        coalesced,
                        parenthesized,
                        right.kind(),
                        operation,
                    );
                if !error.is_empty() {
                    return Err(Self::fail(&error, &token));
                }
                let place = joined(&left, &right, &token);
                left = node(
                    SNode::Coalesce {
                        left: Box::new(left),
                        right: Box::new(right),
                    },
                    place,
                );
                coalesced = true;
                continue;
            }
            if self.cursor.eat("||").is_none() {
                return Ok(left);
            }
            let error =
                crate::translation::optional_value_rules::read_optional_logical_mixing_error(
                    "or", coalesced, false, "", "",
                );
            if !error.is_empty() {
                return Err(Self::fail(&error, &token));
            }
            let right = self.and()?;
            left = binary(BinaryOp::Or, left, right, &token);
        }
    }

    pub(super) fn and(&mut self) -> Result<SExpr> {
        let mut left = self.bitwise()?;
        while self.cursor.is("&&") {
            let token = self.cursor.advance();
            let right = self.bitwise()?;
            left = binary(BinaryOp::And, left, right, &token);
        }
        Ok(left)
    }

    pub(super) fn bitwise(&mut self) -> Result<SExpr> {
        let left = self.equality()?;
        let token = self.peek();
        if token.kind == TokenKind::Punct && ["|", "^", "&"].contains(&token.value.as_str()) {
            return Err(unsupported(
                &format!("bitwise {}", token.value),
                "bitwise operators are outside the portable core",
                Some(span(&token, &token)),
            ));
        }
        Ok(left)
    }

    pub(super) fn equality(&mut self) -> Result<SExpr> {
        let mut left = self.relational()?;
        loop {
            let token = self.peek();
            if self.cursor.is("==") || self.cursor.is("!=") {
                return Err(unsupported(
                    &format!("loose equality {}", token.value),
                    "use === or !==",
                    Some(span(&token, &token)),
                ));
            }
            let op = match (token.kind, token.value.as_str()) {
                (TokenKind::Punct, "===") => BinaryOp::Eq,
                (TokenKind::Punct, "!==") => BinaryOp::Ne,
                _ => return Ok(left),
            };
            self.cursor.advance();
            let right = self.relational()?;
            left = if is_tag_field(&left) || is_tag_field(&right) {
                self.tag_test(op, left, right, &token)?
            } else {
                binary(op, left, right, &token)
            };
        }
    }

    /// `x.$ === 'tag'` tests the alternative of a data value.
    pub(super) fn tag_test(
        &self,
        op: BinaryOp,
        left: SExpr,
        right: SExpr,
        token: &Token,
    ) -> Result<SExpr> {
        let place = joined(&left, &right, token);
        let (field, tag) = if is_tag_field(&left) {
            (left, right)
        } else {
            (right, left)
        };
        let SNode::Field { object, .. } = field.node else {
            return Ok(field);
        };
        let SNode::Str { value: tag } = tag.node else {
            return Err(unsupported(
                "computed tag test",
                "compare the $ tag with a string literal",
                Some(place),
            ));
        };
        let candidates: Vec<&SData> = self
            .data_types
            .iter()
            .filter(|data| has_ctor(data, &tag))
            .collect();
        let data = match candidates.as_slice() {
            [data] => (*data).clone(),
            [] => {
                return Err(type_error(
                    format!("no @typedef has tag {tag}"),
                    Some(place),
                ));
            }
            _ => {
                return Err(type_error(
                    format!("tag {tag} is ambiguous between data types"),
                    Some(place),
                ));
            }
        };
        let arity = data
            .ctors
            .iter()
            .find(|ctor| ctor.name == tag)
            .map_or(0, |ctor| ctor.fields.len());
        let pattern = SPattern {
            node: SPatternNode::Ctor {
                path: vec![ROOT.to_owned(), data.name.clone(), tag.clone()],
                args: (0..arity).map(|_| wild(place)).collect(),
            },
            span: Some(place),
        };
        let equal = op == BinaryOp::Eq;
        let rows = vec![
            SRow {
                patterns: vec![pattern],
                body: node(SNode::Bool { value: equal }, place),
                span: Some(place),
            },
            SRow {
                patterns: vec![wild(place)],
                body: node(SNode::Bool { value: !equal }, place),
                span: Some(place),
            },
        ];
        let mut expr = node(
            SNode::Match {
                scrutinees: vec![object.as_ref().clone()],
                rows,
            },
            place,
        );
        expr.tag_test = Some(Box::new(STagTest {
            object: *object,
            tag,
            data,
            negated: op == BinaryOp::Ne,
        }));
        Ok(expr)
    }

    pub(super) fn relational(&mut self) -> Result<SExpr> {
        let mut left = self.shift()?;
        loop {
            let token = self.peek();
            if self.cursor.is("in") || self.cursor.is("instanceof") {
                return Err(unsupported(
                    &format!("{} operator", token.value),
                    "objects are outside the portable core",
                    Some(span(&token, &token)),
                ));
            }
            let op = match (token.kind, token.value.as_str()) {
                (TokenKind::Punct, "<") => BinaryOp::Lt,
                (TokenKind::Punct, "<=") => BinaryOp::Le,
                (TokenKind::Punct, ">") => BinaryOp::Gt,
                (TokenKind::Punct, ">=") => BinaryOp::Ge,
                _ => return Ok(left),
            };
            self.cursor.advance();
            let right = self.shift()?;
            left = binary(op, left, right, &token);
        }
    }

    pub(super) fn shift(&mut self) -> Result<SExpr> {
        let left = self.additive()?;
        let token = self.peek();
        if self.cursor.is("<<") || self.cursor.is(">>") || self.cursor.is(">>>") {
            return Err(unsupported(
                &format!("shift {}", token.value),
                "bit shifts are outside the portable core",
                Some(span(&token, &token)),
            ));
        }
        Ok(left)
    }

    /// `+` adds numbers and concatenates strings; the checker decides which by the operand types.
    pub(super) fn additive(&mut self) -> Result<SExpr> {
        let mut left = self.multiplicative()?;
        loop {
            let token = self.peek();
            if !self.cursor.is("+") && !self.cursor.is("-") {
                return Ok(left);
            }
            self.cursor.advance();
            let right = self.multiplicative()?;
            let op = if token.value == "+" {
                BinaryOp::Plus
            } else {
                BinaryOp::Sub
            };
            left = binary(op, left, right, &token);
        }
    }

    pub(super) fn multiplicative(&mut self) -> Result<SExpr> {
        let mut left = self.exponent()?;
        loop {
            let token = self.peek();
            let op = match (token.kind, token.value.as_str()) {
                (TokenKind::Punct, "*") => BinaryOp::Mul,
                (TokenKind::Punct, "/") => BinaryOp::Div,
                (TokenKind::Punct, "%") => BinaryOp::Rem,
                _ => return Ok(left),
            };
            self.cursor.advance();
            let right = self.exponent()?;
            left = binary(op, left, right, &token);
        }
    }

    pub(super) fn exponent(&mut self) -> Result<SExpr> {
        let left = self.unary()?;
        if self.cursor.is("**") {
            let token = self.peek();
            return Err(unsupported(
                "exponentiation",
                "powers are outside the portable core; use recursion",
                Some(span(&token, &token)),
            ));
        }
        Ok(left)
    }

    pub(super) fn unary(&mut self) -> Result<SExpr> {
        let token = self.peek();
        if token.kind == TokenKind::Identifier && self.cursor.eat("typeof").is_some() {
            let arg = self.unary()?;
            let place = Span::new(token.start, arg.span.map_or(token.end, |arg| arg.end));
            return Ok(node(SNode::TypeOf { arg: Box::new(arg) }, place));
        }
        for (symbol, op) in [("!", UnaryOp::Not), ("-", UnaryOp::Neg)] {
            if self.cursor.eat(symbol).is_some() {
                let arg = self.unary()?;
                let place = Span::new(token.start, arg.span.map_or(token.end, |arg| arg.end));
                return Ok(node(
                    SNode::Unary {
                        op,
                        arg: Box::new(arg),
                    },
                    place,
                ));
            }
        }
        let here = Some(span(&token, &token));
        if self.cursor.is("+") {
            return Err(unsupported(
                "unary +",
                "unary plus throws a TypeError on BigInt values",
                here,
            ));
        }
        if self.cursor.is("~") {
            return Err(unsupported(
                "bitwise ~",
                "bitwise operators are outside the portable core",
                here,
            ));
        }
        if self.cursor.is("++") || self.cursor.is("--") {
            return Err(unsupported(
                &format!("{} operator", token.value),
                "mutation is outside the portable core",
                here,
            ));
        }
        if token.kind == TokenKind::Identifier && token.value == "await" {
            if !self.in_async {
                return Err(unsupported(
                    "await outside an async function",
                    "await is only valid in async functions and at the top level of a module",
                    here,
                ));
            }
            self.cursor.advance();
            self.sequential_async = true;
            return self.awaited(Self::unary);
        }
        if token.kind == TokenKind::Identifier
            && ["typeof", "void", "delete", "await", "yield"].contains(&token.value.as_str())
        {
            return Err(unsupported(
                &format!("{} operator", token.value),
                "outside the portable core",
                here,
            ));
        }
        self.postfix()
    }

    /// An expression whose value is awaited: when it is a call of an async
    /// function, that call is awaited where it is made, which is its ordinary
    /// call. Any other value is awaited as itself.
    pub(super) fn awaited(&mut self, read: fn(&mut Self) -> Result<SExpr>) -> Result<SExpr> {
        let before = self.unawaited.len();
        let expr = read(self)?;
        let path = match &expr.node {
            SNode::App { func, .. } => match &func.node {
                SNode::Name { path } => Some(path),
                _ => None,
            },
            SNode::Name { path } => Some(path),
            _ => None,
        };
        let awaits_last = match (path, self.unawaited.last()) {
            (Some(path), Some((name, _))) => {
                path.first().map(String::as_str) == Some(ROOT) && path[1..].join(".") == *name
            }
            _ => false,
        };
        if self.unawaited.len() > before && awaits_last {
            self.unawaited.pop();
        }
        Ok(expr)
    }

    pub(super) fn postfix(&mut self) -> Result<SExpr> {
        let mut expr = self.primary()?;
        loop {
            let token = self.peek();
            let start = expr.span.map_or(token.start, |place| place.start);
            if self.cursor.is(".") && self.cursor.is_kind_at(TokenKind::Identifier, 1) {
                self.cursor.advance();
                let field = self.cursor.advance();
                if self.cursor.is("(") {
                    let array_form =
                        crate::translation::frontend_rules::read_array_method_form(&field.value);
                    if !array_form.is_empty() {
                        let args = self.arguments(&field.value)?;
                        let place = Span::new(start, self.to_here(&token).end);
                        #[allow(clippy::cast_precision_loss)]
                        let count = args.len() as f64;
                        let form = super::surface_rules::read_java_script_array_method_surface_form(
                            &field.value,
                            count,
                        );
                        if form == "copySlice" {
                            expr = node(
                                SNode::CopySlice {
                                    object: Box::new(expr),
                                },
                                place,
                            );
                            continue;
                        }
                        if form == "method" {
                            expr = node(
                                SNode::Method {
                                    object: Box::new(expr),
                                    op: field.value,
                                    args: args.into_iter().map(Some).collect(),
                                },
                                place,
                            );
                            continue;
                        }
                        expr = Self::array_method_expression(&array_form, args, Some(expr), place)?;
                        continue;
                    }
                    let mapping = crate::translation::frontend_rules::read_string_map_operation(
                        "JavaScript",
                        &field.value,
                    );
                    if !mapping.is_empty() {
                        let args = self.arguments(&field.value)?;
                        if !args.is_empty() {
                            return Err(unsupported(
                                &format!(".{}() with {} arguments", field.value, args.len()),
                                "string maps take no arguments",
                                Some(self.to_here(&token)),
                            ));
                        }
                        expr = node(
                            SNode::StringMap {
                                op: mapping,
                                object: Box::new(expr),
                            },
                            self.to_here(&token),
                        );
                        continue;
                    }
                    let operation = crate::translation::frontend_rules::read_string_test_operation(
                        "JavaScript",
                        &field.value,
                    );
                    if !operation.is_empty() {
                        let args = self.arguments(&field.value)?;
                        if args.len() != 1 {
                            return Err(unsupported(
                                &format!(".{}() with {} arguments", field.value, args.len()),
                                "search the whole string, with one argument",
                                Some(self.to_here(&token)),
                            ));
                        }
                        let place = Span::new(start, self.to_here(&token).end);
                        expr = node(
                            SNode::StringTest {
                                op: operation,
                                object: Box::new(expr),
                                search: Box::new(
                                    args.into_iter().next().expect("one search argument"),
                                ),
                            },
                            place,
                        );
                        continue;
                    }
                    if field.value != "toString" {
                        let args = self.arguments(&field.value)?;
                        expr = node(
                            SNode::Method {
                                object: Box::new(expr),
                                op: field.value,
                                args: args.into_iter().map(Some).collect(),
                            },
                            Span::new(start, self.to_here(&token).end),
                        );
                        continue;
                    }
                    let args = self.arguments("toString")?;
                    if !args.is_empty() {
                        return Err(unsupported(
                            "toString with a radix",
                            "only decimal text is portable",
                            Some(self.to_here(&token)),
                        ));
                    }
                    let place = Span::new(start, self.to_here(&token).end);
                    expr = node(
                        SNode::ToString {
                            arg: Box::new(expr),
                        },
                        place,
                    );
                    continue;
                }
                let place = Span::new(start, span(&field, &field).end);
                expr = node(
                    SNode::Field {
                        object: Box::new(expr),
                        field: field.value,
                    },
                    place,
                );
                continue;
            }
            let here = Some(span(&token, &token));
            if self.cursor.eat("?.").is_some() {
                let field = self.cursor.identifier(Some("optional field"))?;
                let place = Span::new(
                    expr.span.map_or(token.start, |place| place.start),
                    field.end,
                );
                expr = node(
                    SNode::OptionalField {
                        object: Box::new(expr),
                        field: field.value,
                    },
                    place,
                );
                continue;
            }
            if self.cursor.is("[") {
                self.cursor.advance();
                let index = self.expr()?;
                let close = self.cursor.expect("]", Some("index"))?;
                let place = Span::new(
                    expr.span.map_or(token.start, |place| place.start),
                    close.end,
                );
                expr = node(
                    SNode::Index {
                        object: Box::new(expr),
                        index: Box::new(index),
                        optional: None,
                        proven_in_bounds: None,
                    },
                    place,
                );
                let next = self.cursor.peek();
                if next.kind == TokenKind::Punct
                    && (ASSIGNMENTS.contains(&next.value.as_str())
                        || self.cursor.is("++")
                        || self.cursor.is("--"))
                {
                    let op = self.cursor.advance().value;
                    let value = if op == "++" || op == "--" {
                        None
                    } else {
                        Some(Box::new(self.expr()?))
                    };
                    let end = self.cursor.peek().start;
                    let SNode::Index { object, index, .. } = expr.node else {
                        unreachable!("an indexed assignment retains its actual target");
                    };
                    expr = node(
                        SNode::ReferenceWrite {
                            object,
                            index: Some(index),
                            field: None,
                            value,
                            op,
                        },
                        Span::new(place.start, end),
                    );
                }
                continue;
            }
            if self.cursor.is("(") {
                let args = self.arguments("function value")?;
                let place = joined(&expr, &node(SNode::Unit, self.cursor.peek().span()), &token);
                expr = node(
                    SNode::App {
                        func: Box::new(expr),
                        args,
                    },
                    place,
                );
                continue;
            }
            if self.cursor.is_kind(TokenKind::Template) {
                return Err(unsupported(
                    "tagged template",
                    "template tags are functions as values",
                    here,
                ));
            }
            if self.cursor.is("++") || self.cursor.is("--") {
                return Err(unsupported(
                    &format!("{} operator", token.value),
                    "mutation is outside the portable core",
                    here,
                ));
            }
            return Ok(expr);
        }
    }
}
