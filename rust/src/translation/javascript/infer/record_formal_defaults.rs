//! Actual record owners/terms transported to the complete shared field policy.
use super::{Env, FieldOf, Inference, Result, SNode, Span, Term, Type};
use crate::translation::javascript_record_parameter_rules::{self as rules, ml_string::Text};
pub(super) struct FieldRead {
    object: Term,
    field: String,
    result: Term,
    pub(super) place: Option<Span>,
}
impl Inference {
    pub(super) fn record_formal_default_term(
        &mut self,
        args: &[crate::translation::surface::SArrayItem],
        env: &Env,
        place: Option<Span>,
    ) -> Result<Term> {
        let (value, fallback) =
            crate::translation::record_formal_defaults::operands(args, place, "JavaScript")?;
        let SNode::Field { object, field } = &value.node else {
            unreachable!("actual default field operands")
        };
        let object = self.expr(object, env)?;
        let result = self.fresh(false);
        let fallback = self.expr(fallback, env)?;
        self.record_formal_fields.push(FieldRead {
            object,
            field: field.clone(),
            result: result.clone(),
            place,
        });
        Ok(self.binding_default_term(result, fallback, place))
    }
    pub(super) fn solve_record_formal_fields(&mut self) -> Result<bool> {
        let mut waiting = Vec::new();
        let mut progress = false;
        for constraint in std::mem::take(&mut self.record_formal_fields) {
            let owner = self.resolve(&constraint.object);
            let (kind, fields, open) = match &owner {
                Term::Var(_) => ("var", Vec::new(), false),
                Term::Record(owner) => {
                    let owner = owner.borrow();
                    ("record", owner.fields.clone(), owner.open)
                }
                _ => ("other", Vec::new(), false),
            };
            let present = fields.iter().find(|entry| entry.0 == constraint.field);
            let form = rules::read_java_script_record_formal_default_field_form(
                &Text::from_text(kind),
                open,
                present.is_some(),
                &Text::from_text(&constraint.field),
            )
            .to_string();
            if form == "wait" {
                waiting.push(constraint);
                continue;
            }
            if form == "prototype" {
                return Err(crate::translation::diagnostics::unsupported(
                    "record formal inherited property",
                    &format!(
                        "an absent own field cannot replace the actual prototype property {}",
                        constraint.field
                    ),
                    constraint.place,
                ));
            }
            if form == "absent" {
                self.unify(
                    &constraint.result,
                    &Term::Option {
                        element: Box::new(Term::Known(Type::Unit)),
                        absence: "undefined".to_owned(),
                    },
                    constraint.place,
                )?;
            } else if form == "present" {
                self.unify(
                    &constraint.result,
                    &present.expect("admitted actual own field").1,
                    constraint.place,
                )?;
            } else {
                self.fields.push(FieldOf {
                    object: constraint.object,
                    field: constraint.field,
                    result: constraint.result,
                    place: constraint.place,
                });
            }
            progress = true;
        }
        self.record_formal_fields = waiting;
        Ok(progress)
    }
}
