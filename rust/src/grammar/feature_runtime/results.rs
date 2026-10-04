//! The trees and results of the native executor: the syntax tree it builds,
//! one way an expression matches, the deduplicating result set, and the
//! memo entries and shared resource counters of one parse.

use std::cell::{Cell, OnceCell, RefCell};
use std::cmp::Ordering;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::fmt;
use std::ops::Deref;
use std::rc::Rc;

use super::forking::{GrammarFacts, Lead};
use super::operations::{OperationValue, State};
pub(super) use super::ordering::{complete_order, preferred_tokens, same_children};
use super::ordering::{same_output, settled_order, token_conflict};
use super::program::{Name, PrecedenceTag, Settling, TokenRanks};
use crate::grammar::PrecedenceEntry;

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
    pub(super) count: usize,
    pub(super) flat: OnceCell<Vec<Rc<Tree>>>,
    pub(super) link: RefCell<Option<(Children, Children)>>,
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
pub(super) fn join_children(left: &Children, right: &Children) -> Children {
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
#[allow(clippy::struct_excessive_bools)] // the flags of one node, as on the JavaScript tree
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
    pub(super) precedence: Option<PrecedenceTag>,
    /// The innermost precedence over the node's last part, which its rule
    /// reduces with (see `reduction`).
    pub(super) tail: Option<PrecedenceTag>,
    /// A token leaf's lexical precedence where it was matched under one
    /// (see `token_rank`).
    pub(super) priority: Option<i64>,
    /// The precedence a token leaf ending a silent rule was reduced with
    /// (see `lone_reduction`), or a node reduced alone under one (see
    /// `shift_order`).
    pub(super) reduced: Option<PrecedenceTag>,
    /// The precedence a node ending a silent rule was reduced with (see
    /// `lone_reduction`).
    pub(super) closes: Option<PrecedenceTag>,
    /// The silent rules the precedence orders name that reduced the item
    /// alone, innermost first (see `child_parting`).
    pub(super) reduced_to: Vec<Name>,
    /// The silent rules a declared conflict names that reduced the item
    /// alone, innermost first (see `forked_order`).
    pub(super) forked_to: Vec<Name>,
    /// A token leaf a silent rule reduced alone (see `preferred_tokens`).
    pub(super) alone: bool,
    /// The token a node's rule was reduced before, where its optional parts
    /// could begin with it (see `reduced_early`).
    pub(super) before: Option<Lead>,
    pub(super) ambiguous: bool,
    pub(super) literal: bool,
    /// A token of an external scanner, whatever an alias names it (see
    /// `preferred_tokens`).
    pub(super) scanned: bool,
    /// Under keyword lexing, the kind of the token rule that built the leaf,
    /// which an alias keeps.
    pub(super) lexed: Option<Name>,
    /// Under keyword lexing, a plain literal leaf whose text the grammar also
    /// takes as an immediate token (see `KeywordLexing`).
    pub(super) plain: bool,
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
            closes: None,
            reduced_to: Vec::new(),
            forked_to: Vec::new(),
            alone: false,
            before: None,
            ambiguous: false,
            literal: false,
            scanned: false,
            lexed: None,
            plain: false,
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

/// Whether `tree` is a trivia leaf of no kind: white space, which a lexer
/// skips as padding.
pub(super) fn is_separator(tree: &Tree) -> bool {
    tree.ty == TreeType::Token && tree.trivia && tree.kind.is_none()
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
    pub(super) precedence: Option<PrecedenceTag>,
    /// The innermost precedence over the last part of the result (see
    /// `reduction`).
    pub(super) tail: Option<PrecedenceTag>,
    pub(super) ambiguous: bool,
    /// The repair cost: 2 per MISSING leaf, the skipped bytes per ERROR leaf.
    pub(super) cost: usize,
    /// The token the result's rule was reduced before, where its optional
    /// parts could begin with it (see `reduction_facts`), which the node
    /// built of it keeps.
    pub(super) before: Option<Lead>,
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
            before: None,
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
                left.tail.clone()
            } else {
                right.tail
            },
            ambiguous: left.ambiguous || right.ambiguous,
            cost: left.cost + right.cost,
            before: None,
        }
    }

    pub(super) fn key(&self) -> (usize, State) {
        (self.end, self.state.clone())
    }
}

/// The token ranks of a `(matching longest)` grammar and the input bytes, by
/// which a tie between results goes to the tokens a lexer prefers.
#[derive(Clone, Copy, Debug)]
pub(super) struct TokenOrder<'c> {
    pub(super) ranks: &'c TokenRanks,
    pub(super) bytes: &'c [u8],
    /// The precedence orders by which `shift_order` ranks named precedences.
    pub(super) orders: &'c [Vec<PrecedenceEntry>],
    /// What the conflicts of two results ask of the rules (see
    /// `GrammarFacts`).
    pub(super) grammar: &'c GrammarFacts,
}

/// A deduplicating, insertion-ordered set of results keyed by end and state.
#[derive(Clone, Debug, Default)]
pub(super) struct ResultSet<'c> {
    pub(super) items: Vec<Res>,
    index: HashMap<(usize, State), usize>,
    /// The token ranks the `tokens` and `precedence` settling steps read.
    tokens: Option<TokenOrder<'c>>,
    /// The grammar's settling steps.
    settling: Settling,
    /// The precedence the results are parts of, when a precedence
    /// expression holds them (see `shift_order`).
    owner: Option<PrecedenceTag>,
}

impl<'c> ResultSet<'c> {
    pub(super) fn new(tokens: Option<TokenOrder<'c>>, settling: Settling) -> Self {
        Self {
            tokens,
            settling,
            ..Self::default()
        }
    }

    /// The set with `owner` as the precedence its results are parts of.
    pub(super) fn owned(mut self, owner: Option<&PrecedenceTag>) -> Self {
        self.owner = owner.cloned();
        self
    }

    /// Of two results with the same end and state, the lower repair cost
    /// wins, then the one the grammar's settling steps prefer (see
    /// `settled_order`); on a tie the first stays and, when the settling ends
    /// in `ambiguity`, without repairs and unless both build the same trees,
    /// is marked ambiguous. True when `result` was kept.
    pub(super) fn add(&mut self, result: Res) -> bool {
        match self.index.get(&result.key()) {
            None => self.push(result),
            Some(&position) => {
                let existing = &mut self.items[position];
                if result.cost > existing.cost {
                    return false;
                }
                if result.cost == existing.cost {
                    let order = settled_order(
                        (&result.children, result.dynamic),
                        (&existing.children, existing.dynamic),
                        self.settling,
                        self.tokens,
                        self.owner.as_ref(),
                    );
                    if order != Ordering::Greater {
                        if order == Ordering::Equal
                            && self.settling.ambiguity()
                            && existing.cost == 0
                            && !same_output(&result.children, &existing.children)
                        {
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

    /// The result of the end and state `key`, if any.
    pub(super) fn get_key(&self, key: &(usize, State)) -> Option<&Res> {
        self.index.get(key).map(|&position| &self.items[position])
    }

    /// Removes `result` itself, if it is the result of its end and state
    /// (not one that replaced it), keeping the order of the rest.
    pub(super) fn remove(&mut self, result: &Res) {
        let key = result.key();
        if let Some(&position) = self.index.get(&key)
            && Rc::ptr_eq(&self.items[position].children, &result.children)
        {
            self.index.remove(&key);
            self.items.remove(position);
            for slot in self.index.values_mut() {
                if *slot > position {
                    *slot -= 1;
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

    pub(super) const fn len(&self) -> usize {
        self.items.len()
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
///
/// The keyword counts only where it matched in the tree's parse state: in a
/// call made, through some chain of calls up to the first, by calls that
/// build nodes each beginning a node of the tree that goes on past the span,
/// as the items a parser has in progress there. A chain through a call that
/// began no such node lexed the text before the span otherwise (Rust's
/// `m!('"')`, whose token tree also takes `'` alone and a string to the next
/// `"`, where a later `_` type is a keyword `_` token): the tree's parse never
/// reached that state. A call's result is shared by every call that made it,
/// so every chain counts. The calls are kept in `calls`, their makers by
/// index.
///
/// An immediate token a literal (`(immediateToken (literal [))`) outranks
/// the plain literal of the same text, which a lexer so lexes only where no
/// immediate one is valid (Lean's `foo[1:2:3]`, whose `[` opens a subscript,
/// not a range applied to `foo`): the spans where an immediate literal
/// matched (`matched_immediate`) become immediate-only (`immediates`) alike
/// where the tree took the plain literal (a `plain` leaf), whose parse state
/// the immediate one matched in. It mirrors `KeywordLexing` in
/// js/src/grammar-runtime/executor.js.
#[derive(Debug, Default)]
pub(super) struct KeywordLexing {
    only: HashSet<(usize, usize)>,
    matched: HashMap<(usize, usize), HashSet<Option<usize>>>,
    immediates: HashSet<(usize, usize)>,
    matched_immediate: HashMap<(usize, usize), HashSet<Option<usize>>>,
    calls: Vec<Call>,
}

/// One rule call of a parse under keyword lexing: where it began, whether it
/// builds a node and the calls it was made from (`None` for none).
#[derive(Debug)]
struct Call {
    position: usize,
    builds: bool,
    parents: HashSet<Option<usize>>,
}

impl KeywordLexing {
    /// Records a rule call beginning at `position` made from `parent`; its index.
    pub(super) fn call(&mut self, position: usize, builds: bool, parent: Option<usize>) -> usize {
        self.calls.push(Call {
            position,
            builds,
            parents: HashSet::from([parent]),
        });
        self.calls.len() - 1
    }

    /// Records that the call `call` was made again, from `parent`.
    pub(super) fn called(&mut self, call: usize, parent: Option<usize>) {
        self.calls[call].parents.insert(parent);
    }

    /// Records a keyword or `immediate` literal token matched over `span` in
    /// the rule call `call`.
    pub(super) fn matched(&mut self, span: (usize, usize), call: Option<usize>, immediate: bool) {
        let matched = if immediate {
            &mut self.matched_immediate
        } else {
            &mut self.matched
        };
        matched.entry(span).or_default().insert(call);
    }

    /// Whether an immediate token outranks the plain literal over `span`.
    pub(super) fn immediate_only(&self, span: (usize, usize)) -> bool {
        self.immediates.contains(&span)
    }

    /// Whether a keyword-only span's keyword outranks a token rule's leaf over it.
    pub(super) fn outranks(&self, leaf: &Tree, tokens: TokenOrder<'_>) -> bool {
        self.only.contains(&(leaf.start, leaf.end)) && outranks_at(leaf, tokens)
    }

    /// Marks the spans where `root` took a token rule's leaf over a keyword;
    /// true when one is new.
    pub(super) fn conflicts(&mut self, root: &Tree, tokens: TokenOrder<'_>) -> bool {
        let mut found = false;
        let mut reach = None;
        let mut pending = vec![root];
        while let Some(node) = pending.pop() {
            if node.ty == TreeType::Node {
                pending.extend(node.children.iter().rev().map(|child| &**child));
                continue;
            }
            let span = (node.start, node.end);
            let (matched, only) = if node.plain {
                (&self.matched_immediate, &self.immediates)
            } else if node.lexed.is_some() {
                (&self.matched, &self.only)
            } else {
                continue;
            };
            let Some(calls) = matched.get(&span) else {
                continue;
            };
            if only.contains(&span) {
                continue;
            }
            if let Some(kind) = node.lexed.as_ref().filter(|_| !node.plain) {
                let leaf = Tree::new(TreeType::Token, Some(kind.clone()), node.start, node.end);
                if !outranks_at(&leaf, tokens) {
                    continue;
                }
            }
            let reach = reach.get_or_insert_with(|| TreeReach::of(root));
            let mut seen = HashSet::new();
            if !calls
                .iter()
                .any(|call| self.in_parse_state(*call, node, reach, &mut seen))
            {
                continue;
            }
            if node.plain {
                self.immediates.insert(span);
            } else {
                self.only.insert(span);
            }
            found = true;
        }
        self.matched.clear();
        self.matched_immediate.clear();
        self.calls.clear();
        found
    }

    /// Whether a keyword matched in `call` was in the parse state of the
    /// tree's `leaf` over its span: whether some chain of the calls that made
    /// it, up to the first, holds no call that builds a node the tree does
    /// not have in progress there. `seen` holds the calls already searched.
    fn in_parse_state(
        &self,
        call: Option<usize>,
        leaf: &Tree,
        reach: &TreeReach,
        seen: &mut HashSet<usize>,
    ) -> bool {
        let mut pending = vec![call];
        while let Some(current) = pending.pop() {
            let Some(current) = current else {
                return true;
            };
            if !seen.insert(current) {
                continue;
            }
            let call = &self.calls[current];
            if call.builds && call.position < leaf.start {
                let first = reach.starts
                    [reach.starts.partition_point(|start| *start < call.position)..]
                    .first()
                    .copied();
                if let Some(first) = first
                    && first < leaf.start
                    && reach.ends.get(&first).is_none_or(|end| *end < leaf.end)
                {
                    continue;
                }
            }
            pending.extend(call.parents.iter().copied());
        }
        false
    }
}

/// The starts of the leaves of a tree that are not trivia, in order, and for
/// each the farthest end of a node that begins with that leaf. A token the
/// external scanner scanned of no width does not count: the parser scanned it
/// before its lexer, in the parse state the next leaf is lexed in
/// (JavaScript's automatic semicolon before a line break and a keyword
/// `class`). It mirrors treeReach in js/src/grammar-runtime/executor.js.
struct TreeReach {
    starts: Vec<usize>,
    ends: HashMap<usize, usize>,
}

impl TreeReach {
    fn of(root: &Tree) -> Self {
        let mut starts = Vec::new();
        let mut ends = HashMap::new();
        let mut open: Option<usize> = None;
        let mut pending = vec![root];
        while let Some(node) = pending.pop() {
            if node.trivia || (node.ty != TreeType::Node && node.start == node.end && node.scanned)
            {
                continue;
            }
            if node.ty == TreeType::Node {
                open = Some(open.map_or(node.end, |end| end.max(node.end)));
                pending.extend(node.children.iter().rev().map(|child| &**child));
                continue;
            }
            starts.push(node.start);
            if let Some(end) = open.take() {
                let farthest = ends.entry(node.start).or_insert(end);
                *farthest = (*farthest).max(end);
            }
        }
        Self { starts, ends }
    }
}

/// Whether a keyword over the span of a token rule's leaf outranks it.
fn outranks_at(leaf: &Tree, tokens: TokenOrder<'_>) -> bool {
    let keyword = Tree::new(TreeType::Token, None, leaf.start, leaf.end);
    token_conflict(&keyword, leaf, tokens) == Ordering::Greater
}

/// `result` with its one meaningful item recording that the silent rule
/// `name` reduced it alone: a token as `alone`, and any item, when the
/// precedence orders name the rule (`ranked`), the rule after the inner ones
/// it was reduced to (`reduced_to`), and when a declared conflict names it
/// (`forked`), the rule as one it was reduced to there (`forked_to`, see
/// `forked_order`); any other result as it is. It mirrors reducedAlone in
/// js/src/grammar-runtime/executor.js.
pub(super) fn reduced_alone(result: Res, name: &Name, ranked: bool, forked: bool) -> Res {
    let mut children = result.children.to_vec();
    let mut meaningful = children
        .iter()
        .enumerate()
        .filter(|(_, child)| !child.trivia);
    let (Some((at, only)), None) = (meaningful.next(), meaningful.next()) else {
        return result;
    };
    let alone = only.ty == TreeType::Token && !only.alone;
    let reduced = ranked && !only.reduced_to.contains(name);
    let fork = forked && !only.forked_to.contains(name);
    if !alone && !reduced && !fork {
        return result;
    }
    let mut tagged = (**only).clone();
    tagged.alone |= alone;
    if reduced {
        tagged.reduced_to.push(name.clone());
    }
    if fork {
        tagged.forked_to.push(name.clone());
    }
    children[at] = Rc::new(tagged);
    Res {
        children: children_of(children),
        ..result
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
/// embedded-language executor, and the record of what the parse expects.
#[derive(Debug)]
pub(super) struct Shared {
    pub(super) steps: Cell<usize>,
    pub(super) limit: usize,
    pub(super) frames: Cell<usize>,
    pub(super) memo_limit: usize,
    pub(super) expectations: RefCell<Expectations>,
}

/// An item a scanner's `expected` asks about: the index of its program and
/// its id there.
pub(super) type Expectation = (usize, usize);

/// What a scanner's `(expected ITEM)` answers: whether the parse requested the
/// item, a literal or a rule, at the scanner's context offset, as a tree-sitter
/// scanner asks whether a token is valid in the parse state. The context of a
/// token is the offset before the trivia its terminal skips, and of an extra
/// the offset its trivia starts at. A request is recorded where the parse
/// makes it, so one the scanner asks about before the parse made it was
/// answered wrong: the parse is then `stale` and runs again, seeded with the
/// requests so far. The requests only grow, so the runs end. One parse, its
/// rounds and its embedded languages share one record. It mirrors
/// `Expectations` in js/src/grammar-runtime/executor.js.
#[derive(Debug, Default)]
pub(super) struct Expectations {
    requested: HashMap<usize, HashSet<Expectation>>,
    consulted: HashMap<usize, HashSet<Expectation>>,
    pub(super) stale: bool,
}

impl Expectations {
    /// Starts a run of the parse, keeping the requests.
    pub(super) fn restart(&mut self) {
        self.consulted.clear();
        self.stale = false;
    }

    pub(super) fn request(&mut self, position: usize, item: Expectation) {
        if !self.requested.entry(position).or_default().insert(item) {
            return;
        }
        if self
            .consulted
            .get(&position)
            .is_some_and(|items| items.contains(&item))
        {
            self.stale = true;
        }
    }

    pub(super) fn holds(&mut self, position: usize, item: Expectation) -> bool {
        if self
            .requested
            .get(&position)
            .is_some_and(|items| items.contains(&item))
        {
            return true;
        }
        self.consulted.entry(position).or_default().insert(item);
        false
    }
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
        /// the rest of the input an ERROR leaf, or of the cheapest complete
        /// result when the round asks for one more repair point.
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
    /// Under keyword lexing, the call's index in `KeywordLexing`.
    pub(super) call: Option<usize>,
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
