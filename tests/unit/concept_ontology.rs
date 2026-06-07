use meta_language::{Link, LinkId, LinkNetwork, LinkType, NetworkProjection};

#[test]
fn common_concept_ontology_imports_meta_expression_lexicon() {
    let mut network = LinkNetwork::self_describing();
    let report = network.seed_common_concept_ontology();

    assert_eq!(report.lexicon_concepts(), 351);
    assert!(report.structural_concepts() >= 6);
    assert!(report.syntax_mappings() > report.lexicon_concepts());

    let state = network.find_term("Q35657").expect("Wikidata QID seeded");
    let state_link = network.link(state).expect("state concept link");

    assert_eq!(state_link.references(), &[state]);
    assert_eq!(state_link.metadata().link_type(), Some(LinkType::Concept));
    assert!(state_link
        .metadata()
        .definition()
        .expect("QID definition")
        .contains("Wikidata Q35657"));
    assert_eq!(network.reconstruct_concept("Q35657", "ru"), Some("штат"));
}

#[test]
fn same_meta_expression_concept_reuses_one_link_across_languages() {
    let mut network = LinkNetwork::self_describing();
    let _ = network.seed_common_concept_ontology();

    let apple = network.find_term("Q89").expect("apple concept");
    let language_mappings = ["en", "ru", "hi", "zh"]
        .into_iter()
        .map(|language| semantic_mapping_for(&network, apple, language))
        .collect::<Vec<_>>();

    assert!(language_mappings
        .iter()
        .all(|mapping| mapping.references()[0] == apple));
    assert_eq!(network.reconstruct_concept("Q89", "en"), Some("apple"));
    assert_eq!(network.reconstruct_concept("Q89", "ru"), Some("яблоко"));
}

#[test]
fn structural_concepts_map_to_initial_language_syntax() {
    let mut network = LinkNetwork::self_describing();
    let _ = network.seed_common_concept_ontology();

    let function = network.find_term("function").expect("function concept");
    let rust_function = semantic_mapping_for(&network, function, "Rust");
    let python_function = semantic_mapping_for(&network, function, "Python");
    let javascript_function = semantic_mapping_for(&network, function, "JavaScript");

    assert_eq!(rust_function.references()[0], function);
    assert_eq!(python_function.references()[0], function);
    assert_eq!(javascript_function.references()[0], function);
    assert_eq!(network.reconstruct_concept("function", "Rust"), Some("fn"));
    assert_eq!(network.reconstruct_concept("branch", "Python"), Some("if"));
    assert_eq!(
        network.reconstruct_concept("loop", "JavaScript"),
        Some("for")
    );
}

#[test]
fn semantic_projection_surfaces_seeded_concept_layer() {
    let mut network = LinkNetwork::self_describing();
    let _ = network.seed_common_concept_ontology();

    let semantic_links = network
        .projected_links(NetworkProjection::Semantic)
        .collect::<Vec<_>>();

    assert!(semantic_links.iter().any(|link| {
        link.metadata().link_type() == Some(LinkType::Concept)
            && link.metadata().term() == Some("Q89")
    }));
    assert!(semantic_links.iter().any(|link| {
        link.metadata().link_type() == Some(LinkType::Semantic)
            && link.metadata().term() == Some("apple")
            && link.metadata().language() == Some("en")
    }));
    assert!(semantic_links.iter().any(|link| {
        link.metadata().link_type() == Some(LinkType::Concept)
            && link.metadata().term() == Some("function")
    }));
}

fn semantic_mapping_for<'a>(network: &'a LinkNetwork, concept: LinkId, language: &str) -> &'a Link {
    let language = network.find_term(language).expect("language link");

    network
        .projected_links(NetworkProjection::Semantic)
        .find(|link| {
            link.metadata().link_type() == Some(LinkType::Semantic)
                && link.references().first() == Some(&concept)
                && link.references().get(1) == Some(&language)
        })
        .expect("semantic mapping link")
}
