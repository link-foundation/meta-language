//! Typed host construction and existing solver calls selected by generated plans.

use super::term_facts::TermFacts;
use super::{Env, HashSet, Inference, Result, SExpr, SNode, Span, Term, Type, type_error};
use crate::translation::callable_source_rules as rules;
mod named_defaults;
mod optional_arguments;

pub(super) fn has_structural_inference(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::Array(values) => values.iter().any(has_structural_inference),
        serde_json::Value::Object(fields) => {
            if fields
                .get("k")
                .and_then(serde_json::Value::as_str)
                .is_some_and(|kind| kind == "arrayBindingRest" || kind == "arrayBindingDefault")
            {
                return true;
            }
            let text = |field: &str| {
                fields
                    .get(field)
                    .and_then(serde_json::Value::as_str)
                    .unwrap_or_default()
                    .to_owned()
            };
            (fields.get("k").and_then(serde_json::Value::as_str)==Some("math") && (text("op")==crate::translation::javascript_optional_call_rules::read_java_script_optional_call_operation()||text("op")==crate::translation::javascript_optional_chain_rules::read_java_script_optional_chain_operation())) || rules::require_callable_structural_inference(
                &(text("k")),
                &(text("kind")),
                &(text("op")),
            ) || fields
                .iter()
                .any(|(name, value)| name != "span" && has_structural_inference(value))
        }
        _ => false,
    }
}

fn endpoint(facts: &TermFacts, identity: f64) -> Term {
    facts
        .term(identity)
        .expect("a callable constraint retains its actual term")
}

impl Inference {
    fn callable_term(parameters: &[Term], ret: &Term) -> Term {
        let mut facts = TermFacts::new(&[Some(ret.clone())]);
        let parameters = facts.parameter_array(parameters);
        let plan = rules::plan_callable_function_type(
            &facts.starts,
            &facts.counts,
            &facts.labels,
            &facts.targets,
            parameters,
            facts.position(0),
        );
        Term::Function {
            params: plan[1..]
                .iter()
                .map(|identity| endpoint(&facts, *identity))
                .collect(),
            ret: Box::new(endpoint(&facts, plan[0])),
        }
    }

    pub(super) fn unify_callable(
        &mut self,
        left: &Term,
        right: &Term,
        place: Option<Span>,
    ) -> Result<()> {
        let facts = TermFacts::new(&[Some(left.clone()), Some(right.clone())]);
        let plan = rules::plan_callable_type_constraints(
            &facts.starts,
            &facts.counts,
            &facts.labels,
            &facts.targets,
            facts.position(0),
            facts.position(1),
        );
        if matches!(plan.as_slice(), [value] if value.to_bits() == (-1.0_f64).to_bits()) {
            return Err(type_error(
                "one function is used with two different parameter counts",
                place,
            ));
        }
        for pair in plan.as_chunks::<2>().0 {
            self.unify(
                &endpoint(&facts, pair[0]),
                &endpoint(&facts, pair[1]),
                place,
            )?;
        }
        Ok(())
    }

    fn cell_element_plan(cell: &Term) -> Term {
        let facts = TermFacts::new(&[Some(cell.clone())]);
        let plan = rules::plan_callable_cell_read_inference(
            &facts.texts,
            &facts.starts,
            &facts.counts,
            &facts.labels,
            &facts.targets,
            facts.position(0),
            facts.position(0),
        );
        endpoint(&facts, plan[0])
    }

    pub(super) fn unify_cells(
        &mut self,
        left: &Term,
        right: &Term,
        place: Option<Span>,
    ) -> Result<()> {
        self.unify(
            &Self::cell_element_plan(left),
            &Self::cell_element_plan(right),
            place,
        )
    }

    pub(super) fn callable_contains(
        &self,
        term: &Term,
        variable: usize,
        seen: &mut HashSet<usize>,
    ) -> bool {
        let facts = TermFacts::new(&[Some(term.clone())]);
        let plan = rules::read_callable_function_children(
            &facts.starts,
            &facts.counts,
            &facts.labels,
            &facts.targets,
            facts.position(0),
        );
        plan.into_iter()
            .filter_map(|identity| facts.term(identity))
            .any(|child| self.contains(&child, variable, seen))
    }

    pub(super) fn callable_application(
        &mut self,
        head: &SExpr,
        arguments: &[SExpr],
        env: &Env,
        place: Option<Span>,
    ) -> Result<Term> {
        let path = match &head.node {
            SNode::Name { path } => path.as_slice(),
            _ => &[],
        };
        let local = match path {
            [name] => env.get(name).cloned(),
            _ => None,
        };
        if rules::require_callable_named_inference(
            head.node.kind(),
            super::super::callables::number(path.len()),
            local.is_some(),
        ) {
            return self.call(path, arguments, env, place);
        }
        let function = match local {
            Some(local) => local,
            None => self.expr(head, env)?,
        };
        let mut arguments = arguments
            .iter()
            .map(|argument| self.expr(argument, env))
            .collect::<Result<Vec<_>>>()?;
        let resolved = self.resolve(&function);
        if matches!(resolved, Term::Var(_)) {
            let result = self.fresh(false);
            self.computed_calls.push(super::ComputedCall {
                function,
                arguments,
                result: result.clone(),
                place,
            });
            return Ok(result);
        }
        if let Term::Function { params, .. } = &resolved {
            arguments = self.complete_optional_callable_terms(params, &arguments);
        }
        let (kind, count) = match &resolved {
            Term::Function { params, .. } => ("function", params.len()),
            Term::Known(ty) => (ty.kind(), 0),
            _ => ("", 0),
        };
        let direct = rules::accept_callable_application_inference_result(
            kind,
            super::super::callables::number(count),
            super::super::callables::number(arguments.len()),
        );
        let result = if direct {
            None
        } else {
            Some(self.fresh(false))
        };
        let candidate = result
            .as_ref()
            .map(|ret| Self::callable_term(&arguments, ret));
        let mut facts = TermFacts::new(&[Some(resolved), candidate]);
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
        for pair in plan[1..].as_chunks::<2>().0 {
            self.unify(
                &endpoint(&facts, pair[0]),
                &endpoint(&facts, pair[1]),
                place,
            )?;
        }
        Ok(result.unwrap_or_else(|| endpoint(&facts, plan[0])))
    }

    pub(super) fn callable_expr(
        &mut self,
        node: &SExpr,
        env: &Env,
        expected: Option<&Term>,
    ) -> Result<Term> {
        match &node.node {
            SNode::FunctionValue { path } => match self.signature(path) {
                Some(signature) => self.named_default_reference_term(&signature, node.span),
                None => Ok(self.fresh(false)),
            },
            SNode::Lambda { params, body, .. } => {
                let expected = expected.map(|term| self.resolve(term));
                let mut facts = TermFacts::new(&[expected]);
                let lambda = facts
                    .json(&serde_json::to_value(node).expect("a surface lambda is serializable"));
                let plan = rules::plan_callable_lambda_inference(
                    &facts.forms,
                    &facts.texts,
                    &facts.starts,
                    &facts.counts,
                    &facts.labels,
                    &facts.targets,
                    lambda,
                    facts.position(0),
                );
                let mut scope = env.clone();
                let mut parameters = Vec::new();
                for (parameter, selection) in params.iter().zip(plan[2..].as_chunks::<3>().0) {
                    let term = self.callable_selected_type(
                        &facts,
                        selection[1],
                        selection[2].to_bits() == 1.0_f64.to_bits(),
                    )?;
                    let term = self.body_entry_parameter_term(parameter, term);
                    scope.insert(parameter.name.clone(), term.clone());
                    parameters.push(term);
                }
                let result = self.callable_selected_type(
                    &facts,
                    plan[0],
                    plan[1].to_bits() == 1.0_f64.to_bits(),
                )?;
                let body = self.expr_expected(body, &scope, Some(&result))?;
                self.unify(&body, &result, node.span)?;
                let term = Self::callable_term(
                    &parameters,
                    if plan[1].to_bits() == 1.0_f64.to_bits() {
                        &result
                    } else {
                        &body
                    },
                );
                self.lambdas.insert(std::ptr::from_ref(node), term.clone());
                self.lambda_places.insert(
                    std::ptr::from_ref(node),
                    (
                        node.span,
                        params
                            .iter()
                            .map(|parameter| parameter.span.or(node.span))
                            .collect(),
                    ),
                );
                Ok(term)
            }
            SNode::CellAllocate { value, .. } => {
                let element = self.fresh(false);
                let value = self.expr(value, env)?;
                self.unify(&element, &value, node.span)?;
                let term = Term::Cell(Box::new(element));
                self.cells
                    .insert(std::ptr::from_ref(node), (term.clone(), node.span));
                Ok(term)
            }
            SNode::CellRead { cell } => {
                let cell = self.expr(cell, env)?;
                let resolved = self.resolve(&cell);
                let facts = TermFacts::new(&[Some(cell), Some(resolved)]);
                self.callable_read_cell_from_facts(&facts, node.span)
            }
            SNode::CellWrite { cell, value, body } => {
                let value = self.expr(value, env)?;
                let cell = self.expr(cell, env)?;
                self.unify(&cell, &Term::Cell(Box::new(value.clone())), node.span)?;
                let facts =
                    TermFacts::new(&[Some(self.resolve(&cell)), Some(self.resolve(&value))]);
                let plan = rules::plan_callable_cell_option_binding(
                    &facts.texts,
                    &facts.starts,
                    &facts.counts,
                    &facts.labels,
                    &facts.targets,
                    facts.position(0),
                    facts.position(1),
                );
                if let [variable, value] = plan.as_slice() {
                    let variable = facts
                        .value(*variable)
                        .and_then(serde_json::Value::as_u64)
                        .and_then(|value| usize::try_from(value).ok())
                        .expect("the generated binding selects an actual variable ID");
                    self.bindings[variable] = Some(endpoint(&facts, *value));
                }
                self.expr_expected(body, env, expected)
            }
            _ => unreachable!("callable dispatch selects only callable families"),
        }
    }

    fn callable_selected_type(
        &mut self,
        facts: &TermFacts,
        identity: f64,
        declared: bool,
    ) -> Result<Term> {
        if declared {
            let ty: Type = serde_json::from_value(
                facts
                    .materialize(identity)
                    .expect("a declared slot is present"),
            )
            .map_err(|error| {
                type_error(format!("a callable annotation is malformed: {error}"), None)
            })?;
            Ok(self.declared(Some(&ty)))
        } else {
            Ok(facts.term(identity).unwrap_or_else(|| self.fresh(false)))
        }
    }

    fn callable_read_cell_from_facts(
        &mut self,
        facts: &TermFacts,
        place: Option<Span>,
    ) -> Result<Term> {
        let plan = rules::plan_callable_cell_read_inference(
            &facts.texts,
            &facts.starts,
            &facts.counts,
            &facts.labels,
            &facts.targets,
            facts.position(0),
            facts.position(1),
        );
        if let Some(element) = facts.term(plan[0]) {
            return Ok(element);
        }
        let element = self.fresh(false);
        self.unify(
            &endpoint(facts, plan[1]),
            &Term::Cell(Box::new(element.clone())),
            place,
        )?;
        Ok(element)
    }
}
