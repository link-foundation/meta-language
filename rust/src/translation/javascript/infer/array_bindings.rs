//! Original term endpoints selected by the whole generated binding module.
use super::term_facts::TermFacts;
use super::{Env, Inference, Result, SExpr, SNode, Span, Term, type_error};
use crate::translation::array_binding_rules::{self as rules, ml_string::Text};
use crate::translation::surface::array_bindings::validate;

pub(super) struct DefaultBinding {
    value: Term,
    fallback: Term,
    result: Term,
    pub(super) place: Option<Span>,
}

impl Inference {
    pub(super) fn array_binding_expr(
        &mut self,
        node: &SExpr,
        env: &Env,
        expected: Option<&Term>,
    ) -> Result<Term> {
        validate(node)?;
        match &node.node {
            SNode::ArrayBindingRest { object, .. } => {
                let object = self.expr(object, env)?;
                let result = Term::Array(Box::new(self.fresh(false)));
                self.unify(&object, &result, node.span)?;
                Ok(result)
            }
            SNode::ArrayBindingDefault {
                value, fallback, ..
            } => {
                let value = self.expr(value, env)?;
                let fallback = self.expr_expected(fallback, env, expected)?;
                let result = self.fresh(false);
                self.array_binding_defaults.push(DefaultBinding {
                    value,
                    fallback,
                    result: result.clone(),
                    place: node.span,
                });
                Ok(result)
            }
            _ => unreachable!("the validated binding has its actual constructor"),
        }
    }

    pub(super) fn solve_array_binding_defaults(&mut self) -> Result<bool> {
        let mut progress = false;
        let mut waiting = Vec::new();
        for constraint in std::mem::take(&mut self.array_binding_defaults) {
            let facts = TermFacts::new(&[
                Some(self.resolve(&constraint.value)),
                Some(constraint.fallback.clone()),
                Some(constraint.result.clone()),
            ]);
            let texts: Vec<Text> = facts
                .texts
                .iter()
                .map(|text| Text::from_text(text))
                .collect();
            let labels: Vec<Text> = facts
                .labels
                .iter()
                .map(|text| Text::from_text(text))
                .collect();
            let plan = rules::plan_java_script_array_default_inference(
                &texts,
                &facts.starts,
                &facts.counts,
                &labels,
                &facts.targets,
                facts.position(0),
                facts.position(1),
                facts.position(2),
            );
            if plan.is_empty() {
                waiting.push(constraint);
                continue;
            }
            if plan[0] < 0.0 {
                return Err(type_error(
                    "array binding default needs an undefined-only element read",
                    constraint.place,
                ));
            }
            for pair in plan.chunks_exact(2) {
                let left = facts
                    .term(pair[0])
                    .expect("an actual planned type endpoint");
                let right = facts
                    .term(pair[1])
                    .expect("an actual planned type endpoint");
                self.unify(&left, &right, constraint.place)?;
            }
            progress = true;
        }
        self.array_binding_defaults = waiting;
        Ok(progress)
    }
}
