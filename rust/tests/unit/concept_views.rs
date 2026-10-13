//! Paired public entry points over the same orphan and grammar concept links.
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
