//! Walks over the leaves of a result and the comparisons of results by the
//! trees they build, which the order of results and the parting of conflicts
//! read (see `ordering` and `parting`).

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::OnceLock;

use super::grammar_selection::{AcceptedSelection, SelectionMetadata};
use super::grammar_selection_rules::{
    read_grammar_selection_equality_fields, read_grammar_tree_equality_fields,
    read_grammar_tree_recursion_plan,
};
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

/// The pairs of nodes already compared by `same_tree_in`, which a caller
/// keeps while the trees it compares live and none of them changes.
pub(super) type TreeMemo = RefCell<HashMap<(*const Tree, *const Tree), bool>>;

/// Whether two subtrees are the same tree over the same text, whichever of
/// them holds the white space around it.
pub(super) fn same_tree(a: &Rc<Tree>, b: &Rc<Tree>) -> bool {
    same_tree_with(a, b, None)
}

/// `same_tree`, remembering the pairs of nodes it compares in `memo`: two
/// leftmost chains compared node by node share their subtrees, which would
/// otherwise be compared again for every pair above them. It mirrors
/// sameTree in js/src/grammar-runtime/executor.js.
pub(super) fn same_tree_in(a: &Rc<Tree>, b: &Rc<Tree>, memo: &TreeMemo) -> bool {
    same_tree_with(a, b, Some(memo))
}

fn same_tree_with(a: &Rc<Tree>, b: &Rc<Tree>, memo: Option<&TreeMemo>) -> bool {
    if Rc::ptr_eq(a, b) {
        return true;
    }
    if a.ty != b.ty || !same_tree_header(a, b, &grammar_tree_equality_plan(a.ty, false).fields) {
        return false;
    }
    let plan = grammar_tree_equality_plan(a.ty, false);
    if plan.recursion == "leaf" {
        return true;
    }
    let key = (Rc::as_ptr(a), Rc::as_ptr(b));
    if let Some(known) = memo.and_then(|memo| memo.borrow().get(&key).copied()) {
        return known;
    }
    let same = if plan.recursion == "full-embedding" {
        same_embedding(a, b, &TreeMemo::default())
    } else {
        same_children_with(&a.children, &b.children, memo)
    };
    if let Some(memo) = memo {
        memo.borrow_mut().insert(key, same);
    }
    same
}

/// Whether two child lists are the same trees over the same text.
pub(super) fn same_children(a: &Children, b: &Children) -> bool {
    same_children_with(a, b, None)
}

fn same_children_with(a: &Children, b: &Children, memo: Option<&TreeMemo>) -> bool {
    if Rc::ptr_eq(a, b) {
        return true;
    }
    let (first, second) = (meaningful(a), meaningful(b));
    first.len() == second.len()
        && first
            .iter()
            .zip(&second)
            .all(|(x, y)| same_tree_with(x, y, memo))
}

/// Whether two child lists build the same trees, trivia, fields and
/// attributes included: two results that reach them in different ways (Rust's
/// `+.` in a token tree, one run of `(precedence 0 right (repeat1 ...))` or
/// two, both flattened by the silent rule) are one parse, not an ambiguity.
/// It mirrors sameOutput in js/src/grammar-runtime/executor.js.
pub(super) fn same_output(a: &Children, b: &Children) -> bool {
    same_output_with(a, b, &TreeMemo::default())
}

/// Cached projections of the generated portable field and recursive-role plan.
struct GrammarTreeEqualityPlan {
    fields: Vec<String>,
    recursion: String,
}

fn grammar_tree_equality_plan(kind: TreeType, output: bool) -> &'static GrammarTreeEqualityPlan {
    static PLANS: OnceLock<Vec<GrammarTreeEqualityPlan>> = OnceLock::new();
    let plans = PLANS.get_or_init(|| {
        let mut plans = Vec::new();
        for kind in ["node", "token", "error", "missing", "embed"] {
            for output in [false, true] {
                plans.push(GrammarTreeEqualityPlan {
                    fields: read_grammar_tree_equality_fields(kind, output)
                        .split('|')
                        .map(str::to_owned)
                        .collect(),
                    recursion: read_grammar_tree_recursion_plan(kind, output),
                });
            }
        }
        plans
    });
    let index = match kind {
        TreeType::Node => 0,
        TreeType::Token => 1,
        TreeType::Error => 2,
        TreeType::Missing => 3,
        TreeType::Embed => 4,
    };
    &plans[index * 2 + usize::from(output)]
}

fn grammar_selection_equality_fields(alternative: bool) -> &'static [String] {
    static FIELDS: OnceLock<[Vec<String>; 2]> = OnceLock::new();
    &FIELDS.get_or_init(|| {
        [false, true].map(|alternative| {
            read_grammar_selection_equality_fields(alternative)
                .split('|')
                .map(str::to_owned)
                .collect()
        })
    })[usize::from(alternative)]
}

/// Metadata keeps array order, exact key presence and linked object-key order.
fn same_tree_metadata(a: &serde_json::Value, b: &serde_json::Value) -> bool {
    use serde_json::Value;
    match (a, b) {
        (Value::Array(a), Value::Array(b)) => {
            a.len() == b.len() && a.iter().zip(b).all(|(a, b)| same_tree_metadata(a, b))
        }
        (Value::Object(a), Value::Object(b)) => {
            a.len() == b.len()
                && a.iter().zip(b).all(|((key, value), (other_key, other))| {
                    key == other_key && same_tree_metadata(value, other)
                })
        }
        _ => a == b,
    }
}

fn same_tree_header(a: &Tree, b: &Tree, fields: &[String]) -> bool {
    fields.iter().all(|field| match field.as_str() {
        "type" => a.ty == b.ty,
        "kind" => a.kind == b.kind,
        "projectionName" => a.projection_name == b.projection_name,
        "projectionAnonymous" => a.projection_anonymous == b.projection_anonymous,
        "start" => a.start == b.start,
        "end" => a.end == b.end,
        "trivia" => a.trivia == b.trivia,
        "literal" => a.literal == b.literal,
        "field" => a.field == b.field,
        "language" => a.language == b.language,
        "attributes" => a.attributes == b.attributes,
        "lexical" => match (&a.lexical, &b.lexical) {
            (None, None) => true,
            (Some(a), Some(b)) => same_tree_metadata(a, b),
            _ => false,
        },
        _ => false,
    })
}

fn same_output_with(a: &Children, b: &Children, memo: &TreeMemo) -> bool {
    Rc::ptr_eq(a, b)
        || (a.count == b.count
            && items(a)
                .zip(items(b))
                .all(|(a, b)| same_output_tree(&a, &b, memo)))
}

fn same_output_tree(a: &Rc<Tree>, b: &Rc<Tree>, memo: &TreeMemo) -> bool {
    if Rc::ptr_eq(a, b) {
        return true;
    }
    let plan = grammar_tree_equality_plan(a.ty, true);
    if a.ty != b.ty || !same_tree_header(a, b, &plan.fields) {
        return false;
    }
    let key = (Rc::as_ptr(a), Rc::as_ptr(b));
    if let Some(known) = memo.borrow().get(&key).copied() {
        return known;
    }
    let same = match plan.recursion.as_str() {
        "full-children" => same_output_with(&a.children, &b.children, memo),
        "full-embedding" => same_embedding(a, b, memo),
        "leaf" => true,
        _ => false,
    };
    memo.borrow_mut().insert(key, same);
    same
}

fn same_embedding(a: &Tree, b: &Tree, memo: &TreeMemo) -> bool {
    let roots = match (&a.root, &b.root) {
        (None, None) => true,
        (Some(a), Some(b)) => same_output_tree(a, b, memo),
        _ => false,
    };
    roots
        && match (&a.selection, &b.selection) {
            (None, None) => true,
            (Some(a), Some(b)) => same_selection(a, b, memo),
            _ => false,
        }
}

fn same_selection(a: &SelectionMetadata, b: &SelectionMetadata, memo: &TreeMemo) -> bool {
    grammar_selection_equality_fields(false)
        .iter()
        .all(|field| match field.as_str() {
            "mode" => a.mode == b.mode,
            "mapping" => a.mapping == b.mapping,
            "start" => a.start == b.start,
            "end" => a.end == b.end,
            "candidates" => {
                a.candidates.len() == b.candidates.len()
                    && a.candidates
                        .iter()
                        .zip(&b.candidates)
                        .all(|(a, b)| same_tree_metadata(a, b))
            }
            "provenance" => same_tree_metadata(&a.provenance, &b.provenance),
            "ambiguous" => a.ambiguous == b.ambiguous,
            _ => false,
        })
        && a.alternatives.len() == b.alternatives.len()
        && a.alternatives
            .iter()
            .zip(&b.alternatives)
            .all(|(a, b)| same_selection_alternative(a, b, memo))
}

fn same_selection_alternative(
    a: &AcceptedSelection,
    b: &AcceptedSelection,
    memo: &TreeMemo,
) -> bool {
    grammar_selection_equality_fields(true)
        .iter()
        .all(|field| match field.as_str() {
            "language" => a.language == b.language,
            "provenance" => same_tree_metadata(&a.provenance, &b.provenance),
            _ => false,
        })
        && same_output_tree(&a.root, &b.root, memo)
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
