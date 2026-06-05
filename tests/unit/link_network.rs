use meta_language::{
    ByteRange, LinkFlags, LinkNetwork, LinkType, NetworkProjection, ParseConfiguration, Point,
    RegionDetectionPolicy, SourceSpan, TriviaAttachmentPolicy, VerificationIssueKind,
    GRAMMAR_EMBEDDING_TARGETS, MARKUP_LANGUAGE_TARGETS, NATURAL_LANGUAGE_TARGETS, PARITY_TARGETS,
    PROGRAMMING_LANGUAGE_TARGETS,
};

#[test]
fn point_is_a_self_referential_link() {
    let mut network = LinkNetwork::new();

    let point = network.insert_point("concept");
    let link = network.link(point).expect("point link exists");

    assert_eq!(link.references(), &[point]);
    assert_eq!(link.metadata().link_type(), Some(LinkType::Concept));
    assert!(link.metadata().is_named());
}

#[test]
fn relation_and_field_links_are_regular_links() {
    let mut network = LinkNetwork::new();
    let parent = network.insert_point("function");
    let child = network.insert_point("identifier");

    let relation = network.insert_relation(
        [parent, child],
        LinkType::Syntax,
        SourceSpan::new(ByteRange::new(0, 8), Point::new(0, 0), Point::new(0, 8)),
    );
    let field = network.insert_field(parent, "name", child);

    assert_eq!(
        network.link(relation).expect("relation").references(),
        &[parent, child]
    );
    assert_eq!(
        network
            .link(field)
            .expect("field link")
            .metadata()
            .link_type(),
        Some(LinkType::Field)
    );

    let field_references = network.link(field).expect("field link").references();
    assert_eq!(field_references[0], parent);
    assert_eq!(field_references[2], child);
    assert_eq!(
        network
            .link(field_references[1])
            .expect("field label")
            .metadata()
            .term(),
        Some("name")
    );
}

#[test]
fn verification_reports_error_and_missing_links_in_requested_region() {
    let mut network = LinkNetwork::new();
    let clean = network.insert_point("clean");
    let error = network.insert_point("error");
    let missing = network.insert_point("missing");

    network.set_span(
        clean,
        SourceSpan::new(ByteRange::new(0, 4), Point::new(0, 0), Point::new(0, 4)),
    );
    network.set_span(
        error,
        SourceSpan::new(ByteRange::new(5, 10), Point::new(0, 5), Point::new(0, 10)),
    );
    network.set_span(
        missing,
        SourceSpan::new(ByteRange::new(20, 20), Point::new(0, 20), Point::new(0, 20)),
    );
    network.set_flags(error, LinkFlags::error());
    network.set_flags(missing, LinkFlags::missing());

    let local_report = network.verify_full_match(Some(ByteRange::new(0, 12)));
    assert!(!local_report.is_clean());
    assert_eq!(local_report.issues().len(), 1);
    assert_eq!(
        local_report.issues()[0].kind(),
        VerificationIssueKind::ErrorLink
    );

    let full_report = network.verify_full_match(None);
    assert_eq!(full_report.issues().len(), 2);
    assert_eq!(
        full_report
            .issues()
            .iter()
            .map(meta_language::VerificationIssue::kind)
            .collect::<Vec<_>>(),
        vec![
            VerificationIssueKind::ErrorLink,
            VerificationIssueKind::MissingLink,
        ]
    );
}

#[test]
fn parse_configuration_can_attach_trivia_with_either_or_both_policies() {
    assert_eq!(
        ParseConfiguration::default().trivia_attachment_policy(),
        TriviaAttachmentPolicy::Both
    );
    assert_eq!(
        ParseConfiguration::default().region_detection_policy(),
        RegionDetectionPolicy::Both
    );

    let containment = ParseConfiguration::new(TriviaAttachmentPolicy::ContainmentLink);
    let token = ParseConfiguration::new(TriviaAttachmentPolicy::TokenLink);

    assert_eq!(
        containment.trivia_attachment_policy(),
        TriviaAttachmentPolicy::ContainmentLink
    );
    assert_eq!(
        token.trivia_attachment_policy(),
        TriviaAttachmentPolicy::TokenLink
    );
}

#[test]
fn self_description_contains_common_roots() {
    let network = LinkNetwork::self_describing();

    for term in [
        "link",
        "reference",
        "relation link",
        "language",
        "grammar",
        "type",
        "concept",
        "point",
    ] {
        assert!(network.find_term(term).is_some(), "missing term: {term}");
    }

    let link = network.find_term("link").expect("link term");
    let definition = network.definition_for(link).expect("link definition");
    assert!(definition.contains("n-tuple of references"));
}

#[test]
fn parse_is_lossless_by_default_and_matches_explicit_lossless_boundary() {
    let parsed = LinkNetwork::parse("alpha beta", "plain-text", ParseConfiguration::default());
    let explicit =
        LinkNetwork::parse_lossless_text("alpha beta", "plain-text", ParseConfiguration::default());

    assert_eq!(parsed, explicit);
    assert!(parsed
        .projected_links(NetworkProjection::Lossless)
        .any(|link| link.metadata().flags().is_extra()));
    assert!(parsed
        .projected_links(NetworkProjection::ConcreteSyntax)
        .any(|link| link.metadata().link_type() == Some(LinkType::Trivia)));
    assert!(parsed
        .projected_links(NetworkProjection::AbstractSyntax)
        .all(|link| !matches!(
            link.metadata().link_type(),
            Some(LinkType::Token | LinkType::Trivia)
        )));
}

#[test]
fn projections_strip_lower_level_data_without_mutating_the_network() {
    let network = LinkNetwork::parse("a b", "plain-text", ParseConfiguration::default());
    let lossless_count = network.projected_links(NetworkProjection::Lossless).count();
    let syntax_count = network
        .projected_links(NetworkProjection::ConcreteSyntax)
        .count();
    let abstract_count = network
        .projected_links(NetworkProjection::AbstractSyntax)
        .count();

    assert_eq!(lossless_count, network.len());
    assert_eq!(syntax_count, network.len());
    assert!(abstract_count < lossless_count);
    assert_eq!(NetworkProjection::AbstractSyntax.label(), "abstract syntax");
}

#[test]
fn parity_targets_track_competitor_and_ecosystem_test_sources() {
    let target_names = PARITY_TARGETS
        .iter()
        .map(meta_language::ParityTarget::name)
        .collect::<Vec<_>>();

    for expected in [
        "tree-sitter",
        "LibCST",
        "Recast",
        "jscodeshift",
        "Rowan",
        "cstree",
        "Roslyn",
        "links-notation",
        "link-cli",
        "lino-objects-codec",
        "relative-meta-logic",
        "formal-ai",
        "meta-expression",
    ] {
        assert!(
            target_names.contains(&expected),
            "missing parity target: {expected}"
        );
    }

    assert!(PARITY_TARGETS
        .iter()
        .all(|target| !target.capabilities().is_empty()));
    assert!(
        PARITY_TARGETS
            .iter()
            .all(|target| target.test_plan().contains("Port")
                || target.test_plan().contains("Replay"))
    );
}

#[test]
fn language_targets_cover_markup_programming_natural_and_embedding_scope() {
    assert_eq!(MARKUP_LANGUAGE_TARGETS.len(), 2);
    assert_eq!(PROGRAMMING_LANGUAGE_TARGETS.len(), 10);
    assert_eq!(NATURAL_LANGUAGE_TARGETS.len(), 10);

    let markup_names = MARKUP_LANGUAGE_TARGETS
        .iter()
        .map(meta_language::LanguageTarget::name)
        .collect::<Vec<_>>();
    assert!(markup_names.contains(&"Markdown"));
    assert!(markup_names.contains(&"HTML"));

    assert!(GRAMMAR_EMBEDDING_TARGETS.iter().any(|target| {
        target.host_language() == "Markdown"
            && target.embedded_language() == "Programming language region"
    }));
    assert!(GRAMMAR_EMBEDDING_TARGETS.iter().any(|target| {
        target.host_language() == "HTML" && target.embedded_language() == "JavaScript"
    }));

    assert!(PROGRAMMING_LANGUAGE_TARGETS
        .iter()
        .all(|target| target.basis().contains("TIOBE May 2026")));
    assert!(NATURAL_LANGUAGE_TARGETS
        .iter()
        .all(|target| target.basis().contains("Ethnologue/Britannica")));
}
