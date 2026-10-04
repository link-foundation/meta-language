//! Concept distinctions (requirement I195-GRAMMAR-CONCEPT-DISTINCTIONS): two
//! languages' spellings share a concept only under a one-to-one correspondence
//! of meaning that the records justify, required distinctions (ordered and
//! unordered choice, lexical and syntactic precedence, binding and assignment,
//! the foundation models) never merge, and lookalike spellings with different
//! meanings stay distinct. The JavaScript twin is
//! js/tests/concept-distinctions.test.js.

use std::path::PathBuf;

use meta_language::{
    ConceptRecord, CorrespondenceRelation, FoundationRegister, Grammar, GrammarImportError,
    REQUIRED_CONCEPT_DISTINCTIONS, REQUIRED_FOUNDATION_DISTINCTIONS, SourceAlias,
    check_concept_distinctions, concept_correspondence, concept_correspondence_in, concept_records,
    foundation_register, grammar_expr_concept_id, grammar_precedence_concepts, import_abnf,
    import_bnf, import_pest, import_tree_sitter_json, source_meanings_in,
};
use serde_json::Value;

use super::issue_195_observations::{Observation, record};

const FIXTURE: &str = "parity/fixtures/concept-distinctions.json";

fn observe(assertions: &[&str], test_name: &str) {
    record(&Observation {
        requirement_id: "I195-GRAMMAR-CONCEPT-DISTINCTIONS",
        suffix: "behavior",
        fixture_id: "planned:repository-directive:i195-grammar-concept-distinctions",
        fixture_file: FIXTURE,
        assertions,
        test_name,
    });
}

fn fixture() -> Value {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join(FIXTURE);
    let text = std::fs::read_to_string(path).expect("the fixture reads");
    serde_json::from_str(&text).expect("the fixture is JSON")
}

fn alias(value: &Value) -> SourceAlias {
    SourceAlias {
        source: value["source"].as_str().expect("source").to_owned(),
        name: value["name"].as_str().expect("name").to_owned(),
    }
}

fn records() -> Vec<ConceptRecord> {
    concept_records().to_vec()
}

fn register() -> FoundationRegister {
    foundation_register().clone()
}

fn kinds(records: &[ConceptRecord], register: &FoundationRegister) -> Vec<&'static str> {
    check_concept_distinctions(records, register)
        .into_iter()
        .map(|problem| problem.kind)
        .collect()
}

fn record_mut<'a>(records: &'a mut [ConceptRecord], id: &str) -> &'a mut ConceptRecord {
    records
        .iter_mut()
        .find(|entry| entry.id == id)
        .expect("the concept is recorded")
}

fn import(format: &str, source: &str) -> Result<Grammar, GrammarImportError> {
    match format {
        "pest" => import_pest(source),
        "bnf" => import_bnf(source),
        "abnf" => import_abnf(source),
        other => panic!("no importer for {other}"),
    }
}

#[test]
fn every_fixture_correspondence_relates_the_spellings_as_recorded() {
    assert_eq!(
        check_concept_distinctions(concept_records(), foundation_register()),
        []
    );
    let fixture = fixture();
    for expected in fixture["correspondences"].as_array().expect("cases") {
        let (first, second) = (alias(&expected["first"]), alias(&expected["second"]));
        let actual = concept_correspondence(&first, &second, expected["within"].as_str());
        let label = format!(
            "{} {} / {} {}",
            first.source, first.name, second.source, second.name
        );
        assert_eq!(
            actual.relation.as_str(),
            expected["relation"].as_str().expect("relation"),
            "{label}"
        );
        assert_eq!(
            actual.shared.as_deref(),
            expected["shared"].as_str(),
            "{label}"
        );
        assert_eq!(
            actual.justification.is_some(),
            expected["justified"].as_bool().expect("justified"),
            "{label}"
        );
        assert_eq!(
            actual.correspondence.as_deref(),
            expected["correspondence"].as_str(),
            "{label}"
        );
        if let Some(meanings) = expected["meanings"].as_array() {
            let actual_meanings: Vec<&str> = actual
                .first
                .iter()
                .chain(&actual.second)
                .map(String::as_str)
                .collect();
            let expected_meanings: Vec<&str> = meanings
                .iter()
                .map(|meaning| meaning.as_str().expect("meaning"))
                .collect();
            assert_eq!(actual_meanings, expected_meanings, "{label}");
        }
        if actual.relation == CorrespondenceRelation::Shared {
            let shared = actual.shared.clone().expect("a shared concept");
            assert_eq!(
                actual.first.as_slice(),
                std::slice::from_ref(&shared),
                "{label}"
            );
            assert_eq!(
                actual.second.as_slice(),
                std::slice::from_ref(&shared),
                "{label}"
            );
            let justification = actual.justification.expect("a justification");
            assert!(justification.ends_with('.'), "{label}");
            assert!(justification.split_whitespace().count() >= 3, "{label}");
        }
        // An ambiguous spelling justifies no correspondence: it is never reported as shared.
        if actual.relation == CorrespondenceRelation::Ambiguous {
            assert!(actual.first.len() > 1 || actual.second.len() > 1, "{label}");
            assert_eq!(actual.shared, None, "{label}");
        }
    }
    let python_equals = SourceAlias {
        source: "Python".to_owned(),
        name: "=".to_owned(),
    };
    assert_eq!(
        source_meanings_in(
            concept_records(),
            foundation_register(),
            &python_equals,
            None
        ),
        ["binding", "assignment"]
    );
    observe(
        &["sharedOnlyWithJustifiedCorrespondence"],
        "every fixture correspondence relates the spellings as recorded, and shared spellings carry a justification",
    );
}

#[test]
fn check_rejects_records_that_merge_or_fail_to_justify_a_required_distinction() {
    let pairs: Vec<String> = REQUIRED_CONCEPT_DISTINCTIONS
        .iter()
        .map(|entry| entry.concepts.join(" / "))
        .collect();
    assert_eq!(
        pairs,
        [
            "grammar.ordered-choice / grammar.unordered-choice",
            "grammar.lexical-precedence / grammar.syntactic-precedence",
            "binding / assignment",
            "grammar.list / grammar.linked-list",
            "grammar.string / grammar.symbol",
            "grammar.member / grammar.ini.setting",
            "grammar.object / grammar.racket.hash-table",
            "grammar.value / grammar.datum",
            "grammar.identifier / grammar.identifier-name",
            "grammar.document / grammar.program",
            "grammar.boolean-value / grammar.true-value",
            "grammar.symbol / grammar.keyword",
            "grammar.comment / grammar.block-comment",
        ]
    );
    assert_eq!(REQUIRED_FOUNDATION_DISTINCTIONS.len(), 6);
    let foundations = foundation_register();

    let mut missing = records();
    missing.retain(|entry| entry.id != "grammar.ordered-choice");
    assert_eq!(kinds(&missing, foundations), ["unknown-concept"]);

    let mut merged = records();
    let syntactic = record_mut(&mut merged, "grammar.syntactic-precedence")
        .definition
        .clone();
    record_mut(&mut merged, "grammar.lexical-precedence").definition = syntactic;
    assert_eq!(kinds(&merged, foundations), ["indistinct-concepts"]);

    let mut conflated = records();
    record_mut(&mut conflated, "assignment")
        .former_names
        .push("binding".to_owned());
    assert_eq!(kinds(&conflated, foundations), ["conflated-distinction"]);

    let mut represented = records();
    record_mut(&mut represented, "grammar.unordered-choice").represents =
        Some("grammar.ordered-choice".to_owned());
    assert_eq!(kinds(&represented, foundations), ["conflated-distinction"]);

    let mut unrecorded = register();
    unrecorded.distinctions.retain(|entry| {
        !entry
            .models
            .iter()
            .any(|model| model == "universe-model.cumulative-type-hierarchy")
    });
    assert_eq!(
        kinds(concept_records(), &unrecorded),
        ["unrecorded-distinction"]
    );

    let mut same_properties = register();
    let unbounded = same_properties
        .models
        .iter()
        .find(|model| model.id == "integer-model.unbounded-integer")
        .expect("unbounded integer")
        .properties
        .clone();
    same_properties
        .models
        .iter_mut()
        .find(|model| model.id == "integer-model.natural-number")
        .expect("natural number")
        .properties = unbounded;
    assert_eq!(
        kinds(concept_records(), &same_properties),
        ["indistinct-concepts"]
    );

    let mut unknown_model = register();
    unknown_model
        .models
        .retain(|model| model.id != "logic-model.classical-propositions");
    assert!(kinds(concept_records(), &unknown_model).contains(&"unknown-model"));

    let mut unjustified = records();
    record_mut(&mut unjustified, "grammar.sequence").definition = "sequence".to_owned();
    assert_eq!(kinds(&unjustified, foundations), ["unjustified-sharing"]);

    let mut unconstrained = records();
    record_mut(&mut unconstrained, "grammar.unordered-choice")
        .constraints
        .clear();
    assert_eq!(kinds(&unconstrained, foundations), ["unjustified-sharing"]);

    let mut unjustified_model = register();
    unjustified_model
        .models
        .iter_mut()
        .find(|model| model.id == "integer-model.fixed-width-integer")
        .expect("fixed-width integer")
        .definition = "fixed width".to_owned();
    assert_eq!(
        kinds(concept_records(), &unjustified_model),
        ["unjustified-sharing"]
    );
    observe(
        &["requiredDistinctionsPreserved"],
        "checkConceptDistinctions rejects records that merge or fail to justify a required distinction",
    );
}

#[test]
fn lookalike_spellings_import_as_distinct_concepts_and_precedence_splits() {
    let fixture = fixture();
    for case in fixture["lookalikeGrammars"].as_array().expect("grammars") {
        let format = case["format"].as_str().expect("format");
        let grammar =
            import(format, case["source"].as_str().expect("source")).expect("the grammar imports");
        let rule = grammar.rule("a").expect("rule a");
        assert_eq!(
            grammar_expr_concept_id(rule.expr()),
            case["concept"].as_str().expect("concept"),
            "{format}"
        );
    }
    let grammar = import_tree_sitter_json(&fixture["precedence"]["grammar"].to_string())
        .expect("the tree-sitter grammar imports");
    let uses: Vec<Value> = grammar_precedence_concepts(&grammar)
        .into_iter()
        .map(|entry| {
            serde_json::json!({ "rule": entry.rule, "label": entry.label, "concept": entry.concept })
        })
        .collect();
    assert_eq!(
        Value::Array(uses),
        fixture["precedence"]["uses"],
        "precedence uses"
    );

    // A spelling that gains a second meaning makes every correspondence through it ambiguous, never shared.
    let pest_bar = SourceAlias {
        source: "pest".to_owned(),
        name: "|".to_owned(),
    };
    let bnf_bar = SourceAlias {
        source: "bnf".to_owned(),
        name: "|".to_owned(),
    };
    let mut widened = records();
    record_mut(&mut widened, "grammar.unordered-choice")
        .source_aliases
        .push(pest_bar.clone());
    assert_eq!(
        concept_correspondence_in(&widened, foundation_register(), &pest_bar, &bnf_bar, None)
            .relation,
        CorrespondenceRelation::Ambiguous
    );
    assert_eq!(
        concept_correspondence(&pest_bar, &bnf_bar, None).relation,
        CorrespondenceRelation::Distinct
    );

    for lookalike in fixture["correspondences"].as_array().expect("cases") {
        if lookalike["lookalike"].as_bool() != Some(true) {
            continue;
        }
        let relation = concept_correspondence(
            &alias(&lookalike["first"]),
            &alias(&lookalike["second"]),
            lookalike["within"].as_str(),
        )
        .relation;
        assert_ne!(relation, CorrespondenceRelation::Shared);
    }
    observe(
        &["lookalikeConceptsKeptDistinct"],
        "lookalike spellings import as distinct concepts and precedence splits into lexical and syntactic",
    );
}
