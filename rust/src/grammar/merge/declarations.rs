//! The rule fields and grammar declarations of the feature union in a merge:
//! they take part in rule signatures and fingerprints, and the declarations
//! of the sources are united with every conflict reported.

use std::collections::{BTreeMap, BTreeSet};

use super::group::Prepared;
use super::rename::{map_declarations, map_operations};
use crate::grammar::interchange::{
    render_declaration_links, render_links_expression, render_rule_fields,
};
use crate::grammar::{GrammarDeclarations, GrammarRule};

/// The rule fields (parameters, channel, modes and action) as they take part
/// in a rule's signature, with action references printed by `label`; empty
/// for a rule without them, so plain rules keep their definitions.
pub(super) fn rule_fields(rule: &GrammarRule, label: &dyn Fn(&str) -> String) -> String {
    let mut attributes = rule.attributes.clone();
    if let Some(action) = &rule.attributes.action {
        attributes.action = Some(map_operations(action, label));
    }
    let fields = render_rule_fields(&attributes);
    if fields.is_empty() {
        String::new()
    } else {
        format!(" {}", fields.join(" "))
    }
}

/// The declarations of one grammar as one line of links.
pub(super) fn declarations_text(declarations: &GrammarDeclarations) -> String {
    let mut parts: Vec<String> = declarations
        .matching
        .iter()
        .map(|matching| format!("(matching {matching})"))
        .chain(
            declarations
                .settling
                .iter()
                .map(|settling| format!("(settling {})", settling.join(" "))),
        )
        .collect();
    parts.extend(render_declaration_links(declarations));
    parts.join(" ")
}

/// A matching, macro or scanner that two sources declare differently.
pub(super) struct DeclarationConflict {
    /// `matching`, or the macro or scanner name.
    pub(super) name: String,
    /// `different-matching`, `different-macro` or `different-scanner`.
    pub(super) basis: &'static str,
    /// The source that declared it first and the source that differs.
    pub(super) members: Vec<String>,
}

fn single_line(declarations: &GrammarDeclarations) -> String {
    render_declaration_links(declarations)
        .into_iter()
        .next()
        .unwrap_or_default()
}

/// Merges the declarations of the sources in precedence order, with each
/// source's rule names renamed by `canonical`. Imports, modes, extras,
/// conflict groups and precedence orders are united; the first declared
/// matching, macro or scanner of a name wins, and a later different one is a
/// conflict. The settling is the one the source of the matching declares
/// (none: its matching's default), else the first declared: a merged grammar
/// settles its parses as the source it matches like.
pub(super) fn merge_declarations(
    entry: &[&Prepared<'_>],
    canonical: &dyn Fn(&Prepared<'_>, &str) -> String,
) -> (GrammarDeclarations, Vec<DeclarationConflict>) {
    let mut merged = GrammarDeclarations::default();
    let mut conflicts = Vec::new();
    let mut matching_owner = "";
    let mut settlings: Vec<(&str, Option<Vec<String>>)> = Vec::new();
    let mut seen_extras = BTreeSet::new();
    let mut seen_conflicts = BTreeSet::new();
    let mut seen_precedences = BTreeSet::new();
    let mut seen_macros: BTreeMap<String, (String, &str)> = BTreeMap::new();
    let mut seen_scanners: BTreeMap<String, (String, &str)> = BTreeMap::new();
    for source in entry {
        let own = map_declarations(source.grammar.declarations(), &|name| {
            canonical(source, name)
        });
        settlings.push((source.id, own.settling));
        if let Some(matching) = own.matching {
            match &merged.matching {
                None => {
                    merged.matching = Some(matching);
                    matching_owner = source.id;
                }
                Some(first) if *first != matching => conflicts.push(DeclarationConflict {
                    name: "matching".to_owned(),
                    basis: "different-matching",
                    members: vec![matching_owner.to_owned(), source.id.to_owned()],
                }),
                Some(_) => {}
            }
        }
        for name in own.imports {
            if !merged.imports.contains(&name) {
                merged.imports.push(name);
            }
        }
        for name in own.modes {
            if !merged.modes.contains(&name) {
                merged.modes.push(name);
            }
        }
        for extra in own.extras {
            if seen_extras.insert(render_links_expression(&extra)) {
                merged.extras.push(extra);
            }
        }
        for group in own.conflicts {
            let line = single_line(&GrammarDeclarations {
                conflicts: vec![group.clone()],
                ..GrammarDeclarations::default()
            });
            if seen_conflicts.insert(line) {
                merged.conflicts.push(group);
            }
        }
        for order in own.precedences {
            let line = single_line(&GrammarDeclarations {
                precedences: vec![order.clone()],
                ..GrammarDeclarations::default()
            });
            if seen_precedences.insert(line) {
                merged.precedences.push(order);
            }
        }
        for declared in own.macros {
            let line = single_line(&GrammarDeclarations {
                macros: vec![declared.clone()],
                ..GrammarDeclarations::default()
            });
            match seen_macros.get(&declared.name) {
                None => {
                    seen_macros.insert(declared.name.clone(), (line, source.id));
                    merged.macros.push(declared);
                }
                Some((first, owner)) if *first != line => conflicts.push(DeclarationConflict {
                    name: declared.name,
                    basis: "different-macro",
                    members: vec![(*owner).to_owned(), source.id.to_owned()],
                }),
                Some(_) => {}
            }
        }
        for scanner in own.scanners {
            let line = single_line(&GrammarDeclarations {
                scanners: vec![scanner.clone()],
                ..GrammarDeclarations::default()
            });
            match seen_scanners.get(&scanner.name) {
                None => {
                    seen_scanners.insert(scanner.name.clone(), (line, source.id));
                    merged.scanners.push(scanner);
                }
                Some((first, owner)) if *first != line => conflicts.push(DeclarationConflict {
                    name: scanner.name,
                    basis: "different-scanner",
                    members: vec![(*owner).to_owned(), source.id.to_owned()],
                }),
                Some(_) => {}
            }
        }
    }
    merged.settling = if merged.matching.is_none() {
        settlings.into_iter().find_map(|(_, settling)| settling)
    } else {
        settlings
            .into_iter()
            .find(|(owner, _)| *owner == matching_owner)
            .and_then(|(_, settling)| settling)
    };
    (merged, conflicts)
}
