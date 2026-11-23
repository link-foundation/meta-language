use meta_language::link_network::text_location::{
    TextEditUnit, TextLocationEdit, TextLocationOptions, TextNormalization, locate,
    word_ngram_overlap,
};
use meta_language::{LinkNetwork, ParseConfiguration, read_structured_data_lino};
use serde_json::{Value, json};
use std::collections::BTreeSet;
use unicode_segmentation::UnicodeSegmentation;

use super::issue_195_observations::{Observation, record};

const FIXTURE: &str = "parity/fixtures/text-location.lino";
const CASE_FOLDING_FIXTURE: &str = "parity/fixtures/text-location-case-folding.lino";
const GRAPHEME_FIXTURE: &str = "parity/fixtures/text-location-grapheme-conformance.lino";
const NORMALIZATION_FIXTURE: &str = "parity/fixtures/text-location-normalization-conformance.lino";

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

fn verify_exact_cases(cases: &[Value]) {
    for entry in cases {
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
}

#[test]
fn normalization_maps_every_quote_back_to_its_original_bytes() {
    verify_exact_cases(fixture()["exact"].as_array().unwrap());
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

fn verify_overlap_cases(cases: &[Value]) {
    for entry in cases {
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
}

#[test]
fn shared_word_ngrams_keep_every_pair_of_original_ranges() {
    verify_overlap_cases(fixture()["overlap"].as_array().unwrap());
    record(&Observation {
        requirement_id: "I195-I243-WORD-NGRAM-OVERLAP",
        suffix: "behavior",
        fixture_id: "i195-i243-word-ngram-overlap",
        fixture_file: FIXTURE,
        assertions: &["wordNgramsReportEverySharedSourcePair"],
        test_name: "shared_word_ngrams_keep_every_pair_of_original_ranges",
    });
}

#[test]
fn unicode_full_case_folding_keeps_original_quote_and_ngram_spans() {
    let fixture = read_structured_data_lino(include_str!(
        "../../../parity/fixtures/text-location-case-folding.lino"
    ))
    .expect("text location case folding fixture");
    verify_exact_cases(fixture["exact"].as_array().unwrap());
    verify_overlap_cases(fixture["overlap"].as_array().unwrap());
    record(&Observation {
        requirement_id: "I195-I243-UNICODE-CASE-FOLDING",
        suffix: "behavior",
        fixture_id: "i195-i243-unicode-case-folding",
        fixture_file: CASE_FOLDING_FIXTURE,
        assertions: &[
            "fullCaseFoldingMatchesKeepSourceRanges",
            "caseFoldingOverlapKeepsBothSourceRanges",
        ],
        test_name: "unicode_full_case_folding_keeps_original_quote_and_ngram_spans",
    });
}

#[test]
fn unicode_grapheme_boundaries_and_edits_keep_original_ranges() {
    assert_eq!(
        unicode_segmentation::UNICODE_VERSION,
        (17, 0, 0),
        "uncovered native Unicode grapheme version"
    );
    let mut cases =
        include_str!("../../../parity/fixtures/text-location-grapheme-conformance.lino").lines();
    assert!(
        cases
            .next()
            .unwrap()
            .starts_with("(unicode-grapheme-conformance 17.0.0 ")
    );
    assert_eq!(cases.clone().count(), 766);
    let mut clusters = BTreeSet::new();
    for line in cases {
        let groups = unicode_fixture_groups(line);
        let source = groups.concat();
        let mut offset = 0;
        let expected: Vec<_> = groups
            .iter()
            .map(|cluster| {
                let index = offset;
                offset += cluster.len();
                (index, cluster.as_str())
            })
            .collect();
        assert_eq!(
            source.grapheme_indices(true).collect::<Vec<_>>(),
            expected,
            "{line}"
        );
        clusters.extend(groups);
    }
    for cluster in clusters {
        let prefix = "🚀\u{1}";
        let fragment = format!("{prefix}\u{3}\u{2}");
        let source = format!("{prefix}{cluster}\u{2}");
        let found = locate(
            &source,
            &fragment,
            TextLocationOptions {
                max_edits: 1,
                ..TextLocationOptions::default()
            },
        )
        .into_iter()
        .find(|result| result.range.start() == 0 && result.range.end() == source.len())
        .unwrap_or_else(|| panic!("missing complete source span for {cluster:?}"));
        assert_eq!(found.distance, 1);
        let [
            TextLocationEdit::Replace {
                fragment: before,
                source: after,
            },
        ] = found.edits.as_slice()
        else {
            panic!("expected one grapheme replacement for {cluster:?}");
        };
        assert_eq!(before.text, "\u{3}");
        assert_eq!(before.range.start(), prefix.len());
        assert_eq!(before.range.end(), prefix.len() + 1);
        assert_eq!(after.text, cluster);
        assert_eq!(after.range.start(), prefix.len());
        assert_eq!(after.range.end(), prefix.len() + cluster.len());
    }
    record(&Observation {
        requirement_id: "I195-I243-UNICODE-GRAPHEME-BOUNDARIES",
        suffix: "behavior",
        fixture_id: "i195-i243-unicode-grapheme-boundaries",
        fixture_file: GRAPHEME_FIXTURE,
        assertions: &[
            "unicode17GraphemeBoundaries",
            "graphemeEditsKeepOriginalSourceRanges",
        ],
        test_name: "unicode_grapheme_boundaries_and_edits_keep_original_ranges",
    });
}

fn unicode_fixture_groups(line: &str) -> Vec<String> {
    line.strip_prefix("(case (")
        .and_then(|case| case.strip_suffix("))"))
        .expect("Unicode conformance case")
        .split(") (")
        .map(|codes| {
            codes
                .split(' ')
                .map(|code| char::from_u32(u32::from_str_radix(code, 16).unwrap()).unwrap())
                .collect()
        })
        .collect()
}

#[test]
fn unicode_compatibility_composition_locates_complete_original_ranges() {
    assert_eq!(
        unicode_normalization::UNICODE_VERSION,
        (17, 0, 0),
        "uncovered native Unicode normalization version"
    );
    let mut cases =
        include_str!("../../../parity/fixtures/text-location-normalization-conformance.lino")
            .lines();
    assert!(
        cases
            .next()
            .unwrap()
            .starts_with("(unicode-normalization-conformance 17.0.0 ")
    );
    assert_eq!(cases.clone().count(), 20_034);
    let prefix = "🚀\u{1}";
    let options = TextLocationOptions {
        normalize: &[TextNormalization::Nfkc],
        ..TextLocationOptions::default()
    };
    for line in cases {
        let samples = unicode_fixture_groups(line);
        let normalized = &samples[0];
        for text in &samples {
            let source = format!("{prefix}{text}\u{2}");
            let end = prefix.len() + text.len();
            let found = locate(&source, normalized, options)
                .into_iter()
                .find(|result| result.range.start() == prefix.len() && result.range.end() == end)
                .unwrap_or_else(|| panic!("missing complete source span for {line}"));
            assert_eq!(&found.text, text, "{line}");
            let required = if text == normalized {
                &[][..]
            } else {
                options.normalize
            };
            assert_eq!(found.normalizations.as_slice(), required, "{line}");
            assert_eq!(found.distance, 0, "{line}");
            assert!(found.edits.is_empty(), "{line}");
        }
    }

    let halfwidth = "ﾡￂ";
    let source = format!("{prefix}{halfwidth}\u{2}");
    let end = prefix.len() + halfwidth.len();
    let matches = locate(&source, "가", options);
    assert_eq!(matches.len(), 1);
    assert_eq!(matches[0].range.start(), prefix.len());
    assert_eq!(matches[0].range.end(), end);
    assert_eq!(matches[0].text, halfwidth);
    assert_eq!(matches[0].normalizations.as_slice(), options.normalize);
    assert!(locate(&source, "ᄀ", options).is_empty());
    let pairs = word_ngram_overlap(&source, "가", 1, options.normalize);
    assert_eq!(pairs.len(), 1);
    assert_eq!(pairs[0].words, ["가"]);
    assert_eq!(pairs[0].left.start(), prefix.len());
    assert_eq!(pairs[0].left.end(), end);
    assert_eq!(pairs[0].right.start(), 0);
    assert_eq!(pairs[0].right.end(), "가".len());
    record(&Observation {
        requirement_id: "I195-I243-UNICODE-COMPATIBILITY-COMPOSITION",
        suffix: "behavior",
        fixture_id: "i195-i243-unicode-compatibility-composition",
        fixture_file: NORMALIZATION_FIXTURE,
        assertions: &[
            "unicode17CompatibilityNormalization",
            "compatibilityCompositionsKeepOriginalSourceRanges",
        ],
        test_name: "unicode_compatibility_composition_locates_complete_original_ranges",
    });
}
