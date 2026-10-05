//! Repetitions, aliases and lexical longest matches of the native executor,
//! as `js/src/grammar-runtime/executor.js` evaluates them.

use std::rc::Rc;

use super::executor::{Element, Executor, Run};
use super::forking::reduced_early;
use super::operations::State;
use super::parting::starts_widthless;
use super::program::{Expr, Name, Target};
use super::results::{
    Res, ResultSet, Tree, TreeType, children_of, longest_result, no_children, with_leaf,
};
use crate::grammar::RuleKind;

impl Executor<'_> {
    #[allow(clippy::too_many_arguments)]
    pub(super) fn repetition(
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
        // not extended again (it is the same continuation) but marks ambiguity,
        // unless it replaces the result reached before: then the continuations
        // of the replaced one are replaced too, by extending it.
        let mut results = ResultSet::new(self.longest_tokens, self.settling);
        // A rule reduced before a token the next iteration may begin with
        // ends no iteration (see `reduced_early`).
        let rest = self
            .reductions(in_token)
            .and_then(|reductions| self.iteration_keys(reductions, item));
        let mut frontier = vec![Res::new(position, state.clone(), no_children(), 0)];
        let mut count = 0;
        while !frontier.is_empty() {
            if count >= min {
                let mut fresh = Vec::new();
                for result in frontier {
                    if results.add(result.clone()) {
                        fresh.push(result);
                    }
                }
                frontier = fresh;
            }
            if !below_max(count) {
                break;
            }
            let mut next = ResultSet::new(self.longest_tokens, self.settling);
            let continued = self.continued(item, &frontier, in_token)?;
            let pruned = self.preempted(&continued, in_token);
            for ((left, rights), pruned) in continued.into_iter().zip(pruned) {
                // A result an iteration goes on from with a token the
                // external scanner scanned of no width is no parse itself
                // (the optional layout end after `def foo := 12` is taken
                // where the scanner scans it, as the token is the
                // lookahead), and neither is one that token preempts.
                if pruned
                    || self.scans_widthless(in_token)
                        && rights.iter().any(|right| starts_widthless(&right.children))
                {
                    results.remove(left);
                }
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
                    if zero_width(left, &right) {
                        // Zero-width iterations can pad up to the minimum
                        // once; one that takes a token the external scanner
                        // scanned of no width (Lean's layout semicolon between
                        // two structure fields) is a result too, as the parser
                        // shifts that token, though it is not extended again.
                        if count < min || right.children.iter().any(|child| child.scanned) {
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

    pub(super) fn alias(
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
        // An alias of a silent rule names the node the rule does not build, as a
        // tree-sitter alias of a hidden rule does, even around a single child.
        let wraps = matches!(item, Expr::Ref(Target::Rule(index)) if matches!(self.program.rules[*index].kind, RuleKind::Silent));
        Ok(results
            .into_iter()
            .map(|result| {
                let meaningful = result.children.iter().filter(|child| !child.trivia).count();
                if meaningful == 1 && !wraps {
                    let children = result
                        .children
                        .iter()
                        .map(|child| {
                            if child.trivia {
                                child.clone()
                            } else {
                                // A MISSING literal named by an alias is no longer a literal.
                                let mut copy = (**child).clone();
                                copy.kind = Some(name.clone());
                                if copy.ty == TreeType::Missing {
                                    copy.literal = false;
                                }
                                Rc::new(copy)
                            }
                        })
                        .collect();
                    return Res {
                        children: children_of(children),
                        ..result
                    };
                }
                let mut node = Tree::node(name, position, result.end, result.children.clone());
                node.ambiguous = result.ambiguous;
                Res {
                    children: children_of(vec![Rc::new(node)]),
                    ambiguous: false,
                    ..result
                }
            })
            .collect())
    }

    // Lexical longest match: the alternative with the longest match wins, a
    // tie goes to the higher lexical priority and then to the first.
    pub(super) fn longest(
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
            if in_token {
                return Ok(Vec::new());
            }
            return self.element_failed(
                start,
                &skipped.leaves,
                state,
                None,
                false,
                Element::of(items),
                Some(&mut |this, cursor| this.longest(items, cursor, state, false)),
            );
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
}
