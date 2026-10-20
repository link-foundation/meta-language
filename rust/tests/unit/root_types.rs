//! Explicit root type roles use the same public links and model data as JavaScript.
use meta_language::{
    BinaryLinksOptions, LinkId, LinkMetadata, LinkNetwork, LinkType, decode_link_network_binary,
    encode_link_network_binary, read_structured_data_lino,
};
use serde_json::Value;

fn root_type_fixture() -> Value {
    read_structured_data_lino(include_str!(
        "../../../parity/fixtures/root-type-behavior.lino"
    ))
    .unwrap()
}

fn root_type_references(network: &LinkNetwork, id: u64) -> Vec<u64> {
    network
        .link(LinkId::from_u64(id))
        .unwrap()
        .references()
        .iter()
        .map(|id| id.as_u64())
        .collect()
}

fn root_type_kind(name: &str) -> LinkType {
    match name {
        "Concept" => LinkType::Concept,
        "Semantic" => LinkType::Semantic,
        "Type" => LinkType::Type,
        other => panic!("unexpected declared type role {other}"),
    }
}

fn root_type_networks(data: &Value, row: &Value) -> [LinkNetwork; 3] {
    let mut original = LinkNetwork::new();
    for declaration in row["declarations"].as_array().unwrap() {
        let references = declaration["references"]
            .as_array()
            .unwrap()
            .iter()
            .map(|id| LinkId::from_u64(id.as_u64().unwrap()))
            .collect::<Vec<_>>();
        let id = original.insert_dynamic_link(
            &references,
            LinkMetadata::new()
                .with_link_type(root_type_kind(declaration["type"].as_str().unwrap()))
                .with_term(declaration["term"].as_str().unwrap())
                .with_language(data["language"].as_str().unwrap())
                .with_named(true),
        );
        assert_eq!(id.as_u64(), declaration["id"].as_u64().unwrap());
    }
    let bytes = row["binary"]
        .as_array()
        .unwrap()
        .iter()
        .map(|byte| u8::try_from(byte.as_u64().unwrap()).unwrap())
        .collect::<Vec<_>>();
    assert_eq!(original.to_lino(), row["graph"].as_str().unwrap());
    assert_eq!(
        encode_link_network_binary(&original, BinaryLinksOptions::default()).unwrap(),
        bytes
    );
    let networks = [
        original,
        LinkNetwork::from_lino(row["graph"].as_str().unwrap()).unwrap(),
        decode_link_network_binary(&bytes, BinaryLinksOptions::default()).unwrap(),
    ];
    for network in &networks {
        assert_eq!(network.to_lino(), row["graph"].as_str().unwrap());
        assert_eq!(
            encode_link_network_binary(network, BinaryLinksOptions::default()).unwrap(),
            bytes
        );
    }
    networks
}

fn check_root_type_model(network: &LinkNetwork, data: &Value, row: &Value) {
    let typed = row["typed"].as_bool().unwrap();
    assert_eq!(network.links().count(), if typed { 10 } else { 9 });
    for declaration in row["declarations"].as_array().unwrap() {
        let id = declaration["id"].as_u64().unwrap();
        assert_eq!(
            root_type_references(network, id),
            declaration["references"]
                .as_array()
                .unwrap()
                .iter()
                .map(|id| id.as_u64().unwrap())
                .collect::<Vec<_>>()
        );
        let metadata = network.link(LinkId::from_u64(id)).unwrap().metadata();
        assert_eq!(
            metadata.link_type(),
            root_type_kind(declaration["type"].as_str().unwrap())
        );
        assert_eq!(metadata.term(), declaration["term"].as_str());
        assert_eq!(metadata.language(), data["language"].as_str());
    }
    assert_eq!(root_type_references(network, 1), [1, 1]);
    assert_eq!(root_type_references(network, 3), [1, 2]);
    assert_eq!(root_type_references(network, 6), [2, 5]);
    assert_eq!(root_type_references(network, 8), [1, 7]);
    assert_eq!(network.find_term("selected root").unwrap().as_u64(), 1);
    let types = network
        .links()
        .filter(|link| link.metadata().link_type() == LinkType::Type)
        .collect::<Vec<_>>();
    let roles = network
        .links()
        .filter(|link| link.metadata().term() == Some("declared type role"))
        .collect::<Vec<_>>();
    assert_eq!(types.len(), usize::from(typed));
    assert_eq!(roles.len(), usize::from(typed));
    if typed {
        assert_eq!(types[0].id().as_u64(), 1);
        assert_eq!(root_type_references(network, 10), [1, 4]);
        assert!(root_type_references(network, 9).contains(&10));
    }
    assert!(!network.links().any(|link| matches!(
        link.metadata().term(),
        Some("universe membership" | "universal proof logic" | "universal self typing")
    )));
    assert!(
        data["boundary"]
            .as_str()
            .unwrap()
            .contains("not a universe membership or Type:Type axiom")
    );
}

#[test]
fn optional_root_types_preserve_declared_roles_and_foundation_premises_through_both_archives() {
    let data = root_type_fixture();
    let cases = data["cases"].as_array().unwrap();
    assert_eq!(cases.len(), 2);
    assert_eq!(cases[0]["typed"], false);
    assert_eq!(cases[1]["typed"], true);
    for row in cases {
        for network in root_type_networks(&data, row) {
            check_root_type_model(&network, &data, row);
        }
    }
    for (fixture_id, fixture_file) in [
        (
            "conversation-contract:linksFoundationRequest",
            "parity/sources/links-foundation-and-fractal-motivation-2026-10-12.txt",
        ),
        (
            "root-type-behavior",
            "parity/fixtures/root-type-behavior.lino",
        ),
    ] {
        super::issue_195_observations::record(&super::issue_195_observations::Observation {
            requirement_id: "I195-LINK-FOUNDATION-OPTIONAL-TYPE-ROOT",
            suffix: "behavior",
            fixture_id,
            fixture_file,
            assertions: &[
                "rootTypeRolesDeclared",
                "noImplicitUniversalSelfTyping",
                "selectedFoundationPremisesPreserved",
            ],
            test_name: "optional_root_types_preserve_declared_roles_and_foundation_premises_through_both_archives",
        });
    }
}

#[test]
fn altered_root_model_premise_type_roles_and_metadata_fail_the_complete_oracle() {
    let data = root_type_fixture();
    for row in data["cases"].as_array().unwrap() {
        let typed = row["typed"].as_bool().unwrap();
        let source = row["graph"].as_str().unwrap();
        let mut edits = vec![
            ("(3: 1 2 ", "(3: 1 4 "),
            ("(6: 2 5 ", "(6: 2 4 "),
            ("(8: 1 7 ", "(8: 1 4 "),
        ];
        if typed {
            edits.push(("(10: 1 4 ", "(10: 1 1 "));
        }
        for (before, after) in edits {
            assert_eq!(source.matches(before).count(), 1);
            let network = LinkNetwork::from_lino(&source.replacen(before, after, 1)).unwrap();
            assert!(
                std::panic::catch_unwind(|| check_root_type_model(&network, &data, row)).is_err()
            );
        }
        let before = if typed { "(t: type)" } else { "(t: concept)" };
        let after = if typed { "(t: concept)" } else { "(t: type)" };
        let network = LinkNetwork::from_lino(&source.replacen(before, after, 1)).unwrap();
        assert!(std::panic::catch_unwind(|| check_root_type_model(&network, &data, row)).is_err());
    }
}
