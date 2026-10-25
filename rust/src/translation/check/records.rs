//! Typed construction driven by the shared record entry and field plans.

use super::{Checker, Env, Expr, Node, Result, SExpr, Span, Type, type_error, unsupported};
use crate::translation::frontend_rules::ml_array::number_index;
use crate::translation::ir::RecordEntry;
use crate::translation::record_value_rules as rules;
use crate::translation::surface::SRecordEntry;
use crate::translation::types::TypeField;

fn number(value: usize) -> f64 {
    serde_json::Number::from(value)
        .as_f64()
        .expect("an allocated record entry position has a Number representation")
}

fn names(fields: &[TypeField]) -> Vec<Vec<f64>> {
    fields
        .iter()
        .map(|field| field.name.encode_utf16().map(f64::from).collect())
        .collect()
}

fn position(value: f64, length: usize, span: Option<Span>) -> Result<usize> {
    number_index(value)
        .filter(|index| *index < length)
        .ok_or_else(|| type_error("record entries need an exact ordered input length", span))
}

enum OrderChunk {
    Field(String),
    Spread(Expr),
}
fn merge_source_order(
    selected: &[RecordEntry],
    chunks: Vec<OrderChunk>,
    span: Option<Span>,
) -> Result<Expr> {
    let positions = crate::translation::types::array(Type::Float);
    let target = crate::translation::ordered_record_values::property_name_rows(selected);
    let mut items = Vec::new();
    for chunk in chunks {
        items.push(match chunk {
            OrderChunk::Field(name) => {
                let index = selected
                    .iter()
                    .position(|field| field.name == name)
                    .ok_or_else(|| {
                        type_error("an ordered record needs every actual own field", span)
                    })?;
                Expr::new(
                    Node::Array {
                        items: vec![super::text_lit(Type::Float, index.to_string())],
                    },
                    positions.clone(),
                )
            }
            OrderChunk::Spread(object) => {
                let Type::Record {
                    fields,
                    ordered: true,
                    open: false,
                    ..
                } = &object.ty
                else {
                    return Err(unsupported(
                        "ordered record spread",
                        "the actual source own-field order must be retained and rebased",
                        span,
                    ));
                };
                let source = crate::translation::ordered_record_values::property_names(
                    fields.iter().map(|field| field.name.as_str()),
                );
                let actual = Expr::new(
                    Node::Method {
                        object: Box::new(object),
                        op: "readJavaScriptRecordPropertyOrder".to_owned(),
                        args: Vec::new(),
                    },
                    positions.clone(),
                );
                Expr::new(
                    Node::Math {
                        op: "rebaseJavaScriptRecordPropertyOrder".to_owned(),
                        args: vec![target.clone(), source, actual],
                    },
                    positions.clone(),
                )
            }
        });
    }
    let chunks = Expr::new(
        Node::Array { items },
        crate::translation::types::array(positions.clone()),
    );
    Ok(Expr::new(
        Node::Math {
            op: "mergeJavaScriptRecordPropertyOrder".to_owned(),
            args: vec![target, chunks],
        },
        positions,
    ))
}

impl Checker {
    pub(super) fn record_literal(
        &mut self,
        entries: &[SRecordEntry],
        transport: (Option<&serde_json::Value>, Option<&SExpr>, Option<&Type>),
        env: &Env,
        path: &[String],
        expected: Option<&Type>,
        span: Option<Span>,
    ) -> Result<Expr> {
        let (projection, property_order, declared) = transport;
        let expected = expected
            .or_else(|| declared.filter(|ty| matches!(ty, Type::Record { ordered: true, .. })));
        let expected_fields = match expected {
            Some(Type::Record { fields, .. }) => fields.as_slice(),
            _ => &[],
        };
        let expected_names = names(expected_fields);
        let mut values = Vec::new();
        let mut bindings = Vec::new();
        let mut order_chunks = Vec::new();
        if let Some(projection) = projection {
            let field_names = entries
                .iter()
                .map(|entry| {
                    entry
                        .name
                        .as_deref()
                        .unwrap_or_default()
                        .encode_utf16()
                        .map(f64::from)
                        .collect()
                })
                .collect::<Vec<Vec<f64>>>();
            // JSON descriptor admission precedes serde in the linked projection
            // adapter. The typed surface still supplies actual name/spread facts.
            let admitted = entries
                .iter()
                .map(|entry| entry.name.is_some() && !entry.spread)
                .collect::<Vec<_>>();
            let refusal = rules::read_record_projection_error(
                projection.as_str().unwrap_or_default(),
                &field_names,
                &admitted,
            );
            if !refusal.is_empty() {
                return Err(type_error(refusal, span));
            }
            for entry in entries {
                let name = entry
                    .name
                    .as_ref()
                    .expect("shared checked record entry admission");
                let wanted = rules::read_record_field_position(
                    &expected_names,
                    &name.encode_utf16().map(f64::from).collect::<Vec<_>>(),
                );
                let expected = number_index(wanted)
                    .and_then(|index| expected_fields.get(index))
                    .map(|field| &field.ty);
                let value = self.expr(&entry.value, env, path, expected, false)?;
                values.push(RecordEntry {
                    name: name.clone(),
                    value,
                });
            }
            let ordered = property_order.is_some();
            if let Some(Type::Record {
                ordered: expected, ..
            }) = declared
                && *expected != ordered
            {
                return Err(type_error(
                    if *expected {
                        "an ordered record projection needs its actual own-field order"
                    } else {
                        "a structural record projection cannot contain an own-field order"
                    },
                    span,
                ));
            }
            if let Some(Type::Record {
                ordered: expected, ..
            }) = expected
                && *expected != ordered
            {
                return Err(type_error(
                    if *expected {
                        "an ordered record projection needs its actual own-field order"
                    } else {
                        "a structural record projection cannot contain an own-field order"
                    },
                    span,
                ));
            }
            let order = property_order
                .map(|source| {
                    let ty = crate::translation::types::array(Type::Float);
                    let value = self.expr(source, env, path, Some(&ty), false)?;
                    self.coerce(Self::reference_value(value), &ty, span)
                        .map(Box::new)
                })
                .transpose()?;
            let ty = Type::Record {
                fields: values
                    .iter()
                    .map(|entry| TypeField {
                        name: entry.name.clone(),
                        ty: entry.value.ty.clone(),
                    })
                    .collect(),
                open: false,
                reference: false,
                ordered,
            };
            return Ok(Expr::new(
                Node::Record {
                    entries: values,
                    order,
                },
                ty,
            ));
        }
        for selected in rules::read_record_value_order(number(entries.len()), false) {
            let entry = &entries[position(selected, entries.len(), span)?];
            if entry.spread {
                let value = self.expr(&entry.value, env, path, None, false)?;
                let value = Self::reference_value(value);
                let Type::Record { fields, .. } = &value.ty else {
                    return Err(type_error("record spread needs a record value", span));
                };
                let fields = fields.clone();
                let name = self.fresh_name("ml_spread");
                let object = Expr::var(&name, value.ty.clone());
                order_chunks.push(OrderChunk::Spread(object.clone()));
                bindings.push((name, value));
                for (index, field) in fields.into_iter().enumerate() {
                    let name = self.fresh_name("ml_spread_field");
                    let value = Expr::new(
                        Node::RecordField {
                            object: Box::new(object.clone()),
                            field: field.name.clone(),
                            index,
                        },
                        field.ty.clone(),
                    );
                    bindings.push((name.clone(), value));
                    values.push(RecordEntry {
                        name: field.name.clone(),
                        value: Expr::var(name, field.ty),
                    });
                }
            } else {
                let name = entry
                    .name
                    .as_deref()
                    .ok_or_else(|| type_error("record field has no name", span))?;
                order_chunks.push(OrderChunk::Field(name.to_owned()));
                let wanted = rules::read_record_field_position(
                    &expected_names,
                    &name.encode_utf16().map(f64::from).collect::<Vec<_>>(),
                );
                let expected = number_index(wanted)
                    .and_then(|index| expected_fields.get(index))
                    .map(|field| &field.ty);
                let value = self.expr(&entry.value, env, path, expected, false)?;
                let binding = self.fresh_name("ml_field");
                let reference = Expr::var(&binding, value.ty.clone());
                bindings.push((binding, value));
                values.push(RecordEntry {
                    name: name.to_owned(),
                    value: reference,
                });
            }
        }
        let field_names = values
            .iter()
            .map(|entry| entry.name.encode_utf16().map(f64::from).collect())
            .collect::<Vec<Vec<f64>>>();
        let selected = rules::read_record_field_order(&field_names)
            .into_iter()
            .map(|index| position(index, values.len(), span).map(|index| values[index].clone()))
            .collect::<Result<Vec<_>>>()?;
        let ordered = self.ordered_record_source
            || property_order.is_some()
            || matches!(expected, Some(Type::Record { ordered: true, .. }));
        let order = if let Some(source) = property_order {
            let ty = crate::translation::types::array(Type::Float);
            let value = self.expr(source, env, path, Some(&ty), false)?;
            Some(Box::new(self.coerce(
                Self::reference_value(value),
                &ty,
                span,
            )?))
        } else if ordered {
            if entries.iter().any(|entry| entry.spread) {
                Some(Box::new(merge_source_order(&selected, order_chunks, span)?))
            } else {
                let insertion = rules::read_record_field_insertion_order(&field_names);
                let items = insertion
                    .into_iter()
                    .map(|index| {
                        let index = position(index, values.len(), span)?;
                        let at = selected
                            .iter()
                            .position(|field| field.name == values[index].name)
                            .ok_or_else(|| {
                                type_error("an ordered record needs every actual own field", span)
                            })?;
                        Ok(super::text_lit(Type::Float, at.to_string()))
                    })
                    .collect::<Result<Vec<_>>>()?;
                Some(Box::new(Expr::new(
                    Node::Array { items },
                    crate::translation::types::array(Type::Float),
                )))
            }
        } else {
            None
        };
        let ty = Type::Record {
            fields: selected
                .iter()
                .map(|entry| TypeField {
                    name: entry.name.clone(),
                    ty: entry.value.ty.clone(),
                })
                .collect(),
            open: false,
            reference: false,
            ordered,
        };
        let mut body = Expr::new(
            Node::Record {
                entries: selected,
                order,
            },
            ty.clone(),
        );
        for index in rules::read_record_value_order(number(bindings.len()), true) {
            let (name, value) = bindings[position(index, bindings.len(), span)?].clone();
            body = Expr::new(
                Node::Let {
                    name,
                    value: Box::new(value),
                    body: Box::new(body),
                },
                ty.clone(),
            );
        }
        Ok(body)
    }

    /// Consume a checked receiver once; field positions come from the same
    /// UTF16-name planner as literal layout and JavaScript checking.
    pub(super) fn record_field(
        object: Expr,
        field: &str,
        span: Option<Span>,
    ) -> Result<Option<Expr>> {
        let Type::Record { fields, .. } = &object.ty else {
            return Ok(None);
        };
        let index = rules::read_record_field_position(
            &names(fields),
            &field.encode_utf16().map(f64::from).collect::<Vec<_>>(),
        );
        let Some(index) = number_index(index).filter(|index| *index < fields.len()) else {
            return Err(type_error(format!("record has no field {field}"), span));
        };
        let ty = fields[index].ty.clone();
        Ok(Some(Expr::new(
            Node::RecordField {
                object: Box::new(object),
                field: field.to_owned(),
                index,
            },
            ty,
        )))
    }

    pub(super) fn record_source_field(
        &mut self,
        object: &SExpr,
        field: &str,
        env: &Env,
        path: &[String],
        expected: Option<&Type>,
        span: Option<Span>,
    ) -> Result<Expr> {
        let value = self.expr(object, env, path, None, false)?;
        let value = Self::reference_value(value);
        if let Some(field) =
            self.javascript_nullish_field((&value, object), field, env, path, expected, span)?
        {
            return Ok(field);
        }
        if let Some(absent) = self.javascript_absent_record_property(&value, field, expected, span)
        {
            return Ok(absent);
        }
        Self::checked_named_property(value, field, span)
    }

    pub(super) fn checked_named_property(
        value: Expr,
        field: &str,
        span: Option<Span>,
    ) -> Result<Expr> {
        if let Some(record) = Self::record_field(value.clone(), field, span)? {
            return Ok(record);
        }
        if field == "size" && matches!(value.ty, Type::Map { .. } | Type::Set { .. }) {
            return Ok(Expr::new(
                Node::CollectionSize {
                    object: Box::new(value),
                },
                crate::translation::types::FLOAT,
            ));
        }
        if field == "length"
            && let Some(length) = Self::length_field(value)
        {
            return Ok(length);
        }
        Err(unsupported(
            "field access",
            &format!("field {field} is only portable inside a constructor match"),
            span,
        ))
    }
}
