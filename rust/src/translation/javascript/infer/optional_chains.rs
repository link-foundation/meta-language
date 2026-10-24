//! Infer the receiver's present term before its complete lexical continuation.
use super::{Env, Inference, Result, Span, Term, UNIT, type_error};
use crate::translation::surface::SArrayItem;
impl Inference {
    pub(super) fn optional_chain_term(
        &mut self,
        args: &[SArrayItem],
        env: &Env,
        place: Option<Span>,
    ) -> Result<Term> {
        if args.len() != 1 {
            return Err(type_error(
                "an optional chain needs its actual receiver, lexical binding and complete continuation",
                place,
            ));
        }
        let crate::translation::surface::SNode::Let {
            name,
            ty: None,
            value: source,
            body,
        } = &args[0].value.node
        else {
            return Err(type_error(
                "an optional chain needs its actual receiver, lexical binding and complete continuation",
                place,
            ));
        };
        if name.is_empty() {
            return Err(type_error(
                "an optional chain needs its actual receiver, lexical binding and complete continuation",
                place,
            ));
        }
        let receiver = self.expr(source, env)?;
        let resolved = self.resolve(&receiver);
        let kind = match &resolved {
            Term::Known(ty) => ty.kind(),
            Term::Option { .. } => "option",
            Term::Var(_) => "var",
            _ => "value",
        };
        let element = match &resolved {
            Term::Option { element, .. }
                if matches!(
                    self.resolve(element),
                    Term::Known(crate::translation::types::Type::Unit)
                ) =>
            {
                "unit"
            }
            _ => "",
        };
        let form=crate::translation::javascript_optional_chain_rules::read_java_script_optional_chain_inference_form(kind,element);
        if form == "absent" {
            return Ok(Term::Option {
                element: Box::new(Term::Known(UNIT)),
                absence: "undefined".to_owned(),
            });
        }
        let present = match &resolved {
            Term::Option { element, .. } => element.as_ref().clone(),
            Term::Var(_) => self.fresh(false),
            _ => receiver.clone(),
        };
        if form == "unknown" {
            self.unify(
                &receiver,
                &Term::Option {
                    element: Box::new(present.clone()),
                    absence: "both".to_owned(),
                },
                place,
            )?;
        }
        let mut scope = env.clone();
        scope.insert(name.to_owned(), present);
        let result = self.expr(body, &scope)?;
        Ok(if form != "direct" {
            self.join(
                result,
                Term::Option {
                    element: Box::new(Term::Known(UNIT)),
                    absence: "undefined".to_owned(),
                },
                place,
            )
        } else {
            result
        })
    }
}
