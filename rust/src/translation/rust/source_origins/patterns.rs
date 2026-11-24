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
        pattern: &mut SPattern,
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
                origin.source_borrowed_place=true;
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
        self.resolve_standard_sum_source_pattern(pattern, &origin)?;
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
        match &mut pattern.node {
            SPatternNode::IntegerRange { lower, upper, .. } => {
                for endpoint in [lower, upper].into_iter().flatten() {
                    let endpoint_span = endpoint.span.or(source_span);
                    self.pattern(
                        endpoint,
                        &origin,
                        path,
                        result,
                        endpoint_span,
                    )?;
                }
            }
            SPatternNode::Alias {
                name,
                pattern: inner,
            } => {
                if result.contains_key(name) {
                    return Err(unsupported(
                        "duplicate source pattern binding",
                        "each pattern binds a source name at most once",
                        source_span,
                    ));
                }
                result.insert(name.clone(), origin.clone());
                let inner_span = inner.span.or(source_span);
                self.pattern(inner, &origin, path, result, inner_span)?;
            }
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
                for (index, item) in args.iter_mut().enumerate() {
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
                    if let Some(position)=position{field=super::super::copy_closure_lifetimes::project_value_field(&origin,field,position);}
                    if borrowed && position.is_some() {
                        field.wrappers.insert(0, "reference".to_owned());
                        field.source_value_borrow_bindings=crate::translation::rust_owned_binding_rules::merge_rust_owned_loan_bindings(&super::super::copy_closure_lifetimes::value_borrows(&field),&super::super::copy_closure_lifetimes::value_borrows(&origin));
                    }
                    let item_span = item.span.or(source_span);
                    self.pattern(item, &field, path, result, item_span)?;
                }
            }
            SPatternNode::BindOrCtor { name } | SPatternNode::Bind { name } => {
                if !self
                    .lookup_value_owner(std::slice::from_ref(name), path)
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
                let entry = self.lookup_value_owner(names, path);
                let type_entry = match &origin.ty {
                    Some(Type::Named { path: names, .. }) => self.lookup_type_owner(names, &origin.path),
                    _ => None,
                };
                let borrowed = !origin.wrappers.is_empty()
                    && origin.wrappers.iter().all(|layer| layer == "reference");
                let standard_sum=type_entry.as_ref().is_some_and(|entry|self.standard_sum_source_origins.contains_key(&entry.full_name));
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
                for (index, argument) in args.iter_mut().enumerate() {
                    let mut field = fields
                        .as_ref()
                        .and_then(|fields| fields.get(index))
                        .cloned()
                        .unwrap_or_else(SourceOrigin::unknown);
                    if standard_sum&&fields.is_some(){
                        field=self.project_standard_sum_pattern_field(&origin,&field,borrowed);
                    }else if fields.is_some() {
                        let mut direct=origin.clone();direct.wrappers.clear();
                        let nominal=self.nominal_ownership_facts(&direct);
                        if nominal.profile=="copy"||nominal.profile=="owned"{
                            field=self.project_nominal_pattern_field(&origin,&field,index,borrowed);
                        }else{
                            field=super::super::copy_closure_lifetimes::project_value_field(&origin,field,index);
                            if borrowed{field.wrappers.insert(0,"reference".to_owned());field.source_value_borrow_bindings=crate::translation::rust_owned_binding_rules::merge_rust_owned_loan_bindings(&super::super::copy_closure_lifetimes::value_borrows(&field),&super::super::copy_closure_lifetimes::value_borrows(&origin));}
                        }
                    }
                    let argument_span = argument.span.or(source_span);
                    self.pattern(
                        argument,
                        &field,
                        path,
                        result,
                        argument_span,
                    )?;
                }
            }
            _ => {}
        }
        Ok(())
    }

    pub(in super::super) fn cases(
        &mut self,
        patterns: &mut [SPattern],
        scrutinee: &SExpr,
        path: &[String],
    ) -> Result<HashMap<String, SourceOrigin>> {
        self.cases_with_guard_mode(patterns,scrutinee,path,false)
    }

    pub(in super::super) fn cases_readonly_guard(&mut self,patterns:&mut [SPattern],scrutinee:&SExpr,path:&[String])->Result<HashMap<String,SourceOrigin>>{
        // Wildcards without spans carry explicit reference-pattern layers in
        // this parser-owned queue. The committing body traversal must see the
        // same original records after this readonly preview.
        let wildcard_references=self.wildcard_references.clone();
        let result=self.cases_with_guard_mode(patterns,scrutinee,path,true);
        self.wildcard_references=wildcard_references;
        result
    }

    fn cases_with_guard_mode(&mut self,patterns:&mut [SPattern],scrutinee:&SExpr,path:&[String],deferred_guard:bool)->Result<HashMap<String,SourceOrigin>>{
        let origin = self.origin(scrutinee, path);
        let mut maps: Vec<_> = patterns
            .iter_mut()
            .map(|pattern| {
                let mut scope = HashMap::new();
                let source_span = pattern.span.or(scrutinee.span);
                self.pattern(
                    pattern,
                    &origin,
                    path,
                    &mut scope,
                    source_span,
                )?;
                Ok(scope)
            })
            .collect::<Result<Vec<_>>>()?;
        let mut scope = self.scope.clone();
        if deferred_guard{
            let aliases=super::super::standard_sum_guards::standard_sum_guard_alias_count(patterns);
            for map in &maps{let owned=map.values().filter(|origin|self.owned_binding_value(origin)).count();let error=crate::translation::rust_standard_sum_guard_rules::read_rust_standard_sum_guard_alias_transfer_error(aliases as f64,owned as f64);if !error.is_empty(){return Err(unsupported("Rust owned standard sum guard",&error,scrutinee.span));}}
        }
        let span = patterns
                .iter()
                .find_map(|pattern| pattern.span)
                .or(scrutinee.span);
        if let Some(first) = maps.first() {
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
        }
        let nominal_prepared=self.prepare_nominal_pattern_transfers(&mut maps,span,deferred_guard)?;
        if let Some(first)=maps.first(){
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
                if deferred_guard{
                    let identities=candidates.iter().map(|candidate|self.standard_sum_source_identity(candidate)).collect::<Vec<_>>();
                    let error=crate::translation::rust_standard_sum_guard_rules::read_rust_standard_sum_guard_alternative_binding_error(identities.iter().all(Option::is_some),identities.first().is_some_and(|first|identities.iter().all(|identity|identity==first)),candidates.iter().all(|candidate|super::super::standard_sum_guards::same_standard_sum_guard_lineage(origin,candidate)));
                    if !error.is_empty(){return Err(unsupported("Rust owned standard sum guard",&error,span));}
                }
                let mut binding = self.join(&candidates);
                binding.source_record_state_collecting_site=None;
                binding.source_extending_temporary_bindings.clear();
                let disposition=crate::translation::rust_standard_sum_guard_rules::read_rust_standard_sum_guard_binding_disposition(deferred_guard,self.owned_binding_value(&binding));
                if disposition=="readonly"{
                    binding.source_borrowed_place=true;
                    let binding=self.assign_copy_closure_lexical_binding(binding);
                    scope.insert(name.clone(),binding);
                } else if disposition=="transfer" {
                    if !nominal_prepared.contains(name){
                        self.validate_standard_sum_pattern_transfer(&binding,span)?;
                        self.consume_owned_binding(&binding, span)?;
                    }
                    let binding = self.register_owned_binding(binding);
                    let binding=self.assign_copy_closure_lexical_binding(binding);
                    scope.insert(name.clone(), binding);
                } else {
                    let binding=self.assign_copy_closure_lexical_binding(binding);
                    scope.insert(name.clone(), binding);
                }
            }
        }
        Ok(scope)
    }
}
