//! Competitor benchmark orchestration for the E3 grammar-inference suite.
//!
//! The metric math stays in the D1 evaluation module and the inferred grammar
//! comes from the D5 `infer_cfg` entry point. This module owns manifest loading,
//! vendored-corpus integrity checks, report formatting, and bar assertions.
//! Manifest byte counts assume the repository-pinned LF checkout policy for
//! `benches/corpora/**` so CI reports the same fixture totals on every OS.

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;
use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::Instant;

use serde_json::Value;

use crate::{
    emit_gbnf, evaluate, infer_cfg, Grammar, GrammarExpr, GrammarFormat, GrammarOracle,
    GrammarRule, InferenceOptions, MetricScores, PositiveOnlyOracle, SampleConfig, ScoringMode,
};

/// Default manifest path used by the bench and integration gate.
pub const DEFAULT_CORPUS_MANIFEST: &str = "benches/corpus-manifest.json";
/// Default vendored corpus root used by the bench and integration gate.
pub const DEFAULT_CORPORA_ROOT: &str = "benches/corpora";
/// Published `TreeVada` headline average F1 from the E3 competitive-analysis notes.
pub const PUBLISHED_TREEVADA_AVG_F1: f64 = 0.32;
/// Published `NatGI` headline average F1 from the E3 competitive-analysis notes.
pub const PUBLISHED_NATGI_AVG_F1: f64 = 0.57;
/// Deterministic D5 runs must meet the current top published average F1 bar.
pub const D5_REQUIRED_AVG_F1: f64 = PUBLISHED_NATGI_AVG_F1;

/// A parsed competitor corpus manifest.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CompetitorManifest {
    /// Manifest schema revision.
    pub schema: u64,
    /// Corpus subject entries.
    pub entries: Vec<CorpusManifestEntry>,
}

/// One vendored competitor subject.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CorpusManifestEntry {
    /// Competitor/tool name.
    pub tool: String,
    /// Subject directory under `benches/corpora/<tool>/`.
    pub subject: String,
    /// Source repository identifier.
    pub source: String,
    /// Pinned source commit.
    pub commit: String,
    /// SPDX-style license label.
    pub license: String,
    /// Expected number of vendored files below the subject directory.
    pub files: usize,
    /// Expected total byte count below the subject directory.
    pub bytes: u64,
    /// Whether the always-on gate runs this subject.
    pub included: bool,
    /// Required reason when [`Self::included`] is false.
    pub exclude_reason: String,
    /// Relative paths holding positive examples for included runs.
    pub example_paths: Vec<String>,
    /// Golden oracle mode used for included runs.
    pub golden: String,
}

impl CorpusManifestEntry {
    /// Stable `tool/subject` identifier.
    #[must_use]
    pub fn id(&self) -> String {
        format!("{}/{}", self.tool, self.subject)
    }
}

/// One skipped manifest entry rendered in the benchmark log.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SkippedCorpus {
    /// Stable `tool/subject` identifier.
    pub id: String,
    /// Explicit skip reason from the manifest.
    pub reason: String,
}

/// One included corpus result.
#[derive(Clone, Debug, PartialEq)]
pub struct CompetitorRun {
    /// Stable `tool/subject` identifier.
    pub id: String,
    /// Competitor/tool name.
    pub tool: String,
    /// Subject name.
    pub subject: String,
    /// Number of positive example files loaded.
    pub examples: usize,
    /// D1 primary metrics.
    pub scores: MetricScores,
    /// Number of unique precision/recall samples considered by D1.
    pub samples_drawn: usize,
    /// D1 deterministic sample seed.
    pub seed: u64,
    /// Recall source used by D1.
    pub scoring_mode: ScoringMode,
    /// D5 inference plus D1 evaluation wall-clock time.
    pub wall_clock_ms: u128,
    /// Number of rules emitted by D5.
    pub inferred_rules: usize,
    /// Per-subject F1 threshold used by the gate.
    pub required_f1: f64,
    /// Whether the inferred grammar emitted non-empty GBNF.
    pub gbnf_emitted: bool,
}

/// Secondary metric report row.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SecondaryMetricRow {
    /// Metric label.
    pub metric: &'static str,
    /// Reported value or pending status.
    pub value: String,
}

/// Non-fatal suite failure collected so one run reports every problem.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BenchmarkFailure {
    /// Optional corpus identifier.
    pub corpus: Option<String>,
    /// Failure details.
    pub message: String,
}

impl BenchmarkFailure {
    fn new(corpus: Option<String>, message: impl Into<String>) -> Self {
        Self {
            corpus,
            message: message.into(),
        }
    }
}

/// Full suite result.
#[derive(Clone, Debug, PartialEq)]
pub struct CompetitorSuiteReport {
    /// Deterministic D1 sample seed used for all included runs.
    pub seed: u64,
    /// Included corpus runs.
    pub runs: Vec<CompetitorRun>,
    /// Explicitly skipped subjects.
    pub skipped: Vec<SkippedCorpus>,
    /// Secondary metric rows.
    pub secondary: Vec<SecondaryMetricRow>,
    /// Manifest, loading, metric, or bar failures.
    pub failures: Vec<BenchmarkFailure>,
}

/// Fatal setup error that prevents a suite report from being built.
#[derive(Debug)]
pub enum BenchmarkError {
    /// File-system access failed.
    Io {
        /// Path being accessed.
        path: PathBuf,
        /// Source I/O error.
        source: std::io::Error,
    },
    /// JSON parsing failed.
    Json {
        /// Manifest path.
        path: PathBuf,
        /// Source parser error.
        source: serde_json::Error,
    },
    /// Manifest structure was invalid.
    Manifest(String),
}

impl fmt::Display for BenchmarkError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io { path, source } => {
                write!(formatter, "{}: {source}", path.display())
            }
            Self::Json { path, source } => {
                write!(formatter, "{}: {source}", path.display())
            }
            Self::Manifest(message) => formatter.write_str(message),
        }
    }
}

impl Error for BenchmarkError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Io { source, .. } => Some(source),
            Self::Json { source, .. } => Some(source),
            Self::Manifest(_) => None,
        }
    }
}

/// Loads the default manifest and runs the default vendored competitor suite.
///
/// # Errors
///
/// Returns an error when the manifest cannot be read or parsed.
pub fn run_competitor_suite(
    config: &SampleConfig,
) -> Result<CompetitorSuiteReport, BenchmarkError> {
    run_competitor_suite_from_paths(
        Path::new(DEFAULT_CORPUS_MANIFEST),
        Path::new(DEFAULT_CORPORA_ROOT),
        config,
    )
}

/// Loads a manifest and runs a vendored competitor suite from explicit paths.
///
/// # Errors
///
/// Returns an error when the manifest cannot be read or parsed.
pub fn run_competitor_suite_from_paths(
    manifest_path: &Path,
    corpora_root: &Path,
    config: &SampleConfig,
) -> Result<CompetitorSuiteReport, BenchmarkError> {
    let manifest = load_manifest(manifest_path)?;
    let mut report = validate_manifest(&manifest, corpora_root)?;
    report.seed = config.seed;

    for entry in manifest.entries.iter().filter(|entry| entry.included) {
        match run_manifest_entry(entry, corpora_root, config) {
            Ok(run) => {
                if run.scores.f1 < run.required_f1 {
                    report.failures.push(BenchmarkFailure::new(
                        Some(run.id.clone()),
                        format!(
                            "F1 {:.3} is below required NatGI bar {:.3}",
                            run.scores.f1, run.required_f1
                        ),
                    ));
                }
                report.runs.push(run);
            }
            Err(message) => {
                report
                    .failures
                    .push(BenchmarkFailure::new(Some(entry.id()), message));
            }
        }
    }

    if report.runs.is_empty() {
        report.failures.push(BenchmarkFailure::new(
            None,
            "manifest includes no always-on competitor corpus subjects",
        ));
    }

    report.secondary = secondary_rows(&report.runs);
    Ok(report)
}

/// Parses a competitor corpus manifest.
///
/// # Errors
///
/// Returns an error when the manifest cannot be read or parsed.
pub fn load_manifest(path: &Path) -> Result<CompetitorManifest, BenchmarkError> {
    let text = fs::read_to_string(path).map_err(|source| BenchmarkError::Io {
        path: path.to_path_buf(),
        source,
    })?;
    let value = serde_json::from_str::<Value>(&text).map_err(|source| BenchmarkError::Json {
        path: path.to_path_buf(),
        source,
    })?;
    parse_manifest(&value)
}

/// Formats a suite report as the CI failure log and bench output.
#[must_use]
pub fn render_competitor_report(report: &CompetitorSuiteReport) -> String {
    let mut output = String::new();
    let _ = writeln!(
        output,
        "D1/D5 competitor benchmark suite (seed {})",
        report.seed
    );
    let _ = writeln!(
        output,
        "Published bars: TreeVada avg F1 ~= {PUBLISHED_TREEVADA_AVG_F1:.2}; NatGI avg F1 ~= {PUBLISHED_NATGI_AVG_F1:.2}"
    );

    if !report.skipped.is_empty() {
        let _ = writeln!(output);
        for skipped in &report.skipped {
            let _ = writeln!(output, "SKIPPED {}: {}", skipped.id, skipped.reason);
        }
    }

    let _ = writeln!(output);
    let _ = writeln!(
        output,
        "| corpus | examples | precision | recall | F1 | required F1 | wall-clock ms | rules | samples |"
    );
    let _ = writeln!(output, "|---|---:|---:|---:|---:|---:|---:|---:|---:|");
    for run in &report.runs {
        let _ = writeln!(
            output,
            "| {} | {} | {:.3} | {:.3} | {:.3} | {:.3} | {} | {} | {} |",
            run.id,
            run.examples,
            run.scores.precision,
            run.scores.recall,
            run.scores.f1,
            run.required_f1,
            run.wall_clock_ms,
            run.inferred_rules,
            run.samples_drawn
        );
    }

    let _ = writeln!(output);
    let _ = writeln!(output, "| secondary metric | value |");
    let _ = writeln!(output, "|---|---|");
    for row in &report.secondary {
        let _ = writeln!(output, "| {} | {} |", row.metric, row.value);
    }

    if !report.failures.is_empty() {
        let _ = writeln!(output);
        let _ = writeln!(output, "Failures:");
        for failure in &report.failures {
            match &failure.corpus {
                Some(corpus) => {
                    let _ = writeln!(output, "- {corpus}: {}", failure.message);
                }
                None => {
                    let _ = writeln!(output, "- {}", failure.message);
                }
            }
        }
    }

    output
}

fn parse_manifest(value: &Value) -> Result<CompetitorManifest, BenchmarkError> {
    let object = value
        .as_object()
        .ok_or_else(|| BenchmarkError::Manifest("manifest root must be a JSON object".into()))?;
    let schema = required_u64(object, "schema")?;
    let corpus = object
        .get("corpus")
        .and_then(Value::as_array)
        .ok_or_else(|| BenchmarkError::Manifest("manifest corpus must be an array".into()))?;
    let mut entries = Vec::with_capacity(corpus.len());
    for (index, entry) in corpus.iter().enumerate() {
        let object = entry.as_object().ok_or_else(|| {
            BenchmarkError::Manifest(format!("manifest corpus[{index}] must be an object"))
        })?;
        entries.push(CorpusManifestEntry {
            tool: required_string(object, "tool")?,
            subject: required_string(object, "subject")?,
            source: required_string(object, "source")?,
            commit: required_string(object, "commit")?,
            license: required_string(object, "license")?,
            files: usize::try_from(required_u64(object, "files")?).map_err(|_| {
                BenchmarkError::Manifest(format!("manifest corpus[{index}].files overflows usize"))
            })?,
            bytes: required_u64(object, "bytes")?,
            included: required_bool(object, "included")?,
            exclude_reason: required_string(object, "exclude_reason")?,
            example_paths: required_string_array(object, "example_paths")?,
            golden: required_string(object, "golden")?,
        });
    }

    Ok(CompetitorManifest { schema, entries })
}

fn required_string(
    object: &serde_json::Map<String, Value>,
    key: &str,
) -> Result<String, BenchmarkError> {
    object
        .get(key)
        .and_then(Value::as_str)
        .map(ToOwned::to_owned)
        .ok_or_else(|| BenchmarkError::Manifest(format!("manifest field `{key}` must be a string")))
}

fn required_u64(object: &serde_json::Map<String, Value>, key: &str) -> Result<u64, BenchmarkError> {
    object
        .get(key)
        .and_then(Value::as_u64)
        .ok_or_else(|| BenchmarkError::Manifest(format!("manifest field `{key}` must be a u64")))
}

fn required_bool(
    object: &serde_json::Map<String, Value>,
    key: &str,
) -> Result<bool, BenchmarkError> {
    object
        .get(key)
        .and_then(Value::as_bool)
        .ok_or_else(|| BenchmarkError::Manifest(format!("manifest field `{key}` must be a bool")))
}

fn required_string_array(
    object: &serde_json::Map<String, Value>,
    key: &str,
) -> Result<Vec<String>, BenchmarkError> {
    object
        .get(key)
        .and_then(Value::as_array)
        .ok_or_else(|| {
            BenchmarkError::Manifest(format!("manifest field `{key}` must be an array"))
        })?
        .iter()
        .map(|value| {
            value.as_str().map(ToOwned::to_owned).ok_or_else(|| {
                BenchmarkError::Manifest(format!("manifest field `{key}` must contain strings"))
            })
        })
        .collect()
}

fn validate_manifest(
    manifest: &CompetitorManifest,
    corpora_root: &Path,
) -> Result<CompetitorSuiteReport, BenchmarkError> {
    let mut report = CompetitorSuiteReport {
        seed: 0,
        runs: Vec::new(),
        skipped: Vec::new(),
        secondary: Vec::new(),
        failures: Vec::new(),
    };
    let mut entries = BTreeMap::<String, &CorpusManifestEntry>::new();

    for entry in &manifest.entries {
        let id = entry.id();
        if entries.insert(id.clone(), entry).is_some() {
            report.failures.push(BenchmarkFailure::new(
                Some(id.clone()),
                "duplicate manifest entry",
            ));
        }

        if !entry.included {
            if entry.exclude_reason.trim().is_empty() {
                report.failures.push(BenchmarkFailure::new(
                    Some(id.clone()),
                    "excluded corpus must provide a non-empty exclude_reason",
                ));
            } else {
                report.skipped.push(SkippedCorpus {
                    id: id.clone(),
                    reason: entry.exclude_reason.clone(),
                });
            }
        }

        let subject_dir = corpora_root.join(&entry.tool).join(&entry.subject);
        if !subject_dir.is_dir() {
            report.failures.push(BenchmarkFailure::new(
                Some(id.clone()),
                format!(
                    "vendored subject directory {} is missing",
                    subject_dir.display()
                ),
            ));
            continue;
        }

        let (files, bytes) =
            count_subject_files(&subject_dir).map_err(|source| BenchmarkError::Io {
                path: subject_dir.clone(),
                source,
            })?;
        if files != entry.files {
            report.failures.push(BenchmarkFailure::new(
                Some(id.clone()),
                format!("manifest files={} but vendored files={files}", entry.files),
            ));
        }
        if bytes != entry.bytes {
            report.failures.push(BenchmarkFailure::new(
                Some(id.clone()),
                format!("manifest bytes={} but vendored bytes={bytes}", entry.bytes),
            ));
        }

        if entry.included && entry.example_paths.is_empty() {
            report.failures.push(BenchmarkFailure::new(
                Some(id),
                "included corpus must list at least one example path",
            ));
        }
    }

    for subject in vendored_subjects(corpora_root)? {
        if !entries.contains_key(&subject) {
            report.failures.push(BenchmarkFailure::new(
                Some(subject),
                "vendored subject is missing from manifest",
            ));
        }
    }

    Ok(report)
}

fn run_manifest_entry(
    entry: &CorpusManifestEntry,
    corpora_root: &Path,
    config: &SampleConfig,
) -> Result<CompetitorRun, String> {
    if entry.golden != "exact_examples" {
        return Err(format!(
            "unsupported included golden oracle mode `{}`",
            entry.golden
        ));
    }

    let id = entry.id();
    let subject_dir = corpora_root.join(&entry.tool).join(&entry.subject);
    let examples = load_examples(&subject_dir, &entry.example_paths)
        .map_err(|error| format!("failed to load examples: {error}"))?;
    if examples.is_empty() {
        return Err("included corpus loaded no positive examples".into());
    }

    let start = Instant::now();
    let inferred = infer_cfg(&examples, &PositiveOnlyOracle, InferenceOptions::default());
    let golden = exact_examples_grammar(&examples);
    let oracle = GrammarOracle::new(&golden);
    let positive_refs = examples.iter().map(String::as_str).collect::<Vec<_>>();
    let scores = evaluate(
        &inferred.grammar,
        &oracle,
        Some(&golden),
        &positive_refs,
        config,
    )
    .map_err(|error| format!("D1 evaluation failed: {error}"))?;
    let samples_drawn = sample_count(&inferred.grammar, &golden, config)
        .map_err(|error| format!("D1 sample accounting failed: {error}"))?;
    let gbnf_emitted = emit_gbnf(&inferred.grammar).is_ok_and(|(text, _)| !text.trim().is_empty());

    Ok(CompetitorRun {
        id,
        tool: entry.tool.clone(),
        subject: entry.subject.clone(),
        examples: examples.len(),
        scores,
        samples_drawn,
        seed: config.seed,
        scoring_mode: ScoringMode::GoldenGrammar,
        wall_clock_ms: start.elapsed().as_millis(),
        inferred_rules: inferred.report.rules,
        required_f1: D5_REQUIRED_AVG_F1,
        gbnf_emitted,
    })
}

fn sample_count(
    inferred: &Grammar,
    golden: &Grammar,
    config: &SampleConfig,
) -> Result<usize, crate::EvalError> {
    let inferred_count = crate::sample(inferred, config)?.len();
    let golden_count = crate::sample(golden, config)?.len();
    Ok(inferred_count.saturating_add(golden_count))
}

fn exact_examples_grammar(examples: &[String]) -> Grammar {
    let alternatives = examples.iter().map(|example| {
        if example.is_empty() {
            GrammarExpr::Empty
        } else {
            GrammarExpr::Terminal(example.clone())
        }
    });
    Grammar::new()
        .with_source_format(GrammarFormat::Inferred)
        .with_rule(GrammarRule::new(
            "Root",
            finish_choice(alternatives.collect()),
        ))
        .with_start("Root")
}

fn finish_choice(alternatives: Vec<GrammarExpr>) -> GrammarExpr {
    let mut unique = BTreeMap::<String, GrammarExpr>::new();
    for alternative in alternatives {
        unique
            .entry(format!("{alternative:?}"))
            .or_insert(alternative);
    }

    match unique.len() {
        0 => GrammarExpr::Empty,
        1 => unique
            .into_values()
            .next()
            .expect("one choice alternative must exist"),
        _ => GrammarExpr::Choice {
            ordered: false,
            alternatives: unique.into_values().collect(),
        },
    }
}

fn load_examples(subject_dir: &Path, example_paths: &[String]) -> Result<Vec<String>, String> {
    let mut files = Vec::new();
    for relative in example_paths {
        let path = subject_dir.join(relative);
        if path.is_file() {
            files.push(path);
        } else if path.is_dir() {
            collect_files(&path, &mut files)
                .map_err(|error| format!("{}: {error}", path.display()))?;
        } else {
            return Err(format!("example path {} is missing", path.display()));
        }
    }
    files.sort();
    files.dedup();

    files
        .iter()
        .map(|path| {
            fs::read_to_string(path).map_err(|error| format!("{}: {error}", path.display()))
        })
        .collect()
}

fn count_subject_files(path: &Path) -> Result<(usize, u64), std::io::Error> {
    let mut files = Vec::new();
    collect_files(path, &mut files)?;
    let mut bytes = 0u64;
    for file in &files {
        bytes = bytes.saturating_add(fs::metadata(file)?.len());
    }
    Ok((files.len(), bytes))
}

fn collect_files(path: &Path, files: &mut Vec<PathBuf>) -> Result<(), std::io::Error> {
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            collect_files(&path, files)?;
        } else if path.is_file() {
            files.push(path);
        }
    }
    Ok(())
}

fn vendored_subjects(corpora_root: &Path) -> Result<BTreeSet<String>, BenchmarkError> {
    let mut subjects = BTreeSet::new();
    let tools = fs::read_dir(corpora_root).map_err(|source| BenchmarkError::Io {
        path: corpora_root.to_path_buf(),
        source,
    })?;
    for tool in tools {
        let tool = tool.map_err(|source| BenchmarkError::Io {
            path: corpora_root.to_path_buf(),
            source,
        })?;
        let tool_path = tool.path();
        if !tool_path.is_dir() {
            continue;
        }
        let tool_name = tool.file_name().to_string_lossy().into_owned();
        for subject in fs::read_dir(&tool_path).map_err(|source| BenchmarkError::Io {
            path: tool_path.clone(),
            source,
        })? {
            let subject = subject.map_err(|source| BenchmarkError::Io {
                path: tool_path.clone(),
                source,
            })?;
            let subject_path = subject.path();
            if subject_path.is_dir() {
                let subject_name = subject.file_name().to_string_lossy().into_owned();
                subjects.insert(format!("{tool_name}/{subject_name}"));
            }
        }
    }
    Ok(subjects)
}

fn secondary_rows(runs: &[CompetitorRun]) -> Vec<SecondaryMetricRow> {
    let gbnf_successes = runs.iter().filter(|run| run.gbnf_emitted).count();

    vec![
        SecondaryMetricRow {
            metric: "format coverage",
            value: "n/a (pending B*/C* cross-format coverage aggregation)".to_string(),
        },
        SecondaryMetricRow {
            metric: "round-trip fidelity",
            value: "n/a (pending F2 fidelity matrix)".to_string(),
        },
        SecondaryMetricRow {
            metric: "GBNF emit",
            value: format!(
                "{gbnf_successes}/{} included grammars emitted non-empty GBNF",
                runs.len()
            ),
        },
        SecondaryMetricRow {
            metric: "cross-language translation",
            value: "n/a (pending C6 full metric wiring)".to_string(),
        },
    ]
}
