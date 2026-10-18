//! Actual inference-term transport for the complete generated Array.from plans.
use super::term_facts::TermFacts;
use super::{Env, FLOAT, Inference, IterableOf, Result, SExpr, STRING, Span, Term, type_error};
use crate::translation::array_from_rules as rules;
use crate::translation::frontend_rules::ml_array::number_index;

fn endpoint(facts: &TermFacts, identity: f64) -> Term {
    facts
        .term(identity)
        .expect("the generated plan selects an actual inference term")
}

impl Inference {
    pub(super) fn array_from(
        &mut self,
        node: &SExpr,
        args: &[SExpr],
        env: &Env,
        expected: Option<&Term>,
    ) -> Result<Term> {
        let arguments = args
            .iter()
            .map(|argument| self.expr(argument, env))
            .collect::<Result<Vec<_>>>()?;
        let mut facts = TermFacts::new(&[expected.cloned()]);
        let arguments_id = facts.array(&arguments);
        let plan = rules::plan_array_from_inference_inputs(
            &facts.texts,
            &facts.starts,
            &facts.counts,
            &facts.labels,
            &facts.targets,
            arguments_id,
            facts.position(0),
        );
        let element = if number_index(plan[0]) == Some(1) {
            endpoint(&facts, plan[1])
        } else {
            self.fresh(false)
        };
        let result = Term::Array(Box::new(element));
        if let Some(object) = facts.term(plan[2]) {
            self.iterables.push(IterableOf {
                object,
                mapper: facts.term(plan[3]),
                result: result.clone(),
                place: node.span,
            });
        }
        self.collections
            .insert(std::ptr::from_ref(node), (result.clone(), node.span));
        Ok(result)
    }

    pub(super) fn array_from_iterable_element(&mut self, object: &Term) -> Option<Term> {
        let object = match object {
            Term::Known(ty) => self.declared(Some(ty)),
            term => term.clone(),
        };
        let facts = TermFacts::new(&[Some(object)]);
        let plan = rules::plan_array_from_iterable_type(
            &facts.texts,
            &facts.starts,
            &facts.counts,
            &facts.labels,
            &facts.targets,
            facts.position(0),
        );
        match plan.first().and_then(|code| number_index(*code)) {
            Some(0 | 2) => Some(endpoint(&facts, plan[1])),
            Some(1) => Some(Term::Known(STRING)),
            Some(3) => Some(Term::Tuple(vec![
                endpoint(&facts, plan[1]),
                endpoint(&facts, plan[2]),
            ])),
            _ => None,
        }
    }

    pub(super) fn array_from_callback(
        &mut self,
        term: &Term,
        params: &[Term],
        ret: &Term,
        place: Option<Span>,
    ) -> Result<()> {
        let declared = match self.resolve(term) {
            Term::Known(ty) => self.declared(Some(&ty)),
            term => term,
        };
        let mut facts = TermFacts::new(&[Some(term.clone()), Some(declared), Some(ret.clone())]);
        let params_id = facts.array(params);
        let plan = rules::plan_array_from_callback_constraints(
            &facts.texts,
            &facts.starts,
            &facts.counts,
            &facts.labels,
            &facts.targets,
            facts.position(0),
            facts.position(1),
            params_id,
            facts.position(2),
        );
        if plan[0].to_bits() == (-1.0_f64).to_bits() {
            let declared_params = rules::read_array_from_field(
                &facts.starts,
                &facts.counts,
                &facts.labels,
                &facts.targets,
                facts.position(1),
                "params",
            );
            let declared_count = facts.counts[number_index(declared_params)
                .expect("a function has its actual parameter sequence")];
            let provided_count = facts.counts
                [number_index(params_id).expect("the supplied parameter sequence exists")];
            return Err(type_error(
                rules::read_array_from_callback_error(declared_count, provided_count),
                place,
            ));
        }
        if number_index(plan[0]) == Some(1) {
            return self.unify(
                &endpoint(&facts, plan[1]),
                &Term::Function {
                    params: params.to_vec(),
                    ret: Box::new(endpoint(&facts, plan[3])),
                },
                place,
            );
        }
        for pair in plan[1..].as_chunks::<2>().0 {
            self.unify(
                &endpoint(&facts, pair[0]),
                &endpoint(&facts, pair[1]),
                place,
            )?;
        }
        Ok(())
    }

    pub(super) fn solve_array_from_iterable(
        &mut self,
        constraint: &IterableOf,
        element: Term,
    ) -> Result<()> {
        let Term::Array(result) = &constraint.result else {
            unreachable!("an iterable constraint retains its original array result");
        };
        if let Some(mapper) = &constraint.mapper {
            self.array_from_callback(
                mapper,
                &[element, Term::Known(FLOAT)],
                result,
                constraint.place,
            )
        } else {
            self.unify(result, &element, constraint.place)
        }
    }
}
