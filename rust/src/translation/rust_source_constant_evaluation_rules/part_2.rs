// Generated from complete js/src/translation/rust-source-constant-evaluation-rules.js. Do not edit by hand.
use super::MlTemporaryReadRustIntegerRangeSuccessorLoop3Result;
use super::MlTemporaryTestRustIntegerRangeCanonicalDecimalLoop1Result;
use super::ml_array;
use super::source_ml_temporary_compare_rust_integer_range_decimals_loop2;
use super::source_ml_temporary_read_rust_integer_range_successor_loop3;
use super::source_ml_temporary_test_rust_integer_range_canonical_decimal_loop1;

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

#[path = "ml_big_integer_bitwise.rs"]
pub mod ml_big_integer_bitwise;

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
pub fn accept_rust_source_signature_model(model: &self::ml_string::Text) -> bool {
    ((*model) == (self::ml_string::Text::from_units(vec![])))
        || ((*model)
            == (self::ml_string::Text::from_units(vec![
                82, 117, 115, 116, 32, 115, 111, 117, 114, 99, 101, 32, 102, 117, 110, 99, 116,
                105, 111, 110, 32, 115, 105, 103, 110, 97, 116, 117, 114, 101, 32, 49,
            ])))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_source_signature_wrapper(wrapper: &self::ml_string::Text) -> bool {
    ((*wrapper)
        == (self::ml_string::Text::from_units(vec![114, 101, 102, 101, 114, 101, 110, 99, 101])))
        || ((*wrapper) == (self::ml_string::Text::from_units(vec![98, 111, 120])))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_source_signature_owner_kind(kind: &self::ml_string::Text) -> bool {
    ((((((*kind)
        == (self::ml_string::Text::from_units(vec![
            112, 114, 105, 109, 105, 116, 105, 118, 101,
        ])))
        || ((*kind)
            == (self::ml_string::Text::from_units(vec![
                100, 101, 99, 108, 97, 114, 97, 116, 105, 111, 110,
            ]))))
        || ((*kind)
            == (self::ml_string::Text::from_units(vec![108, 105, 98, 114, 97, 114, 121]))))
        || ((*kind) == (self::ml_string::Text::from_units(vec![116, 117, 112, 108, 101]))))
        || ((*kind) == (self::ml_string::Text::from_units(vec![117, 110, 105, 116]))))
        || ((*kind) == (self::ml_string::Text::from_units(vec![115, 108, 105, 99, 101])))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_source_signature_catalog_owner(kind: &self::ml_string::Text) -> bool {
    ((((*kind) == (self::ml_string::Text::from_units(vec![100, 97, 116, 97])))
        || ((*kind) == (self::ml_string::Text::from_units(vec![109, 111, 100, 117, 108, 101]))))
        || ((*kind)
            == (self::ml_string::Text::from_units(vec![
                101, 120, 116, 101, 114, 110, 97, 108, 67, 114, 97, 116, 101,
            ]))))
        || ((*kind) == (self::ml_string::Text::from_units(vec![99, 116, 111, 114])))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_source_signature_primitive(
    kind: &self::ml_string::Text,
    path: &self::ml_string::Text,
    bits: &self::ml_string::Text,
    signed: bool,
) -> bool {
    if (*path) == (self::ml_string::Text::from_units(vec![102, 54, 52])) {
        (*kind) == (self::ml_string::Text::from_units(vec![102, 108, 111, 97, 116]))
    } else if (*path) == (self::ml_string::Text::from_units(vec![98, 111, 111, 108])) {
        (*kind) == (self::ml_string::Text::from_units(vec![98, 111, 111, 108]))
    } else if (*path) == (self::ml_string::Text::from_units(vec![115, 116, 114])) {
        (*kind) == (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103]))
    } else if (*path) == (self::ml_string::Text::from_units(vec![105, 56])) {
        (((*kind) == (self::ml_string::Text::from_units(vec![102, 105, 120, 101, 100])))
            && ((*bits) == (self::ml_string::Text::from_units(vec![56]))))
            && (signed == (true))
    } else if (*path) == (self::ml_string::Text::from_units(vec![117, 56])) {
        (((*kind) == (self::ml_string::Text::from_units(vec![102, 105, 120, 101, 100])))
            && ((*bits) == (self::ml_string::Text::from_units(vec![56]))))
            && (signed == (false))
    } else if (*path) == (self::ml_string::Text::from_units(vec![105, 49, 54])) {
        (((*kind) == (self::ml_string::Text::from_units(vec![102, 105, 120, 101, 100])))
            && ((*bits) == (self::ml_string::Text::from_units(vec![49, 54]))))
            && (signed == (true))
    } else if (*path) == (self::ml_string::Text::from_units(vec![117, 49, 54])) {
        (((*kind) == (self::ml_string::Text::from_units(vec![102, 105, 120, 101, 100])))
            && ((*bits) == (self::ml_string::Text::from_units(vec![49, 54]))))
            && (signed == (false))
    } else if (*path) == (self::ml_string::Text::from_units(vec![105, 51, 50])) {
        (((*kind) == (self::ml_string::Text::from_units(vec![102, 105, 120, 101, 100])))
            && ((*bits) == (self::ml_string::Text::from_units(vec![51, 50]))))
            && (signed == (true))
    } else if (*path) == (self::ml_string::Text::from_units(vec![117, 51, 50])) {
        (((*kind) == (self::ml_string::Text::from_units(vec![102, 105, 120, 101, 100])))
            && ((*bits) == (self::ml_string::Text::from_units(vec![51, 50]))))
            && (signed == (false))
    } else if (*path) == (self::ml_string::Text::from_units(vec![105, 54, 52])) {
        (((*kind) == (self::ml_string::Text::from_units(vec![102, 105, 120, 101, 100])))
            && ((*bits) == (self::ml_string::Text::from_units(vec![54, 52]))))
            && (signed == (true))
    } else if (*path) == (self::ml_string::Text::from_units(vec![117, 54, 52])) {
        (((*kind) == (self::ml_string::Text::from_units(vec![102, 105, 120, 101, 100])))
            && ((*bits) == (self::ml_string::Text::from_units(vec![54, 52]))))
            && (signed == (false))
    } else if (*path) == (self::ml_string::Text::from_units(vec![105, 49, 50, 56])) {
        (((*kind) == (self::ml_string::Text::from_units(vec![102, 105, 120, 101, 100])))
            && ((*bits) == (self::ml_string::Text::from_units(vec![49, 50, 56]))))
            && (signed == (true))
    } else if (*path) == (self::ml_string::Text::from_units(vec![117, 49, 50, 56])) {
        (((*kind) == (self::ml_string::Text::from_units(vec![102, 105, 120, 101, 100])))
            && ((*bits) == (self::ml_string::Text::from_units(vec![49, 50, 56]))))
            && (signed == (false))
    } else if (*path) == (self::ml_string::Text::from_units(vec![117, 115, 105, 122, 101])) {
        (((*kind) == (self::ml_string::Text::from_units(vec![102, 105, 120, 101, 100])))
            && ((*bits) == (self::ml_string::Text::from_units(vec![54, 52]))))
            && (signed == (false))
    } else if (*path) == (self::ml_string::Text::from_units(vec![105, 115, 105, 122, 101])) {
        (((*kind) == (self::ml_string::Text::from_units(vec![102, 105, 120, 101, 100])))
            && ((*bits) == (self::ml_string::Text::from_units(vec![54, 52]))))
            && (signed == (true))
    } else {
        false
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_rust_integer_range_canonical_decimal(value: &self::ml_string::Text) -> bool {
    {
        let digits = value.clone();
        {
            let digits_3 = if digits.starts_with(&self::ml_string::Text::from_units(vec![45])) {
                {
                    {
                        let source_ml_source1_string_range_operand1 = digits;
                        {
                            let source_ml_source1_string_range_operand2 = 1f64;
                            {
                                let source_ml_source1_string_range_position3 =
                                    source_ml_source1_string_range_operand2;
                                {
                                    let source_ml_source1_string_range_position4 = f64::INFINITY;
                                    source_ml_source1_string_range_operand1.slice(
                                        source_ml_source1_string_range_position3,
                                        source_ml_source1_string_range_position4,
                                    )
                                }
                            }
                        }
                    }
                }
            } else {
                digits
            };
            if ((digits_3 == (self::ml_string::Text::from_units(vec![])))
                || (((digits_3.length()) > 1f64)
                    && digits_3.starts_with(&self::ml_string::Text::from_units(vec![48]))))
                || ((*value) == (self::ml_string::Text::from_units(vec![45, 48])))
            {
                false
            } else {
                {
                    let at = 0f64;
                    {
                        let source_ml_source1_s5 = self::source_ml_temporary_test_rust_integer_range_canonical_decimal_loop1(&(digits_3), at);
                        match source_ml_source1_s5 {
                            self::MlTemporaryTestRustIntegerRangeCanonicalDecimalLoop1Result::MlTemporaryTestRustIntegerRangeCanonicalDecimalLoop1Done => {
                                true
                            }
                            self::MlTemporaryTestRustIntegerRangeCanonicalDecimalLoop1Result::MlTemporaryTestRustIntegerRangeCanonicalDecimalLoop1Return(source_ml_temporary_result1) => {
                                source_ml_temporary_result1
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
pub fn compare_rust_integer_range_decimals(
    left: &self::ml_string::Text,
    right: &self::ml_string::Text,
) -> f64 {
    if !(self::test_rust_integer_range_canonical_decimal(left))
        || !(self::test_rust_integer_range_canonical_decimal(right))
    {
        2f64
    } else {
        {
            let negative_left = left.starts_with(&self::ml_string::Text::from_units(vec![45]));
            {
                let negative_right =
                    right.starts_with(&self::ml_string::Text::from_units(vec![45]));
                {
                    let ml_library_inequality_1 = negative_left != negative_right;
                    if ml_library_inequality_1 {
                        if negative_left { -1f64 } else { 1f64 }
                    } else {
                        {
                            let a = if negative_left {
                                {
                                    let source_ml_source1_string_range_operand6 = left.clone();
                                    {
                                        let source_ml_source1_string_range_operand7 = 1f64;
                                        {
                                            let source_ml_source1_string_range_position8 =
                                                source_ml_source1_string_range_operand7;
                                            {
                                                let source_ml_source1_string_range_position9 =
                                                    f64::INFINITY;
                                                source_ml_source1_string_range_operand6.slice(
                                                    source_ml_source1_string_range_position8,
                                                    source_ml_source1_string_range_position9,
                                                )
                                            }
                                        }
                                    }
                                }
                            } else {
                                left.clone()
                            };
                            {
                                let b = if negative_right {
                                    {
                                        let source_ml_source1_string_range_operand10 =
                                            right.clone();
                                        {
                                            let source_ml_source1_string_range_operand11 = 1f64;
                                            {
                                                let source_ml_source1_string_range_position12 =
                                                    source_ml_source1_string_range_operand11;
                                                {
                                                    let source_ml_source1_string_range_position13 =
                                                        f64::INFINITY;
                                                    source_ml_source1_string_range_operand10.slice(
                                                        source_ml_source1_string_range_position12,
                                                        source_ml_source1_string_range_position13,
                                                    )
                                                }
                                            }
                                        }
                                    }
                                } else {
                                    right.clone()
                                };
                                {
                                    let order = if (a.length()) < (b.length()) {
                                        -1f64
                                    } else if (a.length()) > (b.length()) {
                                        1f64
                                    } else {
                                        0f64
                                    };
                                    {
                                        let order_3 = if order == 0f64 {
                                            {
                                                let at = 0f64;
                                                {
                                                    self::source_ml_temporary_compare_rust_integer_range_decimals_loop2(&(a), &(b), order, at)
                                                }
                                            }
                                        } else {
                                            order
                                        };
                                        if order_3 == 0f64 {
                                            0f64
                                        } else if negative_left {
                                            -order_3
                                        } else {
                                            order_3
                                        }
                                    }
                                }
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
pub fn read_rust_integer_range_successor(value: &self::ml_string::Text) -> self::ml_string::Text {
    if (self::test_rust_integer_range_canonical_decimal(value)) {
        {
            let negative = value.starts_with(&self::ml_string::Text::from_units(vec![45]));
            {
                let digits = if negative {
                    {
                        let source_ml_source1_string_range_operand14 = value.clone();
                        {
                            let source_ml_source1_string_range_operand15 = 1f64;
                            {
                                let source_ml_source1_string_range_position16 =
                                    source_ml_source1_string_range_operand15;
                                {
                                    let source_ml_source1_string_range_position17 = f64::INFINITY;
                                    source_ml_source1_string_range_operand14.slice(
                                        source_ml_source1_string_range_position16,
                                        source_ml_source1_string_range_position17,
                                    )
                                }
                            }
                        }
                    }
                } else {
                    value.clone()
                };
                {
                    let result = self::ml_string::Text::from_units(vec![]);
                    {
                        let carry = true;
                        {
                            let at = digits.length() - 1f64;
                            {
                                let source_ml_source1_s18 = self::source_ml_temporary_read_rust_integer_range_successor_loop3(negative, &(digits), result, carry, at);
                                match source_ml_source1_s18 {
                                    self::MlTemporaryReadRustIntegerRangeSuccessorLoop3Result::MlTemporaryReadRustIntegerRangeSuccessorLoop3Done(result_2, carry_2) => {
                                        if negative {
                                            {
                                                let result_4 = if result_2.starts_with(&self::ml_string::Text::from_units(vec![48])) && ((result_2.length()) > 1f64) {
                                                    {
                                                        {
                                                            let source_ml_source1_string_range_operand19 = result_2.clone();
                                                            {
                                                                let source_ml_source1_string_range_operand20 = 1f64;
                                                                {
                                                                    let source_ml_source1_string_range_position21 = source_ml_source1_string_range_operand20;
                                                                    {
                                                                        let source_ml_source1_string_range_position22 = f64::INFINITY;
                                                                        source_ml_source1_string_range_operand19.slice(source_ml_source1_string_range_position21, source_ml_source1_string_range_position22)
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    result_2.clone()
                                                };
                                                if result_4 == (self::ml_string::Text::from_units(vec![48])) {
                                                    self::ml_string::Text::from_units(vec![48])
                                                } else {
                                                    (self::ml_string::Text::from_units(vec![45])).concat(&result_4)
                                                }
                                            }
                                        } else if carry_2 {
                                                (self::ml_string::Text::from_units(vec![49])).concat(&result_2)
                                            } else {
                                                result_2.clone()
                                            }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    } else {
        self::ml_string::Text::from_units(vec![])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_integer_range_source_error(
    ordinary: bool,
    operation: &self::ml_string::Text,
    lower: bool,
    upper: bool,
) -> self::ml_string::Text {
    if ordinary {
        if ((*operation) != (self::ml_string::Text::from_units(vec![46, 46])))
            && ((*operation) != (self::ml_string::Text::from_units(vec![46, 46, 61])))
        {
            self::ml_string::Text::from_units(vec![
                97, 32, 114, 97, 110, 103, 101, 32, 112, 97, 116, 116, 101, 114, 110, 32, 114, 101,
                113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32,
                114, 97, 110, 103, 101, 32, 111, 112, 101, 114, 97, 116, 111, 114,
            ])
        } else if ((lower) || (upper)) {
            if ((*operation) == (self::ml_string::Text::from_units(vec![46, 46, 61]))) && !upper {
                self::ml_string::Text::from_units(vec![
                    97, 110, 32, 105, 110, 99, 108, 117, 115, 105, 118, 101, 32, 114, 97, 110, 103,
                    101, 32, 112, 97, 116, 116, 101, 114, 110, 32, 110, 101, 101, 100, 115, 32,
                    105, 116, 115, 32, 117, 112, 112, 101, 114, 32, 101, 110, 100, 112, 111, 105,
                    110, 116,
                ])
            } else {
                self::ml_string::Text::from_units(vec![])
            }
        } else {
            self::ml_string::Text::from_units(vec![
                97, 32, 114, 97, 110, 103, 101, 32, 112, 97, 116, 116, 101, 114, 110, 32, 110, 101,
                101, 100, 115, 32, 97, 116, 32, 108, 101, 97, 115, 116, 32, 111, 110, 101, 32, 101,
                110, 100, 112, 111, 105, 110, 116,
            ])
        }
    } else {
        self::ml_string::Text::from_units(vec![
            114, 97, 110, 103, 101, 115, 32, 97, 114, 101, 32, 111, 117, 116, 115, 105, 100, 101,
            32, 116, 104, 101, 32, 112, 111, 114, 116, 97, 98, 108, 101, 32, 112, 97, 116, 116,
            101, 114, 110, 32, 108, 97, 110, 103, 117, 97, 103, 101,
        ])
    }
}
