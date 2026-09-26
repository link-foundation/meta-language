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

    fn declare(&mut self, items: &[SItem], path: &[String]) -> Result<()> {
        for item in items {
            let name = item.name().to_owned();
            let full_name = path
                .iter()
                .cloned()
                .chain(std::iter::once(name.clone()))
                .collect::<Vec<_>>()
                .join(".");
            let span = item.span();
            if !matches!(item, SItem::Module(_)) && self.scope_mut(path).names.contains_key(&name) {
                return Err(type_error(
                    format!("duplicate declaration {full_name}"),
                    span,
                ));
            }
            if name.starts_with("ml_") {
                return Err(unsupported(
                    "reserved identifier",
                    &format!("{name} uses the translator's reserved ml_ prefix"),
                    span,
                ));
            }
            let module_path = path.to_vec();
            let decl = match item {
                SItem::Module(module) => {
                    // Modules live in their own namespace, so Lean's `namespace Tree` may accompany `inductive Tree`.
                    let mut inner = path.to_vec();
                    inner.push(name.clone());
                    if !self.scope_mut(path).modules.contains_key(&name) {
                        self.scope_mut(path)
                            .modules
                            .insert(name.clone(), full_name.clone());
                        self.modules.insert(
                            full_name.clone(),
                            Scope {
                                path: inner.clone(),
                                ..Scope::default()
                            },
                        );
                    }
                    self.declare(&module.items, &inner)?;
                    continue;
                }
                SItem::Data(_) => Decl::Data(DataDecl {
                    name: name.clone(),
                    ctors: Vec::new(),
                    span,
                    full_name: full_name.clone(),
                    module_path,
                }),
                SItem::Fn(_) => Decl::Fn(FnDecl {
                    name: name.clone(),
                    params: Vec::new(),
                    ret: UNIT,
                    body: Expr::new(Node::Unit, UNIT),
                    span,
                    full_name: full_name.clone(),
                    module_path,
                    recursive: false,
                    decreasing: None,
                    mutual: false,
                }),
                SItem::Theorem(theorem) => Decl::Theorem(TheoremDecl {
                    name: name.clone(),
                    binders: Vec::new(),
                    prop: Prop::Bool {
                        expr: Expr::new(Node::Unit, UNIT),
                    },
                    proof: Proof {
                        plan: Plan::Close {
                            hints: Hints::default(),
                        },
                        source: String::new(),
                        source_language: theorem.proof.source_language,
                    },
                    span,
                    full_name: full_name.clone(),
                    module_path,
                }),
            };
            self.scope_mut(path)
                .names
                .insert(name.clone(), Entry::Decl(full_name.clone()));
            self.decls.insert(full_name.clone(), decl);
            self.order.push(full_name.clone());
            if let SItem::Data(item) = item {
                let rocq = self.language == Language::Rocq;
                let scope = self.scope_mut(path);
                for ctor in &item.ctors {
                    let key = format!("{name}.{}", ctor.name);
                    if scope.names.contains_key(&key) {
                        return Err(type_error(
                            format!("duplicate constructor {}", ctor.name),
                            span,
                        ));
                    }
                    let entry = Entry::Ctor {
                        data: full_name.clone(),
                        name: ctor.name.clone(),
                    };
                    scope.names.insert(key, entry.clone());
                    if rocq {
                        if scope.names.contains_key(&ctor.name) {
                            return Err(type_error(
                                format!("duplicate declaration {}", ctor.name),
                                span,
                            ));
                        }
                        scope.names.insert(ctor.name.clone(), entry);
                    }
                }
            }
        }
        Ok(())
    }

    fn check_items(&mut self, items: &[SItem]) -> Result<Vec<Item>> {
        // Data layouts and signatures are resolved program-wide before any body,
        // so declarations may refer to each other in any order.
        let mut flat = Vec::new();
        flatten(items, &[], &mut flat);
        for (item, path) in &flat {
            let SItem::Data(item) = item else { continue };
            let mut ctors = Vec::new();
            for ctor in &item.ctors {
                let mut fields = Vec::new();
                for (index, field) in ctor.fields.iter().enumerate() {
                    fields.push(Field {
                        name: field
                            .name
                            .clone()
                            .unwrap_or_else(|| format!("field{index}")),
                        ty: self.resolve_type(Some(&field.ty), path, item.span)?,
                    });
                }
                ctors.push(Ctor {
                    name: ctor.name.clone(),
                    fields,
                });
            }
            if let Some(Decl::Data(entry)) = self.decls.get_mut(&full(path, &item.name)) {
                entry.ctors = ctors;
            }
        }
        for (item, path) in &flat {
            let SItem::Fn(item) = item else { continue };
            let mut params = Vec::new();
            for param in &item.params {
                params.push(Param {
                    name: param.name.clone(),
                    ty: self.resolve_type(param.ty.as_ref(), path, item.span)?,
                    guard: param.guard,
                });
            }
            let ret = self.resolve_type(item.ret.as_ref(), path, item.span)?;
            let name = full(path, &item.name);
            if let Some(Decl::Fn(entry)) = self.decls.get_mut(&name) {
                entry.params = params;
                entry.ret = ret;
            }
            self.signatures.insert(name);
        }
        for (item, path) in &flat {
            match item {
                SItem::Fn(item) => {
                    let name = full(path, &item.name);
                    let Some(Decl::Fn(entry)) = self.decls.get(&name) else {
                        continue;
                    };
                    let ret = entry.ret.clone();
                    let env = Env {
                        vars: entry
                            .params
                            .iter()
                            .map(|param| (param.name.clone(), param.ty.clone()))
                            .collect(),
                        known: HashMap::new(),
                    };
                    let body = self.expr(&item.body, &env, path, Some(&ret), false)?;
                    let body = self.coerce(body, &ret, item.span)?;
                    if let Some(Decl::Fn(entry)) = self.decls.get_mut(&name) {
                        entry.body = body;
                    }
                }
                SItem::Theorem(item) => {
                    let name = full(path, &item.name);
                    let mut binders = Vec::new();
                    for binder in &item.binders {
                        binders.push(Binder {
                            name: binder.name.clone(),
                            ty: self.resolve_type(binder.ty.as_ref(), path, item.span)?,
                        });
                    }
                    let env = Env {
                        vars: binders
                            .iter()
                            .map(|binder| (binder.name.clone(), binder.ty.clone()))
                            .collect(),
                        known: HashMap::new(),
                    };
                    let mut prop = self.prop(&item.prop, &env, path)?;
                    // `theorem t : ∀ n, P n` and `theorem t (n) : P n` state the same
                    // proposition; hoisting leading binders gives every target one shape.
                    while let Prop::Forall {
                        binders: inner,
                        body,
                    } = prop
                    {
                        binders.extend(inner);
                        prop = *body;
                    }
                    let context = ProofScope {
                        checker: self,
                        path,
                        span: item.span,
                    };
                    let proof = normalise_proof(
                        &item.proof,
                        &TheoremHead {
                            full_name: &name,
                            binders: &binders,
                        },
                        &context,
                    )?;
                    if let Some(Decl::Theorem(entry)) = self.decls.get_mut(&name) {
                        entry.binders = binders;
                        entry.prop = prop;
                        entry.proof = proof;
                    }
                }
                SItem::Data(_) | SItem::Module(_) => {}
            }
        }
        Ok(build_items(items, &[]))
    }

    fn resolve_type(&self, ty: Option<&Type>, path: &[String], span: Option<Span>) -> Result<Type> {
        let Some(ty) = ty else {
            return Err(type_error("missing type annotation", span));
        };
        let Type::Named {
            path: segments,
            span: type_span,
        } = ty
        else {
            return Ok(ty.clone());
        };
        if let Some(Entry::Decl(name)) = self.lookup(segments, path, span)? {
            if matches!(self.decls.get(&name), Some(Decl::Data(_))) {
                return Ok(data(name));
            }
        }
        Err(type_error(
            format!("unknown type {}", segments.join(".")),
            type_span.or(span),
        ))
    }

    fn lookup(
        &self,
        segments: &[String],
        path: &[String],
        span: Option<Span>,
    ) -> Result<Option<Entry>> {
        let first = segments.first().map(String::as_str);
        let mut names = segments;
        let mut start = path;
        match first {
            Some("crate") => {
                names = &names[1..];
                start = &[];
            }
            Some("self") => names = &names[1..],
            _ => {
                let mut up = 0;
                while names.first().map(String::as_str) == Some("super") {
                    names = &names[1..];
                    up += 1;
                }
                if up > path.len() {
                    return Err(type_error("super beyond crate root", span));
                }
                if up > 0 {
                    start = &path[..path.len() - up];
                }
            }
        }
        for depth in (0..=start.len()).rev() {
            if let Some(found) = self.lookup_from(&start[..depth], names) {
                return Ok(Some(found));
            }
            if matches!(first, Some("crate" | "self" | "super")) {
                break;
            }
        }
        Ok(None)
    }

    fn lookup_from(&self, module_path: &[String], names: &[String]) -> Option<Entry> {
        let mut scope = self.modules.get(&module_path.join("."))?;
        for index in 0..names.len().saturating_sub(1) {
            let rest = names[index..].join(".");
            if index == names.len() - 2 {
                if let Some(entry) = scope.names.get(&rest) {
                    return Some(entry.clone());
                }
            }
            let module = scope.modules.get(&names[index])?;
            scope = self.modules.get(module)?;
        }
        let last = names.last()?;
        if let Some(entry) = scope.names.get(last) {
            return Some(entry.clone());
        }
        // Inside `namespace T`, the constructors of a sibling `inductive T` are in scope.
        if let Some((sibling, parent_path)) = scope.path.split_last() {
            let parent = self.modules.get(&parent_path.join("."))?;
            if let Some(Entry::Decl(name)) = parent.names.get(sibling) {
                if let Some(Decl::Data(decl)) = self.decls.get(name) {
                    return parent.names.get(&format!("{}.{last}", decl.name)).cloned();
                }
            }
        }
        None
    }

    fn data_decl(&self, name: &str) -> Result<&DataDecl> {
        match self.decls.get(name) {
            Some(Decl::Data(decl)) => Ok(decl),
            _ => Err(type_error(format!("unknown type {name}"), None)),
        }
    }

    fn check_main(&mut self, main: &SMain) -> Result<Main> {
        let mut env = Env::default();
        let mut effects = Vec::new();
        for effect in &main.effects {
            match effect {
                SEffect::Print { expr, style, span } => effects.push(Effect::Print {
                    expr: self.show(expr, &env, &[], *style)?,
                    span: *span,
                }),
                SEffect::Let {
                    name,
                    ty,
                    value,
                    span,
                } => {
                    let expected = match ty {
                        Some(ty) => Some(self.resolve_type(Some(ty), &[], *span)?),
                        None => None,
                    };
                    let mut value = self.expr(value, &env, &[], expected.as_ref(), false)?;
                    if let Some(expected) = &expected {
                        value = self.coerce(value, expected, *span)?;
                    }
                    env.bind_local(name, value.ty.clone());
                    effects.push(Effect::Let {
                        name: name.clone(),
                        value,
                        span: *span,
                    });
                }
                SEffect::Assert { prop, span } => effects.push(Effect::Assert {
                    prop: self.prop(prop, &env, &[])?,
                    span: *span,
                }),
            }
        }
        Ok(Main {
            effects,
            span: main.span,
        })
    }

    fn show(&mut self, expr: &SExpr, env: &Env, path: &[String], style: ShowStyle) -> Result<Expr> {
        let value = self.expr(expr, env, path, None, false)?;
        match value.ty {
            Type::String => return Ok(value),
            Type::Data { .. } | Type::Unit => {
                return Err(unsupported(
                    "output of structured values",
                    &format!(
                        "printing a {} value has no portable textual form",
                        value.ty.key()
                    ),
                    expr.span,
                ))
            }
            _ => {}
        }
        let console_bigint =
            style == ShowStyle::JsConsole && matches!(value.ty, Type::Int | Type::Nat);
        let text = Expr::new(
            Node::ToString {
                arg: Box::new(value),
            },
            STRING,
        );
        if console_bigint {
            // Node's console.log prints BigInt values with their `n` suffix.
            return Ok(plain_binary(
                BinaryOp::Concat,
                text,
                text_lit(STRING, "n"),
                STRING,
            ));
        }
        Ok(text)
    }

    fn prop(&mut self, prop: &SProp, env: &Env, path: &[String]) -> Result<Prop> {
        let pair =
            |checker: &mut Self, left: &SProp, right: &SProp| -> Result<(Box<Prop>, Box<Prop>)> {
                let left = checker.prop(left, env, path)?;
                let right = checker.prop(right, env, path)?;
                Ok((Box::new(left), Box::new(right)))
            };
        Ok(match &prop.node {
            SPropNode::Forall { binders, body } => {
                let mut checked = Vec::new();
                for binder in binders {
                    checked.push(Binder {
                        name: binder.name.clone(),
                        ty: self.resolve_type(binder.ty.as_ref(), path, prop.span)?,
                    });
                }
                let mut inner = env.clone();
                for binder in &checked {
                    inner.bind_local(&binder.name, binder.ty.clone());
                }
                let body = self.prop(body, &inner, path)?;
                Prop::Forall {
                    binders: checked,
                    body: Box::new(body),
                }
            }
            SPropNode::And { left, right } => {
                let (left, right) = pair(self, left, right)?;
                Prop::And { left, right }
            }
            SPropNode::Or { left, right } => {
                let (left, right) = pair(self, left, right)?;
                Prop::Or { left, right }
            }
            SPropNode::Implies { left, right } => {
                let (left, right) = pair(self, left, right)?;
                Prop::Implies { left, right }
            }
            SPropNode::Not { arg } => Prop::Not {
                arg: Box::new(self.prop(arg, env, path)?),
            },
            SPropNode::Bool { expr } => {
                let value = self.expr(expr, env, path, Some(&BOOL), false)?;
                Prop::Bool {
                    expr: coerce(value, &BOOL, self.language, prop.span, None)?,
                }
            }
            SPropNode::Eq(pair)
            | SPropNode::Ne(pair)
            | SPropNode::Lt(pair)
            | SPropNode::Le(pair)
            | SPropNode::Gt(pair)
            | SPropNode::Ge(pair) => {
                let op = match &prop.node {
                    SPropNode::Eq(_) => BinaryOp::Eq,
                    SPropNode::Ne(_) => BinaryOp::Ne,
                    SPropNode::Lt(_) => BinaryOp::Lt,
                    SPropNode::Le(_) => BinaryOp::Le,
                    SPropNode::Gt(_) => BinaryOp::Gt,
                    _ => BinaryOp::Ge,
                };
                let (left, right) = self.operands(&pair.left, &pair.right, env, path, prop.span)?;
                if !matches!(op, BinaryOp::Eq | BinaryOp::Ne) && !left.ty.is_numeric() {
                    return Err(type_error(
                        format!("ordering on {}", left.ty.key()),
                        prop.span,
                    ));
                }
                if pair.reference && matches!(left.ty, Type::Data { .. } | Type::Unit) {
                    return Err(unsupported(
                        "identity comparison of objects",
                        "JavaScript compares objects by identity; use assert.deepStrictEqual",
                        prop.span,
                    ));
                }
                // Equality propositions over data are structural; executable targets get a generated equality.
                let domain = left.ty.clone();
                Prop::compare(
                    op,
                    Comparison {
                        left,
                        right,
                        domain,
                    },
                )
            }
        })
    }

    fn operands(
        &mut self,
        left_surface: &SExpr,
        right_surface: &SExpr,
        env: &Env,
        path: &[String],
        span: Option<Span>,
    ) -> Result<(Expr, Expr)> {
        let mut left = self.expr(left_surface, env, path, None, true)?;
        let right_expected = (!left.ty.is_literal()).then(|| left.ty.clone());
        let mut right = self.expr(right_surface, env, path, right_expected.as_ref(), true)?;
        if left.ty.is_literal() && !right.ty.is_literal() {
            left = self.expr(left_surface, env, path, Some(&right.ty), false)?;
        }
        if left.ty.is_literal() {
            left = self.expr(left_surface, env, path, Some(&self.default_number()), false)?;
            right = self.expr(right_surface, env, path, Some(&left.ty), false)?;
        }
        self.unify(left, right, span)
    }

    const fn default_number(&self) -> Type {
        match self.language {
            Language::JavaScript => INT,
            Language::Rust => fixed(32, true),
            Language::Lean | Language::Rocq => NAT,
        }
    }

    fn fresh_name(&mut self, prefix: &str) -> String {
        self.fresh += 1;
        format!("{prefix}{}", self.fresh)
    }

    fn unify(&self, left: Expr, right: Expr, span: Option<Span>) -> Result<(Expr, Expr)> {
        if left.ty.same(&right.ty) {
            return Ok((left, right));
        }
        if self.language == Language::JavaScript {
            // Guarded parameters are naturals, but every BigInt operation is an integer operation.
            let left = coerce(left, &INT, self.language, span, None)?;
            let right = coerce(right, &INT, self.language, span, None)?;
            return Ok((left, right));
        }
        Err(type_error(
            format!(
                "operand types differ: {} and {}",
                left.ty.key(),
                right.ty.key()
            ),
            span,
        ))
    }

    fn unify_branches(
        &self,
        then: Expr,
        otherwise: Expr,
        expected: Option<&Type>,
        span: Option<Span>,
    ) -> Result<(Expr, Expr)> {
        if then.ty.same(&otherwise.ty) {
            return Ok((then, otherwise));
        }
        if matches!(then.node, Node::Abort { .. }) {
            let ty = otherwise.ty.clone();
            return Ok((Expr { ty, ..then }, otherwise));
        }
        if matches!(otherwise.node, Node::Abort { .. }) {
            let ty = then.ty.clone();
            return Ok((then, Expr { ty, ..otherwise }));
        }
        if let Some(expected) = expected {
            let then = coerce(then, expected, self.language, span, None)?;
            let otherwise = coerce(otherwise, expected, self.language, span, None)?;
            return Ok((then, otherwise));
        }
        self.unify(then, otherwise, span)
    }

    fn coerce(&self, value: Expr, ty: &Type, span: Option<Span>) -> Result<Expr> {
        coerce(value, ty, self.language, span, None)
    }

    fn expr(
        &mut self,
        node: &SExpr,
        env: &Env,
        path: &[String],
        expected: Option<&Type>,
        allow_literal: bool,
    ) -> Result<Expr> {
        let mut result = self.expr_inner(node, env, path, expected, allow_literal)?;
        if node.span.is_some() && result.span.is_none() {
            result.span = node.span;
        }
        Ok(result)
    }

    #[allow(clippy::too_many_lines)]
    fn expr_inner(
        &mut self,
        node: &SExpr,
        env: &Env,
        path: &[String],
        expected: Option<&Type>,
        allow_literal: bool,
    ) -> Result<Expr> {
        let span = node.span;
        match &node.node {
            SNode::Num {
                value,
                ty,
                negative,
            } => {
                let ty = ty
                    .clone()
                    .or_else(|| expected.filter(|ty| ty.is_numeric()).cloned());
                let Some(ty) = ty else {
                    if allow_literal {
                        return Ok(text_lit(Type::Literal, value.clone()));
                    }
                    return self.expr(node, env, path, Some(&self.default_number()), false);
                };
                if !ty.is_numeric() {
                    return Err(type_error(
                        format!("numeric literal where {} is expected", ty.key()),
                        span,
                    ));
                }
                let mut number = Decimal::parse(value)
                    .ok_or_else(|| type_error(format!("invalid numeric literal {value}"), span))?;
                if *negative {
                    number = number.negate();
                }
                if ty.is_natural() && number.is_negative() {
                    return Err(type_error("negative literal for a natural type", span));
                }
                if let Type::Fixed { bits, signed } = ty {
                    let (min, max) = fixed_bounds(bits, signed);
                    if number < Decimal::from_i128(min) || number > Decimal::from_u128(max) {
                        return Err(type_error(
                            format!("literal out of range for {}", ty.key()),
                            span,
                        ));
                    }
                }
                Ok(text_lit(ty, number.to_string()))
            }
            SNode::Bool { value } => Ok(Expr::lit(BOOL, LitValue::Bool(*value))),
            SNode::Str { value } => Ok(text_lit(STRING, value.clone())),
            SNode::Unit => Ok(Expr::new(Node::Unit, UNIT)),
            SNode::Name { path: segments } => {
                if let Some(name) = env.local(node) {
                    return Ok(Expr::var(name, env.vars[name].clone()));
                }
                let _ = segments;
                self.application(node, &[], env, path, expected, span)
            }
            SNode::DotCtor { .. } => self.application(node, &[], env, path, expected, span),
            SNode::App { func, args } => self.application(func, args, env, path, expected, span),
            SNode::Field { field, .. } => Err(unsupported(
                "field access",
                &format!("field {field} is only portable inside a constructor match"),
                span,
            )),
            SNode::Unary {
                op: UnaryOp::Not,
                arg,
            } => {
                let arg = self.expr(arg, env, path, Some(&BOOL), false)?;
                Ok(Expr::new(
                    Node::Unary {
                        op: UnaryOp::Not,
                        arg: Box::new(self.coerce(arg, &BOOL, span)?),
                        semantics: None,
                    },
                    BOOL,
                ))
            }
            SNode::Unary {
                op: UnaryOp::Neg,
                arg,
            } => {
                if let SNode::Num {
                    value,
                    ty,
                    negative,
                } = &arg.node
                {
                    let flipped = SExpr {
                        node: SNode::Num {
                            value: value.clone(),
                            ty: ty.clone(),
                            negative: !negative,
                        },
                        span,
                        block: arg.block,
                    };
                    return self.expr(&flipped, env, path, expected, allow_literal);
                }
                let numeric = expected.filter(|ty| ty.is_numeric());
                let arg = self.expr(arg, env, path, numeric, false)?;
                if arg.ty == Type::Nat && self.language == Language::JavaScript {
                    // A guarded natural is still a BigInt, and its negation an integer.
                    return Ok(Expr::new(
                        Node::Unary {
                            op: UnaryOp::Neg,
                            arg: Box::new(self.coerce(arg, &INT, span)?),
                            semantics: Some(Semantics::Exact),
                        },
                        INT,
                    ));
                }
                if !matches!(arg.ty, Type::Int | Type::Fixed { signed: true, .. }) {
                    return Err(type_error(format!("negation of {}", arg.ty.key()), span));
                }
                let ty = arg.ty.clone();
                let semantics = if matches!(ty, Type::Fixed { .. }) {
                    Semantics::Checked
                } else {
                    Semantics::Exact
                };
                Ok(Expr::new(
                    Node::Unary {
                        op: UnaryOp::Neg,
                        arg: Box::new(arg),
                        semantics: Some(semantics),
                    },
                    ty,
                ))
            }
            SNode::Binary {
                op,
                left,
                right,
                rounding,
            } => self.binary(
                *op,
                left,
                right,
                *rounding,
                span,
                env,
                path,
                expected,
                allow_literal,
            ),
            SNode::If {
                cond,
                then,
                otherwise,
            } => {
                let checked = self.expr(cond, env, path, Some(&BOOL), false)?;
                let cond = self.coerce(checked, &BOOL, span)?;
                let mut then_expr = self.expr(then, env, path, expected, allow_literal)?;
                let else_expected = if then_expr.ty.is_literal() {
                    expected.cloned()
                } else {
                    Some(then_expr.ty.clone())
                };
                let mut else_expr =
                    self.expr(otherwise, env, path, else_expected.as_ref(), allow_literal)?;
                if then_expr.ty.is_literal() && !else_expr.ty.is_literal() {
                    then_expr = self.expr(then, env, path, Some(&else_expr.ty), false)?;
                }
                if then_expr.ty.is_literal() {
                    then_expr = self.expr(then, env, path, Some(&self.default_number()), false)?;
                    else_expr = self.expr(otherwise, env, path, Some(&then_expr.ty), false)?;
                }
                let (then_expr, else_expr) =
                    self.unify_branches(then_expr, else_expr, expected, span)?;
                let ty = then_expr.ty.clone();
                Ok(Expr::new(
                    Node::If {
                        cond: Box::new(cond),
                        then: Box::new(then_expr),
                        otherwise: Box::new(else_expr),
                    },
                    ty,
                ))
            }
            SNode::Let {
                name,
                ty,
                value,
                body,
            } => {
                let declared = match ty {
                    Some(ty) => Some(self.resolve_type(Some(ty), path, span)?),
                    None => None,
                };
                let mut value = self.expr(value, env, path, declared.as_ref(), false)?;
                if let Some(declared) = &declared {
                    value = self.coerce(value, declared, span)?;
                }
                let mut inner = env.clone();
                inner.bind_local(name, value.ty.clone());
                let body = self.expr(body, &inner, path, expected, allow_literal)?;
                let ty = body.ty.clone();
                Ok(Expr::new(
                    Node::Let {
                        name: name.clone(),
                        value: Box::new(value),
                        body: Box::new(body),
                    },
                    ty,
                ))
            }
            SNode::Match { scrutinees, rows } => {
                let compiled = self.compile_match(scrutinees, rows, span, env, path)?;
                self.expr(&compiled, env, path, expected, allow_literal)
            }
            SNode::Match1 { scrutinee, cases } => {
                self.match_cases(scrutinee, cases, span, env, path, expected)
            }
            SNode::ToString { arg } => {
                let arg = self.expr(arg, env, path, None, false)?;
                if arg.ty == Type::String {
                    return Ok(arg);
                }
                if !arg.ty.is_numeric() && arg.ty != Type::Bool {
                    return Err(type_error(format!("toString of {}", arg.ty.key()), span));
                }
                Ok(Expr::new(Node::ToString { arg: Box::new(arg) }, STRING))
            }
            SNode::Show { arg, style } => self.show(arg, env, path, *style),
            SNode::Cast {
                arg,
                to,
                from,
                flavor,
            } => {
                let to = self.resolve_type(Some(to), path, span)?;
                let arg = self.expr(arg, env, path, from.as_ref(), false)?;
                cast_to(arg, &to, *flavor, span)
            }
            SNode::Abort { message } => {
                let Some(expected) = expected else {
                    return Err(type_error("abort needs a known result type", span));
                };
                Ok(Expr::new(
                    Node::Abort {
                        message: message.clone(),
                    },
                    expected.clone(),
                ))
            }
            SNode::CtorObject { tag, fields } => {
                self.ctor_object(tag, fields, span, env, path, expected)
            }
            other => Err(type_error(
                format!("unknown expression {}", other.kind()),
                span,
            )),
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn binary(
        &mut self,
        op: BinaryOp,
        left: &SExpr,
        right: &SExpr,
        rounding: Option<Rounding>,
        span: Option<Span>,
        env: &Env,
        path: &[String],
        expected: Option<&Type>,
        allow_literal: bool,
    ) -> Result<Expr> {
        match op {
            BinaryOp::And | BinaryOp::Or | BinaryOp::Concat => {
                let ty = if op == BinaryOp::Concat { STRING } else { BOOL };
                let checked = self.expr(left, env, path, Some(&ty), false)?;
                let left = self.coerce(checked, &ty, span)?;
                let checked = self.expr(right, env, path, Some(&ty), false)?;
                let right = self.coerce(checked, &ty, span)?;
                Ok(plain_binary(op, left, right, ty))
            }
            _ if comparison(op) => {
                let (left, right) = self.operands(left, right, env, path, span)?;
                if matches!(left.ty, Type::Data { .. } | Type::Unit) {
                    return Err(unsupported(
                        "structural equality of data values",
                        "comparisons of data values are not in the portable core",
                        span,
                    ));
                }
                if !matches!(op, BinaryOp::Eq | BinaryOp::Ne) && !left.ty.is_numeric() {
                    return Err(type_error(format!("ordering on {}", left.ty.key()), span));
                }
                let domain = left.ty.clone();
                Ok(Expr::new(
                    binary_node(
                        op,
                        Box::new(left),
                        Box::new(right),
                        Some(domain),
                        None,
                        None,
                        None,
                    ),
                    BOOL,
                ))
            }
            BinaryOp::Plus => self.plus(
                left,
                right,
                rounding,
                span,
                env,
                path,
                expected,
                allow_literal,
            ),
            _ => self.arithmetic(
                op,
                left,
                right,
                rounding,
                span,
                env,
                path,
                expected,
                allow_literal,
                None,
            ),
        }
    }

    /// JavaScript's `+` concatenates when either operand is a string, reading
    /// the other as `String(value)`, and adds numbers otherwise.
    #[allow(clippy::too_many_arguments)]
    fn plus(
        &mut self,
        left_surface: &SExpr,
        right_surface: &SExpr,
        rounding: Option<Rounding>,
        span: Option<Span>,
        env: &Env,
        path: &[String],
        expected: Option<&Type>,
        allow_literal: bool,
    ) -> Result<Expr> {
        let left = self.expr(left_surface, env, path, None, true)?;
        let right = self.expr(right_surface, env, path, None, true)?;
        if left.ty != Type::String && right.ty != Type::String {
            return self.arithmetic(
                BinaryOp::Add,
                left_surface,
                right_surface,
                rounding,
                span,
                env,
                path,
                expected,
                allow_literal,
                Some((left, right)),
            );
        }
        let left = self.text(left, left_surface, env, path)?;
        let right = self.text(right, right_surface, env, path)?;
        Ok(plain_binary(BinaryOp::Concat, left, right, STRING))
    }

    fn text(&mut self, value: Expr, surface: &SExpr, env: &Env, path: &[String]) -> Result<Expr> {
        if value.ty == Type::String {
            return Ok(value);
        }
        let checked = if value.ty.is_literal() {
            self.expr(surface, env, path, Some(&self.default_number()), false)?
        } else {
            value
        };
        if matches!(checked.ty, Type::Data { .. } | Type::Unit) {
            return Err(unsupported(
                "string conversion of structured values",
                &format!("String({}) has no portable textual form", checked.ty.key()),
                surface.span,
            ));
        }
        Ok(Expr::new(
            Node::ToString {
                arg: Box::new(checked),
            },
            STRING,
        ))
    }

    /// `checked` holds operands already checked without an expected type, as `plus` does.
    #[allow(clippy::too_many_arguments)]
    fn arithmetic(
        &mut self,
        op: BinaryOp,
        left_surface: &SExpr,
        right_surface: &SExpr,
        rounding: Option<Rounding>,
        span: Option<Span>,
        env: &Env,
        path: &[String],
        expected: Option<&Type>,
        allow_literal: bool,
        checked: Option<(Expr, Expr)>,
    ) -> Result<Expr> {
        if !matches!(
            op,
            BinaryOp::Add | BinaryOp::Sub | BinaryOp::Mul | BinaryOp::Div | BinaryOp::Rem
        ) {
            return Err(type_error(
                format!("unknown operator {}", op_name(op)),
                span,
            ));
        }
        let numeric_expected = expected.filter(|ty| ty.is_numeric()).cloned();
        let (checked_left, checked_right) = match checked {
            Some((left, right)) => (Some(left), Some(right)),
            None => (None, None),
        };
        let mut left = match checked_left {
            Some(left) if !left.ty.is_literal() => left,
            _ => self.expr(left_surface, env, path, numeric_expected.as_ref(), true)?,
        };
        let mut right = match checked_right {
            Some(right) if !right.ty.is_literal() => right,
            _ => {
                let right_expected = if left.ty.is_literal() {
                    numeric_expected.clone()
                } else {
                    Some(left.ty.clone())
                };
                self.expr(right_surface, env, path, right_expected.as_ref(), true)?
            }
        };
        if left.ty.is_literal() {
            let left_expected = if right.ty.is_literal() {
                numeric_expected.clone()
            } else {
                Some(right.ty.clone())
            };
            left = self.expr(left_surface, env, path, left_expected.as_ref(), true)?;
        }
        if left.ty.is_literal() && right.ty.is_literal() {
            if allow_literal {
                return Ok(text_lit(Type::Literal, "0"));
            }
            left = self.expr(left_surface, env, path, Some(&self.default_number()), false)?;
            right = self.expr(right_surface, env, path, Some(&left.ty), false)?;
        }
        if right.ty.is_literal() {
            right = self.expr(right_surface, env, path, Some(&left.ty), false)?;
        }
        if left.ty.is_literal() {
            left = self.expr(left_surface, env, path, Some(&right.ty), false)?;
        }
        if self.language == Language::JavaScript && (left.ty == Type::Nat || right.ty == Type::Nat)
        {
            // BigInt arithmetic is integer arithmetic even on guarded naturals: `n - 1n` is -1n for n = 0n.
            let widen = |value: Expr| match &value.node {
                Node::Lit { value } => Ok(Expr::lit(INT, value.clone())),
                _ => coerce(value, &INT, Language::JavaScript, span, None),
            };
            left = widen(left)?;
            right = widen(right)?;
        }
        let (left, right) = self.unify(left, right, span)?;
        if !left.ty.is_numeric() {
            return Err(type_error(format!("arithmetic on {}", left.ty.key()), span));
        }
        let ty = left.ty.clone();
        let (semantics, rounding, by_zero) = arithmetic_semantics(self.language, op, &ty, rounding);
        Ok(Expr::new(
            binary_node(
                op,
                Box::new(left),
                Box::new(right),
                Some(ty.clone()),
                Some(semantics),
                rounding,
                by_zero,
            ),
            ty,
        ))
    }

    fn application(
        &mut self,
        head: &SExpr,
        args: &[SExpr],
        env: &Env,
        path: &[String],
        expected: Option<&Type>,
        span: Option<Span>,
    ) -> Result<Expr> {
        let segments = match &head.node {
            SNode::DotCtor { name } => {
                let Some(Type::Data { name: data_name }) = expected else {
                    return Err(type_error(
                        format!("cannot infer the type of .{name}"),
                        span,
                    ));
                };
                let decl = self.data_decl(data_name)?;
                let full_name = decl.full_name.clone();
                let Some(ctor) = decl.ctors.iter().find(|ctor| ctor.name == *name).cloned() else {
                    return Err(type_error(
                        format!("{data_name} has no constructor {name}"),
                        span,
                    ));
                };
                return self.construct(&full_name, &ctor, args, env, path, span);
            }
            SNode::Name { path } => path,
            _ => {
                return Err(unsupported(
                    "higher-order application",
                    "only named functions and constructors can be applied",
                    span,
                ))
            }
        };
        if let Some(name) = env.local(head) {
            return Err(unsupported(
                "higher-order application",
                &format!("local {name} is not a function in the portable core"),
                span,
            ));
        }
        let Some(entry) = self.lookup(segments, path, span)? else {
            return Err(type_error(
                format!("unknown name {}", segments.join(".")),
                span,
            ));
        };
        let decl_name = match entry {
            Entry::Ctor { data, name } => {
                let decl = self.data_decl(&data)?;
                let Some(ctor) = decl.ctors.iter().find(|ctor| ctor.name == name).cloned() else {
                    return Err(type_error(
                        format!("{data} has no constructor {name}"),
                        span,
                    ));
                };
                return self.construct(&data, &ctor, args, env, path, span);
            }
            Entry::Decl(name) => name,
        };
        let Some(Decl::Fn(entry)) = self.decls.get(&decl_name) else {
            return Err(type_error(
                format!("{} is not a function", segments.join(".")),
                span,
            ));
        };
        if !self.signatures.contains(&decl_name) {
            return Err(type_error(
                format!("{} is used before its signature is known", entry.full_name),
                span,
            ));
        }
        let (full_name, params, ret) = (
            entry.full_name.clone(),
            entry.params.clone(),
            entry.ret.clone(),
        );
        if args.len() != params.len() {
            if args.len() < params.len() {
                return Err(unsupported(
                    "partial application",
                    &format!("{full_name} expects {} arguments", params.len()),
                    span,
                ));
            }
            return Err(type_error(
                format!(
                    "{full_name} expects {} arguments but got {}",
                    params.len(),
                    args.len()
                ),
                span,
            ));
        }
        let mut checked = Vec::new();
        for (arg, param) in args.iter().zip(&params) {
            let value = self.expr(arg, env, path, Some(&param.ty), false)?;
            let flavor = (param.guard == Some(true)).then_some(Flavor::Checked);
            checked.push(coerce(
                value,
                &param.ty,
                self.language,
                arg.span.or(span),
                flavor,
            )?);
        }
        Ok(Expr::new(
            Node::Call {
                func: full_name,
                args: checked,
            },
            ret,
        ))
    }

    fn construct(
        &mut self,
        data_name: &str,
        ctor: &Ctor,
        args: &[SExpr],
        env: &Env,
        path: &[String],
        span: Option<Span>,
    ) -> Result<Expr> {
        if args.len() != ctor.fields.len() {
            return Err(type_error(
                format!(
                    "{data_name}.{} expects {} fields but got {}",
                    ctor.name,
                    ctor.fields.len(),
                    args.len()
                ),
                span,
            ));
        }
        let mut checked = Vec::new();
        for (arg, field) in args.iter().zip(&ctor.fields) {
            let value = self.expr(arg, env, path, Some(&field.ty), false)?;
            checked.push(self.coerce(value, &field.ty, arg.span.or(span))?);
        }
        Ok(Expr::new(
            Node::Ctor {
                data: data_name.to_owned(),
                ctor: ctor.name.clone(),
                args: checked,
            },
            data(data_name),
        ))
    }

    fn ctor_object(
        &mut self,
        tag: &str,
        fields: &[(String, SExpr)],
        span: Option<Span>,
        env: &Env,
        path: &[String],
        expected: Option<&Type>,
    ) -> Result<Expr> {
        let mut candidates = Vec::new();
        for name in &self.order {
            let Some(Decl::Data(entry)) = self.decls.get(name) else {
                continue;
            };
            if let Some(Type::Data { name: expected }) = expected {
                if entry.full_name != *expected {
                    continue;
                }
            }
            if let Some(ctor) = entry.ctors.iter().find(|ctor| ctor.name == tag) {
                candidates.push((entry.full_name.clone(), ctor.clone()));
            }
        }
        if candidates.len() != 1 {
            return Err(type_error(
                if candidates.is_empty() {
                    format!("no data type has tag {tag}")
                } else {
                    format!("tag {tag} is ambiguous")
                },
                span,
            ));
        }
        let (data_name, ctor) = candidates.remove(0);
        let expected_names: Vec<&str> = ctor
            .fields
            .iter()
            .map(|field| field.name.as_str())
            .collect();
        if fields.len() != expected_names.len()
            || fields
                .iter()
                .any(|(name, _)| !expected_names.contains(&name.as_str()))
        {
            return Err(type_error(
                format!("{tag} expects fields {}", expected_names.join(", ")),
                span,
            ));
        }
        let args: Vec<SExpr> = expected_names
            .iter()
            .filter_map(|name| {
                fields
                    .iter()
                    .rev()
                    .find(|(field, _)| field == name)
                    .map(|(_, value)| value.clone())
            })
            .collect();
        self.construct(&data_name, &ctor, &args, env, path, span)
    }

    /// Compiles a surface match (several scrutinees, nested and literal
    /// patterns) into single-level constructor matches and literal if-chains.
    /// Bind patterns name the constructor fields directly so structural
    /// recursion stays visible to the recursion analysis.
    fn compile_match(
        &mut self,
        scrutinees: &[SExpr],
        rows: &[SRow],
        span: Option<Span>,
        env: &Env,
        path: &[String],
    ) -> Result<SExpr> {
        let mut lets = Vec::new();
        let mut columns = Vec::new();
        for scrutinee in scrutinees {
            let checked = self.expr(scrutinee, env, path, None, false)?;
            if let Some(name) = env.local(scrutinee) {
                columns.push(Column {
                    name: name.to_owned(),
                    ty: checked.ty,
                });
                continue;
            }
            let name = self.fresh_name("ml_s");
            lets.push((name.clone(), scrutinee.clone()));
            columns.push(Column {
                name,
                ty: checked.ty,
            });
        }
        let rows = rows
            .iter()
            .map(|row| Row {
                patterns: row.patterns.iter().cloned().map(NPat::Surface).collect(),
                body: row.body.clone(),
                binds: Vec::new(),
            })
            .collect();
        let mut body = self.compile_rows(&columns, rows, span)?;
        for (name, value) in lets.into_iter().rev() {
            body = SExpr::new(
                SNode::Let {
                    name,
                    ty: None,
                    value: Box::new(value),
                    body: Box::new(body),
                },
                span,
            );
        }
        Ok(body)
    }

    #[allow(clippy::too_many_lines)]
    fn compile_rows(
        &mut self,
        columns: &[Column],
        rows: Vec<Row>,
        span: Option<Span>,
    ) -> Result<SExpr> {
        if rows.is_empty() {
            return Err(type_error("non-exhaustive match", span));
        }
        let mut normalised = Vec::with_capacity(rows.len());
        for row in rows {
            let mut patterns = Vec::with_capacity(row.patterns.len());
            for (index, pattern) in row.patterns.into_iter().enumerate() {
                let Some(column) = columns.get(index) else {
                    return Err(type_error("non-exhaustive match", span));
                };
                patterns.push(self.normalise_pattern(pattern, &column.ty, span)?);
            }
            normalised.push(Row { patterns, ..row });
        }
        let first = &normalised[0];
        let Some(refutable) = first
            .patterns
            .iter()
            .position(|pattern| !pattern.irrefutable())
        else {
            let mut binds = first.binds.clone();
            for (pattern, column) in first.patterns.iter().zip(columns) {
                if let NPat::Bind(name) = pattern {
                    binds.push((name.clone(), column.name.clone()));
                }
            }
            let mut body = first.body.clone();
            for (name, variable) in binds.iter().rev() {
                if name != variable {
                    body = surface_let(name, variable, body, span);
                }
            }
            return Ok(body);
        };
        let column = columns[refutable].clone();
        let pivot = first.patterns[refutable].clone();
        let rest = |patterns: &[NPat]| -> Vec<NPat> {
            patterns
                .iter()
                .enumerate()
                .filter(|(index, _)| *index != refutable)
                .map(|(_, pattern)| pattern.clone())
                .collect()
        };
        let bind_column = |row: &Row| -> Vec<(String, String)> {
            let mut binds = row.binds.clone();
            if let NPat::Bind(name) = &row.patterns[refutable] {
                binds.push((name.clone(), column.name.clone()));
            }
            binds
        };
        let remaining: Vec<Column> = columns
            .iter()
            .enumerate()
            .filter(|(index, _)| *index != refutable)
            .map(|(_, column)| column.clone())
            .collect();
        if let NPat::Lit {
            value: pivot_value,
            key: pivot_key,
        } = &pivot
        {
            // Literal patterns on integers, booleans and strings become equality tests.
            let same = |row: &Row| row.patterns[refutable].lit_key() == Some(pivot_key.as_str());
            let matching: Vec<Row> = normalised
                .iter()
                .filter(|row| row.patterns[refutable].irrefutable() || same(row))
                .map(|row| Row {
                    patterns: rest(&row.patterns),
                    binds: bind_column(row),
                    body: row.body.clone(),
                })
                .collect();
            // Where a boolean column is not the pivot it is the other value, so
            // its remaining literal patterns always match there.
            let boolean = column.ty == Type::Bool;
            let others: Vec<Row> = normalised
                .iter()
                .filter(|row| !same(row))
                .map(|row| {
                    let mut row = row.clone();
                    if boolean && matches!(row.patterns[refutable], NPat::Lit { .. }) {
                        row.patterns[refutable] = NPat::Wild;
                    }
                    row
                })
                .collect();
            let then = self.compile_rows(&remaining, matching, span)?;
            if boolean && others.is_empty() {
                return Err(type_error("non-exhaustive match on bool", span));
            }
            let otherwise = self.compile_rows(columns, others, span)?;
            return Ok(SExpr::new(
                SNode::If {
                    cond: Box::new(SExpr::new(
                        SNode::Binary {
                            op: BinaryOp::Eq,
                            left: Box::new(SExpr::name(&column.name)),
                            right: Box::new(pivot_value.clone()),
                            rounding: None,
                        },
                        None,
                    )),
                    then: Box::new(then),
                    otherwise: Box::new(otherwise),
                },
                span,
            ));
        }
        let nat = matches!(pivot, NPat::Nat { .. });
        let ctors = if nat {
            vec![
                Ctor {
                    name: "zero".to_owned(),
                    fields: Vec::new(),
                },
                Ctor {
                    name: "succ".to_owned(),
                    fields: vec![Field {
                        name: String::new(),
                        ty: column.ty.clone(),
                    }],
                },
            ]
        } else {
            let name = column.ty.data_name().unwrap_or_default().to_owned();
            self.data_decl(&name)?.ctors.clone()
        };
        let mut cases = Vec::new();
        for ctor in &ctors {
            let mut field_names = Vec::new();
            for field_index in 0..ctor.fields.len() {
                let named = normalised.iter().find_map(|row| {
                    let pattern = &row.patterns[refutable];
                    if pattern.ctor() != Some(ctor.name.as_str()) {
                        return None;
                    }
                    match pattern.args().get(field_index) {
                        Some(NPat::Bind(name)) => Some(name.clone()),
                        _ => None,
                    }
                });
                let name = named.unwrap_or_else(|| self.fresh_name("ml_f"));
                field_names.push(name);
            }
            let field_columns: Vec<Column> = ctor
                .fields
                .iter()
                .zip(&field_names)
                .map(|(field, name)| Column {
                    name: name.clone(),
                    ty: field.ty.clone(),
                })
                .collect();
            let mut specialised = Vec::new();
            for row in &normalised {
                let pattern = &row.patterns[refutable];
                if pattern.irrefutable() {
                    let mut patterns = vec![NPat::Wild; field_columns.len()];
                    patterns.extend(rest(&row.patterns));
                    specialised.push(Row {
                        patterns,
                        binds: bind_column(row),
                        body: row.body.clone(),
                    });
                } else if pattern.ctor() == Some(ctor.name.as_str()) {
                    let mut patterns = pattern.args().to_vec();
                    patterns.extend(rest(&row.patterns));
                    specialised.push(Row {
                        patterns,
                        ..row.clone()
                    });
                }
            }
            let mut body_columns = field_columns;
            body_columns.extend(remaining.iter().cloned());
            let body = self.compile_rows(&body_columns, specialised, span)?;
            let pattern = if nat {
                if ctor.name == "zero" {
                    SCasePattern::NatZero
                } else {
                    SCasePattern::NatSucc {
                        name: field_names[0].clone(),
                    }
                }
            } else {
                SCasePattern::Ctor {
                    path: vec![ctor.name.clone()],
                    binds: field_names.into_iter().map(Some).collect(),
                }
            };
            cases.push(SCase {
                pattern,
                body,
                span: None,
            });
        }
        Ok(SExpr::new(
            SNode::Match1 {
                scrutinee: Box::new(SExpr::name(&column.name)),
                cases,
            },
            span,
        ))
    }

    /// Resolves a surface pattern against the column type: constructors, naturals, literals, binders.
    fn normalise_pattern(&mut self, pattern: NPat, ty: &Type, span: Option<Span>) -> Result<NPat> {
        let NPat::Surface(pattern) = pattern else {
            return Ok(pattern);
        };
        let natural = *ty == Type::Nat;
        let at = pattern.span.or(span);
        let succ = |inner: NPat| NPat::Nat {
            succ: true,
            args: vec![inner],
        };
        let zero = || NPat::Nat {
            succ: false,
            args: Vec::new(),
        };
        match pattern.node {
            SPatternNode::Wild => Ok(NPat::Wild),
            SPatternNode::BindOrCtor { name } => {
                if let Type::Data { name: data_name } = ty {
                    if let Some(ctor) = self
                        .data_decl(data_name)?
                        .ctors
                        .iter()
                        .find(|ctor| ctor.name == name)
                    {
                        if !ctor.fields.is_empty() {
                            return Err(type_error(
                                format!("{name} pattern needs {} fields", ctor.fields.len()),
                                at,
                            ));
                        }
                        return Ok(NPat::Data {
                            ctor: ctor.name.clone(),
                            args: Vec::new(),
                        });
                    }
                }
                if natural && self.language == Language::Rocq && name == "O" {
                    return Ok(zero());
                }
                if *ty == Type::Bool && (name == "true" || name == "false") {
                    let value = SExpr::new(
                        SNode::Bool {
                            value: name == "true",
                        },
                        None,
                    );
                    return self.literal_pattern(value, ty, span);
                }
                Ok(NPat::Bind(name))
            }
            SPatternNode::NumLit { value, negative } => {
                // A JavaScript `case 0n:` is an `===` test, which the recursion analysis reads as a zero test.
                if natural && self.language != Language::JavaScript {
                    let count = Decimal::parse(&value)
                        .and_then(|value| value.to_i128())
                        .unwrap_or(0);
                    let mut result = zero();
                    for _ in 0..count {
                        result = succ(result);
                    }
                    return Ok(result);
                }
                let value = SExpr::new(
                    SNode::Num {
                        value,
                        ty: None,
                        negative,
                    },
                    None,
                );
                self.literal_pattern(value, ty, span)
            }
            SPatternNode::BoolLit { value, .. } => {
                self.literal_pattern(SExpr::new(SNode::Bool { value }, None), ty, span)
            }
            SPatternNode::StrLit { value } => {
                self.literal_pattern(SExpr::new(SNode::Str { value }, None), ty, span)
            }
            SPatternNode::NatAdd { inner, add } => {
                if !natural {
                    return Err(unsupported(
                        "n + k pattern",
                        &format!(
                            "only natural-number scrutinees support n + k patterns, not {}",
                            ty.key()
                        ),
                        at,
                    ));
                }
                let mut result = self.normalise_pattern(NPat::Surface(*inner), ty, span)?;
                for _ in 0..add {
                    result = succ(result);
                }
                Ok(result)
            }
            SPatternNode::Ctor { path, args } => {
                let name = path.last().cloned().unwrap_or_default();
                if natural && args.len() == 1 && (name == "succ" || name == "S") {
                    let arg = args.into_iter().next().map_or(NPat::Wild, NPat::Surface);
                    return Ok(succ(self.normalise_pattern(arg, ty, span)?));
                }
                if natural && args.is_empty() && (name == "zero" || name == "O") {
                    return Ok(zero());
                }
                let Type::Data { name: data_name } = ty else {
                    return Err(type_error(
                        format!("constructor pattern {name} on {}", ty.key()),
                        at,
                    ));
                };
                let decl = self.data_decl(data_name)?;
                let module_path = decl.module_path.clone();
                let ctor = decl.ctors.iter().find(|ctor| ctor.name == name).cloned();
                if path.len() > 1 {
                    let owner = match self.lookup(&path, &[], at)? {
                        Some(owner) => Some(owner),
                        None => self.lookup(&path, &module_path, at)?,
                    };
                    if !matches!(&owner, Some(Entry::Ctor { data, .. }) if data == data_name) {
                        return Err(type_error(
                            format!("{} is not a constructor of {data_name}", path.join(".")),
                            at,
                        ));
                    }
                }
                let Some(ctor) = ctor else {
                    return Err(type_error(
                        format!("{data_name} has no constructor {name}"),
                        at,
                    ));
                };
                if args.len() != ctor.fields.len() {
                    return Err(type_error(
                        format!(
                            "{name} pattern has {} of {} fields",
                            args.len(),
                            ctor.fields.len()
                        ),
                        at,
                    ));
                }
                let mut normalised = Vec::new();
                for (arg, field) in args.into_iter().zip(&ctor.fields) {
                    normalised.push(self.normalise_pattern(NPat::Surface(arg), &field.ty, span)?);
                }
                Ok(NPat::Data {
                    ctor: name,
                    args: normalised,
                })
            }
        }
    }

    fn literal_pattern(&mut self, value: SExpr, ty: &Type, span: Option<Span>) -> Result<NPat> {
        let checked = self.expr(&value, &Env::default(), &[], Some(ty), false)?;
        if !checked.ty.same(ty) {
            return Err(type_error(
                format!(
                    "literal pattern of type {} on {}",
                    checked.ty.key(),
                    ty.key()
                ),
                span,
            ));
        }
        let text = match &checked.node {
            Node::Lit { value } => value.text(),
            _ => String::new(),
        };
        Ok(NPat::Lit {
            value,
            key: format!("{}:{text}", ty.key()),
        })
    }

    fn match_cases(
        &mut self,
        scrutinee: &SExpr,
        cases: &[SCase],
        span: Option<Span>,
        env: &Env,
        path: &[String],
        expected: Option<&Type>,
    ) -> Result<Expr> {
        let variable = env.local(scrutinee).map(str::to_owned);
        if let Some(variable) = &variable {
            if let Some(fact) = env.known.get(variable) {
                let fact = fact.clone();
                return self.known_match(cases, span, &fact, variable, env, path, expected);
            }
        }
        let scrutinee = self.expr(scrutinee, env, path, None, false)?;
        let ty = scrutinee.ty.clone();
        let mut pending = Vec::new();
        for kase in cases {
            let mut inner = env.clone();
            let pattern = self.pattern(&kase.pattern, &ty, &mut inner, kase.span.or(span))?;
            if let (Some(variable), Pattern::Ctor { ctor, binds, .. }) = (&variable, &pattern) {
                if binds.iter().all(Option::is_some)
                    && !binds.iter().flatten().any(|bind| bind == variable)
                {
                    inner.known.insert(
                        variable.clone(),
                        Fact {
                            ctor: ctor.clone(),
                            binds: binds.iter().flatten().cloned().collect(),
                        },
                    );
                }
            }
            pending.push((pattern, &kase.body, inner));
        }
        let mut result_type = expected.cloned();
        let mut checked = Vec::new();
        for (pattern, surface, inner) in pending {
            let body = self.expr(surface, &inner, path, result_type.as_ref(), true)?;
            if !body.ty.is_literal() && result_type.is_none() {
                result_type = Some(body.ty.clone());
            }
            checked.push((pattern, body, surface, inner));
        }
        let result_type = result_type.unwrap_or_else(|| self.default_number());
        let mut final_cases = Vec::new();
        for (pattern, body, surface, inner) in checked {
            let body = if body.ty.is_literal() {
                self.expr(surface, &inner, path, Some(&result_type), false)?
            } else {
                body
            };
            let body = self.coerce(body, &result_type, surface.span.or(span))?;
            final_cases.push(Case { pattern, body });
        }
        self.check_exhaustive(&ty, &final_cases, span)?;
        Ok(Expr::new(
            Node::Match {
                scrutinee: Box::new(scrutinee),
                cases: final_cases,
            },
            result_type,
        ))
    }

    /// Inside a case that matched `variable` against a constructor, a nested
    /// match on the same variable can only take that constructor's case: it is
    /// the case's body with the fields bound to the outer case's locals. The
    /// value is unchanged, and structural recursion stays visible to Lean and
    /// Rocq, which do not relate the inner match to the outer one.
    #[allow(clippy::too_many_arguments)]
    fn known_match(
        &mut self,
        cases: &[SCase],
        span: Option<Span>,
        fact: &Fact,
        variable: &str,
        env: &Env,
        path: &[String],
        expected: Option<&Type>,
    ) -> Result<Expr> {
        let Some(kase) = cases.iter().find(|candidate| match &candidate.pattern {
            SCasePattern::Ctor { path, .. } => path.last() == Some(&fact.ctor),
            SCasePattern::Wild | SCasePattern::Bind { .. } => true,
            _ => false,
        }) else {
            return Err(type_error(
                format!("non-exhaustive match: no case for {}", fact.ctor),
                span,
            ));
        };
        let alias_span = kase.span.or(span);
        let alias = |name: &str, value: &str, body: SExpr| {
            if name == "_" || name == value {
                body
            } else {
                surface_let(name, value, body, alias_span)
            }
        };
        let mut body = kase.body.clone();
        if let SCasePattern::Bind { name } = &kase.pattern {
            body = alias(name, variable, body);
        }
        if let SCasePattern::Ctor { binds, .. } = &kase.pattern {
            if binds.len() != fact.binds.len() {
                return Err(type_error(
                    format!(
                        "{} pattern binds {} of {} fields",
                        fact.ctor,
                        binds.len(),
                        fact.binds.len()
                    ),
                    alias_span,
                ));
            }
            // Simultaneous binding: every alias reads an outer local before any is shadowed.
            let inner: Vec<(String, String)> = binds
                .iter()
                .zip(&fact.binds)
                .filter_map(|(name, value)| {
                    name.as_ref()
                        .filter(|name| !name.is_empty() && *name != "_" && *name != value)
                        .map(|name| (name.clone(), value.clone()))
                })
                .collect();
            if inner
                .iter()
                .any(|(_, value)| inner.iter().any(|(name, _)| name == value))
            {
                let temporaries: Vec<(String, String)> = inner
                    .iter()
                    .map(|(_, value)| (self.fresh_name("ml_k"), value.clone()))
                    .collect();
                for (index, (name, _)) in inner.iter().enumerate().rev() {
                    body = alias(name, &temporaries[index].0, body);
                }
                for (name, value) in temporaries.iter().rev() {
                    body = alias(name, value, body);
                }
            } else {
                for (name, value) in inner.iter().rev() {
                    body = alias(name, value, body);
                }
            }
        }
        // `knownMatch` passes the caller's `allowLiteral`; `match` is only
        // reached with the default `false`.
        self.expr(&body, env, path, expected, false)
    }

    fn pattern(
        &self,
        pattern: &SCasePattern,
        ty: &Type,
        env: &mut Env,
        span: Option<Span>,
    ) -> Result<Pattern> {
        match pattern {
            SCasePattern::Wild => Ok(Pattern::Wild),
            SCasePattern::Bind { name } => {
                env.bind_local(name, ty.clone());
                Ok(Pattern::Bind { name: name.clone() })
            }
            SCasePattern::NatZero | SCasePattern::NatSucc { .. } => {
                if *ty != Type::Nat {
                    return Err(type_error(
                        format!("natural-number pattern on {}", ty.key()),
                        span,
                    ));
                }
                if let SCasePattern::NatSucc { name } = pattern {
                    env.bind_local(name, ty.clone());
                    return Ok(Pattern::NatSucc { name: name.clone() });
                }
                Ok(Pattern::NatZero)
            }
            SCasePattern::Ctor { path, binds } => {
                let Type::Data { name: data_name } = ty else {
                    return Err(type_error(
                        format!("constructor pattern on {}", ty.key()),
                        span,
                    ));
                };
                let ctor_name = path.last().cloned().unwrap_or_default();
                let Some(ctor) = self
                    .data_decl(data_name)?
                    .ctors
                    .iter()
                    .find(|ctor| ctor.name == ctor_name)
                    .cloned()
                else {
                    return Err(type_error(
                        format!("{data_name} has no constructor {ctor_name}"),
                        span,
                    ));
                };
                if binds.len() != ctor.fields.len() {
                    return Err(type_error(
                        format!(
                            "{ctor_name} pattern binds {} of {} fields",
                            binds.len(),
                            ctor.fields.len()
                        ),
                        span,
                    ));
                }
                let mut checked = Vec::new();
                for (bind, field) in binds.iter().zip(&ctor.fields) {
                    match bind {
                        Some(name) if name != "_" => {
                            env.bind_local(name, field.ty.clone());
                            checked.push(Some(name.clone()));
                        }
                        _ => checked.push(None),
                    }
                }
                Ok(Pattern::Ctor {
                    data: data_name.clone(),
                    ctor: ctor_name,
                    binds: checked,
                })
            }
        }
    }

    fn check_exhaustive(&self, ty: &Type, cases: &[Case], span: Option<Span>) -> Result<()> {
        let patterns: Vec<&Pattern> = cases.iter().map(|kase| &kase.pattern).collect();
        if patterns
            .iter()
            .any(|pattern| matches!(pattern, Pattern::Wild | Pattern::Bind { .. }))
        {
            return Ok(());
        }
        if let Type::Data { name } = ty {
            let covered: HashSet<&str> = patterns
                .iter()
                .filter_map(|pattern| match pattern {
                    Pattern::Ctor { ctor, .. } => Some(ctor.as_str()),
                    _ => None,
                })
                .collect();
            let missing: Vec<&str> = self
                .data_decl(name)?
                .ctors
                .iter()
                .map(|ctor| ctor.name.as_str())
                .filter(|ctor| !covered.contains(ctor))
                .collect();
            if !missing.is_empty() {
                return Err(type_error(
                    format!("match on {name} misses {}", missing.join(", ")),
                    span,
                ));
            }
            return Ok(());
        }
        if ty.is_natural()
            && patterns
                .iter()
                .any(|pattern| matches!(pattern, Pattern::NatZero))
            && patterns
                .iter()
                .any(|pattern| matches!(pattern, Pattern::NatSucc { .. }))
        {
            return Ok(());
        }
        Err(type_error(
            format!("non-exhaustive match on {}", ty.key()),
            span,
        ))
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

/// Inside `match v with | succ k => …`, the value `k + 1` is `v` itself.
/// Rewriting it to `v` keeps recursion such as `fib (n + 1) + fib n`
/// structural for Lean and Rocq without changing any value.
fn reuse_successors(expr: &Expr, predecessors: &HashMap<String, String>) -> Expr {
    match &expr.node {
        Node::Binary {
            op: BinaryOp::Add,
            left,
            right,
            ..
        } if expr.ty == Type::Nat
            && left
                .var_name()
                .is_some_and(|name| predecessors.contains_key(name))
            && matches!(&right.node, Node::Lit { value: LitValue::Text(one) } if one == "1") =>
        {
            let name = &predecessors[left.var_name().unwrap_or_default()];
            Expr::var(name.clone(), expr.ty.clone()).with_span(expr.span)
        }
        Node::Let { name, value, body } => {
            let inner = without(predecessors, &[name.as_str()]);
            Expr {
                node: Node::Let {
                    name: name.clone(),
                    value: Box::new(reuse_successors(value, predecessors)),
                    body: Box::new(reuse_successors(body, &inner)),
                },
                ..expr.clone()
            }
        }
        Node::Match { scrutinee, cases } => {
            let cases = cases
                .iter()
                .map(|kase| {
                    let bound: Vec<&str> = match &kase.pattern {
                        Pattern::NatSucc { name } => vec![name.as_str()],
                        Pattern::Ctor { binds, .. } => {
                            binds.iter().flatten().map(String::as_str).collect()
                        }
                        _ => Vec::new(),
                    };
                    let mut inner = without(predecessors, &bound);
                    if let (Pattern::NatSucc { name }, Some(variable)) =
                        (&kase.pattern, scrutinee.var_name())
                    {
                        if !bound.contains(&variable) {
                            inner.insert(name.clone(), variable.to_owned());
                        }
                    }
                    Case {
                        pattern: kase.pattern.clone(),
                        body: reuse_successors(&kase.body, &inner),
                    }
                })
                .collect();
            Expr {
                node: Node::Match {
                    scrutinee: Box::new(reuse_successors(scrutinee, predecessors)),
                    cases,
                },
                ..expr.clone()
            }
        }
        _ => expr.map_children(&mut |child| reuse_successors(child, predecessors)),
    }
}

fn without(map: &HashMap<String, String>, names: &[&str]) -> HashMap<String, String> {
    map.iter()
        .filter(|(key, value)| !names.contains(&key.as_str()) && !names.contains(&value.as_str()))
        .map(|(key, value)| (key.clone(), value.clone()))
        .collect()
}

/// Marks each function recursive or not and, for recursive functions, finds a
/// structurally decreasing parameter. Lean and Rocq targets require one; the
/// other targets accept general recursion.
fn analyse_recursion(declarations: &mut [Decl]) {
    for decl in declarations.iter_mut() {
        let Decl::Fn(entry) = decl else { continue };
        let mut calls = Vec::new();
        collect_calls(&entry.body, &mut calls);
        entry.recursive = calls.contains(&entry.full_name);
        entry.decreasing = if entry.recursive {
            structural_parameter(entry)
        } else {
            None
        };
        if entry.recursive && entry.decreasing.is_none() {
            expose_natural_cases(entry);
        }
    }
    let mut graph: HashMap<String, HashSet<String>> = HashMap::new();
    for decl in declarations.iter() {
        let Decl::Fn(entry) = decl else { continue };
        let mut calls = Vec::new();
        collect_calls(&entry.body, &mut calls);
        graph.insert(
            entry.full_name.clone(),
            calls
                .into_iter()
                .filter(|call| *call != entry.full_name)
                .collect(),
        );
    }
    for decl in declarations.iter_mut() {
        let Decl::Fn(entry) = decl else { continue };
        let targets = &graph[&entry.full_name];
        if targets
            .iter()
            .any(|target| reaches(&graph, target, &entry.full_name, &mut HashSet::new()))
        {
            entry.mutual = true;
        }
    }
}

fn reaches(
    graph: &HashMap<String, HashSet<String>>,
    from: &str,
    to: &str,
    seen: &mut HashSet<String>,
) -> bool {
    if from == to {
        return true;
    }
    if !seen.insert(from.to_owned()) {
        return false;
    }
    graph
        .get(from)
        .is_some_and(|targets| targets.iter().any(|next| reaches(graph, next, to, seen)))
}

/// The functions an expression calls, in evaluation order.
pub fn collect_calls(expr: &Expr, calls: &mut Vec<String>) {
    if let Node::Call { func, .. } = &expr.node {
        calls.push(func.clone());
    }
    for child in expr.children() {
        collect_calls(child, calls);
    }
}

fn structural_parameter(entry: &FnDecl) -> Option<usize> {
    entry.params.iter().enumerate().position(|(index, param)| {
        decreases_on(
            &entry.body,
            &entry.full_name,
            index,
            &param.name,
            &HashSet::new(),
        )
    })
}

/// Every recursive call's argument at `index` must be a strict subterm of `name`.
fn decreases_on(
    expr: &Expr,
    func: &str,
    index: usize,
    name: &str,
    subterms: &HashSet<String>,
) -> bool {
    match &expr.node {
        Node::Call { func: callee, args } => {
            if callee == func {
                let decreasing = args
                    .get(index)
                    .and_then(Expr::var_name)
                    .is_some_and(|arg| subterms.contains(arg));
                if !decreasing {
                    return false;
                }
            }
            args.iter()
                .all(|arg| decreases_on(arg, func, index, name, subterms))
        }
        Node::Match { scrutinee, cases } => {
            if !decreases_on(scrutinee, func, index, name, subterms) {
                return false;
            }
            let is_param = scrutinee
                .var_name()
                .is_some_and(|variable| variable == name || subterms.contains(variable));
            cases.iter().all(|kase| {
                let mut inner = subterms.clone();
                if is_param {
                    match &kase.pattern {
                        Pattern::NatSucc { name } => {
                            inner.insert(name.clone());
                        }
                        Pattern::Ctor { binds, .. } => {
                            inner.extend(binds.iter().flatten().cloned());
                        }
                        _ => {}
                    }
                }
                decreases_on(&kase.body, func, index, name, &inner)
            })
        }
        Node::Let {
            name: local,
            value,
            body,
        } => {
            if !decreases_on(value, func, index, name, subterms) {
                return false;
            }
            let mut inner = subterms.clone();
            inner.remove(local);
            // `let k := subterm` (introduced by pattern compilation) is itself a subterm.
            if value
                .var_name()
                .is_some_and(|variable| subterms.contains(variable))
            {
                inner.insert(local.clone());
            }
            decreases_on(body, func, index, name, &inner)
        }
        _ => expr
            .children()
            .into_iter()
            .all(|child| decreases_on(child, func, index, name, subterms)),
    }
}

/// `if n == 0 then a else b` on a natural or unsigned machine-integer
/// parameter is the case split `match n with 0 => a | succ p => b'`, where
/// `b'` reads `n - k` as `p - (k - 1)` and `n == k` as `p == k - 1`: in `b`,
/// `n` is at least 1, so both agree with truncated subtraction, with checked
/// machine subtraction, and with a checked conversion of `n - k` back to a
/// natural. A `let m := n` in `b` is an alias read the same way. Writing the
/// split explicitly makes recursion such as `n * fact (n - 1)` structural,
/// which Lean and Rocq require.
fn expose_natural_cases(entry: &mut FnDecl) {
    for index in 0..entry.params.len() {
        let param = &entry.params[index];
        if !param.ty.is_natural() {
            continue;
        }
        let mut fresh = 0;
        let Some(body) = split_natural(&entry.body, &param.name, &mut fresh) else {
            continue;
        };
        if decreases_on(&body, &entry.full_name, index, &param.name, &HashSet::new()) {
            entry.body = body;
            entry.decreasing = Some(index);
            return;
        }
    }
}

/// A natural literal's value.
fn nat_literal(expr: &Expr) -> Option<Decimal> {
    match &expr.node {
        Node::Lit {
            value: LitValue::Text(text),
        } if !text.is_empty() && text.bytes().all(|byte| byte.is_ascii_digit()) => {
            Decimal::parse(text)
        }
        _ => None,
    }
}

/// `k - 1` for a positive canonical decimal `k`.
fn decrement(value: &Decimal) -> String {
    let mut digits: Vec<u8> = value.digits.bytes().collect();
    for digit in digits.iter_mut().rev() {
        if *digit == b'0' {
            *digit = b'9';
        } else {
            *digit -= 1;
            break;
        }
    }
    let text = String::from_utf8(digits).unwrap_or_default();
    let trimmed = text.trim_start_matches('0');
    if trimmed.is_empty() {
        "0".to_owned()
    } else {
        trimmed.to_owned()
    }
}

/// `n == 0`, `0 == n`: whether the zero test selects the `then` branch.
fn zero_test(cond: &Expr, name: &str) -> Option<bool> {
    let Node::Binary {
        op: op @ (BinaryOp::Eq | BinaryOp::Ne),
        left,
        right,
        domain: Some(domain),
        ..
    } = &cond.node
    else {
        return None;
    };
    if !domain.is_natural() {
        return None;
    }
    let zero_on = |a: &Expr, b: &Expr| {
        a.var_name() == Some(name) && nat_literal(b).is_some_and(|value| value.digits == "0")
    };
    (zero_on(left, right) || zero_on(right, left)).then_some(*op == BinaryOp::Eq)
}

/// The locals a match binds in any case.
fn binds(cases: &[Case], name: &str) -> bool {
    cases
        .iter()
        .any(|kase| kase.pattern.bound().contains(&name))
}

/// Splits zero tests on `name` into natural case splits; `None` when nothing changed.
fn split_natural(expr: &Expr, name: &str, fresh: &mut usize) -> Option<Expr> {
    if let Node::If {
        cond,
        then,
        otherwise,
    } = &expr.node
    {
        if let Some(zero_then) = zero_test(cond, name) {
            let (zero, positive) = if zero_then {
                (then, otherwise)
            } else {
                (otherwise, then)
            };
            *fresh += 1;
            let pred = format!("ml_p{fresh}");
            let ty = match &cond.node {
                Node::Binary {
                    domain: Some(domain),
                    ..
                } => domain.clone(),
                _ => NAT,
            };
            let succ_body = predecessor_of(positive, name, &pred, &ty);
            let succ_body = split_or_same(&succ_body, &pred, fresh);
            let succ_body = split_or_same(&succ_body, name, fresh);
            let zero = split_or_same(zero, name, fresh);
            return Some(Expr {
                node: Node::Match {
                    scrutinee: Box::new(Expr::var(name, ty).with_span(cond.span)),
                    cases: vec![
                        Case {
                            pattern: Pattern::NatZero,
                            body: zero,
                        },
                        Case {
                            pattern: Pattern::NatSucc { name: pred },
                            body: succ_body,
                        },
                    ],
                },
                ty: expr.ty.clone(),
                span: expr.span,
            });
        }
    }
    match &expr.node {
        Node::Let { name: local, .. } if local == name => return None,
        Node::Match { cases, .. } if binds(cases, name) => return None,
        _ => {}
    }
    let mut changed = false;
    let copy = expr.map_children(&mut |child| {
        split_natural(child, name, fresh).map_or_else(
            || child.clone(),
            |next| {
                changed = true;
                next
            },
        )
    });
    changed.then_some(copy)
}

fn split_or_same(expr: &Expr, name: &str, fresh: &mut usize) -> Expr {
    split_natural(expr, name, fresh).unwrap_or_else(|| expr.clone())
}

/// Rewrites uses of `name - k` and `name == k` (k ≥ 1) in terms of
/// `pred = name - 1`, including through `let` aliases of `name`.
fn predecessor_of(expr: &Expr, name: &str, pred: &str, ty: &Type) -> Expr {
    let names = HashSet::from([name.to_owned()]);
    Predecessor { pred, ty }.visit(expr, &names)
}

struct Predecessor<'a> {
    pred: &'a str,
    ty: &'a Type,
}

impl Predecessor<'_> {
    fn pred_var(&self) -> Expr {
        Expr::var(self.pred, self.ty.clone())
    }

    fn minus(&self, k: &Decimal, like: &Expr) -> Expr {
        if k.digits == "1" {
            return self.pred_var();
        }
        let Node::Binary {
            op,
            semantics,
            rounding,
            by_zero,
            ..
        } = &like.node
        else {
            return like.clone();
        };
        Expr {
            node: binary_node(
                *op,
                Box::new(self.pred_var()),
                Box::new(text_lit(self.ty.clone(), decrement(k))),
                Some(self.ty.clone()),
                *semantics,
                *rounding,
                *by_zero,
            ),
            ty: self.ty.clone(),
            span: like.span,
        }
    }

    fn visit(&self, node: &Expr, names: &HashSet<String>) -> Expr {
        let is_alias = |candidate: &Expr| {
            candidate
                .var_name()
                .is_some_and(|name| names.contains(name))
        };
        if let Node::Binary {
            op,
            left,
            right,
            domain: Some(domain),
            semantics,
            rounding,
            by_zero,
        } = &node.node
        {
            if domain.is_natural() && is_alias(left) {
                if let Some(k) = nat_literal(right).filter(|k| k.digits != "0") {
                    if *op == BinaryOp::Sub {
                        return self.minus(&k, node);
                    }
                    if matches!(op, BinaryOp::Eq | BinaryOp::Ne) {
                        return Expr {
                            node: binary_node(
                                *op,
                                Box::new(self.pred_var()),
                                Box::new(text_lit(self.ty.clone(), decrement(&k))),
                                Some(domain.clone()),
                                *semantics,
                                *rounding,
                                *by_zero,
                            ),
                            ..node.clone()
                        };
                    }
                }
            }
        }
        // JavaScript's `n - 1n` on a natural `n` is the checked conversion of
        // an integer difference; with n ≥ 1 it is the natural predecessor.
        if let Node::Cast {
            arg, to: Type::Nat, ..
        } = &node.node
        {
            if let Node::Binary {
                op: BinaryOp::Sub,
                left,
                right,
                ..
            } = &arg.node
            {
                if let Node::Cast {
                    arg: inner,
                    from: Type::Nat,
                    ..
                } = &left.node
                {
                    if is_alias(inner) {
                        if let Some(k) = nat_literal(right).filter(|k| k.digits != "0") {
                            let like = Expr {
                                node: binary_node(
                                    BinaryOp::Sub,
                                    Box::new(self.pred_var()),
                                    Box::new(self.pred_var()),
                                    None,
                                    Some(Semantics::Truncated),
                                    None,
                                    None,
                                ),
                                ty: self.ty.clone(),
                                span: node.span,
                            };
                            return self.minus(&k, &like);
                        }
                    }
                }
            }
        }
        match &node.node {
            Node::Let {
                name: local,
                value,
                body,
            } => {
                let mut inner = names.clone();
                inner.remove(local);
                let value_visited = self.visit(value, names);
                if local == self.pred {
                    return Expr {
                        node: Node::Let {
                            name: local.clone(),
                            value: Box::new(value_visited),
                            body: body.clone(),
                        },
                        ..node.clone()
                    };
                }
                if is_alias(value) {
                    inner.insert(local.clone());
                }
                Expr {
                    node: Node::Let {
                        name: local.clone(),
                        value: Box::new(value_visited),
                        body: Box::new(self.visit(body, &inner)),
                    },
                    ..node.clone()
                }
            }
            Node::Match { scrutinee, cases } => {
                let scrutinee = Box::new(self.visit(scrutinee, names));
                if binds(cases, self.pred) {
                    return Expr {
                        node: Node::Match {
                            scrutinee,
                            cases: cases.clone(),
                        },
                        ..node.clone()
                    };
                }
                let cases = cases
                    .iter()
                    .map(|kase| {
                        let bound = kase.pattern.bound();
                        let inner: HashSet<String> = names
                            .iter()
                            .filter(|candidate| !bound.contains(&candidate.as_str()))
                            .cloned()
                            .collect();
                        Case {
                            pattern: kase.pattern.clone(),
                            body: self.visit(&kase.body, &inner),
                        }
                    })
                    .collect();
                Expr {
                    node: Node::Match { scrutinee, cases },
                    ..node.clone()
                }
            }
            _ => node.map_children(&mut |child| self.visit(child, names)),
        }
    }
}
