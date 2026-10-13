//! Source descriptor adapters for the shared standard primitive owner policy.
use super::primitive_owner_rules::accept_rust_standard_primitive_owner;
use super::{Result, RustParser, SourceOrigin, Span, unsupported};
use serde_json::{Value, json};

#[must_use]
pub(crate) fn accept_import_name(name: &str) -> bool {
    let policy = crate::translation::source_primitive_owners::policy();
    regex::Regex::new(policy["aliasPattern"].as_str().expect("alias pattern"))
        .expect("valid shared alias pattern")
        .is_match(name)
        && !policy["forbiddenAliases"]
            .as_array()
            .expect("alias names")
            .iter()
            .any(|alias| alias.as_str() == Some(name))
        && super::indirection_rules::accept_rust_source_lifetime_binding(name, false, false)
}

impl RustParser {
    pub(super) fn read_standard_primitive_position(
        &self,
        source: &[String],
        path: &[String],
    ) -> Result<Option<Value>> {
        if self.primitive_owner_model.is_empty() {
            return Ok(None);
        }
        let resolved = self.resolve(source, path);
        let spelling = resolved.join("::");
        let candidate = crate::translation::source_primitive_owners::policy()["paths"]
            .as_array()
            .expect("primitive paths")
            .iter()
            .any(|value| value.as_str() == Some(spelling.as_str()));
        if !candidate && !(source.len() == 1 && source[0] == "f64") {
            return Ok(None);
        }
        let declared = self.origins.lookup_type_owner(source, path).is_some()
            || self.origins.lookup_type_owner(&resolved, path).is_some();
        let shadowed = resolved.len() > 1
            && self
                .origins
                .lookup_type_owner(&resolved[..1], path)
                .is_some();
        if ((spelling != "f64"
            && !accept_rust_standard_primitive_owner(&spelling, declared, shadowed))
            || (spelling == "f64" && declared))
            && !self.origins.collecting
        {
            return Err(unsupported(
                "Rust primitive owner",
                "the actual standard primitive path and namespace must be unshadowed",
                None,
            ));
        }
        let alias = (source.len() == 1 && spelling != "f64").then(|| source[0].clone());
        Ok(Some(
            json!({"sourcePath":source,"resolvedPath":resolved,"alias":alias}),
        ))
    }

    #[must_use]
    pub(super) fn create_primitive_owner_proof(
        &self,
        parameters: &[SourceOrigin],
        result: &SourceOrigin,
        range: Option<Span>,
    ) -> Value {
        let position = |origin: &SourceOrigin| {
            if origin.ty.as_ref().is_none_or(|ty| ty.kind() != "float") {
                return Value::Null;
            }
            let mut owner = origin.primitive_owner_position.clone().unwrap_or_else(
                || json!({"sourcePath":["f64"],"resolvedPath":["f64"],"alias":null}),
            );
            let record = owner.as_object_mut().expect("primitive position record");
            record.insert("wrappers".to_owned(), json!(origin.wrappers));
            record.insert("lifetimes".to_owned(), json!(origin.source_lifetimes));
            owner
        };
        let constants = self
            .primitive_constant_sites
            .iter()
            .filter(|site| {
                range.is_some_and(|range| {
                    site["span"]["start"].as_u64().is_some_and(|start| {
                        start >= u64::try_from(range.start).expect("source start fits")
                    }) && site["span"]["end"].as_u64().is_some_and(|end| {
                        end <= u64::try_from(range.end).expect("source end fits")
                    })
                })
            })
            .map(|site| {
                let mut site = site.clone();
                site.as_object_mut()
                    .unwrap()
                    .insert("path".to_owned(), Value::Null);
                site
            })
            .collect::<Vec<_>>();
        json!({"model":self.primitive_owner_model,"language":"Rust","namespace":{"declaredTypeOwners":[],"externalPrimitiveRoots":["std","core"],"imports":self.primitive_imports},"parameters":parameters.iter().map(position).collect::<Vec<_>>(),"result":position(result),"constants":constants})
    }
}
