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
    // Keep the old corpus tied to its exact certified historical native view.
    let historical_context = std::process::Command::new("node")
        .current_dir(std::path::Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap())
        .args(["--input-type=module", "-e", concat!(
            "import{readFileSync}from'node:fs';",
            "import{restoreCiLintSource}from'./js/scripts/ci-39496927203-lint-source-history.mjs';",
            "const file='rust/src/self_translation/typescript_erasure_rules.rs';",
            "const current=readFileSync(file,'utf8');process.stdout.write(JSON.stringify({current,historical:restoreCiLintSource(current,file)}));"
        )]).output().unwrap();
    assert!(
        historical_context.status.success(),
        "{}",
        String::from_utf8_lossy(&historical_context.stderr)
    );
    let historical_context: Value = serde_json::from_slice(&historical_context.stdout).unwrap();
    let native_case = data["cases"]
        .as_array()
        .unwrap()
        .iter()
        .find(|case| {
            case["provenance"]["file"] == "rust/src/self_translation/typescript_erasure_rules.rs"
        })
        .unwrap();
    assert_eq!(
        historical_context["historical"],
        native_case["provenance"]["completeSource"]
    );
    assert_eq!(
        historical_context["current"],
        include_str!("../../src/self_translation/typescript_erasure_rules.rs")
    );
    assert_ne!(
        historical_context["historical"],
        historical_context["current"]
    );
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

#[test]
fn current_boolean_declarations_keep_actual_const_owners_truths_and_both_archives() {
    use meta_language::{
        BinaryLinksOptions, ParseConfiguration, decode_link_network_binary,
        emit_typed_program_links, encode_formal_function_declaration_value,
        encode_link_network_binary,
    };
    let data = read_structured_data_lino(include_str!(
        "../../../parity/fixtures/shared-boolean-function-definitions-current.lino"
    ))
    .unwrap();
    assert_eq!(data["model"], "closed boolean function declaration value");
    assert_eq!(data["cases"].as_array().unwrap().len(), 2);
    for case in data["cases"].as_array().unwrap() {
        let source = if case["language"] == "Rust" {
            include_str!("../../src/self_translation/typescript_erasure_rules.rs")
        } else {
            include_str!("../../../js/src/typescript-erasure-rules.js")
        };
        assert_eq!(
            source,
            case["provenance"]["completeSource"].as_str().unwrap()
        );
        let concrete = LinkNetwork::parse(
            source,
            case["language"].as_str().unwrap(),
            ParseConfiguration::default(),
        );
        assert!(concrete.verify_full_match(None).is_clean());
        assert_eq!(concrete.reconstruct_text(), source);
        for restored in [
            LinkNetwork::from_lino(&concrete.to_lino()).unwrap(),
            decode_link_network_binary(
                &encode_link_network_binary(&concrete, BinaryLinksOptions::default()).unwrap(),
                BinaryLinksOptions::default(),
            )
            .unwrap(),
        ] {
            assert_eq!(restored.reconstruct_text(), source);
        }
        let network = LinkNetwork::from_lino(case["links"].as_str().unwrap()).unwrap();
        let root = LinkId::from_u64(case["programRoot"].as_u64().unwrap());
        let before = network.to_lino();
        let selected = encode_formal_function_declaration_value(
            &network,
            root,
            FormalFunctionSelector::Name(case["declaration"].as_str().unwrap()),
            None,
        )
        .unwrap();
        assert_eq!(network.to_lino(), before);
        assert_eq!(
            selected.network.link(selected.root).unwrap().references()[0],
            root
        );
        for link in network.links() {
            let retained = selected.network.link(link.id()).unwrap();
            assert_eq!(retained.references(), link.references());
            assert_eq!(retained.metadata(), link.metadata());
        }
        let identities = selected
            .network
            .links()
            .filter_map(|link| {
                let [parent, label, child] = link.references() else {
                    return None;
                };
                (*parent == selected.root
                    && selected.network.link(*label).unwrap().metadata().term()
                        == Some("selected function declaration"))
                .then_some(*child)
            })
            .collect::<Vec<_>>();
        assert_eq!(
            identities,
            vec![LinkId::from_u64(case["declarationId"].as_u64().unwrap())]
        );
        assert_eq!(
            read_formal_concept_definition(&selected.to_json()).unwrap(),
            case["expected"]
        );
        for graph in [
            LinkNetwork::from_lino(&selected.network.to_lino()).unwrap(),
            decode_link_network_binary(
                &encode_link_network_binary(&selected.network, BinaryLinksOptions::default())
                    .unwrap(),
                BinaryLinksOptions::default(),
            )
            .unwrap(),
        ] {
            let actual = FormalConceptArchive {
                network: graph,
                root: selected.root,
            };
            assert_eq!(
                read_formal_concept_definition(&actual.to_json()).unwrap(),
                case["expected"]
            );
            let checked = decode_typed_program_links(&actual.network, root).unwrap();
            assert!(
                !checked
                    .declarations
                    .iter()
                    .any(|entry| entry.full_name() == "selected_function_definition")
            );
            let checked = serde_json::to_value(checked).unwrap();
            let declaration = checked["declarations"]
                .as_array()
                .unwrap()
                .iter()
                .find(|entry| entry["fullName"] == case["declaration"])
                .unwrap();
            if case["language"] == "Rust" {
                assert_eq!(
                    declaration["sourceConstFunction"],
                    case["sourceConstFunction"]
                );
            } else {
                assert!(declaration.get("sourceConstFunction").is_none());
            }
            let javascript = emit_typed_program_links(&actual.network, root, "JavaScript")
                .unwrap()
                .code;
            assert_eq!(
                javascript,
                case["emissions"]["JavaScript"].as_str().unwrap()
            );
            assert_eq!(
                emit_typed_program_links(&actual.network, root, "Rust")
                    .unwrap()
                    .code,
                case["emissions"]["Rust"].as_str().unwrap()
            );
            let mut script = javascript;
            script.push_str(&format!(
                "\nconsole.log(JSON.stringify({}.map(args=>{}(...args))));",
                case["inputs"],
                case["declaration"].as_str().unwrap()
            ));
            let output = std::process::Command::new("node")
                .args(["--input-type=module", "-e", &script])
                .output()
                .unwrap();
            assert!(
                output.status.success(),
                "{}",
                String::from_utf8_lossy(&output.stderr)
            );
            let results: Value = serde_json::from_slice(&output.stdout).unwrap();
            assert_eq!(results, case["expectedValues"]);
            assert_eq!(
                results,
                json!([false, false, false, true, true, true, true, true])
            );
        }
    }
    let first = archive(&data["cases"][0]);
    let second = archive(&data["cases"][1]);
    let proof = prove_formal_concept_equivalence(&first, &second).unwrap();
    for graph in [
        LinkNetwork::from_lino(&proof.network.to_lino()).unwrap(),
        decode_link_network_binary(
            &encode_link_network_binary(&proof.network, BinaryLinksOptions::default()).unwrap(),
            BinaryLinksOptions::default(),
        )
        .unwrap(),
    ] {
        assert!(verify_formal_concept_equivalence(
            &first,
            &second,
            &FormalConceptArchive {
                network: graph,
                root: proof.root
            }
        ));
    }
}
