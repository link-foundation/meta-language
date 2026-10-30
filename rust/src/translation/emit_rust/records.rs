//! Record syntax delegates to the complete shared rendering source.
use super::{Expr, Result, RustEmitter};
use crate::translation::ir::RecordEntry;
use crate::translation::record_emission_rules as rules;

impl RustEmitter<'_> {
    pub(super) fn record_value(
        &mut self,
        entries: &[RecordEntry],
        order: Option<&Expr>,
        ordered: bool,
    ) -> Result<String> {
        if ordered != order.is_some() {
            return Err(crate::translation::diagnostics::type_error(
                "an ordered record needs its actual own-field order and matching value type",
                None,
            ));
        }
        let values = entries
            .iter()
            .map(|entry| self.expr(&entry.value))
            .collect::<Result<Vec<_>>>()?;
        let Some(order) = order else {
            return Ok(rules::render_record_value("Rust", &[], &values));
        };
        let order = self.expr(order)?;
        Ok(
            crate::translation::ordered_record_layout_rules::render_ordered_record_value(
                "Rust",
                &[],
                &values,
                &order,
            ),
        )
    }

    pub(super) fn record_field(&mut self, object: &Expr, index: usize) -> Result<String> {
        let ordered = matches!(
            &object.ty,
            crate::translation::types::Type::Record { ordered: true, .. }
        );
        let object = self.expr(object)?;
        let object = if ordered {
            crate::translation::ordered_record_layout_rules::render_ordered_record_payload(
                "Rust", &object,
            )
        } else {
            object
        };
        Ok(rules::render_record_field(
            "Rust",
            &object,
            "",
            &index.to_string(),
        ))
    }
}
