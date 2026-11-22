use base64::{Engine as _, engine::general_purpose::STANDARD};
use sha2::{Digest, Sha256};
use std::io::{self, Read};
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

const SOURCE: &str = include_str!("../../data/morphology/open-corpora.lino");

fn read_file(directory: &Path, name: &str) -> io::Result<Vec<u8>> {
    let (size, digest) = SOURCE
        .lines()
        .find_map(|line| {
            let mut fields = line[1..line.len() - 1].split_ascii_whitespace();
            (fields.next()? == "file" && fields.next()? == name).then(|| {
                (
                    fields.next().unwrap().parse::<usize>().unwrap(),
                    fields.next().unwrap(),
                )
            })
        })
        .expect("registered OpenCorpora file");
    let mut bytes = Vec::with_capacity(size);
    std::fs::File::open(directory.join(name))?
        .take(u64::try_from(size).expect("registered file size") + 1)
        .read_to_end(&mut bytes)?;
    if bytes.len() != size || format!("{:x}", Sha256::digest(&bytes)) != digest {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("invalid OpenCorpora file {name}"),
        ));
    }
    Ok(bytes)
}

// OpenCorpora 2.4 Darts units and completion guide. Only the authenticated
// snapshot reaches this reader. A record is encoded after key byte 1.
struct WordDictionary {
    bytes: Vec<u8>,
    size: usize,
    record_size: usize,
}

impl WordDictionary {
    fn new(bytes: Vec<u8>, record_size: usize) -> Self {
        let size = word(&bytes, 0) as usize;
        assert_eq!(bytes.len(), 8 + size * 6);
        assert_eq!(word(&bytes, 4 + size * 4) as usize, size);
        Self {
            bytes,
            size,
            record_size,
        }
    }

    fn unit(&self, index: usize) -> u32 {
        word(&self.bytes, 4 + 4 * index)
    }

    fn transition(&self, index: usize, label: u8) -> Option<usize> {
        let candidate = index ^ unit_offset(self.unit(index)) ^ usize::from(label);
        (candidate < self.size && self.unit(candidate) & 0x8000_00ff == u32::from(label))
            .then_some(candidate)
    }

    fn follow(&self, index: usize, key: &str) -> Option<usize> {
        key.bytes()
            .try_fold(index, |current, byte| self.transition(current, byte))
    }

    fn value(&self, key: &str) -> u32 {
        self.follow(0, key)
            .filter(|&index| self.unit(index) & 256 != 0)
            .map_or(0, |index| {
                self.unit(index ^ unit_offset(self.unit(index))) & 0x7fff_ffff
            })
    }

    fn payloads(
        &self,
        index: usize,
        key: &mut Vec<u8>,
        spelling: &str,
        result: &mut Vec<(String, Vec<u8>)>,
    ) {
        if self.unit(index) & 256 != 0
            && let Ok(encoded) = std::str::from_utf8(key)
            && let Ok(payload) = STANDARD.decode(encoded.trim())
            && payload.len() == self.record_size
        {
            result.push((spelling.to_owned(), payload));
        }
        if key.len() >= 16 {
            return;
        }
        let guide = &self.bytes[8 + 4 * self.size..];
        let mut label = guide[2 * index];
        while label != 0 {
            let child = self
                .transition(index, label)
                .expect("valid OpenCorpora transition");
            key.push(label);
            self.payloads(child, key, spelling, result);
            key.pop();
            label = guide[2 * child + 1];
        }
    }

    fn visit(
        &self,
        remaining: &str,
        spelling: &mut String,
        index: usize,
        result: &mut Vec<(String, Vec<u8>)>,
    ) {
        let Some(character) = remaining.chars().next() else {
            if let Some(key) = self.transition(index, 1) {
                self.payloads(key, &mut Vec::new(), spelling, result);
            }
            return;
        };
        let rest = &remaining[character.len_utf8()..];
        let alternatives = [character, 'ё'];
        let count = if character == 'е' { 2 } else { 1 };
        for &option in &alternatives[..count] {
            if let Some(next) = self.follow(index, option.encode_utf8(&mut [0; 4])) {
                spelling.push(option);
                self.visit(rest, spelling, next, result);
                spelling.pop();
            }
        }
    }

    fn matches(&self, spelling: &str) -> Vec<(String, Vec<u8>)> {
        let mut result = Vec::new();
        self.visit(spelling, &mut String::new(), 0, &mut result);
        result
    }
}

fn word(bytes: &[u8], start: usize) -> u32 {
    u32::from_le_bytes(bytes[start..start + 4].try_into().unwrap())
}

fn unit_offset(unit: u32) -> usize {
    ((unit >> 10) << ((unit & 512) >> 6)) as usize
}

fn paradigm_table(bytes: &[u8]) -> Vec<Vec<usize>> {
    let mut values = bytes
        .chunks_exact(2)
        .map(|value| usize::from(u16::from_le_bytes([value[0], value[1]])));
    let count = values.next().expect("paradigm count");
    let mut paradigms = Vec::with_capacity(count);
    for _ in 0..count {
        let length = values.next().expect("paradigm width");
        let paradigm: Vec<_> = values.by_ref().take(length).collect();
        assert_eq!(paradigm.len(), length);
        assert!(length > 0 && length.is_multiple_of(3));
        paradigms.push(paradigm);
    }
    assert!(values.next().is_none());
    paradigms
}

pub(super) struct DictionaryReading<'a> {
    pub lemma: String,
    pub tag: &'a str,
    pub weight: u32,
    pub source: &'static str,
    prefix: &'a str,
    root: String,
    paradigm_index: usize,
}

pub(super) struct RussianDictionary {
    directory: PathBuf,
    words: WordDictionary,
    probabilities: WordDictionary,
    endings: OnceLock<Vec<WordDictionary>>,
    prefixes: Vec<String>,
    suffixes: Vec<String>,
    tags: Vec<String>,
    paradigms: Vec<Vec<usize>>,
    max_suffix_length: usize,
}

impl RussianDictionary {
    pub fn open(directory: PathBuf) -> io::Result<Self> {
        let metadata: serde_json::Value =
            serde_json::from_slice(&read_file(&directory, "meta.json")?)?;
        let compile_options = metadata
            .as_array()
            .expect("metadata fields")
            .iter()
            .find(|field| field[0] == "compile_options")
            .expect("compiled dictionary options");
        let prefixes = serde_json::from_value(compile_options[1]["paradigm_prefixes"].clone())?;
        let max_suffix_length =
            serde_json::from_value(compile_options[1]["max_suffix_length"].clone())?;
        let words = WordDictionary::new(read_file(&directory, "words.dawg")?, 4);
        let probabilities = WordDictionary::new(read_file(&directory, "p_t_given_w.intdawg")?, 0);
        let paradigms = paradigm_table(&read_file(&directory, "paradigms.array")?);
        let suffixes = serde_json::from_slice(&read_file(&directory, "suffixes.json")?)?;
        let tags = serde_json::from_slice(&read_file(&directory, "gramtab-opencorpora-int.json")?)?;
        Ok(Self {
            directory,
            words,
            probabilities,
            endings: OnceLock::new(),
            prefixes,
            suffixes,
            tags,
            paradigms,
            max_suffix_length,
        })
    }

    fn paradigm_form(&self, paradigm_index: usize, index: usize) -> (&str, &str, &str) {
        let paradigm = &self.paradigms[paradigm_index];
        let length = paradigm.len() / 3;
        (
            &self.prefixes[paradigm[2 * length + index]],
            &self.suffixes[paradigm[index]],
            &self.tags[paradigm[length + index]],
        )
    }

    fn reading(
        &self,
        spelling: &str,
        payload: &[u8],
        weight: u32,
        source: &'static str,
    ) -> Option<DictionaryReading<'_>> {
        let paradigm_index = usize::from(u16::from_be_bytes([payload[0], payload[1]]));
        let index = usize::from(u16::from_be_bytes([payload[2], payload[3]]));
        let (prefix, suffix, tag) = self.paradigm_form(paradigm_index, index);
        let root = spelling
            .strip_prefix(prefix)?
            .strip_suffix(suffix)?
            .to_owned();
        let (first_prefix, first_suffix, _) = self.paradigm_form(paradigm_index, 0);
        Some(DictionaryReading {
            lemma: format!("{first_prefix}{root}{first_suffix}"),
            tag,
            weight,
            source,
            prefix,
            root,
            paradigm_index,
        })
    }

    pub fn analyze(&self, spelling: &str) -> Vec<DictionaryReading<'_>> {
        let offsets: Vec<_> = spelling
            .char_indices()
            .map(|(offset, _)| offset)
            .take(129)
            .collect();
        if offsets.len() > 128 {
            return Vec::new();
        }
        let mut exact: Vec<_> = self
            .words
            .matches(spelling)
            .into_iter()
            .filter_map(|(form, data)| self.reading(&form, &data, 1, "dictionary"))
            .collect();
        if !exact.is_empty() {
            let probabilities: Vec<_> = exact
                .iter()
                .map(|entry| {
                    self.probabilities
                        .value(&format!("{spelling}:{}", entry.tag))
                })
                .collect();
            if probabilities.iter().any(|&value| value != 0) {
                for (reading, probability) in exact.iter_mut().zip(probabilities) {
                    reading.weight = probability;
                }
            }
            return exact;
        }
        let endings = self.endings.get_or_init(|| {
            (0..self.prefixes.len())
                .map(|index| {
                    let bytes = read_file(
                        &self.directory,
                        &format!("prediction-suffixes-{index}.dawg"),
                    )
                    .expect("configured OpenCorpora prediction data");
                    WordDictionary::new(bytes, 8)
                })
                .collect()
        });
        let mut predicted = Vec::new();
        for (prefix, dictionary) in self.prefixes.iter().zip(endings) {
            if !spelling.starts_with(prefix) {
                continue;
            }
            let previous_count = predicted.len();
            let maximum = self
                .max_suffix_length
                .min(offsets.len().saturating_sub(prefix.chars().count() + 1));
            for length in (1..=maximum).rev() {
                let (stem, suffix) = spelling.split_at(offsets[offsets.len() - length]);
                for (ending, data) in dictionary.matches(suffix) {
                    let weight = u32::from_be_bytes(data[..4].try_into().unwrap());
                    if let Some(reading) =
                        self.reading(&format!("{stem}{ending}"), &data[4..], weight, "suffix")
                        && !reading.root.is_empty()
                        && reading.prefix == prefix.as_str()
                    {
                        predicted.push(reading);
                    }
                }
                if predicted.len() > previous_count {
                    break;
                }
            }
        }
        predicted
    }

    pub fn inflections(
        &self,
        reading: &DictionaryReading<'_>,
    ) -> impl Iterator<Item = (String, &str)> {
        (0..self.paradigms[reading.paradigm_index].len() / 3).map(|index| {
            let (prefix, suffix, tag) = self.paradigm_form(reading.paradigm_index, index);
            (format!("{prefix}{}{suffix}", reading.root), tag)
        })
    }
}
