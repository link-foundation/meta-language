use meta_language::{
    ByteRange, LanguageIdentificationDetector, LinkFlags, LinkId, LinkMetadata, LinkNetwork,
    LinkQuery, LinkType, NetworkProjection, ParseConfiguration, Point, ProbabilisticTruthValue,
    Probability, RegionDetectionPolicy, SourceSpan, SubstitutionRule, TriviaAttachmentPolicy,
    TruthValue, VerificationIssueKind, DATA_FORMAT_TARGETS, GRAMMAR_EMBEDDING_TARGETS,
    LANGUAGE_FIXTURES, MARKUP_LANGUAGE_TARGETS, NATURAL_LANGUAGE_TARGETS,
    PROGRAMMING_LANGUAGE_TARGETS, SECOND_TIER_PROGRAMMING_LANGUAGE_TARGETS,
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
fn mutable_snapshot_edits_preserve_parent_bytes_and_share_unchanged_links() {
    for policy in [
        TriviaAttachmentPolicy::ContainmentLink,
        TriviaAttachmentPolicy::TokenLink,
        TriviaAttachmentPolicy::Both,
    ] {
        let network = LinkNetwork::parse_lossless_text(
            "alpha beta",
            "plain-text",
            ParseConfiguration::new(policy),
        );
        let snapshot = network.snapshot(7, "initial parse");
        drop(network);

        let edited = snapshot
            .network()
            .links()
            .find(|link| {
                link.metadata().link_type() == Some(LinkType::Token)
                    && link
                        .metadata()
                        .span()
                        .is_some_and(|span| span.byte_range().start() == 0)
            })
            .map(meta_language::Link::id)
            .expect("first token exists");
        let unchanged = snapshot
            .network()
            .links()
            .find(|link| {
                link.metadata().link_type() == Some(LinkType::Token)
                    && link
                        .metadata()
                        .span()
                        .is_some_and(|span| span.byte_range().start() == 1)
            })
            .map(meta_language::Link::id)
            .expect("second token exists");

        let mut mutable = snapshot.to_mutable("mark first token missing");
        assert_eq!(snapshot.network().shared_link_count(unchanged), Some(2));

        assert!(mutable
            .network_mut()
            .set_flags(edited, LinkFlags::missing()));
        assert_eq!(snapshot.network().reconstruct_text(), "alpha beta");
        assert_eq!(mutable.network().reconstruct_text(), "lpha beta");
        assert_eq!(snapshot.network().shared_link_count(unchanged), Some(2));
        assert_eq!(snapshot.network().shared_link_count(edited), Some(1));

        let committed = mutable.commit();
        assert_eq!(committed.version(), 8);
        assert_eq!(committed.parent_version(), Some(7));
        assert_eq!(committed.network().reconstruct_text(), "lpha beta");
        assert_eq!(snapshot.network().reconstruct_text(), "alpha beta");
        assert_eq!(committed.network().shared_link_count(unchanged), Some(2));
        assert_eq!(committed.network().shared_link_count(edited), Some(1));
    }
}

#[test]
fn apply_edit_reconstructs_fresh_parse_and_preserves_outside_link_ids() {
    let source = "let alpha = 1;\nlet beta = alpha + 1;\n";
    let mut network = LinkNetwork::parse(source, "JavaScript", ParseConfiguration::default());

    let alpha_range = find_range(source, "alpha");
    let before_edit_beta = token_id(&network, "beta", source.find("beta").expect("beta exists"));

    assert!(network.apply_edit(alpha_range, "gamma"));

    let edited_source = source.replacen("alpha", "gamma", 1);
    let fresh = LinkNetwork::parse(&edited_source, "JavaScript", ParseConfiguration::default());

    assert_eq!(network, fresh);
    assert_eq!(network.reconstruct_text(), edited_source);
    assert_eq!(
        token_id(
            &network,
            "beta",
            edited_source.find("beta").expect("beta remains")
        ),
        before_edit_beta
    );
}

#[test]
fn apply_edit_keeps_ids_stable_when_new_links_shift_the_fresh_parse_order() {
    let source = "let alpha = 1;\nlet beta = 2;\n";
    let mut network = LinkNetwork::parse(source, "JavaScript", ParseConfiguration::default());
    let before_edit_beta = token_id(&network, "beta", source.find("beta").expect("beta exists"));

    assert!(network.apply_edit(ByteRange::new(0, 0), "let prefix = 0;\n"));

    let edited_source = "let prefix = 0;\nlet alpha = 1;\nlet beta = 2;\n";
    assert_eq!(network.reconstruct_text(), edited_source);
    assert_eq!(
        token_id(
            &network,
            "beta",
            edited_source.find("beta").expect("beta remains")
        ),
        before_edit_beta
    );
}

#[test]
fn snapshot_diff_reports_structural_changes_from_an_edited_fork() {
    let source = "let alpha = 1;\nlet beta = 2;\n";
    let network = LinkNetwork::parse(source, "JavaScript", ParseConfiguration::default());
    let snapshot = network.snapshot(1, "initial parse");
    drop(network);
    let beta = token_id(
        snapshot.network(),
        "beta",
        source.find("beta").expect("beta exists"),
    );
    let alpha = token_id(
        snapshot.network(),
        "alpha",
        source.find("alpha").expect("alpha exists"),
    );

    let mut mutable = snapshot.to_mutable("rename alpha");
    assert!(mutable
        .network_mut()
        .apply_edit(find_range(source, "alpha"), "gamma"));
    assert_eq!(snapshot.network().shared_link_count(beta), Some(2));
    let committed = mutable.commit();

    let diff = snapshot.structural_diff(&committed);

    assert!(diff.changed().contains(&alpha) || diff.removed().contains(&alpha));
    assert!(!diff.changed().contains(&beta));
    assert!(!diff.added().contains(&beta));
    assert!(!diff.removed().contains(&beta));
    assert_eq!(committed.network().shared_link_count(beta), Some(2));
    assert_eq!(
        committed.network().reconstruct_text(),
        source.replacen("alpha", "gamma", 1)
    );
}

#[test]
fn identical_metadata_terms_share_interned_storage() {
    let mut network = LinkNetwork::new();
    let root = network.insert_point("root");

    network.insert_link([root], LinkMetadata::new().with_term("shared"));
    network.insert_link([root], LinkMetadata::new().with_term("shared"));

    assert!(
        network.interned_string_count("shared").unwrap_or_default() >= 3,
        "expected intern pool and metadata terms to share the same string storage"
    );
    assert_eq!(network.interned_string_count("missing"), None);
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
        .any(|language| language == "sql-ansi"));

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
fn embedded_regions_are_parsed_and_connected_to_region_links() {
    let markdown =
        "Intro\n```JavaScript\nconst value = 1;\n```\n<section><em>HTML</em></section>\n";
    let markdown_network = LinkNetwork::parse(markdown, "Markdown", ParseConfiguration::default());

    assert_eq!(markdown_network.reconstruct_text(), markdown);
    assert_region_has_connected_syntax(&markdown_network, "JavaScript");
    assert_region_has_connected_syntax(&markdown_network, "HTML");
    assert!(markdown_network.links().any(|link| {
        link.metadata().link_type() == Some(LinkType::Token)
            && link.metadata().language() == Some("JavaScript")
            && link.metadata().term() == Some("value")
    }));

    let html = "<script>const value = 1;</script><style>.x { color: red; }</style><p style=\"color: blue\">text</p>";
    let html_network = LinkNetwork::parse(html, "HTML", ParseConfiguration::default());

    assert_eq!(html_network.reconstruct_text(), html);
    assert_region_has_connected_syntax(&html_network, "JavaScript");
    assert_region_has_connected_syntax(&html_network, "CSS");
}

#[test]
fn content_driven_embedded_regions_are_parsed() {
    let markdown = "Intro\n```\nconst value = 1;\n```\n";
    let network = LinkNetwork::parse(
        markdown,
        "Markdown",
        ParseConfiguration::default()
            .with_region_detection_policy(RegionDetectionPolicy::ContentDriven),
    );

    assert_eq!(network.reconstruct_text(), markdown);
    assert_region_has_connected_syntax(&network, "JavaScript");
}

#[test]
fn content_driven_detection_falls_back_to_txt_region() {
    let markdown = "Notes\n```\nplain prose\ncafe au lait\n```\n";
    let network = LinkNetwork::parse(
        markdown,
        "Markdown",
        ParseConfiguration::default()
            .with_region_detection_policy(RegionDetectionPolicy::ContentDriven),
    );
    let regions = network.embedded_regions();

    assert_eq!(regions.len(), 1);
    assert_eq!(regions[0].language(), "txt");
    assert_eq!(
        regions[0].span().byte_range(),
        ByteRange::new(
            markdown
                .find("plain prose")
                .expect("region starts at prose"),
            markdown
                .rfind("```")
                .expect("region ends before closing fence"),
        )
    );
}

#[test]
fn natural_language_parse_adds_segmentation_language_and_unicode_annotations() {
    let latin_source = "Natural language links work.\n";
    let latin_network = LinkNetwork::parse(latin_source, "English", ParseConfiguration::default());

    assert_eq!(latin_network.reconstruct_text(), latin_source);
    assert_token_link(
        &latin_network,
        "Natural",
        ByteRange::new(0, "Natural".len()),
    );
    assert_link_with_term(
        &latin_network,
        LinkType::Semantic,
        "segmentation:unicode-segmentation",
    );
    assert_link_with_term(&latin_network, LinkType::Language, "English");

    let mandarin_source = "你好。\n";
    let mandarin_network = LinkNetwork::parse(
        mandarin_source,
        "Mandarin Chinese",
        ParseConfiguration::default(),
    );

    assert_eq!(mandarin_network.reconstruct_text(), mandarin_source);
    assert_token_link(&mandarin_network, "你好", ByteRange::new(0, "你好".len()));
    assert_link_with_term(
        &mandarin_network,
        LinkType::Semantic,
        "segmentation:lindera-jieba",
    );
    assert_link_with_term(&mandarin_network, LinkType::Language, "Mandarin Chinese");

    let arabic_source = "مرحبا.\n";
    let arabic_network = LinkNetwork::parse(
        arabic_source,
        "Modern Standard Arabic",
        ParseConfiguration::default(),
    );

    assert_eq!(arabic_network.reconstruct_text(), arabic_source);
    assert_link_with_term(&arabic_network, LinkType::Semantic, "bidi:rtl");
    assert_link_with_prefix(&arabic_network, LinkType::Semantic, "normalization:nfc:");
    assert_link_with_prefix(&arabic_network, LinkType::Semantic, "normalization:nfd:");
}

#[test]
fn natural_language_identifier_backend_is_switchable() {
    assert_eq!(
        ParseConfiguration::default().language_identification_detector(),
        LanguageIdentificationDetector::Lingua
    );

    let source = "This sentence gives the detector enough English context.\n";
    let network = LinkNetwork::parse(
        source,
        "English",
        ParseConfiguration::default()
            .with_language_identification_detector(LanguageIdentificationDetector::Whatlang),
    );

    assert_eq!(network.reconstruct_text(), source);
    assert_link_with_term(&network, LinkType::Language, "English");
    assert_link_with_term(&network, LinkType::Semantic, "identifier:whatlang");
}

#[test]
fn natural_language_fixtures_keep_byte_exact_reconstruction_with_language_regions() {
    let natural_languages = NATURAL_LANGUAGE_TARGETS
        .iter()
        .map(meta_language::LanguageTarget::name)
        .collect::<Vec<_>>();
    let mut checked_fixtures = 0;

    for fixture in LANGUAGE_FIXTURES
        .iter()
        .filter(|fixture| natural_languages.contains(&fixture.language()))
    {
        checked_fixtures += 1;
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
            network.links().any(|link| {
                link.metadata().link_type() == Some(LinkType::Region)
                    && link.metadata().language() == Some(fixture.language())
                    && link.metadata().span().is_some()
            }),
            "{} fixture should have a natural-language region",
            fixture.description()
        );
        assert!(
            network.links().any(|link| {
                link.metadata().link_type() == Some(LinkType::Language)
                    && link.metadata().span().is_some()
            }),
            "{} fixture should have a region-scoped language link",
            fixture.description()
        );
    }

    assert_eq!(checked_fixtures, NATURAL_LANGUAGE_TARGETS.len());
}

fn assert_region_has_connected_syntax(network: &LinkNetwork, language: &str) {
    let region_ids = network
        .links()
        .filter(|link| link.metadata().link_type() == Some(LinkType::Region))
        .filter(|link| link.metadata().language() == Some(language))
        .map(meta_language::Link::id)
        .collect::<Vec<_>>();

    assert!(
        !region_ids.is_empty(),
        "expected at least one {language} region"
    );
    assert!(
        network.links().any(|link| {
            link.metadata().link_type() == Some(LinkType::Syntax)
                && link.metadata().language() == Some(language)
                && link
                    .references()
                    .iter()
                    .any(|reference| region_ids.contains(reference))
        }),
        "expected {language} syntax rooted at a region link"
    );
}

fn assert_token_link(network: &LinkNetwork, term: &str, range: ByteRange) {
    assert!(
        network.links().any(|link| {
            link.metadata().link_type() == Some(LinkType::Token)
                && link.metadata().term() == Some(term)
                && link
                    .metadata()
                    .span()
                    .is_some_and(|span| span.byte_range() == range)
        }),
        "expected token link for {term:?} at {range:?}"
    );
}

fn assert_link_with_term(network: &LinkNetwork, link_type: LinkType, term: &str) {
    assert!(
        network.links().any(|link| {
            link.metadata().link_type() == Some(link_type) && link.metadata().term() == Some(term)
        }),
        "expected {link_type:?} link with term {term:?}"
    );
}

fn assert_link_with_prefix(network: &LinkNetwork, link_type: LinkType, prefix: &str) {
    assert!(
        network.links().any(|link| {
            link.metadata().link_type() == Some(link_type)
                && link
                    .metadata()
                    .term()
                    .is_some_and(|term| term.starts_with(prefix))
        }),
        "expected {link_type:?} link with prefix {prefix:?}"
    );
}

fn find_range(source: &str, needle: &str) -> ByteRange {
    let start = source.find(needle).expect("needle exists in source");
    ByteRange::new(start, start + needle.len())
}

fn token_id(network: &LinkNetwork, term: &str, start: usize) -> LinkId {
    network
        .links()
        .find(|link| {
            link.metadata().link_type() == Some(LinkType::Token)
                && link.metadata().term() == Some(term)
                && link
                    .metadata()
                    .span()
                    .is_some_and(|span| span.byte_range().start() == start)
        })
        .map(meta_language::Link::id)
        .expect("token exists")
}

#[test]
fn txt_parse_exposes_whole_buffer_as_single_region() {
    let source = "Plain text region\nUTF-8 line: café\n";
    let network = LinkNetwork::parse(source, "txt", ParseConfiguration::default());
    let regions = network.embedded_regions();

    assert_eq!(network.reconstruct_text(), source);
    assert_eq!(regions.len(), 1);
    assert_eq!(regions[0].language(), "txt");
    assert_eq!(
        regions[0].span().byte_range(),
        ByteRange::new(0, source.len())
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
fn probabilistic_truth_values_cover_relative_meta_logic_probability_cases() {
    let half = Probability::from_ratio(1, 2).expect("valid probability");
    let likely = Probability::from_basis_points(7_500).expect("valid probability");

    let liar = ProbabilisticTruthValue::new(half);
    let event = ProbabilisticTruthValue::new(likely);

    assert_eq!(liar.true_probability().basis_points(), 5_000);
    assert_eq!(liar.false_probability().basis_points(), 5_000);
    assert_eq!(liar.negate(), liar);
    assert_eq!(event.negate().true_probability().basis_points(), 2_500);
    assert_eq!(liar.and(event).true_probability().basis_points(), 3_750);
    assert_eq!(liar.or(event).true_probability().basis_points(), 8_750);
}

#[test]
fn language_targets_cover_markup_programming_natural_and_embedding_scope() {
    assert_eq!(MARKUP_LANGUAGE_TARGETS.len(), 5);
    assert_eq!(PROGRAMMING_LANGUAGE_TARGETS.len(), 10);
    assert_eq!(NATURAL_LANGUAGE_TARGETS.len(), 10);

    let markup_names = MARKUP_LANGUAGE_TARGETS
        .iter()
        .map(meta_language::LanguageTarget::name)
        .collect::<Vec<_>>();
    assert!(markup_names.contains(&"txt"));
    assert!(markup_names.contains(&"Markdown"));
    assert!(markup_names.contains(&"HTML"));
    assert!(markup_names.contains(&"PDF"));
    assert!(markup_names.contains(&"DOCX"));

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
    assert!(PROGRAMMING_LANGUAGE_TARGETS
        .iter()
        .any(|target| target.name() == "sql-ansi"));
    assert!(NATURAL_LANGUAGE_TARGETS
        .iter()
        .all(|target| target.basis().contains("Ethnologue/Britannica")));
}

#[test]
fn data_format_targets_cover_interchange_format_scope() {
    assert_eq!(DATA_FORMAT_TARGETS.len(), 9);

    let names = DATA_FORMAT_TARGETS
        .iter()
        .map(meta_language::LanguageTarget::name)
        .collect::<Vec<_>>();
    assert_eq!(
        names,
        vec!["JSON", "YAML", "TOML", "XML", "INI", "protobuf", "GraphQL", "CSV", "JSON5"]
    );

    assert!(DATA_FORMAT_TARGETS
        .iter()
        .all(|target| target.family() == meta_language::LanguageFamily::DataFormat));
    assert!(DATA_FORMAT_TARGETS
        .iter()
        .all(|target| target.basis().contains("Issue #47")));

    assert!(names.contains(&"CSV"));
    assert!(names.contains(&"JSON5"));
}

#[test]
fn second_tier_programming_targets_cover_next_grammar_wave_scope() {
    assert_eq!(SECOND_TIER_PROGRAMMING_LANGUAGE_TARGETS.len(), 6);

    let names = SECOND_TIER_PROGRAMMING_LANGUAGE_TARGETS
        .iter()
        .map(meta_language::LanguageTarget::name)
        .collect::<Vec<_>>();
    assert_eq!(
        names,
        vec!["PHP", "Swift", "Kotlin", "Scala", "Lua", "Perl"]
    );

    assert!(SECOND_TIER_PROGRAMMING_LANGUAGE_TARGETS
        .iter()
        .all(|target| target.family() == meta_language::LanguageFamily::Programming));
    assert!(SECOND_TIER_PROGRAMMING_LANGUAGE_TARGETS
        .iter()
        .all(|target| target.basis().contains("Issue #47 R-2")));

    assert!(names.contains(&"Perl"));
}

#[test]
fn natural_language_targets_follow_ethnologue_2025_total_speaker_order() {
    let target_names = NATURAL_LANGUAGE_TARGETS
        .iter()
        .map(meta_language::LanguageTarget::name)
        .collect::<Vec<_>>();

    assert_eq!(
        target_names,
        vec![
            "English",
            "Mandarin Chinese",
            "Hindi",
            "Spanish",
            "Modern Standard Arabic",
            "French",
            "Bengali",
            "Portuguese",
            "Russian",
            "Urdu",
        ]
    );
}

#[test]
fn every_language_target_has_an_executable_lossless_fixture() {
    let target_languages = MARKUP_LANGUAGE_TARGETS
        .iter()
        .chain(PROGRAMMING_LANGUAGE_TARGETS.iter())
        .chain(SECOND_TIER_PROGRAMMING_LANGUAGE_TARGETS.iter())
        .chain(NATURAL_LANGUAGE_TARGETS.iter())
        .chain(DATA_FORMAT_TARGETS.iter())
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
