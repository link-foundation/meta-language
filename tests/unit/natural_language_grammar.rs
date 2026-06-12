use meta_language::{
    LinkNetwork, LinkQuery, LinkType, ParseConfiguration, QueryPredicateHost,
    VerificationIssueKind, NATURAL_LANGUAGE_GRAMMAR_FIXTURES, NATURAL_LANGUAGE_TARGETS,
};

#[test]
fn natural_language_grammar_fixtures_cover_targets_with_provenance() {
    let target_languages = NATURAL_LANGUAGE_TARGETS
        .iter()
        .map(meta_language::LanguageTarget::name)
        .collect::<Vec<_>>();
    let fixture_languages = NATURAL_LANGUAGE_GRAMMAR_FIXTURES
        .iter()
        .map(meta_language::NaturalLanguageGrammarFixture::language)
        .collect::<Vec<_>>();

    assert_eq!(fixture_languages, target_languages);
    for fixture in NATURAL_LANGUAGE_GRAMMAR_FIXTURES {
        assert!(fixture.provenance().contains("license:"));
        assert!(fixture.provenance().contains("Universal Dependencies"));
    }
}

#[test]
fn natural_language_targets_emit_ud_morphosyntax_and_parse_cleanly() {
    for fixture in NATURAL_LANGUAGE_GRAMMAR_FIXTURES {
        let network = LinkNetwork::parse(
            fixture.grammatical_source(),
            fixture.language(),
            ParseConfiguration::default(),
        );

        assert_eq!(network.reconstruct_text(), fixture.grammatical_source());
        assert!(
            network.verify_full_match(None).is_clean(),
            "{} grammatical fixture should parse cleanly",
            fixture.language()
        );
        assert_query_matches(&network, "natural-language:sentence", fixture.language());
        assert_query_matches(&network, "upos:PUNCT", fixture.language());
        assert_query_matches(&network, "deprel:root", fixture.language());
    }
}

#[test]
fn natural_language_ungrammatical_fixtures_emit_recoverable_error_links() {
    for fixture in NATURAL_LANGUAGE_GRAMMAR_FIXTURES {
        let network = LinkNetwork::parse(
            fixture.ungrammatical_source(),
            fixture.language(),
            ParseConfiguration::default(),
        );
        let report = network.verify_full_match(None);

        assert_eq!(network.reconstruct_text(), fixture.ungrammatical_source());
        assert!(
            report
                .issues()
                .iter()
                .any(|issue| issue.kind() == VerificationIssueKind::ErrorLink),
            "{} ungrammatical fixture should surface an error link",
            fixture.language()
        );
        assert_query_matches(
            &network,
            "natural-language:error:grammar",
            fixture.language(),
        );
    }
}

#[test]
fn natural_language_morphosyntax_links_are_queryable_by_ud_terms() {
    let network = LinkNetwork::parse(
        "Hawaii is a state.\n",
        "English",
        ParseConfiguration::default(),
    );

    let proper_nouns = network.query_links(
        &LinkQuery::by_type(LinkType::Syntax)
            .with_language("English")
            .with_term("upos:PROPN"),
    );
    let singular_features = network.query_links(
        &LinkQuery::by_type(LinkType::Syntax)
            .with_language("English")
            .with_term("ufeat:Number=Sing"),
    );

    assert!(!proper_nouns.is_empty());
    assert!(!singular_features.is_empty());
}

#[test]
fn unregistered_natural_language_text_remains_clean_during_starter_stage() {
    let source = "1 + 1 = 2\n";
    let network = LinkNetwork::parse(source, "English", ParseConfiguration::default());

    assert_eq!(network.reconstruct_text(), source);
    assert!(network.verify_full_match(None).is_clean());
}

fn assert_query_matches(network: &LinkNetwork, term: &str, language: &str) {
    let matches = network.query_matches_with(
        &LinkQuery::by_type(LinkType::Syntax)
            .with_language(language)
            .with_term(term),
        &AcceptAllPredicates,
    );
    assert!(
        !matches.is_empty(),
        "expected queryable {term} link for {language}"
    );
}

struct AcceptAllPredicates;

impl QueryPredicateHost for AcceptAllPredicates {
    fn evaluate(
        &self,
        _predicate: &meta_language::QueryPredicate,
        _captures: &meta_language::QueryCaptures,
        _network: &LinkNetwork,
    ) -> bool {
        true
    }
}
