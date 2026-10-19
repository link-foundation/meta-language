//! Presence-preserving host serialization for the shared raw metadata policy.
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;

use crate::translation::surface_raw_metadata_rules::accept_surface_raw_metadata_form;

/// One present raw Boolean or null; the containing `Option` records absence.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(transparent)]
pub struct SurfaceRawMetadata(Value);

impl SurfaceRawMetadata {
    /// Preserve an explicitly present null instead of omitting the field.
    #[must_use]
    pub const fn null() -> Self {
        Self(Value::Null)
    }

    /// Read the Boolean value without merging explicit null with absence.
    #[must_use]
    pub fn as_boolean(&self) -> Option<bool> {
        self.0.as_bool()
    }
}

impl From<bool> for SurfaceRawMetadata {
    fn from(value: bool) -> Self {
        Self(Value::Bool(value))
    }
}

impl<'de> Deserialize<'de> for SurfaceRawMetadata {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = Value::deserialize(deserializer)?;
        let form = match &value {
            Value::Bool(_) => "boolean",
            Value::Null => "null",
            _ => "other",
        };
        if !accept_surface_raw_metadata_form(form) {
            return Err(<D::Error as serde::de::Error>::custom(
                "a present raw surface metadata value is a Boolean or null",
            ));
        }
        Ok(Self(value))
    }
}

pub(super) fn read_present_surface_raw_metadata<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<SurfaceRawMetadata>, D::Error> {
    SurfaceRawMetadata::deserialize(deserializer).map(Some)
}
