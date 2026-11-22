use std::cmp::Reverse;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::sync::LazyLock;

use crate::grammar::feature_runtime::{
    FeatureGrammarParser, FeatureParseOptions, GrammarSearchMatch, GrammarSearchOptions,
    GrammarSearchSpan, compile_feature_grammar,
};
use crate::grammar::interchange::parse_grammar_links;

mod value_policy;
use value_policy::{calendar_month_days, preferred_grouping_delimiter};

const NUMBER_WORD_RULES: [&str; 7] = [
    "small_number_word",
    "ones_number_word",
    "tens_number_word",
    "hundreds_number_word",
    "large_number_word_1",
    "large_number_word_2",
    "large_number_word_3",
];
const VALUE_RULES: [&str; 9] = [
    "duration",
    "amount",
    "money_range",
    "number_range",
    "date_range",
    "money_amount",
    "full_date",
    "month_date",
    "short_date",
];
const CAPTURE_NAMES: [&str; 19] = [
    "numeral",
    "scale",
    "first_amount",
    "last_amount",
    "currency",
    "first_money",
    "last_money",
    "duration_amount",
    "duration_unit",
    "duration_comparison",
    "first_date",
    "last_date",
    "calendar_year",
    "calendar_month",
    "calendar_day",
    "month_name",
    "open_date",
    "amount",
    "money_amount",
];

type Vocabulary = HashMap<&'static str, HashMap<String, &'static str>>;

struct ValueProfile {
    locale: &'static str,
    vocabulary: Vocabulary,
    parser: FeatureGrammarParser,
}

impl ValueProfile {
    fn new(locale: &'static str, grammar: &str) -> Self {
        let mut vocabulary = Vocabulary::new();
        for line in include_str!("../../data/natural-language-values/lexicon.lino").lines() {
            let mut fields = line.trim_matches(['(', ')']).split_ascii_whitespace();
            let Some(kind) = fields.next() else { continue };
            let Some(owner) = fields.next() else { continue };
            if owner != "common" && owner != locale {
                continue;
            }
            let Some(value) = fields.next() else { continue };
            let entry = vocabulary.entry(kind).or_default();
            entry.extend(fields.map(|word| (word.to_lowercase(), value)));
        }
        let grammar = parse_grammar_links(grammar).expect("shipped value grammar");
        let parser = compile_feature_grammar(&grammar, None, FeatureParseOptions::default())
            .expect("compilable value grammar");
        Self {
            locale,
            vocabulary,
            parser,
        }
    }

    fn word(&self, kind: &str, spelling: &str) -> Option<&'static str> {
        self.vocabulary
            .get(kind)?
            .get(&spelling.to_lowercase())
            .copied()
    }
}

static RUSSIAN: LazyLock<ValueProfile> = LazyLock::new(|| {
    ValueProfile::new(
        "ru",
        include_str!("../../data/natural-language-values/ru.lino"),
    )
});
static ENGLISH: LazyLock<ValueProfile> = LazyLock::new(|| {
    ValueProfile::new(
        "en",
        include_str!("../../data/natural-language-values/en.lino"),
    )
});
static COMMON: LazyLock<ValueProfile> = LazyLock::new(|| {
    ValueProfile::new(
        "common",
        include_str!("../../data/natural-language-values/common.lino"),
    )
});
static CURRENCIES: LazyLock<HashSet<&'static str>> = LazyLock::new(|| {
    include_str!("../../data/natural-language-values/currencies.lino")
        .trim()
        .trim_matches(['(', ')'])
        .split_ascii_whitespace()
        .skip(2)
        .collect()
});
static WORD_SEPARATOR: LazyLock<regex::Regex> =
    LazyLock::new(|| regex::Regex::new(r"[^\p{L}\p{M}]+").expect("value word separator"));

#[derive(Clone, Copy, Default)]
struct ExactDecimal {
    coefficient: i128,
    places: u32,
}

impl ExactDecimal {
    fn parse(spelling: &str) -> Option<Self> {
        let (whole, fraction) = spelling.split_once('.').unwrap_or((spelling, ""));
        Some(Self {
            coefficient: format!("{whole}{fraction}").parse().ok()?,
            places: u32::try_from(fraction.len()).ok()?,
        })
    }

    fn add(self, other: Self) -> Self {
        let places = self.places.max(other.places);
        let coefficient = self.coefficient * 10_i128.pow(places - self.places)
            + other.coefficient * 10_i128.pow(places - other.places);
        Self {
            coefficient,
            places,
        }
    }

    fn scale(self, factor: i128) -> Self {
        Self {
            coefficient: self.coefficient * factor,
            ..self
        }
    }

    fn or_one(self) -> Self {
        if self.coefficient == 0 {
            Self {
                coefficient: 1,
                places: 0,
            }
        } else {
            self
        }
    }
}

impl fmt::Display for ExactDecimal {
    fn fmt(&self, output: &mut fmt::Formatter<'_>) -> fmt::Result {
        let sign = if self.coefficient < 0 { "-" } else { "" };
        let places = self.places as usize;
        let digits = format!(
            "{:0width$}",
            self.coefficient.unsigned_abs(),
            width = places + 1
        );
        let (integer, fraction) = digits.split_at(digits.len() - places);
        let fraction = fraction.trim_end_matches('0');
        write!(output, "{sign}{integer}")?;
        if !fraction.is_empty() {
            write!(output, ".{fraction}")?;
        }
        Ok(())
    }
}

fn normalize_numeral(text: &str, locale: &str) -> String {
    let signed = text
        .replace([' ', '\u{a0}', '\u{202f}'], "")
        .replace('−', "-");
    let comma = signed.rfind(',');
    let period = signed.rfind('.');
    if let (Some(comma), Some(period)) = (comma, period) {
        return if comma > period {
            signed.replace('.', "").replace(',', ".")
        } else {
            signed.replace(',', "")
        };
    }
    let delimiter = if comma.is_some() { ',' } else { '.' };
    let sections: Vec<_> = signed.split(delimiter).collect();
    if sections.len() == 1 {
        return signed;
    }
    let unsigned = sections[0].trim_start_matches(['+', '-']);
    if sections.len() > 2
        || (preferred_grouping_delimiter(locale, delimiter)
            && matches!(unsigned.as_bytes(), [b'1'..=b'9', ..])
            && unsigned.len() <= 3
            && sections[1].len() == 3)
    {
        return sections.concat();
    }
    sections.join(".")
}

fn within<'capture>(
    matched: &'capture GrammarSearchMatch,
    range: &GrammarSearchSpan,
    name: &str,
) -> impl Iterator<Item = &'capture GrammarSearchSpan> {
    let (start, end) = (range.start, range.end);
    matched.captures[name]
        .iter()
        .filter(move |span| span.start >= start && span.end <= end)
}

fn read_amount(
    matched: &GrammarSearchMatch,
    range: &GrammarSearchSpan,
    profile: &ValueProfile,
) -> Option<ExactDecimal> {
    if let Some(numeral) = within(matched, range, "numeral").next() {
        let scale = within(matched, range, "scale")
            .next()
            .and_then(|span| profile.word("scale", &span.text))
            .map_or(Some(1), |value| value.parse().ok())?;
        return Some(
            ExactDecimal::parse(&normalize_numeral(&numeral.text, profile.locale))?.scale(scale),
        );
    }
    let mut words = NUMBER_WORD_RULES
        .iter()
        .flat_map(|name| within(matched, range, name))
        .collect::<Vec<_>>();
    words.sort_by_key(|span| span.start);
    let mut total = ExactDecimal::default();
    let mut chunk = ExactDecimal::default();
    for span in words {
        let word = profile.word("number", &span.text)?;
        let integer = word.parse::<i128>().unwrap_or_default();
        if integer >= 1000 {
            total = total.add(chunk.or_one().scale(integer));
            chunk = ExactDecimal::default();
        } else if integer == 100 && profile.locale == "en" {
            chunk = chunk.or_one().scale(integer);
        } else {
            chunk = chunk.add(ExactDecimal::parse(word)?);
        }
    }
    Some(total.add(chunk))
}

fn currency_defaults(spelling: &str) -> &'static [&'static str] {
    match spelling {
        "$" => &["USD", "CAD", "AUD", "NZD", "HKD", "SGD"],
        "¥" => &["JPY", "CNY"],
        _ => &[],
    }
}

fn read_currency<'text>(
    matched: &'text GrammarSearchMatch,
    profile: &ValueProfile,
) -> Option<&'text str> {
    let mut currency = None;
    let spellings = &matched.captures["currency"];
    for spelling in spellings {
        let candidate = profile
            .word("currency", &spelling.text)
            .or_else(|| CURRENCIES.get(spelling.text.as_str()).copied())?;
        if !currency_defaults(&spelling.text).is_empty() {
            continue;
        }
        if currency.is_some_and(|code| code != candidate) {
            return None;
        }
        currency = Some(candidate);
    }
    for spelling in spellings {
        let candidates = currency_defaults(&spelling.text);
        let Some(&primary) = candidates.first() else {
            continue;
        };
        if currency.is_some_and(|code| !candidates.contains(&code)) {
            return None;
        }
        currency.get_or_insert(primary);
    }
    currency
}

fn read_date(
    matched: &GrammarSearchMatch,
    range: &GrammarSearchSpan,
    profile: &ValueProfile,
) -> Option<String> {
    let part = |name| {
        within(matched, range, name)
            .next()
            .map(|span| span.text.as_str())
    };
    if part("open_date").is_some() {
        return Some("open".to_owned());
    }
    let year = part("calendar_year");
    let month = part("calendar_month").or_else(|| profile.word("month", part("month_name")?));
    let day = part("calendar_day");
    if year == Some("0000") {
        return None;
    }
    let Some(month) = month else {
        return year.map(str::to_owned);
    };
    let month = month.parse::<u8>().ok()?;
    if !(1..=12).contains(&month) {
        return None;
    }
    if let Some(day) = day {
        let day = day.parse::<u8>().ok()?;
        let year_number = year.unwrap_or("2000").parse::<u16>().ok()?;
        if day == 0 || day > calendar_month_days(year_number, month) {
            return None;
        }
        return Some(format!("{}-{month:02}-{day:02}", year.unwrap_or("-")));
    }
    Some(format!("{}-{month:02}", year?))
}

fn read_duration(matched: &GrammarSearchMatch, profile: &ValueProfile) -> Option<String> {
    let mut months = ExactDecimal::default();
    for (amount, unit) in matched.captures["duration_amount"]
        .iter()
        .zip(&matched.captures["duration_unit"])
    {
        let factor = profile.word("unit", &unit.text)?.parse().ok()?;
        months = months.add(read_amount(matched, amount, profile)?.scale(factor));
    }
    let comparison = matched.captures["duration_comparison"]
        .first()
        .map(|span| span.text.to_lowercase());
    let property = match comparison.as_deref() {
        None => "months",
        Some(text)
            if ["less", "under", "меньше", "менее"]
                .iter()
                .any(|prefix| text.starts_with(prefix)) =>
        {
            "lessThanMonths"
        }
        Some(_) => "moreThanMonths",
    };
    Some(format!(r#"value:duration {{"{property}":{months}}}"#))
}

fn read_range(matched: &GrammarSearchMatch, profile: &ValueProfile, money: bool) -> Option<String> {
    let (first_kind, last_kind) = if money {
        ("first_money", "last_money")
    } else {
        ("first_amount", "last_amount")
    };
    let first = matched.captures[first_kind].first()?;
    let last = matched.captures[last_kind].first()?;
    let mut left = read_amount(matched, first, profile)?;
    let right = read_amount(matched, last, profile)?;
    if let Some(scale) = within(matched, last, "scale").next()
        && within(matched, first, "numeral").next().is_some()
        && within(matched, first, "scale").next().is_none()
    {
        left = left.scale(profile.word("scale", &scale.text)?.parse().ok()?);
    }
    let (minimum, maximum) = if left.add(right.scale(-1)).coefficient <= 0 {
        (left, right)
    } else {
        (right, left)
    };
    if money {
        let currency = read_currency(matched, profile)?;
        return Some(format!(
            r#"value:money {{"min":{minimum},"max":{maximum},"currency":"{currency}"}}"#
        ));
    }
    Some(format!(
        r#"value:number-range {{"min":{minimum},"max":{maximum}}}"#
    ))
}

fn term_for(rule: &str, matched: &GrammarSearchMatch, profile: &ValueProfile) -> Option<String> {
    let span = &matched.span;
    match rule {
        "amount" => Some(format!(
            "value:number {}",
            read_amount(matched, span, profile)?
        )),
        "number_range" => read_range(matched, profile, false),
        "money_range" => read_range(matched, profile, true),
        "money_amount" => {
            let currency = read_currency(matched, profile)?;
            let amount = read_amount(matched, span, profile)?;
            Some(format!(
                r#"value:money {{"amount":{amount},"currency":"{currency}"}}"#
            ))
        }
        "duration" => read_duration(matched, profile),
        "date_range" => {
            let from = read_date(matched, matched.captures["first_date"].first()?, profile)?;
            let to = read_date(matched, matched.captures["last_date"].first()?, profile)?;
            Some(format!(
                r#"value:date-range {{"from":"{from}","to":"{to}"}}"#
            ))
        }
        _ => Some(format!("value:date {}", read_date(matched, span, profile)?)),
    }
}

/// One normalized value and its original UTF-8 extent.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct NaturalLanguageValue {
    pub start: usize,
    pub end: usize,
    pub term: String,
}

/// Finds normalized values, retaining interpretations that share a source span.
#[must_use]
pub fn natural_language_values(text: &str, language: &str) -> Vec<NaturalLanguageValue> {
    let profile = match language.to_ascii_lowercase().as_str() {
        "russian" | "ru" => &*RUSSIAN,
        "english" | "en" => &*ENGLISH,
        _ => &*COMMON,
    };
    if !text.bytes().any(|byte| byte.is_ascii_digit())
        && !WORD_SEPARATOR
            .split(text)
            .any(|word| profile.word("number", word).is_some())
    {
        return Vec::new();
    }
    let mut results = Vec::new();
    let mut compounds = HashSet::new();
    let mut options = GrammarSearchOptions {
        word_boundaries: true,
        captures: CAPTURE_NAMES
            .iter()
            .chain(&NUMBER_WORD_RULES)
            .map(|name| (*name).to_owned())
            .collect(),
        ..GrammarSearchOptions::default()
    };
    for rule in VALUE_RULES {
        if profile.locale == "common" && rule == "duration" {
            continue;
        }
        options.rule = Some(rule.to_owned());
        let Ok(matches) = profile.parser.find_all(text, &options) else {
            continue;
        };
        for matched in matches {
            let GrammarSearchSpan {
                start,
                end,
                text: spelling,
            } = &matched.span;
            let range = (*start, *end);
            if rule == "amount"
                && NUMBER_WORD_RULES
                    .iter()
                    .any(|name| !matched.captures[*name].is_empty())
            {
                compounds.insert(range);
            }
            if (rule == "number_range" && compounds.contains(&range))
                || (rule == "amount"
                    && spelling.starts_with(['-', '−'])
                    && start
                        .checked_sub(1)
                        .is_some_and(|index| text.as_bytes()[index].is_ascii_digit()))
            {
                continue;
            }
            if let Some(term) = term_for(rule, &matched, profile) {
                results.push(NaturalLanguageValue {
                    start: *start,
                    end: *end,
                    term,
                });
            }
        }
    }
    results.sort_by_key(|value| (value.start, Reverse(value.end)));
    results.dedup();
    results
}
