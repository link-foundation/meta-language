//! Trigram language identification shared with the JavaScript runtime.
//!
//! This is franc's algorithm over `data/language-trigrams.json`, which
//! `js/scripts/build-language-identification.mjs` generates from franc-min
//! together with the JavaScript copy. Both runtimes count Unicode scalar
//! values, so they reach the same verdict for the same text.

use std::collections::HashMap;
use std::sync::OnceLock;

use serde_json::Value;

const LANGUAGE_TRIGRAMS_JSON: &str = include_str!("data/language-trigrams.json");
const MAX_LENGTH: usize = 2048;
const MAX_DIFFERENCE: usize = 300;

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
}

fn trigram_data() -> &'static TrigramData {
    static DATA: OnceLock<TrigramData> = OnceLock::new();
    DATA.get_or_init(parse_trigram_data)
}

fn parse_trigram_data() -> TrigramData {
    let value: Value =
        serde_json::from_str(LANGUAGE_TRIGRAMS_JSON).expect("language trigram data is valid JSON");
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
    }
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
