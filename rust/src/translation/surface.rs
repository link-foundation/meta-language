//! The surface AST the four frontends produce and the checker consumes.
//!
//! Its JSON form is the JavaScript runtime's (`{ "k": "binary", "op": "add", … }`),
//! so the frontends of both runtimes can be compared node for node.

use serde::{Deserialize, Deserializer, Serialize};

use super::types::Type;
use super::{Language, Span};

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
pub struct SExpr {
    #[serde(flatten)]
    pub node: SNode,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub span: Option<Span>,
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
}

/// Conversion semantics: exact, checked (aborts when out of range) or clamped.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Flavor {
    Exact,
    Checked,
    Clamp,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "k", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum SNode {
    Num {
        value: String,
        #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
        ty: Option<Type>,
        #[serde(default, skip_serializing_if = "is_false")]
        negative: bool,
    },
    Bool {
        value: bool,
    },
    Str {
        value: String,
    },
    Unit,
    Name {
        path: Vec<String>,
    },
    /// Lean's `.ctor`, resolved against the expected type.
    DotCtor {
        name: String,
    },
    App {
        #[serde(rename = "fn")]
        func: Box<SExpr>,
        args: Vec<SExpr>,
    },
    Field {
        object: Box<SExpr>,
        field: String,
    },
    Unary {
        op: UnaryOp,
        arg: Box<SExpr>,
    },
    Binary {
        op: BinaryOp,
        left: Box<SExpr>,
        right: Box<SExpr>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        rounding: Option<Rounding>,
    },
    If {
        cond: Box<SExpr>,
        then: Box<SExpr>,
        #[serde(rename = "else")]
        otherwise: Box<SExpr>,
    },
    Let {
        name: String,
        #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
        ty: Option<Type>,
        value: Box<SExpr>,
        body: Box<SExpr>,
    },
    Match {
        scrutinees: Vec<SExpr>,
        rows: Vec<SRow>,
    },
    /// A single-level match, as pattern compilation produces it.
    Match1 {
        scrutinee: Box<SExpr>,
        cases: Vec<SCase>,
    },
    ToString {
        arg: Box<SExpr>,
    },
    Show {
        arg: Box<SExpr>,
        style: ShowStyle,
    },
    Cast {
        arg: Box<SExpr>,
        to: Type,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        from: Option<Type>,
        flavor: Flavor,
    },
    Abort {
        message: String,
    },
    /// A JavaScript `{ $: 'tag', field: value }` constructor object, fields in source order.
    CtorObject {
        tag: String,
        #[serde(with = "ordered_fields")]
        fields: Vec<(String, SExpr)>,
    },
    // Frontend-internal forms; the checker rejects them if they reach it.
    Cons {
        head: Box<SExpr>,
        tail: Box<SExpr>,
    },
    Nil,
    List {
        items: Vec<SExpr>,
    },
    Tag {
        tag: String,
    },
    Default,
}

impl SNode {
    /// The `k` discriminant.
    #[must_use]
    pub const fn kind(&self) -> &'static str {
        match self {
            Self::Num { .. } => "num",
            Self::Bool { .. } => "bool",
            Self::Str { .. } => "str",
            Self::Unit => "unit",
            Self::Name { .. } => "name",
            Self::DotCtor { .. } => "dotCtor",
            Self::App { .. } => "app",
            Self::Field { .. } => "field",
            Self::Unary { .. } => "unary",
            Self::Binary { .. } => "binary",
            Self::If { .. } => "if",
            Self::Let { .. } => "let",
            Self::Match { .. } => "match",
            Self::Match1 { .. } => "match1",
            Self::ToString { .. } => "toString",
            Self::Show { .. } => "show",
            Self::Cast { .. } => "cast",
            Self::Abort { .. } => "abort",
            Self::CtorObject { .. } => "ctorObject",
            Self::Cons { .. } => "cons",
            Self::Nil => "nil",
            Self::List { .. } => "list",
            Self::Tag { .. } => "tag",
            Self::Default => "default",
        }
    }
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
    /// Lean's `n + 2`.
    NatAdd {
        inner: Box<SPattern>,
        add: u64,
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

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SParam {
    pub name: String,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub ty: Option<Type>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub span: Option<Span>,
    /// A JavaScript parameter restricted to the naturals by a leading guard.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guard: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rocq_type: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SData {
    pub name: String,
    pub ctors: Vec<SCtor>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub span: Option<Span>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SFn {
    pub name: String,
    pub params: Vec<SParam>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ret: Option<Type>,
    pub body: SExpr,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub span: Option<Span>,
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
        name: String,
        #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
        ty: Option<Type>,
        value: SExpr,
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
    pub effects: Vec<SEffect>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub span: Option<Span>,
}

/// A parsed source program.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SProgram {
    pub language: Language,
    pub items: Vec<SItem>,
    #[serde(default)]
    pub main: Option<SMain>,
}
