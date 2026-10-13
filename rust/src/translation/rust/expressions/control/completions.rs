//! Thin closed-completion orchestration over the shared generated policy.
//! Mirrors `js/src/translation/rust-completions.js`; no new semantic operation.

use super::super::super::completion_rules::{
    accept_rust_return_ownership, read_rust_completion_kind, read_rust_conditional_completion,
};
use super::super::super::{
    Options, Result, RustParser, SExpr, SNode, Token, TokenKind, is_if_or_match, span, unsupported,
};

struct Completion {
    kind: String,
    expression: SExpr,
}

impl RustParser {
    pub(in super::super::super) fn closed_function_completion(
        &mut self,
        path: &[String],
    ) -> Result<SExpr> {
        Ok(self.completion_block(path)?.expression)
    }

    fn completion_block(&mut self, path: &[String]) -> Result<Completion> {
        let open = self.cursor.expect("{", Some("block"))?;
        let previous = self.origins.enter(None);
        let result = self.completion_sequence(path, &open).and_then(|result| {
            self.cursor.expect("}", Some("block"))?;
            Ok(result)
        });
        self.origins.scope = previous;
        result
    }

    fn completion_sequence(&mut self, path: &[String], open: &Token) -> Result<Completion> {
        let mut bindings = Vec::new();
        while self.cursor.is("let") {
            bindings.extend(self.let_statement(path)?);
        }
        let token = self.peek();
        if self.cursor.is("}") {
            return Err(unsupported(
                "block without a value",
                "the block evaluates to (), which is not a portable result",
                span(open, &token),
            ));
        }
        let mut result = if self.cursor.is("return") {
            self.return_completion(path)?
        } else if self.cursor.is("if") {
            self.conditional_completion(path, open)?
        } else {
            self.tail_completion(path)?
        };
        let origin = self.origins.origin(&result.expression, path);
        result.expression = bindings
            .into_iter()
            .rev()
            .fold(result.expression, |body, binding| {
                SExpr::new(
                    SNode::Let {
                        name: binding.name,
                        ty: binding.ty,
                        value: Box::new(binding.value),
                        body: Box::new(body),
                    },
                    binding.span,
                )
            });
        self.origins.mark(&result.expression, origin);
        Ok(result)
    }

    fn return_completion(&mut self, path: &[String]) -> Result<Completion> {
        let token = self.cursor.advance();
        if self.cursor.is(";") || self.cursor.is("}") {
            return Err(unsupported(
                "return without a value",
                "the selected completion model requires a portable value",
                span(&token, self.cursor.peek()),
            ));
        }
        let expression = self.expr(path, Options::default())?;
        let origin = self.origins.origin(&expression, path);
        if !accept_rust_return_ownership(origin.proven, !origin.wrappers.is_empty()) {
            return Err(unsupported(
                "Rust return ownership",
                "the selected completion model requires proved direct value ownership; escaping or indirect returns need their own proof",
                expression.span,
            ));
        }
        if self.cursor.eat(";").is_none() && !self.cursor.is("}") {
            return Err(Self::fail("expected ; or }", self.cursor.peek()));
        }
        if !self.cursor.is("}") {
            return Err(unsupported(
                "unreachable continuation",
                "statements after a completed return are not removed by this model",
                span(&token, self.cursor.peek()),
            ));
        }
        Ok(Completion {
            kind: "returned".into(),
            expression,
        })
    }

    fn tail_completion(&mut self, path: &[String]) -> Result<Completion> {
        let token = self.peek();
        if token.kind == TokenKind::Identifier
            && ["while", "loop", "for"].contains(&token.value.as_str())
        {
            return Err(unsupported(
                &format!("{} loop", token.value),
                "loops over mutable state are outside the portable core; use recursion",
                span(&token, &token),
            ));
        }
        let expression = self.expr(
            path,
            Options {
                statement: true,
                no_struct: false,
            },
        )?;
        let abort = matches!(expression.node, SNode::Abort { .. });
        if self.cursor.eat(";").is_some() && !abort {
            return Err(unsupported(
                "expression statement",
                "statements with effects are outside the portable core in function bodies",
                expression.span,
            ));
        }
        if !self.cursor.is("}") {
            if abort {
                return Err(unsupported(
                    "unreachable continuation",
                    "statements after an abort are not removed by this model",
                    expression.span,
                ));
            }
            if is_if_or_match(&expression) {
                return Err(unsupported(
                    "expression statement",
                    "a block value must be its last expression",
                    expression.span,
                ));
            }
            return Err(Self::fail("expected }", self.cursor.peek()));
        }
        Ok(Completion {
            kind: if abort { "aborted" } else { "value" }.into(),
            expression,
        })
    }

    fn conditional_completion(&mut self, path: &[String], open: &Token) -> Result<Completion> {
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
        let then = self.completion_block(path)?;
        let has_else = self.cursor.eat("else").is_some();
        let otherwise = if has_else {
            Some(if self.cursor.is("if") {
                self.conditional_completion(path, open)?
            } else {
                self.completion_block(path)?
            })
        } else {
            None
        };
        let terminated = self.cursor.eat(";").is_some();
        let action = read_rust_conditional_completion(
            &then.kind,
            otherwise.as_ref().map_or("", |result| result.kind.as_str()),
            has_else,
            !self.cursor.is("}"),
        );
        let otherwise =
            self.completion_continuation(otherwise, &action, terminated, path, open, &token)?;
        let kind = read_rust_completion_kind(&action, &then.kind, &otherwise.kind);
        let origin = self.origins.join(&[
            self.origins.origin(&then.expression, path),
            self.origins.origin(&otherwise.expression, path),
        ]);
        let expression = SExpr::new(
            SNode::If {
                cond: Box::new(cond),
                then: Box::new(then.expression),
                otherwise: Box::new(otherwise.expression),
            },
            span(&token, self.cursor.peek()),
        );
        self.origins.mark(&expression, origin);
        Ok(Completion { kind, expression })
    }
    fn completion_continuation(
        &mut self,
        otherwise: Option<Completion>,
        action: &str,
        terminated: bool,
        path: &[String],
        open: &Token,
        token: &Token,
    ) -> Result<Completion> {
        let otherwise = match action {
            "guarded continuation" => self.completion_sequence(path, open)?,
            "returned conditional" | "aborted conditional" => otherwise.ok_or_else(|| {
                unsupported(
                    "guarded return completion",
                    "the shared policy requires both branches",
                    span(token, self.cursor.peek()),
                )
            })?,
            "conditional value" if !terminated => otherwise.ok_or_else(|| {
                unsupported(
                    "guarded return completion",
                    "the shared policy requires both branches",
                    span(token, self.cursor.peek()),
                )
            })?,
            "conditional value" => {
                return Err(unsupported(
                    "expression statement",
                    "statements with effects are outside the portable core in function bodies",
                    span(token, self.cursor.peek()),
                ));
            }
            "unreachable continuation" => {
                return Err(unsupported(
                    "unreachable continuation",
                    "statements after a completed return are not removed by this model",
                    span(token, self.cursor.peek()),
                ));
            }
            "discarded conditional value" => {
                return Err(unsupported(
                    "expression statement",
                    "a block value must be its last expression",
                    span(token, self.cursor.peek()),
                ));
            }
            _ => {
                return Err(unsupported(
                    "guarded return completion",
                    "every guard branch must complete a return or abort, with an enclosing continuation; mixed return/value branches need a separate completion proof",
                    span(token, self.cursor.peek()),
                ));
            }
        };
        Ok(otherwise)
    }
}
