//! Walks over the leaves of a result and the comparisons of results by the
//! trees they build, which the order of results and the parting of conflicts
//! read (see `ordering` and `parting`).

use std::rc::Rc;

use super::results::{Children, Tree, TreeType};

/// One step of a walk over the leaves of a result: a part of a child list,
/// not yet opened, one tree, or the end of the children of a node the walk
/// entered (the node in progress for the items above it).
pub(super) enum Walk {
    Part(Children),
    Item(Rc<Tree>),
    End(Rc<Tree>),
}

/// Opens the part on top of a walk: its items, or its two linked halves,
/// without flattening it.
pub(super) fn open_part(walk: &mut Vec<Walk>) {
    let Some(Walk::Part(part)) = walk.pop() else {
        return;
    };
    if let Some(items) = part.flat.get() {
        walk.extend(items.iter().rev().cloned().map(Walk::Item));
    } else if let Some((before, after)) = part.link.borrow().clone() {
        walk.push(Walk::Part(after));
        walk.push(Walk::Part(before));
    }
}

/// The item on top of a walk once the parts above it are opened and the
/// ends of nodes passed, without descending into a node.
pub(super) fn next_item(walk: &mut Vec<Walk>) -> Option<Rc<Tree>> {
    loop {
        match walk.last()? {
            Walk::End(_) => {
                walk.pop();
            }
            Walk::Part(_) => open_part(walk),
            Walk::Item(item) => return Some(item.clone()),
        }
    }
}

/// Replaces the node on top of a walk with its children, marking their end
/// when `mark` is set.
pub(super) fn open_node(walk: &mut Vec<Walk>, mark: bool) {
    if let Some(Walk::Item(node)) = walk.pop() {
        let children = node.children.clone();
        if mark {
            walk.push(Walk::End(node));
        }
        walk.push(Walk::Part(children));
    }
}

/// The items of a child list in order, without flattening it.
pub(super) struct Items(Vec<Walk>);

impl Iterator for Items {
    type Item = Rc<Tree>;

    fn next(&mut self) -> Option<Rc<Tree>> {
        loop {
            match self.0.last()? {
                Walk::Part(_) => open_part(&mut self.0),
                Walk::Item(_) => {
                    let Some(Walk::Item(item)) = self.0.pop() else {
                        return None;
                    };
                    return Some(item);
                }
                Walk::End(_) => {
                    self.0.pop();
                }
            }
        }
    }
}

pub(super) fn items(children: &Children) -> Items {
    Items(vec![Walk::Part(children.clone())])
}

/// The meaningful (not trivia) items of a child list.
pub(super) fn meaningful(children: &Children) -> Vec<Rc<Tree>> {
    items(children).filter(|child| !child.trivia).collect()
}

/// Whether two subtrees are the same tree over the same text, whichever of
/// them holds the white space around it.
pub(super) fn same_tree(a: &Rc<Tree>, b: &Rc<Tree>) -> bool {
    if Rc::ptr_eq(a, b) {
        return true;
    }
    if a.ty != b.ty || a.kind != b.kind {
        return false;
    }
    if a.ty != TreeType::Node {
        return a.start == b.start && a.end == b.end;
    }
    same_children(&a.children, &b.children)
}

/// Whether two child lists are the same trees over the same text.
pub(super) fn same_children(a: &Children, b: &Children) -> bool {
    if Rc::ptr_eq(a, b) {
        return true;
    }
    let (first, second) = (meaningful(a), meaningful(b));
    first.len() == second.len() && first.iter().zip(&second).all(|(x, y)| same_tree(x, y))
}

/// Whether two child lists build the same trees, trivia, fields and
/// attributes included: two results that reach them in different ways (Rust's
/// `+.` in a token tree, one run of `(precedence 0 right (repeat1 ...))` or
/// two, both flattened by the silent rule) are one parse, not an ambiguity.
/// It mirrors sameOutput in js/src/grammar-runtime/executor.js.
pub(super) fn same_output(a: &Children, b: &Children) -> bool {
    Rc::ptr_eq(a, b)
        || (a.len() == b.len()
            && a.iter().zip(b.iter()).all(|(first, second)| {
                Rc::ptr_eq(first, second)
                    || (first.ty == second.ty
                        && first.kind == second.kind
                        && first.start == second.start
                        && first.end == second.end
                        && first.trivia == second.trivia
                        && first.field == second.field
                        && first.language == second.language
                        && first.attributes == second.attributes
                        && (first.ty != TreeType::Node
                            || same_output(&first.children, &second.children)))
            }))
}

/// The first child of a list that is not trivia, without flattening it.
pub(super) fn first_meaningful(children: &Children) -> Option<Rc<Tree>> {
    items(children).find(|child| !child.trivia)
}

/// The offset of the first leaf under a node that is not white space.
pub(super) fn first_leaf_start(node: &Rc<Tree>) -> usize {
    let mut current = node.clone();
    while current.ty == TreeType::Node {
        let Some(first) = first_meaningful(&current.children) else {
            return current.start;
        };
        current = first;
    }
    current.start
}

/// The nodes along the leftmost chain of a node: itself, then its first
/// meaningful child while that is a node.
pub(super) fn leftmost_chain(node: &Rc<Tree>) -> Vec<Rc<Tree>> {
    let mut chain = Vec::new();
    let mut current = Some(node.clone());
    while let Some(next) = current.filter(|next| next.ty == TreeType::Node) {
        current = first_meaningful(&next.children);
        chain.push(next);
    }
    chain
}
