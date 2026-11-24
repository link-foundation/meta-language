//! Source-qualified batch transport through existing grammar decorators and
//! complete graph archives. All binding decisions are automatically translated.
use crate::decorators::{Decorator, DecoratorAction, DecoratorLevel, DecoratorSet};
use crate::discovered_concept_links::{
    DiscoveredConceptGraph, create_discovered_concept_links, read_discovered_concept_links,
};
use crate::global_concept_space::{GlobalConceptOptions, GlobalConceptSource};
use crate::grammar::decorate_grammar;
use crate::grammar_composition_batch_rules::{self as rules, ml_string::Text};
use crate::meta_language_archive::{
    MetaLanguageArchive, MetaLanguageArchiveEntry, MetaLanguageArchiveLimits,
    create_meta_language_archive, read_meta_language_archive,
};
use crate::structured_data_links::{decode_structured_data_links, encode_structured_data_links};
use crate::{LinkId, LinkNetwork};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::{collections::BTreeMap, sync::Arc};

/// One exact source-owned rule and its existing Links composition template.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GrammarCompositionPolicy {
    /// Existing source identity, independent of language and rule spellings.
    pub source: String,
    /// Exact existing normal rule.
    pub rule: String,
    /// Complete any/all template with the existing empty insertion slot.
    pub composition: String,
}
/// Reconstructed current grammars and their complete original context.
#[derive(Clone, Debug)]
pub struct GrammarCompositionBatch {
    /// Current sources sharing the revalidated global concept space.
    pub current: DiscoveredConceptGraph,
    /// Complete predecessor definitions and their original proof authority.
    pub original: DiscoveredConceptGraph,
    /// Ordered, fully checked source-qualified requests.
    pub policies: Vec<GrammarCompositionPolicy>,
    /// Complete stored source networks and selected entry roots.
    pub entries: Vec<MetaLanguageArchiveEntry>,
}
impl std::ops::Deref for GrammarCompositionBatch {
    type Target = DiscoveredConceptGraph;
    fn deref(&self) -> &Self::Target {
        &self.current
    }
}
fn fail(message: impl AsRef<str>) -> String {
    format!("grammar composition batch: {}", message.as_ref())
}
fn name(role: &str) -> String {
    rules::read_grammar_composition_batch_entry_name(&Text::from_text(role)).to_string()
}
fn require_fields(value: &Value, role: &str) -> Result<(), String> {
    let declaration =
        rules::read_grammar_composition_batch_field_order(&Text::from_text(role)).to_string();
    let fields = declaration.split('|').collect::<Vec<_>>();
    let object = value
        .as_object()
        .ok_or_else(|| fail(format!("{role} needs ordinary data")))?;
    if object.len() != fields.len() || fields.iter().any(|field| !object.contains_key(*field)) {
        return Err(fail(format!("{role} needs exactly its declared fields")));
    }
    Ok(())
}
fn apply(
    sources: &[GlobalConceptSource],
    policies: &[GrammarCompositionPolicy],
) -> Result<Vec<GlobalConceptSource>, String> {
    let number = |count: usize| {
        serde_json::Number::from(count)
            .as_f64()
            .expect("actual source count")
    };
    let count_issue = rules::read_grammar_composition_batch_count_issue(
        number(sources.len()),
        number(
            sources
                .iter()
                .map(|source| source.grammar.rules().len())
                .sum(),
        ),
        number(policies.len()),
    )
    .to_string();
    if !count_issue.is_empty() {
        return Err(fail(count_issue));
    }
    let mut units = 0usize;
    for source in sources {
        units = units.saturating_add(source.id.encode_utf16().count());
        let issue = rules::read_grammar_composition_batch_text_issue(number(units)).to_string();
        if !issue.is_empty() {
            return Err(fail(issue));
        }
    }
    for source in sources {
        for rule in source.grammar.rules() {
            units = units
                .saturating_add(source.id.encode_utf16().count())
                .saturating_add(1)
                .saturating_add(rule.name().encode_utf16().count());
            let issue = rules::read_grammar_composition_batch_text_issue(number(units)).to_string();
            if !issue.is_empty() {
                return Err(fail(issue));
            }
        }
    }
    for row in policies {
        units = units
            .saturating_add(row.source.encode_utf16().count())
            .saturating_add(1)
            .saturating_add(row.rule.encode_utf16().count())
            .saturating_add(row.composition.encode_utf16().count());
        let issue = rules::read_grammar_composition_batch_text_issue(number(units)).to_string();
        if !issue.is_empty() {
            return Err(fail(issue));
        }
    }
    let source_ids = sources
        .iter()
        .map(|source| Text::from_text(&source.id))
        .collect::<Vec<_>>();
    let mut owners = Vec::new();
    let mut names = Vec::new();
    let mut kinds = Vec::new();
    for source in sources {
        for rule in source.grammar.rules() {
            owners.push(Text::from_text(&source.id));
            names.push(Text::from_text(rule.name()));
            kinds.push(Text::from_text(rule.kind().as_str()));
        }
    }
    let policy_sources = policies
        .iter()
        .map(|row| Text::from_text(&row.source))
        .collect::<Vec<_>>();
    let policy_rules = policies
        .iter()
        .map(|row| Text::from_text(&row.rule))
        .collect::<Vec<_>>();
    let compositions = policies
        .iter()
        .map(|row| Text::from_text(&row.composition))
        .collect::<Vec<_>>();
    let issue = rules::read_grammar_composition_batch_issue(
        &source_ids,
        &owners,
        &names,
        &kinds,
        &policy_sources,
        &policy_rules,
        &compositions,
    )
    .to_string();
    if !issue.is_empty() {
        return Err(fail(issue));
    }
    let mut grouped: BTreeMap<&str, Vec<Decorator>> = BTreeMap::new();
    for (index, row) in policies.iter().enumerate() {
        grouped.entry(&row.source).or_default().push(
            Decorator::new(
                format!("composition-batch-{index}"),
                DecoratorLevel::GrammarRule,
                0,
                vec![("name".to_owned(), row.rule.clone())],
                vec![DecoratorAction::Set {
                    field: "composition".to_owned(),
                    value: row.composition.clone(),
                }],
            )
            .map_err(|error| fail(error.to_string()))?,
        );
    }
    sources
        .iter()
        .map(|source| {
            let set = DecoratorSet::new(grouped.remove(source.id.as_str()).unwrap_or_default())
                .map_err(|error| fail(error.to_string()))?;
            let grammar = decorate_grammar(&source.grammar, &set, DecoratorLevel::GrammarRule)
                .map_err(|error| fail(error.to_string()))?;
            Ok(GlobalConceptSource {
                id: source.id.clone(),
                language: source.language.clone(),
                edition: source.edition.clone(),
                grammar,
            })
        })
        .collect()
}
fn entry(role: &str, kind: &str, network: LinkNetwork, root: LinkId) -> MetaLanguageArchiveEntry {
    MetaLanguageArchiveEntry {
        identifier: name(role),
        kind: kind.to_owned(),
        network: Arc::new(network),
        root,
    }
}
fn select<'a>(
    entries: &'a [MetaLanguageArchiveEntry],
    role: &str,
    kind: &str,
) -> Result<&'a MetaLanguageArchiveEntry, String> {
    entries
        .iter()
        .find(|entry| entry.identifier == name(role) && entry.kind == kind)
        .ok_or_else(|| fail(format!("missing {role} entry")))
}
fn graph_key(
    network: LinkNetwork,
    root: LinkId,
    limits: MetaLanguageArchiveLimits,
) -> Result<String, String> {
    let selected = MetaLanguageArchiveEntry {
        identifier: "composition-comparison".to_owned(),
        kind: "discovered concepts".to_owned(),
        network: Arc::new(network),
        root,
    };
    Ok(create_meta_language_archive(&[selected], limits)?
        .network
        .to_lino())
}

/// Compose a complete batch and retain its original graph, configuration and
/// actual shared decision definition as distinct entries. No parser runs.
///
/// # Errors
/// Refuses malformed archives, unknown/duplicate targets, unsupported existing
/// composition forms, invalid concept contexts or exceeded resource bounds.
pub fn create_grammar_composition_batch_links(
    network: &LinkNetwork,
    root: LinkId,
    policies: &[GrammarCompositionPolicy],
    limits: MetaLanguageArchiveLimits,
) -> Result<MetaLanguageArchive, String> {
    let original = read_discovered_concept_links(network, root)?;
    let current = create_discovered_concept_links(
        &apply(&original.sources, policies)?,
        &GlobalConceptOptions::default(),
    )?;
    let configuration = encode_structured_data_links(
        &json!({"schemaVersion":1,"model":"source-qualified grammar composition batch 1","policies":policies}),
    )?;
    let algorithm = crate::create_source_definition_program_links(
        "meta-language/source-program/grammar-composition-batch-program",
    )?;
    create_meta_language_archive(
        &[
            entry("original", "discovered concepts", network.clone(), root),
            entry(
                "current",
                "discovered concepts",
                current.network,
                current.root,
            ),
            entry(
                "configuration",
                "language configuration",
                configuration.network,
                configuration.root,
            ),
            entry(
                "algorithm",
                "typed program",
                algorithm.network,
                algorithm.root,
            ),
        ],
        limits,
    )
}

/// Eagerly reload a batch without source files; rederive its actual compositions
/// and compare complete source networks, including registrations and allocators.
/// Original proofs never transfer to changed definitions merely by name.
///
/// # Errors
/// Refuses incomplete entries, changed definitions, invalid configurations,
/// altered complete current graphs and all inherited composition errors.
pub fn read_grammar_composition_batch_links(
    network: &LinkNetwork,
    root: LinkId,
    limits: MetaLanguageArchiveLimits,
) -> Result<GrammarCompositionBatch, String> {
    let reconstructed = read_meta_language_archive(network, root, limits)?;
    if reconstructed.entries.len() != 4 {
        return Err(fail("the batch requires exactly its four entries"));
    }
    let original_entry = select(&reconstructed.entries, "original", "discovered concepts")?;
    let current_entry = select(&reconstructed.entries, "current", "discovered concepts")?;
    let configuration_entry = select(
        &reconstructed.entries,
        "configuration",
        "language configuration",
    )?;
    let algorithm = select(&reconstructed.entries, "algorithm", "typed program")?;
    let original = read_discovered_concept_links(&original_entry.network, original_entry.root)?;
    let current = read_discovered_concept_links(&current_entry.network, current_entry.root)?;
    let configuration =
        decode_structured_data_links(&configuration_entry.network, configuration_entry.root)?;
    require_fields(&configuration, "configuration")?;
    if !rules::accept_grammar_composition_batch_configuration(
        configuration["schemaVersion"].as_f64().unwrap_or(-1.0),
        &Text::from_text(configuration["model"].as_str().unwrap_or_default()),
    ) {
        return Err(fail("unsupported batch configuration"));
    }
    let requests = configuration["policies"]
        .as_array()
        .ok_or_else(|| fail("policies need a bounded dense array"))?;
    if requests.len() > 4096 {
        return Err(fail("policies need a bounded dense array"));
    }
    for request in requests {
        require_fields(request, "policy")?;
    }
    let policies: Vec<GrammarCompositionPolicy> =
        serde_json::from_value(configuration["policies"].clone())
            .map_err(|error| fail(error.to_string()))?;
    let expected = create_discovered_concept_links(
        &apply(&original.sources, &policies)?,
        &GlobalConceptOptions::default(),
    )?;
    if graph_key(
        current_entry.network.as_ref().clone(),
        current_entry.root,
        limits,
    )? != graph_key(expected.network, expected.root, limits)?
    {
        return Err(fail(
            "the current complete grammar graph differs from its actual composition",
        ));
    }
    let expected_definition = crate::create_source_definition_program_links(
        "meta-language/source-program/grammar-composition-batch-program",
    )?;
    if graph_key(algorithm.network.as_ref().clone(), algorithm.root, limits)?
        != graph_key(
            expected_definition.network,
            expected_definition.root,
            limits,
        )?
    {
        return Err(fail(
            "the batch decision definition differs from its selected implementation",
        ));
    }
    Ok(GrammarCompositionBatch {
        current,
        original,
        policies,
        entries: reconstructed.entries,
    })
}
