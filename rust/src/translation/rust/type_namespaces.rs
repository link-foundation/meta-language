//! Actual checked catalog classification around the existing anchored lookup.
use super::{
    Result, RustParser, SourceDeclaration, SourceOrigin, SourceOrigins, type_error, unsupported,
};
use super::{SExpr, Span, Token, Type, span};
use crate::translation::ir::{Decl, Item, Program};
use crate::translation::lexer::accept_rust_source_identifier_name;
use crate::translation::type_namespace_rules::{
    accept_rust_type_namespace_anchor, accept_rust_type_namespace_body_constructor,
    accept_rust_type_namespace_body_method, accept_rust_type_namespace_discovery_type,
    accept_rust_type_namespace_method_discovery, accept_rust_type_namespace_unit_clone,
};
use std::collections::HashMap;

/// Use the same scalar property definition as the actual paired Rust lexer.
pub fn accept_namespace_segment(value: &str) -> bool {
    accept_rust_source_identifier_name(value)
}

pub fn require_anchor(source: &[String], path: &[String]) -> Result<()> {
    let parents = source
        .iter()
        .take_while(|part| part.as_str() == "super")
        .count();
    if !accept_rust_type_namespace_anchor(parents > path.len()) {
        return Err(type_error(
            "an owned Rust type namespace spelling traverses above its crate root",
            None,
        ));
    }
    Ok(())
}

pub struct TypeNamespaceCatalog {
    origins: SourceOrigins,
}

impl TypeNamespaceCatalog {
    pub fn new(program: &Program) -> Result<Self> {
        let mut catalog = HashMap::new();
        for declaration in &program.declarations {
            if !accept_namespace_segment(declaration.name())
                || !declaration
                    .module_path()
                    .iter()
                    .all(|part| accept_namespace_segment(part))
            {
                return Err(type_error(
                    "an owned Rust type namespace catalog contains an invalid Rust identifier segment",
                    None,
                ));
            }
            let classification = match declaration {
                Decl::Data(_) => SourceDeclaration::Data,
                Decl::Fn(_) | Decl::Theorem(_) => SourceDeclaration::Function {
                    parameters: Vec::new(),
                    result: SourceOrigin::unknown(),
                },
            };
            catalog.insert(declaration.full_name().to_owned(), classification);
            if let Decl::Data(data) = declaration {
                for constructor in &data.ctors {
                    if !accept_namespace_segment(&constructor.name) {
                        return Err(type_error(
                            "an owned Rust type namespace catalog contains an invalid constructor identifier segment",
                            None,
                        ));
                    }
                    let key = format!("{}.{}", data.full_name, constructor.name);
                    if catalog.contains_key(&key) || program.declaration(&key).is_some() {
                        return Err(type_error(
                            "an owned Rust type namespace cannot admit a constructor-overwritten declaration identity",
                            None,
                        ));
                    }
                    catalog.insert(
                        key,
                        SourceDeclaration::Constructor {
                            owner: data.full_name.clone(),
                            fields: Vec::new(),
                            result: SourceOrigin::unknown(),
                        },
                    );
                }
            }
        }
        let mut modules = program.items.iter().collect::<Vec<_>>();
        while let Some(item) = modules.pop() {
            if let Item::Module {
                name,
                full_name,
                items,
                ..
            } = item
            {
                if !accept_namespace_segment(name) {
                    return Err(type_error(
                        "an owned Rust type namespace catalog contains an invalid module identifier segment",
                        None,
                    ));
                }
                if catalog.contains_key(full_name) {
                    return Err(type_error(
                        "an owned Rust type namespace cannot admit a shared module/declaration identity",
                        None,
                    ));
                }
                catalog.insert(full_name.clone(), SourceDeclaration::Module);
                modules.extend(items);
            }
        }
        Ok(Self {
            origins: SourceOrigins::new(Some(catalog), ""),
        })
    }

    pub fn lookup(&self, source: &[String], path: &[String]) -> Option<(String, &'static str)> {
        self.origins.lookup_type_owner(source, path).map(|value| {
            let kind = match value.declaration {
                SourceDeclaration::Data => "data",
                SourceDeclaration::Module => "module",
                SourceDeclaration::Constructor { .. } => "ctor",
                SourceDeclaration::ExternalCrate => "externalCrate",
                SourceDeclaration::Function { .. } => "fn",
            };
            (value.full_name, kind)
        })
    }
}

impl RustParser {
    pub(super) fn require_type_namespace_without_imports(&self) -> Result<()> {
        if !self.type_namespace_model.is_empty() {
            return Err(unsupported(
                "Rust type namespace imports",
                "imported aliases require a separate source binding proof",
                None,
            ));
        }
        Ok(())
    }

    pub(super) fn require_type_namespace_without_constant(&self) -> Result<()> {
        if !self.type_namespace_model.is_empty() {
            return Err(unsupported(
                "Rust type namespace constant",
                "constant-as-function projection needs a separate owned source binding proof",
                None,
            ));
        }
        Ok(())
    }

    pub(super) fn capture_type_namespace(
        &self,
        parameters: &[SourceOrigin],
        result: &SourceOrigin,
    ) -> Option<serde_json::Value> {
        (!self.type_namespace_model.is_empty()).then(|| serde_json::json!({
            "model":self.type_namespace_model,"language":"Rust",
            "parameters":parameters.iter().map(|origin| &origin.namespace_spelling).collect::<Vec<_>>(),
            "result":result.namespace_spelling
        }))
    }
}

impl RustParser {
    pub(super) fn require_type_namespace_body_constructor(
        &self,
        name: &str,
        range: Option<Span>,
    ) -> Result<()> {
        if !accept_rust_type_namespace_body_constructor(!self.type_namespace_model.is_empty(), name)
        {
            return Err(unsupported(
                "Rust type namespace body constructor",
                &format!("{name} requires a separate actual owner and argument proof"),
                range,
            ));
        }
        Ok(())
    }

    pub(super) fn require_type_namespace_body_method(
        &self,
        receiver: &SExpr,
        name: &str,
        range: Option<Span>,
    ) -> Result<()> {
        let origin = self.origins.origin(receiver, &self.module_path);
        let owner = if let Some(Type::Named { path, .. }) = &origin.ty {
            self.origins.lookup_type_owner(path, &origin.path)
        } else {
            None
        };
        let facts = owner
            .as_ref()
            .and_then(|owner| self.origins.unit_clone_facts.get(&owner.full_name));
        let admitted = accept_rust_type_namespace_unit_clone(
            facts.is_some_and(|(proof, _)| proof.is_some()),
            facts.is_some_and(|(_, unit)| *unit),
            origin.proven && origin.wrappers.is_empty(),
        );
        let discovery = accept_rust_type_namespace_method_discovery(
            self.type_namespace_discovery.is_some(),
            name,
        );
        if !accept_rust_type_namespace_body_method(
            !self.type_namespace_model.is_empty() && !discovery,
            name,
            admitted,
        ) {
            return Err(unsupported(
                "Rust type namespace body method",
                &format!("{name} requires a separate actual method owner proof"),
                range,
            ));
        }
        Ok(())
    }

    pub(super) fn read_type_namespace_discovery_type(
        &mut self,
        name: &str,
        resolved: &[String],
        token: &Token,
    ) -> Option<Type> {
        accept_rust_type_namespace_discovery_type(self.type_namespace_discovery.is_some(), name)
            .then(|| {
                self.origins.typed(
                    Type::Named {
                        path: resolved.to_vec(),
                        span: span(token, token),
                    },
                    &self.module_path,
                    Vec::new(),
                )
            })
    }
}

impl RustParser {
    pub(super) fn require_type_namespace_without_externals(&self, start: &Token) -> Result<()> {
        if !self.type_namespace_model.is_empty()
            && self.module_path.is_empty()
            && self.cursor.is("extern")
            && self.cursor.is_at("crate", 1)
            && self.cursor.is_at("alloc", 2)
            && self.cursor.is_at(";", 3)
        {
            return Err(unsupported(
                "Rust type namespace imports",
                "external crate declarations require a separate source binding proof",
                span(start, start),
            ));
        }
        Ok(())
    }
    pub(super) fn require_unit_clone_attached_enum(
        &self,
        attributes: &[serde_json::Value],
        start: &Token,
    ) -> Result<()> {
        if !self.type_namespace_model.is_empty() && has_clone(attributes) && !self.cursor.is("enum")
        {
            return Err(unsupported(
                "Rust unit Clone derivation",
                "a Clone derive must be attached to an actual datatype",
                span(start, self.cursor.peek()),
            ));
        }
        Ok(())
    }
    pub(super) fn require_unit_clone_attached_variant(
        &self,
        attributes: &[serde_json::Value],
    ) -> Result<()> {
        if !self.type_namespace_model.is_empty() && has_clone(attributes) {
            return Err(unsupported(
                "Rust unit Clone derivation",
                "a Clone derive must be attached to its datatype",
                span(self.cursor.peek(), self.cursor.peek()),
            ));
        }
        Ok(())
    }
}
fn has_clone(attributes: &[serde_json::Value]) -> bool {
    attributes.iter().any(|attribute| {
        attribute["traits"]
            .as_array()
            .is_some_and(|traits| traits.iter().any(|name| name == "Clone"))
    })
}
