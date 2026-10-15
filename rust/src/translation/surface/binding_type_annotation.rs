//! The value of a present, nullable surface binding type annotation.
use super::Type;
use serde::{Deserialize, Deserializer, Serialize};

/// A present binding annotation, containing either null or an actual type.
///
/// `SNode::Let` stores this in an `Option`: `None` omits the field, while
/// `Some(Self::null())` preserves an explicitly present null.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SurfaceBindingTypeAnnotation(Option<Type>);

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
