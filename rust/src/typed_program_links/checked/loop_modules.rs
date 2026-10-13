//! Checked namespace binding adapter for owned scalar loop graph edits.
use super::{Result, error, expression, project_type};
use crate::translation::ir::{Decl, FnDecl, Item, Program};
use crate::translation::source_loops::validate_source_loop;
use crate::translation::surface::{SFn, SItem, SModule, SParam};
use std::collections::HashSet;

pub(super) fn project_function(function: &FnDecl, program: &Program) -> Result<SItem> {
    Ok(SItem::Fn(SFn {
        name: function.name.clone(),
        params: function
            .params
            .iter()
            .map(|parameter| -> Result<SParam> {
                Ok(SParam {
                    name: parameter.name.clone(),
                    ty: Some(project_type(&parameter.ty)?),
                    default_value: parameter.default_value.clone(),
                    guard: parameter.guard.clone(),
                    span: None,
                    rocq_type: None,
                })
            })
            .collect::<Result<_>>()?,
        ret: Some(project_type(&function.ret)?),
        body: expression(&function.body, program)?,
        span: function.span,
        generated: function.generated,
        source_signature: function.source_signature.clone().map(Box::new),
        source_primitive_owners: function.source_primitive_owners.clone(),
        source_empty_option: function.source_empty_option.clone().map(Box::new),
        source_loop: function.source_loop.clone().map(Box::new),
    }))
}

pub(super) fn project_module(
    item: &Item,
    program: &Program,
    path: &[String],
    seen: &mut HashSet<String>,
) -> Result<SItem> {
    let Item::Module {
        name,
        full_name,
        items,
        span,
    } = item
    else {
        return Err(error("an owned loop module needs its actual module item"));
    };
    let actual_path = path
        .iter()
        .cloned()
        .chain(std::iter::once(name.clone()))
        .collect::<Vec<_>>();
    if name.is_empty() || actual_path.join(".") != *full_name || items.is_empty() {
        return Err(error(
            "the owned loop module has an inconsistent namespace binding",
        ));
    }
    let mut projected = Vec::new();
    for child in items {
        if let Item::Module { .. } = child {
            projected.push(project_module(child, program, &actual_path, seen)?);
            continue;
        }
        let Item::Decl { full_name } = child else {
            unreachable!("program items are declarations or modules");
        };
        let Some(Decl::Fn(function)) = program.declaration(full_name) else {
            return Err(error(
                "the Rust typed links adapter does not yet recheck module items",
            ));
        };
        if function.source_loop.is_none() {
            return Err(error(
                "the Rust typed links adapter does not yet recheck module items",
            ));
        }
        if function.module_path != actual_path
            || function.full_name != *full_name
            || actual_path
                .iter()
                .chain(std::iter::once(&function.name))
                .cloned()
                .collect::<Vec<_>>()
                .join(".")
                != *full_name
            || !seen.insert(full_name.clone())
        {
            return Err(error(
                "the owned loop declaration disagrees with its actual namespace binding",
            ));
        }
        validate_source_loop(function, program.source_language.as_str())
            .map_err(|failure| error(failure.to_string()))?;
        projected.push(project_function(function, program)?);
    }
    Ok(SItem::Module(SModule {
        name: name.clone(),
        items: projected,
        span: *span,
    }))
}
