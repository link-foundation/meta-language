//! Presence-preserving owned source decorator transport, without proof logic.
use serde::{Deserialize,Deserializer};
use super::source_signatures::RustSourceExpressionPosition;

pub(crate) fn read_present_source_position<'de,D:Deserializer<'de>>(deserializer:D)->Result<Option<Box<RustSourceExpressionPosition>>,D::Error>{
 Box::<RustSourceExpressionPosition>::deserialize(deserializer).map(Some)
}
