// Generated from complete js/src/translation/rust-standard-collection-value-packet.js. Do not edit by hand.
use super::ml;
use super::ml_array;
use super::ml_math;

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

/// A nullable value retains the distinct JavaScript null and undefined cases.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlOptional<T> {
    Value(T),
    Null,
    Undefined,
}

#[path = "ml_big_integer_bitwise.rs"]
pub mod ml_big_integer_bitwise;

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

#[path = "ml_big_integer_conversion.rs"]
pub mod ml_big_integer_conversion;

#[path = "ml_utf16_text.rs"]
pub mod ml_utf16_text;

#[derive(Clone, Debug, PartialEq)]
pub enum RustStandardCollectionOrigin {
    RustStandardCollectionOrigin(
        bool,
        bool,
        bool,
        self::ml_string::Text,
        self::ml_string::Text,
        (
            Vec<(
                Vec<(self::ml_string::Text, Vec<self::ml_string::Text>)>,
                self::ml_string::Text,
                Vec<self::ml_string::Text>,
            )>,
            self::ml_string::Text,
            Vec<self::ml_string::Text>,
        ),
        Vec<(
            bool,
            (
                Vec<(self::ml_string::Text, Vec<self::ml_string::Text>)>,
                self::ml_string::Text,
                Vec<self::ml_string::Text>,
            ),
            Option<(Option<self::ml_string::Text>,)>,
            Vec<self::ml_string::Text>,
        )>,
        f64,
        f64,
        bool,
        bool,
    ),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RustStandardCollectionCallOrigin {
    RustStandardCollectionCallOrigin(
        self::ml_string::Text,
        self::ml_string::Text,
        self::ml_string::Text,
        self::ml_string::Text,
        self::ml_string::Text,
        bool,
        bool,
        bool,
        bool,
        bool,
        Vec<(
            bool,
            bool,
            self::ml_string::Text,
            bool,
            bool,
            (
                Vec<(self::ml_string::Text, Vec<self::ml_string::Text>)>,
                self::ml_string::Text,
                Vec<self::ml_string::Text>,
            ),
        )>,
        Option<(Option<self::ml_string::Text>,)>,
        Option<(Option<bool>,)>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum RustStandardCollectionScalar {
    RustCollectionBoolean(bool),
    RustCollectionInteger(self::ml::Big),
    RustCollectionString(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum RustStandardCollectionOptionValue {
    RustCollectionReference(
        self::ml_string::Text,
        f64,
        Box<self::RustStandardCollectionScalar>,
        self::ml_string::Text,
    ),
    RustCollectionBoolean(bool),
    RustCollectionInteger(self::ml::Big),
    RustCollectionString(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum RustStandardCollectionOption {
    RustCollectionOption(
        bool,
        Option<(Option<self::RustStandardCollectionOptionValue>,)>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum RustStandardCollectionState {
    RustStandardCollectionState(
        Box<self::RustStandardCollectionOrigin>,
        self::ml_string::Text,
        self::ml_string::Text,
        self::ml_string::Text,
        f64,
        f64,
        Vec<(
            self::RustStandardCollectionScalar,
            Option<(Option<self::RustStandardCollectionScalar>,)>,
        )>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum RustStandardCollectionValue {
    RustCollectionOrderedValues(Vec<self::RustStandardCollectionScalar>),
    RustCollectionOrderedEntries(
        Vec<(
            self::RustStandardCollectionScalar,
            Option<(Option<self::RustStandardCollectionScalar>,)>,
        )>,
    ),
    RustCollectionUnit,
    RustCollectionBoolean(bool),
    RustCollectionInteger(self::ml::Big),
    RustCollectionString(Vec<f64>),
    RustCollectionOption(
        bool,
        Option<(Option<self::RustStandardCollectionOptionValue>,)>,
    ),
    RustStandardCollectionState(
        Box<self::RustStandardCollectionOrigin>,
        self::ml_string::Text,
        self::ml_string::Text,
        self::ml_string::Text,
        f64,
        f64,
        Vec<(
            self::RustStandardCollectionScalar,
            Option<(Option<self::RustStandardCollectionScalar>,)>,
        )>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum RustStandardCollectionTransition {
    RustCollectionCreated(Box<self::RustStandardCollectionState>),
    RustCollectionNormal(
        Box<self::RustStandardCollectionState>,
        Box<self::RustStandardCollectionValue>,
        bool,
    ),
    RustCollectionInvalid(self::ml_string::Text),
    RustCollectionResourceLimit(
        Option<(Option<self::RustStandardCollectionState>,)>,
        self::ml_string::Text,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum RustStandardCollectionOrderingOrigin {
    RustStandardCollectionOrderingOrigin(
        self::ml_string::Text,
        self::ml_string::Text,
        f64,
        bool,
        bool,
        bool,
        bool,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum RustStandardCollectionOrderingState {
    RustCollectionOrderingState(
        Box<self::RustStandardCollectionState>,
        Box<self::RustStandardCollectionState>,
        Box<self::RustStandardCollectionOrderingOrigin>,
        f64,
        self::ml_string::Text,
        f64,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum RustStandardCollectionOrderingTransition {
    RustCollectionOrderingContinue(Box<self::RustStandardCollectionOrderingState>),
    RustCollectionOrderingCompleted(f64),
    RustCollectionOrderingInvalid(self::ml_string::Text),
    RustCollectionOrderingResourceLimit(
        Box<self::RustStandardCollectionOrderingState>,
        self::ml_string::Text,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum RustStandardCollectionEqualityOrigin {
    RustStandardCollectionEqualityOrigin(
        self::ml_string::Text,
        self::ml_string::Text,
        self::ml_string::Text,
        f64,
        bool,
        bool,
        bool,
        bool,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum RustStandardCollectionEqualityState {
    RustCollectionEqualityState(
        Box<self::RustStandardCollectionState>,
        Box<self::RustStandardCollectionState>,
        Box<self::RustStandardCollectionEqualityOrigin>,
        f64,
        f64,
        self::ml_string::Text,
        f64,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum RustStandardCollectionEqualityTransition {
    RustCollectionEqualityContinue(Box<self::RustStandardCollectionEqualityState>),
    RustCollectionEqualityCompleted(bool),
    RustCollectionEqualityInvalid(self::ml_string::Text),
    RustCollectionEqualityResourceLimit(
        Box<self::RustStandardCollectionEqualityState>,
        self::ml_string::Text,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum RustStandardCollectionPacketTransition {
    RustCollectionPacketNormal(
        (
            self::RustStandardCollectionValue,
            self::RustStandardCollectionState,
        ),
        bool,
    ),
    RustCollectionInvalid(self::ml_string::Text),
    RustCollectionResourceLimit(
        Option<(Option<self::RustStandardCollectionState>,)>,
        self::ml_string::Text,
    ),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn resolve_rust_standard_collection_owner(
    path: &self::ml_string::Text,
    ordinary: bool,
    shadowed: bool,
) -> self::ml_string::Text {
    if !ordinary || shadowed {
        self::ml_string::Text::from_units(vec![])
    } else if ((*path)
        == (self::ml_string::Text::from_units(vec![
            115, 116, 100, 58, 58, 99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 115, 58, 58, 72,
            97, 115, 104, 77, 97, 112,
        ])))
        || ((*path)
            == (self::ml_string::Text::from_units(vec![
                115, 116, 100, 58, 58, 99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 115, 58, 58,
                104, 97, 115, 104, 95, 109, 97, 112, 58, 58, 72, 97, 115, 104, 77, 97, 112,
            ])))
    {
        self::ml_string::Text::from_units(vec![
            115, 116, 100, 58, 58, 99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 115, 58, 58, 72,
            97, 115, 104, 77, 97, 112,
        ])
    } else if ((*path)
        == (self::ml_string::Text::from_units(vec![
            115, 116, 100, 58, 58, 99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 115, 58, 58, 72,
            97, 115, 104, 83, 101, 116,
        ])))
        || ((*path)
            == (self::ml_string::Text::from_units(vec![
                115, 116, 100, 58, 58, 99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 115, 58, 58,
                104, 97, 115, 104, 95, 115, 101, 116, 58, 58, 72, 97, 115, 104, 83, 101, 116,
            ])))
    {
        self::ml_string::Text::from_units(vec![
            115, 116, 100, 58, 58, 99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 115, 58, 58, 72,
            97, 115, 104, 83, 101, 116,
        ])
    } else if ((*path)
        == (self::ml_string::Text::from_units(vec![
            115, 116, 100, 58, 58, 99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 115, 58, 58, 66,
            84, 114, 101, 101, 77, 97, 112,
        ])))
        || ((*path)
            == (self::ml_string::Text::from_units(vec![
                115, 116, 100, 58, 58, 99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 115, 58, 58,
                98, 116, 114, 101, 101, 95, 109, 97, 112, 58, 58, 66, 84, 114, 101, 101, 77, 97,
                112,
            ])))
    {
        self::ml_string::Text::from_units(vec![
            115, 116, 100, 58, 58, 99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 115, 58, 58, 66,
            84, 114, 101, 101, 77, 97, 112,
        ])
    } else if ((*path)
        == (self::ml_string::Text::from_units(vec![
            115, 116, 100, 58, 58, 99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 115, 58, 58, 66,
            84, 114, 101, 101, 83, 101, 116,
        ])))
        || ((*path)
            == (self::ml_string::Text::from_units(vec![
                115, 116, 100, 58, 58, 99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 115, 58, 58,
                98, 116, 114, 101, 101, 95, 115, 101, 116, 58, 58, 66, 84, 114, 101, 101, 83, 101,
                116,
            ])))
    {
        self::ml_string::Text::from_units(vec![
            115, 116, 100, 58, 58, 99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 115, 58, 58, 66,
            84, 114, 101, 101, 83, 101, 116,
        ])
    } else {
        self::ml_string::Text::from_units(vec![])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_standard_collection_kind(owner: &self::ml_string::Text) -> self::ml_string::Text {
    {
        let resolved = self::resolve_rust_standard_collection_owner(owner, true, false);
        if self::ml_array::includes(
            &vec![
                self::ml_string::Text::from_units(vec![
                    115, 116, 100, 58, 58, 99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 115, 58,
                    58, 72, 97, 115, 104, 77, 97, 112,
                ]),
                self::ml_string::Text::from_units(vec![
                    115, 116, 100, 58, 58, 99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 115, 58,
                    58, 66, 84, 114, 101, 101, 77, 97, 112,
                ]),
            ],
            &resolved.clone(),
            0f64,
            false,
            |a, b| a == b,
        ) {
            self::ml_string::Text::from_units(vec![109, 97, 112])
        } else if self::ml_array::includes(
            &vec![
                self::ml_string::Text::from_units(vec![
                    115, 116, 100, 58, 58, 99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 115, 58,
                    58, 72, 97, 115, 104, 83, 101, 116,
                ]),
                self::ml_string::Text::from_units(vec![
                    115, 116, 100, 58, 58, 99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 115, 58,
                    58, 66, 84, 114, 101, 101, 83, 101, 116,
                ]),
            ],
            &resolved.clone(),
            0f64,
            false,
            |a, b| a == b,
        ) {
            self::ml_string::Text::from_units(vec![115, 101, 116])
        } else {
            self::ml_string::Text::from_units(vec![])
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_rust_standard_collection_ordered(owner: &self::ml_string::Text) -> bool {
    {
        let resolved = self::resolve_rust_standard_collection_owner(owner, true, false);
        ((resolved
            == (self::ml_string::Text::from_units(vec![
                115, 116, 100, 58, 58, 99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 115, 58, 58,
                66, 84, 114, 101, 101, 77, 97, 112,
            ])))
            || (resolved
                == (self::ml_string::Text::from_units(vec![
                    115, 116, 100, 58, 58, 99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 115, 58,
                    58, 66, 84, 114, 101, 101, 83, 101, 116,
                ]))))
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_standard_collection_source_version_error(minor: f64) -> self::ml_string::Text {
    if (self::ml_math::is_integer(minor) && (minor >= 90f64)) && (minor < 100f64) {
        self::ml_string::Text::from_units(vec![])
    } else {
        self::ml_string::Text::from_units(vec![
            115, 116, 97, 110, 100, 97, 114, 100, 32, 99, 111, 108, 108, 101, 99, 116, 105, 111,
            110, 32, 115, 111, 117, 114, 99, 101, 32, 115, 101, 109, 97, 110, 116, 105, 99, 115,
            32, 97, 114, 101, 32, 117, 110, 99, 111, 118, 101, 114, 101, 100, 32, 111, 117, 116,
            115, 105, 100, 101, 32, 82, 117, 115, 116, 32, 62, 61, 49, 46, 57, 48, 46, 48, 32, 60,
            49, 46, 49, 48, 48, 46, 48,
        ])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_standard_collection_domain_error(
    domain: &self::ml_string::Text,
    pointer_width: f64,
) -> self::ml_string::Text {
    if (pointer_width != 32f64) && (pointer_width != 64f64) {
        self::ml_string::Text::from_units(vec![
            99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 105, 110, 116, 101, 103, 101, 114,
            32, 100, 111, 109, 97, 105, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 101,
            120, 112, 108, 105, 99, 105, 116, 32, 115, 117, 112, 112, 111, 114, 116, 101, 100, 32,
            116, 97, 114, 103, 101, 116, 32, 112, 111, 105, 110, 116, 101, 114, 32, 119, 105, 100,
            116, 104,
        ])
    } else if self::ml_array::includes(
        &vec![
            self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103]),
            self::ml_string::Text::from_units(vec![98, 111, 111, 108]),
            self::ml_string::Text::from_units(vec![105, 56]),
            self::ml_string::Text::from_units(vec![105, 49, 54]),
            self::ml_string::Text::from_units(vec![105, 51, 50]),
            self::ml_string::Text::from_units(vec![105, 54, 52]),
            self::ml_string::Text::from_units(vec![105, 49, 50, 56]),
            self::ml_string::Text::from_units(vec![105, 115, 105, 122, 101]),
            self::ml_string::Text::from_units(vec![117, 56]),
            self::ml_string::Text::from_units(vec![117, 49, 54]),
            self::ml_string::Text::from_units(vec![117, 51, 50]),
            self::ml_string::Text::from_units(vec![117, 54, 52]),
            self::ml_string::Text::from_units(vec![117, 49, 50, 56]),
            self::ml_string::Text::from_units(vec![117, 115, 105, 122, 101]),
        ],
        domain,
        0f64,
        false,
        |a, b| a == b,
    ) {
        self::ml_string::Text::from_units(vec![])
    } else {
        self::ml_string::Text::from_units(vec![
            99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114,
            101, 32, 99, 108, 111, 115, 101, 100, 32, 83, 116, 114, 105, 110, 103, 32, 66, 111,
            111, 108, 101, 97, 110, 32, 111, 114, 32, 105, 110, 116, 101, 103, 101, 114, 32, 100,
            111, 109, 97, 105, 110, 115, 59, 32, 99, 117, 115, 116, 111, 109, 32, 72, 97, 115, 104,
            47, 69, 113, 47, 79, 114, 100, 32, 97, 110, 100, 32, 102, 108, 111, 97, 116, 32, 100,
            111, 109, 97, 105, 110, 115, 32, 110, 101, 101, 100, 32, 115, 101, 112, 97, 114, 97,
            116, 101, 32, 115, 101, 109, 97, 110, 116, 105, 99, 115,
        ])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_standard_collection_integer_width(
    domain: &self::ml_string::Text,
    pointer_width: f64,
) -> f64 {
    if (((self::read_rust_standard_collection_domain_error(domain, pointer_width))
        != (self::ml_string::Text::from_units(vec![])))
        || ((*domain) == (self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103]))))
        || ((*domain) == (self::ml_string::Text::from_units(vec![98, 111, 111, 108])))
    {
        0f64
    } else if ((*domain) == (self::ml_string::Text::from_units(vec![105, 115, 105, 122, 101])))
        || ((*domain) == (self::ml_string::Text::from_units(vec![117, 115, 105, 122, 101])))
    {
        pointer_width
    } else {
        self::ml_string_number::convert_java_script_string_number_units(
            &({
                let source_ml_source2_string_range_operand1 = domain.clone();
                {
                    let source_ml_source2_string_range_operand2 = 1f64;
                    {
                        let source_ml_source2_string_range_position3 =
                            source_ml_source2_string_range_operand2;
                        {
                            let source_ml_source2_string_range_position4 = f64::INFINITY;
                            source_ml_source2_string_range_operand1.slice(
                                source_ml_source2_string_range_position3,
                                source_ml_source2_string_range_position4,
                            )
                        }
                    }
                }
            })
            .0
            .iter()
            .copied()
            .map(f64::from)
            .collect::<Vec<f64>>(),
        )
    }
}
