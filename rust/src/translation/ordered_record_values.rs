//! Mechanical literal transport for actual canonical record field names.
use super::ir::{Expr, LitValue, Node, RecordEntry};
use super::types::{FLOAT, array};
pub(crate) fn property_name_rows(entries: &[RecordEntry]) -> Expr {
    let items = entries
        .iter()
        .map(|entry| {
            Expr::new(
                Node::Array {
                    items: entry
                        .name
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
