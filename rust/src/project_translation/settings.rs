//! Native option and declared decorator host views; algorithms stay shared.
use super::fail;
use crate::{Decorator, DecoratorAction, DecoratorLevel, DecoratorSet, SelfTranslationOptions};
use serde_json::Value;

fn decorators(value: &Value) -> Result<DecoratorSet, String> {
    if let Some(text) = value.as_str() {
        return DecoratorSet::from_lino(text).map_err(|error| fail(&error.to_string()));
    }
    let mut result = Vec::new();
    for record in value
        .as_array()
        .ok_or_else(|| fail("decorators must be linked records or Links Notation"))?
    {
        let field = |name: &str| {
            record[name]
                .as_str()
                .ok_or_else(|| fail("invalid decorator record"))
        };
        let id = field("id")?;
        let level = DecoratorLevel::parse(field("level")?)
            .ok_or_else(|| fail("invalid decorator level"))?;
        let order = record
            .get("order")
            .map_or(Some(0), Value::as_i64)
            .ok_or_else(|| fail("invalid decorator order"))?;
        let mut conditions = Vec::new();
        for condition in record["when"]
            .as_array()
            .ok_or_else(|| fail("invalid decorator conditions"))?
        {
            let pair = condition
                .as_array()
                .filter(|values| values.len() == 2)
                .ok_or_else(|| fail("invalid decorator condition"))?;
            conditions.push((
                pair[0]
                    .as_str()
                    .ok_or_else(|| fail("invalid decorator condition"))?
                    .to_owned(),
                pair[1]
                    .as_str()
                    .ok_or_else(|| fail("invalid decorator condition"))?
                    .to_owned(),
            ));
        }
        let mut actions = Vec::new();
        for action in record["actions"]
            .as_array()
            .ok_or_else(|| fail("invalid decorator actions"))?
        {
            let field = |name: &str| {
                action[name]
                    .as_str()
                    .ok_or_else(|| fail("invalid decorator action"))
            };
            actions.push(match field("op")? {
                "drop" => DecoratorAction::Drop,
                "set" => DecoratorAction::Set {
                    field: field("field")?.to_owned(),
                    value: field("value")?.to_owned(),
                },
                "replace" => DecoratorAction::Replace {
                    field: field("field")?.to_owned(),
                    from: field("from")?.to_owned(),
                    to: field("to")?.to_owned(),
                },
                _ => return Err(fail("unknown decorator action")),
            });
        }
        result.push(
            Decorator::new(id, level, order, conditions, actions)
                .map_err(|error| fail(&error.to_string()))?,
        );
    }
    DecoratorSet::new(result).map_err(|error| fail(&error.to_string()))
}

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
            "decorators" => target.decorators = decorators(value)?,
            // The project owns these derived contexts after applying records.
            "imports" | "semanticTranslation" | "retainDiagnostics" => {}
            "referenceSemantics" => {
                return Err(fail(
                    "the native project driver requires the actual supported source reference interpretation",
                ));
            }
            _ => return Err(fail(&format!("unknown translation option {name}"))),
        }
    }
    Ok(())
}
