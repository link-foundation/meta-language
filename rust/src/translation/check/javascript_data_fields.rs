//! Actual nominal constructors/types into the shared projection decision and
//! existing exhaustive match/optional IR. No replacement property semantics.
use super::{
    Case, Checker, Expr, Language, LitValue, Node, Pattern, Result, STRING, Span, Type, UNIT,
    unsupported,
};
use crate::translation::javascript_data_field_rules as rules;

impl Checker {
    #[allow(clippy::cast_precision_loss)] // actual in-memory field coordinates
    pub(super) fn javascript_data_field(
        &mut self,
        object: &Expr,
        field: &str,
        span: Option<Span>,
    ) -> Result<Option<Expr>> {
        let Type::Data { name } = &object.ty else {
            return Ok(None);
        };
        if self.language != Language::JavaScript {
            return Ok(None);
        }
        let entry = self.data_decl(name)?.clone();
        let positions = entry
            .ctors
            .iter()
            .map(|ctor| {
                ctor.fields
                    .iter()
                    .position(|candidate| candidate.name == field)
            })
            .collect::<Vec<_>>();
        let coordinates = positions
            .iter()
            .map(|position| position.map_or(-1.0, |position| position as f64))
            .collect::<Vec<_>>();
        let types = entry
            .ctors
            .iter()
            .zip(&positions)
            .map(|(ctor, position)| {
                position.map_or_else(String::new, |position| ctor.fields[position].ty.key())
            })
            .collect::<Vec<_>>();
        let inherited=crate::translation::javascript_record_parameter_rules::read_java_script_record_formal_default_field_form(
            &crate::translation::javascript_record_parameter_rules::ml_string::Text::from_text("record"),false,false,
            &crate::translation::javascript_record_parameter_rules::ml_string::Text::from_text(field)).to_string()=="prototype";
        let (error, mode, optional) = rules::read_java_script_data_field_plan(
            "JavaScript",
            field,
            inherited,
            &coordinates,
            &types,
        );
        if !error.is_empty() {
            return Err(unsupported("datatype property", &error, span));
        }
        let mode = mode.to_string();
        if mode == "absent" {
            let ty = crate::translation::types::option(UNIT, "undefined");
            return Ok(Some(Expr::new(
                Node::Let {
                    name: self.fresh_name("ml_absent_data_property_receiver"),
                    value: Box::new(object.clone()),
                    body: Box::new(Expr::new(
                        Node::None {
                            absence: "undefined".to_owned(),
                        },
                        ty.clone(),
                    )),
                },
                ty,
            )));
        }
        let selected = positions
            .iter()
            .enumerate()
            .find_map(|(index, position)| {
                position.map(|position| entry.ctors[index].fields[position].ty.clone())
            })
            .unwrap_or(UNIT);
        let ty = if mode == "tag" {
            STRING
        } else if optional {
            crate::translation::types::option(selected, "undefined")
        } else {
            selected
        };
        let mut cases = Vec::new();
        for (ctor, position) in entry.ctors.iter().zip(positions) {
            let binding = if position.is_some() && mode != "tag" {
                Some(self.fresh_name("ml_data_property_value"))
            } else {
                None
            };
            let pattern = Pattern::Ctor {
                data: entry.full_name.clone(),
                ctor: ctor.name.clone(),
                binds: (0..ctor.fields.len())
                    .map(|at| {
                        if Some(at) == position {
                            binding.clone()
                        } else {
                            None
                        }
                    })
                    .collect(),
            };
            let value = if mode == "tag" {
                Expr::lit(STRING, LitValue::Text(ctor.name.clone()))
            } else if let Some(position) = position {
                Expr::var(
                    binding.expect("the selected actual field has its binder"),
                    ctor.fields[position].ty.clone(),
                )
            } else {
                Expr::new(
                    Node::None {
                        absence: "undefined".to_owned(),
                    },
                    ty.clone(),
                )
            };
            cases.push(Case {
                pattern,
                body: self.coerce(value, &ty, span)?,
            });
        }
        Ok(Some(Expr::new(
            Node::Match {
                scrutinee: Box::new(object.clone()),
                cases,
            },
            ty,
        )))
    }
}
