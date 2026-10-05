//! Reconciliation of corresponding rules across the sources of one language
//! edition, the `reconcile` mode of [`super::merge_grammars`]. It mirrors
//! `js/src/grammar-reconcile.js`.
//!
//! The strict merge unites only the rules it proves equivalent. Real grammars
//! of one language from different sources (a tree-sitter grammar.json and a
//! grammars-v4 ANTLR grammar) almost never are: they spell the same lexical
//! concepts with different details, and they decompose the same constructs in
//! the same way but write them differently. Reconciliation recognizes such
//! corresponding rules under any names:
//!
//! - A lexical rule, one that reaches only other lexical rules and no
//!   recursion, is compared by its lexical role. A rule that spells finitely
//!   many strings is compared by its exact text. Otherwise the role is the
//!   delimiters that every alternative opens and closes with, or else the kind
//!   of character it starts with, a letter or a digit.
//! - A parser rule is compared by a bisimulation up to these roles, after a
//!   normalization that keeps the language of the rule. Fields, precedences,
//!   aliases and token wrappers are dropped. An end-of-input check that ends a
//!   sequence is dropped. A sequence with at most three optional items is
//!   expanded into the alternatives with and without each of them.
//!
//! A class of corresponding rules is reconciled only when every source in it
//! has exactly one strictly distinct rule in it. So a source's own rules are
//! never united by correspondence, and a role that two rules of one source
//! share is no evidence at all: those rules are compared by their exact text.

use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet};

use super::GrammarMergeError;
use super::group::{Node, alias_of, source_label};
use super::normalize::{normalize, quote};
use super::rename::map_references;
use crate::grammar::feature::{FeatureExpr, FeatureForm, FieldValue, UnicodeClassItem};
use crate::grammar::{CharClassItem, GrammarExpr, GrammarRule, RuleKind};

/// How a reconciled class of corresponding rules is justified.
pub(super) const GRAMMAR_RECONCILE_METHOD: &str = "structural-correspondence-up-to-lexical-roles";
/// How a reconciled class of rules that only share a name is justified.
pub(super) const GRAMMAR_NAME_RECONCILE_METHOD: &str = "name-correspondence";

// Feature wrappers that change how a parse is chosen or named, not what it
// accepts.
const WRAPPERS: &[&str] = &[
    "precedence",
    "namedPrecedence",
    "dynamicPrecedence",
    "lexicalPrecedence",
    "token",
    "immediateToken",
    "alias",
];
const MAX_OPTIONALS: usize = 3;
const MAX_EXPANSION: usize = 4096;

/// The class key of a rule: its strict class, a structurally reconciled
/// class, or a strict class reconciled by name.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Key {
    Strict(usize),
    Reconciled(usize),
    Name(usize),
}

/// The reconciled classes of a merge group: a class number for every node
/// and the basis of every reconciled class. `strict` holds the strict
/// bisimulation class of every node.
pub(super) fn reconcile_classes(
    nodes: &[Node<'_>],
    index: &BTreeMap<String, usize>,
    strict: &[usize],
) -> Result<(Vec<usize>, BTreeMap<usize, &'static str>), GrammarMergeError> {
    let tokens = token_rules(nodes, index, &lexical_rules(nodes, index));
    let roles = lexical_roles(nodes, index, strict, &tokens)?;
    // Rules of a rejected class keep their strict classes in the next round,
    // so no accepted class rests on a correspondence that was rejected.
    let mut fixed = vec![false; nodes.len()];
    let (classes, reconciled) = loop {
        let classes = refine_up_to_roles(nodes, index, strict, &tokens, &roles, &fixed)?;
        let mut members: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
        for (position, class) in classes.iter().enumerate() {
            members.entry(*class).or_default().push(position);
        }
        // A class is reconciled when it spans sources and each of its sources
        // has one strict class in it.
        let mut reconciled = BTreeSet::new();
        let mut rejected = false;
        for (class, positions) in &members {
            let by_source = strict_classes_by_source(nodes, strict, positions);
            if by_source.len() < 2 {
                continue;
            }
            if by_source.values().all(|set| set.len() == 1) {
                reconciled.insert(*class);
            } else {
                for &position in positions {
                    fixed[position] = true;
                }
                rejected = true;
            }
        }
        if !rejected {
            break (classes, reconciled);
        }
    };
    let mut keys: Vec<Key> = classes
        .iter()
        .enumerate()
        .map(|(position, class)| {
            if reconciled.contains(class) {
                Key::Reconciled(*class)
            } else {
                Key::Strict(strict[position])
            }
        })
        .collect();
    let mut bases: BTreeMap<Key, &'static str> = keys
        .iter()
        .filter(|key| matches!(key, Key::Reconciled(_)))
        .map(|key| (*key, GRAMMAR_RECONCILE_METHOD))
        .collect();
    reconcile_by_name(nodes, strict, &tokens, &mut keys, &mut bases);
    // Dense class numbers in the order keys first appear.
    let mut numbers: BTreeMap<Key, usize> = BTreeMap::new();
    let classes = keys
        .iter()
        .map(|key| {
            let next = numbers.len();
            *numbers.entry(*key).or_insert(next)
        })
        .collect();
    let bases = bases
        .into_iter()
        .filter_map(|(key, basis)| numbers.get(&key).map(|number| (*number, basis)))
        .collect();
    Ok((classes, bases))
}

// The greatest bisimulation up to lexical roles, by partition refinement as
// in the strict merge. A fixed rule starts in a class of its own strict
// class.
fn refine_up_to_roles(
    nodes: &[Node<'_>],
    index: &BTreeMap<String, usize>,
    strict: &[usize],
    tokens: &[bool],
    roles: &[Option<String>],
    fixed: &[bool],
) -> Result<Vec<usize>, GrammarMergeError> {
    let mut classes = vec![0_usize; nodes.len()];
    let mut count = 0_usize;
    loop {
        let mut signatures = Vec::with_capacity(nodes.len());
        for (position, node) in nodes.iter().enumerate() {
            let prefix = if count != 0 {
                classes[position].to_string()
            } else if fixed[position] {
                format!("!{}", strict[position])
            } else {
                String::new()
            };
            if tokens[position] {
                let role = roles[position].as_deref().unwrap_or_default();
                signatures.push(format!("{prefix}|lex|{role}"));
                continue;
            }
            let current = &classes;
            let internal = |name: &str| {
                let target = index[&alias_of(node.source.id, name)];
                if !tokens[target] {
                    return format!("#{}", current[target]);
                }
                // A token of an exact role reads as its text, so a parser rule
                // that spells the token inline compares equal to one that
                // references it.
                let role = roles[target].as_deref().unwrap_or_default();
                role.strip_prefix("exact(")
                    .and_then(|inner| inner.strip_suffix(')'))
                    .unwrap_or(role)
                    .to_owned()
            };
            let label = source_label(node.source, &internal);
            signatures.push(format!("{prefix}|{}", comparison_text(node.rule, &label)?));
        }
        let ranks: BTreeMap<&String, usize> = signatures
            .iter()
            .collect::<BTreeSet<_>>()
            .into_iter()
            .enumerate()
            .map(|(rank, key)| (key, rank))
            .collect();
        let size = ranks.len();
        classes = signatures.iter().map(|key| ranks[key]).collect();
        if size == count {
            return Ok(classes);
        }
        count = size;
    }
}

fn strict_classes_by_source<'a>(
    nodes: &[Node<'a>],
    strict: &[usize],
    positions: &[usize],
) -> BTreeMap<&'a str, BTreeSet<usize>> {
    let mut by_source: BTreeMap<&str, BTreeSet<usize>> = BTreeMap::new();
    for &position in positions {
        by_source
            .entry(nodes[position].source.id)
            .or_default()
            .insert(strict[position]);
    }
    by_source
}

// The rules no correspondence matched are reconciled by name: the strict
// classes of different sources whose rules have one name up to case, `_` and
// `-`, when each source has one such class, none of them is shared with
// another source yet, and they are all tokens or all not. Their keys become
// the name key of the first of them.
fn reconcile_by_name(
    nodes: &[Node<'_>],
    strict: &[usize],
    tokens: &[bool],
    keys: &mut [Key],
    bases: &mut BTreeMap<Key, &'static str>,
) {
    let mut class_sources: BTreeMap<usize, BTreeSet<&str>> = BTreeMap::new();
    for (position, node) in nodes.iter().enumerate() {
        class_sources
            .entry(strict[position])
            .or_default()
            .insert(node.source.id);
    }
    let shared: BTreeSet<usize> = class_sources
        .into_iter()
        .filter(|(_, sources)| sources.len() > 1)
        .map(|(class, _)| class)
        .collect();
    let mut by_name: BTreeMap<String, Vec<usize>> = BTreeMap::new();
    for (position, node) in nodes.iter().enumerate() {
        if !matches!(keys[position], Key::Strict(_)) || shared.contains(&strict[position]) {
            continue;
        }
        let name: String = node
            .rule
            .name
            .to_lowercase()
            .chars()
            .filter(|character| *character != '-' && *character != '_')
            .collect();
        if !name.is_empty() {
            by_name.entry(name).or_default().push(position);
        }
    }
    let mut used = BTreeSet::new();
    for positions in by_name.values() {
        let by_source = strict_classes_by_source(nodes, strict, positions);
        if by_source.len() < 2 || by_source.values().any(|set| set.len() != 1) {
            continue;
        }
        let mut ids = Vec::new();
        for &position in positions {
            if !ids.contains(&strict[position]) {
                ids.push(strict[position]);
            }
        }
        if ids.iter().any(|id| used.contains(id)) {
            continue;
        }
        if positions
            .iter()
            .any(|&position| tokens[position] != tokens[positions[0]])
        {
            continue;
        }
        let key = Key::Name(strict[positions[0]]);
        used.extend(ids.iter().copied());
        for (position, current) in keys.iter_mut().enumerate() {
            if ids.contains(&strict[position]) && matches!(current, Key::Strict(_)) {
                *current = key;
            }
        }
        bases.insert(key, GRAMMAR_NAME_RECONCILE_METHOD);
    }
}

// Whether each node is lexical: it reaches only lexical kinds and defined
// rules of its source, without recursion.
fn lexical_rules(nodes: &[Node<'_>], index: &BTreeMap<String, usize>) -> Vec<bool> {
    fn visit(
        position: usize,
        nodes: &[Node<'_>],
        index: &BTreeMap<String, usize>,
        state: &mut [u8],
        lexical: &mut [bool],
    ) -> bool {
        match state[position] {
            2 => return lexical[position],
            1 => return false,
            _ => {}
        }
        state[position] = 1;
        let node = &nodes[position];
        let mut result =
            only_lexical_kinds(&node.rule.expr) && node.rule.attributes.parameters.is_empty();
        if result {
            for name in references(&node.rule.expr) {
                match index.get(&alias_of(node.source.id, &name)) {
                    Some(&target) if visit(target, nodes, index, state, lexical) => {}
                    _ => {
                        result = false;
                        break;
                    }
                }
            }
        }
        state[position] = 2;
        lexical[position] = result;
        result
    }
    let mut state = vec![0_u8; nodes.len()];
    let mut lexical = vec![false; nodes.len()];
    for position in 0..nodes.len() {
        visit(position, nodes, index, &mut state, &mut lexical);
    }
    lexical
}

fn wrapped(form: &FeatureForm) -> Option<&GrammarExpr> {
    if WRAPPERS.contains(&form.head.as_str()) {
        form.expression("item")
    } else {
        None
    }
}

fn only_lexical_kinds(expr: &GrammarExpr) -> bool {
    match expr {
        GrammarExpr::Empty
        | GrammarExpr::Terminal(_)
        | GrammarExpr::TerminalInsensitive(_)
        | GrammarExpr::CharRange(..)
        | GrammarExpr::CharClass { .. }
        | GrammarExpr::AnyChar
        | GrammarExpr::NonTerminal(_) => true,
        GrammarExpr::Choice { alternatives, .. } | GrammarExpr::Sequence(alternatives) => {
            alternatives.iter().all(only_lexical_kinds)
        }
        GrammarExpr::Optional(inner)
        | GrammarExpr::ZeroOrMore(inner)
        | GrammarExpr::OneOrMore(inner)
        | GrammarExpr::Repeat { expr: inner, .. }
        | GrammarExpr::And(inner)
        | GrammarExpr::Not(inner)
        | GrammarExpr::Capture { expr: inner, .. } => only_lexical_kinds(inner),
        GrammarExpr::Feature(feature) => match feature.as_ref() {
            FeatureExpr::UnicodeClass { .. } => true,
            FeatureExpr::Form(form) => wrapped(form).is_some_and(only_lexical_kinds),
            FeatureExpr::ByteClass { .. } | FeatureExpr::Call { .. } => false,
        },
    }
}

fn references(expr: &GrammarExpr) -> Vec<String> {
    let names = RefCell::new(Vec::new());
    map_references(expr, &|name| {
        let mut list = names.borrow_mut();
        if !list.iter().any(|known: &String| known == name) {
            list.push(name.to_owned());
        }
        name.to_owned()
    });
    names.into_inner()
}

// Which lexical rules are tokens, compared by their roles: a rule its source
// declares a token, one that references no rule, or one a non-lexical rule
// references. The other lexical rules are compositions of tokens, such as the
// rows and fields of a grammar without recursion, and are compared like
// parser rules.
fn token_rules(nodes: &[Node<'_>], index: &BTreeMap<String, usize>, lexical: &[bool]) -> Vec<bool> {
    let mut tokens: Vec<bool> = nodes
        .iter()
        .enumerate()
        .map(|(position, node)| {
            lexical[position]
                && (node.rule.kind == RuleKind::Token
                    || references(&node.rule.expr)
                        .iter()
                        .all(|name| !index.contains_key(&alias_of(node.source.id, name))))
        })
        .collect();
    for (position, node) in nodes.iter().enumerate() {
        if lexical[position] {
            continue;
        }
        for name in references(&node.rule.expr) {
            if let Some(&target) = index.get(&alias_of(node.source.id, &name))
                && lexical[target]
            {
                tokens[target] = true;
            }
        }
    }
    tokens
}

// The lexical role of every token; `None` for the others.
fn lexical_roles(
    nodes: &[Node<'_>],
    index: &BTreeMap<String, usize>,
    strict: &[usize],
    tokens: &[bool],
) -> Result<Vec<Option<String>>, GrammarMergeError> {
    let mut expanded: BTreeMap<usize, Option<(GrammarExpr, usize)>> = BTreeMap::new();
    // A fragment, a lexical rule that only other lexical rules reference, is
    // part of a token rather than a token; it keeps its exact text.
    let mut fragment = vec![false; nodes.len()];
    let mut token = vec![false; nodes.len()];
    for (position, node) in nodes.iter().enumerate() {
        for name in references(&node.rule.expr) {
            if let Some(&target) = index.get(&alias_of(node.source.id, &name)) {
                if tokens[position] {
                    fragment[target] = true;
                } else {
                    token[target] = true;
                }
            }
        }
    }
    let mut described = Vec::with_capacity(nodes.len());
    for (position, node) in nodes.iter().enumerate() {
        if !tokens[position] {
            described.push(None);
            continue;
        }
        let Some((form, _)) = expand(position, nodes, index, &mut expanded) else {
            described.push(Some((format!("opaque({})", quote(&node.alias)), None)));
            continue;
        };
        let cleaned = lexical_form(&form);
        let exact = format!("exact({})", normalize(&cleaned, &|_| String::new())?.text);
        if fragment[position] && !token[position] {
            described.push(Some((exact, None)));
            continue;
        }
        let role = if finite(&cleaned) {
            None
        } else {
            delimiters(&cleaned).or_else(|| lead(&cleaned))
        };
        described.push(Some((exact, role)));
    }
    // A role two strictly distinct rules of one source share tells them apart
    // from nothing: those rules keep their exact text.
    let mut shared: BTreeMap<(&str, &str), BTreeSet<usize>> = BTreeMap::new();
    for (position, entry) in described.iter().enumerate() {
        if let Some((_, Some(role))) = entry {
            shared
                .entry((nodes[position].source.id, role.as_str()))
                .or_default()
                .insert(strict[position]);
        }
    }
    Ok(described
        .iter()
        .enumerate()
        .map(|(position, entry)| {
            entry.as_ref().map(|(exact, role)| match role {
                Some(role) if shared[&(nodes[position].source.id, role.as_str())].len() == 1 => {
                    role.clone()
                }
                _ => exact.clone(),
            })
        })
        .collect())
}

// The expression of a rule with every reference replaced by its expansion,
// and its size; `None` when it grows past MAX_EXPANSION nodes.
fn expand(
    position: usize,
    nodes: &[Node<'_>],
    index: &BTreeMap<String, usize>,
    expanded: &mut BTreeMap<usize, Option<(GrammarExpr, usize)>>,
) -> Option<(GrammarExpr, usize)> {
    if let Some(known) = expanded.get(&position) {
        return known.clone();
    }
    let node = &nodes[position];
    let mut expand_name = |name: &str| {
        let target = index[&alias_of(node.source.id, name)];
        expand(target, nodes, index, expanded)
    };
    let result = expand_expression(&node.rule.expr, &mut expand_name);
    expanded.insert(position, result.clone());
    result
}

fn expand_expression(
    expr: &GrammarExpr,
    expand_name: &mut dyn FnMut(&str) -> Option<(GrammarExpr, usize)>,
) -> Option<(GrammarExpr, usize)> {
    let mut size = 1_usize;
    let mut part = |inner: &GrammarExpr, size: &mut usize| {
        let (expr, part_size) = expand_expression(inner, expand_name)?;
        *size += part_size;
        Some(expr)
    };
    let boxed = |expr: GrammarExpr| Box::new(expr);
    let result = match expr {
        GrammarExpr::NonTerminal(name) => return expand_name(name),
        GrammarExpr::Choice {
            ordered,
            alternatives,
        } => GrammarExpr::Choice {
            ordered: *ordered,
            alternatives: alternatives
                .iter()
                .map(|item| part(item, &mut size))
                .collect::<Option<_>>()?,
        },
        GrammarExpr::Sequence(items) => GrammarExpr::Sequence(
            items
                .iter()
                .map(|item| part(item, &mut size))
                .collect::<Option<_>>()?,
        ),
        GrammarExpr::Optional(inner) => GrammarExpr::Optional(boxed(part(inner, &mut size)?)),
        GrammarExpr::ZeroOrMore(inner) => GrammarExpr::ZeroOrMore(boxed(part(inner, &mut size)?)),
        GrammarExpr::OneOrMore(inner) => GrammarExpr::OneOrMore(boxed(part(inner, &mut size)?)),
        GrammarExpr::Repeat { expr, min, max } => GrammarExpr::Repeat {
            expr: boxed(part(expr, &mut size)?),
            min: *min,
            max: *max,
        },
        GrammarExpr::And(inner) => GrammarExpr::And(boxed(part(inner, &mut size)?)),
        GrammarExpr::Not(inner) => GrammarExpr::Not(boxed(part(inner, &mut size)?)),
        GrammarExpr::Capture { label, expr } => GrammarExpr::Capture {
            label: label.clone(),
            expr: boxed(part(expr, &mut size)?),
        },
        GrammarExpr::Feature(feature) => match feature.as_ref() {
            FeatureExpr::Form(form) if wrapped(form).is_some() => {
                let mut copy = form.clone();
                for value in &mut copy.fields {
                    if let FieldValue::Expression(inner) = value {
                        *inner = part(inner, &mut size)?;
                    }
                }
                GrammarExpr::Feature(Box::new(FeatureExpr::Form(copy)))
            }
            // A class counts its items, as the JavaScript expansion does.
            FeatureExpr::UnicodeClass { items, .. } => {
                size += items.len();
                expr.clone()
            }
            _ => expr.clone(),
        },
        GrammarExpr::CharClass { items, .. } => {
            size += items.len();
            expr.clone()
        }
        _ => expr.clone(),
    };
    (size <= MAX_EXPANSION).then_some((result, size))
}

// A lexical expression without wrappers or captures, with nested sequences
// flattened and adjacent literals joined.
fn lexical_form(expr: &GrammarExpr) -> GrammarExpr {
    let boxed = |inner: &GrammarExpr| Box::new(lexical_form(inner));
    match expr {
        GrammarExpr::Capture { expr: inner, .. } => lexical_form(inner),
        GrammarExpr::Feature(feature) => match feature.as_ref() {
            FeatureExpr::Form(form) if wrapped(form).is_some() => {
                lexical_form(wrapped(form).expect("a wrapper has an item"))
            }
            _ => expr.clone(),
        },
        GrammarExpr::Sequence(list) => {
            let mut items: Vec<GrammarExpr> = Vec::new();
            for item in list.iter().map(lexical_form) {
                let parts = match item {
                    GrammarExpr::Sequence(parts) => parts,
                    other => vec![other],
                };
                for part in parts {
                    let joined = match (items.last().and_then(literal_value), literal_value(&part))
                    {
                        (Some(previous), Some(value)) => Some(previous + &value),
                        _ => None,
                    };
                    match joined {
                        Some(text) => {
                            *items.last_mut().expect("a previous literal") =
                                GrammarExpr::Terminal(text);
                        }
                        None => items.push(part),
                    }
                }
            }
            match items.len() {
                0 => GrammarExpr::Empty,
                1 => items.pop().expect("one item"),
                _ => GrammarExpr::Sequence(items),
            }
        }
        GrammarExpr::Choice {
            ordered,
            alternatives,
        } => GrammarExpr::Choice {
            ordered: *ordered,
            alternatives: alternatives.iter().map(lexical_form).collect(),
        },
        GrammarExpr::Optional(inner) => GrammarExpr::Optional(boxed(inner)),
        GrammarExpr::ZeroOrMore(inner) => GrammarExpr::ZeroOrMore(boxed(inner)),
        GrammarExpr::OneOrMore(inner) => GrammarExpr::OneOrMore(boxed(inner)),
        GrammarExpr::Repeat { expr, min, max } => GrammarExpr::Repeat {
            expr: boxed(expr),
            min: *min,
            max: *max,
        },
        GrammarExpr::And(inner) => GrammarExpr::And(boxed(inner)),
        GrammarExpr::Not(inner) => GrammarExpr::Not(boxed(inner)),
        _ => expr.clone(),
    }
}

fn literal_value(expr: &GrammarExpr) -> Option<String> {
    match expr {
        GrammarExpr::Terminal(value) => Some(value.clone()),
        GrammarExpr::CharRange(start, end) if start == end => Some(start.to_string()),
        _ => None,
    }
}

// Whether a lexical expression spells finitely many strings.
fn finite(expr: &GrammarExpr) -> bool {
    match expr {
        GrammarExpr::Empty
        | GrammarExpr::Terminal(_)
        | GrammarExpr::TerminalInsensitive(_)
        | GrammarExpr::CharRange(..)
        | GrammarExpr::And(_)
        | GrammarExpr::Not(_) => true,
        GrammarExpr::CharClass { negated, .. } => !negated,
        GrammarExpr::Sequence(items)
        | GrammarExpr::Choice {
            alternatives: items,
            ..
        } => items.iter().all(finite),
        GrammarExpr::Optional(inner) => finite(inner),
        GrammarExpr::Repeat { expr, max, .. } => max.is_some() && finite(expr),
        _ => false,
    }
}

// `delimited(open:close,...)` when every alternative opens with a literal
// starting with ASCII punctuation: a sequence of two or more items that
// starts with such a literal, or such a literal of two or more characters.
// `open` is the first character of the alternative and `close` its last, or
// `_` when it does not end with a literal.
fn delimiters(expr: &GrammarExpr) -> Option<String> {
    fn collect<'a>(item: &'a GrammarExpr, alternatives: &mut Vec<&'a GrammarExpr>) {
        if let GrammarExpr::Choice {
            alternatives: items,
            ..
        } = item
        {
            for inner in items {
                collect(inner, alternatives);
            }
        } else {
            alternatives.push(item);
        }
    }
    let mut alternatives = Vec::new();
    collect(expr, &mut alternatives);
    let mut pairs = BTreeSet::new();
    for alternative in alternatives {
        let whole = literal_value(alternative);
        let items: &[GrammarExpr] = match alternative {
            GrammarExpr::Sequence(items) => items,
            _ => &[],
        };
        let long_enough = whole
            .as_ref()
            .map_or(items.len() >= 2, |value| value.chars().count() >= 2);
        if !long_enough {
            return None;
        }
        let open = whole.clone().or_else(|| literal_value(&items[0]))?;
        let first = open.chars().next().filter(char::is_ascii_punctuation)?;
        let close = whole.or_else(|| items.last().and_then(literal_value));
        let close = close
            .and_then(|value| value.chars().last())
            .map_or_else(|| "_".to_owned(), |last| quote(&last.to_string()));
        pairs.insert(format!("{}:{close}", quote(&first.to_string())));
    }
    Some(format!(
        "delimited({})",
        pairs.into_iter().collect::<Vec<_>>().join(",")
    ))
}

// `lead(letter)` or `lead(digit)` when the first character of every match is
// drawn from letters and not digits, or from digits and not letters, among the
// ASCII letters and digits.
fn lead(expr: &GrammarExpr) -> Option<String> {
    let (atoms, _) = first_atoms(expr)?;
    let accepts = |mut probes: std::ops::RangeInclusive<char>| {
        probes.any(|probe| atoms.iter().any(|atom| atom_accepts(atom, probe)))
    };
    let letter = accepts('a'..='z') || accepts('A'..='Z');
    let digit = accepts('0'..='9');
    if letter == digit {
        return None;
    }
    Some(
        if letter {
            "lead(letter)"
        } else {
            "lead(digit)"
        }
        .to_owned(),
    )
}

enum Atom<'a> {
    Any,
    Literal(char),
    Insensitive(char),
    Range(char, char),
    Class(bool, &'a [CharClassItem]),
    Unicode(bool, &'a [UnicodeClassItem]),
}

// The atoms a match can start with, and whether the expression matches the
// empty string; `None` when a start is unknown.
fn first_atoms(expr: &GrammarExpr) -> Option<(Vec<Atom<'_>>, bool)> {
    let single = |atom| Some((vec![atom], false));
    match expr {
        GrammarExpr::Empty | GrammarExpr::And(_) | GrammarExpr::Not(_) => Some((Vec::new(), true)),
        GrammarExpr::Terminal(value) => value
            .chars()
            .next()
            .map_or(Some((Vec::new(), true)), |first| {
                single(Atom::Literal(first))
            }),
        GrammarExpr::TerminalInsensitive(value) => value
            .chars()
            .next()
            .map_or(Some((Vec::new(), true)), |first| {
                single(Atom::Insensitive(first))
            }),
        GrammarExpr::CharRange(start, end) => single(Atom::Range(*start, *end)),
        GrammarExpr::CharClass { negated, items } => single(Atom::Class(*negated, items)),
        GrammarExpr::AnyChar => single(Atom::Any),
        GrammarExpr::Feature(feature) => match feature.as_ref() {
            FeatureExpr::UnicodeClass { negated, items } => single(Atom::Unicode(*negated, items)),
            _ => None,
        },
        GrammarExpr::Sequence(items) => {
            let mut atoms = Vec::new();
            for item in items {
                let (first, nullable) = first_atoms(item)?;
                atoms.extend(first);
                if !nullable {
                    return Some((atoms, false));
                }
            }
            Some((atoms, true))
        }
        GrammarExpr::Choice { alternatives, .. } => {
            let mut atoms = Vec::new();
            let mut nullable = false;
            for item in alternatives {
                let (first, empty) = first_atoms(item)?;
                atoms.extend(first);
                nullable |= empty;
            }
            Some((atoms, nullable))
        }
        GrammarExpr::Optional(inner) | GrammarExpr::ZeroOrMore(inner) => {
            first_atoms(inner).map(|(atoms, _)| (atoms, true))
        }
        GrammarExpr::OneOrMore(inner) => first_atoms(inner),
        GrammarExpr::Repeat { expr, min, .. } => {
            first_atoms(expr).map(|(atoms, nullable)| (atoms, nullable || *min == 0))
        }
        GrammarExpr::NonTerminal(_) | GrammarExpr::Capture { .. } => None,
    }
}

fn atom_accepts(atom: &Atom<'_>, probe: char) -> bool {
    match atom {
        Atom::Any => true,
        Atom::Literal(value) => *value == probe,
        Atom::Insensitive(value) => value.is_ascii() && value.eq_ignore_ascii_case(&probe),
        Atom::Range(start, end) => (*start..=*end).contains(&probe),
        Atom::Class(negated, items) => {
            items.iter().any(|item| match item {
                CharClassItem::Char(value) => *value == probe,
                CharClassItem::Range(start, end) => (*start..=*end).contains(&probe),
            }) != *negated
        }
        Atom::Unicode(negated, items) => {
            items.iter().any(|item| unicode_item_accepts(item, probe)) != *negated
        }
    }
}

// Whether a class item accepts an ASCII letter or digit probe.
fn unicode_item_accepts(item: &UnicodeClassItem, probe: char) -> bool {
    let letter = probe.is_ascii_alphabetic();
    match item {
        UnicodeClassItem::Char(value) => *value == probe,
        UnicodeClassItem::Range(start, end) => (*start..=*end).contains(&probe),
        UnicodeClassItem::Category(category) => match category.as_str() {
            "L" => letter,
            "Lu" => letter && probe.is_ascii_uppercase(),
            "Ll" => letter && probe.is_ascii_lowercase(),
            "N" | "Nd" => !letter,
            _ => false,
        },
        UnicodeClassItem::Script(script) => match script.as_str() {
            "Latin" => letter,
            "Common" => probe.is_ascii_digit(),
            _ => false,
        },
    }
}

// The comparison text of a parser rule: its language-preserving form with
// references printed by `label`, and its parameters.
fn comparison_text(
    rule: &GrammarRule,
    label: &dyn Fn(&str) -> String,
) -> Result<String, GrammarMergeError> {
    let parameters = &rule.attributes.parameters;
    let parameters = if parameters.is_empty() {
        String::new()
    } else {
        format!(" ({})", parameters.join(" "))
    };
    Ok(format!(
        "{}{parameters}",
        normalize(&parser_form(&rule.expr, true), label)?.text
    ))
}

// A parser expression without wrappers, captures, a closing end-of-input
// check, and with the optional items of a short sequence expanded.
fn parser_form(expr: &GrammarExpr, top: bool) -> GrammarExpr {
    let boxed = |inner: &GrammarExpr| Box::new(parser_form(inner, false));
    match expr {
        GrammarExpr::Capture { expr: inner, .. } => parser_form(inner, top),
        GrammarExpr::Sequence(list) => {
            let mut items: Vec<GrammarExpr> =
                list.iter().map(|item| parser_form(item, false)).collect();
            if top && items.len() > 1 && items.last().is_some_and(is_end_of_input) {
                items.pop();
            }
            let optional = items
                .iter()
                .filter(|item| optional_item(item).is_some())
                .count();
            if optional == 0 || optional > MAX_OPTIONALS {
                return GrammarExpr::Sequence(items);
            }
            let mut alternatives: Vec<Vec<GrammarExpr>> = vec![Vec::new()];
            for item in &items {
                if let Some(inner) = optional_item(item) {
                    alternatives = alternatives
                        .into_iter()
                        .flat_map(|prefix| {
                            let mut with = prefix.clone();
                            with.push(inner.clone());
                            [prefix, with]
                        })
                        .collect();
                } else {
                    for prefix in &mut alternatives {
                        prefix.push(item.clone());
                    }
                }
            }
            GrammarExpr::Choice {
                ordered: false,
                alternatives: alternatives
                    .into_iter()
                    .map(GrammarExpr::Sequence)
                    .collect(),
            }
        }
        GrammarExpr::Choice { alternatives, .. } => GrammarExpr::Choice {
            ordered: false,
            alternatives: alternatives
                .iter()
                .map(|item| parser_form(item, top))
                .collect(),
        },
        GrammarExpr::Optional(inner) => GrammarExpr::Optional(boxed(inner)),
        GrammarExpr::ZeroOrMore(inner) => GrammarExpr::ZeroOrMore(boxed(inner)),
        GrammarExpr::OneOrMore(inner) => GrammarExpr::OneOrMore(boxed(inner)),
        GrammarExpr::Repeat { expr, min, max } => GrammarExpr::Repeat {
            expr: boxed(expr),
            min: *min,
            max: *max,
        },
        GrammarExpr::And(inner) => GrammarExpr::And(boxed(inner)),
        GrammarExpr::Not(inner) => GrammarExpr::Not(boxed(inner)),
        GrammarExpr::Feature(feature) => match feature.as_ref() {
            FeatureExpr::Form(form) => wrapped(form).map_or_else(
                || GrammarExpr::Feature(Box::new(FeatureExpr::Form(item_fields(form)))),
                |inner| parser_form(inner, top),
            ),
            _ => expr.clone(),
        },
        _ => expr.clone(),
    }
}

// A feature form with its `items`, or else its `item`, in parser form, as the
// JavaScript `parserForm` rewrites the `items` or `item` of any form.
fn item_fields(form: &FeatureForm) -> FeatureForm {
    let spec = form.spec();
    let key = if spec.iter().any(|(name, _)| *name == "items") {
        "items"
    } else {
        "item"
    };
    let mut copy = form.clone();
    if let Some(position) = spec.iter().position(|(name, _)| *name == key)
        && let Some(value) = copy.fields.get_mut(position)
    {
        match value {
            FieldValue::Expression(inner) => *inner = parser_form(inner, false),
            FieldValue::Expressions(items) => {
                for inner in items {
                    *inner = parser_form(inner, false);
                }
            }
            _ => {}
        }
    }
    copy
}

fn optional_item(expr: &GrammarExpr) -> Option<&GrammarExpr> {
    match expr {
        GrammarExpr::Optional(inner)
        | GrammarExpr::Repeat {
            expr: inner,
            min: 0,
            max: Some(1),
        } => Some(inner),
        _ => None,
    }
}

fn is_end_of_input(expr: &GrammarExpr) -> bool {
    matches!(expr, GrammarExpr::Not(inner) if **inner == GrammarExpr::AnyChar)
}
