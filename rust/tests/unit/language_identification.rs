use meta_language::link_network::{InvalidLanguageConfidence, rank_language_identification};
use meta_language::{identify_language, read_structured_data_lino};
use serde_json::json;

use super::issue_195_observations::{Observation, record};

#[test]
fn identifies_supported_languages_like_the_javascript_runtime() {
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
        assert_eq!(identify_language(text), expected, "{text:?}");
    }
}

#[test]
fn ranks_cyrillic_languages_and_reports_mixed_script_byte_spans() {
    let fixture = read_structured_data_lino(include_str!(
        "../../../parity/fixtures/ranked-language-identification.lino"
    ))
    .expect("ranked language identification fixture");
    for entry in fixture["inputs"].as_array().expect("inputs") {
        let source = entry["text"].as_str().expect("source text");
        let result = rank_language_identification(source, None).unwrap();
        assert_eq!(result.language, entry["language"].as_str(), "{source}");
        assert_eq!(
            result
                .candidates
                .first()
                .map(|candidate| candidate.language),
            entry["firstCandidate"].as_str(),
            "{source}"
        );
        assert!(
            result
                .candidates
                .iter()
                .all(|candidate| (0.0..=1.0).contains(&candidate.confidence)
                    && candidate.confidence > 0.0)
        );
        assert!(
            result
                .candidates
                .windows(2)
                .all(|pair| pair[0].confidence >= pair[1].confidence)
        );
        let strict = rank_language_identification(source, Some(1.0)).unwrap();
        assert_eq!(strict.language, None, "{source}");
        assert_eq!(strict.candidates, result.candidates, "{source}");
    }
    for entry in fixture["mixed"].as_array().expect("mixed inputs") {
        let source = entry["text"].as_str().expect("source text");
        let result = rank_language_identification(source, None).unwrap();
        let spans: Vec<_> = result
            .spans
            .iter()
            .map(|span| {
                json!({
                    "script": span.script,
                    "start": span.start,
                    "end": span.end,
                    "language": span.language,
                    "text": &source[span.start..span.end],
                })
            })
            .collect();
        assert_eq!(json!(spans), entry["spans"], "{source}");
    }
    for threshold in [-1.0, 1.1, f64::NAN] {
        assert_eq!(
            rank_language_identification("text", Some(threshold)),
            Err(InvalidLanguageConfidence)
        );
    }
    record(&[Observation {
        requirement_id: "I195-NATURAL-LANGUAGE-IDENTIFICATION-RANKING",
        fixture_id: "i195-natural-language-identification-ranking",
        fixture_file: "parity/fixtures/ranked-language-identification.lino",
        assertions: &[
            "rankedCyrillicModels",
            "explicitUnknownThreshold",
            "shortTextEvidence",
            "utf8ScriptSpans",
        ],
    }]);
}
