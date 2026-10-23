//! Source ownership evidence retained while Rust indirection projects to values.
//!
//! Mirrors `js/src/translation/rust-source-origins.js`. Declarations and lexical
//! scopes come from the existing parser; value typing remains in the checker.

use std::collections::{HashMap, HashSet, VecDeque};

use super::array_rules::{
    accept_rust_fresh_array_result_tail, accept_rust_source_array_coercion,
    accept_rust_source_array_owner,
};
use super::indirection_rules::{
    accept_rust_immutable_indirection_layer, accept_rust_source_box_availability,
    accept_rust_source_box_coercion_layer, accept_rust_source_box_owner,
    accept_rust_source_indirection_argument, accept_rust_source_text_equality,
    read_rust_source_expression_origin,
};
use super::product_rules::{accept_rust_immutable_product_type, accept_rust_product_origin};
use super::{Result, SExpr, SNode, Span, Type, unsupported};

mod patterns;
mod type_owners;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct SourceOrigin {
    pub(super) ty: Option<Type>,
    /// Explicit primitive f64 annotation or literal evidence, never fallback inference.
    pub(super) float_source_model: Option<&'static str>,
    pub(super) path: Vec<String>,
    pub(super) wrappers: Vec<String>,
    pub(super) proven: bool,
    pub(super) diverging: bool,
    pub(super) uncertain: bool,
    /// Source Copy evidence does not choose the checker's numeric width.
    pub(super) copy_evidence: Option<()>,
    pub(super) fields: Option<Vec<Self>>,
    pub(super) text_source_model: Option<&'static str>,
    pub(super) array_source_model: Option<&'static str>,
    /// Constructor or actual call source evidence; cleared on local binding.
    pub(super) fresh_array_value: Option<Span>,
    /// Spelled source lifetimes, in reference-layer order; None is elision.
    pub(super) source_lifetimes: Vec<Option<String>>,
    pub(super) source_owner: Option<super::super::source_signatures::SourceOwner>,
    pub(super) primitive_owner_position: Option<serde_json::Value>,
    pub(super) namespace_spelling: Vec<String>,
}

impl SourceOrigin {
    pub(super) const fn unknown() -> Self {
        Self {
            ty: None,
            float_source_model: None,
            path: Vec::new(),
            wrappers: Vec::new(),
            proven: false,
            diverging: false,
            uncertain: false,
            copy_evidence: None,
            fields: None,
            text_source_model: None,
            array_source_model: None,
            source_lifetimes: Vec::new(),
            source_owner: None,
            primitive_owner_position: None,
            namespace_spelling: Vec::new(),
            fresh_array_value: None,
        }
    }

    pub(super) fn with_type(ty: Type, path: &[String], wrappers: Vec<String>) -> Self {
        let fields = match &ty {
            Type::Tuple { elements } => Some(
                elements
                    .iter()
                    .map(|element| Self::with_type(element.clone(), path, Vec::new()))
                    .collect::<Vec<_>>(),
            ),
            _ => None,
        };
        let children_accepted = fields
            .as_ref()
            .is_none_or(|fields| fields.iter().all(Self::direct_copy_value));
        let copy_evidence = accept_rust_immutable_product_type(ty.kind(), children_accepted, false);
        let text_source_model = (ty == super::STRING).then_some("owned string");
        let array_source_model = matches!(ty, Type::Array { .. }).then_some("owned vector");
        Self {
            float_source_model: (ty == super::FLOAT).then_some("binary64 scalar"),
            ty: Some(ty),
            path: path.to_vec(),
            wrappers,
            proven: true,
            diverging: false,
            uncertain: false,
            copy_evidence: copy_evidence.then_some(()),
            fields,
            text_source_model,
            array_source_model,
            source_lifetimes: Vec::new(),
            source_owner: None,
            primitive_owner_position: None,
            namespace_spelling: Vec::new(),
            fresh_array_value: None,
        }
    }

    pub(super) fn value() -> Self {
        Self {
            proven: true,
            ..Self::unknown()
        }
    }

    pub(super) fn direct_copy_value(&self) -> bool {
        self.proven
            && self.copy_evidence.is_some()
            && self.wrappers.is_empty()
            && self
                .fields
                .as_ref()
                .is_none_or(|fields| fields.iter().all(Self::direct_copy_value))
    }

    fn scalar_copy_value(ty: Option<Type>) -> Self {
        Self {
            ty,
            copy_evidence: Some(()),
            ..Self::value()
        }
    }

    fn text_value() -> Self {
        Self {
            ty: Some(super::STRING),
            text_source_model: Some("owned string"),
            ..Self::value()
        }
    }

    fn direct_string_value(&self) -> bool {
        self.text_source_model == Some("owned string") && self.wrappers.is_empty()
    }

    fn borrowed_string_slice(&self) -> bool {
        self.text_source_model == Some("string slice")
            && self.wrappers.len() == 1
            && self.wrappers[0] == "reference"
    }

    fn uncertain() -> Self {
        Self {
            uncertain: true,
            ..Self::unknown()
        }
    }
}

#[derive(Clone, Debug)]
pub(super) enum SourceDeclaration {
    ExternalCrate,
    Module,
    Data,
    Constructor {
        owner: String,
        fields: Vec<SourceOrigin>,
        result: SourceOrigin,
    },
    Function {
        parameters: Vec<SourceOrigin>,
        result: SourceOrigin,
    },
}

pub(super) struct SourceLookup {
    pub(super) declaration: SourceDeclaration,
    pub(super) full_name: String,
}

pub(super) struct SourceOrigins {
    pub(super) shared_array_handles: bool,
    pub(super) array_result_model: String,
    pub(super) owned_local_values: bool,
    fresh_calls: HashSet<Span>,
    pub(super) collecting: bool,
    pub(super) catalog: HashMap<String, SourceDeclaration>,
    pub(super) unit_clone_facts: HashMap<String, (Option<serde_json::Value>, bool)>,
    pub(super) scope: HashMap<String, SourceOrigin>,
    pub(super) current_type: SourceOrigin,
    // Source ranges survive the copies used to construct let chains and rows.
    // Erased wrapper expressions deliberately retain their operand's range, so
    // successive wraps and dereferences update the same source evidence.
    expressions: HashMap<Span, SourceOrigin>,
    pattern_references: HashMap<Span, usize>,
    wildcard_references: VecDeque<usize>,
}

impl SourceOrigins {
    pub(super) fn new(catalog: Option<HashMap<String, SourceDeclaration>>, model: &str) -> Self {
        Self {
            shared_array_handles: false,
            array_result_model: model.to_owned(),
            owned_local_values: false,
            fresh_calls: HashSet::new(),
            collecting: catalog.is_none(),
            catalog: catalog.unwrap_or_default(),
            unit_clone_facts: HashMap::new(),
            scope: HashMap::new(),
            current_type: SourceOrigin::unknown(),
            expressions: HashMap::new(),
            pattern_references: HashMap::new(),
            wildcard_references: VecDeque::new(),
        }
    }

    pub(super) fn enter(
        &mut self,
        scope: Option<HashMap<String, SourceOrigin>>,
    ) -> HashMap<String, SourceOrigin> {
        let next = scope.unwrap_or_else(|| self.scope.clone());
        std::mem::replace(&mut self.scope, next)
    }

    pub(super) fn typed(&mut self, ty: Type, path: &[String], wrappers: Vec<String>) -> Type {
        self.current_type = SourceOrigin::with_type(ty.clone(), path, wrappers);
        ty
    }

    pub(super) fn require_product_origin(
        &self,
        origin: &SourceOrigin,
        span: Option<Span>,
    ) -> Result<()> {
        if !self.collecting
            && (origin.fields.is_none()
                || !accept_rust_product_origin(
                    origin.direct_copy_value(),
                    self.owned_local_product(origin),
                ))
        {
            return Err(unsupported(
                "immutable Rust tuple product",
                "every field requires a direct recursively immutable scalar Copy source value",
                span,
            ));
        }
        Ok(())
    }

    pub(super) fn product(
        &mut self,
        node: &SExpr,
        path: &[String],
        span: Option<Span>,
    ) -> Result<()> {
        let SNode::Tuple { items, .. } = &node.node else {
            return Ok(());
        };
        let fields: Vec<_> = items.iter().map(|item| self.origin(item, path)).collect();
        let ty = self
            .owned_local_values
            .then(|| {
                fields
                    .iter()
                    .map(|field| field.ty.clone())
                    .collect::<Option<Vec<_>>>()
            })
            .flatten()
            .map(|elements| Type::Tuple { elements });
        let origin = SourceOrigin {
            ty,
            copy_evidence: fields
                .iter()
                .all(SourceOrigin::direct_copy_value)
                .then_some(()),
            fields: Some(fields),
            ..SourceOrigin::value()
        };
        self.require_product_origin(&origin, span)?;
        self.mark(node, origin);
        Ok(())
    }

    pub(super) fn owned_local_product(&self, origin: &SourceOrigin) -> bool {
        self.owned_local_values
            && origin
                .ty
                .as_ref()
                .is_some_and(super::owned_local_state::owned_pair)
            && origin.proven
            && origin.wrappers.is_empty()
            && origin.fields.as_ref().is_some_and(|fields| {
                fields
                    .iter()
                    .all(|field| field.proven && field.wrappers.is_empty())
                    && fields
                        .first()
                        .is_some_and(SourceOrigin::direct_string_value)
            })
    }

    pub(super) fn product_projection(
        &mut self,
        node: &SExpr,
        path: &[String],
        span: Option<Span>,
    ) -> Result<()> {
        let SNode::TupleIndex { object, index } = &node.node else {
            return Ok(());
        };
        let origin = self.origin(object, path);
        self.require_product_origin(&origin, span)?;
        let field = origin
            .fields
            .as_ref()
            .and_then(|fields| fields.get(*index))
            .cloned()
            .unwrap_or_else(SourceOrigin::unknown);
        self.mark(node, field);
        Ok(())
    }

    pub(super) fn declaration(
        &mut self,
        name: &str,
        path: &[String],
        entry: SourceDeclaration,
    ) -> Result<()> {
        if self.collecting {
            let mut full_path = path.to_vec();
            full_path.push(name.to_owned());
            let key = full_path.join(".");
            if let Some(existing) = self.catalog.get(&key) {
                let existing_crate = matches!(existing, SourceDeclaration::ExternalCrate);
                let new_crate = matches!(entry, SourceDeclaration::ExternalCrate);
                if existing_crate != new_crate {
                    return Err(unsupported(
                        "extern crate alloc",
                        "a source crate declaration cannot share its module name with another item",
                        None,
                    ));
                }
            }
            self.catalog.insert(key, entry);
        }
        Ok(())
    }

    /// Resolve forward aliases with the completed parser's existing resolver.
    pub(super) fn resolve_catalog_types<F>(&mut self, mut resolve: F)
    where
        F: FnMut(&[String], &[String]) -> Vec<String>,
    {
        let mut resolve_origin = |origin: &mut SourceOrigin| {
            if let Some(Type::Named { path, .. }) = &mut origin.ty {
                *path = resolve(path, &origin.path);
            }
        };
        for declaration in self.catalog.values_mut() {
            match declaration {
                SourceDeclaration::Function { parameters, result } => {
                    parameters.iter_mut().for_each(&mut resolve_origin);
                    resolve_origin(result);
                }
                SourceDeclaration::Constructor { fields, result, .. } => {
                    fields.iter_mut().for_each(&mut resolve_origin);
                    resolve_origin(result);
                }
                SourceDeclaration::ExternalCrate
                | SourceDeclaration::Module
                | SourceDeclaration::Data => {}
            }
        }
    }

    pub(super) fn box_owner(
        &self,
        segments: &[String],
        resolved: &[String],
        path: &[String],
        span: Option<Span>,
    ) -> Result<()> {
        if self.collecting {
            return Ok(());
        }
        let owner = resolved.join("::");
        let shadowed = [segments.first(), resolved.first()]
            .into_iter()
            .flatten()
            .any(|name| {
                self.lookup(std::slice::from_ref(name), path)
                    .is_some_and(|entry| {
                        !matches!(entry.declaration, SourceDeclaration::ExternalCrate)
                    })
            });
        if !accept_rust_source_box_owner(&owner, shadowed) {
            return Err(unsupported(
                "Box owner",
                "only the unshadowed standard immutable Box is portable",
                span,
            ));
        }
        let declared = resolved
            .first()
            .and_then(|name| self.lookup(std::slice::from_ref(name), path))
            .is_some_and(|entry| matches!(entry.declaration, SourceDeclaration::ExternalCrate));
        if !accept_rust_source_box_availability(&owner, declared) {
            return Err(unsupported(
                "Box source crate",
                "alloc requires an actual extern crate alloc declaration",
                span,
            ));
        }
        Ok(())
    }

    pub(super) fn vector_owner(
        &self,
        segments: &[String],
        resolved: &[String],
        path: &[String],
        span: Option<Span>,
    ) -> Result<()> {
        if self.collecting {
            return Ok(());
        }
        let shadowed = [segments.first(), resolved.first()]
            .into_iter()
            .flatten()
            .any(|name| self.lookup(std::slice::from_ref(name), path).is_some());
        if !accept_rust_source_array_owner(&resolved.join("::"), shadowed) {
            return Err(unsupported(
                "Vec owner",
                "only the unshadowed standard vector is admitted by the immutable array model",
                span,
            ));
        }
        Ok(())
    }

    /// Compare retained source types by their existing declaration identities.
    pub(super) fn same_origin(&self, left: &SourceOrigin, right: &SourceOrigin) -> bool {
        if left.proven != right.proven
            || left.diverging != right.diverging
            || left.uncertain != right.uncertain
            || left.wrappers != right.wrappers
            || left.copy_evidence != right.copy_evidence
            || left.float_source_model != right.float_source_model
            || left.text_source_model != right.text_source_model
            || left.array_source_model != right.array_source_model
        {
            return false;
        }
        match (&left.fields, &right.fields) {
            (Some(left), Some(right)) => {
                if left.len() != right.len()
                    || !left
                        .iter()
                        .zip(right)
                        .all(|(left, right)| self.same_origin(left, right))
                {
                    return false;
                }
            }
            (None, None) => {}
            _ => return false,
        }
        match (&left.ty, &right.ty) {
            (
                Some(Type::Named {
                    path: left_path, ..
                }),
                Some(Type::Named {
                    path: right_path, ..
                }),
            ) => {
                match (
                    self.lookup(left_path, &left.path),
                    self.lookup(right_path, &right.path),
                ) {
                    (Some(left), Some(right)) => left.full_name == right.full_name,
                    _ => left_path == right_path && left.path == right.path,
                }
            }
            _ => left.ty == right.ty,
        }
    }

    pub(super) fn origin(&self, node: &SExpr, path: &[String]) -> SourceOrigin {
        if let Some(origin) = node.span.and_then(|span| self.expressions.get(&span)) {
            return origin.clone();
        }
        if let SNode::Name { path: names } = &node.node
            && names.len() == 1
            && let Some(origin) = self.scope.get(&names[0])
        {
            return origin.clone();
        }
        let names = match &node.node {
            SNode::Name { path } => Some(path),
            SNode::App { func, .. } => match &func.node {
                SNode::Name { path } => Some(path),
                _ => None,
            },
            _ => None,
        };
        if let Some(entry) = names.and_then(|names| self.lookup(names, path)) {
            match entry.declaration {
                SourceDeclaration::Function { mut result, .. } => {
                    result.fresh_array_value = node.span.filter(|span| {
                        result.fresh_array_value.is_some() && self.fresh_calls.contains(span)
                    });
                    return result;
                }
                SourceDeclaration::Constructor { result, .. } => return result,
                SourceDeclaration::ExternalCrate
                | SourceDeclaration::Module
                | SourceDeclaration::Data => {}
            }
        }
        if let SNode::Name { path: names } = &node.node
            && names.len() == 2
            && self.lookup(&names[..1], path).is_none()
            && super::rust_fixed_type(&names[0]).is_some()
            && ["MIN", "MAX"].contains(&names[1].as_str())
        {
            return SourceOrigin::scalar_copy_value(super::rust_fixed_type(&names[0]));
        }
        if let SNode::If {
            then, otherwise, ..
        } = &node.node
        {
            return self.join(&[self.origin(then, path), self.origin(otherwise, path)]);
        }
        let operation = match &node.node {
            SNode::Binary { op, .. } => op.name(),
            _ => "",
        };
        let form = read_rust_source_expression_origin(node.node.kind(), operation);
        match form.as_str() {
            "value" => match &node.node {
                SNode::Num { ty, .. } => SourceOrigin::scalar_copy_value(ty.clone()),
                SNode::Bool { .. } => SourceOrigin::scalar_copy_value(Some(super::BOOL)),
                SNode::Unit => SourceOrigin::scalar_copy_value(Some(super::UNIT)),
                _ => SourceOrigin::value(),
            },
            "diverging" => SourceOrigin {
                diverging: true,
                ..SourceOrigin::value()
            },
            "derived-value"
            | "numeric-cast"
            | "primitive-operation"
            | "comparison"
            | "boolean-operation" => {
                let nodes: Vec<&SExpr> = match &node.node {
                    SNode::Unary { arg, .. }
                    | SNode::Cast { arg, .. }
                    | SNode::ToString { arg }
                    | SNode::Show { arg, .. } => vec![arg],
                    SNode::Binary { left, right, .. } => vec![left, right],
                    SNode::StringMap { object, .. } => vec![object],
                    SNode::StringTest { object, search, .. } => vec![object, search],
                    _ => Vec::new(),
                };
                let operands: Vec<_> = nodes
                    .iter()
                    .map(|operand| self.origin(operand, path))
                    .collect();
                if operands.iter().any(|operand| !operand.proven) {
                    let uncertain = operands.iter().any(|operand| {
                        operand.uncertain
                            || (form != "derived-value" && !operand.wrappers.is_empty())
                    });
                    return if uncertain {
                        SourceOrigin::uncertain()
                    } else {
                        SourceOrigin::unknown()
                    };
                }
                if !self.collecting
                    && ((form == "comparison"
                        && operands[0].wrappers != operands[1].wrappers
                        && !accept_rust_source_text_equality(
                            operation,
                            (
                                operands[0].direct_string_value(),
                                operands[1].direct_string_value(),
                                operands[0].borrowed_string_slice(),
                                operands[1].borrowed_string_slice(),
                            ),
                        ))
                        || (form == "primitive-operation"
                            && operands.iter().any(|operand| {
                                operand.wrappers.iter().any(|layer| layer == "box")
                            }))
                        || (["numeric-cast", "boolean-operation"].contains(&form.as_str())
                            && operands.iter().any(|operand| !operand.wrappers.is_empty())))
                {
                    return SourceOrigin::uncertain();
                }
                if form == "derived-value" {
                    if matches!(node.node, SNode::StringTest { .. }) {
                        return SourceOrigin::scalar_copy_value(Some(super::BOOL));
                    }
                    return SourceOrigin::text_value();
                }
                let ty = match &node.node {
                    SNode::Cast { to, .. } => Some(to.clone()),
                    _ if form == "comparison" || form == "boolean-operation" => Some(super::BOOL),
                    _ if operation == "concat" => Some(super::STRING),
                    _ => None,
                };
                let copy_evidence = form != "primitive-operation"
                    || (operation != "concat"
                        && operands
                            .iter()
                            .all(|operand| operand.copy_evidence.is_some()));
                SourceOrigin {
                    ty,
                    copy_evidence: copy_evidence.then_some(()),
                    text_source_model: (operation == "concat").then_some("owned string"),
                    ..SourceOrigin::value()
                }
            }
            _ => SourceOrigin::unknown(),
        }
    }

    pub(super) fn join(&self, origins: &[SourceOrigin]) -> SourceOrigin {
        let returning: Vec<_> = origins.iter().filter(|origin| !origin.diverging).collect();
        let Some(first) = returning.first() else {
            return SourceOrigin {
                diverging: true,
                ..SourceOrigin::value()
            };
        };
        if returning
            .iter()
            .all(|origin| self.same_origin(origin, first))
        {
            return SourceOrigin {
                fresh_array_value: if returning
                    .iter()
                    .all(|origin| origin.fresh_array_value.is_some())
                {
                    first.fresh_array_value
                } else {
                    None
                },
                ..(*first).clone()
            };
        }
        if returning
            .iter()
            .any(|origin| origin.array_source_model.is_some())
        {
            return SourceOrigin {
                array_source_model: Some("unproved array"),
                ..SourceOrigin::uncertain()
            };
        }
        if returning
            .iter()
            .all(|origin| origin.proven && origin.wrappers == first.wrappers)
        {
            let fields = first.fields.as_ref().and_then(|fields| {
                returning
                    .iter()
                    .all(|origin| {
                        origin
                            .fields
                            .as_ref()
                            .is_some_and(|candidate| candidate.len() == fields.len())
                    })
                    .then(|| {
                        (0..fields.len())
                            .map(|index| {
                                let values: Vec<_> = returning
                                    .iter()
                                    .map(|origin| {
                                        origin.fields.as_ref().expect("matched field count")[index]
                                            .clone()
                                    })
                                    .collect();
                                self.join(&values)
                            })
                            .collect()
                    })
            });
            let text_source_model = returning
                .iter()
                .all(|origin| origin.text_source_model == first.text_source_model)
                .then_some(first.text_source_model)
                .flatten();
            return SourceOrigin {
                wrappers: first.wrappers.clone(),
                copy_evidence: returning
                    .iter()
                    .all(|origin| origin.copy_evidence.is_some())
                    .then_some(()),
                fields,
                ty: text_source_model.map(|_| super::STRING),
                text_source_model,
                ..SourceOrigin::value()
            };
        }
        if returning
            .iter()
            .any(|origin| origin.uncertain || !origin.wrappers.is_empty())
        {
            SourceOrigin::uncertain()
        } else {
            SourceOrigin::unknown()
        }
    }

    pub(super) fn mark(&mut self, node: &SExpr, mut origin: SourceOrigin) {
        if !self.array_result_model.is_empty()
            && let SNode::Array { items, .. } = &node.node
        {
            origin.fresh_array_value = node.span.filter(|_| {
                origin.array_source_model == Some("owned vector")
                    && origin.wrappers.is_empty()
                    && items.iter().all(|item| {
                        !item.spread && self.origin(&item.value, &origin.path).direct_copy_value()
                    })
            });
        }
        if let Some(span) = node.span {
            self.expressions.insert(span, origin);
        }
    }

    /// Each constructor's scalar evidence was retained in its live source scope.
    pub(super) fn fresh_array_result(&self, node: &SExpr, path: &[String]) -> bool {
        let children = match &node.node {
            SNode::Let { body, .. } => self.fresh_array_result(body, path),
            SNode::If {
                then, otherwise, ..
            } => self.fresh_array_result(then, path) && self.fresh_array_result(otherwise, path),
            _ => false,
        };
        accept_rust_fresh_array_result_tail(
            node.node.kind(),
            self.origin(node, path).fresh_array_value.is_some(),
            children,
        )
    }

    pub(super) fn certify_array_result(&mut self, name: &str, path: &[String], span: Option<Span>) {
        if self.collecting {
            let full_name = path
                .iter()
                .map(String::as_str)
                .chain([name])
                .collect::<Vec<_>>()
                .join(".");
            if let Some(SourceDeclaration::Function { result, .. }) =
                self.catalog.get_mut(&full_name)
            {
                result.fresh_array_value = span;
            }
        }
    }

    pub(super) fn wrap(&mut self, node: &SExpr, path: &[String], layer: &str) {
        let mut origin = self.origin(node, path);
        origin.wrappers.insert(0, layer.to_owned());
        self.mark(node, origin);
    }

    pub(super) fn dereference(
        &mut self,
        node: &SExpr,
        path: &[String],
        span: Option<Span>,
    ) -> Result<()> {
        let mut origin = self.origin(node, path);
        if !self.collecting && origin.array_source_model.is_some() && origin.wrappers.len() == 1 {
            return Err(unsupported(
                "array dereference",
                "moving an owned vector or an unsized slice needs a separate source ownership proof",
                span,
            ));
        }
        let layer = origin.wrappers.first().map_or("", String::as_str);
        if !self.collecting && (!origin.proven || !accept_rust_immutable_indirection_layer(layer)) {
            return Err(unsupported(
                "dereference",
                "the operand must have a proven immutable reference or standard Box source type",
                span,
            ));
        }
        if !origin.wrappers.is_empty() {
            origin.wrappers.remove(0);
        }
        self.mark(node, origin);
        Ok(())
    }

    pub(super) fn bind(
        &mut self,
        name: &str,
        node: &SExpr,
        path: &[String],
        declared: Option<&SourceOrigin>,
    ) -> Result<()> {
        let actual = self.origin(node, path);
        if self.shared_array_handles && !self.collecting && matches!(actual.ty, Some(Type::Array {..}))
            && !crate::translation::rust_shared_array_handle_rules::accept_rust_shared_array_handle_transfer(
                actual.array_source_model.unwrap_or(""), actual.array_source_model.unwrap_or(""),
                f64::from(u32::try_from(actual.wrappers.len()).unwrap_or(u32::MAX)), f64::from(u32::try_from(actual.wrappers.len()).unwrap_or(u32::MAX)),
            ) {
            return Err(unsupported("Rust shared array handle source", "a binding requires an actual owned shared handle, not its vector payload", node.span));
        }
        if !self.collecting && actual.array_source_model.is_some() && actual.uncertain {
            return Err(unsupported(
                "source array join",
                "every array alternative needs the same proven source type and ownership",
                node.span,
            ));
        }
        if !self.collecting
            && actual.array_source_model == Some("owned vector")
            && actual.wrappers.is_empty()
            && !matches!(node.node, SNode::Array { .. })
            && actual.fresh_array_value.is_none()
        {
            return Err(unsupported(
                "owned vector binding",
                "moving a local vector needs an explicit consumption proof",
                node.span,
            ));
        }
        if let Some(declared) = declared {
            self.argument(&actual, declared, node.span)?;
        }
        let mut binding = declared.cloned().unwrap_or(actual);
        binding.fresh_array_value = None;
        self.scope.insert(name.to_owned(), binding);
        Ok(())
    }

    pub(super) fn argument(
        &self,
        actual: &SourceOrigin,
        expected: &SourceOrigin,
        span: Option<Span>,
    ) -> Result<()> {
        if self.collecting {
            return Ok(());
        }
        if actual.diverging {
            return Ok(());
        }
        if self.shared_array_handles && (matches!(actual.ty, Some(Type::Array {..})) || matches!(expected.ty, Some(Type::Array {..})))
            && !crate::translation::rust_shared_array_handle_rules::accept_rust_shared_array_handle_transfer(
                actual.array_source_model.unwrap_or(""), expected.array_source_model.unwrap_or(""),
                f64::from(u32::try_from(actual.wrappers.len()).unwrap_or(u32::MAX)), f64::from(u32::try_from(expected.wrappers.len()).unwrap_or(u32::MAX)),
            ) {
            return Err(unsupported("Rust shared array handle source", "call and return ownership must preserve actual shared handles without borrowed payload coercion", span));
        }
        if !actual.proven
            && !actual.uncertain
            && actual.wrappers.is_empty()
            && expected.wrappers.is_empty()
        {
            return Ok(());
        }
        if !actual.proven || !expected.proven {
            return Err(unsupported(
                "source indirection",
                "the source ownership of this value cannot be proved",
                span,
            ));
        }
        let actual_model = actual.array_source_model;
        let expected_model = expected.array_source_model;
        let actual = &actual.wrappers;
        let want = &expected.wrappers;
        let indirected = !actual.is_empty() || !want.is_empty();
        let exact = actual == want;
        let borrowed = actual.first().is_some_and(|layer| layer == "reference")
            && want.first().is_some_and(|layer| layer == "reference");
        if let (Some(actual_model), Some(expected_model)) = (actual_model, expected_model)
            && !accept_rust_source_array_coercion(actual_model, expected_model, borrowed)
        {
            return Err(unsupported(
                "source array coercion",
                "an immutable slice cannot become a reference to an owned vector",
                span,
            ));
        }
        let coerced = actual.len().checked_sub(want.len()).is_some_and(|removed| {
            removed > 0
                && actual
                    .iter()
                    .skip(1)
                    .take(removed)
                    .all(|layer| accept_rust_source_box_coercion_layer(layer, true))
                && actual
                    .iter()
                    .take(1)
                    .chain(actual.iter().skip(removed + 1))
                    .eq(want.iter())
        });
        if !accept_rust_source_indirection_argument((indirected, exact, borrowed, coerced)) {
            return Err(unsupported(
                "source indirection",
                "source ownership requires an explicit dereference or matching immutable reference or Box type",
                span,
            ));
        }
        Ok(())
    }

    pub(super) fn direct(&self, node: &SExpr, path: &[String], span: Option<Span>) -> Result<()> {
        self.argument(&self.origin(node, path), &SourceOrigin::value(), span)
    }

    pub(super) fn conversion(&mut self, node: &SExpr, path: &[String]) {
        let SNode::ToString { arg } = &node.node else {
            return;
        };
        let operand = self.origin(arg, path);
        let form = read_rust_source_expression_origin(node.node.kind(), "from");
        let result = if !self.collecting
            && form == "unboxed-conversion"
            && operand.wrappers.iter().any(|layer| layer == "box")
        {
            SourceOrigin::uncertain()
        } else if operand.proven {
            SourceOrigin::text_value()
        } else if operand.uncertain {
            SourceOrigin::uncertain()
        } else {
            SourceOrigin::unknown()
        };
        self.mark(node, result);
    }

    pub(super) fn call(
        &mut self,
        segments: &[String],
        args: &[SExpr],
        path: &[String],
        span: Option<Span>,
    ) -> Result<()> {
        if let Some(span) = span {
            self.fresh_calls.insert(span);
        }
        if self.collecting {
            return Ok(());
        }
        let expected = match self.lookup(segments, path).map(|entry| entry.declaration) {
            Some(SourceDeclaration::Function { parameters, .. }) => parameters,
            Some(SourceDeclaration::Constructor { fields, .. }) => fields,
            _ => return Ok(()),
        };
        if args.len() != expected.len() {
            return Ok(());
        }
        // References can coerce through Box; owned Box cannot become its owned
        // inner value until the source explicitly dereferences it.
        for (argument, expected) in args.iter().zip(&expected) {
            if expected.array_source_model == Some("owned vector") && expected.wrappers.is_empty() {
                return Err(unsupported(
                    "owned vector argument",
                    "passing a vector by value needs an explicit consumption proof",
                    span,
                ));
            }
            self.argument(&self.origin(argument, path), expected, span)?;
        }
        Ok(())
    }
}
