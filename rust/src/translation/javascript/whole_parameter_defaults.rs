//! Actual complete formal intervals and defaults into existing logical parameters.
use super::record_parameters::RecordParameters;
use super::recursive_object_bindings::{
    RecursiveObjectEntry, RecursiveObjectSyntax, index, reserved_names,
};
use super::{JavaScriptParser, Result, SExpr, SNode, Span, Token, unsupported};
use crate::translation::whole_parameter_default_rules::{self as rules, ml_string::Text};
fn default_source(value: SExpr, fallback: SExpr, span: Span) -> SExpr {
    SExpr::new(
        SNode::ArrayBindingDefault {
            value: Box::new(value),
            fallback: Box::new(fallback),
            extra: std::collections::BTreeMap::new(),
        },
        Some(span),
    )
}
fn undefined(span: Span) -> SExpr {
    SExpr::new(
        SNode::None {
            absence: "undefined".to_owned(),
        },
        Some(span),
    )
}
impl JavaScriptParser {
    fn whole_parameter_default(
        &mut self,
        tokens: &[Token],
        start: usize,
        first: usize,
        last: usize,
        binding: f64,
        positions: &[f64],
        names: &[String],
        context: &str,
    ) -> Result<SExpr> {
        self.read_formal_initializer(
            tokens,
            super::formal_initializers::FormalInitializer {
                interval_positions: (first, last, start),
                binding_positions: positions,
                binding,
                bound_names: names,
                context,
                profile: "whole parameter default",
            },
        )
    }
    pub(super) fn read_whole_parameter_defaults(
        &mut self,
        tokens: &[Token],
        start: usize,
        context: &str,
    ) -> Result<Option<RecordParameters>> {
        let kinds = tokens
            .iter()
            .map(|token| Text::from_text(super::record_parameters::token_kind(token)))
            .collect::<Vec<_>>();
        let values = tokens
            .iter()
            .map(|token| Text::from_text(&token.value))
            .collect::<Vec<_>>();
        let reserved = reserved_names()
            .iter()
            .map(|name| Text::from_text(&name.to_string()))
            .collect::<Vec<_>>();
        let plan = rules::plan_java_script_whole_parameter_defaults(
            &kinds,
            &values,
            0.0,
            &reserved,
            rules::read_java_script_object_binding_depth_limit(),
        );
        if !rules::contains_java_script_whole_parameter_default(&plan) && !crate::translation::scalar_closure_default_rules::contains_java_script_scalar_closure_default(&plan,context) {
            return Ok(None);
        }
        if !rules::accept_java_script_whole_parameter_default_context(&Text::from_text(context)) {
            return Err(unsupported(
                "whole parameter default context",
                "the actual defaulted callable requires its complete function-value invocation environment",
                Some(tokens[0].span()),
            ));
        }
        if index(plan[0]) != tokens.len() {
            return Err(unsupported(
                "whole parameter default transport",
                "the complete actual formal interval needs its parser-owned token and name supply",
                Some(tokens[0].span()),
            ));
        }
        let positions = rules::read_java_script_whole_parameter_binding_positions(&plan);
        let names = positions
            .iter()
            .map(|at| tokens[index(*at)].value.clone())
            .collect::<Vec<_>>();
        let mut parameters = Vec::new();
        let mut entries = Vec::new();
        let mut at = 2;
        while at < plan.len() {
            let form = index(plan[at]);
            let slot = index(plan[at + 1]);
            at += 2;
            if slot != parameters.len() {
                return Err(unsupported(
                    "whole parameter default transport",
                    "logical arguments retain their actual ordered slots",
                    Some(tokens[0].span()),
                ));
            }
            if form == 0 || form == 2 {
                let binding = plan[at];
                let token = &tokens[index(binding)];
                at += 1;
                crate::translation::lexer::require_java_script_identifier_role(token, true)?;
                let mut parameter =
                    super::record_parameters::parameter(token.value.clone(), token.span());
                if form == 2 {
                    let name = self
                        .recursive_object_parameter_name("whole_parameter_value", token.span())?;
                    let fallback = self.whole_parameter_default(
                        tokens,
                        start,
                        index(plan[at]),
                        index(plan[at + 1]),
                        binding,
                        &positions,
                        &names,
                        context,
                    )?;
                    at += 2;
                    parameter.name = name.clone();
                    parameter.optional = true;
                    parameter.default_value = Some(undefined(token.span()));
                    entries.push(RecursiveObjectEntry {
                        temporary: false,
                        name: token.value.clone(),
                        value: default_source(
                            SExpr::new(SNode::Name { path: vec![name] }, Some(token.span())),
                            fallback,
                            token.span(),
                        ),
                        span: token.span(),
                    });
                }
                parameters.push(parameter);
                continue;
            }
            let length = index(plan[at]);
            at += 1;
            let tree = plan[at..at + length].to_vec();
            at += length;
            let open = &tokens[index(tree[8])];
            let close = &tokens[index(tree[9])];
            let span = Span::new(open.start, close.end);
            let name = self.recursive_object_parameter_name("whole_parameter_value", span)?;
            let mut parameter = super::record_parameters::parameter(name.clone(), span);
            let mut root = SExpr::new(SNode::Name { path: vec![name] }, Some(span));
            if form == 3 {
                let first = index(plan[at]);
                let last = index(plan[at + 1]);
                at += 2;
                let binding=positions.iter().copied().find(|position|*position>tree[8]&&*position<tree[9]).ok_or_else(||unsupported("whole parameter default binding","an empty pattern needs its separate complete coercion and initialization environment",Some(open.span())))?;
                let fallback = self.whole_parameter_default(
                    tokens, start, first, last, binding, &positions, &names, context,
                )?;
                parameter.optional = true;
                parameter.default_value = Some(undefined(span));
                let selected =
                    self.recursive_object_parameter_name("record_parameter_whole_value", span)?;
                entries.push(RecursiveObjectEntry {
                    temporary: true,
                    name: selected.clone(),
                    value: default_source(root, fallback, span),
                    span,
                });
                root = SExpr::new(
                    SNode::Name {
                        path: vec![selected],
                    },
                    Some(span),
                );
            }
            parameters.push(parameter);
            entries.extend(self.recursive_object_entries(
                &RecursiveObjectSyntax {
                    tokens: tokens.to_vec(),
                    plan: tree,
                    start,
                },
                root,
                &positions,
                context,
                "record formal default",
                true,
            )?);
        }
        self.cursor.index = start + index(plan[0]);
        Ok(Some(RecordParameters::from_recursive_object_formals(
            parameters, names, entries,
        )))
    }
}
