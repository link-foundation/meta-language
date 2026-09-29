//! Helpers shared by the ANTLR and Lark emitters.
//!
//! Both notations impose case conventions on rule names, group alternatives
//! and sequences the same way, and are re-imported by importers that flatten
//! nested sequences and unordered choices. The helpers here keep emitted text
//! stable under that normalisation so emission is a fixpoint from the first
//! re-import.

use std::collections::{BTreeMap, BTreeSet};

use super::{EmitReport, GrammarEmitError, unsupported_error};
use crate::grammar::{CharClassItem, Grammar, GrammarExpr, GrammarFormat};

/// Largest number of operand copies a counted repetition may expand to.
pub(super) const MAX_REPEAT_COPIES: usize = 256;

/// Binding strength of an emitted expression, weakest first.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(super) enum Precedence {
    Choice,
    Sequence,
    Labeled,
    Postfix,
    Prefix,
    Atom,
}

/// Emitted text together with its binding strength.
pub(super) type Rendered = (String, Precedence);

/// Wraps `text` in a group when it binds more weakly than `parent` requires.
pub(super) fn wrap((text, precedence): Rendered, parent: Precedence) -> String {
    if precedence < parent {
        format!("({text})")
    } else {
        text
    }
}

/// Joins sequence items, splicing nested sequences and skipping empty items.
///
/// Importers flatten nested sequences and drop empty items, so the emitted
/// text must not group them either or the second emission would differ.
pub(super) fn join_sequence(items: Vec<Rendered>) -> Rendered {
    let mut parts = items
        .into_iter()
        .filter(|(text, _precedence)| !text.is_empty())
        .collect::<Vec<_>>();
    match parts.len() {
        0 => (String::new(), Precedence::Sequence),
        1 => parts.remove(0),
        _ => (
            parts
                .into_iter()
                .map(|(text, precedence)| {
                    if precedence == Precedence::Sequence {
                        text
                    } else {
                        wrap((text, precedence), Precedence::Labeled)
                    }
                })
                .collect::<Vec<_>>()
                .join(" "),
            Precedence::Sequence,
        ),
    }
}

/// Joins choice alternatives, splicing nested choices.
///
/// An all-empty choice renders as the empty expression because the importers
/// collapse it to `Empty`.
pub(super) fn join_choice(alternatives: Vec<Rendered>) -> Rendered {
    if alternatives
        .iter()
        .all(|(text, _precedence)| text.is_empty())
    {
        return (String::new(), Precedence::Sequence);
    }
    if alternatives.len() == 1 {
        return alternatives.into_iter().next().expect("one alternative");
    }
    let mut output = String::new();
    for (index, (text, _precedence)) in alternatives.into_iter().enumerate() {
        if index > 0 {
            if !output.is_empty() {
                output.push(' ');
            }
            output.push('|');
        }
        if !text.is_empty() {
            if !output.is_empty() {
                output.push(' ');
            }
            output.push_str(&text);
        }
    }
    (output, Precedence::Choice)
}

/// Lowers a counted repetition to copies, nested optionals and a trailing
/// one-or-more loop.
pub(super) fn lower_repeat(
    format: GrammarFormat,
    expr: &GrammarExpr,
    min: usize,
    max: Option<usize>,
) -> Result<GrammarExpr, GrammarEmitError> {
    check_repeat_bounds(format, min, max)?;
    let copies = max.unwrap_or(min);
    if copies > MAX_REPEAT_COPIES {
        return Err(unsupported_error(
            format,
            format!(
                "counted repetition {} expands to {copies} copies",
                repeat_bounds(min, max)
            ),
        ));
    }
    let mut items = Vec::new();
    match max {
        None if min == 0 => return Ok(GrammarExpr::zero_or_more(expr.clone())),
        None => {
            items.extend(std::iter::repeat_n(expr.clone(), min - 1));
            items.push(GrammarExpr::one_or_more(expr.clone()));
        }
        Some(max) => {
            items.extend(std::iter::repeat_n(expr.clone(), min));
            let mut tail = GrammarExpr::Empty;
            for _ in min..max {
                tail = GrammarExpr::optional(if tail == GrammarExpr::Empty {
                    expr.clone()
                } else {
                    GrammarExpr::Sequence(vec![expr.clone(), tail])
                });
            }
            if tail != GrammarExpr::Empty {
                items.push(tail);
            }
        }
    }
    Ok(match items.len() {
        0 => GrammarExpr::Empty,
        1 => items.remove(0),
        _ => GrammarExpr::Sequence(items),
    })
}

/// Rejects repetitions whose maximum is below their minimum.
pub(super) fn check_repeat_bounds(
    format: GrammarFormat,
    min: usize,
    max: Option<usize>,
) -> Result<(), GrammarEmitError> {
    match max {
        Some(max) if max < min => Err(unsupported_error(
            format,
            format!("repeat maximum {max} is below minimum {min}"),
        )),
        _ => Ok(()),
    }
}

/// Renders repetition bounds as `{min,max}` or `{min,}`.
pub(super) fn repeat_bounds(min: usize, max: Option<usize>) -> String {
    max.map_or_else(|| format!("{{{min},}}"), |max| format!("{{{min},{max}}}"))
}

/// Returns the negation flag and items of a single-character set, when `expr`
/// can be written as one.
pub(super) fn char_set_items(expr: &GrammarExpr) -> Option<(bool, Vec<CharClassItem>)> {
    match expr {
        GrammarExpr::CharClass { negated, items } if !items.is_empty() => {
            Some((*negated, items.clone()))
        }
        GrammarExpr::CharRange(start, end) if start <= end => {
            Some((false, vec![CharClassItem::Range(*start, *end)]))
        }
        GrammarExpr::Terminal(value) => {
            let mut chars = value.chars();
            match (chars.next(), chars.next()) {
                (Some(character), None) => Some((false, vec![CharClassItem::Char(character)])),
                _ => None,
            }
        }
        _ => None,
    }
}

/// Returns the complemented set for `!set` followed by any character.
pub(super) fn negated_lookahead_set(
    items: &[GrammarExpr],
    index: usize,
) -> Option<(bool, Vec<CharClassItem>)> {
    let GrammarExpr::Not(inner) = items.get(index)? else {
        return None;
    };
    if items.get(index + 1) != Some(&GrammarExpr::AnyChar) {
        return None;
    }
    char_set_items(inner).map(|(negated, class)| (!negated, class))
}

/// Returns the case variants of `character` when it is a cased letter.
pub(super) fn case_variants(character: char) -> Option<Vec<char>> {
    let mut lower = character.to_lowercase();
    let mut upper = character.to_uppercase();
    let (Some(lower_char), None, Some(upper_char), None) =
        (lower.next(), lower.next(), upper.next(), upper.next())
    else {
        return None;
    };
    if lower_char == upper_char {
        return None;
    }
    let mut variants = vec![lower_char, upper_char];
    if character != lower_char && character != upper_char {
        variants.push(character);
    }
    Some(variants)
}

/// Collects non-terminal references in `expr`, in first-use order.
pub(super) fn collect_references<'expr>(expr: &'expr GrammarExpr, names: &mut Vec<&'expr str>) {
    match expr {
        GrammarExpr::NonTerminal(name) => {
            if !names.contains(&name.as_str()) {
                names.push(name);
            }
        }
        GrammarExpr::Choice { alternatives, .. } => {
            for alternative in alternatives {
                collect_references(alternative, names);
            }
        }
        GrammarExpr::Sequence(items) => {
            for item in items {
                collect_references(item, names);
            }
        }
        GrammarExpr::Optional(inner)
        | GrammarExpr::ZeroOrMore(inner)
        | GrammarExpr::OneOrMore(inner)
        | GrammarExpr::And(inner)
        | GrammarExpr::Not(inner)
        | GrammarExpr::Repeat { expr: inner, .. }
        | GrammarExpr::Capture { expr: inner, .. } => collect_references(inner, names),
        GrammarExpr::Empty
        | GrammarExpr::Terminal(_)
        | GrammarExpr::TerminalInsensitive(_)
        | GrammarExpr::CharRange(_, _)
        | GrammarExpr::CharClass { .. }
        | GrammarExpr::AnyChar => {}
    }
}

/// Emitted names for every rule and every undefined non-terminal reference.
#[derive(Clone, Debug)]
pub(super) struct NamePlan {
    rule_names: Vec<String>,
    by_source: BTreeMap<String, String>,
}

impl NamePlan {
    /// Plans emitted names and records every rename in `report`.
    ///
    /// `candidates` holds one conventional name per rule. Rules listed in
    /// `fixed` keep their candidate verbatim; other rules whose candidate is
    /// their own name are reserved next, and the remaining rules receive
    /// `_2`, `_3`, ... suffixes in rule order when their candidate is taken.
    /// Undefined references are mapped through `reference` and made unique
    /// the same way.
    pub(super) fn new(
        grammar: &Grammar,
        label: &str,
        candidates: &[String],
        fixed: &BTreeSet<usize>,
        reference: impl Fn(&str) -> String,
        report: &mut EmitReport,
    ) -> Self {
        let rules = grammar.rules();
        let mut used = BTreeSet::new();
        let mut names: Vec<Option<String>> = vec![None; rules.len()];
        for &index in fixed {
            used.insert(candidates[index].clone());
            names[index] = Some(candidates[index].clone());
        }
        for (index, rule) in rules.iter().enumerate() {
            if names[index].is_none()
                && candidates[index] == rule.name
                && used.insert(candidates[index].clone())
            {
                names[index] = Some(candidates[index].clone());
            }
        }
        for (index, name) in names.iter_mut().enumerate() {
            if name.is_none() {
                let unique = unique_name(&candidates[index], &used);
                used.insert(unique.clone());
                *name = Some(unique);
            }
        }
        let rule_names = names
            .into_iter()
            .map(|name| name.expect("every rule is named"))
            .collect::<Vec<_>>();

        let mut by_source = BTreeMap::new();
        for (rule, emitted) in rules.iter().zip(&rule_names) {
            if rule.name != *emitted {
                report.add_lossy(format!(
                    "{label} renamed rule {:?} to {emitted:?}",
                    rule.name
                ));
            }
            by_source
                .entry(rule.name.clone())
                .or_insert_with(|| emitted.clone());
        }
        for source in grammar.undefined_nonterminals() {
            let candidate = reference(&source);
            let emitted = unique_name(&candidate, &used);
            used.insert(emitted.clone());
            if emitted != source {
                report.add_lossy(format!(
                    "{label} renamed non-terminal reference {source:?} to {emitted:?}"
                ));
            }
            by_source.insert(source, emitted);
        }
        Self {
            rule_names,
            by_source,
        }
    }

    /// Returns the emitted name of the rule at `index`.
    pub(super) fn rule_name(&self, index: usize) -> &str {
        &self.rule_names[index]
    }

    /// Returns the emitted name for a source rule or reference name.
    pub(super) fn resolve<'name>(&'name self, name: &'name str) -> &'name str {
        self.by_source.get(name).map_or(name, String::as_str)
    }
}

fn unique_name(candidate: &str, used: &BTreeSet<String>) -> String {
    if !used.contains(candidate) {
        return candidate.to_string();
    }
    (2..=used.len() + 2)
        .map(|suffix| format!("{candidate}_{suffix}"))
        .find(|name| !used.contains(name))
        .expect("one of used.len() + 1 suffixes is unused")
}

/// Replaces characters outside `[A-Za-z0-9_]` with `_`.
pub(super) fn ascii_identifier(name: &str) -> String {
    name.chars()
        .map(|character| {
            if character.is_ascii_alphanumeric() || character == '_' {
                character
            } else {
                '_'
            }
        })
        .collect()
}

/// Converts `camelCase` and `PascalCase` names to lower `snake_case`.
pub(super) fn snake_case(name: &str) -> String {
    let mut output = String::new();
    let mut previous: Option<char> = None;
    for character in name.chars() {
        if character.is_ascii_uppercase() {
            if previous
                .is_some_and(|previous| previous.is_ascii_lowercase() || previous.is_ascii_digit())
            {
                output.push('_');
            }
            output.push(character.to_ascii_lowercase());
        } else if character.is_ascii_alphanumeric() || character == '_' {
            output.push(character);
        } else {
            output.push('_');
        }
        previous = Some(character);
    }
    output
}

/// Splits documentation into lines with surrounding whitespace removed,
/// skipping blank lines.
pub(super) fn doc_lines(text: &str) -> impl Iterator<Item = &str> {
    text.lines().map(str::trim).filter(|line| !line.is_empty())
}
