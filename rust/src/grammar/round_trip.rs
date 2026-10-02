//! Mutation-guarded grammar interchange round trips.
//!
//! A plain round trip (import, emit, re-import, compare) cannot tell a correct
//! importer/emitter pair from two that are wrong in mutually cancelling ways,
//! and it cannot tell an emitter that re-emits a cached source from one that
//! prints the grammar it was given. The guard therefore changes the grammar
//! before export, requires the change in the exported text and in the
//! re-imported grammar, and checks both the imported and the re-imported
//! grammar against independent accept and reject samples that do not come
//! from the pair under test. It mirrors `js/src/grammar-round-trip.js`.

use std::error::Error;
use std::fmt;

use super::emit::{EmitReport, GrammarEmitError};
use super::import::GrammarImportError;
use super::merge::{GrammarMergeError, normalized_rule_definition};
use super::runtime::GrammarParser;
use super::{CharClassItem, Grammar, GrammarExpr, GrammarFormat, GrammarRule};

/// The literal alternative the guard adds to the start rule by default.
pub const GRAMMAR_ROUND_TRIP_MARKER: &str = "round-trip-mutation";

/// Imports grammar source text of one notation.
pub type GrammarImportFn<'a> = &'a dyn Fn(&str) -> Result<Grammar, GrammarImportError>;

/// Emits a grammar in one notation, with its fidelity report.
pub type GrammarEmitFn<'a> = &'a dyn Fn(&Grammar) -> Result<(String, EmitReport), GrammarEmitError>;

/// The importer/emitter pair under test and the independent samples.
#[derive(Clone, Copy)]
pub struct GrammarRoundTrip<'a> {
    /// Importer of the notation.
    pub import: GrammarImportFn<'a>,
    /// Emitter of the same notation.
    pub emit: GrammarEmitFn<'a>,
    /// Literal alternative added to the start rule before export.
    pub marker: &'a str,
    /// Texts the grammar must accept.
    pub accepts: &'a [String],
    /// Texts the grammar must reject.
    pub rejects: &'a [String],
}

impl<'a> GrammarRoundTrip<'a> {
    /// Builds a round trip through `import` and `emit` with the default
    /// marker and no samples.
    #[must_use]
    pub const fn new(import: GrammarImportFn<'a>, emit: GrammarEmitFn<'a>) -> Self {
        Self {
            import,
            emit,
            marker: GRAMMAR_ROUND_TRIP_MARKER,
            accepts: &[],
            rejects: &[],
        }
    }

    /// Returns this round trip with independent accept and reject samples.
    #[must_use]
    pub const fn with_samples(mut self, accepts: &'a [String], rejects: &'a [String]) -> Self {
        self.accepts = accepts;
        self.rejects = rejects;
        self
    }
}

/// Whether the pair carried the grammar and the mutation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GrammarRoundTripStatus {
    /// Every check held.
    Preserved,
    /// At least one check failed.
    Broken,
}

impl GrammarRoundTripStatus {
    /// Stable tag shared with the JavaScript runtime.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Preserved => "preserved",
            Self::Broken => "broken",
        }
    }
}

/// What a failed check found.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GrammarRoundTripFailureKind {
    /// A text the grammar must accept was rejected.
    SampleRejected,
    /// A text the grammar must reject was accepted.
    SampleAccepted,
    /// The imported grammar already accepts the marker.
    MarkerAlreadyAccepted,
    /// The emitter reported a lossy conversion of the mutated grammar.
    LossyExport,
    /// The exported text does not carry the mutation.
    MutationNotExported,
    /// The re-imported rules differ from the mutated ones.
    RulesChanged,
    /// The re-imported grammar does not accept the marker.
    MutationNotVisible,
    /// A second export does not settle on the same rules and text.
    ExportNotStable,
}

impl GrammarRoundTripFailureKind {
    /// Stable tag shared with the JavaScript runtime.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SampleRejected => "sample-rejected",
            Self::SampleAccepted => "sample-accepted",
            Self::MarkerAlreadyAccepted => "marker-already-accepted",
            Self::LossyExport => "lossy-export",
            Self::MutationNotExported => "mutation-not-exported",
            Self::RulesChanged => "rules-changed",
            Self::MutationNotVisible => "mutation-not-visible",
            Self::ExportNotStable => "export-not-stable",
        }
    }
}

/// The grammar a failed check looked at.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GrammarRoundTripStage {
    /// The grammar imported from the source.
    Imported,
    /// The exported text of the mutated grammar.
    Exported,
    /// The grammar re-imported from the exported text.
    Reimported,
}

impl GrammarRoundTripStage {
    /// Stable tag shared with the JavaScript runtime.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Imported => "imported",
            Self::Exported => "exported",
            Self::Reimported => "reimported",
        }
    }
}

/// One failed check.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GrammarRoundTripFailure {
    /// What the check found.
    pub kind: GrammarRoundTripFailureKind,
    /// The grammar it looked at.
    pub stage: GrammarRoundTripStage,
    /// The sample, marker, note or rule involved.
    pub detail: String,
}

/// The outcome of [`check_grammar_round_trip`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GrammarRoundTripReport {
    /// Whether every check held.
    pub status: GrammarRoundTripStatus,
    /// Every failed check, in the order the guard ran them.
    pub failures: Vec<GrammarRoundTripFailure>,
    /// The imported grammar with the marker alternative.
    pub mutated: Grammar,
    /// The exported text of the mutated grammar.
    pub exported: String,
    /// The grammar re-imported from the exported text.
    pub reimported: Grammar,
}

/// Why the guard could not run.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GrammarRoundTripError {
    /// The importer rejected a source, such as a malformed grammar.
    Import(GrammarImportError),
    /// The emitter rejected a grammar.
    Emit(GrammarEmitError),
    /// A rule definition could not be normalized for comparison.
    Normalize(GrammarMergeError),
    /// The grammar has no start rule to mutate.
    MissingStartRule,
}

impl fmt::Display for GrammarRoundTripError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Import(error) => write!(formatter, "{error}"),
            Self::Emit(error) => write!(formatter, "{error}"),
            Self::Normalize(error) => write!(formatter, "{error}"),
            Self::MissingStartRule => {
                formatter.write_str("the grammar has no start rule to mutate")
            }
        }
    }
}

impl Error for GrammarRoundTripError {}

impl From<GrammarImportError> for GrammarRoundTripError {
    fn from(error: GrammarImportError) -> Self {
        Self::Import(error)
    }
}

impl From<GrammarEmitError> for GrammarRoundTripError {
    fn from(error: GrammarEmitError) -> Self {
        Self::Emit(error)
    }
}

impl From<GrammarMergeError> for GrammarRoundTripError {
    fn from(error: GrammarMergeError) -> Self {
        Self::Normalize(error)
    }
}

/// Adds `marker` as a literal alternative of the start rule.
///
/// PEG grammars get an ordered choice, every other notation an unordered one,
/// so the mutation is expressible without a lossy fallback.
pub fn mutate_grammar_start_rule(
    grammar: &Grammar,
    marker: &str,
) -> Result<Grammar, GrammarRoundTripError> {
    let start = grammar
        .start_rule()
        .ok_or(GrammarRoundTripError::MissingStartRule)?
        .name
        .clone();
    let ordered = grammar.source_format() == Some(GrammarFormat::Peg);
    let mut mutated = Grammar::new();
    for rule in grammar.rules() {
        let mut rule = rule.clone();
        if rule.name == start {
            rule.expr = GrammarExpr::Choice {
                ordered,
                alternatives: vec![rule.expr, GrammarExpr::Terminal(marker.to_owned())],
            };
        }
        mutated.add_rule(rule);
    }
    if let Some(name) = grammar.start() {
        mutated.set_start(name);
    }
    if let Some(format) = grammar.source_format() {
        mutated.set_source_format(format);
    }
    Ok(mutated)
}

/// Round trips `source` through a mutated export and reports every failure.
///
/// The grammar is imported, mutated, exported and re-imported, and every way
/// the pair failed to carry the grammar and the mutation is reported.
/// Malformed sources are rejected by the importer, whose error is returned.
pub fn check_grammar_round_trip(
    source: &str,
    round_trip: &GrammarRoundTrip<'_>,
) -> Result<GrammarRoundTripReport, GrammarRoundTripError> {
    let mut failures = Vec::new();
    let imported = (round_trip.import)(source)?;
    check_samples(
        &mut failures,
        GrammarRoundTripStage::Imported,
        &imported,
        round_trip,
    );
    if accepts(&imported, round_trip.marker) {
        failures.push(failure(
            GrammarRoundTripFailureKind::MarkerAlreadyAccepted,
            GrammarRoundTripStage::Imported,
            round_trip.marker,
        ));
    }

    let mutated = mutate_grammar_start_rule(&imported, round_trip.marker)?;
    let (unmutated, _) = (round_trip.emit)(&imported)?;
    let (exported, report) = (round_trip.emit)(&mutated)?;
    for note in &report.lossy {
        failures.push(failure(
            GrammarRoundTripFailureKind::LossyExport,
            GrammarRoundTripStage::Exported,
            note,
        ));
    }
    if exported == unmutated || !exported.contains(round_trip.marker) {
        failures.push(failure(
            GrammarRoundTripFailureKind::MutationNotExported,
            GrammarRoundTripStage::Exported,
            round_trip.marker,
        ));
    }

    let reimported = (round_trip.import)(&exported)?;
    compare_rules(&mut failures, &mutated, &reimported)?;
    if !accepts(&reimported, round_trip.marker) {
        failures.push(failure(
            GrammarRoundTripFailureKind::MutationNotVisible,
            GrammarRoundTripStage::Reimported,
            round_trip.marker,
        ));
    }
    check_samples(
        &mut failures,
        GrammarRoundTripStage::Reimported,
        &reimported,
        round_trip,
    );
    // A notation may print an equivalent construct differently once (ABNF has
    // no character classes), so the second export must carry the same rules
    // and be a fixpoint rather than repeat the first text.
    let (again, _) = (round_trip.emit)(&reimported)?;
    let settled = (round_trip.import)(&again)?;
    if definitions(&mutated)? != definitions(&settled)? || (round_trip.emit)(&settled)?.0 != again {
        failures.push(failure(
            GrammarRoundTripFailureKind::ExportNotStable,
            GrammarRoundTripStage::Reimported,
            "a second export does not settle",
        ));
    }

    Ok(GrammarRoundTripReport {
        status: if failures.is_empty() {
            GrammarRoundTripStatus::Preserved
        } else {
            GrammarRoundTripStatus::Broken
        },
        failures,
        mutated,
        exported,
        reimported,
    })
}

fn failure(
    kind: GrammarRoundTripFailureKind,
    stage: GrammarRoundTripStage,
    detail: &str,
) -> GrammarRoundTripFailure {
    GrammarRoundTripFailure {
        kind,
        stage,
        detail: detail.to_owned(),
    }
}

fn compare_rules(
    failures: &mut Vec<GrammarRoundTripFailure>,
    expected: &Grammar,
    actual: &Grammar,
) -> Result<(), GrammarRoundTripError> {
    let expected_names = expected.rule_names();
    let actual_names = actual.rule_names();
    if expected_names != actual_names {
        failures.push(failure(
            GrammarRoundTripFailureKind::RulesChanged,
            GrammarRoundTripStage::Reimported,
            &format!(
                "rule names [{}] became [{}]",
                expected_names.join(", "),
                actual_names.join(", ")
            ),
        ));
    }
    let start = |grammar: &Grammar| grammar.start_rule().map(|rule| rule.name.clone());
    if start(expected) != start(actual) {
        failures.push(failure(
            GrammarRoundTripFailureKind::RulesChanged,
            GrammarRoundTripStage::Reimported,
            "the start rule changed",
        ));
    }
    for rule in expected.rules() {
        if let Some(other) = actual.rule(&rule.name)
            && rule_definition(other)? != rule_definition(rule)?
        {
            failures.push(failure(
                GrammarRoundTripFailureKind::RulesChanged,
                GrammarRoundTripStage::Reimported,
                &format!("rule {} changed its definition", rule.name),
            ));
        }
    }
    Ok(())
}

fn definitions(grammar: &Grammar) -> Result<Vec<String>, GrammarRoundTripError> {
    grammar
        .rules()
        .iter()
        .map(|rule| Ok(format!("{}={}", rule.name, rule_definition(rule)?)))
        .collect()
}

/// The meaning-aware definition of a rule, after spelling every set of single
/// characters as one unordered choice: a character class, a choice of its
/// ranges and an ordered choice of them all consume the same one character.
fn rule_definition(rule: &GrammarRule) -> Result<String, GrammarRoundTripError> {
    let canonical = GrammarRule {
        expr: canonical_characters(&rule.expr),
        ..rule.clone()
    };
    Ok(normalized_rule_definition(&canonical)?)
}

fn canonical_characters(expr: &GrammarExpr) -> GrammarExpr {
    if let Some(mut set) = character_set(expr) {
        return if set.len() == 1 {
            set.remove(0)
        } else {
            GrammarExpr::Choice {
                ordered: false,
                alternatives: set,
            }
        };
    }
    let boxed = |inner: &GrammarExpr| Box::new(canonical_characters(inner));
    match expr {
        GrammarExpr::Choice {
            ordered,
            alternatives,
        } => GrammarExpr::Choice {
            ordered: *ordered,
            alternatives: alternatives.iter().map(canonical_characters).collect(),
        },
        GrammarExpr::Sequence(items) => {
            GrammarExpr::Sequence(items.iter().map(canonical_characters).collect())
        }
        GrammarExpr::Optional(inner) => GrammarExpr::Optional(boxed(inner)),
        GrammarExpr::ZeroOrMore(inner) => GrammarExpr::ZeroOrMore(boxed(inner)),
        GrammarExpr::OneOrMore(inner) => GrammarExpr::OneOrMore(boxed(inner)),
        GrammarExpr::Repeat { expr, min, max } => GrammarExpr::Repeat {
            expr: boxed(expr),
            min: *min,
            max: *max,
        },
        GrammarExpr::And(inner) => GrammarExpr::And(boxed(inner)),
        GrammarExpr::Not(inner) => GrammarExpr::Not(boxed(inner)),
        GrammarExpr::Capture { label, expr } => GrammarExpr::Capture {
            label: label.clone(),
            expr: boxed(expr),
        },
        other => other.clone(),
    }
}

fn character_set(expr: &GrammarExpr) -> Option<Vec<GrammarExpr>> {
    match expr {
        GrammarExpr::Terminal(value) if value.chars().count() == 1 => Some(vec![expr.clone()]),
        GrammarExpr::CharRange(..) => Some(vec![expr.clone()]),
        GrammarExpr::CharClass {
            negated: false,
            items,
        } => Some(
            items
                .iter()
                .map(|item| match item {
                    CharClassItem::Char(value) => GrammarExpr::Terminal(value.to_string()),
                    CharClassItem::Range(start, end) => GrammarExpr::CharRange(*start, *end),
                })
                .collect(),
        ),
        GrammarExpr::Choice { alternatives, .. } => alternatives
            .iter()
            .map(character_set)
            .collect::<Option<Vec<_>>>()
            .map(|sets| sets.into_iter().flatten().collect()),
        _ => None,
    }
}

fn check_samples(
    failures: &mut Vec<GrammarRoundTripFailure>,
    stage: GrammarRoundTripStage,
    grammar: &Grammar,
    round_trip: &GrammarRoundTrip<'_>,
) {
    let parser = GrammarParser::new(grammar.clone());
    for text in round_trip.accepts {
        if !parser.accepts(text) {
            failures.push(failure(
                GrammarRoundTripFailureKind::SampleRejected,
                stage,
                text,
            ));
        }
    }
    for text in round_trip.rejects {
        if parser.accepts(text) {
            failures.push(failure(
                GrammarRoundTripFailureKind::SampleAccepted,
                stage,
                text,
            ));
        }
    }
}

fn accepts(grammar: &Grammar, text: &str) -> bool {
    GrammarParser::new(grammar.clone()).accepts(text)
}
