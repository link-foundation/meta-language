//! Actual field constraints moved intact from the common inference host.
use super::{FLOAT, FieldOf, Inference, Result, Term, Type};

impl Inference {
    pub(super) fn field_step(&mut self, constraint: &FieldOf) -> Result<bool> {
        let resolved = self.resolve(&constraint.object);
        let kind = match &resolved {
            Term::Known(ty) => ty.kind(),
            Term::Option { .. } => "option",
            _ => "value",
        };
        let element = match &resolved {
            Term::Option { element, .. }
                if matches!(self.resolve(element), Term::Known(Type::Unit)) =>
            {
                "unit"
            }
            _ => "",
        };
        let form =
            crate::translation::javascript_nullish_field_rules::read_java_script_nullish_field_form(
                kind, element,
            );
        if form == "absent" {
            return Ok(true);
        }
        if form == "guarded" {
            let Term::Option { element, .. } = resolved else {
                unreachable!("the shared field form retains its actual optional term")
            };
            let next = FieldOf {
                object: *element,
                field: constraint.field.clone(),
                result: constraint.result.clone(),
                place: constraint.place,
            };
            self.fields.push(next);
            return Ok(true);
        }
        if let Term::Record(owner) = self.resolve(&constraint.object) {
            return self.record_field_constraint(
                &owner,
                &constraint.field,
                &constraint.result,
                constraint.place,
            );
        }
        if constraint.field == "size"
            && matches!(
                self.resolve(&constraint.object),
                Term::Map { .. } | Term::Set { .. }
            )
        {
            self.unify(&constraint.result, &Term::Known(FLOAT), constraint.place)?;
            return Ok(true);
        }
        let name = match self.resolve(&constraint.object) {
            Term::Known(Type::Data { name }) => name,
            Term::Array(_) | Term::Tuple(_) | Term::Known(Type::String)
                if constraint.field == "length" =>
            {
                self.unify(&constraint.result, &Term::Known(FLOAT), constraint.place)?;
                return Ok(true);
            }
            other => {
                let owners: Vec<&String> = self
                    .data
                    .iter()
                    .filter(|(_, ctors)| {
                        ctors.iter().any(|ctor| {
                            ctor.fields.iter().any(|field| {
                                field.name.as_deref() == Some(constraint.field.as_str())
                            })
                        })
                    })
                    .map(|(name, _)| name)
                    .collect();
                if !matches!(other, Term::Var(_)) || owners.len() != 1 {
                    return Ok(false);
                }
                let owner = Term::Known(Type::Data {
                    name: owners[0].clone(),
                });
                self.unify(&constraint.object, &owner, constraint.place)?;
                return self.field_step(constraint);
            }
        };
        let declared = self
            .data
            .iter()
            .find(|(data, _)| *data == name)
            .and_then(|(_, ctors)| {
                ctors.iter().find_map(|ctor| {
                    ctor.fields
                        .iter()
                        .position(|field| field.name.as_deref() == Some(constraint.field.as_str()))
                        .map(|index| (ctor.name.clone(), index))
                })
            });
        if let Some((ctor, index)) = declared {
            let term = self.field(&name, &ctor, index);
            self.unify(&constraint.result, &term, constraint.place)?;
        }
        Ok(true)
    }
}
