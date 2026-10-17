//! Shared independent syntax oracles, canonical conversion and linked mutations.
//! Native execution is performed by CI; fixture storage is `LiNo` throughout.

use meta_language::{
    LinkId, LinkNetwork, ResourceDescriptionLinks, decode_resource_description_graph,
    emit_resource_description_ntriples, import_resource_description_ntriples,
    percent_encode_links_text, read_resource_description_ntriples_source,
    read_structured_data_lino,
};
use serde_json::Value;

fn fixture() -> Value {
    read_structured_data_lino(include_str!(
        "../../../parity/fixtures/resource-description-ntriples.lino"
    ))
    .unwrap()
}
fn reload(graph: &ResourceDescriptionLinks) -> ResourceDescriptionLinks {
    ResourceDescriptionLinks {
        network: LinkNetwork::from_lino(&graph.network.to_lino()).unwrap(),
        root: graph.root,
    }
}
fn sorted(triples: &Value) -> Vec<String> {
    let mut values = triples
        .as_array()
        .unwrap()
        .iter()
        .map(|value| serde_json::to_string(value).unwrap())
        .collect::<Vec<_>>();
    values.sort();
    values
}
fn replace_definition(network: &LinkNetwork, id: LinkId, before: &str, after: &str) -> LinkNetwork {
    let source = network
        .to_lino()
        .lines()
        .map(|line| {
            if !line.starts_with(&format!("({id}: ")) {
                return line.to_owned();
            }
            line.replace(
                &format!("(def: {})", percent_encode_links_text(before)),
                &format!("(def: {})", percent_encode_links_text(after)),
            )
        })
        .collect::<Vec<_>>()
        .join("\n");
    LinkNetwork::from_lino(&source).unwrap()
}
fn replace_references(network: &LinkNetwork, id: LinkId, references: &[LinkId]) -> LinkNetwork {
    let source = network
        .to_lino()
        .lines()
        .map(|line| {
            if !line.starts_with(&format!("({id}: ")) {
                return line.to_owned();
            }
            format!(
                "({id}: {} {}",
                references
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(" "),
                &line[line.find("(meta:").unwrap()..]
            )
        })
        .collect::<Vec<_>>()
        .join("\n");
    LinkNetwork::from_lino(&source).unwrap()
}

#[test]
fn independent_ntriples_sources_and_javascript_archives_match_native_terms_and_provenance() {
    let data = fixture();
    for case in data["cases"].as_array().unwrap() {
        let network = LinkNetwork::from_lino(case["archive"].as_str().unwrap()).unwrap();
        let root = LinkId::from_u64(case["root"].as_u64().unwrap());
        let syntax_root = LinkId::from_u64(case["syntaxRoot"].as_u64().unwrap());
        assert_eq!(
            sorted(&decode_resource_description_graph(&network, root).unwrap()["triples"]),
            sorted(&case["triples"]),
            "{}",
            case["id"]
        );
        assert_eq!(
            read_resource_description_ntriples_source(&network, syntax_root).unwrap(),
            case["source"].as_str().unwrap()
        );
        assert_eq!(
            emit_resource_description_ntriples(&network, root)
                .unwrap()
                .source,
            case["canonical"].as_str().unwrap()
        );
        let graph = import_resource_description_ntriples(
            case["source"].as_str().unwrap(),
            "syntax-fixture",
            None,
        )
        .unwrap();
        let network = LinkNetwork::from_lino(&graph.network.to_lino()).unwrap();
        assert_eq!(
            sorted(&decode_resource_description_graph(&network, graph.root).unwrap()["triples"]),
            sorted(&case["triples"])
        );
        assert_eq!(
            read_resource_description_ntriples_source(&network, graph.syntax_root).unwrap(),
            case["source"].as_str().unwrap()
        );
        assert_eq!(
            emit_resource_description_ntriples(&network, graph.root)
                .unwrap()
                .source,
            case["canonical"].as_str().unwrap()
        );
        if case["id"] == "unicode-language-duplicates" {
            let decoded = decode_resource_description_graph(&network, graph.root).unwrap();
            assert_eq!(decoded["triples"].as_array().unwrap().len(), 1);
            assert_eq!(decoded["occurrences"][0]["sourceLanguage"], "EN-us");
            assert_eq!(decoded["occurrences"][1]["sourceLanguage"], "en-US");
        }
    }
}

#[test]
fn ntriples_blank_correspondences_restore_owned_source_scope_after_reload() {
    let data = fixture();
    for case in data["cases"].as_array().unwrap() {
        let graph = import_resource_description_ntriples(
            case["source"].as_str().unwrap(),
            "syntax-fixture",
            None,
        )
        .unwrap();
        let emitted = emit_resource_description_ntriples(&graph.network, graph.root).unwrap();
        let correspondence = reload(&emitted.correspondence);
        let restored = import_resource_description_ntriples(
            &emitted.source,
            "other-source",
            Some(&correspondence),
        )
        .unwrap();
        assert_eq!(
            sorted(
                &decode_resource_description_graph(&restored.network, restored.root).unwrap()["triples"]
            ),
            sorted(&case["triples"])
        );
        let archived = ResourceDescriptionLinks {
            network: LinkNetwork::from_lino(case["correspondenceArchive"].as_str().unwrap())
                .unwrap(),
            root: LinkId::from_u64(case["correspondenceRoot"].as_u64().unwrap()),
        };
        let restored = import_resource_description_ntriples(
            case["canonical"].as_str().unwrap(),
            "other-source",
            Some(&archived),
        )
        .unwrap();
        assert_eq!(
            sorted(
                &decode_resource_description_graph(&restored.network, restored.root).unwrap()["triples"]
            ),
            sorted(&case["triples"])
        );
    }
}

#[test]
fn ntriples_conversion_uses_edited_semantic_links_without_source_replay() {
    let graph = import_resource_description_ntriples("<urn:s><urn:p><urn:o>.", "s", None).unwrap();
    let value = graph
        .network
        .links()
        .find(|link| {
            link.metadata().language() == Some("meta-language resource description graph")
                && link.metadata().definition() == Some("urn:o")
        })
        .unwrap()
        .id();
    let network = replace_definition(&graph.network, value, "urn:o", "urn:edited");
    assert_eq!(
        read_resource_description_ntriples_source(&network, graph.syntax_root).unwrap(),
        "<urn:s><urn:p><urn:o>."
    );
    assert_eq!(
        emit_resource_description_ntriples(&network, graph.root)
            .unwrap()
            .source,
        "<urn:s> <urn:p> <urn:edited> .\n"
    );
}

#[test]
fn ntriples_malformed_inputs_and_unsupported_profiles_remain_distinct() {
    let data = fixture();
    for section in ["malformed", "unsupported"] {
        for case in data[section].as_array().unwrap() {
            let source = case["source"].as_str().unwrap();
            let diagnostic = import_resource_description_ntriples(source, "s", None).unwrap_err();
            assert_eq!(
                diagnostic.kind,
                case["kind"].as_str().unwrap(),
                "{}",
                case["id"]
            );
            assert!(diagnostic.offset <= source.encode_utf16().count());
        }
    }
    let source = "<urn:s> <urn:p> \"\\uD800\" .";
    assert_eq!(
        import_resource_description_ntriples(source, "s", None)
            .unwrap_err()
            .offset,
        source.find('\\').unwrap()
    );
    assert_eq!(
        import_resource_description_ntriples(&" ".repeat(262_145), "s", None)
            .unwrap_err()
            .kind,
        "resourceLimit"
    );
    assert_eq!(
        import_resource_description_ntriples(&"\n ".repeat(5_001), "s", None)
            .unwrap_err()
            .kind,
        "resourceLimit"
    );
}

#[test]
fn ntriples_syntax_and_correspondence_ownership_cannot_be_forged() {
    let mut graph = import_resource_description_ntriples("_:a <urn:p> _:b .", "s", None).unwrap();
    graph
        .network
        .insert_field(graph.syntax_root, "concealed", graph.root);
    assert!(
        read_resource_description_ntriples_source(&graph.network, graph.syntax_root)
            .unwrap_err()
            .message
            .contains("concealed fields")
    );
    let emitted = emit_resource_description_ntriples(&graph.network, graph.root).unwrap();
    let mut correspondence = emitted.correspondence;
    let pairs = correspondence
        .network
        .link(correspondence.root)
        .unwrap()
        .references()[1];
    let first = correspondence.network.link(pairs).unwrap().references()[0];
    correspondence.network = replace_references(&correspondence.network, pairs, &[first, first]);
    assert!(
        import_resource_description_ntriples(&emitted.source, "other", Some(&correspondence))
            .unwrap_err()
            .message
            .contains("one to one")
    );
}
