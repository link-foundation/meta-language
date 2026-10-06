//! Keyword lexing of the native executor: the reparse rounds that make a
//! span keyword-only, immediate-only or a lexed separator, as a tree-sitter
//! lexer lexes it in the parse state the tree had there.

use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

use super::forking::{KeywordLeads, Lead};
use super::program::{Name, Rule};
use super::results::{TokenOrder, Tree, TreeType};
use super::token_conflicts::token_conflict;

/// Keyword lexing under `(matching longest)`. A tree-sitter lexer lexes a
/// keyword wherever the parse state admits it, before any parse goes on, so a
/// token rule's leaf over the same text (an identifier `typedef`) is not taken
/// there even when only it would let the parse go on. A parse records the
/// spans where a keyword token matched (`matched`); a leaf of a token rule in
/// its tree over such a span (`lexed`, the rule's kind, which an alias keeps)
/// that the keyword outranks (see `token_conflict`) makes the span
/// keyword-only (`only`), and the input is parsed again, where no token rule
/// takes a keyword-only span the keyword outranks it on. The spans only grow,
/// so the reparses end.
///
/// The keyword counts only where it matched in the tree's parse state: in a
/// call made, through some chain of calls up to the first, by calls that
/// build nodes each beginning a node of the tree that goes on past the span,
/// as the items a parser has in progress there. A chain through a call that
/// began no such node lexed the text before the span otherwise (Rust's
/// `m!('"')`, whose token tree also takes `'` alone and a string to the next
/// `"`, where a later `_` type is a keyword `_` token): the tree's parse never
/// reached that state. A call's result is shared by every call that made it,
/// so every chain counts. The calls are kept in `calls`, their makers by
/// index.
///
/// An immediate token a literal (`(immediateToken (literal [))`) outranks
/// the plain literal of the same text, which a lexer so lexes only where no
/// immediate one is valid (Lean's `foo[1:2:3]`, whose `[` opens a subscript,
/// not a range applied to `foo`): the spans where an immediate literal
/// matched (`matched_immediate`) become immediate-only (`immediates`) alike
/// where the tree took the plain literal (a `plain` leaf), whose parse state
/// the immediate one matched in.
///
/// An immediate token that matched separator text alone at an offset
/// (`matched_separators`, Make's line break that ends a rule) is lexed there
/// where the tree skipped a separator in its parse state: a tree-sitter lexer
/// that completed it takes no separator transition after it, so the offset
/// skips no separator (`separators`) when the input is parsed again (Make's
/// `a:\nb\n`, whose rule ends at the line break, with no `b` prerequisite).
///
/// A separator a token begins with, though it does not match on to the
/// token's text (`matched_joins`, with how far the token reads on), is read
/// on with where some call that matched it was in the tree's parse state
/// (`joined`, see `joined_start`).
/// It mirrors `KeywordLexing` in js/src/grammar-runtime/executor.js.
#[derive(Debug, Default)]
pub(super) struct KeywordLexing {
    only: HashSet<(usize, usize)>,
    matched: HashMap<(usize, usize), HashSet<Option<usize>>>,
    immediates: HashSet<(usize, usize)>,
    matched_immediate: HashMap<(usize, usize), HashSet<Option<usize>>>,
    separators: HashSet<usize>,
    matched_separators: HashMap<usize, HashSet<Option<usize>>>,
    joined: HashMap<usize, usize>,
    matched_joins: HashMap<usize, (usize, HashSet<Option<usize>>)>,
    calls: Vec<Call>,
}

/// One rule call of a parse under keyword lexing: where it began, whether it
/// builds a node, the kind of the node it builds, the rule's name and the
/// calls it was made from (`None` for none).
#[derive(Debug)]
struct Call {
    position: usize,
    builds: bool,
    rule: Name,
    name: Name,
    parents: HashSet<Option<usize>>,
}

impl KeywordLexing {
    /// Records a rule call beginning at `position` made from `parent`; its index.
    pub(super) fn call(
        &mut self,
        position: usize,
        builds: bool,
        rule: Name,
        name: Name,
        parent: Option<usize>,
    ) -> usize {
        self.calls.push(Call {
            position,
            builds,
            rule,
            name,
            parents: HashSet::from([parent]),
        });
        self.calls.len() - 1
    }

    /// Records that the call `call` was made again, from `parent`.
    pub(super) fn called(&mut self, call: usize, parent: Option<usize>) {
        self.calls[call].parents.insert(parent);
    }

    /// Records a keyword or `immediate` literal token matched over `span` in
    /// the rule call `call`.
    pub(super) fn matched(&mut self, span: (usize, usize), call: Option<usize>, immediate: bool) {
        let matched = if immediate {
            &mut self.matched_immediate
        } else {
            &mut self.matched
        };
        matched.entry(span).or_default().insert(call);
    }

    /// Records an immediate token of separator text alone matched at
    /// `position` in the rule call `call`.
    pub(super) fn matched_separator(&mut self, position: usize, call: Option<usize>) {
        self.matched_separators
            .entry(position)
            .or_default()
            .insert(call);
    }

    /// Records a separator at `position` a token in the rule call `call`
    /// begins with, read on with up to `reach`.
    pub(super) fn match_join(&mut self, position: usize, reach: usize, call: Option<usize>) {
        let join = self
            .matched_joins
            .entry(position)
            .or_insert_with(|| (reach, HashSet::new()));
        join.0 = join.0.max(reach);
        join.1.insert(call);
    }

    /// Whether any separator is read on with (see `joined`).
    pub(super) fn has_joins(&self) -> bool {
        !self.joined.is_empty()
    }

    /// How far a token reads on with the separator at `position`, if it does.
    pub(super) fn joined(&self, position: usize) -> Option<usize> {
        self.joined.get(&position).copied()
    }

    /// Whether an immediate token takes the separator at `position` in, so
    /// the offset skips no separator.
    pub(super) fn lexes_separator(&self, position: usize) -> bool {
        self.separators.contains(&position)
    }

    /// Whether an immediate token outranks the plain literal over `span`.
    pub(super) fn immediate_only(&self, span: (usize, usize)) -> bool {
        self.immediates.contains(&span)
    }

    /// Whether a keyword-only span's keyword outranks a token rule's leaf over it.
    pub(super) fn outranks(&self, leaf: &Tree, tokens: TokenOrder<'_>) -> bool {
        self.only.contains(&(leaf.start, leaf.end)) && outranks_at(leaf, tokens)
    }

    /// Marks the spans where `root` took a token rule's leaf over a keyword;
    /// true when one is new. `rules` are the program's.
    pub(super) fn conflicts(
        &mut self,
        root: &Tree,
        tokens: TokenOrder<'_>,
        rules: &[Rule],
    ) -> bool {
        let mut found = false;
        let mut reach = None;
        let mut pending = vec![root];
        while let Some(node) = pending.pop() {
            if node.ty == TreeType::Node {
                pending.extend(node.children.iter().rev().map(|child| &**child));
                continue;
            }
            if node.trivia && node.kind.is_none() {
                if let Some((join, calls)) = self.matched_joins.get(&node.start)
                    && !self.joined.contains_key(&node.start)
                {
                    let leads = tokens.grammar.keyword_leads(rules);
                    let reach =
                        reach.get_or_insert_with(|| TreeReach::of(root, leads, tokens.bytes));
                    let mut seen = HashSet::new();
                    if calls.iter().any(|call| {
                        self.built_around(*call, node, reach)
                            && self.in_parse_state(*call, node, reach, &mut seen, leads)
                    }) {
                        self.joined.insert(node.start, *join);
                        found = true;
                    }
                }
                let Some(calls) = self.matched_separators.get(&node.start) else {
                    continue;
                };
                if self.separators.contains(&node.start) {
                    continue;
                }
                let leads = tokens.grammar.keyword_leads(rules);
                let reach = reach.get_or_insert_with(|| TreeReach::of(root, leads, tokens.bytes));
                let mut seen = HashSet::new();
                if !calls.iter().any(|call| {
                    self.built_around(*call, node, reach)
                        && self.in_parse_state(*call, node, reach, &mut seen, leads)
                }) {
                    continue;
                }
                self.separators.insert(node.start);
                found = true;
                continue;
            }
            let span = (node.start, node.end);
            let (matched, only) = if node.plain {
                (&self.matched_immediate, &self.immediates)
            } else if node.lexed.is_some() {
                (&self.matched, &self.only)
            } else {
                continue;
            };
            let Some(calls) = matched.get(&span) else {
                continue;
            };
            if only.contains(&span) {
                continue;
            }
            if let Some(kind) = node.lexed.as_ref().filter(|_| !node.plain) {
                let leaf = Tree::new(TreeType::Token, Some(kind.clone()), node.start, node.end);
                if !outranks_at(&leaf, tokens) {
                    continue;
                }
            }
            let leads = tokens.grammar.keyword_leads(rules);
            let reach = reach.get_or_insert_with(|| TreeReach::of(root, leads, tokens.bytes));
            let mut seen = HashSet::new();
            if !calls
                .iter()
                .any(|call| self.in_parse_state(*call, node, reach, &mut seen, leads))
            {
                continue;
            }
            if node.plain {
                self.immediates.insert(span);
            } else {
                self.only.insert(span);
            }
            found = true;
        }
        self.matched.clear();
        self.matched_immediate.clear();
        self.matched_separators.clear();
        self.matched_joins.clear();
        self.calls.clear();
        found
    }

    /// Whether the node the rule call `call` builds, or the first call that
    /// made it that builds one, through some chain, is a node of the tree
    /// around `leaf`: a call whose node the tree lacks lexed in a parse state
    /// the tree never had (Make's `list` of a target `-include`, where the
    /// tree has the `-include` of a directive).
    fn built_around(&self, call: Option<usize>, leaf: &Tree, reach: &TreeReach) -> bool {
        let mut pending = vec![call];
        let mut seen = HashSet::new();
        while let Some(current) = pending.pop() {
            let Some(current) = current else {
                return true;
            };
            if !seen.insert(current) {
                continue;
            }
            let call = &self.calls[current];
            if call.builds {
                if reach
                    .rules
                    .get(&(call.position, call.rule.clone()))
                    .is_some_and(|end| *end > leaf.start)
                {
                    return true;
                }
                continue;
            }
            pending.extend(call.parents.iter().copied());
        }
        false
    }

    /// Whether a keyword matched in `call` was in the parse state of the
    /// tree's `leaf` over its span: whether some chain of the calls that made
    /// it, up to the first, holds no call that builds a node the tree does
    /// not have in progress there. `seen` holds the calls already searched,
    /// and `leads` the FIRST sets of the rules.
    fn in_parse_state(
        &self,
        call: Option<usize>,
        leaf: &Tree,
        reach: &TreeReach,
        seen: &mut HashSet<usize>,
        leads: &KeywordLeads,
    ) -> bool {
        let mut pending = vec![call];
        while let Some(current) = pending.pop() {
            let Some(current) = current else {
                return true;
            };
            if !seen.insert(current) {
                continue;
            }
            let call = &self.calls[current];
            if call.builds && call.position < leaf.start {
                let low = reach.starts.partition_point(|start| *start < call.position);
                let first = reach.starts.get(low).copied();
                if let Some(first) = first
                    && first < leaf.start
                    && reach.ends.get(&first).is_none_or(|end| *end < leaf.end)
                {
                    continue;
                }
                // A call that begins inside a leaf of the tree lexed that
                // leaf's text otherwise (Rocq's `[` of a list where the tree
                // has the token `=[`).
                if let Some(previous) = low.checked_sub(1).map(|at| reach.starts[at])
                    && reach
                        .last
                        .get(&previous)
                        .is_some_and(|end| *end > call.position)
                {
                    continue;
                }
                // A node of the call's rule that the tree closes before a leaf
                // preceding `leaf` is no longer in progress there: the call
                // matched the keyword on a parse that read that leaf otherwise
                // (Rocq's `match ... end > 0 end`, where the first `end` closes
                // the match and the second is an identifier), or by a token
                // the external scanner scanned of no width before it, which
                // the parser shifted before it lexed `leaf` (TypeScript's
                // automatic semicolon before a line break and an identifier
                // `as`).
                if let Some(&own) = reach.rules.get(&(call.position, call.rule.clone()))
                    && own < leaf.start
                    && (reach
                        .starts
                        .get(reach.starts.partition_point(|start| *start < own))
                        .is_some_and(|start| *start < leaf.start)
                        || reach.widthless.contains(&own))
                {
                    continue;
                }
                // A call whose rule begins with no keyword the tree took where
                // the call began read that keyword's text otherwise, as a
                // token a lexer never lexes where the keyword is valid
                // (TypeScript's identifier `return` before an `as`
                // expression, where the tree's `return as` returns the
                // identifier `as`).
                if let Some(keyword) = first
                    .filter(|first| *first < leaf.start)
                    .and_then(|first| reach.keywords.get(&first))
                    && leads
                        .first
                        .get(&call.name)
                        .is_some_and(|set| !set.contains(&Lead::Literal(keyword.clone())))
                {
                    continue;
                }
            }
            pending.extend(call.parents.iter().copied());
        }
        false
    }
}

/// The starts of the leaves of a tree that are not trivia, in order, and for
/// each the farthest end of a node that begins with that leaf. A token the
/// external scanner scanned of no width does not count: the parser scanned it
/// before its lexer, in the parse state the next leaf is lexed in
/// (JavaScript's automatic semicolon before a line break and a keyword
/// `class`). `last` holds the end of each leaf, by its start, `rules` the
/// farthest end of a node of each rule, by its start and rule, and
/// `keywords` the text of each token leaf of the input `bytes` that is a
/// keyword of the `leads`, by its start. `widthless` holds the offsets of the
/// tokens the external scanner scanned of no width. It mirrors treeReach in
/// js/src/grammar-runtime/executor.js.
struct TreeReach {
    starts: Vec<usize>,
    ends: HashMap<usize, usize>,
    last: HashMap<usize, usize>,
    rules: HashMap<(usize, Name), usize>,
    keywords: HashMap<usize, Vec<u8>>,
    widthless: HashSet<usize>,
}

impl TreeReach {
    fn of(root: &Tree, leads: &KeywordLeads, bytes: &[u8]) -> Self {
        let mut starts = Vec::new();
        let mut ends = HashMap::new();
        let mut last = HashMap::new();
        let mut rules = HashMap::new();
        let mut keywords = HashMap::new();
        let mut widthless = HashSet::new();
        let mut open: Option<usize> = None;
        let mut pending = vec![root];
        while let Some(node) = pending.pop() {
            if node.trivia {
                continue;
            }
            if node.ty != TreeType::Node && node.start == node.end && node.scanned {
                widthless.insert(node.start);
                continue;
            }
            if node.ty == TreeType::Node {
                open = Some(open.map_or(node.end, |end| end.max(node.end)));
                if let Some(rule) = &node.rule {
                    let farthest = rules.entry((node.start, rule.clone())).or_insert(node.end);
                    *farthest = (*farthest).max(node.end);
                }
                pending.extend(node.children.iter().rev().map(|child| &**child));
                continue;
            }
            starts.push(node.start);
            let leaf_end = last.entry(node.start).or_insert(node.end);
            *leaf_end = (*leaf_end).max(node.end);
            if node.ty == TreeType::Token
                && node.kind.is_none()
                && node.end - node.start <= leads.longest
                && let Some(text) = bytes.get(node.start..node.end)
                && leads.keywords.contains(text)
            {
                keywords.insert(node.start, text.to_vec());
            }
            if let Some(end) = open.take() {
                let farthest = ends.entry(node.start).or_insert(end);
                *farthest = (*farthest).max(end);
            }
        }
        Self {
            starts,
            ends,
            last,
            rules,
            keywords,
            widthless,
        }
    }
}

/// Whether a keyword over the span of a token rule's leaf outranks it.
fn outranks_at(leaf: &Tree, tokens: TokenOrder<'_>) -> bool {
    let keyword = Tree::new(TreeType::Token, None, leaf.start, leaf.end);
    token_conflict(&keyword, leaf, tokens) == Ordering::Greater
}
