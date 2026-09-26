//! Portable-core types.
//!
//! Numeric types keep their source semantics apart:
//! `nat` and `int` are unbounded, `fixed` is a Rust machine integer whose
//! arithmetic aborts on overflow. Data types are named by their qualified path.
//!
//! Mirrors `js/src/translation/types.js`; the serialised form is the same JSON
//! object (`{ "kind": "fixed", "bits": 32, "signed": true }`).

use serde::{Deserialize, Serialize};

use super::Span;

/// A portable-core type, or a surface type reference before resolution.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum Type {
    Nat,
    Int,
    Bool,
    String,
    Unit,
    Fixed {
        bits: u32,
        signed: bool,
    },
    /// A resolved data type, named by its qualified path.
    Data {
        name: std::string::String,
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
}

pub const NAT: Type = Type::Nat;
pub const INT: Type = Type::Int;
pub const BOOL: Type = Type::Bool;
pub const STRING: Type = Type::String;
pub const UNIT: Type = Type::Unit;

#[must_use]
pub const fn fixed(bits: u32, signed: bool) -> Type {
    Type::Fixed { bits, signed }
}

#[must_use]
pub fn data(name: impl Into<std::string::String>) -> Type {
    Type::Data { name: name.into() }
}

impl Type {
    /// The type's identity: `nat`, `i32`, `data:Tree`.
    #[must_use]
    pub fn key(&self) -> std::string::String {
        match self {
            Self::Fixed { bits, signed } => format!("{}{bits}", if *signed { 'i' } else { 'u' }),
            Self::Data { name } => format!("data:{name}"),
            other => other.kind().to_owned(),
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
            Self::Fixed { .. } => "fixed",
            Self::Data { .. } => "data",
            Self::Named { .. } => "named",
            Self::Literal => "literal",
            Self::Hypothesis => "hypothesis",
        }
    }

    #[must_use]
    pub fn same(&self, other: &Self) -> bool {
        self.key() == other.key()
    }

    #[must_use]
    pub const fn is_numeric(&self) -> bool {
        matches!(self, Self::Nat | Self::Int | Self::Fixed { .. })
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
