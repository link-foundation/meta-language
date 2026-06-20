//! Rust value and type-shape codec for [`LinkNetwork`].
//!
//! The codec uses explicit `ToLinks` / `FromLinks` traits plus the
//! `LinksObject` helper trait for user structs. This is intentionally smaller
//! than a proc-macro crate and avoids adding a serde serializer contract before
//! the links representation has stabilized. User types opt in with ordinary
//! trait impls, while primitives, `Option`, `Vec`, `BTreeMap`, and
//! `Rc<RefCell<T>>` get reusable implementations.

use std::any::{Any, TypeId};
use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;
use std::rc::Rc;
use std::str::FromStr;
use std::sync::Arc;

use crate::link_network::{Link, LinkId, LinkMetadata, LinkNetwork, LinkType};

const TYPE_TERM_PREFIX: &str = "rust::type::";
const LITERAL_TERM_PREFIX: &str = "rust::literal::";

/// High-level Rust type declaration kind represented as links.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum RustTypeKind {
    Primitive,
    Struct,
    Enum,
    Trait,
    Sequence,
    Option,
    Map,
}

impl RustTypeKind {
    const fn as_str(self) -> &'static str {
        match self {
            Self::Primitive => "primitive",
            Self::Struct => "struct",
            Self::Enum => "enum",
            Self::Trait => "trait",
            Self::Sequence => "sequence",
            Self::Option => "option",
            Self::Map => "map",
        }
    }
}

impl fmt::Display for RustTypeKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// One named field, variant, or trait item in a Rust type shape.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct RustFieldShape {
    name: String,
    type_name: String,
}

impl RustFieldShape {
    /// Creates a field shape with a Rust field name and display type name.
    #[must_use]
    pub fn new(name: impl Into<String>, type_name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            type_name: type_name.into(),
        }
    }

    /// Field, variant, or trait-item name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Display type name for the field value.
    #[must_use]
    pub fn type_name(&self) -> &str {
        &self.type_name
    }
}

/// Queryable Rust type declaration shape.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RustTypeShape {
    name: String,
    kind: RustTypeKind,
    fields: Vec<RustFieldShape>,
}

impl RustTypeShape {
    /// Creates a primitive type shape.
    #[must_use]
    pub fn primitive(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            kind: RustTypeKind::Primitive,
            fields: Vec::new(),
        }
    }

    /// Creates a struct type shape.
    #[must_use]
    pub fn structure<I>(name: impl Into<String>, fields: I) -> Self
    where
        I: IntoIterator<Item = RustFieldShape>,
    {
        Self {
            name: name.into(),
            kind: RustTypeKind::Struct,
            fields: fields.into_iter().collect(),
        }
    }

    /// Creates an enum type shape, using field entries as variants.
    #[must_use]
    pub fn enumeration<I>(name: impl Into<String>, variants: I) -> Self
    where
        I: IntoIterator<Item = RustFieldShape>,
    {
        Self {
            name: name.into(),
            kind: RustTypeKind::Enum,
            fields: variants.into_iter().collect(),
        }
    }

    /// Creates a trait type shape, using field entries as required items.
    #[must_use]
    pub fn trait_type<I>(name: impl Into<String>, items: I) -> Self
    where
        I: IntoIterator<Item = RustFieldShape>,
    {
        Self {
            name: name.into(),
            kind: RustTypeKind::Trait,
            fields: items.into_iter().collect(),
        }
    }

    /// Creates a sequence container type shape.
    #[must_use]
    pub fn sequence(name: impl Into<String>, element_type: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            kind: RustTypeKind::Sequence,
            fields: vec![RustFieldShape::new("item", element_type)],
        }
    }

    /// Creates an optional-value container type shape.
    #[must_use]
    pub fn option(name: impl Into<String>, value_type: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            kind: RustTypeKind::Option,
            fields: vec![
                RustFieldShape::new("Some", value_type),
                RustFieldShape::new("None", "()"),
            ],
        }
    }

    /// Creates a map container type shape.
    #[must_use]
    pub fn map(
        name: impl Into<String>,
        key_type: impl Into<String>,
        value_type: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            kind: RustTypeKind::Map,
            fields: vec![
                RustFieldShape::new("entry", "BTreeMapEntry"),
                RustFieldShape::new("key", key_type),
                RustFieldShape::new("value", value_type),
            ],
        }
    }

    /// Rust display name for this type.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Type declaration kind.
    #[must_use]
    pub const fn kind(&self) -> RustTypeKind {
        self.kind
    }

    /// Declared fields, variants, or trait items.
    #[must_use]
    pub fn fields(&self) -> &[RustFieldShape] {
        &self.fields
    }
}

/// Error returned while decoding Rust values from links.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LinksCodecError {
    /// A referenced link id is absent from the network.
    MissingLink(LinkId),
    /// The link exists but is not an object instance.
    ExpectedObject { link: LinkId },
    /// An object instance does not carry the expected self/type references.
    MalformedObject { object: LinkId, reason: String },
    /// An object type did not match the requested Rust type.
    UnexpectedType {
        object: LinkId,
        expected: String,
        actual: Option<String>,
    },
    /// A required field is missing.
    MissingField { object: LinkId, field: String },
    /// A field expected to be singular has multiple values.
    DuplicateField { object: LinkId, field: String },
    /// A field relation is structurally invalid.
    MalformedField { field: LinkId, reason: String },
    /// A literal link is missing or malformed.
    InvalidLiteral {
        object: LinkId,
        type_name: String,
        value: Option<String>,
        reason: String,
    },
    /// A cached object had an unexpected Rust type.
    CacheTypeMismatch { type_name: String },
}

impl fmt::Display for LinksCodecError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingLink(link) => write!(formatter, "missing link {link}"),
            Self::ExpectedObject { link } => write!(formatter, "link {link} is not an object"),
            Self::MalformedObject { object, reason } => {
                write!(formatter, "object {object} is malformed: {reason}")
            }
            Self::UnexpectedType {
                object,
                expected,
                actual,
            } => write!(
                formatter,
                "object {object} has type {:?}, expected {expected:?}",
                actual.as_deref()
            ),
            Self::MissingField { object, field } => {
                write!(formatter, "object {object} is missing field {field:?}")
            }
            Self::DuplicateField { object, field } => {
                write!(formatter, "object {object} has multiple {field:?} fields")
            }
            Self::MalformedField { field, reason } => {
                write!(formatter, "field link {field} is malformed: {reason}")
            }
            Self::InvalidLiteral {
                object,
                type_name,
                value,
                reason,
            } => write!(
                formatter,
                "object {object} has invalid {type_name} literal {:?}: {reason}",
                value.as_deref()
            ),
            Self::CacheTypeMismatch { type_name } => {
                write!(
                    formatter,
                    "cached object was not an Rc<RefCell<{type_name}>>"
                )
            }
        }
    }
}

impl Error for LinksCodecError {}

/// Encodes a Rust value into links.
pub trait ToLinks {
    /// Inserts links for this value and returns the root link id.
    fn to_links(&self, encoder: &mut LinksEncoder) -> LinkId;
}

/// Decodes a Rust value from links.
pub trait FromLinks: Sized {
    /// Reconstructs this value from a root link id.
    fn from_links(decoder: &mut LinksDecoder<'_>, link: LinkId) -> Result<Self, LinksCodecError>;
}

/// Helper trait for user-defined object types that need identity preservation.
///
/// Implement this for structs that may be shared or cyclic through
/// `Rc<RefCell<T>>`. `decode_fields` mutates an already-allocated default
/// value, which lets the decoder register an identity before recursively
/// decoding fields that may point back to the same object.
pub trait LinksObject: Default + 'static {
    /// Stable Rust type name stored in the type-shape link.
    const TYPE_NAME: &'static str;

    /// Queryable type declaration shape.
    fn type_shape() -> RustTypeShape;

    /// Encodes object fields after an identity link has been allocated.
    fn encode_fields(&self, encoder: &mut LinksEncoder, object: LinkId);

    /// Decodes fields into an already-allocated object.
    fn decode_fields(
        &mut self,
        decoder: &mut LinksDecoder<'_>,
        object: LinkId,
    ) -> Result<(), LinksCodecError>;
}

impl LinkNetwork {
    /// Inserts an anonymous object identity linked to its Rust/meta type.
    ///
    /// The resulting object references itself first, preserving a stable
    /// identity link for shared and circular object graphs, and references the
    /// type shape second so the object's declaration remains queryable.
    pub fn insert_object_instance(&mut self, type_id: LinkId) -> LinkId {
        let id = LinkId(self.next_id);
        self.next_id += 1;
        self.links.insert(
            id,
            Arc::new(Link {
                id,
                references: Arc::from(vec![id, type_id]),
                metadata: LinkMetadata::new().with_link_type(LinkType::Object),
            }),
        );
        id
    }
}

/// Stateful Rust value encoder.
#[derive(Debug)]
pub struct LinksEncoder {
    network: LinkNetwork,
    object_cache: BTreeMap<usize, LinkId>,
    registered_shapes: BTreeSet<String>,
}

impl Default for LinksEncoder {
    fn default() -> Self {
        Self::new()
    }
}

impl LinksEncoder {
    /// Creates an encoder with a self-describing network.
    #[must_use]
    pub fn new() -> Self {
        Self {
            network: LinkNetwork::self_describing(),
            object_cache: BTreeMap::new(),
            registered_shapes: BTreeSet::new(),
        }
    }

    /// Creates an encoder that appends to an existing network.
    #[must_use]
    pub const fn with_network(network: LinkNetwork) -> Self {
        Self {
            network,
            object_cache: BTreeMap::new(),
            registered_shapes: BTreeSet::new(),
        }
    }

    /// Returns the underlying network.
    #[must_use]
    pub const fn network(&self) -> &LinkNetwork {
        &self.network
    }

    /// Returns the underlying network mutably.
    pub fn network_mut(&mut self) -> &mut LinkNetwork {
        &mut self.network
    }

    /// Finishes encoding and returns the network.
    #[must_use]
    pub fn into_network(self) -> LinkNetwork {
        self.network
    }

    /// Encodes a value and returns its root link.
    pub fn encode<T: ToLinks + ?Sized>(&mut self, value: &T) -> LinkId {
        value.to_links(self)
    }

    /// Registers a queryable Rust type shape and returns its type link.
    pub fn register_type_shape(&mut self, shape: &RustTypeShape) -> LinkId {
        let shape_type_term = type_term(shape.name());
        let definition = format!("Rust {} type {}", shape.kind(), shape.name());
        let type_id =
            self.network
                .insert_typed_point(&shape_type_term, LinkType::Type, Some(&definition));

        if !self.registered_shapes.insert(shape.name().to_string()) {
            return type_id;
        }

        let kind = self.network.insert_typed_point(
            &format!("rust::type-kind::{}", shape.kind()),
            LinkType::Concept,
            None,
        );
        self.network.insert_dynamic_link(
            &[type_id, kind],
            LinkMetadata::new().with_link_type(LinkType::Relation),
        );

        for field in shape.fields() {
            let field_type_id = self.network.insert_typed_point(
                &type_term(field.type_name()),
                LinkType::Type,
                Some(&format!("Rust referenced type {}", field.type_name())),
            );
            self.network
                .insert_field(type_id, field.name(), field_type_id);
        }

        type_id
    }

    /// Starts an object instance for a Rust type shape.
    pub fn begin_object(&mut self, shape: &RustTypeShape) -> LinkId {
        let type_id = self.register_type_shape(shape);
        self.network.insert_object_instance(type_id)
    }

    /// Encodes a named field relation from an object to a child value.
    pub fn encode_field<T: ToLinks + ?Sized>(
        &mut self,
        object: LinkId,
        name: &str,
        value: &T,
    ) -> LinkId {
        let value = value.to_links(self);
        self.network.insert_field(object, name, value)
    }

    fn encode_literal(&mut self, type_name: &str, value: &str) -> LinkId {
        let object = self.begin_object(&RustTypeShape::primitive(type_name));
        let literal = self.network.insert_typed_point(
            &literal_term(type_name, value),
            LinkType::Concept,
            None,
        );
        self.network.insert_field(object, "value", literal);
        object
    }

    fn encode_rc<T: LinksObject>(&mut self, value: &Rc<RefCell<T>>) -> LinkId {
        let key = Rc::as_ptr(value).cast::<()>() as usize;
        if let Some(existing) = self.object_cache.get(&key) {
            return *existing;
        }

        let object = self.begin_object(&T::type_shape());
        self.object_cache.insert(key, object);
        value.borrow().encode_fields(self, object);
        object
    }
}

/// Stateful Rust value decoder.
#[derive(Debug)]
pub struct LinksDecoder<'network> {
    network: &'network LinkNetwork,
    object_cache: BTreeMap<(TypeId, LinkId), Box<dyn Any>>,
}

impl<'network> LinksDecoder<'network> {
    /// Creates a decoder over an existing network.
    #[must_use]
    pub fn new(network: &'network LinkNetwork) -> Self {
        Self {
            network,
            object_cache: BTreeMap::new(),
        }
    }

    /// Returns the decoded network.
    #[must_use]
    pub const fn network(&self) -> &'network LinkNetwork {
        self.network
    }

    /// Decodes a value from a root link.
    pub fn decode<T: FromLinks>(&mut self, link: LinkId) -> Result<T, LinksCodecError> {
        T::from_links(self, link)
    }

    /// Decodes a singular named field from an object.
    pub fn decode_field<T: FromLinks>(
        &mut self,
        object: LinkId,
        field: &str,
    ) -> Result<T, LinksCodecError> {
        let value = self.field_value(object, field)?;
        T::from_links(self, value)
    }

    fn decode_field_values<T: FromLinks>(
        &mut self,
        object: LinkId,
        field: &str,
    ) -> Result<Vec<T>, LinksCodecError> {
        let values = self.field_values(object, field)?;
        values
            .into_iter()
            .map(|value| T::from_links(self, value))
            .collect()
    }

    fn decode_rc<T: LinksObject>(
        &mut self,
        object: LinkId,
    ) -> Result<Rc<RefCell<T>>, LinksCodecError> {
        let key = (TypeId::of::<T>(), object);
        if let Some(cached) = self.object_cache.get(&key) {
            let typed = cached.downcast_ref::<Rc<RefCell<T>>>().ok_or_else(|| {
                LinksCodecError::CacheTypeMismatch {
                    type_name: T::TYPE_NAME.to_string(),
                }
            })?;
            return Ok(Rc::clone(typed));
        }

        self.expect_object_type(object, T::TYPE_NAME)?;
        let decoded = Rc::new(RefCell::new(T::default()));
        self.object_cache.insert(key, Box::new(Rc::clone(&decoded)));
        decoded.borrow_mut().decode_fields(self, object)?;
        Ok(decoded)
    }

    fn expect_object_type(&self, object: LinkId, expected: &str) -> Result<(), LinksCodecError> {
        let actual = self.object_type_name(object)?;
        if actual.as_deref() == Some(expected) {
            return Ok(());
        }

        Err(LinksCodecError::UnexpectedType {
            object,
            expected: expected.to_string(),
            actual,
        })
    }

    fn object_type_name(&self, object: LinkId) -> Result<Option<String>, LinksCodecError> {
        let link = self
            .network
            .link(object)
            .ok_or(LinksCodecError::MissingLink(object))?;
        if link.metadata().link_type() != Some(LinkType::Object) {
            return Err(LinksCodecError::ExpectedObject { link: object });
        }
        let references = link.references();
        if references.first() != Some(&object) {
            return Err(LinksCodecError::MalformedObject {
                object,
                reason: "first reference must preserve object identity".to_string(),
            });
        }
        let type_id = references
            .get(1)
            .ok_or_else(|| LinksCodecError::MalformedObject {
                object,
                reason: "second reference must point to a type shape".to_string(),
            })?;
        let type_link = self
            .network
            .link(*type_id)
            .ok_or(LinksCodecError::MissingLink(*type_id))?;
        Ok(type_link
            .metadata()
            .term()
            .and_then(|term| term.strip_prefix(TYPE_TERM_PREFIX))
            .map(ToString::to_string))
    }

    fn field_value(&self, object: LinkId, field: &str) -> Result<LinkId, LinksCodecError> {
        let values = self.field_values(object, field)?;
        match values.as_slice() {
            [value] => Ok(*value),
            [] => Err(LinksCodecError::MissingField {
                object,
                field: field.to_string(),
            }),
            _ => Err(LinksCodecError::DuplicateField {
                object,
                field: field.to_string(),
            }),
        }
    }

    fn field_values(&self, object: LinkId, field: &str) -> Result<Vec<LinkId>, LinksCodecError> {
        if self.network.link(object).is_none() {
            return Err(LinksCodecError::MissingLink(object));
        }

        let mut values = Vec::new();
        for link in self
            .network
            .links()
            .filter(|link| link.metadata().link_type() == Some(LinkType::Field))
        {
            let references = link.references();
            if references.first() != Some(&object) {
                continue;
            }
            let [_, label, value] = references else {
                return Err(LinksCodecError::MalformedField {
                    field: link.id(),
                    reason: "field links must have parent, label, and child references".to_string(),
                });
            };
            let Some(label_term) = self
                .network
                .link(*label)
                .ok_or(LinksCodecError::MissingLink(*label))?
                .metadata()
                .term()
            else {
                continue;
            };
            if label_term == field {
                values.push(*value);
            }
        }
        Ok(values)
    }

    fn decode_literal(&self, object: LinkId, type_name: &str) -> Result<String, LinksCodecError> {
        self.expect_object_type(object, type_name)?;
        let literal = self.field_value(object, "value")?;
        let term = self
            .network
            .link(literal)
            .ok_or(LinksCodecError::MissingLink(literal))?
            .metadata()
            .term()
            .map(ToString::to_string);
        let Some(term) = term else {
            return Err(LinksCodecError::InvalidLiteral {
                object,
                type_name: type_name.to_string(),
                value: None,
                reason: "literal link has no term".to_string(),
            });
        };
        let prefix = literal_prefix(type_name);
        term.strip_prefix(&prefix)
            .map(ToString::to_string)
            .ok_or_else(|| LinksCodecError::InvalidLiteral {
                object,
                type_name: type_name.to_string(),
                value: Some(term),
                reason: "literal term has the wrong prefix".to_string(),
            })
    }
}

impl<T: LinksObject> ToLinks for Rc<RefCell<T>> {
    fn to_links(&self, encoder: &mut LinksEncoder) -> LinkId {
        encoder.encode_rc(self)
    }
}

impl<T: LinksObject> FromLinks for Rc<RefCell<T>> {
    fn from_links(decoder: &mut LinksDecoder<'_>, link: LinkId) -> Result<Self, LinksCodecError> {
        decoder.decode_rc(link)
    }
}

impl<T: ToLinks> ToLinks for Option<T> {
    fn to_links(&self, encoder: &mut LinksEncoder) -> LinkId {
        let object = encoder.begin_object(&RustTypeShape::option("Option", "T"));
        match self {
            Some(value) => {
                encoder.encode_field(object, "Some", value);
            }
            None => {
                encoder.encode_field(object, "None", &());
            }
        }
        object
    }
}

impl<T: FromLinks> FromLinks for Option<T> {
    fn from_links(decoder: &mut LinksDecoder<'_>, link: LinkId) -> Result<Self, LinksCodecError> {
        decoder.expect_object_type(link, "Option")?;
        let some = decoder.field_values(link, "Some")?;
        let none = decoder.field_values(link, "None")?;
        match (some.as_slice(), none.as_slice()) {
            ([value], []) => T::from_links(decoder, *value).map(Some),
            ([], [_]) => Ok(None),
            ([], []) => Err(LinksCodecError::MissingField {
                object: link,
                field: "Some/None".to_string(),
            }),
            _ => Err(LinksCodecError::MalformedObject {
                object: link,
                reason: "Option must contain exactly one Some or None variant".to_string(),
            }),
        }
    }
}

impl<T: ToLinks> ToLinks for Vec<T> {
    fn to_links(&self, encoder: &mut LinksEncoder) -> LinkId {
        let object = encoder.begin_object(&RustTypeShape::sequence("Vec", "T"));
        for item in self {
            encoder.encode_field(object, "item", item);
        }
        object
    }
}

impl<T: FromLinks> FromLinks for Vec<T> {
    fn from_links(decoder: &mut LinksDecoder<'_>, link: LinkId) -> Result<Self, LinksCodecError> {
        decoder.expect_object_type(link, "Vec")?;
        decoder.decode_field_values(link, "item")
    }
}

impl<K, V> ToLinks for BTreeMap<K, V>
where
    K: ToLinks,
    V: ToLinks,
{
    fn to_links(&self, encoder: &mut LinksEncoder) -> LinkId {
        let object = encoder.begin_object(&RustTypeShape::map("BTreeMap", "K", "V"));
        for (key, value) in self {
            let entry = encoder.begin_object(&RustTypeShape::structure(
                "BTreeMapEntry",
                [
                    RustFieldShape::new("key", "K"),
                    RustFieldShape::new("value", "V"),
                ],
            ));
            encoder.encode_field(entry, "key", key);
            encoder.encode_field(entry, "value", value);
            encoder.network.insert_field(object, "entry", entry);
        }
        object
    }
}

impl<K, V> FromLinks for BTreeMap<K, V>
where
    K: FromLinks + Ord,
    V: FromLinks,
{
    fn from_links(decoder: &mut LinksDecoder<'_>, link: LinkId) -> Result<Self, LinksCodecError> {
        decoder.expect_object_type(link, "BTreeMap")?;
        let mut map = Self::new();
        for entry in decoder.field_values(link, "entry")? {
            decoder.expect_object_type(entry, "BTreeMapEntry")?;
            let key = decoder.decode_field(entry, "key")?;
            let value = decoder.decode_field(entry, "value")?;
            map.insert(key, value);
        }
        Ok(map)
    }
}

impl ToLinks for () {
    fn to_links(&self, encoder: &mut LinksEncoder) -> LinkId {
        encoder.encode_literal("()", "unit")
    }
}

impl FromLinks for () {
    fn from_links(decoder: &mut LinksDecoder<'_>, link: LinkId) -> Result<Self, LinksCodecError> {
        let _ = decoder.decode_literal(link, "()")?;
        Ok(())
    }
}

impl ToLinks for String {
    fn to_links(&self, encoder: &mut LinksEncoder) -> LinkId {
        encoder.encode_literal("String", self)
    }
}

impl ToLinks for str {
    fn to_links(&self, encoder: &mut LinksEncoder) -> LinkId {
        encoder.encode_literal("String", self)
    }
}

impl FromLinks for String {
    fn from_links(decoder: &mut LinksDecoder<'_>, link: LinkId) -> Result<Self, LinksCodecError> {
        decoder.decode_literal(link, "String")
    }
}

impl ToLinks for char {
    fn to_links(&self, encoder: &mut LinksEncoder) -> LinkId {
        encoder.encode_literal("char", &self.to_string())
    }
}

impl FromLinks for char {
    fn from_links(decoder: &mut LinksDecoder<'_>, link: LinkId) -> Result<Self, LinksCodecError> {
        let value = decoder.decode_literal(link, "char")?;
        let mut chars = value.chars();
        let Some(character) = chars.next() else {
            return Err(invalid_literal(
                link,
                "char",
                Some(value),
                "literal is empty",
            ));
        };
        if chars.next().is_some() {
            return Err(invalid_literal(
                link,
                "char",
                Some(value),
                "literal contains more than one character",
            ));
        }
        Ok(character)
    }
}

macro_rules! impl_fromstr_literal_codec {
    ($type:ty, $name:literal) => {
        impl ToLinks for $type {
            fn to_links(&self, encoder: &mut LinksEncoder) -> LinkId {
                encoder.encode_literal($name, &self.to_string())
            }
        }

        impl FromLinks for $type {
            fn from_links(
                decoder: &mut LinksDecoder<'_>,
                link: LinkId,
            ) -> Result<Self, LinksCodecError> {
                let value = decoder.decode_literal(link, $name)?;
                <$type>::from_str(&value)
                    .map_err(|error| invalid_literal(link, $name, Some(value), &error.to_string()))
            }
        }
    };
}

impl_fromstr_literal_codec!(bool, "bool");
impl_fromstr_literal_codec!(i8, "i8");
impl_fromstr_literal_codec!(i16, "i16");
impl_fromstr_literal_codec!(i32, "i32");
impl_fromstr_literal_codec!(i64, "i64");
impl_fromstr_literal_codec!(i128, "i128");
impl_fromstr_literal_codec!(isize, "isize");
impl_fromstr_literal_codec!(u8, "u8");
impl_fromstr_literal_codec!(u16, "u16");
impl_fromstr_literal_codec!(u32, "u32");
impl_fromstr_literal_codec!(u64, "u64");
impl_fromstr_literal_codec!(u128, "u128");
impl_fromstr_literal_codec!(usize, "usize");
impl_fromstr_literal_codec!(f32, "f32");
impl_fromstr_literal_codec!(f64, "f64");

fn type_term(name: &str) -> String {
    format!("{TYPE_TERM_PREFIX}{name}")
}

fn literal_prefix(type_name: &str) -> String {
    format!("{LITERAL_TERM_PREFIX}{type_name}:")
}

fn literal_term(type_name: &str, value: &str) -> String {
    format!("{}{value}", literal_prefix(type_name))
}

fn invalid_literal(
    object: LinkId,
    type_name: &str,
    value: Option<String>,
    reason: &str,
) -> LinksCodecError {
    LinksCodecError::InvalidLiteral {
        object,
        type_name: type_name.to_string(),
        value,
        reason: reason.to_string(),
    }
}
