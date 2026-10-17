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
        let wanted = expected.map(|term| self.resolve(term));
        let wanted = match wanted {
            Some(Term::Record(record)) => record.borrow().fields.clone(),
            _ => Vec::new(),
        };
        let mut fields = Vec::new();
        let mut retained = Vec::new();
        let mut open = false;
        for entry in entries {
            if entry.spread {
                let term = self.expr(&entry.value, env)?;
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
                let value = self.expr_expected(&entry.value, env, declared)?;
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
