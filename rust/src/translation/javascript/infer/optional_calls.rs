//! Actual terms into the existing callable unifier and deferred optional joins.
use super::{Env, Inference, Result, Span, Term, UNIT, optional_values::optional, type_error};
use crate::translation::surface::SArrayItem;
impl Inference {
    pub(super) fn optional_call_term(
        &mut self,
        args: &[SArrayItem],
        env: &Env,
        place: Option<Span>,
    ) -> Result<Term> {
        if args.is_empty() {
            return Err(type_error(
                "an optional call needs its actual receiver and ordered non-spread arguments",
                place,
            ));
        }
        let receiver = self.expr(&args[0].value, env)?;
        let resolved = self.resolve(&receiver);
        if matches!(
            &resolved,
            Term::Known(crate::translation::types::Type::Unit)
        ) || matches!(&resolved,Term::Option{element,..}if matches!(self.resolve(element),Term::Known(crate::translation::types::Type::Unit)))
        {
            return Ok(optional(Term::Known(UNIT), "undefined"));
        }
        let parameters = args[1..]
            .iter()
            .map(|argument| self.expr(&argument.value, env))
            .collect::<Result<Vec<_>>>()?;
        let ret = self.fresh(false);
        let candidate = Term::Function {
            params: parameters,
            ret: Box::new(ret.clone()),
        };
        if matches!(resolved, Term::Var(_)) {
            self.unify(&receiver, &optional(candidate, "both"), place)?;
        } else {
            let callable = match &resolved {
                Term::Option { element, .. } => element.as_ref(),
                _ => &receiver,
            };
            self.unify(callable, &candidate, place)?;
        }
        Ok(if matches!(resolved, Term::Function { .. }) {
            ret
        } else {
            self.join_optional_guard(ret, place)
        })
    }
}
