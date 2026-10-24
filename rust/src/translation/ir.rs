//! The checked portable core: every expression carries its type and every
//! operator its source semantics. Emitters consume it; the renaming passes
//! give each target unique, legal local names.
//!
//! The JSON form is the JavaScript runtime's checked program, except that
//! `declarations` is an array and `items` refer to declarations by name.

use std::collections::{BTreeSet, HashMap, HashSet};

use serde::{Deserialize, Serialize};

use super::surface::{BinaryOp, External, Flavor, Guard, Rounding, SImport, UnaryOp};
use super::types::Type;
use super::{Language, Span};

mod declarations;
mod record_entry;
mod reference_type;
mod rename;
pub use record_entry::RecordEntry;
mod nodes;
pub use nodes::Node;
#[allow(clippy::trivially_copy_pass_by_ref)] // serde passes fields by reference
const fn is_false(value: &bool) -> bool {
    !*value
}

pub use self::declarations::{
    Ctor, DataDecl, Decl, Field, FnDecl, LambdaParam, Param, TheoremDecl,
};
pub use self::rename::*;

/// Arithmetic semantics: exact on unbounded integers, checked (aborting)
/// machine arithmetic, truncated natural subtraction, or IEEE-754 binary64
/// arithmetic on JavaScript Numbers.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Semantics {
    Exact,
    Checked,
    Truncated,
    /// Round to nearest, even on a tie; `%` is the exact truncated remainder
    /// (C `fmod`); nothing aborts.
    Ieee,
}

/// What division by zero does: abort (JavaScript, Rust) or yield a total value (Lean, Rocq).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ByZero {
    Abort,
    Total,
}

/// A literal's value: numerals and strings are text, booleans are booleans.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LitValue {
    Bool(bool),
    Text(String),
}

impl LitValue {
    /// The value as JavaScript's `String(value)` renders it.
    #[must_use]
    pub fn text(&self) -> String {
        match self {
            Self::Bool(value) => value.to_string(),
            Self::Text(text) => text.clone(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Expr {
    pub node: Node,
    pub ty: Type,
    pub span: Option<Span>,
}

impl Expr {
    #[must_use]
    pub const fn new(node: Node, ty: Type) -> Self {
        Self {
            node,
            ty,
            span: None,
        }
    }

    #[must_use]
    pub fn var(name: impl Into<String>, ty: Type) -> Self {
        Self::new(Node::Var { name: name.into() }, ty)
    }

    #[must_use]
    pub const fn lit(ty: Type, value: LitValue) -> Self {
        Self::new(Node::Lit { value }, ty)
    }

    #[must_use]
    pub const fn with_span(mut self, span: Option<Span>) -> Self {
        self.span = span;
        self
    }

    /// The name of the variable this expression is, if it is one.
    #[must_use]
    pub fn var_name(&self) -> Option<&str> {
        match &self.node {
            Node::Var { name } => Some(name),
            _ => None,
        }
    }

    /// The `k` discriminant.
    #[must_use]
    pub const fn kind(&self) -> &'static str {
        self.node.kind()
    }

    /// Direct subexpressions in the JavaScript runtime's key order.
    #[must_use]
    pub fn children(&self) -> Vec<&Self> {
        match &self.node {
            Node::Lit { .. }
            | Node::Unit
            | Node::None { .. }
            | Node::Var { .. }
            | Node::FunctionRef { .. }
            | Node::Abort { .. }
            | Node::OutNil => Vec::new(),
            Node::Call { args, .. }
            | Node::Ctor { args, .. }
            | Node::Array { items: args }
            | Node::Tuple { items: args, .. }
            | Node::Math { args, .. } => args.iter().collect(),
            Node::Apply { func, args } => {
                std::iter::once(func.as_ref()).chain(args.iter()).collect()
            }
            Node::Lambda { body, .. } => vec![body],
            Node::Unary { arg, .. }
            | Node::Set { items: arg }
            | Node::Map { entries: arg }
            | Node::Some { value: arg }
            | Node::FlattenOption { value: arg }
            | Node::OptionalField { object: arg, .. }
            | Node::OptionalProject { object: arg, .. }
            | Node::NullishNumber { arg }
            | Node::OptionalText { arg, .. }
            | Node::OptionalTypeOf { arg, .. }
            | Node::Truthy { arg }
            | Node::UnwrapOption { arg }
            | Node::OptionalPredicate { arg, .. }
            | Node::ReferenceAllocate { value: arg }
            | Node::ReferenceRead { reference: arg }
            | Node::CellAllocate { value: arg }
            | Node::CellRead { cell: arg }
            | Node::OptionalIndexNumber { arg }
            | Node::StringPoints { object: arg }
            | Node::CollectionItems { object: arg, .. }
            | Node::CollectionSize { object: arg }
            | Node::StringMap { string: arg, .. }
            | Node::ToString { arg, .. }
            | Node::Cast { arg, .. }
            | Node::TupleIndex { object: arg, .. }
            | Node::RecordField { object: arg, .. }
            | Node::Length { array: arg } => vec![arg],
            Node::Record { entries } => entries.iter().map(|entry| &entry.value).collect(),
            Node::Method { object, args, .. } => std::iter::once(object.as_ref())
                .chain(args.iter().flatten())
                .collect(),
            Node::StringTest {
                string: left,
                search: right,
                ..
            }
            | Node::CollectionRead {
                object: left,
                key: right,
                ..
            }
            | Node::Binary { left, right, .. }
            | Node::Coalesce { left, right }
            | Node::StrictEqual { left, right, .. }
            | Node::Append { left, right }
            | Node::Index {
                array: left,
                index: right,
                ..
            }
            | Node::OptionalIndex {
                array: left,
                index: right,
            } => vec![left, right],
            Node::If {
                cond,
                then,
                otherwise,
            } => vec![cond, then, otherwise],
            Node::Let { value, body, .. } => vec![value, body],
            Node::CellWrite { cell, value, body } => vec![cell, value, body],
            Node::Print { text, body } => vec![text, body],
            Node::OutCons { head, tail } => vec![head, tail],
            Node::Match { scrutinee, cases } => {
                let mut children = vec![&**scrutinee];
                children.extend(cases.iter().map(|kase| &kase.body));
                children
            }
        }
    }

    /// Rebuilds the node with every direct subexpression mapped, in key order.
    #[must_use]
    pub fn map_children(&self, visit: &mut impl FnMut(&Self) -> Self) -> Self {
        let node = match &self.node {
            Node::Lit { .. }
            | Node::Unit
            | Node::None { .. }
            | Node::Var { .. }
            | Node::FunctionRef { .. }
            | Node::Abort { .. }
            | Node::OutNil => self.node.clone(),
            Node::Print { text, body } => {
                let text = Box::new(visit(text));
                let body = Box::new(visit(body));
                Node::Print { text, body }
            }
            Node::OutCons { head, tail } => {
                let head = Box::new(visit(head));
                let tail = Box::new(visit(tail));
                Node::OutCons { head, tail }
            }
            Node::Array { items } => Node::Array {
                items: items.iter().map(&mut *visit).collect(),
            },
            Node::Set { items } => Node::Set {
                items: Box::new(visit(items)),
            },
            Node::Map { entries } => Node::Map {
                entries: Box::new(visit(entries)),
            },
            Node::Record { entries } => Node::Record {
                entries: entries
                    .iter()
                    .map(|entry| RecordEntry {
                        name: entry.name.clone(),
                        value: visit(&entry.value),
                    })
                    .collect(),
            },
            Node::RecordField {
                object,
                field,
                index,
            } => Node::RecordField {
                object: Box::new(visit(object)),
                field: field.clone(),
                index: *index,
            },
            Node::Tuple {
                items,
                frozen,
                product_model,
            } => Node::Tuple {
                items: items.iter().map(&mut *visit).collect(),
                frozen: *frozen,
                product_model: product_model.clone(),
            },
            Node::TupleIndex { object, index } => Node::TupleIndex {
                object: Box::new(visit(object)),
                index: *index,
            },
            Node::Append { left, right } => {
                let left = Box::new(visit(left));
                let right = Box::new(visit(right));
                Node::Append { left, right }
            }
            Node::Index {
                array,
                index,
                proven_in_bounds,
            } => {
                let array = Box::new(visit(array));
                let index = Box::new(visit(index));
                Node::Index {
                    array,
                    index,
                    proven_in_bounds: *proven_in_bounds,
                }
            }
            Node::OptionalIndex { array, index } => Node::OptionalIndex {
                array: Box::new(visit(array)),
                index: Box::new(visit(index)),
            },
            Node::OptionalIndexNumber { arg } => Node::OptionalIndexNumber {
                arg: Box::new(visit(arg)),
            },
            Node::Some { value } => Node::Some {
                value: Box::new(visit(value)),
            },
            Node::FlattenOption { value } => Node::FlattenOption {
                value: Box::new(visit(value)),
            },
            Node::Coalesce { left, right } => Node::Coalesce {
                left: Box::new(visit(left)),
                right: Box::new(visit(right)),
            },
            Node::OptionalField {
                object,
                field,
                index,
            } => Node::OptionalField {
                object: Box::new(visit(object)),
                field: field.clone(),
                index: *index,
            },
            Node::OptionalProject { object, field } => Node::OptionalProject {
                object: Box::new(visit(object)),
                field: field.clone(),
            },
            Node::NullishNumber { arg } => Node::NullishNumber {
                arg: Box::new(visit(arg)),
            },
            Node::OptionalText { arg, console } => Node::OptionalText {
                arg: Box::new(visit(arg)),
                console: *console,
            },
            Node::OptionalTypeOf { arg, present } => Node::OptionalTypeOf {
                arg: Box::new(visit(arg)),
                present: present.clone(),
            },
            Node::Truthy { arg } => Node::Truthy {
                arg: Box::new(visit(arg)),
            },
            Node::StrictEqual {
                left,
                right,
                negate,
            } => Node::StrictEqual {
                left: Box::new(visit(left)),
                right: Box::new(visit(right)),
                negate: *negate,
            },
            Node::UnwrapOption { arg } => Node::UnwrapOption {
                arg: Box::new(visit(arg)),
            },
            Node::OptionalPredicate { arg, op } => Node::OptionalPredicate {
                arg: Box::new(visit(arg)),
                op: op.clone(),
            },
            Node::StringPoints { object } => Node::StringPoints {
                object: Box::new(visit(object)),
            },
            Node::CollectionItems { object, op } => Node::CollectionItems {
                object: Box::new(visit(object)),
                op: op.clone(),
            },
            Node::CollectionSize { object } => Node::CollectionSize {
                object: Box::new(visit(object)),
            },
            Node::CollectionRead { object, op, key } => Node::CollectionRead {
                object: Box::new(visit(object)),
                op: op.clone(),
                key: Box::new(visit(key)),
            },
            Node::Length { array } => Node::Length {
                array: Box::new(visit(array)),
            },
            Node::ReferenceAllocate { value } => Node::ReferenceAllocate {
                value: Box::new(visit(value)),
            },
            Node::ReferenceRead { reference } => Node::ReferenceRead {
                reference: Box::new(visit(reference)),
            },
            Node::Math { op, args } => Node::Math {
                op: op.clone(),
                args: args.iter().map(&mut *visit).collect(),
            },
            Node::Call { func, args } => Node::Call {
                func: func.clone(),
                args: args.iter().map(&mut *visit).collect(),
            },
            Node::Apply { func, args } => Node::Apply {
                func: Box::new(visit(func)),
                args: args.iter().map(&mut *visit).collect(),
            },
            Node::Lambda { params, body } => Node::Lambda {
                params: params.clone(),
                body: Box::new(visit(body)),
            },
            Node::CellAllocate { value } => Node::CellAllocate {
                value: Box::new(visit(value)),
            },
            Node::CellRead { cell } => Node::CellRead {
                cell: Box::new(visit(cell)),
            },
            Node::CellWrite { cell, value, body } => Node::CellWrite {
                cell: Box::new(visit(cell)),
                value: Box::new(visit(value)),
                body: Box::new(visit(body)),
            },
            Node::Ctor {
                data,
                ctor,
                args,
                frozen,
            } => Node::Ctor {
                data: data.clone(),
                ctor: ctor.clone(),
                args: args.iter().map(&mut *visit).collect(),
                frozen: *frozen,
            },
            Node::Unary { op, arg, semantics } => Node::Unary {
                op: *op,
                arg: Box::new(visit(arg)),
                semantics: *semantics,
            },
            Node::StringMap { op, string } => Node::StringMap {
                op: op.clone(),
                string: Box::new(visit(string)),
            },
            Node::Method { object, op, args } => Node::Method {
                object: Box::new(visit(object)),
                op: op.clone(),
                args: args
                    .iter()
                    .map(|arg| arg.as_ref().map(&mut *visit))
                    .collect(),
            },
            Node::StringTest { op, string, search } => Node::StringTest {
                op: op.clone(),
                string: Box::new(visit(string)),
                search: Box::new(visit(search)),
            },
            Node::ToString { arg, console } => Node::ToString {
                arg: Box::new(visit(arg)),
                console: *console,
            },
            Node::Cast {
                arg,
                from,
                to,
                flavor,
                message,
                order,
            } => Node::Cast {
                arg: Box::new(visit(arg)),
                from: from.clone(),
                to: to.clone(),
                flavor: *flavor,
                message: message.clone(),
                order: *order,
            },
            Node::Binary {
                op,
                left,
                right,
                domain,
                semantics,
                rounding,
                by_zero,
            } => {
                let left = Box::new(visit(left));
                let right = Box::new(visit(right));
                Node::Binary {
                    op: *op,
                    left,
                    right,
                    domain: domain.clone(),
                    semantics: *semantics,
                    rounding: *rounding,
                    by_zero: *by_zero,
                }
            }
            Node::If {
                cond,
                then,
                otherwise,
            } => {
                let cond = Box::new(visit(cond));
                let then = Box::new(visit(then));
                let otherwise = Box::new(visit(otherwise));
                Node::If {
                    cond,
                    then,
                    otherwise,
                }
            }
            Node::Let { name, value, body } => {
                let value = Box::new(visit(value));
                let body = Box::new(visit(body));
                Node::Let {
                    name: name.clone(),
                    value,
                    body,
                }
            }
            Node::Match { scrutinee, cases } => {
                let scrutinee = Box::new(visit(scrutinee));
                let cases = cases
                    .iter()
                    .map(|kase| Case {
                        pattern: kase.pattern.clone(),
                        body: visit(&kase.body),
                    })
                    .collect();
                Node::Match { scrutinee, cases }
            }
        };
        Self {
            node,
            ty: self.ty.clone(),
            span: self.span,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Case {
    pub pattern: Pattern,
    pub body: Expr,
}

/// A single-level match pattern.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "k", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum Pattern {
    Wild,
    Bind {
        name: String,
    },
    NatZero,
    NatSucc {
        name: String,
    },
    Ctor {
        data: String,
        ctor: String,
        binds: Vec<Option<String>>,
    },
}

impl Pattern {
    /// Locals the pattern binds.
    #[must_use]
    pub fn bound(&self) -> Vec<&str> {
        match self {
            Self::Bind { name } | Self::NatSucc { name } => vec![name],
            Self::Ctor { binds, .. } => binds.iter().flatten().map(String::as_str).collect(),
            Self::Wild | Self::NatZero => Vec::new(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Binder {
    pub name: String,
    #[serde(rename = "type")]
    pub ty: Type,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Comparison {
    pub left: Expr,
    pub right: Expr,
    pub domain: Type,
    /// Over Numbers, `assert.strictEqual` compares with `SameValue`, not
    /// `===`: `NaN` equals `NaN`, and `0` differs from `-0`.
    #[serde(default, rename = "sameValue", skip_serializing_if = "is_false")]
    pub same_value: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "p", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum Prop {
    Forall {
        binders: Vec<Binder>,
        body: Box<Self>,
    },
    And {
        left: Box<Self>,
        right: Box<Self>,
    },
    Or {
        left: Box<Self>,
        right: Box<Self>,
    },
    Implies {
        left: Box<Self>,
        right: Box<Self>,
    },
    Not {
        arg: Box<Self>,
    },
    Bool {
        expr: Expr,
    },
    Eq(Comparison),
    Ne(Comparison),
    Lt(Comparison),
    Le(Comparison),
    Gt(Comparison),
    Ge(Comparison),
}

impl Prop {
    /// The comparison operator and operands of an `eq` … `ge` proposition.
    #[must_use]
    pub const fn comparison(&self) -> Option<(BinaryOp, &Comparison)> {
        match self {
            Self::Eq(comparison) => Some((BinaryOp::Eq, comparison)),
            Self::Ne(comparison) => Some((BinaryOp::Ne, comparison)),
            Self::Lt(comparison) => Some((BinaryOp::Lt, comparison)),
            Self::Le(comparison) => Some((BinaryOp::Le, comparison)),
            Self::Gt(comparison) => Some((BinaryOp::Gt, comparison)),
            Self::Ge(comparison) => Some((BinaryOp::Ge, comparison)),
            _ => None,
        }
    }

    /// Builds the comparison proposition for `op`.
    ///
    /// # Panics
    /// When `op` is not a comparison.
    #[must_use]
    pub fn compare(op: BinaryOp, comparison: Comparison) -> Self {
        match op {
            BinaryOp::Eq => Self::Eq(comparison),
            BinaryOp::Ne => Self::Ne(comparison),
            BinaryOp::Lt => Self::Lt(comparison),
            BinaryOp::Le => Self::Le(comparison),
            BinaryOp::Gt => Self::Gt(comparison),
            BinaryOp::Ge => Self::Ge(comparison),
            other => panic!("{other:?} is not a comparison"),
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Hints {
    pub unfold: Vec<String>,
    pub lemmas: Vec<String>,
    pub hyps: Vec<String>,
    pub library: Vec<String>,
    pub arith: bool,
    pub compute: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "k", rename_all = "camelCase")]
pub enum Plan {
    Close { hints: Hints },
    Induction(Split),
    Cases(Split),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Split {
    pub variable: String,
    #[serde(rename = "type")]
    pub ty: Type,
    pub cases: Vec<PlanCase>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlanCase {
    pub ctor: String,
    pub fields: Vec<String>,
    pub ihs: Vec<String>,
    pub recursive: Vec<bool>,
    pub plan: Plan,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Proof {
    pub plan: Plan,
    pub source: String,
    pub source_language: Language,
}

/// A program item: a declaration, named by its full name, or a module.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "k", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum Item {
    Decl {
        full_name: String,
    },
    Module {
        name: String,
        full_name: String,
        items: Vec<Self>,
        #[serde(default)]
        span: Option<Span>,
    },
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "k", rename_all = "camelCase")]
#[allow(clippy::large_enum_variant)]
pub enum Effect {
    Print {
        expr: Expr,
        #[serde(default)]
        span: Option<Span>,
    },
    Let {
        /// Actual ordered owned Unit-call statement site.
        #[serde(
            default,
            rename = "sourcePureUnitCall",
            skip_serializing_if = "Option::is_none"
        )]
        source_pure_unit_call: Option<serde_json::Value>,
        name: String,
        value: Expr,
        #[serde(default, skip_serializing_if = "is_false")]
        constant: bool,
        #[serde(default)]
        span: Option<Span>,
    },
    Assert {
        prop: Prop,
        #[serde(default)]
        span: Option<Span>,
    },
    /// Prints the lines `expr` holds, oldest first (see `output`).
    Output {
        expr: Expr,
        #[serde(default)]
        span: Option<Span>,
    },
    /// Binds `name` to the value of `pair`, made by the first of `ctors`, or
    /// stops main with the message of its abort, made by the second (see `output`).
    Unwrap {
        name: String,
        pair: Expr,
        data: String,
        ctors: Vec<String>,
        #[serde(rename = "type")]
        ty: Type,
        #[serde(default)]
        span: Option<Span>,
    },
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Main {
    /// Owned worker context for an independently selected complete runtime.
    #[serde(
        default,
        rename = "sourceRuntimeWorker",
        skip_serializing_if = "Option::is_none"
    )]
    pub source_runtime_worker: Option<serde_json::Value>,
    pub effects: Vec<Effect>,
    #[serde(default)]
    pub span: Option<Span>,
    /// Every call of an async function is awaited where it is made.
    #[serde(rename = "sequentialAsync", default, skip_serializing_if = "is_false")]
    pub sequential_async: bool,
}

fn read_present_program_string_representation<'de, D: serde::Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<bool>, D::Error> {
    bool::deserialize(deserializer).map(Some)
}

/// A checked portable-core program.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Program {
    /// Selected shared-reference representation; false retains its default omission.
    #[serde(default, skip_serializing_if = "is_false")]
    pub reference_semantics: bool,
    /// Explicit UTF-16 string encoding selection; absence and false stay distinct.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "read_present_program_string_representation"
    )]
    pub utf16_strings: Option<bool>,
    /// Complete owned source runtime interpretation, separate from its pure core.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_runtime_shell: Option<serde_json::Value>,
    /// Independently selected complete pure Unit program interpretation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_pure_unit_calls: Option<serde_json::Value>,
    pub schema_version: u32,
    pub source_language: Language,
    pub items: Vec<Item>,
    pub main: Option<Main>,
    /// Every declaration in source order.
    pub declarations: Vec<Decl>,
    /// Functions that print take and return the lines printed (see `output`).
    #[serde(default, skip_serializing_if = "is_false")]
    pub output_threaded: bool,
    /// Functions that may abort return the abort's message instead of a value (see `output`).
    #[serde(default, skip_serializing_if = "is_false")]
    pub aborts_threaded: bool,
    /// Imports of the items of other modules of the crate (self-translation only).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub imports: Vec<SImport>,
    /// The other items of the module, which the program calls and reads but
    /// does not declare: a call of one is a `call` of its name.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub externals: Vec<External>,
}

impl Program {
    /// The other item of the module that a call of `name` calls or reads,
    /// when the program declares nothing under that name.
    #[must_use]
    pub fn external(&self, name: &str) -> Option<&External> {
        if self.declaration(name).is_some() {
            return None;
        }
        self.externals
            .iter()
            .find(|external| external.name() == name)
    }

    #[must_use]
    pub fn declaration(&self, full_name: &str) -> Option<&Decl> {
        self.declarations
            .iter()
            .find(|decl| decl.full_name() == full_name)
    }

    /// The data declaration named `full_name`.
    ///
    /// # Panics
    /// When there is none; the checker resolves every data type it records.
    #[must_use]
    pub fn data(&self, full_name: &str) -> &DataDecl {
        match self.declaration(full_name) {
            Some(Decl::Data(decl)) => decl,
            _ => panic!("no data declaration {full_name}"),
        }
    }

    /// Declarations in item order, modules flattened.
    #[must_use]
    pub fn flat_items(&self) -> Vec<&Decl> {
        fn walk<'a>(program: &'a Program, items: &[Item], into: &mut Vec<&'a Decl>) {
            for item in items {
                match item {
                    Item::Decl { full_name } => into.extend(program.declaration(full_name)),
                    Item::Module { items, .. } => walk(program, items, into),
                }
            }
        }
        let mut into = Vec::new();
        walk(self, &self.items, &mut into);
        into
    }
}

/// A per-declaration name supply: target-legal, unique, deterministic.
#[derive(Clone)]
pub struct Scope<'a> {
    ident: &'a dyn Fn(&str) -> String,
    used: HashSet<String>,
}

impl<'a> Scope<'a> {
    pub fn new(ident: &'a dyn Fn(&str) -> String, reserved: &BTreeSet<String>) -> Self {
        Self {
            ident,
            used: reserved.iter().cloned().collect(),
        }
    }

    pub fn fresh(&mut self, name: &str) -> String {
        let name = crate::translation::source_identifiers::render_source_target_identifier(name);
        // A suffixed name is legal on its own: `end_2` needs none of the quoting of Lean's `«end»`.
        let mut candidate = crate::translation::source_identifiers::render_source_target_identifier(
            &(self.ident)(&name),
        );
        let mut index = 2;
        while self.used.contains(&candidate) {
            candidate =
                crate::translation::source_identifiers::render_source_target_identifier(&(self
                    .ident)(
                    &format!("{name}_{index}"),
                ));
            index += 1;
        }
        self.used.insert(candidate.clone());
        candidate
    }

    #[must_use]
    pub fn child(&self) -> Self {
        self.clone()
    }
}

type Env = HashMap<String, String>;
