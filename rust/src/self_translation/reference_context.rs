//! Whole-module host context over existing shared reference selection rules.
use super::{Group, GroupKind, SelfTranslationError, SelfTranslationOptions, family};
use crate::translation::javascript::{
    ModuleContext, parse_javascript_in, requires_javascript_reference_semantics,
};
use crate::translation::semantic_input_rules::{
    refine_self_translation_module_references, select_self_translation_module_references,
};
use crate::translation::types::Type;

fn has_reference(types: Vec<&Type>) -> bool {
    let mut pending = types;
    while let Some(value) = pending.pop() {
        if value.is_reference() {
            return true;
        }
        pending.extend(value.children());
    }
    false
}

pub(super) fn select(
    source: &str,
    language: &str,
    groups: &[Group<'_>],
    options: &SelfTranslationOptions,
) -> Result<bool, SelfTranslationError> {
    let javascript = family(language) == "JavaScript";
    let context = ModuleContext {
        module_directory: Some(options.module_directory.clone()),
        ..ModuleContext::default()
    };
    let lexical = !options.reference_semantics
        && javascript
        && requires_javascript_reference_semantics(source).map_err(|error| {
            SelfTranslationError {
                message: error.to_string(),
            }
        })?;
    let mut selected =
        select_self_translation_module_references(options.reference_semantics, javascript, lexical);
    if !options.reference_semantics && selected && javascript {
        if let Ok(surface) = parse_javascript_in(source, &context) {
            // Complete ordinary frontend ownership rewrites may remove all
            // reference triggers; a refused complete parse keeps lexical facts.
            let reference = surface
                .reference_semantics
                .as_ref()
                .and_then(|value| value.as_boolean())
                == Some(true);
            selected = refine_self_translation_module_references(
                selected,
                options.reference_semantics,
                javascript,
                true,
                reference,
            );
        }
    }
    if !selected {
        for external in options.imports.values().flatten() {
            let (mut params, result) = external.signature();
            params.push(result);
            if has_reference(params) {
                selected = true;
                break;
            }
        }
    }
    if !selected && javascript {
        for group in groups {
            let GroupKind::Item { text, .. } = &group.kind else {
                continue;
            };
            let erased =
                (language == "TypeScript").then(|| super::project_type_script_typing(text));
            let source = match &erased {
                Some(Ok(source)) => source.as_str(),
                Some(Err(_)) => continue,
                None => text,
            };
            if let Ok(surface) = parse_javascript_in(source, &context) {
                if surface
                    .reference_semantics
                    .as_ref()
                    .and_then(|value| value.as_boolean())
                    == Some(true)
                {
                    return Ok(true);
                }
            }
        }
    }
    Ok(selected)
}
