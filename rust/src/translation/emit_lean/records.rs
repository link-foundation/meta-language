//! Record syntax delegates to the complete shared rendering source.
use super::{Expr, LeanEmitter, Result, Type, type_error};
use crate::translation::ir::RecordEntry;
use crate::translation::record_emission_rules as rules;

fn number(value: usize) -> f64 {
    serde_json::Number::from(value)
        .as_f64()
        .expect("an allocated record field position has a Number representation")
}

impl LeanEmitter<'_> {
    pub(super) fn record_value(
        &mut self,
        entries: &[RecordEntry],
        order: Option<&Expr>,
        ordered: bool,
        depth: usize,
    ) -> Result<String> {
        if ordered != order.is_some() {
            return Err(crate::translation::diagnostics::type_error(
                "an ordered record needs its actual own-field order and matching value type",
                None,
            ));
        }
        let values = entries
            .iter()
            .map(|entry| self.expr(&entry.value, depth))
            .collect::<Result<Vec<_>>>()?;
        let Some(order) = order else {
            return Ok(rules::render_record_value("Lean", &[], &values));
        };
        let order = self.expr(order, depth)?;
        Ok(
            crate::translation::ordered_record_layout_rules::render_ordered_record_value(
                "Lean",
                &[],
                &values,
                &order,
            ),
        )
    }

    pub(super) fn record_field(
        &mut self,
        object: &Expr,
        index: usize,
        depth: usize,
    ) -> Result<String> {
        let Type::Record { fields, .. } = &object.ty else {
            return Err(type_error(
                "record projection requires a record value",
                object.span,
            ));
        };
        let count = fields.len();
        let ordered = matches!(
            &object.ty,
            crate::translation::types::Type::Record { ordered: true, .. }
        );
        let object = self.expr(object, depth)?;
        let object = if ordered {
            crate::translation::ordered_record_layout_rules::render_ordered_record_payload(
                "Lean", &object,
            )
        } else {
            object
        };
        Ok(rules::render_record_product_field(
            "Lean",
            &object,
            number(index),
            number(count),
        ))
    }
}
