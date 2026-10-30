//! Actual source operands, scopes and nodes into the shared continuation plan.
use super::{
    Alias, Options, Result, RustParser, SArrayItem, SExpr, SNode, SPattern, SPatternNode, SRow,
    SourceOrigin, Span, Token, span, unsupported,
};
use crate::translation::conditional_chain_rules as rules;
use crate::translation::frontend_rules::ml_array::number_index;
struct Stage {
    patterns: Vec<SPattern>,
    scrutinee: Option<SExpr>,
    condition: Option<SExpr>,
    aliases: Vec<Alias>,
    span: Option<Span>,
}
pub(in crate::translation::rust) struct ConditionalStages {
    stages: Vec<Stage>,
    forms: Vec<f64>,
}
impl ConditionalStages {
    pub(in crate::translation::rust) fn single_pattern(
        pattern: SPattern,
        scrutinee: SExpr,
        span: Option<Span>,
    ) -> Self {
        Self {
            forms: vec![1.0],
            stages: vec![Stage {
                patterns: vec![pattern],
                scrutinee: Some(scrutinee),
                condition: None,
                aliases: Vec::new(),
                span,
            }],
        }
    }
}
pub(super) struct GuardArm {
    pub(super) patterns: Vec<SPattern>,
    pub(super) body: SExpr,
    pub(super) aliases: Vec<Alias>,
    pub(super) guard: Option<SExpr>,
    pub(super) origin: SourceOrigin,
    pub(super) span: Option<Span>,
}
fn aliases_around(aliases: &[Alias], mut body: SExpr, span: Option<Span>) -> SExpr {
    for alias in aliases.iter().rev() {
        body = SExpr::new(
            SNode::Let {
                name: alias.name.clone(),
                ty: None,
                value: Box::new(alias.value.clone()),
                body: Box::new(body),
            },
            span,
        );
    }
    body
}
fn operation(role: &str, values: Vec<SExpr>, span: Option<Span>) -> SExpr {
    SExpr::new(
        SNode::Math {
            op: rules::read_conditional_chain_operation(role),
            name: String::new(),
            args: values
                .into_iter()
                .map(|value| SArrayItem::new(value, false))
                .collect(),
        },
        span,
    )
}
impl RustParser {
    pub(in crate::translation::rust) fn has_conditional_chain(&self) -> bool {
        let kinds = self
            .cursor
            .tokens
            .iter()
            .map(|token| {
                crate::translation::source_diagnostic_attributes::token_kind(token).to_owned()
            })
            .collect::<Vec<_>>();
        let values = self
            .cursor
            .tokens
            .iter()
            .map(|token| token.value.clone())
            .collect::<Vec<_>>();
        let start = serde_json::Number::from(self.cursor.index)
            .as_f64()
            .expect("an actual token position has a finite Number representation");
        rules::find_rust_conditional_chain_pattern(&kinds, &values, start) >= 0.0
    }
    pub(in crate::translation::rust) fn conditional_chain_stages(
        &mut self,
        path: &[String],
        start: &Token,
    ) -> Result<ConditionalStages> {
        let mut stages = Vec::new();
        let mut forms = Vec::new();
        loop {
            let token = self.cursor.peek().clone();
            if self.cursor.eat("let").is_some() {
                let mut aliases = Vec::new();
                let mut patterns = vec![self.pattern(path, &mut aliases)?];
                while self.cursor.eat("|").is_some() {
                    patterns.push(self.pattern(path, &mut aliases)?);
                }
                if patterns.len() > 1 && !aliases.is_empty() {
                    return Err(unsupported(
                        "binding in an or-pattern",
                        "bind the value in each alternative separately",
                        span(start, self.cursor.peek()),
                    ));
                }
                self.cursor.expect("=", Some("conditional pattern"))?;
                let scrutinee = self.comparison(
                    path,
                    Options {
                        no_struct: true,
                        statement: false,
                    },
                )?;
                let scope = self.origins.cases(&patterns, &scrutinee, path)?;
                self.origins.enter(Some(scope));
                for alias in &aliases {
                    self.origins.bind(&alias.name, &alias.value, path, None)?;
                }
                stages.push(Stage {
                    patterns,
                    scrutinee: Some(scrutinee),
                    condition: None,
                    aliases,
                    span: span(&token, self.cursor.peek()),
                });
                forms.push(1.0);
            } else {
                let condition = self.comparison(
                    path,
                    Options {
                        no_struct: true,
                        statement: false,
                    },
                )?;
                self.origins.direct(&condition, path, condition.span)?;
                stages.push(Stage {
                    patterns: Vec::new(),
                    scrutinee: None,
                    condition: Some(condition),
                    aliases: Vec::new(),
                    span: span(&token, self.cursor.peek()),
                });
                forms.push(0.0);
            }
            if self.cursor.is("||") {
                return Err(unsupported(
                    "Rust conditional pattern disjunction",
                    "a let condition requires parentheses around each Boolean disjunction",
                    span(start, self.cursor.peek()),
                ));
            }
            if serde_json::Number::from(stages.len())
                .as_f64()
                .expect("actual source stage count")
                > rules::read_conditional_chain_stage_limit()
            {
                return Err(unsupported(
                    "Rust conditional pattern chain depth",
                    "the actual condition exceeds its shared stage budget",
                    span(start, self.cursor.peek()),
                ));
            }
            if self.cursor.eat("&&").is_none() {
                break;
            }
        }
        Ok(ConditionalStages { stages, forms })
    }
    pub(in crate::translation::rust) fn lower_conditional_chain_stages(
        &self,
        syntax: ConditionalStages,
        body: SExpr,
        otherwise: SExpr,
        range: Option<Span>,
    ) -> Result<SExpr> {
        let ConditionalStages { stages, forms } = syntax;
        let failed = operation("failure", Vec::new(), range);
        let mut result = operation("success", vec![body], range);
        let plan = rules::plan_conditional_chain_layers(&forms);
        if plan.len() != stages.len() * 2 {
            return Err(unsupported(
                "Rust conditional chain transport",
                "every actual stage needs its complete shared layer",
                range,
            ));
        }
        for pair in plan.chunks_exact(2) {
            let stage = &stages[number_index(pair[1]).expect("actual source stage")];
            if pair[0] == 0.0 {
                result = SExpr::new(
                    SNode::If {
                        cond: Box::new(stage.condition.clone().expect("actual Boolean condition")),
                        then: Box::new(result),
                        otherwise: Box::new(failed.clone()),
                    },
                    stage.span,
                );
            } else {
                for alias in stage.aliases.iter().rev() {
                    result = SExpr::new(
                        SNode::Let {
                            name: alias.name.clone(),
                            ty: None,
                            value: Box::new(alias.value.clone()),
                            body: Box::new(result),
                        },
                        stage.span,
                    );
                }
                let mut rows = stage
                    .patterns
                    .iter()
                    .cloned()
                    .map(|pattern| SRow {
                        patterns: vec![pattern],
                        body: result.clone(),
                        span: None,
                    })
                    .collect::<Vec<_>>();
                rows.push(SRow {
                    patterns: vec![SPattern {
                        node: SPatternNode::Wild,
                        span: None,
                    }],
                    body: failed.clone(),
                    span: None,
                });
                result = SExpr::new(
                    SNode::Match {
                        scrutinees: vec![
                            stage.scrutinee.clone().expect("actual pattern scrutinee"),
                        ],
                        rows,
                    },
                    stage.span,
                );
            }
        }
        let mut result = operation("result", vec![result, otherwise], range);
        result.block = true;
        Ok(result)
    }
    pub(super) fn conditional_chain(&mut self, path: &[String], start: &Token) -> Result<SExpr> {
        let previous = self.origins.enter(None);
        let parsed = (|| {
            let syntax = self.conditional_chain_stages(path, start)?;
            let body = self.block(path)?;
            let origin = self.origins.origin(&body, path);
            Ok::<_, crate::translation::diagnostics::TranslationError>((syntax, body, origin))
        })();
        self.origins.scope = previous;
        let (syntax, body, origin) = parsed?;
        if self.cursor.eat("else").is_none() {
            return Err(unsupported(
                "if without else",
                "the value of an if must be defined on both branches",
                span(start, self.cursor.peek()),
            ));
        }
        let otherwise = if self.cursor.is("if") {
            self.if_expr(path)?
        } else {
            self.block(path)?
        };
        let range = span(start, self.cursor.peek());
        let origin = self
            .origins
            .checked_join(&[origin, self.origins.origin(&otherwise, path)])?;
        let result = self.lower_conditional_chain_stages(syntax, body, otherwise, range)?;
        self.origins.mark(&result, origin);
        Ok(result)
    }
    pub(super) fn lower_guarded_match(
        &mut self,
        scrutinee: SExpr,
        arms: Vec<GuardArm>,
        path: &[String],
        span: Option<Span>,
    ) -> Result<SExpr> {
        let arms = arms
            .into_iter()
            .flat_map(|arm| {
                arm.patterns
                    .into_iter()
                    .map(|pattern| GuardArm {
                        patterns: vec![pattern],
                        body: arm.body.clone(),
                        aliases: arm.aliases.clone(),
                        guard: arm.guard.clone(),
                        origin: arm.origin.clone(),
                        span: arm.span,
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let guards = arms
            .iter()
            .map(|arm| if arm.guard.is_some() { 1.0 } else { 0.0 })
            .collect::<Vec<_>>();
        let plan = rules::plan_conditional_match_arms(&guards);
        if plan.is_empty() {
            return Err(unsupported(
                "Rust guarded match depth",
                "the actual arms exceed their shared stage budget",
                span,
            ));
        }
        let count = number_index(plan[0]).expect("actual coverage count");
        let name = self.fresh_product_binding();
        let bound = SExpr::new(
            SNode::Name {
                path: vec![name.clone()],
            },
            span,
        );
        self.origins
            .mark(&bound, self.origins.origin(&scrutinee, path));
        let mut rows = Vec::new();
        for at in &plan[1..1 + count] {
            let arm = &arms[number_index(*at).expect("actual coverage arm")];
            for pattern in &arm.patterns {
                rows.push(SRow {
                    patterns: vec![pattern.clone()],
                    body: aliases_around(&arm.aliases, arm.body.clone(), arm.span),
                    span: arm.span,
                });
            }
        }
        let coverage = SExpr::new(
            SNode::Match {
                scrutinees: vec![bound.clone()],
                rows,
            },
            span,
        );
        let failed = operation("failure", Vec::new(), span);
        let mut result = failed.clone();
        for at in &plan[1 + count..] {
            let arm = &arms[number_index(*at).expect("actual source arm")];
            let mut body = operation(
                "success",
                vec![operation("success", vec![arm.body.clone()], arm.span)],
                arm.span,
            );
            if let Some(guard) = &arm.guard {
                body = SExpr::new(
                    SNode::If {
                        cond: Box::new(guard.clone()),
                        then: Box::new(body),
                        otherwise: Box::new(failed.clone()),
                    },
                    arm.span,
                );
            }
            body = aliases_around(&arm.aliases, body, arm.span);
            let mut rows = arm
                .patterns
                .iter()
                .cloned()
                .map(|pattern| SRow {
                    patterns: vec![pattern],
                    body: body.clone(),
                    span: arm.span,
                })
                .collect::<Vec<_>>();
            rows.push(SRow {
                patterns: vec![SPattern {
                    node: SPatternNode::Wild,
                    span: None,
                }],
                body: failed.clone(),
                span: None,
            });
            let matched = SExpr::new(
                SNode::Match {
                    scrutinees: vec![bound.clone()],
                    rows,
                },
                arm.span,
            );
            result = operation("result", vec![matched, result], span);
        }
        let mut result = SExpr::new(
            SNode::Let {
                name,
                ty: None,
                value: Box::new(scrutinee),
                body: Box::new(operation("result", vec![result, coverage], span)),
            },
            span,
        );
        result.block = true;
        let origin = self.origins.checked_join(
            &arms
                .iter()
                .map(|arm| arm.origin.clone())
                .collect::<Vec<_>>(),
        )?;
        self.origins.mark(&result, origin);
        Ok(result)
    }
}
