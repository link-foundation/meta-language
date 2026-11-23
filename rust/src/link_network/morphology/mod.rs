//! Lemmas, Universal Dependencies morphology, and inverse inflection.

mod agreement_policy;
mod dictionary;
mod english;

use self::agreement_policy::russian_number_agreement_kind;
use self::dictionary::RussianDictionary;
use self::english::english_morphology;
use regex::Regex;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, LazyLock, Mutex};
use unicode_normalization::UnicodeNormalization;

const RULES: &str = include_str!("../../data/morphology/rules.lino");
const RUSSIAN: &str = include_str!("../../data/morphology/russian.lino");

/// One attested or predicted reading of a source word.
#[derive(Clone, Debug, PartialEq)]
pub struct MorphologyReading {
    /// Dictionary headword or predicted lemma.
    pub lemma: String,
    /// Universal Dependencies part of speech.
    pub upos: String,
    /// Universal Dependencies feature assignments in alphabetical order.
    pub features: Vec<String>,
    /// Relative score among this word's readings.
    pub score: f64,
    /// Reading provenance, such as `dictionary` or `suffix`.
    pub source: &'static str,
}

#[derive(Clone)]
struct WeightedReading {
    lemma: String,
    upos: String,
    features: Vec<String>,
    weight: u32,
    source: &'static str,
}

#[derive(Default)]
struct MorphologyProfile {
    forms: HashMap<String, Vec<WeightedReading>>,
    folded_forms: HashMap<String, Vec<WeightedReading>>,
    inflections: HashMap<String, Vec<(String, WeightedReading)>>,
    parts: HashMap<&'static str, Vec<&'static str>>,
    features: HashMap<&'static str, &'static str>,
    suffixes: Vec<Vec<&'static str>>,
    agreement: HashMap<&'static str, &'static str>,
    dictionary: Option<RussianDictionary>,
}

fn fields(line: &'static str) -> impl Iterator<Item = &'static str> {
    line[1..line.len() - 1].split_ascii_whitespace()
}

fn locale(language: &str) -> &'static str {
    match language.to_ascii_lowercase().as_str() {
        "arabic" | "modern standard arabic" | "ar" => "ar",
        "bengali" | "bn" => "bn",
        "chinese" | "mandarin" | "mandarin chinese" | "zh" => "zh",
        "english" | "en" => "en",
        "french" | "fr" => "fr",
        "hindi" | "hi" => "hi",
        "portuguese" | "pt" => "pt",
        "russian" | "ru" => "ru",
        "spanish" | "es" => "es",
        "urdu" | "ur" => "ur",
        _ => "common",
    }
}

fn load_profile(locale: &str) -> MorphologyProfile {
    let mut profile = MorphologyProfile::default();
    for row in RULES.lines() {
        let terms: Vec<_> = fields(row).collect();
        match terms.as_slice() {
            ["part", name, data @ ..] => {
                profile.parts.insert(name, data.to_vec());
            }
            ["feature", name, feature] => {
                profile.features.insert(name, feature);
            }
            ["suffix", owner, data @ ..] if *owner == locale => {
                profile.suffixes.push(data.to_vec())
            }
            ["agreement", owner, lemma, "many", form] if *owner == locale => {
                profile.agreement.insert(lemma, form);
            }
            ["form", owner, form, lemma, weight, upos, features @ ..] if *owner == locale => {
                profile.index(
                    form,
                    WeightedReading {
                        lemma: (*lemma).to_owned(),
                        upos: (*upos).to_owned(),
                        features: features.iter().map(|&tag| tag.to_owned()).collect(),
                        weight: weight.parse().expect("word frequency"),
                        source: "dictionary",
                    },
                );
            }
            _ => {}
        }
    }
    profile
        .suffixes
        .sort_by_key(|fields| std::cmp::Reverse(fields[0].encode_utf16().count()));
    if locale == "ru" {
        if let Some(directory) =
            std::env::var_os("META_LANGUAGE_MORPHOLOGY_DATA").filter(|value| !value.is_empty())
        {
            profile.dictionary = Some(
                RussianDictionary::open(PathBuf::from(directory).join("ru"))
                    .expect("configured OpenCorpora morphology data"),
            );
        } else {
            for row in RUSSIAN.lines() {
                let terms: Vec<_> = fields(row).collect();
                let ["opencorpora", form, lemma, weight, tag] = terms.as_slice() else {
                    unreachable!("Russian vocabulary row")
                };
                let reading = profile.reading(
                    (*lemma).to_owned(),
                    tag,
                    weight.parse().expect("word frequency"),
                    "dictionary",
                );
                profile.index(form, reading);
            }
        }
    }
    profile
}

fn profile(locale: &'static str) -> Arc<MorphologyProfile> {
    static PROFILES: LazyLock<Mutex<HashMap<&'static str, Arc<MorphologyProfile>>>> =
        LazyLock::new(|| Mutex::new(HashMap::new()));
    if let Some(profile) = PROFILES
        .lock()
        .expect("morphology profile cache")
        .get(locale)
        .cloned()
    {
        return profile;
    }
    let loaded = Arc::new(load_profile(locale));
    PROFILES
        .lock()
        .expect("morphology profile cache")
        .entry(locale)
        .or_insert(loaded)
        .clone()
}

impl MorphologyProfile {
    fn index(&mut self, form: &str, reading: WeightedReading) {
        self.forms
            .entry(form.to_owned())
            .or_default()
            .push(reading.clone());
        if form.contains('ё') {
            self.folded_forms
                .entry(form.replace('ё', "е"))
                .or_default()
                .push(reading.clone());
        }
        self.inflections
            .entry(reading.lemma.clone())
            .or_default()
            .push((form.to_owned(), reading));
    }

    fn reading(
        &self,
        lemma: String,
        tag: &str,
        weight: u32,
        source: &'static str,
    ) -> WeightedReading {
        let mut grammemes = tag.split([',', ' ']);
        let part = grammemes.next().unwrap_or_default();
        let mut possessive = false;
        let (mut upos, properties) = self
            .parts
            .get(part)
            .and_then(|fields| fields.split_first())
            .map_or(("X", &[][..]), |(&upos, properties)| (upos, properties));
        let mut features = properties.to_vec();
        for grammeme in grammemes {
            possessive |= grammeme == "Apro";
            if let Some(&feature) = self.features.get(grammeme) {
                if feature == "Proper=Yes" {
                    if upos == "NOUN" {
                        upos = "PROPN";
                    }
                } else {
                    features.push(feature);
                }
            }
        }
        if part == "ADJF" && possessive {
            upos = "DET";
        }
        if part == "NPRO" {
            features.push("PronType=Prs");
        }
        if part == "COMP" {
            features.push("Degree=Cmp");
        }
        features.sort_unstable();
        features.dedup();
        WeightedReading {
            lemma,
            upos: upos.to_owned(),
            features: features.into_iter().map(str::to_owned).collect(),
            weight,
            source,
        }
    }

    fn predictions(&self, spelling: &str) -> Vec<WeightedReading> {
        static DIGITS: LazyLock<Regex> =
            LazyLock::new(|| Regex::new(r"^\p{Nd}+$").expect("Unicode decimal digits"));
        if DIGITS.is_match(spelling) {
            return vec![WeightedReading {
                lemma: spelling.to_owned(),
                upos: "NUM".into(),
                features: vec!["NumType=Card".into()],
                weight: 1,
                source: "shape",
            }];
        }
        let mut predictions = Vec::new();
        let mut longest = 0;
        let length = spelling.encode_utf16().count();
        for fields in &self.suffixes {
            let [ending, replacement, upos, weight, features @ ..] = fields.as_slice() else {
                unreachable!("suffix rule fields")
            };
            let suffix_length = ending.encode_utf16().count();
            if suffix_length < longest {
                break;
            }
            if length <= suffix_length + 1 {
                continue;
            }
            let Some(root) = spelling.strip_suffix(*ending) else {
                continue;
            };
            longest = suffix_length;
            let replacement = if *replacement == "-" { "" } else { replacement };
            let mut features = features.to_vec();
            features.sort_unstable();
            predictions.push(WeightedReading {
                lemma: format!("{root}{replacement}"),
                upos: (*upos).to_owned(),
                features: features.into_iter().map(str::to_owned).collect(),
                weight: weight.parse().expect("suffix frequency"),
                source: "suffix",
            });
        }
        if predictions.is_empty() {
            predictions.push(WeightedReading {
                lemma: spelling.to_owned(),
                upos: "X".into(),
                features: Vec::new(),
                weight: 1,
                source: "identity",
            });
        }
        predictions
    }
}

fn normalize(word: &str) -> String {
    word.nfc().collect::<String>().to_lowercase()
}

/// Supplies all attested or predicted readings of a word, in score order.
#[must_use]
pub fn analyze_morphology(word: &str, language: &str) -> Vec<MorphologyReading> {
    let spelling = normalize(word);
    if spelling.is_empty() {
        return Vec::new();
    }
    let locale = locale(language);
    let profile = profile(locale);
    let mut readings: Vec<WeightedReading> = if let Some(dictionary) = &profile.dictionary {
        dictionary
            .analyze(&spelling)
            .into_iter()
            .map(|entry| profile.reading(entry.lemma, entry.tag, entry.weight, entry.source))
            .collect()
    } else {
        profile
            .forms
            .get(&spelling)
            .into_iter()
            .flatten()
            .chain(profile.folded_forms.get(&spelling).into_iter().flatten())
            .cloned()
            .collect()
    };
    if readings.is_empty() && locale == "en" {
        readings.extend(
            english_morphology(&spelling, false)
                .into_iter()
                .map(|(_, reading)| reading),
        );
    }
    if readings.is_empty() {
        readings = profile.predictions(&spelling);
    }
    let mut unique: Vec<WeightedReading> = Vec::new();
    for reading in readings {
        if let Some(duplicate) = unique.iter_mut().find(|row| {
            row.lemma == reading.lemma
                && row.upos == reading.upos
                && row.features == reading.features
        }) {
            duplicate.weight += reading.weight;
        } else {
            unique.push(reading);
        }
    }
    let total: f64 = unique.iter().map(|reading| f64::from(reading.weight)).sum();
    unique.sort_by_key(|reading| std::cmp::Reverse(reading.weight));
    unique
        .into_iter()
        .map(|reading| MorphologyReading {
            lemma: reading.lemma,
            upos: reading.upos,
            features: reading.features,
            score: (1_000_000.0 * f64::from(reading.weight) / total).round() / 1_000_000.0,
            source: reading.source,
        })
        .collect()
}

/// Returns the first attested form matching all requested Universal Dependencies features.
#[must_use]
pub fn inflect(lemma: &str, features: &[(&str, &str)], language: &str) -> Option<String> {
    let locale = locale(language);
    let profile = profile(locale);
    let spelling = normalize(lemma);
    let requested: Vec<_> = features
        .iter()
        .map(|(name, value)| format!("{name}={value}"))
        .collect();
    let matches = |reading: &WeightedReading| {
        !reading.features.iter().any(|feature| feature == "Abbr=Yes")
            && requested
                .iter()
                .all(|feature| reading.features.contains(feature))
    };
    if let Some(dictionary) = &profile.dictionary {
        for entry in dictionary.analyze(&spelling) {
            if entry.lemma != spelling || entry.source != "dictionary" {
                continue;
            }
            for (form, tag) in dictionary.inflections(&entry) {
                if matches(&profile.reading(spelling.clone(), tag, entry.weight, entry.source)) {
                    return Some(form);
                }
            }
        }
        return None;
    }
    if let Some((form, _)) = profile
        .inflections
        .get(&spelling)
        .into_iter()
        .flatten()
        .find(|(_, reading)| matches(reading))
    {
        return Some(form.clone());
    }
    if locale == "en" {
        return english_morphology(&spelling, true)
            .into_iter()
            .find(|(_, reading)| matches(reading))
            .map(|(form, _)| form);
    }
    None
}

/// Inflects an attested lemma for a cardinal quantity.
#[must_use]
pub fn agree_with_number(lemma: &str, quantity: f64, language: &str) -> Option<String> {
    let number = quantity.abs();
    if !number.is_finite() || number > 9_007_199_254_740_991.0 {
        return None;
    }
    let locale = locale(language);
    if locale != "ru" {
        return inflect(
            lemma,
            &[("Number", if number == 1.0 { "Sing" } else { "Plur" })],
            language,
        );
    }
    if number.fract() != 0.0 {
        return inflect(lemma, &[("Case", "Gen"), ("Number", "Sing")], language);
    }
    match russian_number_agreement_kind(number) {
        0 => inflect(lemma, &[("Case", "Nom"), ("Number", "Sing")], language),
        1 => inflect(lemma, &[("Case", "Gen"), ("Number", "Sing")], language),
        _ => profile(locale)
            .agreement
            .get(normalize(lemma).as_str())
            .map(|&form| form.to_owned())
            .or_else(|| inflect(lemma, &[("Case", "Gen"), ("Number", "Plur")], language)),
    }
}
