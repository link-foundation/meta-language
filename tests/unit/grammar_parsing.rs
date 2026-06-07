use meta_language::{
    LinkNetwork, LinkType, ParseConfiguration, VerificationIssueKind, LANGUAGE_FIXTURES,
};

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
fn grammar_backed_programming_fixtures_emit_syntax_links_and_round_trip() {
    let grammar_backed_languages = [
        "Python",
        "C",
        "Java",
        "C++",
        "C#",
        "JavaScript",
        "Visual Basic",
        "R",
        "sql-ansi",
    ];

    for language in grammar_backed_languages {
        let fixture = LANGUAGE_FIXTURES
            .iter()
            .find(|fixture| fixture.language() == language)
            .expect("grammar-backed language fixture exists");
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
        assert!(
            network.links().any(|link| {
                link.metadata().link_type() == Some(LinkType::Syntax)
                    && link.metadata().language() == Some(language)
                    && link.metadata().span().is_some()
            }),
            "{language} should emit grammar-backed syntax links"
        );
        assert!(
            network
                .links()
                .any(|link| link.metadata().link_type() == Some(LinkType::Syntax)
                    && link.metadata().language() == Some(language)
                    && link.metadata().is_named()),
            "{language} should preserve named syntax metadata"
        );
        if language == "Visual Basic" {
            assert!(
                network.links().any(|link| {
                    link.metadata().link_type() == Some(LinkType::Syntax)
                        && link.metadata().language() == Some(language)
                        && link.metadata().term() == Some("module_block")
                }),
                "{language} should emit Visual Basic grammar node kinds"
            );
        } else if language == "sql-ansi" {
            assert!(
                network.links().any(|link| {
                    link.metadata().link_type() == Some(LinkType::Syntax)
                        && link.metadata().language() == Some(language)
                        && link.metadata().term() == Some("select")
                }),
                "{language} should emit SQL grammar node kinds"
            );
        } else {
            assert!(
                network
                    .links()
                    .any(|link| link.metadata().link_type() == Some(LinkType::Syntax)
                        && link.metadata().language() == Some(language)
                        && !link.metadata().is_named()),
                "{language} should preserve anonymous syntax metadata"
            );
        }
    }
}

#[test]
fn visual_basic_recovery_errors_round_trip_with_flags() {
    let source = "Module Program\n    Sub First(\n    End Sub\n    Sub Main()\n        If Then\n    End Sub\nEnd Module\n";
    let network = LinkNetwork::parse(source, "Visual Basic", ParseConfiguration::default());
    let report = network.verify_full_match(None);

    assert_eq!(network.reconstruct_text(), source);
    assert!(!report.is_clean());
    assert!(report
        .issues()
        .iter()
        .any(|issue| issue.kind() == VerificationIssueKind::ErrorLink));
    assert!(report
        .issues()
        .iter()
        .any(|issue| issue.kind() == VerificationIssueKind::MissingLink));
}

#[test]
fn grammar_backed_parse_emits_field_labels_as_links() {
    let network = LinkNetwork::parse(
        "def f(x):\n    return x\n",
        "Python",
        ParseConfiguration::default(),
    );
    let field_labels = network
        .links()
        .filter(|link| link.metadata().link_type() == Some(LinkType::Field))
        .filter_map(|link| link.references().get(1))
        .filter_map(|label| network.link(*label))
        .filter_map(|label| label.metadata().term())
        .map(str::to_string)
        .collect::<Vec<_>>();

    assert!(
        field_labels.iter().any(|label| label == "name"),
        "expected a function-name field label, got {field_labels:?}"
    );
    assert!(
        field_labels.iter().any(|label| label == "body"),
        "expected a function-body field label, got {field_labels:?}"
    );
    assert!(network.links().any(|link| {
        link.metadata().link_type() == Some(LinkType::Token)
            && link.metadata().flags().is_extra()
            && link
                .metadata()
                .term()
                .is_some_and(|term| term.contains('\n'))
    }));
    assert!(network.links().any(|link| {
        link.metadata().link_type() == Some(LinkType::Trivia)
            && link.metadata().flags().is_extra()
            && link.metadata().span().is_some()
    }));
}

#[test]
fn sql_ansi_fixture_uses_tree_sitter_grammar() {
    let source = "SELECT id, name FROM users WHERE active = TRUE;\n";
    let network = LinkNetwork::parse(source, "sql-ansi", ParseConfiguration::default());

    assert_eq!(network.reconstruct_text(), source);
    assert!(
        network.verify_full_match(None).is_clean(),
        "sql-ansi fixture should parse cleanly"
    );
    assert!(
        network.links().any(|link| {
            link.metadata().link_type() == Some(LinkType::Syntax)
                && link.metadata().language() == Some("sql-ansi")
                && link.metadata().term() == Some("select")
        }),
        "sql-ansi should emit tree-sitter SQL syntax nodes"
    );
}

#[test]
fn parse_marks_recovery_errors_without_losing_original_text() {
    let source = "class C { void M() { if ( }";
    let network = LinkNetwork::parse(source, "C#", ParseConfiguration::default());
    let report = network.verify_full_match(None);

    assert_eq!(network.reconstruct_text(), source);
    assert!(!report.is_clean());
    assert!(report
        .issues()
        .iter()
        .any(|issue| issue.kind() == VerificationIssueKind::ErrorLink));
    assert!(report
        .issues()
        .iter()
        .any(|issue| issue.kind() == VerificationIssueKind::HasErrorLink));
    assert!(network
        .links()
        .any(|link| link.metadata().flags().has_error()));
}
