//! Bounded discovery-output integration and independent CST assertions.
use std::collections::{BTreeMap, BTreeSet};

use meta_language::structured_data_links::{
    decode_structured_data_links, encode_structured_data_links,
};
use meta_language::{
    FeatureParseOptions, GlobalConceptOptions, GlobalConceptSource, GrammarKind, GrammarSourceName,
    LeafText, LinkId, LinkNetwork, SyntaxTree, create_discovered_concept_links,
    parse_grammar_links, parse_with_grammar, percent_encode_links_text,
    read_discovered_concept_links, read_formal_concept_definition, read_structured_data_lino,
};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};

fn fixture() -> Value {
    read_structured_data_lino(include_str!(
        "../../../parity/fixtures/discovered-concept-links.lino"
    ))
    .unwrap()
}

fn legacy_fixture() -> Value {
    read_structured_data_lino(include_str!(
        "../../../parity/fixtures/discovered-concept-links-legacy.lino"
    ))
    .unwrap()
}

fn with_discovered_source_formats(data: &Value) -> Value {
    let mut expected = data["expectedSpace"].clone();
    for source in expected["sources"].as_array_mut().unwrap() {
        let original = data["sources"]
            .as_array()
            .unwrap()
            .iter()
            .find(|original| original["id"] == source["id"])
            .unwrap();
        if let Some(format) = original["grammar"]
            .get("sourceFormat")
            .filter(|format| !format.is_null())
        {
            source["sourceFormat"] = format.clone();
        }
    }
    expected
}

fn discovered_tree_bytes(tree: &Value) -> Vec<u8> {
    if tree["type"] == "node" {
        tree["children"]
            .as_array()
            .unwrap()
            .iter()
            .flat_map(discovered_tree_bytes)
            .collect()
    } else {
        tree["text"].as_str().unwrap().as_bytes().to_vec()
    }
}

fn serialized_link_identity(line: &str) -> LinkId {
    LinkId::from_u64(
        line.strip_prefix('(')
            .unwrap()
            .split_once(':')
            .unwrap()
            .0
            .parse()
            .unwrap(),
    )
}

// Rebuild only the structured envelope island. All formal islands, candidate
// references and their registered identities retain their original bytes.
fn edit_discovered_envelope(
    network: &LinkNetwork,
    root: LinkId,
    edit: impl FnOnce(&mut Value),
) -> LinkNetwork {
    let data_root = network.link(root).unwrap().references()[0];
    let mut ids = BTreeSet::new();
    let mut pending = vec![data_root];
    while let Some(id) = pending.pop() {
        if ids.insert(id) {
            pending.extend(network.link(id).unwrap().references());
        }
    }
    let original = network.to_lino();
    let island = original
        .lines()
        .filter(|line| ids.contains(&serialized_link_identity(line)))
        .collect::<Vec<_>>()
        .join("\n");
    let mut value =
        decode_structured_data_links(&LinkNetwork::from_lino(&island).unwrap(), data_root).unwrap();
    edit(&mut value);
    let replacement = encode_structured_data_links(&value).unwrap();
    let mut next = network
        .links()
        .map(|link| link.id().as_u64())
        .max()
        .unwrap()
        + 1;
    let remapped = replacement
        .network
        .links()
        .map(|link| {
            let id = LinkId::from_u64(next);
            next += 1;
            (link.id(), id)
        })
        .collect::<BTreeMap<_, _>>();
    let mut lines = original
        .lines()
        .filter(|line| !ids.contains(&serialized_link_identity(line)))
        .map(str::to_owned)
        .collect::<Vec<_>>();
    for line in replacement.network.to_lino().lines() {
        let id = serialized_link_identity(line);
        let references = replacement
            .network
            .link(id)
            .unwrap()
            .references()
            .iter()
            .map(|id| remapped[id].to_string())
            .collect::<Vec<_>>()
            .join(" ");
        lines.push(format!(
            "({}: {references} {}",
            remapped[&id],
            &line[line.find("(meta:").unwrap()..]
        ));
    }
    let changed = LinkNetwork::from_lino(&lines.join("\n")).unwrap();
    let mut references = changed.link(root).unwrap().references().to_vec();
    references[0] = remapped[&replacement.root];
    replace_references(&changed, root, &references)
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
            lexical,
        } => {
            assert!(field.is_none() && !trivia && attributes.is_none());
            assert!(lexical.is_none());
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
        .map(meta_language::Link::id)
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
    for data in [fixture(), legacy_fixture()] {
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
}

#[test]
fn retained_legacy_and_current_discovery_archives_revalidate_the_same_meaning_and_cst() {
    let current = fixture();
    let legacy = legacy_fixture();
    assert_eq!(
        format!(
            "{:x}",
            Sha256::digest(include_bytes!(
                "../../../parity/fixtures/discovered-concept-links-legacy.lino"
            ))
        ),
        "69daf66d8a97bedf33b84dcfe1dc5ca40718e86ba8c71034f7f3a1596b78e84e"
    );
    for field in [
        "sources",
        "records",
        "equivalences",
        "formalDefinitions",
        "parseSource",
        "expectedTrees",
    ] {
        assert_eq!(
            current[field], legacy[field],
            "original {field} remains unchanged"
        );
    }
    let current_network = LinkNetwork::from_lino(current["links"].as_str().unwrap()).unwrap();
    let current_loaded = read_discovered_concept_links(
        &current_network,
        LinkId::from_u64(current["root"].as_u64().unwrap()),
    )
    .unwrap();
    let legacy_network = LinkNetwork::from_lino(legacy["links"].as_str().unwrap()).unwrap();
    let loaded = read_discovered_concept_links(
        &legacy_network,
        LinkId::from_u64(legacy["root"].as_u64().unwrap()),
    )
    .unwrap();
    assert_eq!(loaded.space, with_discovered_source_formats(&legacy));
    assert_eq!(bare_space(&loaded.space), bare_space(&current_loaded.space));
    assert_eq!(
        loaded.formal_definitions.keys().collect::<Vec<_>>(),
        current_loaded.formal_definitions.keys().collect::<Vec<_>>()
    );
    for (alias, definition) in &loaded.formal_definitions {
        assert_eq!(
            read_formal_concept_definition(&definition.to_json()).unwrap(),
            read_formal_concept_definition(&current_loaded.formal_definitions[alias].to_json())
                .unwrap()
        );
    }
    for (index, id) in ["first", "second"].iter().enumerate() {
        let source = loaded
            .sources
            .iter()
            .find(|source| source.id == *id)
            .unwrap();
        assert_eq!(
            json!(source.grammar.source_format().map(|format| format.as_str())),
            legacy["sources"][index]["grammar"]["sourceFormat"]
        );
        let tree = parse_with_grammar(
            &source.grammar,
            legacy["parseSource"].as_str().unwrap().as_bytes(),
            &FeatureParseOptions::default(),
        )
        .unwrap();
        assert_eq!(tree_value(&tree), legacy["expectedTrees"][index]);
        assert_eq!(
            discovered_tree_bytes(&tree_value(&tree)),
            legacy["parseSource"].as_str().unwrap().as_bytes()
        );
    }
    assert_eq!(legacy_network.to_lino(), legacy["links"].as_str().unwrap());
    let definitions = legacy["formalDefinitions"]
        .as_object()
        .unwrap()
        .keys()
        .map(|alias| (alias.clone(), loaded.formal_definitions[alias].to_json()))
        .collect();
    let options = GlobalConceptOptions {
        records: Some(legacy["records"].clone()),
        formal_definitions: Some(Value::Object(definitions)),
        equivalences: legacy["equivalences"]
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
        ..GlobalConceptOptions::default()
    };
    let recreated = create_discovered_concept_links(&loaded.sources, &options).unwrap();
    assert_eq!(recreated.root.as_u64(), current["root"].as_u64().unwrap());
    assert_eq!(
        recreated.network.to_lino(),
        current["links"].as_str().unwrap()
    );
    let repeated = create_discovered_concept_links(&loaded.sources, &options).unwrap();
    assert_eq!(repeated.network.to_lino(), recreated.network.to_lino());
}

#[test]
fn discovery_certificates_refuse_mixed_format_provenance_and_all_other_forgeries() {
    for (data, edits) in [
        (fixture(), [0, 1, 2, 3, 4]),
        (legacy_fixture(), [5, 6, 7, 8, 9]),
    ] {
        let network = LinkNetwork::from_lino(data["links"].as_str().unwrap()).unwrap();
        let root = LinkId::from_u64(data["root"].as_u64().unwrap());
        for edit in edits {
            let changed = edit_discovered_envelope(&network, root, |value| match edit {
                0 => value["space"]["sources"][0]["sourceFormat"] = json!("peg"),
                1 => {
                    value["space"]["sources"][0]
                        .as_object_mut()
                        .unwrap()
                        .remove("sourceFormat");
                }
                2 | 8 => value["space"]["sources"][0]["context"]["matching"] = json!("peg"),
                3 | 9 => {
                    value["space"]["proofs"][0]["correspondence"][0][1] =
                        value["space"]["proofs"][0]["correspondence"][0][0].clone()
                }
                4 => value["space"]["sources"][0]["id"] = json!("counterfeit source"),
                5 => value["space"]["sources"][0]["sourceFormat"] = json!("bnf"),
                6 => {
                    let source = value["sources"]
                        .as_array_mut()
                        .unwrap()
                        .iter_mut()
                        .find(|source| source["id"] == "first")
                        .unwrap();
                    source["grammar"]["sourceFormat"] = json!("peg");
                }
                7 => {
                    let source = value["sources"]
                        .as_array_mut()
                        .unwrap()
                        .iter_mut()
                        .find(|source| source["id"] == "first")
                        .unwrap();
                    source["id"] = json!("counterfeit source");
                }
                _ => unreachable!(),
            });
            assert!(
                read_discovered_concept_links(&changed, root)
                    .unwrap_err()
                    .contains("stale or counterfeit global concept certificate"),
                "edit {edit}"
            );
        }
    }
}

#[test]
fn legacy_discovery_formats_admit_explicit_matching_and_refuse_implicit_peg_defaults() {
    for matching in [None, Some("generalized"), Some("peg")] {
        let mut grammar = parse_grammar_links(
            "(grammar (format peg) (start root))\n(rule root normal (literal %CF%80))\n",
        )
        .unwrap();
        let mut declarations = grammar.declarations().clone();
        declarations.matching = matching.map(str::to_owned);
        grammar.set_declarations(declarations);
        let source = GlobalConceptSource {
            id: "authored".to_owned(),
            language: "authored".to_owned(),
            edition: String::new(),
            grammar,
        };
        let packet =
            create_discovered_concept_links(&[source], &GlobalConceptOptions::default()).unwrap();
        let exact = read_discovered_concept_links(&packet.network, packet.root).unwrap();
        assert_eq!(
            exact.sources[0].grammar.source_format().unwrap().as_str(),
            "peg"
        );
        let legacy = edit_discovered_envelope(&packet.network, packet.root, |value| {
            for source in value["space"]["sources"].as_array_mut().unwrap() {
                source.as_object_mut().unwrap().remove("sourceFormat");
            }
        });
        if let Some(matching) = matching {
            let restored = read_discovered_concept_links(&legacy, packet.root).unwrap();
            assert_eq!(restored.space, packet.space);
            let tree = parse_with_grammar(
                &restored.sources[0].grammar,
                "π".as_bytes(),
                &FeatureParseOptions::default(),
            )
            .unwrap();
            assert_eq!(discovered_tree_bytes(&tree_value(&tree)), "π".as_bytes());
            let changed = edit_discovered_envelope(&legacy, packet.root, |value| {
                value["space"]["sources"][0]["context"]["matching"] = json!(if matching == "peg" {
                    "generalized"
                } else {
                    "peg"
                });
            });
            assert!(
                read_discovered_concept_links(&changed, packet.root)
                    .unwrap_err()
                    .contains("stale or counterfeit global concept certificate")
            );
        } else {
            assert!(
                read_discovered_concept_links(&legacy, packet.root)
                    .unwrap_err()
                    .contains("stale or counterfeit global concept certificate")
            );
        }
    }
}
