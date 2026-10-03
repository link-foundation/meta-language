//! The trees and results of the native executor: the syntax tree it builds,
//! one way an expression matches, the deduplicating result set, and the
//! memo entries and shared resource counters of one parse.

use std::cell::{Cell, OnceCell, RefCell};
use std::cmp::Ordering;
use std::collections::{BTreeMap, HashMap};
use std::fmt;
use std::ops::Deref;
use std::rc::Rc;

use super::operations::{OperationValue, State};
use super::program::{Associativity, Name};

/// The type of a syntax tree node.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum TreeType {
    Node,
    Token,
    Error,
    Missing,
    Embed,
}

/// The children of a node, shared between results.
pub(super) type Children = Rc<ChildList>;

/// A list of children. Copying them on every join made a repetition of n
/// items cost O(n²) time and memory, as each iteration (and every prefix the
/// generalized repetition keeps) copied all the children before it. A join
/// instead links its two parts with their child count, and the list is
/// flattened, once, when it is first read; the link is then dropped. A node
/// built from a result shares its list, so the nodes of the prefixes of a
/// repetition are not flattened either.
#[derive(Default)]
pub(super) struct ChildList {
    count: usize,
    flat: OnceCell<Vec<Rc<Tree>>>,
    link: RefCell<Option<(Children, Children)>>,
}

impl ChildList {
    fn flatten(&self) -> Vec<Rc<Tree>> {
        let Some((left, right)) = self.link.borrow_mut().take() else {
            return Vec::new();
        };
        let mut flat = Vec::with_capacity(self.count);
        let mut pending = vec![right, left];
        while let Some(part) = pending.pop() {
            if let Some(items) = part.flat.get() {
                flat.extend_from_slice(items);
            } else if let Some((before, after)) = part.link.borrow().clone() {
                pending.push(after);
                pending.push(before);
            }
        }
        flat
    }
}

impl Deref for ChildList {
    type Target = [Rc<Tree>];

    fn deref(&self) -> &[Rc<Tree>] {
        self.flat.get_or_init(|| self.flatten())
    }
}

impl fmt::Debug for ChildList {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.debug_list().entries(self.iter()).finish()
    }
}

// Unlinks a long chain one link at a time instead of recursively.
impl Drop for ChildList {
    fn drop(&mut self) {
        let mut next = self.link.get_mut().take();
        while let Some((left, _)) = next {
            next = Rc::try_unwrap(left)
                .ok()
                .and_then(|mut list| list.link.get_mut().take());
        }
    }
}

/// A flat list of children.
pub(super) fn children_of(items: Vec<Rc<Tree>>) -> Children {
    Rc::new(ChildList {
        count: items.len(),
        flat: OnceCell::from(items),
        link: RefCell::new(None),
    })
}

/// The children of `left` followed by those of `right`, linked.
fn join_children(left: &Children, right: &Children) -> Children {
    if right.count == 0 {
        return left.clone();
    }
    if left.count == 0 {
        return right.clone();
    }
    Rc::new(ChildList {
        count: left.count + right.count,
        flat: OnceCell::new(),
        link: RefCell::new(Some((left.clone(), right.clone()))),
    })
}

/// One node of the executor's syntax tree.
#[derive(Clone, Debug)]
pub(super) struct Tree {
    pub(super) ty: TreeType,
    pub(super) kind: Option<Name>,
    pub(super) rule: Option<Name>,
    pub(super) field: Option<Name>,
    pub(super) trivia: bool,
    pub(super) start: usize,
    pub(super) end: usize,
    pub(super) children: Children,
    pub(super) attributes: Option<BTreeMap<String, OperationValue>>,
    pub(super) precedence: Option<(i64, Associativity)>,
    pub(super) ambiguous: bool,
    pub(super) literal: bool,
    pub(super) language: Option<Name>,
    pub(super) root: Option<Rc<Self>>,
    pub(super) program: usize,
}

impl Tree {
    pub(super) fn new(ty: TreeType, kind: Option<Name>, start: usize, end: usize) -> Self {
        Self {
            ty,
            kind,
            rule: None,
            field: None,
            trivia: false,
            start,
            end,
            children: no_children(),
            attributes: None,
            precedence: None,
            ambiguous: false,
            literal: false,
            language: None,
            root: None,
            program: 0,
        }
    }

    /// A node of `kind` built by rule `kind`.
    pub(super) fn node(kind: &Name, start: usize, end: usize, children: Children) -> Self {
        let mut node = Self::new(TreeType::Node, Some(kind.clone()), start, end);
        node.rule = Some(kind.clone());
        node.children = children;
        node
    }

    pub(super) fn trivia_leaf(kind: Option<Name>, start: usize, end: usize) -> Self {
        let mut leaf = Self::new(TreeType::Token, kind, start, end);
        leaf.trivia = true;
        leaf
    }
}

pub(super) fn no_children() -> Children {
    children_of(Vec::new())
}

/// `[...left, ...right]`.
pub(super) fn concat(left: &[Rc<Tree>], right: &[Rc<Tree>]) -> Children {
    let mut joined = Vec::with_capacity(left.len() + right.len());
    joined.extend_from_slice(left);
    joined.extend_from_slice(right);
    children_of(joined)
}

/// `[...leaves, leaf]`.
pub(super) fn with_leaf(leaves: &[Rc<Tree>], leaf: Tree) -> Children {
    concat(leaves, &[Rc::new(leaf)])
}

/// One way an expression matches: its end, the state after it, the trees it built.
#[derive(Clone, Debug)]
pub(super) struct Res {
    pub(super) end: usize,
    pub(super) state: State,
    pub(super) children: Children,
    pub(super) dynamic: i64,
    pub(super) precedence: Option<(i64, Associativity)>,
    pub(super) ambiguous: bool,
    /// The repair cost: 2 per MISSING leaf, the skipped bytes per ERROR leaf.
    pub(super) cost: usize,
}

impl Res {
    pub(super) const fn new(end: usize, state: State, children: Children, dynamic: i64) -> Self {
        Self {
            end,
            state,
            children,
            dynamic,
            precedence: None,
            ambiguous: false,
            cost: 0,
        }
    }

    pub(super) fn join(left: &Self, right: Self, in_token: bool) -> Self {
        Self {
            end: right.end,
            state: right.state,
            children: if in_token {
                no_children()
            } else {
                join_children(&left.children, &right.children)
            },
            dynamic: left.dynamic + right.dynamic,
            precedence: None,
            ambiguous: left.ambiguous || right.ambiguous,
            cost: left.cost + right.cost,
        }
    }

    fn key(&self) -> (usize, State) {
        (self.end, self.state.clone())
    }
}

/// A deduplicating, insertion-ordered set of results keyed by end and state.
#[derive(Clone, Debug, Default)]
pub(super) struct ResultSet {
    pub(super) items: Vec<Res>,
    index: HashMap<(usize, State), usize>,
    /// `(matching longest)`: a tie goes to the result with the longer token.
    longest: bool,
}

impl ResultSet {
    pub(super) fn new(longest: bool) -> Self {
        Self {
            longest,
            ..Self::default()
        }
    }

    /// Of two results with the same end and state, the lower repair cost
    /// wins, then the higher dynamic precedence, then, under `(matching
    /// longest)`, the longer tokens; on a tie the first stays and, without
    /// repairs, is marked ambiguous.
    pub(super) fn add(&mut self, result: Res) {
        match self.index.get(&result.key()) {
            None => self.push(result),
            Some(&position) => {
                let existing = &mut self.items[position];
                if result.cost < existing.cost {
                    *existing = result;
                } else if result.cost > existing.cost {
                } else if result.dynamic > existing.dynamic {
                    *existing = result;
                } else if result.dynamic == existing.dynamic {
                    let order = if self.longest {
                        longer_tokens(&result, existing)
                    } else {
                        Ordering::Equal
                    };
                    if order == Ordering::Greater {
                        *existing = result;
                    } else if order == Ordering::Equal && existing.cost == 0 {
                        existing.ambiguous = true;
                    }
                }
            }
        }
    }

    /// Replaces the result of the same end and state in place, or appends.
    pub(super) fn set(&mut self, result: Res) {
        match self.index.get(&result.key()) {
            Some(&position) => self.items[position] = result,
            None => self.push(result),
        }
    }

    fn push(&mut self, result: Res) {
        self.index.insert(result.key(), self.items.len());
        self.items.push(result);
    }

    pub(super) fn contains(&self, result: &Res) -> bool {
        self.index.contains_key(&result.key())
    }

    pub(super) const fn len(&self) -> usize {
        self.items.len()
    }
}

/// One step of a walk over the leaves of a result: a part of a child list,
/// not yet opened, or one tree.
enum Walk {
    Part(Children),
    Item(Rc<Tree>),
}

/// Opens the part on top of a walk: its items, or its two linked halves,
/// without flattening it.
fn open_part(walk: &mut Vec<Walk>) {
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

/// Replaces the node on top of a walk with its children.
fn open_node(walk: &mut Vec<Walk>) {
    if let Some(Walk::Item(node)) = walk.pop() {
        walk.push(Walk::Part(node.children.clone()));
    }
}

/// Which of two results over the same text has the longer tokens, as a
/// lexer takes the longest token: their leaves are walked in order, skipping
/// trivia and the parts both share, and the first leaf pair that differs
/// decides. Greater when `result` has the longer token, Less when `existing`
/// has, Equal when the pair ends alike but differs in kind (two tokens of one
/// length, which a lexer orders by precedence, not length) or every leaf
/// ends alike. It mirrors longerTokens in js/src/grammar-runtime/executor.js.
fn longer_tokens(result: &Res, existing: &Res) -> Ordering {
    let mut left = vec![Walk::Part(result.children.clone())];
    let mut right = vec![Walk::Part(existing.children.clone())];
    loop {
        match (left.last(), right.last()) {
            (None, _) | (_, None) => return Ordering::Equal,
            (Some(Walk::Part(a)), Some(Walk::Part(b))) if Rc::ptr_eq(a, b) => {
                left.pop();
                right.pop();
            }
            (Some(Walk::Part(_)), _) => open_part(&mut left),
            (_, Some(Walk::Part(_))) => open_part(&mut right),
            (Some(Walk::Item(a)), Some(Walk::Item(b))) => {
                let (a_node, b_node) = (a.ty == TreeType::Node, b.ty == TreeType::Node);
                if Rc::ptr_eq(a, b) {
                    left.pop();
                    right.pop();
                } else if a_node || b_node {
                    if a_node {
                        open_node(&mut left);
                    }
                    if b_node {
                        open_node(&mut right);
                    }
                } else if a.trivia || b.trivia {
                    let (a_trivia, b_trivia) = (a.trivia, b.trivia);
                    if a_trivia {
                        left.pop();
                    }
                    if b_trivia {
                        right.pop();
                    }
                } else if a.end != b.end {
                    return a.end.cmp(&b.end);
                } else if a.kind != b.kind {
                    return Ordering::Equal;
                } else {
                    left.pop();
                    right.pop();
                }
            }
        }
    }
}

pub(super) fn longest_result(results: Vec<Res>) -> Option<Res> {
    let mut best: Option<Res> = None;
    for result in results {
        if best.as_ref().is_none_or(|best| result.end > best.end) {
            best = Some(result);
        }
    }
    best
}

pub(super) fn content_start(children: &[Rc<Tree>], fallback: usize) -> usize {
    children
        .iter()
        .find(|child| !child.trivia)
        .map_or(fallback, |child| child.start)
}

/// The step budget and the frame count of one parse, shared with every
/// embedded-language executor.
#[derive(Debug)]
pub(super) struct Shared {
    pub(super) steps: Cell<usize>,
    pub(super) limit: usize,
    pub(super) frames: Cell<usize>,
    pub(super) memo_limit: usize,
}

/// The outcome of a parse that ran to its end.
#[derive(Debug)]
pub(super) enum Outcome {
    Parsed(Rc<Tree>),
    Failed {
        farthest: usize,
        expected: Vec<String>,
        /// The farthest offset where an element failed without a repair.
        element_farthest: Option<usize>,
        /// While repairing, the root of the result that reaches farthest,
        /// the rest of the input an ERROR leaf.
        partial: Option<Rc<Tree>>,
    },
}

/// One memoized rule call.
#[derive(Debug, Default)]
pub(super) struct Entry {
    pub(super) evaluating: bool,
    pub(super) left_recursive: bool,
    pub(super) involved: bool,
    pub(super) seed: Vec<Res>,
    pub(super) results: Vec<Res>,
}

/// The trivia skipped at one offset.
#[derive(Debug)]
pub(super) struct Skipped {
    pub(super) end: usize,
    pub(super) leaves: Children,
}

/// One run of an external scanner.
#[derive(Debug)]
pub(super) struct Scanned {
    pub(super) token_start: usize,
    pub(super) end: usize,
    pub(super) skipped: Vec<Rc<Tree>>,
    pub(super) state: State,
}

/// A rule call: rule index, offset, state, in a token, and, while
/// repairing, made quietly (where nothing is repaired).
pub(super) type MemoKey = (usize, usize, State, bool, bool);
