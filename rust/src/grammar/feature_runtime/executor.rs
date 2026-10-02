//! The native grammar executor, as `js/src/grammar-runtime/executor.js`: it
//! interprets a loaded program over a byte string and builds the lossless
//! concrete syntax tree. Generalized matching keeps every result an
//! expression can produce, deduplicated by end offset and parser state; PEG
//! matching keeps at most one. Rule calls are memoized, left recursion grows a
//! seed to a fixpoint, and the nesting depth, the step count and the memo size
//! are bounded, so a hostile input ends in a rejection instead of a stack
//! overflow or a runaway parse. `rules.rs` holds rule calls, actions and
//! external scanners.

use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

use super::operations::{Abort, Machine, OpError, OpResult, State, Working, evaluate_condition};
use super::program::{Associativity, Compiled, Expr, Matcher, Name, Program, Target};
use super::results::{
    Entry, MemoKey, Outcome, Res, ResultSet, Scanned, Shared, Skipped, Tree, TreeType,
    content_start, longest_result, no_children, with_leaf,
};
use super::text::{column_of, decode_at, text_of};

/// The outcome of a step that a resource limit may end.
pub(super) type Run<T> = Result<T, Abort>;

/// The evaluation frames a parse may nest before it is refused as too deep;
/// the parse thread's stack holds this many with room to spare.
const FRAME_LIMIT: usize = 20_000;

/// Interprets one program over `bytes[begin, end)`.
pub(super) struct Executor<'c> {
    pub(super) compiled: &'c Compiled,
    pub(super) program: &'c Program,
    pub(super) bytes: &'c [u8],
    pub(super) begin: usize,
    pub(super) end: usize,
    pub(super) shared: &'c Shared,
    pub(super) max_depth: usize,
    pub(super) peg: bool,
    pub(super) depth: usize,
    pub(super) memo: HashMap<MemoKey, Rc<RefCell<Entry>>>,
    pub(super) call_stack: Vec<Rc<RefCell<Entry>>>,
    trivia_memo: HashMap<(usize, State), Rc<Skipped>>,
    pub(super) scanner_memo: HashMap<(Name, usize, State), Option<Rc<Scanned>>>,
    embed_memo: HashMap<(Name, usize, usize), Rc<Outcome>>,
    pub(super) farthest: usize,
    pub(super) expected: HashSet<Name>,
    suppressed: usize,
}

impl<'c> Executor<'c> {
    #[allow(clippy::too_many_arguments)]
    pub(super) fn new(
        compiled: &'c Compiled,
        program_index: usize,
        bytes: &'c [u8],
        begin: usize,
        end: usize,
        shared: &'c Shared,
        max_depth: usize,
    ) -> Self {
        let program = &compiled.programs[program_index];
        Self {
            compiled,
            program,
            bytes,
            begin,
            end,
            shared,
            max_depth,
            peg: program.peg,
            depth: 0,
            memo: HashMap::new(),
            call_stack: Vec::new(),
            trivia_memo: HashMap::new(),
            scanner_memo: HashMap::new(),
            embed_memo: HashMap::new(),
            farthest: begin,
            expected: HashSet::new(),
            suppressed: 0,
        }
    }

    pub(super) fn step(&self) -> Run<()> {
        let steps = self.shared.steps.get() + 1;
        self.shared.steps.set(steps);
        if steps > self.shared.limit {
            Err(Abort::StepLimit)
        } else {
            Ok(())
        }
    }

    /// Records an expectation at `position`; only the farthest position is kept.
    pub(super) fn fail(&mut self, position: usize, expectation: &Name) {
        if self.suppressed > 0 {
            return;
        }
        if position > self.farthest {
            self.farthest = position;
            self.expected.clear();
            self.expected.insert(expectation.clone());
        } else if position == self.farthest {
            self.expected.insert(expectation.clone());
        }
    }

    pub(super) fn quietly<T>(&mut self, run: impl FnOnce(&mut Self) -> T) -> T {
        self.suppressed += 1;
        let result = run(self);
        self.suppressed -= 1;
        result
    }

    pub(super) fn text(&self, start: usize, end: usize) -> String {
        text_of(self.bytes, start, end).unwrap_or_default()
    }

    /// Skips trivia: repeatedly the longest match of any trivia expression
    /// allowed in the current mode.
    pub(super) fn skip_trivia(&mut self, position: usize, state: &State) -> Run<Rc<Skipped>> {
        let program = self.program;
        let trivia = &program.trivia;
        if trivia.is_empty() {
            return Ok(Rc::new(Skipped {
                end: position,
                leaves: no_children(),
            }));
        }
        let key = (position, state.clone());
        if let Some(cached) = self.trivia_memo.get(&key) {
            return Ok(cached.clone());
        }
        let mode = state.mode().to_owned();
        let mut leaves = Vec::new();
        let mut cursor = position;
        loop {
            let mut best = cursor;
            let mut best_kind = None;
            for item in trivia {
                if item
                    .modes
                    .as_ref()
                    .is_some_and(|modes| !modes.iter().any(|allowed| **allowed == *mode))
                {
                    continue;
                }
                let end =
                    self.quietly(|this| this.evaluate(&item.expression, cursor, state, true))?;
                if let Some(end) = longest_result(end).map(|result| result.end)
                    && end > best
                {
                    best = end;
                    best_kind.clone_from(&item.kind);
                }
            }
            if best == cursor {
                break;
            }
            leaves.push(Rc::new(Tree::trivia_leaf(best_kind, cursor, best)));
            cursor = best;
        }
        let skipped = Rc::new(Skipped {
            end: cursor,
            leaves: Rc::new(leaves),
        });
        self.trivia_memo.insert(key, skipped.clone());
        Ok(skipped)
    }

    /// The start of a terminal: after trivia in syntactic context, at once in token context.
    pub(super) fn terminal_start(
        &mut self,
        position: usize,
        state: &State,
        in_token: bool,
    ) -> Run<Rc<Skipped>> {
        if in_token {
            Ok(Rc::new(Skipped {
                end: position,
                leaves: no_children(),
            }))
        } else {
            self.skip_trivia(position, state)
        }
    }

    fn terminal(
        &mut self,
        matcher: &Matcher,
        expectation: &Name,
        position: usize,
        state: &State,
        in_token: bool,
    ) -> Run<Vec<Res>> {
        let skipped = self.terminal_start(position, state, in_token)?;
        let start = skipped.end;
        let Some(end) = matcher.matches(self.bytes, start, self.end) else {
            self.fail(start, expectation);
            return Ok(Vec::new());
        };
        let children = if in_token {
            no_children()
        } else {
            with_leaf(
                &skipped.leaves,
                Tree::new(TreeType::Token, None, start, end),
            )
        };
        Ok(vec![Res::new(end, state.clone(), children, 0)])
    }

    // A leaf over the longest match of `item` in token context: token(),
    // immediateToken() and longest() alternatives build on it.
    fn token_leaf(
        &mut self,
        item: &Expr,
        skipped: &Skipped,
        state: &State,
        in_token: bool,
    ) -> Run<Vec<Res>> {
        let start = skipped.end;
        let Some(best) = longest_result(self.evaluate(item, start, state, true)?) else {
            return Ok(Vec::new());
        };
        let children = if in_token {
            no_children()
        } else {
            with_leaf(
                &skipped.leaves,
                Tree::new(TreeType::Token, None, start, best.end),
            )
        };
        Ok(vec![Res::new(best.end, best.state, children, best.dynamic)])
    }

    /// Every result of `expr` at `position` in `state`.
    pub(super) fn evaluate(
        &mut self,
        expr: &Expr,
        position: usize,
        state: &State,
        in_token: bool,
    ) -> Run<Vec<Res>> {
        self.step()?;
        let frames = self.shared.frames.get() + 1;
        if frames > FRAME_LIMIT {
            return Err(Abort::NestingTooDeep);
        }
        self.shared.frames.set(frames);
        let results = self.evaluate_case(expr, position, state, in_token);
        self.shared.frames.set(frames - 1);
        results
    }

    fn evaluate_case(
        &mut self,
        expr: &Expr,
        position: usize,
        state: &State,
        in_token: bool,
    ) -> Run<Vec<Res>> {
        match expr {
            Expr::Empty => Ok(vec![Res::new(position, state.clone(), no_children(), 0)]),
            Expr::Terminal {
                matcher,
                expectation,
            } => self.terminal(matcher, expectation, position, state, in_token),
            Expr::Ref(target) => self.reference(target, position, state, in_token),
            Expr::Seq(items) => self.sequence(items, position, state, in_token, None),
            Expr::Choice { ordered, items } => {
                self.choice(*ordered, items, position, state, in_token)
            }
            Expr::Repeat { item, min, max } => {
                self.repetition(item, *min, *max, position, state, in_token)
            }
            Expr::And(item) | Expr::Not(item) => {
                let matched =
                    self.quietly(|this| this.evaluate(item, position, state, in_token))?;
                let succeeds = matched.is_empty() == matches!(expr, Expr::Not(_));
                Ok(if succeeds {
                    vec![Res::new(position, state.clone(), no_children(), 0)]
                } else {
                    Vec::new()
                })
            }
            Expr::Capture { label, item } => {
                let mut results = self.evaluate(item, position, state, in_token)?;
                if !in_token {
                    for result in &mut results {
                        result.children = Rc::new(
                            result
                                .children
                                .iter()
                                .map(|child| {
                                    if child.trivia {
                                        child.clone()
                                    } else {
                                        let mut copy = (**child).clone();
                                        copy.field.clone_from(label);
                                        Rc::new(copy)
                                    }
                                })
                                .collect(),
                        );
                    }
                }
                Ok(results)
            }
            Expr::Alias { name, item } => self.alias(name, item, position, state, in_token),
            Expr::Precedence {
                level,
                associativity,
                item,
            } => self.precedence(*level, *associativity, item, position, state, in_token),
            Expr::DynamicPrecedence { level, item } => {
                let mut results = self.evaluate(item, position, state, in_token)?;
                for result in &mut results {
                    result.dynamic += level;
                }
                Ok(results)
            }
            Expr::LexicalPrecedence { item, .. } => self.evaluate(item, position, state, in_token),
            Expr::Longest(items) => self.longest(items, position, state, in_token),
            Expr::Token(item) => {
                let skipped = self.terminal_start(position, state, in_token)?;
                self.token_leaf(item, &skipped, state, in_token)
            }
            Expr::ImmediateToken(item) => {
                let skipped = Skipped {
                    end: position,
                    leaves: no_children(),
                };
                self.token_leaf(item, &skipped, state, in_token)
            }
            Expr::Predicate { item, condition } => {
                let results = self.evaluate(item, position, state, in_token)?;
                let mut kept = Vec::new();
                for result in results {
                    let mut machine = ValueMachine::new(self, &result, position);
                    match evaluate_condition(condition, &mut machine) {
                        Ok(true) => kept.push(result),
                        Ok(false) | Err(OpError::Failed) => {}
                        Err(OpError::Abort(abort)) => return Err(abort),
                    }
                }
                if kept.is_empty() {
                    let start = self.terminal_start(position, state, in_token)?.end;
                    self.fail(start, &Name::from("predicate"));
                }
                Ok(kept)
            }
            Expr::Recover { item, synchronize } => {
                self.recover(item, synchronize, position, state, in_token)
            }
            Expr::Missing {
                item,
                kind,
                literal,
            } => {
                let results = self.evaluate(item, position, state, in_token)?;
                if !results.is_empty() {
                    return Ok(results);
                }
                let mut node = Tree::new(TreeType::Missing, kind.clone(), position, position);
                node.literal = *literal;
                let children = if in_token {
                    no_children()
                } else {
                    Rc::new(vec![Rc::new(node)])
                };
                Ok(vec![Res::new(position, state.clone(), children, 0)])
            }
            Expr::Embed { language, item } => self.embed(language, item, position, state, in_token),
        }
    }

    // `keep`, when given, filters the complete sequences before they are
    // deduplicated, so a precedence filter never loses a valid parse to an
    // invalid one that reached the same end first.
    fn sequence(
        &mut self,
        items: &[Expr],
        position: usize,
        state: &State,
        in_token: bool,
        keep: Option<(i64, Associativity)>,
    ) -> Run<Vec<Res>> {
        let mut current = vec![Res::new(position, state.clone(), no_children(), 0)];
        for (index, item) in items.iter().enumerate() {
            let mut next = ResultSet::default();
            let last = index == items.len() - 1;
            for left in &current {
                for right in self.evaluate(item, left.end, &left.state, in_token)? {
                    let joined = Res::join(left, right, in_token);
                    if last
                        && let Some((level, associativity)) = keep
                        && !self.precedence_valid(level, associativity, &joined)
                    {
                        continue;
                    }
                    next.add(joined);
                }
            }
            current = next.items;
            if self.peg {
                current.truncate(1);
            }
            if current.is_empty() {
                return Ok(current);
            }
        }
        Ok(current)
    }

    fn choice(
        &mut self,
        ordered: bool,
        items: &[Expr],
        position: usize,
        state: &State,
        in_token: bool,
    ) -> Run<Vec<Res>> {
        if ordered {
            for item in items {
                let results = self.evaluate(item, position, state, in_token)?;
                if !results.is_empty() {
                    return Ok(results);
                }
            }
            return Ok(Vec::new());
        }
        if self.peg {
            // PEG unordered choice: the longest alternative, the first on a tie.
            let mut best: Option<Res> = None;
            for item in items {
                if let Some(result) = self
                    .evaluate(item, position, state, in_token)?
                    .into_iter()
                    .next()
                    && best.as_ref().is_none_or(|best| result.end > best.end)
                {
                    best = Some(result);
                }
            }
            return Ok(best.into_iter().collect());
        }
        let mut results = ResultSet::default();
        for item in items {
            for result in self.evaluate(item, position, state, in_token)? {
                results.add(result);
            }
        }
        Ok(results.items)
    }

    #[allow(clippy::too_many_arguments)]
    fn repetition(
        &mut self,
        item: &Expr,
        min: usize,
        max: Option<usize>,
        position: usize,
        state: &State,
        in_token: bool,
    ) -> Run<Vec<Res>> {
        let zero_width =
            |left: &Res, right: &Res| right.end == left.end && right.state == left.state;
        let below_max = |count: usize| max.is_none_or(|max| count < max);
        if self.peg {
            // Greedy and possessive: as many iterations as match, never fewer.
            let mut current = Res::new(position, state.clone(), no_children(), 0);
            let mut count = 0;
            while below_max(count) {
                let Some(next) = self
                    .evaluate(item, current.end, &current.state, in_token)?
                    .into_iter()
                    .next()
                else {
                    break;
                };
                if zero_width(&current, &next) {
                    count = count.max(min);
                    current = Res::join(&current, next, in_token);
                    break;
                }
                current = Res::join(&current, next, in_token);
                count += 1;
            }
            return Ok(if count >= min {
                vec![current]
            } else {
                Vec::new()
            });
        }
        // Generalized: a breadth-first frontier by iteration count. Once the
        // minimum is met, a result whose end and state were already reached is
        // not extended again (it is the same continuation) but marks ambiguity.
        let mut results = ResultSet::default();
        let mut frontier = vec![Res::new(position, state.clone(), no_children(), 0)];
        let mut count = 0;
        while !frontier.is_empty() {
            if count >= min {
                let mut fresh = Vec::new();
                for result in frontier {
                    if results.contains(&result) {
                        results.add(result);
                    } else {
                        results.set(result.clone());
                        fresh.push(result);
                    }
                }
                frontier = fresh;
            }
            if !below_max(count) {
                break;
            }
            let mut next = ResultSet::default();
            for left in &frontier {
                for right in self.evaluate(item, left.end, &left.state, in_token)? {
                    if zero_width(left, &right) {
                        // Zero-width iterations can pad up to the minimum once.
                        if count < min {
                            results.add(Res::join(left, right, in_token));
                        }
                        continue;
                    }
                    next.add(Res::join(left, right, in_token));
                }
            }
            frontier = next.items;
            count += 1;
        }
        Ok(results.items)
    }

    fn alias(
        &mut self,
        name: &Name,
        item: &Expr,
        position: usize,
        state: &State,
        in_token: bool,
    ) -> Run<Vec<Res>> {
        let results = self.evaluate(item, position, state, in_token)?;
        if in_token {
            return Ok(results);
        }
        Ok(results
            .into_iter()
            .map(|result| {
                let meaningful = result.children.iter().filter(|child| !child.trivia).count();
                if meaningful == 1 {
                    let children = result
                        .children
                        .iter()
                        .map(|child| {
                            if child.trivia {
                                child.clone()
                            } else {
                                let mut copy = (**child).clone();
                                copy.kind = Some(name.clone());
                                Rc::new(copy)
                            }
                        })
                        .collect();
                    return Res {
                        children: Rc::new(children),
                        ..result
                    };
                }
                let mut node = Tree::node(name, position, result.end, result.children.clone());
                node.ambiguous = result.ambiguous;
                Res {
                    children: Rc::new(vec![Rc::new(node)]),
                    ambiguous: false,
                    ..result
                }
            })
            .collect())
    }

    // Precedence and associativity filter the binary-shaped results: a
    // leftmost or rightmost child node of lower precedence, or of equal
    // precedence on the side the associativity forbids, invalidates a result.
    fn precedence_valid(&mut self, level: i64, associativity: Associativity, result: &Res) -> bool {
        let meaningful: Vec<&Rc<Tree>> = result
            .children
            .iter()
            .filter(|child| !child.trivia)
            .collect();
        let forbids = |child: &Tree, side: Associativity| {
            child.ty == TreeType::Node
                && child.precedence.is_some_and(|(inner, _)| {
                    inner < level || (inner == level && associativity != side)
                })
        };
        let allowed = meaningful.len() < 2
            || !(forbids(meaningful[0], Associativity::Left)
                || forbids(meaningful[meaningful.len() - 1], Associativity::Right));
        if !allowed {
            self.fail(result.end, &Name::from("precedence"));
        }
        allowed
    }

    #[allow(clippy::too_many_arguments)]
    fn precedence(
        &mut self,
        level: i64,
        associativity: Associativity,
        item: &Expr,
        position: usize,
        state: &State,
        in_token: bool,
    ) -> Run<Vec<Res>> {
        let mut results = if in_token {
            self.evaluate(item, position, state, in_token)?
        } else {
            self.filtered(item, position, state, (level, associativity))?
        };
        for result in &mut results {
            result.precedence = Some((level, associativity));
        }
        Ok(results)
    }

    // The results of `expr` that the precedence filter accepts, filtered
    // before a sequence or an unordered choice merges results of the same
    // end and state.
    fn filtered(
        &mut self,
        expr: &Expr,
        position: usize,
        state: &State,
        keep: (i64, Associativity),
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
                let mut results = ResultSet::default();
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
                    .filter(|result| self.precedence_valid(keep.0, keep.1, result))
                    .collect())
            }
        }
    }

    fn priority_of(&self, item: &Expr) -> i64 {
        match item {
            Expr::LexicalPrecedence { level, .. } => *level,
            Expr::Ref(Target::Rule(index)) => self.program.rules[*index].lexical_priority,
            _ => 0,
        }
    }

    // Lexical longest match: the alternative with the longest match wins, a
    // tie goes to the higher lexical priority and then to the first.
    fn longest(
        &mut self,
        items: &[Expr],
        position: usize,
        state: &State,
        in_token: bool,
    ) -> Run<Vec<Res>> {
        let skipped = self.terminal_start(position, state, in_token)?;
        let start = skipped.end;
        let mut best: Option<(Res, i64, &Expr)> = None;
        for item in items {
            let Some(result) = longest_result(self.evaluate(item, start, state, true)?) else {
                continue;
            };
            let priority = self.priority_of(item);
            if best.as_ref().is_none_or(|(best, best_priority, _)| {
                result.end > best.end || (result.end == best.end && priority > *best_priority)
            }) {
                best = Some((result, priority, item));
            }
        }
        let Some((result, _, item)) = best else {
            return Ok(Vec::new());
        };
        let kind = match item {
            Expr::Ref(Target::Rule(index)) => Some(self.program.rules[*index].node_kind.clone()),
            Expr::Ref(Target::External(name)) => Some(name.clone()),
            _ => None,
        };
        let children = if in_token {
            no_children()
        } else {
            with_leaf(
                &skipped.leaves,
                Tree::new(TreeType::Token, kind, start, result.end),
            )
        };
        Ok(vec![Res::new(
            result.end,
            result.state,
            children,
            result.dynamic,
        )])
    }

    // Error recovery: when the item fails, skip at least one unit up to the
    // first offset where the synchronization expression matches and record
    // the skipped bytes as an ERROR node.
    fn recover(
        &mut self,
        item: &Expr,
        synchronize: &Expr,
        position: usize,
        state: &State,
        in_token: bool,
    ) -> Run<Vec<Res>> {
        let results = self.evaluate(item, position, state, in_token)?;
        if !results.is_empty() {
            return Ok(results);
        }
        let skipped = self.terminal_start(position, state, in_token)?;
        let start = skipped.end;
        let mut cursor = start;
        while cursor < self.end {
            cursor += decode_at(self.bytes, cursor, self.end).1;
            let synchronized =
                self.quietly(|this| this.evaluate(synchronize, cursor, state, in_token))?;
            if !synchronized.is_empty() {
                let children = if in_token {
                    no_children()
                } else {
                    with_leaf(
                        &skipped.leaves,
                        Tree::new(TreeType::Error, None, start, cursor),
                    )
                };
                return Ok(vec![Res::new(cursor, state.clone(), children, 0)]);
            }
        }
        Ok(Vec::new())
    }

    // An embedded language parses the region the item matches, on the same
    // bytes, so its offsets stay absolute.
    fn embed(
        &mut self,
        language: &Name,
        item: &Expr,
        position: usize,
        state: &State,
        in_token: bool,
    ) -> Run<Vec<Res>> {
        let program_index = self.compiled.languages[&**language];
        let skipped = self.terminal_start(position, state, in_token)?;
        let start = skipped.end;
        let regions = self.evaluate(item, start, state, true)?;
        let mut results = Vec::new();
        for region in regions {
            let key = (language.clone(), start, region.end);
            let outcome = if let Some(outcome) = self.embed_memo.get(&key) {
                outcome.clone()
            } else {
                let nested_program = &self.compiled.programs[program_index];
                let start_rule = nested_program
                    .start
                    .as_deref()
                    .and_then(|name| nested_program.rule_index.get(name))
                    .copied()
                    .unwrap_or_default();
                let mut nested = Executor::new(
                    self.compiled,
                    program_index,
                    self.bytes,
                    start,
                    region.end,
                    self.shared,
                    self.max_depth.saturating_sub(self.depth),
                );
                let outcome = Rc::new(nested.run(start_rule)?);
                self.embed_memo.insert(key, outcome.clone());
                outcome
            };
            match &*outcome {
                Outcome::Failed { farthest, expected } => {
                    for expectation in expected {
                        self.fail(*farthest, &Name::from(expectation.as_str()));
                    }
                }
                Outcome::Parsed(root) => {
                    let mut child = Tree::new(TreeType::Embed, None, start, region.end);
                    child.language = Some(language.clone());
                    child.root = Some(root.clone());
                    child.program = program_index;
                    let children = if in_token {
                        no_children()
                    } else {
                        with_leaf(&skipped.leaves, child)
                    };
                    results.push(Res::new(region.end, region.state, children, region.dynamic));
                }
            }
        }
        Ok(results)
    }
}

/// The read-only machine conditions see over one result.
pub(super) struct ValueMachine<'e, 'c> {
    executor: &'e mut Executor<'c>,
    working: Working,
    start: usize,
    end: usize,
}

impl<'e, 'c> ValueMachine<'e, 'c> {
    fn new(executor: &'e mut Executor<'c>, result: &Res, from: usize) -> Self {
        Self {
            start: content_start(&result.children, from),
            end: result.end,
            working: result.state.working(),
            executor,
        }
    }
}

impl Machine for ValueMachine<'_, '_> {
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
}
