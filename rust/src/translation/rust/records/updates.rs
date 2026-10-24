//! Source updates use ordered lets and one ordinary constructor match.
use super::super::{
    Result, RustParser, SExpr, SNode, SPattern, SPatternNode, SRow, SourceDeclaration, Span, Type,
    type_error, unsupported,
};
use super::NamedRecord;
use crate::translation::rust_record_rules::{
    accept_rust_record_update_owner, plan_rust_record_update_fields,
};

impl RustParser {
    pub(in super::super) fn record_update(
        &mut self,
        record: &NamedRecord,
        names: &[String],
        values: Vec<SExpr>,
        base: SExpr,
        path: &[String],
        range: Option<Span>,
    ) -> Result<SExpr> {
        let plan = plan_rust_record_update_fields(&record.names, names);
        if plan.first().is_some_and(|code| *code < 0.0) {
            return Err(type_error(
                "a Rust record update has duplicate or unknown fields",
                range,
            ));
        }
        let origin = self.origins.origin(&base, path);
        let owner = if let Some(Type::Named { path: names, .. }) = &origin.ty {
            self.origins
                .lookup(names, &origin.path)
                .filter(|entry| matches!(entry.declaration, SourceDeclaration::Data))
        } else {
            None
        };
        let admitted = accept_rust_record_update_owner(
            record.update,
            owner.is_some(),
            owner.as_ref().map_or("", |entry| entry.full_name.as_str()),
            &record.owner,
            f64::from(
                u32::try_from(origin.wrappers.len())
                    .expect("source indirection fits the shared array limit"),
            ),
            origin.diverging,
        );
        if !admitted
            && (!self.origins.collecting || origin.ty.is_some() || origin.proven || !record.update)
        {
            return Err(unsupported(
                "Rust record update owner",
                "an update needs the same owned source struct; borrowed and enum bases need another interpretation",
                range,
            ));
        }
        let count = values.len();
        let bindings = self.record_binding_names(count + 1 + record.names.len());
        let base_name = &bindings[count];
        let constructor_path = record
            .constructor
            .split('.')
            .map(str::to_owned)
            .collect::<Vec<_>>();
        let constructor = self
            .origins
            .lookup(&constructor_path, &[])
            .map(|entry| entry.declaration);
        let mut rooted = vec!["crate".to_owned()];
        rooted.extend(constructor_path);
        let positions = plan
            .iter()
            .skip(1)
            .map(|position| {
                if *position < 0.0 {
                    Ok(None)
                } else {
                    position
                        .to_string()
                        .parse::<usize>()
                        .map(Some)
                        .map_err(|_| type_error("invalid Rust record source position", range))
                }
            })
            .collect::<Result<Vec<_>>>()?;
        let args = positions
            .iter()
            .enumerate()
            .map(|(index, position)| {
                SExpr::new(
                    SNode::Name {
                        path: vec![bindings[position.unwrap_or(count + 1 + index)].clone()],
                    },
                    range,
                )
            })
            .collect::<Vec<_>>();
        let function = SExpr::new(
            SNode::Name {
                path: rooted.clone(),
            },
            range,
        );
        let body = if args.is_empty() {
            function
        } else {
            SExpr::new(
                SNode::App {
                    func: Box::new(function),
                    args,
                },
                range,
            )
        };
        let pattern = SPattern {
            node: SPatternNode::Ctor {
                path: rooted,
                args: positions
                    .iter()
                    .enumerate()
                    .map(|(index, position)| SPattern {
                        node: if position.is_none() {
                            SPatternNode::Bind {
                                name: bindings[count + 1 + index].clone(),
                            }
                        } else {
                            SPatternNode::Wild
                        },
                        span: None,
                    })
                    .collect(),
            },
            span: None,
        };
        let body = SExpr::new(
            SNode::Match {
                scrutinees: vec![SExpr::new(
                    SNode::Name {
                        path: vec![base_name.clone()],
                    },
                    range,
                )],
                rows: vec![SRow {
                    patterns: vec![pattern],
                    body,
                    span: None,
                }],
            },
            range,
        );
        let mut base_type = vec!["crate".to_owned()];
        base_type.extend(record.owner.split('.').map(str::to_owned));
        let mut body = SExpr::new(
            SNode::Let {
                name: base_name.clone(),
                ty: Some(
                    Type::Named {
                        path: base_type,
                        span: None,
                    }
                    .into(),
                ),
                value: Box::new(base),
                body: Box::new(body),
            },
            range,
        );
        for (index, value) in values.into_iter().enumerate().rev() {
            let field = if let Some(SourceDeclaration::Constructor { fields, .. }) = &constructor {
                positions
                    .iter()
                    .position(|position| *position == Some(index))
                    .and_then(|position| fields.get(position))
            } else {
                None
            };
            let ty = field.and_then(|field| {
                field
                    .ty
                    .as_ref()
                    .map(|ty| self.record_source_type(ty, &field.path).into())
            });
            if let Some(field) = field {
                self.origins
                    .argument(&self.origins.origin(&value, path), field, range)?;
            }
            body = SExpr::new(
                SNode::Let {
                    name: bindings[index].clone(),
                    ty,
                    value: Box::new(value),
                    body: Box::new(body),
                },
                range,
            );
        }
        if let Some(SourceDeclaration::Constructor { result, .. }) = constructor {
            self.origins.mark(&body, result);
        }
        Ok(body)
    }
}
