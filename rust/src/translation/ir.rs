//! The checked portable core: every expression carries its type and every
//! operator its source semantics. Emitters consume it; the renaming passes
//! give each target unique, legal local names.
//!
//! The JSON form is the JavaScript runtime's checked program, except that
//! `declarations` is an array and `items` refer to declarations by name.

use std::collections::{BTreeSet, HashMap, HashSet};

use serde::{Deserialize, Serialize};

use super::surface::{BinaryOp, Flavor, Rounding, UnaryOp};
use super::types::Type;
use super::{Language, Span};

/// Arithmetic semantics: exact on unbounded integers, checked (aborting)
/// machine arithmetic, or truncated natural subtraction.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Semantics {
    Exact,
    Checked,
    Truncated,
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
            Node::Lit { .. } | Node::Unit | Node::Var { .. } | Node::Abort { .. } => Vec::new(),
            Node::Call { args, .. } | Node::Ctor { args, .. } => args.iter().collect(),
            Node::Unary { arg, .. } | Node::ToString { arg } | Node::Cast { arg, .. } => vec![arg],
            Node::Binary { left, right, .. } => vec![left, right],
            Node::If {
                cond,
                then,
                otherwise,
            } => vec![cond, then, otherwise],
            Node::Let { value, body, .. } => vec![value, body],
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
            Node::Lit { .. } | Node::Unit | Node::Var { .. } | Node::Abort { .. } => {
                self.node.clone()
            }
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
            Node::ToString { arg } => Node::ToString {
                arg: Box::new(visit(arg)),
            },
            Node::Cast {
                arg,
                from,
                to,
                flavor,
            } => Node::Cast {
                arg: Box::new(visit(arg)),
                from: from.clone(),
                to: to.clone(),
                flavor: *flavor,
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
    ToString {
        arg: Box<Expr>,
    },
    Cast {
        arg: Box<Expr>,
        from: Type,
        to: Type,
        flavor: Flavor,
    },
    Abort {
        message: String,
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
            Self::ToString { .. } => "toString",
            Self::Cast { .. } => "cast",
            Self::Abort { .. } => "abort",
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
    pub guard: Option<bool>,
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
        #[serde(default)]
        span: Option<Span>,
    },
    Assert {
        prop: Prop,
        #[serde(default)]
        span: Option<Span>,
    },
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Main {
    pub effects: Vec<Effect>,
    #[serde(default)]
    pub span: Option<Span>,
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
}

impl Program {
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
        let base = (self.ident)(name);
        let mut candidate = base.clone();
        let mut index = 2;
        while self.used.contains(&candidate) {
            candidate = format!("{base}_{index}");
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

/// Renames every binder in an expression; `env` maps source names to target names.
///
/// # Panics
/// On a variable the environment does not bind, which a checked program never has.
pub fn rename_expr(expr: &Expr, env: &Env, scope: &mut Scope<'_>) -> Expr {
    match &expr.node {
        Node::Var { name } => {
            let renamed = env
                .get(name)
                .unwrap_or_else(|| panic!("unbound variable {name}"));
            Expr {
                node: Node::Var {
                    name: renamed.clone(),
                },
                ..expr.clone()
            }
        }
        Node::Let { name, value, body } => {
            let value = rename_expr(value, env, scope);
            let fresh = scope.fresh(name);
            let mut inner = env.clone();
            inner.insert(name.clone(), fresh.clone());
            let body = rename_expr(body, &inner, scope);
            Expr {
                node: Node::Let {
                    name: fresh,
                    value: Box::new(value),
                    body: Box::new(body),
                },
                ..expr.clone()
            }
        }
        Node::Match { scrutinee, cases } => {
            let scrutinee = rename_expr(scrutinee, env, scope);
            let cases = cases
                .iter()
                .map(|kase| {
                    let mut inner = env.clone();
                    let pattern = match &kase.pattern {
                        Pattern::NatSucc { name } => {
                            let fresh = scope.fresh(name);
                            inner.insert(name.clone(), fresh.clone());
                            Pattern::NatSucc { name: fresh }
                        }
                        Pattern::Bind { name } => {
                            let fresh = scope.fresh(name);
                            inner.insert(name.clone(), fresh.clone());
                            Pattern::Bind { name: fresh }
                        }
                        Pattern::Ctor { data, ctor, binds } => Pattern::Ctor {
                            data: data.clone(),
                            ctor: ctor.clone(),
                            binds: binds
                                .iter()
                                .map(|bind| {
                                    bind.as_ref().map(|bind| {
                                        let fresh = scope.fresh(bind);
                                        inner.insert(bind.clone(), fresh.clone());
                                        fresh
                                    })
                                })
                                .collect(),
                        },
                        other => other.clone(),
                    };
                    Case {
                        pattern,
                        body: rename_expr(&kase.body, &inner, scope),
                    }
                })
                .collect();
            Expr {
                node: Node::Match {
                    scrutinee: Box::new(scrutinee),
                    cases,
                },
                ..expr.clone()
            }
        }
        _ => expr.map_children(&mut |child| rename_expr(child, env, scope)),
    }
}

pub fn rename_prop(prop: &Prop, env: &Env, scope: &mut Scope<'_>) -> Prop {
    let both = |left: &Prop, right: &Prop, scope: &mut Scope<'_>| {
        let left = rename_prop(left, env, scope);
        (Box::new(left), Box::new(rename_prop(right, env, scope)))
    };
    match prop {
        Prop::Forall { binders, body } => {
            let mut inner = env.clone();
            let binders = binders
                .iter()
                .map(|binder| {
                    let name = scope.fresh(&binder.name);
                    inner.insert(binder.name.clone(), name.clone());
                    Binder {
                        name,
                        ty: binder.ty.clone(),
                    }
                })
                .collect();
            Prop::Forall {
                binders,
                body: Box::new(rename_prop(body, &inner, scope)),
            }
        }
        Prop::And { left, right } => {
            let (left, right) = both(left, right, scope);
            Prop::And { left, right }
        }
        Prop::Or { left, right } => {
            let (left, right) = both(left, right, scope);
            Prop::Or { left, right }
        }
        Prop::Implies { left, right } => {
            let (left, right) = both(left, right, scope);
            Prop::Implies { left, right }
        }
        Prop::Not { arg } => Prop::Not {
            arg: Box::new(rename_prop(arg, env, scope)),
        },
        Prop::Bool { expr } => Prop::Bool {
            expr: rename_expr(expr, env, scope),
        },
        other => {
            let (op, comparison) = other.comparison().expect("comparison");
            let left = rename_expr(&comparison.left, env, scope);
            let right = rename_expr(&comparison.right, env, scope);
            Prop::compare(
                op,
                Comparison {
                    left,
                    right,
                    domain: comparison.domain.clone(),
                },
            )
        }
    }
}

/// A function's parameters and body with target local names.
pub fn rename_function(
    entry: &FnDecl,
    ident: &dyn Fn(&str) -> String,
    reserved: &BTreeSet<String>,
) -> (Vec<Param>, Expr) {
    let mut scope = Scope::new(ident, reserved);
    let mut env = Env::new();
    let params = entry
        .params
        .iter()
        .map(|param| {
            let name = scope.fresh(&param.name);
            env.insert(param.name.clone(), name.clone());
            Param {
                name,
                ..param.clone()
            }
        })
        .collect();
    let body = rename_expr(&entry.body, &env, &mut scope);
    (params, body)
}

/// A theorem's binders, proposition and proof plan with target local names.
pub fn rename_theorem(
    entry: &TheoremDecl,
    ident: &dyn Fn(&str) -> String,
    reserved: &BTreeSet<String>,
) -> (Vec<Binder>, Prop, Plan) {
    let mut scope = Scope::new(ident, reserved);
    let mut env = Env::new();
    let binders = entry
        .binders
        .iter()
        .map(|binder| {
            let name = scope.fresh(&binder.name);
            env.insert(binder.name.clone(), name.clone());
            Binder {
                name,
                ty: binder.ty.clone(),
            }
        })
        .collect();
    let prop = rename_prop(&entry.prop, &env, &mut scope.child());
    let plan = rename_plan(&entry.proof.plan, &env, &scope);
    (binders, prop, plan)
}

fn rename_plan(plan: &Plan, env: &Env, scope: &Scope<'_>) -> Plan {
    let split = match plan {
        Plan::Close { hints } => {
            return Plan::Close {
                hints: Hints {
                    hyps: hints
                        .hyps
                        .iter()
                        .map(|name| env.get(name).unwrap_or(name).clone())
                        .collect(),
                    ..hints.clone()
                },
            };
        }
        Plan::Induction(split) | Plan::Cases(split) => split,
    };
    let renamed = Split {
        // A split variable is always a theorem binder, so the environment names it.
        variable: env.get(&split.variable).cloned().unwrap_or_default(),
        ty: split.ty.clone(),
        cases: split
            .cases
            .iter()
            .map(|kase| {
                let mut inner = env.clone();
                let mut case_scope = scope.child();
                let mut rename = |name: &String| {
                    let fresh = case_scope.fresh(name);
                    inner.insert(name.clone(), fresh.clone());
                    fresh
                };
                let fields = kase.fields.iter().map(&mut rename).collect();
                let ihs = kase.ihs.iter().map(&mut rename).collect();
                PlanCase {
                    ctor: kase.ctor.clone(),
                    fields,
                    ihs,
                    recursive: kase.recursive.clone(),
                    plan: rename_plan(&kase.plan, &inner, &case_scope),
                }
            })
            .collect(),
    };
    match plan {
        Plan::Induction(_) => Plan::Induction(renamed),
        _ => Plan::Cases(renamed),
    }
}

/// The main effects with target local names.
pub fn rename_main(
    main: &Main,
    ident: &dyn Fn(&str) -> String,
    reserved: &BTreeSet<String>,
) -> Vec<Effect> {
    let mut scope = Scope::new(ident, reserved);
    let mut env = Env::new();
    main.effects
        .iter()
        .map(|effect| match effect {
            Effect::Let { name, value, span } => {
                let value = rename_expr(value, &env, &mut scope);
                let fresh = scope.fresh(name);
                env.insert(name.clone(), fresh.clone());
                Effect::Let {
                    name: fresh,
                    value,
                    span: *span,
                }
            }
            Effect::Print { expr, span } => Effect::Print {
                expr: rename_expr(expr, &env, &mut scope),
                span: *span,
            },
            Effect::Assert { prop, span } => Effect::Assert {
                prop: rename_prop(prop, &env, &mut scope),
                span: *span,
            },
        })
        .collect()
}

/// True when any expression of the given kind occurs in the tree.
#[must_use]
pub fn contains_kind(expr: &Expr, kind: &str) -> bool {
    expr.kind() == kind
        || expr
            .children()
            .into_iter()
            .any(|child| contains_kind(child, kind))
}

/// True when any expression of the given kind occurs in the proposition.
#[must_use]
pub fn prop_contains_kind(prop: &Prop, kind: &str) -> bool {
    prop_exprs(prop)
        .into_iter()
        .any(|expr| contains_kind(expr, kind))
}

/// The expressions of a proposition, left to right.
#[must_use]
pub fn prop_exprs(prop: &Prop) -> Vec<&Expr> {
    match prop {
        Prop::Forall { body, .. } => prop_exprs(body),
        Prop::And { left, right } | Prop::Or { left, right } | Prop::Implies { left, right } => {
            let mut exprs = prop_exprs(left);
            exprs.extend(prop_exprs(right));
            exprs
        }
        Prop::Not { arg } => prop_exprs(arg),
        Prop::Bool { expr } => vec![expr],
        other => {
            let (_, comparison) = other.comparison().expect("comparison");
            vec![&comparison.left, &comparison.right]
        }
    }
}
