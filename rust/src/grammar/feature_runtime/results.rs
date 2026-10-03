//! The trees and results of the native executor: the syntax tree it builds,
//! one way an expression matches, the deduplicating result set, and the
//! memo entries and shared resource counters of one parse.

use std::cell::{Cell, OnceCell, RefCell};
use std::cmp::Ordering;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::fmt;
use std::ops::Deref;
use std::rc::Rc;

use super::operations::{OperationValue, State};
use super::program::{Associativity, Name, TokenRank, TokenRanks};

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
    /// The innermost precedence over the node's last part, which its rule
    /// reduces with (see `reduction`).
    pub(super) tail: Option<(i64, Associativity)>,
    /// A token leaf's lexical precedence where it was matched under one
    /// (see `token_rank`).
    pub(super) priority: Option<i64>,
    /// The precedence a token leaf ending a silent rule was reduced with
    /// (see `lone_reduction`).
    pub(super) reduced: Option<(i64, Associativity)>,
    pub(super) ambiguous: bool,
    pub(super) literal: bool,
    /// Under keyword lexing, the kind of the token rule that built the leaf,
    /// which an alias keeps.
    pub(super) lexed: Option<Name>,
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
            tail: None,
            priority: None,
            reduced: None,
            ambiguous: false,
            literal: false,
            lexed: None,
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
    /// The innermost precedence over the last part of the result (see
    /// `reduction`).
    pub(super) tail: Option<(i64, Associativity)>,
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
            tail: None,
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
            tail: if in_token {
                None
            } else if right.children.count == 0 {
                left.tail
            } else {
                right.tail
            },
            ambiguous: left.ambiguous || right.ambiguous,
            cost: left.cost + right.cost,
        }
    }

    fn key(&self) -> (usize, State) {
        (self.end, self.state.clone())
    }
}

/// The token ranks of a `(matching longest)` grammar and the input bytes, by
/// which a tie between results goes to the tokens a lexer prefers.
#[derive(Clone, Copy, Debug)]
pub(super) struct TokenOrder<'c> {
    pub(super) ranks: &'c TokenRanks,
    pub(super) bytes: &'c [u8],
}

/// A deduplicating, insertion-ordered set of results keyed by end and state.
#[derive(Clone, Debug, Default)]
pub(super) struct ResultSet<'c> {
    pub(super) items: Vec<Res>,
    index: HashMap<(usize, State), usize>,
    /// `(matching longest)`: a tie goes to the tokens a lexer prefers.
    tokens: Option<TokenOrder<'c>>,
}

impl<'c> ResultSet<'c> {
    pub(super) fn new(tokens: Option<TokenOrder<'c>>) -> Self {
        Self {
            tokens,
            ..Self::default()
        }
    }

    /// Of two results with the same end and state, the lower repair cost
    /// wins, then, under `(matching longest)`, the tokens a lexer prefers (a
    /// lexer decides them before any parse does) and the shift or reduction
    /// an LR parser keeps by precedence, then the higher dynamic precedence; on a tie the first stays and, without repairs, is marked
    /// ambiguous. True when `result` was kept.
    pub(super) fn add(&mut self, result: Res) -> bool {
        match self.index.get(&result.key()) {
            None => self.push(result),
            Some(&position) => {
                let existing = &mut self.items[position];
                if result.cost > existing.cost {
                    return false;
                }
                if result.cost == existing.cost {
                    let order = self
                        .tokens
                        .map_or(Ordering::Equal, |tokens| {
                            preferred_tokens(&result.children, &existing.children, tokens)
                                .then_with(|| shift_order(&result.children, &existing.children))
                        })
                        .then(result.dynamic.cmp(&existing.dynamic));
                    if order != Ordering::Greater {
                        if order == Ordering::Equal && existing.cost == 0 {
                            existing.ambiguous = true;
                        }
                        return false;
                    }
                }
                *existing = result;
            }
        }
        true
    }

    /// The result of the same end and state as `result`, if any.
    pub(super) fn get(&self, result: &Res) -> Option<&Res> {
        self.index
            .get(&result.key())
            .map(|&position| &self.items[position])
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

    pub(super) const fn len(&self) -> usize {
        self.items.len()
    }
}

/// One step of a walk over the leaves of a result: a part of a child list,
/// not yet opened, one tree, or the end of the children of a node the walk
/// entered (the node in progress for the items above it).
enum Walk {
    Part(Children),
    Item(Rc<Tree>),
    End(Rc<Tree>),
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

/// Replaces the node on top of a walk with its children, marking their end
/// when `mark` is set.
fn open_node(walk: &mut Vec<Walk>, mark: bool) {
    if let Some(Walk::Item(node)) = walk.pop() {
        let children = node.children.clone();
        if mark {
            walk.push(Walk::End(node));
        }
        walk.push(Walk::Part(children));
    }
}

/// The items of a child list in order, without flattening it.
struct Items(Vec<Walk>);

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

fn items(children: &Children) -> Items {
    Items(vec![Walk::Part(children.clone())])
}

/// The meaningful (not trivia) items of a child list.
fn meaningful(children: &Children) -> Vec<Rc<Tree>> {
    items(children).filter(|child| !child.trivia).collect()
}

/// No precedence: level 0, no associativity.
const NO_PRECEDENCE: (i64, Associativity) = (0, Associativity::None);

/// Which of two results over the same text has the tokens a lexer prefers:
/// their leaves are walked in order, skipping trivia and the parts both
/// share, and the first leaf pair that differs decides, as a tree-sitter
/// lexer decides a conflict between two tokens at one offset: the higher
/// lexical precedence, then the longer token, then the more specific (a
/// literal over a pattern) and the earlier one (see `token_ranks` in
/// load.rs). A token where the other result skipped a separator it covers
/// (whitespace trivia) wins too, as the lexer takes a valid token over a
/// separator. Greater when `result` has the preferred token, Less when
/// `existing` has, Equal when neither (a pair without ranks that ends alike
/// but differs in kind, or every leaf alike). It mirrors preferredTokens in
/// js/src/grammar-runtime/executor.js.
pub(super) fn preferred_tokens(
    result: &Children,
    existing: &Children,
    tokens: TokenOrder<'_>,
) -> Ordering {
    let mut left = vec![Walk::Part(result.clone())];
    let mut right = vec![Walk::Part(existing.clone())];
    // The trivia each side skipped since the last leaf both share.
    let mut skipped: [Vec<Rc<Tree>>; 2] = [Vec::new(), Vec::new()];
    // Where one parse has a leaf alone and the other a node that begins with
    // it, one of them reduced that leaf (to a silent rule, or to the node)
    // where the other shifted on: an LR parser decides between them on the
    // token after the leaf, the lookahead, and a leaf conflict after the
    // lookahead is no lexer's, as the two then lex in different parse states
    // (Rust's `$(...);*`, whose `;` is a separator only after `$(` was
    // shifted). `pending` is the end of that leaf, `decided` the lookahead's
    // start.
    let mut pending = usize::MAX;
    let mut decided = usize::MAX;
    let lookahead = |tree: &Rc<Tree>, pending: usize, decided: &mut usize| {
        if *decided == usize::MAX && !tree.trivia {
            let start = first_leaf_start(tree);
            if start >= pending {
                *decided = start;
            }
        }
    };
    let covers = |leaf: &Tree, trivia: &[Rc<Tree>]| {
        trivia
            .iter()
            .any(|item| item.kind.is_none() && item.start == leaf.start && leaf.end >= item.end)
    };
    loop {
        match (left.last(), right.last()) {
            (None, _) | (_, None) => return Ordering::Equal,
            (Some(Walk::End(_)), _) => {
                left.pop();
            }
            (_, Some(Walk::End(_))) => {
                right.pop();
            }
            // A shared part is opened only while the lookahead is pending.
            (Some(Walk::Part(a)), Some(Walk::Part(b)))
                if Rc::ptr_eq(a, b) && (pending == usize::MAX || decided != usize::MAX) =>
            {
                if a.count > 0 {
                    skipped = [Vec::new(), Vec::new()];
                }
                left.pop();
                right.pop();
            }
            (Some(Walk::Part(_)), _) => open_part(&mut left),
            (_, Some(Walk::Part(_))) => open_part(&mut right),
            (Some(Walk::Item(a)), Some(Walk::Item(b))) => {
                let (a, b) = (a.clone(), b.clone());
                let (a_node, b_node) = (a.ty == TreeType::Node, b.ty == TreeType::Node);
                if Rc::ptr_eq(&a, &b) {
                    lookahead(&a, pending, &mut decided);
                    left.pop();
                    right.pop();
                    skipped = [Vec::new(), Vec::new()];
                } else if a_node || b_node {
                    let lone = if a_node { &b } else { &a };
                    if lone.ty != TreeType::Node && !lone.trivia {
                        pending = pending.min(lone.end);
                    }
                    if a_node {
                        open_node(&mut left, false);
                    }
                    if b_node {
                        open_node(&mut right, false);
                    }
                } else if a.trivia || b.trivia {
                    if a.trivia {
                        skipped[0].push(a);
                        left.pop();
                    }
                    if b.trivia {
                        skipped[1].push(b);
                        right.pop();
                    }
                } else if a.start < b.start && covers(&a, &skipped[1]) {
                    return Ordering::Greater;
                } else if b.start < a.start && covers(&b, &skipped[0]) {
                    return Ordering::Less;
                } else if a.end != b.end || a.kind != b.kind {
                    lookahead(&a, pending, &mut decided);
                    lookahead(&b, pending, &mut decided);
                    if a.start.min(b.start) > decided {
                        return Ordering::Equal;
                    }
                    return token_conflict(&a, &b, tokens);
                } else {
                    lookahead(&a, pending, &mut decided);
                    left.pop();
                    right.pop();
                    skipped = [Vec::new(), Vec::new()];
                }
            }
        }
    }
}

/// Which of two results over the same text an LR parser keeps when it decides
/// a shift-reduce conflict by precedence, as tree-sitter does when the grammar
/// is generated: they are walked in order, skipping the subtrees both share,
/// to the first node that the two build from one offset but end apart. The
/// shorter one was reduced where the longer one shifted on, and as the item in
/// progress is the node's own rule, its precedence in the two decides: the
/// higher level wins and, on equal levels, the associativity of the reduced
/// node, right to shift and left to reduce. Greater when `result` is kept,
/// Less when `existing` is, Equal when neither. It mirrors shiftOrder in
/// js/src/grammar-runtime/executor.js.
fn shift_order(result: &Children, existing: &Children) -> Ordering {
    let mut left = vec![Walk::Part(result.clone())];
    let mut right = vec![Walk::Part(existing.clone())];
    loop {
        match (left.last(), right.last()) {
            (None, _) | (_, None) => return Ordering::Equal,
            (Some(Walk::End(_)), _) => {
                left.pop();
            }
            (_, Some(Walk::End(_))) => {
                right.pop();
            }
            (Some(Walk::Part(a)), Some(Walk::Part(b))) if Rc::ptr_eq(a, b) => {
                left.pop();
                right.pop();
            }
            (Some(Walk::Part(_)), _) => open_part(&mut left),
            (_, Some(Walk::Part(_))) => open_part(&mut right),
            (Some(Walk::Item(a)), _) if a.trivia => {
                left.pop();
            }
            (_, Some(Walk::Item(b))) if b.trivia => {
                right.pop();
            }
            (Some(Walk::Item(a)), Some(Walk::Item(b))) => {
                let (a, b) = (a.clone(), b.clone());
                let (a_node, b_node) = (a.ty == TreeType::Node, b.ty == TreeType::Node);
                if Rc::ptr_eq(&a, &b)
                    || (!a_node && !b_node && a.start == b.start && a.end == b.end)
                {
                    left.pop();
                    right.pop();
                    continue;
                }
                if a_node
                    && b_node
                    && a.start == b.start
                    && let Some((x, y)) = chain_pair(&a, &b)
                    && x.end != y.end
                {
                    return if x.end > y.end {
                        shift_preferred(&x, &y)
                    } else {
                        shift_preferred(&y, &x).reverse()
                    };
                }
                let lone = lone_reduction(&a, &b).then_with(|| lone_reduction(&b, &a).reverse());
                if lone != Ordering::Equal {
                    return lone;
                }
                let reduced = extra_reduction(&a, &b, &right)
                    .then_with(|| extra_reduction(&b, &a, &left).reverse());
                if reduced != Ordering::Equal {
                    return reduced;
                }
                if !a_node && !b_node {
                    return Ordering::Equal;
                }
                if a_node {
                    open_node(&mut left, true);
                }
                if b_node {
                    open_node(&mut right, true);
                }
            }
        }
    }
}

/// Of a precedence decided on equal levels, Greater for a shift (right
/// associativity), Less for a reduction (left), Equal for none.
const fn by_associativity(associativity: Associativity) -> Ordering {
    match associativity {
        Associativity::Right => Ordering::Greater,
        Associativity::Left => Ordering::Less,
        Associativity::None => Ordering::Equal,
    }
}

/// Which of two results an LR parser keeps when one of them reduced a token
/// alone, to a silent rule of level 0 or under a precedence (Rust's
/// `(precedence -1 none (literal $))`), where the other shifted on in a node
/// `a` that begins with the token (Rust's `$x:expr` binding of level 1): the
/// innermost such node is the item in progress, and its precedence against
/// the token's decides, as in `shift_preferred`. A token that ends a silent
/// rule reduced under a precedence (Rust's `_let_chain`, `let ... && c` of
/// level 3 left before the `&&` of `c && d`) is reduced with that rule's
/// precedence. Greater when `a`'s result is kept, Less when `b`'s is, Equal
/// when neither. It mirrors loneReduction in js/src/grammar-runtime/executor.js.
fn lone_reduction(a: &Rc<Tree>, b: &Rc<Tree>) -> Ordering {
    if a.ty != TreeType::Node || b.ty != TreeType::Token {
        return Ordering::Equal;
    }
    let mut progress = a.clone();
    loop {
        let Some(first) = first_meaningful(&progress.children) else {
            return Ordering::Equal;
        };
        if first.ty != TreeType::Node {
            if !same_tree(&first, b) {
                return Ordering::Equal;
            }
            break;
        }
        progress = first;
    }
    let (shifted, _) = progress.precedence.unwrap_or(NO_PRECEDENCE);
    let (reduced, associativity) = b.reduced.or(b.precedence).unwrap_or(NO_PRECEDENCE);
    if shifted != reduced {
        return shifted.cmp(&reduced);
    }
    by_associativity(associativity)
}

/// Which of two results an LR parser keeps when one of them reduces a node
/// the other does not build: `a` is that node when `b`, at the same offset in
/// the other result, is a subtree on its leftmost chain and the children of
/// the innermost such node are, subtree for subtree, the next children of the
/// other result's node in progress (`walk` holds its place). The two
/// reductions of the same text then conflict at the end of that node, which
/// one result reduces where the other reduces or shifts in its own node: the
/// higher precedence level wins and, on equal levels where the other node
/// goes on, the associativity of the reduced node. Greater when `a`'s result
/// is kept, Less when the other is, Equal when neither. It mirrors
/// extraReduction in js/src/grammar-runtime/executor.js.
fn extra_reduction(a: &Rc<Tree>, b: &Rc<Tree>, walk: &[Walk]) -> Ordering {
    if a.ty != TreeType::Node {
        return Ordering::Equal;
    }
    let mut parent = a.clone();
    loop {
        let Some(first) = first_meaningful(&parent.children) else {
            return Ordering::Equal;
        };
        if same_tree(&first, b) {
            break;
        }
        if first.ty != TreeType::Node {
            return Ordering::Equal;
        }
        parent = first;
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
    if own.len() > next.len()
        || own
            .iter()
            .zip(&next)
            .any(|(mine, theirs)| !same_tree(mine, theirs))
    {
        return Ordering::Equal;
    }
    let (mine, associativity) = reduction(&parent);
    let (other, _) = container
        .as_ref()
        .and_then(|node| node.precedence)
        .unwrap_or(NO_PRECEDENCE);
    if mine != other {
        return mine.cmp(&other);
    }
    if container.is_some_and(|node| node.end > parent.end) {
        return by_associativity(associativity).reverse();
    }
    Ordering::Equal
}

/// Whether two subtrees are the same tree over the same text, whichever of
/// them holds the white space around it.
fn same_tree(a: &Rc<Tree>, b: &Rc<Tree>) -> bool {
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

/// The first child of a list that is not trivia, without flattening it.
fn first_meaningful(children: &Children) -> Option<Rc<Tree>> {
    items(children).find(|child| !child.trivia)
}

/// The offset of the first leaf under a node that is not white space.
fn first_leaf_start(node: &Rc<Tree>) -> usize {
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
fn leftmost_chain(node: &Rc<Tree>) -> Vec<Rc<Tree>> {
    let mut chain = Vec::new();
    let mut current = Some(node.clone());
    while let Some(next) = current.filter(|next| next.ty == TreeType::Node) {
        current = first_meaningful(&next.children);
        chain.push(next);
    }
    chain
}

/// The outermost node kind on the leftmost chains of two nodes that start at
/// one offset, as the pair of its nodes.
fn chain_pair(a: &Rc<Tree>, b: &Rc<Tree>) -> Option<(Rc<Tree>, Rc<Tree>)> {
    let other = leftmost_chain(b);
    leftmost_chain(a).into_iter().find_map(|node| {
        other
            .iter()
            .find(|candidate| candidate.kind == node.kind && candidate.start == node.start)
            .map(|found| (node.clone(), found.clone()))
    })
}

/// Greater when the shift that built `long` is preferred to the reduction
/// that ended `short` (the same node kind from the same offset), Less when
/// the reduction is, Equal when the precedences cannot tell. The shift is in
/// the innermost node of `long` that goes on past the end of `short`. When
/// that node began with `short`, as a binary expression whose left operand
/// is the expression a statement is of, the long result reduced that operand
/// to a silent rule where the short one reduced its node: the two reductions
/// conflict instead, and a silent rule's reduction is of level 0.
fn shift_preferred(long: &Rc<Tree>, short: &Rc<Tree>) -> Ordering {
    let begin = first_leaf_start(short);
    let mut progress = long.clone();
    while let Some(inner) = items(&progress.children).find(|child| {
        child.ty == TreeType::Node && child.start < short.end && child.end > short.end
    }) {
        if first_leaf_start(&inner) == begin {
            let (reduced, _) = reduction(short);
            return 0.cmp(&reduced);
        }
        progress = inner;
    }
    let (shifted, _) = progress.precedence.unwrap_or(NO_PRECEDENCE);
    let (reduced, associativity) = reduced_before(short, &progress);
    if shifted != reduced {
        return shifted.cmp(&reduced);
    }
    by_associativity(associativity)
}

/// The precedence a node is reduced with: the innermost one over its last
/// part, as a generated parser takes the precedence of a production's last
/// step (Rust's `let` condition, whose value is `(precedence 3 left (ref
/// expression))`, reduces before the `&&` of a binary expression of level
/// 3), else none of level 0.
fn reduction(node: &Tree) -> (i64, Associativity) {
    node.tail.or(node.precedence).unwrap_or(NO_PRECEDENCE)
}

/// The precedence `short` was reduced with where the shift in `progress`
/// went on instead: that of the innermost node along its rightmost chain
/// whose last part is the first part of `progress`, as the production a
/// generated parser completes at the conflict (Rust's `let bar = || baz &&
/// quux`, where the closure of level -1 ends with `baz`, not the `let`
/// condition), or the precedence a token ending a silent rule keeps (see
/// `lone_reduction`); else the precedence `short` reduces with.
fn reduced_before(short: &Rc<Tree>, progress: &Tree) -> (i64, Associativity) {
    if let Some(first) = first_meaningful(&progress.children) {
        let mut node = short.clone();
        while node.ty == TreeType::Node {
            let Some(last) = node
                .children
                .iter()
                .rev()
                .find(|child| !child.trivia)
                .cloned()
            else {
                break;
            };
            if same_tree(&last, &first) {
                return match last.reduced {
                    Some(reduced) if last.ty == TreeType::Token => reduced,
                    _ => reduction(&node),
                };
            }
            node = last;
        }
    }
    reduction(short)
}

/// Of two complete results, each with its trailing trivia, Greater when `a`
/// is preferred, Less when `b` is, Equal on a tie: they end apart before the
/// trailing trivia, so they are ranked as `ResultSet::add` ranks results with
/// one end.
pub(super) fn complete_order(
    a: (&Res, &Children),
    b: (&Res, &Children),
    tokens: Option<TokenOrder<'_>>,
) -> Ordering {
    if a.0.cost != b.0.cost {
        return b.0.cost.cmp(&a.0.cost);
    }
    let (left, right) = (
        join_children(&a.0.children, a.1),
        join_children(&b.0.children, b.1),
    );
    tokens
        .map_or(Ordering::Equal, |tokens| {
            preferred_tokens(&left, &right, tokens).then_with(|| shift_order(&left, &right))
        })
        .then(a.0.dynamic.cmp(&b.0.dynamic))
}

/// The rank of a token leaf, or None for another leaf or an unranked token.
/// A leaf matched under a lexical precedence ranks at that level, as the
/// token defined there (Rust's `//!` marker `!` outranks the comment text).
fn token_rank(leaf: &Tree, tokens: TokenOrder<'_>) -> Option<TokenRank> {
    if leaf.ty != TreeType::Token {
        return None;
    }
    let rank = leaf.kind.as_ref().map_or_else(
        || {
            tokens
                .bytes
                .get(leaf.start..leaf.end)
                .and_then(|text| tokens.ranks.literals.get(text))
                .copied()
        },
        |kind| tokens.ranks.kinds.get(kind).copied(),
    );
    let Some(priority) = leaf.priority else {
        return rank;
    };
    Some(TokenRank {
        priority,
        ..rank.unwrap_or(TokenRank {
            priority,
            specificity: 0,
            order: usize::MAX,
        })
    })
}

/// Two leaves that differ in end or kind: Greater when a lexer prefers `a`,
/// Less when it prefers `b`, Equal when it cannot tell. The ranks decide only
/// between two tokens at one offset; otherwise the longer leaf wins.
fn token_conflict(a: &Tree, b: &Tree, tokens: TokenOrder<'_>) -> Ordering {
    let ranks = if a.start == b.start {
        token_rank(a, tokens).zip(token_rank(b, tokens))
    } else {
        None
    };
    if let Some((first, second)) = ranks
        && first.priority != second.priority
    {
        return first.priority.cmp(&second.priority);
    }
    if a.end != b.end {
        return a.end.cmp(&b.end);
    }
    match ranks {
        None => Ordering::Equal,
        Some((first, second)) if first == second => Ordering::Equal,
        Some((first, second)) if first.specificity != second.specificity => {
            first.specificity.cmp(&second.specificity)
        }
        Some((first, second)) => second.order.cmp(&first.order),
    }
}

/// Keyword lexing under `(matching longest)`. A tree-sitter lexer lexes a
/// keyword wherever the parse state admits it, before any parse goes on, so a
/// token rule's leaf over the same text (an identifier `typedef`) is not taken
/// there even when only it would let the parse go on. A parse records the
/// spans where a keyword token matched (`matched`); a leaf of a token rule in
/// its tree over such a span (`lexed`, the rule's kind, which an alias keeps)
/// that the keyword outranks (see `token_conflict`) makes the span
/// keyword-only (`only`), and the input is parsed again, where no token rule
/// takes a keyword-only span the keyword outranks it on. The spans only grow,
/// so the reparses end.
#[derive(Debug, Default)]
pub(super) struct KeywordLexing {
    only: HashSet<(usize, usize)>,
    pub(super) matched: HashSet<(usize, usize)>,
}

impl KeywordLexing {
    /// Whether a keyword-only span's keyword outranks a token rule's leaf over it.
    pub(super) fn outranks(&self, leaf: &Tree, tokens: TokenOrder<'_>) -> bool {
        self.only.contains(&(leaf.start, leaf.end)) && outranks_at(leaf, tokens)
    }

    /// Marks the spans where `root` took a token rule's leaf over a keyword;
    /// true when one is new.
    pub(super) fn conflicts(&mut self, root: &Tree, tokens: TokenOrder<'_>) -> bool {
        let mut found = false;
        let mut pending = vec![root];
        while let Some(node) = pending.pop() {
            if node.ty == TreeType::Node {
                pending.extend(node.children.iter().rev().map(|child| &**child));
                continue;
            }
            let Some(kind) = &node.lexed else {
                continue;
            };
            let span = (node.start, node.end);
            if !self.matched.contains(&span) || self.only.contains(&span) {
                continue;
            }
            let leaf = Tree::new(TreeType::Token, Some(kind.clone()), node.start, node.end);
            if outranks_at(&leaf, tokens) {
                self.only.insert(span);
                found = true;
            }
        }
        self.matched.clear();
        found
    }
}

/// Whether a keyword over the span of a token rule's leaf outranks it.
fn outranks_at(leaf: &Tree, tokens: TokenOrder<'_>) -> bool {
    let keyword = Tree::new(TreeType::Token, None, leaf.start, leaf.end);
    token_conflict(&keyword, leaf, tokens) == Ordering::Greater
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
/// repairing, how it is made.
pub(super) type MemoKey = (usize, usize, State, bool, Repair);

/// How a rule call is made while repairing: in the open, quietly (where
/// nothing is repaired), or after a MISSING leaf at its offset.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(super) enum Repair {
    Open,
    Quiet,
    Chained,
}
