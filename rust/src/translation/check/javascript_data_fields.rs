//! Actual nominal constructors/types into the shared projection decision and
//! existing exhaustive match/optional IR. No replacement property semantics.
use super::javascript_nominal_spreads::ConstructorGuard;
use super::{
    Case, Checker, Expr, Language, LitValue, Node, Pattern, Result, STRING, Span, Type, UNIT,
    unsupported,
};
use crate::translation::javascript_constructor_presence::{field_value_type, read_value};
use crate::translation::javascript_data_field_rules as rules;

fn value_type(field: &crate::translation::ir::Field) -> &Type {
    if field.optional {
        field_value_type(&field.ty).expect("checked field type")
    } else {
        &field.ty
    }
}

impl Checker {
    pub(super) fn javascript_data_field(
        &mut self,
        object: &Expr,
        field: &str,
        span: Option<Span>,
        guard: Option<&ConstructorGuard>,
        own: bool,
    ) -> Result<Option<Expr>> {
        self.project_javascript_data_field(object, field, span, false, guard, own)
    }

    pub(super) fn javascript_data_field_projection(
        &mut self,
        object: &Expr,
        field: &str,
        span: Option<Span>,
        preserve_presence: bool,
    ) -> Result<Option<Expr>> {
        self.project_javascript_data_field(object, field, span, preserve_presence, None, false)
    }

    #[allow(clippy::cast_precision_loss)] // actual in-memory field coordinates
    fn project_javascript_data_field(
        &mut self,
        object: &Expr,
        field: &str,
        span: Option<Span>,
        preserve_presence: bool,
        guard: Option<&ConstructorGuard>,
        own: bool,
    ) -> Result<Option<Expr>> {
        let Type::Data { name } = &object.ty else {
            return Ok(None);
        };
        if self.language != Language::JavaScript {
            return Ok(None);
        }
        let entry = self.data_decl(name)?.clone();
        let guard = guard.filter(|guard| {
            field != "$" && guard.owner == entry.full_name && !guard.names.is_empty()
        });
        let variants = entry
            .ctors
            .iter()
            .filter(|ctor| guard.is_none_or(|guard| guard.names.contains(&ctor.name)))
            .collect::<Vec<_>>();
        let positions = variants
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
        let types = variants
            .iter()
            .zip(&positions)
            .map(|(ctor, position)| {
                position.map_or_else(String::new, |position| {
                    value_type(&ctor.fields[position]).key()
                })
            })
            .collect::<Vec<_>>();
        let inherited=!own&&crate::translation::javascript_record_parameter_rules::read_java_script_record_formal_default_field_form(
            &crate::translation::javascript_record_parameter_rules::ml_string::Text::from_text("record"),false,false,
            &crate::translation::javascript_record_parameter_rules::ml_string::Text::from_text(field)).to_string()=="prototype";
        if !preserve_presence
            && inherited
            && variants.iter().zip(&positions).any(|(ctor, position)| {
                position.is_some_and(|position| ctor.fields[position].optional)
            })
        {
            return Err(unsupported(
                "datatype property",
                "an absent own field can expose an inherited property",
                span,
            ));
        }
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
                position.map(|position| {
                    let field = &variants[index].fields[position];
                    if preserve_presence {
                        field.ty.clone()
                    } else {
                        value_type(field).clone()
                    }
                })
            })
            .unwrap_or(UNIT);
        let ty = if mode == "tag" {
            STRING
        } else if optional {
            crate::translation::types::option(selected, "undefined")
        } else {
            selected
        };
        let product = (variants.len() != entry.ctors.len()).then(|| Type::Tuple {
            elements: vec![ty.clone()],
        });
        let matched_type = product.as_ref().map_or_else(
            || ty.clone(),
            |product| crate::translation::types::option(product.clone(), "undefined"),
        );
        let coordinates = variants
            .iter()
            .map(|ctor| ctor.name.as_str())
            .zip(positions)
            .collect::<std::collections::HashMap<_, _>>();
        let mut cases = Vec::new();
        for ctor in &entry.ctors {
            let coordinate = coordinates.get(ctor.name.as_str());
            let position = coordinate.copied().flatten();
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
            let value = if coordinate.is_none() {
                cases.push(Case {
                    pattern,
                    body: Expr::new(
                        Node::None {
                            absence: "undefined".to_owned(),
                        },
                        matched_type.clone(),
                    ),
                });
                continue;
            } else if mode == "tag" {
                Expr::lit(STRING, LitValue::Text(ctor.name.clone()))
            } else if let Some(position) = position {
                let field = &ctor.fields[position];
                let value = Expr::var(
                    binding.expect("the selected actual field has its binder"),
                    field.ty.clone(),
                );
                if field.optional && !preserve_presence {
                    read_value(value)
                } else {
                    value
                }
            } else {
                Expr::new(
                    Node::None {
                        absence: "undefined".to_owned(),
                    },
                    ty.clone(),
                )
            };
            let value = self.coerce(value, &ty, span)?;
            let body = if let Some(product) = &product {
                let tuple = Expr::new(
                    Node::Tuple {
                        items: vec![value],
                        frozen: false,
                        product_model: None,
                    },
                    product.clone(),
                );
                Expr::new(
                    Node::Some {
                        value: Box::new(tuple),
                    },
                    matched_type.clone(),
                )
            } else {
                value
            };
            cases.push(Case { pattern, body });
        }
        let value = Expr::new(
            Node::Match {
                scrutinee: Box::new(object.clone()),
                cases,
            },
            matched_type,
        );
        Ok(Some(if let Some(product) = product {
            let tuple = Expr::new(
                Node::UnwrapOption {
                    arg: Box::new(value),
                },
                product,
            );
            Expr::new(
                Node::TupleIndex {
                    object: Box::new(tuple),
                    index: 0,
                },
                ty,
            )
        } else {
            value
        }))
    }
}
