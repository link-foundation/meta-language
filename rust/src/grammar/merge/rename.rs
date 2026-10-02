//! Binding-aware renaming of grammar rules and restoring source names.

use std::collections::{BTreeMap, BTreeSet};

use super::normalize::quote;
use super::{GrammarRenameError, GrammarRenameErrorKind, RenamedGrammar, RuleAlias};
use crate::grammar::{
    FeatureExpr, Grammar, GrammarDeclarations, GrammarExpr, GrammarMacro, GrammarRule,
    GrammarScanner, Operation,
};

/// Renames one rule and every reference to it.
///
/// Recursive references, references inside captures and references qualified
/// with `namespace` (as `namespace.rule` or `namespace::rule`) are renamed.
/// Capture labels are a separate scope and are never renamed. The returned
/// aliases map canonical names back to the original source names, so the
/// grammar can be exported with them.
pub fn rename_grammar_rule(
    grammar: &Grammar,
    from: &str,
    to: &str,
    namespace: Option<&str>,
    aliases: &[RuleAlias],
) -> Result<RenamedGrammar, GrammarRenameError> {
    if to.is_empty() || to.chars().any(char::is_whitespace) {
        return Err(GrammarRenameError::new(
            GrammarRenameErrorKind::InvalidName,
            format!("invalid rule name {}", quote(to)),
        ));
    }
    if grammar.rule(from).is_none() {
        return Err(GrammarRenameError::new(
            GrammarRenameErrorKind::UnknownRule,
            format!("grammar has no rule {from}"),
        ));
    }
    if from == to {
        return Ok(RenamedGrammar {
            grammar: grammar.clone(),
            aliases: aliases.to_vec(),
        });
    }
    let mut mapping = vec![(from.to_owned(), to.to_owned())];
    if let Some(namespace) = namespace {
        mapping.push((format!("{namespace}.{from}"), format!("{namespace}.{to}")));
        mapping.push((format!("{namespace}::{from}"), format!("{namespace}::{to}")));
    }
    let mut taken = grammar.referenced_nonterminals();
    taken.extend(grammar.rules().iter().map(|rule| rule.name.clone()));
    if let Some((_, target)) = mapping.iter().find(|(_, target)| taken.contains(target)) {
        return Err(GrammarRenameError::new(
            GrammarRenameErrorKind::Collision,
            format!("renaming {from} to {to} would capture the existing name {target}"),
        ));
    }
    let mut chained = false;
    let mut next_aliases: Vec<RuleAlias> = aliases
        .iter()
        .map(|alias| {
            if alias.canonical == from {
                chained = true;
                RuleAlias {
                    canonical: to.to_owned(),
                    original: alias.original.clone(),
                }
            } else {
                alias.clone()
            }
        })
        .collect();
    if !chained {
        next_aliases.push(RuleAlias {
            canonical: to.to_owned(),
            original: from.to_owned(),
        });
    }
    Ok(RenamedGrammar {
        grammar: rename_all(grammar, &mapping.into_iter().collect()),
        aliases: next_aliases,
    })
}

/// Renames canonical rule names back to their source names for export.
pub fn restore_source_names(
    grammar: &Grammar,
    aliases: &[RuleAlias],
    namespace: Option<&str>,
) -> Result<Grammar, GrammarRenameError> {
    let mut mapping = BTreeMap::new();
    for RuleAlias {
        canonical,
        original,
    } in aliases
    {
        if grammar.rule(canonical).is_none() || canonical == original {
            continue;
        }
        mapping.insert(canonical.clone(), original.clone());
        if let Some(namespace) = namespace {
            mapping.insert(
                format!("{namespace}.{canonical}"),
                format!("{namespace}.{original}"),
            );
            mapping.insert(
                format!("{namespace}::{canonical}"),
                format!("{namespace}::{original}"),
            );
        }
    }
    let rename = |name: &str| {
        mapping
            .get(name)
            .cloned()
            .unwrap_or_else(|| name.to_owned())
    };
    let rule_names: Vec<String> = grammar
        .rules()
        .iter()
        .map(|rule| rename(&rule.name))
        .collect();
    let unique: BTreeSet<&String> = rule_names.iter().collect();
    let captured = grammar
        .referenced_nonterminals()
        .iter()
        .filter(|name| grammar.rule(name).is_none())
        .any(|name| rule_names.contains(&rename(name)));
    if unique.len() != rule_names.len() || captured {
        return Err(GrammarRenameError::new(
            GrammarRenameErrorKind::Collision,
            "restoring source names would give two bindings the same name",
        ));
    }
    Ok(rename_all(grammar, &mapping))
}

/// Renames rules and every reference to them: in rule bodies, in rule
/// actions and in the declarations (extras, conflict groups, macro bodies and
/// scanner operations). Macro names, scanner names and tokens are separate
/// scopes. A parameter is used through its own `(parameter name)` form,
/// never through a reference, so renaming never touches it.
fn rename_all(grammar: &Grammar, mapping: &BTreeMap<String, String>) -> Grammar {
    let rename = |name: &str| {
        mapping
            .get(name)
            .cloned()
            .unwrap_or_else(|| name.to_owned())
    };
    let mut renamed = Grammar::new();
    for rule in grammar.rules() {
        renamed.add_rule(GrammarRule {
            name: rename(&rule.name),
            ..renamed_rule(rule, &rename)
        });
    }
    if let Some(start) = grammar.start() {
        renamed.set_start(rename(start));
    }
    renamed.source_format = grammar.source_format();
    renamed.set_declarations(map_declarations(grammar.declarations(), &rename));
    renamed
}

/// A rule with its body and action references renamed; its own name is kept.
pub(super) fn renamed_rule(rule: &GrammarRule, rename: &dyn Fn(&str) -> String) -> GrammarRule {
    let mut attributes = rule.attributes.clone();
    if let Some(action) = &rule.attributes.action {
        attributes.action = Some(map_operations(action, rename));
    }
    GrammarRule {
        name: rule.name.clone(),
        expr: map_references(&rule.expr, rename),
        kind: rule.kind,
        concept: rule.concept.clone(),
        doc: rule.doc.clone(),
        attributes,
    }
}

/// The declarations with every rule reference renamed.
pub(super) fn map_declarations(
    declarations: &GrammarDeclarations,
    rename: &dyn Fn(&str) -> String,
) -> GrammarDeclarations {
    GrammarDeclarations {
        matching: declarations.matching.clone(),
        imports: declarations.imports.clone(),
        modes: declarations.modes.clone(),
        extras: declarations
            .extras
            .iter()
            .map(|extra| map_references(extra, rename))
            .collect(),
        conflicts: declarations
            .conflicts
            .iter()
            .map(|group| group.iter().map(|name| rename(name)).collect())
            .collect(),
        macros: declarations
            .macros
            .iter()
            .map(|declared| GrammarMacro {
                name: declared.name.clone(),
                parameters: declared.parameters.clone(),
                expression: map_references(&declared.expression, rename),
            })
            .collect(),
        scanners: declarations
            .scanners
            .iter()
            .map(|scanner| GrammarScanner {
                name: scanner.name.clone(),
                tokens: scanner.tokens.clone(),
                operations: map_operations(&scanner.operations, rename),
            })
            .collect(),
    }
}

/// Operations with every rule reference in their expressions renamed.
pub(super) fn map_operations(
    operations: &[Operation],
    rename: &dyn Fn(&str) -> String,
) -> Vec<Operation> {
    operations
        .iter()
        .map(|operation| {
            let mut copy = operation.clone();
            copy.map_expressions(&mut |expr| *expr = map_references(expr, rename));
            copy
        })
        .collect()
}

pub(super) fn map_references(expr: &GrammarExpr, rename: &dyn Fn(&str) -> String) -> GrammarExpr {
    let map = |inner: &GrammarExpr| Box::new(map_references(inner, rename));
    match expr {
        GrammarExpr::NonTerminal(name) => GrammarExpr::NonTerminal(rename(name)),
        GrammarExpr::Choice {
            ordered,
            alternatives,
        } => GrammarExpr::Choice {
            ordered: *ordered,
            alternatives: alternatives
                .iter()
                .map(|item| map_references(item, rename))
                .collect(),
        },
        GrammarExpr::Sequence(items) => GrammarExpr::Sequence(
            items
                .iter()
                .map(|item| map_references(item, rename))
                .collect(),
        ),
        GrammarExpr::Optional(inner) => GrammarExpr::Optional(map(inner)),
        GrammarExpr::ZeroOrMore(inner) => GrammarExpr::ZeroOrMore(map(inner)),
        GrammarExpr::OneOrMore(inner) => GrammarExpr::OneOrMore(map(inner)),
        GrammarExpr::Repeat { expr, min, max } => GrammarExpr::Repeat {
            expr: map(expr),
            min: *min,
            max: *max,
        },
        GrammarExpr::And(inner) => GrammarExpr::And(map(inner)),
        GrammarExpr::Not(inner) => GrammarExpr::Not(map(inner)),
        GrammarExpr::Capture { label, expr } => GrammarExpr::Capture {
            label: label.clone(),
            expr: map(expr),
        },
        GrammarExpr::Feature(feature) => {
            let mut copy =
                GrammarExpr::rewrite_feature(feature, |inner| map_references(inner, rename));
            if let GrammarExpr::Feature(renamed) = &mut copy
                && let FeatureExpr::Call { name, .. } = renamed.as_mut()
            {
                *name = rename(name);
            }
            copy
        }
        other => other.clone(),
    }
}
