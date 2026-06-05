use meta_language::{
    ByteRange, LinkFlags, LinkMetadata, LinkNetwork, LinkQuery, LinkType, NetworkProjection,
    ParityCapability, ParseConfiguration, Point, RegionDetectionPolicy, SourceSpan,
    SubstitutionRule, TriviaAttachmentPolicy, TruthValue, VerificationIssueKind,
    GRAMMAR_EMBEDDING_TARGETS, LANGUAGE_FIXTURES, MARKUP_LANGUAGE_TARGETS,
    NATURAL_LANGUAGE_TARGETS, PARITY_FIXTURES, PARITY_TARGETS, PROGRAMMING_LANGUAGE_TARGETS,
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
    assert_eq!(parsed.reconstruct_text(), "alpha beta");
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
fn immutable_and_mutable_snapshots_version_network_over_time() {
    let network = LinkNetwork::parse("alpha", "plain-text", ParseConfiguration::default());
    let snapshot = network.snapshot(1, "initial parse");
    let cloned_snapshot = snapshot.clone();

    assert_eq!(snapshot.version(), 1);
    assert_eq!(snapshot.parent_version(), None);
    assert_eq!(snapshot.provenance(), "initial parse");
    assert_eq!(snapshot.shared_snapshot_count(), 2);
    assert_eq!(cloned_snapshot.network().reconstruct_text(), "alpha");

    let mut mutable = snapshot.to_mutable("add shared concept");
    let concept = mutable.network_mut().insert_point("shared concept");

    assert_eq!(mutable.base_version(), 1);
    assert_eq!(mutable.network().find_term("shared concept"), Some(concept));
    assert_eq!(snapshot.network().find_term("shared concept"), None);
    assert_eq!(cloned_snapshot.network().find_term("shared concept"), None);

    let committed = mutable.commit();

    assert_eq!(committed.version(), 2);
    assert_eq!(committed.parent_version(), Some(1));
    assert_eq!(committed.provenance(), "add shared concept");
    assert_eq!(
        committed.network().find_term("shared concept"),
        Some(concept)
    );
    assert_eq!(snapshot.network().reconstruct_text(), "alpha");
}

#[test]
fn reconstruct_text_round_trips_lossless_competitor_sources() {
    for (language, source) in [
        ("Python", "def f(x):\n    return x + 1\n"),
        ("JavaScript", "const x = 1; // keep trivia\n"),
        ("C#", "class C { void M() { } }\n"),
    ] {
        let network = LinkNetwork::parse(source, language, ParseConfiguration::default());

        assert_eq!(network.reconstruct_text(), source);
        assert!(
            network.verify_full_match(None).is_clean(),
            "{language} fixture should parse cleanly"
        );
    }
}

#[test]
fn parse_marks_recovery_errors_without_losing_original_text() {
    let network = LinkNetwork::parse("call(", "JavaScript", ParseConfiguration::default());
    let report = network.verify_full_match(None);

    assert_eq!(network.reconstruct_text(), "call(");
    assert!(!report.is_clean());
    assert!(report
        .issues()
        .iter()
        .any(|issue| issue.kind() == VerificationIssueKind::MissingLink));
}

#[test]
fn mixed_language_regions_are_embedded_in_one_network() {
    let source = "Intro\n```rust\nfn main() {}\n```\n<strong>HTML</strong>\n";
    let network = LinkNetwork::parse(source, "Markdown", ParseConfiguration::default());
    let regions = network.embedded_regions();
    let languages = regions
        .iter()
        .map(meta_language::EmbeddedRegion::language)
        .collect::<Vec<_>>();

    assert_eq!(network.reconstruct_text(), source);
    assert!(languages.contains(&"rust"));
    assert!(languages.contains(&"HTML"));
    assert!(regions
        .iter()
        .all(|region| region.span().byte_range().end() <= source.len()));
}

#[test]
fn content_driven_and_html_region_detection_cover_embedding_targets() {
    let markdown = "# Query\n```\nSELECT 1;\n```\n";
    let markdown_network = LinkNetwork::parse(
        markdown,
        "Markdown",
        ParseConfiguration::default()
            .with_region_detection_policy(RegionDetectionPolicy::ContentDriven),
    );
    let markdown_regions = markdown_network.embedded_regions();
    assert!(markdown_regions
        .iter()
        .map(meta_language::EmbeddedRegion::language)
        .any(|language| language == "SQL"));

    let html = "<script>const x = 1;</script><style>.x { color: red; }</style><p style=\"color: blue\">text</p>";
    let html_network = LinkNetwork::parse(html, "HTML", ParseConfiguration::default());
    let html_regions = html_network.embedded_regions();
    let html_languages = html_regions
        .iter()
        .map(meta_language::EmbeddedRegion::language)
        .collect::<Vec<_>>();

    assert!(html_languages.contains(&"JavaScript"));
    assert_eq!(
        html_languages
            .iter()
            .filter(|language| **language == "CSS")
            .count(),
        2
    );
}

#[test]
fn query_matching_finds_tokens_by_type_term_and_language() {
    let network = LinkNetwork::parse("let x = x + 1", "JavaScript", ParseConfiguration::default());
    let query = LinkQuery::new()
        .with_link_type(LinkType::Token)
        .with_term("x")
        .with_language("JavaScript")
        .with_named(true);

    let matches = network.query_links(&query);

    assert_eq!(matches.len(), 2);
    assert!(matches
        .iter()
        .all(|link| link.metadata().term() == Some("x")));
}

#[test]
fn link_cli_style_substitution_can_create_update_delete_and_swap() {
    let mut network = LinkNetwork::new();
    let one = network.insert_point("1");
    let two = network.insert_point("2");
    let relation = network.insert_link(
        [one, one],
        LinkMetadata::new().with_link_type(LinkType::Relation),
    );

    let update = network.apply_substitution(&SubstitutionRule::new([one, one], [one, two]));
    assert_eq!(update.updated(), &[relation]);
    assert_eq!(
        network.link(relation).expect("updated link").references(),
        &[one, two]
    );

    let create = network.apply_substitution(&SubstitutionRule::create([two, one]));
    assert_eq!(create.created().len(), 1);

    let created = create.created()[0];
    let swap = network.apply_substitution(&SubstitutionRule::new([two, one], [one, two]));
    assert_eq!(swap.updated(), &[created]);
    assert_eq!(
        network.link(created).expect("swapped link").references(),
        &[one, two]
    );

    let delete = network.apply_substitution(&SubstitutionRule::delete([one, two]));
    assert_eq!(delete.deleted().len(), 2);
    assert!(network.link(relation).is_none());
    assert!(network.link(created).is_none());
}

#[test]
fn concept_links_reconstruct_to_target_language_syntax() {
    let mut network = LinkNetwork::self_describing();

    network.insert_concept_mapping("statehood", "English", "Hawaii is a state.");
    network.insert_concept_mapping("statehood", "Spanish", "Hawaii es un estado.");

    assert_eq!(
        network.reconstruct_concept("statehood", "English"),
        Some("Hawaii is a state.")
    );
    assert_eq!(
        network.reconstruct_concept("statehood", "Spanish"),
        Some("Hawaii es un estado.")
    );
    assert!(network
        .projected_links(NetworkProjection::Semantic)
        .any(|link| link.metadata().link_type() == Some(LinkType::Concept)));
}

#[test]
fn object_identity_and_circular_references_are_native_links() {
    let mut network = LinkNetwork::new();
    let root = network.insert_object("root");
    let shared = network.insert_object("shared");

    let left = network.insert_field(root, "left", shared);
    let right = network.insert_field(root, "right", shared);

    assert_eq!(
        network.link(shared).expect("shared").references(),
        &[shared]
    );
    assert_eq!(
        network.link(left).expect("left field").references()[2],
        network.link(right).expect("right field").references()[2]
    );
}

#[test]
fn semantic_truth_values_cover_many_valued_and_paradox_cases() {
    assert_eq!(
        TruthValue::True.and(TruthValue::Unknown),
        TruthValue::Unknown
    );
    assert_eq!(TruthValue::True.and(TruthValue::Both), TruthValue::Both);
    assert_eq!(
        TruthValue::False.or(TruthValue::Unknown),
        TruthValue::Unknown
    );
    assert_eq!(TruthValue::Both.negate(), TruthValue::Both);
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
    assert!(PARITY_TARGETS
        .iter()
        .all(|target| target.test_plan().contains("Executable fixture")));
    assert!(PARITY_TARGETS.iter().any(|target| target
        .capabilities()
        .contains(&ParityCapability::SnapshotVersioning)));
}

#[test]
fn every_parity_target_has_an_executable_fixture_that_passes_core_contract() {
    for target in PARITY_TARGETS {
        assert!(
            PARITY_FIXTURES
                .iter()
                .any(|fixture| fixture.target_name() == target.name()),
            "missing executable fixture for {}",
            target.name()
        );
    }

    for fixture in PARITY_FIXTURES {
        let network = LinkNetwork::parse(
            fixture.source(),
            fixture.language(),
            ParseConfiguration::default(),
        );

        assert_eq!(
            network.reconstruct_text(),
            fixture.expected_reconstruction(),
            "{} fixture failed reconstruction",
            fixture.name()
        );

        for capability in fixture.capabilities() {
            assert!(
                fixture
                    .target()
                    .capabilities()
                    .iter()
                    .any(|target_capability| target_capability == capability),
                "{} fixture advertises capability outside its target",
                fixture.name()
            );
        }
    }
}

#[test]
fn every_parity_target_capability_is_exercised_by_fixtures() {
    for target in PARITY_TARGETS {
        let covered_capabilities = PARITY_FIXTURES
            .iter()
            .filter(|fixture| fixture.target_name() == target.name())
            .flat_map(|fixture| fixture.capabilities().iter().copied())
            .collect::<Vec<_>>();

        for capability in target.capabilities() {
            assert!(
                covered_capabilities.contains(capability),
                "{} target capability is not covered by an executable fixture: {:?}",
                target.name(),
                capability
            );
        }
    }
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

#[test]
fn every_language_target_has_an_executable_lossless_fixture() {
    let target_languages = MARKUP_LANGUAGE_TARGETS
        .iter()
        .chain(PROGRAMMING_LANGUAGE_TARGETS.iter())
        .chain(NATURAL_LANGUAGE_TARGETS.iter())
        .map(meta_language::LanguageTarget::name)
        .collect::<Vec<_>>();

    assert_eq!(LANGUAGE_FIXTURES.len(), target_languages.len());

    for language in &target_languages {
        assert!(
            LANGUAGE_FIXTURES
                .iter()
                .any(|fixture| fixture.language() == *language),
            "missing executable language fixture for {language}"
        );
    }

    for fixture in LANGUAGE_FIXTURES {
        assert!(
            target_languages.contains(&fixture.language()),
            "{} fixture is not tied to a requested language target",
            fixture.language()
        );

        let network = LinkNetwork::parse(
            fixture.source(),
            fixture.language(),
            ParseConfiguration::default(),
        );

        assert_eq!(
            network.reconstruct_text(),
            fixture.source(),
            "{} fixture failed reconstruction",
            fixture.description()
        );
        assert!(
            network.verify_full_match(None).is_clean(),
            "{} fixture should parse cleanly",
            fixture.description()
        );
    }
}
