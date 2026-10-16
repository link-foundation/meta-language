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

impl Checker {
    pub(super) fn record_literal(
        &mut self,
        entries: &[SRecordEntry],
        projection: Option<&serde_json::Value>,
        env: &Env,
        path: &[String],
        expected: Option<&Type>,
        span: Option<Span>,
    ) -> Result<Expr> {
        let expected_fields = match expected {
            Some(Type::Record { fields, .. }) => fields.as_slice(),
            _ => &[],
        };
        let expected_names = names(expected_fields);
        let mut values = Vec::new();
        let mut bindings = Vec::new();
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
            };
            return Ok(Expr::new(Node::Record { entries: values }, ty));
        }
        for selected in rules::read_record_value_order(number(entries.len()), false) {
            let entry = &entries[position(selected, entries.len(), span)?];
            if entry.spread {
                let value = self.expr(&entry.value, env, path, None, false)?;
                let Type::Record { fields, .. } = &value.ty else {
                    return Err(type_error("record spread needs a record value", span));
                };
                let fields = fields.clone();
                let name = self.fresh_name("ml_spread");
                let object = Expr::var(&name, value.ty.clone());
                bindings.push((name, value));
                for (index, field) in fields.into_iter().enumerate() {
                    values.push(RecordEntry {
                        name: field.name.clone(),
                        value: Expr::new(
                            Node::RecordField {
                                object: Box::new(object.clone()),
                                field: field.name,
                                index,
                            },
                            field.ty,
                        ),
                    });
                }
            } else {
                let name = entry
                    .name
                    .as_deref()
                    .ok_or_else(|| type_error("record field has no name", span))?;
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
        };
        let mut body = Expr::new(Node::Record { entries: selected }, ty.clone());
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
        span: Option<Span>,
    ) -> Result<Expr> {
        let value = self.expr(object, env, path, None, false)?;
        let value = Self::reference_value(value);
        if let Some(record) = Self::record_field(value.clone(), field, span)? {
            return Ok(record);
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
