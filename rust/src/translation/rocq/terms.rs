//! Propositions, terms, applications, matches and patterns.

use super::{
    BinaryOp, DECIMAL_STRINGS, EXPRESSION_STOP, Flavor, INT, INT_TO_NAT, NAT, NAT_IDENTITY,
    NAT_TO_INT, PREDECESSOR, Result, RocqParser, SBinder, SComparison, SExpr, SNode, SPattern,
    SPatternNode, SProp, SPropNode, SRow, SUCCESSOR, Token, TokenKind, TranslationError, UnaryOp,
    arity, binary, binary_function, boolean_relation, connective, decimal_conversion, describe,
    is_prop_relation, joined, notation, outside_core, pattern_value, prop_relation, scope_prop,
    span, type_error, unreachable_relation, unsupported, with_scope,
};

impl RocqParser {
    pub(super) fn prop(&mut self) -> Result<SProp> {
        let token = self.peek().clone();
        if self.cursor.eat("forall").is_some() {
            let mut binders = Vec::new();
            if self.is("(") {
                while self.is("(") {
                    binders.extend(self.binder_group()?);
                }
            } else {
                let mut names = Vec::new();
                while self.cursor.is_kind(TokenKind::Identifier) && !self.is(":") {
                    names.push(self.cursor.advance().value);
                }
                self.expect(":", "forall")?;
                let rocq_type = self.peek().value.clone();
                let ty = self.parse_type()?;
                binders.extend(names.into_iter().map(|name| SBinder {
                    name,
                    ty: Some(ty.clone()),
                    rocq_type: Some(rocq_type.clone()),
                    span: None,
                }));
            }
            self.expect(",", "forall")?;
            let body = self.prop()?;
            return Ok(SProp {
                node: SPropNode::Forall {
                    binders,
                    body: Box::new(body),
                },
                span: Some(span(&token, self.peek())),
            });
        }
        if self.is("exists") {
            return Err(unsupported(
                "existential",
                "existential statements are outside the portable proof model",
                Some(span(&token, &token)),
            ));
        }
        let left = self.prop_or()?;
        if self.cursor.eat("->").is_some() {
            let right = self.prop()?;
            return Ok(connective(SPropNode::Implies {
                left: Box::new(left),
                right: Box::new(right),
            }));
        }
        if self.is("<->") {
            return Err(unsupported(
                "logical equivalence",
                "state both implications separately",
                Some(span(self.peek(), self.peek())),
            ));
        }
        Ok(left)
    }

    pub(super) fn prop_or(&mut self) -> Result<SProp> {
        let left = self.prop_and()?;
        if self.cursor.eat("\\/").is_some() {
            let right = self.prop_or()?;
            return Ok(connective(SPropNode::Or {
                left: Box::new(left),
                right: Box::new(right),
            }));
        }
        Ok(left)
    }

    pub(super) fn prop_and(&mut self) -> Result<SProp> {
        let left = self.prop_unary()?;
        if self.cursor.eat("/\\").is_some() {
            let right = self.prop_and()?;
            return Ok(connective(SPropNode::And {
                left: Box::new(left),
                right: Box::new(right),
            }));
        }
        Ok(left)
    }

    pub(super) fn prop_unary(&mut self) -> Result<SProp> {
        let token = self.peek().clone();
        if self.cursor.eat("~").is_some() {
            let arg = self.prop_unary()?;
            return Ok(connective(SPropNode::Not { arg: Box::new(arg) }));
        }
        if self.is("forall") {
            return self.prop();
        }
        if self.is("(") && self.prop_parenthesised() {
            self.cursor.advance();
            let inner = self.prop()?;
            self.expect(")", "proposition")?;
            return self.prop_scope(inner);
        }
        if self.is("True") || self.is("False") {
            return Err(unsupported(
                &token.value,
                "propositional constants are outside the portable proof model",
                Some(span(&token, &token)),
            ));
        }
        let left = self.expr_at(69)?;
        let relation = self.peek().clone();
        if relation.kind == TokenKind::Punct && is_prop_relation(&relation.value) {
            self.cursor.advance();
            let right = self.expr_at(69)?;
            let comparison = SComparison {
                left,
                right,
                reference: false,
                same_value: false,
            };
            return Ok(SProp {
                node: prop_relation(&relation.value, comparison)
                    .unwrap_or_else(|| unreachable_relation(&relation.value)),
                span: Some(span(&token, self.peek())),
            });
        }
        let left_span = left.span;
        let truth = SExpr::new(SNode::Bool { value: true }, None);
        let expr = SExpr::new(
            SNode::Binary {
                op: BinaryOp::Eq,
                left: Box::new(left),
                right: Box::new(truth),
                rounding: None,
            },
            left_span,
        );
        Ok(SProp {
            node: SPropNode::Bool { expr },
            span: Some(span(&token, self.peek())),
        })
    }

    /// `(P)%Z` delimits the literals of a whole proposition.
    pub(super) fn prop_scope(&mut self, inner: SProp) -> Result<SProp> {
        Ok(match self.scope_delimiter()? {
            Some(scope) => scope_prop(inner, &scope),
            None => inner,
        })
    }

    pub(super) fn prop_parenthesised(&self) -> bool {
        let mut depth = 0_i64;
        let mut offset = 0;
        loop {
            // Token values are compared whatever their kind, strings included.
            let token = self.cursor.peek_at(offset);
            if token.kind == TokenKind::Eof {
                return false;
            }
            if token.value == "(" {
                depth += 1;
            }
            if token.value == ")" {
                depth -= 1;
                if depth == 0 {
                    return false;
                }
            }
            if depth == 1
                && [
                    "/\\", "\\/", "->", "~", "forall", "=", "<>", "<", "<=", ">", ">=",
                ]
                .contains(&token.value.as_str())
            {
                return true;
            }
            offset += 1;
        }
    }

    pub(super) fn expr(&mut self) -> Result<SExpr> {
        self.expr_at(200)
    }

    /// Terms, by Rocq notation level: `let`, `if` and `match` at 200, the
    /// boolean relations at 70, `::` and `++` at 60, `+ - ||` at 50, `* / mod
    /// &&` at 40, unary minus at 35, and application.
    pub(super) fn expr_at(&mut self, level: u32) -> Result<SExpr> {
        let token = self.peek().clone();
        if self.is("if") {
            self.cursor.advance();
            let cond = self.expr()?;
            self.expect("then", "if")?;
            let then = self.expr()?;
            self.expect("else", "if")?;
            let otherwise = self.expr()?;
            return Ok(SExpr::new(
                SNode::If {
                    cond: Box::new(cond),
                    then: Box::new(then),
                    otherwise: Box::new(otherwise),
                },
                Some(span(&token, self.peek())),
            ));
        }
        if self.is("let") {
            self.cursor.advance();
            if self.is("(") || self.is("'") {
                return Err(unsupported(
                    "destructuring let",
                    "tuples are outside the portable core",
                    Some(span(&token, self.peek())),
                ));
            }
            let name = self.identifier("let")?.value;
            if self.is("(") {
                return Err(unsupported(
                    "local function",
                    "let-bound functions are outside the portable core",
                    Some(span(&token, self.peek())),
                ));
            }
            let ty = if self.cursor.eat(":").is_some() {
                Some(self.parse_type()?)
            } else {
                None
            };
            self.expect(":=", "let")?;
            let value = self.expr()?;
            self.expect("in", "let")?;
            let body = self.expr()?;
            return Ok(SExpr::new(
                SNode::Let {
                    name,
                    ty,
                    value: Box::new(value),
                    body: Box::new(body),
                },
                Some(span(&token, self.peek())),
            ));
        }
        if self.is("fun") {
            return Err(unsupported(
                "lambda",
                "higher-order values are outside the portable core",
                Some(span(&token, &token)),
            ));
        }
        if self.is("fix") {
            return Err(unsupported(
                "local fixpoint",
                "local recursive functions are outside the portable core",
                Some(span(&token, &token)),
            ));
        }
        self.relation(level)
    }

    pub(super) fn relation(&mut self, level: u32) -> Result<SExpr> {
        let left = self.infix(60)?;
        let token = self.peek().clone();
        if level >= 70
            && token.kind == TokenKind::Punct
            && let Some(op) = boolean_relation(&token.value)
        {
            self.cursor.advance();
            let right = self.infix(60)?;
            return Ok(binary(op, left, right, &token));
        }
        Ok(left)
    }

    /// Levels 60 and below: `::`, `++` (right), `+ - ||` and `* / mod &&` (left).
    pub(super) fn infix(&mut self, level: u32) -> Result<SExpr> {
        if level == 60 {
            let left = self.infix(50)?;
            let token = self.peek().clone();
            if self.is("::") {
                self.cursor.advance();
                let tail = self.cons()?;
                let cons_span = joined(&left, &tail, &token);
                return Ok(SExpr::new(
                    SNode::Cons {
                        head: Box::new(left),
                        tail: Box::new(tail),
                    },
                    Some(cons_span),
                ));
            }
            if self.is("++") {
                self.cursor.advance();
                let right = self.infix(60)?;
                return Ok(binary(BinaryOp::Concat, left, right, &token));
            }
            return Ok(left);
        }
        let mut left = if level == 50 {
            self.infix(40)?
        } else {
            self.prefix()?
        };
        loop {
            let token = self.peek().clone();
            let Some(op) = notation(level, &token) else {
                return Ok(left);
            };
            self.cursor.advance();
            let right = if level == 50 {
                self.infix(40)?
            } else {
                self.prefix()?
            };
            left = binary(op, left, right, &token);
        }
    }

    /// The tail of `::` may itself be a `let` of the program output.
    pub(super) fn cons(&mut self) -> Result<SExpr> {
        if self.is("let") || self.is("if") || self.is("match") {
            return self.expr();
        }
        self.infix(60)
    }

    pub(super) fn prefix(&mut self) -> Result<SExpr> {
        let token = self.peek().clone();
        if self.is("-") {
            self.cursor.advance();
            let arg = self.prefix()?;
            return Ok(SExpr::new(
                SNode::Unary {
                    op: UnaryOp::Neg,
                    arg: Box::new(arg),
                },
                Some(span(&token, self.peek())),
            ));
        }
        self.application()
    }

    pub(super) fn argument_start(&self) -> bool {
        let token = self.peek();
        if token.kind == TokenKind::Identifier {
            let value = token.value.as_str();
            return !EXPRESSION_STOP.contains(&value)
                && value != "mod"
                && !["if", "let", "match", "fun", "fix"].contains(&value);
        }
        matches!(token.kind, TokenKind::Number | TokenKind::String)
            || token.value == "("
            || token.value == "["
    }

    pub(super) fn application(&mut self) -> Result<SExpr> {
        let token = self.peek().clone();
        if self.is("match") {
            return self.match_expr();
        }
        let head = self.atom()?;
        let mut args = Vec::new();
        while self.argument_start() {
            args.push(self.atom()?);
        }
        self.apply_head(head, args, &token)
    }

    #[allow(clippy::too_many_lines)] // one branch per library family, as in the JavaScript runtime
    pub(super) fn apply_head(&self, head: SExpr, args: Vec<SExpr>, token: &Token) -> Result<SExpr> {
        let range = span(token, self.peek());
        let node = |node: SNode| SExpr::new(node, Some(range));
        let name = match &head.node {
            SNode::Name { path } if !path.is_empty() => Some(path.join(".")),
            _ => None,
        };
        if let Some(name) = name {
            if let Some((op, rounding)) = binary_function(&name) {
                let [left, right] = arity::<2>(&name, args, range)?;
                return Ok(node(SNode::Binary {
                    op,
                    left: Box::new(left),
                    right: Box::new(right),
                    rounding,
                }));
            }
            if name == "negb" || name == "Z.opp" {
                let [arg] = arity::<1>(&name, args, range)?;
                let op = if name == "negb" {
                    UnaryOp::Not
                } else {
                    UnaryOp::Neg
                };
                return Ok(node(SNode::Unary {
                    op,
                    arg: Box::new(arg),
                }));
            }
            let cast = if NAT_TO_INT.contains(&name.as_str()) {
                Some((INT, NAT, Flavor::Exact))
            } else if INT_TO_NAT.contains(&name.as_str()) {
                Some((NAT, INT, Flavor::Clamp))
            } else if NAT_IDENTITY.contains(&name.as_str()) {
                Some((NAT, NAT, Flavor::Exact))
            } else {
                None
            };
            if let Some((to, from, flavor)) = cast {
                let [arg] = arity::<1>(&name, args, range)?;
                return Ok(node(SNode::Cast {
                    arg: Box::new(arg),
                    to,
                    from: Some(from),
                    flavor,
                }));
            }
            let predecessor = PREDECESSOR.contains(&name.as_str());
            if predecessor || SUCCESSOR.contains(&name.as_str()) {
                let [arg] = arity::<1>(&name, args, range)?;
                let op = if predecessor {
                    BinaryOp::Sub
                } else {
                    BinaryOp::Add
                };
                let one = node(SNode::Num {
                    value: "1".to_owned(),
                    ty: name.starts_with("Z.").then_some(INT),
                    negative: false,
                    unit: false,
                });
                return Ok(node(SNode::Binary {
                    op,
                    left: Box::new(arg),
                    right: Box::new(one),
                    rounding: None,
                }));
            }
            if let Some((_, accepted)) = DECIMAL_STRINGS.iter().find(|(key, _)| *key == name) {
                let [inner] = arity::<1>(&name, args, range)?;
                if let SNode::App { func, args } = inner.node {
                    let converts = matches!(&func.node, SNode::Name { path } if accepted.contains(&path.join(".").as_str()));
                    if converts
                        && args.len() == 1
                        && let Some(arg) = args.into_iter().next()
                    {
                        return Ok(node(SNode::ToString { arg: Box::new(arg) }));
                    }
                }
                return Err(unsupported(
                    &name,
                    &format!("only {name} ({} x) renders a number", accepted.join(" | ")),
                    Some(range),
                ));
            }
            if name == "tt" && args.is_empty() {
                return Ok(node(SNode::Unit));
            }
            if name == "true" || name == "false" {
                if !args.is_empty() {
                    return Err(type_error(format!("{name} is not a function"), Some(range)));
                }
                return Ok(node(SNode::Bool {
                    value: name == "true",
                }));
            }
            // A name contained in a `DECIMAL_STRINGS` key (`String.Nil`, …)
            // escapes this check, as in the JavaScript runtime.
            if outside_core(&name) && !DECIMAL_STRINGS.iter().any(|(key, _)| key.contains(&name)) {
                if !args.is_empty() && decimal_conversion(&name) {
                    return Ok(node(SNode::App {
                        func: Box::new(head),
                        args,
                    }));
                }
                return Err(unsupported(
                    &format!("Rocq library function {name}"),
                    "outside the portable core",
                    Some(range),
                ));
            }
            if name == "nil" && args.is_empty() {
                return Ok(node(SNode::Nil));
            }
        }
        if args.is_empty() {
            return Ok(head);
        }
        if !matches!(head.node, SNode::Name { .. }) {
            return Err(unsupported(
                "higher-order application",
                "only named functions can be applied",
                Some(range),
            ));
        }
        Ok(node(SNode::App {
            func: Box::new(head),
            args,
        }))
    }

    pub(super) fn match_expr(&mut self) -> Result<SExpr> {
        let token = self.cursor.advance();
        let mut scrutinees = vec![self.expr()?];
        while self.cursor.eat(",").is_some() {
            scrutinees.push(self.expr()?);
        }
        if self.is("as") || self.is("in") || self.is("return") {
            return Err(unsupported(
                "dependent match",
                "return clauses are outside the portable core",
                Some(span(&token, self.peek())),
            ));
        }
        self.expect("with", "match")?;
        let mut rows = Vec::new();
        self.cursor.eat("|");
        if !self.is("end") {
            loop {
                let bar = self.peek().clone();
                let mut aliases = Vec::new();
                let mut patterns = vec![self.pattern(&mut aliases)?];
                while self.cursor.eat(",").is_some() {
                    patterns.push(self.pattern(&mut aliases)?);
                }
                if patterns.len() != scrutinees.len() {
                    return Err(TranslationError::syntax(
                        format!(
                            "alternative has {} patterns, expected {}",
                            patterns.len(),
                            scrutinees.len()
                        ),
                        Some(bar.span()),
                    ));
                }
                self.expect("=>", "alternative")?;
                let mut body = self.expr()?;
                for (name, value) in aliases.into_iter().rev() {
                    let body_span = body.span;
                    body = SExpr::new(
                        SNode::Let {
                            name,
                            ty: None,
                            value: Box::new(value),
                            body: Box::new(body),
                        },
                        body_span,
                    );
                }
                rows.push(SRow {
                    patterns,
                    body,
                    span: Some(span(&bar, self.peek())),
                });
                if self.cursor.eat("|").is_none() {
                    break;
                }
            }
        }
        self.expect("end", "match")?;
        if rows.is_empty() {
            return Err(self.fail_here("expected match alternatives"));
        }
        let match_span = span(&token, self.peek());
        self.scoped(SExpr::new(
            SNode::Match { scrutinees, rows },
            Some(match_span),
        ))
    }

    /// Patterns: constructors (`O`, `S k`, `node l v r`), numerals, `_`,
    /// variables, parentheses and `p as x`. An alias becomes a `let` of the
    /// value the pattern matched, rebuilt from its fields.
    pub(super) fn pattern(&mut self, aliases: &mut Vec<(String, SExpr)>) -> Result<SPattern> {
        let token = self.peek().clone();
        let pattern = if token.kind == TokenKind::Identifier && token.value != "_" {
            self.cursor.advance();
            let path: Vec<String> = token.value.split('.').map(str::to_owned).collect();
            let mut args = Vec::new();
            while self.pattern_argument_start() {
                args.push(self.pattern_atom(aliases)?);
            }
            if args.is_empty() && path.len() == 1 {
                SPattern {
                    node: SPatternNode::BindOrCtor {
                        name: token.value.clone(),
                    },
                    span: Some(span(&token, &token)),
                }
            } else {
                SPattern {
                    node: SPatternNode::Ctor { path, args },
                    span: Some(span(&token, self.peek())),
                }
            }
        } else {
            self.pattern_atom(aliases)?
        };
        while self.cursor.eat("as").is_some() {
            let name = self.identifier("as")?.value;
            let value = pattern_value(&pattern, span(&token, self.peek()))?;
            aliases.push((name, value));
        }
        Ok(pattern)
    }

    pub(super) fn pattern_argument_start(&self) -> bool {
        let token = self.peek();
        (token.kind == TokenKind::Identifier
            && !["as", "with", "end"].contains(&token.value.as_str()))
            || token.kind == TokenKind::Number
            || self.is("(")
            || self.is("_")
    }

    pub(super) fn pattern_atom(&mut self, aliases: &mut Vec<(String, SExpr)>) -> Result<SPattern> {
        let token = self.peek().clone();
        if self.cursor.eat("_").is_some() {
            return Ok(SPattern {
                node: SPatternNode::Wild,
                span: None,
            });
        }
        if token.kind == TokenKind::Number {
            self.cursor.advance();
            if self.is("%") {
                return Err(unsupported(
                    "scoped numeral pattern",
                    "numeral patterns on N or Z match binary constructors, which have no portable counterpart",
                    Some(span(&token, self.peek())),
                ));
            }
            return Ok(SPattern {
                node: SPatternNode::NumLit {
                    value: token.value.clone(),
                    negative: false,
                },
                span: Some(span(&token, &token)),
            });
        }
        if self.cursor.eat("(").is_some() {
            let inner = self.pattern(aliases)?;
            if self.is("|") {
                return Err(unsupported(
                    "or-pattern",
                    "disjunctive patterns are outside the portable core",
                    Some(span(&token, self.peek())),
                ));
            }
            self.expect(")", "pattern")?;
            return Ok(inner);
        }
        if token.kind == TokenKind::Identifier {
            self.cursor.advance();
            let path: Vec<String> = token.value.split('.').map(str::to_owned).collect();
            let node = if path.len() == 1 {
                if token.value == "true" || token.value == "false" {
                    SPatternNode::BoolLit {
                        value: token.value == "true",
                        negative: false,
                    }
                } else {
                    SPatternNode::BindOrCtor {
                        name: token.value.clone(),
                    }
                }
            } else {
                SPatternNode::Ctor {
                    path,
                    args: Vec::new(),
                }
            };
            return Ok(SPattern {
                node,
                span: Some(span(&token, &token)),
            });
        }
        if token.kind == TokenKind::String {
            return Err(unsupported(
                "string pattern",
                "string patterns are outside the portable core in Rocq",
                Some(span(&token, &token)),
            ));
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

    pub(super) fn atom(&mut self) -> Result<SExpr> {
        let token = self.peek().clone();
        if token.kind == TokenKind::Number {
            self.cursor.advance();
            return self.scoped(SExpr::new(
                SNode::Num {
                    value: token.value.clone(),
                    ty: None,
                    negative: false,
                    unit: false,
                },
                Some(span(&token, &token)),
            ));
        }
        if token.kind == TokenKind::String {
            self.cursor.advance();
            return self.scoped(SExpr::new(
                SNode::Str {
                    value: token.value.clone(),
                },
                Some(span(&token, &token)),
            ));
        }
        if self.cursor.eat("[").is_some() {
            let mut items = Vec::new();
            while !self.is("]") {
                items.push(self.expr()?);
                if self.cursor.eat(";").is_none() {
                    break;
                }
            }
            self.expect("]", "list")?;
            return Ok(SExpr::new(
                SNode::List { items },
                Some(span(&token, self.peek())),
            ));
        }
        if self.cursor.eat("(").is_some() {
            if self.cursor.eat(")").is_some() {
                return Ok(SExpr::new(SNode::Unit, Some(span(&token, &token))));
            }
            let inner = self.expr()?;
            if self.cursor.eat(":").is_some() {
                let ty = self.parse_type()?;
                self.expect(")", "type ascription")?;
                let ascribed_span = Some(span(&token, self.peek()));
                if let SNode::Num {
                    value, negative, ..
                } = inner.node
                {
                    return self.scoped(SExpr {
                        node: SNode::Num {
                            value,
                            ty: Some(ty),
                            negative,
                            unit: false,
                        },
                        span: ascribed_span,
                        block: inner.block,
                        tag_test: None,
                    });
                }
                return self.scoped(SExpr::new(
                    SNode::Cast {
                        arg: Box::new(inner),
                        to: ty,
                        from: None,
                        flavor: Flavor::Exact,
                    },
                    ascribed_span,
                ));
            }
            if self.is(",") {
                return Err(unsupported(
                    "tuple",
                    "product values are outside the portable core",
                    Some(span(&token, self.peek())),
                ));
            }
            self.expect(")", "parenthesised expression")?;
            return self.scoped(inner);
        }
        if token.kind == TokenKind::Identifier {
            if self.is("match") {
                return self.match_expr();
            }
            self.cursor.advance();
            if token.value == "_" {
                return Err(unsupported(
                    "implicit argument hole",
                    "outside the portable core",
                    Some(span(&token, &token)),
                ));
            }
            return self.scoped(SExpr::new(
                SNode::Name {
                    path: token.value.split('.').map(str::to_owned).collect(),
                },
                Some(span(&token, &token)),
            ));
        }
        Err(self.fail_here("expected an expression"))
    }

    pub(super) fn scope_delimiter(&mut self) -> Result<Option<String>> {
        if !self.is("%") {
            return Ok(None);
        }
        self.cursor.advance();
        let scope = self.identifier("scope")?;
        if !["N", "nat", "Z", "string", "bool", "list", "type"].contains(&scope.value.as_str()) {
            return Err(unsupported(
                &format!("%{} scope", scope.value),
                "outside the portable core",
                Some(span(&scope, &scope)),
            ));
        }
        Ok(Some(scope.value))
    }

    /// `e%N`, `(e)%Z`: the delimiting scope decides the type of notation numerals.
    pub(super) fn scoped(&mut self, node: SExpr) -> Result<SExpr> {
        Ok(match self.scope_delimiter()? {
            Some(scope) => with_scope(node, &scope),
            None => node,
        })
    }
}
