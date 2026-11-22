//! Actual inference owners and shared selected field positions.
use super::{Env, Inference, Rc, RecordTerm, RefCell, Result, Span, Term};
use crate::translation::object_rest_rules as rules;
use crate::translation::types::Type;
pub(super) struct RestFields {
    object: Term,
    excluded: Vec<String>,
    result: Term,
    pub(super) place: Option<Span>,
}
impl Inference {
    pub(super) fn object_rest_term(
        &mut self,
        args: &[crate::translation::surface::SArrayItem],
        env: &Env,
        place: Option<Span>,
    ) -> Result<Term> {
        let (object, excluded) =
            crate::translation::object_rest::operands(args, place, "JavaScript")?;
        let object = self.expr(&object, env)?;
        let result = self.fresh(false);
        self.object_rest_fields.push(RestFields {
            object,
            excluded,
            result: result.clone(),
            place,
        });
        Ok(result)
    }
    pub(super) fn solve_object_rest_fields(&mut self) -> Result<bool> {
        let mut waiting = Vec::new();
        let mut progress = false;
        for constraint in std::mem::take(&mut self.object_rest_fields) {
            let mut owner = self.resolve(&constraint.object);
            if let Term::Known(Type::Data { name }) = &owner {
                let target = self.resolve(&constraint.result);
                if matches!(target, Term::Var(_)) {
                    waiting.push(constraint);
                    continue;
                }
                let constructors = self
                    .data
                    .iter()
                    .filter(|(owner, _)| owner == name)
                    .flat_map(|(_, constructors)| constructors)
                    .map(|constructor| {
                        constructor
                            .fields
                            .iter()
                            .map(|field| (field.name.as_deref(), field.optional))
                            .collect()
                    });
                if !matches!(&target, Term::Known(Type::Data { name: target }) if target == name)
                    || !crate::translation::object_rest::accept_nominal(
                        constructors,
                        &constraint.excluded,
                    )
                {
                    return Err(crate::translation::diagnostics::unsupported(
                        "nominal object rest receiver",
                        "the actual tag and every required constructor field must be retained in the same declared owner",
                        constraint.place,
                    ));
                }
                progress = true;
                continue;
            }
            if let Term::Known(ty @ Type::Record { .. }) = &owner {
                owner = self.declared(Some(ty));
            }
            let (kind, mut fields, open, mut optional_fields) = match &owner {
                Term::Var(_) => ("var", Vec::new(), false, Default::default()),
                Term::Record(owner) => {
                    let owner = owner.borrow();
                    (
                        "record",
                        owner.fields.clone(),
                        owner.open,
                        owner.optional_fields.clone(),
                    )
                }
                _ => ("other", Vec::new(), false, Default::default()),
            };
            let form = rules::read_java_script_object_rest_owner_form(
                &rules::ml_string::Text::from_text(kind),
                open,
            )
            .to_string();
            if form == "wait" {
                waiting.push(constraint);
                continue;
            }
            if form != "copy" {
                return Err(crate::translation::diagnostics::unsupported(
                    "object rest receiver",
                    "every actual enumerable own field and its source order must be represented",
                    constraint.place,
                ));
            }
            fields.sort_by(|left, right| left.0.encode_utf16().cmp(right.0.encode_utf16()));
            let names = fields
                .iter()
                .map(|field| field.0.clone())
                .collect::<Vec<_>>();
            let positions =
                crate::translation::object_rest::positions(&names, &constraint.excluded);
            let fields: Vec<_> = positions
                .iter()
                .map(|position| fields[*position].clone())
                .collect();
            optional_fields.retain(|name| fields.iter().any(|field| field.0 == *name));
            self.unify(
                &constraint.result,
                &Term::Record(Rc::new(RefCell::new(RecordTerm {
                    source_class_owner: None,
                    fields,
                    optional_fields,
                    open: false,
                }))),
                constraint.place,
            )?;
            progress = true;
        }
        self.object_rest_fields = waiting;
        Ok(progress)
    }
}
