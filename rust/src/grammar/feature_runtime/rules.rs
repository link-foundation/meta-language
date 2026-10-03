//! Rule calls, semantic actions, external scanners and the whole-input run of
//! the native executor, as the second half of
//! `js/src/grammar-runtime/executor.js`.

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

use super::executor::{Executor, Run};
use super::operations::{
    Abort, Machine, OpError, OpResult, OperationValue, State, Working, run_statements,
};
use super::program::{Expr, Name, Rule, Target};
use super::results::{
    Children, Entry, Outcome, Res, ResultSet, Scanned, Tree, TreeType, children_of, concat,
    content_start, longest_result, no_children, with_leaf,
};
use super::text::{column_of, decode_at};
use crate::grammar::RuleKind;

impl Executor<'_> {
    /// A rule call or an external token, memoized with left-recursion growth.
    pub(super) fn reference(
        &mut self,
        target: &Target,
        position: usize,
        state: &State,
        in_token: bool,
    ) -> Run<Vec<Res>> {
        let index = match target {
            Target::External(name) => return self.scanner_token(name, position, state, in_token),
            Target::Rule(index) => *index,
        };
        let program = self.program;
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
        let entry = Rc::new(RefCell::new(Entry {
            evaluating: true,
            ..Entry::default()
        }));
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
        let mut current = ResultSet::new(self.longest_tokens);
        for result in first {
            current.set(result);
        }
        loop {
            entry.borrow_mut().seed.clone_from(&current.items);
            let mut merged = current.clone();
            for result in self.rule_body(rule, position, state, in_token)? {
                merged.set(result);
            }
            let grew = merged.len() > current.len();
            current = merged;
            if !grew {
                break;
            }
        }
        Ok(current.items)
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
                |this, cursor| this.rule_body(rule, cursor, state, false),
            );
        }
        for result in self.evaluate(&rule.expression, position, state, in_token)? {
            if matches!(rule.kind, RuleKind::Silent) || in_token {
                let mut scratch = Tree::new(
                    TreeType::Node,
                    Some(rule.node_kind.clone()),
                    position,
                    result.end,
                );
                scratch.children = result.children.clone();
                if let Some(acted) = self.run_action(rule, result, &mut scratch, position)? {
                    built.push(acted);
                }
                continue;
            }
            let mut node = Tree::node(
                &rule.node_kind,
                position,
                result.end,
                result.children.clone(),
            );
            node.precedence = result.precedence;
            node.ambiguous = result.ambiguous;
            if let Some(acted) = self.run_action(rule, result, &mut node, position)? {
                built.push(Res {
                    children: children_of(vec![Rc::new(node)]),
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
        let key = (name.clone(), start, state.clone());
        let scanned = if let Some(scanned) = self.scanner_memo.get(&key) {
            scanned.clone()
        } else {
            let scanned = self.run_scanner(name, start, state)?.map(Rc::new);
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
                |this, cursor| this.scanner_token(name, cursor, state, false),
            );
        };
        let children = if in_token {
            no_children()
        } else {
            let token = Tree::new(
                TreeType::Token,
                Some(name.clone()),
                scanned.token_start,
                scanned.end,
            );
            with_leaf(&concat(&skipped.leaves, &scanned.skipped), token)
        };
        Ok(vec![Res::new(
            scanned.end,
            scanned.state.clone(),
            children,
            0,
        )])
    }

    fn run_scanner(&mut self, name: &Name, start: usize, state: &State) -> Run<Option<Scanned>> {
        let program = self.program;
        let scanner = &program.scanners[program.external[&**name]];
        let mut machine = ScannerMachine {
            executor: self,
            requested: name.clone(),
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
    /// farthest, with the rest of the input as an ERROR leaf.
    pub(super) fn run(&mut self, start_rule: usize) -> Run<Outcome> {
        let results = self.reference(
            &Target::Rule(start_rule),
            self.begin,
            &State::initial(),
            false,
        )?;
        let mut complete: Vec<(Res, Children)> = Vec::new();
        let mut partial: Option<(usize, Res, Children)> = None;
        for result in results {
            let trailing = self.skip_trivia(result.end, &result.state)?;
            if trailing.end == self.end {
                complete.push((result, trailing.leaves.clone()));
                continue;
            }
            self.fail(trailing.end, &Name::from("end of input"));
            let Some(points) = &self.repair_points else {
                continue;
            };
            let rest = with_leaf(
                &trailing.leaves,
                Tree::new(TreeType::Error, None, trailing.end, self.end),
            );
            let cost = result.cost + self.end - trailing.end;
            let repaired = Res { cost, ..result };
            if points.contains(&trailing.end) {
                complete.push((repaired, rest));
                continue;
            }
            if self
                .element_farthest
                .is_none_or(|farthest| trailing.end > farthest)
            {
                self.element_farthest = Some(trailing.end);
            }
            if partial.as_ref().is_none_or(|(end, best, _)| {
                trailing.end > *end || (trailing.end == *end && cost < best.cost)
            }) {
                partial = Some((trailing.end, repaired, rest));
            }
        }
        if complete.is_empty() {
            let mut expected: Vec<String> = self.expected.iter().map(ToString::to_string).collect();
            expected.sort();
            let partial = self.repair_points.is_some().then(|| {
                Rc::new(partial.map_or_else(
                    || self.error_root(start_rule),
                    |(_, result, trailing)| self.root(start_rule, &result, &trailing, false),
                ))
            });
            return Ok(Outcome::Failed {
                farthest: self.farthest,
                expected,
                element_farthest: self.element_farthest,
                partial,
            });
        }
        let several = complete.len() > 1;
        let mut chosen = &complete[0];
        for candidate in &complete {
            if candidate.0.cost < chosen.0.cost {
                chosen = candidate;
            }
        }
        // Repaired results of equal cost are not ambiguities.
        let several = several && chosen.0.cost == 0;
        let root = self.root(start_rule, &chosen.0, &chosen.1, several);
        Ok(Outcome::Parsed(Rc::new(root)))
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
