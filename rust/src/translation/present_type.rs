//! Preserve omission while rejecting a present null at typed metadata edges.
use serde::{Deserialize, Deserializer};

use super::types::Type;

pub fn read_present_type<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<Type>, D::Error> {
    Type::deserialize(deserializer).map(Some)
}
