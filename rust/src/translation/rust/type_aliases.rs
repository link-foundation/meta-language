//! Source token, type and origin transport around shared alias decisions.
use super::{Result, RustParser, SourceDeclaration, SourceOrigin, Type, unsupported};
use crate::translation::rust_type_alias_rules as rules;
use crate::translation::rust_generic_type_alias_rules as generic_rules;
use std::collections::{BTreeMap, HashSet};
use std::rc::Rc;

#[derive(Clone)]
pub(super) struct AliasDeclaration {
    pub(super) key: String,
    pub(super) keyword: super::Span,
    pub(super) first: usize,
    pub(super) end: usize,
    pub(super) path: Vec<String>,
    pub(super) parameters: Vec<String>,
    pub(super) parameter_bindings: Vec<String>,
    pub(super) default_spans: Vec<Option<(usize,usize)>>,
}
#[derive(Default)]
pub(super) struct SourceTypeAliases {
    pub(super) declarations: BTreeMap<String, AliasDeclaration>,
    cache: BTreeMap<String, SourceOrigin>,
    pub(super) templates: BTreeMap<String, Rc<super::generic_type_aliases::GenericAliasTemplate>>,
    pub(super) defaults: BTreeMap<String,Vec<Option<Rc<super::generic_type_aliases::GenericAliasTemplate>>>>,
    pub(super) stack: HashSet<String>,
    pub(super) expansions: usize,
    pub(super) nodes: usize,
    checked: bool,
}
fn children(ty: &Type) -> Vec<&Type> {
    match ty {
        Type::Array { element, .. } => vec![element],
        Type::Tuple { elements } => elements.iter().collect(),
        Type::Function { params, ret } => {
            params.iter().chain(std::iter::once(ret.as_ref())).collect()
        }
        _ => Vec::new(),
    }
}
impl RustParser {
    pub(super) fn alias_error(
        &self,
        message: &str,
        row: Option<&AliasDeclaration>,
    ) -> super::TranslationError {
        let location = row.map_or_else(|| self.cursor.peek().span(), |row| row.keyword);
        unsupported("Rust type alias", message, Some(location))
    }
    pub(super) fn initialize_type_aliases(&mut self) -> Result<()> {
        if let Some(error)=self.origins.standard_sum_metadata_budget.error.as_deref(){return Err(unsupported("Rust standard sum source",error,None));}
        self.source_type_aliases = SourceTypeAliases::default();
        if !self.ordinary_completion_context() {
            return Ok(());
        }
        if !self.cursor.tokens.iter().any(|token| {
            token.kind == super::TokenKind::Identifier
                && token.value == "type"
                && token.raw == "type"
        }) {
            return Ok(());
        }
        let kinds = self
            .cursor
            .tokens
            .iter()
            .map(|token| {
                match token.kind {
                    super::TokenKind::Identifier
                        if matches!(token.value.as_str(), "mod" | "type")
                            && token.raw == token.value =>
                    {
                        "keyword"
                    }
                    super::TokenKind::Identifier => "identifier",
                    super::TokenKind::Punct => "punct",
                    super::TokenKind::Eof => "eof",
                    _ => "atom",
                }
                .to_owned()
            })
            .collect::<Vec<_>>();
        let values = self
            .cursor
            .tokens
            .iter()
            .map(|token| token.value.clone())
            .collect::<Vec<_>>();
        let raws = self.cursor.tokens.iter().map(|token| token.raw.clone()).collect::<Vec<_>>();
        let plan = generic_rules::read_rust_generic_type_alias_declaration_plan(&kinds, &values, &raws);
        if plan.first().map(String::as_str) != Some("aliases") {
            return Err(self.alias_error(&plan[1], None));
        }
        let mut position = 1;
        while position < plan.len() {
            let fields = &plan[position..];
            let key = fields[0].clone();
            let count = fields[5].parse::<usize>().expect("shared alias parameter count");
            let parameters = fields[6..6 + count].to_vec();
            let directory=if count>0 {generic_rules::read_rust_generic_type_alias_parameter_directory(&kinds,&values,&raws,(fields[1].parse::<usize>().unwrap()+2) as f64)}else{Vec::new()};
            let default_spans=(0..count).map(|index|{
                if directory[5+index*4]=="-1" {None}else{
                    let start=directory[5+index*4].parse::<usize>().unwrap();let end=directory[6+index*4].parse::<usize>().unwrap();let offset=directory[7+index*4].parse::<usize>().unwrap();
                    Some((self.cursor.tokens[start].start,self.cursor.tokens[end].start+offset))
                }
            }).collect();
            let keyword = self.cursor.tokens[fields[1].parse::<usize>().unwrap()].span();
            let parameter_bindings = parameters.iter().enumerate().map(|(index, _)|format!("{key}/parameter/{}/{index}", keyword.start)).collect();
            let mut path = key.split('.').map(str::to_owned).collect::<Vec<_>>();
            path.pop();
            self.source_type_aliases.declarations.insert(
                key.clone(),
                AliasDeclaration {
                    key,
                    keyword,
                    first: self.cursor.tokens[fields[2].parse::<usize>().unwrap()].start,
                    end: self.cursor.tokens[fields[3].parse::<usize>().unwrap()].start,
                    path,
                    parameters,
                    parameter_bindings,
                    default_spans,
                },
            );
            position += 6 + count;
        }
        Ok(())
    }
    fn alias_data_owner(&self, segments: &[String], path: &[String]) -> Option<String> {
        for key in rules::read_rust_type_alias_lookup_keys(segments, path) {
            let kind = self.alias_namespace_kind(&key);
            if kind == "data" {
                return Some(key);
            }
            if rules::read_rust_type_alias_namespace_disposition(false, kind) == "block" {
                return None;
            }
        }
        None
    }
    pub(super) fn alias_namespace_kind(&self, key: &str) -> &'static str {
        match self.origins.namespace_entry(key,"type") {
            Some(SourceDeclaration::Data) => "data",
            Some(SourceDeclaration::Module) => "module",
            Some(SourceDeclaration::ExternalCrate) => "externalCrate",
            _ => "",
        }
    }
    fn qualify_alias_type(&self, ty: &Type, path: &[String]) -> Type {
        match ty {
            Type::Named { path: names, span } => {
                let resolved = self.resolve(names, path);
                let target = self
                    .alias_data_owner(&resolved, path)
                    .map_or(resolved, |name| {
                        std::iter::once("crate".to_owned())
                            .chain(name.split('.').map(str::to_owned))
                            .collect()
                    });
                Type::Named {
                    path: target,
                    span: *span,
                }
            }
            Type::Array { element, reference } => Type::Array {
                element: Box::new(self.qualify_alias_type(element, path)),
                reference: *reference,
            },
            Type::Tuple { elements } => Type::Tuple {
                elements: elements
                    .iter()
                    .map(|ty| self.qualify_alias_type(ty, path))
                    .collect(),
            },
            Type::Function { params, ret } => Type::Function {
                params: params
                    .iter()
                    .map(|ty| self.qualify_alias_type(ty, path))
                    .collect(),
                ret: Box::new(self.qualify_alias_type(ret, path)),
            },
            _ => ty.clone(),
        }
    }
    fn qualify_alias_origin(&self, origin: &SourceOrigin) -> SourceOrigin {
        let mut result = origin.clone();
        result.ty = origin
            .ty
            .as_ref()
            .map(|ty| self.qualify_alias_type(ty, &origin.path));
        result.fields = origin.fields.as_ref().map(|fields| {
            fields
                .iter()
                .map(|origin| self.qualify_alias_origin(origin))
                .collect()
        });
        result.callable_parameters = origin.callable_parameters.as_ref().map(|parameters| {
            parameters
                .iter()
                .map(|origin| self.qualify_alias_origin(origin))
                .collect()
        });
        result.callable_result = origin
            .callable_result
            .as_deref()
            .map(|origin| Box::new(self.qualify_alias_origin(origin)));
        if let Some(sum) = result.standard_sum_source.as_mut() {
            if let Some(original) = origin.standard_sum_source.as_deref() {
                sum.arguments = original.arguments.iter().map(|argument|self.qualify_alias_origin(argument)).collect();
            }
        }
        result
    }
    fn measure_alias_origin(&self, origin: &SourceOrigin, row: &AliasDeclaration) -> Result<usize> {
        let mut origins = vec![origin];
        let mut types = Vec::new();
        let mut nodes = 0;
        let mut position = 0;
        while position < origins.len() {
            let origin = origins[position];
            position += 1;
            nodes += 1;
            if let Some(ty) = &origin.ty {
                types.push(ty);
            }
            if let Some(fields) = &origin.fields {
                origins.extend(fields);
            }
            if let Some(parameters) = &origin.callable_parameters {
                origins.extend(parameters);
            }
            if let Some(result) = &origin.callable_result {
                origins.push(result.as_ref());
            }
            if let Some(sum) = origin.standard_sum_source.as_deref() {
                origins.extend(sum.arguments.iter());
            }
            let message = rules::read_rust_type_alias_clone_error(
                self.source_type_aliases.nodes as f64,
                nodes as f64,
            );
            if !message.is_empty() {
                return Err(self.alias_error(&message, Some(row)));
            }
        }
        let mut position = 0;
        while position < types.len() {
            let ty = types[position];
            position += 1;
            nodes += 1;
            types.extend(children(ty));
            let message = rules::read_rust_type_alias_clone_error(
                self.source_type_aliases.nodes as f64,
                nodes as f64,
            );
            if !message.is_empty() {
                return Err(self.alias_error(&message, Some(row)));
            }
        }
        Ok(nodes)
    }
    fn copy_cached_alias_origin(&mut self, row: &AliasDeclaration) -> Result<SourceOrigin> {
        let needed =
            self.measure_alias_origin(self.source_type_aliases.cache.get(&row.key).unwrap(), row)?;
        self.source_type_aliases.nodes += needed;
        Ok(self.qualify_alias_origin(self.source_type_aliases.cache.get(&row.key).unwrap()))
    }
    pub(super) fn copy_type_alias_origin(&mut self, origin: &SourceOrigin, row: &AliasDeclaration) -> Result<SourceOrigin> {
        let needed = self.measure_alias_origin(origin, row)?;
        self.source_type_aliases.nodes += needed;
        Ok(self.qualify_alias_origin(origin))
    }
    pub(super) fn default_type_alias_access_origin(&mut self,row:&AliasDeclaration)->Result<Option<SourceOrigin>>{
        let defaults=row.default_spans.iter().map(Option::is_some).collect::<Vec<_>>();
        let plan=generic_rules::read_rust_generic_type_alias_default_argument_plan(&defaults,0.0,false);
        if plan.first().map(String::as_str)!=Some("arguments"){return Ok(None);}
        if self.source_type_aliases.cache.contains_key(&row.key){return self.copy_cached_alias_origin(row).map(Some);}
        let mut origin=self.read_generic_type_alias_default_value(row)?;
        origin.expanded_source_owner=origin.source_owner.clone();
        let cached=self.copy_type_alias_origin(&origin,row)?;self.source_type_aliases.cache.insert(row.key.clone(),cached);Ok(Some(origin))
    }
    pub(super) fn copy_current_type_alias_origin(&mut self, row: &AliasDeclaration) -> Result<SourceOrigin> {
        let needed = self.measure_alias_origin(&self.origins.current_type, row)?;
        self.source_type_aliases.nodes += needed;
        Ok(self.qualify_alias_origin(&self.origins.current_type))
    }
    fn expand_type_alias(&mut self, row: &AliasDeclaration) -> Result<SourceOrigin> {
        if self.source_type_aliases.cache.contains_key(&row.key) {
            return self.copy_cached_alias_origin(row);
        }
        let message = rules::read_rust_type_alias_expansion_error(
            self.source_type_aliases.stack.len() as f64,
            self.source_type_aliases.expansions as f64,
            self.source_type_aliases.stack.contains(&row.key),
        );
        if !message.is_empty() {
            return Err(self.alias_error(&message, Some(row)));
        }
        let previous_token_start = self.cursor.peek().start;
        let previous_path = std::mem::replace(&mut self.module_path, row.path.clone());
        let previous_lifetimes = std::mem::take(&mut self.lifetime_bindings);
        let previous_header = std::mem::replace(&mut self.lifetime_header, true);
        let previous_inherent = self.inherent_owner.take();
        self.cursor.index = self
            .cursor
            .tokens
            .iter()
            .position(|token| token.start == row.first)
            .expect("original type alias source start");
        self.source_type_aliases.expansions += 1;
        self.source_type_aliases.stack.insert(row.key.clone());
        let result = (|| -> Result<SourceOrigin> {
            self.parse_type()?;
            if self.cursor.peek().start != row.end || !self.cursor.is(";") {
                return Err(self.alias_error(
                    "the complete alias definition needs one admitted source type",
                    Some(row),
                ));
            }
            if self
                .origins
                .current_type
                .source_lifetimes
                .iter()
                .any(Option::is_none)
            {
                return Err(self.alias_error(
                    "reference aliases require explicit source lifetimes",
                    Some(row),
                ));
            }
            let needed = self.measure_alias_origin(&self.origins.current_type, row)?;
            self.source_type_aliases.nodes += needed;
            let origin = self.qualify_alias_origin(&self.origins.current_type);
            self.source_type_aliases
                .cache
                .insert(row.key.clone(), origin);
            self.copy_cached_alias_origin(row)
        })();
        self.cursor.index = self.cursor.tokens.iter().position(|token|token.start == previous_token_start).expect("original alias caller token");
        self.module_path = previous_path;
        self.lifetime_bindings = previous_lifetimes;
        self.lifetime_header = previous_header;
        self.inherent_owner = previous_inherent;
        self.source_type_aliases.stack.remove(&row.key);
        result
    }
    pub(super) fn read_type_alias_value(&mut self, segments: &[String]) -> Result<Option<Type>> {
        if !self.ordinary_completion_context() {
            return Ok(None);
        }
        let resolved = self.resolve(segments, &self.module_path);
        for key in rules::read_rust_type_alias_lookup_keys(&resolved, &self.module_path) {
            if let Some(row) = self.source_type_aliases.declarations.get(&key).cloned() {
                let mut origin = if row.parameters.is_empty() { self.expand_type_alias(&row)? } else { self.read_generic_type_alias_value(&row)? };
                origin.expanded_source_owner = origin.source_owner.clone();
                let ty = origin.ty.clone().unwrap();
                self.origins.current_type = origin;
                return Ok(Some(ty));
            }
            if rules::read_rust_type_alias_namespace_disposition(
                false,
                self.alias_namespace_kind(&key),
            ) == "block"
            {
                return Ok(None);
            }
        }
        Ok(None)
    }
    pub(super) fn read_type_alias_item(&mut self, path: &[String]) -> Result<()> {
        let Some(row) = self
            .source_type_aliases
            .declarations
            .values()
            .find(|row| {
                row.keyword.start == self.cursor.peek().start && row.path.as_slice() == path
            })
            .cloned()
        else {
            return Err(self.alias_error("the alias needs its actual module declaration", None));
        };
        self.cursor.index = self
            .cursor
            .tokens
            .iter()
            .position(|token| token.start == row.end)
            .expect("original type alias source end")
            + 1;
        Ok(())
    }
    pub(super) fn finish_type_aliases(&mut self) -> Result<()> {
        if !self.ordinary_completion_context() || self.source_type_aliases.checked {
            return Ok(());
        }
        self.source_type_aliases.cache.clear();
        self.source_type_aliases.templates.clear();
        self.source_type_aliases.defaults.clear();
        let mut rows = self
            .source_type_aliases
            .declarations
            .values()
            .cloned()
            .collect::<Vec<_>>();
        rows.sort_by_key(|row| row.keyword.start);
        for row in rows {
            if matches!(
                self.origins.namespace_entry(&row.key,"type"),
                Some(SourceDeclaration::Data | SourceDeclaration::Module)
            ) {
                return Err(self.alias_error(
                    "a type alias cannot replace another source type or module",
                    Some(&row),
                ));
            }
            if !row.parameters.is_empty() {
                self.prepare_generic_type_alias(&row)?;
                continue;
            }
            let origin = self.expand_type_alias(&row)?;
            let mut types = origin.ty.as_ref().into_iter().collect::<Vec<_>>();
            let mut position = 0;
            while position < types.len() {
                let ty = types[position];
                position += 1;
                types.extend(children(ty));
                if let Type::Named { path, .. } = ty {
                    let resolved = self.resolve(path, &origin.path);
                    if self.alias_data_owner(&resolved, &origin.path).is_none() {
                        return Err(self.alias_error(
                            "the complete alias definition needs an actual source datatype owner",
                            Some(&row),
                        ));
                    }
                }
            }
        }
        self.source_type_aliases.checked = true;
        Ok(())
    }
    pub(super) fn resolve_type_alias_path(
        &self,
        names: &[String],
        path: &[String],
        role: &str,
    ) -> Vec<String> {
        if !self.ordinary_completion_context() || self.source_type_aliases.declarations.is_empty() {
            return names.to_vec();
        }
        for prefix in rules::read_rust_type_alias_access_prefix_lengths(
            names.len() as f64,
            role != "associated value",
        ) {
            let prefix = prefix as usize;
            let resolved = self.resolve(&names[..prefix], path);
            for key in rules::read_rust_type_alias_lookup_keys(&resolved, &self.module_path) {
                let disposition = rules::read_rust_type_alias_namespace_disposition(
                    self.source_type_aliases.declarations.contains_key(&key),
                    self.alias_namespace_kind(&key),
                );
                if disposition == "block" {
                    return names.to_vec();
                }
                if disposition != "alias" {
                    continue;
                }
                let Some(origin) = self.source_type_aliases.cache.get(&key) else {
                    return names.to_vec();
                };
                if role == "associated integer constant" {
                    let owner = origin
                        .expanded_source_owner
                        .as_ref()
                        .or(origin.source_owner.as_ref());
                    let (bits, signed) = match origin.ty.as_ref() {
                        Some(Type::Fixed { bits, signed }) => (bits.to_string(), *signed),
                        _ => (String::new(), false),
                    };
                    let count =
                        |value: usize| u32::try_from(value).map_or(f64::INFINITY, f64::from);
                    let primitive=crate::translation::rust_constant_integer_rules::read_rust_constant_primitive_alias_owner(origin.ty.as_ref().map_or("",Type::kind),owner.map_or("",|owner|owner.kind.as_str()),&owner.map(|owner|owner.path.join("::")).unwrap_or_default(),owner.map_or(-1.0,|owner|count(owner.arguments.len())),count(origin.wrappers.len()),&bits,signed);
                    if !primitive.is_empty() {
                        return std::iter::once(primitive)
                            .chain(names[prefix..].iter().cloned())
                            .collect();
                    }
                }

                let Some(Type::Named { path: target, .. }) = origin.ty.as_ref() else {
                    return names.to_vec();
                };
                let resolved = self.resolve(target, &origin.path);
                let Some(owner) = self.alias_data_owner(&resolved, &origin.path) else {
                    return names.to_vec();
                };
                let syntax = self
                    .named_records
                    .get(&owner)
                    .map_or("", |record| record.syntax);
                if !rules::accept_rust_type_alias_nominal_access(
                    "named",
                    origin.wrappers.len() as f64,
                    (names.len() - prefix) as f64,
                    role,
                    syntax,
                ) {
                    return names.to_vec();
                }
                return std::iter::once("crate".to_owned())
                    .chain(owner.split('.').map(str::to_owned))
                    .chain(names[prefix..].iter().cloned())
                    .collect();
            }
        }
        names.to_vec()
    }
    pub(super) fn require_type_alias_value_constructor(
        &self,
        names: &[String],
        path: &[String],
        span: Option<super::Span>,
    ) -> Result<()> {
        if !self.ordinary_completion_context() || self.source_type_aliases.declarations.is_empty() {
            return Ok(());
        }
        let resolved = self.resolve(names, path);
        let local = names.len() == 1 && self.origins.scope.contains_key(&names[0]);
        let keys = rules::read_rust_type_alias_lookup_keys(&resolved, &self.module_path);
        let value = keys.iter().any(|key| {
            matches!(
                self.origins.namespace_entry(key,"value"),
                Some(
                    SourceDeclaration::Function { .. }
                        | SourceDeclaration::Constant { .. }
                        | SourceDeclaration::Constructor { .. }
                )
            )
        });
        let alias = keys
            .iter()
            .any(|key| self.source_type_aliases.declarations.contains_key(key));
        let message = rules::read_rust_type_alias_value_constructor_error(local, value, alias);
        if message.is_empty() {
            Ok(())
        } else {
            Err(unsupported(
                "Rust type alias value constructor",
                message,
                span,
            ))
        }
    }
}

impl RustParser {
    pub(super) fn primitive_type_alias_member_path(
        &self,
        names: &[String],
        path: &[String],
        absolute: bool,
    ) -> Vec<String> {
        if absolute || !self.ordinary_completion_context() || names.len() < 2 {
            return Vec::new();
        }
        let resolved = self.resolve(&names[..names.len() - 1], path);
        for key in rules::read_rust_type_alias_lookup_keys(&resolved, path) {
            let disposition = rules::read_rust_type_alias_namespace_disposition(
                self.source_type_aliases.declarations.contains_key(&key),
                self.alias_namespace_kind(&key),
            );
            if disposition == "block" {
                return Vec::new();
            }
            if disposition != "alias" {
                continue;
            }
            let Some(origin) = self.source_type_aliases.cache.get(&key) else {
                return Vec::new();
            };
            let owner = origin
                .expanded_source_owner
                .as_ref()
                .or(origin.source_owner.as_ref());
            let (bits, signed) = match origin.ty.as_ref() {
                Some(Type::Fixed { bits, signed }) => (bits.to_string(), *signed),
                _ => (String::new(), false),
            };
            let count = |value: usize| u32::try_from(value).map_or(f64::INFINITY, f64::from);
            let selected = rules::read_rust_type_alias_primitive_member_owner(
                origin.ty.as_ref().map_or("", Type::kind),
                &bits,
                signed,
                owner.map_or("", |value| value.kind.as_str()),
                &owner.map(|value| value.path.join("::")).unwrap_or_default(),
                owner.map_or(-1.0, |value| count(value.arguments.len())),
                origin.proven,
                count(origin.wrappers.len()),
                1.0,
            );
            return if selected.is_empty() {
                Vec::new()
            } else {
                vec![
                    "core".to_owned(),
                    "primitive".to_owned(),
                    selected,
                    names.last().unwrap().clone(),
                ]
            };
        }
        Vec::new()
    }
}

impl RustParser {
    pub(super) fn type_alias_string_member_owner(
        &self,
        names: &[String],
        path: &[String],
    ) -> Option<crate::translation::source_signatures::SourceOwner> {
        if !self.ordinary_completion_context() || names.len() < 2 {
            return None;
        }
        let resolved = self.resolve(&names[..names.len() - 1], path);
        for key in rules::read_rust_type_alias_lookup_keys(&resolved, path) {
            let disposition = rules::read_rust_type_alias_namespace_disposition(
                self.source_type_aliases.declarations.contains_key(&key),
                self.alias_namespace_kind(&key),
            );
            if disposition == "block" {
                return None;
            }
            if disposition != "alias" {
                continue;
            }
            let origin = self.source_type_aliases.cache.get(&key)?;
            let owner = origin
                .expanded_source_owner
                .as_ref()
                .or(origin.source_owner.as_ref())?;
            let count = |value: usize| u32::try_from(value).map_or(f64::INFINITY, f64::from);
            return rules::accept_rust_type_alias_owned_string_member(
                origin.ty.as_ref().map_or("", Type::kind),
                &owner.kind,
                &owner.path.join("::"),
                count(owner.arguments.len()),
                origin.proven,
                count(origin.wrappers.len()),
                names.last().unwrap(),
            )
            .then(|| owner.clone());
        }
        None
    }
}

impl RustParser {
    pub(super) fn type_alias_empty_vector_origin(
        &self,
        names: &[String],
        path: &[String],
    ) -> Option<SourceOrigin> {
        if !self.ordinary_completion_context() || names.len() < 2 {
            return None;
        }
        let resolved = self.resolve(&names[..names.len() - 1], path);
        for key in rules::read_rust_type_alias_lookup_keys(&resolved, path) {
            let disposition = rules::read_rust_type_alias_namespace_disposition(
                self.source_type_aliases.declarations.contains_key(&key),
                self.alias_namespace_kind(&key),
            );
            if disposition == "block" {
                return None;
            }
            if disposition != "alias" {
                continue;
            }
            let origin = self.source_type_aliases.cache.get(&key)?;
            let owner = origin
                .expanded_source_owner
                .as_ref()
                .or(origin.source_owner.as_ref())?;
            let element = match origin.ty.as_ref() {
                Some(Type::Array { element, .. }) => element.kind(),
                _ => "",
            };
            let count = |value: usize| u32::try_from(value).map_or(f64::INFINITY, f64::from);
            return rules::accept_rust_type_alias_empty_vector_member(
                origin.ty.as_ref().map_or("", Type::kind),
                &owner.kind,
                &owner.path.join("::"),
                count(owner.arguments.len()),
                element,
                origin.array_source_model.unwrap_or(""),
                origin.proven,
                count(origin.wrappers.len()),
                names.last().unwrap(),
            )
            .then(|| origin.clone());
        }
        None
    }
}
