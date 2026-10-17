//! Actual inference-term transport for the complete collection construction plans.

use super::term_facts::TermFacts;
use super::{Env, Inference, Result, SExpr, Term};
use crate::translation::collection_construction_rules as rules;
use crate::translation::frontend_rules::ml_array::number_index;

fn endpoint(facts: &TermFacts, identity: f64) -> Term {
    facts
        .term(identity)
        .expect("the collection plan selects an actual inference term")
}

impl Inference {
    pub(super) fn collection_construction(
        &mut self,
        node: &SExpr,
        items: &SExpr,
        env: &Env,
        expected: Option<&Term>,
    ) -> Result<Term> {
        let expected = expected.map(|term| match self.resolve(term) {
            Term::Known(ty) => self.declared(Some(&ty)),
            term => term,
        });
        let facts = TermFacts::new(&[expected]);
        let plan = rules::plan_collection_construction_inference(
            &facts.texts,
            &facts.starts,
            &facts.counts,
            &facts.labels,
            &facts.targets,
            node.node.kind(),
            facts.position(0),
        );
        let contextual = number_index(plan[1]) == Some(1);
        let first = if contextual {
            endpoint(&facts, plan[2])
        } else {
            self.fresh(false)
        };
        let (element, result) = match number_index(plan[0]) {
            Some(1) => (
                first.clone(),
                Term::Set {
                    element: Box::new(first),
                    reference: false,
                },
            ),
            Some(2) => {
                let second = if contextual {
                    endpoint(&facts, plan[3])
                } else {
                    self.fresh(false)
                };
                (
                    Term::Tuple(vec![first.clone(), second.clone()]),
                    Term::Map {
                        key: Box::new(first),
                        value: Box::new(second),
                        reference: false,
                    },
                )
            }
            _ => unreachable!("only actual collection constructors reach this adapter"),
        };
        let expected_items = Term::Array(Box::new(element));
        let actual_items = self.expr_expected(items, env, Some(&expected_items))?;
        self.unify(&actual_items, &expected_items, node.span)?;
        self.collections
            .insert(std::ptr::from_ref(node), (result.clone(), node.span));
        Ok(result)
    }
}
