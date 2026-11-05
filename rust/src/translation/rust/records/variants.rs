//! Named enum fields retain source names and their actual constructor owner.
use super::super::{Result, RustParser, SField, SourceOrigin, Token, span, type_error};
use super::NamedRecord;
use crate::translation::rust_source_render_interfaces::plan_rust_record_fields;

impl RustParser {
    pub(in super::super) fn named_variant_fields(
        &mut self,
        path: &[String],
        name: &str,
        variant: &Token,
    ) -> Result<(Vec<SField>, Vec<SourceOrigin>)> {
        self.cursor.expect("{", Some("record variant"))?;
        let mut fields = Vec::new();
        let mut origins = Vec::new();
        let mut names = Vec::new();
        while !self.cursor.is("}") {
            self.attributes(None)?;
            let field = self.cursor.identifier(Some("record variant field"))?;
            self.cursor.expect(":", Some("record variant field"))?;
            names.push(field.value.clone());
            let ty = self.parse_type()?;
            fields.push(SField {
                name: Some(field.value),
                ty,
                rocq_type: None,
                span: None,
            });
            origins.push(self.origins.current_type.clone());
            if self.cursor.eat(",").is_none() {
                break;
            }
        }
        self.cursor.expect("}", Some("record variant"))?;
        if plan_rust_record_fields(&names, &names)
            .first()
            .is_some_and(|code| *code < 0.0)
        {
            return Err(type_error(
                "a Rust record declares a duplicate field",
                span(variant, self.cursor.peek()),
            ));
        }
        let constructor = path
            .iter()
            .map(String::as_str)
            .chain([name, variant.value.as_str()])
            .collect::<Vec<_>>()
            .join(".");
        self.named_records.insert(
            constructor.clone(),
            NamedRecord {
                owner: path
                    .iter()
                    .map(String::as_str)
                    .chain(std::iter::once(name))
                    .collect::<Vec<_>>()
                    .join("."),
                constructor,
                names,
                syntax: "named",
                update: false,
            },
        );
        Ok((fields, origins))
    }
}
