//! Record syntax delegates to the complete shared rendering source.
use super::{Expr, Result, RustEmitter};
use crate::translation::ir::RecordEntry;
use crate::translation::record_emission_rules as rules;

impl RustEmitter<'_> {
    pub(super) fn record_value(&mut self, entries: &[RecordEntry]) -> Result<String> {
        let values = entries
            .iter()
            .map(|entry| self.expr(&entry.value))
            .collect::<Result<Vec<_>>>()?;
        Ok(rules::render_record_value("Rust", &[], &values))
    }

    pub(super) fn record_field(&mut self, object: &Expr, index: usize) -> Result<String> {
        let object = self.expr(object)?;
        Ok(rules::render_record_field(
            "Rust",
            &object,
            "",
            &index.to_string(),
        ))
    }
}
