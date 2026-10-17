//! Presence and descriptor transport for an internal checked record projection.
use serde::{Deserialize, Deserializer};
use serde_json::Value;

use super::{ConstructorPrototypeDefinition, SRecordEntry, Type};
use crate::translation::record_value_rules::read_record_projection_error;

type RecordProjectionFields = (
    Vec<SRecordEntry>,
    Option<Value>,
    Option<Type>,
    Option<ConstructorPrototypeDefinition>,
);

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct RecordFields {
    entries: Vec<SRecordEntry>,
    #[serde(default, deserialize_with = "read_present_record_projection")]
    record_projection: Option<Value>,
    #[serde(rename = "type", default)]
    ty: Option<Type>,
    #[serde(default)]
    prototype_definition: Option<ConstructorPrototypeDefinition>,
}

// Preserve every present value, including null, so shared admission cannot
// confuse an invalid marker with the ordinary raw-source record path.
fn read_present_record_projection<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<Value>, D::Error> {
    Value::deserialize(deserializer).map(Some)
}

// Serde variant hooks return every field as an ordered tuple (FIELDS):
// fn<'de, D>(D) -> Result<FIELDS, D::Error> where D: Deserializer<'de>.
// Primary contract: https://serde.rs/variant-attrs.html
// See the deserialize_with attribute on that page; named fields use this order.
// Only marked records inspect original descriptors before typed conversion;
// unmarked records retain the existing public SRecordEntry defaults and shape.
pub(super) fn read_record_projection<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<RecordProjectionFields, D::Error> {
    let value = Value::deserialize(deserializer)?;
    if let Some(projection) = value.get("recordProjection") {
        let entries = value
            .get("entries")
            .and_then(Value::as_array)
            .ok_or_else(|| {
                <D::Error as serde::de::Error>::custom(
                    "checked record projection needs an ordered field array",
                )
            })?;
        let names = entries
            .iter()
            .map(|entry| {
                entry["name"]
                    .as_str()
                    .unwrap_or_default()
                    .encode_utf16()
                    .map(f64::from)
                    .collect()
            })
            .collect::<Vec<Vec<f64>>>();
        let admitted = entries
            .iter()
            .map(|entry| {
                entry.as_object().is_some_and(|fields| {
                    fields.get("name").is_some_and(Value::is_string)
                        && fields.get("spread") == Some(&Value::Bool(false))
                        && fields.get("value").is_some_and(Value::is_object)
                        && fields
                            .keys()
                            .all(|field| field == "name" || field == "spread" || field == "value")
                })
            })
            .collect::<Vec<_>>();
        let refusal = read_record_projection_error(
            projection.as_str().unwrap_or_default(),
            &names,
            &admitted,
        );
        if !refusal.is_empty() {
            return Err(<D::Error as serde::de::Error>::custom(refusal));
        }
    }
    let fields: RecordFields =
        serde_json::from_value(value).map_err(<D::Error as serde::de::Error>::custom)?;
    Ok((
        fields.entries,
        fields.record_projection,
        fields.ty,
        fields.prototype_definition,
    ))
}
