//! Original tokens and terms supplied to the complete shared owner plan.
use super::{
    JavaScriptParser, Result, SExpr, SNode, Span, Token, TokenKind, TranslationError, unsupported,
};
use crate::translation::frontend_rules::ml_array::number_index;
use crate::translation::recursive_object_binding_rules::{self as rules, ml_string::Text};
use crate::translation::surface::SArrayItem;
use std::collections::BTreeMap;
use std::sync::OnceLock;
pub(super) struct RecursiveObjectSyntax {
    pub(super) tokens: Vec<Token>,
    pub(super) plan: Vec<f64>,
    pub(super) start: usize,
}
#[derive(Clone)]
pub(super) struct RecursiveObjectEntry {
    pub(super) temporary: bool,
    pub(super) name: String,
    pub(super) value: SExpr,
    pub(super) span: Span,
}
pub(super) fn index(value: f64) -> usize {
    number_index(value).expect("shared recursive plan selects an actual nonnegative position")
}
pub(super) fn reserved_names() -> &'static [Text] {
    static NAMES: OnceLock<Vec<Text>> = OnceLock::new();
    NAMES.get_or_init(|| {
        super::record_parameters::reserved_names()
            .iter()
            .map(|name| Text::from_text(&name.to_string()))
            .collect()
    })
}
pub(super) fn columns(tokens: &[Token]) -> (Vec<Text>, Vec<Text>) {
    (
        tokens
            .iter()
            .map(|token| Text::from_text(super::record_parameters::token_kind(token)))
            .collect(),
        tokens
            .iter()
            .map(|token| Text::from_text(&token.value))
            .collect(),
    )
}
pub(super) fn binding_positions(plan: &[f64]) -> Vec<f64> {
    rules::read_java_script_recursive_object_binding_positions(plan)
}
impl JavaScriptParser {
    pub(super) fn recursive_object_plan(&self, start: usize) -> RecursiveObjectSyntax {
        let tokens = self.cursor.tokens[start..=self.matching(start)].to_vec();
        let (kinds, values) = columns(&tokens);
        let plan = rules::plan_java_script_recursive_object_bindings(
            &kinds,
            &values,
            0.0,
            reserved_names(),
            &[],
            rules::read_java_script_object_binding_depth_limit(),
        );
        RecursiveObjectSyntax {
            tokens,
            plan,
            start,
        }
    }
    pub(super) fn require_recursive_object_plan(
        &mut self,
        syntax: &RecursiveObjectSyntax,
        start: &Token,
    ) -> Result<()> {
        let plan = &syntax.plan;
        let reason = number_index(-plan[0]);
        if reason == Some(4) {
            self.cursor.index = syntax.start + syntax.tokens.len();
            return Err(unsupported(
                "empty object binding",
                "object destructibility needs a checked representation",
                Some(self.to_here(start)),
            ));
        }
        let token = plan
            .get(1)
            .copied()
            .and_then(number_index)
            .and_then(|at| syntax.tokens.get(at))
            .unwrap_or_else(|| self.cursor.peek());
        if reason == Some(6) {
            return Err(unsupported(
                "rest property",
                "name every field",
                Some(token.span()),
            ));
        }
        if reason == Some(9) {
            return Err(unsupported(
                "object binding depth",
                "the actual recursive pattern exceeds its shared source depth budget",
                Some(token.span()),
            ));
        }
        if reason == Some(8) {
            crate::translation::lexer::require_java_script_identifier_role(token, true)?;
            use crate::translation::javascript_record_parameter_rules as old;
            if old::accept_java_script_object_binding_name(
                &old::ml_string::Text::from_text(super::record_parameters::token_kind(token)),
                &old::ml_string::Text::from_text(&token.value),
                super::record_parameters::reserved_names(),
            ) {
                return Err(TranslationError::syntax(
                    format!("duplicate lexical binding {}", token.value),
                    Some(token.span()),
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
                Some(token.span()),
            ));
        }
        if index(plan[0]) != syntax.tokens.len() {
            return Err(unsupported(
                "object binding transport",
                "the declaration needs its complete actual object pattern and lexical mode",
                Some(self.cursor.peek().span()),
            ));
        }
        Ok(())
    }
    pub(super) fn recursive_object_parameter_name(
        &mut self,
        prefix: &str,
        span: Span,
    ) -> Result<String> {
        if self.record_parameter_names.is_none() {
            self.record_parameter_names = Some(
                self.cursor
                    .tokens
                    .iter()
                    .filter(|token| token.kind == TokenKind::Identifier)
                    .map(|token| token.value.clone())
                    .chain(self.externals.iter().map(|item| item.name().to_owned()))
                    .chain(self.literal_bindings.keys().cloned())
                    .chain(self.global_binding_names.iter().cloned())
                    .collect(),
            );
        }
        loop {
            if self.record_parameter_ordinal >= 9_007_199_254_740_991 {
                return Err(unsupported(
                    "record parameter source transport",
                    "the parser-owned logical argument name ordinal exceeds its safe integer range",
                    Some(span),
                ));
            }
            let name = format!("{prefix}{}", self.record_parameter_ordinal);
            self.record_parameter_ordinal += 1;
            if self
                .record_parameter_names
                .as_mut()
                .expect("actual names initialized")
                .insert(name.clone())
            {
                return Ok(name);
            }
        }
    }
    pub(super) fn recursive_object_entries(
        &mut self,
        syntax: &RecursiveObjectSyntax,
        root: SExpr,
        positions: &[f64],
        context: &str,
        profile: &str,
        formal: bool,
    ) -> Result<Vec<RecursiveObjectEntry>> {
        let tokens = &syntax.tokens;
        let plan = &syntax.plan;
        let names = positions
            .iter()
            .map(|at| tokens[index(*at)].value.clone())
            .collect::<Vec<_>>();
        let mut owners = BTreeMap::from([(0usize, root)]);
        let mut entries = Vec::new();
        for ordinal in 1..index(plan[1]) {
            let at = 2 + ordinal * 8;
            let owner = owners
                .get(&index(plan[at + 1]))
                .expect("actual preceding owner")
                .clone();
            let property = &tokens[index(plan[at + 2])];
            let binding = number_index(plan[at + 3]).map(|at| &tokens[at]);
            let mut span = Span::new(
                property.start,
                binding.map_or_else(|| tokens[index(plan[at + 7])].end, |token| token.end),
            );
            let value = if index(plan[at]) == 2 {
                let excluded =
                    rules::read_java_script_recursive_object_excluded_positions(plan, plan[at + 1])
                        .iter()
                        .map(|at| {
                            let token = &tokens[index(*at)];
                            (token.value.clone(), token.span())
                        })
                        .collect::<Vec<_>>();
                crate::translation::object_rest::source_read(owner, &excluded, span)
            } else {
                let fallback = if let Some(first) = number_index(plan[at + 4]) {
                    Some(
                        self.read_formal_initializer(
                            tokens,
                            super::formal_initializers::FormalInitializer {
                                interval_positions: (first, index(plan[at + 5]), syntax.start),
                                binding_positions: positions,
                                binding:
                                    rules::read_java_script_recursive_object_initializer_binding(
                                        plan,
                                        serde_json::Number::from(ordinal)
                                            .as_f64()
                                            .expect("bounded ordinal"),
                                    ),
                                bound_names: &names,
                                context,
                                profile,
                            },
                        )?,
                    )
                } else {
                    None
                };
                if let Some(fallback) = fallback {
                    if let Some(origin) = fallback.span {
                        span.end = origin.end;
                    }
                    SExpr::new(SNode::Math{op:crate::translation::javascript_record_parameter_rules::read_java_script_record_formal_default_operation().to_string(),name:String::new(),args:vec![SArrayItem::new(owner,false),SArrayItem::new(SExpr::new(SNode::Str{value:property.value.clone()},Some(property.span())),false),SArrayItem::new(fallback,false)]},Some(span))
                } else {
                    SExpr::new(
                        SNode::Field {
                            object: Box::new(owner),
                            field: property.value.clone(),
                        },
                        Some(span),
                    )
                }
            };
            if let Some(token) = binding {
                crate::translation::lexer::require_java_script_identifier_role(token, true)?;
                entries.push(RecursiveObjectEntry {
                    temporary: false,
                    name: token.value.clone(),
                    value,
                    span: token.span(),
                });
            } else {
                let span = Span::new(
                    tokens[index(plan[at + 6])].start,
                    tokens[index(plan[at + 7])].end,
                );
                let name = if formal {
                    self.recursive_object_parameter_name("record_parameter_nested_value", span)?
                } else {
                    self.generated_count += 1;
                    format!(
                        "{}object_binding_value{}",
                        self.generated_prefix, self.generated_count
                    )
                };
                owners.insert(
                    ordinal,
                    SExpr::new(
                        SNode::Name {
                            path: vec![name.clone()],
                        },
                        Some(span),
                    ),
                );
                entries.push(RecursiveObjectEntry {
                    temporary: true,
                    name,
                    value,
                    span,
                });
            }
        }
        Ok(entries)
    }
}
