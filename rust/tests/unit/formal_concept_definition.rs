use meta_language::{
    FormalConceptArchive, FormalConceptDefinitionInput, FormalConceptDefinitionOptions,
    encode_formal_concept_definition, prove_formal_concept_equivalence,
    read_formal_concept_definition, verify_formal_concept_equivalence,
};
use serde_json::Value;

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
    assert!(prove_formal_concept_equivalence(&archives[0], &archives[1]).is_none());
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
}
