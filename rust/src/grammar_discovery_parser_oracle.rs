//! Native parser observations used as discovery inputs.
use crate::grammar_discovery_parser_oracle_rules as rules;
use crate::{
    FeatureParseOptions, GlobalConceptSource, Grammar, LinkNetwork, MetaLanguageArchiveEntry,
    MetaLanguageArchiveLimits, TypedProgramLinks, compile_feature_grammar,
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
    /// Exact Unicode scalar sequence passed to the parser.
    pub word: Vec<String>,
    /// Acceptance returned by the oracle within its explicit resource limits.
    pub accepted: bool,
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
    let groups = [positives, negatives, candidates];
    if groups
        .iter()
        .fold(0usize, |sum, group| sum.saturating_add(group.len()))
        > limit("queries")
    {
        return Err("grammar discovery oracle requires at most 256 input words".into());
    }
    for word in groups.into_iter().flatten() {
        if word.len() > limit("symbols") || word.iter().any(|symbol| symbol.chars().count() != 1) {
            return Err(
                "grammar discovery oracle words require at most 128 Unicode scalar symbols".into(),
            );
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
    let options = FeatureParseOptions::default();
    let mut queried = HashMap::new();
    let mut observations = Vec::new();
    for (words, expected) in [(positives, 1.0), (negatives, 0.0), (candidates, -1.0)] {
        for word in words {
            let source = word.concat();
            let accepted = match queried.entry(source) {
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
                    observations.push(GrammarDiscoveryOracleObservation {
                        word: word.clone(),
                        accepted: outcome.ok,
                    });
                    *entry.insert(outcome.ok)
                }
            };
            if !rules::accept_grammar_discovery_parser_oracle_label(expected, accepted) {
                return Err("grammar discovery oracle disagrees with a supplied label".into());
            }
        }
    }
    let declared: HashSet<_> = positives.iter().chain(negatives).collect();
    let mut result = GrammarDiscoveryOracleObservations {
        positives: positives.to_vec(),
        negatives: negatives.to_vec(),
        observations,
    };
    for observation in &result.observations {
        let selection = rules::read_grammar_discovery_parser_oracle_training_class(
            declared.contains(&observation.word),
            observation.accepted,
        );
        if selection > 0.0 {
            result.positives.push(observation.word.clone());
        } else if selection < 0.0 {
            result.negatives.push(observation.word.clone());
        }
    }
    Ok(result)
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct OraclePremises {
    positives: Vec<Vec<String>>,
    negatives: Vec<Vec<String>>,
    candidates: Vec<Vec<String>>,
    observations: Vec<GrammarDiscoveryOracleObservation>,
}

pub(crate) fn create_oracle_entry(
    grammar: &Grammar,
    positives: &[Vec<String>],
    negatives: &[Vec<String>],
    candidates: &[Vec<String>],
    observations: &[GrammarDiscoveryOracleObservation],
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
    let data: OraclePremises = serde_json::from_value(decode_structured_data_links(
        &premises.network,
        premises.root,
    )?)
    .map_err(|_| invalid())?;
    let replayed = observe_grammar_discovery_parser_oracle(
        &source.grammar,
        &data.positives,
        &data.negatives,
        &data.candidates,
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
