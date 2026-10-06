//! The bundled semantic lexicon (`data/semantic-lexicon.json`): its concepts,
//! their definitions and the concrete syntax each concept has per language.
use std::collections::{BTreeMap, BTreeSet};
use std::sync::OnceLock;

use serde_json::Value;

pub(super) struct SemanticLexicon {
    pub(super) concept_count: usize,
    pub(super) concepts: Vec<SemanticLexiconConcept>,
}

pub(super) struct SemanticLexiconConcept {
    id: String,
    pub(super) entity_id: Option<String>,
    url: Option<String>,
    description: Option<String>,
    labels: BTreeMap<String, Vec<String>>,
    primary: BTreeMap<String, String>,
}

impl SemanticLexiconConcept {
    pub(super) fn id(&self) -> &str {
        &self.id
    }

    pub(super) fn definition(&self) -> String {
        let mut details = Vec::new();
        if let Some(entity_id) = &self.entity_id {
            if is_wikidata_qid(entity_id) {
                details.push(format!("Wikidata {entity_id}"));
            } else {
                details.push(format!("entity {entity_id}"));
            }
        } else {
            details.push(format!("concept {}", self.id));
        }

        if let Some(description) = &self.description {
            details.push(description.clone());
        }
        if let Some(url) = &self.url {
            details.push(url.clone());
        }

        details.join("; ")
    }

    pub(super) fn syntax_entries(&self) -> Vec<ConceptSyntaxEntry<'_>> {
        let primary_languages = self
            .primary
            .keys()
            .map(String::as_str)
            .collect::<BTreeSet<_>>();
        let mut seen = BTreeSet::new();
        let mut entries = Vec::new();

        for (language, syntax) in &self.primary {
            push_syntax_entry(&mut entries, &mut seen, language, syntax, true);
        }

        for (language, labels) in &self.labels {
            for (index, label) in labels.iter().enumerate() {
                let canonical = !primary_languages.contains(language.as_str()) && index == 0;
                push_syntax_entry(&mut entries, &mut seen, language, label, canonical);
            }
        }

        entries
    }
}

pub(super) struct ConceptSyntaxEntry<'a> {
    pub(super) language: &'a str,
    pub(super) syntax: &'a str,
    pub(super) canonical: bool,
}

const SEMANTIC_LEXICON_JSON: &str = include_str!("../data/semantic-lexicon.json");

pub(super) fn semantic_lexicon() -> &'static SemanticLexicon {
    static LEXICON: OnceLock<SemanticLexicon> = OnceLock::new();
    LEXICON.get_or_init(parse_semantic_lexicon)
}

fn parse_semantic_lexicon() -> SemanticLexicon {
    let root: Value =
        serde_json::from_str(SEMANTIC_LEXICON_JSON).expect("semantic lexicon JSON must parse");
    let root = root
        .as_object()
        .expect("semantic lexicon root must be an object");
    let concepts = root
        .get("concepts")
        .and_then(Value::as_array)
        .expect("semantic lexicon concepts must be an array")
        .iter()
        .map(parse_concept)
        .collect::<Vec<_>>();
    let concept_count = root
        .get("conceptCount")
        .and_then(Value::as_u64)
        .map_or(concepts.len(), |count| {
            usize::try_from(count).expect("semantic lexicon concept count must fit usize")
        });

    assert_eq!(
        concept_count,
        concepts.len(),
        "semantic lexicon conceptCount must match concepts array length"
    );

    SemanticLexicon {
        concept_count,
        concepts,
    }
}

fn parse_concept(value: &Value) -> SemanticLexiconConcept {
    let concept = value
        .as_object()
        .expect("semantic lexicon concept must be an object");
    SemanticLexiconConcept {
        id: required_string_field(concept, "id"),
        entity_id: optional_string_field(concept, "entityId"),
        url: optional_string_field(concept, "url"),
        description: optional_string_field(concept, "description"),
        labels: string_list_map_field(concept, "labels"),
        primary: string_map_field(concept, "primary"),
    }
}

fn required_string_field(object: &serde_json::Map<String, Value>, field: &str) -> String {
    object
        .get(field)
        .and_then(Value::as_str)
        .unwrap_or_else(|| panic!("semantic lexicon field {field} must be a string"))
        .to_string()
}

fn optional_string_field(object: &serde_json::Map<String, Value>, field: &str) -> Option<String> {
    object
        .get(field)
        .and_then(Value::as_str)
        .map(str::to_string)
}

fn string_map_field(
    object: &serde_json::Map<String, Value>,
    field: &str,
) -> BTreeMap<String, String> {
    object
        .get(field)
        .and_then(Value::as_object)
        .map(|entries| {
            entries
                .iter()
                .filter_map(|(language, value)| {
                    Some((language.clone(), value.as_str()?.to_string()))
                })
                .collect()
        })
        .unwrap_or_default()
}

fn string_list_map_field(
    object: &serde_json::Map<String, Value>,
    field: &str,
) -> BTreeMap<String, Vec<String>> {
    object
        .get(field)
        .and_then(Value::as_object)
        .map(|entries| {
            entries
                .iter()
                .map(|(language, values)| {
                    (
                        language.clone(),
                        values
                            .as_array()
                            .into_iter()
                            .flatten()
                            .filter_map(Value::as_str)
                            .map(str::to_string)
                            .collect(),
                    )
                })
                .collect()
        })
        .unwrap_or_default()
}

fn push_syntax_entry<'a>(
    entries: &mut Vec<ConceptSyntaxEntry<'a>>,
    seen: &mut BTreeSet<(&'a str, &'a str)>,
    language: &'a str,
    syntax: &'a str,
    canonical: bool,
) {
    if seen.insert((language, syntax)) {
        entries.push(ConceptSyntaxEntry {
            language,
            syntax,
            canonical,
        });
    }
}

pub(super) fn is_wikidata_qid(value: &str) -> bool {
    value.strip_prefix('Q').is_some_and(|suffix| {
        !suffix.is_empty() && suffix.chars().all(|character| character.is_ascii_digit())
    })
}

pub(super) fn is_wordnet_cili_id(value: &str) -> bool {
    value.starts_with("ili:") || value.starts_with("ili-")
}
