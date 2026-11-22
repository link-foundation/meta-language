//! Actual source exclusion names, field identities and intervals.
use super::Span;
use super::diagnostics::{Result, type_error};
use super::surface::{SArrayItem, SExpr, SNode};
pub(super) fn operands(
    args: &[SArrayItem],
    span: Option<Span>,
    language: &str,
) -> Result<(SExpr, Vec<String>)> {
    let error = || {
        type_error(
            "object rest needs its actual receiver and exact original excluded property names",
            span,
        )
    };
    if language != "JavaScript" || args.len() != 2 || args.iter().any(|argument| argument.spread) {
        return Err(error());
    }
    let SNode::Array { items, .. } = &args[1].value.node else {
        return Err(error());
    };
    let mut names = Vec::new();
    for item in items {
        let SNode::Str { value } = &item.value.node else {
            return Err(error());
        };
        if item.spread {
            return Err(error());
        }
        names.push(value.clone());
    }
    Ok((args[0].value.clone(), names))
}
pub(super) fn positions(names: &[String], excluded: &[String]) -> Vec<usize> {
    let rows = |values: &[String]| {
        values
            .iter()
            .map(|value| value.encode_utf16().map(f64::from).collect::<Vec<_>>())
            .collect::<Vec<_>>()
    };
    super::object_rest_rules::select_java_script_object_rest_fields(&rows(names), &rows(excluded))
        .iter()
        .map(|position| {
            super::frontend_rules::ml_array::number_index(*position)
                .expect("shared object-rest plan selects actual fields")
        })
        .collect()
}

pub(super) fn accept_nominal<'a>(
    constructors: impl Iterator<Item = Vec<(Option<&'a str>, bool)>>,
    excluded: &[String],
) -> bool {
    if excluded.iter().any(|name| name == "$") {
        return false;
    }
    let statuses = constructors
        .map(|fields| {
            excluded
                .iter()
                .map(|name| {
                    match fields
                        .iter()
                        .find(|(field, _)| *field == Some(name.as_str()))
                    {
                        None => -1.0,
                        Some((_, true)) => 1.0,
                        Some((_, false)) => 0.0,
                    }
                })
                .collect()
        })
        .collect::<Vec<Vec<f64>>>();
    super::object_rest_rules::accept_java_script_nominal_object_rest(&statuses)
}

pub(super) fn source_read(object: SExpr, excluded: &[(String, Span)], span: Span) -> SExpr {
    let items = excluded
        .iter()
        .map(|(name, origin)| {
            SArrayItem::new(
                SExpr::new(
                    SNode::Str {
                        value: name.clone(),
                    },
                    Some(*origin),
                ),
                false,
            )
        })
        .collect();
    SExpr::new(
        SNode::Math {
            op: super::object_rest_rules::read_java_script_object_rest_operation().to_string(),
            name: String::new(),
            args: vec![
                SArrayItem::new(object, false),
                SArrayItem::new(
                    SExpr::new(
                        SNode::Array {
                            items,
                            element: None,
                        },
                        Some(span),
                    ),
                    false,
                ),
            ],
        },
        Some(span),
    )
}
