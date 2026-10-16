//! Structural transport for the shared literal-pool extension plan.

use std::collections::BTreeMap;

use super::GrammarRoundTripError;
use crate::grammar::feature_runtime::lexical_kernel as kernel;
use crate::grammar::{FeatureExpr, FeatureForm, FieldValue, Grammar, GrammarExpr, GrammarLexer};

fn as_form(expr: &GrammarExpr) -> Option<&FeatureForm> {
    match expr {
        GrammarExpr::Feature(value) => match &**value {
            FeatureExpr::Form(form) => Some(form),
            _ => None,
        },
        _ => None,
    }
}

fn literal_values(expr: &GrammarExpr, values: &mut Vec<String>) {
    match expr {
        GrammarExpr::Terminal(value) if !value.is_empty() => values.push(value.clone()),
        GrammarExpr::Sequence(items)
        | GrammarExpr::Choice {
            alternatives: items,
            ..
        } => {
            for item in items {
                literal_values(item, values);
            }
        }
        GrammarExpr::Optional(item)
        | GrammarExpr::ZeroOrMore(item)
        | GrammarExpr::OneOrMore(item)
        | GrammarExpr::And(item)
        | GrammarExpr::Not(item)
        | GrammarExpr::Repeat { expr: item, .. }
        | GrammarExpr::Capture { expr: item, .. } => literal_values(item, values),
        GrammarExpr::Feature(_) => {
            if let Some(form) = as_form(expr) {
                if let Some(item) = form.expression("item") {
                    literal_values(item, values);
                }
                for item in form.expressions("items") {
                    literal_values(item, values);
                }
            }
        }
        _ => {}
    }
}

fn form(head: &str, fields: Vec<FieldValue>) -> GrammarExpr {
    GrammarExpr::feature(FeatureExpr::Form(FeatureForm::new(head, fields)))
}

fn named(value: &str) -> FieldValue {
    FieldValue::Name(value.to_owned())
}

pub(super) fn extend_literal(
    grammar: &Grammar,
    lexer: &GrammarLexer,
    owner: &str,
    value: &str,
) -> Result<GrammarLexer, GrammarRoundTripError> {
    let failure = || {
        GrammarRoundTripError::LexicalDeclaration(format!(
            "invalid lexical literal extension in lexer {}",
            lexer.name
        ))
    };
    let GrammarExpr::Sequence(entries) = &lexer.expression else {
        return Err(failure());
    };
    let mut columns: [Vec<String>; 6] = std::array::from_fn(|_| Vec::new());
    let mut parser_literals = Vec::new();
    for entry in entries {
        let entry = as_form(entry).ok_or_else(failure)?;
        columns[0].push(entry.head.clone());
        columns[1].push(entry.text("name").unwrap_or_default().to_owned());
        columns[2].push(entry.text("value").unwrap_or_default().to_owned());
        columns[3].push(match entry.expression("item") {
            Some(GrammarExpr::NonTerminal(target)) => target.clone(),
            _ => String::new(),
        });
        columns[4].push(match entry.expression("item") {
            Some(GrammarExpr::Terminal(_)) => "literal".to_owned(),
            _ => String::new(),
        });
        columns[5].push(match entry.expression("item") {
            Some(GrammarExpr::Terminal(value)) => value.clone(),
            _ => String::new(),
        });
        if kernel::entry_form(&entry.head) == 1
            && let Some(GrammarExpr::NonTerminal(target)) = entry.expression("item")
            && let Some(rule) = grammar.rule(target)
        {
            literal_values(&rule.expr, &mut parser_literals);
        }
    }
    let plan = kernel::literal_extension(owner, value, &columns, &parser_literals);
    if plan[0] < 0.0 {
        return Err(failure());
    }
    if plan[0] == 0.0 {
        return Ok(lexer.clone());
    }
    let action = kernel::position(plan[0]);
    let insertion = kernel::position(plan[1]);
    let identity = if action == 2 {
        as_form(&entries[kernel::position(plan[2])])
            .and_then(|entry| entry.text("name"))
            .ok_or_else(failure)?
            .to_owned()
    } else if action == 1 {
        format!("@literal:{}", kernel::position(plan[2]))
    } else {
        String::new()
    };
    let added = match action {
        1 | 2 => {
            let mut added = vec![form(
                "lexicalLiteral",
                vec![FieldValue::Text(value.to_owned()), named(&identity)],
            )];
            if action == 1 {
                added.push(form(
                    "lexicalRule",
                    vec![
                        named(&identity),
                        named(&identity),
                        named("default"),
                        named("default"),
                        named("emit"),
                        FieldValue::Expression(GrammarExpr::Terminal(value.to_owned())),
                        FieldValue::Block(Some(Vec::new())),
                    ],
                ));
            }
            added
        }
        3 => vec![form(
            "lexicalObligation",
            vec![
                named(""),
                named("literal"),
                FieldValue::Text(value.to_owned()),
            ],
        )],
        _ => return Err(failure()),
    };
    let renames = plan[3..]
        .chunks_exact(2)
        .map(|pair| {
            (
                kernel::position(pair[0]),
                format!("@literal:{}", kernel::position(pair[1])),
            )
        })
        .collect::<BTreeMap<_, _>>();
    let mut items = Vec::new();
    for index in 0..=entries.len() {
        if index == insertion {
            items.extend(added.iter().cloned());
        }
        if let Some(entry) = entries.get(index) {
            let mut entry = entry.clone();
            if let Some(name) = renames.get(&index)
                && let GrammarExpr::Feature(feature) = &mut entry
                && let FeatureExpr::Form(form) = &mut **feature
            {
                let fields = form.spec();
                for (field, value) in fields.iter().zip(&mut form.fields) {
                    if field.0 == "name" || field.0 == "type" {
                        *value = named(name);
                    }
                }
            }
            items.push(entry);
        }
    }
    Ok(GrammarLexer {
        name: lexer.name.clone(),
        expression: GrammarExpr::Sequence(items),
    })
}

pub(super) fn canonical_owners(grammar: &Grammar) -> Result<Grammar, GrammarRoundTripError> {
    let mut declarations = grammar.declarations().clone();
    for lexer in &mut declarations.lexers {
        let failure = || {
            GrammarRoundTripError::LexicalDeclaration(format!(
                "nonbijective original parser owners in lexer {}",
                lexer.name
            ))
        };
        let GrammarExpr::Sequence(entries) = &mut lexer.expression else {
            return Err(failure());
        };
        let forms = entries
            .iter()
            .map(as_form)
            .collect::<Option<Vec<_>>>()
            .ok_or_else(failure)?;
        let kinds = forms
            .iter()
            .map(|entry| entry.head.clone())
            .collect::<Vec<_>>();
        let names = forms
            .iter()
            .map(|entry| entry.text("name").unwrap_or_default().to_owned())
            .collect::<Vec<_>>();
        let targets = forms
            .iter()
            .map(|entry| match entry.expression("item") {
                Some(GrammarExpr::NonTerminal(target)) if grammar.rule(target).is_some() => {
                    target.clone()
                }
                _ => String::new(),
            })
            .collect::<Vec<_>>();
        let plan = kernel::owner_comparison(&kinds, &names, &targets);
        if plan.first() == Some(&-1.0) {
            return Err(failure());
        }
        let replacements = plan
            .chunks_exact(2)
            .map(|pair| {
                (
                    kernel::position(pair[0]),
                    targets[kernel::position(pair[1])].clone(),
                )
            })
            .collect::<BTreeMap<_, _>>();
        for (index, replacement) in replacements {
            if let GrammarExpr::Feature(feature) = &mut entries[index]
                && let FeatureExpr::Form(entry) = &mut **feature
                && let Some(field) = entry.spec().iter().position(|(name, _)| *name == "name")
            {
                entry.fields[field] = named(&replacement);
            }
        }
    }
    let mut canonical = grammar.clone();
    canonical.set_declarations(declarations);
    Ok(canonical)
}
