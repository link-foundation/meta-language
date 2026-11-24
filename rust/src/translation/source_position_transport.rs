//! Presence-preserving owned source decorator transport, without proof logic.
use serde::{Deserialize,Deserializer};
use super::source_signatures::RustSourceExpressionPosition;

pub(crate) fn read_present_box<'de, D: Deserializer<'de>, T: Deserialize<'de>>(
    deserializer: D,
) -> Result<Option<Box<T>>, D::Error> {
    Box::<T>::deserialize(deserializer).map(Some)
}

pub(crate) fn read_present_source_position<'de,D:Deserializer<'de>>(deserializer:D)->Result<Option<Box<RustSourceExpressionPosition>>,D::Error>{
 read_present_box(deserializer)
}
