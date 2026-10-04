//! Where two parses of one text part, as `ordering` asks of their trees: a
//! leaf one reduced where the other shifted on, a child of one reduced alone
//! where the other shifted on, and two nodes over the same tokens.

use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

use super::ordering::{
    by_associativity, first_leaf_start, first_meaningful, items, leftmost_chain, meaningful,
    same_tree,
};
use super::program::{Name, PrecedenceTag, compare_precedence};
use super::results::{Children, Tree, TreeType};
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

/// Whether two subtrees hold the same tokens: leaves of one span each, of one
/// kind or built by one token rule.
pub(super) fn same_tokens(a: &Rc<Tree>, b: &Rc<Tree>) -> bool {
    fn leaves(tree: &Rc<Tree>, out: &mut Vec<Rc<Tree>>) {
        if tree.trivia {
            return;
        }
        if tree.ty == TreeType::Node {
            for child in tree.children.iter() {
                leaves(child, out);
            }
        } else {
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
