//! Native checked-expression adapter for the shared formal reference-store plan.
mod class_slots;

use std::borrow::Cow;
use std::collections::{HashMap, HashSet};
use std::convert::Infallible;

use super::{compare, let_in, literal, logical, prop_any};
use crate::translation::Language;
use crate::translation::callable_emission_rules as rules;
use crate::translation::ir::{
    Decl, Effect, Expr, FnDecl, Item, LambdaParam, LitValue, Node, Param, Program, RecordEntry,
};
use crate::translation::record_write_source_interfaces as record_rules;
use crate::translation::surface::{BinaryOp, UnaryOp};
use crate::translation::types::{BOOL, NAT, Type, UNIT, array};

pub(in crate::translation) fn prepare(source: &Program) -> Cow<'_, Program> {
    if source.source_language != Language::JavaScript {
        return Cow::Borrowed(source);
    }
    let mut observed_collection = false;
    let mut observed_cell = false;
    let mut structural_state = false;
    let mut observe = |expr: &Expr| {
        observed_collection |= matches!(expr.node, Node::CollectionItems { .. });
        observed_cell |= matches!(expr.node, Node::CellAllocate { .. });
        let structural_operand = reference_receiver(expr)
            || matches!(&expr.node, Node::Binary { domain: Some(domain), .. } if domain.is_reference());
        structural_state |= rules::observe_formal_structural_state(expr.kind(), structural_operand);
    };
    for entry in &source.declarations {
        if let Decl::Fn(function) = entry {
            walk(&function.body, &mut observe);
        }
    }
    for effect in source.main.iter().flat_map(|main| &main.effects) {
        if let Effect::Let { value, .. } | Effect::Print { expr: value, .. } = effect {
            walk(value, &mut observe);
        }
    }
    let standalone = source
        .main
        .as_ref()
        .is_none_or(|main| main.effects.is_empty());
    if !rules::accept_formal_reference_store_lowering(
        observed_collection,
        observed_cell,
        structural_state,
        standalone,
    ) {
        return Cow::Borrowed(source);
    }
    let prepared = class_slots::lower(source);
    let owner = prepared.as_ref().unwrap_or(source);
    let closed_class_library = standalone
        && owner
            .source_class_owners
            .as_ref()
            .and_then(|owners| owners.get("owners"))
            .and_then(serde_json::Value::as_array)
            .is_some_and(|owners| !owners.is_empty());
    let declared: HashSet<_> = owner
        .declarations
        .iter()
        .filter_map(|entry| match entry {
            Decl::Fn(function) => Some(function.full_name.as_str()),
            _ => None,
        })
        .collect();
    let mut roots = Vec::new();
    let mut callers = Vec::new();
    let mut callees = Vec::new();
    let mut address_crosses_import = false;
    for entry in &owner.declarations {
        if let Decl::Fn(function) = entry {
            let mut root =
                closed_class_library && function_contains_store_address(function, structural_state);
            walk(&function.body, |expr| {
                root |= needs_store(expr);
                if let Node::Call { func, args } = &expr.node {
                    address_crosses_import |= standalone
                        && structural_state
                        && !declared.contains(func.as_str())
                        && std::iter::once(&expr.ty)
                            .chain(args.iter().map(|argument| &argument.ty))
                            .any(|ty| contains_reference_store_address(ty, true));
                    callers.push(function.full_name.clone());
                    callees.push(func.clone());
                }
            });
            if root {
                roots.push(function.full_name.clone());
            }
        }
    }
    if address_crosses_import {
        return Cow::Borrowed(source);
    }
    // Closed class helpers share private addresses along actual local edges.
    // Other standalone functions initialize independent stores.
    let effectful = if standalone && !closed_class_library {
        roots.into_iter().collect()
    } else {
        rules::plan_formal_reference_store_functions(&roots, &callers, &callees)
            .into_iter()
            .collect()
    };
    let Some(mut lowering) = ReferenceStore::new(
        owner,
        effectful,
        structural_state,
        standalone,
        closed_class_library,
    ) else {
        return Cow::Borrowed(source);
    };
    lowering
        .lower_program(owner)
        .map_or(Cow::Borrowed(source), Cow::Owned)
}

fn function_contains_store_address(function: &FnDecl, structural_state: bool) -> bool {
    function_signature_types(function)
        .any(|ty| contains_reference_store_address(ty, structural_state))
}

fn function_signature_types(function: &FnDecl) -> impl Iterator<Item = &Type> {
    std::iter::once(&function.ret)
        .chain(function.params.iter().map(|parameter| &parameter.ty))
        .chain(function.annotated_ret.iter())
        .chain(function.semantic_ret.iter())
}

fn contains_actual_class_owner(ty: &Type, owners: &HashSet<&str>) -> bool {
    matches!(ty, Type::Record { source_class_owner: Some(owner), .. } if owners.contains(owner.as_str()))
        || ty
            .children()
            .into_iter()
            .any(|child| contains_actual_class_owner(child, owners))
}

fn walk(root: &Expr, mut visit: impl FnMut(&Expr)) {
    let mut pending = vec![root];
    while let Some(expr) = pending.pop() {
        visit(expr);
        pending.extend(expr.children());
    }
}

fn needs_store(expr: &Expr) -> bool {
    rules::require_formal_reference_store(expr.kind(), reference_receiver(expr))
}

fn reference_receiver(expr: &Expr) -> bool {
    matches!(&expr.node, Node::Method { object, .. } if object.ty.is_reference())
}

fn target_type(source: &Type, structural_state: bool) -> Type {
    if matches!(source, Type::Cell { .. }) || source.is_reference() && structural_state {
        return NAT;
    }
    let ty = source
        .map_children::<Infallible>(|child| Ok(target_type(child, structural_state)))
        .unwrap();
    if source.is_reference() {
        ty.with_reference(false)
    } else {
        ty
    }
}

fn contains_reference_store_address(source: &Type, structural_state: bool) -> bool {
    matches!(source, Type::Cell { .. })
        || structural_state && source.is_reference()
        || source
            .map_children(|child| {
                if contains_reference_store_address(child, structural_state) {
                    Err(())
                } else {
                    Ok(UNIT)
                }
            })
            .is_err()
}

fn pack(items: Vec<Expr>) -> Expr {
    let ty = Type::Tuple {
        elements: items.iter().map(|item| item.ty.clone()).collect(),
    };
    Expr::new(
        Node::Tuple {
            items,
            frozen: false,
            product_model: None,
        },
        ty,
    )
}

fn project(object: &Expr, index: usize) -> Expr {
    let Type::Tuple { elements } = &object.ty else {
        unreachable!("reference store packet")
    };
    Expr::new(
        Node::TupleIndex {
            object: Box::new(object.clone()),
            index,
        },
        elements[index].clone(),
    )
}

fn items(ty: Type, items: Vec<Expr>) -> Expr {
    Expr::new(Node::Array { items }, ty)
}

fn append(left: Expr, right: Expr) -> Expr {
    let ty = left.ty.clone();
    Expr::new(
        Node::Append {
            left: Box::new(left),
            right: Box::new(right),
        },
        ty,
    )
}

fn choose(cond: Expr, then: Expr, otherwise: Expr) -> Expr {
    let ty = then.ty.clone();
    Expr::new(
        Node::If {
            cond: Box::new(cond),
            then: Box::new(then),
            otherwise: Box::new(otherwise),
        },
        ty,
    )
}

fn truth(value: bool) -> Expr {
    Expr::lit(BOOL, LitValue::Bool(value))
}

fn negate(arg: Expr) -> Expr {
    Expr::new(
        Node::Unary {
            op: UnaryOp::Not,
            arg: Box::new(arg),
            semantics: None,
        },
        BOOL,
    )
}

fn absent(value: &Expr, absence: &str) -> Option<Expr> {
    crate::translation::optional_value_rules::read_optional_output_absences(absence)
        .into_iter()
        .map(|absence| {
            let ty = Type::Option {
                element: Box::new(UNIT),
                absence: absence.clone(),
            };
            let right = Expr::new(Node::None { absence }, ty);
            Expr::new(
                Node::StrictEqual {
                    left: Box::new(value.clone()),
                    right: Box::new(right),
                    negate: false,
                },
                BOOL,
            )
        })
        .reduce(|left, right| logical(BinaryOp::Or, left, right))
}

fn same_key(left: Expr, right: Expr) -> Expr {
    let equal = compare(BinaryOp::Eq, left.clone(), right.clone());
    if left.ty != Type::Float {
        return equal;
    }
    let nan = |value| {
        Expr::new(
            Node::Math {
                op: "isNaN".to_owned(),
                args: vec![value],
            },
            BOOL,
        )
    };
    logical(
        BinaryOp::Or,
        equal,
        logical(BinaryOp::And, nan(left), nan(right)),
    )
}

fn method(object: Expr, operation: &str, args: Vec<Expr>, ty: Type) -> Expr {
    Expr::new(
        Node::Method {
            object: Box::new(object),
            op: operation.to_owned(),
            args: args.into_iter().map(Some).collect(),
        },
        ty,
    )
}

enum Step {
    Bind(String, Box<Expr>),
    Print(Box<Expr>),
}

fn sequence(steps: Vec<Step>, value: Expr) -> Expr {
    steps
        .into_iter()
        .rev()
        .fold(value, |body, step| match step {
            Step::Bind(name, value) => let_in(name, *value, body),
            Step::Print(text) => {
                let ty = body.ty.clone();
                Expr::new(
                    Node::Print {
                        text,
                        body: Box::new(body),
                    },
                    ty,
                )
            }
        })
}

struct ReferenceStore {
    positions: HashMap<String, usize>,
    ty: Type,
    effectful: HashSet<String>,
    structural_state: bool,
    standalone: bool,
    closed_class_library: bool,
    shared_functions: HashMap<String, (String, String)>,
    local_closures: HashMap<String, Type>,
    prefix: String,
    count: usize,
}

impl ReferenceStore {
    fn new(
        source: &Program,
        effectful: HashSet<String>,
        structural_state: bool,
        standalone: bool,
        closed_class_library: bool,
    ) -> Option<Self> {
        let value = serde_json::to_value(source).ok()?;
        let prototype_owners: HashSet<_> = source
            .source_class_owners
            .iter()
            .filter_map(|owners| owners.get("owners").and_then(serde_json::Value::as_array))
            .flatten()
            .filter(|owner| owner["representation"] == "prototype helpers")
            .filter_map(|owner| owner["constructorName"].as_str())
            .collect();
        let mut pending = vec![&value];
        let mut types = HashMap::new();
        while let Some(value) = pending.pop() {
            match value {
                serde_json::Value::Object(fields) => {
                    if let Some(kind) = fields.get("kind").and_then(serde_json::Value::as_str) {
                        let callable = kind == "record"
                            && fields
                                .get("sourceClassOwner")
                                .and_then(serde_json::Value::as_str)
                                .is_some_and(|name| !prototype_owners.contains(name))
                            && fields
                                .get("fields")
                                .and_then(serde_json::Value::as_array)
                                .is_some_and(|fields| {
                                    fields
                                        .iter()
                                        .any(|field| field["type"]["kind"] == "function")
                                });
                        if !rules::accept_formal_reference_type(callable) {
                            return None;
                        }
                        if kind == "cell"
                            || structural_state
                                && fields.get("reference").and_then(serde_json::Value::as_bool)
                                    == Some(true)
                        {
                            let ty: Type = serde_json::from_value(value.clone()).ok()?;
                            let payload = if let Type::Cell { element } = &ty {
                                if !rules::accept_formal_local_cell_element(element.kind()) {
                                    return None;
                                }
                                array(target_type(element, structural_state))
                            } else {
                                array(target_type(&ty.with_reference(false), structural_state))
                            };
                            types.entry(ty.key()).or_insert(payload);
                        }
                    }
                    pending.extend(fields.values());
                }
                serde_json::Value::Array(entries) => pending.extend(entries),
                _ => {}
            }
        }
        if types.is_empty() {
            return None;
        }
        let mut types: Vec<_> = types.into_iter().collect();
        types.sort_unstable_by(|first, second| first.0.encode_utf16().cmp(second.0.encode_utf16()));
        let mut positions = HashMap::with_capacity(types.len());
        let mut elements = Vec::with_capacity(types.len());
        for (key, ty) in types {
            positions.insert(key, elements.len());
            elements.push(ty);
        }
        let prefix = crate::translation::source_identifiers::read_source_identifier_supply(
            crate::translation::source_identifiers::collect_source_identifier_texts(&value),
        );
        Some(Self {
            positions,
            ty: Type::Tuple { elements },
            effectful,
            structural_state,
            standalone,
            closed_class_library,
            shared_functions: HashMap::new(),
            local_closures: HashMap::new(),
            prefix,
            count: 0,
        })
    }

    fn target_type(&self, ty: &Type) -> Type {
        target_type(ty, self.structural_state)
    }

    fn operand(&mut self, value: &Expr, store: &mut Expr, steps: &mut Vec<Step>) -> Option<Expr> {
        let value = self.expression(value, store, steps)?;
        Some(match value.node {
            Node::Var { .. } | Node::Lit { .. } | Node::Unit => value,
            _ => self.bind(steps, value),
        })
    }

    fn name(&mut self, role: &str) -> String {
        self.count += 1;
        format!("{}{role}{}", self.prefix, self.count)
    }

    fn bind(&mut self, steps: &mut Vec<Step>, value: Expr) -> Expr {
        let name = self.name("referenceOperand");
        let variable = Expr::var(&name, value.ty.clone());
        steps.push(Step::Bind(name, Box::new(value)));
        variable
    }

    fn operands(&mut self, expr: &Expr, store: &mut Expr, steps: &mut Vec<Step>) -> Option<Expr> {
        let mut valid = true;
        let mut result = expr.map_children(&mut |child| {
            if valid && let Some(value) = self.operand(child, store, steps) {
                return value;
            }
            valid = false;
            child.clone()
        });
        result.ty = self.target_type(&result.ty);
        lower_node_types_with(&mut result.node, &|ty| self.target_type(ty));
        valid.then_some(result)
    }

    fn shares_store(&self, name: &str) -> bool {
        (!self.standalone || self.closed_class_library) && self.effectful.contains(name)
    }

    fn pure(&self, expr: &Expr) -> Option<Expr> {
        match &expr.node {
            Node::Call { func, .. } | Node::FunctionRef { func } if self.shares_store(func) => {
                return None;
            }
            Node::Truthy { arg } if reference_truth(&arg.ty) => return None,
            Node::Var { name } if self.local_closures.contains_key(name) => return None,
            _ if needs_store(expr) => return None,
            _ => {}
        }
        let mut valid = true;
        let mut result = expr.map_children(&mut |child| {
            self.pure(child).unwrap_or_else(|| {
                valid = false;
                child.clone()
            })
        });
        result.ty = self.target_type(&result.ty);
        lower_node_types_with(&mut result.node, &|ty| self.target_type(ty));
        valid.then_some(result)
    }

    fn values(&self, store: &Expr, source: &Type) -> Option<(usize, Expr)> {
        let index = *self.positions.get(&source.key())?;
        Some((index, project(store, index)))
    }

    fn read(&self, store: &Expr, source: &Type, address: Expr) -> Option<Expr> {
        let (_, array) = self.values(store, source)?;
        let ty = array.ty.element()?.clone();
        Some(Expr::new(
            Node::Index {
                array: Box::new(array),
                index: Box::new(address),
                proven_in_bounds: true,
            },
            ty,
        ))
    }

    fn write(&mut self, store: &mut Expr, steps: &mut Vec<Step>, index: usize, values: Expr) {
        let Type::Tuple { elements } = &self.ty else {
            unreachable!("reference store type")
        };
        let mut fields: Vec<_> = (0..elements.len()).map(|at| project(store, at)).collect();
        fields[index] = values;
        *store = self.bind(steps, pack(fields));
    }

    fn callback(&mut self, arguments: &[Type], make: impl FnOnce(&[Expr]) -> Expr) -> Expr {
        let variables: Vec<_> = arguments
            .iter()
            .map(|ty| Expr::var(self.name("referenceCallback"), ty.clone()))
            .collect();
        let body = make(&variables);
        let params = variables
            .iter()
            .map(|value| LambdaParam {
                name: value.var_name().unwrap().to_owned(),
                ty: value.ty.clone(),
                span: None,
                default_value: None,
                guard: None,
                rocq_type: None,
            })
            .collect();
        let ty = Type::Function {
            params: arguments.to_vec(),
            ret: Box::new(body.ty.clone()),
        };
        Expr::new(
            Node::Lambda {
                params,
                body: Box::new(body),
            },
            ty,
        )
    }

    fn replace(&mut self, values: Expr, address: Expr, replacement: Expr) -> Option<Expr> {
        let ty = values.ty.clone();
        let initial = pack(vec![literal(NAT, &0), items(ty.clone(), vec![])]);
        let callback = self.callback(&[initial.ty.clone(), ty.element()?.clone()], |parameters| {
            let position = project(&parameters[0], 0);
            let matches = compare(BinaryOp::Eq, address, position.clone());
            let item = choose(matches, replacement, parameters[1].clone());
            let next = Expr::new(
                Node::Binary {
                    op: BinaryOp::Add,
                    left: Box::new(position),
                    right: Box::new(literal(NAT, &1)),
                    domain: Some(NAT),
                    semantics: None,
                    rounding: None,
                    by_zero: None,
                },
                NAT,
            );
            let output = append(project(&parameters[0], 1), items(ty, vec![item]));
            pack(vec![next, output])
        });
        let packet = method(
            values,
            "reduce",
            vec![callback, initial.clone()],
            initial.ty,
        );
        Some(project(&packet, 1))
    }

    fn packet(&mut self, expr: &Expr, mut store: Expr) -> Option<Expr> {
        let mut steps = Vec::new();
        let value = self.expression(expr, &mut store, &mut steps)?;
        Some(sequence(steps, pack(vec![value, store])))
    }

    fn unpack(&mut self, packet: Expr, store: &mut Expr, steps: &mut Vec<Step>) -> Expr {
        let packet = self.bind(steps, packet);
        *store = project(&packet, 1);
        project(&packet, 0)
    }

    fn with_bindings<T>(&mut self, names: Vec<&str>, lower: impl FnOnce(&mut Self) -> T) -> T {
        let shadowed: Vec<_> = names
            .into_iter()
            .map(|name| (name, self.local_closures.remove(name)))
            .collect();
        let result = lower(self);
        for (name, ty) in shadowed {
            if let Some(ty) = ty {
                self.local_closures.insert(name.to_owned(), ty);
            } else {
                self.local_closures.remove(name);
            }
        }
        result
    }

    fn closure_type(&self, value: &Expr) -> Option<Option<Type>> {
        let Node::Lambda { params, body } = &value.node else {
            return Some(None);
        };
        let mut effectful = false;
        walk(body, |expr| {
            effectful |= needs_store(expr)
                || matches!(&expr.node, Node::Call { func, .. } if self.shares_store(func))
                || matches!(&expr.node, Node::Var { name } if self.local_closures.contains_key(name));
        });
        if !effectful {
            return Some(None);
        }
        if params
            .iter()
            .any(|parameter| parameter.default_value.is_some() || parameter.guard.is_some())
        {
            return None;
        }
        let Type::Function { mut params, ret } = self.target_type(&value.ty) else {
            return None;
        };
        params.push(self.ty.clone());
        let ret = Type::Tuple {
            elements: vec![*ret, self.ty.clone()],
        };
        Some(Some(Type::Function {
            params,
            ret: Box::new(ret),
        }))
    }

    fn lower_closure(&mut self, value: &Expr, ty: Type) -> Option<Expr> {
        let Node::Lambda { params, body } = &value.node else {
            return None;
        };
        let name = self.name("referenceStore");
        let state = Expr::var(&name, self.ty.clone());
        let names = params
            .iter()
            .map(|parameter| parameter.name.as_str())
            .collect();
        let body = self.with_bindings(names, |lowering| lowering.packet(body, state))?;
        let mut params = params.clone();
        for parameter in &mut params {
            parameter.ty = self.target_type(&parameter.ty);
        }
        params.push(LambdaParam {
            name,
            ty: self.ty.clone(),
            span: None,
            default_value: None,
            guard: None,
            rocq_type: None,
        });
        Some(
            Expr::new(
                Node::Lambda {
                    params,
                    body: Box::new(body),
                },
                ty,
            )
            .with_span(value.span),
        )
    }

    fn expression(&mut self, expr: &Expr, store: &mut Expr, steps: &mut Vec<Step>) -> Option<Expr> {
        match &expr.node {
            Node::Var { name } if self.local_closures.contains_key(name) => None,
            Node::Let { name, value, body } => {
                let closure = self.closure_type(value)?;
                let value = if let Some(ty) = &closure {
                    self.lower_closure(value, ty.clone())?
                } else {
                    self.expression(value, store, steps)?
                };
                let body = self.with_bindings(vec![name], |lowering| {
                    if let Some(ty) = closure {
                        lowering.local_closures.insert(name.clone(), ty);
                    }
                    lowering.packet(body, store.clone())
                })?;
                let packet = let_in(name.clone(), value, body);
                Some(self.unpack(packet, store, steps))
            }
            Node::Apply { func, args }
                if let Some(name) = func.var_name()
                    && let Some(Type::Function { params, ret }) =
                        self.local_closures.get(name).cloned() =>
            {
                if args.len() + 1 != params.len() {
                    return None;
                }
                let mut args = args
                    .iter()
                    .map(|argument| self.operand(argument, store, steps))
                    .collect::<Option<Vec<_>>>()?;
                args.push(store.clone());
                let func = Expr::var(
                    name,
                    Type::Function {
                        params,
                        ret: ret.clone(),
                    },
                );
                let packet = Expr::new(
                    Node::Apply {
                        func: Box::new(func),
                        args,
                    },
                    *ret,
                );
                Some(self.unpack(packet, store, steps))
            }
            Node::Print { text, body } => {
                let text = self.expression(text, store, steps)?;
                steps.push(Step::Print(Box::new(text)));
                self.expression(body, store, steps)
            }
            Node::If {
                cond,
                then,
                otherwise,
            } => {
                let cond = self.expression(cond, store, steps)?;
                let then = self.packet(then, store.clone())?;
                let otherwise = self.packet(otherwise, store.clone())?;
                Some(self.unpack(choose(cond, then, otherwise), store, steps))
            }
            Node::Binary {
                op: op @ (BinaryOp::And | BinaryOp::Or),
                left,
                right,
                ..
            } => {
                let cond = self.expression(left, store, steps)?;
                let right = self.packet(right, store.clone())?;
                let immediate = pack(vec![truth(*op == BinaryOp::Or), store.clone()]);
                let packet = if *op == BinaryOp::And {
                    choose(cond, right, immediate)
                } else {
                    choose(cond, immediate, right)
                };
                Some(self.unpack(packet, store, steps))
            }
            Node::Coalesce { left, right } => {
                let Type::Option { absence, .. } = &left.ty else {
                    return None;
                };
                let left = self.expression(left, store, steps)?;
                let left = self.bind(steps, left);
                let cond = absent(&left, absence)?;
                let right = self.packet(right, store.clone())?;
                let present = Expr::new(
                    Node::UnwrapOption {
                        arg: Box::new(left),
                    },
                    self.target_type(&expr.ty),
                );
                let packet = pack(vec![present, store.clone()]);
                Some(self.unpack(choose(cond, right, packet), store, steps))
            }
            Node::Match { scrutinee, cases } => {
                let scrutinee = self.expression(scrutinee, store, steps)?;
                let mut cases = cases.clone();
                for case in &mut cases {
                    case.body = self.with_bindings(case.pattern.bound(), |lowering| {
                        lowering.packet(&case.body, store.clone())
                    })?;
                }
                let ty = cases.first()?.body.ty.clone();
                let packet = Expr::new(
                    Node::Match {
                        scrutinee: Box::new(scrutinee),
                        cases,
                    },
                    ty,
                );
                Some(self.unpack(packet, store, steps))
            }
            Node::ReferenceRead { reference } | Node::CellRead { cell: reference } => {
                let address = self.expression(reference, store, steps)?;
                self.read(store, &reference.ty, address)
            }
            Node::ReferenceAllocate { value } | Node::CellAllocate { value } => {
                let value = self.expression(value, store, steps)?;
                let value = self.bind(steps, value);
                let (index, values) = self.values(store, &expr.ty)?;
                let measured = Expr::new(
                    Node::Length {
                        array: Box::new(values.clone()),
                    },
                    NAT,
                );
                let address = self.bind(steps, measured);
                let updated = append(values.clone(), items(values.ty, vec![value]));
                self.write(store, steps, index, updated);
                Some(address)
            }
            Node::Lambda { .. } | Node::FunctionRef { .. } => self.pure(expr),
            Node::CellWrite { cell, value, body } => {
                let address = self.expression(cell, store, steps)?;
                let address = self.bind(steps, address);
                let value = self.expression(value, store, steps)?;
                let value = self.bind(steps, value);
                let (index, values) = self.values(store, &cell.ty)?;
                let replacement = self.replace(values, address, value)?;
                self.write(store, steps, index, replacement);
                self.expression(body, store, steps)
            }
            Node::Truthy { arg } if reference_truth(&arg.ty) => {
                let value = self.expression(arg, store, steps)?;
                let value = self.bind(steps, value);
                match &arg.ty {
                    Type::Option { absence, .. } => Some(negate(absent(&value, absence)?)),
                    _ => Some(truth(true)),
                }
            }
            _ => {
                let mut result = self.operands(expr, store, steps)?;
                if let Node::Call { func, args } = &mut result.node
                    && self.shares_store(func)
                {
                    if let Some((_, implementation)) = self.shared_functions.get(func) {
                        func.clone_from(implementation);
                    }
                    args.push(store.clone());
                    result.ty = Type::Tuple {
                        elements: vec![result.ty, self.ty.clone()],
                    };
                    return Some(self.unpack(result, store, steps));
                }
                if let Node::Method { object, .. } = &expr.node
                    && object.ty.is_reference()
                {
                    return self.collection_method(&object.ty, result, store, steps);
                }
                Some(result)
            }
        }
    }

    fn collection_method(
        &mut self,
        source: &Type,
        call: Expr,
        store: &mut Expr,
        steps: &mut Vec<Step>,
    ) -> Option<Expr> {
        let Node::Method {
            object: address,
            op,
            args,
        } = call.node
        else {
            return None;
        };
        let address = *address;
        let operation = &op;
        let kind = source.kind();
        let identity = if kind == "array" {
            use crate::translation::array_state_method_rules as array_rules;
            array_rules::read_array_state_method_identity(&array_rules::ml_string::Text::from_text(
                operation,
            ))
            .to_string()
        } else {
            crate::translation::reference_method_rules::read_reference_method_identity(
                kind, operation,
            )
        };
        let args: Vec<_> = args.into_iter().collect::<Option<_>>()?;
        let arity = serde_json::Number::from(args.len()).as_f64()?;
        let selected = rules::read_formal_reference_store_operation(
            kind,
            operation,
            &identity,
            arity,
            matches!(source, Type::Record { ordered: true, .. }),
            &record_rules::read_record_field_write_operator(operation),
        );
        if selected.is_empty() {
            return None;
        }
        let value = self.read(store, source, address.clone())?;
        let value = self.bind(steps, value);
        let (replacement, output) = match selected.as_str() {
            "write field" => {
                let Type::Record { fields, .. } = &value.ty else {
                    return None;
                };
                let name = record_rules::read_record_field_write_name(operation);
                let entries = fields
                    .iter()
                    .enumerate()
                    .map(|(index, field)| RecordEntry {
                        name: field.name.clone(),
                        value: if field.name == name {
                            args[0].clone()
                        } else {
                            Expr::new(
                                Node::RecordField {
                                    object: Box::new(value.clone()),
                                    field: field.name.clone(),
                                    index,
                                },
                                field.ty.clone(),
                            )
                        },
                    })
                    .collect();
                (
                    Expr::new(
                        Node::Record {
                            entries,
                            order: None,
                        },
                        value.ty.clone(),
                    ),
                    Expr::new(Node::Unit, UNIT),
                )
            }
            "fill" => {
                let callback = self.callback(&[value.ty.element()?.clone()], |_| args[0].clone());
                (
                    method(value.clone(), "map", vec![callback], value.ty),
                    address.clone(),
                )
            }
            _ => {
                let (replacement, output) =
                    self.keyed_method(&selected, value, address.clone(), args, call.ty, steps)?;
                let Some(replacement) = replacement else {
                    return Some(output);
                };
                (replacement, output)
            }
        };
        let (index, values) = self.values(store, source)?;
        let values = self.replace(values, address, replacement)?;
        self.write(store, steps, index, values);
        Some(output)
    }

    fn keyed_method(
        &mut self,
        operation: &str,
        value: Expr,
        address: Expr,
        args: Vec<Expr>,
        result: Type,
        steps: &mut Vec<Step>,
    ) -> Option<(Option<Expr>, Expr)> {
        let is_map = matches!(value.ty, Type::Map { .. });
        let element = match &value.ty {
            Type::Map { key, value, .. } => Type::Tuple {
                elements: vec![key.as_ref().clone(), value.as_ref().clone()],
            },
            Type::Set { element, .. } => element.as_ref().clone(),
            _ => return None,
        };
        let iterable = array(element.clone());
        let construct = |entries| {
            Expr::new(
                if is_map {
                    Node::Map {
                        entries: Box::new(entries),
                    }
                } else {
                    Node::Set {
                        items: Box::new(entries),
                    }
                },
                value.ty.clone(),
            )
        };
        if operation == "clear" {
            return Some((
                Some(construct(items(iterable, vec![]))),
                Expr::new(Node::Unit, UNIT),
            ));
        }
        let key = args.first()?.clone();
        let read = |op: &str, ty| {
            Expr::new(
                Node::CollectionRead {
                    object: Box::new(value.clone()),
                    op: op.to_owned(),
                    key: Box::new(key.clone()),
                },
                ty,
            )
        };
        if operation == "has" {
            return Some((None, read("has", BOOL)));
        }
        if operation == "get" {
            return Some((None, read("get", result)));
        }
        let exists = self.bind(steps, read("has", BOOL));
        let entries = Expr::new(
            Node::CollectionItems {
                object: Box::new(value.clone()),
                op: if is_map { "entries" } else { "values" }.to_owned(),
            },
            iterable.clone(),
        );
        let matches = |item: &Expr| {
            same_key(
                if is_map {
                    project(item, 0)
                } else {
                    item.clone()
                },
                key.clone(),
            )
        };
        if operation == "delete" {
            let callback = self.callback(&[element], |params| negate(matches(&params[0])));
            let remaining = method(entries, "filter", vec![callback], iterable);
            return Some((Some(construct(remaining)), exists));
        }
        let (inserted, existing) = if is_map {
            let new_value = args.get(1)?.clone();
            let callback = self.callback(&[element], |params| {
                let item = &params[0];
                choose(
                    matches(item),
                    pack(vec![project(item, 0), new_value.clone()]),
                    item.clone(),
                )
            });
            (
                pack(vec![key.clone(), new_value]),
                construct(method(
                    entries.clone(),
                    "map",
                    vec![callback],
                    iterable.clone(),
                )),
            )
        } else {
            (key, value.clone())
        };
        let inserted = construct(append(entries, items(iterable, vec![inserted])));
        Some((Some(choose(exists, existing, inserted)), address))
    }

    fn lower_program(&mut self, source: &Program) -> Option<Program> {
        let mut result = source.clone();
        let Type::Tuple { elements } = &self.ty else {
            unreachable!("reference store type")
        };
        let initial = pack(
            elements
                .iter()
                .map(|ty| items(ty.clone(), vec![]))
                .collect(),
        );
        let structural_boundary = self.standalone && self.structural_state;
        let address_escapes = |ty: &Type| contains_reference_store_address(ty, structural_boundary);
        let class_owners: HashSet<_> = source
            .source_class_owners
            .iter()
            .filter_map(|owners| owners.get("owners").and_then(serde_json::Value::as_array))
            .flatten()
            .filter_map(|owner| owner["constructorName"].as_str())
            .collect();
        if self.closed_class_library {
            for declaration in &source.declarations {
                if let Decl::Fn(function) = declaration
                    && self.effectful.contains(&function.full_name)
                {
                    let name = self.name("referenceStoreFunction");
                    let mut path = function.module_path.clone();
                    path.push(name.clone());
                    self.shared_functions
                        .insert(function.full_name.clone(), (name, path.join(".")));
                }
            }
        }
        let mut wrappers = HashMap::new();
        let declarations = std::mem::take(&mut result.declarations);
        for mut declaration in declarations {
            let mut wrapper = None;
            match &mut declaration {
                Decl::Fn(function) => {
                    let address_boundary =
                        function_contains_store_address(function, structural_boundary);
                    if address_boundary
                        && !(self.closed_class_library
                            && (function.generated
                                || function_signature_types(function)
                                    .any(|ty| contains_actual_class_owner(ty, &class_owners))))
                    {
                        return None;
                    }
                    for parameter in &mut function.params {
                        parameter.ty = self.target_type(&parameter.ty);
                    }
                    if self.effectful.contains(&function.full_name) {
                        let implementation =
                            self.shared_functions.get(&function.full_name).cloned();
                        if implementation.is_some() && !address_boundary {
                            wrapper = Some(function.clone());
                        }
                        let name = self.name("referenceStore");
                        let store = Expr::var(&name, self.ty.clone());
                        function.body = self.packet(&function.body, store)?;
                        if !self.shares_store(&function.full_name) {
                            function.body =
                                let_in(name, initial.clone(), project(&function.body, 0));
                        } else {
                            function.params.push(Param {
                                name,
                                optional: false,
                                ty: self.ty.clone(),
                                guard: None,
                                default_value: None,
                            });
                            function.annotated_ret = None;
                            function.semantic_ret = None;
                        }
                        function.ret = function.body.ty.clone();
                        if let Some((name, full_name)) = implementation {
                            wrappers.insert(function.full_name.clone(), wrapper.is_some());
                            function.name = name;
                            function.full_name = full_name;
                            if let Some(wrapper) = &mut wrapper {
                                let mut args: Vec<_> = wrapper
                                    .params
                                    .iter()
                                    .map(|parameter| {
                                        Expr::var(&parameter.name, parameter.ty.clone())
                                    })
                                    .collect();
                                args.push(initial.clone());
                                let call = Expr::new(
                                    Node::Call {
                                        func: function.full_name.clone(),
                                        args,
                                    },
                                    function.ret.clone(),
                                );
                                wrapper.body = project(&call, 0);
                                wrapper.recursive = false;
                                wrapper.mutual = false;
                                wrapper.decreasing = None;
                            }
                        }
                    } else {
                        function.body = self.pure(&function.body)?;
                        function.ret = self.target_type(&function.ret);
                        function.annotated_ret = function
                            .annotated_ret
                            .as_ref()
                            .map(|ty| self.target_type(ty));
                        function.semantic_ret = function
                            .semantic_ret
                            .as_ref()
                            .map(|ty| self.target_type(ty));
                    }
                }
                Decl::Data(data) => {
                    for field in data.ctors.iter_mut().flat_map(|ctor| &mut ctor.fields) {
                        if address_escapes(&field.ty) {
                            return None;
                        }
                        field.ty = self.target_type(&field.ty);
                    }
                }
                Decl::Theorem(theorem) => {
                    if prop_any(&theorem.prop, &|expr| {
                        self.pure(expr).as_ref() != Some(expr)
                    }) {
                        return None;
                    }
                }
            }
            result.declarations.push(declaration);
            if let Some(wrapper) = wrapper {
                result.declarations.push(Decl::Fn(wrapper));
            }
        }
        if self.standalone {
            if self.closed_class_library {
                result.items = self.lower_items(result.items, &wrappers);
            }
            return Some(result);
        }
        let name = self.name("referenceStore");
        let mut state = Expr::var(&name, self.ty.clone());
        let main = result.main.as_mut()?;
        let mut effects = vec![binding(name, initial)];
        for mut effect in std::mem::take(&mut main.effects) {
            let expr = match &mut effect {
                Effect::Let { value, .. } | Effect::Print { expr: value, .. } => value,
                Effect::Assert { prop, .. } => {
                    if prop_any(prop, &|expr| self.pure(expr).as_ref() != Some(expr)) {
                        return None;
                    }
                    effects.push(effect);
                    continue;
                }
                _ => return None,
            };
            let packet = self.packet(expr, state)?;
            let name = self.name("referenceStep");
            let reference = Expr::var(&name, packet.ty.clone());
            effects.push(binding(name, packet));
            let name = self.name("referenceStore");
            state = Expr::var(&name, self.ty.clone());
            effects.push(binding(name, project(&reference, 1)));
            *expr = project(&reference, 0);
            effects.push(effect);
        }
        main.effects = effects;
        Some(result)
    }

    fn lower_items(&self, items: Vec<Item>, wrappers: &HashMap<String, bool>) -> Vec<Item> {
        let mut lowered = Vec::new();
        for mut item in items {
            if let Item::Decl { full_name } = &item
                && let Some((_, implementation)) = self.shared_functions.get(full_name)
            {
                lowered.push(Item::Decl {
                    full_name: implementation.clone(),
                });
                if !wrappers.get(full_name).copied().unwrap_or(false) {
                    continue;
                }
            }
            if let Item::Module { items, .. } = &mut item {
                *items = self.lower_items(std::mem::take(items), wrappers);
            }
            lowered.push(item);
        }
        lowered
    }
}

fn binding(name: String, value: Expr) -> Effect {
    Effect::Let {
        name,
        value,
        source_pure_unit_call: None,
        constant: false,
        span: None,
    }
}

fn reference_truth(ty: &Type) -> bool {
    ty.is_reference() || matches!(ty, Type::Option { element, .. } if element.is_reference())
}

fn lower_node_types_with(node: &mut Node, lower: &impl Fn(&Type) -> Type) {
    match node {
        Node::Binary { domain, .. } => *domain = domain.as_ref().map(lower),
        Node::PropositionValue { domain, .. } => *domain = lower(domain),
        Node::Cast { from, to, .. } => {
            *from = lower(from);
            *to = lower(to);
        }
        Node::Lambda { params, .. } => {
            for parameter in params {
                parameter.ty = lower(&parameter.ty);
            }
        }
        _ => {}
    }
}
