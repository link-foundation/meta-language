//! Precedence and associativity of the native executor, as the precedence
//! filter of `js/src/grammar-runtime/executor.js`.

use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

use super::executor::{Executor, Run};
use super::forking::{declared_fork, direct_edge, edge_names, lookahead_of};
use super::operations::State;
use super::ordering::first_meaningful;
use super::program::{Associativity, Expr, Name, PrecedenceTag, Target, compare_precedence};
use super::results::{Res, ResultSet, Tree, TreeType, children_of};
use crate::grammar::RuleKind;

/// The node kinds the leftmost and rightmost operand of a precedence
/// expression can match as one child, or None when unknown.
pub(super) struct Operands {
    left: Option<HashSet<Name>>,
    right: Option<HashSet<Name>>,
    /// The rules the expression takes directly as its last part (see
    /// `reduction_below`).
    last: Vec<Name>,
}

/// A precedence filter: the precedence, the operand kinds and the address of the precedence's item, by which its rule is known (see
/// `reaches_owner`), with the results whose right operand is of one part,
/// which wait for the others (see `lone_pending`).
pub(super) struct Keep {
    tag: PrecedenceTag,
    operands: Rc<Operands>,
    item: usize,
    pending: RefCell<Vec<Res>>,
}

/// The verdict of the precedence filter on an operand.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum Conflict {
    No,
    Yes,
    /// A right operand of one part: valid unless a result ends before it
    /// (see `lone_pending`).
    Lone,
}

/// Whether an expression may match nothing, as far as its shape tells.
pub(super) fn nullable(expr: &Expr) -> bool {
    match expr {
        Expr::Empty | Expr::And(_) | Expr::Not(_) => true,
        Expr::Repeat { item, min, .. } => *min == 0 || nullable(item),
        Expr::Seq(items) => items.iter().all(nullable),
        Expr::Choice { items, .. } => items.iter().any(nullable),
        Expr::Capture { item, .. }
        | Expr::Precedence { item, .. }
        | Expr::DynamicPrecedence { item, .. }
        | Expr::Alias { item, .. } => nullable(item),
        _ => false,
    }
}

/// The union of kind sets, or None when any is unknown.
fn union(sets: impl Iterator<Item = Option<HashSet<Name>>>) -> Option<HashSet<Name>> {
    let mut all = HashSet::new();
    for set in sets {
        all.extend(set?);
    }
    Some(all)
}

impl Executor<'_> {
    // Precedence and associativity filter the binary-shaped results: a
    // leftmost or rightmost child node of lower precedence, or of equal
    // precedence on the side the associativity forbids, invalidates a result
    // when it conflicts, that is when its own child facing the operator could
    // have been the operand instead (`-a->t` but not `f(a)->t`).
    pub(super) fn precedence_valid(&mut self, keep: &Keep, result: &Res) -> bool {
        let meaningful: Vec<Rc<Tree>> = result
            .children
            .iter()
            .filter(|child| !child.trivia)
            .cloned()
            .collect();
        let verdict = if meaningful.len() < 2 {
            Conflict::No
        } else if self.conflicts(keep, &meaningful[0], Associativity::Left, meaningful.get(1))
            == Conflict::Yes
        {
            Conflict::Yes
        } else {
            self.conflicts(
                keep,
                &meaningful[meaningful.len() - 1],
                Associativity::Right,
                None,
            )
        };
        match verdict {
            Conflict::No => true,
            Conflict::Lone => {
                keep.pending.borrow_mut().push(result.clone());
                false
            }
            Conflict::Yes => {
                self.fail(result.end, &Name::from("precedence"));
                false
            }
        }
    }

    // The `results` of a precedence expression with the `pending` ones whose
    // right operand is a node of one part (Rust's bare range `..`) that an
    // LR parser shifts: where a result of the expression ends before that
    // operand (`..` in `.. ..`, `a..` in `a .. ..`), the parser reduces it
    // there first, as the operand's level is not above the operator's
    // (`a ..= ..` stands).
    fn lone_pending(&mut self, results: Vec<Res>, pending: Vec<Res>) -> Vec<Res> {
        let ends: HashSet<usize> = results.iter().map(|result| result.end).collect();
        let mut found = ResultSet::new(self.longest_tokens);
        for result in results {
            found.set(result);
        }
        for result in pending {
            let meaningful: Vec<&Rc<Tree>> = result
                .children
                .iter()
                .filter(|child| !child.trivia)
                .collect();
            if ends.contains(&meaningful[meaningful.len() - 2].end) {
                self.fail(result.end, &Name::from("precedence"));
                continue;
            }
            found.add(result);
        }
        found.items
    }

    // Whether the operand `child` on `side` conflicts with the precedence of
    // `keep` (see `precedence_valid`).
    pub(super) fn conflicts(
        &mut self,
        keep: &Keep,
        child: &Tree,
        side: Associativity,
        next: Option<&Rc<Tree>>,
    ) -> Conflict {
        let Some(inner) = &child.precedence else {
            return Conflict::No;
        };
        if child.ty != TreeType::Node {
            return Conflict::No;
        }
        let order = compare_precedence(inner, &keep.tag, &self.program.precedence_orders);
        if order == std::cmp::Ordering::Greater
            || (order == std::cmp::Ordering::Equal && keep.tag.associativity == side)
        {
            return Conflict::No;
        }
        if let Some(kind) = &child.rule {
            if side == Associativity::Right && self.shifts_below(keep, child, kind) {
                return Conflict::No;
            }
            if side == Associativity::Right && self.lexed_shift(kind) {
                return Conflict::No;
            }
            if !self.reaches_owner(keep.item, kind, side) {
                return Conflict::No;
            }
            if next.is_some()
                && declared_fork(
                    &self.program.grammar,
                    child,
                    lookahead_of(next, self.bytes),
                    &self.program.precedence_orders,
                )
            {
                return Conflict::No;
            }
        }
        // A child of one part (Rust's bare range `..` in `a ..= ..`) has no
        // operand of its own the operator could have taken instead; on the
        // right, the parse before it could still have been reduced first.
        let facing: Vec<&Rc<Tree>> = child
            .children
            .iter()
            .filter(|grandchild| !grandchild.trivia)
            .collect();
        if facing.len() < 2 {
            return if side == Associativity::Right {
                Conflict::Lone
            } else {
                Conflict::No
            };
        }
        let kinds = if side == Associativity::Left {
            &keep.operands.left
        } else {
            &keep.operands.right
        };
        let Some(kinds) = kinds else {
            return Conflict::Yes;
        };
        let edge = if side == Associativity::Left {
            facing.last()
        } else {
            facing.first()
        };
        if edge
            .and_then(|edge| edge.kind.as_ref())
            .is_some_and(|kind| kinds.contains(kind))
        {
            Conflict::Yes
        } else {
            Conflict::No
        }
    }

    // Whether a generated parser shifts into the right operand `child`, of
    // the rule `rule`, before it could reduce the operand's first part up to
    // the operator's operand, as tree-sitter's handle_conflict decides: where
    // the child's rule takes a part at its edge directly (`primary_expression`
    // of TypeScript's member expression), the shift into the child conflicts
    // with the reduction of that part into the rule above it on the way to
    // the operand (`expression`), not with the operator's own reduction, and
    // the shift wins when the child's precedence is higher than that
    // reduction's (`member` ranks above the rule `expression`), so that
    // `<C>e.f` asserts the type of `e.f`. It mirrors shiftsBelow in
    // js/src/grammar-runtime/executor.js.
    fn shifts_below(&mut self, keep: &Keep, child: &Tree, rule: &Name) -> bool {
        let Some(kind) = first_meaningful(&child.children).and_then(|first| first.kind.clone())
        else {
            return false;
        };
        let Some(&index) = self.program.rule_index.get(&**rule) else {
            return false;
        };
        let key = (keep.item, rule.clone(), kind);
        let reduction = if let Some(reduction) = self.below_memo.get(&key) {
            reduction.clone()
        } else {
            let reduction = self.reduction_below(&keep.operands.last, index, &key.2);
            self.below_memo.insert(key, reduction.clone());
            reduction
        };
        let (Some(reduction), Some(inner)) = (reduction, &child.precedence) else {
            return false;
        };
        compare_precedence(inner, &reduction, &self.program.precedence_orders)
            == std::cmp::Ordering::Greater
    }

    // The precedence of the deepest reduction of a part that the rule at
    // `index` takes directly at its start, made on the way from the right
    // operand of a precedence, whose last part takes the rules `last`
    // directly, down to a node of `kind`, or None when none is.
    fn reduction_below(&self, last: &[Name], index: usize, kind: &Name) -> Option<PrecedenceTag> {
        let rules = &self.program.rules;
        let rule = &rules[index];
        let mut direct = Vec::new();
        direct_edge(
            &rule.expression,
            Associativity::Right,
            Some(&rule.name),
            rules,
            &mut direct,
            None,
        );
        let direct: HashSet<Name> = direct.into_iter().map(|(name, _)| name).collect();
        let derives = |name: &Name| {
            let mut names = HashSet::new();
            let target = self.program.rule_index.get(&**name).map_or_else(
                || Target::External(name.clone()),
                |&index| Target::Rule(index),
            );
            edge_names(&Expr::Ref(target), Associativity::Right, rules, &mut names);
            names.iter().any(|each| {
                each == kind
                    || self
                        .program
                        .rule_index
                        .get(&**each)
                        .is_some_and(|&at| rules[at].node_kind == *kind)
            })
        };
        let mut found = None;
        let mut seen = HashSet::new();
        let mut level = last.to_vec();
        while !level.is_empty() {
            let mut deeper = Vec::new();
            for name in &level {
                let Some(&at) = self.program.rule_index.get(&**name) else {
                    continue;
                };
                if !seen.insert(name.clone()) {
                    continue;
                }
                let mut parts = Vec::new();
                direct_edge(
                    &rules[at].expression,
                    Associativity::Right,
                    Some(name),
                    rules,
                    &mut parts,
                    None,
                );
                for (part, tag) in parts {
                    if direct.contains(&part) && derives(&part) {
                        found = Some(
                            tag.unwrap_or_else(|| PrecedenceTag::unranked(Some(name.clone()))),
                        );
                    }
                    if self
                        .program
                        .rule_index
                        .get(&*part)
                        .is_some_and(|&at| matches!(rules[at].kind, RuleKind::Silent))
                    {
                        deeper.push(part);
                    }
                }
            }
            level = deeper;
        }
        found
    }

    // The rule a node of `kind` is built by under its own name, if any.
    pub(super) fn own_rule(&self, kind: &str) -> Option<usize> {
        self.program
            .rule_index
            .get(kind)
            .copied()
            .filter(|&index| &*self.program.rules[index].node_kind == kind)
    }

    // Whether a node of `kind` goes on after its first child only with
    // tokens of raised lexical precedence (Rust's `B<C>`, whose `<` is
    // `(token (lexicalPrecedence 1 (literal <)))`): a lexer takes such a
    // token wherever the parse admits it, so the parse shifts it instead of
    // reducing the operator before the node, and the lower precedence of the
    // node as a right operand is no conflict.
    pub(super) fn lexed_shift(&mut self, kind: &Name) -> bool {
        if let Some(&shifts) = self.shift_memo.get(kind) {
            return shifts;
        }
        let program = self.program;
        let mut body = self
            .own_rule(kind)
            .map(|index| &program.rules[index].expression);
        while let Some(
            Expr::Precedence { item, .. }
            | Expr::DynamicPrecedence { item, .. }
            | Expr::Capture { item, .. },
        ) = body
        {
            body = Some(item);
        }
        let shifts = match body {
            Some(Expr::Seq(items)) if items.len() > 1 && !nullable(&items[0]) => {
                self.lead_priority_seq(&items[1..], &mut HashSet::new())
                    .unwrap_or(0)
                    > 0
            }
            _ => false,
        };
        self.shift_memo.insert(kind.clone(), shifts);
        shifts
    }

    // The lowest lexical precedence of the tokens `expr` can begin with, or
    // None when it may match nothing first.
    pub(super) fn lead_priority(&self, expr: &Expr, visiting: &mut HashSet<usize>) -> Option<i64> {
        match expr {
            Expr::Token(item) | Expr::ImmediateToken(item) => Some(match &**item {
                Expr::LexicalPrecedence { level, .. } => *level,
                _ => 0,
            }),
            Expr::LexicalPrecedence { level, .. } => Some(*level),
            Expr::Ref(Target::Rule(index)) => {
                let rule = &self.program.rules[*index];
                if matches!(rule.kind, RuleKind::Token | RuleKind::Atomic) {
                    return Some(rule.lexical_priority);
                }
                if !visiting.insert(*index) {
                    return None;
                }
                self.lead_priority(&rule.expression, visiting)
            }
            Expr::Seq(items) => self.lead_priority_seq(items, visiting),
            Expr::Choice { items, .. } => items
                .iter()
                .filter_map(|item| self.lead_priority(item, visiting))
                .min(),
            Expr::Capture { item, .. }
            | Expr::Precedence { item, .. }
            | Expr::DynamicPrecedence { item, .. }
            | Expr::Alias { item, .. }
            | Expr::Repeat { item, .. } => self.lead_priority(item, visiting),
            Expr::Empty | Expr::And(_) | Expr::Not(_) => None,
            _ => Some(0),
        }
    }

    pub(super) fn lead_priority_seq(
        &self,
        items: &[Expr],
        visiting: &mut HashSet<usize>,
    ) -> Option<i64> {
        let mut lowest: Option<i64> = None;
        for item in items {
            if let Some(lead) = self.lead_priority(item, visiting) {
                lowest = Some(lowest.map_or(lead, |lowest| lowest.min(lead)));
            }
            if !nullable(item) {
                return lowest;
            }
        }
        lowest
    }

    // Whether the rule whose body holds the precedence of `item` may stand at
    // the edge of a `kind` node that faces the operator on `side` (its last
    // part for the left operand): only then could a generated parser build
    // the operator's node inside the operand, so that the two conflict. A
    // field of Rust's `a.0.1` is never a field expression, so `a.0` is no
    // operand of lower precedence there. True when either rule is unknown.
    pub(super) fn reaches_owner(&mut self, item: usize, kind: &Name, side: Associativity) -> bool {
        let program = self.program;
        let owner = self.owner_of(item);
        let (Some(owner), Some(index)) = (owner, self.own_rule(kind)) else {
            return true;
        };
        let key = (kind.clone(), side);
        let names = if let Some(names) = self.edge_memo.get(&key) {
            Rc::clone(names)
        } else {
            let mut names = HashSet::new();
            self.edge_rules(&program.rules[index].expression, side, &mut names);
            let names = Rc::new(names);
            self.edge_memo.insert(key, Rc::clone(&names));
            names
        };
        names.contains(&owner)
    }

    // The index of the rule whose body holds the precedence of `item` (the
    // address of the precedence's item), or None.
    pub(super) fn owner_of(&mut self, item: usize) -> Option<usize> {
        let program = self.program;
        let owners = self.owners.get_or_insert_with(|| {
            let mut owners = HashMap::new();
            for (index, rule) in program.rules.iter().enumerate() {
                let mut pending = vec![&rule.expression];
                while let Some(expr) = pending.pop() {
                    match expr {
                        Expr::Precedence { item, .. } => {
                            owners
                                .entry(std::ptr::from_ref(&**item) as usize)
                                .or_insert(index);
                            pending.push(item);
                        }
                        Expr::Seq(items) | Expr::Choice { items, .. } | Expr::Longest(items) => {
                            pending.extend(items.iter().rev());
                        }
                        Expr::Repeat { item, .. }
                        | Expr::And(item)
                        | Expr::Not(item)
                        | Expr::Capture { item, .. }
                        | Expr::Alias { item, .. }
                        | Expr::DynamicPrecedence { item, .. }
                        | Expr::LexicalPrecedence { item, .. }
                        | Expr::Token(item)
                        | Expr::ImmediateToken(item)
                        | Expr::Predicate { item, .. }
                        | Expr::Recover { item, .. }
                        | Expr::Missing { item, .. }
                        | Expr::Embed { item, .. } => pending.push(item),
                        Expr::Empty | Expr::Terminal { .. } | Expr::Ref(_) => {}
                    }
                }
            }
            owners
        });
        owners.get(&item).copied()
    }

    // Adds to `names` the rules `expr` may match at its edge facing the
    // operator on `side`, through the silent rules there, as `unit_kinds`
    // walks.
    pub(super) fn edge_rules(&self, expr: &Expr, side: Associativity, names: &mut HashSet<usize>) {
        match expr {
            Expr::Ref(Target::Rule(index)) => {
                if !names.insert(*index) {
                    return;
                }
                let rule = &self.program.rules[*index];
                if matches!(rule.kind, RuleKind::Silent) {
                    self.edge_rules(&rule.expression, side, names);
                }
            }
            Expr::Seq(items) => {
                let ordered: Box<dyn Iterator<Item = &Expr>> = if side == Associativity::Left {
                    Box::new(items.iter().rev())
                } else {
                    Box::new(items.iter())
                };
                for item in ordered {
                    self.edge_rules(item, side, names);
                    if !matches!(item, Expr::Repeat { min: 0, .. }) {
                        return;
                    }
                }
            }
            Expr::Choice { items, .. } => {
                for item in items {
                    self.edge_rules(item, side, names);
                }
            }
            Expr::Capture { item, .. }
            | Expr::Precedence { item, .. }
            | Expr::DynamicPrecedence { item, .. }
            | Expr::Alias { item, .. }
            | Expr::Repeat { item, .. } => self.edge_rules(item, side, names),
            _ => {}
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub(super) fn precedence(
        &mut self,
        level: i64,
        name: Option<&Name>,
        associativity: Associativity,
        item: &Expr,
        position: usize,
        state: &State,
        in_token: bool,
    ) -> Run<Vec<Res>> {
        // The rule a precedence ranks is the one whose body holds it.
        let address = std::ptr::from_ref(item) as usize;
        let rule = self
            .owner_of(address)
            .map(|index| self.program.rules[index].name.clone());
        let tag = PrecedenceTag {
            level,
            name: name.cloned(),
            associativity,
            rule,
        };
        let mut results = if in_token {
            self.evaluate(item, position, state, in_token)?
        } else {
            let keep = Keep {
                tag: tag.clone(),
                operands: self.operand_kinds(item),
                item: address,
                pending: RefCell::new(Vec::new()),
            };
            let results = self.filtered(item, position, state, &keep)?;
            let pending = keep.pending.take();
            if pending.is_empty() {
                results
            } else {
                self.lone_pending(results, pending)
            }
        };
        for result in &mut results {
            result.precedence = Some(tag.clone());
            if in_token {
                continue;
            }
            // The innermost precedence over the last part of a result is the
            // one its rule reduces with (see `reduction` in results.rs).
            if result.tail.is_none() {
                result.tail = Some(tag.clone());
            }
            // A token reduced alone keeps the precedence on its leaf, for the
            // conflict with a shift after it (see `lone_reduction`), and a
            // node reduced alone keeps it as the precedence it was reduced
            // with, for the conflict with another reduction of it (see
            // `shift_order`).
            let mut meaningful = result
                .children
                .iter()
                .enumerate()
                .filter(|(_, child)| !child.trivia);
            if let (Some((at, only)), None) = (meaningful.next(), meaningful.next())
                && matches!(only.ty, TreeType::Token | TreeType::Node)
            {
                let mut tagged = (**only).clone();
                if only.ty == TreeType::Token {
                    tagged.precedence = Some(tag.clone());
                } else {
                    tagged.reduced = Some(tag.clone());
                }
                let mut children = result.children.to_vec();
                children[at] = Rc::new(tagged);
                result.children = children_of(children);
            }
        }
        Ok(results)
    }

    // The node kinds the leftmost and rightmost operand of a precedence
    // expression can match as one child, or None when unknown.
    pub(super) fn operand_kinds(&mut self, item: &Expr) -> Rc<Operands> {
        let key = std::ptr::from_ref(item) as usize;
        if let Some(operands) = self.operand_memo.get(&key) {
            return Rc::clone(operands);
        }
        let mut last = Vec::new();
        direct_edge(
            item,
            Associativity::Left,
            None,
            &self.program.rules,
            &mut last,
            None,
        );
        let operands = Rc::new(Operands {
            left: self.edge_kinds(item, Associativity::Left),
            right: self.edge_kinds(item, Associativity::Right),
            last: last.into_iter().map(|(name, _)| name).collect(),
        });
        self.operand_memo.insert(key, Rc::clone(&operands));
        operands
    }

    pub(super) fn edge_kinds(&self, item: &Expr, side: Associativity) -> Option<HashSet<Name>> {
        match item {
            Expr::Capture { item, .. } => self.edge_kinds(item, side),
            Expr::Seq(items) if !items.is_empty() => {
                let operand = if side == Associativity::Left {
                    &items[0]
                } else {
                    &items[items.len() - 1]
                };
                self.unit_kinds(operand, &mut HashSet::new(), side)
            }
            Expr::Choice { items, .. } => {
                union(items.iter().map(|item| self.edge_kinds(item, side)))
            }
            _ => None,
        }
    }

    // The node kinds `expr` can match as one child, or None when unknown;
    // the child at the edge of a repetition (or an optional) is one of its
    // item. The operand on the `side` of the operator faces it with the
    // opposite end of a sequence a silent rule inlines: its first item for
    // the right operand, its last for the left one, and the items after it
    // while those may match nothing.
    pub(super) fn unit_kinds(
        &self,
        expr: &Expr,
        visiting: &mut HashSet<usize>,
        side: Associativity,
    ) -> Option<HashSet<Name>> {
        match expr {
            Expr::Ref(Target::Rule(index)) => {
                let rule = &self.program.rules[*index];
                if !matches!(rule.kind, RuleKind::Silent) {
                    return Some(HashSet::from([rule.node_kind.clone()]));
                }
                if !visiting.insert(*index) {
                    return Some(HashSet::new());
                }
                self.unit_kinds(&rule.expression, visiting, side)
            }
            Expr::Seq(items) if !items.is_empty() => {
                let ordered: Box<dyn Iterator<Item = &Expr>> = if side == Associativity::Left {
                    Box::new(items.iter().rev())
                } else {
                    Box::new(items.iter())
                };
                let mut kinds = Vec::new();
                for item in ordered {
                    kinds.push(self.unit_kinds(item, visiting, side));
                    if !matches!(item, Expr::Repeat { min: 0, .. }) {
                        break;
                    }
                }
                union(kinds.into_iter())
            }
            Expr::Choice { items, .. } => union(
                items
                    .iter()
                    .map(|item| self.unit_kinds(item, visiting, side)),
            ),
            Expr::Capture { item, .. }
            | Expr::Precedence { item, .. }
            | Expr::DynamicPrecedence { item, .. }
            | Expr::Repeat { item, .. } => self.unit_kinds(item, visiting, side),
            // A scanner token is a leaf of its own kind.
            Expr::Alias { name, .. } | Expr::Ref(Target::External(name)) => {
                Some(HashSet::from([name.clone()]))
            }
            _ => None,
        }
    }

    // The results of `expr` that the precedence filter accepts, filtered
    // before a sequence or an unordered choice merges results of the same
    // end and state.
    pub(super) fn filtered(
        &mut self,
        expr: &Expr,
        position: usize,
        state: &State,
        keep: &Keep,
    ) -> Run<Vec<Res>> {
        match expr {
            Expr::Seq(items) if !items.is_empty() => {
                self.sequence(items, position, state, false, Some(keep))
            }
            Expr::Choice {
                ordered: false,
                items,
            } if !self.peg => {
                self.step()?;
                let mut results = ResultSet::new(self.longest_tokens);
                for item in items {
                    for result in self.filtered(item, position, state, keep)? {
                        results.add(result);
                    }
                }
                Ok(results.items)
            }
            _ => {
                let results = self.evaluate(expr, position, state, false)?;
                Ok(results
                    .into_iter()
                    .filter(|result| self.precedence_valid(keep, result))
                    .collect())
            }
        }
    }

    pub(super) fn priority_of(&self, item: &Expr) -> i64 {
        match item {
            Expr::LexicalPrecedence { level, .. } => *level,
            Expr::Ref(Target::Rule(index)) => self.program.rules[*index].lexical_priority,
            _ => 0,
        }
    }
}
