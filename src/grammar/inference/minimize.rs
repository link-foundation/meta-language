//! MDL/Occam minimization for inferred grammars.
//!
//! The minimizer is deterministic and conservative. It greedily accepts
//! transformations that reduce a two-part MDL score while preserving recall on
//! the supplied positive examples and rejecting sampled strings that the input
//! grammar did not accept beyond the configured precision budget.

mod cost;
mod transform;

use super::eval::{sample, GrammarOracle, SampleConfig};
use crate::grammar::Grammar;
use transform::{apply_candidate, enumerate_candidates, Candidate, CandidateKind};

const DEFAULT_PRECISION_BUDGET: f64 = 0.0;
const DEFAULT_SAMPLE_BUDGET: usize = 256;
const DEFAULT_MAX_ITERATIONS: usize = 64;
const COST_EPSILON: f64 = 1e-9;

/// Two-part Minimum Description Length cost in bits. Lower is better.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Mdl {
    /// `L(G)`: bits to encode the grammar itself.
    pub grammar_bits: f64,
    /// `L(D | G)`: bits to encode the examples given the grammar.
    pub data_bits: f64,
}

impl Mdl {
    /// Returns `grammar_bits + data_bits`.
    #[must_use]
    pub fn total(self) -> f64 {
        self.grammar_bits + self.data_bits
    }
}

/// Options for [`minimize`].
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MinimizeOptions {
    /// Maximum sampled precision drop tolerated by the D1 gate.
    pub precision_budget: f64,
    /// Number of deterministic samples used by the precision gate.
    pub sample_budget: usize,
    /// Defensive cap for greedy minimization iterations and sampler depth.
    pub max_iterations: usize,
}

impl Default for MinimizeOptions {
    fn default() -> Self {
        Self {
            precision_budget: DEFAULT_PRECISION_BUDGET,
            sample_budget: DEFAULT_SAMPLE_BUDGET,
            max_iterations: DEFAULT_MAX_ITERATIONS,
        }
    }
}

/// Counts and rejection reasons recorded during minimization.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct MinimizeReport {
    /// Number of non-terminal merge candidates accepted.
    pub merges_applied: usize,
    /// Number of single-use inlining candidates accepted.
    pub inlines_applied: usize,
    /// Number of local choice factoring or deduplication candidates accepted.
    pub factorings_applied: usize,
    /// Number of unreachable-rule pruning candidates accepted.
    pub prunes_applied: usize,
    /// Number of candidates rejected because they did not strictly reduce MDL.
    pub candidates_rejected_by_mdl: usize,
    /// Number of cost-reducing candidates rejected by recall or precision gates.
    pub candidates_rejected_by_gate: usize,
}

/// Minimized grammar and before/after accounting.
#[derive(Clone, Debug, PartialEq)]
pub struct MinimizeResult {
    /// The minimized grammar.
    pub grammar: Grammar,
    /// MDL cost of the input grammar.
    pub before: Mdl,
    /// MDL cost of the minimized grammar.
    pub after: Mdl,
    /// Deterministic transform report.
    pub report: MinimizeReport,
}

/// Computes deterministic two-part MDL for `grammar` on `examples`.
///
/// The total matches the D1 [`super::eval::mdl`] encoding: `L(G)` is the public
/// grammar symbol count multiplied by a deterministic alphabet code width, and
/// `L(D | G)` charges accepted examples by emitted Unicode scalars plus a stop
/// bit while rejected examples use a UTF-8 escape penalty.
#[must_use]
pub fn mdl_cost(grammar: &Grammar, examples: &[String]) -> Mdl {
    cost::mdl_cost(grammar, examples)
}

/// Generalises and minimises an inferred grammar under an MDL objective.
///
/// The search is greedy and deterministic: candidates are enumerated in rule
/// order, scored by strict MDL decrease, gated by positive-example recall and a
/// sampled precision proxy against the input grammar, then the best admissible
/// candidate is applied until fixpoint or `opts.max_iterations`.
#[must_use]
pub fn minimize(grammar: &Grammar, examples: &[String], opts: MinimizeOptions) -> MinimizeResult {
    let before = mdl_cost(grammar, examples);
    let mut current = grammar.clone();
    let mut current_cost = before;
    let mut report = MinimizeReport::default();

    for _ in 0..opts.max_iterations {
        let candidates = enumerate_candidates(&current);
        if candidates.is_empty() {
            break;
        }

        let mut best: Option<ScoredCandidate> = None;
        for (order, candidate) in candidates.into_iter().enumerate() {
            let trial = apply_candidate(&current, candidate);
            if trial == current {
                report.candidates_rejected_by_mdl =
                    report.candidates_rejected_by_mdl.saturating_add(1);
                continue;
            }

            let trial_cost = mdl_cost(&trial, examples);
            let delta = trial_cost.total() - current_cost.total();
            if delta >= -COST_EPSILON {
                report.candidates_rejected_by_mdl =
                    report.candidates_rejected_by_mdl.saturating_add(1);
                continue;
            }

            if !passes_gate(grammar, &trial, examples, opts) {
                report.candidates_rejected_by_gate =
                    report.candidates_rejected_by_gate.saturating_add(1);
                continue;
            }

            let scored = ScoredCandidate {
                order,
                candidate,
                grammar: trial,
                cost: trial_cost,
                delta,
            };
            if best
                .as_ref()
                .map_or(true, |best| scored.is_better_than(best))
            {
                best = Some(scored);
            }
        }

        let Some(best) = best else {
            break;
        };

        record_acceptance(&mut report, best.candidate.kind());
        current = best.grammar;
        current_cost = best.cost;
    }

    MinimizeResult {
        grammar: current,
        before,
        after: current_cost,
        report,
    }
}

#[derive(Clone, Debug, PartialEq)]
struct ScoredCandidate {
    order: usize,
    candidate: Candidate,
    grammar: Grammar,
    cost: Mdl,
    delta: f64,
}

impl ScoredCandidate {
    fn is_better_than(&self, other: &Self) -> bool {
        self.delta < other.delta - COST_EPSILON
            || ((self.delta - other.delta).abs() <= COST_EPSILON && self.order < other.order)
    }
}

fn record_acceptance(report: &mut MinimizeReport, kind: CandidateKind) {
    match kind {
        CandidateKind::Merge => {
            report.merges_applied = report.merges_applied.saturating_add(1);
        }
        CandidateKind::Inline => {
            report.inlines_applied = report.inlines_applied.saturating_add(1);
        }
        CandidateKind::Factor => {
            report.factorings_applied = report.factorings_applied.saturating_add(1);
        }
        CandidateKind::Prune => {
            report.prunes_applied = report.prunes_applied.saturating_add(1);
        }
    }
}

fn passes_gate(
    baseline: &Grammar,
    trial: &Grammar,
    examples: &[String],
    opts: MinimizeOptions,
) -> bool {
    let trial_oracle = GrammarOracle::new(trial);
    if !examples.iter().all(|example| trial_oracle.accepts(example)) {
        return false;
    }

    if opts.sample_budget == 0 {
        return true;
    }

    let config = SampleConfig {
        count: opts.sample_budget.max(1),
        max_depth: opts.max_iterations.max(1),
        ..SampleConfig::default()
    };
    let Ok(samples) = sample(trial, &config) else {
        return false;
    };
    if samples.is_empty() {
        return false;
    }

    let baseline_oracle = GrammarOracle::new(baseline);
    let accepted = samples
        .iter()
        .filter(|sample| baseline_oracle.accepts(sample))
        .count();
    let precision = ratio(accepted, samples.len());
    let budget = if opts.precision_budget.is_finite() {
        opts.precision_budget.max(0.0)
    } else {
        DEFAULT_PRECISION_BUDGET
    };

    precision + budget + COST_EPSILON >= 1.0
}

fn ratio(numerator: usize, denominator: usize) -> f64 {
    debug_assert!(denominator > 0);
    usize_to_f64(numerator) / usize_to_f64(denominator)
}

#[allow(clippy::cast_precision_loss)]
const fn usize_to_f64(value: usize) -> f64 {
    value as f64
}
