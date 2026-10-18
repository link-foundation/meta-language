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

mod array_bindings;
mod array_callbacks;
mod array_from;
mod arrays;
mod callables;
mod collections;
mod expressions;
mod methods;
mod numbers;
mod optional_values;
mod patterns;
mod products;
mod records;
mod solver;
mod term_facts;
mod term_types;

pub(super) use products::{infer_javascript_product_types, infer_javascript_types};

use arrays::fill_arrays;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

use super::{
    BOOL, BinaryOp, FLOAT, INT, ROOT, Result, SCtor, SEffect, SExpr, SFn, SItem, SNode, SPattern,
    SPatternNode, SProgram, SProp, SPropNode, STRING, Span, Type, UnaryOp, type_error,
};
use crate::translation::types::UNIT;

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
fn fill_functions(
    items: &mut [SItem],
    inferred: &mut impl Iterator<Item = (Vec<Type>, Type, Option<Type>)>,
) {
    for item in items {
        match item {
            SItem::Fn(function) => {
                let Some((params, ret, semantic_ret)) = inferred.next() else {
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
                if semantic_ret.is_some() {
                    function.semantic_ret = semantic_ret;
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
    /// Clones retain the actual shared absence owner used by JS unification.
    Option {
        element: Box<Self>,
        absence: Rc<RefCell<String>>,
    },
    /// A product preserves each ordered field's independently inferred type.
    Tuple(Vec<Self>),
    Union(Vec<Self>),
    Map {
        key: Box<Self>,
        value: Box<Self>,
        reference: bool,
    },
    Set {
        element: Box<Self>,
        reference: bool,
    },
    /// Clones retain the actual mutable owner of open inferred record fields.
    Record(Rc<RefCell<RecordTerm>>),
    /// Callable slots retain their independently inferred variables.
    Function {
        params: Vec<Self>,
        ret: Box<Self>,
    },
    /// A shared scalar cell retains its independently inferred element.
    Cell(Box<Self>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct RecordTerm {
    fields: Vec<(String, Term)>,
    open: bool,
}

#[derive(Clone)]
struct Signature {
    params: Vec<Term>,
    defaults: Vec<bool>,
    optional_params: Vec<bool>,
    ret: Term,
    declared_ret: Option<Term>,
}

#[derive(Clone)]
struct FunctionResult {
    result: Term,
    declared: Option<Term>,
    body: Term,
}
#[derive(Clone)]
struct CallArguments {
    parameters: Vec<Term>,
    arguments: Vec<Term>,
    defaults: Vec<bool>,
    optional_params: Vec<bool>,
    place: Option<Span>,
}
struct Coalescing {
    left: Term,
    right: Term,
    place: Option<Span>,
}
struct Join {
    left: Term,
    right: Term,
    result: Term,
    place: Option<Span>,
}
struct Arithmetic {
    left: Term,
    right: Option<Term>,
    result: Term,
    place: Option<Span>,
}

struct Equality {
    operation: BinaryOp,
    left: Term,
    right: Term,
    place: Option<Span>,
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

struct IndexOf {
    object: Term,
    index: f64,
    literal: bool,
    optional: bool,
    proven_in_bounds: bool,
    result: Term,
    place: Option<Span>,
}

struct IterableOf {
    object: Term,
    mapper: Option<Term>,
    result: Term,
    place: Option<Span>,
}

struct Inference {
    /// Constructors of every data type, in declaration order.
    data: Vec<(String, Vec<SCtor>)>,
    bindings: Vec<Option<Term>>,
    unification_changes: usize,
    bigints: HashSet<usize>,
    plus: Vec<Plus>,
    coalesces: Vec<Coalescing>,
    array_binding_defaults: Vec<array_bindings::DefaultBinding>,
    joins: Vec<Join>,
    results: Vec<FunctionResult>,
    calls: Vec<CallArguments>,
    conditions: Vec<(Term, Option<Span>)>,
    arithmetic: Vec<Arithmetic>,
    equalities: Vec<Equality>,
    methods: Vec<methods::Method>,
    includes: Vec<methods::Includes>,
    array_includes: HashSet<*const SExpr>,
    fields: Vec<FieldOf>,
    indices: Vec<IndexOf>,
    iterables: Vec<IterableOf>,
    collections: HashMap<*const SExpr, (Term, Option<Span>)>,
    signatures: HashMap<String, Signature>,
    context: Vec<String>,
    /// The types of the fields the translator makes up, which are inferred
    /// like parameters, by data type, constructor and position.
    field_terms: HashMap<(String, String, usize), Term>,
    /// Array literals with no element of their own, whose element type the
    /// rest of the program fixes, by node.
    empty_arrays: HashMap<*const SExpr, (Term, Option<Span>)>,
    records: HashMap<*const SExpr, Term>,
    record_spreads: Vec<records::RecordSpread>,
    expression_terms: HashMap<*const SExpr, (Term, Option<Span>)>,
    lambdas: HashMap<*const SExpr, Term>,
    lambda_places: HashMap<*const SExpr, (Option<Span>, Vec<Option<Span>>)>,
    cells: HashMap<*const SExpr, (Term, Option<Span>)>,
    record_expressions: bool,
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
            unification_changes: 0,
            bigints: HashSet::new(),
            plus: Vec::new(),
            coalesces: Vec::new(),
            array_binding_defaults: Vec::new(),
            joins: Vec::new(),
            results: Vec::new(),
            calls: Vec::new(),
            conditions: Vec::new(),
            arithmetic: Vec::new(),
            equalities: Vec::new(),
            methods: Vec::new(),
            includes: Vec::new(),
            array_includes: HashSet::new(),
            fields: Vec::new(),
            indices: Vec::new(),
            iterables: Vec::new(),
            collections: HashMap::new(),
            signatures: HashMap::new(),
            context: vec![ROOT.to_owned()],
            field_terms: HashMap::new(),
            empty_arrays: HashMap::new(),
            records: HashMap::new(),
            record_spreads: Vec::new(),
            expression_terms: HashMap::new(),
            lambdas: HashMap::new(),
            lambda_places: HashMap::new(),
            cells: HashMap::new(),
            record_expressions: false,
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
            Some(term) => Some(describe_term(term)),
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
                    self.unification_changes += 1;
                }
                Ok(())
            }
            (Term::Var(variable), other) | (other, Term::Var(variable)) => {
                let integer = matches!(other, Term::Known(ty) if is_integer(ty));
                if self.bigints.contains(variable) && !integer {
                    let value = match other {
                        Term::Option { element, .. } => self.resolve(element),
                        _ => other.clone(),
                    };
                    let value_kind = match &value {
                        Term::Var(_) => "var",
                        Term::Known(ty) => ty.kind(),
                        _ => "",
                    };
                    let integer = matches!(&value, Term::Known(ty) if is_integer(ty));
                    let operation =
                        crate::translation::optional_value_rules::read_optional_big_int_constraint(
                            if matches!(other, Term::Option { .. }) {
                                "option"
                            } else {
                                ""
                            },
                            value_kind,
                            integer,
                        );
                    if operation == "mark" {
                        let Term::Var(variable) = value else {
                            unreachable!("the generated BigInt plan requires an actual variable");
                        };
                        self.bigints.insert(variable);
                    } else if operation == "error" {
                        return Err(type_error(
                            format!(
                                "a BigInt is used as {}; JavaScript does not mix BigInt with other types",
                                describe_term(other)
                            ),
                            place,
                        ));
                    }
                }
                if self.contains(other, *variable, &mut HashSet::new()) {
                    return Err(type_error(
                        "a recursive inferred type needs an explicit data type",
                        place,
                    ));
                }
                self.bindings[*variable] = Some(other.clone());
                self.unification_changes += 1;
                Ok(())
            }
            (Term::Array(x), Term::Array(y))
            | (Term::Set { element: x, .. }, Term::Set { element: y, .. }) => {
                self.unify(x, y, place)
            }
            (Term::Record(left), Term::Record(right)) => self.unify_records(left, right, place),
            (
                Term::Map {
                    key: a, value: b, ..
                },
                Term::Map {
                    key: c, value: d, ..
                },
            ) => {
                self.unify(a, c, place)?;
                self.unify(b, d, place)
            }
            (Term::Union(_), Term::Union(_)) if self.same_term(&a, &b) => Ok(()),
            (Term::Option { .. }, _) | (_, Term::Option { .. }) => {
                self.unify_optional(&a, &b, place)
            }
            (Term::Function { .. }, Term::Function { .. }) => self.unify_callable(&a, &b, place),
            (Term::Cell(_), Term::Cell(_)) => self.unify_cells(&a, &b, place),
            (Term::Tuple(left), Term::Tuple(right)) => {
                if left.len() != right.len() {
                    return Err(type_error(
                        "one tuple is used with two different lengths",
                        place,
                    ));
                }
                for (left, right) in left.iter().zip(right) {
                    self.unify(left, right, place)?;
                }
                Ok(())
            }
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
        let left = self.numeric_value(&constraint.left);
        let right = self.numeric_value(&constraint.right);
        if self.resolve(&left) == Term::Known(STRING) || self.resolve(&right) == Term::Known(STRING)
        {
            return Err(type_error(
                "nullable string addition needs a string or number result determined by the values",
                constraint.place,
            ));
        }
        self.unify(&left, &right, constraint.place)?;
        self.unify(&constraint.result, &left, constraint.place)?;
        Ok(true)
    }

    fn field_step(&mut self, constraint: &FieldOf) -> Result<bool> {
        if let Term::Record(owner) = self.resolve(&constraint.object) {
            return self.record_field_constraint(
                &owner,
                &constraint.field,
                &constraint.result,
                constraint.place,
            );
        }
        if constraint.field == "size"
            && matches!(
                self.resolve(&constraint.object),
                Term::Map { .. } | Term::Set { .. }
            )
        {
            self.unify(&constraint.result, &Term::Known(FLOAT), constraint.place)?;
            return Ok(true);
        }
        let name = match self.resolve(&constraint.object) {
            Term::Known(Type::Data { name }) => name,
            Term::Array(_) | Term::Tuple(_) | Term::Known(Type::String)
                if constraint.field == "length" =>
            {
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
        let term = self.expr_inner(expr, env)?;
        if self.record_expressions {
            self.expression_terms
                .insert(std::ptr::from_ref(expr), (term.clone(), expr.span));
        }
        Ok(term)
    }

    fn expr_inner(&mut self, expr: &SExpr, env: &Env) -> Result<Term> {
        match &expr.node {
            // The 1 of `x++` has the type of `x`, a Number or a BigInt.
            SNode::Num { unit: true, .. } => Ok(self.fresh(false)),
            SNode::Num { ty, .. } => Ok(match ty {
                Some(ty) => self.declared(Some(ty)),
                None => self.fresh(true),
            }),
            SNode::ArrayBindingRest { .. } | SNode::ArrayBindingDefault { .. } => {
                self.array_binding_expr(expr, env, None)
            }
            SNode::Bool { .. } => Ok(Term::Known(BOOL)),
            SNode::Str { .. } => Ok(Term::Known(STRING)),
            SNode::Unit => Ok(Term::Known(UNIT)),
            SNode::None { .. }
            | SNode::Some { .. }
            | SNode::FlattenOption { .. }
            | SNode::Coalesce { .. }
            | SNode::OptionalField { .. } => self.optional_expr(expr, env, None),
            SNode::Name { path } => {
                if let [name] = path.as_slice()
                    && let Some(term) = env.get(name)
                {
                    return Ok(term.clone());
                }
                self.call(path, &[], env, expr.span)
            }
            SNode::App { func, args } => self.callable_application(func, args, env, expr.span),
            SNode::Record { entries, .. } => self.record_expr(expr, entries, env, None),
            SNode::FunctionValue { .. }
            | SNode::Lambda { .. }
            | SNode::CellAllocate { .. }
            | SNode::CellRead { .. }
            | SNode::CellWrite { .. } => self.callable_expr(expr, env, None),
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
                    self.product_condition(&arg, expr.span);
                    return Ok(Term::Known(BOOL));
                }
                let result = self.fresh(false);
                self.arithmetic.push(Arithmetic {
                    left: arg,
                    right: None,
                    result: result.clone(),
                    place: expr.span,
                });
                Ok(result)
            }
            SNode::Binary {
                op, left, right, ..
            } => self.binary(*op, left, right, expr.span, env),
            SNode::If { .. } => self.infer_conditional_expression(expr, env),
            SNode::Let { .. } => self.infer_binding_expression(expr, env),
            SNode::Match { .. } => self.infer_match_expression(expr, env),
            SNode::StringMap { object, .. } => {
                let value = self.expr(object, env)?;
                self.unify(&value, &Term::Known(STRING), object.span)?;
                Ok(Term::Known(STRING))
            }
            SNode::CopySlice { object } => self.expr(object, env),
            SNode::ArrayFrom { args, .. } => self.array_from(expr, args, env, None),
            SNode::Set { items, .. } | SNode::Map { entries: items, .. } => {
                self.collection_construction(expr, items, env, None)
            }
            SNode::Method { object, op, args } => self.method(object, op, args, env, expr.span),
            SNode::StringTest { op, object, search } if op == "includes" => {
                self.includes(expr, object, search, env)
            }
            SNode::StringTest { object, search, .. } => {
                let object_type = self.expr(object, env)?;
                self.unify(&object_type, &Term::Known(STRING), expr.span)?;
                let search_type = self.expr(search, env)?;
                self.unify(&search_type, &Term::Known(STRING), expr.span)?;
                Ok(Term::Known(BOOL))
            }
            SNode::TypeOf { arg } | SNode::ToString { arg } | SNode::Show { arg, .. } => {
                self.expr(arg, env)?;
                Ok(Term::Known(STRING))
            }
            SNode::CtorObject {
                tag,
                fields,
                prototype_definition,
                ..
            } => {
                if let Some(definition) = prototype_definition {
                    self.expr(&definition.value, env)?;
                }
                self.ctor_object(tag, fields, expr.span, env)
            }
            SNode::Print { expr, body, .. } => {
                self.expr(expr, env)?;
                self.expr(body, env)
            }
            SNode::Array { items, .. } => {
                let element = self.fresh(false);
                self.empty_arrays.insert(
                    std::ptr::from_ref::<SExpr>(expr),
                    (element.clone(), expr.span),
                );
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
            SNode::Tuple { items, .. } => Ok(Term::Tuple(
                items
                    .iter()
                    .map(|item| self.expr(item, env))
                    .collect::<Result<Vec<_>>>()?,
            )),
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
                        Term::Known(_)
                        | Term::Tuple(_)
                        | Term::Record(_)
                        | Term::Union(_)
                        | Term::Map { .. }
                        | Term::Set { .. }
                        | Term::Option { .. }
                        | Term::Function { .. }
                        | Term::Cell(_) => false,
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
            SNode::ForOfIterable { object, .. } => {
                let object = self.expr(object, env)?;
                let result = Term::Array(Box::new(self.fresh(false)));
                self.iterables.push(IterableOf {
                    object,
                    mapper: None,
                    result: result.clone(),
                    place: expr.span,
                });
                self.collections
                    .insert(std::ptr::from_ref(expr), (result.clone(), expr.span));
                Ok(result)
            }
            SNode::StringPoints { object } => {
                let value = self.expr(object, env)?;
                self.unify(&value, &Term::Known(STRING), expr.span)?;
                Ok(Term::Array(Box::new(Term::Known(STRING))))
            }
            SNode::ReferenceWrite {
                object,
                index,
                value,
                ..
            } => {
                self.expr(object, env)?;
                if let Some(index) = index {
                    self.expr(index, env)?;
                }
                if let Some(value) = value {
                    self.expr(value, env)?;
                }
                Ok(Term::Known(UNIT))
            }
            SNode::Index {
                object,
                index,
                optional,
                proven_in_bounds,
            } => {
                let element = self.fresh(false);
                let object_type = self.expr(object, env)?;
                self.expr(index, env)?;
                let position = super::products::read_javascript_product_index(index);
                let literal = matches!(index.node, SNode::Num { .. })
                    || matches!(&index.node, SNode::Unary { op: UnaryOp::Neg, arg } if matches!(arg.node, SNode::Num { .. }));
                self.indices.push(IndexOf {
                    object: object_type,
                    index: position,
                    literal,
                    optional: optional
                        .as_ref()
                        .and_then(super::super::surface::SurfaceRawMetadata::as_boolean)
                        == Some(true),
                    proven_in_bounds: proven_in_bounds
                        .as_ref()
                        .and_then(super::super::surface::SurfaceRawMetadata::as_boolean)
                        == Some(true),
                    result: element.clone(),
                    place: object.span.or(expr.span),
                });
                Ok(element)
            }
            SNode::TupleIndex { object, index } => {
                let element = self.fresh(false);
                let object_type = self.expr(object, env)?;
                #[allow(clippy::cast_precision_loss)]
                let position = *index as f64;
                self.indices.push(IndexOf {
                    object: object_type,
                    index: position,
                    literal: true,
                    optional: false,
                    proven_in_bounds: true,
                    result: element.clone(),
                    place: object.span.or(expr.span),
                });
                Ok(element)
            }
            SNode::Length { object, integer } => {
                let array = self.expr(object, env)?;
                if matches!(self.resolve(&array), Term::Tuple(_)) {
                    return Ok(if *integer {
                        self.fresh(true)
                    } else {
                        Term::Known(FLOAT)
                    });
                }
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
        Term::Option { .. } => "option".to_owned(),
        Term::Tuple(_) => "tuple".to_owned(),
        Term::Union(_) => "union".to_owned(),
        Term::Map { .. } => "map".to_owned(),
        Term::Set { .. } => "set".to_owned(),
        Term::Record(_) => "record".to_owned(),
        Term::Function { .. } => "function".to_owned(),
        Term::Cell(_) => "cell".to_owned(),
        Term::Var(_) => "var".to_owned(),
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
