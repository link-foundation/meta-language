//! Separators a token reads on with, as the separator joins of
//! `js/src/grammar-runtime/executor.js`: a tree-sitter lexer resets the start
//! of a token only on a separator no token valid there begins with, so a
//! separator some valid token begins with is the token's own.

use std::collections::HashSet;
use std::rc::Rc;

use super::executor::{Executor, Run};
use super::operations::State;
use super::program::{Expr, Matcher, Name, Target};
use super::results::{Tree, is_separator};
use super::text::text_of;

impl Executor<'_> {
    /// Records, under keyword lexing, each separator of `leaves` (the trivia
    /// before a token of `item`) the token begins with, though it does not
    /// match on to the token's text: it is read on with (see `joined_start`),
    /// unless the token stops inside it: a lexer then resets the token's
    /// start where only the separator goes on (Make's `\\` before a
    /// newline); one alive at the separator's end reads on from the lexer's
    /// start state. It mirrors the join loop of tokenBeforeExtra in
    /// js/src/grammar-runtime/executor.js.
    pub(super) fn record_joins(
        &mut self,
        item: &Expr,
        leaves: &[Rc<Tree>],
        state: &State,
    ) -> Run<()> {
        let Some(keywords) = self.keywords else {
            return Ok(());
        };
        let call = self.call_stack.last().and_then(|frame| frame.borrow().call);
        for leaf in leaves {
            if !is_separator(leaf) {
                continue;
            }
            let reach = self.prefix_reach(item, leaf.start, state)?;
            if reach >= leaf.end {
                keywords.borrow_mut().match_join(leaf.start, reach, call);
            }
        }
        Ok(())
    }

    /// The farthest offset to which the text from `position` begins a match
    /// of the lexical `expr`, as far as a lexer reads on with it. It mirrors
    /// prefixReach in js/src/grammar-runtime/executor.js.
    fn prefix_reach(&mut self, expr: &Expr, position: usize, state: &State) -> Run<usize> {
        let key = (std::ptr::from_ref(expr).addr(), position);
        if let Some(reach) = self.prefix_reaches.get(&key) {
            return Ok(*reach);
        }
        // A rule that reaches itself at one offset reads on no farther.
        self.prefix_reaches.insert(key, position);
        let reach = self.prefix_reach_of(expr, position, state)?;
        self.prefix_reaches.insert(key, reach);
        Ok(reach)
    }

    /// The ends of the matches of `item` at `position`, in token context.
    fn prefix_ends(&mut self, item: &Expr, position: usize, state: &State) -> Run<Vec<usize>> {
        Ok(self
            .quietly(|this| this.evaluate(item, position, state, true))?
            .iter()
            .map(|result| result.end)
            .collect())
    }

    fn prefix_reach_of(&mut self, expr: &Expr, position: usize, state: &State) -> Run<usize> {
        match expr {
            Expr::Terminal { matcher, .. } => Ok(match matcher {
                Matcher::Literal(text) => {
                    position + self.common_prefix(text.iter().copied(), position, false)
                }
                Matcher::Insensitive { folded, .. } => {
                    position + self.common_prefix(folded.bytes(), position, true)
                }
                _ => matcher
                    .matches(self.bytes, position, self.end)
                    .map_or(position, |end| end.max(position)),
            }),
            Expr::Seq(items) => {
                let (mut best, mut starts) = (position, vec![position]);
                for item in items {
                    let mut next = Vec::new();
                    let mut seen = HashSet::new();
                    for start in starts {
                        best = best.max(self.prefix_reach(item, start, state)?);
                        for end in self.prefix_ends(item, start, state)? {
                            if seen.insert(end) {
                                next.push(end);
                            }
                        }
                    }
                    starts = next;
                    if starts.is_empty() {
                        return Ok(best);
                    }
                }
                Ok(starts.into_iter().fold(best, usize::max))
            }
            Expr::Choice { items, .. } | Expr::Longest(items) => {
                let mut best = position;
                for item in items {
                    best = best.max(self.prefix_reach(item, position, state)?);
                }
                Ok(best)
            }
            Expr::Repeat { item, max, .. } => {
                let max = max.unwrap_or(usize::MAX);
                let (mut best, mut frontier) = (position, vec![position]);
                let mut seen = HashSet::from([position]);
                let mut count = 0;
                while count < max && !frontier.is_empty() {
                    let mut next = Vec::new();
                    for start in frontier {
                        best = best.max(self.prefix_reach(item, start, state)?);
                        for end in self.prefix_ends(item, start, state)? {
                            if seen.insert(end) {
                                next.push(end);
                            }
                        }
                    }
                    frontier = next;
                    count += 1;
                }
                Ok(best)
            }
            Expr::Ref(Target::Rule(index)) => {
                let program = self.program;
                self.prefix_reach(&program.rules[*index].expression, position, state)
            }
            Expr::Token(item)
            | Expr::ImmediateToken(item)
            | Expr::Alias { item, .. }
            | Expr::Capture { item, .. }
            | Expr::Precedence { item, .. }
            | Expr::DynamicPrecedence { item, .. }
            | Expr::LexicalPrecedence { item, .. } => self.prefix_reach(item, position, state),
            _ => Ok(position),
        }
    }

    /// The count of the bytes of `text` the input from `position` begins
    /// with, ASCII case folded where `fold`.
    fn common_prefix(&self, text: impl Iterator<Item = u8>, position: usize, fold: bool) -> usize {
        let folded = |byte: u8| {
            if fold {
                byte.to_ascii_lowercase()
            } else {
                byte
            }
        };
        text.enumerate()
            .take_while(|(at, byte)| {
                position + at < self.end && folded(self.bytes[position + at]) == folded(*byte)
            })
            .count()
    }

    /// The start of the leaf of a token matched at `start` after the trivia
    /// `leaves`, with the count of the leaves before it: a lexer resets the
    /// start of a token only on a separator no token valid there reads on
    /// with, so a separator some valid token begins with is the token's own
    /// (Solidity's `^` of `pragma solidity ^0.8.0;` takes the space before
    /// it, as a version may begin with one). The separators so read on with
    /// in the tree's parse state are `joined` (see `KeywordLexing`). It
    /// mirrors joinedStart in js/src/grammar-runtime/executor.js.
    pub(super) fn joined_start(&self, start: usize, leaves: &[Rc<Tree>]) -> (usize, usize) {
        let unjoined = (start, leaves.len());
        let Some(keywords) = self.keywords else {
            return unjoined;
        };
        let keywords = keywords.borrow();
        if !keywords.has_joins() || leaves.is_empty() {
            return unjoined;
        }
        let (mut reset, mut carry) = (leaves.len(), None);
        for (index, leaf) in leaves.iter().enumerate() {
            let reach = if is_separator(leaf) {
                carry.max(keywords.joined(leaf.start))
            } else {
                None
            };
            if reach.is_some_and(|reach| reach > leaf.start) {
                if reset == leaves.len() {
                    reset = index;
                }
                carry = reach;
                continue;
            }
            (reset, carry) = (leaves.len(), None);
        }
        if reset == leaves.len() {
            unjoined
        } else {
            (leaves[reset].start, reset)
        }
    }

    /// The kind of a leaf from `from` of a token matched over `[start,
    /// end)`: an anonymous token that took a separator before it is still
    /// named by its own text (see `joined_start`). It mirrors joinedKind in
    /// js/src/grammar-runtime/executor.js.
    pub(super) fn joined_kind(
        &self,
        kind: Option<Name>,
        from: usize,
        start: usize,
        end: usize,
    ) -> Option<Name> {
        if kind.is_some() || from == start {
            return kind;
        }
        text_of(self.bytes, start, end).map(|text| Name::from(format!("'{text}")))
    }
}
