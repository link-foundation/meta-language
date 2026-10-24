//! Actual homogeneous value owners participate in the ordinary constraint loop.
use super::{Env, Inference, Result, Span, Term, Type};
use crate::translation::surface::SArrayItem;
pub(super) struct Enumeration {
    pub(super) object: Term,
    pub(super) element: Term,
    pub(super) place: Option<Span>,
}
impl Inference {
    pub(super) fn enumeration_values_term(
        &mut self,
        args: &[SArrayItem],
        env: &Env,
        place: Option<Span>,
    ) -> Result<Term> {
        let values = args
            .iter()
            .map(|argument| self.expr(&argument.value, env))
            .collect::<Result<Vec<_>>>()?;
        let element = self.fresh(false);
        self.enumerations.push(Enumeration {
            object: values.first().cloned().unwrap_or(Term::Known(Type::Unit)),
            element: element.clone(),
            place,
        });
        Ok(Term::Array(Box::new(element)))
    }
    pub(super) fn solve_enumerations(&mut self) -> Result<bool> {
        let mut waiting = Vec::new();
        let mut progress = false;
        for constraint in std::mem::take(&mut self.enumerations) {
            let mut object = self.resolve(&constraint.object);
            while let Term::Option { element, .. } = object {
                object = self.resolve(&element);
            }
            if matches!(&object, Term::Var(_))
                || matches!(&object,Term::Record(record)if record.borrow().open)
            {
                waiting.push(constraint);
                continue;
            }
            let members = match object {
                Term::Record(record) => record
                    .borrow()
                    .fields
                    .iter()
                    .map(|(_, term)| term.clone())
                    .collect(),
                Term::Tuple(elements) => elements,
                Term::Array(element) => vec![*element],
                Term::Known(Type::String) => vec![Term::Known(Type::String)],
                _ => Vec::new(),
            };
            for member in members {
                self.unify(&constraint.element, &member, constraint.place)?;
            }
            progress = true;
        }
        self.enumerations = waiting;
        Ok(progress)
    }
}
