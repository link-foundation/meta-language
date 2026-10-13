//! Checked declaration records, re-exported through the unchanged IR API.
use super::{Binder, Expr, Guard, Proof, Prop, Span, Type};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Field {
    pub name: String,
    #[serde(rename = "type")]
    pub ty: Type,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Ctor {
    pub name: String,
    pub fields: Vec<Field>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Param {
    pub name: String,
    #[serde(rename = "type")]
    pub ty: Type,
    /// A JavaScript parameter guarded to the naturals.
    #[serde(default)]
    pub guard: Option<Guard>,
    #[serde(default, rename = "default")]
    pub default_value: Option<crate::translation::surface::SExpr>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DataDecl {
    pub name: String,
    pub ctors: Vec<Ctor>,
    #[serde(default)]
    pub span: Option<Span>,
    pub full_name: String,
    pub module_path: Vec<String>,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub generated: bool,
    /// A pair of printed lines and a value (see `output`).
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub output: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FnDecl {
    pub name: String,
    pub params: Vec<Param>,
    pub ret: Type,
    pub body: Expr,
    #[serde(default)]
    pub span: Option<Span>,
    pub full_name: String,
    pub module_path: Vec<String>,
    pub recursive: bool,
    /// The structurally decreasing parameter of a recursive function.
    pub decreasing: Option<usize>,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub mutual: bool,
    /// A function the translator made up, such as a lifted loop.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub generated: bool,
    /// Optional owned source-header facts, checked against this signature.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_signature: Option<crate::translation::source_signatures::SourceSignature>,
    /// Actual standard primitive source owners, retained independently of value identity.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_primitive_owners: Option<serde_json::Value>,
    /// Optional owned standard empty-option source interpretation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_empty_option: Option<serde_json::Value>,
    /// Optional actual Rust type spellings with rechecked scoped ownership.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_type_namespace: Option<serde_json::Value>,
    /// Optional owned scalar loop state and checked continuation interpretation.
    #[serde(
        default,
        rename = "sourceLoop",
        skip_serializing_if = "Option::is_none"
    )]
    pub source_loop: Option<crate::translation::source_loops::SourceLoop>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TheoremDecl {
    pub name: String,
    pub binders: Vec<Binder>,
    pub prop: Prop,
    pub proof: Proof,
    #[serde(default)]
    pub span: Option<Span>,
    pub full_name: String,
    pub module_path: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "k", rename_all = "camelCase")]
#[allow(clippy::large_enum_variant)] // programs hold a handful of declarations
pub enum Decl {
    Data(DataDecl),
    Fn(FnDecl),
    Theorem(TheoremDecl),
}

impl Decl {
    #[must_use]
    pub fn name(&self) -> &str {
        match self {
            Self::Data(decl) => &decl.name,
            Self::Fn(decl) => &decl.name,
            Self::Theorem(decl) => &decl.name,
        }
    }

    #[must_use]
    pub fn full_name(&self) -> &str {
        match self {
            Self::Data(decl) => &decl.full_name,
            Self::Fn(decl) => &decl.full_name,
            Self::Theorem(decl) => &decl.full_name,
        }
    }

    #[must_use]
    pub fn module_path(&self) -> &[String] {
        match self {
            Self::Data(decl) => &decl.module_path,
            Self::Fn(decl) => &decl.module_path,
            Self::Theorem(decl) => &decl.module_path,
        }
    }

    #[must_use]
    pub const fn span(&self) -> Option<Span> {
        match self {
            Self::Data(decl) => decl.span,
            Self::Fn(decl) => decl.span,
            Self::Theorem(decl) => decl.span,
        }
    }

    /// `data`, `fn` or `theorem`.
    #[must_use]
    pub const fn kind(&self) -> &'static str {
        match self {
            Self::Data(_) => "data",
            Self::Fn(_) => "fn",
            Self::Theorem(_) => "theorem",
        }
    }
}
