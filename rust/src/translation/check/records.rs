//! Typed construction driven by the shared record entry and field plans.

use super::{Checker, Env, Expr, Node, Result, SExpr, Span, Type, type_error, unsupported};
use crate::translation::frontend_rules::ml_array::number_index;
use crate::translation::ir::RecordEntry;
use crate::translation::javascript_constructor_presence as presence;
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

pub(super) fn widen_javascript_record_presence(
    value: &Expr,
    ty: &Type,
    span: Option<Span>,
) -> Result<Option<Expr>> {
    let (
        Type::Record { fields: source, source_class_owner: None, ordered: false, open: false, reference: false },
        Type::Record { fields: target, source_class_owner: None, ordered: false, open: false, reference: false },
    ) = (&value.ty, ty) else {
        return Ok(None);
    };
    if !target.iter().any(|field| field.optional)
        || source.iter().any(|field| !target.iter().any(|wanted| wanted.name == field.name))
        || target.iter().any(|field| !field.optional && !source.iter().any(|actual| actual.name == field.name))
    {
        return Ok(None);
    }
    let name = "ml_record_presence_source".to_owned();
    let owner = Expr::var(name.clone(), value.ty.clone());
    let mut entries = Vec::with_capacity(target.len());
    for field in target {
        let contents = if let Some((index, actual)) = source.iter().enumerate().find(|(_, actual)| actual.name == field.name) {
            let read = Expr::new(Node::RecordField { object: Box::new(owner.clone()), field: actual.name.clone(), index }, actual.ty.clone());
            if field.optional == actual.optional && field.ty.same(&actual.ty) {
                read
            } else if field.optional && !actual.optional {
                let payload_type = presence::field_value_type(&field.ty).expect("checked optional field presence");
                let payload = super::coerce(read, payload_type, super::Language::JavaScript, span, None)?;
                presence::own_value(payload, &field.ty)
            } else {
                return Ok(None);
            }
        } else {
            Expr::new(Node::None { absence: "undefined".to_owned() }, field.ty.clone())
        };
        entries.push(RecordEntry { name: field.name.clone(), value: contents });
    }
    let body = Expr::new(Node::Record { entries, order: None }, ty.clone());
    Ok(Some(Expr::new(Node::Let { name, value: Box::new(value.clone()), body: Box::new(body) }, ty.clone())))
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
        let nominal = match expected {
            Some(Type::Data { .. }) => expected.cloned(),
            _ => declared
                .map(|ty| self.resolve_type(Some(ty), path, span))
                .transpose()?,
        };
        if let Some(selected) = nominal.as_ref() {
            if let Some(value) = self.nominal_spread_literal(
                entries,
                selected,
                projection.is_some() || property_order.is_some(),
                env,
                path,
                span,
            )? {
                return Ok(value);
            }
        }
        let expected = expected.or_else(|| {
            declared.filter(|ty| matches!(ty, Type::Record { fields, ordered, source_class_owner, .. }
                if *ordered || source_class_owner.is_some() || fields.iter().any(|field| field.optional)))
        });
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
                source_class_owner: match expected{Some(Type::Record{source_class_owner,..})=>source_class_owner.clone(),_=>None},
                fields: values
                    .iter()
                    .map(|entry| TypeField {
                        optional: expected_fields.iter().any(|field| field.name == entry.name && field.optional),
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
        let mut optional = Vec::new();
        let mut possibly_absent = Vec::new();
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
                    optional.push(field.optional);
                    possibly_absent.push(field.optional);
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
                let field = number_index(wanted).and_then(|index| expected_fields.get(index));
                let wanted_type = field.map(|field| {
                    if field.optional {
                        presence::field_value_type(&field.ty).expect("checked field presence")
                    } else {
                        &field.ty
                    }
                });
                let mut value = self.expr(&entry.value, env, path, wanted_type, false)?;
                let owns_optional = field.is_some_and(|field| field.optional);
                if let Some(field) = field.filter(|field| field.optional) {
                    let wanted_type = wanted_type.expect("an optional field has its value type");
                    value = presence::own_value(self.coerce(value, wanted_type, span)?, &field.ty);
                }
                optional.push(owns_optional);
                possibly_absent.push(false);
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
        let indices = rules::read_record_field_order(&field_names)
            .into_iter()
            .map(|index| position(index, values.len(), span))
            .collect::<Result<Vec<_>>>()?;
        let mut optional_names = std::collections::HashSet::new();
        let mut selected = Vec::new();
        for index in indices {
            let mut entry = values[index].clone();
            if optional[index] {
                optional_names.insert(entry.name.clone());
            }
            if possibly_absent[index] {
                let ty = entry.value.ty.clone();
                let read = presence::field_value_type(&ty).expect("checked field presence");
                let Type::Option { element, .. } = &ty else { unreachable!("checked field presence") };
                for earlier in (0..index).rev().filter(|earlier| values[*earlier].name == entry.name) {
                    let right = if optional[earlier] {
                        self.coerce(values[earlier].value.clone(), &ty, span)?
                    } else {
                        let value = self.coerce(values[earlier].value.clone(), read, span)?;
                        presence::own_value(value, &ty)
                    };
                    let name = self.fresh_name("ml_optional_spread_field");
                    let chosen = Expr::var(&name, ty.clone());
                    let condition = Self::javascript_optional_presence(chosen.clone(), element, span)?;
                    let body = Expr::new(Node::If {
                        cond: Box::new(condition), then: Box::new(chosen), otherwise: Box::new(right),
                    }, ty.clone());
                    entry.value = Expr::new(Node::Let {
                        name, value: Box::new(entry.value), body: Box::new(body),
                    }, ty.clone());
                    if !possibly_absent[earlier] { break; }
                }
            }
            selected.push(entry);
        }
        for field in expected_fields.iter().filter(|field| field.optional) {
            if let Some(entry) = selected.iter_mut().find(|entry| entry.name == field.name) {
                if optional_names.insert(field.name.clone()) {
                    let wanted = presence::field_value_type(&field.ty).expect("checked field presence");
                    let value = self.coerce(entry.value.clone(), wanted, span)?;
                    entry.value = presence::own_value(value, &field.ty);
                }
            } else {
                optional_names.insert(field.name.clone());
                selected.push(RecordEntry {
                    name: field.name.clone(),
                    value: Expr::new(Node::None { absence: "undefined".to_owned() }, field.ty.clone()),
                });
            }
        }
        if !optional_names.is_empty() {
            selected.sort_by(|left, right| left.name.encode_utf16().cmp(right.name.encode_utf16()));
        }
        let ordered = self.ordered_record_source
            || property_order.is_some()
            || matches!(expected, Some(Type::Record { ordered: true, .. }));
        if ordered && !optional_names.is_empty() {
            return Err(unsupported("ordered optional record fields",
                "the actual dynamic own-field permutation must be represented", span));
        }
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
            source_class_owner: match expected{Some(Type::Record{source_class_owner,..})=>source_class_owner.clone(),_=>None},
            fields: selected
                .iter()
                .map(|entry| TypeField {
                    optional: optional_names.contains(&entry.name),
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
        if self.checked_projection && matches!(value.ty, Type::Record { .. }) {
            return Ok(Self::record_field(value, field, span)?.expect("checked record field"));
        }
        if let Some(field) =
            self.javascript_nullish_field((&value, object), field, env, path, expected, span)?
        {
            return Ok(field);
        }
        if let Some(field) = self.javascript_data_field(&value, field, span)? {
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
            let optional = matches!(&value.ty, Type::Record { fields, .. }
                if fields.iter().any(|entry| entry.name == field && entry.optional));
            return Ok(if optional { presence::read_value(record) } else { record });
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
