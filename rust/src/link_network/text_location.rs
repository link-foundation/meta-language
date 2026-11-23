//! Quote location and word n-gram overlap with original byte coordinates.

use std::collections::BTreeMap;
use std::sync::LazyLock;

use regex::Regex;
use unicode_normalization::char::{canonical_combining_class, compose, decompose_compatible};
use unicode_segmentation::UnicodeSegmentation;

use super::LinkNetwork;
use crate::source::ByteRange;

mod policy;

/// A transformation applied to both texts before matching.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TextNormalization {
    /// Unicode compatibility composition.
    Nfkc,
    /// Typographic quotation marks become their ASCII equivalents.
    Quotes,
    /// Dash punctuation and the minus sign become a hyphen.
    Dashes,
    /// Cyrillic ё and Ё become е and Е respectively.
    Yo,
    /// Apply Unicode 17.0 default full case folding.
    Case,
    /// Remove soft hyphens.
    SoftHyphen,
    /// Convert consecutive whitespace to one ASCII space.
    Whitespace,
}

use TextNormalization::{Case, Dashes, Nfkc, Quotes, SoftHyphen, Whitespace, Yo};

const NORMALIZATION_ORDER: [TextNormalization; 7] =
    [Nfkc, Quotes, Dashes, Yo, Case, SoftHyphen, Whitespace];

/// Units compared in approximate mode.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum TextEditUnit {
    /// Unicode extended grapheme clusters.
    #[default]
    Grapheme,
    /// Consecutive Unicode letters, marks, numbers, and underscores.
    Word,
}

/// Matching options. A zero edit budget performs an exact projection match.
#[derive(Clone, Copy, Debug, Default)]
pub struct TextLocationOptions<'normalization> {
    /// Transformations permitted for the comparison.
    pub normalize: &'normalization [TextNormalization],
    /// Maximum number of insertions, deletions or substitutions.
    pub max_edits: usize,
    /// Unit of comparison for a nonzero edit budget.
    pub unit: TextEditUnit,
}

/// A location in one of the two inputs, with its projected text.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TextLocationUnit {
    /// Byte range in the unmodified input.
    pub range: ByteRange,
    /// Value after the selected projection.
    pub text: String,
}

/// A change from the requested fragment to the source occurrence.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TextLocationEdit {
    /// One unit occurs only in the source.
    Insert {
        /// Range and projected text on the source side.
        source: TextLocationUnit,
    },
    /// One unit occurs only in the requested fragment.
    Delete {
        /// Range and projected text on the fragment side.
        fragment: TextLocationUnit,
    },
    /// A source unit differs from the corresponding fragment unit.
    Replace {
        /// Range and projected text on the fragment side.
        fragment: TextLocationUnit,
        /// Range and projected text on the source side.
        source: TextLocationUnit,
    },
}

/// One matching slice of the original source.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TextLocationMatch {
    /// Byte range in the unmodified source.
    pub range: ByteRange,
    /// Unmodified source slice.
    pub text: String,
    /// Transformations needed to make an exact projected match.
    pub normalizations: Vec<TextNormalization>,
    /// Number of approximate edits.
    pub distance: usize,
    /// Approximate edits in input order.
    pub edits: Vec<TextLocationEdit>,
}

/// A word n-gram occurring in both input texts.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WordNgramMatch {
    /// The projected words shared by the two texts.
    pub words: Vec<String>,
    /// Location in the first input.
    pub left: ByteRange,
    /// Location in the second input.
    pub right: ByteRange,
}

static DASH: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^[\p{Pd}−]$").expect("dash category"));
static WORD: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[\p{L}\p{M}\p{N}_]$").expect("word categories"));

static CASE_FOLDING: LazyLock<BTreeMap<char, Vec<char>>> = LazyLock::new(|| {
    include_str!("../data/text-location-case-folding.lino")
        .lines()
        .skip(1)
        .map(|line| {
            let mut scalars = line[1..line.len() - 1].split_whitespace().map(|code| {
                let value = u32::from_str_radix(code, 16).expect("case folding code point");
                char::from_u32(value).expect("case folding scalar value")
            });
            let source = scalars.next().expect("case folding source scalar");
            (source, scalars.collect())
        })
        .collect()
});

#[derive(Clone, Copy)]
struct NormalizedScalar {
    value: char,
    combining_class: u8,
    range: ByteRange,
}

fn covering_range(left: ByteRange, right: ByteRange) -> ByteRange {
    ByteRange::new(left.start().min(right.start()), left.end().max(right.end()))
}

fn compatibility_projection(source: &str) -> Vec<TextLocationUnit> {
    let graphemes = source.grapheme_indices(true);
    if source.is_ascii() {
        return graphemes
            .map(|(index, segment)| TextLocationUnit {
                range: ByteRange::new(index, index + segment.len()),
                text: segment.to_owned(),
            })
            .collect();
    }

    // Keep the original grapheme range through decomposition, ordering and composition.
    let mut ordered: Vec<NormalizedScalar> = Vec::new();
    let mut combining_start = 0;
    for (index, segment) in graphemes {
        let range = ByteRange::new(index, index + segment.len());
        for scalar in segment.chars() {
            decompose_compatible(scalar, |value| {
                let combining_class = canonical_combining_class(value);
                if combining_class == 0 {
                    ordered[combining_start..].sort_by_key(|entry| entry.combining_class);
                    combining_start = ordered.len() + 1;
                }
                ordered.push(NormalizedScalar {
                    value,
                    combining_class,
                    range,
                });
            });
        }
    }
    ordered[combining_start..].sort_by_key(|entry| entry.combining_class);

    let mut starter: Option<usize> = None;
    let mut last_class = 0;
    let mut length = 0;
    for index in 0..ordered.len() {
        let entry = ordered[index];
        if (last_class == 0 || last_class < entry.combining_class)
            && let Some(starter) = starter
            && let Some(composite) = compose(ordered[starter].value, entry.value)
        {
            let first = &mut ordered[starter];
            first.value = composite;
            first.range = covering_range(first.range, entry.range);
            continue;
        }
        last_class = entry.combining_class;
        if last_class == 0 {
            starter = Some(length);
        }
        ordered[length] = entry;
        length += 1;
    }
    ordered.truncate(length);

    // Expansions, reordering and compositions can make source ranges overlap.
    let mut ranges: Vec<(usize, ByteRange)> = Vec::new();
    for (index, entry) in ordered.iter().enumerate() {
        let (mut first, mut range) = (index, entry.range);
        while ranges
            .last()
            .is_some_and(|(_, previous)| previous.end() > range.start())
        {
            let (previous_first, previous) = ranges.pop().expect("overlapping source range");
            first = previous_first;
            range = covering_range(previous, range);
        }
        ranges.push((first, range));
    }
    ranges
        .iter()
        .enumerate()
        .map(|(index, &(first, range))| {
            let end = ranges
                .get(index + 1)
                .map_or(ordered.len(), |(first, _)| *first);
            let text = ordered[first..end]
                .iter()
                .map(|entry| entry.value)
                .collect();
            TextLocationUnit { range, text }
        })
        .collect()
}

fn project(source: &str, names: &[TextNormalization]) -> Vec<TextLocationUnit> {
    let mut result: Vec<TextLocationUnit> = Vec::new();
    let mut emit = |text: &str, range: ByteRange| {
        for mut value in text.chars() {
            if names.contains(&Quotes) {
                value = mapped_scalar(value, policy::normalize_quote_scalar);
            }
            if names.contains(&Dashes) && DASH.is_match(value.encode_utf8(&mut [0; 4])) {
                value = '-';
            }
            if names.contains(&Yo) {
                value = mapped_scalar(value, policy::normalize_yo_scalar);
            }
            if names.contains(&SoftHyphen) && value == '\u{ad}' {
                continue;
            }
            if names.contains(&Whitespace) && value.is_whitespace() {
                if let Some(previous) = result.last_mut().filter(|unit| unit.text == " ") {
                    previous.range = ByteRange::new(previous.range.start(), range.end());
                } else {
                    result.push(TextLocationUnit {
                        range,
                        text: " ".to_owned(),
                    });
                }
            } else if names.contains(&Case) {
                let folded = CASE_FOLDING
                    .get(&value)
                    .map_or(std::slice::from_ref(&value), Vec::as_slice);
                result.extend(folded.iter().map(|value| TextLocationUnit {
                    range,
                    text: value.to_string(),
                }));
            } else {
                result.push(TextLocationUnit {
                    range,
                    text: value.to_string(),
                });
            }
        }
    };
    if names.contains(&Nfkc) {
        for TextLocationUnit { text, range } in compatibility_projection(source) {
            emit(&text, range);
        }
    } else {
        for (index, scalar) in source.char_indices() {
            emit(
                scalar.encode_utf8(&mut [0; 4]),
                ByteRange::new(index, index + scalar.len_utf8()),
            );
        }
    }
    result
}

fn mapped_scalar(value: char, mapping: impl Fn(f64) -> f64) -> char {
    char::from_u32(mapping(f64::from(u32::from(value))) as u32)
        .expect("mapping of a Unicode scalar")
}

fn word_units(entries: Vec<TextLocationUnit>) -> Vec<TextLocationUnit> {
    let mut result: Vec<TextLocationUnit> = Vec::new();
    let mut previous = false;
    for entry in entries {
        let word = WORD.is_match(&entry.text);
        if word && previous {
            let group = result.last_mut().expect("previous word");
            group.text.push_str(&entry.text);
            group.range = ByteRange::new(group.range.start(), entry.range.end());
        } else if word {
            result.push(entry);
        }
        previous = word;
    }
    result
}

fn grapheme_units(entries: &[TextLocationUnit]) -> Vec<TextLocationUnit> {
    let text = entries
        .iter()
        .map(|entry| entry.text.as_str())
        .collect::<String>();
    let mut index = 0;
    text.graphemes(true)
        .map(|segment| {
            let next = index + segment.chars().count();
            let range = ByteRange::new(entries[index].range.start(), entries[next - 1].range.end());
            index = next;
            TextLocationUnit {
                text: segment.to_owned(),
                range,
            }
        })
        .collect()
}

fn required_normalizations(
    source: &str,
    fragment: &str,
    names: &[TextNormalization],
) -> Vec<TextNormalization> {
    let mut active: Vec<_> = NORMALIZATION_ORDER
        .into_iter()
        .filter(|name| names.contains(name))
        .collect();
    let normalized = |text, names| {
        project(text, names)
            .into_iter()
            .map(|unit| unit.text)
            .collect::<String>()
    };
    for name in NORMALIZATION_ORDER {
        if let Some(index) = active.iter().position(|active| *active == name) {
            active.remove(index);
            if normalized(source, &active) != normalized(fragment, &active) {
                active.insert(index, name);
            }
        }
    }
    active
}

fn start_boundary(units: &[TextLocationUnit], start: usize) -> bool {
    start == 0 || units[start - 1].range.start() != units[start].range.start()
}

fn end_boundary(units: &[TextLocationUnit], end: usize) -> bool {
    end == units.len() || units[end - 1].range.end() != units[end].range.end()
}

struct Search<'input, 'normalization> {
    source: &'input str,
    fragment: &'input str,
    names: &'normalization [TextNormalization],
    words: Vec<TextLocationUnit>,
    needle: Vec<TextLocationUnit>,
}

impl Search<'_, '_> {
    fn occurrence(
        &self,
        first: usize,
        end: usize,
        distance: usize,
        edits: Vec<TextLocationEdit>,
    ) -> TextLocationMatch {
        let range = ByteRange::new(
            self.words[first].range.start(),
            self.words[end - 1].range.end(),
        );
        let text = &self.source[range.start()..range.end()];
        let normalizations = if distance == 0 {
            required_normalizations(text, self.fragment, self.names)
        } else {
            Vec::new()
        };
        TextLocationMatch {
            range,
            text: text.to_owned(),
            normalizations,
            distance,
            edits,
        }
    }

    fn exact(&self) -> Vec<TextLocationMatch> {
        let mut prefix = vec![0; self.needle.len()];
        let mut cursor = 0;
        for index in 1..self.needle.len() {
            while cursor > 0 && self.needle[index].text != self.needle[cursor].text {
                cursor = prefix[cursor - 1];
            }
            cursor += usize::from(self.needle[index].text == self.needle[cursor].text);
            prefix[index] = cursor;
        }
        cursor = 0;
        let mut matches = Vec::new();
        for (index, word) in self.words.iter().enumerate() {
            while cursor > 0 && word.text != self.needle[cursor].text {
                cursor = prefix[cursor - 1];
            }
            cursor += usize::from(word.text == self.needle[cursor].text);
            if cursor == self.needle.len() {
                let end = index + 1;
                let first = end - cursor;
                if start_boundary(&self.words, first) && end_boundary(&self.words, end) {
                    matches.push(self.occurrence(first, end, 0, Vec::new()));
                }
                cursor = prefix[cursor - 1];
            }
        }
        matches
    }

    fn align(&self, start: usize, budget: usize) -> Option<TextLocationMatch> {
        let length = (self.words.len() - start).min(self.needle.len().saturating_add(budget));
        let outside = budget.saturating_add(1);
        let initial: BTreeMap<_, _> = (0..=length.min(budget))
            .map(|column| (column, (column, Alignment::Insert)))
            .collect();
        let mut rows = vec![initial];
        for row in 1..=self.needle.len() {
            let previous = &rows[row - 1];
            let mut cells = BTreeMap::new();
            let distance_at = |cells: &BTreeMap<_, (usize, Alignment)>, column| {
                cells
                    .get(&column)
                    .map_or(outside, |(distance, _)| *distance)
            };
            for column in row.saturating_sub(budget)..=length.min(row.saturating_add(budget)) {
                let equal =
                    column > 0 && self.needle[row - 1].text == self.words[start + column - 1].text;
                let diagonal = column.checked_sub(1).map_or(outside, |before| {
                    distance_at(previous, before).saturating_add(usize::from(!equal))
                });
                let deletion = distance_at(previous, column).saturating_add(1);
                let insertion = column.checked_sub(1).map_or(outside, |before| {
                    distance_at(&cells, before).saturating_add(1)
                });
                let selected =
                    policy::select_text_edit(diagonal as f64, deletion as f64, insertion as f64)
                        as usize;
                let distance = [diagonal, deletion, insertion][selected];
                if distance <= budget {
                    let change = [
                        if equal {
                            Alignment::Equal
                        } else {
                            Alignment::Replace
                        },
                        Alignment::Delete,
                        Alignment::Insert,
                    ][selected];
                    cells.insert(column, (distance, change));
                }
            }
            if cells.is_empty() {
                return None;
            }
            rows.push(cells);
        }
        let (&end, &(distance, _)) = rows
            .last()?
            .iter()
            .filter(|(column, _)| **column > 0 && end_boundary(&self.words, start + **column))
            .min_by_key(|(_, (distance, _))| *distance)?;
        let mut row = self.needle.len();
        let mut column = end;
        let mut edits = Vec::new();
        while row > 0 || column > 0 {
            match rows[row][&column].1 {
                Alignment::Equal => {
                    row -= 1;
                    column -= 1;
                }
                Alignment::Replace => {
                    row -= 1;
                    column -= 1;
                    edits.push(TextLocationEdit::Replace {
                        fragment: self.needle[row].clone(),
                        source: self.words[start + column].clone(),
                    });
                }
                Alignment::Delete => {
                    row -= 1;
                    edits.push(TextLocationEdit::Delete {
                        fragment: self.needle[row].clone(),
                    });
                }
                Alignment::Insert => {
                    column -= 1;
                    edits.push(TextLocationEdit::Insert {
                        source: self.words[start + column].clone(),
                    });
                }
            }
        }
        edits.reverse();
        Some(self.occurrence(start, start + end, distance, edits))
    }
}

#[derive(Clone, Copy)]
enum Alignment {
    Equal,
    Replace,
    Delete,
    Insert,
}

/// Locates a fragment and returns its original byte spans in source order.
#[must_use]
pub fn locate(
    source: &str,
    fragment: &str,
    options: TextLocationOptions<'_>,
) -> Vec<TextLocationMatch> {
    let units = |text| {
        let entries = project(text, options.normalize);
        if options.max_edits == 0 {
            return entries;
        }
        match options.unit {
            TextEditUnit::Grapheme => grapheme_units(&entries),
            TextEditUnit::Word => word_units(entries),
        }
    };
    let search = Search {
        source,
        fragment,
        names: options.normalize,
        words: units(source),
        needle: units(fragment),
    };
    if search.needle.is_empty() {
        return Vec::new();
    }
    if options.max_edits == 0 {
        return search.exact();
    }
    let budget = options
        .max_edits
        .min(search.needle.len().max(search.words.len()));
    (0..search.words.len())
        .filter(|&index| start_boundary(&search.words, index))
        .filter_map(|index| search.align(index, budget))
        .collect()
}

impl LinkNetwork {
    /// Locates a fragment in the reconstructed original source.
    #[must_use]
    pub fn locate(
        &self,
        fragment: &str,
        options: TextLocationOptions<'_>,
    ) -> Vec<TextLocationMatch> {
        locate(&self.reconstruct_text(), fragment, options)
    }
}

/// Reports all pairs of shared word n-grams in left-to-right source order.
#[must_use]
pub fn word_ngram_overlap(
    left: &str,
    right: &str,
    size: usize,
    normalize: &[TextNormalization],
) -> Vec<WordNgramMatch> {
    if size == 0 {
        return Vec::new();
    }
    let left_words = word_units(project(left, normalize));
    let right_words = word_units(project(right, normalize));
    let phrase = |units: &[TextLocationUnit]| {
        units
            .iter()
            .map(|unit| unit.text.clone())
            .collect::<Vec<_>>()
    };
    let range = |units: &[TextLocationUnit]| {
        ByteRange::new(units[0].range.start(), units[units.len() - 1].range.end())
    };
    let mut right_spans = BTreeMap::<_, Vec<_>>::new();
    for units in right_words.windows(size) {
        right_spans
            .entry(phrase(units))
            .or_default()
            .push(range(units));
    }
    let mut matches = Vec::new();
    for units in left_words.windows(size) {
        let words = phrase(units);
        if let Some(spans) = right_spans.get(&words) {
            matches.extend(spans.iter().map(|&right| WordNgramMatch {
                words: words.clone(),
                left: range(units),
                right,
            }));
        }
    }
    matches
}
