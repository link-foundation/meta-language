//! Where two parses of one text part, as `ordering` asks of their trees: a
//! leaf one reduced where the other shifted on, a child of one reduced alone
//! where the other shifted on, and two nodes over the same tokens.

use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

use super::executor::{Executor, Run};
use super::forking::{GrammarFacts, lookahead_of, token_at};
use super::ordering::{by_associativity, reduction, shift_preferred};
use super::program::{Expr, Name, PrecedenceTag, compare_precedence};
use super::results::{Children, Res, Tree, TreeType, children_of, is_separator};
use super::walk::{
    Walk, first_leaf_start, first_meaningful, items, leftmost_chain, meaningful, same_output,
    same_tree,
};
use crate::grammar::PrecedenceEntry;

/// Whether a parse that has `leaf` alone and one that has `node`, which
/// begins with it, part at its end: one of them reduced the leaf, a silent
/// rule alone or as the end of `node` or of a node along its leftmost chain.
/// It mirrors reducedFirst in js/src/grammar-runtime/executor.js.
pub(super) fn reduced_first(node: &Rc<Tree>, leaf: &Tree) -> bool {
    let chain = leftmost_chain(node);
    let first = chain
        .last()
        .and_then(|inner| first_meaningful(&inner.children));
    leaf.alone
        || first.is_some_and(|first| first.start == leaf.start && first.alone)
        || chain.iter().any(|inner| inner.end == leaf.end)
}

/// Whether a node of the kind and span of `inner` is on the leftmost chain of
/// `outer`, which ends with it.
pub(super) fn holds_first(outer: &Rc<Tree>, inner: &Tree) -> bool {
    leftmost_chain(outer)
        .iter()
        .any(|node| node.kind == inner.kind && node.start == inner.start && node.end == inner.end)
}

/// Whether two leaves of one span are one token to the lexer: of one kind,
/// or built by one token rule (JavaScript's `identifier` and its alias
/// `shorthand_property_identifier`).
pub(super) fn one_token(a: &Tree, b: &Tree) -> bool {
    a.kind == b.kind || (a.start == b.start && a.lexed.is_some() && a.lexed == b.lexed)
}

/// Whether a leaf is a token the external scanner scanned of no width.
fn widthless(leaf: &Tree) -> bool {
    leaf.ty != TreeType::Node && leaf.start == leaf.end && leaf.scanned
}

/// Whether two leaves are one token: of one span, and of one kind or built by
/// one token rule.
fn one_leaf(leaf: &Tree, other: &Tree) -> bool {
    leaf.ty == other.ty
        && leaf.start == other.start
        && leaf.end == other.end
        && one_token(leaf, other)
}

/// The first value `visit` gives for the leaves under `children` that are
/// not white space, last first.
fn leaves_backward<T>(
    children: &Children,
    visit: &mut impl FnMut(&Rc<Tree>) -> Option<T>,
) -> Option<T> {
    children
        .iter()
        .rev()
        .filter(|child| !child.trivia)
        .find_map(|child| {
            if child.ty == TreeType::Node {
                leaves_backward(&child.children, visit)
            } else {
                visit(child)
            }
        })
}

/// Whether the first leaf of `children` that is not white space is a token
/// the external scanner scanned of no width.
pub(super) fn starts_widthless(children: &Children) -> bool {
    let mut first = first_meaningful(children);
    while let Some(node) = first.clone().filter(|node| node.ty == TreeType::Node) {
        first = first_meaningful(&node.children);
    }
    first.is_some_and(|leaf| widthless(&leaf))
}

/// Whether the last leaf under `children` a repair may have inserted is a
/// MISSING leaf: the last child that is not a separator, unless tokens the
/// external scanner scanned of no width end `children` (a layout token opening
/// an indented block): then the last leaf before them, however deep in a node.
/// It mirrors `lastRepaired` in js/src/grammar-runtime/executor.js.
pub(super) fn ends_missing(children: &Children) -> bool {
    let mut skipped = false;
    let found = leaves_backward(children, &mut |leaf| {
        if widthless(leaf) {
            skipped = true;
            None
        } else {
            Some(leaf.ty)
        }
    });
    match found {
        Some(ty) if skipped => ty == TreeType::Missing,
        None if skipped => false,
        _ => children
            .iter()
            .rev()
            .find(|child| !is_separator(child))
            .is_some_and(|last| last.ty == TreeType::Missing),
    }
}

/// Whether `children` hold the token `last` and go on past it with a token
/// the lexer lexed, not one the external scanner scanned of no width.
fn lexed_past(children: &Children, last: &Tree) -> bool {
    let mut after: Option<Rc<Tree>> = None;
    leaves_backward(children, &mut |leaf| {
        if leaf.start >= last.end && (leaf.end > last.end || widthless(leaf)) {
            after = Some(leaf.clone());
            None
        } else {
            Some(after.as_ref().is_some_and(|after| !widthless(after)) && one_leaf(leaf, last))
        }
    })
    .unwrap_or(false)
}

/// Which of the results in `continued`, each paired with the results of the
/// next item after it, a token the external scanner scans of no width
/// preempts: where the next item after a result begins with such a token
/// (Lean's layout end after `12` in `def foo := 12` before a line break), an
/// LR parser in the state of that result runs the scanner there before its
/// lexer, and the token it scans is its lookahead, so another result in the
/// same state that holds the same last token and goes on past it with a token
/// the lexer lexed (`12 partial`, an application across the line break) is no
/// parse. A result in another state has another scanner state, which may scan
/// nothing there (a layout end one parse has queued and the other has not).
/// Nor does a repaired result preempt one of a lower repair cost, another
/// version of the parse to tree-sitter's recovery, which keeps the cheaper
/// one (TypeScript's `{ c : 0 @9 , e }`, whose object goes on past `e` where a
/// statement block, repaired at more cost, scans an automatic semicolon).
fn preempted(continued: &[(&Res, Vec<Res>)]) -> Vec<bool> {
    let mut pruned = vec![false; continued.len()];
    for (left, rights) in continued {
        if !rights.iter().any(|right| starts_widthless(&right.children)) {
            continue;
        }
        let Some(last) = leaves_backward(&left.children, &mut |leaf| {
            (!widthless(leaf)).then(|| leaf.clone())
        }) else {
            continue;
        };
        for (index, (other, _)) in continued.iter().enumerate() {
            if other.end > left.end
                && other.state == left.state
                && other.cost >= left.cost
                && !pruned[index]
                && lexed_past(&other.children, &last)
            {
                pruned[index] = true;
            }
        }
    }
    pruned
}

impl Executor<'_> {
    /// Each of `lefts` with the results of `item` after it.
    pub(super) fn continued<'r>(
        &mut self,
        item: &Expr,
        lefts: &'r [Res],
        in_token: bool,
    ) -> Run<Vec<(&'r Res, Vec<Res>)>> {
        let mut continued = Vec::with_capacity(lefts.len());
        for left in lefts {
            continued.push((left, self.continuation(item, left, in_token)?));
        }
        Ok(continued)
    }

    /// Whether the external scanner may scan a token of no width here: outside
    /// a token, in a grammar that has one.
    pub(super) fn scans_widthless(&self, in_token: bool) -> bool {
        !in_token && !self.program.external.is_empty()
    }

    /// Which of `continued` a token the external scanner scans of no width
    /// preempts, as `preempted` tells, where it may scan one.
    pub(super) fn preempted(&self, continued: &[(&Res, Vec<Res>)], in_token: bool) -> Vec<bool> {
        if self.scans_widthless(in_token) {
            preempted(continued)
        } else {
            vec![false; continued.len()]
        }
    }
}

/// Whether two subtrees hold the same tokens: leaves of one span each, of one
/// kind or built by one token rule. A token the external scanner scanned of
/// no width does not count: where one subtree holds it (Lean's layout
/// semicolon in `let y := Foo` before a line break, which a `let` takes
/// before its body), the parser of the other scanned it there too, before its
/// lexer.
pub(super) fn same_tokens(a: &Rc<Tree>, b: &Rc<Tree>) -> bool {
    fn leaves(tree: &Rc<Tree>, out: &mut Vec<Rc<Tree>>) {
        if tree.trivia {
            return;
        }
        if tree.ty == TreeType::Node {
            for child in tree.children.iter() {
                leaves(child, out);
            }
        } else if tree.start != tree.end || !tree.scanned {
            out.push(tree.clone());
        }
    }
    let (mut first, mut second) = (Vec::new(), Vec::new());
    leaves(a, &mut first);
    leaves(b, &mut second);
    first.len() == second.len()
        && first.iter().zip(&second).all(|(leaf, other)| {
            leaf.ty == other.ty
                && leaf.start == other.start
                && leaf.end == other.end
                && one_token(leaf, other)
        })
}

/// Which of two results an LR parser keeps when two nodes of one kind from
/// one offset, which end apart, part before either ends, where their
/// children first differ by end: the parse whose child ends first shifts on
/// in its node, where the other reduced that child alone to the silent rule
/// the longer child's node in progress takes it as (JavaScript's `new f()`
/// before a template, whose `new_expression` shifts `(` as its arguments
/// under `new`, where the call `f()` reduced `f` to an `expression`, which
/// the order ranks below `new`). The precedence of the node that shifts
/// against that reduction's decides, as in `shift_preferred`. Greater when
/// `a`'s result is kept, Less when `b`'s is, Equal when neither. It mirrors
/// childParting in js/src/grammar-runtime/executor.js.
pub(super) fn child_parting(
    a: &Rc<Tree>,
    b: &Rc<Tree>,
    orders: &[Vec<PrecedenceEntry>],
) -> Ordering {
    let (first, second) = (meaningful(&a.children), meaningful(&b.children));
    for (index, (x, y)) in first.iter().zip(&second).enumerate() {
        if same_tree(x, y) {
            continue;
        }
        if first_leaf_start(x) != first_leaf_start(y) || x.end == y.end {
            return Ordering::Equal;
        }
        let (short, long, node, own, kept) = if x.end < y.end {
            (x, y, a, &first, Ordering::Greater)
        } else {
            (y, x, b, &second, Ordering::Less)
        };
        if index + 1 == own.len() {
            return Ordering::Equal;
        }
        let mut progress = long.clone();
        let head = loop {
            if progress.ty != TreeType::Node {
                return Ordering::Equal;
            }
            let Some(head) = first_meaningful(&progress.children) else {
                return Ordering::Equal;
            };
            if same_tree(&head, short) {
                break head;
            }
            progress = head;
        };
        // The reduction in conflict is the first of the head's the short
        // child was not reduced to as well.
        let rule = head
            .reduced_to
            .iter()
            .find(|name| !short.reduced_to.contains(name));
        let (false, Some(rule)) = (progress.end == short.end, rule) else {
            return Ordering::Equal;
        };
        let shifted = node
            .precedence
            .clone()
            .unwrap_or_else(|| PrecedenceTag::unranked(node.rule.clone()));
        // A node's own precedence is its rule's, not the one it was reduced
        // with.
        let reduced = head
            .reduced
            .clone()
            .or_else(|| {
                if head.ty == TreeType::Token {
                    head.precedence.clone()
                } else {
                    None
                }
            })
            .unwrap_or_else(|| PrecedenceTag::unranked(Some(rule.clone())));
        let order = compare_precedence(&shifted, &reduced, orders)
            .then_with(|| by_associativity(reduced.associativity));
        return if kept == Ordering::Greater {
            order
        } else {
            order.reverse()
        };
    }
    Ordering::Equal
}

/// Whether `children`, or a node below them, is a node of the kind and span
/// of `node`. It mirrors hasNode in js/src/grammar-runtime/executor.js.
pub(super) fn has_node(children: &Children, node: &Tree) -> bool {
    items(children).any(|child| {
        child.ty == TreeType::Node
            && ((child.kind == node.kind && child.start == node.start && child.end == node.end)
                || has_node(&child.children, node))
    })
}

/// The first end where two parses part, a node of one kind from one offset
/// that one of them ends there and the other goes on past (the `module` of
/// `declare module "m" {}`, which ends before the body in one), or
/// `usize::MAX`. It mirrors partingEnd in js/src/grammar-runtime/executor.js.
pub(super) fn parting_end(first: &Children, second: &Children) -> usize {
    type Spans = HashMap<(Option<Name>, usize), HashSet<usize>>;
    fn spans(children: &Children, found: &mut Spans) {
        for child in items(children) {
            if child.ty != TreeType::Node {
                continue;
            }
            found
                .entry((child.kind.clone(), child.start))
                .or_default()
                .insert(child.end);
            spans(&child.children, found);
        }
    }
    let (mut mine, mut theirs) = (Spans::new(), Spans::new());
    spans(first, &mut mine);
    spans(second, &mut theirs);
    let mut end = usize::MAX;
    for (key, ends) in &mine {
        let Some(other) = theirs.get(key) else {
            continue;
        };
        for at in ends {
            if !other.contains(at) {
                end = other.iter().fold(end.min(*at), |end, at| end.min(*at));
            }
        }
    }
    end
}

/// Which of two results an LR parser keeps when two nodes from one offset end
/// their leftmost chains apart and nothing else decides them (Rust's closure
/// `|a| b` and or-pattern `|a|b` in a tuple pattern): the two parses part at
/// the first end only one chain has, where one reduced the innermost node
/// ending there (the or-pattern `|a` of level -2) and the other shifted on in
/// the innermost node going past it (the closure parameters `|a|`), as in
/// `shift_preferred`, when the reduced node's children begin the other
/// node's, so the two parses agree up to that end. Two last parts of one kind
/// from one offset where the short node ends (Lean's `g do return x`, where
/// one parse ends the `do`, and the `return` in it, before `x`), or of two
/// rules a conflict declares, are where the conflict is. Greater when `a`'s result is kept, Less when `b`'s is, Equal
/// when neither. It mirrors chainConflict in
/// js/src/grammar-runtime/executor.js.
pub(super) fn chain_conflict(
    a: &Rc<Tree>,
    b: &Rc<Tree>,
    orders: &[Vec<PrecedenceEntry>],
    grammar: &GrammarFacts,
    bytes: &[u8],
) -> Ordering {
    let (first, second) = (leftmost_chain(a), leftmost_chain(b));
    let ends = |chain: &[Rc<Tree>]| {
        chain
            .iter()
            .map(|node| node.end)
            .collect::<HashSet<usize>>()
    };
    let (mine, theirs) = (ends(&first), ends(&second));
    let Some(&end) = mine.symmetric_difference(&theirs).min() else {
        return Ordering::Equal;
    };
    let (reducing, shifting, kept) = if mine.contains(&end) {
        (&first, &second, Ordering::Less)
    } else {
        (&second, &first, Ordering::Greater)
    };
    let (Some(mut short), Some(mut long)) = (
        reducing.iter().rev().find(|node| node.end == end).cloned(),
        shifting.iter().rev().find(|node| node.end > end).cloned(),
    ) else {
        return Ordering::Equal;
    };
    loop {
        let (own, next) = (meaningful(&short.children), meaningful(&long.children));
        if own.len() > next.len() || own.is_empty() {
            return Ordering::Equal;
        }
        let at = own.len() - 1;
        if own[..at].iter().zip(&next).any(|(x, y)| !same_tree(x, y)) {
            return Ordering::Equal;
        }
        let (last, other) = (own[at].clone(), next[at].clone());
        if own.len() < next.len() && same_tree(&last, &other) {
            break;
        }
        if last.ty != TreeType::Node
            || other.ty != TreeType::Node
            || last.end != short.end
            || other.end <= last.end
        {
            // The long node's part at the short one's last may go on past
            // it, as a node that begins with it (Lean's `-x ^ 3 * 7`).
            if other.ty == TreeType::Node && other.end > last.end && begins_with(&other, &last) {
                break;
            }
            return Ordering::Equal;
        }
        // So do two of rules a conflict declares, reductions of one handle
        // (Lean's `do_return` and `return`).
        let forked =
            last.kind != other.kind && grammar.conflicting(last.rule.as_ref(), other.rule.as_ref());
        if (last.kind != other.kind && !forked) || last.start != other.start {
            if begins_with(&other, &last) {
                break;
            }
            return Ordering::Equal;
        }
        (short, long) = (last, other);
    }
    let lookahead = lookahead_of(token_at(&long, short.end).as_ref(), bytes);
    let order = shift_preferred(&long, &short, orders, grammar, lookahead.as_ref());
    if kept == Ordering::Greater {
        order
    } else {
        order.reverse()
    }
}

/// Whether `child` is a subtree along the leftmost chain of `node`. It
/// mirrors beginsWith in js/src/grammar-runtime/executor.js.
fn begins_with(node: &Rc<Tree>, child: &Rc<Tree>) -> bool {
    let mut current = node.clone();
    while current.ty == TreeType::Node {
        let Some(first) = first_meaningful(&current.children) else {
            return false;
        };
        if same_tree(&first, child) {
            return true;
        }
        current = first;
    }
    false
}

/// Of `extra_reduction`, where the other result goes on past `b` in a node
/// of its own (Lean's `(f x).y` in a command, the projection `.y` with no term
/// after the command's `(f x)`, where the projection of level 90 takes
/// `(f x)` as its term): it reduced `b` where `parent` shifted on, and the
/// shift's precedence against the reduction's decides, as in
/// `shift_preferred`. `own` are the children of `parent`, `next` the ones
/// that follow `b` in the other result, `other` the precedence it reduced
/// with. It mirrors the fallback of extraReduction in
/// js/src/grammar-runtime/executor.js.
pub(super) fn shifted_past(
    parent: &Tree,
    own: &[Rc<Tree>],
    next: &[Rc<Tree>],
    other: &PrecedenceTag,
    orders: &[Vec<PrecedenceEntry>],
) -> Ordering {
    let (Some(mine), Some(theirs)) = (own.get(1), next.get(1)) else {
        return Ordering::Equal;
    };
    if theirs.ty != TreeType::Node
        || same_tree(mine, theirs)
        || first_leaf_start(mine) != first_leaf_start(theirs)
        || theirs.end != parent.end
    {
        return Ordering::Equal;
    }
    // Unless the other builds the rest of `parent` as one node, the same
    // tokens by another name (Solidity's `revert Error();`, whose `()` the
    // silent `call_arguments` of a call takes where the other parse aliases
    // it to `revert_arguments`): both shift its tokens, and the conflict is
    // at its end, where `a`'s result reduces `parent` and the other shifts
    // on: the reduced node's associativity decides on equal levels.
    if next.len() == 2
        && same_output(
            &children_of(own[1..].to_vec()),
            &children_of(meaningful(&theirs.children)),
        )
    {
        let reduced = reduction(parent);
        return compare_precedence(&reduced, other, orders)
            .then_with(|| by_associativity(reduced.associativity).reverse());
    }
    let shifted = parent
        .precedence
        .clone()
        .unwrap_or_else(|| PrecedenceTag::unranked(parent.rule.clone()));
    compare_precedence(&shifted, other, orders).then_with(|| by_associativity(other.associativity))
}

/// Which of two results an LR parser keeps when one of them ended a silent
/// rule under a precedence with the token `a` (Lean's `pp` in `set_option
/// pp.all true`, a `name` of level 0 right before the projection `.all`)
/// where the other shifted on over the same token `b` in the same silent
/// rule, which a later item of the other result (`walk` holds its place) ends
/// with that precedence (the `all` of the name `pp.all`): the two are that
/// rule reduced from one offset and ending apart, and on its equal levels the
/// associativity of the rule decides, right to shift and left to reduce.
/// Greater when `a`'s result is kept, Less when `b`'s is, Equal when neither.
/// It mirrors silentParting in js/src/grammar-runtime/executor.js.
pub(super) fn silent_parting(a: &Tree, b: &Tree, walk: &[Walk]) -> Ordering {
    let reduced = a.reduced.as_ref().or(a.precedence.as_ref());
    let Some(reduced) = reduced.filter(|tag| tag.rule.is_some()) else {
        return Ordering::Equal;
    };
    if b.precedence.is_some() || b.reduced.is_some() {
        return Ordering::Equal;
    }
    let decide = |item: &Tree| {
        let closes = if item.ty == TreeType::Node {
            item.closes.as_ref()
        } else {
            item.reduced.as_ref().or(item.precedence.as_ref())
        }?;
        if closes.rule != reduced.rule || !PrecedenceTag::same(Some(closes), Some(reduced)) {
            return Some(Ordering::Equal);
        }
        Some(by_associativity(reduced.associativity).reverse())
    };
    for step in walk.iter().rev().skip(1) {
        let found = match step {
            Walk::Item(item) if !item.trivia => decide(item),
            Walk::Item(_) => None,
            Walk::Part(part) => items(part)
                .filter(|item| !item.trivia)
                .find_map(|item| decide(&item)),
            Walk::End(_) => return Ordering::Equal,
        };
        if let Some(order) = found {
            return order;
        }
    }
    Ordering::Equal
}

/// Which of two results an LR parser keeps when one of them reduces a node
/// the other does not build: `a` is that node when `b`, at the same offset in
/// the other result, is a subtree on its leftmost chain and the children of
/// the innermost such node are, subtree for subtree, the next children of the
/// other result's node in progress (`walk` holds its place). The two
/// reductions of the same text then conflict at the end of that node, which
/// one result reduces where the other reduces or shifts in its own node: the
/// higher precedence level wins and, on equal levels where the other node
/// goes on, the associativity of the reduced node. The results themselves
/// are in progress under `owner`, the precedence a precedence expression
/// holds them with (TypeScript's `extends A<X>`, whose
/// `_extends_clause_single` takes `A` and `<X>` under the name `extends`
/// where an `instantiation_expression` reduces them under `instantiation`,
/// which the order ranks below), else under none. Greater when `a`'s result
/// is kept, Less when the other is, Equal when neither. It mirrors
/// extraReduction in js/src/grammar-runtime/executor.js.
pub(super) fn extra_reduction(
    a: &Rc<Tree>,
    b: &Rc<Tree>,
    walk: &[Walk],
    orders: &[Vec<PrecedenceEntry>],
    owner: Option<&PrecedenceTag>,
    grammar: &GrammarFacts,
) -> Ordering {
    if a.ty != TreeType::Node {
        return Ordering::Equal;
    }
    let mut parent = a.clone();
    let mut chain = vec![a.clone()];
    loop {
        let Some(first) = first_meaningful(&parent.children) else {
            return Ordering::Equal;
        };
        if same_tree(&first, b) {
            chain.push(first);
            break;
        }
        if first.ty != TreeType::Node {
            return Ordering::Equal;
        }
        parent = first.clone();
        chain.push(first);
    }
    let own = meaningful(&parent.children);
    // The next children of the node in progress, from `b` on, and that node.
    let mut next = Vec::new();
    let mut container = None;
    for step in walk.iter().rev() {
        if next.len() >= own.len() {
            break;
        }
        match step {
            Walk::Item(item) => {
                if !item.trivia {
                    next.push(item.clone());
                }
            }
            Walk::Part(part) => {
                let wanted = own.len() - next.len();
                next.extend(items(part).filter(|item| !item.trivia).take(wanted));
            }
            Walk::End(node) => {
                container = Some(node.clone());
                break;
            }
        }
    }
    if container.is_none()
        && let Some(Walk::End(node)) = walk.iter().rev().find(|step| matches!(step, Walk::End(_)))
    {
        container = Some(node.clone());
    }
    let other = container.as_ref().map_or_else(
        || owner.cloned().unwrap_or(PrecedenceTag::unranked(None)),
        |node| {
            node.precedence
                .clone()
                .unwrap_or_else(|| PrecedenceTag::unranked(node.rule.clone()))
        },
    );
    if own.len() > next.len()
        || own
            .iter()
            .zip(&next)
            .any(|(mine, theirs)| !same_tree(mine, theirs))
    {
        return shifted_past(&parent, &own, &next, &other, orders);
    }
    let mine = reduction(&parent);
    let order = compare_precedence(&mine, &other, orders);
    if order != Ordering::Equal {
        return order;
    }
    // With nothing after either, where a silent rule a declared conflict
    // names reduced an item of the chain alone (Solidity's `a;`, an
    // `identifier` that `_identifier_path` reduces in a `user_defined_type`
    // where the other parse reduces it to `_primary_expression`), the two
    // reduce apart on one lookahead: tree-sitter goes on with both and keeps
    // the tree of the lower symbol where they merge, a token before any rule
    // and then the rule defined first.
    if container.is_none() {
        let forked: Vec<Name> = chain
            .iter()
            .flat_map(|item| item.forked_to.iter())
            .filter(|name| !b.forked_to.contains(name))
            .cloned()
            .collect();
        if grammar.declares_any(&forked) && rest_count(walk) == own.len() {
            if b.ty != TreeType::Node {
                return Ordering::Less;
            }
            return grammar
                .rank(b.rule.as_ref())
                .cmp(&grammar.rank(a.rule.as_ref()));
        }
    }
    if container.is_none_or(|node| node.end > parent.end) {
        return by_associativity(mine.associativity).reverse();
    }
    Ordering::Equal
}

/// The count of the meaningful items left in the walk of the node in
/// progress (see `extra_reduction`).
fn rest_count(walk: &[Walk]) -> usize {
    let mut count = 0;
    for step in walk.iter().rev() {
        match step {
            Walk::Item(item) => count += usize::from(!item.trivia),
            Walk::Part(part) => count += items(part).filter(|item| !item.trivia).count(),
            Walk::End(_) => break,
        }
    }
    count
}
