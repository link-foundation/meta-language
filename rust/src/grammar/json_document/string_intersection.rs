use std::collections::{BTreeSet, HashMap};

use super::super::{CharClassItem, ExprBuilder, Grammar, GrammarExpr, GrammarRule};
use super::pattern_policy::string_intersection_limits;
use super::patterns::complement_scalar_ranges;

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
            GrammarExpr::CharClass { negated, items } => {
                let mut intervals: Vec<_> = items
                    .iter()
                    .map(|item| match *item {
                        CharClassItem::Char(character) => {
                            (u32::from(character), u32::from(character))
                        }
                        CharClassItem::Range(first, last) => (u32::from(first), u32::from(last)),
                    })
                    .collect();
                if *negated {
                    work.charge(intervals.len())?;
                    intervals.sort_unstable();
                    intervals = complement_scalar_ranges(&intervals);
                }
                for (first, last) in intervals {
                    self.range(
                        char::from_u32(first)?,
                        char::from_u32(last)?,
                        start,
                        end,
                        work,
                    )?;
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

type ProductState = Vec<Vec<usize>>;

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
    fn state(
        &mut self,
        states: ProductState,
        groups: &[Vec<usize>],
        work: &mut WorkLimit,
    ) -> Option<usize> {
        if let Some(&index) = self.indices.get(&states) {
            return Some(index);
        }
        let index = self.rows.len();
        if index == self.state_limit {
            return None;
        }
        let mut accepting_branches = 0;
        for group in groups {
            work.charge(group.len())?;
            if group.iter().all(|&machine| states[machine].contains(&1)) {
                accepting_branches += 1;
                if accepting_branches > 1 {
                    break;
                }
            }
        }
        self.indices.insert(states.clone(), index);
        self.rows.push(ProductRow {
            states,
            accepts: accepting_branches == 1,
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

pub(super) fn intersect_json_string_expressions(
    grammar: &Grammar,
    left_expression: &GrammarExpr,
    right_expression: &GrammarExpr,
    rule_prefix: &str,
) -> Option<StringIntersection> {
    select_exclusive_json_string_expressions(
        grammar,
        &[vec![left_expression, right_expression]],
        rule_prefix,
    )
}

/// Each branch is a conjunction. The decoder accepts when exactly one branch matches.
pub(super) fn select_exclusive_json_string_expressions(
    grammar: &Grammar,
    branches: &[Vec<&GrammarExpr>],
    rule_prefix: &str,
) -> Option<StringIntersection> {
    if branches.is_empty() {
        return Some(StringIntersection::Impossible);
    }
    if let [branch] = branches
        && let [expression] = branch.as_slice()
    {
        return Some(StringIntersection::Match(
            (**expression).clone(),
            Vec::new(),
        ));
    }
    let (maximum_source_states, maximum_product_states, maximum_work) =
        string_intersection_limits();
    let mut work = WorkLimit(maximum_work as usize);
    let mut machines = Vec::new();
    let mut machine_indices = HashMap::new();
    let mut groups = Vec::with_capacity(branches.len());
    for branch in branches {
        let mut group = Vec::with_capacity(branch.len());
        for &expression in branch {
            work.charge(1)?;
            let identity = std::ptr::from_ref(expression);
            let index = match machine_indices.get(&identity) {
                Some(&index) => index,
                None => {
                    let index = machines.len();
                    machines.push(Automaton::new(
                        grammar,
                        expression,
                        maximum_source_states as usize,
                        &mut work,
                    )?);
                    machine_indices.insert(identity, index);
                    index
                }
            };
            group.push(index);
        }
        groups.push(group);
    }
    let mut product = Product {
        state_limit: maximum_product_states as usize,
        ..Product::default()
    };
    let start = machines
        .iter()
        .map(|machine| machine.closure([0], &mut work))
        .collect::<Option<Vec<_>>>()?;
    product.state(start, &groups, &mut work)?;
    let mut accepting = Vec::new();
    let mut index = 0;
    while let Some(row) = product.rows.get(index) {
        if row.accepts {
            accepting.push(index);
        }
        let machine_edges: Vec<_> = machines
            .iter()
            .zip(&row.states)
            .map(|(machine, states)| machine.edges(states))
            .collect();
        let edge_count: usize = machine_edges.iter().map(Vec::len).sum();
        let bounds: BTreeSet<_> = machine_edges
            .iter()
            .flatten()
            .flat_map(|edge| [edge.first, edge.last + 1])
            .collect();
        let bounds: Vec<_> = bounds.into_iter().collect();
        for bounds in bounds.windows(2) {
            work.charge(1 + edge_count + machine_edges.len())?;
            let first = bounds[0];
            let transitions: Vec<_> = machine_edges
                .iter()
                .map(|edges| targets(edges, first))
                .collect();
            let mut viable = false;
            for group in &groups {
                work.charge(group.len())?;
                if group
                    .iter()
                    .all(|&machine| !transitions[machine].is_empty())
                {
                    viable = true;
                    break;
                }
            }
            if !viable {
                continue;
            }
            let states = machines
                .iter()
                .zip(transitions)
                .map(|(machine, set)| machine.closure(set, &mut work))
                .collect::<Option<Vec<_>>>()?;
            let target = product.state(states, &groups, &mut work)?;
            product.transition(index, first, bounds[1] - 1, target);
        }
        index += 1;
    }
    Some(match product.compile(rule_prefix, accepting) {
        Some((expression, rules)) => StringIntersection::Match(expression, rules),
        None => StringIntersection::Impossible,
    })
}
