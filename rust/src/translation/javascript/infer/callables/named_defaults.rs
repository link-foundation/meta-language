//! Original solver terms through shared default/guard interface selections.
use super::super::Signature;
use super::{Inference, Result, Span, Term, Type};
use crate::translation::named_default_callable_rules as rules;
impl Inference {
    pub(in crate::translation::javascript::infer) fn named_default_reference_term(
        &self,
        signature: &Signature,
        span: Option<Span>,
    ) -> Result<Term> {
        let plan = rules::plan_named_callable_default_reference(
            "JavaScript",
            &signature.invocation_defaults,
            &signature.guarded_inputs,
        );
        if plan.is_empty() {
            return Ok(Self::callable_term(&signature.params, &signature.ret));
        }
        if plan[0] < 0.0 {
            return Err(crate::translation::diagnostics::unsupported(
                "named callable default depth",
                "the actual parameter list exceeds its complete source interface budget",
                span,
            ));
        }
        let mut parameters = Vec::new();
        for (index, term) in signature.params.iter().enumerate() {
            let mode = plan[index + 1];
            let term = if mode >= 2.0 {
                Term::Known(Type::Int)
            } else {
                term.clone()
            };
            if mode == 0.0 || mode == 2.0 {
                parameters.push(term);
                continue;
            }
            let selected = self.resolve(&term);
            let kind = super::super::optional_values::kind(&selected);
            let original = match &selected {
                Term::Option { absence, .. } => absence.borrow().clone(),
                _ => String::new(),
            };
            let absence = rules::read_named_callable_default_reference_absence(kind, &original);
            if absence.is_empty() {
                return Err(crate::translation::diagnostics::unsupported(
                    "named callable default domain",
                    "an optional interface needs its complete actual absence policy",
                    span,
                ));
            }
            let element = match selected {
                Term::Option { element, .. } => *element,
                _ => term,
            };
            parameters.push(super::super::optional_values::optional(element, &absence));
        }
        Ok(Self::callable_term(&parameters, &signature.ret))
    }
}
