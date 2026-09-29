//! `let`, assignments, `while`, `do … while`, `for`, `break` and `continue`.

use super::{
    joined, node, span, unsupported, JavaScriptParser, Jumps, Result, SExpr, SNode, Stmt, Token,
    TokenKind, TranslationError, ASSIGNMENTS, COMPOUND,
};
use crate::translation::surface::BinaryOp;

impl JavaScriptParser {
    /// `let a = 1n, b = a;` declares assignable locals; each needs a value, since undefined is not portable.
    pub(super) fn let_declarations(&mut self) -> Result<Vec<Stmt>> {
        let start = self.cursor.advance();
        if self.cursor.is("{") || self.cursor.is("[") {
            return Err(unsupported(
                "let destructuring",
                "declare each binding with its own let",
                Some(self.to_here(&start)),
            ));
        }
        let mut statements = Vec::new();
        loop {
            let name = self.cursor.identifier(Some("let"))?;
            reserved(&name)?;
            // The binding is in its temporal dead zone in its own initialiser.
            self.scope.tdz.insert(name.value.clone());
            if self.cursor.eat("=").is_none() {
                return Err(unsupported(
                    &format!("let {} without a value", name.value),
                    "an uninitialised let holds undefined, which is not a portable value; give it an initial value",
                    Some(span(&start, &name)),
                ));
            }
            let value = self.expr()?;
            self.declare_local(&name.value, true);
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
        let op = match prefix {
            Some(prefix) => prefix.value,
            None => self.cursor.advance().value,
        };
        let place = self.to_here(&start);
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
        let start = self.cursor.advance();
        if self.cursor.is("await") {
            return Err(unsupported(
                "for await",
                "asynchronous iteration is outside the portable core",
                Some(self.to_here(&start)),
            ));
        }
        self.cursor.expect("(", Some("for"))?;
        let declared =
            usize::from(self.cursor.is("let") || self.cursor.is("const") || self.cursor.is("var"));
        if self.cursor.is_kind_at(TokenKind::Identifier, declared)
            && (self.cursor.is_at("of", declared + 1) || self.cursor.is_at("in", declared + 1))
        {
            let kind = self.cursor.peek_at(declared + 1).value.clone();
            return Err(unsupported(
                &format!("for…{kind} loop"),
                "iteration over arrays, strings and objects is outside the portable core; count with for (let i = …; …; …)",
                Some(self.to_here(&start)),
            ));
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

    pub(super) fn jump_statement(&mut self) -> Result<Stmt> {
        let token = self.cursor.advance();
        let place = span(&token, &token);
        let next = self.cursor.peek().start;
        // A line break after break or continue ends the statement, so a following name is not a label.
        let newline =
            token.end < next && self.source.units()[token.end..next].contains(&u16::from(b'\n'));
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

/// Local names with the translator's `ml_` prefix could meet the names it makes up.
pub(super) fn reserved(token: &Token) -> Result<()> {
    if token.value.starts_with("ml_") {
        return Err(unsupported(
            "reserved identifier",
            &format!("{} uses the translator's reserved ml_ prefix", token.value),
            Some(span(token, token)),
        ));
    }
    Ok(())
}
