//! Read selected type positions through the normal Rust source type checker.
use super::{
    Language, Result, RustParser, SFn, SItem, SourceDeclaration as CatalogDeclaration,
    SourceOrigin, Token, TokenCursor, Type,
    fn_mut_source_declarations::{FnMutSourceDeclaration, FnMutSourceDeclarations},
    inherent::InherentOwner,
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
    pub(super) result: SourceOrigin,
}

pub(super) struct FnMutSourceHeaderRead {
    declaration: FnMutSourceDeclaration,
    callback: CheckedCallbackHeader,
    pub(super) resolved: Option<ResolvedFnMutSourceHeader>,
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
    pub(super) fn preview_fn_mut_source_header(
        &mut self,
        declaration: &FnMutSourceDeclaration,
        bindings: Vec<(String, SourceOrigin)>,
    ) -> Result<(CheckedCallbackHeader, ResolvedFnMutSourceHeader)> {
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
                    resolved: None,
                });
            let parsed = self.function(&declaration.owner);
            let read = std::mem::replace(&mut self.fn_mut_source_header_read, old_read)
                .expect("the actual declaration replay keeps its header context");
            self.fn_mut_source_type_bindings = old_bindings;
            self.cursor = old_cursor;
            parsed?;
            let header = read.resolved.ok_or_else(|| {
                unsupported(
                    "Rust source FnMut function",
                    "the actual declaration replay must consume the complete formal and result types",
                    None,
                )
            })?;
            Ok((read.callback, header))
        })();
        self.origins.current_type = old_origin;
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::translation::{
        lexer::tokenize,
        rust::{SourceOrigins, SourceProfiles},
        types::Type,
    };
    use std::sync::Arc;

    #[test]
    fn the_actual_callback_type_positions_resolve_concrete_binders() {
        let source = "fn apply<T>(value: u8, mut map: impl FnMut(&T, bool) -> T) -> T { map(&value, false) }";
        let tokens = Arc::new(tokenize(source, Language::Rust).unwrap().tokens);
        let directory = Rc::new(FnMutSourceDeclarations::new(source, Arc::clone(&tokens)).unwrap());
        let declaration = directory.capture(&[], &tokens[0]).unwrap().unwrap();
        let value = tokens.iter().position(|token| token.value == "u8").unwrap();
        let mut parser = RustParser::new(
            (*tokens).clone(),
            SourceOrigins::new(None, ""),
            SourceProfiles {
                exclusive_source: "Rust exclusive source loans 1",
                fn_mut_source: "Rust closed source FnMut specializations 1",
                ..SourceProfiles::default()
            },
            false,
            None,
        );
        let argument = parser
            .read_fn_mut_source_type_range(&directory, &(value..value + 1))
            .unwrap();
        parser.fn_mut_source_declarations = Some(Rc::clone(&directory));
        let (admitted, header) = parser
            .preview_fn_mut_source_header(&declaration, vec![("T".into(), argument)])
            .unwrap();
        let expected = Type::Fixed {
            bits: 8,
            signed: false,
        };
        assert_eq!(admitted.owner.path, ["apply"]);
        assert_eq!(admitted.parameters[0].ty.as_ref(), Some(&expected));
        assert_eq!(admitted.parameters[0].wrappers, ["reference"]);
        assert_eq!(admitted.parameters[1].ty, Some(Type::Bool));
        assert_eq!(admitted.result.ty.as_ref(), Some(&expected));
        assert_eq!(header.parameters[0].ty.as_ref(), Some(&expected));
        assert!(matches!(
            header.parameters[1].ty,
            Some(Type::Function { .. })
        ));
        assert_eq!(header.result.ty, Some(expected));
        assert_eq!(parser.cursor.index, 0);
        assert!(parser.fn_mut_source_type_bindings.is_empty());
    }
}
