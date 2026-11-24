//! Transport checked class factory witnesses into direct, receiver-owning calls.
use std::collections::{HashMap, HashSet};
use std::convert::Infallible;
use std::ptr;

use super::{lower_node_types_with, walk};
use crate::translation::callable_emission_rules as rules;
use crate::translation::ir::{
    Decl, Effect, Expr, FnDecl, LambdaParam, Node, Param, Program, RecordEntry,
};
use crate::translation::record_write_source_interfaces as record_rules;
use crate::translation::source_exception_rules::accept_linked_java_script_source_exception;
use crate::translation::types::{Type, UNIT};

pub(super) fn lower(source: &Program) -> Option<Program> {
    let mut plan = ClassSlots::default();
    let owners = source
        .source_class_owners
        .as_ref()?
        .get("owners")?
        .as_array()?;
    for owner in owners {
        if owner.get("representation")?.as_str()? != "closure slots" {
            continue;
        }
        let methods = owner.get("methods")?.as_array()?;
        if methods.is_empty() {
            continue;
        }
        let name = owner.get("constructorName")?.as_str()?;
        let constructor = function(source, name)?;
        let mut fields = HashMap::new();
        for method in methods {
            let field = method.get("name")?.as_str()?;
            let helper_name = method.get("helper")?.as_str()?;
            let helper = function(source, helper_name)?;
            if !rules::accept_formal_class_method_factory(
                helper.body.kind(),
                count(helper.params.len()),
                helper
                    .params
                    .first()
                    .is_some_and(|parameter| parameter.guard.is_some()),
            ) {
                return None;
            }
            let Node::Lambda { params, body } = &helper.body.node else {
                return None;
            };
            if fields.insert(field, helper_name).is_some()
                || plan.helpers.insert(helper_name, (params, body)).is_some()
            {
                return None;
            }
        }
        plan.constructor(source, constructor, &fields)?;
        plan.methods.insert(name, fields);
    }
    if plan.methods.is_empty() {
        return None;
    }
    plan.apply(source)
}

fn count(size: usize) -> f64 {
    serde_json::Number::from(size)
        .as_f64()
        .expect("a checked array length is a Number")
}

fn function<'a>(source: &'a Program, name: &str) -> Option<&'a FnDecl> {
    match source.declaration(name)? {
        Decl::Fn(function) => Some(function),
        _ => None,
    }
}

fn class_owner(ty: &Type) -> Option<&str> {
    match ty {
        Type::Record {
            source_class_owner, ..
        } => source_class_owner.as_deref(),
        _ => None,
    }
}

fn abrupt(expr: &Expr) -> bool {
    matches!(&expr.node, Node::Abort { .. })
        || matches!(&expr.node, Node::Math { op, .. }
            if accept_linked_java_script_source_exception("math", op))
}

fn tail(mut expr: &Expr) -> &Expr {
    while let Node::Let { body, .. } = &expr.node {
        expr = body;
    }
    expr
}

#[derive(Default)]
struct ClassSlots<'a> {
    methods: HashMap<&'a str, HashMap<&'a str, &'a str>>,
    helpers: HashMap<&'a str, (&'a [LambdaParam], &'a Expr)>,
    installations: HashSet<*const Expr>,
    placeholders: HashMap<*const Expr, &'a str>,
    payloads: HashSet<*const Expr>,
}

impl<'a> ClassSlots<'a> {
    fn constructor(
        &mut self,
        source: &'a Program,
        constructor: &'a FnDecl,
        fields: &HashMap<&str, &str>,
    ) -> Option<()> {
        let mut body = &constructor.body;
        let mut allocation = None;
        let mut installed = HashSet::new();
        loop {
            if let Node::If {
                then, otherwise, ..
            } = &body.node
            {
                if abrupt(then) {
                    body = otherwise;
                } else if abrupt(otherwise) {
                    body = then;
                } else {
                    break;
                }
                continue;
            }
            let Node::Let {
                name,
                value,
                body: rest,
            } = &body.node
            else {
                break;
            };
            if let Node::ReferenceAllocate { value: payload } = &value.node
                && class_owner(&value.ty) == Some(constructor.full_name.as_str())
            {
                if allocation.replace(name.as_str()).is_some() {
                    return None;
                }
                self.constructor_payload(payload, source, fields)?;
            } else if let Some(receiver) = allocation
                && installed.len() < fields.len()
            {
                let Node::Method { object, op, args } = &value.node else {
                    return None;
                };
                let field = record_rules::read_record_field_write_name(op);
                let helper = fields.get(field.as_str())?;
                let factory = args.first()?.as_ref()?;
                let Node::Call {
                    func,
                    args: operands,
                } = &factory.node
                else {
                    return None;
                };
                if !rules::accept_formal_class_method_installation(
                    &record_rules::read_record_field_write_operator(op),
                    object.var_name() == Some(receiver),
                    count(args.len()),
                    func.as_str() == *helper,
                    operands.first().and_then(Expr::var_name) == Some(receiver),
                    count(operands.len()),
                ) || !installed.insert(field)
                {
                    return None;
                }
                self.installations.insert(ptr::from_ref(value.as_ref()));
            }
            body = rest;
        }
        (allocation.is_some() && body.var_name() == allocation && installed.len() == fields.len())
            .then_some(())
    }

    fn constructor_payload(
        &mut self,
        mut body: &'a Expr,
        source: &'a Program,
        fields: &HashMap<&str, &str>,
    ) -> Option<()> {
        let mut local = HashMap::new();
        let mut uses = HashMap::new();
        walk(body, |expr| {
            if let Some(name) = expr.var_name() {
                *uses.entry(name.to_owned()).or_insert(0) += 1;
            }
        });
        while let Node::Let {
            name,
            value,
            body: rest,
        } = &body.node
        {
            if local.insert(name.as_str(), value.as_ref()).is_some() {
                return None;
            }
            body = rest;
        }
        let Node::Record { entries, .. } = &body.node else {
            return None;
        };
        self.payloads.insert(ptr::from_ref(body));
        for entry in entries {
            if fields.contains_key(entry.name.as_str()) {
                let name = entry.value.var_name()?;
                if uses.get(name) != Some(&1) {
                    return None;
                }
                let placeholder = local.get(name)?;
                let func = match &placeholder.node {
                    Node::FunctionRef { func } => func,
                    Node::Lambda { body, .. } => match &tail(body).node {
                        Node::Call { func, .. } => func,
                        _ => return None,
                    },
                    _ => return None,
                };
                let stub = function(source, func)?;
                let Node::Call { func: target, args } = &stub.body.node else {
                    return None;
                };
                if !rules::accept_formal_class_method_placeholder(
                    stub.body.kind(),
                    count(stub.params.len()),
                    func == target,
                    count(args.len()),
                ) {
                    return None;
                }
                self.placeholders.insert(ptr::from_ref(*placeholder), func);
            }
        }
        Some(())
    }

    fn type_fields(&self, source: &Type) -> Option<&HashMap<&'a str, &'a str>> {
        self.methods.get(class_owner(source)?)
    }

    fn ty(&self, source: &Type) -> Type {
        let mut result = source
            .map_children::<Infallible>(|child| Ok(self.ty(child)))
            .unwrap();
        if let Type::Record { fields, .. } = &mut result
            && let Some(methods) = self.type_fields(source)
        {
            for field in fields {
                if methods.contains_key(field.name.as_str()) {
                    field.ty = UNIT;
                }
            }
        }
        result
    }

    fn rewrite(&self, expr: &Expr) -> Option<Expr> {
        if self.installations.contains(&ptr::from_ref(expr))
            || self.placeholders.contains_key(&ptr::from_ref(expr))
        {
            return Some(Expr::new(Node::Unit, UNIT));
        }
        match &expr.node {
            Node::Record { entries, .. } if let Some(fields) = self.type_fields(&expr.ty) => {
                if !self.payloads.contains(&ptr::from_ref(expr)) {
                    return None;
                }
                let mut result = expr.clone();
                result.ty = self.ty(&expr.ty);
                let Node::Record {
                    entries: lowered, ..
                } = &mut result.node
                else {
                    return None;
                };
                *lowered = entries
                    .iter()
                    .map(|entry| {
                        let value = if fields.contains_key(entry.name.as_str()) {
                            Expr::new(Node::Unit, UNIT)
                        } else {
                            self.rewrite(&entry.value)?
                        };
                        Some(RecordEntry {
                            name: entry.name.clone(),
                            value,
                        })
                    })
                    .collect::<Option<_>>()?;
                return Some(result);
            }
            Node::Apply { func, args }
                if let Node::RecordField { object, field, .. } = &func.node
                    && let Some(helper) = self
                        .type_fields(&object.ty)
                        .and_then(|fields| fields.get(field.as_str())) =>
            {
                let Node::ReferenceRead { reference } = &object.node else {
                    return None;
                };
                let (parameters, body) = self.helpers.get(*helper)?;
                if args.len() != parameters.len()
                    || expr.ty.key() != body.ty.key()
                    || args
                        .iter()
                        .zip(*parameters)
                        .any(|(arg, parameter)| arg.ty.key() != parameter.ty.key())
                {
                    return None;
                }
                let operands = std::iter::once(reference.as_ref())
                    .chain(args)
                    .map(|argument| self.rewrite(argument))
                    .collect::<Option<_>>()?;
                return Some(
                    Expr::new(
                        Node::Call {
                            func: (*helper).to_owned(),
                            args: operands,
                        },
                        self.ty(&expr.ty),
                    )
                    .with_span(expr.span),
                );
            }
            Node::RecordField { object, field, .. }
                if self
                    .type_fields(&object.ty)
                    .is_some_and(|fields| fields.contains_key(field.as_str())) =>
            {
                return None;
            }
            Node::Call { func, .. } | Node::FunctionRef { func }
                if self.helpers.contains_key(func.as_str()) =>
            {
                return None;
            }
            _ => {}
        }
        let mut valid = true;
        let mut result = expr.map_children(&mut |child| {
            self.rewrite(child).unwrap_or_else(|| {
                valid = false;
                child.clone()
            })
        });
        result.ty = self.ty(&result.ty);
        lower_node_types_with(&mut result.node, &|source| self.ty(source));
        valid.then_some(result)
    }

    fn apply(&self, source: &Program) -> Option<Program> {
        let mut result = source.clone();
        for (entry, output) in source.declarations.iter().zip(&mut result.declarations) {
            match (entry, output) {
                (Decl::Fn(original), Decl::Fn(target)) => {
                    let mut body = &original.body;
                    if let Some((parameters, closure)) =
                        self.helpers.get(original.full_name.as_str())
                    {
                        body = closure;
                        target
                            .params
                            .extend(parameters.iter().map(|parameter| Param {
                                name: parameter.name.clone(),
                                ty: parameter.ty.clone(),
                                optional: false,
                                default_value: parameter.default_value.clone(),
                                guard: parameter.guard.clone(),
                            }));
                        target.ret = body.ty.clone();
                        target.annotated_ret = None;
                        target.semantic_ret = None;
                    }
                    for parameter in &mut target.params {
                        parameter.ty = self.ty(&parameter.ty);
                    }
                    target.ret = self.ty(&target.ret);
                    target.annotated_ret = target.annotated_ret.as_ref().map(|ty| self.ty(ty));
                    target.semantic_ret = target.semantic_ret.as_ref().map(|ty| self.ty(ty));
                    target.body = self.rewrite(body)?;
                }
                (_, Decl::Data(data)) => {
                    for field in data.ctors.iter_mut().flat_map(|ctor| &mut ctor.fields) {
                        field.ty = self.ty(&field.ty);
                    }
                }
                (Decl::Theorem(theorem), _) => {
                    if super::prop_any(&theorem.prop, &|expr| {
                        self.rewrite(expr).as_ref() != Some(expr)
                    }) {
                        return None;
                    }
                }
                _ => return None,
            }
        }
        let mut unused: HashSet<_> = self.placeholders.values().copied().collect();
        let placeholders = unused.clone();
        let retain = |expression: &Expr, unused: &mut HashSet<&str>| {
            walk(expression, |expression| {
                if let Node::Call { func, .. } | Node::FunctionRef { func } = &expression.node {
                    unused.remove(func.as_str());
                }
            });
        };
        for entry in &result.declarations {
            if let Decl::Fn(function) = entry
                && !placeholders.contains(function.full_name.as_str())
            {
                retain(&function.body, &mut unused);
            }
        }
        let main = source.main.as_ref()?;
        let output = result.main.as_mut()?;
        for (effect, changed) in main.effects.iter().zip(&mut output.effects) {
            match (effect, changed) {
                (
                    Effect::Let { value: expr, .. } | Effect::Print { expr, .. },
                    Effect::Let { value: target, .. } | Effect::Print { expr: target, .. },
                ) => {
                    *target = self.rewrite(expr)?;
                    retain(target, &mut unused);
                }
                (Effect::Assert { prop, .. }, _) => {
                    if super::prop_any(prop, &|expr| self.rewrite(expr).as_ref() != Some(expr)) {
                        return None;
                    }
                }
                _ => return None,
            }
        }
        result
            .declarations
            .retain(|entry| !unused.contains(entry.full_name()));
        Some(result)
    }
}
