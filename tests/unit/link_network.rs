use meta_language::{
    ByteRange, LinkFlags, LinkNetwork, LinkType, ParseConfiguration, Point, RegionDetectionPolicy,
    SourceSpan, TriviaAttachmentPolicy, VerificationIssueKind,
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
