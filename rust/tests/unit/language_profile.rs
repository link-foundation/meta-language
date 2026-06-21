use meta_language::{
    LanguageProfile, LinkMetadata, LinkNetwork, LinkQuery, LinkType, ParseConfiguration,
    ReplacementRule, TranslationRule, TranslationRuleSet,
};

#[test]
fn language_profiles_are_declared_as_queryable_links() {
    let mut network = LinkNetwork::new();
    let profile = LanguageProfile::new("Custom", "custom")
        .with_link_type(LinkType::Semantic)
        .with_concept("proposition:custom")
        .with_translation_rule("custom render");

    let links = profile.declare_in(&mut network);

    assert!(network.link(links.profile()).is_some());
    assert_eq!(
        network
            .query_links(&LinkQuery::by_type(LinkType::Semantic).with_term("language-profile"))
            .len(),
        1
    );
    assert_eq!(
        network
            .query_links(
                &LinkQuery::by_type(LinkType::Semantic).with_term("language-profile:link-type")
            )
            .len(),
        1
    );
    assert_eq!(
        network
            .query_links(
                &LinkQuery::by_type(LinkType::Semantic).with_term("language-profile:concept")
            )
            .len(),
        1
    );
    assert_eq!(
        network
            .query_links(
                &LinkQuery::by_type(LinkType::Semantic)
                    .with_term("language-profile:translation-rule")
            )
            .len(),
        1
    );
}

#[test]
fn parse_configuration_can_materialize_the_javascript_profile() {
    let network = LinkNetwork::parse(
        "const value = 1;\n",
        "JavaScript",
        ParseConfiguration::default().with_profile("JavaScript"),
    );

    let profile_links =
        network.query_links(&LinkQuery::by_type(LinkType::Semantic).with_term("language-profile"));
    assert_eq!(profile_links.len(), 1);
    assert_eq!(profile_links[0].metadata().language(), Some("JavaScript"));
    assert_eq!(profile_links[0].metadata().definition(), Some("JavaScript"));
}

#[test]
fn profiled_javascript_to_javascript_transform_allows_in_profile_rewrite() {
    let source = "const oldName = call(oldName);\n// oldName in comments stays\n";
    let mut network = LinkNetwork::parse(
        source,
        "JavaScript",
        ParseConfiguration::default().with_profile("JavaScript"),
    );
    let query = LinkQuery::from_sexpression(
        r#"
        (identifier) @target
        (#eq? @target "oldName")
        "#,
    )
    .expect("query parses");
    let captures = network.find(&query);

    let report = network.replace_with_profile(
        &captures,
        &ReplacementRule::captured_text("target", "renamedValue"),
        &LanguageProfile::javascript(),
    );

    assert_eq!(report.text_replacements().len(), 2);
    assert!(report.profile_diagnostics().is_empty());
    assert_eq!(
        network.reconstruct_text(),
        "const renamedValue = call(renamedValue);\n// oldName in comments stays\n"
    );
}

#[test]
fn profiled_javascript_to_javascript_transform_rejects_unsupported_syntax() {
    let source = "const result = 1 + keep;\n";
    let mut network = LinkNetwork::parse(
        source,
        "JavaScript",
        ParseConfiguration::default().with_profile("JavaScript"),
    );
    let query = LinkQuery::from_sexpression(
        r#"
        (number) @literal
        (#eq? @literal "1")
        "#,
    )
    .expect("query parses");
    let captures = network.find(&query);

    let report = network.replace_with_profile(
        &captures,
        &ReplacementRule::captured_text("literal", "match value { _ => 1 }"),
        &LanguageProfile::javascript(),
    );

    assert!(report.text_replacements().is_empty());
    assert_eq!(network.reconstruct_text(), source);
    assert_eq!(report.profile_diagnostics().len(), 1);

    let diagnostic = network
        .link(report.profile_diagnostics()[0])
        .expect("profile violation diagnostic exists");
    assert_eq!(
        diagnostic.metadata().term(),
        Some("language-profile:unsupported-feature")
    );
    assert_eq!(diagnostic.metadata().language(), Some("JavaScript"));
    assert!(diagnostic
        .metadata()
        .definition()
        .expect("diagnostic describes unsupported syntax")
        .contains("JavaScript"));
    assert_eq!(
        network
            .query_links(
                &LinkQuery::by_type(LinkType::Semantic)
                    .with_term("language-profile:unsupported-feature")
            )
            .len(),
        1
    );
}

#[test]
fn profiles_can_be_computed_from_translation_rule_set_domains() {
    let rules = TranslationRuleSet::new("capital-demo").with_rule(
        TranslationRule::new(
            "capital sentence",
            LinkQuery::by_type(LinkType::Semantic).with_term("proposition:capital"),
        )
        .with_template("JavaScript", "capital({subject}, {object})"),
    );

    let profile = LanguageProfile::from_rule_set("JavaScript", "JavaScript", &rules);

    assert!(profile.supports_link_type(LinkType::Semantic));
    assert!(profile.supports_concept("proposition:capital"));
    assert!(profile.supports_translation_rule("capital sentence"));

    let mut network = LinkNetwork::new();
    let semantic = network.insert_link(
        [],
        LinkMetadata::new()
            .with_link_type(LinkType::Semantic)
            .with_term("proposition:capital"),
    );

    assert!(profile.validate_network(&network).is_ok());
    assert_eq!(
        network.link(semantic).expect("semantic link").id(),
        semantic
    );

    network.insert_link(
        [],
        LinkMetadata::new()
            .with_link_type(LinkType::Semantic)
            .with_term("proposition:population"),
    );
    let violation = profile
        .validate_network(&network)
        .expect_err("undeclared rule-set domain term is rejected");
    assert!(violation.feature().contains("proposition:population"));
}
