//! The value of a present, nullable surface binding type annotation.
use super::Type;
use serde::de::{self, DeserializeSeed, IntoDeserializer, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};

/// A present binding annotation, containing either null or an actual type.
///
/// `SNode::Let` stores this in an `Option`: `None` omits the field, while
/// `Some(Self::null())` preserves an explicitly present null.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(transparent)]
pub struct SurfaceBindingTypeAnnotation(Option<Type>);

impl<'de> Deserialize<'de> for SurfaceBindingTypeAnnotation {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_option(BindingTypeAnnotationVisitor)
    }
}

struct BindingTypeAnnotationVisitor;

impl<'de> Visitor<'de> for BindingTypeAnnotationVisitor {
    type Value = SurfaceBindingTypeAnnotation;

    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("null or a type object with a string kind")
    }

    fn visit_none<E: de::Error>(self) -> Result<Self::Value, E> {
        Ok(SurfaceBindingTypeAnnotation::null())
    }

    fn visit_unit<E: de::Error>(self) -> Result<Self::Value, E> {
        self.visit_none()
    }

    fn visit_some<D: Deserializer<'de>>(self, deserializer: D) -> Result<Self::Value, D::Error> {
        deserializer.deserialize_map(self)
    }

    fn visit_map<A: MapAccess<'de>>(self, map: A) -> Result<Self::Value, A::Error> {
        Type::deserialize(de::value::MapAccessDeserializer::new(
            BindingTypeAnnotationMap { map, kind: false },
        ))
        .map(SurfaceBindingTypeAnnotation::from)
    }
}

// Forward every original key and value seed, including duplicates. Only the
// kind tag needs an actual string rather than a derived enum's numeric ordinal.
struct BindingTypeAnnotationMap<A> {
    map: A,
    kind: bool,
}

impl<'de, A: MapAccess<'de>> MapAccess<'de> for BindingTypeAnnotationMap<A> {
    type Error = A::Error;

    fn next_key_seed<K: DeserializeSeed<'de>>(
        &mut self,
        seed: K,
    ) -> Result<Option<K::Value>, Self::Error> {
        let Some(key) = self.map.next_key::<String>()? else {
            return Ok(None);
        };
        self.kind = key == "kind";
        seed.deserialize(key.into_deserializer()).map(Some)
    }

    fn next_value_seed<V: DeserializeSeed<'de>>(
        &mut self,
        seed: V,
    ) -> Result<V::Value, Self::Error> {
        if self.kind {
            seed.deserialize(self.map.next_value::<String>()?.into_deserializer())
        } else {
            self.map.next_value_seed(seed)
        }
    }
}

impl SurfaceBindingTypeAnnotation {
    /// Preserve an explicitly present null annotation.
    #[must_use]
    pub const fn null() -> Self {
        Self(None)
    }

    /// Borrow the declared type, if the present annotation is not null.
    #[must_use]
    pub const fn as_ref(&self) -> Option<&Type> {
        self.0.as_ref()
    }

    /// Read the declared type without changing the consumer's optionality.
    #[must_use]
    pub fn into_option(self) -> Option<Type> {
        self.0
    }
}

impl From<Type> for SurfaceBindingTypeAnnotation {
    fn from(value: Type) -> Self {
        Self(Some(value))
    }
}

pub(super) fn read_present_binding_type_annotation<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<SurfaceBindingTypeAnnotation>, D::Error> {
    SurfaceBindingTypeAnnotation::deserialize(deserializer).map(Some)
}
