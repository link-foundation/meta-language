//! Expressions from operator precedence down to literals and references.

use super::{
    bigint, binary, describe, has_ctor, is_tag_field, joined, node, span, tokenize, type_error,
    unsupported, wild, BinaryOp, JavaScriptParser, Language, Result, SData, SExpr, SNode, SPattern,
    SPatternNode, SRow, STagTest, ShowStyle, Span, Token, TokenCursor, TokenKind, UnaryOp,
    ASSIGNMENTS, GLOBALS, NUMBER_REASON, ROOT,
};

impl JavaScriptParser {
    /// Expressions, by JavaScript precedence.
    pub(super) fn expr(&mut self) -> Result<SExpr> {
        let start = self.peek();
        if start.kind == TokenKind::Identifier && self.cursor.is_at("=>", 1) {
            return Err(unsupported(
                "arrow function",
                "functions as values are outside the portable core",
                Some(span(&start, self.cursor.peek_at(1))),
            ));
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
        loop {
            let token = self.peek();
            if self.cursor.is("??") {
                return Err(unsupported(
                    "nullish coalescing",
                    "null and undefined are outside the portable core",
                    Some(span(&token, &token)),
                ));
            }
            if self.cursor.eat("||").is_none() {
                return Ok(left);
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
                ))
            }
            _ => {
                return Err(type_error(
                    format!("tag {tag} is ambiguous between data types"),
                    Some(place),
                ))
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

    pub(super) fn postfix(&mut self) -> Result<SExpr> {
        let mut expr = self.primary()?;
        loop {
            let token = self.peek();
            let start = expr.span.map_or(token.start, |place| place.start);
            if self.cursor.is(".") && self.cursor.is_kind_at(TokenKind::Identifier, 1) {
                self.cursor.advance();
                let field = self.cursor.advance();
                if self.cursor.is("(") {
                    if field.value != "toString" {
                        return Err(unsupported(
                            &format!("method call .{}()", field.value),
                            "methods of values are outside the portable core",
                            Some(self.to_here(&token)),
                        ));
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
            if self.cursor.is("?.") {
                return Err(unsupported(
                    "optional chaining",
                    "null and undefined are outside the portable core",
                    here,
                ));
            }
            if self.cursor.is("[") {
                return Err(unsupported(
                    "indexing",
                    "arrays and computed properties are outside the portable core",
                    here,
                ));
            }
            if self.cursor.is("(") {
                return Err(unsupported(
                    "call of a computed function",
                    "only named functions can be called",
                    here,
                ));
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

    pub(super) fn primary(&mut self) -> Result<SExpr> {
        let token = self.peek();
        match token.kind {
            TokenKind::Number => {
                self.cursor.advance();
                if self.cursor.is(".")
                    && self.cursor.is_kind_at(TokenKind::Number, 1)
                    && self.cursor.peek_at(1).start == self.cursor.peek().end
                {
                    return Err(unsupported(
                        "JavaScript number",
                        NUMBER_REASON,
                        Some(span(&token, self.cursor.peek_at(1))),
                    ));
                }
                return Ok(node(
                    SNode::Num {
                        value: bigint(&token)?,
                        ty: None,
                        negative: false,
                    },
                    span(&token, &token),
                ));
            }
            TokenKind::String => {
                self.cursor.advance();
                return Ok(node(
                    SNode::Str {
                        value: token.value.clone(),
                    },
                    span(&token, &token),
                ));
            }
            TokenKind::Template => {
                self.cursor.advance();
                return self.template(&token);
            }
            TokenKind::Identifier => return self.identifier(),
            _ => {}
        }
        let here = Some(span(&token, &token));
        if self.cursor.is("(") {
            let close = self.matching(self.cursor.index);
            if self
                .cursor
                .tokens
                .get(close + 1)
                .is_some_and(|after| after.value == "=>")
            {
                return Err(unsupported(
                    "arrow function",
                    "functions as values are outside the portable core",
                    here,
                ));
            }
            self.cursor.advance();
            let mut inner = self.expr()?;
            self.cursor.expect(")", Some("parenthesised expression"))?;
            inner.span = Some(self.to_here(&token));
            return Ok(inner);
        }
        if self.cursor.is("{") {
            return self.object_literal();
        }
        if self.cursor.is("[") {
            return Err(unsupported(
                "array literal",
                "arrays are outside the portable core",
                here,
            ));
        }
        if self.cursor.is("/") {
            return Err(unsupported(
                "regular expression",
                "outside the portable core",
                here,
            ));
        }
        Err(self.fail_here("expected an expression"))
    }

    pub(super) fn matching(&self, index: usize) -> usize {
        let tokens = &self.cursor.tokens;
        let mut depth = 0_i64;
        for (at, token) in tokens.iter().enumerate().skip(index) {
            if token.kind != TokenKind::Punct {
                continue;
            }
            if ["(", "[", "{"].contains(&token.value.as_str()) {
                depth += 1;
            }
            if [")", "]", "}"].contains(&token.value.as_str()) {
                depth -= 1;
                if depth == 0 {
                    return at;
                }
            }
        }
        tokens.len() - 1
    }

    /// Template literals concatenate their text with `String(value)` of each substitution.
    pub(super) fn template(&mut self, token: &Token) -> Result<SExpr> {
        let whole = span(token, token);
        let mut pieces = Vec::new();
        for part in &token.parts {
            if !part.text.is_empty() {
                pieces.push(node(
                    SNode::Str {
                        value: part.text.clone(),
                    },
                    whole,
                ));
            }
            if let Some(expression) = &part.expression {
                let value = self.subexpression(&expression.source, expression.offset)?;
                let place = value.span;
                pieces.push(SExpr::new(
                    SNode::Show {
                        arg: Box::new(value),
                        style: ShowStyle::JsTemplate,
                    },
                    place,
                ));
            }
        }
        let mut pieces = pieces.into_iter();
        let Some(first) = pieces.next() else {
            return Ok(node(
                SNode::Str {
                    value: String::new(),
                },
                whole,
            ));
        };
        Ok(pieces.fold(first, |left, right| {
            node(
                SNode::Binary {
                    op: BinaryOp::Concat,
                    left: Box::new(left),
                    right: Box::new(right),
                    rounding: None,
                },
                whole,
            )
        }))
    }

    pub(super) fn subexpression(&mut self, source: &str, offset: usize) -> Result<SExpr> {
        let mut tokens = tokenize(source, Language::JavaScript)?.tokens;
        for token in &mut tokens {
            token.start += offset;
            token.end += offset;
        }
        let outer = std::mem::replace(
            &mut self.cursor,
            TokenCursor::new(tokens, Language::JavaScript),
        );
        let expr = self.substitution();
        self.cursor = outer;
        expr
    }

    pub(super) fn substitution(&mut self) -> Result<SExpr> {
        let expr = self.expr()?;
        if !self.cursor.at_end() {
            return Err(self.fail_here("expected } after the template substitution"));
        }
        Ok(expr)
    }

    /// `{ $: 'node', left, value: v }` constructs the `node` alternative of a data type.
    pub(super) fn object_literal(&mut self) -> Result<SExpr> {
        let start = self.cursor.advance();
        let mut tag = None;
        let mut fields: Vec<(String, SExpr)> = Vec::new();
        while !self.cursor.is("}") {
            let key = self.peek();
            let here = Some(span(&key, &key));
            if self.cursor.is("...") {
                return Err(unsupported("object spread", "name every field", here));
            }
            if key.kind != TokenKind::Identifier {
                return Err(unsupported(
                    &format!("object key {}", describe(&key)),
                    "keys are identifiers",
                    here,
                ));
            }
            self.cursor.advance();
            if key.value == "$" {
                self.cursor.expect(":", Some("object"))?;
                let value = self.cursor.advance();
                if value.kind != TokenKind::String {
                    return Err(unsupported(
                        "computed tag",
                        "the $ tag is a string literal",
                        Some(span(&value, &value)),
                    ));
                }
                tag = Some(value.value);
            } else {
                if fields.iter().any(|(name, _)| *name == key.value) {
                    return Err(type_error(format!("duplicate field {}", key.value), here));
                }
                if self.cursor.is("(") {
                    return Err(unsupported(
                        "method in an object value",
                        "functions as values are outside the portable core",
                        here,
                    ));
                }
                let value = if self.cursor.eat(":").is_some() {
                    self.expr()?
                } else {
                    self.reference(&key)?
                };
                // Assigning `__proto__` sets the prototype of the JavaScript
                // frontend's field object rather than adding a field.
                if key.value != "__proto__" {
                    fields.push((key.value, value));
                }
            }
            if self.cursor.eat(",").is_none() {
                break;
            }
        }
        self.cursor.expect("}", Some("object"))?;
        let Some(tag) = tag else {
            return Err(unsupported(
                "object without a $ tag",
                "a portable object is an alternative of a @typedef data type",
                Some(self.to_here(&start)),
            ));
        };
        Ok(node(
            SNode::CtorObject { tag, fields },
            self.to_here(&start),
        ))
    }

    pub(super) fn identifier(&mut self) -> Result<SExpr> {
        let token = self.peek();
        let here = Some(span(&token, &token));
        if token.value == "true" || token.value == "false" {
            self.cursor.advance();
            return Ok(node(
                SNode::Bool {
                    value: token.value == "true",
                },
                span(&token, &token),
            ));
        }
        if ["null", "undefined", "NaN", "Infinity"].contains(&token.value.as_str()) {
            return Err(unsupported(&token.value, "outside the portable core", here));
        }
        let expressions = [
            "this",
            "super",
            "new",
            "function",
            "class",
            "import",
            "arguments",
        ];
        if expressions.contains(&token.value.as_str()) {
            return Err(unsupported(
                &format!("{} expression", token.value),
                "outside the portable core",
                here,
            ));
        }
        self.cursor.advance();
        if self.scope.locals.contains(&token.value) || self.scope.tdz.contains(&token.value) {
            return self.reference(&token);
        }
        if self
            .assertion
            .as_ref()
            .is_some_and(|assertion| assertion.name == token.value)
        {
            return Err(unsupported(
                "assertion in an expression",
                "assertions are top-level statements",
                here,
            ));
        }
        // A global: a function, or a namespace path to a method, which must be called.
        let mut segments = vec![token.value.clone()];
        while self.cursor.is(".") && self.cursor.is_kind_at(TokenKind::Identifier, 1) {
            self.cursor.advance();
            segments.push(self.cursor.advance().value);
        }
        let name = segments.join(".");
        let place = self.to_here(&token);
        if !self.cursor.is("(") {
            return Err(unsupported(
                &format!("function value {name}"),
                "functions and namespaces are only portable when a function is called",
                Some(place),
            ));
        }
        let mut args = self.arguments(&name)?;
        let called = self.to_here(&token);
        if name == "String" && args.len() == 1 {
            let arg = args.remove(0);
            return Ok(node(SNode::ToString { arg: Box::new(arg) }, called));
        }
        if name == "Object.freeze" && args.len() == 1 {
            let mut arg = args.remove(0);
            arg.span = Some(called);
            return Ok(arg);
        }
        if GLOBALS.contains(&segments[0].as_str()) || name == "String" {
            return Err(unsupported(
                &name,
                "the JavaScript standard library is outside the portable core",
                Some(called),
            ));
        }
        let mut path = vec![ROOT.to_owned()];
        path.extend(segments);
        if args.is_empty() {
            return Ok(node(SNode::Name { path }, called));
        }
        Ok(node(
            SNode::App {
                func: Box::new(node(SNode::Name { path }, place)),
                args,
            },
            called,
        ))
    }

    /// A local variable; reading one before its declaration throws in JavaScript.
    pub(super) fn reference(&self, token: &Token) -> Result<SExpr> {
        let here = span(token, token);
        if self.scope.tdz.contains(&token.value) {
            return Err(unsupported(
                &format!("use of {} before its declaration", token.value),
                "the binding is in its temporal dead zone, where reading it throws a ReferenceError",
                Some(here),
            ));
        }
        if !self.scope.locals.contains(&token.value) {
            return Err(unsupported(
                &format!("function value {}", token.value),
                "functions are only portable when called",
                Some(here),
            ));
        }
        Ok(node(
            SNode::Name {
                path: vec![token.value.clone()],
            },
            here,
        ))
    }
}
