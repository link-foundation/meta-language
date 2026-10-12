//! Source ownership evidence retained while Rust indirection projects to values.
//!
//! Mirrors `js/src/translation/rust-source-origins.js`. Declarations and lexical
//! scopes come from the existing parser; value typing remains in the checker.

use std::collections::{HashMap, VecDeque};

use super::indirection_rules::{
    accept_rust_immutable_indirection_layer, accept_rust_source_box_availability,
    accept_rust_source_box_coercion_layer, accept_rust_source_box_owner,
    accept_rust_source_indirection_argument, read_rust_source_expression_origin,
};
use super::product_rules::accept_rust_immutable_product_type;
use super::{Result, SExpr, SNode, SPattern, SPatternNode, Span, Type, unsupported};

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct SourceOrigin {
    pub(super) ty: Option<Type>,
    pub(super) path: Vec<String>,
    pub(super) wrappers: Vec<String>,
    pub(super) proven: bool,
    pub(super) diverging: bool,
    pub(super) uncertain: bool,
    /// Source Copy evidence does not choose the checker's numeric width.
    pub(super) copy_evidence: bool,
    pub(super) fields: Option<Vec<Self>>,
}

impl SourceOrigin {
    pub(super) const fn unknown() -> Self {
        Self {
            ty: None,
            path: Vec::new(),
            wrappers: Vec::new(),
            proven: false,
            diverging: false,
            uncertain: false,
            copy_evidence: false,
            fields: None,
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
        Self {
            ty: Some(ty),
            path: path.to_vec(),
            wrappers,
            proven: true,
            diverging: false,
            uncertain: false,
            copy_evidence,
            fields,
        }
    }

    pub(super) const fn value() -> Self {
        Self {
            proven: true,
            ..Self::unknown()
        }
    }

    pub(super) fn direct_copy_value(&self) -> bool {
        self.proven
            && self.copy_evidence
            && self.wrappers.is_empty()
            && self
                .fields
                .as_ref()
                .is_none_or(|fields| fields.iter().all(Self::direct_copy_value))
    }

    fn scalar_copy_value(ty: Option<Type>) -> Self {
        Self {
            ty,
            copy_evidence: true,
            ..Self::value()
        }
    }

    const fn uncertain() -> Self {
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
    pub(super) collecting: bool,
    pub(super) catalog: HashMap<String, SourceDeclaration>,
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
    pub(super) fn new(catalog: Option<HashMap<String, SourceDeclaration>>) -> Self {
        Self {
            collecting: catalog.is_none(),
            catalog: catalog.unwrap_or_default(),
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
        if !self.collecting && (origin.fields.is_none() || !origin.direct_copy_value()) {
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
        let origin = SourceOrigin {
            copy_evidence: fields.iter().all(SourceOrigin::direct_copy_value),
            fields: Some(fields),
            ..SourceOrigin::value()
        };
        self.require_product_origin(&origin, span)?;
        self.mark(node, origin);
        Ok(())
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

    pub(super) fn lookup(&self, segments: &[String], path: &[String]) -> Option<SourceLookup> {
        let mut names = segments.to_vec();
        let mut start = path.to_vec();
        let mut absolute = false;
        match names.first().map(String::as_str) {
            Some("crate") => {
                names.remove(0);
                start.clear();
                absolute = true;
            }
            Some("self") => {
                names.remove(0);
                absolute = true;
            }
            Some("super") => {
                while names.first().is_some_and(|name| name == "super") {
                    names.remove(0);
                    start.pop();
                }
                absolute = true;
            }
            _ => {}
        }
        for depth in (0..=start.len()).rev() {
            let full_name = start[..depth]
                .iter()
                .chain(names.iter())
                .cloned()
                .collect::<Vec<_>>()
                .join(".");
            if let Some(declaration) = self.catalog.get(&full_name) {
                return Some(SourceLookup {
                    declaration: declaration.clone(),
                    full_name,
                });
            }
            if absolute {
                break;
            }
        }
        None
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

    /// Compare retained source types by their existing declaration identities.
    pub(super) fn same_origin(&self, left: &SourceOrigin, right: &SourceOrigin) -> bool {
        if left.proven != right.proven
            || left.diverging != right.diverging
            || left.uncertain != right.uncertain
            || left.wrappers != right.wrappers
            || left.copy_evidence != right.copy_evidence
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
                SourceDeclaration::Function { result, .. }
                | SourceDeclaration::Constructor { result, .. } => return result,
                SourceDeclaration::ExternalCrate
                | SourceDeclaration::Module
                | SourceDeclaration::Data => {}
            }
        }
        if let SNode::Name { path: names } = &node.node {
            if names.len() == 2
                && self.lookup(&names[..1], path).is_none()
                && super::rust_fixed_type(&names[0]).is_some()
                && ["MIN", "MAX"].contains(&names[1].as_str())
            {
                return SourceOrigin::scalar_copy_value(super::rust_fixed_type(&names[0]));
            }
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
                    && ((form == "comparison" && operands[0].wrappers != operands[1].wrappers)
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
                    return SourceOrigin::value();
                }
                let ty = match &node.node {
                    SNode::Cast { to, .. } => Some(to.clone()),
                    _ if form == "comparison" || form == "boolean-operation" => Some(super::BOOL),
                    _ => None,
                };
                let copy_evidence = form != "primitive-operation"
                    || (operation != "concat"
                        && operands.iter().all(|operand| operand.copy_evidence));
                SourceOrigin {
                    ty,
                    copy_evidence,
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
            return (*first).clone();
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
            return SourceOrigin {
                wrappers: first.wrappers.clone(),
                copy_evidence: returning.iter().all(|origin| origin.copy_evidence),
                fields,
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

    pub(super) fn mark(&mut self, node: &SExpr, origin: SourceOrigin) {
        if let Some(span) = node.span {
            self.expressions.insert(span, origin);
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
        if let Some(declared) = declared {
            self.argument(&actual, declared, node.span)?;
        }
        self.scope
            .insert(name.to_owned(), declared.cloned().unwrap_or(actual));
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
        let actual = &actual.wrappers;
        let want = &expected.wrappers;
        let indirected = !actual.is_empty() || !want.is_empty();
        let exact = actual == want;
        let borrowed = actual.first().is_some_and(|layer| layer == "reference")
            && want.first().is_some_and(|layer| layer == "reference");
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
        if !accept_rust_source_indirection_argument(indirected, exact, borrowed, coerced) {
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
            SourceOrigin::value()
        } else if operand.uncertain {
            SourceOrigin::uncertain()
        } else {
            SourceOrigin::unknown()
        };
        self.mark(node, result);
    }

    pub(super) fn call(
        &self,
        segments: &[String],
        args: &[SExpr],
        path: &[String],
        span: Option<Span>,
    ) -> Result<()> {
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
            self.argument(&self.origin(argument, path), expected, span)?;
        }
        Ok(())
    }

    pub(super) fn reference_pattern(&mut self, pattern: &SPattern) {
        if let Some(span) = pattern.span {
            *self.pattern_references.entry(span).or_default() += 1;
        } else if matches!(pattern.node, SPatternNode::Wild) {
            if let Some(references) = self.wildcard_references.back_mut() {
                *references += 1;
            }
        }
    }

    pub(super) fn wildcard_pattern(&mut self) {
        self.wildcard_references.push_back(0);
    }

    fn pattern(
        &mut self,
        pattern: &SPattern,
        origin: &SourceOrigin,
        path: &[String],
        result: &mut HashMap<String, SourceOrigin>,
        source_span: Option<Span>,
    ) -> Result<()> {
        let mut origin = origin.clone();
        let references = if matches!(pattern.node, SPatternNode::Wild) {
            self.wildcard_references.pop_front().unwrap_or_default()
        } else {
            pattern
                .span
                .and_then(|span| self.pattern_references.get(&span))
                .copied()
                .unwrap_or_default()
        };
        for _ in 0..references {
            if origin
                .wrappers
                .first()
                .is_some_and(|layer| layer == "reference")
            {
                origin.wrappers.remove(0);
            } else {
                if !self.collecting && origin.proven {
                    return Err(unsupported(
                        "source pattern",
                        "a reference pattern requires a proven reference source type",
                        source_span,
                    ));
                }
                origin = SourceOrigin::uncertain();
            }
        }
        if !self.collecting
            && !matches!(
                pattern.node,
                SPatternNode::Wild | SPatternNode::Bind { .. } | SPatternNode::BindOrCtor { .. }
            )
            && origin.wrappers.iter().any(|layer| layer == "box")
        {
            return Err(unsupported(
                "source pattern",
                "an owned Box requires an explicit dereference before a constructor or literal pattern",
                source_span,
            ));
        }
        match &pattern.node {
            SPatternNode::BindOrCtor { name } | SPatternNode::Bind { name } => {
                if !self
                    .lookup(std::slice::from_ref(name), path)
                    .is_some_and(|entry| {
                        matches!(entry.declaration, SourceDeclaration::Constructor { .. })
                    })
                {
                    result.insert(name.clone(), origin);
                }
            }
            SPatternNode::Ctor { path: names, args } => {
                let entry = self.lookup(names, path);
                let type_entry = match &origin.ty {
                    Some(Type::Named { path: names, .. }) => self.lookup(names, &origin.path),
                    _ => None,
                };
                let borrowed = !origin.wrappers.is_empty()
                    && origin.wrappers.iter().all(|layer| layer == "reference");
                let fields = match (entry, type_entry) {
                    (
                        Some(SourceLookup {
                            declaration: SourceDeclaration::Constructor { owner, fields, .. },
                            ..
                        }),
                        Some(SourceLookup {
                            declaration: SourceDeclaration::Data,
                            full_name,
                        }),
                    ) if owner == full_name && (origin.wrappers.is_empty() || borrowed) => {
                        Some(fields)
                    }
                    _ => None,
                };
                for (index, argument) in args.iter().enumerate() {
                    let mut field = fields
                        .as_ref()
                        .and_then(|fields| fields.get(index))
                        .cloned()
                        .unwrap_or_else(SourceOrigin::unknown);
                    if borrowed && fields.is_some() {
                        field.wrappers.insert(0, "reference".to_owned());
                    }
                    self.pattern(
                        argument,
                        &field,
                        path,
                        result,
                        argument.span.or(source_span),
                    )?;
                }
            }
            _ => {}
        }
        Ok(())
    }

    pub(super) fn cases(
        &mut self,
        patterns: &[SPattern],
        scrutinee: &SExpr,
        path: &[String],
    ) -> Result<HashMap<String, SourceOrigin>> {
        let origin = self.origin(scrutinee, path);
        let maps: Vec<_> = patterns
            .iter()
            .map(|pattern| {
                let mut scope = HashMap::new();
                self.pattern(
                    pattern,
                    &origin,
                    path,
                    &mut scope,
                    pattern.span.or(scrutinee.span),
                )?;
                Ok(scope)
            })
            .collect::<Result<Vec<_>>>()?;
        let mut scope = self.scope.clone();
        if let Some(first) = maps.first() {
            let span = patterns
                .iter()
                .find_map(|pattern| pattern.span)
                .or(scrutinee.span);
            if !self.collecting
                && maps.len() > 1
                && maps.iter().any(|map| {
                    map.len() != first.len() || first.keys().any(|name| !map.contains_key(name))
                })
            {
                return Err(unsupported(
                    "or-pattern ownership",
                    "every alternative must bind the same names with the same source ownership",
                    span,
                ));
            }
            for (name, origin) in first {
                let candidates: Vec<_> = maps
                    .iter()
                    .filter_map(|map| map.get(name).cloned())
                    .collect();
                if !self.collecting
                    && maps.len() > 1
                    && (candidates.len() != maps.len()
                        || candidates.iter().any(|candidate| {
                            !candidate.proven || candidate.wrappers != origin.wrappers
                        }))
                {
                    return Err(unsupported(
                        "or-pattern ownership",
                        "every alternative must bind the same names with the same source ownership",
                        span,
                    ));
                }
                scope.insert(name.clone(), self.join(&candidates));
            }
        }
        Ok(scope)
    }
}
