//! Native parser observations used as discovery inputs.
use crate::grammar_discovery_parser_oracle_rules as rules;
use crate::{
    FeatureParseOptions, GlobalConceptSource, Grammar, LinkNetwork, MetaLanguageArchiveEntry,
    MetaLanguageArchiveLimits, SyntaxTree, TypedProgramLinks, compile_feature_grammar,
    create_discovered_concept_links, create_meta_language_archive, decode_structured_data_links,
    decode_typed_program_links, encode_structured_data_links, read_discovered_concept_links,
    read_meta_language_archive,
};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

pub(crate) const GRAMMAR_DISCOVERY_PARSER_ORACLE: &str = "regular-discovery-parser-oracle";

#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
fn limit(resource: &str) -> usize {
    // The shared policy returns finite nonnegative integer constants.
    rules::read_grammar_discovery_parser_oracle_limit(resource) as usize
}

fn create_policy_links() -> Result<TypedProgramLinks, String> {
    let network = LinkNetwork::from_lino(include_str!(
        "data/grammar-discovery-parser-oracle-program.lino"
    ))
    .map_err(|error| error.to_string())?;
    let roots = network
        .links()
        .filter(|link| {
            link.metadata().language() == Some("meta-language typed program")
                && link.metadata().term() == Some("typed program")
                && link.metadata().definition() == Some("1")
        })
        .map(crate::Link::id)
        .collect::<Vec<_>>();
    let [root] = roots.as_slice() else {
        return Err("grammar discovery oracle needs one typed policy root".into());
    };
    decode_typed_program_links(&network, *root).map_err(|error| error.to_string())?;
    Ok(TypedProgramLinks {
        network,
        root: *root,
    })
}

/// The answer to one distinct discovery query, in query order.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GrammarDiscoveryOracleObservation {
    /// Unicode scalars or named scanner tokens for the query.
    pub word: Vec<String>,
    /// Acceptance returned by the oracle within its explicit resource limits.
    pub accepted: bool,
    /// The exact source spelling for a scanner query.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
}

/// Native source spellings aligned with the three lists of scanner words.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GrammarDiscoveryOracleSources {
    /// Source spellings for declared positive words.
    pub positives: Vec<String>,
    /// Source spellings for declared negative words.
    pub negatives: Vec<String>,
    /// Source spellings for unlabelled candidate words.
    pub candidates: Vec<String>,
}

/// Checked training inputs and the oracle calls that produced them.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GrammarDiscoveryOracleObservations {
    /// Declared positives followed by newly accepted candidates.
    pub positives: Vec<Vec<String>>,
    /// Declared negatives followed by newly rejected candidates.
    pub negatives: Vec<Vec<String>>,
    /// Each distinct word in the order first queried.
    pub observations: Vec<GrammarDiscoveryOracleObservation>,
}

/// Validate declared labels and classify independent Unicode scalar candidates
/// with an existing native grammar. Inconclusive parser outcomes are errors.
///
/// # Errors
/// Returns an error for invalid words, conflicting labels, or an oracle that
/// cannot compile or classify a word inside its bounded query profile.
pub fn observe_grammar_discovery_parser_oracle(
    grammar: &Grammar,
    positives: &[Vec<String>],
    negatives: &[Vec<String>],
    candidates: &[Vec<String>],
) -> Result<GrammarDiscoveryOracleObservations, String> {
    observe_parser_oracle(grammar, positives, negatives, candidates, None)
}

fn accepted_token_sequence(tree: Option<&SyntaxTree>) -> Result<Vec<&str>, String> {
    let mut pending: Vec<_> = tree.into_iter().collect();
    let mut sequence = Vec::new();
    while let Some(node) = pending.pop() {
        match node {
            SyntaxTree::Node { trivia: true, .. } | SyntaxTree::Token { trivia: true, .. } => {}
            SyntaxTree::Node { children, .. } => pending.extend(children.iter().rev()),
            SyntaxTree::Token {
                kind: Some(kind), ..
            } => sequence.push(kind.as_str()),
            _ => return Err("scanner oracle requires a complete named token sequence".into()),
        }
    }
    Ok(sequence)
}

pub(crate) fn observe_parser_oracle(
    grammar: &Grammar,
    positives: &[Vec<String>],
    negatives: &[Vec<String>],
    candidates: &[Vec<String>],
    sources: Option<&GrammarDiscoveryOracleSources>,
) -> Result<GrammarDiscoveryOracleObservations, String> {
    let groups = [positives, negatives, candidates];
    let spellings =
        sources.map(|source| [&source.positives, &source.negatives, &source.candidates]);
    if groups
        .iter()
        .fold(0usize, |sum, group| sum.saturating_add(group.len()))
        > limit("queries")
    {
        return Err("grammar discovery oracle requires at most 256 input words".into());
    }
    if let Some(spellings) = spellings
        && spellings.iter().zip(groups).any(|(sources, words)| {
            sources.len() != words.len()
                || sources
                    .iter()
                    .any(|source| source.len() > limit("source bytes"))
        })
    {
        return Err("scanner oracle needs one bounded source spelling per word".into());
    }
    for word in groups.into_iter().flatten() {
        let scanner = spellings.is_some();
        if word.len() > limit("symbols")
            || if scanner {
                word.iter().any(String::is_empty)
                    || word
                        .iter()
                        .fold(0usize, |sum, symbol| sum.saturating_add(symbol.len()))
                        > limit("source bytes")
            } else {
                word.iter().any(|symbol| symbol.chars().count() != 1)
            }
        {
            return Err(if scanner {
                "scanner oracle words require bounded nonempty token names"
            } else {
                "grammar discovery oracle words require at most 128 Unicode scalar symbols"
            }
            .into());
        }
    }
    let defaults = FeatureParseOptions {
        max_depth: Some(limit("nesting")),
        step_limit: Some(limit("steps")),
        memo_limit: Some(limit("memo")),
        memory_limit: Some(limit("memory")),
        ..FeatureParseOptions::default()
    };
    let parser =
        compile_feature_grammar(grammar, None, defaults).map_err(|error| error.to_string())?;
    let options = FeatureParseOptions {
        reject_ambiguity: spellings.is_some().then_some(true),
        ..FeatureParseOptions::default()
    };
    let mut queried = HashMap::new();
    let mut answers = HashMap::new();
    let mut observations = Vec::new();
    let declared: HashSet<_> = positives.iter().chain(negatives).collect();
    let mut training_positives = positives.to_vec();
    let mut training_negatives = negatives.to_vec();
    for (group, (words, expected)) in groups.into_iter().zip([1.0, 0.0, -1.0]).enumerate() {
        for (index, word) in words.iter().enumerate() {
            let source = spellings.map_or_else(
                || word.concat(),
                |spellings| spellings[group][index].clone(),
            );
            let (previous_word, accepted) = match queried.entry(source) {
                std::collections::hash_map::Entry::Occupied(entry) => *entry.get(),
                std::collections::hash_map::Entry::Vacant(entry) => {
                    let outcome = parser
                        .parse_tree(entry.key().as_bytes(), &options)
                        .map_err(|error| error.to_string())?;
                    let reason = outcome
                        .rejection
                        .as_ref()
                        .map_or("unknown", |value| value.reason);
                    if !rules::accept_grammar_discovery_parser_oracle_answer(outcome.ok, reason) {
                        return Err(format!(
                            "grammar discovery oracle could not classify input: {reason}"
                        ));
                    }
                    if spellings.is_some()
                        && outcome.ok
                        && !accepted_token_sequence(outcome.tree.as_ref())?
                            .into_iter()
                            .eq(word.iter().map(String::as_str))
                    {
                        return Err(
                            "scanner oracle token sequence differs from the accepted source".into(),
                        );
                    }
                    observations.push(GrammarDiscoveryOracleObservation {
                        word: word.clone(),
                        accepted: outcome.ok,
                        source: spellings.is_some().then(|| entry.key().clone()),
                    });
                    *entry.insert((word, outcome.ok))
                }
            };
            if previous_word != word {
                return Err("scanner oracle source has conflicting token sequences".into());
            }
            if !rules::accept_grammar_discovery_parser_oracle_label(expected, accepted) {
                return Err("grammar discovery oracle disagrees with a supplied label".into());
            }
            if let Some(previous) = answers.insert(word, accepted) {
                if previous != accepted {
                    return Err("scanner oracle spellings for one token sequence disagree".into());
                }
                continue;
            }
            let selection = rules::read_grammar_discovery_parser_oracle_training_class(
                declared.contains(word),
                accepted,
            );
            if selection > 0.0 {
                training_positives.push(word.clone());
            } else if selection < 0.0 {
                training_negatives.push(word.clone());
            }
        }
    }
    Ok(GrammarDiscoveryOracleObservations {
        positives: training_positives,
        negatives: training_negatives,
        observations,
    })
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct OraclePremises {
    positives: Vec<Vec<String>>,
    negatives: Vec<Vec<String>>,
    candidates: Vec<Vec<String>>,
    observations: Vec<GrammarDiscoveryOracleObservation>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    sources: Option<GrammarDiscoveryOracleSources>,
}

pub(crate) fn create_oracle_entry(
    grammar: &Grammar,
    positives: &[Vec<String>],
    negatives: &[Vec<String>],
    candidates: &[Vec<String>],
    observations: &[GrammarDiscoveryOracleObservation],
    sources: Option<&GrammarDiscoveryOracleSources>,
) -> Result<MetaLanguageArchiveEntry, String> {
    let definition = create_discovered_concept_links(
        &[GlobalConceptSource {
            id: "discovery-parser-oracle".into(),
            language: "Discovery parser oracle".into(),
            edition: "1".into(),
            grammar: grammar.clone(),
        }],
        &Default::default(),
    )?;
    let premises = serde_json::to_value(OraclePremises {
        positives: positives.to_vec(),
        negatives: negatives.to_vec(),
        candidates: candidates.to_vec(),
        observations: observations.to_vec(),
        sources: sources.cloned(),
    })
    .map_err(|error| error.to_string())?;
    let premises = encode_structured_data_links(&premises)?;
    let policy = create_policy_links()?;
    let archive = create_meta_language_archive(
        &[
            MetaLanguageArchiveEntry {
                identifier: "parser-oracle-definition".into(),
                kind: "discovered concepts".into(),
                network: Arc::new(definition.network),
                root: definition.root,
            },
            MetaLanguageArchiveEntry {
                identifier: "parser-oracle-policy".into(),
                kind: "typed program".into(),
                network: Arc::new(policy.network),
                root: policy.root,
            },
            MetaLanguageArchiveEntry {
                identifier: "parser-oracle-premises".into(),
                kind: "grammar inference".into(),
                network: Arc::new(premises.network),
                root: premises.root,
            },
        ],
        MetaLanguageArchiveLimits::default(),
    )?;
    Ok(MetaLanguageArchiveEntry {
        identifier: GRAMMAR_DISCOVERY_PARSER_ORACLE.into(),
        kind: "grammar inference".into(),
        network: Arc::new(archive.network),
        root: archive.root,
    })
}

pub(crate) fn read_oracle_entry(
    entry: &MetaLanguageArchiveEntry,
    positives: &[Vec<String>],
    negatives: &[Vec<String>],
    exact_training: bool,
    scanner: bool,
) -> Result<Vec<GrammarDiscoveryOracleObservation>, String> {
    let invalid = || "invalid grammar discovery oracle checkpoint".to_owned();
    if entry.kind != "grammar inference" {
        return Err(invalid());
    }
    let archive = read_meta_language_archive(
        &entry.network,
        entry.root,
        MetaLanguageArchiveLimits::default(),
    )?;
    let [definition, policy, premises] = archive.entries.as_slice() else {
        return Err(invalid());
    };
    if definition.identifier != "parser-oracle-definition"
        || definition.kind != "discovered concepts"
        || policy.identifier != "parser-oracle-policy"
        || policy.kind != "typed program"
        || premises.identifier != "parser-oracle-premises"
        || premises.kind != "grammar inference"
    {
        return Err(invalid());
    }
    let bundled = create_policy_links()?;
    if policy.root != bundled.root || policy.network.to_lino() != bundled.network.to_lino() {
        return Err(invalid());
    }
    let definition = read_discovered_concept_links(&definition.network, definition.root)?;
    let [source] = definition.sources.as_slice() else {
        return Err(invalid());
    };
    if source.id != "discovery-parser-oracle"
        || source.language != "Discovery parser oracle"
        || source.edition != "1"
    {
        return Err(invalid());
    }
    let value = decode_structured_data_links(&premises.network, premises.root)?;
    if value.get("sources").is_some() != scanner {
        return Err(invalid());
    }
    let data: OraclePremises = serde_json::from_value(value.clone()).map_err(|_| invalid())?;
    if serde_json::to_value(&data).map_err(|_| invalid())? != value {
        return Err(invalid());
    }
    let replayed = observe_parser_oracle(
        &source.grammar,
        &data.positives,
        &data.negatives,
        &data.candidates,
        data.sources.as_ref(),
    )?;
    if replayed.observations != data.observations
        || (exact_training
            && (replayed.positives.len() != positives.len()
                || replayed.negatives.len() != negatives.len()))
        || !positives.starts_with(&replayed.positives)
        || !negatives.starts_with(&replayed.negatives)
    {
        return Err(invalid());
    }
    Ok(replayed.observations)
}
