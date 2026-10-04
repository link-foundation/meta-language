//! Trivia, terminals and tokens of the native executor: the lexical half of
//! `js/src/grammar-runtime/executor.js`.

use std::cmp::Ordering;
use std::rc::Rc;

use super::executor::{Element, Executor, Run};
use super::operations::State;
use super::program::{Expr, Matcher, Name, Target};
use super::results::{
    Res, Skipped, Tree, TreeType, children_of, longest_result, no_children, preferred_tokens,
    with_leaf,
};
use crate::grammar::RuleKind;

// Whether a token is a keyword, a literal closed by lookaheads (`(seq
// (literal typedef) (not (ref word_characters)))`).
pub(super) fn is_keyword(expr: &Expr) -> bool {
    match expr {
        Expr::Seq(items) if items.len() >= 2 => {
            matches!(
                items[0],
                Expr::Terminal {
                    matcher: Matcher::Literal(_),
                    ..
                }
            ) && items[1..]
                .iter()
                .all(|item| matches!(item, Expr::Not(_) | Expr::And(_)))
        }
        _ => false,
    }
}

// A literal that took the separators after it (see `before_separator`) is
// still that literal, as a tree-sitter lexer names it: its leaf is an
// anonymous alias of the literal (`'\n` over `\n\n`). Any other terminal
// leaf is named by its text.
fn separator_run_kind(matcher: &Matcher, start: usize, end: usize) -> Option<Name> {
    match matcher {
        Matcher::Literal(literal) if end - start > literal.len() => {
            Some(Name::from(format!("'{}", String::from_utf8_lossy(literal))))
        }
        _ => None,
    }
}

impl Executor<'_> {
    /// Skips trivia: repeatedly the longest match of any trivia expression
    /// allowed in the current mode. Inside an extra that builds a node, only
    /// the extras that are no rule (white space) are trivia, as no extra nests
    /// in another.
    pub(super) fn skip_trivia(&mut self, position: usize, state: &State) -> Run<Rc<Skipped>> {
        let program = self.program;
        let trivia = &program.trivia;
        if trivia.is_empty() {
            return Ok(Rc::new(Skipped {
                end: position,
                leaves: no_children(),
            }));
        }
        let key = (position, state.clone(), self.in_extra);
        if let Some(cached) = self.trivia_memo.get(&key) {
            return Ok(cached.clone());
        }
        let mode = state.mode().to_owned();
        let mut leaves = Vec::new();
        // A scanner in the trivia answers `expected` for where they start.
        let context = self.scan_context.replace(position);
        let end = self.skip_from(position, &mut leaves, &mode, state);
        self.scan_context = context;
        let skipped = Rc::new(Skipped {
            end: end?,
            leaves: children_of(leaves),
        });
        self.trivia_memo.insert(key, skipped.clone());
        Ok(skipped)
    }

    /// Skips the trivia from `position`, pushing their leaves; gives where
    /// they end.
    fn skip_from(
        &mut self,
        position: usize,
        leaves: &mut Vec<Rc<Tree>>,
        mode: &str,
        state: &State,
    ) -> Run<usize> {
        let trivia = &self.program.trivia;
        let mut cursor = position;
        loop {
            let mut best = cursor;
            let mut best_kind = None;
            for item in trivia {
                if item
                    .modes
                    .as_ref()
                    .is_some_and(|modes| !modes.iter().any(|allowed| **allowed == *mode))
                    || (self.in_extra && item.kind.is_some())
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
            if let Some((node, end)) = self.extra_node(best_kind.as_ref(), cursor, best, state)? {
                leaves.push(node);
                cursor = end;
            } else {
                leaves.push(Rc::new(Tree::trivia_leaf(best_kind, cursor, best)));
                cursor = best;
            }
        }
        Ok(cursor)
    }

    /// The node an extra of a rule that builds one makes of its text, parsed
    /// as syntax, as a tree-sitter extra of a rule that is no token is a node
    /// with its children (Rust's doc comments); None for any other extra.
    /// Under `(matching longest)` the parse is the one with the tokens a lexer
    /// prefers, which may end before the longest (Rust's `////` is a comment
    /// without a doc marker); otherwise the one that ends at `end`. Gives the
    /// node and its end.
    pub(super) fn extra_node(
        &mut self,
        kind: Option<&Name>,
        start: usize,
        end: usize,
        state: &State,
    ) -> Run<Option<(Rc<Tree>, usize)>> {
        let Some(&index) = kind.and_then(|kind| self.program.rule_index.get(&**kind)) else {
            return Ok(None);
        };
        if !matches!(self.program.rules[index].kind, RuleKind::Normal) {
            return Ok(None);
        }
        self.in_extra = true;
        let results =
            self.quietly(|this| this.reference(&Target::Rule(index), start, state, false));
        self.in_extra = false;
        let mut best: Option<Res> = None;
        for result in results? {
            if result.cost != 0
                || !result
                    .children
                    .iter()
                    .any(|child| child.ty == TreeType::Node)
            {
                continue;
            }
            let better = match (self.lexing, &best) {
                (_, None) if self.lexing.is_some() => true,
                (None, best) => best.is_none() && result.end == end,
                (Some(tokens), Some(best)) => {
                    let order = preferred_tokens(&result.children, &best.children, tokens);
                    order == Ordering::Greater
                        || (order == Ordering::Equal && result.end > best.end)
                }
                (Some(_), None) => true,
            };
            if better {
                best = Some(result);
            }
        }
        Ok(best.and_then(|best| {
            let node = best
                .children
                .iter()
                .find(|child| child.ty == TreeType::Node)?;
            let mut extra = (**node).clone();
            extra.trivia = true;
            Some((Rc::new(extra), best.end))
        }))
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

    /// The starts of an immediate token: at once, or, in syntactic context
    /// under `(matching longest)`, after the trivia up to an extra that is no
    /// separator (such as a comment): a lexer skips no separator before an
    /// immediate token, but lexes an extra token before it as before any
    /// other.
    pub(super) fn immediate_starts(
        &mut self,
        position: usize,
        state: &State,
        in_token: bool,
    ) -> Run<Vec<Rc<Skipped>>> {
        let mut starts = vec![Rc::new(Skipped {
            end: position,
            leaves: no_children(),
        })];
        if in_token || self.lexing.is_none() {
            return Ok(starts);
        }
        let skipped = self.skip_trivia(position, state)?;
        for (index, leaf) in skipped.leaves.iter().enumerate() {
            if leaf.kind.is_some() {
                starts.push(Rc::new(Skipped {
                    end: leaf.end,
                    leaves: children_of(skipped.leaves[..=index].to_vec()),
                }));
            }
        }
        Ok(starts)
    }

    /// A terminal: its matcher, its expectation and the id of the literal
    /// where a scanner's `expected` asks about it.
    pub(super) fn terminal(
        &mut self,
        terminal: (&Matcher, &Name, Option<usize>),
        position: usize,
        state: &State,
        in_token: bool,
    ) -> Run<Vec<Res>> {
        let (matcher, expectation, expected) = terminal;
        if !in_token && let Some(id) = expected {
            self.shared
                .expectations
                .borrow_mut()
                .request(position, (self.program_index, id));
        }
        let skipped = self.terminal_start(position, state, in_token)?;
        let mut start = skipped.end;
        let mut leaves: &[Rc<Tree>] = &skipped.leaves;
        let found = if self.lexing.is_some()
            && let Some((at, from, end)) = self.before_separator(matcher, leaves)
        {
            start = from;
            leaves = &leaves[..at];
            Some(end)
        } else {
            None
        };
        // A literal the grammar also takes as an immediate token (see
        // `KeywordLexing`) is not lexed plainly where the immediate one
        // outranks it.
        let plain = !in_token
            && self.keywords.is_some()
            && matches!(matcher, Matcher::Literal(literal)
                if self.longest_tokens.is_some_and(|tokens| tokens.ranks.immediate.contains(literal)));
        let Some(end) = found
            .or_else(|| matcher.matches(self.bytes, start, self.end))
            .filter(|end| {
                !plain
                    || !self
                        .keywords
                        .is_some_and(|keywords| keywords.borrow().immediate_only((start, *end)))
            })
        else {
            self.fail(start, expectation);
            if in_token {
                return Ok(Vec::new());
            }
            let (kind, literal) = match matcher {
                Matcher::Literal(literal) => {
                    (Some(Name::from(String::from_utf8_lossy(literal))), true)
                }
                _ => (None, false),
            };
            return self.element_failed(
                start,
                leaves,
                state,
                kind,
                literal,
                Element::of(matcher),
                Some(&mut |this, cursor| this.terminal(terminal, cursor, state, false)),
            );
        };
        let children = if in_token {
            no_children()
        } else {
            let kind = separator_run_kind(matcher, start, end);
            let mut leaf = Tree::new(TreeType::Token, kind, start, end);
            leaf.plain = plain;
            with_leaf(leaves, leaf)
        };
        Ok(vec![Res::new(end, state.clone(), children, 0)])
    }

    // Under `(matching longest)` a lexer takes a valid token over a separator
    // (an anonymous trivia leaf, such as whitespace) it covers: a terminal
    // that matches at the start of such a leaf, at least as far, is matched
    // there, before it and the trivia after it, so `\n` ends a line where
    // whitespace is trivia. As in a tree-sitter lexer, whose separators loop
    // back to the start of every token, the token then also takes each next
    // separator it matches the same way (`\n\n` is one `\n` token). The
    // index of the first such leaf, its start and the end of the match, or
    // None when no such separator precedes the terminal.
    pub(super) fn before_separator(
        &self,
        matcher: &Matcher,
        leaves: &[Rc<Tree>],
    ) -> Option<(usize, usize, usize)> {
        let covers = |leaf: &Tree, from: usize| {
            leaf.kind.is_none()
                && leaf.start == from
                && matcher
                    .matches(self.bytes, from, self.end)
                    .is_some_and(|end| end >= leaf.end)
        };
        let at = leaves.iter().position(|leaf| covers(leaf, leaf.start))?;
        let from = leaves[at].start;
        let mut end = matcher.matches(self.bytes, from, self.end)?;
        for leaf in &leaves[at + 1..] {
            if !covers(leaf, end) {
                break;
            }
            end = matcher.matches(self.bytes, end, self.end)?;
        }
        Some((at, from, end))
    }

    // A leaf over the longest match of `item` in token context: token(),
    // immediateToken() and longest() alternatives build on it. A token under
    // a lexical precedence keeps its level on the leaf, as the token's rank
    // where it is matched (see `token_rank`).
    pub(super) fn token_leaf(
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
            let mut leaf = Tree::new(TreeType::Token, None, start, best.end);
            if let Expr::LexicalPrecedence { level, .. } = item {
                leaf.priority = Some(*level);
            }
            // A token of an external scanner is marked, whatever an alias
            // names it (see `preferred_tokens`).
            leaf.scanned = matches!(item, Expr::Ref(Target::External(_)));
            with_leaf(&skipped.leaves, leaf)
        };
        Ok(vec![Res::new(best.end, best.state, children, best.dynamic)])
    }
}
