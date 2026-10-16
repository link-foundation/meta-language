//! Record syntax delegates to the complete shared rendering source.
use super::{Expr, JavaScriptEmitter, Result};
use crate::translation::ir::RecordEntry;
use crate::translation::lexer::json_string;
use crate::translation::record_emission_rules as rules;

impl JavaScriptEmitter<'_> {
    pub(super) fn record_value(&mut self, entries: &[RecordEntry]) -> Result<String> {
        let labels = entries
            .iter()
            .map(|entry| json_string(&entry.name))
            .collect::<Vec<_>>();
        let values = entries
            .iter()
            .map(|entry| self.expr(&entry.value))
            .collect::<Result<Vec<_>>>()?;
        Ok(rules::render_record_value("JavaScript", &labels, &values))
    }

    pub(super) fn record_field(&mut self, object: &Expr, field: &str) -> Result<String> {
        let object = self.expr(object)?;
        Ok(rules::render_record_field(
            "JavaScript",
            &object,
            &json_string(field),
            "",
        ))
    }
}
