//! `let`, assignments, `while`, `do … while`, `for`, `break` and `continue`.

use super::{
    ASSIGNMENTS, COMPOUND, HashSet, JavaScriptParser, Jumps, Result, SExpr, SNode, Stmt, Token,
    TokenKind, TranslationError, accept_java_script_line_terminator, joined, node, span,
    unsupported,
};
use crate::translation::surface::BinaryOp;

impl JavaScriptParser {
    /// `let a = 1n, b = a;` declares assignable locals; omission starts with undefined.
    pub(super) fn let_declarations(&mut self) -> Result<Vec<Stmt>> {
        let start = self.cursor.advance();
        if self.cursor.is("{") {
            return Err(unsupported(
                "let destructuring",
                "declare each binding with its own let",
                Some(self.to_here(&start)),
            ));
        }
        let mut statements = Vec::new();
        let mut names = HashSet::new();
        loop {
            if self.cursor.is("[") {
                statements.extend(self.mutable_array_declaration(&start, &mut names)?);
                if self.cursor.eat(",").is_none() {
                    return Ok(statements);
                }
                continue;
            }
            if self.cursor.is("{") {
                return Err(unsupported(
                    "let destructuring",
                    "declare each binding with its own let",
                    Some(self.to_here(&start)),
                ));
            }
            let name = self.cursor.identifier(Some("let"))?;
            reserved(&name)?;
            if !names.insert(name.value.clone()) {
                return Err(TranslationError::syntax(
                    format!("duplicate lexical binding {}", name.value),
                    Some(name.span()),
                ));
            }
            // The binding is in its temporal dead zone in its own initialiser.
            self.scope.tdz.insert(name.value.clone());
            let value = if self.cursor.eat("=").is_some() {
                self.expr()?
            } else {
                node(
                    SNode::None {
                        absence:
                            crate::translation::optional_value_rules::read_optional_source_literal(
                                "undefined",
                                false,
                            ),
                    },
                    span(&start, &name),
                )
            };
            self.declare_local(&name.value, true);
            self.lexical
                .binding(&name.value, Some(self.cursor.peek().start));
            statements.push(Stmt::Let {
                name: name.value,
                value,
                span: self.to_here(&start),
            });
            if self.cursor.eat(",").is_none() {
                return Ok(statements);
            }
        }
    }

    /// Whether the statement is `x = e`, `x op= e`, `x++`, `x--`, `++x` or `--x`.
    pub(super) fn starts_assignment(&self) -> bool {
        if (self.cursor.is("++") || self.cursor.is("--"))
            && self.cursor.is_kind_at(TokenKind::Identifier, 1)
        {
            return true;
        }
        if !self.cursor.is_kind(TokenKind::Identifier) {
            return false;
        }
        let next = self.cursor.peek_at(1);
        next.kind == TokenKind::Punct
            && (ASSIGNMENTS.contains(&next.value.as_str())
                || next.value == "++"
                || next.value == "--")
    }

    /// An assignment of a local: `x op= e` is `x = x op e`, and `x++` is `x = x + 1`.
    pub(super) fn assignment(&mut self) -> Result<Stmt> {
        let start = self.peek();
        let prefix = (self.cursor.is("++") || self.cursor.is("--")).then(|| self.cursor.advance());
        let target = self.cursor.identifier(Some("assignment"))?;
        let operator = prefix.unwrap_or_else(|| self.cursor.advance());
        let op = operator.value.clone();
        let place = self.to_here(&start);
        self.lexical.read(&target, true, place);
        if self.scope.tdz.contains(&target.value) {
            return Err(unsupported(
                &format!("assignment of {} before its declaration", target.value),
                "the binding is in its temporal dead zone, where assigning it throws a ReferenceError",
                Some(place),
            ));
        }
        if !self.scope.locals.contains(&target.value) {
            return Err(unsupported(
                &format!("assignment of {}", target.value),
                "only local variables declared with let, and parameters, are assignable",
                Some(place),
            ));
        }
        if !self.scope.mutable.contains(&target.value) {
            return Err(unsupported(
                &format!("assignment of constant {}", target.value),
                "assigning a const binding throws a TypeError; declare it with let",
                Some(place),
            ));
        }
        let read = node(
            SNode::Name {
                path: vec![target.value.clone()],
            },
            span(&target, &target),
        );
        if op == "??=" {
            let expr =
                self.scalar_assignment_expression(read, &operator, false, Some(Self::expr))?;
            return Ok(Stmt::Expr {
                expr,
                span: self.to_here(&start),
            });
        }
        let binary = |op, left: SExpr, right: SExpr, place| {
            node(
                SNode::Binary {
                    op,
                    left: Box::new(left),
                    right: Box::new(right),
                    rounding: None,
                },
                place,
            )
        };
        let value = if op == "++" || op == "--" {
            // One of the variable's own type: a Number or a BigInt.
            let one = node(
                SNode::Num {
                    value: "1".to_owned(),
                    ty: None,
                    negative: false,
                    unit: true,
                },
                place,
            );
            let op = if op == "++" {
                BinaryOp::Add
            } else {
                BinaryOp::Sub
            };
            binary(op, read, one, place)
        } else if op == "=" {
            self.expr()?
        } else if let Some((_, compound)) = COMPOUND.iter().find(|(text, _)| *text == op) {
            let right = self.expr()?;
            let whole = joined(&read, &right, &start);
            binary(*compound, read, right, whole)
        } else if op=="**=" {
            let right=self.expr()?;
            let whole=joined(&read,&right,&start);
            node(SNode::Math{op:"exponentiateJavaScriptNumericValue".to_owned(),name:op,args:vec![super::SArrayItem::new(read,false),super::SArrayItem::new(right,false)]},whole)
        } else if !crate::translation::number_bitwise_rules::read_java_script_number_bitwise_assignment(&op).is_empty() {
            let right = self.expr()?;
            let whole = joined(&read, &right, &start);
            node(SNode::Math {
                op: crate::translation::number_bitwise_rules::read_java_script_number_bitwise_assignment(&op),
                name: op,
                args: vec![super::SArrayItem::new(read, false), super::SArrayItem::new(right, false)],
            }, whole)
        } else {
            return Err(unsupported(
                &format!("{op} assignment"),
                "the portable compound assignments are +=, -=, *=, /=, %=, &&= and ||=",
                Some(place),
            ));
        };
        Ok(Stmt::Assign {
            name: target.value,
            value,
            span: self.to_here(&start),
        })
    }

    /// The body of a loop, where `break` and `continue` apply to it.
    fn loop_body(&mut self) -> Result<Vec<Stmt>> {
        self.jumps.loops += 1;
        let body = self.branch();
        self.jumps.loops -= 1;
        body
    }

    pub(super) fn while_statement(&mut self) -> Result<Stmt> {
        let start = self.cursor.advance();
        self.cursor.expect("(", Some("while"))?;
        let cond = self.expr()?;
        self.cursor.expect(")", Some("while"))?;
        let body = self.loop_body()?;
        Ok(Stmt::While {
            cond,
            body,
            span: self.to_here(&start),
        })
    }

    pub(super) fn do_statement(&mut self) -> Result<Stmt> {
        let start = self.cursor.advance();
        let body = self.loop_body()?;
        self.cursor.expect("while", Some("do"))?;
        self.cursor.expect("(", Some("do"))?;
        let cond = self.expr()?;
        self.cursor.expect(")", Some("do"))?;
        self.cursor.eat(";");
        Ok(Stmt::DoWhile {
            body,
            cond,
            span: self.to_here(&start),
        })
    }

    /// `for (let i = 0n; i < n; i++) …`: the variables of its head belong to the loop.
    pub(super) fn for_statement(&mut self) -> Result<Stmt> {
        self.with_lexical_scope(&[], 0, Self::for_statement_in_scope)
    }

    fn for_statement_in_scope(&mut self) -> Result<Stmt> {
        let start = self.cursor.advance();
        if self.cursor.is("await") {
            return Err(unsupported(
                "for await",
                "asynchronous iteration is outside the portable core",
                Some(self.to_here(&start)),
            ));
        }
        self.cursor.expect("(", Some("for"))?;
        if (self.cursor.is("const") || self.cursor.is("let"))
            && self.cursor.is_at("[", 1)
            && self
                .cursor
                .tokens
                .get(self.matching(self.cursor.index + 1) + 1)
                .is_some_and(|token| token.value == "of")
        {
            return self.for_of(&start);
        }
        let declared =
            usize::from(self.cursor.is("let") || self.cursor.is("const") || self.cursor.is("var"));
        if self.cursor.is_kind_at(TokenKind::Identifier, declared)
            && (self.cursor.is_at("of", declared + 1) || self.cursor.is_at("in", declared + 1))
        {
            if self.cursor.is_at("in", declared + 1) {
                return Err(unsupported(
                    "for…in loop",
                    "iteration over the keys of objects is outside the portable core; count with for (let i = …; …; …)",
                    Some(self.to_here(&start)),
                ));
            }
            if declared == 0 || self.cursor.is("var") {
                return Err(unsupported(
                    "for…of without const or let",
                    "declare the loop variable with const or let, so each iteration has its own",
                    Some(self.to_here(&start)),
                ));
            }
            return self.for_of(&start);
        }
        let outer = self.scope.clone();
        let result = self.for_rest(&start);
        self.scope = outer;
        result
    }

    fn for_rest(&mut self, start: &Token) -> Result<Stmt> {
        let mut init = Vec::new();
        if self.cursor.is("let") {
            init.extend(self.let_declarations()?);
        } else if self.cursor.is("const") || self.cursor.is("var") {
            let token = self.peek();
            return Err(unsupported(
                &format!("for with {}", token.value),
                "declare the loop variables with let",
                Some(span(&token, &token)),
            ));
        } else {
            while !self.cursor.is(";") {
                if !self.starts_assignment() {
                    return Err(self.fail_here("expected an assignment in the for initialiser"));
                }
                init.push(self.assignment()?);
                if self.cursor.eat(",").is_none() {
                    break;
                }
            }
        }
        self.cursor.expect(";", Some("for"))?;
        let cond = if self.cursor.is(";") {
            None
        } else {
            Some(self.expr()?)
        };
        self.cursor.expect(";", Some("for"))?;
        let mut update = Vec::new();
        while !self.cursor.is(")") {
            if !self.starts_assignment() {
                let token = self.peek();
                return Err(unsupported(
                    "for update",
                    "the update of a for loop is a list of assignments",
                    Some(span(&token, &token)),
                ));
            }
            update.push(self.assignment()?);
            if self.cursor.eat(",").is_none() {
                break;
            }
        }
        self.cursor.expect(")", Some("for"))?;
        let body = self.loop_body()?;
        Ok(Stmt::For {
            init,
            cond,
            update,
            body,
            span: self.to_here(start),
        })
    }

    /// `for (const x of xs) body` reads the elements of an array in order: it
    /// is a loop over the indices of the array, whose body first binds `x` to
    /// the element at the index. The array is evaluated once, before the loop.
    fn for_of(&mut self, start: &Token) -> Result<Stmt> {
        let mutable = self.cursor.advance().value == "let";
        let bindings = if self.cursor.is("[") {
            Some(self.read_array_binding()?)
        } else {
            None
        };
        let name = if bindings.is_none() {
            Some(self.cursor.identifier(Some("for…of"))?)
        } else {
            None
        };
        if let Some(name) = &name {
            reserved(name)?;
        }
        self.cursor.expect("of", Some("for…of"))?;
        let value = if let Some(bindings) = &bindings {
            self.array_binding_initializer(bindings)?
        } else {
            self.expr()?
        };
        let iterable_span = value.span;
        let iterable = SExpr::new(
            SNode::ForOfIterable {
                object: Box::new(value),
                ty: None,
            },
            iterable_span,
        );
        self.cursor.expect(")", Some("for"))?;
        self.generated_count += 1;
        let values = format!(
            "{}temporary_values{}",
            self.generated_prefix, self.generated_count
        );
        let index = format!(
            "{}temporary_index{}",
            self.generated_prefix, self.generated_count
        );
        let outer = self.scope.clone();
        if let Some(bindings) = &bindings {
            for binding in bindings {
                self.declare_local(&binding.name, mutable);
                self.lexical
                    .binding(&binding.name, Some(self.cursor.peek().start));
            }
        } else if let Some(name) = &name {
            self.declare_local(&name.value, mutable);
            self.lexical
                .binding(&name.value, Some(self.cursor.peek().start));
        }
        let body = self.loop_body();
        self.scope = outer;
        let body = body?;
        let place = self.to_here(start);
        let read = |local: &str| {
            node(
                SNode::Name {
                    path: vec![local.to_owned()],
                },
                place,
            )
        };
        let number = |value: &str| {
            node(
                SNode::Num {
                    value: value.to_owned(),
                    ty: None,
                    negative: false,
                    unit: false,
                },
                place,
            )
        };
        let binary = |op, left, right| {
            node(
                SNode::Binary {
                    op,
                    left: Box::new(left),
                    right: Box::new(right),
                    rounding: None,
                },
                place,
            )
        };
        let element = node(
            SNode::Index {
                object: Box::new(read(&values)),
                index: Box::new(read(&index)),
                optional: None,
                proven_in_bounds: Some(true.into()),
            },
            place,
        );
        let length = node(
            SNode::Length {
                object: Box::new(read(&values)),
                integer: true,
            },
            place,
        );
        let mut declarations = if let Some(bindings) = bindings {
            let element_name = format!(
                "{}temporary_element{}",
                self.generated_prefix, self.generated_count
            );
            let mut declarations = vec![Stmt::Const {
                name: element_name.clone(),
                value: element,
                span: place,
            }];
            declarations.extend(self.bind_array_elements(
                bindings,
                &read(&element_name),
                mutable,
                place,
            ));
            declarations
        } else {
            let name = name
                .expect("the ordinary for-of form has an actual lexical binder")
                .value;
            vec![if mutable {
                Stmt::Let {
                    name,
                    value: element,
                    span: place,
                }
            } else {
                Stmt::Const {
                    name,
                    value: element,
                    span: place,
                }
            }]
        };
        declarations.push(Stmt::Block {
            body,
            inline: false,
            span: place,
        });
        self.owned_array_iteration_sources
            .insert((place, values.clone(), index.clone()));
        Ok(Stmt::Block {
            body: vec![
                Stmt::Const {
                    name: values,
                    value: iterable,
                    span: place,
                },
                Stmt::For {
                    init: vec![Stmt::Let {
                        name: index.clone(),
                        value: number("0"),
                        span: place,
                    }],
                    cond: Some(binary(BinaryOp::Lt, read(&index), length)),
                    update: vec![Stmt::Assign {
                        name: index.clone(),
                        value: binary(BinaryOp::Add, read(&index), number("1")),
                        span: place,
                    }],
                    body: declarations,
                    span: place,
                },
            ],
            inline: false,
            span: place,
        })
    }

    pub(super) fn jump_statement(&mut self) -> Result<Stmt> {
        let token = self.cursor.advance();
        let place = span(&token, &token);
        let next = self.cursor.peek().start;
        // A line break after break or continue ends the statement, so a following name is not a label.
        let newline = self.source.units()[token.end..next]
            .iter()
            .any(|unit| accept_java_script_line_terminator(f64::from(*unit)));
        if self.cursor.is_kind(TokenKind::Identifier)
            && !self.cursor.is("case")
            && !self.cursor.is("default")
            && !newline
        {
            return Err(unsupported(
                &format!("labelled {}", token.value),
                "labels are outside the portable core; a break or continue applies to the innermost loop",
                Some(self.to_here(&token)),
            ));
        }
        let Jumps { loops, switches } = self.jumps;
        if token.value == "continue" && loops == 0 {
            return Err(TranslationError::syntax(
                "continue outside a loop",
                Some(place),
            ));
        }
        if token.value == "break" && loops == 0 && switches == 0 {
            return Err(TranslationError::syntax(
                "break outside a loop or switch",
                Some(place),
            ));
        }
        self.cursor.eat(";");
        Ok(if token.value == "break" {
            Stmt::Break { span: place }
        } else {
            Stmt::Continue { span: place }
        })
    }
}

/// Source binding roles retain the ECMAScript early errors.
pub(super) fn reserved(token: &Token) -> Result<()> {
    crate::translation::lexer::require_java_script_identifier_role(token, true)
}
