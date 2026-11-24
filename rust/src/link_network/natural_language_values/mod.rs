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
const CAPTURE_NAMES: [&str; 20] = [
    "numeral",
    "scale",
    "word_scale",
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
static CURRENCY_SYMBOL_ALTERNATIVES: LazyLock<HashMap<String, Vec<&'static str>>> =
    LazyLock::new(|| {
        include_str!("../../data/natural-language-values/currency-symbol-alternatives.lino")
            .lines()
            .skip(1)
            .map(|line| {
                let mut fields = line
                    .trim_matches(['(', ')'])
                    .split_ascii_whitespace()
                    .skip(1);
                let symbol = fields.next().expect("currency symbol");
                (symbol.to_lowercase(), fields.collect())
            })
            .collect()
    });
static DECIMAL_DIGITS: LazyLock<HashMap<char, char>> = LazyLock::new(|| {
    include_str!("../../data/natural-language-values/decimal-digits.lino")
        .trim()
        .trim_matches(['(', ')'])
        .split_ascii_whitespace()
        .skip(3)
        .flat_map(|zero| {
            let start = u32::from_str_radix(zero, 16).expect("decimal digit block start");
            (0..=9).map(move |digit| {
                (
                    char::from_u32(start + digit).expect("decimal digit code point"),
                    char::from_digit(digit, 10).expect("ASCII decimal digit"),
                )
            })
        })
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

fn normalize_decimal_digits(text: &str) -> String {
    text.chars()
        .map(|character| DECIMAL_DIGITS.get(&character).copied().unwrap_or(character))
        .collect()
}

fn normalize_numerals(text: &str, locale: &str) -> Vec<String> {
    let normalized = normalize_decimal_digits(text).replace('−', "-");
    let signed = normalized.replace([' ', '\u{a0}', '\u{202f}', '٬'], "");
    if signed.contains('٫') {
        return vec![signed.replace('٫', ".")];
    }
    let comma = signed.rfind(',');
    let period = signed.rfind('.');
    if let (Some(comma), Some(period)) = (comma, period) {
        return vec![if comma > period {
            signed.replace('.', "").replace(',', ".")
        } else {
            signed.replace(',', "")
        }];
    }
    let delimiter = if comma.is_some() { ',' } else { '.' };
    let sections: Vec<_> = signed.split(delimiter).collect();
    if sections.len() == 1 {
        return vec![signed];
    }
    if sections.len() > 2 {
        return vec![sections.concat()];
    }
    let decimal = sections.join(".");
    let unsigned = sections[0].trim_start_matches(['+', '-']);
    if normalized != signed
        || sections[1].len() != 3
        || unsigned.len() > 3
        || !matches!(unsigned.as_bytes(), [b'1'..=b'9', ..])
    {
        return vec![decimal];
    }
    let grouped = sections.concat();
    if preferred_grouping_delimiter(locale, delimiter) {
        vec![grouped, decimal]
    } else {
        vec![decimal, grouped]
    }
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

fn read_scale(
    matched: &GrammarSearchMatch,
    range: &GrammarSearchSpan,
    profile: &ValueProfile,
) -> Option<&'static str> {
    within(matched, range, "scale")
        .next()
        .and_then(|span| profile.word("scale", &span.text))
        .or_else(|| {
            within(matched, range, "word_scale")
                .next()
                .and_then(|span| profile.word("number", &span.text))
        })
}

fn read_amounts(
    matched: &GrammarSearchMatch,
    range: &GrammarSearchSpan,
    profile: &ValueProfile,
) -> Option<Vec<ExactDecimal>> {
    if let Some(numeral) = within(matched, range, "numeral").next() {
        let scale =
            read_scale(matched, range, profile).map_or(Some(1), |value| value.parse().ok())?;
        return normalize_numerals(&numeral.text, profile.locale)
            .iter()
            .map(|spelling| ExactDecimal::parse(spelling).map(|amount| amount.scale(scale)))
            .collect();
    }
    let mut words = NUMBER_WORD_RULES
        .iter()
        .flat_map(|name| within(matched, range, name))
        .collect::<Vec<_>>();
    words.sort_by_key(|span| span.start);
    let mut total = ExactDecimal::default();
    let mut chunk: Option<ExactDecimal> = None;
    let implicit_one = ExactDecimal {
        coefficient: 1,
        places: 0,
    };
    for span in words {
        let word = profile.word("number", &span.text)?;
        let integer = word.parse::<i128>().unwrap_or_default();
        if integer >= 1000 {
            total = total.add(chunk.unwrap_or(implicit_one).scale(integer));
            chunk = None;
        } else if integer == 100 && profile.locale == "en" {
            chunk = Some(chunk.unwrap_or(implicit_one).scale(integer));
        } else {
            chunk = Some(chunk.unwrap_or_default().add(ExactDecimal::parse(word)?));
        }
    }
    Some(vec![chunk.map_or(total, |chunk| total.add(chunk))])
}

fn read_currency_codes(
    matched: &GrammarSearchMatch,
    profile: &ValueProfile,
) -> Option<Vec<&'static str>> {
    let mut candidates: Option<Vec<&str>> = None;
    for spelling in &matched.captures["currency"] {
        let spelling = spelling.text.as_str();
        let explicit;
        let codes = if let Some(alternatives) =
            CURRENCY_SYMBOL_ALTERNATIVES.get(&spelling.to_lowercase())
        {
            alternatives.as_slice()
        } else {
            explicit = [profile
                .word("currency", spelling)
                .or_else(|| CURRENCIES.get(spelling).copied())?];
            &explicit
        };
        if let Some(candidates) = &mut candidates {
            candidates.retain(|code| codes.contains(code));
            if candidates.is_empty() {
                return None;
            }
        } else {
            candidates = Some(codes.to_vec());
        }
    }
    candidates
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
    let year = part("calendar_year").map(normalize_decimal_digits);
    let month = part("calendar_month")
        .map(normalize_decimal_digits)
        .or_else(|| {
            profile
                .word("month", part("month_name")?)
                .map(str::to_owned)
        });
    let day = part("calendar_day").map(normalize_decimal_digits);
    if year.as_deref() == Some("0000") {
        return None;
    }
    let Some(month) = month else {
        return year;
    };
    let month = month.parse::<u8>().ok()?;
    if !(1..=12).contains(&month) {
        return None;
    }
    if let Some(day) = day {
        let day = day.parse::<u8>().ok()?;
        let year_number = year.as_deref().unwrap_or("2000").parse::<u16>().ok()?;
        if day == 0 || day > calendar_month_days(year_number, month) {
            return None;
        }
        return Some(format!(
            "{}-{month:02}-{day:02}",
            year.as_deref().unwrap_or("-")
        ));
    }
    Some(format!("{}-{month:02}", year?))
}

fn read_durations(matched: &GrammarSearchMatch, profile: &ValueProfile) -> Option<Vec<String>> {
    let mut readings = vec![ExactDecimal::default()];
    for (amount, unit) in matched.captures["duration_amount"]
        .iter()
        .zip(&matched.captures["duration_unit"])
    {
        let factor = profile.word("unit", &unit.text)?.parse().ok()?;
        let additions: Vec<_> = read_amounts(matched, amount, profile)?
            .into_iter()
            .map(|value| value.scale(factor))
            .collect();
        readings = readings
            .into_iter()
            .flat_map(|months| additions.iter().map(move |&addition| months.add(addition)))
            .collect();
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
    Some(
        readings
            .into_iter()
            .map(|months| format!(r#"value:duration {{"{property}":{months}}}"#))
            .collect(),
    )
}

fn read_ranges(
    matched: &GrammarSearchMatch,
    profile: &ValueProfile,
    money: bool,
) -> Option<Vec<(ExactDecimal, ExactDecimal)>> {
    let (first_kind, last_kind) = if money {
        ("first_money", "last_money")
    } else {
        ("first_amount", "last_amount")
    };
    let first = matched.captures[first_kind].first()?;
    let last = matched.captures[last_kind].first()?;
    let mut first_amounts = read_amounts(matched, first, profile)?;
    let last_amounts = read_amounts(matched, last, profile)?;
    if let Some(scale) = read_scale(matched, last, profile)
        && within(matched, first, "numeral").next().is_some()
        && read_scale(matched, first, profile).is_none()
    {
        let scale = scale.parse().ok()?;
        for amount in &mut first_amounts {
            *amount = amount.scale(scale);
        }
    }
    Some(
        first_amounts
            .into_iter()
            .flat_map(|left| {
                last_amounts.iter().map(move |&right| {
                    if left.add(right.scale(-1)).coefficient <= 0 {
                        (left, right)
                    } else {
                        (right, left)
                    }
                })
            })
            .collect(),
    )
}

fn terms_for(
    rule: &str,
    matched: &GrammarSearchMatch,
    profile: &ValueProfile,
) -> Option<Vec<String>> {
    let span = &matched.span;
    let terms = match rule {
        "amount" => read_amounts(matched, span, profile)?
            .into_iter()
            .map(|amount| format!("value:number {amount}"))
            .collect(),
        "number_range" => read_ranges(matched, profile, false)?
            .into_iter()
            .map(|(minimum, maximum)| {
                format!(r#"value:number-range {{"min":{minimum},"max":{maximum}}}"#)
            })
            .collect(),
        "money_range" => {
            let currencies = read_currency_codes(matched, profile)?;
            read_ranges(matched, profile, true)?
                .into_iter()
                .flat_map(|(minimum, maximum)| {
                    currencies.iter().map(move |currency| {
                        format!(r#"value:money {{"min":{minimum},"max":{maximum},"currency":"{currency}"}}"#)
                    })
                })
                .collect()
        }
        "money_amount" => {
            let currencies = read_currency_codes(matched, profile)?;
            read_amounts(matched, span, profile)?
                .into_iter()
                .flat_map(|amount| {
                    currencies.iter().map(move |currency| {
                        format!(r#"value:money {{"amount":{amount},"currency":"{currency}"}}"#)
                    })
                })
                .collect()
        }
        "duration" => read_durations(matched, profile)?,
        "date_range" => {
            let from = read_date(matched, matched.captures["first_date"].first()?, profile)?;
            let to = read_date(matched, matched.captures["last_date"].first()?, profile)?;
            vec![format!(
                r#"value:date-range {{"from":"{from}","to":"{to}"}}"#
            )]
        }
        _ => vec![format!("value:date {}", read_date(matched, span, profile)?)],
    };
    Some(terms)
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
    if !text
        .chars()
        .any(|character| DECIMAL_DIGITS.contains_key(&character))
        && !WORD_SEPARATOR
            .split(text)
            .any(|word| profile.word("number", word).is_some())
    {
        return Vec::new();
    }
    let mut results = Vec::new();
    let mut seen = HashSet::new();
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
                    && text[..*start]
                        .chars()
                        .next_back()
                        .is_some_and(|character| DECIMAL_DIGITS.contains_key(&character)))
            {
                continue;
            }
            for term in terms_for(rule, &matched, profile).into_iter().flatten() {
                if seen.insert((*start, *end, term.clone())) {
                    results.push(NaturalLanguageValue {
                        start: *start,
                        end: *end,
                        term,
                    });
                }
            }
        }
    }
    results.sort_by_key(|value| (value.start, Reverse(value.end)));
    results
}
