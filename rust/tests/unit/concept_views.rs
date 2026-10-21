//! Paired public entry points over the same orphan and grammar concept links.
use meta_language::{BinaryLinksOptions, decode_link_network_binary, encode_link_network_binary};
use meta_language::{
    LinkId, LinkNetwork, create_concept_views, read_concept_ontology, read_concept_taxonomy,
    read_structured_data_lino,
};
use serde_json::Value;

fn fixture() -> Value {
    read_structured_data_lino(include_str!("../../../parity/fixtures/concept-views.lino")).unwrap()
}

#[test]
fn paired_entry_points_retain_orphan_definitions_roots_and_declared_models() {
    let data = fixture();
    for name in ["orphanOnly", "withBase"] {
        let value = &data[name];
        let network = LinkNetwork::from_lino(value["links"].as_str().unwrap()).unwrap();
        let root = LinkId::from_u64(value["root"].as_u64().unwrap());
        let ontology = read_concept_ontology(&network, root, None).unwrap();
        let taxonomy = read_concept_taxonomy(&network, root, None).unwrap();
        assert_eq!(ontology, value["expectedOntology"]);
        assert_eq!(taxonomy, value["expectedTaxonomy"]);
        for concept in ontology["concepts"].as_array().unwrap() {
            assert!(
                network
                    .link(LinkId::from_u64(concept["link"].as_u64().unwrap()))
                    .is_some()
            );
            for definition in concept["definitionRoots"].as_array().unwrap() {
                assert!(
                    network
                        .link(LinkId::from_u64(definition.as_u64().unwrap()))
                        .is_some()
                );
            }
            for provenance in concept["provenanceRoots"].as_array().unwrap() {
                assert!(
                    network
                        .link(LinkId::from_u64(provenance.as_u64().unwrap()))
                        .is_some()
                );
            }
            assert!(
                taxonomy["concepts"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .any(|other| other["identity"] == concept["identity"]
                        && other["link"] == concept["link"])
            );
        }
        assert_eq!(ontology["roots"].as_array().unwrap().len(), 2);
        assert_eq!(taxonomy["roots"].as_array().unwrap().len(), 2);
        assert!(ontology["roots"][0]["typeModel"].is_null());
        assert!(ontology["roots"][1]["typeModel"].is_number());
        assert!(
            taxonomy["roots"]
                .as_array()
                .unwrap()
                .iter()
                .all(|root| root["typeModel"].is_null())
        );
        assert_eq!(ontology["relations"][0]["kind"], "association");
        assert_eq!(taxonomy["relations"][0]["kind"], "classification");
        assert_eq!(
            read_concept_ontology(
                &network,
                root,
                Some(LinkId::from_u64(
                    ontology["roots"][0]["link"].as_u64().unwrap()
                ))
            )
            .unwrap(),
            value["selectedOntology"]
        );
        assert_eq!(
            read_concept_taxonomy(
                &network,
                root,
                Some(LinkId::from_u64(
                    taxonomy["roots"][1]["link"].as_u64().unwrap()
                ))
            )
            .unwrap(),
            value["selectedTaxonomy"]
        );
        assert!(
            read_concept_ontology(
                &network,
                root,
                Some(LinkId::from_u64(
                    taxonomy["roots"][0]["link"].as_u64().unwrap()
                ))
            )
            .is_err()
        );
    }
}

#[test]
fn native_creation_matches_the_shared_link_graph_without_name_deduplication() {
    let data = fixture();
    let orphan = create_concept_views(None, &data["options"]).unwrap();
    assert_eq!(orphan.root.as_u64(), data["orphanOnly"]["root"]);
    assert_eq!(orphan.network.to_lino(), data["orphanOnly"]["links"]);
    let network = LinkNetwork::from_lino(data["base"]["links"].as_str().unwrap()).unwrap();
    let root = LinkId::from_u64(data["base"]["root"].as_u64().unwrap());
    let with_base = create_concept_views(Some((&network, root)), &data["options"]).unwrap();
    assert_eq!(with_base.root.as_u64(), data["withBase"]["root"]);
    assert_eq!(with_base.network.to_lino(), data["withBase"]["links"]);
    let mut duplicate = data["options"].clone();
    duplicate["orphans"] = serde_json::json!([duplicate["orphans"][0], duplicate["orphans"][0]]);
    assert!(create_concept_views(None, &duplicate).is_err());
    let mut misplaced = data["options"].clone();
    misplaced["taxonomy"]["relations"] = serde_json::json!([misplaced["ontology"]["relations"][0]]);
    assert!(create_concept_views(None, &misplaced).is_err());
}

#[test]
fn public_concept_views_retain_independent_memberships_and_every_root_through_both_archives() {
    let data = fixture();
    let names = [
        "fixture.orphan.recursion",
        "fixture.orphan.text",
        "fixture.orphan.unknown",
        "fixture.orphan.second-text",
    ];
    let traversal = [
        vec![vec![names[0], names[1], names[2]], vec![names[1], names[2]]],
        vec![vec![names[1], names[0], names[3]], vec![names[0], names[3]]],
    ];
    let base = LinkNetwork::from_lino(data["base"]["links"].as_str().unwrap()).unwrap();
    let base_root = LinkId::from_u64(data["base"]["root"].as_u64().unwrap());
    for (source, name) in [(None, "orphanOnly"), (Some((&base, base_root)), "withBase")] {
        let packet = create_concept_views(source, &data["options"]).unwrap();
        let binary =
            encode_link_network_binary(&packet.network, BinaryLinksOptions::default()).unwrap();
        for network in [
            LinkNetwork::from_lino(&packet.network.to_lino()).unwrap(),
            decode_link_network_binary(&binary, BinaryLinksOptions::default()).unwrap(),
        ] {
            assert_eq!(network.to_lino(), data[name]["links"]);
            let views = [
                read_concept_ontology(&network, packet.root, None).unwrap(),
                read_concept_taxonomy(&network, packet.root, None).unwrap(),
            ];
            assert_eq!(views[0], data[name]["expectedOntology"]);
            assert_eq!(views[1], data[name]["expectedTaxonomy"]);
            for (index, view) in views.iter().enumerate() {
                for (position, root) in view["roots"].as_array().unwrap().iter().enumerate() {
                    let root_id = LinkId::from_u64(root["link"].as_u64().unwrap());
                    assert_eq!(
                        network.link(root_id).unwrap().metadata().term(),
                        Some("concept view root")
                    );
                    let selected = if index == 0 {
                        read_concept_ontology(&network, packet.root, Some(root_id))
                    } else {
                        read_concept_taxonomy(&network, packet.root, Some(root_id))
                    }
                    .unwrap();
                    assert_eq!(
                        selected["selectedRoots"],
                        serde_json::json!([root_id.as_u64()])
                    );
                    let reached = selected["reachable"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .map(|id| {
                            view["concepts"]
                                .as_array()
                                .unwrap()
                                .iter()
                                .find(|concept| concept["link"] == *id)
                                .unwrap()["identity"]
                                .as_str()
                                .unwrap()
                        })
                        .collect::<Vec<_>>();
                    assert_eq!(reached, traversal[index][position]);
                    assert_eq!(selected["concepts"], view["concepts"]);
                    if let Some(id) = root["typeModel"].as_u64() {
                        assert!(network.link(LinkId::from_u64(id)).is_some());
                    }
                }
                let kinds = view["relations"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|edge| edge["kind"].as_str().unwrap())
                    .collect::<Vec<_>>();
                assert_eq!(
                    kinds,
                    if index == 0 {
                        ["association", "axiom"]
                    } else {
                        ["classification", "subsumption"]
                    }
                );
                for edge in view["relations"].as_array().unwrap() {
                    let link = network
                        .link(LinkId::from_u64(edge["link"].as_u64().unwrap()))
                        .unwrap();
                    assert_eq!(link.metadata().definition(), edge["kind"].as_str());
                    assert!(
                        network
                            .link(LinkId::from_u64(edge["model"].as_u64().unwrap()))
                            .is_some()
                    );
                }
                for name in names {
                    let concept = view["concepts"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .find(|concept| concept["identity"] == name)
                        .unwrap();
                    let other = views[1 - index]["concepts"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .find(|concept| concept["identity"] == name)
                        .unwrap();
                    assert_eq!(other["link"], concept["link"]);
                    for field in ["definitionRoots", "provenanceRoots"] {
                        let roots = concept[field].as_array().unwrap();
                        assert_ne!(roots.as_slice(), &[] as &[Value]);
                        for id in roots {
                            assert!(
                                network
                                    .link(LinkId::from_u64(id.as_u64().unwrap()))
                                    .is_some()
                            );
                        }
                    }
                    let original = data["options"]["orphans"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .find(|orphan| orphan["identity"] == name)
                        .unwrap();
                    assert_eq!(concept["provenance"], original["provenance"]);
                }
            }
        }
    }
    let mut options = data["options"].clone();
    options["ontology"] = serde_json::json!({"members": &names[..2], "roots": [{"identity": names[0]}], "relations": []});
    options["taxonomy"] = serde_json::json!({"members": &names[1..3], "roots": [{"identity": names[1]}, {"identity": names[2], "typeModel": options["orphans"][2]["definition"]}], "relations": []});
    let packet = create_concept_views(None, &options).unwrap();
    let binary =
        encode_link_network_binary(&packet.network, BinaryLinksOptions::default()).unwrap();
    for network in [
        LinkNetwork::from_lino(&packet.network.to_lino()).unwrap(),
        decode_link_network_binary(&binary, BinaryLinksOptions::default()).unwrap(),
    ] {
        let ontology = read_concept_ontology(&network, packet.root, None).unwrap();
        let taxonomy = read_concept_taxonomy(&network, packet.root, None).unwrap();
        let identities = |view: &Value| {
            view["concepts"]
                .as_array()
                .unwrap()
                .iter()
                .map(|concept| concept["identity"].as_str().unwrap().to_owned())
                .collect::<Vec<_>>()
        };
        assert_eq!(identities(&ontology), names[..2]);
        assert_eq!(identities(&taxonomy), names[1..3]);
        assert_eq!(ontology["concepts"][1], taxonomy["concepts"][0]);
        assert!(taxonomy["roots"][0]["typeModel"].is_null());
        assert!(
            network
                .link(LinkId::from_u64(
                    taxonomy["roots"][1]["typeModel"].as_u64().unwrap()
                ))
                .is_some()
        );
        assert_eq!(taxonomy["relations"], serde_json::json!([]));
    }
    for (requirement_id, assertions) in [
        (
            "I195-ONTOLOGY-LIBRARY-ENTRYPOINT",
            [
                "ontologyEntryPointAvailable",
                "grammarUnusedConceptsAvailable",
                "ontologyDefinitionsAndRelationsInspectable",
            ],
        ),
        (
            "I195-TAXONOMY-LIBRARY-ENTRYPOINT",
            [
                "taxonomyEntryPointAvailable",
                "classificationAndSubsumptionInspectable",
                "grammarUnusedTaxonomyConceptsAvailable",
            ],
        ),
        (
            "I195-ONTOLOGY-TAXONOMY-MEMBERSHIP",
            [
                "sharedOntologyTaxonomyIdentity",
                "membershipIndependentOfGrammarReachability",
                "definitionsAndProvenanceRetained",
            ],
        ),
        (
            "I195-ONTOLOGY-TAXONOMY-RELATION-DISTINCTION",
            [
                "ontologyAndTaxonomyRolesDistinct",
                "arbitraryRelationsDoNotImplyClassification",
                "hierarchySemanticsExplicit",
            ],
        ),
        (
            "I195-ONTOLOGY-MULTIPLE-LINK-ROOTS",
            [
                "multipleOntologyRootsSelectable",
                "ontologyRootsAreLinks",
                "ontologyRootTypeRoleExplicit",
            ],
        ),
        (
            "I195-TAXONOMY-MULTIPLE-LINK-ROOTS",
            [
                "multipleTaxonomyRootsSelectable",
                "taxonomyRootsAreLinks",
                "taxonomyRootTypeRoleExplicit",
            ],
        ),
    ] {
        super::issue_195_observations::record(&super::issue_195_observations::Observation {
            requirement_id,
            suffix: "behavior",
            fixture_id: "conversation-contract:recursiveDefinitionsRequest",
            fixture_file: "parity/sources/recursive-formal-definitions-2026-10-12.txt",
            assertions: &assertions,
            test_name: "public_concept_views_retain_independent_memberships_and_every_root_through_both_archives",
        });
    }
}
