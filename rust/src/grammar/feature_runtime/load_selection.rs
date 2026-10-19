//! Grammar source closure transport, separate from runtime selection policies.
use super::{Context, LoadResult, load_in_context};

pub(super) fn load_embedded_grammar_sources(
    embedded: Vec<String>,
    selected: Vec<String>,
    context: &mut Context<'_>,
) -> LoadResult<()> {
    for language in embedded {
        if context.languages.contains_key(&language) {
            continue;
        }
        context.languages.insert(language.clone(), usize::MAX);
        let grammar = context.resolve(&language, "embed")?;
        let loaded = load_in_context(&grammar, context)?;
        context.languages.insert(language, loaded);
    }
    for language in selected {
        if context.languages.contains_key(&language) {
            continue;
        }
        let Some(grammar) = context.resolver.and_then(|resolve| resolve(&language)) else {
            continue;
        };
        context.languages.insert(language.clone(), usize::MAX);
        let capture_origins = context.capture_origins;
        context.capture_origins = true;
        let loaded = load_in_context(&grammar, context);
        context.capture_origins = capture_origins;
        let loaded = loaded?;
        context.languages.insert(language, loaded);
    }
    Ok(())
}
