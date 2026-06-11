use meta_language::{Link, LinkId, LinkNetwork, LinkQuery, LinkType, NetworkProjection};

#[test]
fn common_concept_ontology_imports_meta_expression_lexicon() {
    let mut network = LinkNetwork::self_describing();
    let report = network.seed_common_concept_ontology();

    assert_eq!(report.lexicon_concepts(), 351);
    assert!(report.structural_concepts() >= 6);
    assert!(report.alias_links() > 0);
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
    assert_eq!(
        network
            .query_links(
                &LinkQuery::by_type(LinkType::Semantic)
                    .with_language("Wikidata")
                    .with_term("Q35657")
            )
            .first()
            .map(|link| link.references()[0]),
        Some(state)
    );
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
fn concept_interning_reuses_only_exact_ids() {
    let mut network = LinkNetwork::self_describing();

    let resume = network.intern_concept("lex:en:resume", Some("English resume noun."));
    let same_resume = network.intern_concept("lex:en:resume", Some("Updated definition."));
    let case_miss = network.intern_concept("lex:en:Resume", None);
    let diacritic_miss = network.intern_concept("lex:en:résumé", None);
    let sense_miss = network.intern_concept("lex:en:resume#verb", None);

    assert_eq!(same_resume, resume);
    assert_ne!(case_miss, resume);
    assert_ne!(diacritic_miss, resume);
    assert_ne!(sense_miss, resume);
    assert_eq!(network.find_term("lex:en:resume"), Some(resume));
    assert_eq!(
        network
            .link(resume)
            .expect("resume concept")
            .metadata()
            .definition(),
        Some("Updated definition.")
    );
}

#[test]
fn concept_expression_links_share_language_free_concept() {
    let mut network = LinkNetwork::self_describing();

    let english = network.insert_concept_expression("concept:light", "en", "light");
    let spanish = network.insert_concept_expression("concept:light", "es", "luz");
    let concept = network
        .find_term("concept:light")
        .expect("language-free concept");

    assert_eq!(
        network
            .link(english)
            .expect("English expression")
            .references()
            .first(),
        Some(&concept)
    );
    assert_eq!(
        network
            .link(spanish)
            .expect("Spanish expression")
            .references()
            .first(),
        Some(&concept)
    );
    assert_eq!(
        network.reconstruct_concept("concept:light", "en"),
        Some("light")
    );
    assert_eq!(
        network.reconstruct_concept("concept:light", "es"),
        Some("luz")
    );
}

#[test]
fn external_concept_aliases_are_queryable_without_becoming_concept_ids() {
    let mut network = LinkNetwork::self_describing();
    let apple = network.intern_concept("concept:apple-fruit", Some("Apple fruit concept."));

    let wikidata_alias = network.insert_concept_alias(apple, "Wikidata", "Q89");
    let cili_alias = network.insert_concept_alias(apple, "WordNet CILI", "ili:00001740-n");

    assert_eq!(network.find_term("Q89"), None);
    assert_eq!(
        network
            .query_links(
                &LinkQuery::by_type(LinkType::Semantic)
                    .with_language("Wikidata")
                    .with_term("Q89")
            )
            .into_iter()
            .map(Link::id)
            .collect::<Vec<_>>(),
        vec![wikidata_alias]
    );
    assert_eq!(
        network
            .query_links(
                &LinkQuery::by_type(LinkType::Semantic)
                    .with_language("WordNet CILI")
                    .with_term("ili:00001740-n")
            )
            .into_iter()
            .map(Link::id)
            .collect::<Vec<_>>(),
        vec![cili_alias]
    );
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

#[test]
fn concept_ontology_imports_from_lino_round_trip() {
    let mut source = LinkNetwork::new();
    let concept = source.intern_concept(
        "concept:equity#finance",
        Some("Finance sense of equity; exact id carries the sense."),
    );
    source.insert_concept_expression("concept:equity#finance", "en", "equity");
    source.insert_concept_expression("concept:equity#finance", "es", "patrimonio");
    source.insert_concept_alias(concept, "Wikidata", "Q430265");
    source.insert_concept_alias(concept, "WordNet CILI", "ili:13371337-n");

    let lino = source.to_lino();
    let mut imported = LinkNetwork::new();
    let report = imported
        .import_concept_ontology_lino(&lino)
        .expect("concept set LiNo imports");
    let imported_concept = imported
        .find_term("concept:equity#finance")
        .expect("imported concept");

    assert_eq!(report.concepts(), 1);
    assert_eq!(report.syntax_mappings(), 2);
    assert_eq!(report.alias_links(), 2);
    assert_eq!(
        imported.reconstruct_concept("concept:equity#finance", "es"),
        Some("patrimonio")
    );
    assert_eq!(
        imported
            .query_links(
                &LinkQuery::by_type(LinkType::Semantic)
                    .with_language("Wikidata")
                    .with_term("Q430265")
            )
            .first()
            .map(|link| link.references()[0]),
        Some(imported_concept)
    );

    let len_after_first_import = imported.len();
    let second_report = imported
        .import_concept_ontology_lino(&lino)
        .expect("concept set import is idempotent");
    assert_eq!(second_report, report);
    assert_eq!(imported.len(), len_after_first_import);
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
