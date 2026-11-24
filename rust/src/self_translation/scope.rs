//! Retrying declaration scopes while retaining the dependency binder's results.
//! Mirrors translateBindingGroups and collectLiteralBindings in JavaScript.
use super::{
    Context, Group, GroupKind, Translated, family, translate_group, translate_group_bound,
};
use crate::decorators::DecoratorSet;
use crate::translation::frontend_rules::{
    accept_binding_scope, accept_literal_binding, accept_module_binding_scope, find_binding_run_end,
};
use crate::translation::javascript::{
    ModuleBindingScope, ModuleContext, parse_javascript_bound_in_with_module_scope,
};
use crate::translation::surface::{SEffect, SExpr, SNode};
use std::collections::{HashMap, HashSet};

pub(super) struct BindingScopeContext {
    pub(super) utf16_strings: bool,
    pub(super) module_scope_known: bool,
    pub(super) module_binding_names: Vec<String>,
}

// Only the collection/provenance adapter is host code; run boundaries and
// retry eligibility use the same JavaScript-generated frontend decisions.
pub(super) fn translate_binding_groups<'a>(
    groups: &[Group<'a>],
    source: &'a str,
    from: &str,
    to: &str,
    decorators: &DecoratorSet,
    initial: &[Translated],
    context: BindingScopeContext,
) -> Vec<(Group<'a>, Translated)> {
    let BindingScopeContext {
        utf16_strings,
        module_scope_known,
        module_binding_names,
    } = context;
    let module_scope = ModuleBindingScope {
        names: module_binding_names,
        known: module_scope_known,
    };
    let bind = family(from) == "JavaScript" && family(to) == "Rust";
    if bind && !module_scope_known {
        return groups
            .iter()
            .cloned()
            .zip(initial.iter().cloned())
            .collect();
    }
    let (literals, literal_groups) = collect_literal_bindings(
        groups,
        bind,
        from,
        to,
        decorators,
        utf16_strings,
        Some(&module_scope),
    );
    let terms: Vec<String> = groups
        .iter()
        .enumerate()
        .map(|(index, group)| match &group.kind {
            GroupKind::Item { term, .. } if !literal_groups.contains(&index) => term.clone(),
            _ => String::new(),
        })
        .collect();
    let mut results = Vec::new();
    let mut index = 0;
    while index < groups.len() {
        #[allow(
            clippy::cast_precision_loss,
            clippy::cast_possible_truncation,
            clippy::cast_sign_loss
        )]
        let end = if bind {
            find_binding_run_end(&terms, index as f64) as usize
        } else {
            index + 1
        };
        let run = &groups[index..end];
        let isolated: Vec<_> = run
            .iter()
            .enumerate()
            .map(|(offset, group)| (group.clone(), initial[index + offset].clone()))
            .collect();
        let statuses: Vec<String> = isolated
            .iter()
            .map(|(_, out)| out.status.to_owned())
            .collect();
        let reasons: Vec<String> = isolated
            .iter()
            .map(|(_, out)| out.reason.clone().unwrap_or_default())
            .collect();
        if accept_binding_scope(&statuses, &reasons) {
            let items: Vec<_> = run.iter().flat_map(|group| group.items.clone()).collect();
            let first = items.first().expect("a binding run has source items");
            let last = items.last().expect("a binding run has source items");
            let GroupKind::Item { term, .. } = &run[0].kind else {
                unreachable!("only item runs can combine")
            };
            let combined = Group {
                kind: GroupKind::Item {
                    text: source[first.start..last.end].to_owned(),
                    term: term.clone(),
                },
                items,
                after: run.last().expect("a binding run is nonempty").after,
            };
            let out = translate_group_bound(
                &combined,
                from,
                to,
                decorators,
                &Context {
                    utf16_strings,
                    module_binding_scope: Some(module_scope.clone()),
                    ..Context::default()
                },
                &literals,
            );
            if out.status == "translated" {
                results.push((combined, out));
            } else {
                results.extend(isolated);
            }
        } else {
            results.extend(isolated);
        }
        index = end;
    }
    let module_terms: Vec<String> = groups
        .iter()
        .map(|group| match &group.kind {
            GroupKind::Item { term, .. } => term.clone(),
            GroupKind::Comment { .. } => String::new(),
            _ => "blocked".to_owned(),
        })
        .collect();
    if bind
        && accept_module_binding_scope(
            &module_terms,
            results.iter().any(|(_, out)| out.status == "carried"),
        )
    {
        let items: Vec<_> = groups
            .iter()
            .flat_map(|group| group.items.clone())
            .collect();
        let first = items.first().expect("a declaration module has items");
        let last = items.last().expect("a declaration module has items");
        let term = match &groups[0].kind {
            GroupKind::Item { term, .. } | GroupKind::Comment { term, .. } => term.clone(),
            _ => unreachable!("provenance and carried groups are not module declarations"),
        };
        let combined = Group {
            kind: GroupKind::Item {
                text: source[first.start..last.end].to_owned(),
                term,
            },
            items,
            after: groups
                .last()
                .expect("a declaration module is nonempty")
                .after,
        };
        let out = translate_group_bound(
            &combined,
            from,
            to,
            decorators,
            &Context {
                utf16_strings,
                module_binding_scope: Some(module_scope),
                ..Context::default()
            },
            &literals,
        );
        if out.status == "translated" {
            return vec![(combined, out)];
        }
    }
    results
}

pub(super) fn collect_literal_bindings(
    groups: &[Group<'_>],
    bind: bool,
    from: &str,
    to: &str,
    decorators: &DecoratorSet,
    utf16_strings: bool,
    module_scope: Option<&ModuleBindingScope>,
) -> (HashMap<String, SExpr>, HashSet<usize>) {
    let mut literals = HashMap::new();
    let mut literal_groups = HashSet::new();
    if bind {
        for (index, group) in groups.iter().enumerate() {
            let GroupKind::Item { text, .. } = &group.kind else {
                continue;
            };
            let erased = (from == "TypeScript").then(|| super::project_type_script_typing(text));
            let source = match &erased {
                Some(Ok(erased)) => erased.as_str(),
                Some(Err(_)) => continue,
                None => text,
            };
            let Ok(parsed) = parse_javascript_bound_in_with_module_scope(
                source,
                &ModuleContext::default(),
                &HashMap::new(),
                family(to) == "Rust",
                module_scope,
            ) else {
                continue;
            };
            let Some(main) = &parsed.main else {
                continue;
            };
            let Some(SEffect::Let {
                name,
                value,
                constant,
                ..
            }) = main.effects.first()
            else {
                continue;
            };
            let kind = match &value.node {
                SNode::Num { .. } => "num",
                SNode::Bool { .. } => "bool",
                SNode::Str { .. } => "str",
                _ => "",
            };
            #[allow(clippy::cast_precision_loss)]
            if accept_literal_binding(
                kind,
                *constant,
                main.effects.len() as f64,
                parsed.items.len() as f64,
            ) {
                if translate_group(
                    group,
                    from,
                    to,
                    decorators,
                    &Context {
                        utf16_strings,
                        module_binding_scope: module_scope.cloned(),
                        ..Context::default()
                    },
                )
                .status
                    != "translated"
                {
                    continue;
                }
                literals.insert(name.clone(), value.clone());
                literal_groups.insert(index);
            }
        }
    }
    (literals, literal_groups)
}
