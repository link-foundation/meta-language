//! State-merging regular inference for labelled symbol examples.
//!
//! The implementation follows the classic PTA/APTA plus red-blue state-merging
//! shape used by RPNI, EDSM, and ALERGIA. It is a clean-room Rust
//! implementation derived from the public issue specification and the cited
//! papers/permissive references named there; no GPL or LGPL implementation is
//! linked, vendored, or consulted by this crate.

use std::collections::{BTreeMap, BTreeSet, VecDeque};

use crate::grammar::{Grammar, GrammarExpr, GrammarFormat, GrammarRule};
use crate::semantics::ProbabilisticTruthValue;

/// A token or character symbol consumed by the regular learner.
pub type Symbol = String;

/// Labelled examples over a token or character alphabet.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Sample {
    /// Strings known to be in the target language.
    pub positives: Vec<Vec<Symbol>>,
    /// Strings known not to be in the target language.
    pub negatives: Vec<Vec<Symbol>>,
}

impl Sample {
    /// Builds a labelled sample from positive and negative symbol strings.
    #[must_use]
    pub const fn new(positives: Vec<Vec<Symbol>>, negatives: Vec<Vec<Symbol>>) -> Self {
        Self {
            positives,
            negatives,
        }
    }

    /// Builds a positive-only sample.
    #[must_use]
    pub const fn positive_only(positives: Vec<Vec<Symbol>>) -> Self {
        Self {
            positives,
            negatives: Vec::new(),
        }
    }
}

/// One state in an augmented prefix-tree acceptor.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct AptaState {
    /// Whether at least one positive example ends in this state.
    pub accepting: bool,
    /// Whether at least one negative example ends in this state.
    pub rejecting: bool,
    /// Number of positive examples whose path visits this state.
    pub arrival_count: u64,
    /// Number of positive examples ending in this state.
    pub final_count: u64,
}

/// Augmented prefix-tree acceptor built directly from a [`Sample`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Apta {
    /// APTA states. State `0` is always the root.
    pub states: Vec<AptaState>,
    /// Deterministic outgoing transitions for each state.
    pub transitions: Vec<BTreeMap<Symbol, usize>>,
    /// Positive-example edge frequencies for stochastic ALERGIA output.
    pub transition_counts: Vec<BTreeMap<Symbol, u64>>,
}

impl Apta {
    /// Builds an APTA from `sample`, sharing all common prefixes.
    #[must_use]
    pub fn from_sample(sample: &Sample) -> Self {
        let mut apta = Self::new();

        for positive in &sample.positives {
            apta.insert(positive, true);
        }
        for negative in &sample.negatives {
            apta.insert(negative, false);
        }

        apta
    }

    fn new() -> Self {
        Self {
            states: vec![AptaState::default()],
            transitions: vec![BTreeMap::new()],
            transition_counts: vec![BTreeMap::new()],
        }
    }

    fn insert(&mut self, symbols: &[Symbol], positive: bool) {
        let mut state = 0usize;
        if positive {
            self.states[state].arrival_count = self.states[state].arrival_count.saturating_add(1);
        }

        for symbol in symbols {
            let next = if let Some(next) = self.transitions[state].get(symbol).copied() {
                next
            } else {
                let next = self.states.len();
                self.states.push(AptaState::default());
                self.transitions.push(BTreeMap::new());
                self.transition_counts.push(BTreeMap::new());
                self.transitions[state].insert(symbol.clone(), next);
                next
            };

            if positive {
                *self.transition_counts[state]
                    .entry(symbol.clone())
                    .or_default() += 1;
                self.states[next].arrival_count = self.states[next].arrival_count.saturating_add(1);
            }
            state = next;
        }

        if positive {
            self.states[state].accepting = true;
            self.states[state].final_count = self.states[state].final_count.saturating_add(1);
        } else {
            self.states[state].rejecting = true;
        }
    }
}

/// State-merging strategy used by [`infer_dfa`].
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MergeStrategy {
    /// Regular Positive and Negative Inference: first consistent red-blue merge.
    Rpni,
    /// Evidence-driven state merging: highest-scoring consistent red-blue merge.
    Edsm,
    /// Positive-only stochastic merging with an ALERGIA-style Hoeffding test.
    ///
    /// `alpha` is a compatibility confidence in `(0, 1)`. Lower values are
    /// stricter in this crate's API and therefore permit fewer merges.
    Alergia {
        /// Compatibility confidence controlling the Hoeffding bound.
        alpha: f64,
    },
}

/// Public state in an inferred partial DFA.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InferredState {
    /// Whether this state accepts when input ends here.
    pub accepting: bool,
    /// Whether this state was backed by negative evidence during inference.
    pub rejecting: bool,
    /// Number of positive examples whose path visits this state.
    pub arrival_count: u64,
    /// Number of positive examples ending in this state.
    pub final_count: u64,
}

/// One accepted state merge performed by a learner.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MergeEvent {
    /// Target red state receiving the source state.
    pub target: usize,
    /// Source blue state merged into the target.
    pub source: usize,
    /// Overlapping accept/reject evidence observed during recursive folding.
    pub evidence: usize,
}

/// Inferred deterministic finite automaton.
///
/// The automaton is partial: absent transitions reject. States that cannot reach
/// an accepting state are pruned after learning, while their negative labels are
/// still used internally to reject inconsistent merges.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InferredAutomaton {
    /// Productive DFA states. State `0` is the initial state.
    pub states: Vec<InferredState>,
    /// Deterministic transitions for each state.
    pub transitions: Vec<BTreeMap<Symbol, usize>>,
    /// Final-state probabilities for stochastic ALERGIA output.
    pub final_probabilities: Vec<Option<ProbabilisticTruthValue>>,
    /// Transition probabilities for stochastic ALERGIA output.
    pub transition_probabilities: Vec<BTreeMap<Symbol, ProbabilisticTruthValue>>,
    /// Accepted merge history in deterministic order.
    pub merge_history: Vec<MergeEvent>,
}

impl InferredAutomaton {
    /// Returns `true` when `input` is accepted by the inferred automaton.
    #[must_use]
    pub fn accepts(&self, input: &[Symbol]) -> bool {
        let mut state = 0usize;

        for symbol in input {
            let Some(next) = self
                .transitions
                .get(state)
                .and_then(|transitions| transitions.get(symbol))
            else {
                return false;
            };
            state = *next;
        }

        self.states
            .get(state)
            .is_some_and(|state| state.accepting && !state.rejecting)
    }

    /// Convenience helper for character-level automata.
    #[must_use]
    pub fn accepts_text(&self, text: &str) -> bool {
        let symbols = text
            .chars()
            .map(|value| value.to_string())
            .collect::<Vec<_>>();
        self.accepts(&symbols)
    }

    /// Converts the automaton into a right-linear grammar.
    #[must_use]
    pub fn to_grammar(&self) -> Grammar {
        let mut grammar = Grammar::new().with_source_format(GrammarFormat::Inferred);
        if self.states.is_empty() {
            return grammar;
        }

        for state in 0..self.states.len() {
            grammar.add_rule(GrammarRule::new(
                state_name(state),
                self.state_expression(state),
            ));
        }
        grammar.set_start(state_name(0));
        grammar
    }

    fn state_expression(&self, state: usize) -> GrammarExpr {
        let mut alternatives = Vec::new();

        if self.states[state].accepting && !self.states[state].rejecting {
            alternatives.push(GrammarExpr::Empty);
        }

        for (symbol, target) in &self.transitions[state] {
            alternatives.push(GrammarExpr::Sequence(vec![
                GrammarExpr::Terminal(symbol.clone()),
                GrammarExpr::NonTerminal(state_name(*target)),
            ]));
        }

        match alternatives.as_slice() {
            [only] => only.clone(),
            _ => GrammarExpr::Choice {
                ordered: false,
                alternatives,
            },
        }
    }
}

/// Infers a regular automaton from labelled examples.
#[must_use]
pub fn infer_dfa(sample: &Sample, strategy: MergeStrategy) -> InferredAutomaton {
    let apta = Apta::from_sample(sample);
    let mut machine = WorkAutomaton::from_apta(&apta);
    let mut merge_history = Vec::new();
    let mut red = BTreeSet::from([0usize]);

    loop {
        red = normalised_red(&machine, &red);
        let blue = blue_states(&machine, &red);
        if blue.is_empty() {
            break;
        }

        match strategy {
            MergeStrategy::Rpni => rpni_step(&mut machine, &mut red, &blue, &mut merge_history),
            MergeStrategy::Edsm => edsm_step(&mut machine, &mut red, &blue, &mut merge_history),
            MergeStrategy::Alergia { alpha } => {
                alergia_step(&mut machine, &mut red, &blue, alpha, &mut merge_history);
            }
        }
    }

    machine.into_inferred(
        matches!(strategy, MergeStrategy::Alergia { .. }),
        merge_history,
    )
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct WorkState {
    accepting: bool,
    rejecting: bool,
    arrival_count: u64,
    final_count: u64,
    rank: usize,
    active: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct WorkAutomaton {
    states: Vec<WorkState>,
    transitions: Vec<BTreeMap<Symbol, usize>>,
    transition_counts: Vec<BTreeMap<Symbol, u64>>,
    parent: Vec<usize>,
}

impl WorkAutomaton {
    fn from_apta(apta: &Apta) -> Self {
        let ranks = canonical_ranks(apta);
        Self {
            states: apta
                .states
                .iter()
                .enumerate()
                .map(|(index, state)| WorkState {
                    accepting: state.accepting,
                    rejecting: state.rejecting,
                    arrival_count: state.arrival_count,
                    final_count: state.final_count,
                    rank: ranks[index],
                    active: true,
                })
                .collect(),
            transitions: apta.transitions.clone(),
            transition_counts: apta.transition_counts.clone(),
            parent: (0..apta.states.len()).collect(),
        }
    }

    fn representative(&self, mut state: usize) -> usize {
        while self.parent[state] != state {
            state = self.parent[state];
        }
        state
    }

    fn active_representative(&self, state: usize) -> Option<usize> {
        let representative = self.representative(state);
        self.states
            .get(representative)
            .filter(|state| state.active)
            .map(|_| representative)
    }

    fn active_sorted(&self) -> Vec<usize> {
        let mut states = self
            .states
            .iter()
            .enumerate()
            .filter_map(|(index, state)| state.active.then_some(index))
            .collect::<Vec<_>>();
        states.sort_by_key(|state| (self.states[*state].rank, *state));
        states
    }

    fn red_sorted(&self, red: &BTreeSet<usize>) -> Vec<usize> {
        let mut states = red
            .iter()
            .filter_map(|state| self.active_representative(*state))
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect::<Vec<_>>();
        states.sort_by_key(|state| (self.states[*state].rank, *state));
        states
    }

    fn try_merge(&self, target: usize, source: usize, alpha: Option<f64>) -> Option<MergeAttempt> {
        if target == source {
            return None;
        }

        if let Some(alpha) = alpha {
            let mut seen = BTreeSet::new();
            if !self.alergia_compatible(target, source, alpha, &mut seen) {
                return None;
            }
        }

        let mut candidate = self.clone();
        let mut evidence = 0usize;
        if !candidate.merge_into(target, source, &mut evidence) {
            return None;
        }
        candidate.normalise_transitions();

        Some(MergeAttempt {
            machine: candidate,
            event: MergeEvent {
                target,
                source,
                evidence,
            },
        })
    }

    fn merge_into(&mut self, target: usize, source: usize, evidence: &mut usize) -> bool {
        let target = self.representative(target);
        let source = self.representative(source);
        if target == source {
            return true;
        }
        if !self.states[target].active || !self.states[source].active {
            return false;
        }

        if (self.states[target].accepting && self.states[source].rejecting)
            || (self.states[target].rejecting && self.states[source].accepting)
        {
            return false;
        }
        if self.states[target].accepting && self.states[source].accepting {
            *evidence = evidence.saturating_add(1);
        }
        if self.states[target].rejecting && self.states[source].rejecting {
            *evidence = evidence.saturating_add(1);
        }

        self.states[target].accepting |= self.states[source].accepting;
        self.states[target].rejecting |= self.states[source].rejecting;
        self.states[target].arrival_count = self.states[target]
            .arrival_count
            .saturating_add(self.states[source].arrival_count);
        self.states[target].final_count = self.states[target]
            .final_count
            .saturating_add(self.states[source].final_count);

        let source_transitions = self.transitions[source].clone();
        let source_counts = self.transition_counts[source].clone();
        self.states[source].active = false;
        self.parent[source] = target;

        for (symbol, source_next) in source_transitions {
            let source_next = self.representative(source_next);
            let source_count = source_counts.get(&symbol).copied().unwrap_or_default();

            if let Some(target_next) = self.transitions[target].get(&symbol).copied() {
                let target_next = self.representative(target_next);
                *self.transition_counts[target]
                    .entry(symbol.clone())
                    .or_default() += source_count;

                if target_next != source_next
                    && !self.merge_into(target_next, source_next, evidence)
                {
                    return false;
                }
                let target_next = self.representative(target_next);
                self.transitions[target].insert(symbol, target_next);
            } else {
                self.transitions[target].insert(symbol.clone(), source_next);
                self.transition_counts[target].insert(symbol, source_count);
            }
        }

        self.transitions[source].clear();
        self.transition_counts[source].clear();
        true
    }

    fn normalise_transitions(&mut self) {
        for state in 0..self.states.len() {
            if !self.states[state].active {
                continue;
            }
            let transitions = self.transitions[state].clone();
            self.transitions[state].clear();
            for (symbol, target) in transitions {
                let target = self.representative(target);
                self.transitions[state].insert(symbol, target);
            }
        }
    }

    fn alergia_compatible(
        &self,
        left: usize,
        right: usize,
        alpha: f64,
        seen: &mut BTreeSet<(usize, usize)>,
    ) -> bool {
        let left = self.representative(left);
        let right = self.representative(right);
        if left == right {
            return true;
        }

        let key = if left < right {
            (left, right)
        } else {
            (right, left)
        };
        if !seen.insert(key) {
            return true;
        }

        let left_state = &self.states[left];
        let right_state = &self.states[right];
        if (left_state.accepting && right_state.rejecting)
            || (left_state.rejecting && right_state.accepting)
        {
            return false;
        }

        if !proportions_compatible(
            left_state.final_count,
            left_state.arrival_count,
            right_state.final_count,
            right_state.arrival_count,
            alpha,
        ) {
            return false;
        }

        for symbol in self.outgoing_symbols(left, right) {
            let left_count = self.transition_counts[left]
                .get(&symbol)
                .copied()
                .unwrap_or_default();
            let right_count = self.transition_counts[right]
                .get(&symbol)
                .copied()
                .unwrap_or_default();

            if !proportions_compatible(
                left_count,
                left_state.arrival_count,
                right_count,
                right_state.arrival_count,
                alpha,
            ) {
                return false;
            }

            if let (Some(left_target), Some(right_target)) = (
                self.transitions[left].get(&symbol).copied(),
                self.transitions[right].get(&symbol).copied(),
            ) {
                if !self.alergia_compatible(left_target, right_target, alpha, seen) {
                    return false;
                }
            }
        }

        true
    }

    fn outgoing_symbols(&self, left: usize, right: usize) -> BTreeSet<Symbol> {
        self.transitions[left]
            .keys()
            .chain(self.transitions[right].keys())
            .chain(self.transition_counts[left].keys())
            .chain(self.transition_counts[right].keys())
            .cloned()
            .collect()
    }

    fn productive_states(&self) -> BTreeSet<usize> {
        let mut reverse = vec![Vec::<usize>::new(); self.states.len()];
        for state in self.active_sorted() {
            for target in self.transitions[state].values() {
                let target = self.representative(*target);
                if self.states[target].active {
                    reverse[target].push(state);
                }
            }
        }

        let mut productive = BTreeSet::new();
        let mut queue = self
            .active_sorted()
            .into_iter()
            .filter(|state| self.states[*state].accepting && !self.states[*state].rejecting)
            .collect::<VecDeque<_>>();

        while let Some(state) = queue.pop_front() {
            if !productive.insert(state) {
                continue;
            }
            for predecessor in &reverse[state] {
                queue.push_back(*predecessor);
            }
        }

        productive
    }

    fn into_inferred(
        self,
        include_probabilities: bool,
        merge_history: Vec<MergeEvent>,
    ) -> InferredAutomaton {
        let productive = self.productive_states();
        let mut active = self
            .active_sorted()
            .into_iter()
            .filter(|state| productive.contains(state) || *state == self.representative(0))
            .collect::<Vec<_>>();
        if active.is_empty() {
            active.push(self.representative(0));
        }

        let mut state_map = vec![None; self.states.len()];
        for (new, old) in active.iter().enumerate() {
            state_map[*old] = Some(new);
        }

        let mut states = Vec::new();
        let mut transitions = Vec::new();
        let mut final_probabilities = Vec::new();
        let mut transition_probabilities = Vec::new();

        for old in active {
            let state = &self.states[old];
            states.push(InferredState {
                accepting: state.accepting && !state.rejecting,
                rejecting: state.rejecting,
                arrival_count: state.arrival_count,
                final_count: state.final_count,
            });

            let mut remapped_transitions = BTreeMap::new();
            let mut remapped_probabilities = BTreeMap::new();
            for (symbol, target) in &self.transitions[old] {
                let target = self.representative(*target);
                let Some(target) = state_map[target] else {
                    continue;
                };
                remapped_transitions.insert(symbol.clone(), target);

                if include_probabilities && state.arrival_count > 0 {
                    let count = self.transition_counts[old]
                        .get(symbol)
                        .copied()
                        .unwrap_or_default()
                        .min(state.arrival_count);
                    if let Some(probability) =
                        ProbabilisticTruthValue::from_ratio(count, state.arrival_count)
                    {
                        remapped_probabilities.insert(symbol.clone(), probability);
                    }
                }
            }
            transitions.push(remapped_transitions);
            transition_probabilities.push(remapped_probabilities);

            let final_probability = if include_probabilities && state.arrival_count > 0 {
                ProbabilisticTruthValue::from_ratio(
                    state.final_count.min(state.arrival_count),
                    state.arrival_count,
                )
            } else {
                None
            };
            final_probabilities.push(final_probability);
        }

        InferredAutomaton {
            states,
            transitions,
            final_probabilities,
            transition_probabilities,
            merge_history,
        }
    }
}

#[derive(Clone, Debug)]
struct MergeAttempt {
    machine: WorkAutomaton,
    event: MergeEvent,
}

fn rpni_step(
    machine: &mut WorkAutomaton,
    red: &mut BTreeSet<usize>,
    blue: &[usize],
    merge_history: &mut Vec<MergeEvent>,
) {
    let blue_state = blue[0];
    for red_state in machine.red_sorted(red) {
        if let Some(attempt) = machine.try_merge(red_state, blue_state, None) {
            *machine = attempt.machine;
            merge_history.push(attempt.event);
            return;
        }
    }

    red.insert(blue_state);
}

fn edsm_step(
    machine: &mut WorkAutomaton,
    red: &mut BTreeSet<usize>,
    blue: &[usize],
    merge_history: &mut Vec<MergeEvent>,
) {
    let mut best = None::<MergeAttempt>;
    let red_states = machine.red_sorted(red);

    for blue_state in blue {
        for red_state in &red_states {
            let Some(attempt) = machine.try_merge(*red_state, *blue_state, None) else {
                continue;
            };

            let is_better = match &best {
                Some(current) => {
                    attempt.event.evidence > current.event.evidence
                        || (attempt.event.evidence == current.event.evidence
                            && merge_tie_key(&attempt.event) < merge_tie_key(&current.event))
                }
                None => true,
            };

            if is_better {
                best = Some(attempt);
            }
        }
    }

    if let Some(attempt) = best {
        *machine = attempt.machine;
        merge_history.push(attempt.event);
    } else {
        red.insert(blue[0]);
    }
}

fn alergia_step(
    machine: &mut WorkAutomaton,
    red: &mut BTreeSet<usize>,
    blue: &[usize],
    alpha: f64,
    merge_history: &mut Vec<MergeEvent>,
) {
    let blue_state = blue[0];
    for red_state in machine.red_sorted(red) {
        if let Some(attempt) = machine.try_merge(red_state, blue_state, Some(alpha)) {
            *machine = attempt.machine;
            merge_history.push(attempt.event);
            return;
        }
    }

    red.insert(blue_state);
}

const fn merge_tie_key(event: &MergeEvent) -> (usize, usize) {
    (event.source, event.target)
}

fn normalised_red(machine: &WorkAutomaton, red: &BTreeSet<usize>) -> BTreeSet<usize> {
    red.iter()
        .filter_map(|state| machine.active_representative(*state))
        .collect()
}

fn blue_states(machine: &WorkAutomaton, red: &BTreeSet<usize>) -> Vec<usize> {
    let mut blue = BTreeSet::new();

    for red_state in machine.red_sorted(red) {
        for target in machine.transitions[red_state].values() {
            if let Some(target) = machine.active_representative(*target) {
                if !red.contains(&target) {
                    blue.insert(target);
                }
            }
        }
    }

    let mut blue = blue.into_iter().collect::<Vec<_>>();
    blue.sort_by_key(|state| (machine.states[*state].rank, *state));
    blue
}

fn canonical_ranks(apta: &Apta) -> Vec<usize> {
    let mut paths = vec![Vec::<Symbol>::new(); apta.states.len()];
    let mut queue = VecDeque::from([0usize]);
    let mut seen = BTreeSet::from([0usize]);

    while let Some(state) = queue.pop_front() {
        for (symbol, target) in &apta.transitions[state] {
            if seen.insert(*target) {
                paths[*target] = paths[state]
                    .iter()
                    .cloned()
                    .chain([symbol.clone()])
                    .collect();
                queue.push_back(*target);
            }
        }
    }

    let mut ordered = (0..apta.states.len()).collect::<Vec<_>>();
    ordered.sort_by_key(|state| (paths[*state].len(), paths[*state].clone(), *state));

    let mut ranks = vec![0usize; apta.states.len()];
    for (rank, state) in ordered.into_iter().enumerate() {
        ranks[state] = rank;
    }
    ranks
}

fn proportions_compatible(
    left_count: u64,
    left_total: u64,
    right_count: u64,
    right_total: u64,
    alpha: f64,
) -> bool {
    if left_total == 0 || right_total == 0 {
        return left_count == right_count;
    }

    let left = ratio(left_count, left_total);
    let right = ratio(right_count, right_total);
    let bound = compatibility_bound(alpha, left_total, right_total);

    (left - right).abs() <= bound
}

fn compatibility_bound(alpha: f64, left_total: u64, right_total: u64) -> f64 {
    let confidence = normalised_alpha(alpha);
    let significance = (1.0 - confidence).max(f64::MIN_POSITIVE);
    let multiplier = (0.5 * (2.0 / significance).ln()).sqrt();

    multiplier * (1.0 / count_to_f64(left_total).sqrt() + 1.0 / count_to_f64(right_total).sqrt())
}

fn normalised_alpha(alpha: f64) -> f64 {
    if alpha.is_finite() {
        alpha.clamp(0.000_001, 0.999_999)
    } else {
        0.5
    }
}

fn ratio(count: u64, total: u64) -> f64 {
    count_to_f64(count) / count_to_f64(total)
}

fn count_to_f64(count: u64) -> f64 {
    f64::from(u32::try_from(count).unwrap_or(u32::MAX))
}

fn state_name(state: usize) -> String {
    format!("q{state}")
}
