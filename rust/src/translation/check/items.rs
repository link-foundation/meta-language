//! Declarations, item and type resolution, `main`, propositions and unification.

use super::arrays::array_text;
use super::{
    BOOL, BinaryOp, Binder, Checker, Comparison, Ctor, DataDecl, Decl, Effect, Entry, Env, Expr,
    Field, FnDecl, HashMap, Hints, INT, Item, Language, Main, NAT, Node, Param, Plan, Proof,
    ProofScope, Prop, Result, SEffect, SExpr, SItem, SMain, SProp, SPropNode, STRING, Scope,
    ShowStyle, Span, TheoremDecl, TheoremHead, Type, UNIT, array, build_items, coerce, data, fixed,
    flatten, full, normalise_proof, plain_binary, text_lit, type_error, unsupported,
};

impl Checker {
    pub(super) fn declare(&mut self, items: &[SItem], path: &[String]) -> Result<()> {
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
            let generated = match item {
                SItem::Fn(function) => function.generated,
                SItem::Data(data) => data.generated,
                _ => false,
            };
            if name.starts_with("ml_") && !generated {
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
                SItem::Data(data) => Decl::Data(DataDecl {
                    name: name.clone(),
                    ctors: Vec::new(),
                    span,
                    generated: data.generated,
                    output: false,
                    full_name: full_name.clone(),
                    module_path,
                }),
                SItem::Fn(function) => Decl::Fn(FnDecl {
                    name: name.clone(),
                    params: Vec::new(),
                    ret: UNIT,
                    body: Expr::new(Node::Unit, UNIT),
                    span,
                    generated: function.generated,
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

    pub(super) fn check_items(&mut self, items: &[SItem]) -> Result<Vec<Item>> {
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
                    guard: param.guard.clone(),
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

    pub(super) fn resolve_type(
        &self,
        ty: Option<&Type>,
        path: &[String],
        span: Option<Span>,
    ) -> Result<Type> {
        let Some(ty) = ty else {
            return Err(type_error("missing type annotation", span));
        };
        if let Some(element) = ty.element() {
            return Ok(array(self.resolve_type(Some(element), path, span)?));
        }
        let Type::Named {
            path: segments,
            span: type_span,
        } = ty
        else {
            return Ok(ty.clone());
        };
        if let Some(Entry::Decl(name)) = self.lookup(segments, path, span)?
            && matches!(self.decls.get(&name), Some(Decl::Data(_)))
        {
            return Ok(data(name));
        }
        Err(type_error(
            format!("unknown type {}", segments.join(".")),
            type_span.or(span),
        ))
    }

    pub(super) fn lookup(
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

    pub(super) fn lookup_from(&self, module_path: &[String], names: &[String]) -> Option<Entry> {
        let mut scope = self.modules.get(&module_path.join("."))?;
        for index in 0..names.len().saturating_sub(1) {
            let rest = names[index..].join(".");
            if index == names.len() - 2
                && let Some(entry) = scope.names.get(&rest)
            {
                return Some(entry.clone());
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
            if let Some(Entry::Decl(name)) = parent.names.get(sibling)
                && let Some(Decl::Data(decl)) = self.decls.get(name)
            {
                return parent.names.get(&format!("{}.{last}", decl.name)).cloned();
            }
        }
        None
    }

    pub(super) fn data_decl(&self, name: &str) -> Result<&DataDecl> {
        match self.decls.get(name) {
            Some(Decl::Data(decl)) => Ok(decl),
            _ => Err(type_error(format!("unknown type {name}"), None)),
        }
    }

    pub(super) fn check_main(&mut self, main: &SMain) -> Result<Main> {
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
            sequential_async: main.sequential_async,
        })
    }

    pub(super) fn show(
        &mut self,
        expr: &SExpr,
        env: &Env,
        path: &[String],
        style: ShowStyle,
    ) -> Result<Expr> {
        let value = self.expr(expr, env, path, None, false)?;
        let bigint = matches!(value.ty, Type::Int | Type::Nat);
        refuse_format(style, &value.ty, bigint, expr.span)?;
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
                ));
            }
            Type::Array { .. } => return Err(array_text(expr.span)),
            _ => {}
        }
        let console_bigint = style.is_console() && bigint;
        // console.log prints -0 as "-0", where String(-0) is "0".
        let console = style.is_console() && value.ty.is_float();
        let text = Expr::new(
            Node::ToString {
                arg: Box::new(value),
                console,
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

    pub(super) fn prop(&mut self, prop: &SProp, env: &Env, path: &[String]) -> Result<Prop> {
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
                if checked.iter().any(|binder| binder.ty.is_float()) {
                    return Err(unsupported(
                        "quantification over Numbers",
                        "a theorem quantifies over integers, naturals, booleans, strings or data",
                        prop.span,
                    ));
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
                if !matches!(op, BinaryOp::Eq | BinaryOp::Ne) && !left.ty.is_ordered() {
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
                // Over Numbers, assert.strictEqual's SameValue differs from ===: NaN equals NaN, and 0 differs from -0.
                let domain = left.ty.clone();
                let same_value = pair.same_value
                    && domain.is_float()
                    && matches!(op, BinaryOp::Eq | BinaryOp::Ne);
                Prop::compare(
                    op,
                    Comparison {
                        left,
                        right,
                        domain,
                        same_value,
                    },
                )
            }
        })
    }

    pub(super) fn operands(
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

    pub(super) const fn default_number(&self) -> Type {
        match self.language {
            Language::JavaScript => INT,
            Language::Rust => fixed(32, true),
            Language::Lean | Language::Rocq => NAT,
        }
    }

    pub(super) fn fresh_name(&mut self, prefix: &str) -> String {
        self.fresh += 1;
        format!("{prefix}{}", self.fresh)
    }

    pub(super) fn unify(
        &self,
        left: Expr,
        right: Expr,
        span: Option<Span>,
    ) -> Result<(Expr, Expr)> {
        if left.ty.same(&right.ty) {
            return Ok((left, right));
        }
        if left.ty.is_float() || right.ty.is_float() {
            if self.language == Language::JavaScript
                && (left.ty.is_numeric() || right.ty.is_numeric())
            {
                return Err(type_error(
                    "mixing BigInt and Number: JavaScript throws \"TypeError: Cannot mix BigInt and other types, use explicit conversions\"",
                    span,
                ));
            }
        } else if self.language == Language::JavaScript {
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

    pub(super) fn unify_branches(
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
}

/// Refuses the `console.log` arguments whose `util.format` rendering is not kept.
fn refuse_format(style: ShowStyle, ty: &Type, bigint: bool, place: Option<Span>) -> Result<()> {
    match style {
        ShowStyle::JsFormatFirst if *ty == Type::String => Err(unsupported(
            "console.log with a computed first string",
            "util.format reads the % directives of a first string argument, which only the run knows; pass a literal format string, or print one template literal",
            place,
        )),
        ShowStyle::JsFormatNumber if !bigint && !ty.is_float() => Err(unsupported(
            &format!("%d of a {}", ty.key()),
            "%d prints a BigInt or a Number; the conversion Number(value) of other values is not kept",
            place,
        )),
        ShowStyle::JsFormatInteger if !bigint => Err(unsupported(
            &format!("%i of a {}", ty.key()),
            "%i prints a BigInt; the conversion parseInt(value) of other values is not kept",
            place,
        )),
        _ => Ok(()),
    }
}
