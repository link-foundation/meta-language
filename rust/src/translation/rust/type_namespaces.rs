//! Actual checked catalog classification around the existing anchored lookup.
use super::{
    Result, RustParser, SourceDeclaration, SourceOrigin, SourceOrigins, type_error, unsupported,
};
use crate::translation::ir::{Decl, Item, Program};
use crate::translation::type_namespace_rules::accept_rust_type_namespace_anchor;
use std::collections::HashMap;

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
                full_name, items, ..
            } = item
            {
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
