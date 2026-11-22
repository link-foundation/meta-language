//! Read selected type positions through the normal Rust source type checker.
use super::{
    Language, Result, RustParser, SFn, SItem, SourceDeclaration as CatalogDeclaration,
    SourceOrigin, Token, TokenCursor, Type,
    fn_mut_source_declarations::{FnMutSourceDeclaration, FnMutSourceDeclarations},
    inherent::InherentOwner,
    fn_mut_source_captures::SourceCapture,
    owned_record_state::Slot,
    unsupported,
};
use crate::translation::{
    source_fn_mut_specializations::SourceDeclaration, source_loop_folds::SourceFoldSpan,
    source_signatures::SourceOwner,
};
use std::{
    collections::{BTreeMap, HashSet},
    ops::Range,
    rc::Rc,
};

pub(super) struct CheckedCallbackHeader {
    pub(super) owner: SourceDeclaration,
    pub(super) parameters: Vec<SourceOrigin>,
    pub(super) result: SourceOrigin,
}

pub(super) struct ResolvedFnMutSourceHeader {
    pub(super) parameters: Vec<SourceOrigin>,
    pub(super) exclusive_parameters: Vec<usize>,
    pub(super) result: SourceOrigin,
}

pub(super) struct FnMutSourceHeaderRead {
    declaration: FnMutSourceDeclaration,
    pub(super) callback: CheckedCallbackHeader,
    pub(super) body_replay: bool,
    pub(super) callback_binding: Option<String>,
    pub(super) captures: Vec<SourceCapture>,
    pub(super) state_parameters: Vec<Slot>,
    pub(super) resolved: Option<ResolvedFnMutSourceHeader>,
}

pub(super) struct ReplayedFnMutSourceHeader {
    pub(super) function: SFn,
    pub(super) callback: CheckedCallbackHeader,
    pub(super) resolved: ResolvedFnMutSourceHeader,
    pub(super) state_parameters: Vec<Slot>,
}

pub(super) struct FnMutSourceInstance {
    pub(super) owner: Vec<String>,
    pub(super) declaration: Range<usize>,
    pub(super) function: SFn,
}

fn append_instances(
    items: &mut Vec<SItem>,
    path: &mut Vec<String>,
    by_owner: &mut BTreeMap<Vec<String>, Vec<SItem>>,
) {
    for item in items.iter_mut() {
        if let SItem::Module(module) = item {
            path.push(module.name.clone());
            append_instances(&mut module.items, path, by_owner);
            path.pop();
        }
    }
    if let Some(instances) = by_owner.remove(path) {
        items.extend(instances);
    }
}

impl RustParser {
    pub(super) fn select_fn_mut_source_template(
        &self,
        segments: &[String],
        path: &[String],
    ) -> Option<FnMutSourceDeclaration> {
        if segments.len() == 1 && self.origins.scope.contains_key(&segments[0]) {
            return None;
        }
        let resolved = self.resolve(segments, path);
        let owner = self.origins.lookup_value_owner(&resolved, path)?;
        match owner.declaration {
            CatalogDeclaration::FnMutTemplate(template) => Some(*template),
            _ => None,
        }
    }

    pub(super) fn read_fn_mut_source_type_arguments(
        &mut self,
        declaration: &FnMutSourceDeclaration,
    ) -> Result<Vec<(String, SourceOrigin)>> {
        let directory = Rc::clone(self.fn_mut_source_declarations.as_ref().ok_or_else(|| {
            unsupported(
                "Rust source FnMut function",
                "the original declaration inventory is required",
                None,
            )
        })?);
        let owner = directory.checked_owner(declaration);
        if self.cursor.is("::") && self.cursor.is_at("<", 1) {
            self.cursor.advance();
            self.cursor.advance();
            let mut arguments = Vec::new();
            for (index, binder) in owner.generics.into_iter().enumerate() {
                if index > 0 {
                    self.cursor
                        .expect(",", Some("source FnMut type arguments"))?;
                }
                self.parse_type()?;
                arguments.push((binder.name, self.origins.current_type.clone()));
            }
            self.cursor.eat(",");
            self.split_type_closing_token();
            self.cursor
                .expect(">", Some("source FnMut type arguments"))?;
            return Ok(arguments);
        }
        if owner.generics.is_empty() {
            return Ok(Vec::new());
        }
        if let [binder] = owner.generics.as_slice()
            && let Some(range) = &declaration.result
            && let Some(expected) = self.standard_sum_source.expected_origin.as_ref()
            && let Some(expected) = self.origins.standard_sum_source_origin(expected)
            && let Some(sum) = expected.standard_sum_source
            && sum.owner == "std::result::Result"
            && let [_, error] = sum.arguments.as_slice()
        {
            let tokens = directory.read_tokens(range);
            let source_result = &tokens[..tokens.len() - 1];
            if source_result.iter().any(|token| token.value == "Result")
                && let [.., comma, argument, close] = source_result
                && comma.value == ","
                && argument.value == binder.name
                && close.value == ">"
            {
                return Ok(vec![(binder.name.clone(), error.clone())]);
            }
        }
        Err(unsupported(
            "Rust source FnMut type arguments",
            "concrete source type arguments need actual turbofish positions or the selected standard Result error context",
            Some(super::Span::new(owner.range.start, owner.range.end)),
        ))
    }

    pub(super) fn preview_fn_mut_source_header(
        &mut self,
        declaration: &FnMutSourceDeclaration,
        bindings: Vec<(String, SourceOrigin)>,
    ) -> Result<ReplayedFnMutSourceHeader> {
        self.replay_fn_mut_source_declaration(declaration, bindings, false, Vec::new())
    }

    pub(super) fn replay_fn_mut_source_declaration(
        &mut self,
        declaration: &FnMutSourceDeclaration,
        bindings: Vec<(String, SourceOrigin)>,
        body_replay: bool,
        captures: Vec<SourceCapture>,
    ) -> Result<ReplayedFnMutSourceHeader> {
        let directory = Rc::clone(self.fn_mut_source_declarations.as_ref().ok_or_else(|| {
            unsupported(
                "Rust source FnMut function",
                "the original declaration inventory is required",
                None,
            )
        })?);
        let mut module = declaration.owner.clone();
        let mut rooted = vec!["crate".to_owned()];
        rooted.extend(declaration.owner.iter().cloned());
        let inherent = if declaration.owner.is_empty() {
            None
        } else {
            self.origins
                .lookup_type_owner(&rooted, &[])
                .and_then(|owner| {
                    if matches!(owner.declaration, CatalogDeclaration::Data) {
                        module.pop();
                        Some(InherentOwner {
                            path: declaration.owner.clone(),
                            module_path: module.clone(),
                        })
                    } else {
                        None
                    }
                })
        };
        let old_inherent = std::mem::replace(&mut self.inherent_owner, inherent);
        let old_module = std::mem::replace(&mut self.module_path, module);
        let old_origin = self.origins.current_type.clone();
        let old_scope = std::mem::take(&mut self.origins.scope);
        let old_ownership = self.origins.snapshot_owned_bindings();
        let old_temporaries = self.origins.owned_temporary_frame();
        let key = declaration
            .owner
            .iter()
            .chain(std::iter::once(&declaration.name))
            .cloned()
            .collect::<Vec<_>>()
            .join(".");
        let old_method = self.inherent_methods.get(&key).cloned();
        let result = (|| {
            let callback =
                self.read_fn_mut_source_callback_header(&directory, declaration, bindings.clone())?;
            let cursor = TokenCursor::new(
                directory.read_tokens(&declaration.declaration),
                Language::Rust,
            );
            let old_cursor = std::mem::replace(&mut self.cursor, cursor);
            let old_bindings = std::mem::replace(&mut self.fn_mut_source_type_bindings, bindings);
            let old_read = self
                .fn_mut_source_header_read
                .replace(FnMutSourceHeaderRead {
                    declaration: declaration.clone(),
                    callback,
                    body_replay,
                    callback_binding: None,
                    captures,
                    state_parameters: Vec::new(),
                    resolved: None,
                });
            let parsed = self.function(&declaration.owner);
            let complete = self.cursor.at_end();
            let read = std::mem::replace(&mut self.fn_mut_source_header_read, old_read)
                .expect("the actual declaration replay keeps its header context");
            self.fn_mut_source_type_bindings = old_bindings;
            self.cursor = old_cursor;
            let function = parsed?;
            if !complete {
                return Err(unsupported(
                    "Rust source FnMut function",
                    "the actual declaration replay must finish its complete original interval",
                    function.span,
                ));
            }
            let header = read.resolved.ok_or_else(|| {
                unsupported(
                    "Rust source FnMut function",
                    "the actual declaration replay must consume the complete formal and result types",
                    None,
                )
            })?;
            Ok(ReplayedFnMutSourceHeader {
                function,
                callback: read.callback,
                resolved: header,
                state_parameters: read.state_parameters,
            })
        })();
        self.origins.current_type = old_origin;
        self.origins.scope = old_scope;
        self.origins.restore_owned_bindings(old_ownership);
        self.origins.restore_owned_temporary_frame(old_temporaries);
        self.module_path = old_module;
        self.inherent_owner = old_inherent;
        if let Some(method) = old_method {
            self.inherent_methods.insert(key, method);
        } else {
            self.inherent_methods.remove(&key);
        }
        result
    }

    pub(super) fn consume_fn_mut_source_generics(
        &mut self,
        path: &[String],
        name: &str,
        start: &Token,
    ) -> Result<bool> {
        let Some(read) = &self.fn_mut_source_header_read else {
            return Ok(false);
        };
        let owner = &read.callback.owner;
        if read.declaration.owner != path
            || read.declaration.name != name
            || owner.range.start != start.start
        {
            return Err(unsupported(
                "Rust source FnMut function",
                "specialization reparses its actual original generic function owner",
                Some(start.span()),
            ));
        }
        if !owner.generics.is_empty() {
            self.cursor.expect("<", Some("source generic binders"))?;
            for (at, binder) in owner.generics.iter().enumerate() {
                if at > 0 {
                    self.cursor.expect(",", Some("source generic binders"))?;
                }
                let token = self.cursor.identifier(Some("source generic binder"))?;
                if token.value != binder.name
                    || token.start != binder.start
                    || token.end != binder.end
                {
                    return Err(unsupported(
                        "Rust source FnMut function",
                        "generic binders retain their actual original source identity",
                        Some(token.span()),
                    ));
                }
            }
            self.cursor.eat(",");
            self.cursor.expect(">", Some("source generic binders"))?;
        }
        Ok(true)
    }

    pub(super) fn read_fn_mut_source_formal_type(
        &mut self,
        path: &[String],
        name: &str,
        index: usize,
    ) -> Result<Option<Type>> {
        let Some(read) = &self.fn_mut_source_header_read else {
            return Ok(None);
        };
        let callback = &read.callback;
        let original = &callback.owner.callback;
        if index != original.index {
            return Ok(None);
        }
        let token = self.cursor.peek();
        if original.name != name || token.start != original.type_range.start {
            return Err(unsupported(
                "Rust source FnMut callback",
                "the source callback must belong to its actual formal position",
                Some(token.span()),
            ));
        }
        let mut parameters = callback.parameters.clone();
        let mut result = callback.result.clone();
        let states = read.captures.iter().map(|state| state.alias.origin.clone()).collect::<Vec<_>>();
        let source_count = parameters.len();
        parameters.extend(states.iter().cloned());
        let mut owner_arguments = Vec::new();
        for origin in parameters.iter_mut().chain(std::iter::once(&mut result)) {
            let owner = origin
                .expanded_source_owner
                .as_ref()
                .or(origin.source_owner.as_ref())
                .ok_or_else(|| {
                    unsupported(
                        "Rust source FnMut callback",
                        "the actual callable interface requires every source type owner",
                        Some(token.span()),
                    )
                })?;
            owner_arguments.push(owner.clone());
            origin.ty = origin
                .ty
                .as_ref()
                .map(|ty| self.record_source_type(ty, &origin.path));
        }
        let result = if read.body_replay {
            super::fn_mut_source_invocations::fn_mut_result_packet(&result, &parameters[source_count..], path)?
        } else {
            result
        };
        if read.body_replay {
            owner_arguments[parameters.len()] = result
                .source_owner
                .clone()
                .expect("the expanded result retains its actual tuple owner");
        }
        let ty = Type::Function {
            params: parameters
                .iter()
                .map(|parameter| {
                    parameter
                        .ty
                        .clone()
                        .expect("the actual callback type reader returns a type")
                })
                .collect(),
            ret: Box::new(
                result
                    .ty
                    .clone()
                    .expect("the actual callback type reader returns a type"),
            ),
        };
        let mut origin = SourceOrigin::with_type(ty.clone(), path, Vec::new());
        origin.source_owner = Some(SourceOwner {
            kind: "function".to_owned(),
            path: Vec::new(),
            arguments: owner_arguments,
        });
        origin.copy_evidence = None;
        origin.source_fn_mut_parameter = true;
        origin.callable_parameters = Some(parameters);
        origin.callable_result = Some(Box::new(result));
        while !self.cursor.at_end() && self.cursor.peek().end <= original.type_range.end {
            self.cursor.advance();
        }
        if self.cursor.tokens[self.cursor.index - 1].end != original.type_range.end {
            return Err(unsupported(
                "Rust source FnMut callback",
                "the admitted source callback must consume its complete original type interval",
                Some(self.cursor.peek().span()),
            ));
        }
        self.origins.current_type = origin;
        Ok(Some(ty))
    }

    pub(super) fn read_fn_mut_source_exclusive_type(&mut self) -> Result<Option<Type>> {
        if self.fn_mut_source_header_read.is_none()
            || !self.cursor.is("&")
            || !self.cursor.is_at("mut", 1)
        {
            return Ok(None);
        }
        let start = self.cursor.advance();
        self.cursor.advance();
        let referent = self.parse_type()?;
        let origin = &mut self.origins.current_type;
        if !origin.proven
            || !origin.wrappers.is_empty()
            || matches!(&referent, Type::Function { .. })
        {
            return Err(unsupported(
                "Rust source FnMut function",
                "an actual exclusive formal needs one direct noncallable referent",
                Some(start.span()),
            ));
        }
        origin.wrappers.push("reference".to_owned());
        origin.source_lifetimes.push(None);
        origin.copy_evidence = None;
        Ok(Some(referent))
    }

    pub(super) fn finish_fn_mut_source_items(&mut self, items: &mut Vec<SItem>) -> Result<()> {
        let Some(directory) = &self.fn_mut_source_declarations else {
            return Ok(());
        };
        if !self.origins.collecting && !self.header_discovery {
            let admitted: HashSet<_> = self
                .fn_mut_source_instances
                .iter()
                .map(|instance| instance.declaration.start)
                .collect();
            let uninstantiated = self
                .origins
                .value_catalog
                .iter()
                .filter_map(|(name, row)| {
                    let CatalogDeclaration::FnMutTemplate(template) = row else {
                        return None;
                    };
                    (!admitted.contains(&template.declaration.start)).then_some((name, template))
                })
                .min_by_key(|(_, template)| template.declaration.start);
            if let Some((name, template)) = uninstantiated {
                let source = directory.position(&template.declaration);
                return Err(unsupported(
                    "Rust source FnMut function",
                    &format!(
                        "an uninstantiated generic source declaration still needs complete actual body admission: {name}"
                    ),
                    Some(super::Span::new(source.start, source.end)),
                ));
            }
        }
        let mut by_owner: BTreeMap<_, Vec<_>> = BTreeMap::new();
        for instance in self.fn_mut_source_instances.drain(..) {
            by_owner
                .entry(instance.owner)
                .or_default()
                .push(SItem::Fn(instance.function));
        }
        append_instances(items, &mut Vec::new(), &mut by_owner);
        if !by_owner.is_empty() {
            return Err(unsupported(
                "Rust source FnMut function",
                "each specialized body needs its actual parsed owner module",
                None,
            ));
        }
        Ok(())
    }

    pub(super) fn capture_fn_mut_source_item(&mut self, path: &[String]) -> Result<bool> {
        let Some(directory) = &self.fn_mut_source_declarations else {
            return Ok(false);
        };
        let start = self.cursor.peek();
        let Some(declaration) = directory.capture(path, start)? else {
            return Ok(false);
        };
        if self.const_function_keyword.is_some() {
            return Err(unsupported(
                "Rust source FnMut function",
                "const generic FnMut calls need their independent const trait/effect interpretation",
                Some(start.span()),
            ));
        }
        if declaration.callback.parameter + 1 != declaration.formal_count {
            return Err(unsupported(
                "Rust source FnMut function",
                "this closed callback profile requires the actual callback as the final original source operand",
                Some(start.span()),
            ));
        }
        let original = directory.position(&declaration.declaration);
        let close = self.cursor.tokens[self.cursor.index..]
            .partition_point(|token| token.end < original.end)
            + self.cursor.index;
        if !self
            .cursor
            .tokens
            .get(close)
            .is_some_and(|token| token.value == "}" && token.end == original.end)
        {
            return Err(unsupported(
                "Rust source FnMut function",
                "the original generic body must remain in the actual token stream",
                Some(super::Span::new(original.start, original.end)),
            ));
        }
        let name = declaration.name.clone();
        self.origins.declaration(
            &name,
            path,
            CatalogDeclaration::FnMutTemplate(Box::new(declaration)),
        )?;
        self.cursor.index = close + 1;
        Ok(true)
    }

    pub(super) fn resolve_fn_mut_source_type_parameter(
        &self,
        segments: &[String],
        token: &Token,
    ) -> Result<Option<SourceOrigin>> {
        let [name] = segments else {
            return Ok(None);
        };
        let Some((_, origin)) = self
            .fn_mut_source_type_bindings
            .iter()
            .find(|(source, _)| source == name)
        else {
            return Ok(None);
        };
        if self.cursor.is("<") {
            return Err(unsupported(
                "Rust source FnMut type parameter",
                "the actual source binder has one concrete type, without its own arguments",
                Some(token.span()),
            ));
        }
        let mut origin = origin.clone();
        origin
            .expanded_source_owner
            .clone_from(&origin.source_owner);
        Ok(Some(origin))
    }

    pub(super) fn read_fn_mut_source_callback_header(
        &mut self,
        directory: &FnMutSourceDeclarations,
        declaration: &FnMutSourceDeclaration,
        bindings: Vec<(String, SourceOrigin)>,
    ) -> Result<CheckedCallbackHeader> {
        let owner = directory.checked_owner(declaration);
        if owner.generics.len() != bindings.len()
            || owner
                .generics
                .iter()
                .zip(&bindings)
                .any(|(binder, (name, origin))| {
                    binder.name != *name
                        || origin.ty.is_none()
                        || origin.source_owner.is_none()
                        || !origin.wrappers.is_empty()
                })
        {
            return Err(unsupported(
                "Rust source FnMut type arguments",
                "every actual generic binder needs its direct concrete source type in declaration order",
                None,
            ));
        }
        let previous_bindings = std::mem::replace(&mut self.fn_mut_source_type_bindings, bindings);
        let module = self
            .inherent_owner
            .as_ref()
            .filter(|owner| owner.path == declaration.owner)
            .map_or_else(
                || declaration.owner.clone(),
                |owner| owner.module_path.clone(),
            );
        let previous_module = std::mem::replace(&mut self.module_path, module);
        let previous_origin =
            std::mem::replace(&mut self.origins.current_type, SourceOrigin::unknown());
        let previous_lifetime_header = std::mem::replace(&mut self.lifetime_header, true);
        let result = (|| {
            let parameters = declaration
                .callback
                .parameters
                .iter()
                .map(|range| self.read_fn_mut_source_type_range(directory, range))
                .collect::<Result<Vec<_>>>()?;
            let result =
                self.read_fn_mut_source_type_range(directory, &declaration.callback.result)?;
            for (origin, position) in parameters.iter().zip(&owner.callback.parameter_ranges) {
                if origin.ty.as_ref().is_some_and(|ty| ty.kind() == "function")
                    || origin.wrappers.len() > 1
                    || origin.wrappers.iter().any(|layer| layer != "reference")
                {
                    return Err(unsupported(
                        "Rust source FnMut callback",
                        "callback inputs need direct values or immutable whole references",
                        Some(super::Span::new(position.start, position.end)),
                    ));
                }
            }
            if !result.wrappers.is_empty() {
                return Err(unsupported(
                    "Rust source FnMut callback",
                    "the closed callback transfers an owned result",
                    Some(super::Span::new(
                        owner.callback.result_range.start,
                        owner.callback.result_range.end,
                    )),
                ));
            }
            Ok(CheckedCallbackHeader {
                owner,
                parameters,
                result,
            })
        })();
        self.lifetime_header = previous_lifetime_header;
        self.origins.current_type = previous_origin;
        self.module_path = previous_module;
        self.fn_mut_source_type_bindings = previous_bindings;
        result
    }

    fn read_fn_mut_source_type_range(
        &mut self,
        directory: &FnMutSourceDeclarations,
        range: &Range<usize>,
    ) -> Result<SourceOrigin> {
        let original = TokenCursor::new(directory.read_tokens(range), Language::Rust);
        let previous_cursor = std::mem::replace(&mut self.cursor, original);
        let result = self.parse_type().and_then(|_| {
            if !self.cursor.at_end() {
                let SourceFoldSpan { start, end } = directory.position(range);
                return Err(unsupported(
                    "Rust source FnMut type argument",
                    "the normal type parser must consume the complete original type position",
                    Some(super::Span::new(start, end)),
                ));
            }
            Ok(self.origins.current_type.clone())
        });
        self.cursor = previous_cursor;
        result
    }
}

include!("../../../tests/internal/translation/rust/fn_mut_source_headers.rs");
