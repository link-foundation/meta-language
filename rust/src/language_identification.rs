//! Trigram language identification shared with the JavaScript runtime.
//!
//! This is franc's algorithm over `data/language-trigrams.lino`, which
//! `js/scripts/build-language-identification.mjs` generates from franc-min,
//! extra franc Cyrillic models, and CLDR alphabets. Both runtimes count
//! Unicode scalar values and reach the same verdict for the same text.

use std::collections::HashMap;
use std::fmt;
use std::sync::OnceLock;

use serde_json::Value;

mod confidence;

const LANGUAGE_TRIGRAMS_LINKS: &str = include_str!("data/language-trigrams.lino");
const MAX_LENGTH: usize = 2048;
const MAX_DIFFERENCE: usize = 300;
const MINIMUM_LETTERS: usize = 3;
const DEFAULT_CONFIDENCE: f64 = 0.06;

/// Engine term recorded on identification annotations.
pub const LANGUAGE_IDENTIFIER_TERM: &str = "identifier:trigram";

struct Script {
    name: String,
    ranges: Vec<(u32, u32)>,
}

/// A language code with its trigram ranks.
type LanguageModel = (String, HashMap<String, usize>);

struct TrigramData {
    languages: HashMap<String, &'static str>,
    cyrillic_languages: HashMap<String, String>,
    cyrillic_alphabets: HashMap<String, String>,
    scripts: Vec<Script>,
    models: HashMap<String, Vec<LanguageModel>>,
}

fn trigram_data() -> &'static TrigramData {
    static DATA: OnceLock<TrigramData> = OnceLock::new();
    DATA.get_or_init(parse_trigram_data)
}

fn parse_trigram_data() -> TrigramData {
    let value: Value =
        crate::structured_data_links::read_structured_data_lino(LANGUAGE_TRIGRAMS_LINKS)
            .expect("language trigram data is valid Links Notation");
    let languages = value["languages"]
        .as_object()
        .expect("languages object")
        .iter()
        .map(|(code, name)| {
            let name = crate::natural_language::canonical_natural_language(
                name.as_str().expect("language name"),
            )
            .expect("supported natural language");
            (code.clone(), name)
        })
        .collect();
    let scripts = value["scripts"]
        .as_array()
        .expect("scripts array")
        .iter()
        .map(|script| Script {
            name: script["name"].as_str().expect("script name").to_string(),
            ranges: script["ranges"]
                .as_array()
                .expect("script ranges")
                .iter()
                .map(|range| {
                    let bound = |index: usize| {
                        u32::try_from(range[index].as_u64().expect("range bound"))
                            .expect("code point")
                    };
                    (bound(0), bound(1))
                })
                .collect(),
        })
        .collect();
    let models = value["models"]
        .as_array()
        .expect("models array")
        .iter()
        .map(|model| {
            let languages = model["languages"]
                .as_array()
                .expect("model languages")
                .iter()
                .map(|language| {
                    let ranks = language["trigrams"]
                        .as_str()
                        .expect("trigram model")
                        .split('|')
                        .enumerate()
                        .map(|(rank, trigram)| (trigram.to_string(), rank))
                        .collect();
                    (language["code"].as_str().expect("code").to_string(), ranks)
                })
                .collect();
            (
                model["script"].as_str().expect("script").to_string(),
                languages,
            )
        })
        .collect();
    TrigramData {
        languages,
        cyrillic_languages: read_string_map(&value["cyrillicLanguages"]),
        cyrillic_alphabets: read_string_map(&value["cyrillicAlphabets"]),
        scripts,
        models,
    }
}

fn read_string_map(value: &Value) -> HashMap<String, String> {
    value
        .as_object()
        .expect("language data string map")
        .iter()
        .map(|(key, value)| {
            (
                key.clone(),
                value.as_str().expect("language data string").to_owned(),
            )
        })
        .collect()
}

/// A language and its evidence score between zero and one.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LanguageIdentificationCandidate {
    pub language: &'static str,
    pub confidence: f64,
}

/// A continuous script region with half-open UTF-8 byte offsets.
#[derive(Clone, Debug, PartialEq)]
pub struct LanguageIdentificationSpan {
    pub script: &'static str,
    pub start: usize,
    pub end: usize,
    pub language: Option<&'static str>,
    pub candidates: Vec<LanguageIdentificationCandidate>,
}

/// Ranked candidates and script spans for the examined part of the input.
#[derive(Clone, Debug, PartialEq)]
pub struct LanguageIdentification {
    pub language: Option<&'static str>,
    pub candidates: Vec<LanguageIdentificationCandidate>,
    pub spans: Vec<LanguageIdentificationSpan>,
}

/// The confidence threshold was not a finite number between zero and one.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct InvalidLanguageConfidence;

impl fmt::Display for InvalidLanguageConfidence {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("minimum confidence must be between zero and one")
    }
}

impl std::error::Error for InvalidLanguageConfidence {}

#[derive(Default)]
struct ScriptRun {
    script: &'static str,
    start: usize,
    end: usize,
    characters: Vec<char>,
    letters: usize,
}

#[derive(Default)]
struct LanguageEvidence {
    spans: Vec<LanguageIdentificationSpan>,
    scores: Vec<LanguageIdentificationCandidate>,
    letters: usize,
}

/// Ranks language evidence globally and within each continuous script span.
/// The default threshold is used when `minimum_confidence` is `None`.
///
/// # Errors
///
/// Returns an error when the threshold is outside the closed interval from zero to one.
pub fn rank_language_identification(
    text: &str,
    minimum_confidence: Option<f64>,
) -> Result<LanguageIdentification, InvalidLanguageConfidence> {
    let threshold = minimum_confidence.unwrap_or(DEFAULT_CONFIDENCE);
    if !(0.0..=1.0).contains(&threshold) {
        return Err(InvalidLanguageConfidence);
    }

    let data = trigram_data();
    let mut evidence = LanguageEvidence::default();
    let mut run: Option<ScriptRun> = None;
    for (start, character) in text.char_indices().take(MAX_LENGTH) {
        let script = data
            .scripts
            .iter()
            .find(|script| in_ranges(u32::from(character), &script.ranges));
        let Some(script) = script else {
            if let Some(run) = &mut run
                && run.characters.last() != Some(&' ')
            {
                run.characters.push(' ');
            }
            continue;
        };
        if let Some(current) = &run
            && current.script != script.name
        {
            evidence.record(data, run.take().expect("current script span"), threshold);
        }
        let current = run.get_or_insert_with(|| ScriptRun {
            script: &script.name,
            start,
            ..ScriptRun::default()
        });
        current.characters.push(character);
        current.end = start + character.len_utf8();
        current.letters += 1;
    }
    if let Some(current) = run {
        evidence.record(data, current, threshold);
    }
    for candidate in &mut evidence.scores {
        candidate.confidence /= score_number(evidence.letters);
    }
    sort_candidates(&mut evidence.scores);
    Ok(LanguageIdentification {
        language: select_language(&evidence.scores, threshold),
        candidates: evidence.scores,
        spans: evidence.spans,
    })
}

impl LanguageEvidence {
    fn record(&mut self, data: &'static TrigramData, mut run: ScriptRun, threshold: f64) {
        if run.characters.last() == Some(&' ') {
            run.characters.pop();
        }
        let candidates = rank_script(data, &run);
        for candidate in &candidates {
            let weighted = candidate.confidence * score_number(run.letters);
            if let Some(total) = self
                .scores
                .iter_mut()
                .find(|total| total.language == candidate.language)
            {
                total.confidence += weighted;
            } else {
                self.scores.push(LanguageIdentificationCandidate {
                    confidence: weighted,
                    ..*candidate
                });
            }
        }
        self.letters += run.letters;
        self.spans.push(LanguageIdentificationSpan {
            script: run.script,
            start: run.start,
            end: run.end,
            language: select_language(&candidates, threshold),
            candidates,
        });
    }
}

fn score_number(value: usize) -> f64 {
    f64::from(u32::try_from(value).expect("bounded language evidence"))
}

fn rank_script(
    data: &'static TrigramData,
    run: &ScriptRun,
) -> Vec<LanguageIdentificationCandidate> {
    if run.letters < MINIMUM_LETTERS {
        return Vec::new();
    }
    let Some(models) = data.models.get(run.script) else {
        return data
            .languages
            .get(run.script)
            .copied()
            .map_or_else(Vec::new, |language| {
                vec![LanguageIdentificationCandidate {
                    language,
                    confidence: 1.0,
                }]
            });
    };

    let trigrams = trigram_counts(&run.characters);
    let maximum_distance = trigrams.len() * MAX_DIFFERENCE;
    let cyrillic_letters = if run.script == "Cyrillic" {
        clean_characters(&run.characters)
    } else {
        Vec::new()
    };
    let mut candidates = Vec::new();
    for (code, model) in models {
        if let Some(alphabet) = data.cyrillic_alphabets.get(code)
            && cyrillic_letters
                .iter()
                .any(|&character| character != ' ' && !alphabet.contains(character))
        {
            continue;
        }
        let distance = trigram_distance(&trigrams, model);
        if distance < maximum_distance {
            let language = data
                .languages
                .get(code)
                .copied()
                .or_else(|| data.cyrillic_languages.get(code).map(String::as_str))
                .expect("named language model");
            candidates.push(LanguageIdentificationCandidate {
                language,
                confidence: confidence::trigram_confidence(
                    score_number(distance),
                    score_number(maximum_distance),
                    score_number(run.letters),
                ),
            });
        }
    }
    sort_candidates(&mut candidates);
    candidates
}

fn sort_candidates(candidates: &mut [LanguageIdentificationCandidate]) {
    candidates.sort_by(|left, right| right.confidence.total_cmp(&left.confidence));
}

fn select_language(
    candidates: &[LanguageIdentificationCandidate],
    threshold: f64,
) -> Option<&'static str> {
    let first = candidates.first()?;
    if first.confidence < threshold
        || candidates.get(1).is_some_and(|second| {
            !confidence::minimum_language_lead(first.confidence, second.confidence)
        })
    {
        return None;
    }
    Some(first.language)
}

/// Returns the canonical name of the supported natural language the text is
/// written in, or `None` when the text carries no usable evidence.
#[must_use]
pub fn identify_language(text: &str) -> Option<&'static str> {
    let data = trigram_data();
    let characters: Vec<char> = text.chars().take(MAX_LENGTH).collect();
    if characters.is_empty() {
        return None;
    }

    let script = top_script(&data.scripts, &characters)?;
    let Some(models) = data.models.get(script) else {
        return data.languages.get(script).copied();
    };

    let trigrams = trigram_counts(&characters);
    let (code, distance) = models
        .iter()
        .filter(|(code, _)| data.languages.contains_key(code))
        .map(|(code, model)| (code, trigram_distance(&trigrams, model)))
        .fold(
            None,
            |best: Option<(&String, usize)>, candidate| match best {
                Some(best) if best.1 <= candidate.1 => Some(best),
                _ => Some(candidate),
            },
        )?;
    // A distance of MAX_DIFFERENCE per character means no trigram matched.
    if distance >= characters.len() * MAX_DIFFERENCE {
        return None;
    }
    data.languages.get(code).copied()
}

fn top_script<'a>(scripts: &'a [Script], characters: &[char]) -> Option<&'a str> {
    let mut top = None;
    let mut top_count = 0;
    for script in scripts {
        let count = characters
            .iter()
            .filter(|character| in_ranges(u32::from(**character), &script.ranges))
            .count();
        if count > top_count {
            top = Some(script.name.as_str());
            top_count = count;
        }
    }
    top
}

fn in_ranges(code_point: u32, ranges: &[(u32, u32)]) -> bool {
    ranges
        .binary_search_by(|&(start, end)| {
            if code_point < start {
                std::cmp::Ordering::Greater
            } else if code_point > end {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Equal
            }
        })
        .is_ok()
}

/// ECMAScript `\s`, which franc collapses.
const fn is_ecmascript_whitespace(character: char) -> bool {
    matches!(
        character,
        '\t' | '\n' | '\u{0b}' | '\u{0c}' | '\r' | ' ' | '\u{a0}' | '\u{1680}' | '\u{2000}'
            ..='\u{200a}'
                | '\u{2028}'
                | '\u{2029}'
                | '\u{202f}'
                | '\u{205f}'
                | '\u{3000}'
                | '\u{feff}'
    )
}

/// franc's `clean`: ASCII punctuation and digits become spaces, whitespace
/// runs collapse to one space, the result is trimmed and lowercased.
fn clean_characters(characters: &[char]) -> Vec<char> {
    let mut cleaned = String::new();
    for &character in characters {
        if ('\u{21}'..='\u{40}').contains(&character) || is_ecmascript_whitespace(character) {
            if !cleaned.is_empty() && !cleaned.ends_with(' ') {
                cleaned.push(' ');
            }
        } else {
            cleaned.push(character);
        }
    }
    if cleaned.ends_with(' ') {
        cleaned.pop();
    }
    cleaned.to_lowercase().chars().collect()
}

/// Trigram counts in first-occurrence order, stably sorted by count.
fn trigram_counts(characters: &[char]) -> Vec<(String, usize)> {
    let mut padded = vec![' '];
    padded.extend(clean_characters(characters));
    padded.push(' ');
    let mut counts: Vec<(String, usize)> = Vec::new();
    let mut positions: HashMap<String, usize> = HashMap::new();
    for window in padded.windows(3) {
        let trigram: String = window.iter().collect();
        if let Some(&position) = positions.get(&trigram) {
            counts[position].1 += 1;
        } else {
            positions.insert(trigram.clone(), counts.len());
            counts.push((trigram, 1));
        }
    }
    counts.sort_by_key(|(_, count)| *count);
    counts
}

fn trigram_distance(trigrams: &[(String, usize)], model: &HashMap<String, usize>) -> usize {
    trigrams
        .iter()
        .map(|(trigram, count)| {
            model
                .get(trigram)
                .map_or(MAX_DIFFERENCE, |rank| count.abs_diff(rank + 1))
        })
        .sum()
}
