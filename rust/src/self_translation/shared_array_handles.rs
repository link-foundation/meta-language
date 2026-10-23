//! Whole-source publication facts, with actual concrete visibility owners.
use super::{SelfTranslationError, SelfTranslationOptions, SelfTranslationSignature};
use crate::translation::emit_common::Emitted;
use crate::translation::ir::Decl;
use crate::translation::rust_shared_array_handle_rules as rules;
use crate::translation::surface::{External, ExternalParam};
use std::collections::{HashMap, HashSet};
fn failure(reason: &str) -> SelfTranslationError {
    SelfTranslationError {
        message: format!("Rust shared array handle module: {reason}"),
    }
}
pub(super) fn validate_options(
    source: &str,
    language: &str,
    options: &SelfTranslationOptions,
) -> Result<(), SelfTranslationError> {
    let model = &options.rust_shared_array_handle_source_model;
    if !rules::accept_rust_shared_array_handle_source_interpretation(model) {
        return Err(failure("select the installed source model"));
    }
    if model.is_empty() {
        return Ok(());
    }
    if language != "Rust" {
        return Err(failure(
            "the selected source interpretation requires Rust source",
        ));
    }
    if !options.module_directory.is_empty() || !options.imports.is_empty() || options.utf16_strings
    {
        return Err(failure(
            "additional module or representation interpretations require independent composition proofs",
        ));
    }
    let units = u32::try_from(source.encode_utf16().take(65_537).count()).unwrap_or(u32::MAX);
    if !rules::accept_rust_shared_array_handle_source_size(f64::from(units)) {
        return Err(failure(
            "the profile requires its complete source within65536 UTF16 units",
        ));
    }
    Ok(())
}
pub(super) fn read_whole_module(
    source: &str,
    model: &str,
) -> Result<(Emitted, Vec<SelfTranslationSignature>), SelfTranslationError> {
    let surface =
        crate::translation::rust::parse_rust_with_shared_array_handle_source_model(source, model)
            .map_err(|error| failure(&error.to_string()))?;
    let program = crate::translation::check::check_program(&surface)
        .map_err(|error| failure(&error.to_string()))?;
    if program.main.is_some() {
        return Err(failure(
            "whole executable entry points require an independent runtime composition proof",
        ));
    }
    let tree = crate::native_grammar_parser::parse_native("native-rust", source);
    if tree.has_error {
        return Err(failure(
            "the complete concrete syntax tree must have no errors",
        ));
    }
    let text = |node: &crate::native_grammar_parser::NativeNode| {
        source
            .get(node.start..node.end)
            .ok_or_else(|| failure("concrete node ranges must retain actual UTF8 source bytes"))
    };
    let mut owners = HashMap::new();
    for (node, _) in tree
        .children
        .iter()
        .filter(|(node, _)| node.term == "function_item")
    {
        let names = node
            .children
            .iter()
            .filter(|(_, field)| field.as_deref() == Some("name"))
            .collect::<Vec<_>>();
        if names.len() != 1 {
            return Err(failure("each concrete function must have one actual name"));
        }
        let name = text(&names[0].0)?.to_owned();
        let visibility = node
            .children
            .iter()
            .filter(|(child, _)| child.term == "visibility_modifier")
            .collect::<Vec<_>>();
        if visibility.len() > 1 || owners.contains_key(&name) {
            return Err(failure(
                "each source name and visibility has one complete concrete owner",
            ));
        }
        let exported = visibility
            .first()
            .map(|(node, _)| text(node))
            .transpose()?
            .is_some_and(|visibility| visibility == "pub");
        owners.insert(name, exported);
    }
    if owners.len() != program.declarations.len() {
        return Err(failure(
            "every checked declaration must have its actual whole-module concrete owner",
        ));
    }
    let mut signatures = Vec::new();
    for declaration in &program.declarations {
        let Decl::Fn(function) = declaration else {
            return Err(failure("only actual flat function owners are published"));
        };
        if !function.module_path.is_empty()
            || function.full_name != function.name
            || !owners.contains_key(&function.name)
        {
            return Err(failure("only actual flat function owners are published"));
        }
        if owners[&function.name] {
            signatures.push(External::Function {
                name: function.name.clone(),
                params: function
                    .params
                    .iter()
                    .map(|param| ExternalParam {
                        name: param.name.clone(),
                        ty: param.ty.clone(),
                    })
                    .collect(),
                ret: function.ret.clone(),
            });
        }
    }
    let mut emitted = crate::translation::emit_javascript::emit_javascript(&program)
        .map_err(|error| failure(&error.to_string()))?;
    let mut targets = HashMap::new();
    for (name, exported) in &owners {
        let mappings = emitted
            .mappings
            .iter()
            .filter(|mapping| mapping.kind == "function" && mapping.source == *name)
            .collect::<Vec<_>>();
        if mappings.len() != 1 || targets.contains_key(&mappings[0].target) {
            return Err(failure(
                "each actual checked function requires one distinct emitted target",
            ));
        }
        targets.insert(mappings[0].target.clone(), *exported);
    }
    let mut published = HashSet::new();
    let mut definitions = Vec::new();
    for definition in &emitted.definitions {
        let candidates = targets
            .keys()
            .filter(|target| definition.starts_with(&format!("function {target}(")))
            .collect::<Vec<_>>();
        if candidates.len() != 1 || !published.insert(candidates[0].clone()) {
            return Err(failure(
                "each emitted definition must have its actual ordered source mapping",
            ));
        }
        definitions.push(if targets[candidates[0]] {
            format!("export {definition}")
        } else {
            definition.clone()
        });
    }
    if published.len() != targets.len() {
        return Err(failure(
            "all checked function definitions must be published together",
        ));
    }
    emitted.definitions = definitions;
    Ok((emitted, signatures))
}
