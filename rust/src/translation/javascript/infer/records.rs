//! Retained record owners connected to the complete shared planning kernels.
use super::term_facts::TermFacts;
use super::{Env, Inference, Rc, RecordTerm, RefCell, Result, SExpr, Span, Term, type_error};
use crate::translation::frontend_rules::ml_array::number_index;
use crate::translation::record_value_rules as rules;
use crate::translation::surface::SRecordEntry;

pub(super) struct RecordSpreadEntry {
    name: Option<String>,
    spread: bool,
    term: Term,
}

pub(super) struct RecordSpread {
    owner: Rc<RefCell<RecordTerm>>,
    entries: Vec<RecordSpreadEntry>,
    pub(super) place: Option<Span>,
}
pub(super) struct PendingNominalSpread {
    node: SExpr,
    pointer: *const SExpr,
    entries: Vec<SRecordEntry>,
    env: Env,
    expected: Option<Term>,
    leading: Term,
    terms: Vec<Term>,
    result: Term,
    pub(super) place: Option<Span>,
}

fn names(fields: &[(String, Term)]) -> Vec<Vec<f64>> {
    fields
        .iter()
        .map(|(name, _)| name.encode_utf16().map(f64::from).collect())
        .collect()
}

fn position(value: f64, length: usize) -> usize {
    number_index(value)
        .filter(|index| *index < length)
        .expect("a generated record plan selects an actual retained field")
}

fn spread_role(term: &Term) -> String {
    let kind = match term {
        Term::Var(_) => "var",
        Term::Record(_) => "record",
        Term::Known(ty) => ty.kind(),
        _ => "",
    };
    rules::read_record_spread_role(kind)
}

fn insertion_fields(fields: &[(String, Term)]) -> Vec<(String, Term)> {
    rules::read_record_field_insertion_order(&names(fields))
        .into_iter()
        .map(|index| fields[position(index, fields.len())].clone())
        .collect()
}

impl Inference {
    fn add_record_field(
        &mut self,
        owner: &Rc<RefCell<RecordTerm>>,
        field: &(String, Term),
        place: Option<Span>,
    ) -> Result<()> {
        let mut inputs = vec![Some(field.1.clone()), Some(Term::Record(owner.clone()))];
        inputs.extend(self.bindings.iter().cloned());
        let facts = TermFacts::new(&inputs);
        let bindings = (0..self.bindings.len())
            .map(|index| facts.position(index + 2))
            .collect::<Vec<_>>();
        if rules::read_record_type_owner_presence(
            &facts.texts,
            &facts.numbers(),
            &facts.starts,
            &facts.counts,
            &facts.labels,
            &facts.targets,
            &bindings,
            facts.position(0),
            facts.position(1),
        ) {
            return Err(type_error(
                "a recursive inferred type needs an explicit data type",
                place,
            ));
        }
        owner.borrow_mut().fields.push(field.clone());
        self.unification_changes += 1;
        Ok(())
    }

    pub(super) fn unify_records(
        &mut self,
        left: &Rc<RefCell<RecordTerm>>,
        right: &Rc<RefCell<RecordTerm>>,
        place: Option<Span>,
    ) -> Result<()> {
        let mut phase = 0.0;
        let mut index = 0.0;
        loop {
            // Re-read both actual owners after each recursive unify/addition;
            // snapshots never replace their shared mutable field identities.
            let (left_fields, left_open) = {
                let left = left.borrow();
                (left.fields.clone(), left.open)
            };
            let (right_fields, right_open) = {
                let right = right.borrow();
                (right.fields.clone(), right.open)
            };
            let plan = rules::read_record_type_constraint_step(
                &names(&left_fields),
                &names(&right_fields),
                left_open,
                right_open,
                phase,
                index,
            );
            let [action, left_index, right_index, next_phase, next_index] = plan.as_slice() else {
                unreachable!("the shared record constraint step has five cells")
            };
            match number_index(*action).expect("a record constraint action is integral") {
                0 => self.unify(
                    &left_fields[position(*left_index, left_fields.len())].1,
                    &right_fields[position(*right_index, right_fields.len())].1,
                    place,
                )?,
                1 => self.add_record_field(
                    right,
                    &left_fields[position(*left_index, left_fields.len())],
                    place,
                )?,
                2 => self.add_record_field(
                    left,
                    &right_fields[position(*right_index, right_fields.len())],
                    place,
                )?,
                3 => {
                    return Err(type_error(
                        rules::read_record_type_constraint_error(
                            *action,
                            &left_fields[position(*left_index, left_fields.len())].0,
                        ),
                        place,
                    ));
                }
                4 => {
                    return Err(type_error(
                        rules::read_record_type_constraint_error(
                            *action,
                            &right_fields[position(*right_index, right_fields.len())].0,
                        ),
                        place,
                    ));
                }
                5 => {}
                6 => return Ok(()),
                _ => unreachable!("the shared record constraint action is declared"),
            }
            phase = *next_phase;
            index = *next_index;
        }
    }

    pub(super) fn record_field_constraint(
        &mut self,
        owner: &Rc<RefCell<RecordTerm>>,
        field: &str,
        result: &Term,
        place: Option<Span>,
    ) -> Result<bool> {
        let (fields, open) = {
            let owner = owner.borrow();
            (owner.fields.clone(), owner.open)
        };
        let form=crate::translation::javascript_record_parameter_rules::read_java_script_record_formal_default_field_form(
            &crate::translation::javascript_record_parameter_rules::ml_string::Text::from_text("record"),open,fields.iter().any(|item|item.0==field),
            &crate::translation::javascript_record_parameter_rules::ml_string::Text::from_text(field)).to_string();
        if form == "absent" {
            let element = self.fresh(false);
            self.unify(
                result,
                &super::optional_values::optional(element, "undefined"),
                place,
            )?;
            return Ok(true);
        }
        let name = field.encode_utf16().map(f64::from).collect::<Vec<_>>();
        let plan = rules::read_record_field_inference_step(&names(&fields), &name, open);
        let [action, selected] = plan.as_slice() else {
            unreachable!("a record field plan has two cells")
        };
        match number_index(*action).expect("a record field action is integral") {
            0 => self.unify(result, &fields[position(*selected, fields.len())].1, place)?,
            1 => {
                self.add_record_field(owner, &(field.to_owned(), result.clone()), place)?;
                self.unify(result, result, place)?;
            }
            2 => {}
            _ => unreachable!("the shared record field action is declared"),
        }
        Ok(true)
    }

    pub(super) fn record_expr(
        &mut self,
        node: &SExpr,
        entries: &[SRecordEntry],
        env: &Env,
        expected: Option<&Term>,
    ) -> Result<Term> {
        self.record_expr_leading(node, entries, env, expected, None, None)
    }
    fn record_expr_leading(
        &mut self,
        node: &SExpr,
        entries: &[SRecordEntry],
        env: &Env,
        expected: Option<&Term>,
        leading_term: Option<Term>,
        entry_terms: Option<&[Term]>,
    ) -> Result<Term> {
        let wanted = expected.map(|term| self.resolve(term));
        let leading = if let Some(term) = leading_term {
            Some(term)
        } else if let Some(entry) = entries.first().filter(|entry| entry.spread) {
            Some(self.expr(&entry.value, env)?)
        } else {
            None
        };
        let source = leading.as_ref().map(|term| match self.resolve(term) {
            Term::Option { element, .. } => self.resolve(&element),
            other => other,
        });
        let owner = match &wanted {
            Some(Term::Known(crate::translation::types::Type::Data { name })) => Some(name.clone()),
            _ => match &source {
                Some(Term::Known(crate::translation::types::Type::Data { name })) => {
                    Some(name.clone())
                }
                _ => None,
            },
        };
        if let Some(leading) = leading
            .as_ref()
            .filter(|_| matches!(source, Some(Term::Var(_))) && owner.is_none())
        {
            let mut terms = Vec::new();
            for (index, entry) in entries.iter().enumerate() {
                terms.push(if index == 0 {
                    leading.clone()
                } else {
                    self.expr(&entry.value, env)?
                });
            }
            let result = self.fresh(false);
            self.nominal_spreads.push(PendingNominalSpread {
                node: node.clone(),
                pointer: std::ptr::from_ref(node),
                entries: entries.to_vec(),
                env: env.clone(),
                expected: expected.cloned(),
                leading: leading.clone(),
                terms,
                result: result.clone(),
                place: node.span,
            });
            return Ok(result);
        }
        if let Some(owner) = owner.filter(|_| entries.first().is_some_and(|entry| entry.spread)) {
            let ctors = self
                .data
                .iter()
                .find(|(name, _)| name == &owner)
                .map(|(_, ctors)| ctors.clone())
                .unwrap_or_default();
            let ctor = ctors.first();
            let nominal = Term::Known(crate::translation::types::Type::Data {
                name: owner.clone(),
            });
            let mut owners = Vec::new();
            for (index, entry) in entries.iter().enumerate() {
                if entry.spread {
                    let term = if let Some(term) = entry_terms.and_then(|terms| terms.get(index)) {
                        term.clone()
                    } else if index == 0 {
                        leading.clone().expect("actual leading spread")
                    } else {
                        self.expr(&entry.value, env)?
                    };
                    let present = match self.resolve(&term) {
                        Term::Option { element, .. } => *element,
                        _ => term,
                    };
                    self.unify(&present, &nominal, entry.value.span.or(node.span))?;
                    owners.push(
                        if let Term::Known(crate::translation::types::Type::Data { name }) =
                            self.resolve(&present)
                        {
                            name
                        } else {
                            String::new()
                        },
                    );
                } else {
                    let field = ctor.and_then(|ctor| {
                        ctor.fields
                            .iter()
                            .enumerate()
                            .find(|(_, field)| field.name.as_deref() == entry.name.as_deref())
                    });
                    let declared = field.map(|(index, _)| {
                        self.field(
                            &owner,
                            &ctor.expect("actual declared constructor").name,
                            index,
                        )
                    });
                    let value = if let Some(term) = entry_terms.and_then(|terms| terms.get(index)) {
                        term.clone()
                    } else {
                        self.expr_expected(&entry.value, env, declared.as_ref())?
                    };
                    if let Some(declared) = declared {
                        self.unify(&value, &declared, entry.value.span.or(node.span))?;
                    }
                    owners.push(String::new());
                }
            }
            let fields = ctor
                .map(|ctor| {
                    ctor.fields
                        .iter()
                        .map(|field| field.name.clone().unwrap_or_default())
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default();
            let kinds = entries
                .iter()
                .map(|entry| if entry.spread { "spread" } else { "field" }.to_owned())
                .collect::<Vec<_>>();
            let names = entries
                .iter()
                .map(|entry| {
                    if entry.spread {
                        String::new()
                    } else {
                        entry.name.clone().unwrap_or_default()
                    }
                })
                .collect::<Vec<_>>();
            let prototype = matches!(
                &node.node,
                crate::translation::surface::SNode::Record {
                    prototype_definition: Some(_),
                    ..
                }
            );
            let count = serde_json::Number::from(ctors.len())
                .as_f64()
                .expect("actual constructor count is a Number");
            let error=crate::translation::javascript_nominal_spread_rules::read_java_script_nominal_spread_error("JavaScript",count,&fields,&kinds,&names,&owners,&owner,prototype);
            if !error.is_empty() {
                return Err(type_error(error, node.span));
            }
            self.records
                .insert(std::ptr::from_ref(node), nominal.clone());
            return Ok(nominal);
        }
        let wanted = match wanted {
            Some(Term::Record(record)) => record.borrow().fields.clone(),
            _ => Vec::new(),
        };
        let mut fields = Vec::new();
        let mut retained = Vec::new();
        let mut open = false;
        for (entry_index, entry) in entries.iter().enumerate() {
            if entry.spread {
                let term = if let Some(term) = entry_terms.and_then(|terms| terms.get(entry_index))
                {
                    term.clone()
                } else if entry_index == 0 {
                    if let Some(term) = leading.clone() {
                        term
                    } else {
                        self.expr(&entry.value, env)?
                    }
                } else {
                    self.expr(&entry.value, env)?
                };
                retained.push(RecordSpreadEntry {
                    name: None,
                    spread: true,
                    term: term.clone(),
                });
                let source = self.resolve(&term);
                match spread_role(&source).as_str() {
                    "open" => open = true,
                    "fields" => {
                        let Term::Record(record) = source else {
                            unreachable!("the shared spread role selects an actual record")
                        };
                        fields.extend(record.borrow().fields.iter().cloned());
                    }
                    _ => {
                        return Err(type_error(
                            "an object spread needs a known record shape",
                            entry.value.span.or(node.span),
                        ));
                    }
                }
            } else {
                let name = entry
                    .name
                    .as_deref()
                    .expect("an owned ordinary record field has its name");
                let position = rules::read_record_field_position(
                    &names(&wanted),
                    &name.encode_utf16().map(f64::from).collect::<Vec<_>>(),
                );
                let declared = number_index(position)
                    .and_then(|index| wanted.get(index))
                    .map(|(_, term)| term);
                let value = if let Some(term) = entry_terms.and_then(|terms| terms.get(entry_index))
                {
                    term.clone()
                } else {
                    self.expr_expected(&entry.value, env, declared)?
                };
                if let Some(declared) = declared {
                    self.unify(&value, declared, entry.value.span.or(node.span))?;
                }
                let term = declared.cloned().unwrap_or(value);
                fields.push((name.to_owned(), term.clone()));
                retained.push(RecordSpreadEntry {
                    name: Some(name.to_owned()),
                    spread: false,
                    term,
                });
            }
        }
        let owner = Rc::new(RefCell::new(RecordTerm {
            fields: insertion_fields(&fields),
            open,
        }));
        let term = Term::Record(owner.clone());
        if open {
            self.record_spreads.push(RecordSpread {
                owner,
                entries: retained,
                place: node.span,
            });
        }
        self.records.insert(std::ptr::from_ref(node), term.clone());
        Ok(term)
    }
    pub(super) fn solve_nominal_spreads(&mut self) -> Result<bool> {
        let mut progress = false;
        let mut waiting = Vec::new();
        for constraint in std::mem::take(&mut self.nominal_spreads) {
            let source = match self.resolve(&constraint.leading) {
                Term::Option { element, .. } => self.resolve(&element),
                other => other,
            };
            let expected_data = constraint.expected.as_ref().is_some_and(|term| {
                matches!(
                    self.resolve(term),
                    Term::Known(crate::translation::types::Type::Data { .. })
                )
            });
            if matches!(source, Term::Var(_)) && !expected_data {
                waiting.push(constraint);
                continue;
            }
            let result = self.record_expr_leading(
                &constraint.node,
                &constraint.entries,
                &constraint.env,
                constraint.expected.as_ref(),
                Some(constraint.leading),
                Some(&constraint.terms),
            )?;
            self.records.remove(&std::ptr::from_ref(&constraint.node));
            self.records.insert(constraint.pointer, result.clone());
            self.unify(&constraint.result, &result, constraint.place)?;
            progress = true;
        }
        self.nominal_spreads.extend(waiting);
        Ok(progress)
    }

    pub(super) fn solve_record_spreads(&mut self) -> Result<bool> {
        let mut progress = false;
        let mut waiting = Vec::new();
        for constraint in std::mem::take(&mut self.record_spreads) {
            if constraint
                .entries
                .iter()
                .any(|entry| entry.spread && spread_role(&self.resolve(&entry.term)) == "open")
            {
                waiting.push(constraint);
                continue;
            }
            let mut fields = Vec::new();
            for entry in &constraint.entries {
                if entry.spread {
                    let source = self.resolve(&entry.term);
                    if spread_role(&source) != "fields" {
                        return Err(type_error(
                            "an object spread needs a known record shape",
                            constraint.place,
                        ));
                    }
                    let Term::Record(record) = source else {
                        unreachable!("the shared spread role selects an actual record")
                    };
                    fields.extend(record.borrow().fields.iter().cloned());
                } else {
                    fields.push((
                        entry
                            .name
                            .clone()
                            .expect("a retained ordinary field has its name"),
                        entry.term.clone(),
                    ));
                }
            }
            let shape = Rc::new(RefCell::new(RecordTerm {
                fields: insertion_fields(&fields),
                open: false,
            }));
            self.unify(
                &Term::Record(constraint.owner.clone()),
                &Term::Record(shape),
                constraint.place,
            )?;
            constraint.owner.borrow_mut().open = false;
            progress = true;
        }
        self.record_spreads = waiting;
        Ok(progress)
    }
}
