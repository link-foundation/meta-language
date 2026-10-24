//! Ordinary source control flow; actual terms follow shared source decisions.
use super::{
    Binding, Options, Result, RustParser, SExpr, SNode, Token, TokenKind, is_if_or_match, span,
    unsupported,
};
use crate::translation::rust_unit_value_rules as rules;

struct Completion {
    kind: String,
    expression: SExpr,
}

impl RustParser {
    pub(super) fn ordinary_completion_selected(&self) -> bool {
        let profiles = [
            self.completion_model.clone(),
            self.source_signature_model.clone(),
            self.scalar_loop_model.clone(),
            self.primitive_owner_model.clone(),
            self.empty_option_source_model.clone(),
            self.type_namespace_model.clone(),
            self.pure_unit_call_model.clone(),
            self.local_value_state_model.clone(),
            self.const_function_model.clone(),
            self.shared_array_handle_source_model.clone(),
            self.origins.array_result_model.clone(),
        ];
        if !rules::accept_rust_ordinary_completion_profiles(&profiles) {
            return false;
        }
        let mut count = 64usize;
        loop {
            let values = self
                .cursor
                .tokens
                .iter()
                .skip(self.cursor.index)
                .take(count)
                .enumerate()
                .map(|(index, token)| {
                    if token.kind == TokenKind::Punct && (token.value == "{" || token.value == "}")
                    {
                        token.value.clone()
                    } else if self.cursor.is_at("return", index) {
                        "return".to_owned()
                    } else {
                        String::new()
                    }
                })
                .collect::<Vec<_>>();
            let presence = rules::read_rust_function_return_presence(&values);
            if presence != -1.0 {
                return presence == 1.0;
            }
            if count >= self.cursor.tokens.len() - self.cursor.index {
                return false;
            }
            count = count.saturating_mul(2);
        }
    }
    pub(super) fn ordinary_function_completion(&mut self, path: &[String]) -> Result<SExpr> {
        Ok(self.ordinary_completion_block(path)?.expression)
    }
    fn ordinary_completion_block(&mut self, path: &[String]) -> Result<Completion> {
        let open = self.cursor.expect("{", Some("block"))?;
        let previous = self.origins.enter(None);
        let result = self
            .ordinary_completion_sequence(path, &open)
            .and_then(|result| {
                self.cursor.expect("}", Some("block"))?;
                Ok(result)
            });
        self.origins.scope = previous;
        result
    }
    fn ordinary_completion_sequence(
        &mut self,
        path: &[String],
        open: &Token,
    ) -> Result<Completion> {
        let mut bindings = Vec::new();
        let mut result = loop {
            if self.cursor.eat(";").is_some() {
                continue;
            }
            if self.cursor.is("let") {
                bindings.extend(self.let_statement(path)?);
                continue;
            }
            let token = self.peek();
            if self.cursor.is("}") {
                break Completion {
                    kind: "value".into(),
                    expression: SExpr::new(SNode::Unit, span(open, &token)),
                };
            }
            if self.cursor.is("return") {
                let bare = self.cursor.is_at(";", 1) || self.cursor.is_at("}", 1);
                if !bare && !self.is_terminal_return() {
                    return Err(unsupported(
                        "early return",
                        "write the result as the tail expression of the block",
                        span(&token, &token),
                    ));
                }
                self.cursor.advance();
                let expression = if bare {
                    SExpr::new(SNode::Unit, span(&token, &token))
                } else {
                    self.expr(path, Options::default())?
                };
                if self.cursor.eat(";").is_none() && !self.cursor.is("}") {
                    return Err(Self::fail("expected ; or }", self.cursor.peek()));
                }
                while self.cursor.eat(";").is_some() {}
                if !self.cursor.is("}") {
                    return Err(unsupported(
                        "unreachable continuation",
                        "statements after a completed return are not removed by this model",
                        span(&token, self.cursor.peek()),
                    ));
                }
                break Completion {
                    kind: "returned".into(),
                    expression,
                };
            }
            if self.cursor.is("if") {
                break self.ordinary_conditional_completion(path, open)?;
            }
            if token.kind == TokenKind::Identifier
                && ["while", "loop", "for"].contains(&token.value.as_str())
            {
                return Err(unsupported(
                    &format!("{} loop", token.value),
                    "loops over mutable state are outside the portable core; use recursion",
                    span(&token, &token),
                ));
            }
            let expression = if token.kind == TokenKind::Macro
                && rules::read_rust_function_macro_disposition(&token.value) == "print"
            {
                self.function_print_value(path)?
            } else {
                self.expr(
                    path,
                    Options {
                        statement: true,
                        no_struct: false,
                    },
                )?
            };
            let terminated = self.cursor.eat(";").is_some();
            let abort = matches!(expression.node, SNode::Abort { .. });
            if terminated && !abort {
                let origin = self.origins.origin(&expression, path);
                let disposition = rules::read_rust_unit_statement_disposition(
                    expression.node.kind(),
                    self.origins.collecting,
                    origin.proven,
                    origin.ty.as_ref().map_or("", super::Type::kind),
                    !origin.wrappers.is_empty(),
                );
                if disposition != "bind" {
                    return Err(unsupported(
                        "expression statement",
                        "statements with effects are outside the portable core in function bodies",
                        expression.span,
                    ));
                }
                let place = expression.span;
                bindings.push(Binding {
                    name: self.fresh_product_binding(),
                    ty: None,
                    value: expression,
                    span: place,
                });
                continue;
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
            break Completion {
                kind: if abort { "aborted" } else { "value" }.into(),
                expression,
            };
        };
        let origin = self.origins.origin(&result.expression, path);
        for binding in bindings.into_iter().rev() {
            result.expression = SExpr::new(
                SNode::Let {
                    name: binding.name,
                    ty: binding.ty.map(Into::into),
                    value: Box::new(binding.value),
                    body: Box::new(result.expression),
                },
                binding.span,
            );
        }
        self.origins.mark(&result.expression, origin);
        Ok(result)
    }
    fn ordinary_conditional_completion(
        &mut self,
        path: &[String],
        open: &Token,
    ) -> Result<Completion> {
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
        let then = self.ordinary_completion_block(path)?;
        let has_else = self.cursor.eat("else").is_some();
        if has_else && self.cursor.is("if") && !rules::accept_rust_ordinary_else_if(&then.kind) {
            return Err(unsupported(
                "guarded return completion",
                "an else-if after a continuing alternative needs its complete joined completion carrier",
                span(&token, self.cursor.peek()),
            ));
        }
        let mut otherwise = if has_else {
            Some(if self.cursor.is("if") {
                self.ordinary_conditional_completion(path, open)?
            } else {
                self.ordinary_completion_block(path)?
            })
        } else {
            None
        };
        let terminated = self.cursor.eat(";").is_some();
        let action = rules::read_rust_ordinary_conditional_completion(
            &then.kind,
            otherwise.as_ref().map_or("", |value| value.kind.as_str()),
            has_else,
            !self.cursor.is("}"),
            terminated,
        );
        match action.as_str() {
            "guarded continuation" => {
                otherwise = Some(self.ordinary_completion_sequence(path, open)?)
            }
            "implicit Unit fallback" => {
                otherwise = Some(Completion {
                    kind: "value".into(),
                    expression: SExpr::new(SNode::Unit, span(&token, &token)),
                })
            }
            "returned conditional" | "aborted conditional" | "terminal mixed conditional" => {}
            "conditional value" => {
                if terminated {
                    return Err(unsupported(
                        "expression statement",
                        "statements with effects are outside the portable core in function bodies",
                        span(&token, self.cursor.peek()),
                    ));
                }
            }
            "unreachable continuation" => {
                return Err(unsupported(
                    "unreachable continuation",
                    "statements after a completed return are not removed by this model",
                    span(&token, self.cursor.peek()),
                ));
            }
            "discarded conditional value" => {
                return Err(unsupported(
                    "expression statement",
                    "a block value must be its last expression",
                    span(&token, self.cursor.peek()),
                ));
            }
            _ => {
                return Err(unsupported(
                    "guarded return completion",
                    "every guard branch must complete a return or abort, with an enclosing continuation; mixed return/value branches need a separate completion proof",
                    span(&token, self.cursor.peek()),
                ));
            }
        }
        let otherwise = otherwise.ok_or_else(|| {
            unsupported(
                "guarded return completion",
                "the selected source action needs an actual alternative",
                span(&token, self.cursor.peek()),
            )
        })?;
        let origin = self.origins.join(&[
            self.origins.origin(&then.expression, path),
            self.origins.origin(&otherwise.expression, path),
        ]);
        let kind = rules::read_rust_ordinary_completion_kind(&action, &then.kind, &otherwise.kind);
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
}
