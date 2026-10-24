//! Expressions from operator precedence down to literals and references.

use super::{
    ASSIGNMENTS, BinaryOp, External, FLOAT, GLOBALS, JavaScriptParser, Language, ROOT, Result,
    SArrayItem, SData, SExpr, SNode, SPattern, SPatternNode, SRow, STagTest, ShowStyle, Span,
    Token, TokenCursor, TokenKind, UnaryOp, binary, has_ctor, is_tag_field, joined, node,
    number_literal, span, tokenize, type_error, unsupported, wild,
};

mod values;
use crate::translation::surface::SurfaceRawMetadata;

impl JavaScriptParser {
    /// Expressions, by JavaScript precedence.
    pub(super) fn expr(&mut self) -> Result<SExpr> {
        let start = self.peek();
        if start.kind == TokenKind::Identifier && self.cursor.is_at("=>", 1) {
            return self.read_closure(false);
        }
        let mut value = self.or()?;
        if self.cursor.eat("?").is_some() {
            let then = self.expr()?;
            self.cursor.expect(":", Some("conditional expression"))?;
            let otherwise = self.expr()?;
            let place = joined(&value, &otherwise, &start);
            value = node(
                SNode::If {
                    cond: Box::new(value),
                    then: Box::new(then),
                    otherwise: Box::new(otherwise),
                },
                place,
            );
        }
        let next = self.cursor.peek();
        if next.kind == TokenKind::Punct && ASSIGNMENTS.contains(&next.value.as_str()) {
            let operator = self.cursor.advance();
            return self.scalar_assignment_expression(value, &operator, false, Some(Self::expr));
        }
        Ok(value)
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
                    .and_then(SurfaceRawMetadata::as_boolean)
                    .unwrap_or(false);
                let error =
                    crate::translation::optional_value_rules::read_optional_logical_mixing_error(
                        "left",
                        coalesced,
                        parenthesized,
                        left.node.kind(),
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
                    .and_then(SurfaceRawMetadata::as_boolean)
                    .unwrap_or(false);
                let error =
                    crate::translation::optional_value_rules::read_optional_logical_mixing_error(
                        "right",
                        coalesced,
                        parenthesized,
                        right.node.kind(),
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
        self.read_bitwise_precedence(0)
    }

    fn read_bitwise_precedence(&mut self, level: u8) -> Result<SExpr> {
        let mut left = if level < 2 {
            self.read_bitwise_precedence(level + 1)?
        } else {
            self.equality()?
        };
        while self.peek().kind == TokenKind::Punct && crate::translation::number_bitwise_rules::accept_java_script_number_bitwise_precedence(&self.peek().value, f64::from(level)) {
            let token = self.cursor.advance();
            let right = if level < 2 { self.read_bitwise_precedence(level + 1)? } else { self.equality()? };
            left = Self::number_bitwise(&token, left, Some(right));
        }
        Ok(left)
    }

    fn number_bitwise(token: &Token, left: SExpr, right: Option<SExpr>) -> SExpr {
        let place = if let Some(value) = &right {
            joined(&left, value, token)
        } else {
            Span::new(token.start, left.span.map_or(token.end, |place| place.end))
        };
        let mut args = vec![SArrayItem::new(left, false)];
        if let Some(value) = right {
            args.push(SArrayItem::new(value, false));
        }
        node(SNode::Math {
            op: crate::translation::number_bitwise_rules::read_java_script_number_bitwise_operation(&token.value),
            name: token.value.clone(), args,
        }, place)
    }

    pub(super) fn equality(&mut self) -> Result<SExpr> {
        let mut left = self.relational()?;
        loop {
            let token = self.peek();
            let loose=crate::translation::javascript_comparison_rules::read_java_script_loose_equality_operation(if token.kind==TokenKind::Punct{&token.value}else{""});
            if !loose.is_empty() {
                self.cursor.advance();
                let right = self.relational()?;
                let place = joined(&left, &right, &token);
                left = node(
                    SNode::Math {
                        op: loose,
                        name: token.value,
                        args: vec![SArrayItem::new(left, false), SArrayItem::new(right, false)],
                    },
                    place,
                );
                continue;
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
        let receiver = if candidates.len() > 1 {
            self.read_tag_receiver_owner(&object)
        } else {
            String::new()
        };
        let owners = candidates
            .iter()
            .map(|data| data.name.clone())
            .collect::<Vec<_>>();
        let selected =
            crate::translation::javascript_tag_context_rules::read_java_script_tag_owner_position(
                &owners, &receiver,
            );
        let Some(data) = crate::translation::frontend_rules::ml_array::number_index(selected)
            .and_then(|index| candidates.get(index))
        else {
            return Err(type_error(
                if candidates.is_empty() {
                    format!("no @typedef has tag {tag}")
                } else {
                    format!("tag {tag} is ambiguous between data types")
                },
                Some(place),
            ));
        };
        let data = (*data).clone();
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
        let mut left = self.additive()?;
        while self.cursor.is("<<") || self.cursor.is(">>") || self.cursor.is(">>>") {
            let token = self.cursor.advance();
            let right = self.additive()?;
            left = Self::number_bitwise(&token, left, Some(right));
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
        let start = self.peek();
        let left = self.unary()?;
        if self.cursor.eat("**").is_some() {
            let parenthesized = left
                .parenthesized
                .as_ref()
                .and_then(SurfaceRawMetadata::as_boolean)
                .unwrap_or(false);
            if !crate::translation::big_integer_conversion_rules::accept_java_script_exponentiation_base(&start.value,parenthesized){
                return Err(Self::fail("an unparenthesized unary expression cannot be the left operand of exponentiation",&start));
            }
            let right = self.exponent()?;
            let place = joined(&left, &right, &start);
            return Ok(node(
                SNode::Math {
                    op: "exponentiateJavaScriptNumericValue".to_owned(),
                    name: "**".to_owned(),
                    args: vec![SArrayItem::new(left, false), SArrayItem::new(right, false)],
                },
                place,
            ));
        }
        Ok(left)
    }

    pub(super) fn unary(&mut self) -> Result<SExpr> {
        let token = self.peek();
        if token.kind == TokenKind::Identifier && self.cursor.eat("void").is_some() {
            let arg = self.unary()?;
            let place = Span::new(token.start, arg.span.map_or(token.end, |arg| arg.end));
            return self.void_expression(arg, place);
        }
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
        if self.cursor.eat("+").is_some() {
            let arg = self.unary()?;
            let place = Span::new(token.start, arg.span.map_or(token.end, |arg| arg.end));
            return Ok(node(SNode::Math {
                op: crate::translation::string_number_rules::read_java_script_primitive_unary_operation("+"),
                name: "+".to_owned(), args: vec![super::SArrayItem::new(arg, false)],
            }, place));
        }
        if self.cursor.eat("~").is_some() {
            let arg = self.unary()?;
            return Ok(Self::number_bitwise(&token, arg, None));
        }
        if self.cursor.is("++") || self.cursor.is("--") {
            let operator = self.cursor.advance();
            let target = self.unary()?;
            return self.scalar_assignment_expression(target, &operator, false, None);
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
                        if field.value == "concat" {
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
                        expr = node(
                            SNode::Method {
                                op: field.value,
                                object: Box::new(expr),
                                args: args.into_iter().map(Some).collect(),
                            },
                            Span::new(start, self.to_here(&token).end),
                        );
                        continue;
                    }
                    let operation = crate::translation::frontend_rules::read_string_test_operation(
                        "JavaScript",
                        &field.value,
                    );
                    if !operation.is_empty() {
                        let args = self.arguments(&field.value)?;
                        let place = Span::new(start, self.to_here(&token).end);
                        expr = node(
                            SNode::Method {
                                op: field.value,
                                object: Box::new(expr),
                                args: args.into_iter().map(Some).collect(),
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
                    let place = Span::new(start, self.to_here(&token).end);
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
                if self.cursor.is("(") {
                    let path_length = match &expr.node {
                        SNode::Name { path } | SNode::FunctionValue { path } => path.len(),
                        _ => 0,
                    };
                    if !crate::translation::javascript_optional_call_rules::accept_java_script_optional_call_head(expr.node.kind(),super::callables::number(path_length)){
                        return Err(unsupported("optional method call","the property reference needs its complete original receiver and chain extent",Some(joined(&expr,&node(SNode::Unit,token.span()),&token))));
                    }
                    let args = self.arguments("optional callable")?;
                    if crate::translation::javascript_optional_call_rules::require_java_script_optional_call_chain(&self.cursor.peek().value){
                        return Err(unsupported("optional call chain","the chain needs its distinct skipped and returned value continuations",Some(joined(&expr,&node(SNode::Unit,self.cursor.peek().span()),&token))));
                    }
                    let place =
                        joined(&expr, &node(SNode::Unit, self.cursor.peek().span()), &token);
                    expr=node(SNode::Math{op:crate::translation::javascript_optional_call_rules::read_java_script_optional_call_operation(),name:String::new(),args:std::iter::once(expr).chain(args).map(|value|super::SArrayItem::new(value,false)).collect()},place);
                    continue;
                }
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
                let index = self.sequence()?;
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
                    self.indexed_reference_writes += 1;
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
                let end = self.cursor.tokens[self.cursor.index - 1].end;
                if self.source.units()[end..token.start]
                    .iter()
                    .any(|unit| super::accept_java_script_line_terminator(f64::from(*unit)))
                {
                    return Ok(expr);
                }
                self.cursor.advance();
                return self.scalar_assignment_expression(expr, &token, true, None);
            }
            return Ok(expr);
        }
    }
}
