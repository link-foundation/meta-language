//! Loading source-local matcher programs and checked command statements.

use std::collections::{HashMap, HashSet};

use super::super::compile::Compiler;
use super::super::lexical_plan::{self, LexicalRule, LexicalSource};
use super::super::program::Name;
use super::{Context, LoadResult, Resolved, load_in_context, refuse};
use crate::grammar::Grammar;

pub(super) fn configure_compiler(compiler: &mut Compiler<'_>, sources: &[LexicalSource]) {
    compiler.lexer_owners = sources
        .iter()
        .map(|source| {
            (
                source.plan.name.to_string(),
                source
                    .plan
                    .original_owners
                    .iter()
                    .map(ToString::to_string)
                    .collect(),
            )
        })
        .collect();
}

pub(super) fn is_elected(sources: &[LexicalSource], name: &str) -> bool {
    sources.iter().any(|source| {
        source
            .plan
            .bindings
            .iter()
            .any(|(_, public)| &**public == name)
    })
}

pub(super) fn default_lexer(_grammar: &Grammar, sources: &[LexicalSource]) -> Name {
    if sources.len() == 1 {
        sources[0].plan.name.clone()
    } else {
        Name::from("")
    }
}

pub(super) fn wrap_parser_owner(
    expression: super::super::program::Expr,
    grammar: &Grammar,
    sources: &[LexicalSource],
    name: &str,
) -> super::super::program::Expr {
    let family = default_lexer(grammar, sources);
    if let Some(source) = sources.iter().find(|source| source.plan.name == family)
        && let Some(owner) = source.plan.owners.get(name)
    {
        return super::super::program::Expr::LexicalScope {
            name: family,
            owner: owner.clone(),
            item: Box::new(expression),
        };
    }
    expression
}

pub(super) fn read_lexers(resolved: &Resolved) -> LoadResult<Vec<LexicalSource>> {
    let mut names = HashSet::new();
    let mut sources = Vec::new();
    for declaration in &resolved.lexers {
        if !names.insert(&declaration.name) {
            return refuse(
                "declaration",
                format!("duplicate lexer {}", declaration.name),
            );
        }
        let source = lexical_plan::read_source(declaration)?;
        for reference in source
            .plan
            .owners
            .keys()
            .chain(source.plan.bindings.iter().map(|(_, public)| public))
        {
            if !resolved.rules.contains(reference) {
                return refuse(
                    "declaration",
                    format!(
                        "undefined public lexical reference {reference} in lexer {}",
                        source.plan.name
                    ),
                );
            }
        }
        sources.push(source);
    }
    Ok(sources)
}

pub(super) fn load_lexers(
    parent: usize,
    sources: Vec<LexicalSource>,
    context: &mut Context<'_>,
) -> LoadResult<()> {
    for mut source in sources {
        let index = if source.grammar.rules().is_empty() {
            parent
        } else {
            load_in_context(&source.grammar, context)?
        };
        let matcher = &context.programs[index];
        let external = HashMap::new();
        let mut compiler = Compiler::new(
            &matcher.rule_index,
            &external,
            &source.modes,
            HashMap::new(),
        );
        for entry in &source.rules {
            let name = entry.text("name").expect("validated source token name");
            let obligations =
                lexical_plan::selected_obligations(&source.plan, &source.grammar, name)?;
            let action = compiler.statements(
                lexical_plan::action(entry),
                "action",
                &format!("lexer {}:{name}", source.plan.name),
                None,
            )?;
            source.plan.rules.push(LexicalRule {
                name: Name::from(name),
                token_type: Name::from(entry.text("type").unwrap_or_default()),
                mode: Name::from(entry.text("mode").unwrap_or_default()),
                channel: Name::from(entry.text("channel").unwrap_or_default()),
                disposition: Name::from(entry.text("disposition").unwrap_or_default()),
                action,
                matcher: matcher.rule_index[name],
                obligations,
            });
        }
        source.plan.program = index;
        context.programs[parent]
            .lexers
            .insert(source.plan.name.clone(), source.plan);
    }
    Ok(())
}
