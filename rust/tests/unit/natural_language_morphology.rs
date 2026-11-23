use std::path::Path;
use std::process::Command;

use meta_language::link_network::morphology::{
    MorphologyReading, agree_with_number, analyze_morphology, inflect,
};
use meta_language::{LinkNetwork, LinkType, ParseConfiguration};
use serde_json::Value;

use super::issue_195_observations::{Observation, record};
use super::project_data::read_project_data;

const FIXTURE: &str = "parity/fixtures/natural-language-morphology.lino";

fn has_reading(readings: &[MorphologyReading], expected: &[Value]) -> bool {
    readings.iter().any(|reading| {
        let feature = expected[2].as_str().unwrap();
        expected[0].as_str() == Some(reading.lemma.as_str())
            && expected[1].as_str() == Some(reading.upos.as_str())
            && if feature == "_" {
                reading.features.is_empty()
            } else {
                reading.features.iter().any(|tag| tag == feature)
            }
    })
}

#[test]
fn morphology_preserves_linked_readings_and_inflects_attested_lemmas() {
    let fixture: Value = read_project_data(include_str!(
        "../../../parity/fixtures/natural-language-morphology.lino"
    ))
    .expect("morphology fixtures");
    let text = fixture["text"].as_str().unwrap();
    let language = fixture["language"].as_str().unwrap();
    let network = LinkNetwork::parse(text, language, ParseConfiguration::default());
    assert_eq!(network.reconstruct_text(), text);
    assert!(network.verify_full_match(None).is_clean());
    let mut surfaces = Vec::<(&str, Vec<Vec<&str>>)>::new();
    for link in network.links() {
        let metadata = link.metadata();
        if !metadata
            .term()
            .is_some_and(|term| term.starts_with("morphology:"))
        {
            continue;
        }
        let span = metadata.span().unwrap().byte_range();
        let spelling = &text[span.start()..span.end()];
        let (token, children) = link.references().split_first().unwrap();
        let token = network.link(*token).unwrap().metadata();
        assert_eq!(token.link_type(), Some(LinkType::Token));
        assert_eq!(token.term(), Some(spelling));
        let terms: Vec<_> = children
            .iter()
            .map(|&id| {
                let child = network.link(id).unwrap().metadata();
                assert_eq!(child.span().unwrap().byte_range(), span);
                child.term().unwrap()
            })
            .collect();
        for required in ["lemma:", "upos:", "ufeat:"] {
            assert!(terms.iter().any(|term| term.starts_with(required)));
        }
        if let Some((_, readings)) = surfaces.last_mut().filter(|(word, _)| *word == spelling) {
            readings.push(terms);
        } else {
            surfaces.push((spelling, vec![terms]));
        }
    }
    let words = fixture["words"].as_array().unwrap();
    assert_eq!(surfaces.len(), words.len());
    for ((spelling, alternatives), expected) in surfaces.iter().zip(words) {
        let expected = expected.as_array().unwrap();
        assert_eq!(*spelling, expected[0].as_str().unwrap());
        assert!(
            has_reading(&analyze_morphology(spelling, language), &expected[1..]),
            "{spelling}"
        );
        let terms = ["lemma", "upos", "ufeat"]
            .into_iter()
            .zip(&expected[1..])
            .map(|(kind, value)| format!("{kind}:{}", value.as_str().unwrap()))
            .collect::<Vec<_>>();
        assert!(
            alternatives
                .iter()
                .any(|nodes| terms.iter().all(|term| nodes.contains(&term.as_str()))),
            "{spelling}"
        );
    }
    let ambiguous = &fixture["ambiguous"];
    let alternatives = analyze_morphology(ambiguous["word"].as_str().unwrap(), language);
    for reading in ambiguous["readings"].as_array().unwrap() {
        assert!(has_reading(&alternatives, reading.as_array().unwrap()));
    }
    assert!(
        alternatives
            .iter()
            .all(|reading| (0.0..=1.0).contains(&reading.score))
    );
    assert!(
        alternatives
            .windows(2)
            .all(|pair| pair[0].score >= pair[1].score)
    );
    assert!(
        (alternatives
            .iter()
            .map(|reading| reading.score)
            .sum::<f64>()
            - 1.0)
            .abs()
            < 0.000_01
    );
    let unknown = fixture["unknown"].as_array().unwrap();
    let predictions: Vec<_> = analyze_morphology(unknown[0].as_str().unwrap(), language)
        .into_iter()
        .filter(|reading| reading.source == "suffix")
        .collect();
    assert!(has_reading(&predictions, &unknown[1..]));
    assert_eq!(analyze_morphology("елка", language)[0].lemma, "ёлка");
    for row in fixture["inflections"].as_array().unwrap() {
        let features: Vec<_> = row[1]
            .as_object()
            .unwrap()
            .iter()
            .map(|(name, value)| (name.as_str(), value.as_str().unwrap()))
            .collect();
        assert_eq!(
            inflect(
                row[0].as_str().unwrap(),
                &features,
                row[3].as_str().unwrap()
            )
            .as_deref(),
            row[2].as_str()
        );
    }
    assert_eq!(
        inflect("инженер", &[("Case", "Unsupported")], language),
        None
    );
    for row in fixture["numberAgreement"].as_array().unwrap() {
        assert_eq!(
            agree_with_number(
                row[0].as_str().unwrap(),
                row[1].as_f64().unwrap(),
                row[3].as_str().unwrap()
            )
            .as_deref(),
            row[2].as_str()
        );
    }
    assert_eq!(agree_with_number("год", f64::INFINITY, language), None);
    record(&Observation {
        requirement_id: "I195-I236-OPEN-VOCABULARY-MORPHOLOGY",
        suffix: "behavior",
        fixture_id: "i195-i236-open-vocabulary-morphology",
        fixture_file: FIXTURE,
        assertions: &[
            "everyWordHasLinkedReading",
            "ambiguousReadingsAreScored",
            "unseenWordsHaveSuffixReadings",
            "inflectionAndNumberAgreement",
        ],
        test_name: "morphology_preserves_linked_readings_and_inflects_attested_lemmas",
    });
}

#[test]
fn configured_dictionary_loads_only_for_its_language() {
    const CHILD: &str = "META_LANGUAGE_MORPHOLOGY_TEST_CHILD";
    const LAZY_FIXTURE: &str = "parity/fixtures/natural-language-morphology-lazy-loading.lino";
    if std::env::var_os(CHILD).is_some() {
        let lazy: Value = read_project_data(include_str!(
            "../../../parity/fixtures/natural-language-morphology-lazy-loading.lino"
        ))
        .expect("lazy morphology fixture");
        let word = lazy["word"].as_str().unwrap();
        let language = lazy["language"].as_str().unwrap();
        let lemma = format!("lemma:{}", lazy["lemma"].as_str().unwrap());
        let network = LinkNetwork::parse(word, language, ParseConfiguration::default());
        assert_eq!(network.reconstruct_text(), word);
        assert!(network.verify_full_match(None).is_clean());
        assert!(
            network
                .links()
                .any(|link| link.metadata().term() == Some(lemma.as_str()))
        );
        let failure = std::panic::catch_unwind(|| {
            analyze_morphology(
                lazy["dictionaryWord"].as_str().unwrap(),
                lazy["dictionaryLanguage"].as_str().unwrap(),
            )
        })
        .expect_err("the configured dictionary path is a regular file");
        let message = failure.downcast_ref::<String>().unwrap();
        assert!(message.starts_with("configured OpenCorpora morphology data:"));
        return;
    }
    let run = Command::new(std::env::current_exe().expect("morphology test executable"))
        .args([
            "--exact",
            "natural_language_morphology::configured_dictionary_loads_only_for_its_language",
            "--test-threads",
            "1",
        ])
        .env(CHILD, "1")
        .env(
            "META_LANGUAGE_MORPHOLOGY_DATA",
            Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("..")
                .join(LAZY_FIXTURE),
        )
        .output()
        .expect("isolated morphology test");
    let stdout = String::from_utf8_lossy(&run.stdout);
    assert!(
        run.status.success() && stdout.contains("1 passed"),
        "{stdout}\n{}",
        String::from_utf8_lossy(&run.stderr)
    );
    record(&Observation {
        requirement_id: "I195-I236-OPEN-VOCABULARY-MORPHOLOGY-LAZY-DICTIONARY",
        suffix: "behavior",
        fixture_id: "i195-i236-open-vocabulary-morphology-lazy-dictionary",
        fixture_file: LAZY_FIXTURE,
        assertions: &["configuredDictionaryLoadsOnlyForItsLanguage"],
        test_name: "configured_dictionary_loads_only_for_its_language",
    });
}
