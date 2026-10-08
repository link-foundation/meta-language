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

mod rename;
#[allow(clippy::trivially_copy_pass_by_ref)] // serde passes fields by reference
const fn is_false(value: &bool) -> bool {
    !*value
}

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

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Expr {
    #[serde(flatten)]
    pub node: Node,
    #[serde(rename = "type")]
    pub ty: Type,
    #[serde(default, skip_serializing_if = "Option::is_none")]
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
            | Node::Var { .. }
            | Node::Abort { .. }
            | Node::OutNil => Vec::new(),
            Node::Call { args, .. }
            | Node::Ctor { args, .. }
            | Node::Array { items: args }
            | Node::Math { args, .. } => args.iter().collect(),
            Node::Unary { arg, .. }
            | Node::StringMap { string: arg, .. }
            | Node::ToString { arg, .. }
            | Node::Cast { arg, .. }
            | Node::Length { array: arg } => vec![arg],
            Node::StringTest {
                string: left,
                search: right,
                ..
            }
            | Node::Binary { left, right, .. }
            | Node::Append { left, right }
            | Node::Index {
                array: left,
                index: right,
            } => vec![left, right],
            Node::If {
                cond,
                then,
                otherwise,
            } => vec![cond, then, otherwise],
            Node::Let { value, body, .. } => vec![value, body],
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
            | Node::Var { .. }
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
            Node::Append { left, right } => {
                let left = Box::new(visit(left));
                let right = Box::new(visit(right));
                Node::Append { left, right }
            }
            Node::Index { array, index } => {
                let array = Box::new(visit(array));
                let index = Box::new(visit(index));
                Node::Index { array, index }
            }
            Node::Length { array } => Node::Length {
                array: Box::new(visit(array)),
            },
            Node::Math { op, args } => Node::Math {
                op: op.clone(),
                args: args.iter().map(&mut *visit).collect(),
            },
            Node::Call { func, args } => Node::Call {
                func: func.clone(),
                args: args.iter().map(&mut *visit).collect(),
            },
            Node::Ctor { data, ctor, args } => Node::Ctor {
                data: data.clone(),
                ctor: ctor.clone(),
                args: args.iter().map(&mut *visit).collect(),
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
#[serde(tag = "k", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum Node {
    Lit {
        value: LitValue,
    },
    Unit,
    Var {
        name: String,
    },
    Call {
        #[serde(rename = "fn")]
        func: String,
        args: Vec<Expr>,
    },
    Ctor {
        data: String,
        ctor: String,
        args: Vec<Expr>,
    },
    Unary {
        op: UnaryOp,
        arg: Box<Expr>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        semantics: Option<Semantics>,
    },
    Binary {
        op: BinaryOp,
        left: Box<Expr>,
        right: Box<Expr>,
        /// The operand type of comparisons and arithmetic.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        domain: Option<Type>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        semantics: Option<Semantics>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        rounding: Option<Rounding>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        by_zero: Option<ByZero>,
    },
    If {
        cond: Box<Expr>,
        then: Box<Expr>,
        #[serde(rename = "else")]
        otherwise: Box<Expr>,
    },
    Let {
        name: String,
        value: Box<Expr>,
        body: Box<Expr>,
    },
    Match {
        scrutinee: Box<Expr>,
        cases: Vec<Case>,
    },
    StringMap {
        op: String,
        string: Box<Expr>,
    },
    StringTest {
        op: String,
        string: Box<Expr>,
        search: Box<Expr>,
    },
    ToString {
        arg: Box<Expr>,
        /// `console.log` of a Number, which prints -0 as "-0" where `String(-0)` is "0".
        #[serde(default, skip_serializing_if = "is_false")]
        console: bool,
    },
    Cast {
        arg: Box<Expr>,
        from: Type,
        to: Type,
        flavor: Flavor,
        /// The message of the guard a checked conversion to a parameter is.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        message: Option<String>,
        /// The place of that guard among the callee's guards.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        order: Option<usize>,
    },
    Abort {
        message: String,
    },
    /// Prints `text`, then is `body`.
    Print {
        text: Box<Expr>,
        body: Box<Expr>,
    },
    /// A JavaScript array of `items`.
    Array {
        items: Vec<Expr>,
    },
    /// The elements of `left`, then those of `right`.
    Append {
        left: Box<Expr>,
        right: Box<Expr>,
    },
    /// The element of `array` at `index`, a Number or an integer; a read
    /// outside the array aborts.
    Index {
        array: Box<Expr>,
        index: Box<Expr>,
    },
    /// The number of elements of `array`, as a Number or an integer.
    Length {
        array: Box<Expr>,
    },
    /// A `Math` or `Number` function of Numbers; `maxOf` and `minOf` fold
    /// `max` and `min` over one array.
    Math {
        op: String,
        args: Vec<Expr>,
    },
    /// No lines printed (see `output`).
    OutNil,
    /// The lines `tail` with `head` printed after them.
    OutCons {
        head: Box<Expr>,
        tail: Box<Expr>,
    },
}

impl Node {
    #[must_use]
    pub const fn kind(&self) -> &'static str {
        match self {
            Self::Lit { .. } => "lit",
            Self::Unit => "unit",
            Self::Var { .. } => "var",
            Self::Call { .. } => "call",
            Self::Ctor { .. } => "ctor",
            Self::Unary { .. } => "unary",
            Self::Binary { .. } => "binary",
            Self::If { .. } => "if",
            Self::Let { .. } => "let",
            Self::Match { .. } => "match",
            Self::StringMap { .. } => "stringMap",
            Self::StringTest { .. } => "stringTest",
            Self::ToString { .. } => "toString",
            Self::Cast { .. } => "cast",
            Self::Abort { .. } => "abort",
            Self::Print { .. } => "print",
            Self::Array { .. } => "array",
            Self::Append { .. } => "append",
            Self::Index { .. } => "index",
            Self::Length { .. } => "length",
            Self::Math { .. } => "math",
            Self::OutNil => "outNil",
            Self::OutCons { .. } => "outCons",
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
    pub default_value: Option<super::surface::SExpr>,
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
    pub effects: Vec<Effect>,
    #[serde(default)]
    pub span: Option<Span>,
    /// Every call of an async function is awaited where it is made.
    #[serde(rename = "sequentialAsync", default, skip_serializing_if = "is_false")]
    pub sequential_async: bool,
}

/// A checked portable-core program.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Program {
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
        // A suffixed name is legal on its own: `end_2` needs none of the quoting of Lean's `«end»`.
        let mut candidate = (self.ident)(name);
        let mut index = 2;
        while self.used.contains(&candidate) {
            candidate = (self.ident)(&format!("{name}_{index}"));
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
