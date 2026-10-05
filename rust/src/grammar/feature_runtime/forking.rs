//! What the conflicts of two results ask of a program's rules beyond their
//! precedences: the rules each rule takes directly at an edge, the order the
//! rules are defined in and the conflicts the grammar declares, by which an
//! LR parser's reduction before a shift and a generalized LR parser's merge of
//! two forks are known (see `shift_reduction`, `forked_order` and
//! `declared_fork`).

use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use std::sync::{Mutex, OnceLock};

mod lead_sets;

use super::precedence::nullable;
use super::program::{Associativity, Expr, Name, PrecedenceTag, Rule, Target, compare_precedence};
use super::results::{ChildList, Children, Tree, TreeType};
use super::walk::{first_meaningful, meaningful, same_tree};
use crate::grammar::{PrecedenceEntry, RuleKind};
use lead_sets::{first_of, first_of_items, first_sets, follow_sets, left_corner_follow};

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
    /// The rules each rule is directly as a whole, by rule name (see
    /// `unit_closure`).
    wholes: HashMap<Name, Vec<Name>>,
    /// The verdicts of `items_order`, by the reduction, the lookahead and the
    /// parts the items go on after.
    item_orders: Mutex<HashMap<ItemsKey, Ordering>>,
    /// The verdicts of `declared_fork`, by rule and lookahead.
    forks: Mutex<HashMap<(Name, Lead), bool>>,
    /// The reductions before a following token, found once asked for (see
    /// `reduction_facts`).
    reductions: OnceLock<Reductions>,
}

/// A token a parser may see first, as tree-sitter's FIRST sets name it: a
/// literal by its text, a token rule, an external token or a rule a lexer
/// matches whole by its name.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub(super) enum Lead {
    Literal(Vec<u8>),
    Ref(Name),
}

/// What `items_order` decides by: the reduction's rule, level and name, the
/// lookahead and the parts the items go on after.
type ItemsKey = (Name, i64, Option<Name>, Lead, Vec<Name>);

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
    /// `lr` asks for the items of an LR parser even where no conflict is
    /// declared, as a grammar settled by precedence compares a reduction with
    /// them (see `items_order`).
    pub(super) fn new(rules: &[Rule], conflicts: &[Vec<String>], lr: bool) -> Self {
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
        let (mut tails, mut items, mut wholes) = (HashMap::new(), HashMap::new(), HashMap::new());
        if lr || !conflicts.is_empty() {
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
                let mut whole = Vec::new();
                whole_refs(&rule.expression, rules, &mut whole);
                wholes.insert(rule.name.clone(), whole);
            }
        }
        Self {
            heads,
            ranks,
            conflicts,
            tails,
            items,
            wholes,
            item_orders: Mutex::default(),
            forks: Mutex::default(),
            reductions: OnceLock::new(),
        }
    }

    fn rank(&self, rule: Option<&Name>) -> usize {
        rule.and_then(|rule| self.ranks.get(rule))
            .copied()
            .unwrap_or(0)
    }

    /// The reductions an LR parser makes before a token their rule could go
    /// on with (see `reduction_facts`), by the program's `rules`.
    pub(super) fn reductions(&self, rules: &[Rule]) -> &Reductions {
        self.reductions
            .get_or_init(|| reduction_facts(&self.conflicts, rules))
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
    // An item one parse reduces to a silent rule a conflict declares and the
    // other shifts on in its node (Lean's `let x := 2`, whose `x` a `do_let`
    // reduces to a `pattern` and a `let` takes as its name): the parses fork
    // there, between that rule and the node's.
    let enclosing = |list: &[Rc<Tree>], index: usize| {
        list[index + 1..]
            .iter()
            .find(|step| {
                step.ty == TreeType::Node
                    && step.start <= list[index].start
                    && step.end >= list[index].end
            })
            .and_then(|step| step.rule.clone())
    };
    let forks = |own: &Tree, other: &Tree| -> Vec<Name> {
        own.forked_to
            .iter()
            .filter(|name| !other.forked_to.contains(name))
            .cloned()
            .collect()
    };
    for index in 0..at {
        let mut mine = forks(&first[index], &second[index]);
        let mut theirs = forks(&second[index], &first[index]);
        if mine.len() == theirs.len() {
            continue;
        }
        if mine.is_empty() {
            mine.extend(enclosing(&first, index));
        } else {
            theirs.extend(enclosing(&second, index));
        }
        if grammar.conflicts.iter().any(|group| {
            mine.iter().any(|name| group.contains(name))
                && theirs.iter().any(|name| group.contains(name))
        }) {
            return Some(
                grammar
                    .rank(b.rule.as_ref())
                    .cmp(&grammar.rank(a.rule.as_ref())),
            );
        }
    }
    let reductions = |before: &Tree, other: &Tree, next: &Tree| {
        let mut names: Vec<Name> = before
            .reduced_to
            .iter()
            .chain(&before.forked_to)
            .filter(|name| !other.reduced_to.contains(name) && !other.forked_to.contains(name))
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

/// The reductions an LR parser makes before a token its rule could go on
/// with (see `reduction_facts`): `splits` holds, by the address and length
/// of the items of each such sequence, where its optional parts begin and
/// the tokens it is reduced before; `keys` all the marked tokens.
#[derive(Debug, Default)]
pub(super) struct Reductions {
    first: Vec<HashSet<Lead>>,
    pub(super) splits: HashMap<(usize, usize), Split>,
    pub(super) keys: HashSet<Lead>,
}

/// A sequence of a left-associative precedence that ends in optional parts
/// (see `reduction_facts`): the index its optional parts begin at, the
/// tokens that follow its rule in every context (`always`, reduced before
/// wherever the parts begin with them) and the tokens that follow it in some
/// (`marked`, reduced before where the enclosing parts may go on with them,
/// see `reduced_early`).
#[derive(Debug)]
pub(super) struct Split {
    pub(super) optional_from: usize,
    pub(super) always: HashSet<Lead>,
    pub(super) marked: HashSet<Lead>,
}

impl Reductions {
    /// The key of the sequence `items` in `splits`.
    pub(super) fn key(items: &[Expr]) -> (usize, usize) {
        (items.as_ptr().addr(), items.len())
    }

    /// The marked tokens (see `Split`) the sequence `items` may begin with,
    /// or None when it begins with none. It mirrors the sets restKeys and
    /// iterationKeys in js/src/grammar-runtime/executor.js compute.
    pub(super) fn marked(&self, items: &[Expr], rules: &[Rule]) -> Option<HashSet<Lead>> {
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
fn reduction_facts(conflicts: &[HashSet<Name>], rules: &[Rule]) -> Reductions {
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

/// Adds to `found` the rules `expr` is directly as a whole: a rule an
/// alternative of it is alone (see `unit_closure`).
fn whole_refs(expr: &Expr, rules: &[Rule], found: &mut Vec<Name>) {
    match expr {
        Expr::Ref(target) => found.push(target_name(target, rules).clone()),
        Expr::Choice { items, .. } => {
            for item in items {
                whole_refs(item, rules, found);
            }
        }
        Expr::Seq(items) if items.len() == 1 && !nullable(&items[0]) => {
            whole_refs(&items[0], rules, found);
        }
        Expr::Precedence { item, .. }
        | Expr::Capture { item, .. }
        | Expr::DynamicPrecedence { item, .. } => whole_refs(item, rules, found),
        _ => {}
    }
}

/// The rules `slot` may be as a whole, itself included: a rule an alternative
/// of it is alone, through such rules (Rocq's `ltac_expression`, a
/// `tactic_invocation` among others). It mirrors unitClosure in
/// js/src/grammar-runtime/executor.js.
fn unit_closure<'g>(grammar: &'g GrammarFacts, slot: &'g Name) -> HashSet<&'g Name> {
    let mut units = HashSet::new();
    let mut pending = vec![slot];
    while let Some(name) = pending.pop() {
        if units.insert(name) {
            pending.extend(grammar.wholes.get(name).into_iter().flatten());
        }
    }
    units
}

/// How the items that shift `lookahead` after the part a node of `short`'s
/// rightmost chain reduced with `reduced` ends with rank against that
/// reduction, as tree-sitter's `handle_conflict` ranks a shift-reduce
/// conflict: Greater when some rank above it and none below, Less when some
/// rank below it and none above, else Equal. A precedence of none and a named
/// one are incomparable, so such an item ranks neither way. A node that ends
/// with a token (Rust's range `a + b..`) has no part an item goes on after.
/// It mirrors itemsOrder in js/src/grammar-runtime/executor.js.
pub(super) fn items_order(
    grammar: &GrammarFacts,
    short: &Rc<Tree>,
    reduced: &PrecedenceTag,
    lookahead: Option<&Lead>,
    orders: &[Vec<PrecedenceEntry>],
) -> Ordering {
    let (Some(rule), Some(lookahead)) = (&reduced.rule, lookahead) else {
        return Ordering::Equal;
    };
    let Some(tails) = grammar.tails.get(rule) else {
        return Ordering::Equal;
    };
    let mut node = short.clone();
    while node.ty == TreeType::Node && node.rule.as_ref() != Some(rule) {
        let Some(last) = node
            .children
            .iter()
            .rev()
            .find(|child| !child.trivia)
            .cloned()
        else {
            return Ordering::Equal;
        };
        node = last;
    }
    let Some(last) = (node.ty == TreeType::Node)
        .then(|| node.children.iter().rev().find(|child| !child.trivia))
        .flatten()
        .filter(|last| last.ty == TreeType::Node)
    else {
        return Ordering::Equal;
    };
    let slots: Vec<Name> = tails
        .iter()
        .filter(|slot| {
            last.reduced_to.contains(slot)
                || last
                    .rule
                    .as_ref()
                    .is_some_and(|own| unit_closure(grammar, slot).contains(own))
        })
        .cloned()
        .collect();
    let key = (
        rule.clone(),
        reduced.level,
        reduced.name.clone(),
        lookahead.clone(),
        slots,
    );
    if let Some(&order) = grammar
        .item_orders
        .lock()
        .ok()
        .as_ref()
        .and_then(|memo| memo.get(&key))
    {
        return order;
    }
    let (mut more, mut less) = (false, false);
    for slot in &key.4 {
        for (_, item) in shift_items(grammar, slot) {
            if !item.rest.contains(lookahead) {
                continue;
            }
            match compare_precedence(&item.tag, reduced, orders) {
                Ordering::Greater => more = true,
                Ordering::Less => less = true,
                Ordering::Equal => {}
            }
        }
    }
    let order = match (more, less) {
        (true, false) => Ordering::Greater,
        (false, true) => Ordering::Less,
        _ => Ordering::Equal,
    };
    if let Ok(mut memo) = grammar.item_orders.lock() {
        memo.insert(key, order);
    }
    order
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

/// Whether a rule on the right edge of `children` was reduced before its
/// optional parts in the parse that goes on with one of the tokens `keys`
/// (Lean's `#check` before `@`, which may begin the next command's
/// attributes): an LR parser decides by that lookahead alone, so the rule
/// never took the parts that begin with it (see `reduction_facts`). It
/// mirrors reducedEarly in js/src/grammar-runtime/executor.js.
pub(super) fn reduced_early(children: &Children, keys: &HashSet<Lead>) -> bool {
    let mut children = children.clone();
    loop {
        let Some(last) = children.iter().rev().find(|child| !child.trivia).cloned() else {
            return false;
        };
        if last.ty != TreeType::Node {
            return false;
        }
        if last
            .before
            .as_ref()
            .is_some_and(|before| keys.contains(before))
        {
            return true;
        }
        children = last.children.clone();
    }
}

/// The token a parser sees first among `children` at or after `offset`, as
/// `lookahead_of` names it, or None. It mirrors lookaheadAfter in
/// js/src/grammar-runtime/executor.js.
pub(super) fn lookahead_after(children: &ChildList, offset: usize, bytes: &[u8]) -> Option<Lead> {
    children
        .iter()
        .find_map(|child| token_at(child, offset))
        .and_then(|found| lookahead_of(Some(&found), bytes))
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
