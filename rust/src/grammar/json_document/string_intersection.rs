use std::collections::{BTreeSet, HashMap};

use super::super::{CharClassItem, ExprBuilder, Grammar, GrammarExpr, GrammarRule};
use super::pattern_policy::string_intersection_limits;

const EXPRESSION: ExprBuilder = ExprBuilder;

struct WorkLimit(usize);

impl WorkLimit {
    fn charge(&mut self, cost: usize) -> Option<()> {
        self.0 = self.0.checked_sub(cost)?;
        Some(())
    }
}

#[derive(Clone, Copy)]
struct Edge {
    first: u32,
    last: u32,
    target: usize,
}

#[derive(Default)]
struct SourceState {
    empty: Vec<usize>,
    edges: Vec<Edge>,
}

struct Automaton<'grammar> {
    grammar: &'grammar Grammar,
    states: Vec<SourceState>,
    state_limit: usize,
}

impl<'grammar> Automaton<'grammar> {
    fn new(
        grammar: &'grammar Grammar,
        expression: &GrammarExpr,
        state_limit: usize,
        work: &mut WorkLimit,
    ) -> Option<Self> {
        let mut automaton = Self {
            grammar,
            states: vec![SourceState::default(), SourceState::default()],
            state_limit,
        };
        automaton.lower(expression, 0, 1, 0, work)?;
        Some(automaton)
    }

    fn state(&mut self) -> Option<usize> {
        let index = self.states.len();
        if index == self.state_limit {
            return None;
        }
        self.states.push(SourceState::default());
        Some(index)
    }

    fn epsilon(&mut self, start: usize, end: usize) {
        self.states[start].empty.push(end);
    }

    fn range(
        &mut self,
        first: char,
        last: char,
        start: usize,
        end: usize,
        work: &mut WorkLimit,
    ) -> Option<()> {
        work.charge(1)?;
        self.states[start].edges.push(Edge {
            first: first.into(),
            last: last.into(),
            target: end,
        });
        Some(())
    }

    fn lower(
        &mut self,
        node: &GrammarExpr,
        mut start: usize,
        end: usize,
        depth: usize,
        work: &mut WorkLimit,
    ) -> Option<()> {
        if depth > 128 {
            return None;
        }
        work.charge(1)?;
        let child_depth = depth + 1;
        match node {
            GrammarExpr::Empty => self.epsilon(start, end),
            GrammarExpr::Terminal(text) => {
                let mut characters = text.chars().peekable();
                if characters.peek().is_none() {
                    self.epsilon(start, end);
                }
                while let Some(character) = characters.next() {
                    let next = if characters.peek().is_none() {
                        end
                    } else {
                        self.state()?
                    };
                    self.range(character, character, start, next, work)?;
                    start = next;
                }
            }
            GrammarExpr::CharRange(first, last) => self.range(*first, *last, start, end, work)?,
            GrammarExpr::CharClass {
                negated: false,
                items,
            } => {
                for item in items {
                    let (first, last) = match *item {
                        CharClassItem::Char(character) => (character, character),
                        CharClassItem::Range(first, last) => (first, last),
                    };
                    self.range(first, last, start, end, work)?;
                }
            }
            GrammarExpr::NonTerminal(name) => {
                let grammar = self.grammar;
                let source = grammar.rule(name)?.expr();
                self.lower(source, start, end, child_depth, work)?;
            }
            GrammarExpr::Sequence(items) => {
                if items.is_empty() {
                    self.epsilon(start, end);
                }
                for (index, item) in items.iter().enumerate() {
                    let next = if index + 1 == items.len() {
                        end
                    } else {
                        self.state()?
                    };
                    self.lower(item, start, next, child_depth, work)?;
                    start = next;
                }
            }
            GrammarExpr::Choice {
                ordered: false,
                alternatives,
            } => {
                for item in alternatives {
                    self.lower(item, start, end, child_depth, work)?;
                }
            }
            _ => {
                let (item, minimum, maximum) = match node {
                    GrammarExpr::Optional(item) => (item, 0, Some(1)),
                    GrammarExpr::ZeroOrMore(item) => (item, 0, None),
                    GrammarExpr::OneOrMore(item) => (item, 1, None),
                    GrammarExpr::Repeat { expr, min, max } => (expr, *min, *max),
                    _ => return None,
                };
                for _ in 0..minimum {
                    let next = self.state()?;
                    self.lower(item, start, next, child_depth, work)?;
                    start = next;
                }
                if let Some(maximum) = maximum {
                    self.epsilon(start, end);
                    for _ in minimum..maximum {
                        let next = self.state()?;
                        self.lower(item, start, next, child_depth, work)?;
                        self.epsilon(next, end);
                        start = next;
                    }
                } else {
                    let repeat_start = self.state()?;
                    let next = self.state()?;
                    self.epsilon(start, repeat_start);
                    self.epsilon(repeat_start, end);
                    self.lower(item, repeat_start, next, child_depth, work)?;
                    self.epsilon(next, repeat_start);
                }
            }
        }
        Some(())
    }

    fn closure(
        &self,
        starts: impl IntoIterator<Item = usize>,
        work: &mut WorkLimit,
    ) -> Option<Vec<usize>> {
        let mut visited: BTreeSet<_> = starts.into_iter().collect();
        let mut pending: Vec<_> = visited.iter().copied().collect();
        let mut index = 0;
        while let Some(&state) = pending.get(index) {
            index += 1;
            work.charge(1 + self.states[state].empty.len())?;
            pending.extend(
                self.states[state]
                    .empty
                    .iter()
                    .copied()
                    .filter(|target| visited.insert(*target)),
            );
        }
        Some(visited.into_iter().collect())
    }

    fn edges(&self, states: &[usize]) -> Vec<Edge> {
        states
            .iter()
            .flat_map(|state| self.states[*state].edges.iter().copied())
            .collect()
    }
}

type ProductState = (Vec<usize>, Vec<usize>);

struct ProductRow {
    states: ProductState,
    accepts: bool,
    edges: Vec<Edge>,
    incoming: Vec<usize>,
}

#[derive(Default)]
struct Product {
    rows: Vec<ProductRow>,
    indices: HashMap<ProductState, usize>,
    state_limit: usize,
}

impl Product {
    fn state(&mut self, states: ProductState) -> Option<usize> {
        if let Some(&index) = self.indices.get(&states) {
            return Some(index);
        }
        let index = self.rows.len();
        if index == self.state_limit {
            return None;
        }
        let accepts = states.0.contains(&1) && states.1.contains(&1);
        self.indices.insert(states.clone(), index);
        self.rows.push(ProductRow {
            states,
            accepts,
            edges: Vec::new(),
            incoming: Vec::new(),
        });
        Some(index)
    }

    fn transition(&mut self, index: usize, first: u32, last: u32, target: usize) {
        let edges = &mut self.rows[index].edges;
        if let Some(previous) = edges
            .last_mut()
            .filter(|edge| edge.target == target && edge.last + 1 == first)
        {
            previous.last = last;
        } else {
            edges.push(Edge {
                first,
                last,
                target,
            });
            self.rows[target].incoming.push(index);
        }
    }

    fn compile(
        self,
        rule_prefix: &str,
        mut accepting: Vec<usize>,
    ) -> Option<(GrammarExpr, Vec<GrammarRule>)> {
        let mut productive: BTreeSet<_> = accepting.iter().copied().collect();
        let mut index = 0;
        while let Some(&state) = accepting.get(index) {
            index += 1;
            accepting.extend(
                self.rows[state]
                    .incoming
                    .iter()
                    .copied()
                    .filter(|previous| productive.insert(*previous)),
            );
        }
        if !productive.contains(&0) {
            return None;
        }
        let name = |index| format!("{rule_prefix}_{index}");
        let mut rules = Vec::new();
        for (index, row) in self.rows.into_iter().enumerate() {
            if !productive.contains(&index) {
                continue;
            }
            let mut alternatives = if row.accepts {
                vec![EXPRESSION.empty()]
            } else {
                Vec::new()
            };
            let mut transitions: Vec<(usize, Vec<CharClassItem>)> = Vec::new();
            let mut indices = HashMap::new();
            for Edge {
                first,
                last,
                target,
            } in row.edges
            {
                if !productive.contains(&target) {
                    continue;
                }
                let first = char::from_u32(first).expect("JSON source scalar boundary");
                let last = char::from_u32(last).expect("JSON source scalar boundary");
                let offset = *indices.entry(target).or_insert_with(|| {
                    let offset = transitions.len();
                    transitions.push((target, Vec::new()));
                    offset
                });
                transitions[offset].1.push(if first == last {
                    CharClassItem::Char(first)
                } else {
                    CharClassItem::Range(first, last)
                });
            }
            alternatives.extend(transitions.into_iter().map(|(target, characters)| {
                EXPRESSION.seq([
                    EXPRESSION.char_class(false, characters),
                    EXPRESSION.nt(name(target)),
                ])
            }));
            rules.push(GrammarRule::new(
                name(index),
                EXPRESSION.choice_unordered(alternatives),
            ));
        }
        Some((EXPRESSION.nt(name(0)), rules))
    }
}

fn targets(edges: &[Edge], point: u32) -> Vec<usize> {
    edges
        .iter()
        .filter(|edge| edge.first <= point && point <= edge.last)
        .map(|edge| edge.target)
        .collect()
}

pub(super) enum StringIntersection {
    Match(GrammarExpr, Vec<GrammarRule>),
    Impossible,
}

/// Build a deterministic product over JSON source characters. None means the work bound was reached.
pub(super) fn intersect_json_string_expressions(
    grammar: &Grammar,
    left_expression: &GrammarExpr,
    right_expression: &GrammarExpr,
    rule_prefix: &str,
) -> Option<StringIntersection> {
    let (maximum_source_states, maximum_product_states, maximum_work) =
        string_intersection_limits();
    let mut work = WorkLimit(maximum_work as usize);
    let left = Automaton::new(
        grammar,
        left_expression,
        maximum_source_states as usize,
        &mut work,
    )?;
    let right = Automaton::new(
        grammar,
        right_expression,
        maximum_source_states as usize,
        &mut work,
    )?;
    let mut product = Product {
        state_limit: maximum_product_states as usize,
        ..Product::default()
    };
    product.state((
        left.closure([0], &mut work)?,
        right.closure([0], &mut work)?,
    ))?;
    let mut accepting = Vec::new();
    let mut index = 0;
    while let Some(row) = product.rows.get(index) {
        if row.accepts {
            accepting.push(index);
        }
        let left_edges = left.edges(&row.states.0);
        let right_edges = right.edges(&row.states.1);
        let bounds: BTreeSet<_> = left_edges
            .iter()
            .chain(&right_edges)
            .flat_map(|edge| [edge.first, edge.last + 1])
            .collect();
        let bounds: Vec<_> = bounds.into_iter().collect();
        for bounds in bounds.windows(2) {
            work.charge(1 + left_edges.len() + right_edges.len())?;
            let first = bounds[0];
            let left_targets = targets(&left_edges, first);
            if left_targets.is_empty() {
                continue;
            }
            let right_targets = targets(&right_edges, first);
            if right_targets.is_empty() {
                continue;
            }
            let target = product.state((
                left.closure(left_targets, &mut work)?,
                right.closure(right_targets, &mut work)?,
            ))?;
            product.transition(index, first, bounds[1] - 1, target);
        }
        index += 1;
    }
    Some(match product.compile(rule_prefix, accepting) {
        Some((expression, rules)) => StringIntersection::Match(expression, rules),
        None => StringIntersection::Impossible,
    })
}
