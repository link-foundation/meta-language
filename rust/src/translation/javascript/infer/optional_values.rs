//! Apply generated optional plans to the common solver's actual term owners.
use super::term_facts::TermFacts;
use super::{
    Coalescing, Env, FLOAT, FieldOf, HashSet, Inference, Join, Rc, RefCell, Result, SExpr, SNode,
    Span, Term,
};
use crate::translation::frontend_rules::ml_array::number_index;
use crate::translation::optional_inference_rules as plans;
use crate::translation::optional_value_rules as values;
use crate::translation::parameter_default_rules::read_parameter_default_argument_inference;
use crate::translation::structured_type_rules::merge_optional_absence;

fn endpoint(facts: &TermFacts, identity: f64) -> Term {
    facts
        .term(identity)
        .expect("an optional plan retains the actual selected term")
}

pub(super) fn optional(element: Term, absence: &str) -> Term {
    Term::Option {
        element: Box::new(element),
        absence: Rc::new(RefCell::new(absence.to_owned())),
    }
}

pub(super) const fn kind(term: &Term) -> &str {
    match term {
        Term::Var(_) => "var",
        Term::Known(ty) => ty.kind(),
        Term::Array(_) => "array",
        Term::Option { .. } => "option",
        Term::Tuple(_) => "tuple",
        Term::Record(_) => "record",
        Term::Union(_) => "union",
        Term::Map { .. } => "map",
        Term::Set { .. } => "set",
        Term::Function { .. } => "function",
        Term::Cell(_) => "cell",
    }
}

impl Inference {
    pub(super) fn parameter_default_argument_inference(
        &self,
        defaulted: bool,
        optional_parameter: bool,
        argument: &Term,
    ) -> Option<(Term, bool)> {
        let actual = self.resolve(argument);
        let absence = match &actual {
            Term::Option { absence, .. } => absence.borrow().clone(),
            _ => String::new(),
        };
        match read_parameter_default_argument_inference(
            defaulted,
            optional_parameter,
            kind(&actual),
            &absence,
        )
        .as_str()
        {
            "defer" => None,
            "element" => {
                let Term::Option { element, .. } = actual else {
                    unreachable!(
                        "the generated default inference selects the actual optional child"
                    );
                };
                Some((*element, true))
            }
            "value" => Some((actual, true)),
            _ => Some((argument.clone(), false)),
        }
    }

    pub(super) fn same_term(&self, left: &Term, right: &Term) -> bool {
        let a = self.resolve(left);
        let b = self.resolve(right);
        let facts = TermFacts::resolved(&[Some(a), Some(b)], &|term| self.resolve(term));
        let plan = plans::plan_inference_term_identity(
            &facts.texts,
            &facts.starts,
            &facts.counts,
            &facts.labels,
            &facts.targets,
            facts.position(0),
            facts.position(1),
        );
        !plan.is_empty()
            && plan[1..]
                .as_chunks::<2>()
                .0
                .iter()
                .all(|pair| self.same_term(&endpoint(&facts, pair[0]), &endpoint(&facts, pair[1])))
    }

    pub(super) fn unify_optional(
        &mut self,
        left: &Term,
        right: &Term,
        place: Option<Span>,
    ) -> Result<()> {
        let facts = TermFacts::new(&[Some(left.clone()), Some(right.clone())]);
        let plan = plans::plan_optional_type_constraints(
            &facts.texts,
            &facts.starts,
            &facts.counts,
            &facts.labels,
            &facts.targets,
            facts.position(0),
            facts.position(1),
        );
        self.unify(
            &endpoint(&facts, plan[0]),
            &endpoint(&facts, plan[1]),
            place,
        )?;
        if let (
            Some(Term::Option { absence: first, .. }),
            Some(Term::Option {
                absence: second, ..
            }),
        ) = (facts.term(plan[2]), facts.term(plan[3]))
        {
            let merged = merge_optional_absence(&first.borrow(), &second.borrow());
            if *first.borrow() != merged || *second.borrow() != merged {
                self.unification_changes += 1;
            }
            first.borrow_mut().clone_from(&merged);
            *second.borrow_mut() = merged;
        }
        Ok(())
    }

    pub(super) fn optional_expr(
        &mut self,
        node: &SExpr,
        env: &Env,
        expected: Option<&Term>,
    ) -> Result<Term> {
        match &node.node {
            SNode::None { absence } => Ok(optional(self.fresh(false), absence)),
            SNode::Some { value, ty } => {
                let declared = self.declared(Some(ty));
                let Term::Option { element, .. } = &declared else {
                    return Ok(declared);
                };
                let value = self.expr_expected(value, env, Some(element))?;
                self.unify(&value, element, node.span)?;
                Ok(declared)
            }
            SNode::FlattenOption { value } => self.expr(value, env),
            SNode::Coalesce { left, right } => {
                let left = self.expr(left, env)?;
                let right = self.expr_expected(right, env, expected)?;
                let constraint = Coalescing {
                    left,
                    right: right.clone(),
                    place: node.span,
                };
                if !self.coalescing_step(&constraint, false)? {
                    self.coalesces.push(constraint);
                }
                Ok(right)
            }
            SNode::OptionalField { object, field } => {
                let object = self.expr(object, env)?;
                let facts = TermFacts::new(&[Some(object.clone()), Some(self.resolve(&object))]);
                let plan = plans::plan_optional_field_inference(
                    &facts.texts,
                    &facts.starts,
                    &facts.counts,
                    &facts.labels,
                    &facts.targets,
                    facts.position(0),
                    facts.position(1),
                );
                let element = facts.term(plan[0]).unwrap_or_else(|| self.fresh(false));
                if let Some(bind) = plan.get(1).and_then(|identity| facts.term(*identity)) {
                    self.unify(&bind, &optional(element.clone(), "both"), node.span)?;
                }
                let field_type = self.fresh(false);
                self.fields.push(FieldOf {
                    object: element,
                    field: field.clone(),
                    result: field_type.clone(),
                    place: node.span,
                });
                Ok(optional(field_type, "undefined"))
            }
            _ => unreachable!("optional inference dispatch selects optional Surface nodes"),
        }
    }

    pub(super) fn coalescing_step(&mut self, constraint: &Coalescing, force: bool) -> Result<bool> {
        let facts = TermFacts::new(&[
            Some(constraint.left.clone()),
            Some(self.resolve(&constraint.left)),
            Some(constraint.right.clone()),
        ]);
        let plan = plans::plan_optional_coalescing_constraints(
            &facts.texts,
            &facts.starts,
            &facts.counts,
            &facts.labels,
            &facts.targets,
            facts.position(0),
            facts.position(1),
            facts.position(2),
            force,
        );
        if plan.is_empty() {
            return Ok(false);
        }
        let right = endpoint(&facts, plan[2]);
        let right = if number_index(plan[0]) == Some(1) {
            optional(right, "both")
        } else {
            right
        };
        self.unify(&endpoint(&facts, plan[1]), &right, constraint.place)?;
        Ok(true)
    }

    pub(super) fn additional_absence(&self, actual: &Term, declared: &Term) -> bool {
        let facts = TermFacts::new(&[Some(self.resolve(actual)), Some(self.resolve(declared))]);
        let plan = plans::plan_optional_additional_absence(
            &facts.texts,
            &facts.starts,
            &facts.counts,
            &facts.labels,
            &facts.targets,
            facts.position(0),
            facts.position(1),
        );
        number_index(plan[0]) == Some(1)
            || plan[1..].as_chunks::<2>().0.iter().any(|pair| {
                self.additional_absence(&endpoint(&facts, pair[0]), &endpoint(&facts, pair[1]))
            })
    }

    pub(super) fn join(&mut self, left: Term, right: Term, place: Option<Span>) -> Term {
        let result = self.fresh(false);
        self.joins.push(Join {
            left,
            right,
            result: result.clone(),
            place,
        });
        result
    }

    fn join_step(&mut self, constraint: &Join) -> Result<bool> {
        let facts = TermFacts::new(&[
            Some(constraint.left.clone()),
            Some(constraint.right.clone()),
            Some(self.resolve(&constraint.left)),
            Some(self.resolve(&constraint.right)),
            Some(self.resolve(&constraint.result)),
        ]);
        let plan = plans::plan_optional_join_constraints(
            &facts.texts,
            &facts.starts,
            &facts.counts,
            &facts.labels,
            &facts.targets,
            facts.position(0),
            facts.position(1),
            facts.position(2),
            facts.position(3),
            facts.position(4),
        );
        let before = self.unification_changes;
        if plan[0] == 0.0 {
            self.unify(
                &endpoint(&facts, plan[2]),
                &endpoint(&facts, plan[3]),
                constraint.place,
            )?;
        }
        let selected = endpoint(&facts, plan[1]);
        let value = if number_index(plan[0]) == Some(2) {
            optional(selected, "undefined")
        } else {
            self.resolve(&selected)
        };
        let Term::Var(variable) = constraint.result else {
            unreachable!("a deferred join owns a result variable");
        };
        let current = self.resolve(&constraint.result);
        let alias = match current {
            Term::Var(id) => serde_json::Number::from(id)
                .as_f64()
                .expect("a bounded inference variable has a Number representation"),
            _ => -1.0,
        };
        let original_id = serde_json::Number::from(variable)
            .as_f64()
            .expect("a bounded inference variable has a Number representation");
        if values::require_inference_join_alias(kind(&current), alias, original_id) {
            self.unify(&current, &value, constraint.place)?;
        }
        let original = matches!(value, Term::Var(id) if id == variable);
        let same = !original && self.same_term(&constraint.result, &value);
        let recursive = !original && !same && self.contains(&value, variable, &mut HashSet::new());
        match values::read_optional_join_binding(original, same, recursive).as_str() {
            "identity" => Ok(false),
            "changes" => Ok(self.unification_changes != before),
            _ => {
                self.bindings[variable] = Some(value);
                Ok(true)
            }
        }
    }

    pub(super) fn solve_optional(&mut self) -> Result<bool> {
        let mut progress = false;
        let mut waiting = Vec::new();
        for constraint in std::mem::take(&mut self.coalesces) {
            if self.coalescing_step(&constraint, false)? {
                progress = true;
            } else {
                waiting.push(constraint);
            }
        }
        self.coalesces = waiting;
        Ok(progress)
    }

    pub(super) fn solve_joins_and_results(&mut self) -> Result<bool> {
        let mut progress = false;
        let joins = std::mem::take(&mut self.joins);
        for constraint in &joins {
            progress |= self.join_step(constraint)?;
        }
        self.joins = joins;
        for result in self.results.clone() {
            let actual = self.resolve(&result.body);
            let additional = result
                .declared
                .as_ref()
                .is_some_and(|declared| self.additional_absence(&result.body, declared));
            let eligible = (result.declared.is_none() || additional)
                && kind(&actual) != "var"
                && matches!(result.result, Term::Var(_));
            let same = eligible && self.same_term(&result.result, &actual);
            let recursive = match result.result {
                Term::Var(id) if eligible && !same => {
                    self.contains(&actual, id, &mut HashSet::new())
                }
                _ => false,
            };
            if values::require_optional_result_binding(
                result.declared.is_some(),
                additional,
                kind(&actual),
                kind(&result.result),
                same,
                recursive,
            ) {
                let Term::Var(variable) = result.result else {
                    unreachable!("the generated result plan requires a variable");
                };
                self.bindings[variable] = Some(actual);
                progress = true;
            }
        }
        for call in self.calls.clone() {
            for (index, (argument, parameter)) in
                call.arguments.iter().zip(&call.parameters).enumerate()
            {
                let Some((argument, selected)) = self.parameter_default_argument_inference(
                    call.defaults[index],
                    call.optional_params[index],
                    argument,
                ) else {
                    continue;
                };
                let actual = self.resolve(&argument);
                if selected {
                    let changes = self.unification_changes;
                    self.unify(&actual, parameter, call.place)?;
                    progress |= changes != self.unification_changes;
                }
                let current = self.resolve(parameter);
                let recursive = matches!(parameter, Term::Var(id) if kind(&actual) == "option" && kind(&current) != "option" && self.contains(&actual, *id, &mut HashSet::new()));
                let current_absence = match &current {
                    Term::Option { absence, .. } => absence.borrow().clone(),
                    _ => String::new(),
                };
                let actual_absence = match &actual {
                    Term::Option { absence, .. } => absence.borrow().clone(),
                    _ => String::new(),
                };
                let operation = values::read_optional_call_binding(
                    kind(parameter),
                    kind(&actual),
                    kind(&current),
                    &current_absence,
                    &actual_absence,
                    recursive,
                );
                match operation.as_str() {
                    "elements" | "merge" => {
                        let (
                            Term::Option {
                                element: first,
                                absence: first_absence,
                            },
                            Term::Option {
                                element: second,
                                absence: second_absence,
                            },
                        ) = (&current, &actual)
                        else {
                            unreachable!("the generated call plan selects actual optional owners");
                        };
                        self.unify(first, second, call.place)?;
                        if values::read_optional_call_binding(
                            kind(parameter),
                            kind(&actual),
                            kind(&current),
                            &first_absence.borrow(),
                            &second_absence.borrow(),
                            false,
                        ) == "merge"
                        {
                            "both".clone_into(&mut first_absence.borrow_mut());
                            progress = true;
                        }
                    }
                    "bind" => {
                        let Term::Var(variable) = parameter else {
                            unreachable!("the generated call binding requires a variable");
                        };
                        let Term::Option { element, absence } = &actual else {
                            unreachable!(
                                "the generated call binding copies an actual optional record"
                            );
                        };
                        // JS's explicit {...actual} copies this policy slot;
                        // its recursive element remains the same actual term.
                        self.bindings[*variable] =
                            Some(optional(element.as_ref().clone(), &absence.borrow()));
                        progress = true;
                    }
                    _ => {}
                }
            }
        }
        Ok(progress)
    }

    pub(super) fn numeric_value(&self, term: &Term) -> Term {
        self.numeric_value_with_strings(term, true)
    }

    pub(super) fn numeric_addition_value(&self, term: &Term) -> Term {
        self.numeric_value_with_strings(term, false)
    }

    fn numeric_value_with_strings(&self, term: &Term, strings: bool) -> Term {
        let resolved = self.resolve(term);
        if (strings || kind(&resolved) != "string") && crate::translation::string_number_rules::read_java_script_primitive_numeric_inference_kind(kind(&resolved)) == "float" { return Term::Known(FLOAT); }
        match values::read_optional_numeric_term(kind(&resolved)).as_str() {
            "element" => {
                let Term::Option { element, .. } = resolved else {
                    unreachable!("the generated numeric plan requires Option");
                };
                self.numeric_value_with_strings(&element, strings)
            }
            "float" => Term::Known(FLOAT),
            _ => term.clone(),
        }
    }
}
