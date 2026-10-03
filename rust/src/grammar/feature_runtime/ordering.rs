//! The order of the results of one end and state, as the result set of
//! `js/src/grammar-runtime/executor.js` settles them: the token conflicts a
//! tree-sitter lexer settles, the shifts and reductions an LR parser keeps by
//! precedence, and the reductions of extras and lone nodes.

use std::cmp::Ordering;
use std::collections::HashSet;
use std::rc::Rc;

use super::program::{Associativity, PrecedenceTag, TokenRank, compare_precedence};
use super::results::{Children, Res, TokenOrder, Tree, TreeType, join_children};
use crate::grammar::PrecedenceEntry;

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
                } else if a_node && b_node && (nests_first(&a, &b) || nests_first(&b, &a)) {
                    // A node one parse wraps deeper (Rust's `.. ..` as the
                    // left operand of `..=` or of an assignment) is the first
                    // part of the other parse's node: that one is entered
                    // alone until the two line up, so the node meets its
                    // match, not a leaf of it.
                    if nests_first(&a, &b) {
                        open_node(&mut left, false);
                    } else {
                        open_node(&mut right, false);
                    }
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
pub(super) fn shift_order(
    result: &Children,
    existing: &Children,
    orders: &[Vec<PrecedenceEntry>],
) -> Ordering {
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
                        shift_preferred(&x, &y, orders)
                    } else {
                        shift_preferred(&y, &x, orders).reverse()
                    };
                }
                // The same node reduced on in two ways (Rust's `m!(x);` in a
                // block, a macro invocation that `_expression_except_range`
                // reduces under `(precedence 1 none (ref macro_invocation))`
                // and `_declaration_statement` of level 0) conflicts at its
                // end: the higher precedence it was reduced with wins.
                if a_node
                    && b_node
                    && a.start == b.start
                    && let Some((x, y)) = chain_pair(&a, &b)
                {
                    let reduced = |node: &Tree| {
                        node.reduced
                            .clone()
                            .unwrap_or(PrecedenceTag::unranked(None))
                    };
                    let order = compare_precedence(&reduced(&x), &reduced(&y), orders);
                    if order != Ordering::Equal {
                        return order;
                    }
                }
                let lone = lone_reduction(&a, &b, orders)
                    .then_with(|| lone_reduction(&b, &a, orders).reverse());
                if lone != Ordering::Equal {
                    return lone;
                }
                let reduced = extra_reduction(&a, &b, &right, orders)
                    .then_with(|| extra_reduction(&b, &a, &left, orders).reverse());
                if reduced != Ordering::Equal {
                    return reduced;
                }
                if a_node && b_node && a.start == b.start {
                    let parted = chain_conflict(&a, &b, orders);
                    if parted != Ordering::Equal {
                        return parted;
                    }
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
fn lone_reduction(a: &Rc<Tree>, b: &Rc<Tree>, orders: &[Vec<PrecedenceEntry>]) -> Ordering {
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
    let shifted = progress
        .precedence
        .clone()
        .unwrap_or_else(|| PrecedenceTag::unranked(progress.rule.clone()));
    let reduced = b
        .reduced
        .clone()
        .or_else(|| b.precedence.clone())
        .unwrap_or(PrecedenceTag::unranked(None));
    compare_precedence(&shifted, &reduced, orders)
        .then_with(|| by_associativity(reduced.associativity))
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
fn extra_reduction(
    a: &Rc<Tree>,
    b: &Rc<Tree>,
    walk: &[Walk],
    orders: &[Vec<PrecedenceEntry>],
) -> Ordering {
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
    let mine = reduction(&parent);
    let other = container.as_ref().map_or_else(
        || PrecedenceTag::unranked(None),
        |node| {
            node.precedence
                .clone()
                .unwrap_or_else(|| PrecedenceTag::unranked(node.rule.clone()))
        },
    );
    let order = compare_precedence(&mine, &other, orders);
    if order != Ordering::Equal {
        return order;
    }
    if container.is_none_or(|node| node.end > parent.end) {
        return by_associativity(mine.associativity).reverse();
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

/// Whether a node of the kind and span of `inner` is on the leftmost chain of
/// the longer node `outer`. It mirrors nestsFirst in
/// js/src/grammar-runtime/executor.js.
fn nests_first(outer: &Rc<Tree>, inner: &Rc<Tree>) -> bool {
    outer.end > inner.end
        && leftmost_chain(outer).iter().any(|node| {
            node.kind == inner.kind && node.start == inner.start && node.end == inner.end
        })
}

/// Which of two results an LR parser keeps when two nodes from one offset end
/// their leftmost chains apart and nothing else decides them (Rust's closure
/// `|a| b` and or-pattern `|a|b` in a tuple pattern): the two parses part at
/// the first end only one chain has, where one reduced the innermost node
/// ending there (the or-pattern `|a` of level -2) and the other shifted on in
/// the innermost node going past it (the closure parameters `|a|`), as in
/// `shift_preferred`, when the reduced node's children begin the other
/// node's, so the two parses agree up to that end. Greater when `a`'s result
/// is kept, Less when `b`'s is, Equal when neither. It mirrors chainConflict
/// in js/src/grammar-runtime/executor.js.
fn chain_conflict(a: &Rc<Tree>, b: &Rc<Tree>, orders: &[Vec<PrecedenceEntry>]) -> Ordering {
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
    let (Some(short), Some(long)) = (
        reducing.iter().rev().find(|node| node.end == end),
        shifting.iter().rev().find(|node| node.end > end),
    ) else {
        return Ordering::Equal;
    };
    let (own, next) = (meaningful(&short.children), meaningful(&long.children));
    if own.len() >= next.len() || own.iter().zip(&next).any(|(x, y)| !same_tree(x, y)) {
        return Ordering::Equal;
    }
    let order = shift_preferred(long, short, orders);
    if kept == Ordering::Greater {
        order
    } else {
        order.reverse()
    }
}

/// Greater when the shift that built `long` is preferred to the reduction
/// that ended `short` (the same node kind from the same offset), Less when
/// the reduction is, Equal when the precedences cannot tell. The shift is in
/// the innermost node of `long` that goes on past the end of `short`. When
/// that node began with `short`, as a binary expression whose left operand
/// is the expression a statement is of, the long result reduced that operand
/// to a silent rule where the short one reduced its node: the two reductions
/// conflict instead, and a silent rule's reduction is of level 0.
fn shift_preferred(long: &Rc<Tree>, short: &Rc<Tree>, orders: &[Vec<PrecedenceEntry>]) -> Ordering {
    let begin = first_leaf_start(short);
    let mut progress = long.clone();
    while let Some(inner) = items(&progress.children).find(|child| {
        child.ty == TreeType::Node && child.start < short.end && child.end > short.end
    }) {
        if first_leaf_start(&inner) == begin {
            return compare_precedence(&reduction(short), &PrecedenceTag::unranked(None), orders)
                .reverse();
        }
        progress = inner;
    }
    let shifted = progress
        .precedence
        .clone()
        .unwrap_or_else(|| PrecedenceTag::unranked(progress.rule.clone()));
    let reduced = reduced_before(short, &progress);
    compare_precedence(&shifted, &reduced, orders)
        .then_with(|| by_associativity(reduced.associativity))
}

/// The precedence a node is reduced with: the innermost one over its last
/// part, as a generated parser takes the precedence of a production's last
/// step (Rust's `let` condition, whose value is `(precedence 3 left (ref
/// expression))`, reduces before the `&&` of a binary expression of level
/// 3), else none of level 0, ranked by the node's rule.
fn reduction(node: &Tree) -> PrecedenceTag {
    node.tail
        .clone()
        .or_else(|| node.precedence.clone())
        .unwrap_or_else(|| PrecedenceTag::unranked(node.rule.clone()))
}

/// The precedence `short` was reduced with where the shift in `progress`
/// went on instead: that of the innermost node along its rightmost chain
/// whose last part is the first part of `progress`, as the production a
/// generated parser completes at the conflict (Rust's `let bar = || baz &&
/// quux`, where the closure of level -1 ends with `baz`, not the `let`
/// condition), or the precedence a token ending a silent rule keeps (see
/// `lone_reduction`); else the precedence `short` reduces with.
fn reduced_before(short: &Rc<Tree>, progress: &Tree) -> PrecedenceTag {
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
                return match &last.reduced {
                    Some(reduced) if last.ty == TreeType::Token => reduced.clone(),
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
            preferred_tokens(&left, &right, tokens)
                .then_with(|| shift_order(&left, &right, tokens.orders))
        })
        .then(a.0.dynamic.cmp(&b.0.dynamic))
}

/// The rank of a token leaf, or None for another leaf or an unranked token.
/// A leaf matched under a lexical precedence ranks at that level, as the
/// token defined there (Rust's `//!` marker `!` outranks the comment text).
pub(super) fn token_rank(leaf: &Tree, tokens: TokenOrder<'_>) -> Option<TokenRank> {
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
pub(super) fn token_conflict(a: &Tree, b: &Tree, tokens: TokenOrder<'_>) -> Ordering {
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
