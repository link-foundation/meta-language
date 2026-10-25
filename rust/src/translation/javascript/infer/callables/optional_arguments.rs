//! Actual solver terms into shared optional argument and ownership decisions.
use super::{Inference, Result, Term, TermFacts, endpoint};
use crate::translation::callable_source_rules as rules;
use crate::translation::optional_callable_rules as optional_rules;
impl Inference {
    pub(in crate::translation::javascript::infer) fn complete_optional_callable_terms(
        &self,
        parameters: &[Term],
        arguments: &[Term],
    ) -> Vec<Term> {
        let excess = crate::translation::excess_argument_rules::plan_java_script_excess_arguments(
            "JavaScript",
            serde_json::Number::from(parameters.len())
                .as_f64()
                .expect("actual parameters"),
            serde_json::Number::from(arguments.len())
                .as_f64()
                .expect("actual arguments"),
        );
        let retained = if excess.is_empty() {
            arguments.to_vec()
        } else {
            excess[1..1 + parameters.len()]
                .iter()
                .map(|index| {
                    arguments[crate::translation::frontend_rules::ml_array::number_index(*index)
                        .expect("actual supplied argument")]
                    .clone()
                })
                .collect()
        };
        let arguments = retained.as_slice();
        let selected = parameters
            .iter()
            .map(|term| self.resolve(term))
            .collect::<Vec<_>>();
        let kinds = selected
            .iter()
            .map(|term| super::super::optional_values::kind(term).to_owned())
            .collect::<Vec<_>>();
        let absences = selected
            .iter()
            .map(|term| match term {
                Term::Option { absence, .. } => absence.borrow().clone(),
                _ => String::new(),
            })
            .collect::<Vec<_>>();
        let count = serde_json::Number::from(arguments.len())
            .as_f64()
            .expect("actual supplied callable arguments");
        let plan = optional_rules::plan_optional_callable_arguments(
            "JavaScript",
            &kinds,
            &absences,
            count,
        );
        if plan.is_empty() || arguments.len() == parameters.len() {
            return arguments.to_vec();
        }
        plan[1..]
            .iter()
            .enumerate()
            .map(|(index, slot)| {
                if *slot < 0.0 {
                    parameters[index].clone()
                } else {
                    arguments[crate::translation::frontend_rules::ml_array::number_index(*slot)
                        .expect("actual callable argument")]
                    .clone()
                }
            })
            .collect()
    }
    pub(in crate::translation::javascript::infer) fn body_entry_parameter_term(
        &self,
        parameter: &crate::translation::surface::SParam,
        term: Term,
    ) -> Term {
        let (kind, absence) =
            parameter
                .default_value
                .as_ref()
                .map_or(("", ""), |value| match &value.node {
                    crate::translation::surface::SNode::None { absence } => {
                        ("none", absence.as_str())
                    }
                    _ => (value.node.kind(), ""),
                });
        if !optional_rules::accept_body_entry_optional_parameter(parameter.optional, kind, absence)
        {
            return term;
        }
        match self.resolve(&term) {
            Term::Option { element, absence } => super::super::optional_values::optional(
                *element,
                &crate::translation::structured_type_rules::merge_optional_absence(
                    &absence.borrow(),
                    "undefined",
                ),
            ),
            _ => super::super::optional_values::optional(term, "undefined"),
        }
    }
    pub(in crate::translation::javascript::infer) fn solve_computed_callable_arguments(
        &mut self,
    ) -> Result<bool> {
        let mut waiting = Vec::new();
        let mut progress = false;
        for call in std::mem::take(&mut self.computed_calls) {
            let function = self.resolve(&call.function);
            if matches!(function, Term::Var(_)) {
                waiting.push(call);
                continue;
            }
            let arguments = match &function {
                Term::Function { params, .. } => {
                    self.complete_optional_callable_terms(params, &call.arguments)
                }
                _ => call.arguments.clone(),
            };
            let candidate = Self::callable_term(&arguments, &call.result);
            let mut facts = TermFacts::new(&[Some(function), Some(candidate)]);
            let arguments = facts.array(&arguments);
            let plan = rules::plan_callable_application_constraints(
                &facts.texts,
                &facts.starts,
                &facts.counts,
                &facts.labels,
                &facts.targets,
                facts.position(0),
                arguments,
                facts.position(1),
            );
            for pair in plan[1..].chunks_exact(2) {
                self.unify(
                    &endpoint(&facts, pair[0]),
                    &endpoint(&facts, pair[1]),
                    call.place,
                )?;
            }
            if plan[0] >= 0.0 {
                self.unify(&call.result, &endpoint(&facts, plan[0]), call.place)?;
            }
            progress = true;
        }
        self.computed_calls = waiting;
        Ok(progress)
    }
    pub(in crate::translation::javascript::infer) fn settle_unknown_computed_callable(
        &mut self,
    ) -> Result<bool> {
        let found = self
            .computed_calls
            .iter()
            .find(|call| matches!(self.resolve(&call.function), Term::Var(_)))
            .map(|call| {
                (
                    call.function.clone(),
                    call.arguments.clone(),
                    call.result.clone(),
                    call.place,
                )
            });
        let Some((function, arguments, result, place)) = found else {
            return Ok(false);
        };
        self.unify(&function, &Self::callable_term(&arguments, &result), place)?;
        Ok(true)
    }
}
