//! Types of JavaScript functions written without `JSDoc`.
//!
//! A parameter or result without a declared type gets a type variable; the
//! bodies, the calls and the top-level statements constrain the variables by
//! unification, and each variable takes the one type every use agrees on. A
//! `BigInt` literal is a `bigint` of any integer type, `+` of a string is
//! concatenation, and a type nothing constrains is a Number. Two uses that
//! need different types are a type error: the function would need a declared
//! type for each use.
//!
//! Mirrors `js/src/translation/javascript-infer.js`.

mod arrays;

use arrays::fill_arrays;
use std::collections::{HashMap, HashSet};

use super::{
    BOOL, BinaryOp, FLOAT, INT, ROOT, Result, SCtor, SEffect, SExpr, SFn, SItem, SNode, SPattern,
    SPatternNode, SProgram, SProp, SPropNode, STRING, Span, Type, UnaryOp, type_error,
};
use crate::translation::frontend_rules::accept_argument_count;
use crate::translation::types::UNIT;

/// Fills in the missing parameter and result types of the functions of a parsed JavaScript program.
pub(super) fn infer_javascript_types(mut program: SProgram) -> Result<SProgram> {
    let mut functions = Vec::new();
    collect_functions(&program.items, &[ROOT.to_owned()], &mut functions);
    let untyped = program.items.iter().any(|item| {
        matches!(item, SItem::Data(data) if data.ctors.iter().any(|ctor| {
            ctor.fields.iter().any(|field| field.ty == Type::Literal)
        }))
    });
    if !untyped
        && functions.iter().all(|(_, function)| {
            function.ret.is_some() && function.params.iter().all(|param| param.ty.is_some())
        })
    {
        return Ok(program);
    }
    let mut inference = Inference::new(&program.items);
    let inferred = inference.run(&functions, &program)?;
    let arrays: HashMap<*const SExpr, Type> = inference
        .empty_arrays
        .iter()
        .map(|(node, (term, place))| (*node, inference.surface(term, *place)))
        .collect();
    let mut inferred = inferred.into_iter();
    fill_functions(&mut program.items, &mut inferred);
    fill_fields(&mut program.items, &inference);
    if !arrays.is_empty() {
        fill_arrays(&mut program, &arrays);
    }
    Ok(program)
}

/// Writes the inferred types of the fields the translator makes up back.
fn fill_fields(items: &mut [SItem], inference: &Inference) {
    for item in items {
        let SItem::Data(data) = item else {
            continue;
        };
        for ctor in &mut data.ctors {
            for (index, field) in ctor.fields.iter_mut().enumerate() {
                let key = (data.name.clone(), ctor.name.clone(), index);
                if let Some(term) = inference.field_terms.get(&key) {
                    field.ty = inference.surface(term, field.span);
                }
            }
        }
    }
}

fn collect_functions<'a>(
    items: &'a [SItem],
    path: &[String],
    into: &mut Vec<(Vec<String>, &'a SFn)>,
) {
    for item in items {
        match item {
            SItem::Fn(function) => {
                let mut full = path.to_vec();
                full.push(function.name.clone());
                into.push((full, function));
            }
            SItem::Module(module) => {
                let mut full = path.to_vec();
                full.push(module.name.clone());
                collect_functions(&module.items, &full, into);
            }
            SItem::Data(_) | SItem::Theorem(_) => {}
        }
    }
}

/// Writes the inferred types back, visiting the functions in the order they were collected.
fn fill_functions(items: &mut [SItem], inferred: &mut impl Iterator<Item = (Vec<Type>, Type)>) {
    for item in items {
        match item {
            SItem::Fn(function) => {
                let Some((params, ret)) = inferred.next() else {
                    return;
                };
                for (param, ty) in function.params.iter_mut().zip(params) {
                    if param.ty.is_none() {
                        param.ty = Some(ty);
                    }
                }
                if function.ret.is_none() {
                    function.ret = Some(ret);
                }
            }
            SItem::Module(module) => fill_functions(&mut module.items, inferred),
            SItem::Data(_) | SItem::Theorem(_) => {}
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Term {
    Var(usize),
    Known(Type),
    /// An array, whose element type may still be unknown.
    Array(Box<Self>),
}

#[derive(Clone)]
struct Signature {
    params: Vec<Term>,
    defaults: Vec<bool>,
    ret: Term,
}

struct Plus {
    left: Term,
    right: Term,
    result: Term,
    place: Option<Span>,
}

struct FieldOf {
    object: Term,
    field: String,
    result: Term,
    place: Option<Span>,
}

struct Inference {
    /// Constructors of every data type, in declaration order.
    data: Vec<(String, Vec<SCtor>)>,
    bindings: Vec<Option<Term>>,
    bigints: HashSet<usize>,
    plus: Vec<Plus>,
    fields: Vec<FieldOf>,
    signatures: HashMap<String, Signature>,
    context: Vec<String>,
    /// The types of the fields the translator makes up, which are inferred
    /// like parameters, by data type, constructor and position.
    field_terms: HashMap<(String, String, usize), Term>,
    /// Array literals with no element of their own, whose element type the
    /// rest of the program fixes, by node.
    empty_arrays: HashMap<*const SExpr, (Term, Option<Span>)>,
}

type Env = HashMap<String, Term>;

impl Inference {
    fn new(items: &[SItem]) -> Self {
        let data = items
            .iter()
            .filter_map(|item| match item {
                SItem::Data(data) => Some((data.name.clone(), data.ctors.clone())),
                _ => None,
            })
            .collect();
        Self {
            data,
            bindings: Vec::new(),
            bigints: HashSet::new(),
            plus: Vec::new(),
            fields: Vec::new(),
            signatures: HashMap::new(),
            context: vec![ROOT.to_owned()],
            field_terms: HashMap::new(),
            empty_arrays: HashMap::new(),
        }
    }

    /// The type of a constructor field as a term, one per field even when it is not declared.
    fn field(&mut self, data: &str, ctor: &str, index: usize) -> Term {
        let declared = self
            .data
            .iter()
            .find(|(name, _)| name == data)
            .and_then(|(_, ctors)| ctors.iter().find(|candidate| candidate.name == ctor))
            .and_then(|ctor| ctor.fields.get(index))
            .map(|field| field.ty.clone());
        match declared {
            Some(ty) if ty != Type::Literal => self.declared(Some(&ty)),
            _ => {
                let key = (data.to_owned(), ctor.to_owned(), index);
                if let Some(term) = self.field_terms.get(&key) {
                    return term.clone();
                }
                let term = self.fresh(false);
                self.field_terms.insert(key, term.clone());
                term
            }
        }
    }

    fn fresh(&mut self, bigint: bool) -> Term {
        let id = self.bindings.len();
        self.bindings.push(None);
        if bigint {
            self.bigints.insert(id);
        }
        Term::Var(id)
    }

    fn resolve(&self, term: &Term) -> Term {
        let mut current = term.clone();
        while let Term::Var(id) = current {
            match &self.bindings[id] {
                Some(next) => current = next.clone(),
                None => break,
            }
        }
        current
    }

    /// A declared surface type as a term; the frontend writes `bigint` as `int`.
    fn declared(&mut self, ty: Option<&Type>) -> Term {
        match ty {
            None => self.fresh(false),
            Some(Type::Named { path, .. }) => Term::Known(Type::Data {
                name: path.last().cloned().unwrap_or_default(),
            }),
            Some(Type::Nat) => Term::Known(INT),
            Some(Type::Array { element }) => Term::Array(Box::new(self.declared(Some(element)))),
            Some(other) => Term::Known(other.clone()),
        }
    }

    fn unify(&mut self, left: &Term, right: &Term, place: Option<Span>) -> Result<()> {
        let a = self.resolve(left);
        let b = self.resolve(right);
        // Only a function that finishes without a return value makes
        // undefined, the unit value.
        let other = match (&a, &b) {
            (Term::Known(Type::Unit), other) | (other, Term::Known(Type::Unit)) => Some(other),
            _ => None,
        };
        let described = match other {
            Some(Term::Known(Type::Unit)) | None => None,
            Some(Term::Var(variable)) => self
                .bigints
                .contains(variable)
                .then(|| "a bigint".to_owned()),
            Some(Term::Known(ty)) => Some(describe(ty)),
            Some(Term::Array(_)) => Some("an array".to_owned()),
        };
        if let Some(described) = described {
            return Err(type_error(
                format!(
                    "the function returns {described} on one path and finishes without a return value, returning undefined, on another; return a value on every path"
                ),
                place,
            ));
        }
        match (&a, &b) {
            (Term::Var(x), Term::Var(y)) => {
                if x != y {
                    if self.bigints.contains(x) {
                        self.bigints.insert(*y);
                    }
                    self.bindings[*x] = Some(b.clone());
                }
                Ok(())
            }
            (Term::Var(variable), other) | (other, Term::Var(variable)) => {
                let integer = matches!(other, Term::Known(ty) if is_integer(ty));
                if self.bigints.contains(variable) && !integer {
                    return Err(type_error(
                        format!(
                            "a BigInt is used as {}; JavaScript does not mix BigInt with other types",
                            describe_term(other)
                        ),
                        place,
                    ));
                }
                self.bindings[*variable] = Some(other.clone());
                Ok(())
            }
            (Term::Array(x), Term::Array(y)) => self.unify(x, y, place),
            (Term::Known(x), Term::Known(y)) if x == y || (is_integer(x) && is_integer(y)) => {
                Ok(())
            }
            _ => Err(type_error(
                format!(
                    "one value is used as {} and as {}; declare the types of the function with JSDoc",
                    describe_term(&a),
                    describe_term(&b)
                ),
                place,
            )),
        }
    }

    fn run(
        &mut self,
        functions: &[(Vec<String>, &SFn)],
        program: &SProgram,
    ) -> Result<Vec<(Vec<Type>, Type)>> {
        for (path, function) in functions {
            let params = function
                .params
                .iter()
                .map(|param| self.declared(param.ty.as_ref()))
                .collect();
            let ret = self.declared(function.ret.as_ref());
            let defaults = function
                .params
                .iter()
                .map(|param| param.default_value.is_some())
                .collect();
            self.signatures.insert(
                path.join("."),
                Signature {
                    params,
                    defaults,
                    ret,
                },
            );
        }
        for (path, function) in functions {
            let signature = self.signatures[&path.join(".")].clone();
            let ret = signature.ret.clone();
            let env: Env = function
                .params
                .iter()
                .zip(&signature.params)
                .map(|(param, term)| (param.name.clone(), term.clone()))
                .collect();
            self.context = path[..path.len() - 1].to_vec();
            for (param, term) in function.params.iter().zip(&signature.params) {
                if let Some(default_value) = &param.default_value {
                    let value = self.expr(default_value, &Env::new())?;
                    self.unify(&value, term, param.span)?;
                }
            }
            let body = self.expr(&function.body, &env)?;
            self.unify(&body, &ret, function.span)?;
        }
        self.context = vec![ROOT.to_owned()];
        let mut env = Env::new();
        for effect in program.main.iter().flat_map(|main| &main.effects) {
            match effect {
                SEffect::Print { expr, .. } => {
                    self.expr(expr, &env)?;
                }
                SEffect::Let { name, value, .. } => {
                    let term = self.expr(value, &env)?;
                    env.insert(name.clone(), term);
                }
                SEffect::Assert { prop, .. } => self.prop(prop, &env)?,
            }
        }
        self.solve()?;
        Ok(functions
            .iter()
            .map(|(path, function)| {
                let signature = &self.signatures[&path.join(".")];
                let params = signature
                    .params
                    .iter()
                    .map(|term| self.surface(term, function.span))
                    .collect();
                (params, self.surface(&signature.ret, function.span))
            })
            .collect())
    }

    /// Deferred `+` and field constraints, until nothing more is known; then `+` of unknowns adds numbers.
    fn solve(&mut self) -> Result<()> {
        let mut progress = true;
        while progress {
            progress = false;
            let mut waiting = Vec::new();
            for constraint in std::mem::take(&mut self.plus) {
                if self.plus_step(&constraint, false)? {
                    progress = true;
                } else {
                    waiting.push(constraint);
                }
            }
            self.plus = waiting;
            let mut waiting = Vec::new();
            for constraint in std::mem::take(&mut self.fields) {
                if self.field_step(&constraint)? {
                    progress = true;
                } else {
                    waiting.push(constraint);
                }
            }
            self.fields = waiting;
            // `.length` of a value nothing else fixes reads an array.
            let length = self.fields.iter().find(|constraint| {
                constraint.field == "length"
                    && matches!(self.resolve(&constraint.object), Term::Var(_))
            });
            if let (false, Some(length)) = (progress, length) {
                let (object, place) = (length.object.clone(), length.place);
                let array = Term::Array(Box::new(self.fresh(false)));
                self.unify(&object, &array, place)?;
                progress = true;
            }
            if !progress && !self.plus.is_empty() {
                let first = self.plus.remove(0);
                self.plus_step(&first, true)?;
                progress = true;
            }
        }
        Ok(())
    }

    fn plus_step(&mut self, constraint: &Plus, force: bool) -> Result<bool> {
        let a = self.resolve(&constraint.left);
        let b = self.resolve(&constraint.right);
        if a == Term::Known(STRING) || b == Term::Known(STRING) {
            self.unify(&constraint.result, &Term::Known(STRING), constraint.place)?;
            return Ok(true);
        }
        if !force && (matches!(a, Term::Var(_)) || matches!(b, Term::Var(_))) {
            return Ok(false);
        }
        self.unify(&constraint.left, &constraint.right, constraint.place)?;
        self.unify(&constraint.result, &constraint.left, constraint.place)?;
        Ok(true)
    }

    fn field_step(&mut self, constraint: &FieldOf) -> Result<bool> {
        let name = match self.resolve(&constraint.object) {
            Term::Known(Type::Data { name }) => name,
            Term::Array(_) if constraint.field == "length" => {
                self.unify(&constraint.result, &Term::Known(FLOAT), constraint.place)?;
                return Ok(true);
            }
            other => {
                let owners: Vec<&String> = self
                    .data
                    .iter()
                    .filter(|(_, ctors)| {
                        ctors.iter().any(|ctor| {
                            ctor.fields.iter().any(|field| {
                                field.name.as_deref() == Some(constraint.field.as_str())
                            })
                        })
                    })
                    .map(|(name, _)| name)
                    .collect();
                if !matches!(other, Term::Var(_)) || owners.len() != 1 {
                    return Ok(false);
                }
                let owner = Term::Known(Type::Data {
                    name: owners[0].clone(),
                });
                self.unify(&constraint.object, &owner, constraint.place)?;
                return self.field_step(constraint);
            }
        };
        let declared = self
            .data
            .iter()
            .find(|(data, _)| *data == name)
            .and_then(|(_, ctors)| {
                ctors.iter().find_map(|ctor| {
                    ctor.fields
                        .iter()
                        .position(|field| field.name.as_deref() == Some(constraint.field.as_str()))
                        .map(|index| (ctor.name.clone(), index))
                })
            });
        if let Some((ctor, index)) = declared {
            let term = self.field(&name, &ctor, index);
            self.unify(&constraint.result, &term, constraint.place)?;
        }
        Ok(true)
    }

    /// The inferred type as a surface type; an unconstrained `BigInt` is `bigint`, anything else unconstrained a Number.
    fn surface(&self, term: &Term, place: Option<Span>) -> Type {
        match self.resolve(term) {
            Term::Var(id) => {
                if self.bigints.contains(&id) {
                    INT
                } else {
                    FLOAT
                }
            }
            Term::Known(Type::Data { name }) => Type::Named {
                path: vec![ROOT.to_owned(), name],
                span: place,
            },
            Term::Known(ty) => ty,
            Term::Array(element) => Type::Array {
                element: Box::new(self.surface(&element, place)),
            },
        }
    }

    fn signature(&self, path: &[String]) -> Option<Signature> {
        let found = self.signatures.get(&path.join(".")).or_else(|| {
            // A call inside a namespace may name a sibling relative to it.
            (1..=self.context.len()).rev().find_map(|depth| {
                let mut candidate = self.context[..depth].to_vec();
                candidate.extend_from_slice(path.get(1..).unwrap_or_default());
                self.signatures.get(&candidate.join("."))
            })
        })?;
        Some(found.clone())
    }

    fn expr(&mut self, expr: &SExpr, env: &Env) -> Result<Term> {
        match &expr.node {
            // The 1 of `x++` has the type of `x`, a Number or a BigInt.
            SNode::Num { unit: true, .. } => Ok(self.fresh(false)),
            SNode::Num { ty, .. } => Ok(match ty {
                Some(ty) => self.declared(Some(ty)),
                None => self.fresh(true),
            }),
            SNode::Bool { .. } => Ok(Term::Known(BOOL)),
            SNode::Str { .. } => Ok(Term::Known(STRING)),
            SNode::Unit => Ok(Term::Known(UNIT)),
            SNode::Name { path } => {
                if let [name] = path.as_slice()
                    && let Some(term) = env.get(name)
                {
                    return Ok(term.clone());
                }
                self.call(path, &[], env, expr.span)
            }
            SNode::App { func, args } => match &func.node {
                SNode::Name { path } => self.call(path, args, env, expr.span),
                _ => Ok(self.fresh(false)),
            },
            SNode::Field { object, field } => {
                let object = self.expr(object, env)?;
                let result = self.fresh(false);
                self.fields.push(FieldOf {
                    object,
                    field: field.clone(),
                    result: result.clone(),
                    place: expr.span,
                });
                Ok(result)
            }
            SNode::Unary { op, arg } => {
                let arg = self.expr(arg, env)?;
                if *op == UnaryOp::Not {
                    self.unify(&arg, &Term::Known(BOOL), expr.span)?;
                    return Ok(Term::Known(BOOL));
                }
                Ok(arg)
            }
            SNode::Binary {
                op, left, right, ..
            } => self.binary(*op, left, right, expr.span, env),
            SNode::If {
                cond,
                then,
                otherwise,
            } => {
                let condition = self.expr(cond, env)?;
                self.unify(&condition, &Term::Known(BOOL), cond.span.or(expr.span))?;
                let then = self.expr(then, env)?;
                let otherwise = self.expr(otherwise, env)?;
                self.unify(&otherwise, &then, expr.span)?;
                Ok(then)
            }
            SNode::Let {
                name, value, body, ..
            } => {
                let value = self.expr(value, env)?;
                let mut scope = env.clone();
                scope.insert(name.clone(), value);
                self.expr(body, &scope)
            }
            SNode::Match { scrutinees, rows } => {
                let scrutinees = scrutinees
                    .iter()
                    .map(|scrutinee| self.expr(scrutinee, env))
                    .collect::<Result<Vec<_>>>()?;
                let mut result: Option<Term> = None;
                for row in rows {
                    let place = row.span.or(expr.span);
                    let mut scope = env.clone();
                    for (pattern, scrutinee) in row.patterns.iter().zip(&scrutinees) {
                        self.pattern(pattern, scrutinee, &mut scope, place)?;
                    }
                    let body = self.expr(&row.body, &scope)?;
                    match &result {
                        Some(result) => self.unify(&body, result, place)?,
                        None => result = Some(body),
                    }
                }
                Ok(result.unwrap_or_else(|| self.fresh(false)))
            }
            SNode::TypeOf { arg } | SNode::ToString { arg } | SNode::Show { arg, .. } => {
                self.expr(arg, env)?;
                Ok(Term::Known(STRING))
            }
            SNode::CtorObject { tag, fields } => self.ctor_object(tag, fields, expr.span, env),
            SNode::Print { expr, body, .. } => {
                self.expr(expr, env)?;
                self.expr(body, env)
            }
            SNode::Array { items, .. } => {
                let element = self.fresh(false);
                if items.iter().all(|item| item.spread) {
                    self.empty_arrays.insert(
                        std::ptr::from_ref::<SExpr>(expr),
                        (element.clone(), expr.span),
                    );
                }
                for item in items {
                    let value = self.expr(&item.value, env)?;
                    let want = if item.spread {
                        Term::Array(Box::new(element.clone()))
                    } else {
                        element.clone()
                    };
                    self.unify(&value, &want, item.value.span.or(expr.span))?;
                }
                Ok(Term::Array(Box::new(element)))
            }
            SNode::Math { op, args, .. } => {
                // Math takes Numbers: an argument of no known type is one, and the checker refuses any other.
                for item in args {
                    let ty = self.expr(&item.value, env)?;
                    let ty = self.resolve(&ty);
                    let unknown = match &ty {
                        Term::Var(_) => true,
                        Term::Array(element) => {
                            item.spread && matches!(self.resolve(element), Term::Var(_))
                        }
                        Term::Known(_) => false,
                    };
                    if unknown {
                        let want = if item.spread {
                            Term::Array(Box::new(Term::Known(FLOAT)))
                        } else {
                            Term::Known(FLOAT)
                        };
                        self.unify(&ty, &want, item.value.span.or(expr.span))?;
                    }
                }
                let predicate = ["isInteger", "isSafeInteger", "isFinite", "isNaN"];
                Ok(Term::Known(if predicate.contains(&op.as_str()) {
                    BOOL
                } else {
                    FLOAT
                }))
            }
            SNode::Index { object, index } => {
                let element = self.fresh(false);
                let array = self.expr(object, env)?;
                self.unify(
                    &array,
                    &Term::Array(Box::new(element.clone())),
                    object.span.or(expr.span),
                )?;
                self.expr(index, env)?;
                Ok(element)
            }
            SNode::Length { object, integer } => {
                let array = self.expr(object, env)?;
                let element = self.fresh(false);
                self.unify(&array, &Term::Array(Box::new(element)), expr.span)?;
                Ok(if *integer {
                    self.fresh(true)
                } else {
                    Term::Known(FLOAT)
                })
            }
            _ => Ok(self.fresh(false)),
        }
    }

    fn call(
        &mut self,
        path: &[String],
        args: &[SExpr],
        env: &Env,
        place: Option<Span>,
    ) -> Result<Term> {
        let arg_terms = args
            .iter()
            .map(|arg| self.expr(arg, env))
            .collect::<Result<Vec<_>>>()?;
        #[allow(clippy::cast_precision_loss)]
        let argument_count = args.len() as f64;
        let Some(signature) = self
            .signature(path)
            .filter(|signature| accept_argument_count(signature.defaults.clone(), argument_count))
        else {
            return Ok(self.fresh(false));
        };
        for ((term, param), arg) in arg_terms.iter().zip(&signature.params).zip(args) {
            self.unify(term, param, arg.span.or(place))?;
        }
        Ok(signature.ret)
    }

    fn binary(
        &mut self,
        op: BinaryOp,
        left: &SExpr,
        right: &SExpr,
        place: Option<Span>,
        env: &Env,
    ) -> Result<Term> {
        let left_term = self.expr(left, env)?;
        let right_term = self.expr(right, env)?;
        match op {
            BinaryOp::And | BinaryOp::Or => {
                self.unify(&left_term, &Term::Known(BOOL), left.span.or(place))?;
                self.unify(&right_term, &Term::Known(BOOL), right.span.or(place))?;
                Ok(Term::Known(BOOL))
            }
            BinaryOp::Eq
            | BinaryOp::Ne
            | BinaryOp::Lt
            | BinaryOp::Le
            | BinaryOp::Gt
            | BinaryOp::Ge => {
                self.unify(&left_term, &right_term, place)?;
                Ok(Term::Known(BOOL))
            }
            BinaryOp::Add | BinaryOp::Sub | BinaryOp::Mul | BinaryOp::Div | BinaryOp::Rem => {
                self.unify(&left_term, &right_term, place)?;
                Ok(left_term)
            }
            BinaryOp::Concat => Ok(Term::Known(STRING)),
            BinaryOp::Plus => {
                let result = self.fresh(false);
                self.plus.push(Plus {
                    left: left_term,
                    right: right_term,
                    result: result.clone(),
                    place,
                });
                Ok(result)
            }
        }
    }

    fn pattern(
        &mut self,
        pattern: &SPattern,
        ty: &Term,
        scope: &mut Env,
        place: Option<Span>,
    ) -> Result<()> {
        match &pattern.node {
            SPatternNode::Ctor { path, args } => {
                let (name, tag) = match path.as_slice() {
                    [.., name, tag] => (name.clone(), tag.clone()),
                    _ => return Ok(()),
                };
                self.unify(ty, &Term::Known(Type::Data { name: name.clone() }), place)?;
                let count = self
                    .data
                    .iter()
                    .find(|(data, _)| *data == name)
                    .and_then(|(_, ctors)| ctors.iter().find(|ctor| ctor.name == tag))
                    .map_or(0, |ctor| ctor.fields.len());
                for (index, arg) in args.iter().enumerate() {
                    let term = if index < count {
                        self.field(&name, &tag, index)
                    } else {
                        self.fresh(false)
                    };
                    self.pattern(arg, &term, scope, place)?;
                }
                Ok(())
            }
            SPatternNode::BindOrCtor { name } | SPatternNode::Bind { name } => {
                scope.insert(name.clone(), ty.clone());
                Ok(())
            }
            SPatternNode::NumLit { .. } => {
                let literal = self.fresh(true);
                self.unify(ty, &literal, place)
            }
            SPatternNode::BoolLit { .. } => self.unify(ty, &Term::Known(BOOL), place),
            SPatternNode::StrLit { .. } => self.unify(ty, &Term::Known(STRING), place),
            _ => Ok(()),
        }
    }

    fn ctor_object(
        &mut self,
        tag: &str,
        fields: &[(String, SExpr)],
        place: Option<Span>,
        env: &Env,
    ) -> Result<Term> {
        let owners: Vec<(String, Vec<SCtor>)> = self
            .data
            .iter()
            .filter(|(_, ctors)| ctors.iter().any(|ctor| ctor.name == tag))
            .cloned()
            .collect();
        let mut terms = Vec::new();
        for (name, value) in fields {
            terms.push((name, self.expr(value, env)?, value.span));
        }
        let [(name, ctors)] = owners.as_slice() else {
            return Ok(self.fresh(false));
        };
        let Some(ctor) = ctors.iter().find(|ctor| ctor.name == tag) else {
            return Ok(self.fresh(false));
        };
        for (field, term, span) in terms {
            if let Some(index) = ctor
                .fields
                .iter()
                .position(|candidate| candidate.name.as_deref() == Some(field.as_str()))
            {
                let declared = self.field(name, tag, index);
                self.unify(&term, &declared, span.or(place))?;
            }
        }
        Ok(Term::Known(Type::Data { name: name.clone() }))
    }

    fn prop(&mut self, prop: &SProp, env: &Env) -> Result<()> {
        match &prop.node {
            SPropNode::Eq(comparison)
            | SPropNode::Ne(comparison)
            | SPropNode::Lt(comparison)
            | SPropNode::Le(comparison)
            | SPropNode::Gt(comparison)
            | SPropNode::Ge(comparison) => {
                let left = self.expr(&comparison.left, env)?;
                let right = self.expr(&comparison.right, env)?;
                self.unify(&left, &right, prop.span)
            }
            SPropNode::And { left, right } | SPropNode::Or { left, right } => {
                self.prop(left, env)?;
                self.prop(right, env)
            }
            SPropNode::Not { arg } => self.prop(arg, env),
            SPropNode::Bool { expr } => {
                let term = self.expr(expr, env)?;
                self.unify(&term, &Term::Known(BOOL), prop.span)
            }
            SPropNode::Forall { .. } | SPropNode::Implies { .. } => Ok(()),
        }
    }
}

const fn is_integer(ty: &Type) -> bool {
    matches!(ty, Type::Int | Type::Nat | Type::Fixed { .. })
}

fn describe_term(term: &Term) -> String {
    match term {
        Term::Known(ty) => describe(ty),
        Term::Array(_) => "an array".to_owned(),
        Term::Var(_) => "an unknown type".to_owned(),
    }
}

fn describe(ty: &Type) -> String {
    match ty {
        Type::Float => "a number".to_owned(),
        Type::Int | Type::Nat | Type::Fixed { .. } => "a bigint".to_owned(),
        Type::Bool => "a boolean".to_owned(),
        Type::String => "a string".to_owned(),
        Type::Data { name } => format!("a {name}"),
        Type::Array { .. } => "an array".to_owned(),
        other => other.kind().to_owned(),
    }
}
