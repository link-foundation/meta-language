use super::issue_195_observations::{Observation, record};
use meta_language::language_catalog::identify_language_with_threshold;
use meta_language::{LinkNetwork, LinkType, ParseConfiguration, identify_language};
use serde::Deserialize;
use unicode_normalization::UnicodeNormalization;

const FIXTURE_FILE: &str = "parity/fixtures/language-identification.lino";
const FIXTURE: &str = include_str!("../../../parity/fixtures/language-identification.lino");

#[derive(Deserialize)]
struct Fixture {
    ranking: RankingFixture,
    cyrillic: CyrillicFixture,
    mixed: Vec<MixedFixture>,
    short: ShortFixture,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct RankingFixture {
    source: String,
    candidates: Vec<(String, f64)>,
    rejecting_threshold: f64,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct CyrillicFixture {
    russian: String,
    unsupported: Vec<(String, String)>,
    foreign_letters: String,
}

#[derive(Deserialize)]
struct MixedFixture {
    source: String,
    spans: Vec<(usize, usize, String, Option<String>)>,
}

#[derive(Deserialize)]
struct ShortFixture {
    unknown: Vec<String>,
    known: Vec<(String, String)>,
}

fn observe(section: &str, assertions: &[&str], test_name: &str) {
    let requirement_id = format!(
        "I195-LANGUAGE-IDENTIFICATION-{}",
        section.to_ascii_uppercase()
    );
    let fixture_id = format!("language-identification:{section}");
    record(&Observation {
        requirement_id: &requirement_id,
        suffix: "behavior",
        fixture_id: &fixture_id,
        fixture_file: FIXTURE_FILE,
        assertions,
        test_name,
    });
}

#[test]
fn package_entry_point_identifies_an_english_sentence() {
    let fixture: Fixture = super::project_data::read_project_data(FIXTURE).unwrap();
    assert_eq!(
        identify_language(&fixture.ranking.source).language,
        Some("English")
    );
    observe(
        "export",
        &["packageEntryPointIdentifiesLanguage"],
        "package_entry_point_identifies_an_english_sentence",
    );
}

#[test]
fn ranked_candidates_remain_visible_after_threshold_rejection() {
    let fixture: Fixture = super::project_data::read_project_data(FIXTURE).unwrap();
    let case = fixture.ranking;
    let report = identify_language(&case.source);
    assert_eq!(report.candidates.len(), case.candidates.len());
    for (candidate, (language, confidence)) in report.candidates.iter().zip(&case.candidates) {
        assert_eq!(candidate.language, language);
        assert!((0.0..=1.0).contains(&candidate.confidence));
        assert!((candidate.confidence - confidence).abs() < 1e-14);
    }
    let rejected =
        identify_language_with_threshold(&case.source, case.rejecting_threshold).unwrap();
    assert_eq!(rejected.language, None);
    assert_eq!(rejected.spans[0].language, None);
    assert_eq!(rejected.candidates, report.candidates);
    assert_eq!(rejected.spans[0].candidates, report.spans[0].candidates);
    let boundary =
        identify_language_with_threshold(&case.source, report.candidates[0].confidence).unwrap();
    assert_eq!(boundary.language, Some("English"));
    for threshold in [-0.1, 1.1, f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        assert!(identify_language_with_threshold(&case.source, threshold).is_err());
    }
    observe(
        "ranking",
        &["candidatesRankedWithConfidence", "thresholdSelectsUnknown"],
        "ranked_candidates_remain_visible_after_threshold_rejection",
    );
}

#[test]
fn distinguishes_russian_from_unsupported_cyrillic_languages() {
    let fixture: Fixture = super::project_data::read_project_data(FIXTURE).unwrap();
    let case = fixture.cyrillic;
    let russian_report = identify_language(&case.russian);
    assert_eq!(russian_report.language, Some("Russian"));
    let decomposed: String = case.russian.nfd().collect();
    assert_eq!(
        identify_language(&decomposed).candidates,
        russian_report.candidates
    );

    let expect_unknown = |source: &str, label: &str| {
        let report = identify_language_with_threshold(source, 0.0).unwrap();
        assert_eq!(report.language, None, "{label}");
        assert!(report.candidates.is_empty(), "{label}");
        assert_eq!(report.spans.len(), 1, "{label}");
        assert_eq!(report.spans[0].language, None, "{label}");
    };
    for (language, source) in &case.unsupported {
        expect_unknown(source, language);
    }
    for letter in case.foreign_letters.chars() {
        let label = letter.to_string();
        expect_unknown(&format!("{} {letter}", case.russian), &label);
        expect_unknown(
            &format!("{} {}", case.russian, letter.to_uppercase()),
            &label,
        );
    }
    observe(
        "cyrillic",
        &[
            "russianEvidenceRetained",
            "unsupportedCyrillicRemainsUnknown",
        ],
        "distinguishes_russian_from_unsupported_cyrillic_languages",
    );
}

#[test]
fn mixed_scripts_expose_minorities_and_original_byte_spans() {
    let fixture: Fixture = super::project_data::read_project_data(FIXTURE).unwrap();
    for case in &fixture.mixed {
        let report = identify_language(&case.source);
        assert_eq!(report.spans.len(), case.spans.len());
        for (span, (start, end, text, language)) in report.spans.iter().zip(&case.spans) {
            assert_eq!((span.start, span.end), (*start, *end));
            assert_eq!(&case.source[span.start..span.end], text);
            assert_eq!(span.language, language.as_deref());
        }
    }
    let report = identify_language(&fixture.mixed[0].source);
    assert_eq!(report.language, None);
    assert!(
        report
            .candidates
            .iter()
            .any(|candidate| candidate.language == "Russian")
    );
    assert!(
        report.spans[0]
            .candidates
            .iter()
            .any(|candidate| candidate.language == "English")
    );
    observe(
        "mixed",
        &["mixedScriptSpansHaveLanguagesAndByteOffsets"],
        "mixed_scripts_expose_minorities_and_original_byte_spans",
    );
}

#[test]
fn short_neutral_and_bounded_inputs_follow_documented_policy() {
    let fixture: Fixture = super::project_data::read_project_data(FIXTURE).unwrap();
    for source in fixture.short.unknown {
        let report = identify_language_with_threshold(&source, 0.0).unwrap();
        assert_eq!(report.language, None, "{source}");
        assert!(report.candidates.is_empty(), "{source}");
    }
    for (source, language) in fixture.short.known {
        assert_eq!(
            identify_language(&source).language,
            Some(language.as_str()),
            "{source}"
        );
    }
    let long = "👋".repeat(2048) + &fixture.ranking.source;
    assert!(identify_language(&long).spans.is_empty());
    observe(
        "short",
        &["shortAndNonTextInputsUseDocumentedPolicy"],
        "short_neutral_and_bounded_inputs_follow_documented_policy",
    );
}

#[test]
fn parse_annotations_identify_supported_languages_like_the_javascript_runtime() {
    for (text, expected) in [
        ("Hawaii is a state.", Some("English")),
        ("Hawaii est un etat.\n", Some("French")),
        ("Hawaii e um estado.\n", Some("Portuguese")),
        ("Гавайи это штат.", Some("Russian")),
        ("مرحبا.\n", Some("Urdu")),
        ("سلام۔\n", Some("Modern Standard Arabic")),
        ("我喜欢学习。", Some("Mandarin Chinese")),
        ("আমি বাড়ি যাই।", Some("Bengali")),
        ("12345", None),
        ("ok", None),
        ("Ωμέγα", None),
        ("", None),
    ] {
        for declared in ["English", "Hindi"] {
            let network = LinkNetwork::parse(text, declared, ParseConfiguration::default());
            let detected = network
                .links()
                .filter(|link| link.metadata().link_type() == Some(LinkType::Language))
                .find_map(|link| link.metadata().language());
            assert_eq!(
                detected,
                expected.or(Some(declared)),
                "{text:?} ({declared})"
            );
        }
    }
}
