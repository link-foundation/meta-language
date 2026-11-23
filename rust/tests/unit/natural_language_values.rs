use meta_language::link_network::natural_language_values::natural_language_values;
use meta_language::{LinkNetwork, LinkType, ParseConfiguration};

use super::issue_195_observations::{Observation, record};
use super::project_data::read_project_data;

const FIXTURE: &str = "parity/fixtures/natural-language-values.lino";
const UNICODE_FIXTURE: &str = "parity/fixtures/natural-language-unicode-decimal-values.lino";

fn check_value_fixtures(fixtures: &str) {
    let data = read_project_data(fixtures).expect("value fixtures");
    for fixture in data["fixtures"].as_array().expect("fixture list") {
        let source = fixture["source"].as_str().expect("source");
        let language = fixture["language"].as_str().expect("language");
        let network = LinkNetwork::parse(source, language, ParseConfiguration::default());
        assert_eq!(network.reconstruct_text(), source);
        let mut entries = Vec::new();
        for link in network.links() {
            let metadata = link.metadata();
            let Some(term) = metadata.term().filter(|term| term.starts_with("value:")) else {
                continue;
            };
            assert_eq!(metadata.link_type(), Some(LinkType::Semantic));
            assert_eq!(metadata.language(), Some(language));
            let span = metadata.span().expect("original value span").byte_range();
            entries.push((&source[span.start()..span.end()], term));
        }
        let standalone = natural_language_values(source, language);
        assert_eq!(
            standalone
                .iter()
                .map(|value| (&source[value.start..value.end], value.term.as_str()))
                .collect::<Vec<_>>(),
            entries
        );
        for (key, expected) in [("expected", true), ("absent", false)] {
            for item in fixture[key].as_array().into_iter().flatten() {
                let entry = (
                    item[0].as_str().expect("spelling"),
                    item[1].as_str().expect("term"),
                );
                assert_eq!(entries.contains(&entry), expected, "{entry:?}");
            }
        }
    }
}

#[test]
fn natural_language_values_preserve_spans_and_alternatives() {
    check_value_fixtures(include_str!(
        "../../../parity/fixtures/natural-language-values.lino"
    ));
    record(&Observation {
        requirement_id: "I195-I242-NATURAL-LANGUAGE-VALUES",
        suffix: "behavior",
        fixture_id: "i195-i242-natural-language-values",
        fixture_file: FIXTURE,
        assertions: &[
            "localizedExactNumbers",
            "currencyRanges",
            "inflectedDurations",
            "dateAlternatives",
            "utf8SourceSpans",
            "explicitZeroInNumberWordCompounds",
        ],
        test_name: "natural_language_values_preserve_spans_and_alternatives",
    });
}

#[test]
fn unicode_decimal_digits_retain_typed_values_and_original_spans() {
    check_value_fixtures(include_str!(
        "../../../parity/fixtures/natural-language-unicode-decimal-values.lino"
    ));
    record(&Observation {
        requirement_id: "I195-I242-UNICODE-DECIMAL-VALUES",
        suffix: "behavior",
        fixture_id: "i195-i242-unicode-decimal-values",
        fixture_file: UNICODE_FIXTURE,
        assertions: &[
            "unicodeDecimalsAndGrouping",
            "unicodeCalendarValidationAndAlternatives",
            "unicodeValueSourceSpans",
        ],
        test_name: "unicode_decimal_digits_retain_typed_values_and_original_spans",
    });
}
