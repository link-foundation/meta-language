//! Deterministic, meaning-aware merging of grammars from several sources, and
//! binding-aware renaming of grammar rules.
//!
//! This mirrors `js/src/grammar-merge.js`: both runtimes normalize rule
//! definitions the same way, prove equivalence by the same recursive structural
//! bisimulation and print the same canonical definitions, so their decisions
//! agree.

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

use serde::Serialize;
use sha2::{Digest, Sha256};

use super::{CharClassItem, Grammar, GrammarExpr, GrammarFormat, GrammarRule};

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

/// Renames one rule and every reference to it.
///
/// Recursive references, references inside captures and references qualified
/// with `namespace` (as `namespace.rule` or `namespace::rule`) are renamed.
/// Capture labels are a separate scope and are never renamed. The returned
/// aliases map canonical names back to the original source names, so the
/// grammar can be exported with them.
pub fn rename_grammar_rule(
    grammar: &Grammar,
    from: &str,
    to: &str,
    namespace: Option<&str>,
    aliases: &[RuleAlias],
) -> Result<RenamedGrammar, GrammarRenameError> {
    if to.is_empty() || to.chars().any(char::is_whitespace) {
        return Err(GrammarRenameError::new(
            GrammarRenameErrorKind::InvalidName,
            format!("invalid rule name {}", quote(to)),
        ));
    }
    if grammar.rule(from).is_none() {
        return Err(GrammarRenameError::new(
            GrammarRenameErrorKind::UnknownRule,
            format!("grammar has no rule {from}"),
        ));
    }
    if from == to {
        return Ok(RenamedGrammar {
            grammar: grammar.clone(),
            aliases: aliases.to_vec(),
        });
    }
    let mut mapping = vec![(from.to_owned(), to.to_owned())];
    if let Some(namespace) = namespace {
        mapping.push((format!("{namespace}.{from}"), format!("{namespace}.{to}")));
        mapping.push((format!("{namespace}::{from}"), format!("{namespace}::{to}")));
    }
    let mut taken = grammar.referenced_nonterminals();
    taken.extend(grammar.rules().iter().map(|rule| rule.name.clone()));
    if let Some((_, target)) = mapping.iter().find(|(_, target)| taken.contains(target)) {
        return Err(GrammarRenameError::new(
            GrammarRenameErrorKind::Collision,
            format!("renaming {from} to {to} would capture the existing name {target}"),
        ));
    }
    let mut chained = false;
    let mut next_aliases: Vec<RuleAlias> = aliases
        .iter()
        .map(|alias| {
            if alias.canonical == from {
                chained = true;
                RuleAlias {
                    canonical: to.to_owned(),
                    original: alias.original.clone(),
                }
            } else {
                alias.clone()
            }
        })
        .collect();
    if !chained {
        next_aliases.push(RuleAlias {
            canonical: to.to_owned(),
            original: from.to_owned(),
        });
    }
    Ok(RenamedGrammar {
        grammar: rename_all(grammar, &mapping.into_iter().collect()),
        aliases: next_aliases,
    })
}

/// Renames canonical rule names back to their source names for export.
pub fn restore_source_names(
    grammar: &Grammar,
    aliases: &[RuleAlias],
    namespace: Option<&str>,
) -> Result<Grammar, GrammarRenameError> {
    let mut mapping = BTreeMap::new();
    for RuleAlias {
        canonical,
        original,
    } in aliases
    {
        if grammar.rule(canonical).is_none() || canonical == original {
            continue;
        }
        mapping.insert(canonical.clone(), original.clone());
        if let Some(namespace) = namespace {
            mapping.insert(
                format!("{namespace}.{canonical}"),
                format!("{namespace}.{original}"),
            );
            mapping.insert(
                format!("{namespace}::{canonical}"),
                format!("{namespace}::{original}"),
            );
        }
    }
    let rename = |name: &str| {
        mapping
            .get(name)
            .cloned()
            .unwrap_or_else(|| name.to_owned())
    };
    let rule_names: Vec<String> = grammar
        .rules()
        .iter()
        .map(|rule| rename(&rule.name))
        .collect();
    let unique: BTreeSet<&String> = rule_names.iter().collect();
    let captured = grammar
        .referenced_nonterminals()
        .iter()
        .filter(|name| grammar.rule(name).is_none())
        .any(|name| rule_names.contains(&rename(name)));
    if unique.len() != rule_names.len() || captured {
        return Err(GrammarRenameError::new(
            GrammarRenameErrorKind::Collision,
            "restoring source names would give two bindings the same name",
        ));
    }
    Ok(rename_all(grammar, &mapping))
}

/// The canonical text of a rule definition after meaning-aware normalization.
pub fn normalized_rule_definition(rule: &GrammarRule) -> Result<String, GrammarMergeError> {
    Ok(format!(
        "{}:{}",
        rule.kind.as_str(),
        normalize(&rule.expr, &name_label)?.text
    ))
}

fn rename_all(grammar: &Grammar, mapping: &BTreeMap<String, String>) -> Grammar {
    let rename = |name: &str| {
        mapping
            .get(name)
            .cloned()
            .unwrap_or_else(|| name.to_owned())
    };
    let mut renamed = Grammar::new();
    for rule in grammar.rules() {
        renamed.add_rule(GrammarRule {
            name: rename(&rule.name),
            expr: map_references(&rule.expr, &rename),
            kind: rule.kind,
            concept: rule.concept.clone(),
            doc: rule.doc.clone(),
        });
    }
    if let Some(start) = grammar.start() {
        renamed.set_start(rename(start));
    }
    renamed.source_format = grammar.source_format();
    renamed
}

fn map_references(expr: &GrammarExpr, rename: &dyn Fn(&str) -> String) -> GrammarExpr {
    let map = |inner: &GrammarExpr| Box::new(map_references(inner, rename));
    match expr {
        GrammarExpr::NonTerminal(name) => GrammarExpr::NonTerminal(rename(name)),
        GrammarExpr::Choice {
            ordered,
            alternatives,
        } => GrammarExpr::Choice {
            ordered: *ordered,
            alternatives: alternatives
                .iter()
                .map(|item| map_references(item, rename))
                .collect(),
        },
        GrammarExpr::Sequence(items) => GrammarExpr::Sequence(
            items
                .iter()
                .map(|item| map_references(item, rename))
                .collect(),
        ),
        GrammarExpr::Optional(inner) => GrammarExpr::Optional(map(inner)),
        GrammarExpr::ZeroOrMore(inner) => GrammarExpr::ZeroOrMore(map(inner)),
        GrammarExpr::OneOrMore(inner) => GrammarExpr::OneOrMore(map(inner)),
        GrammarExpr::Repeat { expr, min, max } => GrammarExpr::Repeat {
            expr: map(expr),
            min: *min,
            max: *max,
        },
        GrammarExpr::And(inner) => GrammarExpr::And(map(inner)),
        GrammarExpr::Not(inner) => GrammarExpr::Not(map(inner)),
        GrammarExpr::Capture { label, expr } => GrammarExpr::Capture {
            label: label.clone(),
            expr: map(expr),
        },
        other => other.clone(),
    }
}

struct Prepared<'a> {
    id: &'a str,
    language: &'a str,
    edition: &'a str,
    precedence: u32,
    grammar: &'a Grammar,
    names: BTreeSet<&'a str>,
}

impl Prepared<'_> {
    fn format(&self) -> &'static str {
        format_of(self.grammar)
    }

    fn has_rule(&self, name: &str) -> bool {
        self.names.contains(name)
    }
}

fn prepare_sources(sources: &[GrammarMergeSource]) -> Result<Vec<Prepared<'_>>, GrammarMergeError> {
    let mut seen = BTreeSet::new();
    let mut prepared = Vec::with_capacity(sources.len());
    for source in sources {
        let id = source.id.as_str();
        if id.is_empty() || id.contains(':') {
            return Err(GrammarMergeError::new(format!(
                "invalid merge source id {}",
                quote(id)
            )));
        }
        if !seen.insert(id) {
            return Err(GrammarMergeError::new(format!(
                "duplicate merge source id {id}"
            )));
        }
        if source.language.is_empty() {
            return Err(GrammarMergeError::new(format!(
                "merge source {id} has no language"
            )));
        }
        let mut names = BTreeSet::new();
        for rule in source.grammar.rules() {
            if !names.insert(rule.name.as_str()) {
                return Err(GrammarMergeError::new(format!(
                    "merge source {id} defines rule {} twice",
                    rule.name
                )));
            }
        }
        prepared.push(Prepared {
            id,
            language: &source.language,
            edition: &source.edition,
            precedence: source.precedence,
            grammar: &source.grammar,
            names,
        });
    }
    prepared.sort_by(|left, right| {
        left.precedence
            .cmp(&right.precedence)
            .then_with(|| left.id.cmp(right.id))
    });
    Ok(prepared)
}

fn normalize_samples(samples: &BTreeMap<String, Vec<String>>) -> BTreeMap<String, Vec<String>> {
    samples
        .iter()
        .filter(|(_, values)| !values.is_empty())
        .map(|(alias, values)| {
            let unique: BTreeSet<&String> = values.iter().collect();
            (alias.clone(), unique.into_iter().cloned().collect())
        })
        .collect()
}

fn group_key(language: &str, edition: &str) -> String {
    format!("{language}@{edition}")
}

fn source_of(alias: &str) -> &str {
    alias.split_once(':').map_or(alias, |(source, _)| source)
}

fn format_of(grammar: &Grammar) -> &'static str {
    grammar
        .source_format()
        .map_or("none", GrammarFormat::as_str)
}

fn group_fingerprint(
    language: &str,
    edition: &str,
    entry: &[&Prepared<'_>],
    required: &[&(String, String)],
    samples: &BTreeMap<String, Vec<String>>,
) -> Result<String, GrammarMergeError> {
    let mut lines = vec![
        "grammar-merge v1".to_owned(),
        format!("group {} {}", quote(language), quote(edition)),
    ];
    let mut ids = BTreeSet::new();
    for source in entry {
        ids.insert(source.id);
        let start = source
            .grammar
            .start_rule()
            .map_or_else(|| "-".to_owned(), |rule| quote(&rule.name));
        lines.push(format!(
            "source {} {} {} {start}",
            quote(source.id),
            source.precedence,
            quote(source.format())
        ));
        let label = source_label(source, &name_label);
        for rule in source.grammar.rules() {
            lines.push(format!(
                "rule {} {}:{}",
                quote(&rule.name),
                rule.kind.as_str(),
                normalize(&rule.expr, &label)?.text
            ));
            // Concept and documentation annotations exist only in the Rust IR;
            // they are fingerprinted so that changing them re-merges the group.
            if let Some(concept) = &rule.concept {
                lines.push(format!("concept {} {}", quote(&rule.name), quote(concept)));
            }
            if let Some(doc) = &rule.doc {
                lines.push(format!("doc {} {}", quote(&rule.name), quote(doc)));
            }
        }
    }
    for (first, second) in required {
        lines.push(format!("required {} {}", quote(first), quote(second)));
    }
    for (alias, values) in samples {
        if ids.contains(source_of(alias)) {
            let values: Vec<String> = values.iter().map(|value| quote(value)).collect();
            lines.push(format!("samples {} {}", quote(alias), values.join(",")));
        }
    }
    let mut text = lines.join("\n");
    text.push('\n');
    Ok(Sha256::digest(text.as_bytes())
        .iter()
        .fold(String::with_capacity(64), |mut hex, byte| {
            use std::fmt::Write as _;
            let _ = write!(hex, "{byte:02x}");
            hex
        }))
}

fn source_label<'a>(
    source: &'a Prepared<'_>,
    internal: &'a dyn Fn(&str) -> String,
) -> impl Fn(&str) -> String + 'a {
    move |name: &str| {
        if source.has_rule(name) {
            internal(name)
        } else {
            format!("ext({},{})", quote(source.format()), quote(name))
        }
    }
}

fn name_label(name: &str) -> String {
    format!("ref({})", quote(name))
}

fn alias_of(source: &str, name: &str) -> String {
    format!("{source}:{name}")
}

struct Node<'a> {
    alias: String,
    source: &'a Prepared<'a>,
    rule: &'a GrammarRule,
}

fn merge_group(
    language: &str,
    edition: &str,
    entry: &[&Prepared<'_>],
    fingerprint: String,
    samples: &BTreeMap<String, Vec<String>>,
    previous_identities: &BTreeMap<String, String>,
) -> Result<MergedGrammarGroup, GrammarMergeError> {
    let mut nodes = Vec::new();
    let mut index = BTreeMap::new();
    let mut externals = BTreeSet::new();
    for source in entry {
        for rule in source.grammar.rules() {
            let alias = alias_of(source.id, &rule.name);
            index.insert(alias.clone(), nodes.len());
            nodes.push(Node {
                alias,
                source,
                rule,
            });
        }
        for name in source.grammar.referenced_nonterminals() {
            if !source.has_rule(&name) {
                externals.insert(name);
            }
        }
    }

    let classes = refine(&nodes, &index)?;
    let mut members: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
    let mut class_order = Vec::new();
    for (position, class) in classes.iter().enumerate() {
        let list = members.entry(*class).or_default();
        if list.is_empty() {
            class_order.push(*class);
        }
        list.push(position);
    }

    let mut names: BTreeMap<usize, String> = BTreeMap::new();
    let mut taken = externals;
    let mut renamed = BTreeSet::new();
    for class in &class_order {
        for &position in &members[class] {
            if let Some(previous) = previous_identities.get(&nodes[position].alias)
                && !taken.contains(previous)
            {
                names.insert(*class, previous.clone());
                taken.insert(previous.clone());
                break;
            }
        }
    }
    for class in &class_order {
        if names.contains_key(class) {
            continue;
        }
        let best = &nodes[members[class][0]];
        let mut name = best.rule.name.clone();
        if taken.contains(&name) {
            let sanitized: String = best
                .source
                .id
                .chars()
                .map(|character| {
                    if character.is_ascii_alphanumeric() || character == '_' {
                        character
                    } else {
                        '_'
                    }
                })
                .collect();
            let base = format!("{}_from_{sanitized}", best.rule.name);
            name.clone_from(&base);
            let mut suffix = 2_usize;
            while taken.contains(&name) {
                name = format!("{base}_{suffix}");
                suffix += 1;
            }
            renamed.insert(*class);
        }
        taken.insert(name.clone());
        names.insert(*class, name);
    }

    let canonical =
        |source: &str, name: &str| -> &String { &names[&classes[index[&alias_of(source, name)]]] };
    let mut grammar = Grammar::new();
    let mut definitions = BTreeMap::new();
    for class in &class_order {
        let representative = &nodes[members[class][0]];
        let source = representative.source;
        let rename = |name: &str| {
            if source.has_rule(name) {
                canonical(source.id, name).clone()
            } else {
                name.to_owned()
            }
        };
        let form = normalize(
            &map_references(&representative.rule.expr, &rename),
            &name_label,
        )?;
        definitions.insert(
            *class,
            format!("{}:{}", representative.rule.kind.as_str(), form.text),
        );
        grammar.add_rule(GrammarRule {
            name: names[class].clone(),
            expr: form.expr(),
            kind: representative.rule.kind,
            concept: representative.rule.concept.clone(),
            doc: representative.rule.doc.clone(),
        });
    }

    let mut start_classes = Vec::new();
    for source in entry {
        if let Some(start) = source.grammar.start_rule() {
            let class = classes[index[&alias_of(source.id, &start.name)]];
            if !start_classes.contains(&class) {
                start_classes.push(class);
            }
        }
    }
    if let Some(first) = start_classes.first() {
        grammar.set_start(names[first].clone());
    }
    let formats: BTreeSet<&str> = entry.iter().map(|source| source.format()).collect();
    grammar.source_format = if formats.len() == 1 {
        entry[0].grammar.source_format()
    } else {
        Some(GrammarFormat::MetaLanguage)
    };

    let identities: BTreeMap<String, String> = nodes
        .iter()
        .map(|node| {
            (
                node.alias.clone(),
                canonical(node.source.id, &node.rule.name).clone(),
            )
        })
        .collect();

    let mut decisions = Vec::new();
    for class in &class_order {
        let group = &members[class];
        let merged = group.len() > 1;
        decisions.push(GrammarMergeDecision {
            kind: if merged {
                GrammarMergeDecisionKind::Merged
            } else {
                GrammarMergeDecisionKind::KeptUnique
            },
            name: names[class].clone(),
            members: group
                .iter()
                .map(|&position| nodes[position].alias.clone())
                .collect(),
            basis: if merged {
                GRAMMAR_MERGE_METHOD
            } else {
                "no-equivalent-rule"
            }
            .to_owned(),
            definition: definitions.remove(class),
        });
        if renamed.contains(class) {
            decisions.push(GrammarMergeDecision {
                kind: GrammarMergeDecisionKind::RenamedForCollision,
                name: names[class].clone(),
                members: vec![nodes[group[0]].alias.clone()],
                basis: "precedence".to_owned(),
                definition: None,
            });
        }
    }

    let mut alternatives = Vec::new();
    if let Some((first, rest)) = start_classes.split_first()
        && !rest.is_empty()
    {
        alternatives.push(GrammarMergeAlternative {
            reason: GrammarMergeAlternativeReason::StartRule,
            name: names[first].clone(),
            options: rest.iter().map(|class| names[class].clone()).collect(),
        });
    }
    let mut by_name: BTreeMap<&str, Vec<usize>> = BTreeMap::new();
    for class in &class_order {
        for &position in &members[class] {
            let list = by_name
                .entry(nodes[position].rule.name.as_str())
                .or_default();
            if !list.contains(class) {
                list.push(*class);
            }
        }
    }
    for (name, list) in by_name {
        if list.len() < 2 {
            continue;
        }
        let options: Vec<String> = list.iter().map(|class| names[class].clone()).collect();
        decisions.push(GrammarMergeDecision {
            kind: GrammarMergeDecisionKind::HomonymKeptDistinct,
            name: name.to_owned(),
            members: options.clone(),
            basis: "different-definitions".to_owned(),
            definition: None,
        });
        alternatives.push(GrammarMergeAlternative {
            reason: GrammarMergeAlternativeReason::DistinctMeaning,
            name: name.to_owned(),
            options,
        });
    }

    let nominations = nominate(&nodes, &classes, samples);
    for nomination in &nominations {
        if nomination.outcome != GrammarMergeNominationOutcome::Unproven {
            continue;
        }
        let first = identities[&nomination.members[0]].clone();
        let second = identities[&nomination.members[1]].clone();
        decisions.push(GrammarMergeDecision {
            kind: GrammarMergeDecisionKind::Uncertain,
            name: first.clone(),
            members: nomination.members.to_vec(),
            basis: nomination.basis.as_str().to_owned(),
            definition: None,
        });
        alternatives.push(GrammarMergeAlternative {
            reason: GrammarMergeAlternativeReason::UncertainMatch,
            name: nomination.basis.as_str().to_owned(),
            options: vec![first, second],
        });
    }

    Ok(MergedGrammarGroup {
        key: group_key(language, edition),
        language: language.to_owned(),
        edition: edition.to_owned(),
        fingerprint,
        sources: entry.iter().map(|source| source.id.to_owned()).collect(),
        grammar,
        identities,
        decisions,
        nominations,
        alternatives,
    })
}

// Greatest structural bisimulation by partition refinement: rules start in one
// class per normalized shape and are split until every member of a class
// references members of the same classes in the same way. Class numbers are
// ranks of sorted signatures, so they do not depend on the input order.
fn refine(
    nodes: &[Node<'_>],
    index: &BTreeMap<String, usize>,
) -> Result<Vec<usize>, GrammarMergeError> {
    let mut classes = vec![0_usize; nodes.len()];
    let mut count = 0_usize;
    loop {
        let mut keys = Vec::with_capacity(nodes.len());
        for (position, node) in nodes.iter().enumerate() {
            let current = &classes;
            let internal =
                |name: &str| format!("#{}", current[index[&alias_of(node.source.id, name)]]);
            let label = source_label(node.source, &internal);
            let prefix = if count == 0 {
                String::new()
            } else {
                classes[position].to_string()
            };
            keys.push(format!(
                "{prefix}|{}:{}",
                node.rule.kind.as_str(),
                normalize(&node.rule.expr, &label)?.text
            ));
        }
        let ranks: BTreeMap<&String, usize> = keys
            .iter()
            .collect::<BTreeSet<_>>()
            .into_iter()
            .enumerate()
            .map(|(rank, key)| (key, rank))
            .collect();
        let size = ranks.len();
        classes = keys.iter().map(|key| ranks[key]).collect();
        if size == count {
            return Ok(classes);
        }
        count = size;
    }
}

fn nominate(
    nodes: &[Node<'_>],
    classes: &[usize],
    samples: &BTreeMap<String, Vec<String>>,
) -> Vec<GrammarMergeNomination> {
    let mut nominations = Vec::new();
    let mut seen = BTreeSet::new();
    let mut add = |basis: GrammarMergeNominationBasis, first: usize, second: usize| {
        if !seen.insert((basis.as_str(), first, second)) {
            return;
        }
        nominations.push(GrammarMergeNomination {
            basis,
            members: [nodes[first].alias.clone(), nodes[second].alias.clone()],
            outcome: if classes[first] == classes[second] {
                GrammarMergeNominationOutcome::Proven
            } else {
                GrammarMergeNominationOutcome::Unproven
            },
        });
    };
    let by_key = group_in_order(nodes.iter().enumerate().map(|(position, node)| {
        let key: String = node
            .rule
            .name
            .to_lowercase()
            .chars()
            .filter(|character| *character != '-' && *character != '_')
            .collect();
        (key, position)
    }));
    for group in by_key {
        for (offset, &left) in group.iter().enumerate() {
            for &right in &group[offset + 1..] {
                if nodes[left].rule.name != nodes[right].rule.name {
                    add(GrammarMergeNominationBasis::NameSimilarity, left, right);
                }
            }
        }
    }
    let by_samples = group_in_order(nodes.iter().enumerate().filter_map(|(position, node)| {
        samples.get(&node.alias).map(|values| {
            let key: Vec<String> = values.iter().map(|value| quote(value)).collect();
            (key.join(","), position)
        })
    }));
    for group in by_samples {
        for (offset, &left) in group.iter().enumerate() {
            for &right in &group[offset + 1..] {
                add(GrammarMergeNominationBasis::IdenticalSamples, left, right);
            }
        }
    }
    nominations
}

// Groups values by key, keeping the order in which keys first appear.
fn group_in_order(entries: impl Iterator<Item = (String, usize)>) -> Vec<Vec<usize>> {
    let mut slots: BTreeMap<String, usize> = BTreeMap::new();
    let mut groups: Vec<Vec<usize>> = Vec::new();
    for (key, value) in entries {
        let slot = *slots.entry(key).or_insert_with(|| {
            groups.push(Vec::new());
            groups.len() - 1
        });
        groups[slot].push(value);
    }
    groups
}

fn edition_alternatives(groups: &[MergedGrammarGroup]) -> Vec<GrammarMergeAlternative> {
    let mut editions: Vec<(String, Vec<String>)> = Vec::new();
    for group in groups {
        match editions.last_mut() {
            Some((language, list)) if *language == group.language => {
                list.push(group.edition.clone());
            }
            _ => editions.push((group.language.clone(), vec![group.edition.clone()])),
        }
    }
    editions
        .into_iter()
        .filter(|(_, list)| list.len() > 1)
        .map(|(language, options)| GrammarMergeAlternative {
            reason: GrammarMergeAlternativeReason::Edition,
            name: language,
            options,
        })
        .collect()
}

fn required_failures(
    required: &BTreeSet<(String, String)>,
    groups: &[MergedGrammarGroup],
) -> Vec<GrammarMergeFailure> {
    let mut group_of = BTreeMap::new();
    for (position, group) in groups.iter().enumerate() {
        for id in &group.sources {
            group_of.insert(id.as_str(), position);
        }
    }
    let lookup = |alias: &String| {
        group_of.get(source_of(alias)).and_then(|&position| {
            groups[position]
                .identities
                .get(alias)
                .map(|name| (position, name))
        })
    };
    let mut failures = Vec::new();
    for (first, second) in required {
        let reason = match (lookup(first), lookup(second)) {
            (Some((left_group, left)), Some((right_group, right))) => {
                if left_group != right_group {
                    Some(GrammarMergeFailureReason::DifferentLanguageOrEdition)
                } else if left != right {
                    Some(GrammarMergeFailureReason::NotProven)
                } else {
                    None
                }
            }
            _ => Some(GrammarMergeFailureReason::UnknownRule),
        };
        if let Some(reason) = reason {
            failures.push(GrammarMergeFailure {
                kind: GrammarMergeFailureKind::UnresolvedRequiredEquivalence,
                members: [first.clone(), second.clone()],
                reason,
            });
        }
    }
    failures
}

// Meaning-aware normalization. Each form carries its canonical text and its
// normalized shape; `label` prints a reference, which lets the same pass serve
// source names, external names and bisimulation classes.
#[derive(Clone)]
struct Form {
    text: String,
    shape: Shape,
}

#[derive(Clone)]
enum Shape {
    Leaf(GrammarExpr),
    Sequence(Vec<Form>),
    Choice {
        ordered: bool,
        items: Vec<Form>,
    },
    Repeat {
        min: usize,
        max: Option<usize>,
        inner: Box<Form>,
    },
    And(Box<Form>),
    Not(Box<Form>),
    Capture {
        label: Option<String>,
        inner: Box<Form>,
    },
}

impl Form {
    const fn leaf(expr: GrammarExpr, text: String) -> Self {
        Self {
            text,
            shape: Shape::Leaf(expr),
        }
    }

    fn empty() -> Self {
        Self::leaf(GrammarExpr::Empty, "empty".to_owned())
    }

    const fn is_empty(&self) -> bool {
        matches!(self.shape, Shape::Leaf(GrammarExpr::Empty))
    }

    fn expr(&self) -> GrammarExpr {
        match &self.shape {
            Shape::Leaf(expr) => expr.clone(),
            Shape::Sequence(items) => GrammarExpr::Sequence(items.iter().map(Self::expr).collect()),
            Shape::Choice { ordered, items } => GrammarExpr::Choice {
                ordered: *ordered,
                alternatives: items.iter().map(Self::expr).collect(),
            },
            Shape::Repeat { min, max, inner } => {
                let expr = Box::new(inner.expr());
                match (*min, *max) {
                    (0, None) => GrammarExpr::ZeroOrMore(expr),
                    (1, None) => GrammarExpr::OneOrMore(expr),
                    (0, Some(1)) => GrammarExpr::Optional(expr),
                    (min, max) => GrammarExpr::Repeat { expr, min, max },
                }
            }
            Shape::And(inner) => GrammarExpr::And(Box::new(inner.expr())),
            Shape::Not(inner) => GrammarExpr::Not(Box::new(inner.expr())),
            Shape::Capture { label, inner } => GrammarExpr::Capture {
                label: label.clone(),
                expr: Box::new(inner.expr()),
            },
        }
    }
}

fn normalize(
    expr: &GrammarExpr,
    label: &dyn Fn(&str) -> String,
) -> Result<Form, GrammarMergeError> {
    Ok(match expr {
        GrammarExpr::Empty => Form::empty(),
        GrammarExpr::Terminal(value) => literal(value),
        GrammarExpr::TerminalInsensitive(value) => {
            if value.to_lowercase() == value.to_uppercase() {
                literal(value)
            } else {
                Form::leaf(
                    GrammarExpr::TerminalInsensitive(value.clone()),
                    format!("ilit({})", quote(value)),
                )
            }
        }
        GrammarExpr::CharRange(start, end) => char_range(*start, *end),
        GrammarExpr::CharClass { negated, items } => char_class(*negated, items),
        GrammarExpr::AnyChar => Form::leaf(GrammarExpr::AnyChar, "any".to_owned()),
        GrammarExpr::NonTerminal(name) => {
            Form::leaf(GrammarExpr::NonTerminal(name.clone()), label(name))
        }
        GrammarExpr::Sequence(items) => sequence_form(
            items
                .iter()
                .map(|item| normalize(item, label))
                .collect::<Result<_, _>>()?,
        ),
        GrammarExpr::Choice {
            ordered,
            alternatives,
        } => choice_form(
            alternatives
                .iter()
                .map(|item| normalize(item, label))
                .collect::<Result<_, _>>()?,
            *ordered,
        ),
        GrammarExpr::ZeroOrMore(inner) => repeat_form(normalize(inner, label)?, 0, None)?,
        GrammarExpr::OneOrMore(inner) => repeat_form(normalize(inner, label)?, 1, None)?,
        GrammarExpr::Optional(inner) => repeat_form(normalize(inner, label)?, 0, Some(1))?,
        GrammarExpr::Repeat { expr, min, max } => repeat_form(normalize(expr, label)?, *min, *max)?,
        GrammarExpr::And(inner) => {
            let inner = normalize(inner, label)?;
            Form {
                text: format!("and({})", inner.text),
                shape: Shape::And(Box::new(inner)),
            }
        }
        GrammarExpr::Not(inner) => {
            let inner = normalize(inner, label)?;
            Form {
                text: format!("not({})", inner.text),
                shape: Shape::Not(Box::new(inner)),
            }
        }
        GrammarExpr::Capture {
            label: name,
            expr: inner,
        } => {
            let inner = normalize(inner, label)?;
            Form {
                text: format!(
                    "capture({},{})",
                    name.as_deref().map_or_else(|| "_".to_owned(), quote),
                    inner.text
                ),
                shape: Shape::Capture {
                    label: name.clone(),
                    inner: Box::new(inner),
                },
            }
        }
    })
}

fn literal(value: &str) -> Form {
    Form::leaf(
        GrammarExpr::Terminal(value.to_owned()),
        format!("lit({})", quote(value)),
    )
}

fn char_range(start: char, end: char) -> Form {
    if start == end {
        return literal(&start.to_string());
    }
    Form::leaf(
        GrammarExpr::CharRange(start, end),
        format!(
            "range({},{})",
            quote(&start.to_string()),
            quote(&end.to_string())
        ),
    )
}

fn char_class(negated: bool, items: &[CharClassItem]) -> Form {
    let mut unique = BTreeMap::new();
    for item in items {
        match *item {
            CharClassItem::Range(start, end) if start != end => {
                unique.insert(
                    format!("{}-{}", quote(&start.to_string()), quote(&end.to_string())),
                    CharClassItem::Range(start, end),
                );
            }
            CharClassItem::Range(value, _) | CharClassItem::Char(value) => {
                unique.insert(quote(&value.to_string()), CharClassItem::Char(value));
            }
        }
    }
    if !negated && unique.len() == 1 {
        return match unique.into_values().next() {
            Some(CharClassItem::Range(start, end)) => char_range(start, end),
            Some(CharClassItem::Char(value)) => literal(&value.to_string()),
            None => Form::empty(),
        };
    }
    let texts: Vec<&str> = unique.keys().map(String::as_str).collect();
    let text = format!(
        "class({}[{}])",
        if negated { "!" } else { "" },
        texts.join(",")
    );
    Form::leaf(
        GrammarExpr::CharClass {
            negated,
            items: unique.into_values().collect(),
        },
        text,
    )
}

fn sequence_form(forms: Vec<Form>) -> Form {
    let mut items: Vec<Form> = Vec::new();
    for form in forms {
        let parts = match form.shape {
            Shape::Sequence(parts) => parts,
            shape => vec![Form {
                text: form.text,
                shape,
            }],
        };
        for part in parts {
            if part.is_empty() {
                continue;
            }
            // `x x*` matches exactly what `x+` matches, in PEG and in CFG alike.
            let folds = matches!(
                (&part.shape, items.last()),
                (Shape::Repeat { min: 0, max: None, inner }, Some(previous)) if inner.text == previous.text
            );
            if folds && let Some(previous) = items.pop() {
                items.push(some_form(previous));
            } else {
                items.push(part);
            }
        }
    }
    match items.len() {
        0 => Form::empty(),
        1 => items.remove(0),
        _ => Form {
            text: format!(
                "seq({})",
                items
                    .iter()
                    .map(|item| item.text.as_str())
                    .collect::<Vec<_>>()
                    .join(",")
            ),
            shape: Shape::Sequence(items),
        },
    }
}

fn choice_form(forms: Vec<Form>, ordered: bool) -> Form {
    let mut flattened = Vec::new();
    for form in forms {
        match form.shape {
            Shape::Choice {
                ordered: nested,
                items,
            } if nested == ordered => flattened.extend(items),
            shape => flattened.push(Form {
                text: form.text,
                shape,
            }),
        }
    }
    // A repeated alternative adds nothing: in an ordered choice the later copy
    // can never match where the earlier one failed, and union is idempotent.
    let mut seen = BTreeSet::new();
    let mut items: Vec<Form> = flattened
        .into_iter()
        .filter(|form| seen.insert(form.text.clone()))
        .collect();
    if !ordered {
        items.sort_by(|left, right| left.text.cmp(&right.text));
    }
    if items.len() == 1 {
        return items.remove(0);
    }
    Form {
        text: format!(
            "{}({})",
            if ordered { "first" } else { "alt" },
            items
                .iter()
                .map(|item| item.text.as_str())
                .collect::<Vec<_>>()
                .join(",")
        ),
        shape: Shape::Choice { ordered, items },
    }
}

fn some_form(inner: Form) -> Form {
    Form {
        text: format!("some({})", inner.text),
        shape: Shape::Repeat {
            min: 1,
            max: None,
            inner: Box::new(inner),
        },
    }
}

fn repeat_form(inner: Form, min: usize, max: Option<usize>) -> Result<Form, GrammarMergeError> {
    if max.is_some_and(|max| max < min) {
        return Err(GrammarMergeError::new(format!(
            "invalid repetition bounds {min}..{}",
            max.map_or_else(String::new, |max| max.to_string())
        )));
    }
    if inner.is_empty() || max == Some(0) {
        return Ok(Form::empty());
    }
    let text = match (min, max) {
        (1, Some(1)) => return Ok(inner),
        (0, None) => format!("many({})", inner.text),
        (1, None) => format!("some({})", inner.text),
        (0, Some(1)) => format!("opt({})", inner.text),
        (min, max) => format!(
            "rep({min},{},{})",
            max.map_or_else(|| "*".to_owned(), |max| max.to_string()),
            inner.text
        ),
    };
    Ok(Form {
        text,
        shape: Shape::Repeat {
            min,
            max,
            inner: Box::new(inner),
        },
    })
}

fn quote(value: &str) -> String {
    serde_json::to_string(value).unwrap_or_default()
}
