use meta_language::link_network::text_location::{
    TextEditUnit, TextLocationEdit, TextLocationOptions, TextNormalization, locate,
    word_ngram_overlap,
};
use meta_language::{LinkNetwork, ParseConfiguration, read_structured_data_lino};
use serde_json::{Value, json};

use super::issue_195_observations::{Observation, record};

const FIXTURE: &str = "parity/fixtures/text-location.lino";

fn fixture() -> Value {
    read_structured_data_lino(include_str!("../../../parity/fixtures/text-location.lino"))
        .expect("text location fixture")
}

fn normalization_name(value: TextNormalization) -> &'static str {
    match value {
        TextNormalization::Nfkc => "nfkc",
        TextNormalization::Quotes => "quotes",
        TextNormalization::Dashes => "dashes",
        TextNormalization::Yo => "yo",
        TextNormalization::Case => "case",
        TextNormalization::SoftHyphen => "softHyphen",
        TextNormalization::Whitespace => "whitespace",
    }
}

fn normalizations(value: &Value) -> Vec<TextNormalization> {
    const ALL: [TextNormalization; 7] = [
        TextNormalization::Nfkc,
        TextNormalization::Quotes,
        TextNormalization::Dashes,
        TextNormalization::Yo,
        TextNormalization::Case,
        TextNormalization::SoftHyphen,
        TextNormalization::Whitespace,
    ];
    value
        .as_array()
        .unwrap()
        .iter()
        .map(|value| {
            ALL.into_iter()
                .find(|item| value.as_str() == Some(normalization_name(*item)))
                .unwrap()
        })
        .collect()
}

#[test]
fn normalization_maps_every_quote_back_to_its_original_bytes() {
    for entry in fixture()["exact"].as_array().unwrap() {
        let source = entry["source"].as_str().unwrap();
        let fragment = entry["fragment"].as_str().unwrap();
        let normalize = normalizations(&entry["normalize"]);
        let network =
            LinkNetwork::parse_lossless_text(source, "English", ParseConfiguration::default());
        let matches = network.locate(
            fragment,
            TextLocationOptions {
                normalize: &normalize,
                ..TextLocationOptions::default()
            },
        );
        let observed: Vec<_> = matches.iter().map(|result| json!({
            "text": result.text,
            "normalizations": result.normalizations.iter().copied().map(normalization_name).collect::<Vec<_>>(),
        })).collect();
        assert_eq!(json!(observed), entry["matches"], "{source}");
        let mut from = 0;
        for result in matches {
            assert_eq!(
                result.range.start(),
                from + source[from..].find(result.text.as_str()).unwrap()
            );
            assert_eq!(result.range.end(), result.range.start() + result.text.len());
            assert_eq!(
                &source[result.range.start()..result.range.end()],
                result.text
            );
            assert_eq!(result.distance, 0);
            assert!(result.edits.is_empty());
            from = result.range.start() + result.text.chars().next().unwrap().len_utf8();
        }
    }
    record(&Observation {
        requirement_id: "I195-I243-NORMALIZED-TEXT-LOCATION",
        suffix: "behavior",
        fixture_id: "i195-i243-normalized-text-location",
        fixture_file: FIXTURE,
        assertions: &[
            "allNormalizedMatchesKeepSourceRanges",
            "neededNormalizationsAreReported",
        ],
        test_name: "normalization_maps_every_quote_back_to_its_original_bytes",
    });
}

#[test]
fn approximation_returns_an_edit_script_in_the_chosen_unit() {
    for entry in fixture()["approximate"].as_array().unwrap() {
        let source = entry["source"].as_str().unwrap();
        let fragment = entry["fragment"].as_str().unwrap();
        let unit = if entry["unit"] == "word" {
            TextEditUnit::Word
        } else {
            TextEditUnit::Grapheme
        };
        assert!(locate(source, fragment, TextLocationOptions::default()).is_empty());
        let found = locate(
            source,
            fragment,
            TextLocationOptions {
                max_edits: 1,
                unit,
                ..TextLocationOptions::default()
            },
        )
        .into_iter()
        .find(|result| entry["match"].as_str() == Some(result.text.as_str()))
        .expect("matching source text");
        assert_eq!(found.distance, 1);
        assert_eq!(found.edits.len(), 1);
        let (kind, before, after) = match &found.edits[0] {
            TextLocationEdit::Insert { source } => ("insert", None, Some(source)),
            TextLocationEdit::Delete { fragment } => ("delete", Some(fragment), None),
            TextLocationEdit::Replace { fragment, source } => {
                ("replace", Some(fragment), Some(source))
            }
        };
        assert_eq!(kind, entry["operation"].as_str().unwrap());
        assert_eq!(
            before.map(|unit| unit.text.as_str()),
            entry["original"].as_str()
        );
        assert_eq!(
            after.map(|unit| unit.text.as_str()),
            entry["replacement"].as_str()
        );
        if let Some(unit) = before {
            assert_eq!(&fragment[unit.range.start()..unit.range.end()], unit.text);
        }
        if let Some(unit) = after {
            assert_eq!(&source[unit.range.start()..unit.range.end()], unit.text);
        }
    }
    record(&Observation {
        requirement_id: "I195-I243-APPROXIMATE-TEXT-LOCATION",
        suffix: "behavior",
        fixture_id: "i195-i243-approximate-text-location",
        fixture_file: FIXTURE,
        assertions: &[
            "editBudgetAndUnitControlMatches",
            "reportedEditScriptsKeepSourceRanges",
        ],
        test_name: "approximation_returns_an_edit_script_in_the_chosen_unit",
    });
}

#[test]
fn shared_word_ngrams_keep_every_pair_of_original_ranges() {
    for entry in fixture()["overlap"].as_array().unwrap() {
        let left = entry["left"].as_str().unwrap();
        let right = entry["right"].as_str().unwrap();
        let size = entry["size"].as_u64().unwrap() as usize;
        let normalize = normalizations(&entry["normalize"]);
        let found = word_ngram_overlap(left, right, size, &normalize);
        assert!(found.iter().all(|result| result.words.len() == size));
        let actual: Vec<_> = found
            .into_iter()
            .map(|result| {
                json!({
                    "words":result.words,
                    "left":&left[result.left.start()..result.left.end()],
                    "right":&right[result.right.start()..result.right.end()],
                })
            })
            .collect();
        assert_eq!(json!(actual), entry["matches"]);
    }
    record(&Observation {
        requirement_id: "I195-I243-WORD-NGRAM-OVERLAP",
        suffix: "behavior",
        fixture_id: "i195-i243-word-ngram-overlap",
        fixture_file: FIXTURE,
        assertions: &["wordNgramsReportEverySharedSourcePair"],
        test_name: "shared_word_ngrams_keep_every_pair_of_original_ranges",
    });
}
