//! Deterministic evaluation utilities for inferred grammars.

mod metrics;
mod recognizer;
mod sampler;

use std::error::Error;
use std::fmt;

use crate::grammar::Grammar;

/// Decides language membership for a target language.
pub trait MembershipOracle {
    /// Returns `true` when `text` belongs to the oracle's language.
    fn accepts(&self, text: &str) -> bool;
}

/// [`Grammar`] backed membership oracle.
#[derive(Clone, Copy, Debug)]
pub struct GrammarOracle<'g>(pub &'g Grammar);

impl<'g> GrammarOracle<'g> {
    /// Builds an oracle over `grammar`.
    #[must_use]
    pub const fn new(grammar: &'g Grammar) -> Self {
        Self(grammar)
    }

    /// Returns `true` when `text` is accepted by the wrapped grammar.
    #[must_use]
    pub fn accepts(&self, text: &str) -> bool {
        <Self as MembershipOracle>::accepts(self, text)
    }
}

impl MembershipOracle for GrammarOracle<'_> {
    fn accepts(&self, text: &str) -> bool {
        recognizer::accepts(self.0, text)
    }
}

/// Deterministic sampler configuration.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SampleConfig {
    /// Deterministic PRNG seed.
    pub seed: u64,
    /// Number of derivations to draw before duplicate removal.
    pub count: usize,
    /// Maximum non-terminal recursion depth before shortest terminating choices are forced.
    pub max_depth: usize,
    /// Maximum generated repetitions for `*`, `+`, and unbounded counted repetition.
    pub repeat_cap: usize,
}

impl Default for SampleConfig {
    fn default() -> Self {
        Self {
            seed: 0xD1E5_EED5_17A7_E001,
            count: 256,
            max_depth: 16,
            repeat_cap: 4,
        }
    }
}

/// Primary inference-evaluation metrics.
#[derive(Clone, Debug, PartialEq)]
pub struct MetricScores {
    /// Fraction of inferred samples accepted by the golden oracle.
    pub precision: f64,
    /// Fraction of golden samples or held-out positives accepted by the inferred grammar.
    pub recall: f64,
    /// Harmonic mean of [`Self::precision`] and [`Self::recall`].
    pub f1: f64,
    /// Raw grammar size measured as rule-name symbols plus expression nodes.
    pub size_symbols: usize,
    /// Deterministic two-part MDL score in bits; lower is better.
    pub mdl_bits: f64,
}

/// Recall source used by an evaluation report.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScoringMode {
    /// Recall was measured by sampling a golden grammar.
    GoldenGrammar,
    /// Recall was measured against held-out positive corpus examples.
    Corpus,
}

/// End-to-end benchmark result for one corpus.
#[derive(Clone, Debug, PartialEq)]
pub struct BenchmarkReport {
    /// Corpus identifier.
    pub corpus: &'static str,
    /// Metric values for this corpus run.
    pub scores: MetricScores,
    /// Number of unique samples/examples considered by precision and recall.
    pub samples_drawn: usize,
    /// Sampler seed used for this run.
    pub seed: u64,
    /// Recall source used for this report.
    pub scoring_mode: ScoringMode,
}

/// Evaluation failure.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EvalError {
    /// The grammar has no start rule to sample or recognize.
    EmptyGrammar,
    /// Corpus-mode recall was requested without held-out positive examples.
    EmptyCorpus,
    /// Sampling produced no unique strings for the named source.
    EmptySample {
        /// Source that produced no samples.
        source: &'static str,
    },
    /// A reachable rule cannot produce a finite string.
    NonTerminating {
        /// Rule name that cannot terminate.
        rule: String,
    },
    /// A reachable non-terminal references a missing rule.
    UnknownRule {
        /// Missing rule name.
        rule: String,
    },
    /// A character range has its start after its end.
    InvalidCharRange {
        /// Inclusive range start.
        start: char,
        /// Inclusive range end.
        end: char,
    },
    /// A character class has no character that the sampler can emit.
    EmptyCharClass,
    /// A counted repetition has a maximum smaller than its minimum.
    InvalidRepeat {
        /// Minimum repetition count.
        min: usize,
        /// Maximum repetition count.
        max: usize,
    },
    /// A named corpus was not registered.
    CorpusNotFound {
        /// Requested corpus name.
        corpus: String,
    },
}

impl fmt::Display for EvalError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyGrammar => formatter.write_str("grammar has no start rule"),
            Self::EmptyCorpus => formatter.write_str("corpus-mode recall requires positives"),
            Self::EmptySample { source } => write!(formatter, "{source} sampling produced no text"),
            Self::NonTerminating { rule } => write!(formatter, "rule `{rule}` cannot terminate"),
            Self::UnknownRule { rule } => write!(formatter, "unknown grammar rule `{rule}`"),
            Self::InvalidCharRange { start, end } => {
                write!(formatter, "invalid character range {start:?}..={end:?}")
            }
            Self::EmptyCharClass => formatter.write_str("character class has no sampleable member"),
            Self::InvalidRepeat { min, max } => {
                write!(formatter, "invalid repetition bounds {min}..={max}")
            }
            Self::CorpusNotFound { corpus } => write!(formatter, "unknown corpus `{corpus}`"),
        }
    }
}

impl Error for EvalError {}

/// Registry entry for an in-repository golden corpus.
#[derive(Clone, Copy, Debug)]
pub struct GoldenCorpus {
    name: &'static str,
    positives: &'static [&'static str],
    golden_grammar: fn() -> Grammar,
}

impl GoldenCorpus {
    /// Builds a corpus descriptor.
    #[must_use]
    pub const fn new(
        name: &'static str,
        positives: &'static [&'static str],
        golden_grammar: fn() -> Grammar,
    ) -> Self {
        Self {
            name,
            positives,
            golden_grammar,
        }
    }

    /// Corpus identifier.
    #[must_use]
    pub const fn name(&self) -> &'static str {
        self.name
    }

    /// Held-out positive examples for corpus-mode recall or MDL data.
    #[must_use]
    pub const fn positives(&self) -> &'static [&'static str] {
        self.positives
    }

    /// Builds the corpus golden grammar.
    #[must_use]
    pub fn golden_grammar(&self) -> Grammar {
        (self.golden_grammar)()
    }
}

const LIST_POSITIVES: &[&str] = &["a", "b", "a,b", "b,a"];
const ASSIGNMENT_POSITIVES: &[&str] = &["let a=1;", "let b=2;", "let x=9;"];

/// Built-in smoke corpora for exercising the harness without vendored competitors.
pub const GOLDEN_CORPORA: &[GoldenCorpus] = &[
    GoldenCorpus::new("inference-eval:list", LIST_POSITIVES, list_corpus_grammar),
    GoldenCorpus::new(
        "inference-eval:assignment",
        ASSIGNMENT_POSITIVES,
        assignment_corpus_grammar,
    ),
];

/// Generates deterministic strings from `grammar`.
///
/// The sampler uses `SplitMix64` with the exact transition implemented in this
/// module, never system entropy. Duplicate draws are removed while preserving
/// first-seen order. Reachable rules that cannot emit any finite string return
/// [`EvalError::NonTerminating`].
pub fn sample(grammar: &Grammar, config: &SampleConfig) -> Result<Vec<String>, EvalError> {
    sampler::sample(grammar, config)
}

/// Evaluates an inferred grammar against a golden oracle.
///
/// Precision is the fraction of samples drawn from `inferred` that `golden`
/// accepts. When `golden_sampler` is present, recall is the fraction of samples
/// drawn from that grammar that `inferred` accepts. When `golden_sampler` is
/// absent, recall is the fraction of `positives` accepted by `inferred`.
pub fn evaluate(
    inferred: &Grammar,
    golden: &dyn MembershipOracle,
    golden_sampler: Option<&Grammar>,
    positives: &[&str],
    config: &SampleConfig,
) -> Result<MetricScores, EvalError> {
    evaluate_outcome(inferred, golden, golden_sampler, positives, config)
        .map(|outcome| outcome.scores)
}

/// Runs one registered corpus against an inferred grammar.
pub fn run_corpus(
    corpus: &GoldenCorpus,
    inferred: &Grammar,
    config: &SampleConfig,
) -> Result<BenchmarkReport, EvalError> {
    let golden = corpus.golden_grammar();
    let oracle = GrammarOracle(&golden);
    let outcome = evaluate_outcome(inferred, &oracle, Some(&golden), corpus.positives(), config)?;

    Ok(BenchmarkReport {
        corpus: corpus.name(),
        scores: outcome.scores,
        samples_drawn: outcome.samples_drawn,
        seed: config.seed,
        scoring_mode: outcome.scoring_mode,
    })
}

/// Runs a registered corpus by name.
pub fn run_named_corpus(
    corpus: &str,
    inferred: &Grammar,
    config: &SampleConfig,
) -> Result<BenchmarkReport, EvalError> {
    let descriptor = GOLDEN_CORPORA
        .iter()
        .find(|candidate| candidate.name() == corpus)
        .ok_or_else(|| EvalError::CorpusNotFound {
            corpus: corpus.to_string(),
        })?;
    run_corpus(descriptor, inferred, config)
}

/// Counts grammar symbols as rule-name symbols plus expression nodes.
#[must_use]
pub fn size_symbols(grammar: &Grammar) -> usize {
    metrics::size_symbols(grammar)
}

/// Computes the deterministic two-part MDL score for `grammar` and `data`.
///
/// `L(G)` is `size_symbols(G) * ceil(log2(alphabet(G)))`, where the alphabet is
/// the distinct set of rule names, terminals, character primitives, and grammar
/// operators. `L(D | G)` uses a fixed deterministic code: accepted examples cost
/// one emitted-symbol bit per Unicode scalar plus a stop bit; rejected examples
/// fall back to their UTF-8 byte length plus a 64-bit escape penalty.
#[must_use]
pub fn mdl(grammar: &Grammar, data: &[&str]) -> f64 {
    metrics::mdl(grammar, data)
}

#[derive(Clone, Debug)]
struct EvaluationOutcome {
    scores: MetricScores,
    samples_drawn: usize,
    scoring_mode: ScoringMode,
}

fn evaluate_outcome(
    inferred: &Grammar,
    golden: &dyn MembershipOracle,
    golden_sampler: Option<&Grammar>,
    positives: &[&str],
    config: &SampleConfig,
) -> Result<EvaluationOutcome, EvalError> {
    let inferred_samples = sample(inferred, config)?;
    if inferred_samples.is_empty() {
        return Err(EvalError::EmptySample { source: "inferred" });
    }

    let precision_hits = inferred_samples
        .iter()
        .filter(|text| golden.accepts(text))
        .count();
    let precision = metrics::ratio(precision_hits, inferred_samples.len());
    let inferred_oracle = GrammarOracle(inferred);

    let (recall, mdl_bits, samples_drawn, scoring_mode) =
        if let Some(golden_grammar) = golden_sampler {
            let reference_samples = sample(golden_grammar, config)?;
            if reference_samples.is_empty() {
                return Err(EvalError::EmptySample { source: "golden" });
            }
            let recall_hits = reference_samples
                .iter()
                .filter(|text| inferred_oracle.accepts(text))
                .count();
            let data = reference_samples
                .iter()
                .map(String::as_str)
                .collect::<Vec<_>>();
            (
                metrics::ratio(recall_hits, reference_samples.len()),
                mdl(inferred, &data),
                inferred_samples
                    .len()
                    .saturating_add(reference_samples.len()),
                ScoringMode::GoldenGrammar,
            )
        } else {
            if positives.is_empty() {
                return Err(EvalError::EmptyCorpus);
            }
            let recall_hits = positives
                .iter()
                .filter(|text| inferred_oracle.accepts(text))
                .count();
            (
                metrics::ratio(recall_hits, positives.len()),
                mdl(inferred, positives),
                inferred_samples.len().saturating_add(positives.len()),
                ScoringMode::Corpus,
            )
        };

    let f1 = if precision + recall == 0.0 {
        0.0
    } else {
        2.0 * precision * recall / (precision + recall)
    };

    Ok(EvaluationOutcome {
        scores: MetricScores {
            precision,
            recall,
            f1,
            size_symbols: size_symbols(inferred),
            mdl_bits,
        },
        samples_drawn,
        scoring_mode,
    })
}

fn list_corpus_grammar() -> Grammar {
    let expr = Grammar::expr();
    Grammar::builder()
        .start("list")
        .rule(
            "list",
            expr.seq([
                expr.nt("item"),
                expr.rep0(expr.seq([expr.term(","), expr.nt("item")])),
            ]),
        )
        .rule(
            "item",
            expr.choice_unordered([expr.term("a"), expr.term("b")]),
        )
        .build()
}

fn assignment_corpus_grammar() -> Grammar {
    let expr = Grammar::expr();
    Grammar::builder()
        .start("assignment")
        .rule(
            "assignment",
            expr.seq([
                expr.term("let "),
                expr.nt("letter"),
                expr.term("="),
                expr.nt("digit"),
                expr.term(";"),
            ]),
        )
        .rule("letter", expr.char_range('a', 'z'))
        .rule("digit", expr.char_range('0', '9'))
        .build()
}
