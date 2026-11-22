//! Record syntax delegates to the complete shared rendering source.
use super::{Expr, JavaScriptEmitter, Result};
use crate::translation::ir::RecordEntry;
use crate::translation::lexer::json_string;
use crate::translation::record_emission_rules as rules;
use crate::translation::types::Type;

impl JavaScriptEmitter<'_> {
    pub(super) fn record_value(
        &mut self,
        entries: &[RecordEntry],
        order: Option<&Expr>,
        ty: &Type,
    ) -> Result<String> {
        let Type::Record { ordered, fields, .. } = ty else {
            return Err(crate::translation::diagnostics::type_error("a record value needs its actual layout", None));
        };
        if *ordered != order.is_some() {
            return Err(crate::translation::diagnostics::type_error(
                "an ordered record needs its actual own-field order and matching value type",
                None,
            ));
        }
        let labels = entries
            .iter()
            .map(|entry| json_string(&entry.name))
            .collect::<Vec<_>>();
        let values = entries
            .iter()
            .map(|entry| self.expr(&entry.value))
            .collect::<Result<Vec<_>>>()?;
        let Some(order) = order else {
            if fields.iter().any(|field| field.optional) {
                let optional = fields.iter().map(|field| field.optional).collect::<Vec<_>>();
                return Ok(rules::render_java_script_optional_record_value(&labels, &values, &optional));
            }
            return Ok(rules::render_record_value("JavaScript", &labels, &values));
        };
        let order = self.expr(order)?;
        Ok(
            crate::translation::ordered_record_layout_rules::render_ordered_record_value(
                "JavaScript",
                &labels,
                &values,
                &order,
            ),
        )
    }

    pub(super) fn record_field(&mut self, object: &Expr, field: &str) -> Result<String> {
        let optional = matches!(&object.ty, Type::Record { fields, .. }
            if fields.iter().any(|entry| entry.name == field && entry.optional));
        let object = self.expr(object)?;
        let label = json_string(field);
        if optional {
            return Ok(rules::render_java_script_optional_record_field(&object, &label));
        }
        Ok(rules::render_record_field(
            "JavaScript",
            &object,
            &label,
            "",
        ))
    }
}
