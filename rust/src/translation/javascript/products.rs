//! Source fact collection; the shared generated solver supplies all closure decisions.

use std::collections::{HashMap, HashSet};

use super::product_source_rules::solve_product_source_facts;
use super::{ROOT, Result, SEffect, SExpr, SFn, SItem, SNode, SProgram, Span, Type, unsupported};
use crate::translation::primitive_value_rules::{accept_tuple_index, read_tuple_position};

impl super::JavaScriptParser {
    /// Collect every parameter name while nested defaults keep their own delimiters.
    pub(super) fn parameter_binding_names(&self) -> Vec<String> {
        let mut names = Vec::new();
        let mut depth = 0usize;
        let mut binding = true;
        for token in &self.cursor.tokens[self.cursor.index..] {
            if depth == 0 && token.value == ")" {
                break;
            }
            if depth == 0 && binding && token.kind == super::TokenKind::Identifier {
                names.push(token.value.clone());
                binding = false;
            }
            if token.kind == super::TokenKind::Punct {
                match token.value.as_str() {
                    "(" | "[" | "{" => depth += 1,
                    ")" | "]" | "}" => depth = depth.saturating_sub(1),
                    "," if depth == 0 => binding = true,
                    _ => {}
                }
            }
        }
        names
    }

    /// Read module bindings before function scopes are entered, as the JavaScript author does.
    pub(super) fn prescan_product_bindings(&mut self) {
        self.function_names
            .extend(self.externals.iter().filter_map(|item| match item {
                super::External::Function { name, .. } => Some(name.clone()),
                super::External::Constant { .. } => None,
            }));
        let tokens = &self.cursor.tokens;
        let mut depth = 0usize;
        for index in self.cursor.index..tokens.len() {
            let token = &tokens[index];
            if depth == 0 && token.kind == super::TokenKind::Identifier {
                if token.value == "import" {
                    let mut braces = 0usize;
                    for at in index + 1..tokens.len() {
                        let imported = &tokens[at];
                        if braces == 0
                            && (imported.kind == super::TokenKind::String
                                || matches!(imported.value.as_str(), "from" | ";"))
                        {
                            break;
                        }
                        let previous = tokens[at - 1].value.as_str();
                        if imported.kind == super::TokenKind::Identifier
                            && (matches!(previous, "import" | "as")
                                || (matches!(previous, "{" | ",")
                                    && tokens.get(at + 1).is_none_or(|next| next.value != "as")))
                        {
                            self.global_binding_names.insert(imported.value.clone());
                        }
                        match imported.value.as_str() {
                            "{" => braces += 1,
                            "}" => braces = braces.saturating_sub(1),
                            _ => {}
                        }
                    }
                }
                let named_function = token.value == "function"
                    && tokens
                        .get(index + 1)
                        .is_some_and(|next| next.kind == super::TokenKind::Identifier);
                let constant_function = token.value == "const"
                    && tokens
                        .get(index + 1)
                        .is_some_and(|next| next.kind == super::TokenKind::Identifier)
                    && tokens.get(index + 2).is_some_and(|next| next.value == "=")
                    && self.starts_function(index + 3);
                if named_function || constant_function {
                    self.function_names.insert(tokens[index + 1].value.clone());
                }
            }
            if token.kind == super::TokenKind::Punct {
                match token.value.as_str() {
                    "(" | "[" | "{" => depth += 1,
                    ")" | "]" | "}" => depth = depth.saturating_sub(1),
                    _ => {}
                }
            }
        }
        self.global_binding_names
            .extend(self.function_names.iter().cloned());
        self.global_binding_names
            .extend(self.externals.iter().map(|item| item.name().to_owned()));
        self.global_binding_names
            .extend(self.literal_bindings.keys().cloned());
    }
}

pub(super) fn read_javascript_product_index(node: &SExpr) -> f64 {
    match &node.node {
        SNode::Num {
            value, negative, ..
        } => value.parse::<f64>().unwrap_or(f64::NAN) * if *negative { -1.0 } else { 1.0 },
        SNode::Unary {
            op: super::UnaryOp::Neg,
            arg,
        } if matches!(arg.node, SNode::Num { .. }) => -read_javascript_product_index(arg),
        _ => f64::NAN,
    }
}

// Number conversion retains the former usize-to-binary64 rounding. The
// unchanged shared guard runs before the existing numeric index conversion.
pub(super) fn product_index_position(index: f64, length: usize) -> Option<usize> {
    let length = serde_json::Number::from(length)
        .as_f64()
        .expect("a native tuple length has a finite binary64 representation");
    accept_tuple_index(index, length)
        .then(|| read_tuple_position(index))
        .and_then(crate::translation::frontend_rules::ml_array::number_index)
}

const fn scalar(ty: &Type) -> bool {
    matches!(
        ty,
        Type::Bool | Type::Float | Type::Fixed { .. } | Type::Unit
    )
}

fn needs_product_origin(ty: &Type) -> bool {
    matches!(ty, Type::Tuple { elements } if copy_tuple(ty) && elements.first().is_some_and(|first|
        elements.iter().any(|element| matches!(element, Type::Tuple { .. }) || element != first)))
}

fn copy_tuple(ty: &Type) -> bool {
    matches!(ty, Type::Tuple { elements } if !elements.is_empty()
        && elements.iter().all(|element| scalar(element) || copy_tuple(element)))
}

pub(super) fn children(node: &SExpr) -> Vec<&SExpr> {
    let mut values: Vec<&SExpr> = match &node.node {
        SNode::ArrayBindingRest { object, .. } => vec![object],
        SNode::ArrayBindingDefault {
            value, fallback, ..
        } => vec![value, fallback],
        SNode::Method { object, args, .. } => std::iter::once(object.as_ref())
            .chain(args.iter().flatten())
            .collect(),
        SNode::Record {
            entries,
            prototype_definition,
            ..
        } => {
            let mut values: Vec<_> = entries.iter().map(|entry| &entry.value).collect();
            if let Some(definition) = prototype_definition {
                values.push(&definition.value);
            }
            values
        }
        SNode::CopySlice { object } => vec![object],
        SNode::App { func, args } => {
            let mut values = vec![func.as_ref()];
            values.extend(args);
            values
        }
        SNode::Lambda { params, body, .. } => params
            .iter()
            .filter_map(|parameter| parameter.default_value.as_ref())
            .chain(std::iter::once(body.as_ref()))
            .collect(),
        SNode::CellWrite { cell, value, body } => vec![cell, value, body],
        SNode::Field { object, .. }
        | SNode::Some { value: object, .. }
        | SNode::FlattenOption { value: object }
        | SNode::OptionalField { object, .. }
        | SNode::CellAllocate { value: object, .. }
        | SNode::CellRead { cell: object }
        | SNode::TypeOf { arg: object }
        | SNode::Unary { arg: object, .. }
        | SNode::StringMap { object, .. }
        | SNode::ToString { arg: object }
        | SNode::Show { arg: object, .. }
        | SNode::Cast { arg: object, .. }
        | SNode::Length { object, .. }
        | SNode::ForOfIterable { object, .. }
        | SNode::StringPoints { object }
        | SNode::TupleIndex { object, .. } => vec![object],
        SNode::ReferenceWrite {
            object,
            index,
            value,
            ..
        } => {
            let mut values = vec![object.as_ref()];
            values.extend(index.iter().map(Box::as_ref));
            values.extend(value.iter().map(Box::as_ref));
            values
        }
        SNode::StringTest { object, search, .. } => vec![object, search],
        SNode::Binary { left, right, .. } | SNode::Coalesce { left, right } => vec![left, right],
        SNode::Let { value, body, .. }
        | SNode::Print {
            expr: value, body, ..
        } => vec![value, body],
        SNode::Cons { head, tail } => vec![head, tail],
        SNode::Index { object, index, .. } => vec![object, index],
        SNode::If {
            cond,
            then,
            otherwise,
        } => vec![cond, then, otherwise],
        SNode::Match { scrutinees, rows } => {
            let mut values: Vec<_> = scrutinees.iter().collect();
            values.extend(rows.iter().map(|row| &row.body));
            values
        }
        SNode::Match1 { scrutinee, cases } => {
            let mut values = vec![scrutinee.as_ref()];
            values.extend(cases.iter().map(|case| &case.body));
            values
        }
        SNode::CtorObject {
            fields,
            prototype_definition,
            ..
        } => {
            let mut values: Vec<_> = fields.iter().map(|(_, value)| value).collect();
            if let Some(definition) = prototype_definition {
                values.push(&definition.value);
            }
            values
        }
        SNode::ArrayFrom { args, .. } => args.iter().collect(),
        SNode::List { items } | SNode::Tuple { items, .. } => items.iter().collect(),
        SNode::Array { items, .. } | SNode::Math { args: items, .. } => {
            items.iter().map(|item| &item.value).collect()
        }
        _ => Vec::new(),
    };
    if let Some(test) = &node.tag_test {
        values.push(&test.object);
    }
    values
}

fn relevant_expression(node: &SExpr, nested_bindings: &HashSet<Span>) -> bool {
    matches!(&node.node, SNode::Tuple { product_model: Some(model), .. } if model == "immutable value product")
        || (matches!(node.node, SNode::Index { .. })
            && node
                .span
                .is_some_and(|span| nested_bindings.contains(&span)))
        || children(node)
            .iter()
            .any(|child| relevant_expression(child, nested_bindings))
}

pub(super) fn has_javascript_products(program: &SProgram, nested_bindings: &HashSet<Span>) -> bool {
    fn items_relevant(items: &[SItem], nested_bindings: &HashSet<Span>) -> bool {
        items.iter().any(|item| match item {
            SItem::Fn(function) => {
                function.ret.as_ref().is_some_and(needs_product_origin)
                    || function
                        .params
                        .iter()
                        .any(|param| param.ty.as_ref().is_some_and(needs_product_origin))
                    || function
                        .params
                        .iter()
                        .filter_map(|param| param.default_value.as_ref())
                        .any(|value| relevant_expression(value, nested_bindings))
                    || relevant_expression(&function.body, nested_bindings)
            }
            SItem::Module(module) => items_relevant(&module.items, nested_bindings),
            _ => false,
        })
    }
    items_relevant(&program.items, nested_bindings)
        || program
            .main
            .iter()
            .flat_map(|main| &main.effects)
            .any(|effect| match effect {
                SEffect::Let { value, .. } | SEffect::Print { expr: value, .. } => {
                    relevant_expression(value, nested_bindings)
                }
                SEffect::Assert { prop, .. } => prop_expressions(prop)
                    .iter()
                    .any(|expr| relevant_expression(expr, nested_bindings)),
            })
}

pub(super) fn prop_expressions(prop: &super::SProp) -> Vec<&SExpr> {
    match &prop.node {
        super::SPropNode::Eq(comparison)
        | super::SPropNode::Ne(comparison)
        | super::SPropNode::Lt(comparison)
        | super::SPropNode::Le(comparison)
        | super::SPropNode::Gt(comparison)
        | super::SPropNode::Ge(comparison) => vec![&comparison.left, &comparison.right],
        super::SPropNode::And { left, right }
        | super::SPropNode::Or { left, right }
        | super::SPropNode::Implies { left, right } => {
            let mut values = prop_expressions(left);
            values.extend(prop_expressions(right));
            values
        }
        super::SPropNode::Not { arg } | super::SPropNode::Forall { body: arg, .. } => {
            prop_expressions(arg)
        }
        super::SPropNode::Bool { expr } => vec![expr],
    }
}

#[derive(Clone)]
struct Function<'a> {
    item: &'a SFn,
    path: Vec<String>,
    result: usize,
    params: Vec<usize>,
    defaults: Vec<Option<usize>>,
}

struct Entry {
    ty: Option<Type>,
    span: Option<Span>,
}
struct Projection<'a> {
    node: &'a SExpr,
    object: usize,
}
struct Use<'a> {
    node: &'a SExpr,
    values: Vec<usize>,
    reason: &'static str,
}

struct Facts<'a> {
    types: &'a HashMap<*const SExpr, Type>,
    functions: HashMap<String, Function<'a>>,
    function_order: Vec<String>,
    called_results: HashSet<usize>,
    bases: Vec<bool>,
    conjunctions: Vec<bool>,
    targets: Vec<f64>,
    sources: Vec<f64>,
    potential_bases: Vec<bool>,
    potential_conjunctions: Vec<bool>,
    potential_targets: Vec<f64>,
    potential_sources: Vec<f64>,
    entries: Vec<Entry>,
    projections: Vec<Projection<'a>>,
    uses: Vec<Use<'a>>,
}

impl<'a> Facts<'a> {
    fn new(types: &'a HashMap<*const SExpr, Type>) -> Self {
        Self {
            types,
            functions: HashMap::new(),
            function_order: Vec::new(),
            called_results: HashSet::new(),
            bases: Vec::new(),
            conjunctions: Vec::new(),
            targets: Vec::new(),
            sources: Vec::new(),
            potential_bases: Vec::new(),
            potential_conjunctions: Vec::new(),
            potential_targets: Vec::new(),
            potential_sources: Vec::new(),
            entries: Vec::new(),
            projections: Vec::new(),
            uses: Vec::new(),
        }
    }

    fn fact(&mut self, ty: Option<Type>, node: Option<&'a SExpr>) -> usize {
        let id = self.bases.len();
        self.bases.push(ty.as_ref().is_some_and(scalar));
        self.conjunctions.push(true);
        self.potential_bases.push(false);
        self.potential_conjunctions.push(false);
        self.entries.push(Entry {
            ty,
            span: node.and_then(|node| node.span),
        });
        id
    }

    #[allow(clippy::cast_precision_loss)] // fact addresses are bounded by the source tree's allocations
    fn edge(&mut self, target: usize, source: usize, potential: bool) {
        self.targets.push(target as f64);
        self.sources.push(source as f64);
        if potential {
            self.potential_targets.push(target as f64);
            self.potential_sources.push(source as f64);
        }
    }

    fn collect(&mut self, items: &'a [SItem], path: &[String]) {
        for item in items {
            match item {
                SItem::Module(module) => {
                    let mut inner = path.to_vec();
                    inner.push(module.name.clone());
                    self.collect(&module.items, &inner);
                }
                SItem::Fn(function) => {
                    let result = self.fact(function.ret.clone(), None);
                    self.entries[result].span = function.span;
                    let params = function
                        .params
                        .iter()
                        .map(|param| self.fact(param.ty.clone(), None))
                        .collect::<Vec<_>>();
                    let defaults = function
                        .params
                        .iter()
                        .map(|param| {
                            param
                                .default_value
                                .as_ref()
                                .map(|value| self.fact(param.ty.clone(), Some(value)))
                        })
                        .collect();
                    if function.ret.as_ref().is_some_and(needs_product_origin) {
                        self.potential_bases[result] = true;
                    }
                    for (param, id) in function.params.iter().zip(&params) {
                        self.entries[*id].span = param.span;
                        if param.ty.as_ref().is_some_and(needs_product_origin) {
                            self.potential_bases[*id] = true;
                        }
                    }
                    let mut full = path.to_vec();
                    full.push(function.name.clone());
                    self.function_order.push(full.join("."));
                    self.functions.insert(
                        full.join("."),
                        Function {
                            item: function,
                            path: path.to_vec(),
                            result,
                            params,
                            defaults,
                        },
                    );
                }
                _ => {}
            }
        }
    }

    fn signature(&self, path: &[String], context: &[String]) -> Option<Function<'a>> {
        self.functions
            .get(&path.join("."))
            .or_else(|| {
                (1..=context.len()).rev().find_map(|depth| {
                    let mut candidate = context[..depth].to_vec();
                    candidate.extend_from_slice(path.get(1..).unwrap_or_default());
                    self.functions.get(&candidate.join("."))
                })
            })
            .cloned()
    }

    fn call(
        &mut self,
        node: &'a SExpr,
        path: &[String],
        args: &'a [SExpr],
        env: &HashMap<String, usize>,
        context: &[String],
        id: usize,
    ) {
        let function = self.signature(path, context);
        let values = args
            .iter()
            .map(|arg| self.expression(arg, env, context))
            .collect::<Vec<_>>();
        let Some(function) = function else {
            self.uses.push(Use {
                node,
                values,
                reason: "a product cannot escape to an unknown or external caller",
            });
            return;
        };
        self.called_results.insert(function.result);
        for (index, param) in function.params.iter().enumerate() {
            let value = values
                .get(index)
                .copied()
                .or(function.defaults[index])
                .unwrap_or_else(|| self.fact(None, None));
            self.edge(*param, value, true);
        }
        let potential = matches!(self.entries[id].ty, Some(Type::Tuple { .. }));
        self.edge(id, function.result, potential);
    }

    fn expression(
        &mut self,
        node: &'a SExpr,
        env: &HashMap<String, usize>,
        context: &[String],
    ) -> usize {
        let ty = self
            .types
            .get(&std::ptr::from_ref(node))
            .cloned()
            .or_else(|| match &node.node {
                SNode::Num { ty, .. } => ty.clone(),
                _ => None,
            });
        let potential = matches!(ty, Some(Type::Tuple { .. }));
        let id = self.fact(ty, Some(node));
        match &node.node {
            SNode::Tuple {
                items,
                product_model,
                ..
            } => {
                let admitted = product_model.as_deref() == Some("immutable value product");
                if admitted {
                    self.potential_bases[id] = true;
                }
                for item in items {
                    let value = self.expression(item, env, context);
                    if admitted {
                        self.edge(id, value, false);
                    }
                }
            }
            SNode::Name { path } => {
                if let [name] = path.as_slice()
                    && let Some(value) = env.get(name)
                {
                    self.edge(id, *value, true);
                } else {
                    self.call(node, path, &[], env, context, id);
                }
            }
            SNode::App { func, args } => {
                if let SNode::Name { path } = &func.node
                    && !(path.len() == 1 && env.contains_key(&path[0]))
                {
                    self.call(node, path, args, env, context, id);
                } else {
                    let mut values = vec![self.expression(func, env, context)];
                    values.extend(args.iter().map(|arg| self.expression(arg, env, context)));
                    self.uses.push(Use {
                        node,
                        values,
                        reason: "a product needs a statically owned direct caller",
                    });
                }
            }
            SNode::Let {
                name, value, body, ..
            } => {
                let value = self.expression(value, env, context);
                let mut scope = env.clone();
                scope.insert(name.clone(), value);
                let body = self.expression(body, &scope, context);
                self.edge(id, body, potential);
            }
            SNode::If {
                cond,
                then,
                otherwise,
            } => {
                let condition = self.expression(cond, env, context);
                self.uses.push(Use {
                    node,
                    values: vec![condition],
                    reason: "whole product truthiness needs its own shared value predicate",
                });
                let yes = self.expression(then, env, context);
                let no = self.expression(otherwise, env, context);
                self.edge(id, yes, potential);
                self.edge(id, no, potential);
            }
            SNode::Index { object, index, .. } => {
                let object = self.expression(object, env, context);
                self.expression(index, env, context);
                self.projections.push(Projection { node, object });
                self.edge(id, object, potential);
            }
            SNode::TupleIndex { object, .. } => {
                let object = self.expression(object, env, context);
                self.projections.push(Projection { node, object });
                self.edge(id, object, potential);
            }
            SNode::Print { expr, body, .. } => {
                let printed = self.expression(expr, env, context);
                self.uses.push(Use {
                    node,
                    values: vec![printed],
                    reason: "whole product display needs its own value-format operation",
                });
                let body = self.expression(body, env, context);
                self.edge(id, body, potential);
            }
            _ => {
                let values = children(node)
                    .into_iter()
                    .map(|child| self.expression(child, env, context))
                    .collect();
                self.uses.push(Use { node, values, reason: "a product only supports owned bindings, direct calls, and fixed positional reads" });
            }
        }
        id
    }
}

pub(super) fn validate_javascript_products(
    program: &mut SProgram,
    types: &HashMap<*const SExpr, Type>,
    exported_names: &HashSet<String>,
    nested_bindings: &HashSet<Span>,
) -> Result<()> {
    let mut facts = Facts::new(types);
    facts.collect(&program.items, &[ROOT.to_owned()]);
    let functions = facts
        .function_order
        .iter()
        .map(|key| facts.functions[key].clone())
        .collect::<Vec<_>>();
    for function in &functions {
        let mut defaults = HashMap::new();
        for (index, param) in function.item.params.iter().enumerate() {
            if let Some(value) = &param.default_value {
                let id = facts.expression(value, &defaults, &function.path);
                facts.edge(
                    function.defaults[index].expect("the declared default has an allocated fact"),
                    id,
                    true,
                );
            }
            defaults.insert(param.name.clone(), function.params[index]);
        }
    }
    for function in &functions {
        let env = function
            .item
            .params
            .iter()
            .zip(&function.params)
            .map(|(param, id)| (param.name.clone(), *id))
            .collect();
        let body = facts.expression(&function.item.body, &env, &function.path);
        facts.edge(
            function.result,
            body,
            matches!(function.item.ret, Some(Type::Tuple { .. })),
        );
    }
    let mut env = HashMap::new();
    for effect in program.main.iter().flat_map(|main| &main.effects) {
        match effect {
            SEffect::Let { name, value, .. } => {
                let id = facts.expression(value, &env, &[ROOT.to_owned()]);
                env.insert(name.clone(), id);
            }
            SEffect::Print { expr, .. } => {
                facts.expression(expr, &env, &[ROOT.to_owned()]);
            }
            SEffect::Assert { prop, .. } => {
                let values = prop_expressions(prop)
                    .into_iter()
                    .map(|value| facts.expression(value, &env, &[ROOT.to_owned()]))
                    .collect();
                if let Some(node) = prop_expressions(prop).first().copied() {
                    facts.uses.push(Use { node, values, reason: "a product only supports owned bindings, direct calls, and fixed positional reads" });
                }
            }
        }
    }
    let potential = solve_product_source_facts(
        &facts.potential_bases,
        &facts.potential_targets,
        &facts.potential_sources,
        &facts.potential_conjunctions,
    );
    let owned = solve_product_source_facts(
        &facts.bases,
        &facts.targets,
        &facts.sources,
        &facts.conjunctions,
    );
    for (id, entry) in facts.entries.iter().enumerate() {
        if potential[id] && !owned[id] {
            return Err(unsupported(
                "immutable tuple product origin",
                "every product position and every actual caller needs a fresh recursively immutable scalar Copy value",
                entry.span,
            ));
        }
    }
    for usage in &facts.uses {
        if usage.values.iter().any(|id| potential[*id]) {
            return Err(unsupported(
                "immutable tuple product use",
                usage.reason,
                usage.node.span,
            ));
        }
    }
    for function in &functions {
        if potential[function.result] && !facts.called_results.contains(&function.result) {
            return Err(unsupported(
                "Object.freeze operand",
                "a returned immutable product needs a closed set of actual callers",
                function.item.body.span,
            ));
        }
    }
    for function in &functions {
        if [
            Some(function.item.name.as_str()),
            function.path.get(1).map(String::as_str),
        ]
        .into_iter()
        .flatten()
        .any(|name| exported_names.contains(name))
            && std::iter::once(&function.result)
                .chain(&function.params)
                .any(|id| potential[*id])
        {
            return Err(unsupported(
                "exported immutable tuple function",
                "external callers cannot be proved to use the selected immutable value model",
                function.item.span,
            ));
        }
    }
    let mut admitted = HashMap::new();
    for projection in &facts.projections {
        if potential[projection.object] {
            #[allow(clippy::cast_precision_loss)]
            let index = match &projection.node.node {
                SNode::TupleIndex { index, .. } => *index as f64,
                SNode::Index { index, .. } => read_javascript_product_index(index),
                _ => f64::NAN,
            };
            let valid = match &facts.entries[projection.object].ty {
                Some(Type::Tuple { elements }) => product_index_position(index, elements.len()),
                _ => None,
            };
            let Some(index) = valid else {
                return Err(unsupported(
                    "immutable tuple index",
                    "a product read needs a fixed in-range positional index",
                    projection.node.span,
                ));
            };
            admitted.insert(std::ptr::from_ref(projection.node), index);
        } else if projection
            .node
            .span
            .is_some_and(|span| nested_bindings.contains(&span))
        {
            return Err(unsupported(
                "nested array binding",
                "nested destructuring requires a proved immutable product; arbitrary array iterators and accessors need their own evaluation model",
                projection.node.span,
            ));
        }
    }
    drop(facts);
    drop(functions);
    apply_projections(program, &admitted);
    Ok(())
}

fn apply_projections(program: &mut SProgram, admitted: &HashMap<*const SExpr, usize>) {
    fn walk(node: &mut SExpr, admitted: &HashMap<*const SExpr, usize>) {
        if let Some(index) = admitted.get(&std::ptr::from_ref(node))
            && let SNode::Index { object, .. } = &mut node.node
        {
            let object = std::mem::replace(object, Box::new(SExpr::new(SNode::Unit, None)));
            node.node = SNode::TupleIndex {
                object,
                index: *index,
            };
        }
        match &mut node.node {
            SNode::ArrayBindingRest { object, .. } => walk(object, admitted),
            SNode::ArrayBindingDefault {
                value, fallback, ..
            } => {
                walk(value, admitted);
                walk(fallback, admitted);
            }
            SNode::Record {
                entries,
                prototype_definition,
                ..
            } => {
                for entry in entries {
                    walk(&mut entry.value, admitted);
                }
                if let Some(definition) = prototype_definition {
                    walk(&mut definition.value, admitted);
                }
            }
            SNode::Lambda { params, body, .. } => {
                for parameter in params {
                    if let Some(default) = &mut parameter.default_value {
                        walk(default, admitted);
                    }
                }
                walk(body, admitted);
            }
            SNode::CellWrite { cell, value, body } => {
                walk(cell, admitted);
                walk(value, admitted);
                walk(body, admitted);
            }
            SNode::Method { object, args, .. } => {
                walk(object, admitted);
                for arg in args.iter_mut().flatten() {
                    walk(arg, admitted);
                }
            }
            SNode::CopySlice { object } => walk(object, admitted),
            SNode::App { func, args } => {
                walk(func, admitted);
                for arg in args {
                    walk(arg, admitted);
                }
            }
            SNode::Field { object, .. }
            | SNode::Some { value: object, .. }
            | SNode::FlattenOption { value: object }
            | SNode::OptionalField { object, .. }
            | SNode::CellAllocate { value: object, .. }
            | SNode::CellRead { cell: object }
            | SNode::TypeOf { arg: object }
            | SNode::Unary { arg: object, .. }
            | SNode::StringMap { object, .. }
            | SNode::ToString { arg: object }
            | SNode::Show { arg: object, .. }
            | SNode::Cast { arg: object, .. }
            | SNode::Length { object, .. }
            | SNode::ForOfIterable { object, .. }
            | SNode::StringPoints { object }
            | SNode::TupleIndex { object, .. } => walk(object, admitted),
            SNode::ReferenceWrite {
                object,
                index,
                value,
                ..
            } => {
                walk(object, admitted);
                if let Some(index) = index {
                    walk(index, admitted);
                }
                if let Some(value) = value {
                    walk(value, admitted);
                }
            }
            SNode::StringTest { object, search, .. } => {
                walk(object, admitted);
                walk(search, admitted);
            }
            SNode::Binary { left, right, .. }
            | SNode::Coalesce { left, right }
            | SNode::Let {
                value: left,
                body: right,
                ..
            }
            | SNode::Print {
                expr: left,
                body: right,
                ..
            }
            | SNode::Cons {
                head: left,
                tail: right,
            }
            | SNode::Index {
                object: left,
                index: right,
                ..
            } => {
                walk(left, admitted);
                walk(right, admitted);
            }
            SNode::If {
                cond,
                then,
                otherwise,
            } => {
                walk(cond, admitted);
                walk(then, admitted);
                walk(otherwise, admitted);
            }
            SNode::Match { scrutinees, rows } => {
                for value in scrutinees {
                    walk(value, admitted);
                }
                for row in rows {
                    walk(&mut row.body, admitted);
                }
            }
            SNode::Match1 { scrutinee, cases } => {
                walk(scrutinee, admitted);
                for case in cases {
                    walk(&mut case.body, admitted);
                }
            }
            SNode::CtorObject {
                fields,
                prototype_definition,
                ..
            } => {
                for (_, value) in fields {
                    walk(value, admitted);
                }
                if let Some(definition) = prototype_definition {
                    walk(&mut definition.value, admitted);
                }
            }
            SNode::ArrayFrom { args, .. } => {
                for argument in args {
                    walk(argument, admitted);
                }
            }
            SNode::List { items } | SNode::Tuple { items, .. } => {
                for value in items {
                    walk(value, admitted);
                }
            }
            SNode::Array { items, .. } | SNode::Math { args: items, .. } => {
                for item in items {
                    walk(&mut item.value, admitted);
                }
            }
            _ => {}
        }
        if let Some(test) = &mut node.tag_test {
            walk(&mut test.object, admitted);
        }
    }
    fn walk_items(items: &mut [SItem], admitted: &HashMap<*const SExpr, usize>) {
        for item in items {
            match item {
                SItem::Fn(function) => {
                    for param in &mut function.params {
                        if let Some(default) = &mut param.default_value {
                            walk(default, admitted);
                        }
                    }
                    walk(&mut function.body, admitted);
                }
                SItem::Module(module) => walk_items(&mut module.items, admitted),
                _ => {}
            }
        }
    }
    fn walk_prop(prop: &mut super::SProp, admitted: &HashMap<*const SExpr, usize>) {
        match &mut prop.node {
            super::SPropNode::Eq(pair)
            | super::SPropNode::Ne(pair)
            | super::SPropNode::Lt(pair)
            | super::SPropNode::Le(pair)
            | super::SPropNode::Gt(pair)
            | super::SPropNode::Ge(pair) => {
                walk(&mut pair.left, admitted);
                walk(&mut pair.right, admitted);
            }
            super::SPropNode::And { left, right }
            | super::SPropNode::Or { left, right }
            | super::SPropNode::Implies { left, right } => {
                walk_prop(left, admitted);
                walk_prop(right, admitted);
            }
            super::SPropNode::Not { arg } | super::SPropNode::Forall { body: arg, .. } => {
                walk_prop(arg, admitted);
            }
            super::SPropNode::Bool { expr } => walk(expr, admitted),
        }
    }
    walk_items(&mut program.items, admitted);
    for effect in program.main.iter_mut().flat_map(|main| &mut main.effects) {
        match effect {
            SEffect::Let { value, .. } | SEffect::Print { expr: value, .. } => {
                walk(value, admitted);
            }
            SEffect::Assert { prop, .. } => walk_prop(prop, admitted),
        }
    }
}
