//! Source owner resolution for concrete standard collections.
use std::{collections::HashSet, rc::Rc};

use super::{
    Result, RustParser, SData, SExpr, SItem, SProgram, SourceDeclaration, SourceOrigin, Token,
    Type, span, unsupported,
};
use crate::translation::rust_standard_collection_carriers::{
    CollectionSourceDeclarations, create_source_carrier, project_source_library,
};
use crate::translation::rust_standard_collection_value_packet::{
    ml_string::Text, read_rust_standard_collection_integer_width,
    read_rust_standard_collection_kind, resolve_rust_standard_collection_owner,
};
use crate::translation::source_signatures::SourceOwner;

mod packets;

pub(super) struct StandardCollectionSource {
    library: Rc<dyn CollectionSourceDeclarations>,
    carriers: Vec<SourceCarrier>,
}

#[derive(Clone)]
pub(super) struct SourceCarrier {
    data: SData,
    owner: SourceOwner,
    domains: Vec<String>,
    arguments: Vec<SourceOrigin>,
}

impl StandardCollectionSource {
    pub(super) fn new(library: Rc<dyn CollectionSourceDeclarations>) -> Self {
        Self {
            library,
            carriers: Vec::new(),
        }
    }
}

fn scalar_domain(origin: &SourceOrigin) -> Option<String> {
    let owner = origin
        .expanded_source_owner
        .as_ref()
        .or(origin.source_owner.as_ref())?;
    if !origin.proven || !origin.wrappers.is_empty() || !owner.arguments.is_empty() {
        return None;
    }
    let ty = origin.ty.as_ref()?;
    if *ty == super::STRING
        && owner.kind == "library"
        && matches!(
            owner.path.join("::").as_str(),
            "std::string::String" | "alloc::string::String"
        )
        && origin.text_source_model == Some("owned string")
    {
        return Some("String".to_owned());
    }
    if owner.kind != "primitive" {
        return None;
    }
    let domain = match owner.path.as_slice() {
        [domain] => domain,
        [root, primitive, domain]
            if matches!(root.as_str(), "std" | "core") && primitive == "primitive" =>
        {
            domain
        }
        _ => return None,
    };
    if *ty == super::BOOL && domain == "bool" {
        return Some(domain.clone());
    }
    let Type::Fixed { bits, signed } = ty else {
        return None;
    };
    let width = read_rust_standard_collection_integer_width(&Text::from_text(domain), 64.0);
    (width > 0.0 && width == f64::from(*bits) && *signed == domain.starts_with('i'))
        .then(|| domain.clone())
}

impl RustParser {
    pub(super) fn inherit_standard_collection_source(
        &mut self,
        previous: Option<StandardCollectionSource>,
    ) {
        if let Some(collection) = &previous {
            self.reserved_binding_names.extend(
                collection
                    .carriers
                    .iter()
                    .map(|carrier| carrier.data.name.clone()),
            );
        }
        self.standard_collections = previous;
    }

    pub(super) fn refuse_collection_observation(
        &self,
        expression: &SExpr,
        path: &[String],
        operation: &str,
        direct_only: bool,
    ) -> Result<()> {
        let Some(collection) = self.standard_collections.as_ref() else {
            return Ok(());
        };
        let mut pending = vec![self.origins.origin(expression, path)];
        let mut seen = HashSet::new();
        let mut work = 0;
        while let Some(origin) = pending.pop() {
            work += 1;
            if work > 65_536 {
                return Err(unsupported(
                    "Rust collection observation",
                    "the source type exceeds its finite traversal budget",
                    expression.span,
                ));
            }
            if let Some(Type::Named { path, .. }) = &origin.ty
                && let Some(owner) = self.origins.lookup_type_owner(path, &origin.path)
            {
                if collection
                    .carriers
                    .iter()
                    .any(|carrier| carrier.data.name == owner.full_name)
                {
                    return Err(unsupported(
                        "Rust collection observation",
                        &format!(
                            "{operation} requires a separate source interpretation for an opaque collection"
                        ),
                        expression.span,
                    ));
                }
                if !direct_only && seen.insert(owner.full_name.clone()) {
                    for declaration in self.origins.value_catalog.values() {
                        work += 1;
                        if work > 65_536 {
                            return Err(unsupported(
                                "Rust collection observation",
                                "the source catalog exceeds its finite traversal budget",
                                expression.span,
                            ));
                        }
                        if let SourceDeclaration::Constructor {
                            owner: actual,
                            fields,
                            ..
                        } = declaration
                            && actual == &owner.full_name
                        {
                            pending.extend(fields.iter().cloned());
                        }
                    }
                }
            }
            if direct_only {
                continue;
            }
            pending.extend(origin.fields.into_iter().flatten());
            match origin.ty {
                Some(Type::Array { element, .. } | Type::Option { element, .. }) => {
                    pending.push(SourceOrigin::with_type(*element, &origin.path, Vec::new()));
                }
                Some(Type::Tuple { elements }) => {
                    pending.extend(
                        elements
                            .into_iter()
                            .map(|ty| SourceOrigin::with_type(ty, &origin.path, Vec::new())),
                    );
                }
                _ => {}
            }
        }
        Ok(())
    }

    fn standard_collection_owner(&self, segments: &[String], path: &[String]) -> Option<String> {
        self.standard_collections.as_ref()?;
        let resolved = self.resolve(segments, path);
        let shadowed = [segments, resolved.as_slice()].iter().any(|names| {
            let Some(first) = names.first() else {
                return true;
            };
            self.origins.lookup_type_owner(names, path).is_some()
                || self.origins.lookup_type_owner(&names[..1], path).is_some()
                || self.origins.lookup_value_owner(&names[..1], path).is_some()
                || self.origins.scope.contains_key(first)
                || path.first().is_some_and(|_| {
                    self.source_type_aliases.declarations.contains_key(&format!(
                        "{}.{}",
                        path.join("."),
                        first
                    ))
                })
                || self.source_type_aliases.declarations.contains_key(first)
        });
        let owner = resolve_rust_standard_collection_owner(
            &Text::from_text(&resolved.join("::")),
            self.ordinary_completion_context(),
            shadowed,
        );
        (owner != Text::from_text("")).then(|| owner.to_string())
    }

    pub(super) fn standard_collection_import(&self, names: &[String], path: &[String]) -> bool {
        self.standard_collections.is_some()
            && (names.first().is_some_and(|first| first == "std")
                && self.standard_collection_owner(names, path).is_some()
                || self.standard_collection_trait(
                    names,
                    path,
                    ["std::cmp::PartialEq", "core::cmp::PartialEq"],
                )
                || self.standard_collection_trait(
                    names,
                    path,
                    ["std::clone::Clone", "core::clone::Clone"],
                ))
    }

    fn standard_collection_trait(
        &self,
        names: &[String],
        path: &[String],
        owners: [&str; 2],
    ) -> bool {
        let resolved = self.resolve(names, path);
        owners.contains(&resolved.join("::").as_str())
            && !self.origins.scope.contains_key(&names[0])
            && self.origins.lookup_type_owner(names, path).is_none()
            && self
                .origins
                .lookup_type_owner(&resolved[..1], path)
                .is_none()
            && self
                .origins
                .lookup_value_owner(&resolved[..1], path)
                .is_none()
    }

    pub(super) fn refuse_collection_library_path(
        &self,
        names: &[String],
        path: &[String],
        token: &Token,
    ) -> Result<()> {
        let Some(collection) = self.standard_collections.as_ref() else {
            return Ok(());
        };
        let resolved = self.resolve(names, path);
        let relative = resolved
            .strip_prefix(&["crate".to_owned()])
            .unwrap_or(&resolved);
        if relative.first().is_some_and(|name| {
            collection
                .carriers
                .iter()
                .any(|carrier| &carrier.data.name == name)
        }) || relative.iter().enumerate().any(|(index, _)| {
            collection
                .library
                .program()
                .declaration(&relative[..=index].join("."))
                .is_some()
        }) {
            return Err(unsupported(
                "Rust standard collection source interface",
                "projected implementation names are private to the source adapter",
                Some(token.span()),
            ));
        }
        Ok(())
    }

    pub(super) fn standard_collection_type(
        &mut self,
        names: &[String],
        token: &Token,
    ) -> Result<Option<Type>> {
        let Some(owner) = self.standard_collection_owner(names, &self.module_path) else {
            return Ok(None);
        };
        let map =
            read_rust_standard_collection_kind(&Text::from_text(&owner)) == Text::from_text("map");
        let mut domains = Vec::new();
        let mut arguments = Vec::new();
        self.cursor
            .expect("<", Some("concrete standard collection type"))?;
        for index in 0..if map { 2 } else { 1 } {
            if index != 0 {
                self.cursor
                    .expect(",", Some("standard collection type argument"))?;
            }
            let start = self.peek();
            self.parse_type()?;
            let domain = scalar_domain(&self.origins.current_type).ok_or_else(|| unsupported(
                "Rust standard collection source type",
                "each key and value needs its actual closed direct String, Boolean or integer source owner",
                span(&start, self.cursor.peek()),
            ))?;
            domains.push(domain);
            arguments.push(self.origins.current_type.clone());
        }
        self.cursor.eat(",");
        self.split_type_closing_token();
        self.cursor.expect(">", Some("standard collection type"))?;
        let location = Some(token.span());
        let collection = self
            .standard_collections
            .as_mut()
            .expect("selected source collection profile");
        let owner_segments: Vec<_> = owner.split("::").map(str::to_owned).collect();
        let carrier =
            if let Some(index) = collection.carriers.iter().position(|carrier| {
                carrier.owner.path == owner_segments && carrier.domains == domains
            }) {
                &collection.carriers[index]
            } else {
                if collection.carriers.len() >= 256 {
                    return Err(unsupported(
                        "Rust standard collection source type",
                        "concrete carrier specializations exceed their finite budget",
                        location,
                    ));
                }
                let base = format!(
                    "RustStandardCollection{}_{}_{}_64",
                    owner_segments.last().expect("canonical owner"),
                    domains[0],
                    domains.get(1).map_or("unit", String::as_str)
                );
                let name = (0..=100_000)
                    .map(|index| {
                        if index == 0 {
                            base.clone()
                        } else {
                            format!("{base}_{index}")
                        }
                    })
                    .find(|candidate| {
                        !self.reserved_binding_names.contains(candidate)
                            && collection
                                .library
                                .program()
                                .declaration(candidate)
                                .is_none()
                    })
                    .ok_or_else(|| {
                        unsupported(
                            "Rust standard collection source type",
                            "the concrete carrier exhausted its fresh name supply",
                            location,
                        )
                    })?;
                let (data, owner) = create_source_carrier(
                    collection.library.as_ref(),
                    name,
                    &owner,
                    &domains,
                    self.source_version_minor,
                    location,
                )?;
                self.reserved_binding_names.insert(data.name.clone());
                collection.carriers.push(SourceCarrier {
                    data,
                    owner,
                    domains,
                    arguments,
                });
                collection.carriers.last().expect("retained specialization")
            };
        self.origins
            .register_catalog_entry(carrier.data.name.clone(), SourceDeclaration::Data);
        self.origins
            .standard_collection_owners
            .insert(carrier.data.name.clone(), carrier.owner.clone());
        let ty = Type::Named {
            path: vec!["crate".to_owned(), carrier.data.name.clone()],
            span: None,
        };
        let ty = self.origins.typed(ty, &self.module_path, Vec::new());
        self.origins.current_type.source_owner = Some(carrier.owner.clone());
        self.origins.current_type.expanded_source_owner = Some(carrier.owner.clone());
        Ok(Some(ty))
    }

    pub(super) fn standard_collection_origin(
        &self,
        origin: &SourceOrigin,
    ) -> Option<&SourceCarrier> {
        let collection = self.standard_collections.as_ref()?;
        if !origin.proven || origin.wrappers.iter().any(|layer| layer != "reference") {
            return None;
        }
        let Type::Named { path, .. } = origin.ty.as_ref()? else {
            return None;
        };
        let [anchor, name] = path.as_slice() else {
            return None;
        };
        if anchor != "crate" {
            return None;
        }
        let owner = origin
            .expanded_source_owner
            .as_ref()
            .or(origin.source_owner.as_ref())?;
        collection
            .carriers
            .iter()
            .find(|carrier| &carrier.data.name == name && &carrier.owner == owner)
    }

    pub(super) fn finish_standard_collection_source(
        &mut self,
        program: &mut SProgram,
    ) -> Result<()> {
        let Some(collection) = &mut self.standard_collections else {
            return Ok(());
        };
        let carriers = collection
            .carriers
            .iter()
            .map(|carrier| SItem::Data(carrier.data.clone()))
            .collect();
        project_source_library(program, collection.library.as_ref(), carriers)
    }
}
