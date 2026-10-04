//! What the conflicts of two results ask of a program's rules beyond their
//! precedences: the rules each rule takes directly at an edge, the order the
//! rules are defined in and the conflicts the grammar declares, by which an
//! LR parser's reduction before a shift and a generalized LR parser's merge of
//! two forks are known (see `shift_reduction`, `forked_order` and
//! `declared_fork`).

use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use std::sync::Mutex;

use super::ordering::{first_meaningful, meaningful, same_tree};
use super::precedence::nullable;
use super::program::{
    Associativity, Expr, Matcher, Name, PrecedenceTag, Rule, Target, compare_precedence,
};
use super::results::{Tree, TreeType};
use crate::grammar::{PrecedenceEntry, RuleKind};

/// The rules each rule takes directly as its first part, by rule name (see
/// `shift_reduction`), the order the rules are defined in and the groups of
/// rules whose conflicts the grammar declares (see `forked_order`), with
/// what `declared_fork` asks of them. It mirrors grammarFacts in
/// js/src/grammar-runtime/executor.js.
#[derive(Debug, Default)]
pub(super) struct GrammarFacts {
    heads: HashMap<Name, HashSet<Name>>,
    ranks: HashMap<Name, usize>,
    conflicts: Vec<HashSet<Name>>,
    /// The rules each rule takes directly as its last part, by rule name.
    tails: HashMap<Name, Vec<Name>>,
    /// The items of each rule, by rule name (see `shift_items`).
    items: HashMap<Name, Vec<ShiftItem>>,
    /// The verdicts of `declared_fork`, by rule and lookahead.
    forks: Mutex<HashMap<(Name, Lead), bool>>,
}

/// A token a parser may see first, as tree-sitter's FIRST sets name it: a
/// literal by its text, a token rule, an external token or a rule a lexer
/// matches whole by its name.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub(super) enum Lead {
    Literal(Vec<u8>),
    Ref(Name),
}

/// A production of a rule by its first part (see `shift_items`): the part,
/// the production's precedence and the tokens the parts after it can begin
/// with.
#[derive(Debug)]
struct ShiftItem {
    head: Name,
    tag: PrecedenceTag,
    rest: HashSet<Lead>,
}

impl GrammarFacts {
    pub(super) fn new(rules: &[Rule], conflicts: &[Vec<String>]) -> Self {
        let mut heads = HashMap::new();
        let mut ranks = HashMap::new();
        for (index, rule) in rules.iter().enumerate() {
            let mut found = Vec::new();
            direct_edge(
                &rule.expression,
                Associativity::Right,
                None,
                rules,
                &mut found,
                None,
            );
            heads.insert(
                rule.name.clone(),
                found.into_iter().map(|(name, _)| name).collect(),
            );
            ranks.insert(rule.name.clone(), index);
        }
        let conflicts: Vec<HashSet<Name>> = conflicts
            .iter()
            .map(|group| group.iter().map(|name| Name::from(name.as_str())).collect())
            .collect();
        let (mut tails, mut items) = (HashMap::new(), HashMap::new());
        if !conflicts.is_empty() {
            let first = first_sets(rules);
            for rule in rules {
                let mut found = Vec::new();
                direct_edge(
                    &rule.expression,
                    Associativity::Left,
                    Some(&rule.name),
                    rules,
                    &mut found,
                    None,
                );
                tails.insert(
                    rule.name.clone(),
                    found.into_iter().map(|(name, _)| name).collect(),
                );
                let mut own = Vec::new();
                rule_items(
                    &rule.expression,
                    &rule.name,
                    None,
                    &[],
                    rules,
                    &first,
                    &mut own,
                );
                items.insert(rule.name.clone(), own);
            }
        }
        Self {
            heads,
            ranks,
            conflicts,
            tails,
            items,
            forks: Mutex::default(),
        }
    }

    fn rank(&self, rule: Option<&Name>) -> usize {
        rule.and_then(|rule| self.ranks.get(rule))
            .copied()
            .unwrap_or(0)
    }

    /// Whether a declared conflict names both rules.
    pub(super) fn conflicting(&self, a: Option<&Name>, b: Option<&Name>) -> bool {
        let (Some(a), Some(b)) = (a, b) else {
            return false;
        };
        self.conflicts
            .iter()
            .any(|group| group.contains(a) && group.contains(b))
    }
}

/// The name a rule call calls: the rule's own or the external token's.
pub(super) const fn target_name<'r>(target: &'r Target, rules: &'r [Rule]) -> &'r Name {
    match target {
        Target::Rule(index) => &rules[*index].name,
        Target::External(name) => name,
    }
}

/// Adds to `found` the rules `expr` takes directly at its edge facing the
/// operator on `side` (its first part for the right operand, as
/// `edge_rules`), without entering them, each with the precedence of
/// `owner`'s production that takes it there (or None under none). It mirrors
/// directEdge in js/src/grammar-runtime/executor.js.
pub(super) fn direct_edge(
    expr: &Expr,
    side: Associativity,
    owner: Option<&Name>,
    rules: &[Rule],
    found: &mut Vec<(Name, Option<PrecedenceTag>)>,
    tag: Option<&PrecedenceTag>,
) {
    match expr {
        Expr::Ref(target) => found.push((target_name(target, rules).clone(), tag.cloned())),
        Expr::Seq(items) => {
            let ordered: Box<dyn Iterator<Item = &Expr>> = if side == Associativity::Left {
                Box::new(items.iter().rev())
            } else {
                Box::new(items.iter())
            };
            for item in ordered {
                direct_edge(item, side, owner, rules, found, tag);
                if !matches!(item, Expr::Repeat { min: 0, .. }) {
                    break;
                }
            }
        }
        Expr::Choice { items, .. } => {
            for item in items {
                direct_edge(item, side, owner, rules, found, tag);
            }
        }
        Expr::Precedence {
            level,
            name,
            associativity,
            item,
        } => {
            let tag = PrecedenceTag {
                level: *level,
                name: name.clone(),
                associativity: *associativity,
                rule: owner.cloned(),
            };
            direct_edge(item, side, owner, rules, found, Some(&tag));
        }
        Expr::Capture { item, .. }
        | Expr::DynamicPrecedence { item, .. }
        | Expr::Alias { item, .. }
        | Expr::Repeat { item, .. } => direct_edge(item, side, owner, rules, found, tag),
        _ => {}
    }
}

/// Adds to `names` the rules and external tokens `expr` may match at its
/// edge facing the operator on `side`, through the silent rules there, by
/// name, as edgeRules in js/src/grammar-runtime/executor.js.
pub(super) fn edge_names(
    expr: &Expr,
    side: Associativity,
    rules: &[Rule],
    names: &mut HashSet<Name>,
) {
    match expr {
        Expr::Ref(target) => {
            if !names.insert(target_name(target, rules).clone()) {
                return;
            }
            if let Target::Rule(index) = target
                && matches!(rules[*index].kind, RuleKind::Silent)
            {
                edge_names(&rules[*index].expression, side, rules, names);
            }
        }
        Expr::Seq(items) => {
            let ordered: Box<dyn Iterator<Item = &Expr>> = if side == Associativity::Left {
                Box::new(items.iter().rev())
            } else {
                Box::new(items.iter())
            };
            for item in ordered {
                edge_names(item, side, rules, names);
                if !matches!(item, Expr::Repeat { min: 0, .. }) {
                    return;
                }
            }
        }
        Expr::Choice { items, .. } => {
            for item in items {
                edge_names(item, side, rules, names);
            }
        }
        Expr::Capture { item, .. }
        | Expr::Precedence { item, .. }
        | Expr::DynamicPrecedence { item, .. }
        | Expr::Alias { item, .. }
        | Expr::Repeat { item, .. } => edge_names(item, side, rules, names),
        _ => {}
    }
}

/// The silent rule the shift in `progress` takes its first part as where
/// `short` ends with that part reduced to no such rule (TypeScript's `keyof
/// U & V`, whose intersection takes `U` as a `type` where the type query
/// `keyof U` takes it as a `primary_type`), or None: an LR parser reduces the
/// part to that rule, before it can shift, where it reduces `short` instead,
/// and the two reductions conflict. Where the rule of `progress` takes the
/// part as it ends `short`, by its kind or a rule it was reduced to
/// (JavaScript's `new module.Klass()`, whose member expression takes
/// `module` as a `primary_expression`), the shift needs no reduction and none
/// conflicts. It mirrors shiftReduction in js/src/grammar-runtime/executor.js.
pub(super) fn shift_reduction(
    short: &Rc<Tree>,
    progress: &Tree,
    grammar: &GrammarFacts,
) -> Option<Name> {
    let head = first_meaningful(&progress.children)?;
    if head.end != short.end {
        return None;
    }
    let mut node = short.clone();
    while node.ty == TreeType::Node {
        let last = meaningful(&node.children).pop()?;
        if same_tree(&last, &head) {
            let direct = progress
                .rule
                .as_ref()
                .and_then(|rule| grammar.heads.get(rule));
            let taken = |name: &Name| direct.is_some_and(|direct| direct.contains(name));
            if last.kind.as_ref().is_some_and(taken) || last.reduced_to.iter().any(taken) {
                return None;
            }
            return head
                .reduced_to
                .iter()
                .find(|name| !last.reduced_to.contains(name))
                .cloned();
        }
        node = last;
    }
    None
}

/// Adds to `out` the meaningful (not trivia) trees of `tree` in the order
/// a parser shifts and reduces them: each node after its children.
fn steps(tree: &Rc<Tree>, out: &mut Vec<Rc<Tree>>) {
    if tree.trivia {
        return;
    }
    if tree.ty == TreeType::Node {
        for child in tree.children.iter() {
            steps(child, out);
        }
    }
    out.push(tree.clone());
}

/// Which of two nodes of different kinds over the same tokens a generalized
/// LR parser keeps when their parses forked at a conflict the grammar
/// declares, or None when they did not: the two reduce alike up to the first
/// reduction they make apart (TypeScript's `<A>(a): T => a`, the `A` that the
/// arrow function's type parameters reduce to a `type_parameter` and the type
/// assertion's type arguments to a `primary_type`), and when a declared
/// conflict names a rule each reduced to there, both parses go on and
/// tree-sitter keeps the tree of the lower symbol where they merge, as its
/// `ts_subtree_compare` does: here, the rule defined first, unless the two
/// reduce one handle. Greater when `a` is kept, Less when `b` is. It mirrors forkedOrder in
/// js/src/grammar-runtime/executor.js.
pub(super) fn forked_order(
    a: &Rc<Tree>,
    b: &Rc<Tree>,
    grammar: &GrammarFacts,
    bytes: &[u8],
) -> Option<Ordering> {
    if grammar.conflicts.is_empty() {
        return None;
    }
    let (mut first, mut second) = (Vec::new(), Vec::new());
    steps(a, &mut first);
    steps(b, &mut second);
    let same = |x: &Tree, y: &Tree| {
        x.ty == y.ty && x.kind == y.kind && x.start == y.start && x.end == y.end
    };
    let at = first
        .iter()
        .zip(&second)
        .take_while(|(x, y)| same(x, y))
        .count();
    if at == 0 || at == first.len() || at == second.len() {
        return None;
    }
    let reductions = |before: &Tree, other: &Tree, next: &Tree| {
        let mut names: Vec<Name> = before
            .reduced_to
            .iter()
            .filter(|name| !other.reduced_to.contains(name))
            .cloned()
            .collect();
        if next.ty == TreeType::Node
            && next.end == before.end
            && let Some(rule) = &next.rule
        {
            names.push(rule.clone());
        }
        names
    };
    let mine = reductions(&first[at - 1], &second[at - 1], &first[at]);
    let theirs = reductions(&second[at - 1], &first[at - 1], &second[at]);
    let declared = grammar.conflicts.iter().any(|group| {
        mine.iter().any(|name| group.contains(name))
            && theirs.iter().any(|name| group.contains(name))
    });
    // Two reductions of one handle (Lean's `do return x`, a `return` and a
    // `do_return` of the same children) are two reduce actions of one table
    // entry: the version of the last, of the rule defined later, is the one
    // the parser goes on with, and the other merges into it where both shift
    // the next token. With none left but zero-width ones (the `return x` that
    // ends a file's last `do` block), both are accepted and the tree of the
    // lower symbols is kept.
    let follows = bytes
        .get(a.end..)
        .is_some_and(|rest| rest.iter().any(|byte| !matches!(byte, 9..=13 | 32)));
    let handle = at == first.len() - 1 && at == second.len() - 1 && follows;
    declared.then(|| {
        let order = grammar
            .rank(b.rule.as_ref())
            .cmp(&grammar.rank(a.rule.as_ref()));
        if handle { order.reverse() } else { order }
    })
}

/// The tokens each of the `rules` can begin with, by rule index, as
/// tree-sitter's FIRST sets (see `Lead`). It mirrors firstSets in
/// js/src/grammar-runtime/executor.js.
fn first_sets(rules: &[Rule]) -> Vec<HashSet<Lead>> {
    let mut sets = vec![HashSet::new(); rules.len()];
    let mut changed = true;
    while changed {
        changed = false;
        for (index, rule) in rules.iter().enumerate() {
            if matches!(rule.kind, RuleKind::Token | RuleKind::Atomic) {
                continue;
            }
            let mut found = HashSet::new();
            first_of(&rule.expression, rules, &sets, &mut found);
            let size = sets[index].len();
            sets[index].extend(found);
            changed |= sets[index].len() != size;
        }
    }
    sets
}

/// Adds to `found` the tokens `expr` can begin with, by the FIRST sets of
/// the rules known so far (see `first_sets`). It mirrors firstOf in
/// js/src/grammar-runtime/executor.js.
fn first_of(expr: &Expr, rules: &[Rule], sets: &[HashSet<Lead>], found: &mut HashSet<Lead>) {
    match expr {
        Expr::Terminal {
            matcher: Matcher::Literal(text),
            ..
        } => {
            found.insert(Lead::Literal(text.clone()));
        }
        Expr::Ref(Target::External(name)) => {
            found.insert(Lead::Ref(name.clone()));
        }
        Expr::Ref(Target::Rule(index)) => {
            if matches!(rules[*index].kind, RuleKind::Token | RuleKind::Atomic) {
                found.insert(Lead::Ref(rules[*index].name.clone()));
            } else {
                found.extend(sets[*index].iter().cloned());
            }
        }
        Expr::Seq(items) => {
            for item in items {
                first_of(item, rules, sets, found);
                if !nullable(item) {
                    break;
                }
            }
        }
        Expr::Choice { items, .. } => {
            for item in items {
                first_of(item, rules, sets, found);
            }
        }
        Expr::Alias { name, item } => {
            found.insert(Lead::Ref(name.clone()));
            first_of(item, rules, sets, found);
        }
        Expr::Token(item)
        | Expr::ImmediateToken(item)
        | Expr::LexicalPrecedence { item, .. }
        | Expr::Capture { item, .. }
        | Expr::Precedence { item, .. }
        | Expr::DynamicPrecedence { item, .. }
        | Expr::Repeat { item, .. } => first_of(item, rules, sets, found),
        _ => {}
    }
}

/// Adds to `items` the productions of the rule `owner` within `expr`, each
/// by its first part (see `ShiftItem`), where `rest` is what follows `expr`
/// in the production and `tag` the precedence over it. It mirrors the walk
/// of shiftItems in js/src/grammar-runtime/executor.js.
fn rule_items<'e>(
    expr: &'e Expr,
    owner: &Name,
    tag: Option<&PrecedenceTag>,
    rest: &[&'e Expr],
    rules: &[Rule],
    first: &[HashSet<Lead>],
    items: &mut Vec<ShiftItem>,
) {
    match expr {
        Expr::Ref(target) => {
            let mut leads = HashSet::new();
            for part in rest {
                first_of(part, rules, first, &mut leads);
                if !nullable(part) {
                    break;
                }
            }
            items.push(ShiftItem {
                head: target_name(target, rules).clone(),
                tag: tag
                    .cloned()
                    .unwrap_or_else(|| PrecedenceTag::unranked(Some(owner.clone()))),
                rest: leads,
            });
        }
        Expr::Seq(parts) => {
            for (index, part) in parts.iter().enumerate() {
                let after: Vec<&Expr> = parts[index + 1..]
                    .iter()
                    .chain(rest.iter().copied())
                    .collect();
                rule_items(part, owner, tag, &after, rules, first, items);
                if !nullable(part) {
                    break;
                }
            }
        }
        Expr::Choice { items: choices, .. } => {
            for choice in choices {
                rule_items(choice, owner, tag, rest, rules, first, items);
            }
        }
        Expr::Precedence {
            level,
            name,
            associativity,
            item,
        } => {
            let tag = PrecedenceTag {
                level: *level,
                name: name.clone(),
                associativity: *associativity,
                rule: Some(owner.clone()),
            };
            rule_items(item, owner, Some(&tag), rest, rules, first, items);
        }
        Expr::Repeat {
            item, max: Some(1), ..
        }
        | Expr::Capture { item, .. }
        | Expr::DynamicPrecedence { item, .. }
        | Expr::Alias { item, .. } => rule_items(item, owner, tag, rest, rules, first, items),
        Expr::Repeat { item, .. } => {
            let again: Vec<&Expr> = std::iter::once(expr).chain(rest.iter().copied()).collect();
            rule_items(item, owner, tag, &again, rules, first, items);
        }
        _ => {}
    }
}

/// The items of an LR parser that go on after the part `slot`: each
/// production of a rule that may begin where `slot` begins and whose own
/// first part is `slot`, with its rule. It mirrors shiftItems in
/// js/src/grammar-runtime/executor.js.
fn shift_items<'g>(grammar: &'g GrammarFacts, slot: &Name) -> Vec<(&'g Name, &'g ShiftItem)> {
    let mut found = Vec::new();
    let mut seen = HashSet::new();
    let mut pending = vec![slot];
    while let Some(name) = pending.pop() {
        let Some((rule, items)) = grammar.items.get_key_value(name) else {
            continue;
        };
        if !seen.insert(name) {
            continue;
        }
        pending.extend(grammar.heads.get(name).into_iter().flatten());
        found.extend(
            items
                .iter()
                .filter(|item| &item.head == slot)
                .map(|item| (rule, item)),
        );
    }
    found
}

/// The token a parser sees first in `tree`, as its FIRST sets name it (see
/// `Lead`), or None when the tree holds none. It mirrors lookaheadOf in
/// js/src/grammar-runtime/executor.js.
pub(super) fn lookahead_of(tree: Option<&Rc<Tree>>, bytes: &[u8]) -> Option<Lead> {
    let mut token = tree?.clone();
    while token.ty == TreeType::Node {
        token = first_meaningful(&token.children)?;
    }
    Some(token.kind.clone().map_or_else(
        || {
            Lead::Literal(
                bytes
                    .get(token.start..token.end)
                    .unwrap_or_default()
                    .to_vec(),
            )
        },
        Lead::Ref,
    ))
}

/// The first meaningful leaf of `tree` that begins at or after `offset`, or
/// None. It mirrors tokenAt in js/src/grammar-runtime/executor.js.
pub(super) fn token_at(tree: &Rc<Tree>, offset: usize) -> Option<Rc<Tree>> {
    if tree.ty != TreeType::Node {
        return (!tree.trivia && tree.start >= offset).then(|| tree.clone());
    }
    tree.children.iter().find_map(|child| {
        if child.end <= offset && child.end > child.start {
            return None;
        }
        token_at(child, offset)
    })
}

/// Whether a generated parser forks where the left operand `child` ends
/// before the token `lookahead`, as tree-sitter's `handle_conflict` leaves a
/// shift-reduce conflict to the grammar's declared conflicts: the items that
/// shift that token after the part the child's rule ends with (TypeScript's
/// `!g` before `<`: a call, an instantiation and a binary expression after
/// the `expression` `g`) rank some above the child's reduction and some below
/// it, and a declared conflict names the child's rule with all of theirs.
/// Both parses then go on, and the one that reduced the child is kept (see
/// `shift_order`). It mirrors declaredFork in
/// js/src/grammar-runtime/executor.js.
pub(super) fn declared_fork(
    grammar: &GrammarFacts,
    child: &Tree,
    lookahead: Option<Lead>,
    orders: &[Vec<PrecedenceEntry>],
) -> bool {
    let (Some(rule), Some(precedence), Some(lookahead)) =
        (&child.rule, &child.precedence, lookahead)
    else {
        return false;
    };
    let Some(tails) = grammar.tails.get(rule) else {
        return false;
    };
    let key = (rule.clone(), lookahead);
    if let Some(&forks) = grammar
        .forks
        .lock()
        .ok()
        .as_ref()
        .and_then(|forks| forks.get(&key))
    {
        return forks;
    }
    let mut names = HashSet::from([rule]);
    let (mut more, mut less) = (false, false);
    for slot in tails {
        for (name, item) in shift_items(grammar, slot) {
            if !item.rest.contains(&key.1) {
                continue;
            }
            names.insert(name);
            match compare_precedence(&item.tag, precedence, orders) {
                Ordering::Greater => more = true,
                Ordering::Less => less = true,
                Ordering::Equal => {}
            }
        }
    }
    let forks = more
        && less
        && grammar
            .conflicts
            .iter()
            .any(|group| names.iter().all(|name| group.contains(*name)));
    if let Ok(mut memo) = grammar.forks.lock() {
        memo.insert(key, forks);
    }
    forks
}
