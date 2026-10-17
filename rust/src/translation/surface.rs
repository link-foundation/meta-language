//! The surface AST the four frontends produce and the checker consumes.
//!
//! Its JSON form is the JavaScript runtime's (`{ "k": "binary", "op": "add", … }`),
//! so the frontends of both runtimes can be compared node for node.

use serde::{Deserialize, Deserializer, Serialize};

use super::types::Type;
use super::{Language, Span};

mod externals;
pub use externals::{External, ExternalParam, SImport, SImportName};
mod array_item;
pub use array_item::SArrayItem;
mod binding_type_annotation;
pub use binding_type_annotation::SurfaceBindingTypeAnnotation;
mod raw_metadata;
pub use raw_metadata::SurfaceRawMetadata;
mod nodes;
pub use nodes::SNode;
pub(crate) mod array_bindings;
mod record_entry;
pub use record_entry::SRecordEntry;
mod record_projection;

#[allow(clippy::trivially_copy_pass_by_ref)] // serde passes fields by reference
const fn is_false(value: &bool) -> bool {
    !*value
}

/// Numeral text: the JavaScript frontends store some pattern numerals as
/// numbers and others as decimal strings.
fn numeral<'de, D: Deserializer<'de>>(deserializer: D) -> Result<String, D::Error> {
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum Numeral {
        Text(String),
        Number(serde_json::Number),
    }
    Ok(match Numeral::deserialize(deserializer)? {
        Numeral::Text(text) => text,
        Numeral::Number(number) => number.to_string(),
    })
}

/// A surface expression.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "array_bindings::SurfaceExpression")]
pub struct SExpr {
    #[serde(flatten)]
    pub node: SNode,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub span: Option<Span>,
    /// Grouping is raw source metadata, with absence independent of its value.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "raw_metadata::read_present_surface_raw_metadata"
    )]
    pub parenthesized: Option<SurfaceRawMetadata>,
    /// Set on a Rust block expression (`{ … }`) used as a value.
    #[serde(default, skip_serializing_if = "is_false")]
    pub block: bool,
    /// Set on the match a JavaScript `x.$ === 'tag'` test reads as, so an
    /// `if` on it can narrow `x`.
    #[serde(rename = "tagTest", default, skip_serializing_if = "Option::is_none")]
    pub tag_test: Option<Box<STagTest>>,
}

/// A JavaScript `x.$ === 'tag'` (or `!==`) test: the tested value, the tag
/// and the data type that has it.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct STagTest {
    pub object: SExpr,
    pub tag: String,
    pub data: SData,
    pub negated: bool,
}

impl SExpr {
    #[must_use]
    pub const fn new(node: SNode, span: Option<Span>) -> Self {
        Self {
            node,
            span,
            parenthesized: None,
            block: false,
            tag_test: None,
        }
    }

    /// A name reference without a span, as the checker builds them.
    #[must_use]
    pub fn name(name: &str) -> Self {
        Self::new(
            SNode::Name {
                path: vec![name.to_owned()],
            },
            None,
        )
    }

    /// The single-segment name this expression refers to, if it is one.
    #[must_use]
    pub fn simple_name(&self) -> Option<&str> {
        match &self.node {
            SNode::Name { path } if path.len() == 1 => Some(&path[0]),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum UnaryOp {
    Not,
    Neg,
}

/// Surface binary operators: `plus` is JavaScript's overloaded `+`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    And,
    Or,
    Concat,
    Plus,
}

impl BinaryOp {
    /// The operator's name in the checked program's JSON, such as `add`.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Add => "add",
            Self::Sub => "sub",
            Self::Mul => "mul",
            Self::Div => "div",
            Self::Rem => "rem",
            Self::Eq => "eq",
            Self::Ne => "ne",
            Self::Lt => "lt",
            Self::Le => "le",
            Self::Gt => "gt",
            Self::Ge => "ge",
            Self::And => "and",
            Self::Or => "or",
            Self::Concat => "concat",
            Self::Plus => "plus",
        }
    }
}

/// Division rounding a source operator asks for.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Rounding {
    Trunc,
    Euclid,
    Floor,
}

/// How a value is printed: each source language's own rendering.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ShowStyle {
    #[serde(rename = "lean")]
    Lean,
    #[serde(rename = "rocq")]
    Rocq,
    #[serde(rename = "rust")]
    Rust,
    #[serde(rename = "js-console")]
    JsConsole,
    #[serde(rename = "js-template")]
    JsTemplate,
    /// The first of several `console.log` arguments, when it is not a literal string.
    #[serde(rename = "js-format-first")]
    JsFormatFirst,
    /// The argument of a `%d` directive.
    #[serde(rename = "js-format-number")]
    JsFormatNumber,
    /// The argument of a `%i` directive.
    #[serde(rename = "js-format-integer")]
    JsFormatInteger,
}

impl ShowStyle {
    /// True for the values `console.log` shows: its argument, or those `util.format` reads.
    #[must_use]
    pub const fn is_console(self) -> bool {
        matches!(
            self,
            Self::JsConsole | Self::JsFormatFirst | Self::JsFormatNumber | Self::JsFormatInteger
        )
    }
}

/// Conversion semantics: exact, checked (aborts when out of range) or clamped.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Flavor {
    Exact,
    Checked,
    Clamp,
}

/// A source prototype definition, preserved until the checker refuses it.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConstructorPrototypeDefinition {
    /// The complete initializer expression.
    pub value: Box<SExpr>,
    /// The uncomputed property key's source range.
    pub span: Span,
    /// The original property position, including the constructor tag.
    pub index: usize,
}

/// An object whose keys keep their insertion order, as JavaScript objects do.
mod ordered_fields {
    use std::fmt;

    use serde::de::{MapAccess, Visitor};
    use serde::ser::SerializeMap;
    use serde::{Deserializer, Serializer};

    use super::SExpr;

    #[allow(clippy::ptr_arg)]
    pub fn serialize<S: Serializer>(
        fields: &Vec<(String, SExpr)>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(fields.len()))?;
        for (name, value) in fields {
            map.serialize_entry(name, value)?;
        }
        map.end()
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Vec<(String, SExpr)>, D::Error> {
        struct Fields;
        impl<'de> Visitor<'de> for Fields {
            type Value = Vec<(String, SExpr)>;
            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("an object of fields")
            }
            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut fields = Vec::new();
                while let Some(entry) = map.next_entry()? {
                    fields.push(entry);
                }
                Ok(fields)
            }
        }
        deserializer.deserialize_map(Fields)
    }
}

/// One row of a multi-scrutinee surface match.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SRow {
    pub patterns: Vec<SPattern>,
    pub body: SExpr,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub span: Option<Span>,
}

/// One case of a single-level match.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SCase {
    pub pattern: SCasePattern,
    pub body: SExpr,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub span: Option<Span>,
}

/// Patterns of a single-level match: one constructor, fields bound to names.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "k", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum SCasePattern {
    Wild,
    Bind {
        name: String,
    },
    NatZero,
    NatSucc {
        name: String,
    },
    Ctor {
        path: Vec<String>,
        binds: Vec<Option<String>>,
    },
}

/// A surface pattern.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SPattern {
    #[serde(flatten)]
    pub node: SPatternNode,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub span: Option<Span>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "k", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum SPatternNode {
    Wild,
    /// A lone name: a nullary constructor when the scrutinee type has one, else a binder.
    BindOrCtor {
        name: String,
    },
    /// A binder, which the JavaScript frontend's generated matches use.
    Bind {
        name: String,
    },
    Ctor {
        path: Vec<String>,
        args: Vec<SPattern>,
    },
    NumLit {
        #[serde(deserialize_with = "numeral")]
        value: String,
        #[serde(default, skip_serializing_if = "is_false")]
        negative: bool,
    },
    BoolLit {
        value: bool,
        #[serde(default, skip_serializing_if = "is_false")]
        negative: bool,
    },
    StrLit {
        value: String,
    },
    /// Lean's `n + 2`; `add` holds the numeral's decimal digits.
    NatAdd {
        inner: Box<SPattern>,
        add: String,
    },
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SBinder {
    pub name: String,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub ty: Option<Type>,
    /// The Rocq type as written (`N`, `nat`, `Z`), which proof steps consult.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rocq_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub span: Option<Span>,
}

/// A surface proposition.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SProp {
    #[serde(flatten)]
    pub node: SPropNode,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub span: Option<Span>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SComparison {
    pub left: SExpr,
    pub right: SExpr,
    /// JavaScript's `assert.equal`, which compares objects by identity.
    #[serde(default, skip_serializing_if = "is_false")]
    pub reference: bool,
    /// `assert.strictEqual` and its kin compare with `SameValue`
    /// (`Object.is`): `NaN` equals `NaN`, and `0` differs from `-0`.
    #[serde(default, rename = "sameValue", skip_serializing_if = "is_false")]
    pub same_value: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "p", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum SPropNode {
    Forall {
        binders: Vec<SBinder>,
        body: Box<SProp>,
    },
    And {
        left: Box<SProp>,
        right: Box<SProp>,
    },
    Or {
        left: Box<SProp>,
        right: Box<SProp>,
    },
    Implies {
        left: Box<SProp>,
        right: Box<SProp>,
    },
    Not {
        arg: Box<SProp>,
    },
    Bool {
        expr: SExpr,
    },
    Eq(SComparison),
    Ne(SComparison),
    Lt(SComparison),
    Le(SComparison),
    Gt(SComparison),
    Ge(SComparison),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SRule {
    pub name: String,
    pub reverse: bool,
}

/// A proof step: `t` names the tactic family.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "t", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum SStep {
    Compute {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        tactic: Option<String>,
    },
    Arith {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        tactic: Option<String>,
    },
    Intro {
        names: Vec<String>,
    },
    Unfold {
        names: Vec<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        tactic: Option<String>,
    },
    Simp {
        rules: Vec<SRule>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        only: Option<bool>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        tactic: Option<String>,
    },
    Rewrite {
        rules: Vec<SRule>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        only: Option<bool>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        tactic: Option<String>,
    },
    Induction(SSplit),
    Cases(SSplit),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SSplit {
    pub variable: String,
    pub cases: Vec<SSplitCase>,
    /// Rocq names cases by position; Lean by constructor.
    #[serde(default, skip_serializing_if = "is_false")]
    pub positional: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub blocks: Option<usize>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SSplitCase {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ctor: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index: Option<usize>,
    pub binds: Vec<Option<String>>,
    pub steps: Vec<SStep>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SProof {
    pub steps: Vec<SStep>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    pub source_language: Language,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SField {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type")]
    pub ty: Type,
    /// The type as written, on a Rocq constructor field declared as a binder `(x : N)`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rocq_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub span: Option<Span>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SCtor {
    pub name: String,
    pub fields: Vec<SField>,
}

/// The leading `if (n < 0n) throw …` of a JavaScript parameter: a negative
/// argument aborts with its message, the guards in their order.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Guard {
    pub message: String,
    pub order: usize,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SParam {
    pub name: String,
    /// An omitted documented parameter passes its actual undefined default.
    #[serde(default, skip_serializing_if = "is_false")]
    pub optional: bool,
    #[serde(default, rename = "default", skip_serializing_if = "Option::is_none")]
    pub default_value: Option<SExpr>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub ty: Option<Type>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub span: Option<Span>,
    /// A JavaScript parameter restricted to the naturals by a leading guard.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guard: Option<Guard>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rocq_type: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SData {
    pub name: String,
    pub ctors: Vec<SCtor>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub span: Option<Span>,
    /// A data type the translator made up, whose name may use the reserved `ml_` prefix.
    #[serde(default, skip_serializing_if = "is_false")]
    pub generated: bool,
    /// Optional actual field-free Rust Clone derivation owned by this datatype.
    #[serde(
        default,
        rename = "sourceUnitCloneDerivation",
        skip_serializing_if = "Option::is_none"
    )]
    pub source_unit_clone_derivation: Option<Box<serde_json::Value>>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SFn {
    /// Actual complete pure Unit header and body sites under the selected model.
    #[serde(
        default,
        rename = "sourcePureUnit",
        skip_serializing_if = "Option::is_none"
    )]
    pub source_pure_unit: Option<Box<serde_json::Value>>,
    pub name: String,
    pub params: Vec<SParam>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ret: Option<Type>,
    /// Actual inferred return when source annotations omit an absence case.
    #[serde(
        default,
        rename = "semanticRet",
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::translation::present_type::read_present_type"
    )]
    pub semantic_ret: Option<Type>,
    pub body: SExpr,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub span: Option<Span>,
    /// A function the translator made up, such as a lifted loop.
    #[serde(default, skip_serializing_if = "is_false")]
    pub generated: bool,
    /// Optional owned facts from the actually parsed source function header.
    #[serde(
        default,
        rename = "sourceSignature",
        skip_serializing_if = "Option::is_none"
    )]
    pub source_signature: Option<Box<super::source_signatures::SourceSignature>>,
    /// Optional owned scalar loop state and checked continuation interpretation.
    #[serde(
        default,
        rename = "sourceLoop",
        skip_serializing_if = "Option::is_none"
    )]
    pub source_loop: Option<Box<super::source_loops::SourceLoop>>,
    /// Owned actual standard primitive paths, imported aliases and body sites.
    #[serde(
        default,
        rename = "sourcePrimitiveOwners",
        skip_serializing_if = "Option::is_none"
    )]
    pub source_primitive_owners: Option<serde_json::Value>,
    /// Optional owned standard empty-option source interpretation.
    #[serde(
        default,
        rename = "sourceEmptyOption",
        skip_serializing_if = "Option::is_none"
    )]
    pub source_empty_option: Option<Box<serde_json::Value>>,
    /// Optional actual Rust type spellings with rechecked scoped ownership.
    #[serde(
        default,
        rename = "sourceTypeNamespace",
        skip_serializing_if = "Option::is_none"
    )]
    pub source_type_namespace: Option<Box<serde_json::Value>>,
    /// Ordered owned local String moves, copies and replacements.
    #[serde(
        default,
        rename = "sourceLocalState",
        skip_serializing_if = "Option::is_none"
    )]
    pub source_local_state: Option<Box<serde_json::Value>>,
    /// Owned actual const qualification of a closed primitive predicate.
    #[serde(
        default,
        rename = "sourceConstFunction",
        skip_serializing_if = "Option::is_none"
    )]
    pub source_const_function: Option<Box<serde_json::Value>>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct STheorem {
    pub name: String,
    pub binders: Vec<SBinder>,
    pub prop: SProp,
    pub proof: SProof,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub span: Option<Span>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SModule {
    pub name: String,
    pub items: Vec<SItem>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub span: Option<Span>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "k", rename_all = "camelCase")]
pub enum SItem {
    Data(SData),
    Fn(SFn),
    Theorem(STheorem),
    Module(SModule),
}

impl SItem {
    #[must_use]
    pub fn name(&self) -> &str {
        match self {
            Self::Data(item) => &item.name,
            Self::Fn(item) => &item.name,
            Self::Theorem(item) => &item.name,
            Self::Module(item) => &item.name,
        }
    }

    #[must_use]
    pub const fn span(&self) -> Option<Span> {
        match self {
            Self::Data(item) => item.span,
            Self::Fn(item) => item.span,
            Self::Theorem(item) => item.span,
            Self::Module(item) => item.span,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "k", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum SEffect {
    Print {
        expr: SExpr,
        style: ShowStyle,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        span: Option<Span>,
    },
    Let {
        /// Actual ordered owned Unit-call statement site.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        source_pure_unit_call: Option<serde_json::Value>,
        name: String,
        #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
        ty: Option<Type>,
        value: SExpr,
        #[serde(default, skip_serializing_if = "is_false")]
        constant: bool,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        span: Option<Span>,
    },
    Assert {
        prop: SProp,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        span: Option<Span>,
    },
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SMain {
    /// Owned worker context for an independently selected complete runtime.
    #[serde(
        default,
        rename = "sourceRuntimeWorker",
        skip_serializing_if = "Option::is_none"
    )]
    pub source_runtime_worker: Option<serde_json::Value>,
    pub effects: Vec<SEffect>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub span: Option<Span>,
    /// Every call of an async function is awaited where it is made.
    #[serde(rename = "sequentialAsync", default, skip_serializing_if = "is_false")]
    pub sequential_async: bool,
}

/// A parsed source program.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SProgram {
    /// Preserve the raw program representation selector without erasing null.
    #[serde(
        default,
        rename = "referenceSemantics",
        skip_serializing_if = "Option::is_none",
        deserialize_with = "raw_metadata::read_present_surface_raw_metadata"
    )]
    pub reference_semantics: Option<SurfaceRawMetadata>,
    /// Complete owned source runtime interpretation, separate from its pure core.
    #[serde(
        default,
        rename = "sourceRuntimeShell",
        skip_serializing_if = "Option::is_none"
    )]
    pub source_runtime_shell: Option<serde_json::Value>,
    /// Independently selected complete pure Unit program interpretation.
    #[serde(
        default,
        rename = "sourcePureUnitCalls",
        skip_serializing_if = "Option::is_none"
    )]
    pub source_pure_unit_calls: Option<serde_json::Value>,
    pub language: Language,
    pub items: Vec<SItem>,
    #[serde(default)]
    pub main: Option<SMain>,
    /// Imports of the items of other modules of the crate (self-translation only).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub imports: Vec<SImport>,
    /// The other items of the module, which the program calls and reads.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub externals: Vec<External>,
}
