use meta_language::{
    FormalizationLevel, LinkMetadata, LinkNetwork, LinkQuery, LinkType, ParseConfiguration,
    TranslationRule, TranslationRuleRegistry, TranslationRuleSet,
};

#[test]
fn user_supplied_rules_drive_cross_language_reconstruction() {
    let mut network = LinkNetwork::new();
    let proposition = network.insert_concept_expression("capital", "English", "capital");
    let france = network.insert_concept_expression("Q142", "English", "France");
    network.insert_concept_expression("Q142", "Spanish", "Francia");
    let paris = network.insert_concept_expression("Q90", "English", "Paris");
    network.insert_concept_expression("Q90", "Spanish", "Paris");
    network.insert_link(
        [proposition, france, paris],
        LinkMetadata::new()
            .with_link_type(LinkType::Semantic)
            .with_named(true)
            .with_term("proposition:capital"),
    );
    let rules = TranslationRuleSet::new("capital-demo").with_rule(
        TranslationRule::new(
            "capital sentence",
            LinkQuery::by_type(LinkType::Semantic).with_term("proposition:capital"),
        )
        .with_reference_capture("subject", 1)
        .with_reference_capture("object", 2)
        .with_template("Spanish", "{object} es la capital de {subject}."),
    );

    assert_eq!(
        network.reconstruct_text_as_with_rules("Spanish", ParseConfiguration::default(), &rules,),
        "Paris es la capital de Francia."
    );
}

#[test]
fn translation_rule_registry_can_replace_the_active_rule_set() {
    let mut network = LinkNetwork::new();
    let concept = network.insert_concept_expression("greeting", "English", "hello");
    network.insert_link(
        [concept],
        LinkMetadata::new()
            .with_link_type(LinkType::Semantic)
            .with_named(true)
            .with_term("proposition:greeting"),
    );
    let english = TranslationRuleSet::new("greetings").with_rule(
        TranslationRule::new(
            "english greeting",
            LinkQuery::by_type(LinkType::Semantic).with_term("proposition:greeting"),
        )
        .with_template("English", "hello"),
    );
    let spanish = TranslationRuleSet::new("greetings").with_rule(
        TranslationRule::new(
            "spanish greeting",
            LinkQuery::by_type(LinkType::Semantic).with_term("proposition:greeting"),
        )
        .with_template("Spanish", "hola"),
    );
    let mut registry = TranslationRuleRegistry::new().with_rule_set(english);

    assert_eq!(
        network.reconstruct_text_as_with_registry(
            "English",
            ParseConfiguration::default(),
            &registry,
        ),
        "hello"
    );

    registry.replace_rule_set(spanish);

    assert_eq!(
        network.reconstruct_text_as_with_registry(
            "Spanish",
            ParseConfiguration::default(),
            &registry,
        ),
        "hola"
    );
}

#[test]
fn statehood_demo_is_available_as_a_loadable_rule_set() {
    let rules = TranslationRuleSet::from_lino(TranslationRuleSet::statehood_demo_lino())
        .expect("statehood rule set loads from LiNo");
    let network = LinkNetwork::parse(
        "Hawaii is a state.\n",
        "English",
        ParseConfiguration::default(),
    );

    assert_eq!(
        network.reconstruct_text_as_with_rules("Russian", ParseConfiguration::default(), &rules,),
        "Гавайи это штат.\n"
    );
    assert_eq!(
        network.reconstruct_text_as_with_rules(
            "Russian",
            ParseConfiguration::default().with_formalization_level(FormalizationLevel::Concept),
            &rules,
        ),
        "statehood(Q782, Q35657)\n"
    );
}

#[test]
fn missing_translation_rules_record_diagnostic_links() {
    let mut network = LinkNetwork::new();
    let concept = network.insert_point("unmatched");
    let semantic = network.insert_link(
        [concept],
        LinkMetadata::new()
            .with_link_type(LinkType::Semantic)
            .with_named(true)
            .with_term("proposition:unmatched"),
    );
    let rules = TranslationRuleSet::new("empty");

    assert_eq!(
        network.reconstruct_text_as_with_rules_mut(
            "Spanish",
            ParseConfiguration::default(),
            &rules,
        ),
        ""
    );

    let diagnostic = network
        .links()
        .find(|link| {
            link.metadata().link_type() == Some(LinkType::Semantic)
                && link.metadata().term() == Some("translation-rule:missing")
        })
        .expect("missing translation produces a diagnostic link");
    assert_eq!(diagnostic.references(), &[semantic]);
    assert_eq!(diagnostic.metadata().language(), Some("Spanish"));
    assert!(diagnostic
        .metadata()
        .definition()
        .expect("diagnostic names the unmatched structure")
        .contains("proposition:unmatched"));
}

#[test]
fn translation_rule_sets_round_trip_through_lino() {
    let rules = TranslationRuleSet::new("capital-demo").with_rule(
        TranslationRule::new(
            "capital sentence",
            LinkQuery::by_type(LinkType::Semantic)
                .with_term("proposition:capital")
                .with_named(true),
        )
        .with_reference_capture("subject", 1)
        .with_reference_capture("object", 2)
        .with_template("English", "{object} is the capital of {subject}.")
        .with_template("Spanish", "{object} es la capital de {subject}."),
    );

    let lino = rules.to_lino();
    let restored =
        TranslationRuleSet::from_lino(&lino).expect("serialized rule set must load from LiNo");

    assert_eq!(restored, rules);
    assert_eq!(restored.to_lino(), lino);
}
