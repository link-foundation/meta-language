//! Host tree/column transport; the shared source plan determines tuple positions.
use super::{Checker, Column, HashSet, NPat, Result, Row, SExpr, SNode, Span, Type};

fn collect_source_strings(value: &serde_json::Value, names: &mut HashSet<String>) {
    match value {
        serde_json::Value::String(value) if value.starts_with("ml_tuple_field") => {
            names.insert(value.clone());
        }
        serde_json::Value::Array(items) => {
            for item in items {
                collect_source_strings(item, names);
            }
        }
        serde_json::Value::Object(items) => {
            for item in items.values() {
                collect_source_strings(item, names);
            }
        }
        _ => {}
    }
}

fn collect_pattern_names(pattern: &NPat, names: &mut HashSet<String>) {
    let name = match pattern {
        NPat::Bind(name) | NPat::Data { ctor: name, .. } => Some(name),
        _ => None,
    };
    if let Some(name) = name.filter(|name| name.starts_with("ml_tuple_field")) {
        names.insert(name.clone());
    }
    if let NPat::Lit { value, .. } = pattern {
        collect_source_strings(
            &serde_json::to_value(value).expect("a surface literal serializes"),
            names,
        );
    }
    for item in pattern.args() {
        collect_pattern_names(item, names);
    }
}

impl Checker {
    pub(super) fn compile_tuple_pattern_column(
        &mut self,
        columns: &[Column],
        rows: &[Row],
        position: usize,
        span: Option<Span>,
    ) -> Result<SExpr> {
        let column = &columns[position];
        let Type::Tuple { elements } = &column.ty else {
            unreachable!("tuple normalization retains its actual type")
        };
        let mut reserved = columns
            .iter()
            .map(|column| column.name.clone())
            .collect::<HashSet<_>>();
        for row in rows {
            for pattern in &row.patterns {
                collect_pattern_names(pattern, &mut reserved);
            }
            for (name, variable) in &row.binds {
                reserved.insert(name.clone());
                reserved.insert(variable.clone());
            }
            collect_source_strings(
                &serde_json::to_value(&row.body).expect("a surface body serializes"),
                &mut reserved,
            );
        }
        let fields = elements
            .iter()
            .map(|ty| {
                let name = loop {
                    let name = self.fresh_name("ml_tuple_field");
                    if reserved.insert(name.clone()) {
                        break name;
                    }
                };
                Column {
                    name,
                    ty: ty.clone(),
                }
            })
            .collect::<Vec<_>>();
        let mut expanded = Vec::new();
        for row in rows {
            let pattern = &row.patterns[position];
            let mut patterns = if let NPat::Tuple { args } = pattern {
                args.clone()
            } else {
                vec![NPat::Wild; fields.len()]
            };
            patterns.extend(
                row.patterns
                    .iter()
                    .enumerate()
                    .filter(|(index, _)| *index != position)
                    .map(|(_, item)| item.clone()),
            );
            let mut binds = row.binds.clone();
            if let NPat::Bind(name) = pattern {
                binds.push((name.clone(), column.name.clone()));
            }
            expanded.push(Row {
                patterns,
                body: row.body.clone(),
                binds,
            });
        }
        let mut next_columns = fields.clone();
        next_columns.extend(
            columns
                .iter()
                .enumerate()
                .filter(|(index, _)| *index != position)
                .map(|(_, item)| item.clone()),
        );
        let mut body = self.compile_rows(&next_columns, expanded, span)?;
        for (index, field) in fields.into_iter().enumerate().rev() {
            body = SExpr::new(
                SNode::Let {
                    name: field.name,
                    ty: None,
                    value: Box::new(SExpr::new(
                        SNode::TupleIndex {
                            object: Box::new(SExpr::name(&column.name)),
                            index,
                        },
                        None,
                    )),
                    body: Box::new(body),
                },
                span,
            );
        }
        Ok(body)
    }
}
