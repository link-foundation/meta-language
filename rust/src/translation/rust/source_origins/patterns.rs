//! Source ownership retained through reference, constructor and alternative patterns.

use std::collections::HashMap;

use super::super::{Result, SExpr, SPattern, SPatternNode, Span, Type, unsupported};
use super::{SourceDeclaration, SourceLookup, SourceOrigin, SourceOrigins};

impl SourceOrigins {
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
                scope.insert(name.clone(), self.join(&candidates));
            }
        }
        Ok(scope)
    }
}
