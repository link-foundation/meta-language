//! The native grammar executor, as `js/src/grammar-runtime/executor.js`: it
//! interprets a loaded program over a byte string and builds the lossless
//! concrete syntax tree. Generalized matching keeps every result an
//! expression can produce, deduplicated by end offset and parser state; PEG
//! matching keeps at most one. Rule calls are memoized, left recursion grows a
//! seed to a fixpoint, and the nesting depth, the step count and the memo size
//! are bounded, so a hostile input ends in a rejection instead of a stack
//! overflow or a runaway parse. Automatic recovery reruns a failed parse with
//! repair points, where a failing element becomes a MISSING leaf or skips to
//! its next match behind an ERROR leaf. `rules.rs` holds rule calls, actions
//! and external scanners, `lexing.rs` trivia, terminals and tokens,
//! `repetition.rs` repetitions, aliases and longest matches, and
//! `precedence.rs` the precedence filter.

use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

use super::forking::{Lead, Reductions, lookahead_after, reduced_early};
use super::lexing::is_keyword;
use super::operations::{Abort, OpError, State, ValueMachine, evaluate_condition};
use super::parting::ends_missing;
use super::precedence::{Keep, Operands};
use super::program::{
    Associativity, Compiled, Expr, InExtra, Matcher, Name, PrecedenceTag, Program, Settling,
    SettlingStep, Target,
};
use super::repetition::{ImmediateLiterals, immediate_pattern};
use super::results::{
    ChildList, Children, Entry, KeywordLexing, MemoKey, Outcome, Repair, Res, ResultSet, Scanned,
    Shared, Skipped, TokenOrder, Tree, TreeType, children_of, concat, is_separator, no_children,
    with_leaf,
};
use super::text::{decode_at, text_of};

/// The outcome of a step that a resource limit may end.
pub(super) type Run<T> = Result<T, Abort>;

/// The scans a repair point keeps, by the failing element, its offset, the
/// state and whether it is inside an extra: the offset of the first later
/// match and the results there.
type RepairMemo = HashMap<(Element, usize, State, InExtra), Rc<(usize, Vec<Res>)>>;

/// The match of a failing element from a later offset, which a repair point
/// scans for a skip; none for a scanner token, at which no skip ends.
pub(super) type Retry<'r, 'c> =
    Option<&'r mut dyn FnMut(&mut Executor<'c>, usize) -> Run<Vec<Res>>>;

/// The evaluation frames a parse may nest before it is refused as too deep;
/// the parse thread's stack holds this many with room to spare.
const FRAME_LIMIT: usize = 20_000;

/// The repair cost of a MISSING leaf; an ERROR leaf costs the bytes it skips.
const MISSING_COST: usize = 2;

// The kind of the MISSING leaf of a failed terminal or token: the literal
// text for a literal or a keyword (a literal and the lookaheads after it, as
// tree-sitter names its keyword token), also under a lexical precedence (the
// closing `/` of a JavaScript regular expression), else none.
fn missing_of(expr: &Expr) -> (Option<Name>, bool) {
    match expr {
        Expr::Seq(items) if is_keyword(expr) => missing_of(&items[0]),
        Expr::LexicalPrecedence { item, .. } => missing_of(item),
        Expr::Terminal {
            matcher: Matcher::Literal(literal),
            ..
        } => (Some(Name::from(String::from_utf8_lossy(literal))), true),
        _ => (None, false),
    }
}

/// A scan of a scanner token: its name, its start, the context offset where
/// the scanner asks what the parse expects, and the state.
pub(super) type ScanKey = (Name, usize, Option<usize>, State);

/// The marked tokens the rest of a sequence or an iteration may begin with
/// (see `rest_keys`), shared between the memo and its readers.
pub(super) type RestKeys = Option<Rc<HashSet<Lead>>>;

/// Interprets one program over `bytes[begin, end)`.
pub(super) struct Executor<'c> {
    pub(super) compiled: &'c Compiled,
    pub(super) program: &'c Program,
    /// The index of the program, by which the items a scanner's `expected`
    /// asks about are distinct across embedded languages.
    pub(super) program_index: usize,
    pub(super) bytes: &'c [u8],
    pub(super) begin: usize,
    pub(super) end: usize,
    pub(super) shared: &'c Shared,
    pub(super) max_depth: usize,
    pub(super) peg: bool,
    pub(super) longest_tokens: Option<TokenOrder<'c>>,
    /// The grammar's settling steps; the lexing of a tree-sitter lexer
    /// (`lexing`) follows its `tokens` step, the LR reductions (`reducing`)
    /// its `precedence` step.
    pub(super) settling: Settling,
    pub(super) lexing: Option<TokenOrder<'c>>,
    pub(super) reducing: bool,
    pub(super) depth: usize,
    pub(super) memo: HashMap<MemoKey, Rc<RefCell<Entry>>>,
    pub(super) call_stack: Vec<Rc<RefCell<Entry>>>,
    pub(super) trivia_memo: HashMap<(usize, State, InExtra, bool, i64), Rc<Skipped>>,
    pub(super) operand_memo: HashMap<usize, Rc<Operands>>,
    /// The kinds whose nodes go on only with tokens of raised lexical
    /// precedence (see `lexed_shift`).
    pub(super) shift_memo: HashMap<Name, bool>,
    /// The rule holding each precedence, by the address of its item.
    pub(super) owners: Option<HashMap<usize, usize>>,
    /// The rules at the edge of a kind's node facing an operator.
    pub(super) edge_memo: HashMap<(Name, Associativity), Rc<HashSet<usize>>>,
    /// The reduction below a right operand's first part, by the address of
    /// the precedence's item, the operand's rule and the part's kind (see
    /// `shifts_below`).
    pub(super) below_memo: HashMap<(usize, Name, Name), Option<PrecedenceTag>>,
    /// The marked tokens (see `Reductions`) the parts after each item of a
    /// sequence, by the address of its items and the item's index, and each
    /// iteration of a repetition, by the address of its item, may begin
    /// with (see `rest_keys` and `iteration_keys`).
    pub(super) rest_memo: HashMap<(usize, Option<usize>), RestKeys>,
    /// Whether an extra that builds a node is being parsed, and which (see
    /// `extra_node`).
    pub(super) in_extra: InExtra,
    /// Where the innermost extra that builds a node starts.
    pub(super) extra_start: usize,
    /// The scans of each scanner token at an offset, by the context offset
    /// too where the scanner asks what the parse expects.
    pub(super) scanner_memo: HashMap<ScanKey, Option<Rc<Scanned>>>,
    /// The context offset of a scan inside a token or trivia: where the
    /// token's terminal or the trivia started (see `Expectations`).
    pub(super) scan_context: Option<usize>,
    pub(super) embed_memo: HashMap<(Name, usize, usize), Rc<Outcome>>,
    pub(super) farthest: usize,
    pub(super) expected: HashSet<Name>,
    pub(super) suppressed: usize,
    /// Automatic recovery: the offsets where a failing element is repaired,
    /// and the farthest offset where an element failed without a repair.
    pub(super) repair_points: Option<HashSet<usize>>,
    pub(super) element_farthest: Option<usize>,
    /// The scan past each failing element at a repair point: the offset of
    /// its first later match and the results there (see `element_failed`).
    pub(super) repair_memo: RepairMemo,
    /// The repair points where a continuation after a MISSING leaf is open.
    pub(super) chained: HashSet<usize>,
    /// Under `(matching longest)`, the keyword lexing of the parse (see
    /// `KeywordLexing`), or none.
    pub(super) keywords: Option<&'c RefCell<KeywordLexing>>,
    /// The starts of separators a token lexed at them took, going on past
    /// the trivia, each with the chains of node calls the token was lexed
    /// in, their kinds and offsets (see `token_before_extra` and
    /// `widen_tokens`).
    pub(super) separator_taken: HashMap<usize, HashSet<Vec<(Name, usize)>>>,
    /// The tokens an alias of no rule names, by that name (see
    /// `aliased_tokens`).
    pub(super) aliased: Option<HashMap<Name, Vec<&'c Expr>>>,
    /// The immediate literals of each unordered choice, by the address of
    /// its items (see `immediate_literals`).
    pub(super) immediate_memo: HashMap<usize, Option<Rc<ImmediateLiterals>>>,
    /// The reach of each lexical expression's match from an offset, by the
    /// expression's address and the offset (see `prefix_reach`).
    pub(super) prefix_reaches: HashMap<(usize, usize), usize>,
}

/// An element whose scan a repair point keeps (see `element_failed`): an
/// expression or a rule, by its address, or a scanner token, by its name.
#[derive(Clone, PartialEq, Eq, Hash)]
pub(super) enum Element {
    At(usize),
    Scanner(Name),
}

impl Element {
    pub(super) fn of<T: ?Sized>(element: &T) -> Self {
        Self::At(std::ptr::from_ref(element).cast::<()>().addr())
    }
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
        let longest_tokens = program.token_ranks.as_ref().map(|ranks| TokenOrder {
            ranks,
            bytes,
            orders: &program.precedence_orders,
            grammar: &program.grammar,
            trivia: &program.trivia,
        });
        Self {
            compiled,
            program,
            program_index,
            bytes,
            begin,
            end,
            shared,
            max_depth,
            peg: program.peg,
            longest_tokens,
            settling: program.settling,
            lexing: longest_tokens.filter(|_| program.settling.has(SettlingStep::Tokens)),
            reducing: program.settling.has(SettlingStep::Precedence),
            depth: 0,
            memo: HashMap::new(),
            call_stack: Vec::new(),
            trivia_memo: HashMap::new(),
            operand_memo: HashMap::new(),
            shift_memo: HashMap::new(),
            owners: None,
            edge_memo: HashMap::new(),
            below_memo: HashMap::new(),
            rest_memo: HashMap::new(),
            in_extra: InExtra::Outside,
            extra_start: usize::MAX,
            scanner_memo: HashMap::new(),
            scan_context: None,
            embed_memo: HashMap::new(),
            farthest: begin,
            expected: HashSet::new(),
            suppressed: 0,
            repair_points: None,
            element_farthest: None,
            repair_memo: HashMap::new(),
            chained: HashSet::new(),
            keywords: None,
            separator_taken: HashMap::new(),
            aliased: None,
            immediate_memo: HashMap::new(),
            prefix_reaches: HashMap::new(),
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

    /// Takes `count` memo cells of the parse's memory budget.
    pub(super) fn retain(&self, count: usize) -> Run<()> {
        let cells = self.shared.cells.get() + count;
        self.shared.cells.set(cells);
        if cells > self.shared.memory_limit {
            Err(Abort::MemoryBudget)
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

    /// How a rule call at `position` is made while repairing, so a call made
    /// quietly or after a MISSING leaf is memoized apart from the same call
    /// made in the open.
    pub(super) fn repair_mode(&self, position: usize) -> Repair {
        if self.repair_points.is_none() {
            Repair::Open
        } else if self.suppressed > 0 {
            Repair::Quiet
        } else if self.chained.contains(&position) {
            Repair::Chained
        } else {
            Repair::Open
        }
    }

    /// Automatic recovery. An element (a terminal, a token, a token or atomic
    /// rule, a scanner token) that fails in syntactic context at `start`, its
    /// offset after trivia, is noted; at a repair point it yields instead a
    /// zero-width MISSING leaf and, when `retry` matches the element at a
    /// later code point boundary, a result that skips the bytes up to the
    /// first such offset as an ERROR leaf. The scan for that offset depends
    /// only on the element, `start` and the state, so it is made once per
    /// `element` (the expression, rule or scanner token that failed) there.
    /// A token of an external scanner has no `retry`: as in tree-sitter,
    /// whose recovery lexes the skipped input in its error state, where a
    /// scanner refuses to run (tree-sitter-rust's error sentinel), no skip
    /// ends at such a token, and a scanner that reads to the end of the input
    /// before it fails would make the scan quadratic.
    #[allow(clippy::too_many_arguments)]
    pub(super) fn element_failed(
        &mut self,
        start: usize,
        leaves: &[Rc<Tree>],
        state: &State,
        kind: Option<Name>,
        literal: bool,
        element: Element,
        mut retry: Retry<'_, 'c>,
    ) -> Run<Vec<Res>> {
        if self.suppressed > 0 {
            return Ok(Vec::new());
        }
        if !self
            .repair_points
            .as_ref()
            .is_some_and(|points| points.contains(&start))
        {
            if self
                .element_farthest
                .is_none_or(|farthest| start > farthest)
            {
                self.element_farthest = Some(start);
            }
            return Ok(Vec::new());
        }
        // As in tree-sitter, whose missing leaf has no padding, the MISSING
        // leaf comes before the separators (white space) that end the trivia
        // before it, after any other extra.
        let kept = leaves.len()
            - leaves
                .iter()
                .rev()
                .take_while(|leaf| is_separator(leaf))
                .count();
        let at = match kept {
            0 => leaves.first().map_or(start, |leaf| leaf.start),
            _ => leaves[kept - 1].end,
        };
        let mut missing = Tree::new(TreeType::Missing, kind, at, at);
        missing.literal = literal;
        let placed = concat(&with_leaf(&leaves[..kept], missing), &leaves[kept..]);
        let mut repaired = Res::new(start, state.clone(), placed, 0);
        repaired.cost = MISSING_COST;
        // The leaf is open (see `sequence`) until a rule it ends reduces.
        repaired.open = true;
        let mut results = vec![repaired];
        let key = (element, start, state.clone(), self.in_extra);
        let scan = if let Some(scan) = self.repair_memo.get(&key) {
            scan.clone()
        } else {
            let mut scan = Rc::new((start, Vec::new()));
            let mut cursor = start;
            while let Some(retry) = retry.as_mut().filter(|_| cursor < self.end) {
                cursor += decode_at(self.bytes, cursor, self.end).1;
                let found = self.quietly(|this| retry(this, cursor))?;
                if !found.is_empty() {
                    scan = Rc::new((cursor, found));
                    break;
                }
            }
            self.repair_memo.insert(key, scan.clone());
            scan
        };
        let (end, found) = &*scan;
        let error: Children = with_leaf(leaves, Tree::new(TreeType::Error, None, start, *end));
        for result in found {
            results.push(Res {
                children: concat(&error, &result.children),
                cost: result.cost + end - start,
                ..result.clone()
            });
        }
        Ok(results)
    }

    pub(super) fn text(&self, start: usize, end: usize) -> String {
        text_of(self.bytes, start, end).unwrap_or_default()
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
                expected,
            } => self.terminal((matcher, expectation, *expected), position, state, in_token),
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
                        result.children = children_of(
                            result
                                .children
                                .iter()
                                .map(|child| {
                                    // A child a silent rule captured keeps
                                    // its field: tree-sitter names a child by
                                    // the innermost field over it (Lean's
                                    // `type` of `Nat` in the `binders` of
                                    // `∀ x : Nat, p`).
                                    if child.trivia || child.field.is_some() {
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
            Expr::Alias { name, item } => self.alias(expr, (name, item), position, state, in_token),
            Expr::Precedence {
                level,
                name,
                associativity,
                item,
            } => self.precedence(
                *level,
                name.as_ref(),
                *associativity,
                item,
                position,
                state,
                in_token,
            ),
            Expr::DynamicPrecedence { level, item } => {
                let mut results = self.evaluate(item, position, state, in_token)?;
                for result in &mut results {
                    result.dynamic += level;
                }
                Ok(results)
            }
            Expr::LexicalPrecedence { item, .. } => self.evaluate(item, position, state, in_token),
            Expr::Longest(items) => self.longest(items, position, state, in_token),
            Expr::Token(item) | Expr::ImmediateToken(item) => {
                if matches!(expr, Expr::ImmediateToken(_)) && !in_token {
                    self.outrank_trivia(item, position, state)?;
                }
                let starts = if matches!(expr, Expr::Token(_)) {
                    let skipped = self.terminal_start(position, state, in_token)?;
                    if self.lexing.is_some() && !skipped.leaves.is_empty() {
                        vec![self.token_before_extra(item, skipped, state)?]
                    } else {
                        vec![skipped]
                    }
                } else {
                    self.immediate_starts(position, state, in_token)?
                };
                let mut found = ResultSet::new(self.longest_tokens, self.settling);
                let immediate = matches!(expr, Expr::ImmediateToken(_))
                    && matches!(&**item, Expr::Terminal { matcher, .. } if matches!(matcher, Matcher::Literal(_)));
                let keywords = self
                    .keywords
                    .filter(|_| !in_token && (immediate || is_keyword(item)));
                // A scanner token is lexed at the first start where its
                // scanner succeeds, as a lexer runs the external scanner
                // before it lexes an extra: after a comment only where it
                // fails before it.
                let scanned = matches!(**item, Expr::Ref(Target::External(_)));
                // The external scanner skips separators itself, so only a
                // token of the lexer takes one in (Lean's layout tokens do
                // not).
                let separating = self
                    .keywords
                    .zip(self.lexing)
                    .filter(|_| !in_token && matches!(expr, Expr::ImmediateToken(_)) && !scanned);
                // The token is requested where the parse asks for it, though
                // its item is evaluated in token context (TypeScript's
                // function signature ends with `(immediateToken (ref
                // function_signature_automatic_semicolon))`, which the
                // automatic semicolon scanner asks about).
                if !in_token {
                    self.request_item(item, position);
                }
                let context = self.scan_context;
                if !in_token {
                    self.scan_context = Some(position);
                }
                let lexed = (|| -> Run<()> {
                    for skipped in &starts {
                        for result in self.token_leaf(item, skipped, state, in_token)? {
                            if let Some(keywords) = keywords {
                                let call =
                                    self.call_stack.last().and_then(|frame| frame.borrow().call);
                                keywords.borrow_mut().matched(
                                    (skipped.end, result.end),
                                    call,
                                    immediate,
                                );
                            }
                            if let Some((keywords, lexing)) = separating
                                && skipped.end == position
                                && lexing.separator_text(skipped.end, result.end)
                            {
                                let call =
                                    self.call_stack.last().and_then(|frame| frame.borrow().call);
                                keywords.borrow_mut().matched_separator(position, call);
                            }
                            found.add(result);
                        }
                        if scanned && !found.items.is_empty() {
                            break;
                        }
                    }
                    Ok(())
                })();
                self.scan_context = context;
                lexed?;
                let results = found.items;
                if !results.is_empty() || in_token {
                    return Ok(results);
                }
                let skipped = &starts[0];
                let (kind, literal) = missing_of(item);
                self.element_failed(
                    skipped.end,
                    &skipped.leaves,
                    state,
                    kind,
                    literal,
                    Element::of(expr),
                    (!scanned).then_some(&mut |this: &mut Self, cursor| {
                        this.evaluate(expr, cursor, state, false)
                    }),
                )
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
                    children_of(vec![Rc::new(node)])
                };
                Ok(vec![Res::new(position, state.clone(), children, 0)])
            }
            Expr::Embed { language, item } => self.embed(language, item, position, state, in_token),
        }
    }

    // The results of `item` after `left`. While repairing, a sequence may
    // continue after a MISSING leaf at a repair point, repairing what follows,
    // but what follows may not do so again at that offset: a second
    // continuation there is matched quietly. Chains of zero-width MISSING
    // leaves, which would make every rule left-recursive at that offset, are
    // so never built. The MISSING leaf may end a node `left` holds when tokens
    // the external scanner scanned of no width follow it, as a layout token
    // opening an indented block does: each such repaired block would otherwise
    // open another at the offset, up to the scanner's deepest indentation.
    pub(super) fn continuation(
        &mut self,
        item: &Expr,
        left: &Res,
        in_token: bool,
    ) -> Run<Vec<Res>> {
        let repaired = self
            .repair_points
            .as_ref()
            .is_some_and(|points| points.contains(&left.end))
            && ends_missing(&left.children);
        if !repaired {
            return self.evaluate(item, left.end, &left.state, in_token);
        }
        if self.chained.contains(&left.end) {
            return self
                .quietly(|executor| executor.evaluate(item, left.end, &left.state, in_token));
        }
        self.chained.insert(left.end);
        let results = self.evaluate(item, left.end, &left.state, in_token);
        self.chained.remove(&left.end);
        results
    }

    // `keep`, when given, filters the complete sequences before they are
    // deduplicated, so a precedence filter never loses a valid parse to an
    // invalid one that reached the same end first, and its precedence is the
    // one the sequence's parts are in progress under. A MISSING leaf is open
    // until a rule it ends reduces (a rule other than a token, or an
    // iteration): as tree-sitter inserts a missing token only where a rule
    // reduces before the lookahead, no part of the same rule after it takes
    // input (`[ 0 .92 ]` misses no `,` before `.92`).
    pub(super) fn sequence(
        &mut self,
        items: &[Expr],
        position: usize,
        state: &State,
        in_token: bool,
        keep: Option<&Keep>,
    ) -> Run<Vec<Res>> {
        let reductions = self.reductions(in_token);
        let split =
            reductions.and_then(|reductions| reductions.splits.get(&Reductions::key(items)));
        // The offset each result's optional parts begin at, where its rule
        // could have been reduced (see `reduction_facts`), by the result's
        // children.
        let mut boundaries: HashMap<*const ChildList, usize> = HashMap::new();
        let mut first_parts: HashMap<*const ChildList, bool> = HashMap::new();
        let mut current = vec![Res::new(position, state.clone(), no_children(), 0)];
        for (index, item) in items.iter().enumerate() {
            let mut next =
                ResultSet::new(self.longest_tokens, self.settling).owned(keep.map(Keep::owner));
            let last = index == items.len() - 1;
            let rest = match reductions {
                Some(reductions) if !last => self.rest_keys(reductions, items, index),
                _ => None,
            };
            let continued = self.continued(item, &current, in_token)?;
            let pruned = self.preempted(&continued, in_token);
            for ((left, rights), pruned) in continued.into_iter().zip(pruned) {
                if pruned {
                    continue;
                }
                for right in rights {
                    if rest
                        .as_ref()
                        .is_some_and(|rest| reduced_early(&right.children, rest))
                    {
                        continue;
                    }
                    if left.open && right.end > left.end {
                        continue;
                    }
                    let right_single = keep.is_none()
                        || right.children.iter().filter(|child| !child.trivia).count() == 1;
                    let left_single = if index == 0 {
                        right_single
                    } else {
                        first_parts
                            .get(&Rc::as_ptr(&left.children))
                            .copied()
                            .unwrap_or(true)
                    };
                    let mut joined = Res::join(left, right, in_token);
                    if keep.is_some() {
                        first_parts.insert(Rc::as_ptr(&joined.children), left_single);
                    }
                    if let Some(split) = split
                        && index >= split.optional_from
                    {
                        let boundary = if index == split.optional_from {
                            Some(left.end)
                        } else {
                            boundaries.get(&Rc::as_ptr(&left.children)).copied()
                        };
                        if !last {
                            if let Some(boundary) = boundary {
                                boundaries.insert(Rc::as_ptr(&joined.children), boundary);
                            }
                        } else if let Some(lookahead) = boundary.and_then(|boundary| {
                            lookahead_after(&joined.children, boundary, self.bytes)
                        }) {
                            if split.always.contains(&lookahead) {
                                continue;
                            }
                            if split.marked.contains(&lookahead) {
                                joined.before = Some(lookahead);
                            }
                        }
                    }
                    if last
                        && let Some(keep) = keep
                        && !self.precedence_valid_edges(keep, &joined, left_single, right_single)
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

    /// The reductions before a following token (see `reduction_facts`) the
    /// results are checked against: under a settling with `precedence`,
    /// outside a token.
    pub(super) fn reductions(&self, in_token: bool) -> Option<&'c Reductions> {
        let tokens = self.longest_tokens.filter(|_| self.reducing && !in_token)?;
        Some(tokens.grammar.reductions(&self.program.rules))
    }

    /// The marked tokens (see `Reductions`) an iteration of `item` may begin
    /// with, or None when it begins with none. It mirrors iterationKeys in
    /// js/src/grammar-runtime/executor.js.
    pub(super) fn iteration_keys(&mut self, reductions: &Reductions, item: &Expr) -> RestKeys {
        if reductions.keys.is_empty() {
            return None;
        }
        let rules = &self.program.rules;
        self.rest_memo
            .entry((std::ptr::from_ref(item).addr(), None))
            .or_insert_with(|| {
                reductions
                    .marked(std::slice::from_ref(item), rules)
                    .map(Rc::new)
            })
            .clone()
    }

    /// The marked tokens (see `Reductions`) the parts of `items` after the
    /// one at `index` may begin with, or None when they begin with none. It
    /// mirrors restKeys in js/src/grammar-runtime/executor.js.
    fn rest_keys(&mut self, reductions: &Reductions, items: &[Expr], index: usize) -> RestKeys {
        if reductions.keys.is_empty() {
            return None;
        }
        let rules = &self.program.rules;
        self.rest_memo
            .entry((items.as_ptr().addr(), Some(index)))
            .or_insert_with(|| reductions.marked(&items[index + 1..], rules).map(Rc::new))
            .clone()
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
        let mut results = ResultSet::new(self.longest_tokens, self.settling);
        let outranked = if self.lexing.is_some() && !in_token {
            self.immediate_literals(items)
        } else {
            None
        };
        for item in items {
            if let Some(literals) = &outranked
                && let Some(pattern) = immediate_pattern(item)
                && self.literal_outranks(pattern, &literals.0, position, state)?
            {
                continue;
            }
            for result in self.evaluate(item, position, state, in_token)? {
                results.add(result);
            }
        }
        Ok(results.items)
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
                Outcome::Failed {
                    farthest, expected, ..
                } => {
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
