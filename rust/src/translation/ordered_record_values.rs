//! Mechanical literal transport for actual canonical record field names.
use super::ir::{Expr, LitValue, Node, RecordEntry};
use super::types::{FLOAT, array};
pub(crate) fn property_name_rows(entries: &[RecordEntry]) -> Expr {
    property_names(entries.iter().map(|entry| entry.name.as_str()))
}
pub(crate) fn property_names<'a>(names: impl IntoIterator<Item = &'a str>) -> Expr {
    let items = names
        .into_iter()
        .map(|name| {
            Expr::new(
                Node::Array {
                    items: name
                        .encode_utf16()
                        .map(|unit| Expr::lit(FLOAT, LitValue::Text(unit.to_string())))
                        .collect(),
                },
                array(FLOAT),
            )
        })
        .collect();
    Expr::new(Node::Array { items }, array(array(FLOAT)))
}

pub(crate) fn order_read(
    target: &str,
    expression: &Expr,
    rendered: &str,
) -> super::diagnostics::Result<String> {
    let error = || {
        super::diagnostics::type_error(
            "own-field order projection needs one actual owned ordered record",
            expression.span,
        )
    };
    let Node::Method { object, op, args } = &expression.node else {
        return Err(error());
    };
    let super::types::Type::Record {
        fields,
        ordered,
        reference,
        ..
    } = &object.ty
    else {
        return Err(error());
    };
    if !super::javascript_enumeration_rules::accept_linked_ordered_record_order_read(
        "method", op, "record", *ordered, *reference,
    ) || !args.is_empty()
    {
        return Err(error());
    }
    let labels = fields
        .iter()
        .map(|field| super::lexer::json_string(&field.name))
        .collect::<Vec<_>>();
    Ok(
        super::ordered_record_layout_rules::render_ordered_record_observed_order(
            target, rendered, &labels,
        ),
    )
}
