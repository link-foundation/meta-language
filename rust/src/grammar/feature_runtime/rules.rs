//! Rule calls, semantic actions, external scanners and the whole-input run of
//! the native executor, as the second half of
//! `js/src/grammar-runtime/executor.js`.

use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::HashSet;
use std::rc::Rc;
use std::sync::Arc;

use super::executor::{Element, Executor, Run};
use super::forking::target_name;
use super::operations::{
    Abort, Machine, OpError, OpResult, OperationValue, State, Working, run_statements,
};
use super::program::{Expr, Name, PrecedenceTag, Rule, Target};
use super::results::{
    Children, Entry, Outcome, Res, ResultSet, Scanned, Tree, TreeType, children_of, complete_order,
    concat, content_start, is_separator, longest_result, no_children, reduced_alone, same_children,
    with_leaf,
};
use super::text::{column_of, decode_at};
use crate::grammar::RuleKind;

impl Executor<'_> {
    /// Records that the parse requests `item`, a literal or a rule a
    /// scanner's `expected` may ask about, at `position` (see
    /// `Expectations`).
    pub(super) fn request_item(&self, item: &Expr, position: usize) {
        let program = self.program;
        let id = match item {
            Expr::Ref(target) => program
                .expected_references
                .get(&**target_name(target, &program.rules))
                .copied(),
            Expr::Terminal { expected, .. } => *expected,
            _ => None,
        };
        if let Some(id) = id {
            self.shared
                .expectations
                .borrow_mut()
                .request(position, (self.program_index, id));
        }
    }

    /// A rule call or an external token, memoized with left-recursion growth.
    pub(super) fn reference(
        &mut self,
        target: &Target,
        position: usize,
        state: &State,
        in_token: bool,
    ) -> Run<Vec<Res>> {
        let program = self.program;
        if !in_token && !program.expected_references.is_empty() {
            let name = match target {
                Target::External(name) => name,
                Target::Rule(index) => &program.rules[*index].name,
            };
            if let Some(&id) = program.expected_references.get(&**name) {
                self.shared
                    .expectations
                    .borrow_mut()
                    .request(position, (self.program_index, id));
            }
        }
        let index = match target {
            Target::External(name) => return self.scanner_token(name, position, state, in_token),
            Target::Rule(index) => *index,
        };
        let rule = &program.rules[index];
        if let Some(modes) = &rule.modes
            && !modes.iter().any(|mode| **mode == *state.mode())
        {
            self.fail(position, &rule.node_kind);
            return Ok(Vec::new());
        }
        // While repairing, a call made quietly (where nothing is repaired) or
        // after a MISSING leaf at its offset is memoized apart from the same
        // call made in the open.
        let key = (
            index,
            position,
            state.clone(),
            in_token,
            self.repair_mode(position),
        );
        if let Some(known) = self.memo.get(&key).cloned() {
            if let Some(keywords) = self.keywords {
                let parent = self.call_stack.last().and_then(|frame| frame.borrow().call);
                if let Some(call) = known.borrow().call {
                    keywords.borrow_mut().called(call, parent);
                }
            }
            let seed = {
                let mut entry = known.borrow_mut();
                if !entry.evaluating {
                    return Ok(entry.results.clone());
                }
                // Left recursion: answer with the current seed and mark every
                // call between the two as depending on it, so none of them is memoized.
                entry.left_recursive = true;
                entry.seed.clone()
            };
            for frame in self.call_stack.iter().rev() {
                if Rc::ptr_eq(frame, &known) {
                    break;
                }
                frame.borrow_mut().involved = true;
            }
            return Ok(seed);
        }
        // Under keyword lexing, the call is recorded with where it began,
        // whether it builds a node and the call it was made from: the parse
        // states of a keyword matched in it (see `KeywordLexing`).
        let call = self.keywords.map(|keywords| {
            let parent = self.call_stack.last().and_then(|frame| frame.borrow().call);
            keywords.borrow_mut().call(
                position,
                matches!(rule.kind, RuleKind::Normal),
                rule.node_kind.clone(),
                parent,
            )
        });
        let entry = Rc::new(RefCell::new(Entry {
            evaluating: true,
            call,
            position,
            ..Entry::default()
        }));
        self.retain(1)?;
        self.memo.insert(key.clone(), entry.clone());
        self.call_stack.push(entry.clone());
        self.depth += 1;
        let outcome = if self.depth > self.max_depth {
            Err(Abort::NestingTooDeep)
        } else {
            self.rule_body(rule, position, state, in_token)
                .and_then(|results| {
                    if entry.borrow().left_recursive {
                        self.grow(&entry, rule, position, state, in_token, results)
                    } else {
                        Ok(results)
                    }
                })
        };
        self.depth -= 1;
        self.call_stack.pop();
        let results = {
            let mut entry = entry.borrow_mut();
            entry.evaluating = false;
            let results = outcome?;
            self.retain(results.len())?;
            entry.results.clone_from(&results);
            if entry.involved || self.memo.len() > self.shared.memo_limit {
                drop(entry);
                self.memo.remove(&key);
            }
            results
        };
        Ok(results)
    }

    fn grow(
        &mut self,
        entry: &Rc<RefCell<Entry>>,
        rule: &Rule,
        position: usize,
        state: &State,
        in_token: bool,
        first: Vec<Res>,
    ) -> Run<Vec<Res>> {
        if self.peg {
            let mut best = first.into_iter().next();
            while let Some(current) = &best {
                entry.borrow_mut().seed = vec![current.clone()];
                match self
                    .rule_body(rule, position, state, in_token)?
                    .into_iter()
                    .next()
                {
                    Some(next) if next.end > current.end => best = Some(next),
                    _ => break,
                }
            }
            return Ok(best.into_iter().collect());
        }
        // The seed grows while a pass reaches a new end or settles an end on
        // another tree: a left operand ranked anew (Rust's `impl A + B + C`,
        // where `bounded_type` over `impl A + B` outranks `impl` over `A + B`)
        // changes the trees grown from it, so the next pass grows them again;
        // at most one pass per end changes a tree, for an order that is not
        // transitive. A pass grows only the results the pass before added or
        // changed: the trees grown from the others are already merged, so a
        // chain of n operators takes n passes of one seed each, not n of n.
        // A first pass without results was made with the empty seed a pass
        // would grow from: there is nothing to grow.
        if first.is_empty() {
            return Ok(first);
        }
        let mut current = ResultSet::new(self.longest_tokens, self.settling);
        for result in first {
            current.set(result);
        }
        let mut seed = current.items.clone();
        let mut settled = 0;
        loop {
            entry.borrow_mut().seed = seed;
            let mut merged = current.clone();
            let mut changed = false;
            let mut renewed: Vec<(usize, State)> = Vec::new();
            for result in self.rule_body(rule, position, state, in_token)? {
                let key = result.key();
                let existing = merged
                    .get(&result)
                    .map(|existing| (existing.children.clone(), existing.ambiguous));
                match existing {
                    Some((children, _)) if same_children(&children, &result.children) => {
                        merged.set(result);
                    }
                    Some((_, ambiguous)) => {
                        if merged.add(result) {
                            changed = true;
                            renewed.push(key);
                        } else if let Some(tied) = merged
                            .get_key(&key)
                            .filter(|kept| kept.ambiguous && !ambiguous)
                        {
                            // A tie with a tree of an earlier pass is the
                            // ambiguity the rule body marks when both trees
                            // meet in one pass.
                            let marked = self.ambiguous_result(rule, tied.clone(), in_token);
                            merged.set(marked);
                            renewed.push(key);
                        }
                    }
                    None => {
                        merged.add(result);
                        renewed.push(key);
                    }
                }
            }
            let grew = merged.len() > current.len();
            current = merged;
            if !grew && !changed {
                break;
            }
            if !grew {
                settled += 1;
                if settled > current.len() {
                    break;
                }
            }
            let mut listed = HashSet::new();
            seed = renewed
                .into_iter()
                .filter(|key| listed.insert(key.clone()))
                .filter_map(|key| current.get_key(&key).cloned())
                .collect();
        }
        Ok(current.items)
    }

    /// `result` of `rule`, which a tie marked ambiguous, marked where
    /// `rule_body` marks it: on the node the rule builds, else on the result,
    /// unless a conflict declares the silent rule's ambiguity.
    fn ambiguous_result(&self, rule: &Rule, result: Res, in_token: bool) -> Res {
        if matches!(rule.kind, RuleKind::Silent) || in_token {
            let expected = matches!(rule.kind, RuleKind::Silent)
                && self.program.conflicts.contains(&*rule.node_kind);
            return Res {
                ambiguous: !expected,
                ..result
            };
        }
        let children = result
            .children
            .iter()
            .map(|child| {
                if child.ty == TreeType::Node {
                    let mut node = (**child).clone();
                    node.ambiguous = true;
                    Rc::new(node)
                } else {
                    Rc::clone(child)
                }
            })
            .collect();
        Res {
            children: children_of(children),
            ambiguous: false,
            ..result
        }
    }

    fn rule_body(
        &mut self,
        rule: &Rule,
        position: usize,
        state: &State,
        in_token: bool,
    ) -> Run<Vec<Res>> {
        let mut built = Vec::new();
        if matches!(rule.kind, RuleKind::Token | RuleKind::Atomic) {
            let skipped = self.terminal_start(position, state, in_token)?;
            let start = skipped.end;
            let mut results =
                self.quietly(|this| this.evaluate(&rule.expression, start, state, true))?;
            if matches!(rule.kind, RuleKind::Token) || self.peg {
                results = longest_result(results).into_iter().collect();
            }
            for result in results {
                let mut leaf = Tree::new(
                    TreeType::Token,
                    Some(rule.node_kind.clone()),
                    start,
                    result.end,
                );
                if let (Some(keywords), Some(tokens), false) =
                    (self.keywords, self.longest_tokens, in_token)
                {
                    if keywords.borrow().outranks(&leaf, tokens) {
                        continue;
                    }
                    leaf.lexed = Some(rule.node_kind.clone());
                }
                let Some(acted) = self.run_action(rule, result, &mut leaf, start)? else {
                    continue;
                };
                built.push(Res {
                    children: if in_token {
                        no_children()
                    } else {
                        with_leaf(&skipped.leaves, leaf)
                    },
                    precedence: None,
                    ambiguous: false,
                    ..acted
                });
            }
            if !built.is_empty() {
                return Ok(built);
            }
            self.fail(start, &rule.node_kind);
            if in_token {
                return Ok(built);
            }
            return self.element_failed(
                start,
                &skipped.leaves,
                state,
                Some(rule.node_kind.clone()),
                false,
                Element::of(rule),
                Some(&mut |this, cursor| this.rule_body(rule, cursor, state, false)),
            );
        }
        // A silent rule builds no node to carry an ambiguity, so one inside a
        // silent rule a conflict declares is expected there, as on a node.
        let expected = matches!(rule.kind, RuleKind::Silent)
            && self.program.conflicts.contains(&*rule.node_kind);
        for result in self.evaluate(&rule.expression, position, state, in_token)? {
            if matches!(rule.kind, RuleKind::Silent) || in_token {
                let mut scratch = Tree::new(
                    TreeType::Node,
                    Some(rule.node_kind.clone()),
                    position,
                    result.end,
                );
                scratch.children = result.children.clone();
                let Some(mut acted) = self.run_action(rule, result, &mut scratch, position)? else {
                    continue;
                };
                // An item a silent rule reduces alone records it, for the
                // conflict with a shift where the item is not reduced (see
                // `preferred_tokens` and `child_parting`).
                if !in_token {
                    let ranked = self.program.ranked_silent.contains(&*rule.node_kind);
                    let forked = self.program.conflicts.contains(&*rule.node_kind);
                    acted = reduced_alone(acted, &rule.node_kind, ranked, forked);
                }
                let Some(tail) = acted.tail.clone() else {
                    built.push(Res {
                        ambiguous: acted.ambiguous && !expected,
                        ..acted
                    });
                    continue;
                };
                // The rule is one part of the rule that refers to it, which
                // reduces with the precedence around that part, not inside it;
                // a token or a node the rule ends with keeps the precedence
                // the rule reduces with, for the conflict with a shift after
                // it (see `lone_reduction`).
                let last = acted
                    .children
                    .iter()
                    .enumerate()
                    .rfind(|(_, child)| !child.trivia)
                    .filter(|(_, child)| match child.ty {
                        TreeType::Token => !PrecedenceTag::same(
                            child.reduced.as_ref().or(child.precedence.as_ref()),
                            Some(&tail),
                        ),
                        TreeType::Node => !PrecedenceTag::same(child.closes.as_ref(), Some(&tail)),
                        _ => false,
                    })
                    .map(|(at, child)| (at, Rc::clone(child)));
                let children = match last {
                    Some((at, child)) => {
                        let mut tagged = (*child).clone();
                        if tagged.ty == TreeType::Node {
                            tagged.closes = Some(tail);
                        } else {
                            tagged.reduced = Some(tail);
                        }
                        let mut children = acted.children.to_vec();
                        children[at] = Rc::new(tagged);
                        children_of(children)
                    }
                    None => acted.children.clone(),
                };
                built.push(Res {
                    children,
                    ambiguous: acted.ambiguous && !expected,
                    tail: None,
                    ..acted
                });
                continue;
            }
            let mut node = Tree::node(
                &rule.node_kind,
                position,
                result.end,
                result.children.clone(),
            );
            node.precedence.clone_from(&result.precedence);
            node.tail.clone_from(&result.tail);
            node.ambiguous = result.ambiguous;
            node.before.clone_from(&result.before);
            if let Some(acted) = self.run_action(rule, result, &mut node, position)? {
                built.push(Res {
                    children: children_of(vec![Rc::new(node)]),
                    tail: None,
                    ambiguous: false,
                    ..acted
                });
            }
        }
        Ok(built)
    }

    // Runs the rule's action over one result: the node is fresh, so attributes
    // and a new kind are set in place; `fail` or a failed operation drops the result.
    fn run_action(
        &mut self,
        rule: &Rule,
        result: Res,
        node: &mut Tree,
        from: usize,
    ) -> Run<Option<Res>> {
        let Some(action) = &rule.action else {
            return Ok(Some(result));
        };
        let start = if node.ty == TreeType::Token {
            node.start
        } else {
            content_start(&node.children, from)
        };
        let mut machine = ActionMachine {
            children: node.children.clone(),
            executor: self,
            node,
            working: result.state.working(),
            start,
            end: result.end,
        };
        match run_statements(action, &mut machine) {
            Ok(_) => Ok(Some(Res {
                state: machine.working.settle(),
                ..result
            })),
            Err(OpError::Failed) => Ok(None),
            Err(OpError::Abort(abort)) => Err(abort),
        }
    }

    // An external scanner run for one requested token: a cursor over the
    // input, a token start that `skip` moves, and an optional end `mark`.
    fn scanner_token(
        &mut self,
        name: &Name,
        position: usize,
        state: &State,
        in_token: bool,
    ) -> Run<Vec<Res>> {
        let skipped = self.terminal_start(position, state, in_token)?;
        let start = skipped.end;
        let program = self.program;
        let consults = program.scanners[program.external[&**name]].consults;
        // Where the scanner answers `expected` for (see `Expectations`).
        let context = if in_token {
            self.scan_context.unwrap_or(start)
        } else {
            position
        };
        let key = (
            name.clone(),
            start,
            consults.then_some(context),
            state.clone(),
        );
        let scanned = if let Some(scanned) = self.scanner_memo.get(&key) {
            scanned.clone()
        } else {
            let scanned = self.run_scanner(name, start, context, state)?.map(Rc::new);
            self.scanner_memo.insert(key, scanned.clone());
            scanned
        };
        let Some(scanned) = scanned else {
            self.fail(start, name);
            if in_token {
                return Ok(Vec::new());
            }
            return self.element_failed(
                start,
                &skipped.leaves,
                state,
                Some(name.clone()),
                false,
                Element::Scanner(name.clone()),
                None,
            );
        };
        let children = if in_token {
            no_children()
        } else {
            let mut token = Tree::new(
                TreeType::Token,
                Some(name.clone()),
                scanned.token_start,
                scanned.end,
            );
            token.scanned = true;
            with_leaf(&concat(&skipped.leaves, &scanned.skipped), token)
        };
        Ok(vec![Res::new(
            scanned.end,
            scanned.state.clone(),
            children,
            0,
        )])
    }

    fn run_scanner(
        &mut self,
        name: &Name,
        start: usize,
        context: usize,
        state: &State,
    ) -> Run<Option<Scanned>> {
        let program = self.program;
        let scanner = &program.scanners[program.external[&**name]];
        let mut machine = ScannerMachine {
            executor: self,
            requested: name.clone(),
            context,
            state,
            working: state.working(),
            cursor: start,
            token_start: start,
            mark: None,
            skipped: Vec::new(),
        };
        let signal = match run_statements(&scanner.operations, &mut machine) {
            Ok(signal) => signal,
            Err(OpError::Failed) => return Ok(None),
            Err(OpError::Abort(abort)) => return Err(abort),
        };
        if signal.as_deref() != Some(&**name) {
            return Ok(None);
        }
        let end = machine.mark.unwrap_or(machine.cursor);
        if end < machine.token_start {
            return Ok(None);
        }
        Ok(Some(Scanned {
            token_start: machine.token_start,
            end,
            skipped: machine.skipped,
            state: machine.working.settle(),
        }))
    }

    /// Parses the whole range from `start_rule`: the tree, or the farthest
    /// failure with its sorted expectations. Resource limits abort. While
    /// repairing, a failure carries the root of the result that reaches
    /// farthest, with the rest of the input as an ERROR leaf, or of the
    /// cheapest complete result when the round asks for one more repair point.
    pub(super) fn run(&mut self, start_rule: usize) -> Run<Outcome> {
        let results = self.reference(
            &Target::Rule(start_rule),
            self.begin,
            &State::initial(),
            false,
        )?;
        // A complete result, with the offset where the rest of the input it
        // takes as ERROR starts.
        let mut complete: Vec<(Res, Children, Option<usize>)> = Vec::new();
        // The result that reaches farthest, with its cost before the rest.
        let mut partial: Option<(usize, usize, Res, Children)> = None;
        for result in results {
            let trailing = self.skip_trivia(result.end, &result.state)?;
            if trailing.end == self.end {
                complete.push((result, trailing.leaves.clone(), None));
                continue;
            }
            self.fail(trailing.end, &Name::from("end of input"));
            if self.repair_points.is_none() {
                continue;
            }
            let rest = concat(
                &trailing.leaves,
                &self.rest_leaves(trailing.end, &result.state)?,
            );
            let before = result.cost;
            let cost = before + self.end - trailing.end;
            let repaired = Res { cost, ..result };
            if self
                .repair_points
                .as_ref()
                .is_some_and(|points| points.contains(&trailing.end))
            {
                complete.push((repaired, rest, Some(trailing.end)));
                continue;
            }
            if self
                .element_farthest
                .is_none_or(|farthest| trailing.end > farthest)
            {
                self.element_farthest = Some(trailing.end);
            }
            if partial.as_ref().is_none_or(|(end, _, best, _)| {
                trailing.end > *end || (trailing.end == *end && cost < best.cost)
            }) {
                partial = Some((trailing.end, before, repaired, rest));
            }
        }
        if complete.is_empty() {
            let mut expected: Vec<String> = self.expected.iter().map(ToString::to_string).collect();
            expected.sort();
            let partial = self.repair_points.is_some().then(|| {
                Rc::new(partial.map_or_else(
                    || self.error_root(start_rule),
                    |(_, _, result, trailing)| self.root(start_rule, &result, &trailing, false),
                ))
            });
            return Ok(Outcome::Failed {
                farthest: self.farthest,
                expected,
                element_farthest: self.element_farthest,
                partial,
            });
        }
        // The complete results end apart, before their trailing trivia, so they
        // are ranked here as ResultSet::add ranks results with one end: the
        // lower cost, then the grammar's settling steps. A tie is an
        // ambiguity when the settling ends in `ambiguity`; repaired results
        // of equal cost are not ambiguities.
        let mut chosen = &complete[0];
        let mut tied = false;
        for candidate in &complete[1..] {
            match complete_order(
                (&candidate.0, &candidate.1),
                (&chosen.0, &chosen.1),
                self.settling,
                self.longest_tokens,
            ) {
                Ordering::Greater => {
                    chosen = candidate;
                    tied = false;
                }
                Ordering::Equal => tied = true,
                Ordering::Less => {}
            }
        }
        let several = tied && self.settling.ambiguity() && chosen.0.cost == 0;
        let root = Rc::new(self.root(start_rule, &chosen.0, &chosen.1, several));
        // When the cheapest complete result takes the rest of the input as
        // ERROR at a repair point, a result that reached past that point
        // without completing costs at least one more repair, at its end. While
        // that could still complete for less, the round asks for its end as
        // the next repair point, so a later error is repaired where it is and
        // not by skipping all the input after an earlier one; the complete
        // result stands when the rounds end.
        if let (Some(rest), Some((end, before, _, _))) = (chosen.2, &partial)
            && *end > rest
            && before + 1 < chosen.0.cost
        {
            let mut expected: Vec<String> = self.expected.iter().map(ToString::to_string).collect();
            expected.sort();
            return Ok(Outcome::Failed {
                farthest: self.farthest,
                expected,
                element_farthest: Some(*end),
                partial: Some(root),
            });
        }
        Ok(Outcome::Parsed(root))
    }

    /// The rest of the input from `start` as an ERROR leaf. As tree-sitter
    /// keeps the white space at the end of the input out of an ERROR node,
    /// the separators that end the input follow the leaf; the rest costs the
    /// same.
    fn rest_leaves(&mut self, start: usize, state: &State) -> Run<Vec<Rc<Tree>>> {
        let mut at = self.end;
        while at > start
            && matches!(
                self.bytes[at - 1],
                b'\t' | b'\n' | 0x0b | 0x0c | b'\r' | b' '
            )
        {
            at -= 1;
        }
        if at > start && at < self.end {
            let tail = self.skip_trivia(at, state)?;
            if tail.end == self.end && tail.leaves.iter().all(|leaf| is_separator(leaf)) {
                let mut leaves = vec![Rc::new(Tree::new(TreeType::Error, None, start, at))];
                leaves.extend(tail.leaves.iter().cloned());
                return Ok(leaves);
            }
        }
        Ok(vec![Rc::new(Tree::new(
            TreeType::Error,
            None,
            start,
            self.end,
        ))])
    }

    fn root(&self, start_rule: usize, result: &Res, trailing: &[Rc<Tree>], several: bool) -> Tree {
        let ambiguous = several || result.ambiguous;
        match &result.children[..] {
            [only] if only.ty == TreeType::Node => {
                let mut root = (**only).clone();
                root.end = self.end;
                root.children = concat(&only.children, trailing);
                root.ambiguous = only.ambiguous || ambiguous;
                root
            }
            children => {
                let kind = &self.program.rules[start_rule].node_kind;
                let mut root = Tree::node(kind, self.begin, self.end, concat(children, trailing));
                root.ambiguous = ambiguous;
                root
            }
        }
    }

    // The root when the start rule matches nothing even with repairs.
    fn error_root(&self, start_rule: usize) -> Tree {
        let kind = &self.program.rules[start_rule].node_kind;
        let error = Tree::new(TreeType::Error, None, self.begin, self.end);
        Tree::node(
            kind,
            self.begin,
            self.end,
            children_of(vec![Rc::new(error)]),
        )
    }
}

/// The machine a semantic action runs on: the result's text and children,
/// and the fresh node it may annotate or rename.
struct ActionMachine<'e, 'c, 'n> {
    executor: &'e mut Executor<'c>,
    node: &'n mut Tree,
    children: Children,
    working: Working,
    start: usize,
    end: usize,
}

impl ActionMachine<'_, '_, '_> {
    // A field name selects the direct children captured under it, or else
    // the direct children of that kind.
    fn named(&self, field: &str) -> Vec<Rc<Tree>> {
        let meaningful = self.children.iter().filter(|child| !child.trivia);
        let captured: Vec<Rc<Tree>> = meaningful
            .clone()
            .filter(|child| child.field.as_deref() == Some(field))
            .cloned()
            .collect();
        if captured.is_empty() {
            meaningful
                .filter(|child| child.kind.as_deref() == Some(field))
                .cloned()
                .collect()
        } else {
            captured
        }
    }
}

fn attribute_of(child: &Tree, name: &str) -> OpResult<OperationValue> {
    child
        .attributes
        .as_ref()
        .and_then(|attributes| attributes.get(name))
        .cloned()
        .ok_or(OpError::Failed)
}

impl Machine for ActionMachine<'_, '_, '_> {
    fn state(&mut self) -> &mut Working {
        &mut self.working
    }

    fn step(&mut self) -> Result<(), Abort> {
        self.executor.step()
    }

    fn column(&mut self) -> usize {
        column_of(self.executor.bytes, self.start, self.executor.begin)
    }

    fn at_end(&mut self) -> bool {
        self.end == self.executor.end
    }

    fn matched(&mut self) -> OpResult<String> {
        Ok(self.executor.text(self.start, self.end))
    }

    fn attribute(&mut self, field: &str, name: &str) -> OpResult<OperationValue> {
        let named = self.named(field);
        attribute_of(named.first().ok_or(OpError::Failed)?, name)
    }

    fn attributes(&mut self, field: &str, name: &str) -> OpResult<Vec<OperationValue>> {
        self.named(field)
            .iter()
            .map(|child| attribute_of(child, name))
            .collect()
    }

    fn field_text(&mut self, field: &str) -> OpResult<String> {
        let named = self.named(field);
        let child = named.first().ok_or(OpError::Failed)?;
        let start = if child.ty == TreeType::Node {
            content_start(&child.children, child.start)
        } else {
            child.start
        };
        Ok(self.executor.text(start, child.end))
    }

    fn set_attribute(&mut self, name: &str, value: OperationValue) {
        self.node
            .attributes
            .get_or_insert_with(Default::default)
            .insert(name.to_owned(), value);
    }

    fn build_node(&mut self, kind: &Arc<str>) {
        self.node.kind = Some(kind.clone());
    }
}

/// The machine an external scanner runs on.
struct ScannerMachine<'e, 'c, 's> {
    executor: &'e mut Executor<'c>,
    requested: Name,
    /// The offset the scanner answers `expected` for.
    context: usize,
    state: &'s State,
    working: Working,
    cursor: usize,
    token_start: usize,
    mark: Option<usize>,
    skipped: Vec<Rc<Tree>>,
}

impl ScannerMachine<'_, '_, '_> {
    // The end of the longest match of `item` at the cursor, in the state the
    // scanner started in.
    fn match_at(&mut self, item: &Expr) -> OpResult<Option<usize>> {
        let cursor = self.cursor;
        let state = self.state;
        let results = self
            .executor
            .quietly(|this| this.evaluate(item, cursor, state, true))?;
        Ok(longest_result(results).map(|result| result.end))
    }
}

impl Machine for ScannerMachine<'_, '_, '_> {
    fn state(&mut self) -> &mut Working {
        &mut self.working
    }

    fn step(&mut self) -> Result<(), Abort> {
        self.executor.step()
    }

    fn requested(&self) -> Option<&str> {
        Some(&self.requested)
    }

    fn expected(&mut self, id: usize) -> bool {
        let item = (self.executor.program_index, id);
        self.executor
            .shared
            .expectations
            .borrow_mut()
            .holds(self.context, item)
    }

    fn column(&mut self) -> usize {
        column_of(self.executor.bytes, self.cursor, self.executor.begin)
    }

    fn at_end(&mut self) -> bool {
        self.cursor >= self.executor.end
    }

    fn lookahead(&mut self, item: &Expr) -> OpResult<bool> {
        Ok(self.match_at(item)?.is_some())
    }

    fn advance(&mut self) -> OpResult<()> {
        if self.cursor >= self.executor.end {
            return Err(OpError::Failed);
        }
        self.cursor += decode_at(self.executor.bytes, self.cursor, self.executor.end).1;
        Ok(())
    }

    fn consume(&mut self, item: &Expr) -> OpResult<()> {
        self.cursor = self.match_at(item)?.ok_or(OpError::Failed)?;
        Ok(())
    }

    fn skip(&mut self, item: &Expr) -> OpResult<()> {
        let end = self.match_at(item)?;
        let Some(end) = end.filter(|_| self.cursor == self.token_start) else {
            return Err(OpError::Failed);
        };
        if end > self.cursor {
            let mut leaf = Tree::new(TreeType::Token, None, self.cursor, end);
            leaf.trivia = true;
            self.skipped.push(Rc::new(leaf));
        }
        self.cursor = end;
        self.token_start = end;
        Ok(())
    }

    fn mark(&mut self) {
        self.mark = Some(self.cursor);
    }
}
