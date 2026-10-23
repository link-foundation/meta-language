//! Native option and declared decorator host views; algorithms stay shared.
use super::fail;
use crate::SelfTranslationOptions;
use serde_json::Value;

pub(super) fn apply(target: &mut SelfTranslationOptions, value: &Value) -> Result<(), String> {
    let values = value
        .as_object()
        .ok_or_else(|| fail("translation options must be a record"))?;
    for (name, value) in values {
        match name.as_str() {
            "utf16Strings" => {
                target.utf16_strings = value
                    .as_bool()
                    .ok_or_else(|| fail("utf16Strings must be Boolean"))?
            }
            "moduleDirectory" => {
                target.module_directory = serde_json::from_value(value.clone())
                    .map_err(|_| fail("invalid module directory"))?
            }
            "rustSharedArrayHandleSourceModel" => {
                target.rust_shared_array_handle_source_model = value
                    .as_str()
                    .ok_or_else(|| fail("invalid Rust source model"))?
                    .to_owned()
            }
            "decorators" => target.decorators = crate::decorator_data::read(value)?,
            // The project owns these derived contexts after applying records.
            "imports" | "semanticTranslation" | "retainDiagnostics" => {}
            "referenceSemantics" => {
                target.reference_semantics = value
                    .as_bool()
                    .ok_or_else(|| fail("referenceSemantics must be Boolean"))?;
            }
            _ => return Err(fail(&format!("unknown translation option {name}"))),
        }
    }
    Ok(())
}
