//! Actual normalized pattern nodes into shared alias-column selection plans.
use super::{Column, NPat, Result, Row, Span, type_error};
use crate::translation::frontend_rules::ml_array::number_index;
use crate::translation::pattern_alias_rules as rules;
pub(super) fn project_pattern_alias_row(
    row: Row,
    columns: &[Column],
    span: Option<Span>,
) -> Result<Row> {
    if !row
        .patterns
        .iter()
        .any(|pattern| matches!(pattern, NPat::Alias { .. }))
    {
        return Ok(row);
    }
    let mut nodes = Vec::new();
    let mut kinds = Vec::new();
    let mut names = Vec::new();
    let mut children = Vec::new();
    let mut roots = Vec::new();
    let count = |value: usize| {
        serde_json::Number::from(value)
            .as_f64()
            .expect("actual pattern node index is finite")
    };
    for root in &row.patterns {
        roots.push(count(nodes.len()));
        let mut node = root;
        loop {
            nodes.push(node);
            if let NPat::Alias {
                name,
                pattern: inner,
            } = node
            {
                kinds.push("alias".to_owned());
                names.push(name.clone());
                children.push(count(nodes.len()));
                node = inner;
            } else {
                kinds.push("pattern".to_owned());
                names.push(String::new());
                children.push(-1.0);
                break;
            }
        }
    }
    let bound_names = row
        .binds
        .iter()
        .map(|(name, _)| name.clone())
        .collect::<Vec<_>>();
    let plan = rules::plan_pattern_alias_bindings(&kinds, &names, &children, &roots, &bound_names);
    if plan.first() != Some(&0.0)
        || plan.get(1).copied().and_then(number_index) != Some(columns.len())
    {
        return Err(type_error(
            "a pattern alias needs complete distinct source bindings and column roots",
            span,
        ));
    }
    let patterns = plan[2..2 + columns.len()]
        .iter()
        .map(|index| (*nodes[number_index(*index).expect("shared selected pattern index")]).clone())
        .collect();
    let mut binds = row.binds.clone();
    for pair in plan[2 + columns.len()..].chunks_exact(2) {
        let column = number_index(pair[0]).expect("shared actual column index");
        let node = number_index(pair[1]).expect("shared actual alias node index");
        binds.push((names[node].clone(), columns[column].name.clone()));
    }
    Ok(Row {
        patterns,
        binds,
        ..row
    })
}
