//! The reductions an LR parser makes before a token its rule could go on
//! with, as tree-sitter settles a shift-reduce conflict by precedence (see
//! `reduction_facts`).

use std::collections::{HashMap, HashSet};

use super::super::precedence::nullable;
use super::super::program::{Associativity, Expr, Name, Rule};
use super::Lead;
use super::lead_sets::{first_of_items, first_sets, follow_sets, left_corner_follow};
use crate::grammar::RuleKind;

/// The reductions an LR parser makes before a token its rule could go on
/// with (see `reduction_facts`): `splits` holds, by the address and length
/// of the items of each such sequence, where its optional parts begin and
/// the tokens it is reduced before; `keys` all the marked tokens.
#[derive(Debug, Default)]
pub(in crate::grammar::feature_runtime) struct Reductions {
    first: Vec<HashSet<Lead>>,
    pub(in crate::grammar::feature_runtime) splits: HashMap<(usize, usize), Split>,
    pub(in crate::grammar::feature_runtime) keys: HashSet<Lead>,
}

/// A sequence of a left-associative precedence that ends in optional parts
/// (see `reduction_facts`): the index its optional parts begin at, the
/// tokens that follow its rule in every context (`always`, reduced before
/// wherever the parts begin with them) and the tokens that follow it in some
/// (`marked`, reduced before where the enclosing parts may go on with them,
/// see `reduced_early`).
#[derive(Debug)]
pub(in crate::grammar::feature_runtime) struct Split {
    pub(in crate::grammar::feature_runtime) optional_from: usize,
    pub(in crate::grammar::feature_runtime) always: HashSet<Lead>,
    pub(in crate::grammar::feature_runtime) marked: HashSet<Lead>,
}

impl Reductions {
    /// The key of the sequence `items` in `splits`.
    pub(in crate::grammar::feature_runtime) fn key(items: &[Expr]) -> (usize, usize) {
        (items.as_ptr().addr(), items.len())
    }

    /// The marked tokens (see `Split`) the sequence `items` may begin with,
    /// or None when it begins with none. It mirrors the sets restKeys and
    /// iterationKeys in js/src/grammar-runtime/executor.js compute.
    pub(in crate::grammar::feature_runtime) fn marked(
        &self,
        items: &[Expr],
        rules: &[Rule],
    ) -> Option<HashSet<Lead>> {
        let mut keys = HashSet::new();
        first_of_items(items, rules, &self.first, &mut keys);
        keys.retain(|key| self.keys.contains(key));
        (!keys.is_empty()).then_some(keys)
    }
}

/// Adds to `found` the sequences of a left-associative precedence `expr`
/// is, through a choice, a field or an alias (see `reduction_facts`).
fn left_sequences<'e>(expr: &'e Expr, found: &mut Vec<&'e [Expr]>) {
    match expr {
        Expr::Choice { items, .. } => {
            for item in items {
                left_sequences(item, found);
            }
        }
        Expr::Capture { item, .. } | Expr::Alias { item, .. } => left_sequences(item, found),
        Expr::Precedence {
            associativity: Associativity::Left,
            item,
            ..
        } => {
            if let Expr::Seq(items) = &**item {
                found.push(items);
            }
        }
        _ => {}
    }
}

/// The reductions an LR parser makes before a token its rule could go on
/// with, as tree-sitter settles a shift-reduce conflict by precedence: a
/// rule of a left-associative precedence whose sequence ends in optional
/// parts (Lean's `hash_command`, `#check` and any expressions, of level 0
/// left; Rust's `break_expression`) is complete before them, and a token
/// they may begin with that may also follow the rule is shifted by the
/// rule's own item of that precedence and reduced by its completed one, of
/// the same precedence: left associativity reduces (see `Split`). A
/// conflict the grammar declares (`conflicts`) of the rule and a rule that
/// begins with the token keeps both parses (Lean's `hash_command` and
/// `explicit`). It mirrors reductionFacts in
/// js/src/grammar-runtime/executor.js.
pub(super) fn reduction_facts(conflicts: &[HashSet<Name>], rules: &[Rule]) -> Reductions {
    let first = first_sets(rules);
    let (mut follow, mut corner, mut shared) = (None, None, None);
    let mut splits = HashMap::new();
    let mut keys = HashSet::new();
    let begins = |name: &Name, key: &Lead| match rules.iter().position(|rule| rule.name == *name) {
        Some(index) if !matches!(rules[index].kind, RuleKind::Token | RuleKind::Atomic) => {
            first[index].contains(key)
        }
        _ => *key == Lead::Ref(name.clone()),
    };
    let forked = |name: &Name, key: &Lead| {
        conflicts.iter().any(|group| {
            group.contains(name)
                && group
                    .iter()
                    .any(|other| other != name && begins(other, key))
        })
    };
    for (index, rule) in rules.iter().enumerate() {
        if !matches!(rule.kind, RuleKind::Normal) {
            continue;
        }
        let mut sequences = Vec::new();
        left_sequences(&rule.expression, &mut sequences);
        for items in sequences {
            let mut split = items.len();
            while split > 0 && nullable(&items[split - 1]) {
                split -= 1;
            }
            if split == 0 || split == items.len() {
                continue;
            }
            let follow = follow.get_or_insert_with(|| follow_sets(rules, &first));
            let corner = corner.get_or_insert_with(|| left_corner_follow(rules, &first));
            let shared = shared.get_or_insert_with(|| shared_aliases(rules));
            let mut parts = HashSet::new();
            first_of_items(&items[split..], rules, &first, &mut parts);
            parts.retain(|key| {
                follow[index].contains(key) && !shared.contains(key) && !forked(&rule.name, key)
            });
            if parts.is_empty() {
                continue;
            }
            let (always, marked): (HashSet<Lead>, HashSet<Lead>) = parts
                .into_iter()
                .partition(|key| corner[index].contains(key));
            keys.extend(marked.iter().cloned());
            splits.insert(
                Reductions::key(items),
                Split {
                    optional_from: split,
                    always,
                    marked,
                },
            );
        }
    }
    Reductions {
        first,
        splits,
        keys,
    }
}

/// The keys of the aliases that name tokens of different content (Lean's
/// `unnamed_token`, a number and the `#` of a command alike): the token an
/// LR parser sees is the aliased one, so such a key names no one lookahead.
/// It mirrors sharedAliases in js/src/grammar-runtime/executor.js.
fn shared_aliases(rules: &[Rule]) -> HashSet<Lead> {
    let mut contents: HashMap<&Name, HashSet<String>> = HashMap::new();
    let mut pending: Vec<&Expr> = rules.iter().map(|rule| &rule.expression).collect();
    while let Some(expr) = pending.pop() {
        match expr {
            Expr::Alias { name, item } => {
                contents
                    .entry(name)
                    .or_default()
                    .insert(format!("{item:?}"));
                pending.push(item);
            }
            Expr::Seq(items) | Expr::Choice { items, .. } | Expr::Longest(items) => {
                pending.extend(items);
            }
            Expr::Repeat { item, .. }
            | Expr::And(item)
            | Expr::Not(item)
            | Expr::Capture { item, .. }
            | Expr::Precedence { item, .. }
            | Expr::DynamicPrecedence { item, .. }
            | Expr::LexicalPrecedence { item, .. }
            | Expr::Token(item)
            | Expr::ImmediateToken(item)
            | Expr::Predicate { item, .. }
            | Expr::Recover { item, .. }
            | Expr::Missing { item, .. }
            | Expr::Embed { item, .. } => pending.push(item),
            _ => {}
        }
    }
    contents
        .into_iter()
        .filter(|(_, seen)| seen.len() > 1)
        .map(|(name, _)| Lead::Ref(name.clone()))
        .collect()
}
