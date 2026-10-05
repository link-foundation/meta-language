//! The order of the results of one end and state, as the result set of
//! `js/src/grammar-runtime/executor.js` settles them: the token conflicts a
//! tree-sitter lexer settles, the shifts and reductions an LR parser keeps by
//! precedence, and the reductions of extras and lone nodes.

use std::cmp::Ordering;
use std::rc::Rc;

use super::forking::{
    GrammarFacts, Lead, declared_fork, forked_leaves, forked_order, items_order, lookahead_of,
    shift_reduction, token_at,
};
use super::parting::{
    chain_conflict, child_parting, has_node, holds_first, one_token, parting_end, reduced_first,
    same_tokens, shifted_past, silent_parting,
};
use super::program::{
    Associativity, PrecedenceTag, Settling, SettlingStep, TokenRank, compare_precedence,
};
use super::reducing::{ending_reduction, first_reduction, leading_dynamic};
use super::results::{Children, Res, TokenOrder, Tree, TreeType, join_children};
use super::walk::{
    Walk, first_leaf_start, first_meaningful, items, leftmost_chain, meaningful, next_item,
    open_node, open_part, same_tree,
};
use crate::grammar::PrecedenceEntry;

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
    // shifted): a silent rule reduced the leaf alone in one of them, or the
    // node, or one along its leftmost chain, ends with it. Where neither
    // reduced it (the `{` of a JavaScript block and of an object), the two
    // shift it alike. Two leaves alike part at their end too where only one
    // was reduced alone. `pending` is the end of that leaf, `decided` the
    // lookahead's start.
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
            (None, None) => return Ordering::Equal,
            (None, _) | (_, None) => {
                // The next token of the other parse covers a separator the
                // ended one skipped (Go's `\n` that ends the last line, where
                // `\s` is trivia): the lexer takes that valid token over the
                // separator.
                let ended = left.is_empty();
                let rest = if ended { &mut right } else { &mut left };
                if next_item(rest).is_some_and(|first| {
                    first.ty == TreeType::Token
                        && !first.trivia
                        && covers(&first, &skipped[usize::from(!ended)])
                }) {
                    return if ended {
                        Ordering::Less
                    } else {
                        Ordering::Greater
                    };
                }
                return scanned_after_end(result, existing, rest, decided, ended);
            }
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
                    let (node, lone) = if a_node { (&a, &b) } else { (&b, &a) };
                    if lone.ty != TreeType::Node && !lone.trivia && reduced_first(node, lone) {
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
                } else if a.end != b.end || !one_token(&a, &b) {
                    // Two leaves of one span the same token rule built
                    // (JavaScript's `identifier` and its alias
                    // `shorthand_property_identifier`) are one token to the
                    // lexer.
                    lookahead(&a, pending, &mut decided);
                    lookahead(&b, pending, &mut decided);
                    if a.start.min(b.start) > decided {
                        return Ordering::Equal;
                    }
                    // A scanner token against a token the lexer would lex
                    // there: tree-sitter runs the external scanner before its
                    // lexer wherever one of its tokens is valid, and takes
                    // the token it scans (JavaScript's automatic semicolon
                    // after `return` before a line break, not the next
                    // line's expression).
                    return a
                        .scanned
                        .cmp(&b.scanned)
                        .then_with(|| token_conflict(&a, &b, tokens));
                } else {
                    lookahead(&a, pending, &mut decided);
                    // One leaf a silent rule reduced alone, the other not:
                    // the two part at its end (Rust's `$` of a token tree
                    // pattern, a lone token, and of `$(...);*`).
                    if a.alone != b.alone {
                        pending = pending.min(a.end);
                    }
                    left.pop();
                    right.pop();
                    skipped = [Vec::new(), Vec::new()];
                }
            }
        }
    }
}

/// Of two results where one (`result` when `result_ended`) ended and the
/// other goes on (`rest`) with a token of the external scanner, which then
/// scans it, of no width, in the state both share (TypeScript's automatic
/// semicolon after `namespace A {}` before a line break, which the expression
/// statement takes and the declaration cannot): the lexer took it before any
/// parse decided, if the two did not part before it (`declare module "m" {}`
/// before a line break, whose `{` one parse shifts as the module's body where
/// the other reduced the module, decides first), and if the node the ended
/// parse closes with is one the other reduced too (where `{}` after a line
/// break is a statement block in one and an object in the other, the two
/// reduced apart, and the block could take the token itself). Where the
/// ended parse built nothing (an optional layout semicolon of Lean, taken or
/// not), the two part at the token, which the scanner scans first. Less when
/// `result` ended, Greater when `existing` did, Equal when the token does not
/// decide. It mirrors the end of preferredTokens in
/// js/src/grammar-runtime/executor.js.
fn scanned_after_end(
    result: &Children,
    existing: &Children,
    rest: &mut Vec<Walk>,
    decided: usize,
    result_ended: bool,
) -> Ordering {
    let (ended, other) = if result_ended {
        (result, existing)
    } else {
        (existing, result)
    };
    if let Some(last) = meaningful(ended).pop()
        && (last.ty != TreeType::Node || !has_node(other, &last))
    {
        return Ordering::Equal;
    }
    while let Some(step) = rest.last() {
        match step {
            Walk::End(_) => {
                rest.pop();
            }
            Walk::Part(_) => open_part(rest),
            Walk::Item(item) if item.trivia => {
                rest.pop();
            }
            Walk::Item(item) if item.ty == TreeType::Node => open_node(rest, false),
            Walk::Item(item) => {
                if item.start != item.end
                    || !item.scanned
                    || item.start > decided
                    || item.start > parting_end(result, existing)
                {
                    return Ordering::Equal;
                }
                return if result_ended {
                    Ordering::Less
                } else {
                    Ordering::Greater
                };
            }
        }
    }
    Ordering::Equal
}

/// Which of two results over the same text an LR parser keeps when it decides
/// a shift-reduce conflict by precedence, as tree-sitter does when the grammar
/// is generated: they are walked in order, skipping the subtrees both share,
/// to the first node that the two build from one offset but end apart. The
/// shorter one was reduced where the longer one shifted on, and as the item in
/// progress is the node's own rule, its precedence in the two decides: the
/// higher level wins and, on equal levels, the associativity of the reduced
/// node, right to shift and left to reduce. `owner` is the precedence the
/// two results are parts of, when a precedence expression holds them (see
/// `extra_reduction`), and `dynamic` the order of the two results' dynamic
/// precedences, which a fork at a declared conflict reads first. Greater when
/// `result` is kept, Less when `existing` is, Equal when neither. It mirrors
/// shiftOrder in js/src/grammar-runtime/executor.js.
pub(super) fn shift_order(
    (result, existing): (&Children, &Children),
    dynamic: Ordering,
    orders: &[Vec<PrecedenceEntry>],
    grammar: &GrammarFacts,
    bytes: &[u8],
    owner: Option<&PrecedenceTag>,
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
                    if !Rc::ptr_eq(&a, &b) {
                        let parted = silent_parting(&a, &b, &right)
                            .then_with(|| silent_parting(&b, &a, &left).reverse());
                        if parted != Ordering::Equal {
                            return parted;
                        }
                        // Or where one token rule lexed the leaf both reduced
                        // alone, each to a rule a declared conflict names, as
                        // children of one node (Java's `b` of `a = b::m;`, a
                        // `type_identifier` of an `unannotated_type` and an
                        // `identifier` of a `primary_expression`): the higher
                        // dynamic precedence wins, and then the rule defined
                        // first, as tree-sitter keeps where the forks merge.
                        // Under nodes apart, the reductions above part the
                        // two first (C's `aff;`, an `expression_statement`
                        // against a silent `empty_declaration`).
                        if same_parent(&left, &right) {
                            let forked = forked_leaves(&a, &b, grammar);
                            if forked != Ordering::Equal {
                                return dynamic.then(forked);
                            }
                        }
                    }
                    left.pop();
                    right.pop();
                    continue;
                }
                if a_node && b_node && a.start == b.start {
                    if a.end == b.end {
                        let reduced = first_reduction(&a, &b, orders);
                        if reduced != Ordering::Equal {
                            return reduced;
                        }
                    }
                    let split = chain_pair(&a, &b, true);
                    // A node the other builds whole on its leftmost chain
                    // (Lean's `foo 2` in `#check foo 2 3`) is reduced alike in
                    // both: they part above it (see `extra_reduction`).
                    let whole = split.is_none()
                        && (leftmost_chain(&b).iter().any(|node| same_tree(node, &a))
                            || leftmost_chain(&a).iter().any(|node| same_tree(node, &b)));
                    let parting = split.filter(|(x, y)| x.end != y.end).or_else(|| {
                        if whole {
                            None
                        } else {
                            chain_pair(&a, &b, false)
                        }
                    });
                    if let Some((x, y)) = parting
                        && x.end != y.end
                    {
                        let (x, y) = parted_pair(x, y);
                        let inner = child_parting(&x, &y, orders);
                        if inner != Ordering::Equal {
                            return inner;
                        }
                        let (long, short) = if x.end > y.end { (&x, &y) } else { (&y, &x) };
                        let lookahead = lookahead_of(token_at(long, short.end).as_ref(), bytes);
                        if declared_fork(grammar, short, lookahead.clone(), orders) {
                            return if x.end > y.end {
                                Ordering::Less
                            } else {
                                Ordering::Greater
                            };
                        }
                        return if x.end > y.end {
                            shift_preferred(&x, &y, orders, grammar, lookahead.as_ref())
                        } else {
                            shift_preferred(&y, &x, orders, grammar, lookahead.as_ref()).reverse()
                        };
                    }
                }
                // The same node reduced on in two ways (Rust's `m!(x);` in a
                // block, a macro invocation that `_expression_except_range`
                // reduces under `(precedence 1 none (ref macro_invocation))`
                // and `_declaration_statement` of level 0) conflicts at its
                // end: the higher precedence it was reduced with wins.
                if a_node
                    && b_node
                    && a.start == b.start
                    && let Some((x, y)) = chain_pair(&a, &b, false)
                {
                    let own = |node: &Tree, other: &Tree| {
                        node.reduced.clone().unwrap_or_else(|| {
                            PrecedenceTag::unranked(
                                node.reduced_to
                                    .iter()
                                    .find(|name| !other.reduced_to.contains(name))
                                    .cloned(),
                            )
                        })
                    };
                    let order = compare_precedence(&own(&x, &y), &own(&y, &x), orders);
                    if order != Ordering::Equal {
                        return order;
                    }
                }
                // Two nodes of different kinds over the same tokens
                // (JavaScript's `{}`, a `statement_block` and an `object`),
                // neither of which holds the other, conflict where both are
                // reduced: the higher precedence they are reduced with wins.
                if a_node
                    && b_node
                    && a.start == b.start
                    && a.end == b.end
                    && a.kind != b.kind
                    && !holds_first(&a, &b)
                    && !holds_first(&b, &a)
                    && same_tokens(&a, &b)
                {
                    // Unless one of them reduced their first token alone
                    // where the other shifted it (Lean's `f a.b`, a
                    // projection of the application `f a` and a
                    // `tactic_apply` of `f` to `a.b`): they part there, on
                    // that token, before either is reduced.
                    let head = |node: &Rc<Tree>| {
                        leftmost_chain(node)
                            .last()
                            .and_then(|last| first_meaningful(&last.children))
                    };
                    if let (Some(x), Some(y)) = (head(&a), head(&b))
                        && x.ty == TreeType::Token
                        && y.ty == TreeType::Token
                        && x.alone != y.alone
                    {
                        let lone = if x.alone {
                            lone_reduction(&b, &x, orders).reverse()
                        } else {
                            lone_reduction(&a, &y, orders)
                        };
                        if lone != Ordering::Equal {
                            return lone;
                        }
                    }
                    // When their parses forked at a declared conflict,
                    // tree-sitter keeps the higher dynamic precedence where
                    // they merge, before the lower symbol (Java's `A<B> c;`,
                    // a `generic_type` of dynamic precedence 10 against the
                    // `binary_expression` `A < B`).
                    if let Some(forked) = forked_order(&a, &b, grammar, bytes) {
                        return dynamic.then_with(|| leading_dynamic(&a, &b)).then(forked);
                    }
                    // Where the reductions that close the two differ, those
                    // decide the reduce/reduce conflict on the last token
                    // (Java's `v = 1` of `@A(v = 1)`, an `element_value_pair`
                    // that `_element_value` of precedence 2 closes, against
                    // an `assignment_expression` of 1).
                    let (close_a, close_b) = (closing_reduction(&a), closing_reduction(&b));
                    let (own_a, own_b) = if PrecedenceTag::same(close_a.as_ref(), close_b.as_ref())
                    {
                        (None, None)
                    } else {
                        (close_a, close_b)
                    };
                    let order = compare_precedence(
                        &own_a.unwrap_or_else(|| reduction(&a)),
                        &own_b.unwrap_or_else(|| reduction(&b)),
                        orders,
                    );
                    if order != Ordering::Equal {
                        return order;
                    }
                }
                let ending = ending_reduction(&a, &b, &left, orders)
                    .then_with(|| ending_reduction(&b, &a, &right, orders).reverse());
                if ending != Ordering::Equal {
                    return ending;
                }
                let lone = lone_reduction(&a, &b, orders)
                    .then_with(|| lone_reduction(&b, &a, orders).reverse());
                if lone != Ordering::Equal {
                    return lone;
                }
                let reduced = extra_reduction(&a, &b, &right, orders, owner)
                    .then_with(|| extra_reduction(&b, &a, &left, orders, owner).reverse());
                if reduced != Ordering::Equal {
                    return reduced;
                }
                if a_node && b_node && a.start == b.start {
                    let parted = chain_conflict(&a, &b, orders, grammar, bytes);
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

/// Whether the leaves on top of two walks are children of one kind of node at
/// one offset, so that no reduction above them parts the two first.
fn same_parent(left: &[Walk], right: &[Walk]) -> bool {
    let parent = |walk: &[Walk]| {
        walk.iter().rev().find_map(|step| match step {
            Walk::End(node) => Some(node.clone()),
            _ => None,
        })
    };
    match (parent(left), parent(right)) {
        (None, None) => true,
        (Some(x), Some(y)) => Rc::ptr_eq(&x, &y) || (x.kind == y.kind && x.start == y.start),
        _ => false,
    }
}

/// The precedence of the reduction that closes `node` on its last token, or
/// None: the precedence the token was reduced with alone, or the one it was
/// lexed under, or the one its last child node closes with (Java's
/// `element_value_pair` `v = 1`, whose `1` the silent `_element_value` of
/// precedence 2 reduced, while the pair itself is reduced with none). It
/// mirrors closingReduction in js/src/grammar-runtime/executor.js.
fn closing_reduction(node: &Tree) -> Option<PrecedenceTag> {
    let last = meaningful(&node.children).pop()?;
    match last.ty {
        TreeType::Token => last.reduced.clone().or_else(|| last.precedence.clone()),
        TreeType::Node => last.closes.clone(),
        _ => None,
    }
}

/// Of a precedence decided on equal levels, Greater for a shift (right
/// associativity), Less for a reduction (left), Equal for none.
pub(super) const fn by_associativity(associativity: Associativity) -> Ordering {
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
/// precedence, and so is a node that ends one (`closes`, the `!c` of
/// `let ... && !c`). Greater when `a`'s result is kept, Less when `b`'s is,
/// Equal when neither. It mirrors loneReduction in
/// js/src/grammar-runtime/executor.js.
fn lone_reduction(a: &Rc<Tree>, b: &Rc<Tree>, orders: &[Vec<PrecedenceEntry>]) -> Ordering {
    let closing = b.ty == TreeType::Node && b.closes.is_some();
    if a.ty != TreeType::Node || (b.ty != TreeType::Token && !closing) {
        return Ordering::Equal;
    }
    let mut progress = a.clone();
    loop {
        let Some(first) = first_meaningful(&progress.children) else {
            return Ordering::Equal;
        };
        if same_tree(&first, b) {
            break;
        }
        if first.ty != TreeType::Node {
            return Ordering::Equal;
        }
        progress = first;
    }
    let shifted = progress
        .precedence
        .clone()
        .unwrap_or_else(|| PrecedenceTag::unranked(progress.rule.clone()));
    let reduced = if closing {
        b.closes.clone()
    } else {
        b.reduced.clone().or_else(|| b.precedence.clone())
    }
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
/// goes on, the associativity of the reduced node. The results themselves
/// are in progress under `owner`, the precedence a precedence expression
/// holds them with (TypeScript's `extends A<X>`, whose
/// `_extends_clause_single` takes `A` and `<X>` under the name `extends`
/// where an `instantiation_expression` reduces them under `instantiation`,
/// which the order ranks below), else under none. Greater when `a`'s result
/// is kept, Less when the other is, Equal when neither. It mirrors
/// extraReduction in js/src/grammar-runtime/executor.js.
fn extra_reduction(
    a: &Rc<Tree>,
    b: &Rc<Tree>,
    walk: &[Walk],
    orders: &[Vec<PrecedenceEntry>],
    owner: Option<&PrecedenceTag>,
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
    if container.is_none_or(|node| node.end > parent.end) {
        return by_associativity(mine.associativity).reverse();
    }
    Ordering::Equal
}

/// The outermost node kind on the leftmost chains of two nodes that start at
/// one offset, as the pair of its nodes. When `distinct`, a node the other
/// chain holds too, the same tree over the same text, is not paired: the two
/// parses built it alike (Rust's `a + b` in `a + b..*c`, the left operand of
/// both the range `a + b..` and the binary expression `a + b..*c`), so they
/// part above it. It mirrors chainPair in js/src/grammar-runtime/executor.js.
fn chain_pair(a: &Rc<Tree>, b: &Rc<Tree>, distinct: bool) -> Option<(Rc<Tree>, Rc<Tree>)> {
    let (mine, other) = (leftmost_chain(a), leftmost_chain(b));
    let shared = |node: &Rc<Tree>, chain: &[Rc<Tree>]| {
        distinct
            && chain
                .iter()
                .any(|peer| peer.end == node.end && same_tree(peer, node))
    };
    mine.iter()
        .filter(|node| !shared(node, &other))
        .find_map(|node| {
            other
                .iter()
                .find(|candidate| {
                    candidate.kind == node.kind
                        && candidate.start == node.start
                        && !shared(candidate, &mine)
                })
                .map(|found| (node.clone(), found.clone()))
        })
}

/// The innermost pair of nodes of one kind from one offset that end apart,
/// below two such nodes `a` and `b` along their leftmost chains: the two
/// parses part where the first of them ends, so the decision is that pair's
/// (Rust's `g(|| a, |p| p)`, where the closures `||` and `|| a, |p|` part at
/// the parameters `||`, the or-pattern `| a` of level -2 going on past them).
/// It mirrors partedPair in js/src/grammar-runtime/executor.js.
fn parted_pair(a: Rc<Tree>, b: Rc<Tree>) -> (Rc<Tree>, Rc<Tree>) {
    let mut pair = (a, b);
    loop {
        let below = match (
            first_meaningful(&pair.0.children),
            first_meaningful(&pair.1.children),
        ) {
            (Some(left), Some(right))
                if left.ty == TreeType::Node && right.ty == TreeType::Node =>
            {
                chain_pair(&left, &right, false)
            }
            _ => None,
        };
        match below {
            Some(below) if below.0.end != below.1.end => pair = below,
            _ => return pair,
        }
    }
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

/// Greater when the shift that built `long` is preferred to the reduction
/// that ended `short` (the same node kind from the same offset), Less when
/// the reduction is, Equal when the precedences cannot tell. The shift is in
/// the innermost node of `long` that goes on past the end of `short`. When
/// that node began with `short`, as a binary expression whose left operand
/// is the expression a statement is of, the long result reduced that operand
/// to a silent rule where the short one reduced its node: the two reductions
/// conflict instead, and a silent rule's reduction is of level 0. Where the
/// shift's own precedence cannot tell, every item that shifts `lookahead`
/// after the reduced part may, as tree-sitter's `handle_conflict` compares
/// the reduction with each (Rocq's `try x; [a | b]`, where `tactic_branch` of
/// none cannot rank the `tactical` of `tactic_application` but
/// `tactic_sequence`, ranked below it, shifts `;` too: the tactical is
/// reduced).
pub(super) fn shift_preferred(
    long: &Rc<Tree>,
    short: &Rc<Tree>,
    orders: &[Vec<PrecedenceEntry>],
    grammar: &GrammarFacts,
    lookahead: Option<&Lead>,
) -> Ordering {
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
    if let Some(before) = shift_reduction(short, &progress, grammar) {
        let order = compare_precedence(&PrecedenceTag::unranked(Some(before)), &reduced, orders);
        if order != Ordering::Equal {
            return order;
        }
    }
    compare_precedence(&shifted, &reduced, orders)
        .then_with(|| items_order(grammar, short, &reduced, lookahead, orders))
        .then_with(|| by_associativity(reduced.associativity))
}

/// The precedence a node is reduced with: the innermost one over its last
/// part, as a generated parser takes the precedence of a production's last
/// step (Rust's `let` condition, whose value is `(precedence 3 left (ref
/// expression))`, reduces before the `&&` of a binary expression of level
/// 3), else none of level 0, ranked by the node's rule.
pub(super) fn reduction(node: &Tree) -> PrecedenceTag {
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
/// `lone_reduction`); else the precedence of the innermost silent rule on
/// the chain that the chain's token or node ends (TypeScript's `namespace N
/// {}` and `module "m" {}`, whose `module_name_and_body` of level 0 right
/// ends with the name where the body is left out, so the body shifts); else
/// the precedence `short` reduces with.
fn reduced_before(short: &Rc<Tree>, progress: &Tree) -> PrecedenceTag {
    let mut closing = None;
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
                if last.ty == TreeType::Token
                    && let Some(reduced) = &last.reduced
                {
                    return reduced.clone();
                }
                // A token a silent rule reduced alone where the shift takes
                // it as the first part of its node (Lean's `c` in `fun x c
                // s`, a `_pattern` of level 0 in one parse and the
                // constructor of `c s`, of level 80, in the other) is that
                // rule's reduction, not the node's it ends.
                if last.ty == TreeType::Token && last.alone && !first.alone {
                    return last
                        .precedence
                        .clone()
                        .unwrap_or(PrecedenceTag::unranked(None));
                }
                return reduction(&node);
            }
            node = last;
            let closes = if node.ty == TreeType::Token {
                node.reduced.as_ref().or(node.precedence.as_ref())
            } else {
                node.closes.as_ref()
            };
            if let Some(closes) = closes {
                closing = Some(closes.clone());
            }
        }
    }
    closing.unwrap_or_else(|| reduction(short))
}

/// The order of two results of one text that end alike, each its children
/// and dynamic precedence, by the grammar's settling steps: Greater when `a`
/// is preferred, Less when `b` is, Equal on a tie. `tokens` holds the token
/// ranks, the input bytes and the precedence orders the `tokens` step (the
/// tokens a lexer prefers, as a lexer decides them before any parse does)
/// and the `precedence` step (the shift or reduction an LR parser keeps by
/// precedence, under `owner`) read; `dynamic` prefers the higher dynamic
/// precedence. It mirrors settledOrder in js/src/grammar-runtime/executor.js.
pub(super) fn settled_order(
    a: (&Children, i64),
    b: (&Children, i64),
    settling: Settling,
    tokens: Option<TokenOrder<'_>>,
    owner: Option<&PrecedenceTag>,
) -> Ordering {
    for step in settling.steps() {
        let order = match step {
            SettlingStep::Tokens => {
                tokens.map_or(Ordering::Equal, |tokens| preferred_tokens(a.0, b.0, tokens))
            }
            SettlingStep::Precedence => tokens.map_or(Ordering::Equal, |tokens| {
                shift_order(
                    (a.0, b.0),
                    a.1.cmp(&b.1),
                    tokens.orders,
                    tokens.grammar,
                    tokens.bytes,
                    owner,
                )
            }),
            SettlingStep::Dynamic => a.1.cmp(&b.1),
            SettlingStep::First | SettlingStep::Ambiguity => break,
        };
        if order != Ordering::Equal {
            return order;
        }
    }
    Ordering::Equal
}

/// Of two complete results, each with its trailing trivia, Greater when `a`
/// is preferred, Less when `b` is, Equal on a tie: they end apart before the
/// trailing trivia, so they are ranked as `ResultSet::add` ranks results with
/// one end.
pub(super) fn complete_order(
    a: (&Res, &Children),
    b: (&Res, &Children),
    settling: Settling,
    tokens: Option<TokenOrder<'_>>,
) -> Ordering {
    if a.0.cost != b.0.cost {
        return b.0.cost.cmp(&a.0.cost);
    }
    settled_order(
        (&join_children(&a.0.children, a.1), a.0.dynamic),
        (&join_children(&b.0.children, b.1), b.0.dynamic),
        settling,
        tokens,
        None,
    )
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
