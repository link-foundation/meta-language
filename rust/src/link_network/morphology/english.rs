use std::sync::LazyLock;

use super::{WeightedReading, fields, sha256};
use base64::{Engine as _, engine::general_purpose::STANDARD};
use miniz_oxide::inflate::decompress_to_vec_zlib_with_limit;

const DATA: &str = include_str!("../../data/morphology/english.lino");

struct EnglishDictionary {
    table: String,
    features: Vec<Vec<&'static str>>,
    forms: Vec<u32>,
    lemmas: Vec<u32>,
}

impl EnglishDictionary {
    fn load() -> Self {
        let mut lines = DATA.lines();
        let header: Vec<_> = fields(lines.next().expect("English morphology header")).collect();
        let ["english-morphology", "1", text_size, row_count, digest] = header.as_slice() else {
            panic!("invalid English morphology data header");
        };
        let text_size: usize = text_size.parse().expect("English morphology text size");
        let row_count: usize = row_count.parse().expect("English morphology row count");
        let limit = 8 * 1024 * 1024;
        assert!(text_size > 0 && text_size <= limit && row_count > 0 && row_count <= limit / 4);
        let size = text_size + row_count * 4;
        assert!(size <= limit, "English morphology data size");

        let mut features = Vec::new();
        let mut blocks = String::new();
        for line in lines {
            if let Some(bundle) = line.strip_prefix("(bundle ") {
                features.push(
                    bundle
                        .trim_end_matches(')')
                        .split_ascii_whitespace()
                        .collect(),
                );
            } else if let Some(block) = line.strip_prefix("(data ") {
                blocks.push_str(block.trim_end_matches(')'));
            }
        }
        let compressed = STANDARD
            .decode(blocks)
            .expect("base64 English morphology data");
        let bytes = decompress_to_vec_zlib_with_limit(&compressed, size)
            .expect("compressed English morphology data");
        assert_eq!(bytes.len(), size, "English morphology data size");
        assert_eq!(sha256(&bytes), *digest);
        let (text, offsets) = bytes.split_at(text_size);
        let table = String::from_utf8(text.to_vec()).expect("English morphology rows");
        let mut forms = Vec::with_capacity(row_count);
        let mut offset = 0;
        for line in table.split_inclusive('\n') {
            forms.push(offset as u32);
            offset += line.len();
        }
        assert_eq!(forms.len(), row_count, "English morphology row count");
        let lemmas = offsets
            .chunks_exact(4)
            .map(|bytes| u32::from_le_bytes(bytes.try_into().expect("English lemma offset")))
            .collect();
        Self {
            table,
            features,
            forms,
            lemmas,
        }
    }

    fn key(&self, offset: u32, by_lemma: bool) -> &str {
        let mut terms = self.table[offset as usize + 1..].split(' ');
        terms
            .nth(by_lemma as usize)
            .expect("English morphology spelling")
    }

    fn lookup(&self, word: &str, by_lemma: bool) -> Vec<(String, WeightedReading)> {
        let offsets = if by_lemma { &self.lemmas } else { &self.forms };
        let start = offsets.partition_point(|&offset| self.key(offset, by_lemma) < word);
        offsets[start..]
            .iter()
            .copied()
            .take_while(|&offset| self.key(offset, by_lemma) == word)
            .map(|offset| {
                let row = self.table[offset as usize + 1..]
                    .lines()
                    .next()
                    .expect("English morphology row");
                let mut terms = row.trim_end_matches(')').split(' ');
                let form = terms.next().expect("English morphology form").to_owned();
                let lemma = terms.next().expect("English morphology lemma").to_owned();
                let weight = terms
                    .next()
                    .expect("English morphology weight")
                    .parse()
                    .expect("numeric weight");
                let bundle: usize = terms
                    .next()
                    .expect("English morphology features")
                    .parse()
                    .expect("numeric bundle");
                let (upos, features) = self.features[bundle]
                    .split_first()
                    .expect("English part of speech");
                (
                    form,
                    WeightedReading {
                        lemma,
                        upos: (*upos).to_owned(),
                        features: features.iter().map(|&tag| tag.to_owned()).collect(),
                        weight,
                        source: "dictionary",
                    },
                )
            })
            .collect()
    }
}

pub(super) fn english_morphology(word: &str, by_lemma: bool) -> Vec<(String, WeightedReading)> {
    static DICTIONARY: LazyLock<EnglishDictionary> = LazyLock::new(EnglishDictionary::load);
    DICTIONARY.lookup(word, by_lemma)
}
