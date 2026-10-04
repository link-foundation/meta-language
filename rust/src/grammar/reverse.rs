//! Reverse conversion of a grammar back to its source format.
//!
//! The checked cycle is source grammar -> native links -> exported grammar ->
//! native links: the source is imported, written as the native links form and
//! read back from those links alone, so the emitter is handed a grammar decoded
//! from links and never the source text. The export is re-imported and
//! compared with the imported grammar rule by rule (names, start rule, kinds,
//! definitions modulo the documented canonical spelling of single-character
//! sets, and rule documentation, the provenance an importer keeps from
//! comments), and both grammars are run on independent accept and reject
//! samples. It mirrors `js/src/grammar-reverse.js`.

use super::Grammar;
use super::interchange::{parse_grammar_links, render_grammar_links};
use super::round_trip::{
    GrammarEmitFn, GrammarImportFn, GrammarRoundTripError, canonical_rule_definition,
};
use super::runtime::GrammarParser;

/// The importer/emitter pair under test and the independent samples.
#[derive(Clone, Copy)]
pub struct GrammarReverseConversion<'a> {
    /// Importer of the notation.
    pub import: GrammarImportFn<'a>,
    /// Emitter of the same notation.
    pub emit: GrammarEmitFn<'a>,
    /// Texts the grammar must accept.
    pub accepts: &'a [String],
    /// Texts the grammar must reject.
    pub rejects: &'a [String],
}

impl<'a> GrammarReverseConversion<'a> {
    /// Builds a reverse conversion through `import` and `emit` with no samples.
    #[must_use]
    pub const fn new(import: GrammarImportFn<'a>, emit: GrammarEmitFn<'a>) -> Self {
        Self {
            import,
            emit,
            accepts: &[],
            rejects: &[],
        }
    }

    /// Returns this conversion with independent accept and reject samples.
    #[must_use]
    pub const fn with_samples(mut self, accepts: &'a [String], rejects: &'a [String]) -> Self {
        self.accepts = accepts;
        self.rejects = rejects;
        self
    }
}

/// Whether the cycle kept the grammar.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GrammarReverseStatus {
    /// Every check held.
    Equivalent,
    /// At least one check failed.
    Different,
}

impl GrammarReverseStatus {
    /// Stable tag shared with the JavaScript runtime.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Equivalent => "equivalent",
            Self::Different => "different",
        }
    }
}

/// What a failed check found.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GrammarReverseFailureKind {
    /// Reading the links back writes different links.
    LinksNotFaithful,
    /// The emitter reported a lossy conversion.
    LossyExport,
    /// The re-imported rules differ from the imported ones.
    RulesChanged,
    /// A re-imported rule lost or changed its documentation.
    DocChanged,
    /// A text the grammar must accept was rejected.
    SampleRejected,
    /// A text the grammar must reject was accepted.
    SampleAccepted,
}

impl GrammarReverseFailureKind {
    /// Stable tag shared with the JavaScript runtime.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LinksNotFaithful => "links-not-faithful",
            Self::LossyExport => "lossy-export",
            Self::RulesChanged => "rules-changed",
            Self::DocChanged => "doc-changed",
            Self::SampleRejected => "sample-rejected",
            Self::SampleAccepted => "sample-accepted",
        }
    }
}

/// The step of the cycle a failed check looked at.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GrammarReverseStage {
    /// The grammar imported from the source.
    Imported,
    /// The native links of the imported grammar.
    Links,
    /// The text exported from the grammar decoded from links.
    Exported,
    /// The grammar re-imported from the exported text.
    Reimported,
}

impl GrammarReverseStage {
    /// Stable tag shared with the JavaScript runtime.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Imported => "imported",
            Self::Links => "links",
            Self::Exported => "exported",
            Self::Reimported => "reimported",
        }
    }
}

/// One failed check.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GrammarReverseFailure {
    /// What the check found.
    pub kind: GrammarReverseFailureKind,
    /// The step it looked at.
    pub stage: GrammarReverseStage,
    /// The sample, note or rule involved.
    pub detail: String,
}

/// The outcome of [`check_grammar_reverse_conversion`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GrammarReverseReport {
    /// Whether every check held.
    pub status: GrammarReverseStatus,
    /// Every failed check, in the order they ran.
    pub failures: Vec<GrammarReverseFailure>,
    /// The native links of the imported grammar.
    pub links: String,
    /// The text exported from the grammar decoded from those links.
    pub exported: String,
    /// The native links of the grammar re-imported from the export.
    pub reimported_links: String,
}

/// Runs the reverse conversion cycle on `source` and reports every failure.
///
/// # Errors
///
/// Returns the importer's error for a malformed source, the links reader's
/// error, the emitter's error, or a normalization error.
pub fn check_grammar_reverse_conversion(
    source: &str,
    conversion: &GrammarReverseConversion<'_>,
) -> Result<GrammarReverseReport, GrammarRoundTripError> {
    let mut failures = Vec::new();
    let imported = (conversion.import)(source)?;
    sample_failures(
        &mut failures,
        GrammarReverseStage::Imported,
        &imported,
        conversion,
    );

    let links = render_grammar_links(&imported);
    let decoded = parse_grammar_links(&links)?;
    if render_grammar_links(&decoded) != links {
        failures.push(failure(
            GrammarReverseFailureKind::LinksNotFaithful,
            GrammarReverseStage::Links,
            "reading the links back writes different links",
        ));
    }

    let (exported, report) = (conversion.emit)(&decoded)?;
    for note in &report.lossy {
        failures.push(failure(
            GrammarReverseFailureKind::LossyExport,
            GrammarReverseStage::Exported,
            note,
        ));
    }

    let reimported = (conversion.import)(&exported)?;
    let reimported_links = render_grammar_links(&reimported);
    compare_grammars(&mut failures, &imported, &reimported)?;
    sample_failures(
        &mut failures,
        GrammarReverseStage::Reimported,
        &reimported,
        conversion,
    );

    Ok(GrammarReverseReport {
        status: if failures.is_empty() {
            GrammarReverseStatus::Equivalent
        } else {
            GrammarReverseStatus::Different
        },
        failures,
        links,
        exported,
        reimported_links,
    })
}

fn failure(
    kind: GrammarReverseFailureKind,
    stage: GrammarReverseStage,
    detail: &str,
) -> GrammarReverseFailure {
    GrammarReverseFailure {
        kind,
        stage,
        detail: detail.to_owned(),
    }
}

fn compare_grammars(
    failures: &mut Vec<GrammarReverseFailure>,
    expected: &Grammar,
    actual: &Grammar,
) -> Result<(), GrammarRoundTripError> {
    let mut changed = |detail: &str| {
        failures.push(failure(
            GrammarReverseFailureKind::RulesChanged,
            GrammarReverseStage::Reimported,
            detail,
        ));
    };
    let expected_names = expected.rule_names();
    let actual_names = actual.rule_names();
    if expected_names != actual_names {
        changed(&format!(
            "rule names [{}] became [{}]",
            expected_names.join(", "),
            actual_names.join(", ")
        ));
    }
    let start = |grammar: &Grammar| grammar.start_rule().map(|rule| rule.name.clone());
    if start(expected) != start(actual) {
        changed("the start rule changed");
    }
    for rule in expected.rules() {
        let Some(other) = actual.rule(&rule.name) else {
            continue;
        };
        if canonical_rule_definition(other)? != canonical_rule_definition(rule)? {
            failures.push(failure(
                GrammarReverseFailureKind::RulesChanged,
                GrammarReverseStage::Reimported,
                &format!("rule {} changed its definition", rule.name),
            ));
        }
        if other.doc() != rule.doc() {
            failures.push(failure(
                GrammarReverseFailureKind::DocChanged,
                GrammarReverseStage::Reimported,
                &format!("rule {} changed its documentation", rule.name),
            ));
        }
    }
    Ok(())
}

fn sample_failures(
    failures: &mut Vec<GrammarReverseFailure>,
    stage: GrammarReverseStage,
    grammar: &Grammar,
    conversion: &GrammarReverseConversion<'_>,
) {
    let parser = GrammarParser::new(grammar.clone());
    for text in conversion.accepts {
        if !parser.accepts(text) {
            failures.push(failure(
                GrammarReverseFailureKind::SampleRejected,
                stage,
                text,
            ));
        }
    }
    for text in conversion.rejects {
        if parser.accepts(text) {
            failures.push(failure(
                GrammarReverseFailureKind::SampleAccepted,
                stage,
                text,
            ));
        }
    }
}
