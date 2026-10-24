//! Actual method operand owners connected by generated JavaScript decisions.

use super::{
    BOOL, Env, FLOAT, HashMap, HashSet, Inference, Rc, RefCell, Result, SEffect, SExpr, SItem,
    SNode, SProgram, SProp, SPropNode, STRING, Span, Term, Type,
};

/// Replay actual solved receiver selections without rebuilding unrelated nodes.
pub(super) fn fill_methods(
    program: &mut SProgram,
    owners: &HashSet<*const SExpr>,
    observed: &mut HashMap<*const SExpr, Type>,
) {
    fn walk(
        node: &mut SExpr,
        owners: &HashSet<*const SExpr>,
        observed: &mut HashMap<*const SExpr, Type>,
    ) {
        if let SNode::CtorObject {
            fields,
            prototype_definition,
            ..
        } = &mut node.node
        {
            for (_, value) in fields {
                walk(value, owners, observed);
            }
            if let Some(definition) = prototype_definition {
                walk(&mut definition.value, owners, observed);
            }
        } else {
            for child in super::super::lowering::children_mut(node) {
                walk(child, owners, observed);
            }
        }
        if owners.contains(&std::ptr::from_ref::<SExpr>(node)) {
            let original = std::mem::replace(&mut node.node, SNode::Unit);
            let SNode::StringTest { object, op, search } = original else {
                unreachable!("the retained owner is an actual includes expression")
            };
            let old = std::ptr::from_ref(search.as_ref());
            node.node = SNode::Method {
                object,
                op,
                args: vec![Some(*search)],
            };
            if let Some(ty) = observed.remove(&old) {
                let SNode::Method { args, .. } = &node.node else {
                    unreachable!()
                };
                observed.insert(
                    std::ptr::from_ref(args[0].as_ref().expect("the actual search operand")),
                    ty,
                );
            }
        }
    }
    fn items(
        entries: &mut [SItem],
        owners: &HashSet<*const SExpr>,
        observed: &mut HashMap<*const SExpr, Type>,
    ) {
        for item in entries {
            match item {
                SItem::Fn(function) => {
                    for param in &mut function.params {
                        if let Some(value) = &mut param.default_value {
                            walk(value, owners, observed);
                        }
                    }
                    walk(&mut function.body, owners, observed);
                }
                SItem::Module(module) => items(&mut module.items, owners, observed),
                SItem::Data(_) | SItem::Theorem(_) => {}
            }
        }
    }
    fn prop(
        proposition: &mut SProp,
        owners: &HashSet<*const SExpr>,
        observed: &mut HashMap<*const SExpr, Type>,
    ) {
        match &mut proposition.node {
            SPropNode::Eq(value)
            | SPropNode::Ne(value)
            | SPropNode::Lt(value)
            | SPropNode::Le(value)
            | SPropNode::Gt(value)
            | SPropNode::Ge(value) => {
                walk(&mut value.left, owners, observed);
                walk(&mut value.right, owners, observed);
            }
            SPropNode::And { left, right }
            | SPropNode::Or { left, right }
            | SPropNode::Implies { left, right } => {
                prop(left, owners, observed);
                prop(right, owners, observed);
            }
            SPropNode::Not { arg } | SPropNode::Forall { body: arg, .. } => {
                prop(arg, owners, observed);
            }
            SPropNode::Bool { expr } => walk(expr, owners, observed),
        }
    }
    items(&mut program.items, owners, observed);
    if let Some(main) = &mut program.main {
        for effect in &mut main.effects {
            match effect {
                SEffect::Let { value, .. } => walk(value, owners, observed),
                SEffect::Print { expr, .. } => walk(expr, owners, observed),
                SEffect::Assert { prop: value, .. } => prop(value, owners, observed),
            }
        }
    }
}

pub(super) struct Method {
    object: Term,
    args: Vec<Term>,
    operation: String,
    result: Term,
    place: Option<Span>,
}

pub(super) struct Includes {
    node: *const SExpr,
    object: Term,
    search: Term,
    place: Option<Span>,
}

impl Inference {
    fn infer_shared_collection_method(&mut self, method: &Method, receiver: &Term) -> Result<bool> {
        if let Term::Array(element) = receiver
            && !crate::translation::array_state_method_rules::read_array_state_method_result(
                &method.operation,
            )
            .is_empty()
        {
            for (index, argument) in method.args.iter().enumerate() {
                let role=crate::translation::array_state_method_rules::read_array_state_method_argument_role(&method.operation,
                    serde_json::Number::from(index).as_f64().expect("an actual argument position is a Number"));
                if role == "element" {
                    self.unify(argument, element, method.place)?;
                } else if !role.is_empty() {
                    match self.resolve(argument) {
                        Term::Var(_) => self.unify(argument, &Term::Known(FLOAT), method.place)?,
                        Term::Option { element, .. }
                            if matches!(self.resolve(&element), Term::Var(_)) =>
                        {
                            self.unify(&element, &Term::Known(FLOAT), method.place)?
                        }
                        _ => {}
                    }
                }
            }
            self.unify(&method.result, receiver, method.place)?;
            return Ok(true);
        }
        use crate::translation::{
            collection_access_rules as access, reference_method_rules as references,
        };
        let (kind, key, value) = match receiver {
            Term::Array(element) => ("array", element.as_ref().clone(), element.as_ref().clone()),
            Term::Map { key, value, .. } => ("map", key.as_ref().clone(), value.as_ref().clone()),
            Term::Set { element, .. } => {
                ("set", element.as_ref().clone(), element.as_ref().clone())
            }
            _ => return Ok(false),
        };
        let reference_result = references::read_reference_method_result(kind, &method.operation);
        let result = if reference_result.is_empty() {
            access::read_collection_access_result(kind, &method.operation)
        } else {
            reference_result.clone()
        };
        if result.is_empty() {
            return Ok(false);
        }
        for (index, argument) in method.args.iter().enumerate() {
            let role = if reference_result.is_empty() {
                String::new()
            } else {
                references::read_reference_method_argument_role(
                    kind,
                    &method.operation,
                    serde_json::Number::from(index)
                        .as_f64()
                        .expect("an actual argument position is a Number"),
                )
            };
            match role.as_str() {
                "key" | "element" => self.unify(argument, &key, method.place)?,
                "value" => self.unify(argument, &value, method.place)?,
                _ => {}
            }
        }
        let optional = |element| Term::Option {
            element: Box::new(element),
            absence: Rc::new(RefCell::new("undefined".to_owned())),
        };
        let output = match result.as_str() {
            "receiver" => receiver.clone(),
            "length" => Term::Known(FLOAT),
            "boolean" => Term::Known(BOOL),
            "unit" => Term::Known(crate::translation::types::UNIT),
            "optional element" => optional(key),
            "optional value" => optional(value),
            "keys" => Term::Array(Box::new(key)),
            "values" => Term::Array(Box::new(value)),
            "entries" => Term::Array(Box::new(Term::Tuple(vec![key, value]))),
            _ => return Ok(false),
        };
        self.unify(&method.result, &output, method.place)?;
        Ok(true)
    }

    pub(super) fn includes(
        &mut self,
        expression: &SExpr,
        object: &SExpr,
        search: &SExpr,
        env: &Env,
    ) -> Result<Term> {
        let object = self.expr(object, env)?;
        let search = self.expr(search, env)?;
        self.includes.push(Includes {
            node: std::ptr::from_ref(expression),
            object,
            search,
            place: expression.span,
        });
        Ok(Term::Known(BOOL))
    }

    pub(super) fn solve_includes(&mut self, force: bool) -> Result<bool> {
        let mut progress = false;
        let mut waiting = Vec::new();
        let mut pending = std::mem::take(&mut self.includes);
        if force && pending.len() > 1 {
            waiting = pending.split_off(1);
        }
        for constraint in pending {
            let receiver = self.resolve(&constraint.object);
            let kind = match &receiver {
                Term::Var(_) => "var",
                Term::Array(_) => "array",
                Term::Known(ty) => ty.kind(),
                _ => "",
            };
            let form =
                super::super::surface_rules::read_java_script_includes_receiver_form(kind, force);
            if form == "defer" {
                waiting.push(constraint);
                continue;
            }
            if form == "array" {
                let element = match receiver {
                    Term::Array(element) => *element,
                    Term::Known(Type::Array { element, .. }) => self.declared(Some(&element)),
                    _ => unreachable!("the generated array selection retains its actual type"),
                };
                self.unify(&constraint.search, &element, constraint.place)?;
                self.array_includes.insert(constraint.node);
            } else {
                self.unify(&constraint.object, &Term::Known(STRING), constraint.place)?;
                self.unify(&constraint.search, &Term::Known(STRING), constraint.place)?;
            }
            progress = true;
        }
        self.includes = waiting;
        Ok(progress)
    }
    pub(super) fn method(
        &mut self,
        object: &SExpr,
        operation: &str,
        args: &[Option<SExpr>],
        env: &Env,
        place: Option<Span>,
    ) -> Result<Term> {
        let object = self.expr(object, env)?;
        let mut actual = Vec::with_capacity(args.len());
        for argument in args {
            actual.push(if let Some(argument) = argument {
                self.expr(argument, env)?
            } else {
                self.fresh(false)
            });
        }
        let result = self.fresh(false);
        self.methods.push(Method {
            object,
            args: actual,
            operation: operation.to_owned(),
            result: result.clone(),
            place,
        });
        Ok(result)
    }

    pub(super) fn solve_methods(&mut self) -> Result<bool> {
        let mut progress = false;
        let mut waiting = Vec::new();
        for constraint in std::mem::take(&mut self.methods) {
            if self.method_step(&constraint)? {
                progress = true;
            } else {
                waiting.push(constraint);
            }
        }
        self.methods = waiting;
        Ok(progress)
    }

    fn method_step(&mut self, method: &Method) -> Result<bool> {
        use super::super::surface_rules::{
            read_java_script_method_argument_kind, read_java_script_method_receiver_kind,
            read_java_script_string_method_result_form,
        };
        let mut receiver = self.resolve(&method.object);
        if let Term::Known(ty) = &receiver
            && matches!(ty, Type::Map { .. } | Type::Set { .. })
        {
            receiver = self.declared(Some(ty));
        }
        if matches!(receiver, Term::Var(_)) {
            match read_java_script_method_receiver_kind(&method.operation).as_str() {
                "string" => self.unify(&method.object, &Term::Known(STRING), method.place)?,
                "array" => {
                    let element = self.fresh(false);
                    self.unify(
                        &method.object,
                        &Term::Array(Box::new(element)),
                        method.place,
                    )?;
                }
                _ => return Ok(false),
            }
            receiver = self.resolve(&method.object);
        }
        if self.infer_shared_collection_method(method, &receiver)? {
            return Ok(true);
        }
        let kind = match &receiver {
            Term::Known(ty) => ty.kind(),
            Term::Array(_) => "array",
            _ => "",
        };
        if kind != "string" && kind != "array" {
            return Ok(false);
        }
        if kind == "array"
            && self.array_callback_method(
                &receiver,
                &method.operation,
                &method.args,
                &method.result,
                method.place,
            )?
        {
            return Ok(true);
        }
        for (index, argument) in method.args.iter().enumerate() {
            #[allow(clippy::cast_precision_loss)]
            let position = index as f64;
            let argument_kind = if kind == "array"
                && position < crate::translation::primitive_method_rules::read_primitive_method_argument_count(kind, &method.operation)
            {
                crate::translation::primitive_method_rules::read_primitive_method_checked_argument_kind(
                    kind, &method.operation, position,
                )
            } else {
                read_java_script_method_argument_kind(kind, &method.operation, position)
            };
            match argument_kind.as_str() {
                "float" => self.unify(argument, &Term::Known(FLOAT), method.place)?,
                "number-value" => {
                    let value = self.numeric_value(argument);
                    self.unify(&value, &Term::Known(FLOAT), method.place)?;
                }
                "string" => self.unify(argument, &Term::Known(STRING), method.place)?,
                "element" => {
                    if let Term::Array(element) = &receiver {
                        self.unify(argument, element, method.place)?;
                    }
                }
                _ => {}
            }
        }
        if kind == "string" {
            let form = if crate::translation::primitive_method_rules::read_primitive_method_minimum_arguments(kind, &method.operation) >= 0.0 {
                crate::translation::primitive_method_rules::read_primitive_method_checked_result_form(kind, &method.operation)
            } else {
                read_java_script_string_method_result_form(&method.operation)
            };
            let result = match form.as_str() {
                "bool" => Term::Known(BOOL),
                "float" => Term::Known(FLOAT),
                "array-string" => Term::Array(Box::new(Term::Known(STRING))),
                "optional-float" => self.declared(Some(&Type::Option {
                    element: Box::new(FLOAT),
                    absence: "undefined".to_owned(),
                })),
                "optional-string" => self.declared(Some(&Type::Option {
                    element: Box::new(STRING),
                    absence: "undefined".to_owned(),
                })),
                _ => Term::Known(STRING),
            };
            self.unify(&method.result, &result, method.place)?;
        } else {
            let form = crate::translation::primitive_method_rules::read_primitive_method_checked_result_form(kind, &method.operation);
            let result = match form.as_str() {
                "bool" => Term::Known(BOOL),
                "float" => Term::Known(FLOAT),
                "string" => Term::Known(STRING),
                "optional-element" => {
                    let Term::Array(element) = &receiver else {
                        unreachable!("the shared optional-element result has an array owner");
                    };
                    Term::Option {
                        element: element.clone(),
                        absence: Rc::new(RefCell::new("undefined".to_owned())),
                    }
                }
                _ => receiver,
            };
            self.unify(&method.result, &result, method.place)?;
        }
        Ok(true)
    }
}
