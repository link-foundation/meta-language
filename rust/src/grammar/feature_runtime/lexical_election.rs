//! Bounded matcher and operation transport for the generated lexical policy.
//! Source rule order is projected unchanged; winner, mode, channel and token
//! decisions are calls to the complete self-translated JavaScript kernels.

use std::rc::Rc;

use serde_json::json;

use super::executor::{Element, Executor, Run};
use super::lexical_kernel as kernel;
use super::lexical_plan::LexicalPlan;
use super::load::GrammarRuntimeError;
use super::operations::{Abort, Machine, OpError, OperationValue, State, Working, run_statements};
use super::program::{Expr, Matcher, Name, Target};
use super::results::{Res, Skipped, Tree, TreeType, children_of, longest_result};
use super::text::decode_at;

#[derive(Clone, Debug)]
pub(super) struct LexicalWinner {
    pub(super) rule: usize,
    pub(super) end: usize,
    pub(super) state: State,
}

struct CommandMachine<'a, 'c> {
    executor: &'a Executor<'c>,
    working: Working,
}

impl Machine for CommandMachine<'_, '_> {
    fn state(&mut self) -> &mut Working {
        &mut self.working
    }
    fn step(&mut self) -> Result<(), Abort> {
        self.executor.step()
    }
    fn column(&mut self) -> usize {
        0
    }
    fn at_end(&mut self) -> bool {
        false
    }
}

impl<'c> Executor<'c> {
    pub(super) fn lexical_reference(
        &mut self,
        target: &Target,
        position: usize,
        state: &State,
        in_token: bool,
    ) -> Run<Option<Vec<Res>>> {
        if in_token || self.lexer_family.is_empty() {
            return Ok(None);
        }
        let program = self.program;
        let plan = &program.lexers[&self.lexer_family];
        let name = match target {
            Target::Rule(index) => &program.rules[*index].name,
            Target::External(name) => name,
        };
        if let Some((source, _)) = plan.bindings.iter().find(|(_, public)| public == name) {
            return self
                .elected_terminal(plan, Some(source), position, state, Some(name.clone()))
                .map(Some);
        }
        Ok(None)
    }

    pub(super) fn evaluate_lexical_scope(
        &mut self,
        name: &Name,
        owner: &Name,
        item: &Expr,
        position: usize,
        state: &State,
        in_token: bool,
    ) -> Run<Vec<Res>> {
        let previous = std::mem::replace(&mut self.lexer_family, name.clone());
        let program = self.program;
        let unresolved = program
            .lexers
            .get(name)
            .filter(|_| !owner.is_empty())
            .and_then(|plan| {
                plan.obligations
                    .iter()
                    .find(|entry| entry.name.as_str() == owner.as_ref())
                    .map(|entry| (plan.name.clone(), entry.clone()))
            });
        let result = if let Some((plan, entry)) = unresolved {
            self.lexical_error(format!(
                "unresolved {} in {plan}:{owner}: {}",
                entry.category, entry.source
            ))
        } else {
            self.evaluate(item, position, state, in_token)
                .map(|mut results| {
                    for result in &mut results {
                        result.lexical_family = Some(name.clone());
                    }
                    results
                })
        };
        self.lexer_family = previous;
        result
    }

    pub(super) fn evaluate_lexical_token(
        &mut self,
        name: &Name,
        token: &Name,
        position: usize,
        state: &State,
    ) -> Run<Vec<Res>> {
        let program = self.program;
        let plan = &program.lexers[name];
        let kind = plan
            .bindings
            .iter()
            .find(|(source, _)| source == token)
            .map_or_else(|| token.clone(), |(_, public)| public.clone());
        self.elected_terminal(plan, Some(token), position, state, Some(kind))
    }

    pub(super) fn lexical_error<T>(&self, message: impl Into<String>) -> Run<T> {
        *self.shared.runtime_error.borrow_mut() = Some(GrammarRuntimeError {
            reason: "operation",
            message: message.into(),
        });
        Err(Abort::Operation)
    }

    pub(super) fn elected_winner(
        &mut self,
        plan: &'c LexicalPlan,
        position: usize,
        state: &State,
    ) -> Run<Option<Rc<LexicalWinner>>> {
        let key = (plan.name.clone(), position, state.clone());
        if let Some(winner) = self.lexer_winners.get(&key) {
            return Ok(winner.clone());
        }
        let stack_name = Name::from(format!("lexer:{}:modes", plan.name));
        let stack = state
            .0
            .stacks
            .get(&stack_name)
            .cloned()
            .unwrap_or_else(|| vec![OperationValue::Text("default".to_owned())]);
        let mode = match stack.last() {
            Some(OperationValue::Text(mode)) => mode.as_str(),
            _ => "",
        };
        if !plan.rules.is_empty() && !self.lexer_workers.contains_key(&plan.name) {
            let worker = Self::new(
                self.compiled,
                plan.program,
                self.bytes,
                self.begin,
                self.end,
                self.shared,
                self.max_depth.saturating_sub(self.depth),
            );
            self.lexer_workers
                .insert(plan.name.clone(), Box::new(worker));
        }
        let (mut ends, mut eligible) = (Vec::new(), Vec::new());
        for rule in &plan.rules {
            let worker = self
                .lexer_workers
                .get_mut(&plan.name)
                .expect("the isolated matcher worker was retained");
            let result = if kernel::active_mode(&rule.mode, mode) {
                let results = worker.quietly(|worker| {
                    worker.reference(&Target::Rule(rule.matcher), position, state, true)
                })?;
                longest_result(results)
            } else {
                None
            };
            ends.push(
                result
                    .as_ref()
                    .map_or(-1.0, |result| kernel::number(result.end)),
            );
            eligible.push(result.as_ref().is_some_and(|result| result.end > position));
        }
        let selected = kernel::winner(&ends, &eligible);
        if selected < 0.0 {
            self.retain(1)?;
            self.lexer_winners.insert(key, None);
            return Ok(None);
        }
        let index = kernel::position(selected);
        let rule = &plan.rules[index];
        if let Some(unresolved) = rule.obligations.first() {
            return self.lexical_error(format!(
                "unresolved {} in {}:{}: {}",
                unresolved.category, plan.name, rule.name, unresolved.source
            ));
        }
        let mut working = state.working();
        let native_modes = working.modes;
        let Some(modes) = stack
            .iter()
            .map(|value| match value {
                OperationValue::Text(mode) => Some(Name::from(mode.as_str())),
                OperationValue::Integer(_) => None,
            })
            .collect::<Option<Vec<_>>>()
        else {
            return self.lexical_error(format!(
                "invalid source-family mode stack in lexer {}",
                plan.name
            ));
        };
        working.modes = modes;
        let mut machine = CommandMachine {
            executor: self,
            working,
        };
        match run_statements(&rule.action, &mut machine) {
            Ok(_) => {}
            Err(OpError::Abort(abort)) => return Err(abort),
            Err(OpError::Failed) => {
                return self.lexical_error(format!(
                    "failed lexer command in {}:{}",
                    plan.name, rule.name
                ));
            }
        }
        let mut working = machine.working;
        working.stacks.insert(
            stack_name,
            working
                .modes
                .iter()
                .map(|mode| OperationValue::Text(mode.to_string()))
                .collect(),
        );
        working.modes = native_modes;
        let winner = Rc::new(LexicalWinner {
            rule: index,
            end: kernel::position(ends[index]),
            state: working.settle(),
        });
        self.retain(1)?;
        self.lexer_winners.insert(key, Some(winner.clone()));
        Ok(Some(winner))
    }

    fn elected_leaf(
        plan: &LexicalPlan,
        winner: &LexicalWinner,
        start: usize,
        kind: Option<Name>,
        trivia: bool,
    ) -> Rc<Tree> {
        let rule = &plan.rules[winner.rule];
        let mut tree = Tree::new(TreeType::Token, kind, start, winner.end);
        tree.trivia = trivia;
        tree.lexical = Some(
            json!({ "lexer": plan.name, "rule": rule.name, "type": rule.token_type, "channel": rule.channel, "disposition": rule.disposition }),
        );
        Rc::new(tree)
    }

    pub(super) fn skip_elected_trivia(
        &mut self,
        plan: &'c LexicalPlan,
        position: usize,
        state: &State,
    ) -> Run<Rc<Skipped>> {
        let mut leaves = Vec::new();
        let mut states = vec![state.clone()];
        let mut cursor = position;
        let mut current = state.clone();
        while let Some(winner) = self.elected_winner(plan, cursor, &current)? {
            let rule = &plan.rules[winner.rule];
            if !kernel::trivia(&rule.channel, &rule.disposition) {
                break;
            }
            let identity = kernel::trivia_identity(&rule.name, &rule.token_type, &rule.disposition);
            let kind = plan
                .bindings
                .iter()
                .find(|(source, _)| &**source == identity)
                .map_or_else(|| Name::from(identity), |(_, public)| public.clone());
            leaves.push(Self::elected_leaf(plan, &winner, cursor, Some(kind), true));
            cursor = winner.end;
            current = winner.state.clone();
            states.push(current.clone());
        }
        Ok(Rc::new(Skipped {
            end: cursor,
            leaves: children_of(leaves),
            state: current,
            states,
        }))
    }

    pub(super) fn elected_terminal(
        &mut self,
        plan: &'c LexicalPlan,
        expected: Option<&str>,
        position: usize,
        state: &State,
        kind: Option<Name>,
    ) -> Run<Vec<Res>> {
        let skipped = self.skip_elected_trivia(plan, position, state)?;
        if let Some(winner) = self.elected_winner(plan, skipped.end, &skipped.state)? {
            let rule = &plan.rules[winner.rule];
            if kernel::expected_token(
                expected.unwrap_or_default(),
                &rule.token_type,
                expected.is_none(),
            ) {
                let selected = kind
                    .clone()
                    .or_else(|| {
                        plan.bindings
                            .iter()
                            .find(|(source, _)| source == &rule.token_type)
                            .map(|(_, public)| public.clone())
                    })
                    .or_else(|| Some(rule.token_type.clone()));
                let children = super::results::with_leaf(
                    &skipped.leaves,
                    Self::elected_leaf(plan, &winner, skipped.end, selected, false),
                );
                return Ok(vec![Res::new(
                    winner.end,
                    winner.state.clone(),
                    children,
                    0,
                )]);
            }
        }
        let expectation = Name::from(expected.unwrap_or("token"));
        self.fail(skipped.end, &expectation);
        let mut boundary = skipped.end;
        let mut boundary_state = skipped.state.clone();
        let key = (plan.name.clone(), expectation.clone(), kind.clone());
        if !self.lexer_terminals.contains_key(&key) {
            self.retain(1)?;
        }
        let next = self.lexer_terminals.len();
        let identity = *self.lexer_terminals.entry(key).or_insert(next);
        let missing_kind = kind.clone().or_else(|| Some(expectation.clone()));
        let mut retry = |this: &mut Self, cursor| {
            while boundary < cursor {
                if let Some(token) = this.elected_winner(plan, boundary, &boundary_state)? {
                    boundary = token.end;
                    boundary_state = token.state.clone();
                } else {
                    boundary += decode_at(this.bytes, boundary, this.end).1;
                }
            }
            if boundary == cursor {
                this.elected_terminal(plan, expected, cursor, &boundary_state, kind.clone())
            } else {
                Ok(Vec::new())
            }
        };
        self.element_failed(
            skipped.end,
            &skipped.leaves,
            &skipped.state,
            missing_kind,
            false,
            Element::Lexical(identity),
            Some(&mut retry),
        )
    }

    pub(super) fn lexical_terminal(
        &mut self,
        matcher: &Matcher,
        position: usize,
        state: &State,
    ) -> Run<Vec<Res>> {
        let program = self.program;
        let plan = &program.lexers[&self.lexer_family];
        match matcher {
            Matcher::Literal(value) => {
                let Some(expected) = plan.literals.get(value) else {
                    return self
                        .lexical_error(format!("unbound parser literal in lexer {}", plan.name));
                };
                self.elected_terminal(
                    plan,
                    Some(expected),
                    position,
                    state,
                    Some(Name::from(String::from_utf8_lossy(value))),
                )
            }
            Matcher::Any => self.elected_terminal(plan, None, position, state, None),
            _ => self.lexical_error(format!(
                "unsupported parser terminal in lexer {}",
                plan.name
            )),
        }
    }
}
