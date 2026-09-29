//! Statements: bindings, destructuring, control flow, switches, `main`, console and assertions.

use super::imperative::lower_top_level;
use super::loops::reserved;
use super::lowering::statement_span;
use super::{
    assertion_kind, has_ctor, node, prop_of, span, type_error, unsupported, AssertionKind,
    CaseTest, Clause, JavaScriptParser, Result, SComparison, SData, SEffect, SExpr, SNode, SProp,
    SPropNode, ShowStyle, Span, Stmt, Switch, Token, TokenKind, ERRORS,
};

impl JavaScriptParser {
    pub(super) fn statement(&mut self) -> Result<Stmt> {
        let token = self.peek();
        if self.cursor.is("const") {
            return self.const_statement();
        }
        if self.cursor.is("let") {
            let mut statements = self.let_declarations()?;
            self.cursor.eat(";");
            if statements.len() == 1 {
                return Ok(statements.remove(0));
            }
            return Ok(Stmt::Block {
                body: statements,
                inline: true,
                span: self.to_here(&token),
            });
        }
        if self.cursor.is("var") {
            return Err(unsupported(
                "var declaration",
                "var bindings are hoisted to the function and shared by its blocks; use let or const",
                Some(span(&token, &token)),
            ));
        }
        if self.cursor.is("return") {
            if self.top_level {
                return Err(unsupported(
                    "top-level return statement",
                    "return leaves a function, and a module has none to leave",
                    Some(span(&token, &token)),
                ));
            }
            self.cursor.advance();
            let next = self.cursor.peek().start;
            let line_break = token.end < next
                && self.source.units()[token.end..next].contains(&u16::from(b'\n'));
            if self.cursor.is(";") || self.cursor.is("}") || line_break {
                return Err(unsupported(
                    "return without a value",
                    "the function would return undefined, which is not a portable value",
                    Some(self.to_here(&token)),
                ));
            }
            // An async function returning a call of another adopts the Promise
            // it returns, which awaits it.
            let expr = if self.in_async {
                self.awaited(Self::expr)?
            } else {
                self.expr()?
            };
            self.cursor.eat(";");
            return Ok(Stmt::Return {
                expr,
                span: self.to_here(&token),
            });
        }
        if self.cursor.is("throw") {
            return self.throw_statement();
        }
        if self.cursor.is("if") {
            return self.if_statement();
        }
        if self.cursor.is("switch") {
            return self.switch_statement().map(Stmt::Switch);
        }
        if self.cursor.is("{") {
            let body = self.block_statements()?;
            return Ok(Stmt::Block {
                body,
                inline: false,
                span: self.to_here(&token),
            });
        }
        if self.cursor.eat(";").is_some() {
            return Ok(Stmt::Empty);
        }
        if self.cursor.is("while") {
            return self.while_statement();
        }
        if self.cursor.is("do") {
            return self.do_statement();
        }
        if self.cursor.is("for") {
            return self.for_statement();
        }
        if self.cursor.is("break") || self.cursor.is("continue") {
            return self.jump_statement();
        }
        if self.cursor.is("console") && self.cursor.is_at(".", 1) {
            let (expr, place) = self.console_call()?;
            return Ok(Stmt::Print {
                expr,
                style: ShowStyle::JsConsole,
                span: place,
            });
        }
        if token.kind == TokenKind::Identifier {
            let statements = ["try", "function", "class", "with", "debugger"];
            if statements.contains(&token.value.as_str()) {
                return Err(unsupported(
                    &format!("{} statement", token.value),
                    "outside the portable core",
                    Some(span(&token, &token)),
                ));
            }
            let colon = self.cursor.peek_at(1);
            if colon.kind == TokenKind::Punct && colon.value == ":" {
                return Err(unsupported(
                    &format!("label {}", token.value),
                    "labels are outside the portable core; a break or continue applies to the innermost loop",
                    Some(span(&token, colon)),
                ));
            }
        }
        if self.starts_assignment() {
            let statement = self.assignment()?;
            self.cursor.eat(";");
            return Ok(statement);
        }
        let expr = self.expr()?;
        self.cursor.eat(";");
        Ok(Stmt::Expr {
            expr,
            span: self.to_here(&token),
        })
    }

    pub(super) fn const_statement(&mut self) -> Result<Stmt> {
        let start = self.cursor.advance();
        if self.cursor.is("{") {
            return self.destructuring(&start);
        }
        let (name, value, span) = self.const_binding(&start)?;
        Ok(Stmt::Const { name, value, span })
    }

    /// `const name = value;` after the `const`.
    pub(super) fn const_binding(&mut self, start: &Token) -> Result<(String, SExpr, Span)> {
        if self.cursor.is("[") {
            return Err(unsupported(
                "array destructuring",
                "arrays are outside the portable core",
                Some(self.to_here(start)),
            ));
        }
        let name = self.cursor.identifier(Some("const"))?;
        reserved(&name)?;
        if self.cursor.eat("=").is_none() {
            return Err(self.fail_here("expected = in const"));
        }
        let value = self.expr()?;
        if self.cursor.is(",") {
            return Err(unsupported(
                "several declarators",
                "declare one binding per const",
                Some(self.to_here(start)),
            ));
        }
        self.cursor.eat(";");
        self.declare_local(&name.value, false);
        Ok((name.value, value, self.to_here(start)))
    }

    /// `const { left, value: v } = t;` binds fields of a switch subject.
    pub(super) fn destructuring(&mut self, start: &Token) -> Result<Stmt> {
        self.cursor.expect("{", Some("destructuring"))?;
        let mut pairs = Vec::new();
        while !self.cursor.is("}") {
            if self.cursor.is("...") {
                let token = self.peek();
                return Err(unsupported(
                    "rest property",
                    "name every field",
                    Some(span(&token, &token)),
                ));
            }
            let key = self.cursor.identifier(Some("destructuring"))?;
            let local = if self.cursor.eat(":").is_some() {
                self.cursor.identifier(Some("destructuring"))?
            } else {
                key.clone()
            };
            reserved(&local)?;
            if self.cursor.is("=") {
                return Err(unsupported(
                    "default value",
                    "every field has a value",
                    Some(self.to_here(&key)),
                ));
            }
            pairs.push((key.value.clone(), local.value.clone(), span(&key, &local)));
            if self.cursor.eat(",").is_none() {
                break;
            }
        }
        self.cursor.expect("}", Some("destructuring"))?;
        self.cursor.expect("=", Some("destructuring"))?;
        let object = self.expr()?;
        self.cursor.eat(";");
        let whole = self.to_here(start);
        let statements = pairs
            .iter()
            .map(|(field, name, place)| Stmt::Const {
                name: name.clone(),
                value: node(
                    SNode::Field {
                        object: Box::new(object.clone()),
                        field: field.clone(),
                    },
                    *place,
                ),
                span: whole,
            })
            .collect();
        for (_, name, _) in &pairs {
            self.declare_local(name, false);
        }
        Ok(Stmt::Block {
            body: statements,
            inline: true,
            span: whole,
        })
    }

    pub(super) fn throw_statement(&mut self) -> Result<Stmt> {
        let start = self.cursor.advance();
        if self.cursor.eat("new").is_none() {
            return Err(unsupported(
                "throw of a non-error value",
                "throw new Error('…'), which aborts the program",
                Some(self.to_here(&start)),
            ));
        }
        let kind = self.cursor.identifier(Some("throw"))?;
        if !ERRORS.contains(&kind.value.as_str()) {
            return Err(unsupported(
                &format!("throw new {}", kind.value),
                "throw an Error, RangeError or TypeError",
                Some(span(&kind, &kind)),
            ));
        }
        self.cursor.expect("(", Some("throw"))?;
        let message = if self.cursor.is(")") {
            String::new()
        } else {
            let token = self.cursor.advance();
            if token.kind != TokenKind::String {
                return Err(unsupported(
                    "computed error message",
                    "the message of an abort is a string literal",
                    Some(span(&token, &token)),
                ));
            }
            token.value
        };
        self.cursor.expect(")", Some("throw"))?;
        self.cursor.eat(";");
        Ok(Stmt::Throw {
            message,
            span: self.to_here(&start),
        })
    }

    pub(super) fn if_statement(&mut self) -> Result<Stmt> {
        let start = self.cursor.advance();
        self.cursor.expect("(", Some("if"))?;
        let cond = self.expr()?;
        self.cursor.expect(")", Some("if"))?;
        let then = self.branch()?;
        let otherwise = if self.cursor.eat("else").is_some() {
            Some(self.branch()?)
        } else {
            None
        };
        Ok(Stmt::If {
            cond,
            then,
            otherwise,
            span: self.to_here(&start),
        })
    }

    pub(super) fn branch(&mut self) -> Result<Vec<Stmt>> {
        if self.cursor.is("{") {
            return self.block_statements();
        }
        if self.cursor.is("const") || self.cursor.is("let") || self.cursor.is("function") {
            return Err(self.fail_here("expected a statement"));
        }
        Ok(vec![self.statement()?])
    }

    /// `switch (x.$) { case 'tag': … }` matches the constructors of a data
    /// type; `switch (n) { case 0n: … default: … }` matches literals.
    pub(super) fn switch_statement(&mut self) -> Result<Switch> {
        let start = self.cursor.advance();
        self.cursor.expect("(", Some("switch"))?;
        let discriminant = self.expr()?;
        self.cursor.expect(")", Some("switch"))?;
        self.cursor.expect("{", Some("switch"))?;
        let tag_object = match discriminant.node {
            SNode::Field {
                ref object,
                ref field,
            } if field == "$" => Some(object.as_ref().clone()),
            _ => None,
        };
        let tagged = tag_object.is_some();
        if let Some(object) = &tag_object {
            if object.simple_name().is_none() {
                return Err(unsupported(
                    "switch on the tag of an expression",
                    "bind the value with const and switch on its tag",
                    discriminant.span,
                ));
            }
        }
        let mut clauses = Vec::new();
        let mut tests = Vec::new();
        while !self.cursor.is("}") {
            let clause_start = self.peek();
            if self.cursor.eat("default").is_some() {
                tests.push(CaseTest::Default {
                    span: span(&clause_start, &clause_start),
                });
            } else {
                self.cursor.expect("case", Some("switch"))?;
                tests.push(self.case_test(tagged)?);
            }
            self.cursor.expect(":", Some("case"))?;
            if self.cursor.is("case") || self.cursor.is("default") {
                continue;
            }
            let mut body = Vec::new();
            self.jumps.switches += 1;
            while !self.cursor.is("case") && !self.cursor.is("default") && !self.cursor.is("}") {
                if self.cursor.is("const") || self.cursor.is("let") {
                    let token = self.peek();
                    return Err(unsupported(
                        "declaration in a case clause",
                        "case clauses share one scope; wrap the case body in braces",
                        Some(span(&token, &token)),
                    ));
                }
                body.push(self.statement()?);
            }
            self.jumps.switches -= 1;
            clauses.push(Clause {
                tests: std::mem::take(&mut tests),
                body,
                span: self.to_here(&clause_start),
            });
        }
        if !tests.is_empty() {
            clauses.push(Clause {
                tests,
                body: Vec::new(),
                span: self.to_here(&start),
            });
        }
        self.cursor.expect("}", Some("switch"))?;
        let whole = self.to_here(&start);
        let (scrutinee, tag) = match tag_object {
            Some(object) => {
                let subject = object.simple_name().unwrap_or_default().to_owned();
                let data = self.switch_data(&clauses, whole)?;
                (object, Some((subject, data)))
            }
            None => (discriminant, None),
        };
        Ok(Switch {
            scrutinee,
            tag,
            clauses,
            span: whole,
        })
    }

    pub(super) fn case_test(&mut self, tagged: bool) -> Result<CaseTest> {
        let token = self.peek();
        if tagged {
            if !self.cursor.is_kind(TokenKind::String) {
                return Err(unsupported(
                    "case test",
                    "the cases of a tag switch are string literal tags",
                    Some(span(&token, &token)),
                ));
            }
            self.cursor.advance();
            return Ok(CaseTest::Tag {
                tag: token.value.clone(),
                span: span(&token, &token),
            });
        }
        let negative = self.cursor.eat("-").is_some();
        let value = self.peek();
        if value.kind == TokenKind::Number {
            self.cursor.advance();
            if value.suffix != "n" {
                return Err(unsupported(
                    "case test on a Number",
                    "a value switch compares BigInt or boolean literals; compare Numbers with if and ===",
                    Some(span(&token, &value)),
                ));
            }
            return Ok(CaseTest::NumLit {
                value: value.value.clone(),
                negative,
                span: span(&token, &value),
            });
        }
        if !negative && (self.cursor.is("true") || self.cursor.is("false")) {
            self.cursor.advance();
            return Ok(CaseTest::BoolLit {
                value: token.value == "true",
                span: span(&token, &token),
            });
        }
        Err(unsupported(
            "case test",
            "the cases of a value switch are BigInt or boolean literals",
            Some(span(&token, &value)),
        ))
    }

    /// The @typedef whose tags a tag switch names.
    pub(super) fn switch_data(&self, clauses: &[Clause], place: Span) -> Result<SData> {
        let tags: Vec<&str> = clauses
            .iter()
            .flat_map(|clause| &clause.tests)
            .filter_map(|test| match test {
                CaseTest::Tag { tag, .. } => Some(tag.as_str()),
                _ => None,
            })
            .collect();
        let candidates: Vec<&SData> = self
            .data_types
            .iter()
            .filter(|data| tags.iter().all(|tag| has_ctor(data, tag)))
            .collect();
        match candidates.as_slice() {
            [data] => Ok((*data).clone()),
            [] => Err(type_error(
                format!("no @typedef has tags {}", tags.join(", ")),
                Some(place),
            )),
            _ => Err(type_error(
                format!("tags {} are ambiguous between data types", tags.join(", ")),
                Some(place),
            )),
        }
    }

    /// Whether the next top-level statement declares or assigns variables, or is a block, if or loop.
    pub(super) fn starts_top_level_imperative(&self) -> bool {
        let words = ["let", "var", "if", "for", "while", "do"];
        if self.cursor.is_kind(TokenKind::Identifier)
            && words.iter().any(|word| self.cursor.is(word))
        {
            return true;
        }
        self.cursor.is("{") || self.starts_assignment()
    }

    /// A top-level statement read as a function body's, where `return` has no function to leave.
    pub(super) fn top_level_statement(&mut self) -> Result<Stmt> {
        self.top_level = true;
        let statement = self.statement();
        self.top_level = false;
        statement
    }

    /// Top-level statements that declare or assign variables are one
    /// expression, whose value carries the top-level variables they declare
    /// or assign; main binds each of them for the statements after it.
    pub(super) fn flush(
        &mut self,
        pending: &mut Vec<Stmt>,
        effects: &mut Vec<SEffect>,
    ) -> Result<()> {
        if pending.is_empty() {
            return Ok(());
        }
        let statements = std::mem::take(pending);
        let variables: Vec<String> = effects
            .iter()
            .filter_map(|effect| match effect {
                SEffect::Let { name, .. } if !name.starts_with("ml_") => Some(name.clone()),
                _ => None,
            })
            .collect();
        let bounds = |statement: &Stmt| statement_span(statement).unwrap_or(Span::new(0, 0));
        let place = Span::new(
            bounds(&statements[0]).start,
            bounds(&statements[statements.len() - 1]).end,
        );
        effects.extend(lower_top_level(self, &statements, &variables, place)?);
        Ok(())
    }

    pub(super) fn main_statement(&mut self) -> Result<SEffect> {
        let token = self.peek();
        if self.cursor.is("const") {
            if self.cursor.is_at("{", 1) {
                return Err(unsupported(
                    "top-level destructuring",
                    "bind each value with const",
                    Some(span(&token, self.cursor.peek_at(1))),
                ));
            }
            let start = self.cursor.advance();
            let (name, value, span) = self.const_binding(&start)?;
            return Ok(SEffect::Let {
                name,
                ty: None,
                value,
                span: Some(span),
            });
        }
        if self.cursor.is("console") && self.cursor.is_at(".", 1) {
            return self.console_statement();
        }
        if let Some(assertion) = &self.assertion {
            if self.cursor.is(&assertion.name) {
                return self.assert_statement();
            }
        }
        let statements = ["switch", "try", "throw", "class", "return"];
        if token.kind == TokenKind::Identifier && statements.contains(&token.value.as_str()) {
            return Err(unsupported(
                &format!("top-level {} statement", token.value),
                "the top level prints with console.log, binds with const or let, assigns, branches with if, loops and asserts",
                Some(span(&token, &token)),
            ));
        }
        Err(unsupported(
            "top-level expression statement",
            "a statement that discards its value has no portable effect",
            Some(span(&token, &token)),
        ))
    }

    pub(super) fn console_statement(&mut self) -> Result<SEffect> {
        let (expr, place) = self.console_call()?;
        Ok(SEffect::Print {
            expr,
            style: ShowStyle::JsConsole,
            span: Some(place),
        })
    }

    /// `console.log(expr);`: the printed expression and the statement's span.
    fn console_call(&mut self) -> Result<(SExpr, Span)> {
        let start = self.cursor.advance();
        self.cursor.expect(".", Some("console"))?;
        let method = self.cursor.identifier(Some("console"))?;
        if method.value != "log" {
            return Err(unsupported(
                &format!("console.{}", method.value),
                "console.log, which writes a line to standard output, is the portable output",
                Some(span(&start, &method)),
            ));
        }
        let mut args = self.arguments("console.log")?;
        self.cursor.eat(";");
        let place = self.to_here(&start);
        if args.len() > 1 {
            return Err(unsupported(
                "console.log with several arguments",
                "several arguments are formatted by util.format; pass one string",
                Some(place),
            ));
        }
        let expr = args.pop().unwrap_or_else(|| {
            node(
                SNode::Str {
                    value: String::new(),
                },
                place,
            )
        });
        Ok((expr, place))
    }

    pub(super) fn assert_statement(&mut self) -> Result<SEffect> {
        let start = self.cursor.advance();
        let method = if self.cursor.eat(".").is_some() {
            Some(self.cursor.identifier(Some("assertion"))?.value)
        } else {
            None
        };
        let mut args = self.arguments("assertion")?;
        self.cursor.eat(";");
        let place = self.to_here(&start);
        let (local, strict) = self
            .assertion
            .as_ref()
            .map_or((String::new(), false), |assertion| {
                (assertion.name.clone(), assertion.strict)
            });
        let name = method
            .as_ref()
            .map_or_else(|| local.clone(), |method| format!("{local}.{method}"));
        if method.is_none() || method.as_deref() == Some("ok") {
            if args.len() != 1 {
                return Err(unsupported(
                    &format!("{name} message"),
                    "custom assertion messages are not kept",
                    Some(place),
                ));
            }
            let mut prop = prop_of(args.remove(0));
            prop.span = Some(place);
            return Ok(SEffect::Assert {
                prop,
                span: Some(place),
            });
        }
        let Some(kind) = assertion_kind(method.as_deref().unwrap_or_default(), strict) else {
            return Err(unsupported(
                &name,
                "the portable assertions are assert, ok, strictEqual, notStrictEqual and deepStrictEqual",
                Some(place),
            ));
        };
        let Ok([left, right]) = <[SExpr; 2]>::try_from(args) else {
            return Err(unsupported(
                &format!("{name} message"),
                "custom assertion messages are not kept",
                Some(place),
            ));
        };
        let (reference, equal) = match kind {
            AssertionKind::Eq => (true, true),
            AssertionKind::Ne => (true, false),
            AssertionKind::Deep => (false, true),
            AssertionKind::NotDeep => (false, false),
        };
        // These assertions compare primitives with Object.is (SameValue): NaN equals NaN, and 0 differs from -0.
        let comparison = SComparison {
            left,
            right,
            reference,
            same_value: true,
        };
        let prop = SProp {
            node: if equal {
                SPropNode::Eq(comparison)
            } else {
                SPropNode::Ne(comparison)
            },
            span: Some(place),
        };
        Ok(SEffect::Assert {
            prop,
            span: Some(place),
        })
    }

    pub(super) fn arguments(&mut self, context: &str) -> Result<Vec<SExpr>> {
        self.cursor.expect("(", Some(context))?;
        let mut args = Vec::new();
        while !self.cursor.is(")") {
            if self.cursor.is("...") {
                let token = self.peek();
                return Err(unsupported(
                    "spread argument",
                    "pass each argument",
                    Some(span(&token, &token)),
                ));
            }
            args.push(self.expr()?);
            if self.cursor.eat(",").is_none() {
                break;
            }
        }
        self.cursor.expect(")", Some(context))?;
        Ok(args)
    }
}
