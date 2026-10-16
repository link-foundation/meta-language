//! Record syntax delegates to the complete shared rendering source.
use super::{Expr, Result, RocqEmitter, Type};
use crate::translation::diagnostics::type_error;
use crate::translation::ir::RecordEntry;
use crate::translation::record_emission_rules as rules;

fn number(value: usize) -> f64 {
    serde_json::Number::from(value)
        .as_f64()
        .expect("an allocated record field position has a Number representation")
}

impl RocqEmitter<'_> {
    pub(super) fn record_value(&mut self, entries: &[RecordEntry]) -> Result<String> {
        let values = entries
            .iter()
            .map(|entry| self.expr(&entry.value))
            .collect::<Result<Vec<_>>>()?;
        Ok(rules::render_record_value("Rocq", &[], &values))
    }

    pub(super) fn record_field(&mut self, object: &Expr, index: usize) -> Result<String> {
        let Type::Record { fields, .. } = &object.ty else {
            return Err(type_error(
                "record projection requires a record value",
                object.span,
            ));
        };
        let count = fields.len();
        let object = self.expr(object)?;
        Ok(rules::render_record_product_field(
            "Rocq",
            &object,
            number(index),
            number(count),
        ))
    }
}
