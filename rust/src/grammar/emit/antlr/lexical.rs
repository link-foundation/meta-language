//! Native field transport for the shared source-local lexer emission plan.

use std::collections::HashMap;

use super::super::structural::NamePlan;
use super::{FORMAT, plan_doc};
use crate::grammar::emit::{GrammarEmitError, unsupported_error};
use crate::grammar::feature_runtime::lexical_kernel as kernel;
use crate::grammar::interchange::render_links_expression;
use crate::grammar::{FeatureExpr, FeatureForm, Grammar, GrammarExpr};

#[derive(Clone, Debug)]
pub(super) struct RuleEmission {
    pub(super) expression: GrammarExpr,
    pub(super) command: String,
}

#[derive(Debug)]
pub(super) struct LexicalEmission {
    pub(super) rules: HashMap<String, RuleEmission>,
    pub(super) order: Vec<usize>,
}

fn refuse(reason: impl std::fmt::Display) -> GrammarEmitError {
    unsupported_error(FORMAT, format!("source-free lexical commands: {reason}"))
}

fn form_of(expr: &GrammarExpr) -> Option<&FeatureForm> {
    match expr {
        GrammarExpr::Feature(feature) => match &**feature {
            FeatureExpr::Form(form) => Some(form),
            _ => None,
        },
        _ => None,
    }
}

fn command_of(entry: &FeatureForm) -> Result<String, GrammarEmitError> {
    let actions = entry.operations("action").unwrap_or_default();
    let operations = actions
        .iter()
        .map(|action| action.head.clone())
        .collect::<Vec<_>>();
    let modes = actions
        .iter()
        .map(|action| action.text("mode").unwrap_or_default().to_owned())
        .collect::<Vec<_>>();
    let result = kernel::command_emission(
        entry.text("name").unwrap_or_default(),
        entry.text("type").unwrap_or_default(),
        entry.text("channel").unwrap_or_default(),
        entry.text("disposition").unwrap_or_default(),
        &operations,
        &modes,
    );
    let [command] = result.as_slice() else {
        return Err(refuse(format!(
            "cannot replay the declaration of {:?}",
            entry.text("name").unwrap_or_default()
        )));
    };
    Ok(command.clone())
}

pub(super) fn source_free_lexer_emission(
    grammar: &Grammar,
    names: &NamePlan,
) -> Result<Option<LexicalEmission>, GrammarEmitError> {
    if grammar.source_format() != Some(FORMAT) {
        return Ok(None);
    }
    let lexers = &grammar.declarations().lexers;
    let mut needs_command_recovery = false;
    let mut needs_source_body = false;
    for lexer in lexers {
        let GrammarExpr::Sequence(entries) = &lexer.expression else {
            return Err(refuse("a lexer needs an ordered declaration list"));
        };
        for entry in entries.iter().filter_map(form_of) {
            if entry.head != "lexicalRule" && entry.head != "lexicalFragment" {
                continue;
            }
            let rule = grammar.rule(entry.text("name").unwrap_or_default());
            if let Some(rule) = rule {
                let Some(item) = entry.expression("item") else {
                    return Err(refuse("invalid lexical declaration descriptors"));
                };
                if render_links_expression(rule.expr()) != render_links_expression(item) {
                    needs_source_body = true;
                }
            }
            if entry.head != "lexicalRule" {
                continue;
            }
            if rule.is_some_and(|rule| plan_doc(rule.doc(), true).command.is_some()) {
                continue;
            }
            if !command_of(entry)?.is_empty() {
                needs_command_recovery = true;
            }
        }
    }
    if !needs_command_recovery && !needs_source_body {
        return Ok(None);
    }

    // Reuse only the native declaration validator/lowering projection. Its
    // matcher programs and compiled lexer state never leave the runtime.
    let plans = lexers
        .iter()
        .map(kernel::declaration_emission_facts)
        .collect::<Result<Vec<_>, _>>();
    let plans = match plans {
        Ok(plans) => plans,
        Err(error) => {
            if kernel::emission_disposition(
                needs_command_recovery,
                "invalid lexical declaration descriptors",
            ) == 0
            {
                return Ok(None);
            }
            return Err(refuse(error));
        }
    };
    let entries = lexers
        .iter()
        .flat_map(|lexer| {
            let GrammarExpr::Sequence(entries) = &lexer.expression else {
                unreachable!("the ordered source was inspected before validation")
            };
            entries.iter().map(|entry| {
                form_of(entry).expect("the actual declaration validator checked each feature form")
            })
        })
        .collect::<Vec<_>>();
    let rules = grammar.rules();
    let indices = rules
        .iter()
        .enumerate()
        .map(|(index, rule)| (rule.name.as_str(), index))
        .collect::<HashMap<_, _>>();
    let locals = entries
        .iter()
        .copied()
        .filter(|entry| entry.head == "lexicalRule" || entry.head == "lexicalFragment")
        .collect::<Vec<_>>();
    let obligations = plans.iter().map(|plan| plan.obligations).sum();
    let modes = plans
        .iter()
        .flat_map(|plan| plan.modes.iter().cloned())
        .collect::<Vec<_>>();
    let channels = plans
        .iter()
        .flat_map(|plan| plan.channels.iter().cloned())
        .collect::<Vec<_>>();
    let types = plans.iter().map(|plan| plan.types).sum();
    let mut columns: [Vec<String>; 9] = std::array::from_fn(|_| Vec::new());
    for entry in &entries {
        let name = entry.text("name").unwrap_or_default();
        columns[0].push(entry.head.clone());
        columns[1].push(name.to_owned());
        columns[2].push(entry.text("mode").unwrap_or_default().to_owned());
        columns[3].push(match entry.expression("item") {
            Some(GrammarExpr::NonTerminal(target)) => target.clone(),
            _ => String::new(),
        });
        let index = indices.get(name).copied();
        columns[4].push(
            index
                .map_or("", |index| rules[index].kind.as_str())
                .to_owned(),
        );
        columns[5].push(index.map_or("", |index| names.rule_name(index)).to_owned());
    }
    for rule in rules {
        columns[6].push(rule.name.clone());
        columns[7].push(rule.kind.as_str().to_owned());
    }
    for entry in &locals {
        let mut references = Vec::new();
        super::super::structural::collect_references(
            entry.expression("item").expect("validated lexical item"),
            &mut references,
        );
        columns[8].extend(references.into_iter().map(str::to_owned));
    }
    let issue = kernel::emission_issue(
        lexers.len(),
        obligations,
        &modes,
        &channels,
        types,
        &columns,
    );
    match kernel::emission_disposition(needs_command_recovery, &issue) {
        0 => return Ok(None),
        2 => return Err(refuse(issue)),
        _ => {}
    }
    let mut selected = HashMap::new();
    let mut order = Vec::new();
    for entry in locals {
        let name = entry
            .text("name")
            .expect("the shared declaration validator checked its source name");
        let index = indices[name];
        let command = if entry.head == "lexicalRule" {
            command_of(entry)?
        } else {
            String::new()
        };
        if let Some(original) = plan_doc(rules[index].doc(), true).command
            && kernel::command_plan(&original, name) != kernel::command_plan(&command, name)
        {
            return Err(refuse(
                "rule documentation disagrees with its actual lexical declaration",
            ));
        }
        selected.insert(
            name.to_owned(),
            RuleEmission {
                expression: entry
                    .expression("item")
                    .expect("validated lexical item")
                    .clone(),
                command,
            },
        );
        order.push(index);
    }
    Ok(Some(LexicalEmission {
        rules: selected,
        order,
    }))
}
