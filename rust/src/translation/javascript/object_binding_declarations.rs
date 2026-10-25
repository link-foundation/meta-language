//! Original lexical declaration tokens and terms through shared pattern plans.
use super::{
    JavaScriptParser, Result, SExpr, SNode, Span, Stmt, Token, TranslationError, unsupported,
};
use crate::translation::frontend_rules::ml_array::number_index;
use crate::translation::javascript_record_parameter_rules::{self as rules, ml_string::Text};
use crate::translation::surface::SArrayItem;
pub(super) struct ObjectBindingDeclaration {
    pub(super) statements: Vec<Stmt>,
    pub(super) bindings: Vec<Token>,
    pub(super) span: Span,
}
impl JavaScriptParser {
    pub(super) fn object_binding_names(&self, start: usize) -> Vec<Token> {
        let close = self.matching(start);
        let tokens = &self.cursor.tokens[start..=close];
        let kinds = tokens
            .iter()
            .map(|token| Text::from_text(super::record_parameters::token_kind(token)))
            .collect::<Vec<_>>();
        let values = tokens
            .iter()
            .map(|token| Text::from_text(&token.value))
            .collect::<Vec<_>>();
        let plan = rules::plan_java_script_object_pattern_bindings(
            &kinds,
            &values,
            0.0,
            super::record_parameters::reserved_names(),
            &[],
            2.0,
        );
        if plan[0] < 0.0 {
            return Vec::new();
        }
        rules::read_java_script_object_pattern_binding_positions(&plan)
            .iter()
            .map(|position| {
                tokens[number_index(*position).expect("actual binding position")].clone()
            })
            .collect()
    }
    pub(super) fn object_binding_declaration(
        &mut self,
        start_token: &Token,
        mode: &str,
    ) -> Result<ObjectBindingDeclaration> {
        let start = self.cursor.index;
        let close = self.matching(start);
        let tokens = self.cursor.tokens[start..=close].to_vec();
        let kinds = tokens
            .iter()
            .map(|token| Text::from_text(super::record_parameters::token_kind(token)))
            .collect::<Vec<_>>();
        let values = tokens
            .iter()
            .map(|token| Text::from_text(&token.value))
            .collect::<Vec<_>>();
        let reserved = super::record_parameters::reserved_names();
        let plan = rules::plan_java_script_object_pattern_bindings(
            &kinds,
            &values,
            0.0,
            reserved,
            &[],
            2.0,
        );
        if plan[0] == -4.0 {
            self.cursor.index = start + tokens.len();
            return Err(unsupported(
                "empty object binding",
                "object destructibility needs a checked representation",
                Some(self.to_here(start_token)),
            ));
        }
        let selected = plan
            .get(1)
            .copied()
            .and_then(number_index)
            .and_then(|position| tokens.get(position))
            .unwrap_or_else(|| self.cursor.peek());
        if plan[0] == -6.0 {
            return Err(unsupported(
                "rest property",
                "name every field",
                Some(selected.span()),
            ));
        }
        if plan[0] == -8.0 {
            crate::translation::lexer::require_java_script_identifier_role(selected, true)?;
            if rules::accept_java_script_object_binding_name(
                &Text::from_text(super::record_parameters::token_kind(selected)),
                &Text::from_text(&selected.value),
                reserved,
            ) {
                return Err(TranslationError::syntax(
                    format!("duplicate lexical binding {}", selected.value),
                    Some(selected.span()),
                ));
            }
        }
        if plan[0] < 0.0 {
            return Err(unsupported(
                "object binding syntax",
                &format!(
                    "flat actual property names and distinct lexical bindings need their complete source meanings (reason {})",
                    -plan[0]
                ),
                Some(selected.span()),
            ));
        }
        let end = number_index(plan[0]).expect("actual admitted pattern end");
        let count = number_index(plan[1]).expect("actual binding count");
        let statement =
            rules::read_java_script_object_binding_statement(&Text::from_text(mode)).to_string();
        if end != tokens.len() || statement.is_empty() {
            return Err(unsupported(
                "object binding transport",
                "the declaration needs its complete actual object pattern and lexical mode",
                Some(self.cursor.peek().span()),
            ));
        }
        let positions = rules::read_java_script_object_pattern_binding_positions(&plan);
        let bindings = positions
            .iter()
            .map(|position| {
                tokens[number_index(*position).expect("actual binding position")].clone()
            })
            .collect::<Vec<_>>();
        for token in &bindings {
            crate::translation::lexer::require_java_script_identifier_role(token, true)?;
            self.lexical.declaration(&token.value, token.span(), 1);
            self.scope.tdz.insert(token.value.clone());
        }
        self.cursor.index = start + end;
        self.cursor.expect("=", Some("destructuring"))?;
        let value = self.expr()?;
        let whole = self.to_here(start_token);
        self.generated_count += 1;
        let name = format!(
            "{}temporary_binding{}",
            self.generated_prefix, self.generated_count
        );
        let object = SExpr::new(
            SNode::Name {
                path: vec![name.clone()],
            },
            Some(whole),
        );
        let mut statements = vec![Stmt::Const {
            name,
            value,
            span: whole,
        }];
        let bound_names = bindings
            .iter()
            .map(|token| token.value.clone())
            .collect::<Vec<_>>();
        for index in 0..count {
            let at = 2 + index * 4;
            let property = &tokens[number_index(plan[at]).expect("actual property position")];
            let local = &bindings[index];
            let place = Span::new(property.start, local.end);
            let fallback = if let Some(first) = number_index(plan[at + 2]) {
                Some(self.read_formal_initializer(
                    &tokens,
                    super::formal_initializers::FormalInitializer {
                        interval_positions: (
                            first,
                            number_index(plan[at + 3]).expect("actual initializer end"),
                            start,
                        ),
                        binding_positions: &positions,
                        binding: plan[at + 1],
                        bound_names: &bound_names,
                        context: "ordinary object binding declaration",
                        profile: "object binding initializer",
                    },
                )?)
            } else {
                None
            };
            let read = if let Some(fallback) = fallback {
                let place = Span::new(
                    property.start,
                    fallback.span.map_or(local.end, |span| span.end),
                );
                SExpr::new(
                    SNode::Math {
                        op: rules::read_java_script_record_formal_default_operation().to_string(),
                        name: String::new(),
                        args: vec![
                            SArrayItem::new(object.clone(), false),
                            SArrayItem::new(
                                SExpr::new(
                                    SNode::Str {
                                        value: property.value.clone(),
                                    },
                                    Some(property.span()),
                                ),
                                false,
                            ),
                            SArrayItem::new(fallback, false),
                        ],
                    },
                    Some(place),
                )
            } else {
                SExpr::new(
                    SNode::Field {
                        object: Box::new(object.clone()),
                        field: property.value.clone(),
                    },
                    Some(place),
                )
            };
            statements.push(if statement == "let" {
                Stmt::Let {
                    name: local.value.clone(),
                    value: read,
                    span: whole,
                }
            } else {
                Stmt::Const {
                    name: local.value.clone(),
                    value: read,
                    span: whole,
                }
            });
        }
        for token in &bindings {
            self.declare_local(&token.value, mode == "let");
            self.lexical
                .binding(&token.value, Some(self.cursor.peek().start));
        }
        Ok(ObjectBindingDeclaration {
            statements,
            bindings,
            span: whole,
        })
    }
}
