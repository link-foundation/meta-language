//! Actual source-origin transport into complete datatype field positions.
use super::super::{Result, RustParser, SourceOrigin, Type};
use crate::translation::source_data_field_signatures::{
    RustConstructorFieldSignatures, RustDataTypeFieldSignatures,
};
use crate::translation::source_signatures::SourceOwner;
impl RustParser {
    pub(in super::super) fn capture_data_field_signatures<'a>(
        &self,
        name: &str,
        path: &[String],
        groups: impl IntoIterator<Item = (&'a str, &'a [SourceOrigin])>,
    ) -> Result<Option<Box<RustDataTypeFieldSignatures>>> {
        if !self.retain_source_type_owners {
            return Ok(None);
        }
        let mut unit = SourceOrigin::with_type(Type::Unit, &[], Vec::new());
        unit.source_owner = Some(SourceOwner {
            kind: "unit".into(),
            path: Vec::new(),
            arguments: Vec::new(),
        });
        let mut ctors = Vec::new();
        for (name, fields) in groups {
            let signature = self
                .capture_source_signature(fields, &unit, Vec::new(), false)?
                .expect("retained source owners select complete source positions");
            ctors.push(RustConstructorFieldSignatures {
                name: name.to_owned(),
                fields: signature.parameters,
            });
        }
        let mut data_type_path = path.to_vec();
        data_type_path.push(name.to_owned());
        Ok(Some(Box::new(RustDataTypeFieldSignatures {
            model: "Rust data type field signatures 1".into(),
            language: "Rust".into(),
            data_type_name: data_type_path.join("."),
            ctors,
        })))
    }
}
