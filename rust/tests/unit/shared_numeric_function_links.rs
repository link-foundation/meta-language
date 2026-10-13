//! Paired real-source archives under the explicitly guarded numeric value model.
use meta_language::{
    FormalConceptArchive, FormalFunctionSelector, GlobalConceptOptions, GlobalConceptSource,
    LinkId, LinkNetwork, create_global_concept_space, decode_typed_program_links,
    encode_shared_numeric_function_definition, global_concept_space_to_links,
    prove_formal_concept_equivalence, read_formal_concept_definition,
    read_global_concept_space_links, read_structured_data_lino, verify_formal_concept_equivalence,
};
use serde_json::{Value, json};

fn fixture() -> Value {
    read_structured_data_lino(include_str!(
        "../../../parity/fixtures/shared-numeric-function-definitions.lino"
    ))
    .unwrap()
}
fn archive(value: &Value) -> FormalConceptArchive {
    FormalConceptArchive::from_json(value).unwrap()
}
fn input(value: &Value) -> (LinkNetwork, LinkId) {
    (
        LinkNetwork::from_lino(value["programLinks"].as_str().unwrap()).unwrap(),
        LinkId::from_u64(value["programRoot"].as_u64().unwrap()),
    )
}

#[test]
fn native_numeric_profile_matches_the_shared_prepared_definitions_and_preserves_origin_language() {
    let fixture = fixture();
    for case in fixture["cases"].as_array().unwrap() {
        let (network, root) = input(case);
        let definition = encode_shared_numeric_function_definition(
            &network,
            root,
            FormalFunctionSelector::Name(case["declaration"].as_str().unwrap()),
            Some(&case["provenance"]),
        )
        .unwrap();
        assert_eq!(
            read_formal_concept_definition(&definition.to_json()).unwrap(),
            case["expected"]
        );
        let typed_root = definition
            .network
            .link(definition.root)
            .unwrap()
            .references()[0];
        let program = decode_typed_program_links(&definition.network, typed_root).unwrap();
        assert_eq!(
            program.source_language.as_str(),
            case["language"].as_str().unwrap()
        );
        let restored = FormalConceptArchive {
            network: LinkNetwork::from_lino(&definition.network.to_lino()).unwrap(),
            root: definition.root,
        };
        assert_eq!(
            read_formal_concept_definition(&restored.to_json()).unwrap(),
            case["expected"]
        );
    }
}

#[test]
fn native_numeric_certificates_share_cross_language_recursion_without_changing_the_old_model() {
    let fixture = fixture();
    for pair in fixture["equivalent"].as_array().unwrap() {
        let one = &fixture["cases"][pair[0].as_u64().unwrap() as usize];
        let two = &fixture["cases"][pair[1].as_u64().unwrap() as usize];
        let first = archive(one);
        let second = archive(two);
        let proof = prove_formal_concept_equivalence(&first, &second).unwrap();
        let reloaded = FormalConceptArchive {
            network: LinkNetwork::from_lino(&proof.network.to_lino()).unwrap(),
            root: proof.root,
        };
        assert!(verify_formal_concept_equivalence(
            &first, &second, &reloaded
        ));
        let first_old =
            archive(&json!({"links":one["conservativeLinks"],"root":one["conservativeRoot"]}));
        let second_old =
            archive(&json!({"links":two["conservativeLinks"],"root":two["conservativeRoot"]}));
        assert!(prove_formal_concept_equivalence(&first_old, &second_old).is_none());
    }
    for pair in fixture["distinct"].as_array().unwrap() {
        assert!(
            prove_formal_concept_equivalence(
                &archive(&fixture["cases"][pair[0].as_u64().unwrap() as usize]),
                &archive(&fixture["cases"][pair[1].as_u64().unwrap() as usize])
            )
            .is_none()
        );
    }
}

#[test]
fn native_global_registry_reuses_only_checked_shared_numeric_definitions() {
    let fixture = fixture();
    let sources: Vec<_> = fixture["globalSources"]
        .as_array()
        .unwrap()
        .iter()
        .map(|source| GlobalConceptSource::from_json(source).unwrap())
        .collect();
    let options = GlobalConceptOptions {
        records: Some(json!([])),
        formal_definitions: Some(
            json!({"javascript:function":fixture["cases"][0],"rust:function":fixture["cases"][1]}),
        ),
        ..GlobalConceptOptions::default()
    };
    let space = create_global_concept_space(&sources, &options).unwrap();
    assert_eq!(
        space["mappings"]["javascript:function"],
        space["mappings"]["rust:function"]
    );
    assert_eq!(space["concepts"].as_array().unwrap().len(), 1);
    assert!(!space["proofs"].as_array().unwrap().is_empty());
    let links = global_concept_space_to_links(&space).unwrap();
    let reload = read_global_concept_space_links(&links, &options).unwrap();
    assert_eq!(reload, space);
    let mut changed = options;
    let last = fixture["cases"].as_array().unwrap().last().unwrap();
    changed.formal_definitions =
        Some(json!({"javascript:function":fixture["cases"][0],"rust:function":last}));
    let space = create_global_concept_space(&sources, &changed).unwrap();
    assert_ne!(
        space["mappings"]["javascript:function"],
        space["mappings"]["rust:function"]
    );
}

#[test]
fn native_profile_refuses_unmodeled_types_defaults_effects_and_operations() {
    let fixture = fixture();
    for refusal in fixture["refusals"]
        .as_array()
        .unwrap()
        .iter()
        .chain(fixture["originRefusals"].as_array().unwrap())
        .chain(fixture["encodedOriginRefusals"].as_array().unwrap())
    {
        let (network, root) = input(refusal);
        assert!(
            encode_shared_numeric_function_definition(
                &network,
                root,
                FormalFunctionSelector::Name(refusal["declaration"].as_str().unwrap()),
                None
            )
            .is_err(),
            "{}",
            refusal["name"]
        );
    }
}

#[test]
fn native_numeric_model_refuses_counterfeit_basis_and_unsupported_source_premises() {
    let fixture = fixture();
    let original = archive(&fixture["cases"][0]);
    for invalid in fixture["invalidDefinitions"].as_array().unwrap() {
        let definition = archive(invalid);
        assert!(read_formal_concept_definition(&definition.to_json()).is_err());
        assert!(prove_formal_concept_equivalence(&original, &definition).is_none());
    }
}
