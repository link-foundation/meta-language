//! The concept records shared with the JavaScript runtime.
//!
//! `src/data/concept-records.json` is generated from
//! `parity/naming/canonical-concepts.json` by
//! `js/scripts/build-concept-records.mjs`; the npm package ships the same
//! file, so both runtimes agree on every canonical identity, its readable
//! English phrase, definition, constraints, source aliases and former names.

use std::collections::{BTreeMap, BTreeSet};
use std::sync::OnceLock;

use serde::Deserialize;

const CONCEPT_RECORDS_JSON: &str = include_str!("data/concept-records.json");

/// Whether a canonical name is a concept (a noun phrase) or an operation (a verb phrase).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ConceptRole {
    /// A concept, named by a noun phrase.
    Concept,
    /// An operation or relation, named by a verb phrase.
    Operation,
}

/// The name a source (a grammar format, a programming language, a surface) gives a concept.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct SourceAlias {
    /// The source, such as `ebnf`, `Rust` or `Russian grammar surface`.
    pub source: String,
    /// The name or notation the source uses for the concept.
    pub name: String,
}

/// A concept whose phrase is a synonym of another's, with what distinguishes them.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct ConceptDistinction {
    /// The identity of the synonymous concept.
    pub id: String,
    /// What distinguishes the two concepts.
    pub reason: String,
}

/// The record of one canonical concept or operation.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ConceptRecord {
    /// Stable identity, such as `grammar.character-class`.
    pub id: String,
    /// The readable English phrase the identity spells, such as `character class`.
    pub phrase: String,
    /// Whether the phrase is a noun phrase or a verb phrase.
    pub role: ConceptRole,
    /// What the concept means.
    pub definition: String,
    /// The constraints the concept satisfies.
    pub constraints: Vec<String>,
    /// The names other sources give the concept.
    pub source_aliases: Vec<SourceAlias>,
    /// Former names that still decode to the concept.
    pub former_names: Vec<String>,
    /// The concept this one is a use of, such as the operation a stage performs.
    #[serde(default)]
    pub represents: Option<String>,
    /// The concepts with a synonymous phrase this one is distinct from.
    #[serde(default)]
    pub distinct_from: Vec<ConceptDistinction>,
}

#[derive(Deserialize)]
struct ConceptRecordFile {
    concepts: Vec<ConceptRecord>,
}

/// Returns every concept record in register order.
#[must_use]
pub fn concept_records() -> &'static [ConceptRecord] {
    static RECORDS: OnceLock<Vec<ConceptRecord>> = OnceLock::new();
    RECORDS.get_or_init(|| {
        serde_json::from_str::<ConceptRecordFile>(CONCEPT_RECORDS_JSON)
            .expect("the embedded concept records are valid")
            .concepts
    })
}

/// Returns the record whose identity or former name is `name`.
#[must_use]
pub fn concept_record(name: &str) -> Option<&'static ConceptRecord> {
    static NAMES: OnceLock<BTreeMap<&'static str, &'static ConceptRecord>> = OnceLock::new();
    NAMES
        .get_or_init(|| {
            let mut names = BTreeMap::new();
            for record in concept_records() {
                names.entry(record.id.as_str()).or_insert(record);
                for former in &record.former_names {
                    names.entry(former.as_str()).or_insert(record);
                }
            }
            names
        })
        .get(name)
        .copied()
}

/// Returns the records a source (such as `ebnf` or `Rust`) names `name`, in register order.
#[must_use]
pub fn concept_records_for_source_name(source: &str, name: &str) -> Vec<&'static ConceptRecord> {
    type SourceNameIndex =
        BTreeMap<&'static str, BTreeMap<&'static str, Vec<&'static ConceptRecord>>>;
    static SOURCES: OnceLock<SourceNameIndex> = OnceLock::new();
    SOURCES
        .get_or_init(|| {
            let mut sources: SourceNameIndex = BTreeMap::new();
            for record in concept_records() {
                let mut seen = BTreeSet::new();
                for alias in &record.source_aliases {
                    let key = (alias.source.as_str(), alias.name.as_str());
                    if seen.insert(key) {
                        sources
                            .entry(key.0)
                            .or_default()
                            .entry(key.1)
                            .or_default()
                            .push(record);
                    }
                }
            }
            sources
        })
        .get(source)
        .and_then(|names| names.get(name))
        .cloned()
        .unwrap_or_default()
}
