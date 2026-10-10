//! Trivia, terminals and tokens of the native executor: the lexical half of
//! `js/src/grammar-runtime/executor.js`.

use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::rc::Rc;

use super::executor::{Element, Executor, Run};
use super::forking::Lead;
use super::operations::State;
use super::program::{Expr, InExtra, Matcher, Name, Program, Rule, Target};
use super::results::{
    Entry, Res, Skipped, Tree, TreeType, children_of, concat, is_separator, longest_result,
    no_children, preferred_tokens, with_leaf,
};
use crate::grammar::RuleKind;

// Whether a token is a keyword, a literal closed by lookaheads (`(seq
// (literal typedef) (not (ref word_characters)))`).
pub(in crate::grammar::feature_runtime) fn is_keyword(expr: &Expr) -> bool {
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

/// A UTF-8 byte order mark.
const BYTE_ORDER_MARK: &[u8] = b"\xef\xbb\xbf";

/// What an extra's text is where it starts (see `extra_node`).
pub(super) enum Extra {
    /// The node an extra of a rule that builds one makes, and its end.
    Node(Rc<Tree>, usize),
    /// A trivia leaf.
    Leaf,
    /// No extra.
    None,
}

impl Executor<'_> {
    /// Skips trivia: repeatedly the longest match of any trivia expression
    /// allowed in the current mode. Inside an extra that builds a node, only
    /// the extras that are no rule (white space) are trivia, unless the extras
    /// nest in it (see `nesting_extras` in load.rs): then all are but at its
    /// own start, where the extra itself is being parsed.
    pub(super) fn skip_trivia(&mut self, position: usize, state: &State) -> Run<Rc<Skipped>> {
        let program = self.program;
        let trivia = &program.trivia;
        if trivia.is_empty() {
            return Ok(Rc::new(Skipped {
                end: position,
                leaves: no_children(),
            }));
        }
        let outrank = self
            .lex_frame(position)
            .and_then(|frame| frame.borrow().outranks.get(&position).copied())
            .unwrap_or(0);
        let at_extra = self.in_extra == InExtra::Nesting && position == self.extra_start;
        let key = (position, state.clone(), self.in_extra, at_extra, outrank);
        if let Some(cached) = self.trivia_memo.get(&key) {
            return Ok(cached.clone());
        }
        let mode = state.mode().to_owned();
        let mut leaves = Vec::new();
        // A scanner in the trivia answers `expected` for where they start.
        let context = self.scan_context.replace(position);
        let end = self.skip_from(position, &mut leaves, &mode, state, outrank, at_extra);
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
        outrank: i64,
        at_extra: bool,
    ) -> Run<usize> {
        let trivia = &self.program.trivia;
        // A separator an immediate token takes there is lexed, not skipped
        // (see `KeywordLexing`).
        let lexed = self
            .keywords
            .is_some_and(|keywords| keywords.borrow().lexes_separator(position));
        // A tree-sitter lexer skips a byte order mark at the start of the input.
        let mut cursor = if position == 0
            && self.longest_tokens.is_some()
            && self.bytes.starts_with(BYTE_ORDER_MARK)
        {
            leaves.push(Rc::new(Tree::trivia_leaf(None, 0, BYTE_ORDER_MARK.len())));
            BYTE_ORDER_MARK.len()
        } else {
            position
        };
        loop {
            let mut best = cursor;
            let mut best_kind = None;
            for item in trivia {
                if item
                    .modes
                    .as_ref()
                    .is_some_and(|modes| !modes.iter().any(|allowed| **allowed == *mode))
                    || (item.kind.is_some()
                        && (self.in_extra == InExtra::Flat || (at_extra && cursor == position)))
                    || (cursor == position
                        && outrank > 0
                        && self.priority_of(&item.expression) < outrank)
                    || (cursor == position && lexed && item.kind.is_none())
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
            match self.extra_node(best_kind.as_ref(), cursor, best, state)? {
                Extra::Node(node, end) => {
                    leaves.push(node);
                    cursor = end;
                }
                Extra::Leaf => {
                    leaves.push(Rc::new(Tree::trivia_leaf(best_kind, cursor, best)));
                    cursor = best;
                }
                Extra::None => break,
            }
        }
        Ok(cursor)
    }

    /// A lexer takes a valid token of a raised lexical precedence over an
    /// extra of a lower one that starts where it does, however longer the
    /// extra is: in JavaScript's `"//"` the string fragment `//` after the
    /// quote is no comment running to the end of the line. So where an
    /// immediate token of a raised level matches, in syntactic context, no
    /// trivia of a lower level is lexed at its offset (see `skip_trivia`).
    /// The level holds in the rule call the token is lexed for (see
    /// `lex_frame`), as a lexer lexes it in one parse state: after the opening
    /// quote of a string, not after a string the parse tried to open at its
    /// closing quote.
    pub(super) fn outrank_trivia(
        &mut self,
        item: &Expr,
        position: usize,
        state: &State,
    ) -> Run<()> {
        let level = self.priority_of(item);
        if level <= 0 {
            return Ok(());
        }
        let Some(frame) = self.lex_frame(position) else {
            return Ok(());
        };
        if frame
            .borrow()
            .outranks
            .get(&position)
            .is_some_and(|known| *known >= level)
        {
            return Ok(());
        }
        let results = self.quietly(|this| this.evaluate(item, position, state, true))?;
        if longest_result(results).is_some_and(|result| result.end > position) {
            frame.borrow_mut().outranks.insert(position, level);
        }
        Ok(())
    }

    /// The innermost rule call that began before `position`: the one a token
    /// at `position` is lexed for, the calls that begin there being part of
    /// the same parse state.
    fn lex_frame(&self, position: usize) -> Option<Rc<RefCell<Entry>>> {
        self.call_stack
            .iter()
            .rev()
            .find(|frame| frame.borrow().position < position)
            .cloned()
    }

    /// The node an extra of a rule that builds one makes of its text, parsed
    /// as syntax, as a tree-sitter extra of a rule that is no token is a node
    /// with its children (Rust's doc comments); a leaf for any other extra,
    /// and none where the text is no extra.
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
    ) -> Run<Extra> {
        let Some(&index) = kind.and_then(|kind| self.program.rule_index.get(&**kind)) else {
            return Ok(Extra::Leaf);
        };
        if !matches!(self.program.rules[index].kind, RuleKind::Normal) {
            return Ok(Extra::Leaf);
        }
        let outer = (self.in_extra, self.extra_start);
        let nesting = kind.is_some_and(|kind| self.program.nesting_extras.contains(kind));
        self.in_extra = if nesting {
            InExtra::Nesting
        } else {
            InExtra::Flat
        };
        self.extra_start = start;
        let results =
            self.quietly(|this| this.reference(&Target::Rule(index), start, state, false));
        (self.in_extra, self.extra_start) = outer;
        let parsed: Vec<Res> = results?
            .into_iter()
            .filter(|result| result.cost == 0)
            .collect();
        // An extra other extras nest in is syntax to a tree-sitter lexer, which
        // lexes the nested extras in it: where it has no parse as syntax, it is
        // no extra, though its text matches as one token (Rocq's
        // `(* a (* b *)`, whose inner comment closes and leaves the outer open).
        if parsed.is_empty() && nesting {
            return Ok(Extra::None);
        }
        let mut best: Option<Res> = None;
        for result in parsed {
            if !result
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
        Ok(best
            .and_then(|best| {
                let node = best
                    .children
                    .iter()
                    .find(|child| child.ty == TreeType::Node)?;
                let mut extra = (**node).clone();
                extra.trivia = true;
                Some(Extra::Node(Rc::new(extra), best.end))
            })
            .unwrap_or(Extra::Leaf))
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

    /// The start of a token rule's own token where the trivia before it
    /// holds an extra token of that rule: a tree-sitter parser takes a token
    /// as an extra only in a parse state with no action on it, so a rule that
    /// names the extra takes it as its token (GraphQL's `comma`, an extra
    /// that ends a `variable_definition` and an `object_field`).
    pub(super) fn own_extra(
        &mut self,
        rule: &Rule,
        skipped: Rc<Skipped>,
        state: &State,
        in_token: bool,
    ) -> Run<Rc<Skipped>> {
        if self.lexing.is_none() || in_token {
            return Ok(skipped);
        }
        let own = |leaf: &Rc<Tree>| {
            leaf.ty == TreeType::Token
                && leaf
                    .kind
                    .as_deref()
                    .and_then(|kind| self.program.rule_index.get(kind))
                    .is_some_and(|&index| std::ptr::eq(&raw const self.program.rules[index], rule))
        };
        match skipped.leaves.iter().position(own) {
            Some(at) => Ok(Rc::new(Skipped {
                end: skipped.leaves[at].start,
                leaves: children_of(skipped.leaves[..at].to_vec()),
            })),
            // A token rule that matches a separator before it takes it, as a
            // token does (see `token_before_extra`): Make's `raw_line` of a
            // define directive keeps its indentation.
            None if rule.kind == RuleKind::Token && !skipped.leaves.is_empty() => {
                self.token_before_extra(&rule.expression, skipped, state)
            }
            None => Ok(skipped),
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

    /// Under `(matching longest)`, whether a lexer lexes another token than
    /// the literal `text` over `[start, end)`: tree-sitter merges the lex
    /// states of parse states whose tokens do not conflict, so a state's lexer
    /// also lexes tokens no item of the state takes, and the longest token
    /// wins. A token rule merges with a literal when it never matches the
    /// literal's text and no token that may follow the literal anywhere in
    /// the grammar, nor a separator, begins with the input it matches past
    /// the literal. So in TypeScript's `0 .9` the number `.9` is lexed after
    /// `0`, where only a member access `.` is valid, as no property name
    /// begins with `9`; Lean's projection `.1` keeps its `.`, a number
    /// following it. It mirrors mergedLonger in
    /// js/src/grammar-runtime/executor.js.
    fn merged_longer(&mut self, text: &[u8], start: usize, end: usize, state: &State) -> Run<bool> {
        let program = self.program;
        let merged = program
            .grammar
            .merged_lexing(&program.rules, &program.trivia);
        let mut longer = false;
        for &index in &merged.tokens {
            let expression = &program.rules[index].expression;
            let ends: Vec<usize> = self
                .quietly(|this| this.evaluate(expression, start, state, true))?
                .iter()
                .map(|result| result.end)
                .collect();
            if ends.iter().any(|&reach| reach > end) && !ends.contains(&end) {
                longer = true;
                break;
            }
        }
        if !longer || self.skip_trivia(end, state)?.end > end {
            return Ok(false);
        }
        for key in merged.follow.get(text).into_iter().flatten() {
            match key {
                Lead::Literal(literal) => {
                    if literal
                        .first()
                        .is_some_and(|byte| self.bytes.get(end) == Some(byte))
                    {
                        return Ok(false);
                    }
                }
                Lead::Ref(name) => {
                    if program.external.contains_key(&**name) {
                        return Ok(false);
                    }
                    for item in self.aliased_tokens(name) {
                        if longest_result(
                            self.quietly(|this| this.evaluate(item, end, state, true))?,
                        )
                        .is_some_and(|result| result.end > end)
                        {
                            return Ok(false);
                        }
                    }
                    if let Some(&index) = program.rule_index.get(&**name)
                        && matches!(
                            program.rules[index].kind,
                            RuleKind::Token | RuleKind::Atomic
                        )
                        && longest_result(self.quietly(|this| {
                            this.reference(&Target::Rule(index), end, state, true)
                        })?)
                        .is_some_and(|result| result.end > end)
                    {
                        return Ok(false);
                    }
                }
            }
        }
        Ok(true)
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
        let mut found = found
            .or_else(|| matcher.matches(self.bytes, start, self.end))
            .filter(|end| {
                !plain
                    || !self
                        .keywords
                        .is_some_and(|keywords| keywords.borrow().immediate_only((start, *end)))
            });
        if let (Some(end), false, Matcher::Literal(literal)) = (found, in_token, matcher)
            && self.lexing.is_some()
            && self.merged_longer(literal, start, end, state)?
        {
            found = None;
        }
        let Some(end) = found else {
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
            let (from, count) = self.joined_start(start, leaves);
            let kind = self.joined_kind(separator_run_kind(matcher, start, end), from, start, end);
            let mut leaf = Tree::new(TreeType::Token, kind, from, end);
            leaf.plain = plain;
            with_leaf(&leaves[..count], leaf)
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
    // None when no such separator precedes the terminal. A token of an extra
    // of a silent rule that builds no node (INI's newline `_blank`) is taken
    // over the same way, but only once: tree-sitter shifts a valid token
    // before it reduces the same token to an extra.
    pub(super) fn before_separator(
        &self,
        matcher: &Matcher,
        leaves: &[Rc<Tree>],
    ) -> Option<(usize, usize, usize)> {
        let reaches = |leaf: &Tree| {
            matcher
                .matches(self.bytes, leaf.start, self.end)
                .is_some_and(|end| end >= leaf.end)
        };
        let covers =
            |leaf: &Tree, from: usize| leaf.kind.is_none() && leaf.start == from && reaches(leaf);
        let at = leaves.iter().position(|leaf| {
            covers(leaf, leaf.start) || (self.silent_extra(leaf) && reaches(leaf))
        })?;
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

    /// Whether `leaf` is the token of an extra of a silent rule.
    fn silent_extra(&self, leaf: &Tree) -> bool {
        leaf.ty == TreeType::Token
            && leaf.trivia
            && leaf
                .kind
                .as_ref()
                .and_then(|kind| self.program.rule_index.get(&**kind))
                .is_some_and(|&index| matches!(self.program.rules[index].kind, RuleKind::Silent))
    }

    /// The start of a token under `(matching longest)` after `skipped`, the
    /// trivia before its end: at the first separator or extra of a silent
    /// rule (see `before_separator`) whose text the token's item also
    /// matches, at least as far, and without the trivia from it on (CSV's row
    /// ends with a `\n` token where `\s` is trivia); otherwise `skipped`. An
    /// extra that is a token rule is taken over too where the token wins the
    /// lexical conflict with it, lexing longer at no lower precedence or matching a
    /// nonempty token at a higher one: Make's `raw_line` of a define directive,
    /// `#comment\n`, is no comment of a lower precedence.
    pub(super) fn token_before_extra(
        &mut self,
        item: &Expr,
        skipped: Rc<Skipped>,
        state: &State,
    ) -> Run<Rc<Skipped>> {
        let level = self.priority_of(item);
        for (at, leaf) in skipped.leaves.iter().enumerate() {
            let plain = is_separator(leaf) || self.silent_extra(leaf);
            let other = if plain || leaf.ty != TreeType::Token || !leaf.trivia {
                None
            } else {
                leaf.kind
                    .as_ref()
                    .and_then(|kind| self.program.rule_index.get(&**kind))
                    .map(|&index| &self.program.rules[index])
                    .filter(|rule| rule.kind == RuleKind::Token)
                    .map(|rule| rule.lexical_priority)
            };
            if !plain && other.is_none() {
                continue;
            }
            let reach =
                longest_result(self.quietly(|this| this.evaluate(item, leaf.start, state, true))?)
                    .map(|result| result.end);
            let takes = match (plain, other, reach) {
                (true, _, Some(reach)) => reach >= leaf.end,
                (false, Some(other), Some(reach)) => {
                    (reach > leaf.end && level >= other) || (reach > leaf.start && level > other)
                }
                _ => false,
            };
            if !takes {
                continue;
            }
            if is_separator(leaf) && reach.is_some_and(|reach| reach > skipped.end) {
                let chain: Vec<(Name, usize)> = self
                    .call_stack
                    .iter()
                    .filter_map(|entry| {
                        let entry = entry.borrow();
                        entry.builds.clone().map(|kind| (kind, entry.position))
                    })
                    .collect();
                self.separator_taken
                    .entry(leaf.start)
                    .or_default()
                    .insert(chain);
            }
            self.record_joins(item, &skipped.leaves[..at], state)?;
            return Ok(Rc::new(Skipped {
                end: leaf.start,
                leaves: children_of(skipped.leaves[..at].to_vec()),
            }));
        }
        self.record_joins(item, &skipped.leaves, state)?;
        Ok(skipped)
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
        // A wrapped scanner keeps its skipped trivia outside its token.
        if !in_token && let Expr::Ref(Target::External(name)) = item {
            let mut results = self.scanner_token(name, start, state, false, true)?;
            for result in &mut results {
                let children: Vec<_> = result
                    .children
                    .iter()
                    .map(|child| {
                        if child.scanned {
                            let mut leaf = (**child).clone();
                            leaf.kind = None;
                            Rc::new(leaf)
                        } else {
                            Rc::clone(child)
                        }
                    })
                    .collect();
                result.children = concat(&skipped.leaves, &children);
            }
            return Ok(results);
        }
        let Some(best) = longest_result(self.evaluate(item, start, state, true)?) else {
            return Ok(Vec::new());
        };
        let children = if in_token {
            no_children()
        } else {
            let (from, count) = self.joined_start(start, &skipped.leaves);
            let kind = self.joined_kind(None, from, start, best.end);
            let mut leaf = Tree::new(TreeType::Token, kind, from, best.end);
            if let Expr::LexicalPrecedence { level, .. } = item {
                leaf.priority = Some(*level);
            }
            // A token of an external scanner is marked, whatever an alias
            // names it (see `preferred_tokens`).
            leaf.scanned = matches!(item, Expr::Ref(Target::External(_)));
            with_leaf(&skipped.leaves[..count], leaf)
        };
        Ok(vec![Res::new(best.end, best.state, children, best.dynamic)])
    }
}

impl<'c> Executor<'c> {
    /// The tokens an alias of no rule names `name` (Make's `unnamed_token`,
    /// the alias of its inline tokens of text and blanks), any of which
    /// follows where the name does (see `merged_longer`), as `aliased` of
    /// mergedLexing in js/src/grammar-runtime/executor.js.
    fn aliased_tokens(&mut self, name: &Name) -> Vec<&'c Expr> {
        let program: &'c Program = self.program;
        self.aliased
            .get_or_insert_with(|| {
                let mut aliased = HashMap::new();
                for rule in &program.rules {
                    if !matches!(rule.kind, RuleKind::Token | RuleKind::Atomic) {
                        aliased_items(&rule.expression, program, &mut aliased);
                    }
                }
                aliased
            })
            .get(name)
            .cloned()
            .unwrap_or_default()
    }
}

/// Adds the item of each token an alias of no rule names in `expr` to
/// `aliased`, by the alias name.
fn aliased_items<'c>(
    expr: &'c Expr,
    program: &Program,
    aliased: &mut HashMap<Name, Vec<&'c Expr>>,
) {
    match expr {
        Expr::Alias { name, item } => {
            if !program.rule_index.contains_key(&**name)
                && let Expr::Token(inner) | Expr::ImmediateToken(inner) = &**item
            {
                aliased.entry(name.clone()).or_default().push(&**inner);
            }
            aliased_items(item, program, aliased);
        }
        Expr::Seq(items) | Expr::Choice { items, .. } | Expr::Longest(items) => {
            for item in items {
                aliased_items(item, program, aliased);
            }
        }
        Expr::Repeat { item, .. }
        | Expr::And(item)
        | Expr::Not(item)
        | Expr::Capture { item, .. }
        | Expr::Precedence { item, .. }
        | Expr::DynamicPrecedence { item, .. }
        | Expr::LexicalPrecedence { item, .. }
        | Expr::Token(item)
        | Expr::ImmediateToken(item)
        | Expr::Predicate { item, .. }
        | Expr::Missing { item, .. }
        | Expr::Embed { item, .. }
        | Expr::Recover { item, .. } => aliased_items(item, program, aliased),
        Expr::Empty | Expr::Terminal { .. } | Expr::Ref(_) => {}
    }
}
