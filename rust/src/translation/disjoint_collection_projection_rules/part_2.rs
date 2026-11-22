// Generated from complete js/src/translation/disjoint-collection-projection-rules.js. Do not edit by hand.
use super::MlSource1TemporaryReadCollectionConstructionFieldLoop1Result;
use super::last;
use super::ml_array;
use super::replace;
use super::source_ml_source1_temporary_read_collection_construction_field_loop1;

/// JavaScript strings retain every UTF-16 code unit, including isolated surrogates.
pub mod ml_string {
    #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub struct Text(pub Vec<u16>);

    impl std::fmt::Display for Text {
        fn fmt(&self, out: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            out.write_str(&String::from_utf16_lossy(&self.0))
        }
    }

    impl Text {
        #[must_use]
        pub fn from_text(text: &str) -> Self {
            Self(text.encode_utf16().collect())
        }
        #[must_use]
        pub const fn from_units(units: Vec<u16>) -> Self {
            Self(units)
        }
        #[must_use]
        pub const fn length(&self) -> f64 {
            self.0.len() as f64
        }
        #[must_use]
        pub fn concat(&self, other: &Self) -> Self {
            let mut units = self.0.clone();
            units.extend_from_slice(&other.0);
            Self(units)
        }
        #[must_use]
        pub fn starts_with(&self, search: &Self) -> bool {
            self.0.starts_with(&search.0)
        }
        #[must_use]
        pub fn ends_with(&self, search: &Self) -> bool {
            self.0.ends_with(&search.0)
        }
        #[must_use]
        pub fn contains(&self, search: &Self) -> bool {
            self.index_of(search, 0.0) >= 0.0
        }
        #[must_use]
        pub fn slice(&self, start: f64, end: f64) -> Self {
            let first = relative(start, self.0.len());
            let last = relative(end, self.0.len());
            Self(self.0[first..last.max(first)].to_vec())
        }
        #[must_use]
        pub fn index_of(&self, search: &Self, position: f64) -> f64 {
            let start = absolute(position, self.0.len());
            if search.0.is_empty() {
                return start as f64;
            }
            if search.0.len() > self.0.len() {
                return -1.0;
            }
            (start..=self.0.len() - search.0.len())
                .find(|&index| self.0[index..].starts_with(&search.0))
                .map_or(-1.0, |index| index as f64)
        }
        #[must_use]
        pub fn last_index_of(&self, search: &Self, position: f64) -> f64 {
            let position = if position.is_nan() {
                f64::INFINITY
            } else {
                position
            };
            let start = absolute(position, self.0.len());
            if search.0.is_empty() {
                return start as f64;
            }
            if search.0.len() > self.0.len() {
                return -1.0;
            }
            (0..=start.min(self.0.len() - search.0.len()))
                .rev()
                .find(|&index| self.0[index..].starts_with(&search.0))
                .map_or(-1.0, |index| index as f64)
        }
        #[must_use]
        pub fn split(&self, separator: Option<&Self>, limit: f64) -> Vec<Self> {
            let maximum = uint32(limit) as usize;
            if maximum == 0 {
                return Vec::new();
            }
            let Some(separator) = separator else {
                return vec![self.clone()];
            };
            if separator.0.is_empty() {
                return self
                    .0
                    .iter()
                    .take(maximum)
                    .map(|&unit| Self(vec![unit]))
                    .collect();
            }
            let mut out = Vec::new();
            let mut start = 0;
            loop {
                let index = self.index_of(separator, start as f64);
                if index < 0.0 {
                    break;
                }
                out.push(Self(self.0[start..index as usize].to_vec()));
                if out.len() == maximum {
                    return out;
                }
                start = index as usize + separator.0.len();
            }
            out.push(Self(self.0[start..].to_vec()));
            out
        }
        #[must_use]
        pub fn code_point_at(&self, position: f64) -> Option<f64> {
            let index = integer(position);
            if index < 0.0 || index >= self.0.len() as f64 {
                return None;
            }
            let index = index as usize;
            let first = self.0[index];
            if (0xd800..=0xdbff).contains(&first)
                && let Some(&second) = self.0.get(index + 1)
                && (0xdc00..=0xdfff).contains(&second)
            {
                return Some(f64::from(
                    (u32::from(first) - 0xd800) * 1024 + u32::from(second) - 0xdc00 + 0x10000,
                ));
            }
            Some(f64::from(first))
        }
        #[must_use]
        pub fn code_points(&self) -> Vec<Self> {
            let mut out = Vec::new();
            let mut index = 0;
            while index < self.0.len() {
                let first = self.0[index];
                let width = if (0xd800..=0xdbff).contains(&first)
                    && self
                        .0
                        .get(index + 1)
                        .is_some_and(|second| (0xdc00..=0xdfff).contains(second))
                {
                    2
                } else {
                    1
                };
                out.push(Self(self.0[index..index + width].to_vec()));
                index += width;
            }
            out
        }
        #[must_use]
        pub fn repeat(&self, count: f64) -> Self {
            let count = integer(count);
            assert!(
                count >= 0.0 && count.is_finite(),
                "Invalid count value: {count}"
            );
            if self.0.is_empty() {
                return Self(Vec::new());
            }
            let size = count * self.0.len() as f64;
            assert!(size <= 536_870_888.0, "Invalid string length");
            Self(self.0.repeat(count as usize))
        }
        #[must_use]
        pub fn pad(&self, length: f64, fill: &Self, start: bool) -> Self {
            let length = integer(length).max(0.0);
            if length <= self.0.len() as f64 || fill.0.is_empty() {
                return self.clone();
            }
            assert!(length <= 536_870_888.0, "Invalid string length");
            let count = length as usize - self.0.len();
            let padding: Vec<u16> = fill.0.iter().copied().cycle().take(count).collect();
            let pad = Self(padding);
            if start {
                pad.concat(self)
            } else {
                self.concat(&pad)
            }
        }
        #[must_use]
        pub fn replace(&self, search: &Self, replacement: &Self, all: bool) -> Self {
            let mut out = Vec::new();
            let mut start = 0;
            let mut from = 0;
            loop {
                let index = self.index_of(search, from as f64);
                if index < 0.0 {
                    break;
                }
                let index = index as usize;
                out.extend_from_slice(&self.0[start..index]);
                substitute(&mut out, &self.0, &search.0, &replacement.0, index);
                start = index + search.0.len();
                if !all || (search.0.is_empty() && index == self.0.len()) {
                    break;
                }
                from = start + usize::from(search.0.is_empty());
            }
            out.extend_from_slice(&self.0[start..]);
            Self(out)
        }
        #[must_use]
        pub fn trim(&self, start: bool, end: bool, rust: bool) -> Self {
            let whitespace = |unit: u16| {
                char::from_u32(u32::from(unit)).is_some_and(|c| {
                    if rust {
                        c.is_whitespace()
                    } else {
                        (c.is_whitespace() && c != '\u{85}') || c == '\u{feff}'
                    }
                })
            };
            let mut first = 0;
            let mut last = self.0.len();
            if start {
                while first < last && whitespace(self.0[first]) {
                    first += 1;
                }
            }
            if end {
                while last > first && whitespace(self.0[last - 1]) {
                    last -= 1;
                }
            }
            Self(self.0[first..last].to_vec())
        }
        #[must_use]
        pub fn map_case(&self, upper: bool) -> Self {
            let mut out = Vec::new();
            let mut run = String::new();
            let flush = |run: &mut String, out: &mut Vec<u16>| {
                let mapped = if upper {
                    run.to_uppercase()
                } else {
                    run.to_lowercase()
                };
                out.extend(mapped.encode_utf16());
                run.clear();
            };
            for scalar in char::decode_utf16(self.0.iter().copied()) {
                match scalar {
                    Ok(c) => run.push(c),
                    Err(error) => {
                        flush(&mut run, &mut out);
                        out.push(error.unpaired_surrogate());
                    }
                }
            }
            flush(&mut run, &mut out);
            Self(out)
        }
    }
    #[must_use]
    pub fn join(values: &[Text], separator: &Text) -> Text {
        let mut units = Vec::new();
        for (index, value) in values.iter().enumerate() {
            if index != 0 {
                units.extend_from_slice(&separator.0);
            }
            units.extend_from_slice(&value.0);
        }
        Text(units)
    }
    #[must_use]
    pub const fn integer(value: f64) -> f64 {
        if value.is_nan() { 0.0 } else { value.trunc() }
    }
    #[must_use]
    pub const fn absolute(value: f64, length: usize) -> usize {
        integer(value).max(0.0).min(length as f64) as usize
    }
    #[must_use]
    pub fn relative(value: f64, length: usize) -> usize {
        let value = integer(value);
        if value < 0.0 {
            (length as f64 + value).max(0.0) as usize
        } else {
            value.min(length as f64) as usize
        }
    }
    fn uint32(value: f64) -> u32 {
        if !value.is_finite() || value == 0.0 {
            0
        } else {
            value.trunc().rem_euclid(4_294_967_296.0) as u32
        }
    }
    fn substitute(
        out: &mut Vec<u16>,
        text: &[u16],
        search: &[u16],
        replacement: &[u16],
        position: usize,
    ) {
        let mut index = 0;
        while index < replacement.len() {
            if replacement[index] == 36 && index + 1 < replacement.len() {
                match replacement[index + 1] {
                    36 => {
                        out.push(36);
                        index += 2;
                        continue;
                    }
                    38 => {
                        out.extend_from_slice(search);
                        index += 2;
                        continue;
                    }
                    96 => {
                        out.extend_from_slice(&text[..position]);
                        index += 2;
                        continue;
                    }
                    39 => {
                        out.extend_from_slice(&text[position + search.len()..]);
                        index += 2;
                        continue;
                    }
                    _ => {}
                }
            }
            out.push(replacement[index]);
            index += 1;
        }
    }
}

#[path = "ml_string_number.rs"]
pub mod ml_string_number;

pub mod ml_source_exception {
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct SourceException {
        pub class_name: String,
        pub message_units: Vec<u16>,
        pub message_present: bool,
    }
    impl SourceException {
        #[must_use]
        pub fn raise(self) -> ! {
            std::panic::panic_any((self.class_name, self.message_units, self.message_present))
        }
        #[must_use]
        pub fn from_text(class_name: &str, message: &str) -> Self {
            Self {
                class_name: class_name.to_owned(),
                message_units: message.encode_utf16().collect(),
                message_present: true,
            }
        }
    }
    impl std::fmt::Display for SourceException {
        fn fmt(&self, out: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            out.write_str(&String::from_utf16_lossy(&self.message_units))
        }
    }
    impl std::error::Error for SourceException {}
}

#[path = "ml_utf16_text.rs"]
pub mod ml_utf16_text;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn refuse_required_number_fact() -> f64 {
    (self::ml_source_exception::SourceException {
        class_name: String::from("RangeError"),
        message_units: (self::ml_string::Text::from_units(vec![
            109, 105, 115, 115, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 100, 32, 78,
            117, 109, 98, 101, 114, 32, 102, 97, 99, 116, 32, 99, 111, 108, 117, 109, 110, 32, 118,
            97, 108, 117, 101,
        ]))
        .0,
        message_present: true,
    })
    .raise()
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn refuse_required_text_fact() -> self::ml_string::Text {
    (self::ml_source_exception::SourceException {
        class_name: String::from("RangeError"),
        message_units: (self::ml_string::Text::from_units(vec![
            109, 105, 115, 115, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 100, 32, 116,
            101, 120, 116, 32, 102, 97, 99, 116, 32, 99, 111, 108, 117, 109, 110, 32, 118, 97, 108,
            117, 101,
        ]))
        .0,
        message_present: true,
    })
    .raise()
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn refuse_required_boolean_fact() -> bool {
    (self::ml_source_exception::SourceException {
        class_name: String::from("RangeError"),
        message_units: (self::ml_string::Text::from_units(vec![
            109, 105, 115, 115, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 100, 32, 66,
            111, 111, 108, 101, 97, 110, 32, 102, 97, 99, 116, 32, 99, 111, 108, 117, 109, 110, 32,
            118, 97, 108, 117, 101,
        ]))
        .0,
        message_present: true,
    })
    .raise()
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_required_number_fact(values: &[f64], index: f64) -> f64 {
    if ((index < 0f64) || ((index % 1f64) != 0f64)) || (index >= (values.len() as f64)) {
        (self::ml_source_exception::SourceException {
            class_name: String::from("RangeError"),
            message_units: (self::ml_string::Text::from_units(vec![
                105, 110, 118, 97, 108, 105, 100, 32, 114, 101, 113, 117, 105, 114, 101, 100, 32,
                78, 117, 109, 98, 101, 114, 32, 102, 97, 99, 116, 32, 99, 111, 108, 117, 109, 110,
                32, 112, 111, 115, 105, 116, 105, 111, 110,
            ]))
            .0,
            message_present: true,
        })
        .raise()
    } else {
        (self::ml_array::optional_at(values, self::ml_array::number_index(index)))
            .unwrap_or_else(self::refuse_required_number_fact)
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_required_text_fact(
    values: &[self::ml_string::Text],
    index: f64,
) -> self::ml_string::Text {
    if ((index < 0f64) || ((index % 1f64) != 0f64)) || (index >= (values.len() as f64)) {
        (self::ml_source_exception::SourceException {
            class_name: String::from("RangeError"),
            message_units: (self::ml_string::Text::from_units(vec![
                105, 110, 118, 97, 108, 105, 100, 32, 114, 101, 113, 117, 105, 114, 101, 100, 32,
                116, 101, 120, 116, 32, 102, 97, 99, 116, 32, 99, 111, 108, 117, 109, 110, 32, 112,
                111, 115, 105, 116, 105, 111, 110,
            ]))
            .0,
            message_present: true,
        })
        .raise()
    } else {
        (self::ml_array::optional_at(values, self::ml_array::number_index(index)))
            .unwrap_or_else(self::refuse_required_text_fact)
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_required_boolean_fact(values: &[bool], index: f64) -> bool {
    if ((index < 0f64) || ((index % 1f64) != 0f64)) || (index >= (values.len() as f64)) {
        (self::ml_source_exception::SourceException {
            class_name: String::from("RangeError"),
            message_units: (self::ml_string::Text::from_units(vec![
                105, 110, 118, 97, 108, 105, 100, 32, 114, 101, 113, 117, 105, 114, 101, 100, 32,
                66, 111, 111, 108, 101, 97, 110, 32, 102, 97, 99, 116, 32, 99, 111, 108, 117, 109,
                110, 32, 112, 111, 115, 105, 116, 105, 111, 110,
            ]))
            .0,
            message_present: true,
        })
        .raise()
    } else {
        (self::ml_array::optional_at(values, self::ml_array::number_index(index)))
            .unwrap_or_else(self::refuse_required_boolean_fact)
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_collection_construction_field(
    starts: &[f64],
    counts: &[f64],
    labels: &[self::ml_string::Text],
    targets: &[f64],
    node: f64,
    field_2: &self::ml_string::Text,
) -> f64 {
    if node < 0f64 {
        -1f64
    } else {
        {
            let start = self::read_required_number_fact(starts, node);
            {
                let count_2 = self::read_required_number_fact(counts, node);
                {
                    let at = start;
                    {
                        let source_ml_source2_s1 = self::source_ml_source1_temporary_read_collection_construction_field_loop1(labels, targets, field_2, start, count_2, at);
                        match source_ml_source2_s1 {
                            self::MlSource1TemporaryReadCollectionConstructionFieldLoop1Result::MlSource1TemporaryReadCollectionConstructionFieldLoop1Done => {
                                -1f64
                            }
                            self::MlSource1TemporaryReadCollectionConstructionFieldLoop1Result::MlSource1TemporaryReadCollectionConstructionFieldLoop1Return(source_ml_source1_temporary_result1) => {
                                source_ml_source1_temporary_result1
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_collection_construction_element(
    starts: &[f64],
    counts: &[f64],
    targets: &[f64],
    node: f64,
    index: f64,
) -> f64 {
    if ((node < 0f64) || (index < 0f64))
        || (index >= (self::read_required_number_fact(counts, node)))
    {
        -1f64
    } else {
        self::read_required_number_fact(
            targets,
            (self::read_required_number_fact(starts, node) + index),
        )
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_collection_construction_kind(
    texts: &[self::ml_string::Text],
    starts: &[f64],
    counts: &[f64],
    labels: &[self::ml_string::Text],
    targets: &[f64],
    node: f64,
) -> self::ml_string::Text {
    {
        let kind_2 = self::read_collection_construction_field(
            starts,
            counts,
            labels,
            targets,
            node,
            &(self::ml_string::Text::from_units(vec![107, 105, 110, 100])),
        );
        if kind_2 < 0f64 {
            self::ml_string::Text::from_units(vec![])
        } else {
            self::read_required_text_fact(texts, kind_2)
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_collection_construction_inference(
    texts: &[self::ml_string::Text],
    starts: &[f64],
    counts: &[f64],
    labels: &[self::ml_string::Text],
    targets: &[f64],
    kind_2: &self::ml_string::Text,
    expected: f64,
) -> Vec<f64> {
    {
        let context = (self::read_collection_construction_kind(
            texts, starts, counts, labels, targets, expected,
        )) == (*kind_2);
        if (*kind_2) == (self::ml_string::Text::from_units(vec![115, 101, 116])) {
            vec![
                1f64,
                if context { 1f64 } else { 0f64 },
                if context {
                    self::read_collection_construction_field(
                        starts,
                        counts,
                        labels,
                        targets,
                        expected,
                        &(self::ml_string::Text::from_units(vec![
                            101, 108, 101, 109, 101, 110, 116,
                        ])),
                    )
                } else {
                    -1f64
                },
                -1f64,
            ]
        } else if (*kind_2) == (self::ml_string::Text::from_units(vec![109, 97, 112])) {
            vec![
                2f64,
                if context { 1f64 } else { 0f64 },
                if context {
                    self::read_collection_construction_field(
                        starts,
                        counts,
                        labels,
                        targets,
                        expected,
                        &(self::ml_string::Text::from_units(vec![107, 101, 121])),
                    )
                } else {
                    -1f64
                },
                if context {
                    self::read_collection_construction_field(
                        starts,
                        counts,
                        labels,
                        targets,
                        expected,
                        &(self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101])),
                    )
                } else {
                    -1f64
                },
            ]
        } else {
            vec![0f64, 0f64, -1f64, -1f64]
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_collection_construction_checked(
    texts: &[self::ml_string::Text],
    starts: &[f64],
    counts: &[f64],
    labels: &[self::ml_string::Text],
    targets: &[f64],
    kind_2: &self::ml_string::Text,
    items: f64,
) -> Vec<f64> {
    {
        let array = (self::read_collection_construction_kind(
            texts, starts, counts, labels, targets, items,
        )) == (self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121]));
        {
            let element = self::read_collection_construction_field(
                starts,
                counts,
                labels,
                targets,
                items,
                &(self::ml_string::Text::from_units(vec![101, 108, 101, 109, 101, 110, 116])),
            );
            if (*kind_2) == (self::ml_string::Text::from_units(vec![115, 101, 116])) {
                if array {
                    vec![0f64, element, -1f64]
                } else {
                    vec![1f64, -1f64, -1f64]
                }
            } else {
                {
                    let tuple =
                        (self::read_collection_construction_kind(
                            texts, starts, counts, labels, targets, element,
                        )) == (self::ml_string::Text::from_units(vec![116, 117, 112, 108, 101]));
                    {
                        let elements = self::read_collection_construction_field(
                            starts,
                            counts,
                            labels,
                            targets,
                            element,
                            &(self::ml_string::Text::from_units(vec![
                                101, 108, 101, 109, 101, 110, 116, 115,
                            ])),
                        );
                        if ((!array || !tuple) || (elements < 0f64))
                            || ((self::read_required_number_fact(counts, elements)) != 2f64)
                        {
                            vec![2f64, -1f64, -1f64]
                        } else {
                            vec![
                                0f64,
                                self::read_collection_construction_element(
                                    starts, counts, targets, elements, 0f64,
                                ),
                                self::read_collection_construction_element(
                                    starts, counts, targets, elements, 1f64,
                                ),
                            ]
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_collection_construction_key_admission(
    kind_2: &self::ml_string::Text,
    primitive: bool,
) -> f64 {
    if primitive {
        0f64
    } else if (*kind_2) == (self::ml_string::Text::from_units(vec![115, 101, 116])) {
        3f64
    } else {
        4f64
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_collection_construction_diagnostic(
    code: f64,
    part: &self::ml_string::Text,
) -> self::ml_string::Text {
    if ((*part) == (self::ml_string::Text::from_units(vec![109, 101, 115, 115, 97, 103, 101])))
        && (code == 1f64)
    {
        self::ml_string::Text::from_units(vec![
            83, 101, 116, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 110, 101,
            101, 100, 115, 32, 97, 110, 32, 105, 116, 101, 114, 97, 98, 108, 101, 32, 97, 114, 114,
            97, 121,
        ])
    } else if ((*part)
        == (self::ml_string::Text::from_units(vec![109, 101, 115, 115, 97, 103, 101])))
        && (code == 2f64)
    {
        self::ml_string::Text::from_units(vec![
            77, 97, 112, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 110, 101,
            101, 100, 115, 32, 97, 110, 32, 105, 116, 101, 114, 97, 98, 108, 101, 32, 111, 102, 32,
            107, 101, 121, 47, 118, 97, 108, 117, 101, 32, 112, 97, 105, 114, 115,
        ])
    } else if ((*part)
        == (self::ml_string::Text::from_units(vec![109, 101, 115, 115, 97, 103, 101])))
        && (code == 3f64)
    {
        self::ml_string::Text::from_units(vec![
            115, 101, 116, 32, 107, 101, 121, 115, 32, 110, 101, 101, 100, 32, 101, 120, 112, 108,
            105, 99, 105, 116, 32, 114, 101, 102, 101, 114, 101, 110, 99, 101, 32, 105, 100, 101,
            110, 116, 105, 116, 105, 101, 115, 32, 102, 111, 114, 32, 115, 116, 114, 117, 99, 116,
            117, 114, 101, 100, 32, 118, 97, 108, 117, 101, 115,
        ])
    } else if ((*part)
        == (self::ml_string::Text::from_units(vec![109, 101, 115, 115, 97, 103, 101])))
        && (code == 4f64)
    {
        self::ml_string::Text::from_units(vec![
            109, 97, 112, 32, 107, 101, 121, 115, 32, 110, 101, 101, 100, 32, 101, 120, 112, 108,
            105, 99, 105, 116, 32, 114, 101, 102, 101, 114, 101, 110, 99, 101, 32, 105, 100, 101,
            110, 116, 105, 116, 105, 101, 115, 32, 102, 111, 114, 32, 115, 116, 114, 117, 99, 116,
            117, 114, 101, 100, 32, 118, 97, 108, 117, 101, 115,
        ])
    } else if ((*part)
        == (self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116])))
        && (code == 3f64)
    {
        self::ml_string::Text::from_units(vec![
            83, 101, 116, 32, 111, 98, 106, 101, 99, 116, 32, 105, 100, 101, 110, 116, 105, 116,
            121,
        ])
    } else if ((*part)
        == (self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116])))
        && (code == 4f64)
    {
        self::ml_string::Text::from_units(vec![
            77, 97, 112, 32, 111, 98, 106, 101, 99, 116, 32, 105, 100, 101, 110, 116, 105, 116, 121,
        ])
    } else {
        self::ml_string::Text::from_units(vec![])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_collection_construction_binding(
    target: &self::ml_string::Text,
    kind_2: &self::ml_string::Text,
    role: &self::ml_string::Text,
) -> self::ml_string::Text {
    {
        let set = (*kind_2) == (self::ml_string::Text::from_units(vec![115, 101, 116]));
        if (*target) == (self::ml_string::Text::from_units(vec![82, 111, 99, 113])) {
            if (*role)
                == (self::ml_string::Text::from_units(vec![112, 114, 101, 118, 105, 111, 117, 115]))
            {
                if set {
                    self::ml_string::Text::from_units(vec![
                        109, 108, 95, 112, 114, 101, 118, 105, 111, 117, 115,
                    ])
                } else {
                    self::ml_string::Text::from_units(vec![
                        40, 102, 115, 116, 32, 109, 108, 95, 112, 114, 101, 118, 105, 111, 117,
                        115, 41,
                    ])
                }
            } else if set {
                self::ml_string::Text::from_units(vec![109, 108, 95, 118, 97, 108, 117, 101])
            } else {
                self::ml_string::Text::from_units(vec![
                    40, 102, 115, 116, 32, 109, 108, 95, 118, 97, 108, 117, 101, 41,
                ])
            }
        } else if (*role)
            == (self::ml_string::Text::from_units(vec![112, 114, 101, 118, 105, 111, 117, 115]))
        {
            if set {
                self::ml_string::Text::from_units(vec![
                    109, 108, 95, 112, 114, 101, 118, 105, 111, 117, 115,
                ])
            } else {
                self::ml_string::Text::from_units(vec![
                    109, 108, 95, 112, 114, 101, 118, 105, 111, 117, 115, 46, 49,
                ])
            }
        } else if set {
            self::ml_string::Text::from_units(vec![109, 108, 95, 118, 97, 108, 117, 101])
        } else {
            self::ml_string::Text::from_units(vec![109, 108, 95, 118, 97, 108, 117, 101, 46, 49])
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_collection_construction_type(
    target: &self::ml_string::Text,
    kind_2: &self::ml_string::Text,
    first: &self::ml_string::Text,
    second: &self::ml_string::Text,
) -> self::ml_string::Text {
    if (*target) == (self::ml_string::Text::from_units(vec![76, 101, 97, 110])) {
        if (*kind_2) == (self::ml_string::Text::from_units(vec![115, 101, 116])) {
            ((self::ml_string::Text::from_units(vec![40, 65, 114, 114, 97, 121, 32])).concat(first))
                .concat(&self::ml_string::Text::from_units(vec![41]))
        } else {
            ((((self::ml_string::Text::from_units(vec![40, 65, 114, 114, 97, 121, 32, 40]))
                .concat(first))
            .concat(&self::ml_string::Text::from_units(vec![32, 215, 32])))
            .concat(second))
            .concat(&self::ml_string::Text::from_units(vec![41, 41]))
        }
    } else if (*target) == (self::ml_string::Text::from_units(vec![82, 111, 99, 113])) {
        if (*kind_2) == (self::ml_string::Text::from_units(vec![115, 101, 116])) {
            ((self::ml_string::Text::from_units(vec![40, 108, 105, 115, 116, 32])).concat(first))
                .concat(&self::ml_string::Text::from_units(vec![41]))
        } else {
            ((((self::ml_string::Text::from_units(vec![40, 108, 105, 115, 116, 32, 40]))
                .concat(first))
            .concat(&self::ml_string::Text::from_units(vec![32, 42, 32])))
            .concat(second))
            .concat(&self::ml_string::Text::from_units(vec![
                41, 41, 37, 116, 121, 112, 101,
            ]))
        }
    } else {
        self::ml_string::Text::from_units(vec![])
    }
}
