//! The portable-core checker.
//!
//! Resolves names, checks types, and fixes the exact operator semantics of a
//! surface program produced by one of the language frontends. The result is
//! the portable-core program the emitters consume: every expression carries
//! its type and every operator carries the source language's semantics, so a
//! target can never silently substitute its own (for example, Lean's Euclidean
//! `Int` division for JavaScript's truncating `BigInt` division).
//!
//! Mirrors `js/src/translation/check.js`, including the order in which fresh
//! names are drawn, so both runtimes produce the same checked program.

use std::collections::{HashMap, HashSet};

use super::decimal::Decimal;
use super::diagnostics::{type_error, unsupported, Result};
use super::ir::{
    Binder, ByZero, Case, Comparison, Ctor, DataDecl, Decl, Effect, Expr, Field, FnDecl, Hints,
    Item, LitValue, Main, Node, Param, Pattern, Plan, Program, Proof, Prop, Semantics, TheoremDecl,
};
use super::proof::{normalise_proof, ProofContext, ProofName, TheoremHead};
use super::surface::{
    BinaryOp, Flavor, Rounding, SCase, SCasePattern, SEffect, SExpr, SItem, SMain, SNode, SPattern,
    SPatternNode, SProgram, SProp, SPropNode, SRow, ShowStyle, UnaryOp,
};
use super::types::{data, fixed, fixed_bounds, Type, BOOL, INT, NAT, STRING, UNIT};
use super::{Language, Span};

mod expressions;
mod items;
mod matches;
mod recursion;
pub use self::recursion::*;

/// Checks a surface program into the portable core.
///
/// # Errors
/// On name, type and exhaustiveness errors, and on constructs outside the portable core.
pub fn check_program(surface: &SProgram) -> Result<Program> {
    Checker::new(surface.language).program(surface)
}

/// Inserts the source language's implicit conversions, which only JavaScript has.
///
/// # Errors
/// When the value cannot take the type.
pub fn coerce(
    value: Expr,
    ty: &Type,
    language: Language,
    span: Option<Span>,
    flavor: Option<Flavor>,
) -> Result<Expr> {
    if value.ty.is_literal() {
        return Err(type_error("untyped literal", span));
    }
    if value.ty.same(ty) {
        return Ok(value);
    }
    if matches!(value.node, Node::Abort { .. }) {
        return Ok(Expr {
            ty: ty.clone(),
            ..value
        });
    }
    if language == Language::JavaScript && value.ty == Type::Nat && *ty == Type::Int {
        return cast_to(value, &INT, Flavor::Exact, span);
    }
    if language == Language::JavaScript && value.ty == Type::Int && *ty == Type::Nat {
        return cast_to(value, &NAT, flavor.unwrap_or(Flavor::Checked), span);
    }
    Err(type_error(
        format!("expected {} but found {}", ty.key(), value.ty.key()),
        span,
    ))
}

fn cast_to(arg: Expr, to: &Type, flavor: Flavor, span: Option<Span>) -> Result<Expr> {
    if arg.ty.same(to) {
        return Ok(arg);
    }
    let from = arg.ty.clone();
    let cast = |arg: Expr, flavor: Flavor| {
        Expr::new(
            Node::Cast {
                arg: Box::new(arg),
                from: from.clone(),
                to: to.clone(),
                flavor,
            },
            to.clone(),
        )
    };
    match (&from, to) {
        (Type::Nat, Type::Int) => Ok(cast(arg, Flavor::Exact)),
        (Type::Int, Type::Nat) => {
            if !matches!(flavor, Flavor::Checked | Flavor::Clamp) {
                return Err(type_error(
                    "int to nat conversion needs checked or clamp semantics",
                    span,
                ));
            }
            Ok(cast(arg, flavor))
        }
        (
            Type::Fixed {
                bits: from_bits,
                signed: from_signed,
            },
            Type::Fixed { bits, signed },
        ) => {
            let (source_min, source_max) = fixed_bounds(*from_bits, *from_signed);
            let (target_min, target_max) = fixed_bounds(*bits, *signed);
            if source_min >= target_min && source_max <= target_max {
                return Ok(cast(arg, Flavor::Exact));
            }
            Err(unsupported(
                "narrowing integer cast",
                &format!(
                    "{} as {} wraps in Rust and has no portable encoding yet",
                    from.key(),
                    to.key()
                ),
                span,
            ))
        }
        _ => Err(type_error(
            format!("cannot convert {} to {}", from.key(), to.key()),
            span,
        )),
    }
}

/// A name in a module scope: a declaration or a data constructor.
#[derive(Clone, Debug)]
enum Entry {
    Decl(String),
    Ctor { data: String, name: String },
}

#[derive(Default)]
struct Scope {
    path: Vec<String>,
    names: HashMap<String, Entry>,
    /// Submodule name → full name.
    modules: HashMap<String, String>,
}

/// For a local, the constructor it is known to have matched and the locals naming its fields.
#[derive(Clone, Debug)]
struct Fact {
    ctor: String,
    binds: Vec<String>,
}

#[derive(Clone, Debug, Default)]
struct Env {
    vars: HashMap<String, Type>,
    known: HashMap<String, Fact>,
}

impl Env {
    fn has(&self, name: &str) -> bool {
        self.vars.contains_key(name)
    }

    /// Binds a local, forgetting constructor facts the new binding shadows.
    fn bind_local(&mut self, name: &str, ty: Type) {
        self.vars.insert(name.to_owned(), ty);
        self.known.retain(|variable, fact| {
            variable != name && !fact.binds.iter().any(|bind| bind == name)
        });
    }

    /// The single-segment local a surface expression names, if it is one.
    fn local<'a>(&self, expr: &'a SExpr) -> Option<&'a str> {
        expr.simple_name().filter(|name| self.has(name))
    }
}

#[derive(Clone, Debug)]
struct Column {
    name: String,
    ty: Type,
}

/// Natural numeral patterns above this unfold to equality tests rather than successor chains.
const NAT_PATTERN_UNFOLD: i128 = 16;

/// A match-compilation pattern: surface patterns are normalised against their column type.
#[derive(Clone, Debug)]
enum NPat {
    Surface(SPattern),
    Wild,
    Bind(String),
    Nat { succ: bool, args: Vec<Self> },
    Data { ctor: String, args: Vec<Self> },
    Lit { value: SExpr, key: String },
}

impl NPat {
    const fn irrefutable(&self) -> bool {
        matches!(self, Self::Wild | Self::Bind(_))
    }

    fn ctor(&self) -> Option<&str> {
        match self {
            Self::Nat { succ, .. } => Some(if *succ { "succ" } else { "zero" }),
            Self::Data { ctor, .. } => Some(ctor),
            _ => None,
        }
    }

    fn args(&self) -> &[Self] {
        match self {
            Self::Nat { args, .. } | Self::Data { args, .. } => args,
            _ => &[],
        }
    }

    /// A natural literal pattern's value.
    fn nat_value(&self) -> Option<Decimal> {
        match self {
            Self::Lit {
                value:
                    SExpr {
                        node: SNode::Num { value, .. },
                        ..
                    },
                ..
            } => Decimal::parse(value),
            _ => None,
        }
    }

    fn lit_key(&self) -> Option<&str> {
        match self {
            Self::Lit { key, .. } => Some(key),
            _ => None,
        }
    }
}

#[derive(Clone, Debug)]
struct Row {
    patterns: Vec<NPat>,
    body: SExpr,
    binds: Vec<(String, String)>,
}

const fn comparison(op: BinaryOp) -> bool {
    matches!(
        op,
        BinaryOp::Eq | BinaryOp::Ne | BinaryOp::Lt | BinaryOp::Le | BinaryOp::Gt | BinaryOp::Ge
    )
}

#[allow(clippy::too_many_arguments)]
const fn binary_node(
    op: BinaryOp,
    left: Box<Expr>,
    right: Box<Expr>,
    domain: Option<Type>,
    semantics: Option<Semantics>,
    rounding: Option<Rounding>,
    by_zero: Option<ByZero>,
) -> Node {
    Node::Binary {
        op,
        left,
        right,
        domain,
        semantics,
        rounding,
        by_zero,
    }
}

fn plain_binary(op: BinaryOp, left: Expr, right: Expr, ty: Type) -> Expr {
    Expr::new(
        binary_node(op, Box::new(left), Box::new(right), None, None, None, None),
        ty,
    )
}

fn text_lit(ty: Type, value: impl Into<String>) -> Expr {
    Expr::lit(ty, LitValue::Text(value.into()))
}

fn surface_let(name: &str, value: &str, body: SExpr, span: Option<Span>) -> SExpr {
    SExpr::new(
        SNode::Let {
            name: name.to_owned(),
            ty: None,
            value: Box::new(SExpr::name(value)),
            body: Box::new(body),
        },
        span,
    )
}

struct Checker {
    language: Language,
    /// Declaration full names in declaration order.
    order: Vec<String>,
    decls: HashMap<String, Decl>,
    /// Functions whose parameter and result types are resolved.
    signatures: HashSet<String>,
    fresh: usize,
    modules: HashMap<String, Scope>,
}

impl Checker {
    fn new(language: Language) -> Self {
        let mut modules = HashMap::new();
        modules.insert(String::new(), Scope::default());
        Self {
            language,
            order: Vec::new(),
            decls: HashMap::new(),
            signatures: HashSet::new(),
            fresh: 0,
            modules,
        }
    }

    fn program(mut self, surface: &SProgram) -> Result<Program> {
        self.declare(&surface.items, &[])?;
        let items = self.check_items(&surface.items)?;
        let main = match &surface.main {
            Some(main) => Some(self.check_main(main)?),
            None => None,
        };
        for name in &self.order {
            if let Some(Decl::Fn(entry)) = self.decls.get_mut(name) {
                entry.body = reuse_successors(&entry.body, &HashMap::new());
            }
        }
        let mut declarations: Vec<Decl> = self
            .order
            .iter()
            .filter_map(|name| self.decls.remove(name))
            .collect();
        analyse_recursion(&mut declarations);
        Ok(Program {
            schema_version: 1,
            source_language: self.language,
            items,
            main,
            declarations,
        })
    }

    fn scope_mut(&mut self, path: &[String]) -> &mut Scope {
        self.modules
            .get_mut(&path.join("."))
            .unwrap_or_else(|| unreachable!("module {} is declared", path.join(".")))
    }
}

/// Proof names resolve from the theorem's module.
struct ProofScope<'a> {
    checker: &'a Checker,
    path: &'a [String],
    span: Option<Span>,
}

impl ProofContext for ProofScope<'_> {
    fn lookup(&self, name: &str) -> Result<Option<ProofName>> {
        let segments: Vec<String> = name.split('.').map(str::to_owned).collect();
        Ok(
            match self.checker.lookup(&segments, self.path, self.span)? {
                None => None,
                Some(Entry::Ctor { .. }) => Some(ProofName::Other),
                Some(Entry::Decl(full_name)) => Some(match self.checker.decls.get(&full_name) {
                    Some(Decl::Fn(_)) => ProofName::Function(full_name),
                    Some(Decl::Theorem(_)) => ProofName::Theorem(full_name),
                    _ => ProofName::Other,
                }),
            },
        )
    }

    fn ctors(&self, data: &str) -> Vec<Ctor> {
        self.checker
            .data_decl(data)
            .map(|decl| decl.ctors.clone())
            .unwrap_or_default()
    }
}

fn full(path: &[String], name: &str) -> String {
    let mut parts = path.to_vec();
    parts.push(name.to_owned());
    parts.join(".")
}

fn flatten<'a>(items: &'a [SItem], path: &[String], out: &mut Vec<(&'a SItem, Vec<String>)>) {
    for item in items {
        if let SItem::Module(module) = item {
            let mut inner = path.to_vec();
            inner.push(module.name.clone());
            flatten(&module.items, &inner, out);
        } else {
            out.push((item, path.to_vec()));
        }
    }
}

fn build_items(items: &[SItem], path: &[String]) -> Vec<Item> {
    items
        .iter()
        .map(|item| match item {
            SItem::Module(module) => {
                let mut inner = path.to_vec();
                inner.push(module.name.clone());
                Item::Module {
                    name: module.name.clone(),
                    full_name: inner.join("."),
                    items: build_items(&module.items, &inner),
                    span: module.span,
                }
            }
            other => Item::Decl {
                full_name: full(path, other.name()),
            },
        })
        .collect()
}

const fn op_name(op: BinaryOp) -> &'static str {
    match op {
        BinaryOp::Add => "add",
        BinaryOp::Sub => "sub",
        BinaryOp::Mul => "mul",
        BinaryOp::Div => "div",
        BinaryOp::Rem => "rem",
        BinaryOp::Eq => "eq",
        BinaryOp::Ne => "ne",
        BinaryOp::Lt => "lt",
        BinaryOp::Le => "le",
        BinaryOp::Gt => "gt",
        BinaryOp::Ge => "ge",
        BinaryOp::And => "and",
        BinaryOp::Or => "or",
        BinaryOp::Concat => "concat",
        BinaryOp::Plus => "plus",
    }
}

fn arithmetic_semantics(
    language: Language,
    op: BinaryOp,
    ty: &Type,
    rounding: Option<Rounding>,
) -> (Semantics, Option<Rounding>, Option<ByZero>) {
    let division = matches!(op, BinaryOp::Div | BinaryOp::Rem);
    if matches!(ty, Type::Fixed { .. }) {
        // Rust integer arithmetic with overflow checks: `/` and `%` truncate,
        // `div_euclid` and `rem_euclid` round Euclidean; all panic on zero.
        return if division {
            (
                Semantics::Checked,
                Some(rounding.unwrap_or(Rounding::Trunc)),
                Some(ByZero::Abort),
            )
        } else {
            (Semantics::Checked, None, None)
        };
    }
    if op == BinaryOp::Sub {
        let semantics = if *ty == Type::Nat {
            Semantics::Truncated
        } else {
            Semantics::Exact
        };
        return (semantics, None, None);
    }
    if !division {
        return (Semantics::Exact, None, None);
    }
    if language == Language::JavaScript {
        return (Semantics::Exact, Some(Rounding::Trunc), Some(ByZero::Abort));
    }
    // Lean and Rocq division is total: x / 0 = 0 and x % 0 = x.
    if *ty == Type::Nat {
        return (Semantics::Exact, Some(Rounding::Trunc), Some(ByZero::Total));
    }
    let default = if language == Language::Lean {
        Rounding::Euclid
    } else {
        Rounding::Floor
    };
    (
        Semantics::Exact,
        Some(rounding.unwrap_or(default)),
        Some(ByZero::Total),
    )
}
