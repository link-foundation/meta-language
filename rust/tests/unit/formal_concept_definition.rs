use meta_language::{
    FormalConceptArchive, FormalConceptDefinitionInput, FormalConceptDefinitionOptions,
    encode_formal_concept_definition, prove_formal_concept_equivalence,
    read_formal_concept_definition, verify_formal_concept_equivalence,
};
use serde_json::Value;

use super::issue_195_observations::{Observation, record};

fn observe(requirement_id: &str, assertions: &[&str], test_name: &str) {
    record(&Observation {
        requirement_id,
        suffix: "behavior",
        fixture_id: "formal-concept-definitions",
        fixture_file: "parity/fixtures/formal-concept-definitions.json",
        assertions,
        test_name,
    });
}

fn contains_cycle(prepared: &Value) -> bool {
    fn visit(node: usize, prepared: &Value, states: &mut [u8]) -> bool {
        if states[node] == 1 {
            return true;
        }
        if states[node] == 2 {
            return false;
        }
        states[node] = 1;
        let first = prepared["offsets"][node].as_u64().unwrap() as usize;
        let end = prepared["offsets"][node + 1].as_u64().unwrap() as usize;
        for edge in first..end {
            let target = prepared["edges"][edge].as_u64().unwrap() as usize;
            if visit(target, prepared, states) {
                return true;
            }
        }
        states[node] = 2;
        false
    }
    let mut states = vec![0; prepared["shapes"].as_array().unwrap().len()];
    visit(0, prepared, &mut states)
}

fn fixture() -> Value {
    serde_json::from_str(include_str!(
        "../../../parity/fixtures/formal-concept-definitions.json"
    ))
    .unwrap()
}

#[test]
fn formal_archives_prepare_the_same_shared_cyclic_and_labeled_graphs() {
    let fixture = fixture();
    for definition in fixture["definitions"].as_array().unwrap() {
        let archive = FormalConceptArchive::from_json(&definition["archive"]).unwrap();
        let root = archive.network.link(archive.root).unwrap();
        assert_eq!(root.metadata().term(), Some("formal concept definition"));
        assert_eq!(root.references().len(), 2);
        let basis = archive.network.link(root.references()[1]).unwrap();
        assert_eq!(basis.metadata().term(), Some("formal primitive model"));
        assert_eq!(
            basis.metadata().definition(),
            definition["prepared"]["model"].as_str()
        );
        assert_eq!(basis.references().len(), 1);
        assert!(archive.network.link(basis.references()[0]).is_some());
        assert_eq!(
            read_formal_concept_definition(&definition["archive"]).unwrap(),
            definition["prepared"],
            "{}",
            definition["name"]
        );
    }
    for model in ["source text value", "source trivia interpretation"] {
        let archive = encode_formal_concept_definition(
            FormalConceptDefinitionInput::Declaration(None),
            model,
        )
        .unwrap();
        let expected = fixture["definitions"]
            .as_array()
            .unwrap()
            .iter()
            .find(|entry| entry["name"] == model)
            .unwrap();
        assert_eq!(
            read_formal_concept_definition(&archive.to_json()).unwrap(),
            expected["prepared"]
        );
    }
    let provenance = serde_json::json!({"source": "first imported text"});
    let archive = encode_formal_concept_definition(
        FormalConceptDefinitionInput::Declaration(None),
        FormalConceptDefinitionOptions {
            model: "source text value".to_owned(),
            provenance: Some(provenance),
        },
    )
    .unwrap();
    assert_eq!(
        read_formal_concept_definition(&archive.to_json()).unwrap(),
        fixture["definitions"][0]["prepared"]
    );
    assert!(
        archive
            .network
            .to_lino()
            .contains("first%20imported%20text")
    );
    let recursive = &fixture["definitions"][4]["prepared"];
    assert!(contains_cycle(recursive));
    let mut incoming = vec![0; recursive["shapes"].as_array().unwrap().len()];
    for edge in recursive["edges"].as_array().unwrap() {
        incoming[edge.as_u64().unwrap() as usize] += 1;
    }
    assert!(incoming.iter().any(|count| *count > 1));
    assert!(
        recursive["shapes"]
            .as_array()
            .unwrap()
            .iter()
            .any(|shape| shape.as_str().unwrap().contains("source language"))
    );
    assert!(
        recursive["shapes"]
            .as_array()
            .unwrap()
            .iter()
            .any(|shape| shape.as_str().unwrap().contains("reference semantics"))
    );
    observe(
        "I195-FORMAL-CONCEPT-DEFINITIONS",
        &[
            "definitionsAreInspectableLinks",
            "foundationalPrimitivesExplicit",
        ],
        "formal_archives_prepare_the_same_shared_cyclic_and_labeled_graphs",
    );
    observe(
        "I195-FORMAL-CONCEPT-REFERENCES",
        &[
            "recursiveConceptReferencesPreserved",
            "sharedAndCyclicDefinitionsReloaded",
            "definitionAssumptionsPreserved",
        ],
        "formal_archives_prepare_the_same_shared_cyclic_and_labeled_graphs",
    );
}

#[test]
fn formal_certificates_recheck_every_pair_and_copied_premise() {
    let fixture = fixture();
    let archives = fixture["definitions"]
        .as_array()
        .unwrap()
        .iter()
        .map(|entry| FormalConceptArchive::from_json(&entry["archive"]).unwrap())
        .collect::<Vec<_>>();
    for proof in fixture["proofs"].as_array().unwrap() {
        let first = &archives[proof["first"].as_u64().unwrap() as usize];
        let second = &archives[proof["second"].as_u64().unwrap() as usize];
        let certificate = FormalConceptArchive::from_json(&proof["certificate"]).unwrap();
        assert!(verify_formal_concept_equivalence(
            first,
            second,
            &certificate
        ));
        let own = prove_formal_concept_equivalence(first, second).unwrap();
        assert!(verify_formal_concept_equivalence(first, second, &own));
        let restored = FormalConceptArchive::from_json(&own.to_json()).unwrap();
        assert!(verify_formal_concept_equivalence(first, second, &restored));
    }
    for proof in fixture["invalidCertificates"].as_array().unwrap() {
        let first = &archives[proof["first"].as_u64().unwrap() as usize];
        let second = &archives[proof["second"].as_u64().unwrap() as usize];
        let certificate = FormalConceptArchive::from_json(&proof["certificate"]).unwrap();
        assert!(
            !verify_formal_concept_equivalence(first, second, &certificate),
            "{}",
            proof["name"]
        );
    }
    for pair in fixture["negativeEquivalences"].as_array().unwrap() {
        assert!(
            prove_formal_concept_equivalence(
                &archives[pair["first"].as_u64().unwrap() as usize],
                &archives[pair["second"].as_u64().unwrap() as usize],
            )
            .is_none(),
            "{}",
            pair["name"],
        );
    }
    observe(
        "I195-FORMAL-CONCEPT-MEANING-COMPARISON",
        &[
            "formalDefinitionsCompared",
            "unprovedMeaningEquivalenceRejected",
        ],
        "formal_certificates_recheck_every_pair_and_copied_premise",
    );
}

#[test]
fn unknown_counterfeit_cyclic_dangling_and_duplicate_bases_are_refused() {
    let fixture = fixture();
    let first = FormalConceptArchive::from_json(&fixture["definitions"][0]["archive"]).unwrap();
    for entry in fixture["rejections"].as_array().unwrap() {
        assert!(
            read_formal_concept_definition(&entry["archive"]).is_err(),
            "{}",
            entry["name"]
        );
        let archive = FormalConceptArchive::from_json(&entry["archive"]).unwrap();
        assert!(prove_formal_concept_equivalence(&first, &archive).is_none());
    }
    assert!(
        encode_formal_concept_definition(
            FormalConceptDefinitionInput::Declaration(None),
            "proof by descriptive prose"
        )
        .is_err()
    );
    observe(
        "I195-FORMAL-CONCEPT-MEANING-COMPARISON",
        &["semanticObligationsChecked"],
        "unknown_counterfeit_cyclic_dangling_and_duplicate_bases_are_refused",
    );
}
