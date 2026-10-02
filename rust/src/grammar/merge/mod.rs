//! Deterministic, meaning-aware merging of grammars from several sources, and
//! binding-aware renaming of grammar rules.
//!
//! This mirrors `js/src/grammar-merge.js`: both runtimes normalize rule
//! definitions the same way, prove equivalence by the same recursive structural
//! bisimulation and print the same canonical definitions, so their decisions
//! agree.

mod group;
mod normalize;
mod rename;

use group::{
    Prepared, edition_alternatives, group_fingerprint, group_key, merge_group, name_label,
    normalize_samples, prepare_sources, required_failures, source_of,
};
use normalize::normalize;
pub use rename::{rename_grammar_rule, restore_source_names};

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

use serde::Serialize;

use super::{Grammar, GrammarRule};

/// How an accepted equivalence is justified.
pub const GRAMMAR_MERGE_METHOD: &str = "recursive-structural-bisimulation";

/// One grammar taking part in a merge.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GrammarMergeSource {
    /// Source identifier, unique within one merge and free of `:`.
    pub id: String,
    /// Language the grammar describes.
    pub language: String,
    /// Language edition; different editions are never merged with each other.
    pub edition: String,
    /// Lower precedence wins name collisions.
    pub precedence: u32,
    /// The source grammar.
    pub grammar: Grammar,
}

impl GrammarMergeSource {
    /// Builds a source with an empty edition and precedence zero.
    #[must_use]
    pub fn new(id: impl Into<String>, language: impl Into<String>, grammar: Grammar) -> Self {
        Self {
            id: id.into(),
            language: language.into(),
            edition: String::new(),
            precedence: 0,
            grammar,
        }
    }

    /// Sets the language edition.
    #[must_use]
    pub fn with_edition(mut self, edition: impl Into<String>) -> Self {
        self.edition = edition.into();
        self
    }

    /// Sets the precedence.
    #[must_use]
    pub const fn with_precedence(mut self, precedence: u32) -> Self {
        self.precedence = precedence;
        self
    }
}

/// Optional inputs of [`merge_grammars`].
#[derive(Clone, Debug, Default)]
pub struct GrammarMergeOptions<'a> {
    /// Sample inputs keyed by `sourceId:ruleName`; identical samples only
    /// nominate candidates, they never decide a merge.
    pub samples: BTreeMap<String, Vec<String>>,
    /// Pairs of `sourceId:ruleName` aliases that must end up merged.
    pub required_equivalences: Vec<(String, String)>,
    /// An earlier result: unchanged groups are reused and established
    /// canonical names are kept.
    pub previous: Option<&'a GrammarMergeResult>,
}

/// What the merge decided about one class of rules.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum GrammarMergeDecisionKind {
    /// Several rules were proven equivalent and merged.
    Merged,
    /// The rule has no equivalent in the group.
    KeptUnique,
    /// The rule was renamed because a higher-precedence rule holds its name.
    RenamedForCollision,
    /// Rules sharing a name have different meanings and stay distinct.
    HomonymKeptDistinct,
    /// A nominated match could not be proven and was not merged.
    Uncertain,
}

impl GrammarMergeDecisionKind {
    /// Returns the stable textual form.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Merged => "merged",
            Self::KeptUnique => "kept-unique",
            Self::RenamedForCollision => "renamed-for-collision",
            Self::HomonymKeptDistinct => "homonym-kept-distinct",
            Self::Uncertain => "uncertain",
        }
    }
}

/// One merge decision with its justification.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct GrammarMergeDecision {
    /// Decision kind.
    pub kind: GrammarMergeDecisionKind,
    /// Canonical name (the raw source name for homonyms).
    pub name: String,
    /// Source aliases, or canonical names for homonyms.
    pub members: Vec<String>,
    /// Justification of the decision.
    pub basis: String,
    /// Canonical normalized definition of a merged or unique rule.
    pub definition: Option<String>,
}

/// Why two rules were nominated as a possible match.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum GrammarMergeNominationBasis {
    /// The names differ only in case, `-` or `_`.
    NameSimilarity,
    /// Both rules have the same caller-supplied samples.
    IdenticalSamples,
}

impl GrammarMergeNominationBasis {
    /// Returns the stable textual form.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NameSimilarity => "name-similarity",
            Self::IdenticalSamples => "identical-samples",
        }
    }
}

/// Whether a nomination was confirmed by the structural proof.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum GrammarMergeNominationOutcome {
    /// The nominated rules were proven equivalent.
    Proven,
    /// The nominated rules were not proven equivalent and stay separate.
    Unproven,
}

/// A candidate match nominated by a heuristic.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct GrammarMergeNomination {
    /// Heuristic that nominated the pair.
    pub basis: GrammarMergeNominationBasis,
    /// The two source aliases.
    pub members: [String; 2],
    /// Result of the structural proof.
    pub outcome: GrammarMergeNominationOutcome,
}

/// Why the merge kept several options instead of choosing silently.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum GrammarMergeAlternativeReason {
    /// Sources start from different rules.
    StartRule,
    /// Rules sharing a name have different meanings.
    DistinctMeaning,
    /// A nominated match was not proven.
    UncertainMatch,
    /// A language has several editions.
    Edition,
}

impl GrammarMergeAlternativeReason {
    /// Returns the stable textual form.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StartRule => "start-rule",
            Self::DistinctMeaning => "distinct-meaning",
            Self::UncertainMatch => "uncertain-match",
            Self::Edition => "edition",
        }
    }
}

/// An explicit alternative kept by the merge.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct GrammarMergeAlternative {
    /// Why the alternative exists.
    pub reason: GrammarMergeAlternativeReason,
    /// The chosen option, the shared name, the nomination basis or the language.
    pub name: String,
    /// The options kept.
    pub options: Vec<String>,
}

/// Kind of a merge failure.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum GrammarMergeFailureKind {
    /// A required equivalence was not established.
    UnresolvedRequiredEquivalence,
}

/// Why a required equivalence was not established.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum GrammarMergeFailureReason {
    /// An alias names no rule of the merged sources.
    UnknownRule,
    /// The rules belong to different languages or editions.
    DifferentLanguageOrEdition,
    /// The rules were not proven equivalent.
    NotProven,
}

impl GrammarMergeFailureReason {
    /// Returns the stable textual form.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UnknownRule => "unknown-rule",
            Self::DifferentLanguageOrEdition => "different-language-or-edition",
            Self::NotProven => "not-proven",
        }
    }
}

/// A required equivalence the merge could not establish.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct GrammarMergeFailure {
    /// Failure kind.
    pub kind: GrammarMergeFailureKind,
    /// The two aliases, sorted.
    pub members: [String; 2],
    /// Why the equivalence is unresolved.
    pub reason: GrammarMergeFailureReason,
}

/// The merged grammar of one language edition.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MergedGrammarGroup {
    /// `language@edition`.
    pub key: String,
    /// Language.
    pub language: String,
    /// Edition.
    pub edition: String,
    /// SHA-256 of the group's normalized inputs.
    pub fingerprint: String,
    /// Source ids in precedence order.
    pub sources: Vec<String>,
    /// The merged grammar.
    pub grammar: Grammar,
    /// Canonical rule name of every `sourceId:ruleName` alias.
    pub identities: BTreeMap<String, String>,
    /// Decisions with their justification.
    pub decisions: Vec<GrammarMergeDecision>,
    /// Candidate matches nominated by heuristics.
    pub nominations: Vec<GrammarMergeNomination>,
    /// Alternatives kept explicit.
    pub alternatives: Vec<GrammarMergeAlternative>,
}

/// The result of [`merge_grammars`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GrammarMergeResult {
    /// One merged grammar per language edition, sorted by language and edition.
    pub groups: Vec<MergedGrammarGroup>,
    /// Languages with several editions.
    pub alternatives: Vec<GrammarMergeAlternative>,
    /// Unresolved required equivalences.
    pub failures: Vec<GrammarMergeFailure>,
    /// Keys of groups reused from the previous result.
    pub reused: Vec<String>,
    /// Keys of groups merged again.
    pub recomputed: Vec<String>,
}

impl GrammarMergeResult {
    /// `"complete"` when every required equivalence holds, else `"incomplete"`.
    #[must_use]
    pub const fn status(&self) -> &'static str {
        if self.failures.is_empty() {
            "complete"
        } else {
            "incomplete"
        }
    }

    /// Returns the group with `key`, when present.
    #[must_use]
    pub fn group(&self, key: &str) -> Option<&MergedGrammarGroup> {
        self.groups.iter().find(|group| group.key == key)
    }
}

/// Malformed merge input, or an incomplete merge in [`assert_merge_complete`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GrammarMergeError {
    message: String,
    failures: Vec<GrammarMergeFailure>,
}

impl GrammarMergeError {
    fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            failures: Vec::new(),
        }
    }

    /// The unresolved required equivalences, if any.
    #[must_use]
    pub fn failures(&self) -> &[GrammarMergeFailure] {
        &self.failures
    }
}

impl fmt::Display for GrammarMergeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl Error for GrammarMergeError {}

/// Why a rename was refused.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GrammarRenameErrorKind {
    /// The new name is empty or contains whitespace.
    InvalidName,
    /// The grammar has no rule with the old name.
    UnknownRule,
    /// The new name would capture an existing binding.
    Collision,
}

impl GrammarRenameErrorKind {
    /// Returns the stable textual form.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InvalidName => "invalid-name",
            Self::UnknownRule => "unknown-rule",
            Self::Collision => "collision",
        }
    }
}

/// A refused rename.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GrammarRenameError {
    kind: GrammarRenameErrorKind,
    message: String,
}

impl GrammarRenameError {
    fn new(kind: GrammarRenameErrorKind, message: impl Into<String>) -> Self {
        Self {
            kind,
            message: message.into(),
        }
    }

    /// Why the rename was refused.
    #[must_use]
    pub const fn kind(&self) -> GrammarRenameErrorKind {
        self.kind
    }
}

impl fmt::Display for GrammarRenameError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl Error for GrammarRenameError {}

/// Maps a canonical rule name back to its original source name.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct RuleAlias {
    /// Current name in the grammar.
    pub canonical: String,
    /// Name in the source grammar.
    pub original: String,
}

/// A renamed grammar and its source aliases.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RenamedGrammar {
    /// The grammar after the rename.
    pub grammar: Grammar,
    /// Aliases from canonical names to original source names.
    pub aliases: Vec<RuleAlias>,
}

/// Merges grammars of several sources without user interaction.
///
/// Sources of different languages or editions are never merged with each
/// other. Within one language edition, rules are merged only when their
/// normalized definitions are equivalent under a recursive structural
/// bisimulation; similar names and identical samples only nominate candidates.
pub fn merge_grammars(
    sources: &[GrammarMergeSource],
    options: &GrammarMergeOptions<'_>,
) -> Result<GrammarMergeResult, GrammarMergeError> {
    let prepared = prepare_sources(sources)?;
    let samples = normalize_samples(&options.samples);
    let required: BTreeSet<(String, String)> = options
        .required_equivalences
        .iter()
        .map(|(first, second)| {
            if first <= second {
                (first.clone(), second.clone())
            } else {
                (second.clone(), first.clone())
            }
        })
        .collect();
    let previous_groups: BTreeMap<&str, &MergedGrammarGroup> = options
        .previous
        .map(|previous| {
            previous
                .groups
                .iter()
                .map(|group| (group.key.as_str(), group))
                .collect()
        })
        .unwrap_or_default();

    let mut grouped: BTreeMap<(&str, &str), Vec<&Prepared<'_>>> = BTreeMap::new();
    for source in &prepared {
        grouped
            .entry((source.language, source.edition))
            .or_default()
            .push(source);
    }

    let mut groups = Vec::new();
    let mut reused = Vec::new();
    let mut recomputed = Vec::new();
    for ((language, edition), entry) in grouped {
        let key = group_key(language, edition);
        let ids: BTreeSet<&str> = entry.iter().map(|source| source.id).collect();
        let group_required: Vec<&(String, String)> = required
            .iter()
            .filter(|(first, second)| {
                ids.contains(source_of(first)) || ids.contains(source_of(second))
            })
            .collect();
        let fingerprint = group_fingerprint(language, edition, &entry, &group_required, &samples)?;
        let previous = previous_groups.get(key.as_str()).copied();
        if let Some(previous) = previous.filter(|previous| previous.fingerprint == fingerprint) {
            groups.push(previous.clone());
            reused.push(key);
            continue;
        }
        let empty = BTreeMap::new();
        let previous_identities = previous.map_or(&empty, |previous| &previous.identities);
        groups.push(merge_group(
            language,
            edition,
            &entry,
            fingerprint,
            &samples,
            previous_identities,
        )?);
        recomputed.push(key);
    }

    let alternatives = edition_alternatives(&groups);
    let failures = required_failures(&required, &groups);
    Ok(GrammarMergeResult {
        groups,
        alternatives,
        failures,
        reused,
        recomputed,
    })
}

/// Fails when a merge left a required equivalence unresolved.
pub fn assert_merge_complete(
    result: &GrammarMergeResult,
) -> Result<&GrammarMergeResult, GrammarMergeError> {
    if result.failures.is_empty() {
        return Ok(result);
    }
    let detail = result
        .failures
        .iter()
        .map(|failure| {
            format!(
                "{} = {} ({})",
                failure.members[0],
                failure.members[1],
                failure.reason.as_str()
            )
        })
        .collect::<Vec<_>>()
        .join("; ");
    Err(GrammarMergeError {
        message: format!("unresolved required equivalence: {detail}"),
        failures: result.failures.clone(),
    })
}

/// The canonical text of a rule definition after meaning-aware normalization.
pub fn normalized_rule_definition(rule: &GrammarRule) -> Result<String, GrammarMergeError> {
    Ok(format!(
        "{}:{}",
        rule.kind.as_str(),
        normalize(&rule.expr, &name_label)?.text
    ))
}
