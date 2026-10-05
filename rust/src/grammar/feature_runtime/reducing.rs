//! The reduce/reduce conflicts of two results that shift the same tokens, and
//! the dynamic precedence their stacks hold on the way, which the order of
//! results reads (see `shift_order`).

use std::cmp::Ordering;
use std::rc::Rc;

use super::ordering::reduction;
use super::program::compare_precedence;
use super::results::{Tree, TreeType};
use super::walk::{Walk, first_meaningful, items, leftmost_chain, meaningful, same_tree};
use crate::grammar::PrecedenceEntry;

/// Which of two nodes of one span an LR parser builds when the first action
/// their parses differ in reduces in both, on the same token: a reduce/reduce
/// conflict, which tree-sitter settles for the higher precedence the two
/// reductions close with (Go's `<-chan int(c)`, whose `int` reduces a `<-
/// chan int` channel type of 6, not a `chan int` one of 0 that a unary `<-`
/// would take). The actions are the leaves the two shift and the nodes they
/// reduce, in post-order; leaves reduced apart by silent rules part the
/// parses before, where this does not decide, and where the two differ only
/// in themselves, the reductions that close them do (see
/// `closing_reduction`).
pub(super) fn first_reduction(
    a: &Rc<Tree>,
    b: &Rc<Tree>,
    orders: &[Vec<PrecedenceEntry>],
) -> Ordering {
    fn steps(tree: &Rc<Tree>, out: &mut Vec<Rc<Tree>>) {
        if tree.trivia {
            return;
        }
        if tree.ty == TreeType::Node {
            for child in items(&tree.children) {
                steps(&child, out);
            }
        }
        out.push(tree.clone());
    }
    let (mut first, mut second) = (Vec::new(), Vec::new());
    steps(a, &mut first);
    steps(b, &mut second);
    let silent = |leaf: &Tree| {
        let mut names: Vec<String> = leaf
            .reduced_to
            .iter()
            .chain(&leaf.forked_to)
            .map(ToString::to_string)
            .collect();
        names.push(leaf.alone.to_string());
        names.join(" ")
    };
    let mut at = 0;
    while let (Some(x), Some(y)) = (first.get(at), second.get(at)) {
        if x.ty != y.ty || x.kind != y.kind || x.start != y.start || x.end != y.end {
            break;
        }
        if x.ty != TreeType::Node && silent(x) != silent(y) {
            break;
        }
        at += 1;
    }
    let (Some(x), Some(y)) = (first.get(at), second.get(at)) else {
        return Ordering::Equal;
    };
    if x.ty != TreeType::Node
        || y.ty != TreeType::Node
        || x.end != y.end
        || at == 0
        || (Rc::ptr_eq(x, a) && Rc::ptr_eq(y, b))
    {
        return Ordering::Equal;
    }
    compare_precedence(&reduction(x), &reduction(y), orders)
}

/// Which of two parses that forked at a declared conflict and end with the
/// same dynamic precedence tree-sitter keeps where they merge: the one whose
/// stack held the higher dynamic precedence at the last token where the two
/// differed. After each token it shifts, tree-sitter puts the version of the
/// higher stack sum first, the sum of the nodes reduced by then, and where two
/// versions merge with links of equal dynamic precedence it keeps the first
/// one's (Go's `a[b](c)`: the `generic_type` `a[b]` of 2 is ahead of the
/// `index_expression` of 1 at `(`, and its `type_conversion_expression` of 1
/// then ties with the `call_expression` of 1). A node's own share, the levels
/// in it outside its child nodes, counts from its end.
pub(super) fn leading_dynamic(a: &Rc<Tree>, b: &Rc<Tree>) -> Ordering {
    fn shares(node: &Tree, out: &mut Vec<(usize, i64)>) -> i64 {
        let mut inner = 0;
        for child in items(&node.children) {
            if child.ty == TreeType::Node {
                inner += shares(&child, out);
            }
        }
        if node.dynamic != inner {
            out.push((node.end, node.dynamic - inner));
        }
        node.dynamic
    }
    fn leaves(node: &Tree, starts: &mut Vec<usize>) {
        for child in items(&node.children) {
            if child.trivia {
                continue;
            }
            if child.ty == TreeType::Node {
                leaves(&child, starts);
            } else {
                starts.push(child.start);
            }
        }
    }
    let (mut first, mut second) = (Vec::new(), Vec::new());
    shares(a, &mut first);
    shares(b, &mut second);
    if first.is_empty() && second.is_empty() {
        return Ordering::Equal;
    }
    let mut starts = Vec::new();
    leaves(a, &mut starts);
    let sum = |list: &[(usize, i64)], at: usize| -> i64 {
        list.iter()
            .filter(|(end, _)| *end <= at)
            .map(|(_, share)| share)
            .sum()
    };
    let mut order = Ordering::Equal;
    for at in starts {
        let difference = sum(&first, at).cmp(&sum(&second, at));
        if difference != Ordering::Equal {
            order = difference;
        }
    }
    order
}

/// Which of two results an LR parser keeps when both shift the same tokens
/// and part on a reduce/reduce conflict at the last of them: `a` is a token
/// whose parse goes on, past tokens alone, with a sibling node that ends where
/// the node `b` of the other parse, which begins with that token, ends (Go's
/// `chan<- chan int`, a `chan <-` channel type of `chan int` against a `chan`
/// channel type of `<- chan int`). Both reduce the nodes along the rightmost
/// chains of that sibling and of `b` on that token, the shared ones alike; the
/// first pair that differs conflicts, and the higher precedence it closes with
/// wins (`<- chan T`'s 6 over the 0 of `chan T`). `walk` holds `a` on top and
/// its siblings below it.
pub(super) fn ending_reduction(
    a: &Rc<Tree>,
    b: &Rc<Tree>,
    walk: &[Walk],
    orders: &[Vec<PrecedenceEntry>],
) -> Ordering {
    if a.ty != TreeType::Token || b.ty != TreeType::Node {
        return Ordering::Equal;
    }
    let head = leftmost_chain(b)
        .last()
        .and_then(|last| first_meaningful(&last.children))
        .unwrap_or_else(|| b.clone());
    if !same_tree(&head, a) {
        return Ordering::Equal;
    }
    let sibling_node = |item: &Rc<Tree>| item.ty == TreeType::Node && !item.trivia;
    let mut sibling = None;
    for step in walk.iter().rev().skip(1) {
        match step {
            Walk::Item(item) if sibling_node(item) => sibling = Some(item.clone()),
            Walk::Item(_) => continue,
            Walk::Part(part) => sibling = items(part).find(sibling_node),
            Walk::End(_) => {}
        }
        if sibling.is_some() || matches!(step, Walk::End(_)) {
            break;
        }
    }
    let Some(sibling) = sibling.filter(|sibling| sibling.end == b.end) else {
        return Ordering::Equal;
    };
    let rightmost = |node: &Rc<Tree>| {
        let mut chain = Vec::new();
        let mut current = Some(node.clone());
        while let Some(next) = current.filter(|next| next.ty == TreeType::Node) {
            current = meaningful(&next.children).pop();
            chain.push(next);
        }
        chain
    };
    let (mine, theirs) = (rightmost(&sibling), rightmost(b));
    let (mut i, mut j) = (mine.len(), theirs.len());
    while i > 0 && j > 0 && same_tree(&mine[i - 1], &theirs[j - 1]) {
        i -= 1;
        j -= 1;
    }
    if i == 0 || j == 0 {
        return Ordering::Equal;
    }
    compare_precedence(&reduction(&mine[i - 1]), &reduction(&theirs[j - 1]), orders)
}
