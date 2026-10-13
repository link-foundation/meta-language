//! Shared source-free RDF abstract graph assertions and malformed premise cases.

use meta_language::{
    LinkId, LinkNetwork, ResourceDescriptionLinks, decode_resource_description_graph,
    decode_resource_description_graph_with_limit, import_resource_description_graph,
    merge_resource_description_graphs, percent_encode_links_text, read_structured_data_lino,
    verify_resource_description_graph_witnesses,
};
use serde_json::{Value, json};

fn schema() -> Value {
    read_structured_data_lino(include_str!(
        "../../src/data/resource-description-schema.lino"
    ))
    .unwrap()
}

fn fixture() -> Value {
    read_structured_data_lino(include_str!(
        "../../../parity/fixtures/resource-description-graph-expectations.lino"
    ))
    .unwrap()
}

fn role(network: &LinkNetwork, kind: &str) -> Vec<LinkId> {
    let data = schema();
    network
        .links()
        .filter(|link| link.metadata().term() == data["terms"][kind].as_str())
        .map(|link| link.id())
        .collect()
}

fn reload(graph: &ResourceDescriptionLinks) -> ResourceDescriptionLinks {
    ResourceDescriptionLinks {
        network: LinkNetwork::from_lino(&graph.network.to_lino()).unwrap(),
        root: graph.root,
    }
}

fn decode(graph: &ResourceDescriptionLinks) -> Value {
    decode_resource_description_graph(&graph.network, graph.root).unwrap()
}

fn statements(source: &str, triples: Value) -> ResourceDescriptionLinks {
    import_resource_description_graph(&triples, source).unwrap()
}

fn identifier(text: &str) -> Value {
    json!({"kind":"identifier","value":text})
}
fn literal(text: &str) -> Value {
    json!({"kind":"literal","value":text})
}
fn triple(subject: Value, object: Value) -> Value {
    json!([subject, identifier("urn:property"), object])
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

fn replace_definition(network: &LinkNetwork, id: LinkId, before: &str, after: &str) -> LinkNetwork {
    let text = network
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
    LinkNetwork::from_lino(&text).unwrap()
}

#[test]
fn shared_resource_graph_reloads_and_matches_native_import_merge() {
    let data = fixture();
    let archived = ResourceDescriptionLinks {
        network: LinkNetwork::from_lino(include_str!(
            "../../../parity/fixtures/resource-description-graph.lino"
        ))
        .unwrap(),
        root: LinkId::from_u64(data["root"].as_u64().unwrap()),
    };
    assert_eq!(decode(&archived), data["decoded"]);
    assert!(verify_resource_description_graph_witnesses(&archived.network, archived.root).unwrap());
    let graphs = data["inputs"]
        .as_array()
        .unwrap()
        .iter()
        .map(|input| {
            statements(
                input["sourceIdentifier"].as_str().unwrap(),
                input["triples"].clone(),
            )
        })
        .collect::<Vec<_>>();
    let merged = merge_resource_description_graphs(&graphs).unwrap();
    assert_eq!(decode(&reload(&merged)), data["decoded"]);
    assert_eq!(decode(&merged)["triples"].as_array().unwrap().len(), 5);
    assert_eq!(decode(&merged)["occurrences"].as_array().unwrap().len(), 7);
}

#[test]
fn source_free_unicode_terms_keep_ill_typed_lexical_forms_and_language_provenance() {
    let graph = statements(
        "source A",
        json!([
            triple(
                identifier("urn:subject"),
                json!({"kind":"literal","value":"hello","language":"EN-gb"})
            ),
            triple(
                json!({"kind":"blank","value":"same"}),
                json!({"kind":"literal","value":"1","datatype":"http://www.w3.org/2001/XMLSchema#integer"})
            ),
            triple(
                identifier("urn:subject"),
                json!({"kind":"literal","value":"01","datatype":"http://www.w3.org/2001/XMLSchema#integer"})
            ),
            triple(identifier("urn:unicode:🦀"), literal("空\0\n")),
            triple(
                identifier("urn:subject"),
                json!({"kind":"literal","value":"ill-typed","datatype":"http://www.w3.org/2001/XMLSchema#integer"})
            )
        ]),
    );
    let result = decode(&reload(&graph));
    assert_eq!(result, decode(&graph));
    let triples = result["triples"].as_array().unwrap();
    assert_eq!(triples.len(), 5);
    assert!(triples.iter().any(|triple| triple[2]["value"] == "01"));
    assert!(
        triples
            .iter()
            .any(|triple| triple[2]["value"] == "ill-typed")
    );
    assert!(
        triples
            .iter()
            .any(|triple| triple[0]["sourceIdentifier"] == "source A")
    );
    assert!(
        triples
            .iter()
            .any(|triple| triple[2]["language"] == "en-gb")
    );
    assert!(
        result["occurrences"]
            .as_array()
            .unwrap()
            .iter()
            .any(|value| value["sourceLanguage"] == "EN-gb")
    );
}

#[test]
fn actual_duplicate_statement_and_term_links_keep_ordered_occurrences() {
    let statement = triple(identifier("urn:subject"), literal("same"));
    let graph = statements("source A", json!([statement, statement]));
    assert_eq!(role(&graph.network, "statement").len(), 1);
    assert_eq!(role(&graph.network, "literal").len(), 1);
    assert_eq!(role(&graph.network, "occurrence").len(), 2);
    let result = decode(&reload(&graph));
    assert_eq!(result["triples"].as_array().unwrap().len(), 1);
    assert_eq!(
        result["occurrences"]
            .as_array()
            .unwrap()
            .iter()
            .map(|value| value["position"].as_u64().unwrap())
            .collect::<Vec<_>>(),
        vec![0, 1]
    );
}

#[test]
fn actual_linked_lexical_edit_changes_decoding_after_reload() {
    let graph = statements(
        "source A",
        json!([triple(identifier("urn:subject"), literal("before"))]),
    );
    let term = graph
        .network
        .link(role(&graph.network, "literal")[0])
        .unwrap();
    let edited = ResourceDescriptionLinks {
        network: replace_definition(&graph.network, term.references()[0], "before", "after"),
        root: graph.root,
    };
    assert_eq!(decode(&reload(&edited))["triples"][0][2]["value"], "after");
}

#[test]
fn scope_and_exact_literal_identity_preserve_owl_axioms_without_entailing_identity() {
    let data = fixture();
    let graphs = data["inputs"]
        .as_array()
        .unwrap()
        .iter()
        .map(|input| {
            statements(
                input["sourceIdentifier"].as_str().unwrap(),
                input["triples"].clone(),
            )
        })
        .collect::<Vec<_>>();
    let merged = reload(&merge_resource_description_graphs(&graphs).unwrap());
    let result = decode(&merged);
    let triples = result["triples"].as_array().unwrap();
    let blanks = triples
        .iter()
        .filter(|triple| triple[0]["kind"] == "blank")
        .collect::<Vec<_>>();
    assert_eq!(blanks.len(), 2);
    assert!(
        blanks
            .iter()
            .any(|triple| triple[0]["sourceIdentifier"] == "source A" && triple[2]["value"] == "1")
    );
    assert!(blanks.iter().any(|triple| triple[0]["sourceIdentifier"] == "source B" && triple[2]["value"] == "01"));
    let axiom = triples
        .iter()
        .find(|triple| triple[1]["value"] == "http://www.w3.org/2002/07/owl#sameAs")
        .unwrap();
    assert_eq!(axiom[0]["value"], "urn:left");
    assert_eq!(axiom[2]["value"], "urn:right");
    assert!(!role(&merged.network, "witness").is_empty());
    assert!(verify_resource_description_graph_witnesses(&merged.network, merged.root).unwrap());
}

#[test]
fn counterfeit_witness_dangling_reference_and_foreign_model_are_rejected() {
    let graph = statements(
        "source A",
        json!([triple(identifier("urn:subject"), literal("one"))]),
    );
    let merged = merge_resource_description_graphs(&[graph.clone()]).unwrap();
    let witness = merged
        .network
        .link(role(&merged.network, "witness")[0])
        .unwrap();
    let target = *role(&merged.network, "literal").last().unwrap();
    let counterfeit = replace_references(
        &merged.network,
        witness.id(),
        &[witness.references()[0], target, witness.references()[2]],
    );
    assert!(
        decode_resource_description_graph(&counterfeit, merged.root)
            .unwrap_err()
            .contains("witness")
    );
    let statement = graph
        .network
        .link(role(&graph.network, "statement")[0])
        .unwrap();
    let dangling = replace_references(
        &graph.network,
        statement.id(),
        &[
            LinkId::from_u64(999_999),
            statement.references()[1],
            statement.references()[2],
        ],
    );
    assert!(decode_resource_description_graph(&dangling, graph.root).is_err());
    let foreign = replace_definition(
        &graph.network,
        role(&graph.network, "model")[0],
        schema()["model"].as_str().unwrap(),
        "OWL identity entailment",
    );
    assert!(
        decode_resource_description_graph(&foreign, graph.root)
            .unwrap_err()
            .contains("premises")
    );
    let mut concealed = graph.network.clone();
    concealed.insert_field(graph.root, "unexpected semantic field", graph.root);
    assert!(
        decode_resource_description_graph(&concealed, graph.root)
            .unwrap_err()
            .contains("conceal")
    );
}

#[test]
fn role_fields_datatype_and_language_violations_refuse_import() {
    let bad = [
        json!([
            literal("subject"),
            identifier("urn:property"),
            literal("object")
        ]),
        json!([identifier("urn:subject"), {"kind":"blank","value":"property"}, literal("object")]),
        triple(identifier("urn:subject"), identifier("relative/path")),
        triple(
            identifier("urn:subject"),
            json!({"kind":"literal","value":"hi","datatype":schema()["stringDatatype"],"language":"en"}),
        ),
        triple(
            identifier("urn:subject"),
            json!({"kind":"literal","value":"hi","datatype":schema()["languageDatatype"]}),
        ),
        triple(
            identifier("urn:subject"),
            json!({"kind":"literal","value":"hi","language":"en-a"}),
        ),
        triple(
            identifier("urn:subject"),
            json!({"kind":"identifier","value":"urn:a","opaquePayload":"{}"}),
        ),
    ];
    for triple in bad {
        assert!(import_resource_description_graph(&json!([triple]), "source A").is_err());
    }
    assert!(
        import_resource_description_graph(
            &json!([triple(json!({"kind":"blank","value":"x"}), literal("x"))]),
            ""
        )
        .is_err()
    );
}

#[test]
fn lexical_rules_keep_spelling_and_refuse_malformed_or_unsupported_inputs() {
    for value in [
        "urn:a",
        "https://example.test/空?x=🦀#part",
        "urn:a%20b",
        "scheme:",
    ] {
        let graph = statements(
            "source A",
            json!([triple(identifier(value), literal("value"))]),
        );
        assert_eq!(decode(&graph)["triples"][0][0]["value"], value);
    }
    for value in [
        "relative",
        "urn:a b",
        "urn:a%GG",
        "https://user@other@example.test/a",
        "http://[::1]/",
        "urn:a#b#c",
    ] {
        assert!(
            import_resource_description_graph(
                &json!([triple(identifier(value), literal("value"))]),
                "source A"
            )
            .is_err(),
            "{value}"
        );
    }
    for language in [
        "en",
        "en-US",
        "zh-Hant-TW",
        "de-CH-1901",
        "en-a-example-x-private",
        "x-private",
    ] {
        statements(
            "source A",
            json!([triple(
                identifier("urn:subject"),
                json!({"kind":"literal","value":"text","language":language})
            )]),
        );
    }
    for language in [
        "e",
        "en-",
        "en-a",
        "en-a-one-a-two",
        "sl-rozaj-rozaj",
        "en-💬",
    ] {
        assert!(
            import_resource_description_graph(
                &json!([triple(
                    identifier("urn:subject"),
                    json!({"kind":"literal","value":"text","language":language})
                )]),
                "source A"
            )
            .is_err(),
            "{language}"
        );
    }
}

#[test]
fn source_cycles_and_explicit_structural_budgets_refuse_promptly() {
    let graph = statements(
        "source A",
        json!([triple(identifier("urn:subject"), literal("same"))]),
    );
    assert!(
        decode_resource_description_graph_with_limit(&graph.network, graph.root, 2)
            .unwrap_err()
            .contains("limit")
    );
    assert!(decode_resource_description_graph_with_limit(&graph.network, graph.root, 0).is_err());
    let sources = graph.network.link(graph.root).unwrap().references()[3];
    let malformed = replace_references(&graph.network, sources, &[graph.root]);
    assert!(
        decode_resource_description_graph(&malformed, graph.root)
            .unwrap_err()
            .contains("cyclic")
    );
}
