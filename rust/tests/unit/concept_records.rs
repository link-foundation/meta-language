//! The concept records the runtime ships (docs/vision.md#readable-english-names):
//! every canonical concept has a stable identity, a readable English phrase, a
//! definition, constraints and source aliases; assigning and importing them
//! renames former identities and keeps them as aliases, and round trips
//! through the source formats are unaffected.

use std::collections::BTreeSet;
use std::path::PathBuf;

use meta_language::{
    concept_record, concept_records, concept_records_for_source_name, current_concept_id,
    emit_ebnf, grammar_from_lino, grammar_to_lino, import_ebnf, ConceptRole, LinkNetwork,
    LinkQuery, LinkType, ParseConfiguration, FORMER_CONCEPT_IDS, FORMER_CONCEPT_ID_VOCABULARY,
};
use serde_json::Value;

use super::issue_195_observations::{record, Observation};

const CANONICAL_CONCEPTS: &str = "parity/naming/canonical-concepts.json";

fn observe(requirement_id: &'static str, assertions: &[&str], test_name: &str) {
    let fixture_id = match requirement_id {
        "I195-NAMING-CONCEPT-RECORDS" => "planned:repository-directive:i195-naming-concept-records",
        _ => "planned:repository-directive:i195-naming-convention",
    };
    record(&Observation {
        requirement_id,
        suffix: "behavior",
        fixture_id,
        fixture_file: CANONICAL_CONCEPTS,
        assertions,
        test_name,
    });
}

fn repository_file(relative: &str) -> Value {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join(relative);
    let text = std::fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("read {}: {error}", path.display()));
    serde_json::from_str(&text).expect("the register is JSON")
}

/// The readable phrase an identity spells: its last segment with spaces for hyphens.
fn phrase_of(id: &str) -> String {
    id.rsplit(['.', ':']).next().unwrap_or(id).replace('-', " ")
}

fn has_alias(network: &LinkNetwork, concept: &str, vocabulary: &str, name: &str) -> bool {
    let concept = network.find_term(concept).expect("the concept is present");
    network
        .query_links(
            &LinkQuery::by_type(LinkType::Semantic)
                .with_language(vocabulary)
                .with_term(name),
        )
        .iter()
        .any(|link| link.references().first() == Some(&concept))
}

#[test]
fn the_runtime_records_are_the_register() {
    let register = repository_file(CANONICAL_CONCEPTS);
    let registered = register["concepts"].as_array().expect("concepts");
    let records = concept_records();
    assert_eq!(records.len(), registered.len());
    for (record, registered) in records.iter().zip(registered) {
        assert_eq!(registered["id"], record.id.as_str());
        assert_eq!(registered["phrase"], record.phrase.as_str());
        assert_eq!(registered["definition"], record.definition.as_str());
        let role = match record.role {
            ConceptRole::Concept => "concept",
            ConceptRole::Operation => "operation",
        };
        assert_eq!(registered["role"], role);
        let constraints: Vec<&str> = registered["constraints"]
            .as_array()
            .expect("constraints")
            .iter()
            .filter_map(Value::as_str)
            .collect();
        assert_eq!(record.constraints, constraints);
        let aliases: Vec<(&str, &str)> = registered["sourceAliases"]
            .as_array()
            .expect("source aliases")
            .iter()
            .map(|alias| {
                (
                    alias["source"].as_str().expect("source"),
                    alias["name"].as_str().expect("name"),
                )
            })
            .collect();
        let record_aliases: Vec<(&str, &str)> = record
            .source_aliases
            .iter()
            .map(|alias| (alias.source.as_str(), alias.name.as_str()))
            .collect();
        assert_eq!(record_aliases, aliases);
        assert_eq!(
            registered["formerNames"]
                .as_array()
                .expect("former names")
                .len(),
            record.former_names.len()
        );
        assert_eq!(
            registered["represents"].as_str(),
            record.represents.as_deref()
        );
        assert!(!record.definition.is_empty() && !record.constraints.is_empty());
        assert_eq!(concept_record(&record.id), Some(record));
        assert_eq!(
            record.phrase,
            phrase_of(&record.id),
            "{} spells its phrase",
            record.id
        );
        for alias in &record.source_aliases {
            assert!(concept_records_for_source_name(&alias.source, &alias.name).contains(&record));
        }
    }
    let identities: BTreeSet<&str> = records.iter().map(|record| record.id.as_str()).collect();
    assert_eq!(identities.len(), records.len());
    let rules: Vec<&str> = concept_records_for_source_name("ebnf", "=")
        .iter()
        .map(|record| record.id.as_str())
        .collect();
    assert_eq!(rules, ["grammar.rule"]);
    assert_eq!(concept_record("no such concept"), None);
    observe(
        "I195-NAMING-CONCEPT-RECORDS",
        &[
            "stableIdentity",
            "readablePhrase",
            "definitionRecorded",
            "constraintsRecorded",
            "sourceAliasesRecorded",
        ],
        "the_runtime_records_are_the_register",
    );
}

#[test]
fn every_runtime_name_is_unabbreviated_and_has_a_role() {
    let abbreviations: BTreeSet<String> = repository_file("parity/naming/abbreviations.json")
        ["abbreviations"]
        .as_array()
        .expect("abbreviations")
        .iter()
        .filter_map(|entry| entry["abbreviation"].as_str().map(str::to_owned))
        .collect();
    assert!(abbreviations.contains("char"));
    let mut roles = BTreeSet::new();
    for record in concept_records() {
        for word in record.phrase.split(' ') {
            assert!(
                !abbreviations.contains(word),
                "{} abbreviates {word}",
                record.id
            );
        }
        roles.insert(format!("{:?}", record.role));
    }
    assert_eq!(
        roles,
        BTreeSet::from(["Concept".to_owned(), "Operation".to_owned()])
    );
    let role_of = |id: &str| concept_record(id).expect("a record").role;
    assert_eq!(role_of("operation.parse"), ConceptRole::Operation);
    assert_eq!(role_of("grammar.character-class"), ConceptRole::Concept);
    observe(
        "I195-NAMING-CONVENTION",
        &[
            "conceptsAreNounPhrases",
            "operationsAreVerbPhrases",
            "noAbbreviations",
        ],
        "every_runtime_name_is_unabbreviated_and_has_a_role",
    );
}

#[test]
fn former_names_stay_aliases_that_decode_to_the_canonical_concept() {
    for (former, current) in FORMER_CONCEPT_IDS {
        assert_eq!(current_concept_id(former), *current);
        assert_eq!(
            concept_record(former).map(|record| record.id.as_str()),
            Some(*current)
        );
    }
    assert_eq!(
        current_concept_id("grammar.character-class"),
        "grammar.character-class"
    );

    let mut network = LinkNetwork::new();
    let report = network.seed_concept_records();
    assert_eq!(report.concepts(), concept_records().len());
    assert!(report.links() > report.concepts());
    assert_eq!(
        network.seed_concept_records().links(),
        0,
        "assigning the records again adds nothing"
    );
    for record in concept_records() {
        let concept = network.find_term(&record.id).expect("the record's concept");
        let metadata = network.link(concept).expect("the concept link").metadata();
        assert_eq!(metadata.link_type(), Some(LinkType::Concept));
        assert_eq!(metadata.definition(), Some(record.definition.as_str()));
        assert_eq!(
            network.reconstruct_concept(&record.id, "en"),
            Some(record.phrase.as_str())
        );
        for alias in &record.source_aliases {
            assert!(
                has_alias(&network, &record.id, &alias.source, &alias.name),
                "{} keeps {} {}",
                record.id,
                alias.source,
                alias.name
            );
        }
        for former in &record.former_names {
            assert!(
                has_alias(&network, &record.id, FORMER_CONCEPT_ID_VOCABULARY, former),
                "{} keeps {former}",
                record.id
            );
            assert_eq!(concept_record(former), Some(record));
        }
    }
    observe(
        "I195-NAMING-CONVENTION",
        &["originalNamesKeptAsAliases"],
        "former_names_stay_aliases_that_decode_to_the_canonical_concept",
    );
    observe(
        "I195-NAMING-CONCEPT-RECORDS",
        &["stableIdentity", "sourceAliasesRecorded"],
        "former_names_stay_aliases_that_decode_to_the_canonical_concept",
    );
}

#[test]
fn import_assigns_records_and_source_round_trips_are_unaffected() {
    let mut older = LinkNetwork::new();
    let char_class = older.intern_concept("grammar.char-class", Some("An older definition."));
    older.insert_concept_expression("grammar.char-class", "ru", "класс символов");
    older.insert_concept_alias(char_class, "wikidata", "Q5089612");
    older.intern_concept(
        "project.custom-concept",
        Some("A concept without a record."),
    );

    let mut merged = LinkNetwork::new();
    let report = merged.import_concept_ontology(&older);
    assert_eq!(
        (
            report.concepts(),
            report.assigned(),
            report.renamed(),
            report.alias_links(),
            report.syntax_mappings()
        ),
        (2, 1, 1, 1, 1)
    );
    assert_eq!(
        merged.find_term("grammar.char-class"),
        None,
        "the former identity is not a concept"
    );
    let record = concept_record("grammar.character-class").expect("the record");
    let concept = merged.find_term(&record.id).expect("the renamed concept");
    assert_eq!(
        merged
            .link(concept)
            .expect("concept link")
            .metadata()
            .definition(),
        Some(record.definition.as_str())
    );
    assert_eq!(
        merged.reconstruct_concept(&record.id, "ru"),
        Some("класс символов")
    );
    assert!(has_alias(&merged, &record.id, "wikidata", "Q5089612"));
    assert!(has_alias(
        &merged,
        &record.id,
        FORMER_CONCEPT_ID_VOCABULARY,
        "grammar.char-class"
    ));
    let custom = merged
        .find_term("project.custom-concept")
        .expect("the custom concept");
    assert_eq!(
        merged
            .link(custom)
            .expect("custom link")
            .metadata()
            .definition(),
        Some("A concept without a record.")
    );
    let size = merged.len();
    merged.import_concept_ontology(&older);
    assert_eq!(
        merged.len(),
        size,
        "merging the same ontology again is idempotent"
    );
    let text = merged.to_lino();
    let report = LinkNetwork::new()
        .import_concept_ontology_lino(&text)
        .expect("the merged ontology reads back");
    assert!(report.assigned() > 0);

    // Assigning the records to a parsed network leaves its source reconstruction intact.
    let source = "fn twice(value: i32) -> i32 {\n    value * 2\n}\n";
    let mut parsed = LinkNetwork::parse(source, "Rust", ParseConfiguration::default());
    parsed.seed_concept_records();
    assert_eq!(parsed.reconstruct_text(), source);
    // A grammar written with the source notations the records alias still round-trips.
    let ebnf = "number = digit , { digit } ;\ndigit = \"0\" | \"1\" ;\n";
    let repetitions: Vec<&str> = concept_records_for_source_name("ebnf", "{ }")
        .iter()
        .map(|record| record.id.as_str())
        .collect();
    assert_eq!(
        repetitions,
        [
            "grammar.counted-repetition",
            "grammar.zero-or-more-repetition"
        ]
    );
    let grammar = import_ebnf(ebnf).expect("the grammar imports");
    assert_eq!(emit_ebnf(&grammar).expect("the grammar emits").0, ebnf);
    let surface = grammar_to_lino(&grammar);
    assert_eq!(
        grammar_to_lino(&grammar_from_lino(&surface).expect("the surface reads back")),
        surface
    );
    observe(
        "I195-NAMING-CONCEPT-RECORDS",
        &[
            "stableIdentity",
            "definitionRecorded",
            "sourceAliasesRecorded",
            "roundTripsUnaffected",
        ],
        "import_assigns_records_and_source_round_trips_are_unaffected",
    );
}
