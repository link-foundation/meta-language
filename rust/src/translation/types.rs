//! Portable-core types.
//!
//! Numeric types keep their source semantics apart:
//! `nat` and `int` are unbounded, `fixed` is a Rust machine integer whose
//! arithmetic aborts on overflow, and `float` is a JavaScript Number, an
//! IEEE-754 binary64. Data types are named by their qualified path.
//!
//! Mirrors `js/src/translation/types.js`; the serialised form is the same JSON
//! object (`{ "kind": "fixed", "bits": 32, "signed": true }`).

use serde::{Deserialize, Serialize};

use super::Span;
use super::primitive_value_rules::accept_immutable_product_kind;
use super::structured_type_rules::{
    accept_optional_absence, accept_type_identity, merge_optional_absence,
    render_portable_type_identity, wrap_portable_reference_identity,
};

fn default_absence() -> std::string::String {
    "both".to_owned()
}
const fn is_false(value: &bool) -> bool {
    !*value
}

/// A present product model must be a string; null cannot erase its semantics.
pub(crate) fn deserialize_tuple_product_model<'de, D: serde::Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<std::string::String>, D::Error> {
    std::string::String::deserialize(deserializer)
        .map(Some)
        .map_err(|error| {
            <D::Error as serde::de::Error>::custom(format!("invalid tuple product model: {error}"))
        })
}

/// One named field of a structural record.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TypeField {
    /// The source field name.
    pub name: std::string::String,
    /// The field's recursively structured type.
    #[serde(rename = "type")]
    pub ty: Type,
}

/// A portable-core type, or a surface type reference before resolution.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum Type {
    Nat,
    Int,
    Bool,
    String,
    Unit,
    /// A JavaScript Number: an IEEE-754 binary64.
    Float,
    Fixed {
        bits: u32,
        signed: bool,
    },
    /// A resolved data type, named by its qualified path.
    Data {
        name: std::string::String,
    },
    /// A JavaScript array, optionally retaining a shared reference identity.
    Array {
        element: Box<Self>,
        #[serde(default, skip_serializing_if = "is_false")]
        reference: bool,
    },
    /// A binding whose value lives in a shared reference cell.
    Cell {
        element: Box<Self>,
    },
    /// Absence retains the distinction between JavaScript null and undefined.
    Option {
        element: Box<Self>,
        #[serde(default = "default_absence")]
        absence: std::string::String,
    },
    /// A positional product type; value nodes retain the explicit frozen facet.
    Tuple {
        elements: Vec<Self>,
    },
    /// Named fields, whose declaration order does not affect type identity.
    Record {
        fields: Vec<TypeField>,
        #[serde(default, skip_serializing_if = "is_false")]
        open: bool,
        #[serde(default, skip_serializing_if = "is_false")]
        reference: bool,
    },
    /// An insertion-ordered mapping, optionally retaining a shared reference identity.
    Map {
        key: Box<Self>,
        value: Box<Self>,
        #[serde(default, skip_serializing_if = "is_false")]
        reference: bool,
    },
    /// An insertion-ordered set, optionally retaining a shared reference identity.
    Set {
        element: Box<Self>,
        #[serde(default, skip_serializing_if = "is_false")]
        reference: bool,
    },
    /// The parameter and return types of a callable value.
    Function {
        params: Vec<Self>,
        ret: Box<Self>,
    },
    /// A surface reference to a declared type, resolved by the checker.
    Named {
        path: Vec<std::string::String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        span: Option<Span>,
    },
    /// The type of a numeral whose type the context has not fixed yet.
    Literal,
    /// A proof hypothesis bound by a case split (proof plans only).
    Hypothesis,
    /// The lines a program has printed, newest first: only the targets whose
    /// programs are pure carry them, as a value (see `output`).
    Output,
}

pub const NAT: Type = Type::Nat;
pub const INT: Type = Type::Int;
pub const BOOL: Type = Type::Bool;
pub const STRING: Type = Type::String;
pub const UNIT: Type = Type::Unit;
pub const FLOAT: Type = Type::Float;

#[must_use]
pub const fn fixed(bits: u32, signed: bool) -> Type {
    Type::Fixed { bits, signed }
}

#[must_use]
pub fn array(element: Type) -> Type {
    Type::Array {
        element: Box::new(element),
        reference: false,
    }
}

/// Constructs and flattens an optional type using the shared absence policy.
#[must_use]
pub fn option(element: Type, absence: impl Into<std::string::String>) -> Type {
    let absence = absence.into();
    match element {
        Type::Option {
            element,
            absence: inner,
        } => option(*element, merge_optional_absence(inner, absence)),
        element => Type::Option {
            element: Box::new(element),
            absence,
        },
    }
}

#[must_use]
pub fn data(name: impl Into<std::string::String>) -> Type {
    Type::Data { name: name.into() }
}

impl Type {
    /// Membership in the selected recursive scalar Copy product model.
    #[must_use]
    pub fn immutable_product_type(&self) -> bool {
        accept_immutable_product_kind(self.kind().to_owned())
            && match self {
                Self::Tuple { elements } => elements.iter().all(Self::immutable_product_type),
                _ => true,
            }
    }

    /// Immediate child types, in representation order.
    #[must_use]
    pub fn children(&self) -> Vec<&Self> {
        match self {
            Self::Array { element, .. }
            | Self::Option { element, .. }
            | Self::Set { element, .. }
            | Self::Cell { element } => {
                vec![element]
            }
            Self::Tuple { elements } => elements.iter().collect(),
            Self::Record { fields, .. } => fields.iter().map(|field| &field.ty).collect(),
            Self::Map { key, value, .. } => vec![key, value],
            Self::Function { params, ret } => {
                params.iter().chain(std::iter::once(ret.as_ref())).collect()
            }
            _ => Vec::new(),
        }
    }

    /// Adapts children through the host checker without duplicating type rules.
    /// Record fields are put into their canonical UTF-16 name order.
    ///
    /// # Errors
    /// Returns an error from the supplied child adapter.
    pub fn map_children<E>(
        &self,
        mut map: impl FnMut(&Self) -> Result<Self, E>,
    ) -> Result<Self, E> {
        Ok(match self {
            Self::Array { element, .. } => Self::Array {
                element: Box::new(map(element)?),
                reference: self.is_reference(),
            },
            Self::Cell { element } => Self::Cell {
                element: Box::new(map(element)?),
            },
            Self::Option { element, absence } => Self::Option {
                element: Box::new(map(element)?),
                absence: absence.clone(),
            },
            Self::Set { element, .. } => Self::Set {
                element: Box::new(map(element)?),
                reference: self.is_reference(),
            },
            Self::Tuple { elements } => Self::Tuple {
                elements: elements.iter().map(&mut map).collect::<Result<_, _>>()?,
            },
            Self::Record { fields, open, .. } => {
                let mut fields: Vec<_> = fields
                    .iter()
                    .map(|field| {
                        Ok(TypeField {
                            name: field.name.clone(),
                            ty: map(&field.ty)?,
                        })
                    })
                    .collect::<Result<_, E>>()?;
                fields.sort_by(|first, second| {
                    first.name.encode_utf16().cmp(second.name.encode_utf16())
                });
                Self::Record {
                    fields,
                    open: *open,
                    reference: self.is_reference(),
                }
            }
            Self::Map { key, value, .. } => Self::Map {
                key: Box::new(map(key)?),
                value: Box::new(map(value)?),
                reference: self.is_reference(),
            },
            Self::Function { params, ret } => Self::Function {
                params: params.iter().map(&mut map).collect::<Result<_, _>>()?,
                ret: Box::new(map(ret)?),
            },
            other => other.clone(),
        })
    }

    /// The type's identity: `nat`, `i32`, `data:Tree`.
    #[must_use]
    pub fn key(&self) -> std::string::String {
        let (detail, children) = match self {
            Self::Fixed { bits, signed } => (
                format!("{}{bits}", if *signed { 'i' } else { 'u' }),
                Vec::new(),
            ),
            Self::Data { name } => (name.clone(), Vec::new()),
            Self::Array { element, .. } | Self::Set { element, .. } | Self::Cell { element } => {
                (std::string::String::new(), vec![element.key()])
            }
            Self::Option { element, absence } => {
                let mut absence = absence.clone();
                let mut current = element.as_ref();
                while let Self::Option {
                    element,
                    absence: inner,
                } = current
                {
                    absence = merge_optional_absence(inner.clone(), absence);
                    current = element;
                }
                (absence, vec![current.key()])
            }
            Self::Tuple { elements } => (
                std::string::String::new(),
                elements.iter().map(Self::key).collect(),
            ),
            Self::Record { fields, open, .. } => {
                let mut sorted: Vec<_> = fields.iter().collect();
                sorted.sort_by(|first, second| {
                    first.name.encode_utf16().cmp(second.name.encode_utf16())
                });
                let children = sorted
                    .iter()
                    .map(|field| {
                        serde_json::to_string(&[field.name.clone(), field.ty.key()])
                            .expect("string pairs serialize")
                    })
                    .collect();
                (
                    if *open {
                        "open:".to_owned()
                    } else {
                        std::string::String::new()
                    },
                    children,
                )
            }
            Self::Map { key, value, .. } => {
                (std::string::String::new(), vec![key.key(), value.key()])
            }
            Self::Function { params, ret } => (ret.key(), params.iter().map(Self::key).collect()),
            _ => (std::string::String::new(), Vec::new()),
        };
        wrap_portable_reference_identity(
            render_portable_type_identity(self.kind().to_owned(), detail, children),
            self.is_reference(),
        )
    }

    /// Whether a structural value retains a shared mutable reference identity.
    #[must_use]
    pub const fn is_reference(&self) -> bool {
        match self {
            Self::Array { reference, .. }
            | Self::Record { reference, .. }
            | Self::Map { reference, .. }
            | Self::Set { reference, .. } => *reference,
            _ => false,
        }
    }

    /// The `kind` discriminant as the JavaScript runtime spells it.
    #[must_use]
    pub const fn kind(&self) -> &'static str {
        match self {
            Self::Nat => "nat",
            Self::Int => "int",
            Self::Bool => "bool",
            Self::String => "string",
            Self::Unit => "unit",
            Self::Float => "float",
            Self::Fixed { .. } => "fixed",
            Self::Data { .. } => "data",
            Self::Array { .. } => "array",
            Self::Cell { .. } => "cell",
            Self::Option { .. } => "option",
            Self::Tuple { .. } => "tuple",
            Self::Record { .. } => "record",
            Self::Map { .. } => "map",
            Self::Set { .. } => "set",
            Self::Function { .. } => "function",
            Self::Named { .. } => "named",
            Self::Literal => "literal",
            Self::Hypothesis => "hypothesis",
            Self::Output => "output",
        }
    }

    #[must_use]
    pub fn same(&self, other: &Self) -> bool {
        accept_type_identity(self.key(), other.key())
    }

    /// Whether this optional type admits a particular absence case.
    #[must_use]
    pub fn accepts_absence(&self, actual: &str) -> bool {
        match self {
            Self::Option { absence, .. } => {
                accept_optional_absence(absence.clone(), actual.to_owned())
            }
            _ => false,
        }
    }

    #[must_use]
    pub const fn is_numeric(&self) -> bool {
        matches!(self, Self::Nat | Self::Int | Self::Fixed { .. })
    }

    #[must_use]
    pub const fn is_float(&self) -> bool {
        matches!(self, Self::Float)
    }

    /// True for the types `<` orders: the integers and Numbers.
    #[must_use]
    pub const fn is_ordered(&self) -> bool {
        self.is_numeric() || self.is_float()
    }

    /// True when every value of the type is a non-negative integer.
    #[must_use]
    pub const fn is_natural(&self) -> bool {
        matches!(self, Self::Nat | Self::Fixed { signed: false, .. })
    }

    #[must_use]
    pub const fn is_literal(&self) -> bool {
        matches!(self, Self::Literal)
    }

    /// The element type of an array.
    #[must_use]
    pub fn element(&self) -> Option<&Self> {
        match self {
            Self::Array { element, .. } => Some(element),
            _ => None,
        }
    }

    /// The data type's qualified name.
    #[must_use]
    pub fn data_name(&self) -> Option<&str> {
        match self {
            Self::Data { name } => Some(name),
            _ => None,
        }
    }
}

/// Inclusive bounds of a machine-integer type.
#[must_use]
pub fn fixed_bounds(bits: u32, signed: bool) -> (i128, u128) {
    if signed {
        let max = (1u128 << (bits - 1)) - 1;
        let min = -i128::try_from(max).unwrap_or(i128::MAX) - 1;
        (min, max)
    } else if bits >= 128 {
        (0, u128::MAX)
    } else {
        (0, (1u128 << bits) - 1)
    }
}

/// `u8` … `i128`, `usize`, `isize` (the pointer-sized types are 64 bits).
#[must_use]
pub fn rust_fixed_type(name: &str) -> Option<Type> {
    let signed = match name.as_bytes().first() {
        Some(b'u') => false,
        Some(b'i') => true,
        _ => return None,
    };
    let bits = match &name[1..] {
        "8" => 8,
        "16" => 16,
        "32" => 32,
        "64" | "size" => 64,
        "128" => 128,
        _ => return None,
    };
    Some(fixed(bits, signed))
}
