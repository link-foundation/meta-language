//! Source token/AST transport into the shared, self-translated lexer plan.
//! Selection, literals, commands and bindings are decided by the JavaScript
//! kernels. This adapter copies their selected original fields into Links.

use std::collections::HashSet;

use crate::grammar::feature_runtime::lexical_kernel as kernel;
use crate::grammar::{
    FeatureExpr, FeatureForm, FieldValue, Grammar, GrammarExpr, GrammarLexer, GrammarRule, RuleKind,
};

#[derive(Clone, Debug)]
pub(super) struct Section {
    pub(super) name: String,
    pub(super) kind: String,
    pub(super) token_vocabulary: String,
    pub(super) channels: Vec<String>,
    pub(super) types: Vec<String>,
    pub(super) modes: Vec<String>,
    pub(super) imports: Vec<String>,
    pub(super) unsupported_options: Vec<String>,
}

#[derive(Clone, Debug)]
pub(super) struct Obligation {
    pub(super) category: String,
    pub(super) source: String,
}

#[derive(Clone, Debug)]
pub(super) struct RuleFacts {
    pub(super) section: Option<usize>,
    pub(super) mode: String,
    pub(super) command: String,
    pub(super) retype: String,
    pub(super) obligations: Vec<Obligation>,
}

fn form(head: &str, fields: Vec<FieldValue>) -> GrammarExpr {
    GrammarExpr::feature(FeatureExpr::Form(FeatureForm::new(head, fields)))
}
fn name(value: &str) -> FieldValue {
    FieldValue::Name(value.to_owned())
}
fn text(value: &str) -> FieldValue {
    FieldValue::Text(value.to_owned())
}
fn reference(value: &str) -> FieldValue {
    FieldValue::Expression(GrammarExpr::NonTerminal(value.to_owned()))
}
fn obligation(owner: &str, category: &str, source: &str) -> GrammarExpr {
    form(
        "lexicalObligation",
        vec![name(owner), name(category), text(source)],
    )
}

fn visit(expr: &GrammarExpr, read: &mut impl FnMut(&GrammarExpr)) {
    read(expr);
    match expr {
        GrammarExpr::Sequence(items)
        | GrammarExpr::Choice {
            alternatives: items,
            ..
        } => {
            for item in items {
                visit(item, read);
            }
        }
        GrammarExpr::Optional(item)
        | GrammarExpr::ZeroOrMore(item)
        | GrammarExpr::OneOrMore(item)
        | GrammarExpr::And(item)
        | GrammarExpr::Not(item)
        | GrammarExpr::Repeat { expr: item, .. }
        | GrammarExpr::Capture { expr: item, .. } => visit(item, read),
        GrammarExpr::Feature(feature) => {
            if let FeatureExpr::Form(value) = &**feature {
                if let Some(item) = value.expression("item") {
                    visit(item, read);
                }
                for item in value.expressions("items") {
                    visit(item, read);
                }
            }
        }
        _ => {}
    }
}

fn token_entry(rule: &GrammarRule, facts: &RuleFacts, plan: &[String]) -> GrammarExpr {
    let actions = plan[3..]
        .chunks_exact(2)
        .filter(|pair| pair[0] != "unsupported")
        .map(|pair| {
            FeatureForm::new(
                pair[0].as_str(),
                if pair[0] == "popMode" {
                    Vec::new()
                } else {
                    vec![name(&pair[1])]
                },
            )
        })
        .collect();
    form(
        "lexicalRule",
        vec![
            name(&rule.name),
            name(&plan[0]),
            name(&facts.mode),
            name(&plan[1]),
            name(&plan[2]),
            FieldValue::Expression(rule.expr.clone()),
            FieldValue::Block(Some(actions)),
        ],
    )
}

pub(super) fn declarations(
    sections: &[Section],
    facts: &[RuleFacts],
    rules: &[(GrammarRule, Option<String>)],
) -> Vec<GrammarLexer> {
    let kinds = sections
        .iter()
        .map(|section| section.kind.clone())
        .collect::<Vec<_>>();
    let names = sections
        .iter()
        .map(|section| section.name.clone())
        .collect::<Vec<_>>();
    let vocabularies = sections
        .iter()
        .map(|section| section.token_vocabulary.clone())
        .collect::<Vec<_>>();
    let rule_sections = facts
        .iter()
        .map(|fact| fact.section.map_or(-1.0, |index| kernel::number(index)))
        .collect::<Vec<_>>();
    let rule_kinds = rules
        .iter()
        .map(|(rule, _)| rule.kind.as_str().to_owned())
        .collect::<Vec<_>>();
    let mut lexers = Vec::new();
    for (section_index, section) in sections
        .iter()
        .enumerate()
        .filter(|(_, section)| section.kind != "parser")
    {
        let positions = kernel::section_positions(
            section_index,
            &kinds,
            &names,
            &vocabularies,
            &rule_sections,
            &rule_kinds,
        );
        let separator = positions
            .iter()
            .position(|&position| position.to_bits() == (-1.0_f64).to_bits())
            .expect("the shared section plan contains its separator");
        let lexical = positions[..separator]
            .iter()
            .map(|&position| kernel::position(position))
            .collect::<Vec<_>>();
        let owners = positions[separator + 1..]
            .iter()
            .map(|&position| kernel::position(position))
            .collect::<Vec<_>>();
        if lexical.is_empty() && owners.is_empty() {
            continue;
        }
        let mut entries = Vec::new();
        for (head, values) in [
            ("lexicalChannel", &section.channels),
            ("lexicalType", &section.types),
            ("lexicalMode", &section.modes),
        ] {
            for value in values {
                entries.push(form(head, vec![name(value)]));
            }
        }
        for &owner in &owners {
            let rule = &rules[owner].0;
            entries.push(form(
                "lexicalOwner",
                vec![name(&rule.name), reference(&rule.name)],
            ));
        }
        for source in &section.imports {
            entries.push(obligation("", "import", source));
        }
        for source in &section.unsupported_options {
            entries.push(obligation("", "option", source));
        }
        for &owner in &owners {
            for item in &facts[owner].obligations {
                entries.push(obligation(
                    &rules[owner].0.name,
                    &item.category,
                    &item.source,
                ));
            }
        }
        let (mut literals, mut references) = (Vec::new(), Vec::new());
        for &owner in &owners {
            visit(&rules[owner].0.expr, &mut |expr| match expr {
                GrammarExpr::Terminal(value) if !value.is_empty() => {
                    if !literals.contains(value) {
                        literals.push(value.clone());
                    }
                }
                GrammarExpr::NonTerminal(value) => {
                    if !references.contains(value) {
                        references.push(value.clone());
                    }
                }
                _ => {}
            });
        }
        let local_kinds = lexical
            .iter()
            .map(|&index| rules[index].0.kind.as_str().to_owned())
            .collect::<Vec<_>>();
        let local_values = lexical
            .iter()
            .map(|&index| match &rules[index].0.expr {
                GrammarExpr::Terminal(value) => value.clone(),
                _ => String::new(),
            })
            .collect::<Vec<_>>();
        let local_literals = lexical
            .iter()
            .map(|&index| matches!(&rules[index].0.expr, GrammarExpr::Terminal(_)))
            .collect::<Vec<_>>();
        let mut implicit = 0;
        for value in literals {
            let selected = kernel::literal_rule(
                &value,
                &section.kind,
                &local_kinds,
                &local_values,
                &local_literals,
            );
            if selected >= 0.0 {
                entries.push(form(
                    "lexicalLiteral",
                    vec![
                        text(&value),
                        name(&rules[lexical[kernel::position(selected)]].0.name),
                    ],
                ));
            } else if selected.to_bits() == (-2.0_f64).to_bits() {
                entries.push(obligation("", "literal", &value));
            } else {
                let token = format!("@literal:{implicit}");
                implicit += 1;
                entries.push(form("lexicalLiteral", vec![text(&value), name(&token)]));
                entries.push(form(
                    "lexicalRule",
                    vec![
                        name(&token),
                        name(&token),
                        name("default"),
                        name("default"),
                        name("emit"),
                        FieldValue::Expression(GrammarExpr::Terminal(value)),
                        FieldValue::Block(Some(Vec::new())),
                    ],
                ));
            }
        }
        let (mut token_names, mut legacy_types, mut emitted_types, mut dispositions) =
            (Vec::new(), Vec::new(), Vec::new(), Vec::new());
        for index in lexical {
            let rule = &rules[index].0;
            let fact = &facts[index];
            for item in &fact.obligations {
                entries.push(obligation(&rule.name, &item.category, &item.source));
            }
            if rule.kind == RuleKind::Silent {
                entries.push(form(
                    "lexicalFragment",
                    vec![name(&rule.name), FieldValue::Expression(rule.expr.clone())],
                ));
                continue;
            }
            let plan = kernel::command_plan(&fact.command, &rule.name);
            token_names.push(rule.name.clone());
            legacy_types.push(fact.retype.clone());
            emitted_types.push(plan[0].clone());
            dispositions.push(plan[2].clone());
            entries.push(token_entry(rule, fact, &plan));
            for pair in plan[3..]
                .chunks_exact(2)
                .filter(|pair| pair[0] == "unsupported")
            {
                entries.push(obligation(&rule.name, "command", &pair[1]));
            }
        }
        for binding in kernel::binding_names(
            &section.types,
            &references,
            &token_names,
            &legacy_types,
            &emitted_types,
            &dispositions,
        ) {
            entries.push(form(
                "lexicalBinding",
                vec![name(&binding), reference(&binding)],
            ));
        }
        lexers.push(GrammarLexer {
            name: section.name.clone(),
            expression: GrammarExpr::Sequence(entries),
        });
    }
    lexers
}

pub(super) fn complete_bindings(grammar: &mut Grammar, lexers: &[GrammarLexer]) {
    let mut names = grammar
        .rules()
        .iter()
        .map(|rule| rule.name.clone())
        .collect::<HashSet<_>>();
    for lexer in lexers {
        let GrammarExpr::Sequence(entries) = &lexer.expression else {
            continue;
        };
        let forms = entries
            .iter()
            .filter_map(|entry| match entry {
                GrammarExpr::Feature(feature) => match &**feature {
                    FeatureExpr::Form(value) => Some(value),
                    _ => None,
                },
                _ => None,
            })
            .collect::<Vec<_>>();
        for entry in &forms {
            if kernel::entry_form(&entry.head) != 2 {
                continue;
            }
            let Some(GrammarExpr::NonTerminal(target)) = entry.expression("item") else {
                continue;
            };
            if names.contains(target) {
                continue;
            }
            let token = entry.text("name").unwrap_or_default();
            let sources = forms
                .iter()
                .filter(|candidate| {
                    kernel::entry_form(&candidate.head) == 7
                        && candidate.text("disposition") == Some("emit")
                        && candidate.text("type") == Some(token)
                })
                .map(|candidate| {
                    GrammarExpr::NonTerminal(candidate.text("name").unwrap_or_default().to_owned())
                })
                .collect::<Vec<_>>();
            let expression = match sources.len() {
                0 => GrammarExpr::not(GrammarExpr::Empty),
                1 => sources[0].clone(),
                _ => GrammarExpr::choice(false, sources),
            };
            grammar.add_rule(
                GrammarRule::new(target.as_str(), expression)
                    .with_kind(RuleKind::Token)
                    .with_doc(format!("lexer {} token {token}", lexer.name)),
            );
            names.insert(target.clone());
        }
    }
}
