use meta_language::{
    LinkNetwork, LinkType, NetworkProjection, ParseConfiguration, VerificationIssueKind,
    LANGUAGE_FIXTURES,
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
        "Delphi/Object Pascal",
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
        } else if language == "Delphi/Object Pascal" {
            assert!(
                network.links().any(|link| {
                    link.metadata().link_type() == Some(LinkType::Syntax)
                        && link.metadata().language() == Some(language)
                        && link.metadata().term() == Some("unit")
                }),
                "{language} should emit Pascal grammar node kinds"
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
fn ruby_fixture_uses_tree_sitter_ruby() {
    let source = "def greet(name)\n  puts \"Hello, #{name}\"\nend\n";
    let network = LinkNetwork::parse(source, "Ruby", ParseConfiguration::default());

    assert_eq!(network.reconstruct_text(), source);
    assert!(
        network.verify_full_match(None).is_clean(),
        "Ruby fixture should parse cleanly"
    );
    assert!(
        network.links().any(|link| {
            link.metadata().link_type() == Some(LinkType::Syntax)
                && link.metadata().language() == Some("Ruby")
                && link.metadata().span().is_some()
        }),
        "Ruby should emit grammar-backed syntax links"
    );
    for term in ["program", "method", "call"] {
        assert!(
            network.links().any(|link| {
                link.metadata().link_type() == Some(LinkType::Syntax)
                    && link.metadata().language() == Some("Ruby")
                    && link.metadata().term() == Some(term)
            }),
            "Ruby should emit the {term} grammar node kind"
        );
    }
}

#[test]
fn ruby_alias_uses_tree_sitter_ruby() {
    let source = "value = 1\n";

    for language in ["ruby", "rb"] {
        let network = LinkNetwork::parse(source, language, ParseConfiguration::default());

        assert_eq!(network.reconstruct_text(), source);
        assert!(
            network.verify_full_match(None).is_clean(),
            "{language} alias should parse cleanly"
        );
        assert!(
            network.links().any(|link| {
                link.metadata().link_type() == Some(LinkType::Syntax)
                    && link.metadata().language() == Some(language)
                    && link.metadata().term() == Some("program")
            }),
            "{language} alias should emit Ruby grammar nodes"
        );
    }
}

#[test]
fn delphi_object_pascal_fixture_uses_tree_sitter_pascal() {
    let fixture = LANGUAGE_FIXTURES
        .iter()
        .find(|fixture| fixture.language() == "Delphi/Object Pascal")
        .expect("Delphi/Object Pascal fixture exists");
    let network = LinkNetwork::parse(
        fixture.source(),
        fixture.language(),
        ParseConfiguration::default(),
    );

    assert_eq!(network.reconstruct_text(), fixture.source());
    assert!(
        network.verify_full_match(None).is_clean(),
        "{} fixture should parse cleanly",
        fixture.description()
    );

    for term in ["unit", "declClass", "declProp", "rttiAttributes"] {
        assert!(
            network.links().any(|link| {
                link.metadata().link_type() == Some(LinkType::Syntax)
                    && link.metadata().language() == Some("Delphi/Object Pascal")
                    && link.metadata().term() == Some(term)
            }),
            "Delphi/Object Pascal should emit {term} syntax nodes"
        );
    }
}

#[test]
fn pascal_aliases_use_tree_sitter_pascal() {
    let source = "program Demo;\nbegin\nend.\n";

    for language in ["Delphi", "Object Pascal", "Pascal"] {
        let network = LinkNetwork::parse(source, language, ParseConfiguration::default());

        assert_eq!(network.reconstruct_text(), source);
        assert!(
            network.verify_full_match(None).is_clean(),
            "{language} alias should parse cleanly"
        );
        assert!(
            network.links().any(|link| {
                link.metadata().link_type() == Some(LinkType::Syntax)
                    && link.metadata().language() == Some(language)
                    && link.metadata().term() == Some("program")
            }),
            "{language} alias should emit Pascal grammar nodes"
        );
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
fn typescript_fixture_uses_tree_sitter_typescript() {
    let source = "interface Box<T> {\n    value: T;\n}\n\nconst value: number = 1;\n";
    let network = LinkNetwork::parse(source, "TypeScript", ParseConfiguration::default());

    assert_eq!(network.reconstruct_text(), source);
    assert!(
        network.verify_full_match(None).is_clean(),
        "TypeScript fixture should parse cleanly"
    );
    assert!(
        network.links().any(|link| {
            link.metadata().link_type() == Some(LinkType::Syntax)
                && link.metadata().language() == Some("TypeScript")
                && link.metadata().term() == Some("program")
                && link.metadata().is_named()
        }),
        "TypeScript should emit the tree-sitter program root node"
    );
    assert!(
        network.links().any(|link| {
            link.metadata().link_type() == Some(LinkType::Syntax)
                && link.metadata().language() == Some("TypeScript")
                && link.metadata().term() == Some("interface_declaration")
        }),
        "TypeScript should emit type-system grammar nodes"
    );
    assert!(
        network.links().any(|link| {
            link.metadata().link_type() == Some(LinkType::Syntax)
                && link.metadata().language() == Some("TypeScript")
                && !link.metadata().is_named()
        }),
        "TypeScript should preserve anonymous syntax metadata"
    );
}

#[test]
fn typescript_aliases_and_tsx_use_tree_sitter_typescript() {
    for (language, source) in [
        ("ts", "let count: number = 0;\n"),
        ("typescript", "type Id = string;\n"),
        ("tsx", "const view = <div>hi</div>;\n"),
    ] {
        let network = LinkNetwork::parse(source, language, ParseConfiguration::default());

        assert_eq!(
            network.reconstruct_text(),
            source,
            "{language} fixture failed reconstruction"
        );
        assert!(
            network.verify_full_match(None).is_clean(),
            "{language} fixture should parse cleanly"
        );
        assert!(
            network.links().any(|link| {
                link.metadata().link_type() == Some(LinkType::Syntax)
                    && link.metadata().language() == Some(language)
                    && link.metadata().term() == Some("program")
            }),
            "{language} should emit grammar-backed syntax links"
        );
    }
}

#[test]
fn go_source_uses_tree_sitter_grammar() {
    let source = "package main\n\nfunc main() {\n\tprintln(\"hi\")\n}\n";

    for language in ["Go", "go", "golang"] {
        let network = LinkNetwork::parse(source, language, ParseConfiguration::default());

        assert_eq!(
            network.reconstruct_text(),
            source,
            "{language} fixture failed reconstruction"
        );
        assert!(
            network.verify_full_match(None).is_clean(),
            "{language} fixture should parse cleanly"
        );
        assert!(
            network.links().any(|link| {
                link.metadata().link_type() == Some(LinkType::Syntax)
                    && link.metadata().language() == Some(language)
                    && link.metadata().term() == Some("function_declaration")
            }),
            "{language} should emit tree-sitter Go syntax nodes"
        );
        let concrete_syntax_links = network
            .projected_links(NetworkProjection::ConcreteSyntax)
            .filter(|link| link.metadata().link_type() == Some(LinkType::Syntax))
            .count();
        assert!(
            concrete_syntax_links > 0,
            "{language} should produce grammar-backed concrete syntax links"
        );
    }
}

#[test]
fn typescript_recovery_errors_round_trip_with_flags() {
    let source = "function f(: number {\n";
    let network = LinkNetwork::parse(source, "TypeScript", ParseConfiguration::default());
    let report = network.verify_full_match(None);

    assert_eq!(network.reconstruct_text(), source);
    assert!(!report.is_clean());
    assert!(report
        .issues()
        .iter()
        .any(|issue| issue.kind() == VerificationIssueKind::ErrorLink
            || issue.kind() == VerificationIssueKind::MissingLink));
    assert!(network
        .links()
        .any(|link| link.metadata().flags().has_error() || link.metadata().flags().is_missing()));
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
