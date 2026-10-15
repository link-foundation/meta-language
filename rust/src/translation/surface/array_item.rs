//! Array-item behavior and the independent presence of its serialized flag.
use super::SExpr;
use serde::ser::SerializeStruct;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// An element of an array literal, or with `spread` the elements of an array.
///
/// Construct explicit flags with [`Self::new`]. Deserialization also retains
/// an omitted flag without treating an explicit null as omission.
#[derive(Clone, Debug)]
pub struct SArrayItem {
    pub spread: bool,
    pub value: SExpr,
    spread_present: bool,
}

impl SArrayItem {
    /// Construct an item whose Boolean spread flag is explicitly present.
    #[must_use]
    pub const fn new(value: SExpr, spread: bool) -> Self {
        Self {
            spread,
            value,
            spread_present: true,
        }
    }
}

// Wire presence does not change the existing Boolean/value equality. Cloning
// retains it for lossless serialization.
impl PartialEq for SArrayItem {
    fn eq(&self, other: &Self) -> bool {
        self.spread == other.spread && self.value == other.value
    }
}
impl Eq for SArrayItem {}

impl Serialize for SArrayItem {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let present = self.spread_present || self.spread;
        let mut item = serializer.serialize_struct("SArrayItem", 1 + usize::from(present))?;
        if present {
            item.serialize_field("spread", &self.spread)?;
        }
        item.serialize_field("value", &self.value)?;
        item.end()
    }
}

fn read_present_spread<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<bool>, D::Error> {
    bool::deserialize(deserializer).map(Some)
}

impl<'de> Deserialize<'de> for SArrayItem {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        struct Fields {
            #[serde(default, deserialize_with = "read_present_spread")]
            spread: Option<bool>,
            value: SExpr,
        }
        let fields = Fields::deserialize(deserializer)?;
        Ok(Self {
            spread: fields.spread.unwrap_or_default(),
            value: fields.value,
            spread_present: fields.spread.is_some(),
        })
    }
}
