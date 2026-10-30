//! Source ownership retained through reference, constructor and alternative patterns.

use std::collections::HashMap;

use super::super::{Result, SExpr, SPattern, SPatternNode, Span, Type, unsupported};
use super::{SourceDeclaration, SourceLookup, SourceOrigin, SourceOrigins};

impl SourceOrigins {
    pub(in super::super) fn alias_literal_pattern(&mut self, pattern: &SPattern, node: &SExpr) {
        let form = match pattern.node {
            SPatternNode::StrLit { .. } => "strLit",
            SPatternNode::ByteLit { .. } => "byteLit",
            SPatternNode::TypedNumLit { .. } => "typedNumLit",
            _ => "",
        };
        if (crate::translation::rust_byte_pattern_rules::accept_rust_source_literal_pattern_alias(form)
            || crate::translation::rust_numeric_pattern_rules::accept_rust_source_numeric_pattern_alias(form))
            && let Some(span)=pattern.span {
            self.literal_pattern_aliases.entry(span).or_default().push(node.clone());
        }
    }

    pub(in super::super) fn reference_pattern(&mut self, pattern: &SPattern) {
        if let Some(span) = pattern.span {
            *self.pattern_references.entry(span).or_default() += 1;
        } else if matches!(pattern.node, SPatternNode::Wild)
            && let Some(references) = self.wildcard_references.back_mut()
        {
            *references += 1;
        }
    }

    pub(in super::super) fn wildcard_pattern(&mut self) {
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
        self.pointer_integer_argument(origin, &SourceOrigin::value(), source_span)?;
        let mut origin = origin.clone();
        let references = if matches!(pattern.node, SPatternNode::Wild) && pattern.span.is_none() {
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
                if !origin.source_lifetimes.is_empty() {
                    origin.source_lifetimes.remove(0);
                }
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
        if let Some(aliases) = pattern
            .span
            .and_then(|span| self.literal_pattern_aliases.get(&span))
            .cloned()
        {
            for alias in aliases {
                self.mark(&alias, origin.clone());
            }
        }
        if let SPatternNode::TypedNumLit { owner, .. } = &pattern.node {
            let mut contract = origin.clone();
            if let Some(expanded) = origin.expanded_source_owner.as_ref() {
                contract.source_owner = Some(expanded.clone());
            }
            let actual = contract.pointer_integer_owner();
            let error=crate::translation::rust_numeric_pattern_rules::read_rust_source_numeric_pattern_owner_error(&actual,owner,origin.ty.is_some(),self.collecting);
            if !error.is_empty() {
                return Err(unsupported("Rust numeric pattern", &error, source_span));
            }
        }
        if matches!(pattern.node, SPatternNode::StrLit { .. }) {
            let owner = origin.source_owner.as_ref();
            let owner_path = owner.map(|owner| owner.path.join("::")).unwrap_or_default();
            let count = |value: usize| {
                f64::from(u32::try_from(value).expect("source column count fits the source limit"))
            };
            let error=crate::translation::rust_string_pattern_rules::read_rust_source_string_pattern_error(
                if origin.ty==Some(super::super::STRING){"string"}else{""},origin.text_source_model.unwrap_or(""),origin.proven,count(origin.wrappers.len()),origin.wrappers.iter().all(|layer|layer=="reference"),owner.map_or("",|owner|owner.kind.as_str()),&owner_path,owner.map_or(-1.0,|owner|count(owner.arguments.len())),self.collecting);
            if !error.is_empty() {
                return Err(unsupported("Rust string pattern", &error, source_span));
            }
        }
        match &pattern.node {
            SPatternNode::Tuple { args, rest } => {
                let fields = origin.fields.as_deref().unwrap_or_default();
                let count = |value| {
                    f64::from(
                        u32::try_from(value)
                            .expect("source tuple width fits the shared array limit"),
                    )
                };
                let plan =
                    crate::translation::rust_source_render_interfaces::plan_tuple_pattern_fields(
                        count(fields.len()),
                        count(args.len()),
                        rest.map_or(-1.0, count),
                    );
                let borrowed = !origin.wrappers.is_empty()
                    && origin.wrappers.iter().all(|layer| layer == "reference");
                for (index, item) in args.iter().enumerate() {
                    let position = if plan.first() == Some(&0.0) {
                        plan.iter()
                            .skip(1)
                            .position(|actual| *actual == count(index))
                    } else {
                        None
                    };
                    let mut field = position
                        .and_then(|position| fields.get(position))
                        .cloned()
                        .unwrap_or_else(SourceOrigin::unknown);
                    if borrowed && position.is_some() {
                        field.wrappers.insert(0, "reference".to_owned());
                    }
                    self.pattern(item, &field, path, result, item.span.or(source_span))?;
                }
            }
            SPatternNode::BindOrCtor { name } | SPatternNode::Bind { name } => {
                if !self
                    .lookup(std::slice::from_ref(name), path)
                    .is_some_and(|entry| {
                        matches!(entry.declaration, SourceDeclaration::Constructor { .. })
                    })
                {
                    if result.contains_key(name) {
                        return Err(unsupported(
                            "duplicate source pattern binding",
                            "each pattern binds a source name at most once",
                            source_span,
                        ));
                    }
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

    pub(in super::super) fn cases(
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
                let mut binding = self.join(&candidates);
                binding.source_extending_temporary_bindings.clear();
                if self.owned_binding_value(&binding) {
                    self.consume_owned_binding(&binding, span)?;
                    let binding = self.register_owned_binding(binding);
                    scope.insert(name.clone(), binding);
                } else {
                    scope.insert(name.clone(), binding);
                }
            }
        }
        Ok(scope)
    }
}
