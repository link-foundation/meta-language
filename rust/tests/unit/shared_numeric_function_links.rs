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
        let checked = serde_json::to_value(&program).unwrap();
        let declarations = checked["declarations"].as_array().unwrap();
        let wrapper = declarations
            .iter()
            .find(|declaration| declaration["fullName"] == "selected_function_definition")
            .unwrap();
        assert!(wrapper.get("sourceSignature").is_none());
        if case["language"] == "Rust" {
            let original = declarations
                .iter()
                .find(|declaration| declaration["fullName"] == case["declaration"])
                .unwrap();
            assert_eq!(
                original["sourceSignature"]["model"],
                "Rust source function signature 1"
            );
        }
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
        let one = &fixture["cases"][usize::try_from(pair[0].as_u64().unwrap()).unwrap()];
        let two = &fixture["cases"][usize::try_from(pair[1].as_u64().unwrap()).unwrap()];
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
                &archive(&fixture["cases"][usize::try_from(pair[0].as_u64().unwrap()).unwrap()]),
                &archive(&fixture["cases"][usize::try_from(pair[1].as_u64().unwrap()).unwrap()])
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
    assert_ne!(
        space["proofs"].as_array().unwrap().as_slice(),
        [] as [Value; 0]
    );
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
        .chain(fixture["literalRefusals"].as_array().unwrap())
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

#[test]
fn ieee_value_views_keep_original_text_ties_and_zero_signs() {
    let data = fixture();
    let values: Vec<_> = data["literalCases"]
        .as_array()
        .unwrap()
        .iter()
        .map(archive)
        .collect();
    for (index, value) in values.iter().enumerate() {
        let before = value.network.to_lino();
        assert_eq!(
            read_formal_concept_definition(&value.to_json()).unwrap(),
            data["literalCases"][index]["expected"]
        );
        assert_eq!(value.network.to_lino(), before);
        assert!(value.network.links().any(
            |link| link.metadata().definition() == data["literalCases"][index]["text"].as_str()
        ));
        let (network, root) = input(&data["literalCases"][index]);
        let recreated = encode_shared_numeric_function_definition(
            &network,
            root,
            FormalFunctionSelector::Name("constant"),
            None,
        )
        .unwrap();
        assert_eq!(
            read_formal_concept_definition(&recreated.to_json()).unwrap(),
            data["literalCases"][index]["expected"]
        );
    }
    let proof = prove_formal_concept_equivalence(&values[0], &values[1]).unwrap();
    let reloaded = FormalConceptArchive {
        network: LinkNetwork::from_lino(&proof.network.to_lino()).unwrap(),
        root: proof.root,
    };
    assert!(verify_formal_concept_equivalence(
        &values[0], &values[1], &reloaded
    ));
    assert!(!verify_formal_concept_equivalence(
        &values[0], &values[2], &reloaded
    ));
    assert!(prove_formal_concept_equivalence(&values[0], &values[2]).is_none());
    assert!(prove_formal_concept_equivalence(&values[3], &values[4]).is_none());
}

#[test]
fn boolean_callable_values_preserve_actual_owners_and_recursive_source_free_proofs() {
    use meta_language::{
        FormalConceptDefinitionInput, FormalConceptDefinitionOptions,
        encode_formal_concept_definition, encode_formal_function_definition,
    };
    let data = read_structured_data_lino(include_str!(
        "../../../parity/fixtures/shared-boolean-function-definitions.lino"
    ))
    .unwrap();
    for case in data["cases"].as_array().unwrap() {
        let network = LinkNetwork::from_lino(case["links"].as_str().unwrap()).unwrap();
        let root = LinkId::from_u64(case["programRoot"].as_u64().unwrap());
        let selected = encode_formal_function_definition(
            &network,
            root,
            FormalFunctionSelector::Name(case["declaration"].as_str().unwrap()),
            None,
        )
        .unwrap();
        let typed_root = selected.network.link(selected.root).unwrap().references()[0];
        let definition = encode_formal_concept_definition(
            FormalConceptDefinitionInput::LinkedProgram {
                network: &selected.network,
                root: typed_root,
            },
            FormalConceptDefinitionOptions::from("closed boolean function value"),
        )
        .unwrap();
        let before = definition.network.to_lino();
        assert_eq!(
            read_formal_concept_definition(&definition.to_json()).unwrap(),
            case["expected"]
        );
        assert_eq!(definition.network.to_lino(), before);
        let restored = FormalConceptArchive {
            network: LinkNetwork::from_lino(&before).unwrap(),
            root: definition.root,
        };
        assert_eq!(
            read_formal_concept_definition(&restored.to_json()).unwrap(),
            case["expected"]
        );
        let checked = decode_typed_program_links(&definition.network, typed_root).unwrap();
        assert_eq!(
            checked.source_language.as_str(),
            case["language"].as_str().unwrap()
        );
    }
    for pair in data["equivalent"].as_array().unwrap() {
        let first = &data["cases"][usize::try_from(pair[0].as_u64().unwrap()).unwrap()];
        let second = &data["cases"][usize::try_from(pair[1].as_u64().unwrap()).unwrap()];
        let one = archive(first);
        let two = archive(second);
        let proof = prove_formal_concept_equivalence(&one, &two).unwrap();
        let restored = FormalConceptArchive {
            network: LinkNetwork::from_lino(&proof.network.to_lino()).unwrap(),
            root: proof.root,
        };
        assert!(verify_formal_concept_equivalence(&one, &two, &restored));
        let conservative =
            |case: &Value| archive(&json!({"links":case["links"],"root":case["conservativeRoot"]}));
        assert!(
            prove_formal_concept_equivalence(&conservative(first), &conservative(second)).is_none()
        );
    }
    assert!(
        prove_formal_concept_equivalence(&archive(&data["cases"][8]), &archive(&data["cases"][10]))
            .is_none()
    );
}
