use meta_language::{
    DATA_FORMAT_TARGETS, GRAMMAR_EMBEDDING_TARGETS, LANGUAGE_FIXTURES, LinkNetwork,
    MARKUP_LANGUAGE_TARGETS, NATURAL_LANGUAGE_TARGETS, PROGRAMMING_LANGUAGE_TARGETS,
    ParseConfiguration, SECOND_TIER_PROGRAMMING_LANGUAGE_TARGETS,
};

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

    assert!(
        PROGRAMMING_LANGUAGE_TARGETS
            .iter()
            .all(|target| target.basis().contains("TIOBE May 2026"))
    );
    assert!(
        PROGRAMMING_LANGUAGE_TARGETS
            .iter()
            .any(|target| target.name() == "sql-ansi")
    );
    assert!(
        NATURAL_LANGUAGE_TARGETS
            .iter()
            .all(|target| target.basis().contains("Ethnologue/Britannica"))
    );
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
        vec![
            "JSON", "YAML", "TOML", "XML", "INI", "protobuf", "GraphQL", "CSV", "JSON5"
        ]
    );

    assert!(
        DATA_FORMAT_TARGETS
            .iter()
            .all(|target| target.family() == meta_language::LanguageFamily::DataFormat)
    );
    assert!(
        DATA_FORMAT_TARGETS
            .iter()
            .all(|target| target.basis().contains("Issue #47"))
    );

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

    assert!(
        SECOND_TIER_PROGRAMMING_LANGUAGE_TARGETS
            .iter()
            .all(|target| target.family() == meta_language::LanguageFamily::Programming)
    );
    assert!(
        SECOND_TIER_PROGRAMMING_LANGUAGE_TARGETS
            .iter()
            .all(|target| target.basis().contains("Issue #47 R-2"))
    );

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
