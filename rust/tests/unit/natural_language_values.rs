use meta_language::link_network::natural_language_values::natural_language_values;
use meta_language::{LinkNetwork, LinkType, ParseConfiguration};
use serde_json::Value;

use super::issue_195_observations::{Observation, record};
use super::project_data::read_project_data;

const FIXTURE: &str = "parity/fixtures/natural-language-values.lino";
const UNICODE_FIXTURE: &str = "parity/fixtures/natural-language-unicode-decimal-values.lino";
const WRITTEN_MULTIPLIER_FIXTURE: &str =
    "parity/fixtures/natural-language-written-multipliers.lino";
const NAMED_CALENDAR_DATE_FIXTURE: &str =
    "parity/fixtures/natural-language-named-calendar-dates.lino";
const CURRENCY_AMBIGUITY_FIXTURE: &str = "parity/fixtures/natural-language-currency-ambiguity.lino";
const SEPARATOR_AMBIGUITY_FIXTURE: &str =
    "parity/fixtures/natural-language-separator-ambiguity.lino";

fn check_value_fixtures(fixtures: &str) {
    let data: Value = read_project_data(fixtures).expect("value fixtures");
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
        for item in fixture["exactMoney"].as_array().into_iter().flatten() {
            let spelling = item[0].as_str().expect("currency spelling");
            let start = source.find(spelling).expect("original currency spelling");
            let end = start + spelling.len();
            let interpretations = standalone
                .iter()
                .filter(|value| value.start == start && value.end == end)
                .filter_map(|value| value.term.strip_prefix("value:money "))
                .map(|term| serde_json::from_str::<Value>(term).expect("money fields"))
                .collect::<Vec<_>>();
            let expected = item[2]
                .as_array()
                .expect("compatible currencies")
                .iter()
                .map(|currency| {
                    let mut fields = item[1].clone();
                    fields["currency"] = currency.clone();
                    fields
                })
                .collect::<Vec<_>>();
            assert_eq!(interpretations, expected, "{spelling}");
        }
        for item in fixture["exactValues"].as_array().into_iter().flatten() {
            let spelling = item[0].as_str().expect("exact value spelling");
            let kind = item[1].as_str().expect("value kind");
            let start = source.find(spelling).expect("original value spelling");
            let end = start + spelling.len();
            let prefix = format!("value:{kind} ");
            let readings: Vec<_> = standalone
                .iter()
                .filter(|value| value.start == start && value.end == end)
                .filter_map(|value| value.term.strip_prefix(prefix.as_str()))
                .map(|term| {
                    if kind == "number" {
                        Value::String(term.to_owned())
                    } else {
                        serde_json::from_str(term).expect("typed value fields")
                    }
                })
                .collect();
            assert_eq!(
                readings,
                *item[2].as_array().expect("exact readings"),
                "{spelling}"
            );
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

#[test]
fn digit_amounts_with_written_multipliers_preserve_values_and_spans() {
    check_value_fixtures(include_str!(
        "../../../parity/fixtures/natural-language-written-multipliers.lino"
    ));
    record(&Observation {
        requirement_id: "I195-I242-WRITTEN-MULTIPLIERS",
        suffix: "behavior",
        fixture_id: "i195-i242-written-multipliers",
        fixture_file: WRITTEN_MULTIPLIER_FIXTURE,
        assertions: &[
            "writtenMultipliersNormalizeExactValues",
            "writtenMultiplierRangesPreserveEndpointScales",
            "writtenMultipliersKeepOriginalSpans",
        ],
        test_name: "digit_amounts_with_written_multipliers_preserve_values_and_spans",
    });
}

#[test]
fn named_calendar_dates_and_ranges_preserve_valid_days_and_spans() {
    check_value_fixtures(include_str!(
        "../../../parity/fixtures/natural-language-named-calendar-dates.lino"
    ));
    record(&Observation {
        requirement_id: "I195-I242-NAMED-CALENDAR-DATES",
        suffix: "behavior",
        fixture_id: "i195-i242-named-calendar-dates",
        fixture_file: NAMED_CALENDAR_DATE_FIXTURE,
        assertions: &[
            "namedCalendarDatesAndRanges",
            "namedCalendarDateSpansAndValidation",
        ],
        test_name: "named_calendar_dates_and_ranges_preserve_valid_days_and_spans",
    });
}

#[test]
fn currency_symbols_keep_compatible_codes_at_original_spans() {
    check_value_fixtures(include_str!(
        "../../../parity/fixtures/natural-language-currency-ambiguity.lino"
    ));
    record(&Observation {
        requirement_id: "I195-I242-CURRENCY-SYMBOL-ALTERNATIVES",
        suffix: "behavior",
        fixture_id: "i195-i242-currency-symbol-alternatives",
        fixture_file: CURRENCY_AMBIGUITY_FIXTURE,
        assertions: &[
            "currencySymbolsKeepCompatibleCodes",
            "currencyMarkersIntersectAtOriginalSpans",
            "qualifiedCurrencySymbolsKeepTheirFullSpans",
        ],
        test_name: "currency_symbols_keep_compatible_codes_at_original_spans",
    });
}

#[test]
fn ambiguous_number_separators_keep_exact_alternatives_in_compound_values() {
    check_value_fixtures(include_str!(
        "../../../parity/fixtures/natural-language-separator-ambiguity.lino"
    ));
    record(&Observation {
        requirement_id: "I195-I242-NUMERIC-SEPARATOR-ALTERNATIVES",
        suffix: "behavior",
        fixture_id: "i195-i242-numeric-separator-alternatives",
        fixture_file: SEPARATOR_AMBIGUITY_FIXTURE,
        assertions: &[
            "ambiguousSeparatorsPreserveLocaleOrder",
            "compoundValuesKeepDistinctReadings",
            "unambiguousSeparatorsKeepOneReading",
            "separatorReadingsKeepOriginalSpans",
        ],
        test_name: "ambiguous_number_separators_keep_exact_alternatives_in_compound_values",
    });
}
