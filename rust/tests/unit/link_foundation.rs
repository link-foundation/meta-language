//! Actual public graph construction and source-free replay of the shared model.
use meta_language::{
    BinaryLinksOptions, LinkId, LinkMetadata, LinkNetwork, LinkType, decode_link_network_binary,
    encode_link_network_binary, read_structured_data_lino,
};
use serde_json::Value;

fn foundation_fixture() -> Value {
    read_structured_data_lino(include_str!(
        "../../../parity/fixtures/link-foundation-behavior.lino"
    ))
    .unwrap()
}

fn foundation_references(network: &LinkNetwork, id: u64) -> Vec<u64> {
    network
        .link(LinkId::from_u64(id))
        .unwrap()
        .references()
        .iter()
        .map(|id| id.as_u64())
        .collect()
}

fn foundation_networks(data: &Value) -> [LinkNetwork; 3] {
    let mut original = LinkNetwork::new();
    for row in data["declarations"].as_array().unwrap() {
        let references = row["references"]
            .as_array()
            .unwrap()
            .iter()
            .map(|id| LinkId::from_u64(id.as_u64().unwrap()))
            .collect::<Vec<_>>();
        let kind = match row["type"].as_str().unwrap() {
            "Concept" => LinkType::Concept,
            "Semantic" => LinkType::Semantic,
            "Relation" => LinkType::Relation,
            other => panic!("unexpected declared fixture role {other}"),
        };
        let id = original.insert_dynamic_link(
            &references,
            LinkMetadata::new()
                .with_link_type(kind)
                .with_term(row["term"].as_str().unwrap())
                .with_language(data["language"].as_str().unwrap())
                .with_named(true),
        );
        assert_eq!(id.as_u64(), row["id"].as_u64().unwrap());
    }
    let bytes = data["binary"]
        .as_array()
        .unwrap()
        .iter()
        .map(|byte| u8::try_from(byte.as_u64().unwrap()).unwrap())
        .collect::<Vec<_>>();
    assert_eq!(
        encode_link_network_binary(&original, BinaryLinksOptions::default()).unwrap(),
        bytes
    );
    let networks = [
        original,
        LinkNetwork::from_lino(data["graph"].as_str().unwrap()).unwrap(),
        decode_link_network_binary(&bytes, BinaryLinksOptions::default()).unwrap(),
    ];
    for network in &networks {
        assert_eq!(network.to_lino(), data["graph"].as_str().unwrap());
        assert_eq!(
            encode_link_network_binary(network, BinaryLinksOptions::default()).unwrap(),
            bytes
        );
    }
    networks
}

fn check_foundation_vertex(network: &LinkNetwork, data: &Value) {
    let expected = &data["expected"];
    let vertex = expected["vertex"].as_u64().unwrap();
    let model = expected["model"].as_u64().unwrap();
    assert_eq!(foundation_references(network, vertex), [vertex, vertex]);
    assert_eq!(
        foundation_references(network, expected["modelSelection"].as_u64().unwrap()),
        [vertex, model]
    );
    assert_eq!(
        network
            .link(LinkId::from_u64(model))
            .unwrap()
            .metadata()
            .term(),
        data["selectedModel"].as_str()
    );
    assert_eq!(
        network
            .link(LinkId::from_u64(vertex))
            .unwrap()
            .metadata()
            .language(),
        data["language"].as_str()
    );
    assert_eq!(
        network
            .link(LinkId::from_u64(vertex))
            .unwrap()
            .id()
            .as_u64(),
        vertex
    );
    assert_eq!(
        network.find_term("declared vertex").unwrap().as_u64(),
        vertex
    );
}

fn check_foundation_roles(network: &LinkNetwork, data: &Value) {
    let expected = &data["expected"];
    let vertex = expected["vertex"].as_u64().unwrap();
    let assignments = network
        .links()
        .filter(|link| link.metadata().term() == Some("declared role"))
        .collect::<Vec<_>>();
    assert_eq!(assignments.len(), 3);
    let roles = expected["roles"].as_array().unwrap();
    assert_eq!(
        assignments
            .iter()
            .map(|link| foundation_references(network, link.id().as_u64()))
            .collect::<Vec<_>>(),
        roles
            .iter()
            .map(|role| vec![vertex, role["point"].as_u64().unwrap()])
            .collect::<Vec<_>>()
    );
    for role in roles {
        let point = role["point"].as_u64().unwrap();
        let references = foundation_references(network, role["assignment"].as_u64().unwrap());
        assert_eq!(references, [vertex, point]);
        assert_eq!(
            network
                .link(LinkId::from_u64(point))
                .unwrap()
                .metadata()
                .term(),
            role["term"].as_str()
        );
        assert!(std::ptr::eq(
            network.link(LinkId::from_u64(references[0])).unwrap(),
            network.link(LinkId::from_u64(vertex)).unwrap()
        ));
    }
    let roots = expected["roots"].as_array().unwrap();
    assert_eq!(roots.len(), 2);
    assert_ne!(roots[0]["root"], roots[1]["root"]);
    let entry = foundation_references(network, expected["entry"].as_u64().unwrap());
    for root in roots {
        let id = root["root"].as_u64().unwrap();
        assert!(entry.contains(&id));
        assert_eq!(
            foundation_references(network, root["membership"].as_u64().unwrap()),
            [id, vertex]
        );
        assert_eq!(
            network.link(LinkId::from_u64(id)).unwrap().id().as_u64(),
            id
        );
    }
}

fn check_foundation_composition(network: &LinkNetwork, data: &Value) {
    let expected = &data["expected"];
    let declarations = data["declarations"].as_array().unwrap();
    assert_eq!(network.links().count(), declarations.len());
    for row in declarations {
        let references = foundation_references(network, row["id"].as_u64().unwrap());
        assert_eq!(
            references,
            row["references"]
                .as_array()
                .unwrap()
                .iter()
                .map(|id| id.as_u64().unwrap())
                .collect::<Vec<_>>()
        );
        for child in references {
            assert!(network.link(LinkId::from_u64(child)).is_some());
        }
    }
    let vertex = expected["vertex"].as_u64().unwrap();
    let motif = expected["motif"].as_u64().unwrap();
    assert_eq!(foundation_references(network, motif), [vertex, vertex]);
    for owner in expected["sharedOwners"].as_array().unwrap() {
        assert_eq!(
            foundation_references(network, owner.as_u64().unwrap())[0],
            motif
        );
    }
    let cycle = expected["cycle"].as_array().unwrap();
    let first = cycle[0].as_u64().unwrap();
    let second = cycle[1].as_u64().unwrap();
    assert!(foundation_references(network, first).contains(&second));
    assert!(foundation_references(network, second).contains(&first));
    assert!(foundation_references(network, expected["entry"].as_u64().unwrap()).contains(&first));
    assert!(data["boundary"].as_str().unwrap().contains("no universal self-typing, physical fractal dimension, paradox elimination or consistency proof"));
    assert!(
        network
            .links()
            .all(|link| link.metadata().link_type() != Some(LinkType::Type))
    );
}

fn record_foundation(requirement_id: &str, assertions: &[&str], test_name: &str) {
    for (fixture_id, fixture_file) in [
        (
            "conversation-contract:linksFoundationRequest",
            "parity/sources/links-foundation-and-fractal-motivation-2026-10-12.txt",
        ),
        (
            "link-foundation-behavior",
            "parity/fixtures/link-foundation-behavior.lino",
        ),
    ] {
        super::issue_195_observations::record(&super::issue_195_observations::Observation {
            requirement_id,
            suffix: "behavior",
            fixture_id,
            fixture_file,
            assertions,
            test_name,
        });
    }
}

#[test]
fn explicit_doublet_vertices_retain_self_endpoints_identity_and_selected_model() {
    let data = foundation_fixture();
    for network in foundation_networks(&data) {
        check_foundation_vertex(&network, &data);
    }
    record_foundation(
        "I195-LINK-FOUNDATION-SELF-LOOP-VERTEX",
        &[
            "selfLoopVertexRepresentable",
            "selfReferencingEndpointsPreserved",
            "vertexModelAndIdentityRetained",
        ],
        "explicit_doublet_vertices_retain_self_endpoints_identity_and_selected_model",
    );
}

#[test]
fn one_link_retains_explicit_concept_entity_relationship_and_root_roles() {
    let data = foundation_fixture();
    for network in foundation_networks(&data) {
        check_foundation_roles(&network, &data);
    }
    record_foundation(
        "I195-LINK-FOUNDATION-MULTIPLE-ROLES",
        &[
            "linkIdentitySharedAcrossRoles",
            "rolesAreExplicitLinks",
            "multipleRootRolesPreserved",
        ],
        "one_link_retains_explicit_concept_entity_relationship_and_root_roles",
    );
}

#[test]
fn recursive_link_composition_preserves_shared_motifs_and_cycles_without_consistency_claims() {
    let data = foundation_fixture();
    for network in foundation_networks(&data) {
        check_foundation_composition(&network, &data);
    }
    record_foundation(
        "I195-LINK-FOUNDATION-RECURSIVE-COMPOSITION",
        &[
            "recursiveLinkCompositionPreserved",
            "sharedMotifAndCyclesExplicit",
            "noUnprovedConsistencyInference",
        ],
        "recursive_link_composition_preserves_shared_motifs_and_cycles_without_consistency_claims",
    );
}

#[test]
fn altered_self_endpoints_models_roles_roots_sharing_and_cycles_fail_the_independent_oracle() {
    let data = foundation_fixture();
    for (before, after, check) in [
        (
            "(1: 1 1 ",
            "(1: 1 2 ",
            check_foundation_vertex as fn(&LinkNetwork, &Value),
        ),
        ("(3: 1 2 ", "(3: 1 4 ", check_foundation_vertex),
        ("(7: 1 4 ", "(7: 14 4 ", check_foundation_roles),
        ("(13: 11 1 ", "(13: 11 14 ", check_foundation_roles),
        ("(16: 14 15 ", "(16: 1 15 ", check_foundation_composition),
        ("(18: 17 14 ", "(18: 16 14 ", check_foundation_composition),
    ] {
        let source = data["graph"].as_str().unwrap();
        assert_eq!(source.matches(before).count(), 1);
        let altered = source.replacen(before, after, 1);
        let network = LinkNetwork::from_lino(&altered).unwrap();
        assert!(std::panic::catch_unwind(|| check(&network, &data)).is_err());
    }
}
