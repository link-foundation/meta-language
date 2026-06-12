use meta_language::{
    LinkNetwork, LinkQuery, ParityCapability, ParityVerificationExpectation, ParseConfiguration,
    ReplacementRule, VerificationIssueKind, PARITY_FIXTURES, PARITY_TARGETS,
};

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
        "ast-grep",
        "Semgrep",
        "Comby",
        "GritQL",
        "srcML",
        "difftastic",
        "Babel",
        "SWC",
        "OpenRewrite",
        "Spoon",
        "JavaParser",
        "Rascal",
        "Stratego/Spoofax",
        "TXL",
        "MPS",
        "Coccinelle",
        "GF",
        "Universal Dependencies",
        "LanguageTool",
        "doublets-rs",
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
        assert!(
            fixture.provenance().contains('/'),
            "{} fixture provenance should include an upstream path",
            fixture.name()
        );
        assert!(
            fixture.provenance().contains("license: "),
            "{} fixture provenance should include a license",
            fixture.name()
        );

        let mut network = LinkNetwork::parse(
            fixture.source(),
            fixture.language(),
            ParseConfiguration::default(),
        );
        let verification = network.verify_full_match(None);

        assert_eq!(
            network.reconstruct_text(),
            fixture.expected_reconstruction(),
            "{} fixture failed reconstruction",
            fixture.name()
        );

        match fixture.verification_expectation() {
            ParityVerificationExpectation::Clean => {
                assert!(
                    verification.is_clean(),
                    "{} fixture should parse cleanly: {:?}",
                    fixture.name(),
                    verification.issues()
                );
            }
            ParityVerificationExpectation::Recoverable => {
                assert!(
                    !verification.is_clean(),
                    "{} fixture should expose recovery diagnostics",
                    fixture.name()
                );
                assert!(
                    verification.issues().iter().any(|issue| matches!(
                        issue.kind(),
                        VerificationIssueKind::ErrorLink
                            | VerificationIssueKind::HasErrorLink
                            | VerificationIssueKind::MissingLink
                    )),
                    "{} fixture should report error, has-error, or missing links",
                    fixture.name()
                );
            }
        }

        if let Some(transform) = fixture.transform_expectation() {
            let query =
                LinkQuery::from_sexpression(transform.query()).expect("fixture query parses");
            let captures = network.find(&query);
            assert!(
                !captures.is_empty(),
                "{} fixture transform query should capture links",
                fixture.name()
            );
            let report = network.replace(
                &captures,
                &ReplacementRule::captured_text(transform.capture_name(), transform.replacement()),
            );

            assert!(
                !report.is_empty(),
                "{} fixture transform should make a change",
                fixture.name()
            );
            assert_eq!(
                network.reconstruct_text(),
                transform.expected_output(),
                "{} fixture transform output mismatch",
                fixture.name()
            );
        }

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

        if fixture
            .capabilities()
            .contains(&ParityCapability::LinoSerialization)
        {
            let lino = network.to_lino();
            let restored = LinkNetwork::from_lino(&lino)
                .expect("from_lino reconstructs the serialized network");
            assert_eq!(
                restored.to_lino(),
                lino,
                "{} fixture serialization is not round-trip stable",
                fixture.name()
            );
            assert_eq!(
                restored.links().count(),
                network.links().count(),
                "{} fixture serialization lost links",
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
fn external_competitor_corpora_contribute_multiple_fixtures() {
    for target_name in [
        "tree-sitter",
        "LibCST",
        "Recast",
        "jscodeshift",
        "Rowan",
        "cstree",
        "Roslyn",
    ] {
        let fixture_count = PARITY_FIXTURES
            .iter()
            .filter(|fixture| fixture.target_name() == target_name)
            .count();

        assert!(
            fixture_count >= 2,
            "{target_name} should contribute multiple ported upstream fixtures, got {fixture_count}"
        );
    }
}

#[test]
fn ecosystem_corpora_contribute_required_internal_fixtures() {
    let expected_fixture_counts = [
        ("links-notation", 5),
        ("link-cli", 4),
        ("lino-objects-codec", 3),
        ("relative-meta-logic", 3),
        ("formal-ai", 4),
        ("meta-expression", 3),
    ];

    for (target_name, minimum_count) in expected_fixture_counts {
        let fixture_count = PARITY_FIXTURES
            .iter()
            .filter(|fixture| fixture.target_name() == target_name)
            .count();

        assert!(
            fixture_count >= minimum_count,
            "{target_name} should contribute at least {minimum_count} ported ecosystem fixtures, got {fixture_count}"
        );
    }

    let links_notation_fixtures = fixtures_for("links-notation");
    for expected_name in [
        "doublet",
        "triplet",
        "n-tuple",
        "indented",
        "self reference",
    ] {
        assert_fixture_named(&links_notation_fixtures, expected_name);
    }
    assert!(links_notation_fixtures.iter().any(|fixture| {
        fixture.provenance().contains("TEST_CASE_COMPARISON.md")
            && fixture.provenance().contains("137/138/138/140")
    }));

    let link_cli_fixtures = fixtures_for("link-cli");
    for expected_name in ["create", "update", "delete", "swap"] {
        assert_fixture_named(&link_cli_fixtures, expected_name);
    }
    assert!(link_cli_fixtures.iter().all(|fixture| fixture
        .provenance()
        .contains("Foundation.Data.Doublets.Cli.Tests")));

    let codec_fixtures = fixtures_for("lino-objects-codec");
    for expected_name in ["roundtrip", "shared", "circular"] {
        assert_fixture_named(&codec_fixtures, expected_name);
    }

    let rml_fixtures = fixtures_for("relative-meta-logic");
    for expected_name in ["dependent", "many-valued", "probabilistic", "liar"] {
        assert_fixture_named(&rml_fixtures, expected_name);
    }

    let formal_ai_fixtures = fixtures_for("formal-ai");
    assert!(formal_ai_fixtures
        .iter()
        .any(|fixture| fixture.provenance().contains("data/seed/")));
    assert!(formal_ai_fixtures
        .iter()
        .any(|fixture| fixture.provenance().contains("data/benchmarks/")));
    assert!(formal_ai_fixtures
        .iter()
        .all(|fixture| !fixture.provenance().contains("706")));

    let meta_expression_fixtures = fixtures_for("meta-expression");
    for expected_name in ["Hawaii", "1 + 1", "this statement is false"] {
        assert_fixture_named(&meta_expression_fixtures, expected_name);
    }
    let mut ontology_network = LinkNetwork::self_describing();
    assert_eq!(
        ontology_network
            .seed_common_concept_ontology()
            .lexicon_concepts(),
        351
    );
}

#[test]
fn wave_two_competitor_corpora_are_sampled_with_expected_fixture_shapes() {
    for target_name in [
        "ast-grep",
        "Semgrep",
        "Comby",
        "GritQL",
        "srcML",
        "difftastic",
        "Babel",
        "SWC",
        "OpenRewrite",
        "Spoon",
        "JavaParser",
        "Rascal",
        "Stratego/Spoofax",
        "TXL",
        "MPS",
        "Coccinelle",
        "GF",
        "Universal Dependencies",
        "LanguageTool",
        "doublets-rs",
    ] {
        let fixtures = fixtures_for(target_name);
        assert!(
            !fixtures.is_empty(),
            "{target_name} should have at least one sampled, executable fixture"
        );
        assert!(
            fixtures
                .iter()
                .all(|fixture| fixture.provenance().contains("license:")),
            "{target_name} fixtures should record license provenance"
        );
    }

    for target_name in [
        "ast-grep",
        "Semgrep",
        "Comby",
        "GritQL",
        "Babel",
        "OpenRewrite",
        "Spoon",
        "JavaParser",
        "Stratego/Spoofax",
        "TXL",
        "Coccinelle",
    ] {
        assert!(
            fixtures_for(target_name)
                .iter()
                .any(|fixture| fixture.transform_expectation().is_some()),
            "{target_name} should include a transform-expectation fixture"
        );
    }

    assert!(fixtures_for("srcML")
        .iter()
        .any(|fixture| fixture.provenance().contains("test/parser/testsuite")));
    assert!(fixtures_for("difftastic")
        .iter()
        .any(|fixture| fixture.provenance().contains("sample_files")));
    assert!(fixtures_for("Coccinelle")
        .iter()
        .any(|fixture| fixture.provenance().contains(".cocci")));
    assert!(fixtures_for("Universal Dependencies")
        .iter()
        .any(|fixture| fixture.provenance().contains("Universal Dependencies")));
    assert!(fixtures_for("LanguageTool")
        .iter()
        .any(|fixture| fixture.verification_expectation()
            == ParityVerificationExpectation::Recoverable));
}

fn fixtures_for(target_name: &str) -> Vec<&meta_language::ParityFixture> {
    PARITY_FIXTURES
        .iter()
        .filter(|fixture| fixture.target_name() == target_name)
        .collect()
}

fn assert_fixture_named(fixtures: &[&meta_language::ParityFixture], expected_name: &str) {
    assert!(
        fixtures
            .iter()
            .any(|fixture| fixture.name().contains(expected_name)),
        "missing fixture name containing {expected_name:?}"
    );
}
