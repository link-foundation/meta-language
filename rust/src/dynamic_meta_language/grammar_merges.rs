//! Install and validate grammar merge certificates over the retained sources.
use super::{DynamicMetaLanguage, DynamicMetaLanguageOptions, contexts, data_limits, fail};
use crate::dynamic_meta_language_rules::{
    accept_installed_grammar_merge_source_count, read_dynamic_meta_language_identifier,
    read_installed_grammar_merge_version,
};
use crate::{
    GlobalConceptSource, GrammarMergeOptions, GrammarMergeSource, MergedGrammarGroup,
    MetaLanguageArchiveEntry, MetaLanguageArchiveLimits, assert_merge_complete,
    encode_structured_data_links_with_limits, merge_grammars, render_grammar_links,
};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct GrammarMergeCertificate {
    schema_version: f64,
    sources: Vec<String>,
    result: String,
    fingerprint: String,
}

fn derive(
    sources: &[GlobalConceptSource],
    identifiers: &[String],
) -> Result<MergedGrammarGroup, String> {
    if !accept_installed_grammar_merge_source_count(identifiers.len() as f64)
        || identifiers.iter().collect::<BTreeSet<_>>().len() != identifiers.len()
    {
        return Err(fail("grammar merge needs distinct installed sources"));
    }
    let indexed: BTreeMap<_, _> = sources
        .iter()
        .map(|source| (source.id.as_str(), source))
        .collect();
    let selected: Vec<_> = identifiers
        .iter()
        .enumerate()
        .map(|(precedence, id)| {
            let source = indexed
                .get(id.as_str())
                .ok_or_else(|| fail("unknown source-qualified grammar identifier"))?;
            let precedence = u32::try_from(precedence)
                .map_err(|_| fail("grammar merge source position exceeds its limit"))?;
            Ok(
                GrammarMergeSource::new(&source.id, &source.language, source.grammar.clone())
                    .with_edition(&source.edition)
                    .with_precedence(precedence),
            )
        })
        .collect::<Result<_, String>>()?;
    if selected.iter().any(|source| {
        source.language != selected[0].language || source.edition != selected[0].edition
    }) {
        return Err(fail("grammar merge needs one language edition"));
    }
    let mut result = merge_grammars(
        &selected,
        &GrammarMergeOptions {
            reconcile: true,
            ..GrammarMergeOptions::default()
        },
    )
    .map_err(|error| fail(&error.to_string()))?;
    assert_merge_complete(&result).map_err(|error| fail(&error.to_string()))?;
    result
        .groups
        .pop()
        .ok_or_else(|| fail("missing installed grammar merge group"))
}

pub(super) fn read(
    entries: &[MetaLanguageArchiveEntry],
    sources: &[GlobalConceptSource],
    limits: MetaLanguageArchiveLimits,
    depth: usize,
) -> Result<BTreeMap<String, Vec<String>>, String> {
    let prefix = read_dynamic_meta_language_identifier("grammar merge");
    let positions: BTreeMap<_, _> = sources
        .iter()
        .enumerate()
        .map(|(index, source)| (source.id.as_str(), index))
        .collect();
    let mut reports = BTreeMap::new();
    for entry in entries.iter().filter(|entry| entry.kind == "grammar merge") {
        let certificate: GrammarMergeCertificate =
            serde_json::from_value(contexts::data(entry, limits, depth)?)
                .map_err(|_| fail("invalid installed grammar merge certificate"))?;
        if certificate.schema_version != read_installed_grammar_merge_version()
            || entry.identifier != format!("{prefix}{}", certificate.result)
        {
            return Err(fail("invalid installed grammar merge certificate"));
        }
        let position = *positions
            .get(certificate.result.as_str())
            .ok_or_else(|| fail("grammar merge inputs must precede their installed result"))?;
        if certificate.sources.iter().any(|id| {
            positions
                .get(id.as_str())
                .is_none_or(|index| *index >= position)
        }) {
            return Err(fail(
                "grammar merge inputs must precede their installed result",
            ));
        }
        let report = derive(sources, &certificate.sources)?;
        let actual = &sources[position];
        if certificate.fingerprint != report.fingerprint
            || actual.language != report.language
            || actual.edition != report.edition
            || render_grammar_links(&actual.grammar) != render_grammar_links(&report.grammar)
        {
            return Err(fail("stale installed grammar merge certificate"));
        }
        reports.insert(certificate.result, certificate.sources);
    }
    Ok(reports)
}

impl DynamicMetaLanguage {
    /// Read the original productions, aliases and decisions for an installed merge.
    ///
    /// # Errors
    /// Refuses any failure to derive the report from the retained source grammars.
    pub fn grammar_merge(&self, identifier: &str) -> Result<Option<MergedGrammarGroup>, String> {
        self.grammar_merges
            .get(identifier)
            .map(|sources| derive(&self.sources, sources))
            .transpose()
    }

    /// Apply the ordinary grammar merge with input identifiers in precedence order.
    ///
    /// # Errors
    /// Refuses missing or incompatible sources, duplicate results and archive errors.
    pub fn merge_installed_grammars(
        &self,
        identifiers: &[String],
        result: &str,
    ) -> Result<Self, String> {
        if result.is_empty() || self.sources.iter().any(|source| source.id == result) {
            return Err(fail("grammar merge needs a new result identity"));
        }
        let report = derive(&self.sources, identifiers)?;
        let source = GlobalConceptSource {
            id: result.to_owned(),
            language: report.language,
            edition: report.edition,
            grammar: report.grammar,
        };
        let data = serde_json::to_value(GrammarMergeCertificate {
            schema_version: read_installed_grammar_merge_version(),
            sources: identifiers.to_vec(),
            result: result.to_owned(),
            fingerprint: report.fingerprint,
        })
        .map_err(|error| fail(&error.to_string()))?;
        let links = encode_structured_data_links_with_limits(
            &data,
            data_limits(self.limits, self.depth_limit),
        )?;
        self.extend(
            &[source],
            &DynamicMetaLanguageOptions {
                entries: vec![MetaLanguageArchiveEntry {
                    identifier: format!(
                        "{}{result}",
                        read_dynamic_meta_language_identifier("grammar merge")
                    ),
                    kind: "grammar merge".to_owned(),
                    network: Arc::new(links.network),
                    root: links.root,
                }],
                ..DynamicMetaLanguageOptions::default()
            },
        )
    }
}
