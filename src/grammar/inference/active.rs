//! Active regular-language inference from membership and equivalence queries.
//!
//! This module implements Angluin's L* observation-table learner for regular
//! languages. It is intentionally opt-in: positive-only CFG inference does not
//! depend on this path. Exact equivalence can be supplied by a caller-provided
//! oracle; when only membership is available, the provided adapters use a
//! deterministic bounded sampler as an approximate equivalence oracle.

use std::collections::{btree_map::Entry, BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

use crate::grammar::{Grammar, GrammarExpr, GrammarFormat, GrammarParser, GrammarRule};
use crate::{LinkNetwork, LinkType, ParseConfiguration, ParserRegistry};

/// Input symbol consumed by the active learner.
pub type Symbol = char;

type Word = Vec<Symbol>;

/// Predicate used by [`ParserMembershipOracle`] to decide parser acceptance.
pub type ParserAcceptancePredicate = fn(&LinkNetwork, &str) -> bool;

/// Deterministic finite automaton learned by L*.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Dfa {
    /// Input alphabet in transition-column order.
    pub alphabet: Vec<Symbol>,
    /// Number of states in the automaton.
    pub states: usize,
    /// Start state index.
    pub start: usize,
    /// Accepting flag for each state.
    pub accepting: Vec<bool>,
    /// Total transition function: `delta[state][symbol_index] = next_state`.
    pub delta: Vec<Vec<usize>>,
}

impl Dfa {
    /// Returns `true` when `word` is accepted by this DFA.
    #[must_use]
    pub fn accepts(&self, word: &[Symbol]) -> bool {
        let mut state = self.start;
        if state >= self.states {
            return false;
        }

        for symbol in word {
            let Some(symbol_index) = self.symbol_index(*symbol) else {
                return false;
            };
            let Some(next) = self
                .delta
                .get(state)
                .and_then(|row| row.get(symbol_index))
                .copied()
            else {
                return false;
            };
            if next >= self.states {
                return false;
            }
            state = next;
        }

        self.accepting.get(state).copied().unwrap_or(false)
    }

    /// Convenience helper for character-level text input.
    #[must_use]
    pub fn accepts_text(&self, text: &str) -> bool {
        self.accepts(&text.chars().collect::<Vec<_>>())
    }

    /// Converts this DFA to a right-linear grammar.
    #[must_use]
    pub fn to_grammar(&self) -> Grammar {
        let mut grammar = Grammar::new().with_source_format(GrammarFormat::Inferred);
        if self.states == 0 || self.start >= self.states {
            return grammar;
        }

        for state in 0..self.states {
            grammar.add_rule(GrammarRule::new(
                state_name(state),
                self.state_expression(state),
            ));
        }
        grammar.set_start(state_name(self.start));
        grammar
    }

    fn state_expression(&self, state: usize) -> GrammarExpr {
        let mut alternatives = Vec::new();

        if self.accepting.get(state).copied().unwrap_or(false) {
            alternatives.push(GrammarExpr::Empty);
        }

        if let Some(transitions) = self.delta.get(state) {
            for (symbol_index, target) in transitions.iter().copied().enumerate() {
                if target >= self.states {
                    continue;
                }
                let Some(symbol) = self.alphabet.get(symbol_index) else {
                    continue;
                };
                alternatives.push(GrammarExpr::Sequence(vec![
                    GrammarExpr::Terminal(symbol.to_string()),
                    GrammarExpr::NonTerminal(state_name(target)),
                ]));
            }
        }

        match alternatives.as_slice() {
            [only] => only.clone(),
            _ => GrammarExpr::Choice {
                ordered: false,
                alternatives,
            },
        }
    }

    fn symbol_index(&self, symbol: Symbol) -> Option<usize> {
        self.alphabet
            .iter()
            .position(|candidate| *candidate == symbol)
    }

    fn validate(&self) -> Result<(), ActiveLearningError> {
        validate_alphabet(&self.alphabet)?;
        if self.start >= self.states {
            return Err(ActiveLearningError::InvalidDfa {
                reason: format!(
                    "start state {} is outside {} states",
                    self.start, self.states
                ),
            });
        }
        if self.accepting.len() != self.states {
            return Err(ActiveLearningError::InvalidDfa {
                reason: format!(
                    "accepting vector has {} entries for {} states",
                    self.accepting.len(),
                    self.states
                ),
            });
        }
        if self.delta.len() != self.states {
            return Err(ActiveLearningError::InvalidDfa {
                reason: format!(
                    "transition table has {} rows for {} states",
                    self.delta.len(),
                    self.states
                ),
            });
        }
        for (state, transitions) in self.delta.iter().enumerate() {
            if transitions.len() != self.alphabet.len() {
                return Err(ActiveLearningError::InvalidDfa {
                    reason: format!(
                        "state {state} has {} transitions for {} symbols",
                        transitions.len(),
                        self.alphabet.len()
                    ),
                });
            }
            if let Some(target) = transitions.iter().find(|target| **target >= self.states) {
                return Err(ActiveLearningError::InvalidDfa {
                    reason: format!("state {state} transitions to invalid state {target}"),
                });
            }
        }
        Ok(())
    }
}

/// Learner and approximate-equivalence configuration.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ActiveLearningConfig {
    /// Maximum length for sampled counterexample candidates.
    pub max_word_len: usize,
    /// Maximum number of sampled equivalence candidates to compare.
    pub equivalence_samples: usize,
    /// Deterministic seed used by the bounded sampler.
    pub seed: u64,
    /// Requests the TTT learner. TTT is reserved for a follow-up and currently errors.
    pub use_ttt: bool,
    /// Maximum L* refinement rounds before returning an error.
    pub max_iterations: usize,
}

impl Default for ActiveLearningConfig {
    fn default() -> Self {
        Self {
            max_word_len: 8,
            equivalence_samples: 256,
            seed: 0xA17E_1EAF_DFA5_EED5,
            use_ttt: false,
            max_iterations: 128,
        }
    }
}

/// Error returned by active-learning entry points.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ActiveLearningError {
    /// The alphabet contains the same symbol more than once.
    DuplicateSymbol {
        /// Duplicated alphabet symbol.
        symbol: Symbol,
    },
    /// The requested TTT learner is not shipped in this module yet.
    TttUnavailable,
    /// A configuration value prevents the learner from running.
    InvalidConfig {
        /// Human-readable reason.
        reason: String,
    },
    /// A malformed DFA was supplied or constructed.
    InvalidDfa {
        /// Human-readable reason.
        reason: String,
    },
    /// The L* loop did not converge within the configured refinement budget.
    MaxIterations {
        /// Configured iteration budget.
        max_iterations: usize,
    },
}

impl fmt::Display for ActiveLearningError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DuplicateSymbol { symbol } => {
                write!(formatter, "alphabet contains duplicate symbol {symbol:?}")
            }
            Self::TttUnavailable => formatter.write_str("TTT active learning is not implemented"),
            Self::InvalidConfig { reason } | Self::InvalidDfa { reason } => {
                formatter.write_str(reason)
            }
            Self::MaxIterations { max_iterations } => write!(
                formatter,
                "active learner did not converge within {max_iterations} iterations"
            ),
        }
    }
}

impl Error for ActiveLearningError {}

/// Minimally adequate teacher for active regular-language learning.
pub trait Oracle {
    /// Input alphabet explored by the learner.
    fn alphabet(&self) -> &[Symbol];

    /// Returns `true` when `word` is in the target language.
    fn membership(&self, word: &[Symbol]) -> bool;

    /// Returns a counterexample when `hypothesis` disagrees with the target.
    ///
    /// Returning `None` accepts the hypothesis as equivalent. Exact oracles can
    /// ignore `config`; approximate oracles use it for bounded deterministic
    /// sampling.
    fn equivalence(&self, hypothesis: &Dfa, config: &ActiveLearningConfig) -> Option<Word>;
}

/// Membership-plus-sampling oracle backed by an in-process predicate.
#[derive(Clone, Debug)]
pub struct SamplingEquivalenceOracle<M> {
    alphabet: Vec<Symbol>,
    membership: M,
}

impl<M> SamplingEquivalenceOracle<M> {
    /// Builds a sampling oracle over `alphabet`.
    #[must_use]
    pub const fn new(alphabet: Vec<Symbol>, membership: M) -> Self {
        Self {
            alphabet,
            membership,
        }
    }
}

impl<M> Oracle for SamplingEquivalenceOracle<M>
where
    M: Fn(&[Symbol]) -> bool,
{
    fn alphabet(&self) -> &[Symbol] {
        &self.alphabet
    }

    fn membership(&self, word: &[Symbol]) -> bool {
        (self.membership)(word)
    }

    fn equivalence(&self, hypothesis: &Dfa, config: &ActiveLearningConfig) -> Option<Word> {
        sampled_counterexample(hypothesis, &self.alphabet, config, |word| {
            self.membership(word)
        })
    }
}

/// Active-learning oracle backed by a runtime [`GrammarParser`].
#[derive(Clone, Debug)]
pub struct GrammarAcceptorOracle {
    parser: GrammarParser,
    alphabet: Vec<Symbol>,
}

impl GrammarAcceptorOracle {
    /// Builds an oracle from a first-class grammar acceptor.
    #[must_use]
    pub fn new(grammar: Grammar, alphabet: Vec<Symbol>) -> Self {
        Self {
            parser: GrammarParser::new(grammar),
            alphabet,
        }
    }
}

impl Oracle for GrammarAcceptorOracle {
    fn alphabet(&self) -> &[Symbol] {
        &self.alphabet
    }

    fn membership(&self, word: &[Symbol]) -> bool {
        self.parser.accepts(&word_text(word))
    }

    fn equivalence(&self, hypothesis: &Dfa, config: &ActiveLearningConfig) -> Option<Word> {
        sampled_counterexample(hypothesis, &self.alphabet, config, |word| {
            self.membership(word)
        })
    }
}

/// Active-learning oracle backed by a [`ParserRegistry`] parser.
#[derive(Clone, Debug)]
pub struct ParserMembershipOracle {
    registry: ParserRegistry,
    language: String,
    alphabet: Vec<Symbol>,
    configuration: ParseConfiguration,
    acceptance: ParserAcceptancePredicate,
}

impl ParserMembershipOracle {
    /// Builds a parser-backed oracle with the default clean-structural predicate.
    #[must_use]
    pub fn new(
        registry: ParserRegistry,
        language: impl Into<String>,
        alphabet: Vec<Symbol>,
    ) -> Self {
        Self {
            registry,
            language: language.into(),
            alphabet,
            configuration: ParseConfiguration::default(),
            acceptance: clean_structural_acceptance,
        }
    }

    /// Returns this oracle with a different parse configuration.
    #[must_use]
    pub const fn with_configuration(mut self, configuration: ParseConfiguration) -> Self {
        self.configuration = configuration;
        self
    }

    /// Returns this oracle with a custom acceptance predicate over parser output.
    #[must_use]
    pub const fn with_acceptance_predicate(
        mut self,
        acceptance: ParserAcceptancePredicate,
    ) -> Self {
        self.acceptance = acceptance;
        self
    }
}

impl Oracle for ParserMembershipOracle {
    fn alphabet(&self) -> &[Symbol] {
        &self.alphabet
    }

    fn membership(&self, word: &[Symbol]) -> bool {
        let text = word_text(word);
        let network = self
            .registry
            .parse(&text, &self.language, self.configuration);
        (self.acceptance)(&network, &text)
    }

    fn equivalence(&self, hypothesis: &Dfa, config: &ActiveLearningConfig) -> Option<Word> {
        sampled_counterexample(hypothesis, &self.alphabet, config, |word| {
            self.membership(word)
        })
    }
}

/// Default parser acceptance predicate.
///
/// A parser accepts when it reconstructs the queried text, has no error or
/// missing links, and emits at least one source-spanned structural parser link
/// (`Grammar` or `Syntax`). The spanned structural-link check distinguishes
/// successful registered parsers from lossless fallback tokenization and from
/// unspanned self-description metadata.
#[must_use]
pub fn clean_structural_acceptance(network: &LinkNetwork, text: &str) -> bool {
    network.reconstruct_text() == text
        && network.verify_full_match(None).is_clean()
        && network.links().any(|link| {
            link.metadata().span().is_some()
                && matches!(
                    link.metadata().link_type(),
                    Some(LinkType::Grammar | LinkType::Syntax)
                )
        })
}

/// Learns a DFA via L* against `oracle`.
pub fn learn_dfa(
    oracle: &dyn Oracle,
    config: &ActiveLearningConfig,
) -> Result<Dfa, ActiveLearningError> {
    validate_config(config)?;
    validate_alphabet(oracle.alphabet())?;
    if config.use_ttt {
        return Err(ActiveLearningError::TttUnavailable);
    }

    let mut table = ObservationTable::new(oracle);
    for _ in 0..config.max_iterations {
        table.close_and_consistent();
        let hypothesis = table.hypothesis()?;
        hypothesis.validate()?;

        if let Some(counterexample) = oracle.equivalence(&hypothesis, config) {
            table.add_counterexample(&counterexample);
        } else {
            return Ok(hypothesis);
        }
    }

    Err(ActiveLearningError::MaxIterations {
        max_iterations: config.max_iterations,
    })
}

/// Learns a DFA via L* and lowers it to a right-linear grammar.
pub fn learn_grammar(
    oracle: &dyn Oracle,
    config: &ActiveLearningConfig,
) -> Result<Grammar, ActiveLearningError> {
    learn_dfa(oracle, config).map(|dfa| dfa.to_grammar())
}

struct ObservationTable<'oracle> {
    oracle: &'oracle dyn Oracle,
    alphabet: Vec<Symbol>,
    prefixes: Vec<Word>,
    suffixes: Vec<Word>,
    table: BTreeMap<(Word, Word), bool>,
}

impl<'oracle> ObservationTable<'oracle> {
    fn new(oracle: &'oracle dyn Oracle) -> Self {
        Self {
            oracle,
            alphabet: oracle.alphabet().to_vec(),
            prefixes: vec![Vec::new()],
            suffixes: vec![Vec::new()],
            table: BTreeMap::new(),
        }
    }

    fn close_and_consistent(&mut self) {
        loop {
            self.fill();
            if let Some(prefix) = self.unclosed_prefix() {
                self.add_prefix_closure(&prefix);
                continue;
            }
            if let Some(suffix) = self.inconsistent_suffix() {
                self.add_suffix_closure(&suffix);
                continue;
            }
            break;
        }
    }

    fn hypothesis(&mut self) -> Result<Dfa, ActiveLearningError> {
        self.fill();

        let prefixes = self.prefixes.clone();
        let mut signature_states = BTreeMap::new();
        let mut representatives = Vec::new();
        for prefix in prefixes {
            let signature = self.row(&prefix);
            if let Entry::Vacant(entry) = signature_states.entry(signature) {
                let state = representatives.len();
                entry.insert(state);
                representatives.push(prefix);
            }
        }

        let states = representatives.len();
        let start_signature = self.row(&[]);
        let start = signature_states
            .get(&start_signature)
            .copied()
            .ok_or_else(|| ActiveLearningError::InvalidDfa {
                reason: "observation table has no start row".to_string(),
            })?;

        let mut accepting = vec![false; states];
        let mut delta = vec![vec![0; self.alphabet.len()]; states];
        for (state, representative) in representatives.iter().enumerate() {
            accepting[state] = self.value(representative, &[]);
            for (symbol_index, symbol) in self.alphabet.clone().into_iter().enumerate() {
                let successor = extend(representative, symbol);
                let signature = self.row(&successor);
                let target = signature_states.get(&signature).copied().ok_or_else(|| {
                    ActiveLearningError::InvalidDfa {
                        reason: "closed table did not contain a successor row".to_string(),
                    }
                })?;
                delta[state][symbol_index] = target;
            }
        }

        Ok(Dfa {
            alphabet: self.alphabet.clone(),
            states,
            start,
            accepting,
            delta,
        })
    }

    fn fill(&mut self) {
        let mut rows = self.prefixes.clone();
        rows.extend(self.lower_prefixes());
        let suffixes = self.suffixes.clone();
        for row in rows {
            for suffix in &suffixes {
                self.value(&row, suffix);
            }
        }
    }

    fn unclosed_prefix(&mut self) -> Option<Word> {
        let upper_signatures = self
            .prefixes
            .clone()
            .into_iter()
            .map(|prefix| self.row(&prefix))
            .collect::<BTreeSet<_>>();

        self.lower_prefixes()
            .into_iter()
            .find(|prefix| !upper_signatures.contains(&self.row(prefix)))
    }

    fn inconsistent_suffix(&mut self) -> Option<Word> {
        let prefixes = self.prefixes.clone();
        for left_index in 0..prefixes.len() {
            for right_index in (left_index + 1)..prefixes.len() {
                let left = &prefixes[left_index];
                let right = &prefixes[right_index];
                if self.row(left) != self.row(right) {
                    continue;
                }
                for symbol in self.alphabet.clone() {
                    let left_successor = extend(left, symbol);
                    let right_successor = extend(right, symbol);
                    if self.row(&left_successor) == self.row(&right_successor) {
                        continue;
                    }
                    for suffix in self.suffixes.clone() {
                        if self.value(&left_successor, &suffix)
                            != self.value(&right_successor, &suffix)
                        {
                            let mut distinguishing = vec![symbol];
                            distinguishing.extend(suffix);
                            return Some(distinguishing);
                        }
                    }
                }
            }
        }
        None
    }

    fn lower_prefixes(&self) -> Vec<Word> {
        let known = self.prefixes.iter().cloned().collect::<BTreeSet<_>>();
        let mut lower = Vec::new();
        for prefix in &self.prefixes {
            for symbol in &self.alphabet {
                let word = extend(prefix, *symbol);
                if !known.contains(&word) {
                    lower.push(word);
                }
            }
        }
        lower
    }

    fn row(&mut self, prefix: &[Symbol]) -> Vec<bool> {
        self.suffixes
            .clone()
            .into_iter()
            .map(|suffix| self.value(prefix, &suffix))
            .collect()
    }

    fn value(&mut self, prefix: &[Symbol], suffix: &[Symbol]) -> bool {
        let key = (prefix.to_vec(), suffix.to_vec());
        if let Some(value) = self.table.get(&key) {
            return *value;
        }

        let mut word = prefix.to_vec();
        word.extend(suffix);
        let value = self.oracle.membership(&word);
        self.table.insert(key, value);
        value
    }

    fn add_counterexample(&mut self, word: &[Symbol]) {
        for length in 0..=word.len() {
            self.add_prefix(&word[..length]);
        }
    }

    fn add_prefix_closure(&mut self, word: &[Symbol]) {
        for length in 0..=word.len() {
            self.add_prefix(&word[..length]);
        }
    }

    fn add_prefix(&mut self, word: &[Symbol]) {
        if !self.prefixes.iter().any(|prefix| prefix == word) {
            self.prefixes.push(word.to_vec());
        }
    }

    fn add_suffix_closure(&mut self, suffix: &[Symbol]) {
        for start in 0..=suffix.len() {
            self.add_suffix(&suffix[start..]);
        }
    }

    fn add_suffix(&mut self, suffix: &[Symbol]) {
        if !self.suffixes.iter().any(|candidate| candidate == suffix) {
            self.suffixes.push(suffix.to_vec());
        }
    }
}

fn sampled_counterexample<F>(
    hypothesis: &Dfa,
    alphabet: &[Symbol],
    config: &ActiveLearningConfig,
    membership: F,
) -> Option<Word>
where
    F: Fn(&[Symbol]) -> bool,
{
    sample_words(
        alphabet,
        config.max_word_len,
        config.equivalence_samples,
        config.seed,
    )
    .into_iter()
    .find(|word| hypothesis.accepts(word) != membership(word))
}

fn sample_words(
    alphabet: &[Symbol],
    max_word_len: usize,
    equivalence_samples: usize,
    seed: u64,
) -> Vec<Word> {
    if equivalence_samples == 0 {
        return Vec::new();
    }

    let mut words = Vec::with_capacity(equivalence_samples);
    push_unique(&mut words, Vec::new(), equivalence_samples);

    for length in 1..=max_word_len {
        for symbol in alphabet {
            push_unique(&mut words, vec![*symbol; length], equivalence_samples);
        }
    }

    if alphabet.is_empty() || words.len() >= equivalence_samples {
        words.truncate(equivalence_samples);
        return words;
    }

    let mut rng = SplitMix64::new(seed);
    let random_target = equivalence_samples.saturating_sub(words.len()) / 2;
    let mut random_added = 0usize;
    let mut attempts = 0usize;
    while random_added < random_target && attempts < equivalence_samples.saturating_mul(32) {
        attempts += 1;
        let length = rng.next_usize(max_word_len.saturating_add(1));
        let mut word = Vec::with_capacity(length);
        for _ in 0..length {
            word.push(alphabet[rng.next_usize(alphabet.len())]);
        }
        let before = words.len();
        push_unique(&mut words, word, equivalence_samples);
        if words.len() > before {
            random_added += 1;
        }
    }

    fill_exhaustive(alphabet, max_word_len, equivalence_samples, &mut words);
    words.truncate(equivalence_samples);
    words
}

fn fill_exhaustive(alphabet: &[Symbol], max_word_len: usize, limit: usize, words: &mut Vec<Word>) {
    let mut current = vec![Vec::new()];
    for _ in 0..max_word_len {
        let mut next = Vec::new();
        for prefix in &current {
            for symbol in alphabet {
                let word = extend(prefix, *symbol);
                push_unique(words, word.clone(), limit);
                next.push(word);
                if words.len() >= limit {
                    return;
                }
            }
        }
        current = next;
    }
}

fn push_unique(words: &mut Vec<Word>, word: Word, limit: usize) {
    if words.len() < limit && !words.contains(&word) {
        words.push(word);
    }
}

fn validate_config(config: &ActiveLearningConfig) -> Result<(), ActiveLearningError> {
    if config.max_iterations == 0 {
        return Err(ActiveLearningError::InvalidConfig {
            reason: "max_iterations must be greater than zero".to_string(),
        });
    }
    Ok(())
}

fn validate_alphabet(alphabet: &[Symbol]) -> Result<(), ActiveLearningError> {
    let mut seen = BTreeSet::new();
    for symbol in alphabet {
        if !seen.insert(*symbol) {
            return Err(ActiveLearningError::DuplicateSymbol { symbol: *symbol });
        }
    }
    Ok(())
}

fn extend(prefix: &[Symbol], symbol: Symbol) -> Word {
    let mut word = prefix.to_vec();
    word.push(symbol);
    word
}

fn word_text(word: &[Symbol]) -> String {
    word.iter().collect()
}

fn state_name(state: usize) -> String {
    format!("q{state}")
}

#[derive(Clone, Copy, Debug)]
struct SplitMix64 {
    state: u64,
}

impl SplitMix64 {
    const fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    fn next_u64(&mut self) -> u64 {
        self.state = self.state.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut value = self.state;
        value = (value ^ (value >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        value = (value ^ (value >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        value ^ (value >> 31)
    }

    fn next_usize(&mut self, upper: usize) -> usize {
        if upper == 0 {
            return 0;
        }
        let upper = u64::try_from(upper).unwrap_or(u64::MAX);
        usize::try_from(self.next_u64() % upper).unwrap_or(0)
    }
}
