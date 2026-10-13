//! Bounded discovery-output integration and independent CST assertions.
use meta_language::grammar::{
    FeatureParseOptions, GrammarKind, GrammarSourceName, LeafText, SyntaxTree, parse_with_grammar,
};
use meta_language::{
    GlobalConceptOptions, GlobalConceptSource, LinkId, LinkNetwork,
    create_discovered_concept_links, percent_encode_links_text, read_discovered_concept_links,
    read_structured_data_lino,
};
use serde_json::{Value, json};

fn fixture() -> Value {
    read_structured_data_lino(include_str!(
        "../../../parity/fixtures/discovered-concept-links.lino"
    ))
    .unwrap()
}

fn tree_value(tree: &SyntaxTree) -> Value {
    match tree {
        SyntaxTree::Node {
            kind,
            field,
            trivia,
            start,
            end,
            children,
            attributes,
        } => {
            assert!(field.is_none() && !trivia && attributes.is_none());
            json!({"type": "node", "kind": kind, "start": start, "end": end, "children": children.iter().map(tree_value).collect::<Vec<_>>()})
        }
        SyntaxTree::Token {
            kind,
            field,
            trivia,
            start,
            end,
            attributes,
            text: LeafText::Text(text),
        } => {
            assert!(field.is_none() && !trivia && attributes.is_none());
            json!({"type": "token", "kind": kind, "start": start, "end": end, "text": text})
        }
        _ => panic!(
            "the independently expected recursive CST contains only nodes and exact Greek tokens"
        ),
    }
}

fn bare_space(space: &Value) -> Value {
    let mut space = space.as_object().unwrap().clone();
    space.remove("formalDefinitions");
    Value::Object(space)
}

fn role(network: &LinkNetwork, term: &str) -> Vec<LinkId> {
    network
        .links()
        .filter(|link| {
            link.metadata().language() == Some("meta-language discovered concepts")
                && link.metadata().term() == Some(term)
        })
        .map(|link| link.id())
        .collect()
}

fn replace_references(network: &LinkNetwork, id: LinkId, references: &[LinkId]) -> LinkNetwork {
    let text = network
        .to_lino()
        .lines()
        .map(|line| {
            if !line.starts_with(&format!("({id}: ")) {
                return line.to_owned();
            }
            let metadata = &line[line.find("(meta:").unwrap()..];
            format!(
                "({id}: {} {metadata}",
                references
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(" ")
            )
        })
        .collect::<Vec<_>>()
        .join("\n");
    LinkNetwork::from_lino(&text).unwrap()
}

#[test]
fn discovered_candidates_reload_and_parse_without_original_sources() {
    let data = fixture();
    let network = LinkNetwork::from_lino(data["links"].as_str().unwrap()).unwrap();
    let root = LinkId::from_u64(data["root"].as_u64().unwrap());
    let loaded = read_discovered_concept_links(&network, root).unwrap();
    assert_eq!(loaded.space, data["expectedSpace"]);
    assert_eq!(loaded.sources.len(), 3);
    assert_eq!(role(&network, "discovered concept candidate").len(), 9);
    assert_eq!(
        loaded.space["mappings"]["first:root"],
        loaded.space["mappings"]["second:page"]
    );
    assert_ne!(
        loaded.space["mappings"]["first:root"],
        loaded.space["mappings"]["alternative:root"]
    );
    assert_eq!(
        loaded.space["failures"].as_array().unwrap().last().unwrap()["reason"],
        "no-one-to-one-proof"
    );
    for (index, id) in ["first", "second"].iter().enumerate() {
        let source = loaded
            .sources
            .iter()
            .find(|source| source.id == *id)
            .unwrap();
        let tree = parse_with_grammar(
            &source.grammar,
            data["parseSource"].as_str().unwrap().as_bytes(),
            &FeatureParseOptions::default(),
        )
        .unwrap();
        assert_eq!(tree_value(&tree), data["expectedTrees"][index]);
        assert_eq!(
            source.grammar.rules()[0].source_names[0].name,
            data["sources"][index]["grammar"]["rules"][0]["sourceNames"][0]["name"]
        );
    }
    let candidate = role(&network, "discovered concept candidate")
        .into_iter()
        .find(|id| network.link(*id).unwrap().metadata().definition() == Some("first:root"))
        .unwrap();
    let candidate = network.link(candidate).unwrap();
    let references = network
        .link(candidate.references()[3])
        .unwrap()
        .references();
    assert!(
        references.contains(&candidate.references()[1]),
        "the recursive production references its actual global concept point"
    );
    let unproved = &loaded.formal_definitions["alternative:root"];
    let formal = &unproved.network;
    assert!(
        formal
            .links()
            .any(|link| link.metadata().definition() == Some("unresolved interpretation"))
    );
    assert!(
        loaded.space["formalDefinitions"]
            .get("alternative:root")
            .is_none()
    );
}

#[test]
fn native_discovery_copy_retains_every_production_and_formal_alternative() {
    let data = fixture();
    let mut sources = data["sources"]
        .as_array()
        .unwrap()
        .iter()
        .map(|source| GlobalConceptSource::from_json(source).unwrap())
        .collect::<Vec<_>>();
    sources[0].grammar.set_kinds(vec![GrammarKind {
        name: "expression node".to_owned(),
        source_names: vec![GrammarSourceName {
            source: "independent discovery".to_owned(),
            name: "expression alias".to_owned(),
        }],
    }]);
    let options = GlobalConceptOptions {
        records: Some(data["records"].clone()),
        formal_definitions: Some(data["formalDefinitions"].clone()),
        previous: None,
        equivalences: data["equivalences"]
            .as_array()
            .unwrap()
            .iter()
            .map(|pair| {
                [
                    pair[0].as_str().unwrap().to_owned(),
                    pair[1].as_str().unwrap().to_owned(),
                ]
            })
            .collect(),
    };
    let packet = create_discovered_concept_links(&sources, &options).unwrap();
    assert_eq!(
        bare_space(&packet.space),
        bare_space(&data["expectedSpace"])
    );
    let loaded = read_discovered_concept_links(
        &LinkNetwork::from_lino(&packet.network.to_lino()).unwrap(),
        packet.root,
    )
    .unwrap();
    assert_eq!(
        loaded.sources[0].grammar.kinds(),
        sources[0].grammar.kinds()
    );
    assert_eq!(
        bare_space(&loaded.space),
        bare_space(&data["expectedSpace"])
    );
    assert_eq!(
        loaded.formal_definitions.len(),
        data["formalDefinitions"].as_object().unwrap().len()
    );
    for (alias, archive) in &loaded.formal_definitions {
        let actual = &archive.network;
        let original =
            LinkNetwork::from_lino(data["formalDefinitions"][alias]["links"].as_str().unwrap())
                .unwrap();
        assert_eq!(actual.len(), original.len());
        for link in original.links() {
            if let Some(term) = link.metadata().term()
                && let Some(original_id) = original.find_term(term)
            {
                let actual_id = actual
                    .find_term(term)
                    .expect("registered term survives reload");
                assert_eq!(
                    actual.definition_for(actual_id),
                    original.definition_for(original_id)
                );
            }
        }
    }
}

#[test]
fn discovery_reload_rejects_foreign_origin_dependencies_and_graph_inventories() {
    let data = fixture();
    let network = LinkNetwork::from_lino(data["links"].as_str().unwrap()).unwrap();
    let root = LinkId::from_u64(data["root"].as_u64().unwrap());
    let candidates = role(&network, "discovered concept candidate");
    let mut references = network.link(candidates[0]).unwrap().references().to_vec();
    references[0] = network.link(candidates[1]).unwrap().references()[0];
    let changed = replace_references(&network, candidates[0], &references);
    assert!(
        read_discovered_concept_links(&changed, root)
            .unwrap_err()
            .contains("foreign candidate provenance")
    );
    let candidate = network.link(candidates[0]).unwrap();
    let changed = replace_references(
        &network,
        candidate.references()[3],
        &[candidate.references()[1]],
    );
    assert!(
        read_discovered_concept_links(&changed, root)
            .unwrap_err()
            .contains("foreign recursive concept dependency")
    );
    let definitions = role(&network, "discovered formal definition");
    let inventory = network.link(definitions[0]).unwrap().references()[1];
    let mut references = network.link(inventory).unwrap().references().to_vec();
    references.push(network.link(definitions[1]).unwrap().references()[0]);
    let changed = replace_references(&network, inventory, &references);
    assert!(read_discovered_concept_links(&changed, root).is_err());
}

#[test]
fn changing_a_formal_premise_invalidates_the_archived_sharing_certificate() {
    let data = fixture();
    let network = LinkNetwork::from_lino(data["links"].as_str().unwrap()).unwrap();
    let root = LinkId::from_u64(data["root"].as_u64().unwrap());
    let declaration = role(&network, "discovered formal definition")
        .into_iter()
        .find(|id| {
            network.link(*id).unwrap().metadata().definition()
                == Some("fixture.recursive-expression")
        })
        .unwrap();
    let definition = network.link(declaration).unwrap().references()[0];
    let before = format!(
        "(lang: {})",
        percent_encode_links_text(
            network
                .link(definition)
                .unwrap()
                .metadata()
                .language()
                .unwrap()
        )
    );
    let after = format!(
        "(lang: {})",
        percent_encode_links_text("counterfeit source semantics")
    );
    let changed = network
        .to_lino()
        .lines()
        .map(|line| {
            if line.starts_with(&format!("({definition}: ")) {
                assert!(line.contains(&before));
                line.replacen(&before, &after, 1)
            } else {
                line.to_owned()
            }
        })
        .collect::<Vec<_>>()
        .join("\n");
    let changed = LinkNetwork::from_lino(&changed).unwrap();
    assert!(
        read_discovered_concept_links(&changed, root)
            .unwrap_err()
            .contains("stale or counterfeit global concept certificate")
    );
}
