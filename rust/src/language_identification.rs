//! Trigram language identification shared with the JavaScript runtime.
//!
//! Both runtimes use `data/language-trigrams.lino`, which
//! `js/scripts/build-language-identification.mjs` generates from franc-min
//! together with the JavaScript copy.

use std::collections::HashMap;
use std::fmt;
use std::sync::{LazyLock, OnceLock};

use regex::Regex;
use serde_json::Value;
use unicode_normalization::UnicodeNormalization;

const LANGUAGE_TRIGRAMS_LINKS: &str = include_str!("data/language-trigrams.lino");
const MAX_LENGTH: usize = 2048;
const MAX_DIFFERENCE: usize = 300;
const DEFAULT_THRESHOLD: f64 = 0.1;
const MINIMUM_CONFIDENCE_GAP: f64 = 0.02;
static LETTER: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\A\p{L}\z").expect("letter regex"));
static MARK: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\A\p{M}\z").expect("mark regex"));

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
    scripts: Vec<Script>,
    models: HashMap<String, Vec<LanguageModel>>,
    cyrillic_catalog_alphabet: String,
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
        scripts,
        models,
        cyrillic_catalog_alphabet: value["cyrillicCatalogAlphabet"]
            .as_str()
            .expect("Cyrillic catalog alphabet")
            .to_owned(),
    }
}

/// A catalog language and its model fit between zero and one.
#[derive(Clone, Debug, PartialEq)]
pub struct LanguageCandidate {
    /// Canonical language name.
    pub language: &'static str,
    /// Heuristic model fit between zero and one.
    pub confidence: f64,
}

/// Identification of a run of letters from one script, with their original byte offsets.
#[derive(Clone, Debug, PartialEq)]
pub struct LanguageIdentificationSpan {
    /// Inclusive UTF-8 byte offset.
    pub start: usize,
    /// Exclusive UTF-8 byte offset.
    pub end: usize,
    /// Highest-ranked language meeting the threshold and confidence gap, or `None`.
    pub language: Option<&'static str>,
    /// Candidates ordered by descending confidence.
    pub candidates: Vec<LanguageCandidate>,
}

/// Ranked evidence and verdicts for a text sample and its script runs.
#[derive(Clone, Debug, PartialEq)]
pub struct LanguageIdentification {
    /// Highest-ranked language meeting the threshold and confidence gap, or `None`.
    pub language: Option<&'static str>,
    /// Candidates ordered by descending letter-weighted confidence.
    pub candidates: Vec<LanguageCandidate>,
    /// Identified runs in input order, including runs with no verdict.
    pub spans: Vec<LanguageIdentificationSpan>,
}

/// A language identification threshold outside the inclusive range `0..=1`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct InvalidLanguageIdentificationThreshold;

impl fmt::Display for InvalidLanguageIdentificationThreshold {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("Language identification threshold must be between 0 and 1")
    }
}

impl std::error::Error for InvalidLanguageIdentificationThreshold {}

/// Identifies natural languages using the default threshold of `0.1`.
///
/// The sample covers the first 2,048 Unicode scalar values. Model-based
/// script runs with fewer than three letters have no candidates. Han and
/// Bengali runs can produce a candidate based on script evidence alone.
#[must_use]
pub fn identify_language(text: &str) -> LanguageIdentification {
    language_identification(text, DEFAULT_THRESHOLD)
}

/// Identifies natural languages using an inclusive confidence threshold.
///
/// A verdict requires the threshold and a `0.02` lead when there is a second candidate.
/// The ranked candidates remain visible when the verdict is `None`.
///
/// # Errors
///
/// Returns [`InvalidLanguageIdentificationThreshold`] for a value outside
/// `0..=1` or `NaN`.
pub fn identify_language_with_threshold(
    text: &str,
    threshold: f64,
) -> Result<LanguageIdentification, InvalidLanguageIdentificationThreshold> {
    if !(0.0..=1.0).contains(&threshold) {
        return Err(InvalidLanguageIdentificationThreshold);
    }
    Ok(language_identification(text, threshold))
}

fn language_identification(text: &str, threshold: f64) -> LanguageIdentification {
    let data = trigram_data();
    let mut totals = Vec::<(&'static str, f64)>::new();
    let mut letter_count = 0;
    let spans: Vec<_> = script_runs(data, text)
        .into_iter()
        .map(|run| {
            let candidates = rank_run(data, &text[run.start..run.end], &run);
            letter_count += run.letter_count;
            for candidate in &candidates {
                let weighted = candidate.confidence * f64::from(run.letter_count);
                if let Some((_, sum)) = totals
                    .iter_mut()
                    .find(|(name, _)| *name == candidate.language)
                {
                    *sum += weighted;
                } else {
                    totals.push((candidate.language, weighted));
                }
            }
            LanguageIdentificationSpan {
                start: run.start,
                end: run.end,
                language: select_language(&candidates, threshold),
                candidates,
            }
        })
        .collect();
    if let [span] = spans.as_slice() {
        return LanguageIdentification {
            language: span.language,
            candidates: span.candidates.clone(),
            spans,
        };
    }
    let mut candidates: Vec<_> = totals
        .into_iter()
        .map(|(language, total)| LanguageCandidate {
            language,
            confidence: total / f64::from(letter_count),
        })
        .collect();
    candidates.sort_by(|left, right| right.confidence.total_cmp(&left.confidence));
    LanguageIdentification {
        language: select_language(&candidates, threshold),
        candidates,
        spans,
    }
}

fn select_language(candidates: &[LanguageCandidate], threshold: f64) -> Option<&'static str> {
    let [best, remaining @ ..] = candidates else {
        return None;
    };
    if best.confidence < threshold {
        return None;
    }
    if remaining
        .first()
        .is_some_and(|second| best.confidence - second.confidence < MINIMUM_CONFIDENCE_GAP)
    {
        return None;
    }
    Some(best.language)
}

struct ScriptRun<'a> {
    script: Option<&'a str>,
    start: usize,
    end: usize,
    letter_count: u32,
}

fn script_runs<'a>(data: &'a TrigramData, text: &str) -> Vec<ScriptRun<'a>> {
    let mut runs = Vec::<ScriptRun<'a>>::new();
    let mut attached = false;
    for (start, character) in text.char_indices().take(MAX_LENGTH) {
        let mut utf8 = [0; 4];
        let encoded = character.encode_utf8(&mut utf8);
        let end = start + encoded.len();
        if LETTER.is_match(encoded) {
            let script = data
                .scripts
                .iter()
                .find(|script| in_ranges(u32::from(character), &script.ranges))
                .map(|script| script.name.as_str());
            if let Some(run) = runs.last_mut().filter(|run| run.script == script) {
                run.end = end;
                run.letter_count += 1;
            } else {
                runs.push(ScriptRun {
                    script,
                    start,
                    end,
                    letter_count: 1,
                });
            }
            attached = true;
        } else {
            attached = attached && MARK.is_match(encoded);
            if attached && let Some(run) = runs.last_mut() {
                run.end = end;
            }
        }
    }
    runs
}

fn rank_run(data: &TrigramData, text: &str, run: &ScriptRun<'_>) -> Vec<LanguageCandidate> {
    let Some(script) = run.script else {
        return Vec::new();
    };
    let Some(models) = data.models.get(script) else {
        return data
            .languages
            .get(script)
            .map_or_else(Vec::new, |&language| {
                vec![LanguageCandidate {
                    language,
                    confidence: 1.0,
                }]
            });
    };
    if run.letter_count < 3 || models.is_empty() {
        return Vec::new();
    }

    let characters: Vec<_> = text.nfc().collect();
    if script == "Cyrillic"
        && characters.iter().any(|character| {
            LETTER.is_match(character.encode_utf8(&mut [0; 4]))
                && !data
                    .cyrillic_catalog_alphabet
                    .contains(character.to_lowercase().next().unwrap_or(*character))
        })
    {
        return Vec::new();
    }

    let trigrams = trigram_counts(&characters);
    let maximum = trigrams.len() * MAX_DIFFERENCE;
    let mut ranked: Vec<_> = models
        .iter()
        .map(|(code, model)| {
            (
                data.languages.get(code).copied(),
                trigram_distance(&trigrams, model),
            )
        })
        .collect();
    ranked.sort_by_key(|&(_, distance)| distance);
    // Competing Cyrillic models can reject a Russian verdict, including ties.
    if ranked
        .iter()
        .any(|&(language, distance)| language.is_none() && distance == ranked[0].1)
    {
        return Vec::new();
    }
    let score = |value| f64::from(u32::try_from(value).expect("bounded trigram score"));
    ranked
        .into_iter()
        .filter_map(|(language, distance)| {
            let language = language?;
            (distance < maximum).then(|| LanguageCandidate {
                language,
                confidence: score(maximum - distance) / score(maximum),
            })
        })
        .collect()
}

/// Returns the catalog verdict used by existing parse annotations.
pub(crate) fn identify_catalog_language(text: &str) -> Option<&'static str> {
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
