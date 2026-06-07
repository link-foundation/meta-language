use std::collections::{BTreeMap, BTreeSet};
use std::sync::OnceLock;

use crate::link_network::{Link, LinkId, LinkMetadata, LinkNetwork, LinkType};
use serde_json::Value;

/// Summary returned after seeding the shared concept ontology into a network.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ConceptOntologySeedReport {
    lexicon_concepts: usize,
    structural_concepts: usize,
    syntax_mappings: usize,
}

impl ConceptOntologySeedReport {
    const fn new(
        lexicon_concepts: usize,
        structural_concepts: usize,
        syntax_mappings: usize,
    ) -> Self {
        Self {
            lexicon_concepts,
            structural_concepts,
            syntax_mappings,
        }
    }

    /// Number of concepts imported from meta-expression's semantic lexicon JSON.
    #[must_use]
    pub const fn lexicon_concepts(self) -> usize {
        self.lexicon_concepts
    }

    /// Number of built-in structural programming-language concepts seeded.
    #[must_use]
    pub const fn structural_concepts(self) -> usize {
        self.structural_concepts
    }

    /// Number of semantic concrete-syntax mapping links surfaced by the seed.
    #[must_use]
    pub const fn syntax_mappings(self) -> usize {
        self.syntax_mappings
    }
}

struct SemanticLexicon {
    concept_count: usize,
    concepts: Vec<SemanticLexiconConcept>,
}

struct SemanticLexiconConcept {
    id: String,
    entity_id: Option<String>,
    url: Option<String>,
    description: Option<String>,
    labels: BTreeMap<String, Vec<String>>,
    primary: BTreeMap<String, String>,
}

impl SemanticLexiconConcept {
    fn id(&self) -> &str {
        &self.id
    }

    fn definition(&self) -> String {
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

    fn syntax_entries(&self) -> Vec<ConceptSyntaxEntry<'_>> {
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

struct ConceptSyntaxEntry<'a> {
    language: &'a str,
    syntax: &'a str,
    canonical: bool,
}

struct StructuralConcept {
    id: &'static str,
    definition: &'static str,
    syntax: &'static [(&'static str, &'static str)],
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StatehoodConceptIds {
    pub proposition: LinkId,
    pub subject: LinkId,
    pub object: LinkId,
}

const STATEHOOD_PROPOSITION_SYNTAX: &[(&str, &str)] = &[
    ("English", "Hawaii is a state."),
    ("en", "Hawaii is a state."),
    ("Russian", "Гавайи это штат."),
    ("ru", "Гавайи это штат."),
];

const HAWAII_ENTITY_SYNTAX: &[(&str, &str)] = &[
    ("English", "Hawaii"),
    ("en", "Hawaii"),
    ("Russian", "Гавайи"),
    ("ru", "Гавайи"),
];

const UNITED_STATES_STATE_SYNTAX: &[(&str, &str)] = &[
    ("English", "state"),
    ("en", "state"),
    ("Russian", "штат"),
    ("ru", "штат"),
];

const STRUCTURAL_CONCEPTS: &[StructuralConcept] = &[
    StructuralConcept {
        id: "function",
        definition: "Reusable computation with parameters and a result boundary.",
        syntax: &[
            ("Rust", "fn"),
            ("Python", "def"),
            ("JavaScript", "function"),
            ("C", "function"),
            ("C++", "function"),
            ("C#", "method"),
            ("Java", "method"),
            ("Visual Basic", "Function"),
            ("R", "function"),
            ("sql-ansi", "CREATE FUNCTION"),
            ("Delphi/Object Pascal", "function"),
        ],
    },
    StructuralConcept {
        id: "binding",
        definition: "Association between a name and a value or computation.",
        syntax: &[
            ("Rust", "let"),
            ("Python", "="),
            ("JavaScript", "let"),
            ("C", "="),
            ("C++", "="),
            ("C#", "="),
            ("Java", "="),
            ("Visual Basic", "Dim"),
            ("R", "<-"),
            ("sql-ansi", "AS"),
            ("Delphi/Object Pascal", ":="),
        ],
    },
    StructuralConcept {
        id: "application",
        definition: "Application of a callable expression to arguments.",
        syntax: &[
            ("Rust", "call(...)"),
            ("Python", "call(...)"),
            ("JavaScript", "call(...)"),
            ("C", "call(...)"),
            ("C++", "call(...)"),
            ("C#", "call(...)"),
            ("Java", "call(...)"),
            ("Visual Basic", "Call"),
            ("R", "call(...)"),
            ("sql-ansi", "CALL"),
            ("Delphi/Object Pascal", "call(...)"),
        ],
    },
    StructuralConcept {
        id: "sequence",
        definition: "Ordered execution or evaluation of multiple operations.",
        syntax: &[
            ("Rust", ";"),
            ("Python", "newline"),
            ("JavaScript", ";"),
            ("C", ";"),
            ("C++", ";"),
            ("C#", ";"),
            ("Java", ";"),
            ("Visual Basic", "newline"),
            ("R", ";"),
            ("sql-ansi", ";"),
            ("Delphi/Object Pascal", "begin ... end"),
        ],
    },
    StructuralConcept {
        id: "branch",
        definition: "Conditional selection among alternative operations.",
        syntax: &[
            ("Rust", "if"),
            ("Python", "if"),
            ("JavaScript", "if"),
            ("C", "if"),
            ("C++", "if"),
            ("C#", "if"),
            ("Java", "if"),
            ("Visual Basic", "If"),
            ("R", "if"),
            ("sql-ansi", "CASE"),
            ("Delphi/Object Pascal", "if"),
        ],
    },
    StructuralConcept {
        id: "loop",
        definition: "Repeated execution or evaluation over a condition or iterable.",
        syntax: &[
            ("Rust", "loop"),
            ("Python", "for"),
            ("JavaScript", "for"),
            ("C", "for"),
            ("C++", "for"),
            ("C#", "for"),
            ("Java", "for"),
            ("Visual Basic", "For"),
            ("R", "for"),
            ("sql-ansi", "WHILE"),
            ("Delphi/Object Pascal", "for"),
        ],
    },
    StructuralConcept {
        id: "parameter",
        definition: "Named input accepted by a function abstraction.",
        syntax: &[
            ("Rust", "parameter"),
            ("Python", "parameter"),
            ("JavaScript", "parameter"),
            ("C", "parameter"),
            ("C++", "parameter"),
            ("C#", "parameter"),
            ("Java", "parameter"),
            ("Visual Basic", "parameter"),
            ("R", "parameter"),
            ("sql-ansi", "parameter"),
            ("Delphi/Object Pascal", "parameter"),
        ],
    },
    StructuralConcept {
        id: "argument",
        definition: "Concrete input supplied to a function application.",
        syntax: &[
            ("Rust", "argument"),
            ("Python", "argument"),
            ("JavaScript", "argument"),
            ("C", "argument"),
            ("C++", "argument"),
            ("C#", "argument"),
            ("Java", "argument"),
            ("Visual Basic", "argument"),
            ("R", "argument"),
            ("sql-ansi", "argument"),
            ("Delphi/Object Pascal", "argument"),
        ],
    },
    StructuralConcept {
        id: "return",
        definition: "Transfer of a function result to its caller.",
        syntax: &[
            ("Rust", "return"),
            ("Python", "return"),
            ("JavaScript", "return"),
            ("C", "return"),
            ("C++", "return"),
            ("C#", "return"),
            ("Java", "return"),
            ("Visual Basic", "Return"),
            ("R", "return"),
            ("sql-ansi", "RETURN"),
            ("Delphi/Object Pascal", "Result"),
        ],
    },
    StructuralConcept {
        id: "assignment",
        definition: "Update that stores a value into a named location.",
        syntax: &[
            ("Rust", "="),
            ("Python", "="),
            ("JavaScript", "="),
            ("C", "="),
            ("C++", "="),
            ("C#", "="),
            ("Java", "="),
            ("Visual Basic", "="),
            ("R", "<-"),
            ("sql-ansi", "="),
            ("Delphi/Object Pascal", ":="),
        ],
    },
];

impl LinkNetwork {
    pub(crate) fn seed_statehood_worked_example(&mut self) -> StatehoodConceptIds {
        let proposition = self.insert_typed_point(
            "statehood",
            LinkType::Concept,
            Some("Statehood proposition connecting Hawaii (Q782) to U.S. state (Q35657)."),
        );
        let subject = self.insert_typed_point(
            "Q782",
            LinkType::Concept,
            Some("Wikidata Q782; Hawaii; state of the United States."),
        );
        let object = self.insert_typed_point(
            "Q35657",
            LinkType::Concept,
            Some("Wikidata Q35657; state of the United States."),
        );

        for (language, syntax) in STATEHOOD_PROPOSITION_SYNTAX {
            self.insert_concept_syntax_mapping(proposition, "statehood", language, syntax, true);
        }
        for (language, syntax) in HAWAII_ENTITY_SYNTAX {
            self.insert_concept_syntax_mapping(subject, "Q782", language, syntax, true);
        }
        for (language, syntax) in UNITED_STATES_STATE_SYNTAX {
            self.insert_concept_syntax_mapping(object, "Q35657", language, syntax, true);
        }

        StatehoodConceptIds {
            proposition,
            subject,
            object,
        }
    }

    /// Seeds the network with the shared common concept ontology.
    ///
    /// The seed combines meta-expression's semantic lexicon with structural
    /// programming-language concepts that are shared across the current
    /// language targets.
    #[must_use]
    pub fn seed_common_concept_ontology(&mut self) -> ConceptOntologySeedReport {
        let lexicon = semantic_lexicon();
        let mut syntax_mappings = 0;

        for concept in &lexicon.concepts {
            let definition = concept.definition();
            let concept_link =
                self.insert_typed_point(concept.id(), LinkType::Concept, Some(&definition));

            for entry in concept.syntax_entries() {
                self.insert_concept_syntax_mapping(
                    concept_link,
                    concept.id(),
                    entry.language,
                    entry.syntax,
                    entry.canonical,
                );
                syntax_mappings += 1;
            }
        }

        let mut structural_concepts = BTreeSet::new();
        for concept in STRUCTURAL_CONCEPTS {
            structural_concepts.insert(concept.id);
            let concept_link =
                self.insert_typed_point(concept.id, LinkType::Concept, Some(concept.definition));

            for (language, syntax) in concept.syntax {
                self.insert_concept_syntax_mapping(
                    concept_link,
                    concept.id,
                    language,
                    syntax,
                    true,
                );
                syntax_mappings += 1;
            }
        }

        let _ = self.seed_statehood_worked_example();
        syntax_mappings += STATEHOOD_PROPOSITION_SYNTAX.len()
            + HAWAII_ENTITY_SYNTAX.len()
            + UNITED_STATES_STATE_SYNTAX.len();

        ConceptOntologySeedReport::new(
            lexicon.concept_count,
            structural_concepts.len(),
            syntax_mappings,
        )
    }

    /// Inserts a concept-to-language syntax mapping and returns the semantic link id.
    pub fn insert_concept_mapping(
        &mut self,
        concept: &str,
        language: &str,
        syntax: &str,
    ) -> LinkId {
        let concept_link = self.insert_typed_point(
            concept,
            LinkType::Concept,
            Some("A concept mapping connects shared meaning to language syntax."),
        );
        self.insert_concept_syntax_mapping(concept_link, concept, language, syntax, true)
    }

    fn insert_concept_syntax_mapping(
        &mut self,
        concept_link: LinkId,
        concept: &str,
        language: &str,
        syntax: &str,
        update_reconstruction: bool,
    ) -> LinkId {
        let language_link = self.insert_typed_point(language, LinkType::Language, None);
        self.cache_concept_syntax(concept, language, syntax, update_reconstruction);

        if let Some(existing) =
            self.find_concept_syntax_mapping(concept_link, language_link, syntax, language)
        {
            return existing;
        }

        self.insert_link(
            [concept_link, language_link],
            LinkMetadata::new()
                .with_link_type(LinkType::Semantic)
                .with_named(true)
                .with_term(syntax)
                .with_language(language),
        )
    }

    fn find_concept_syntax_mapping(
        &self,
        concept_link: LinkId,
        language_link: LinkId,
        syntax: &str,
        language: &str,
    ) -> Option<LinkId> {
        self.links()
            .find(|link| {
                let references = link.references();
                link.metadata().link_type() == Some(LinkType::Semantic)
                    && references.len() == 2
                    && references[0] == concept_link
                    && references[1] == language_link
                    && link.metadata().term() == Some(syntax)
                    && link.metadata().language() == Some(language)
            })
            .map(Link::id)
    }
}

const SEMANTIC_LEXICON_JSON: &str = include_str!("data/semantic-lexicon.json");

fn semantic_lexicon() -> &'static SemanticLexicon {
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

fn is_wikidata_qid(value: &str) -> bool {
    value.strip_prefix('Q').is_some_and(|suffix| {
        !suffix.is_empty() && suffix.chars().all(|character| character.is_ascii_digit())
    })
}
