// Generated from complete js/src/translation/disjoint-collection-iteration-rules.js. Do not edit by hand.
use super::MlSource1TemporaryReadReferenceMethodIdentityJoin3;
use super::MlSource1TemporaryReadReferenceMethodIdentityJoin4;
use super::MlSource1TemporaryReadReferenceMethodIdentityJoin5;
use super::MlSource1TemporaryReadReferenceMethodResultJoin1;
use super::MlSource1TemporaryReadReferenceMethodResultJoin2;
use super::ml_array;

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
pub fn read_reference_method_result(
    kind: &self::ml_string::Text,
    operation: &self::ml_string::Text,
) -> self::ml_string::Text {
    {
        let source_ml_source2_s1 = if (*kind)
            == (self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121]))
        {
            if ((*operation) == (self::ml_string::Text::from_units(vec![112, 117, 115, 104])))
                || ((*operation)
                    == (self::ml_string::Text::from_units(vec![117, 110, 115, 104, 105, 102, 116])))
            {
                self::MlSource1TemporaryReadReferenceMethodResultJoin1::MlSource1TemporaryReadReferenceMethodResultJoin1Return(self::ml_string::Text::from_units(vec![108, 101, 110, 103, 116, 104]))
            } else if ((*operation) == (self::ml_string::Text::from_units(vec![112, 111, 112])))
                || ((*operation)
                    == (self::ml_string::Text::from_units(vec![115, 104, 105, 102, 116])))
            {
                self::MlSource1TemporaryReadReferenceMethodResultJoin1::MlSource1TemporaryReadReferenceMethodResultJoin1Return(self::ml_string::Text::from_units(vec![111, 112, 116, 105, 111, 110, 97, 108, 32, 101, 108, 101, 109, 101, 110, 116]))
            } else if (*operation)
                == (self::ml_string::Text::from_units(vec![114, 101, 118, 101, 114, 115, 101]))
            {
                self::MlSource1TemporaryReadReferenceMethodResultJoin1::MlSource1TemporaryReadReferenceMethodResultJoin1Return(self::ml_string::Text::from_units(vec![114, 101, 99, 101, 105, 118, 101, 114]))
            } else {
                self::MlSource1TemporaryReadReferenceMethodResultJoin1::MlSource1TemporaryReadReferenceMethodResultJoin1Next
            }
        } else {
            self::MlSource1TemporaryReadReferenceMethodResultJoin1::MlSource1TemporaryReadReferenceMethodResultJoin1Next
        };
        match source_ml_source2_s1 {
            self::MlSource1TemporaryReadReferenceMethodResultJoin1::MlSource1TemporaryReadReferenceMethodResultJoin1Next => {
                if (((*kind) == (self::ml_string::Text::from_units(vec![115, 101, 116]))) && ((*operation) == (self::ml_string::Text::from_units(vec![97, 100, 100])))) || (((*kind) == (self::ml_string::Text::from_units(vec![109, 97, 112]))) && ((*operation) == (self::ml_string::Text::from_units(vec![115, 101, 116])))) {
                    self::ml_string::Text::from_units(vec![114, 101, 99, 101, 105, 118, 101, 114])
                } else {
                    {
                        let source_ml_source2_s2 = if ((*kind) == (self::ml_string::Text::from_units(vec![115, 101, 116]))) || ((*kind) == (self::ml_string::Text::from_units(vec![109, 97, 112]))) {
                            if ((*operation) == (self::ml_string::Text::from_units(vec![104, 97, 115]))) || ((*operation) == (self::ml_string::Text::from_units(vec![100, 101, 108, 101, 116, 101]))) {
                                self::MlSource1TemporaryReadReferenceMethodResultJoin2::MlSource1TemporaryReadReferenceMethodResultJoin2Return(self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110]))
                            } else if (*operation) == (self::ml_string::Text::from_units(vec![99, 108, 101, 97, 114])) {
                                    self::MlSource1TemporaryReadReferenceMethodResultJoin2::MlSource1TemporaryReadReferenceMethodResultJoin2Return(self::ml_string::Text::from_units(vec![117, 110, 105, 116]))
                                } else if ((*kind) == (self::ml_string::Text::from_units(vec![109, 97, 112]))) && ((*operation) == (self::ml_string::Text::from_units(vec![103, 101, 116]))) {
                                        self::MlSource1TemporaryReadReferenceMethodResultJoin2::MlSource1TemporaryReadReferenceMethodResultJoin2Return(self::ml_string::Text::from_units(vec![111, 112, 116, 105, 111, 110, 97, 108, 32, 118, 97, 108, 117, 101]))
                                    } else {
                                        self::MlSource1TemporaryReadReferenceMethodResultJoin2::MlSource1TemporaryReadReferenceMethodResultJoin2Next
                                    }
                        } else {
                            self::MlSource1TemporaryReadReferenceMethodResultJoin2::MlSource1TemporaryReadReferenceMethodResultJoin2Next
                        };
                        match source_ml_source2_s2 {
                            self::MlSource1TemporaryReadReferenceMethodResultJoin2::MlSource1TemporaryReadReferenceMethodResultJoin2Next => {
                                self::ml_string::Text::from_units(vec![])
                            }
                            self::MlSource1TemporaryReadReferenceMethodResultJoin2::MlSource1TemporaryReadReferenceMethodResultJoin2Return(source_ml_source1_temporary_result2) => {
                                source_ml_source1_temporary_result2
                            }
                        }
                    }
                }
            }
            self::MlSource1TemporaryReadReferenceMethodResultJoin1::MlSource1TemporaryReadReferenceMethodResultJoin1Return(source_ml_source1_temporary_result2_2) => {
                source_ml_source1_temporary_result2_2
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_linked_reference_method_operation(
    expression: &self::ml_string::Text,
    operation: &self::ml_string::Text,
    receiver: &self::ml_string::Text,
    reference: bool,
) -> bool {
    (((*expression) == (self::ml_string::Text::from_units(vec![109, 101, 116, 104, 111, 100])))
        && reference)
        && ((self::read_reference_method_result(receiver, operation))
            != (self::ml_string::Text::from_units(vec![])))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_reference_method_refusal(
    kind: &self::ml_string::Text,
    operation: &self::ml_string::Text,
) -> self::ml_string::Text {
    if (((*kind) == (self::ml_string::Text::from_units(vec![109, 97, 112])))
        || ((*kind) == (self::ml_string::Text::from_units(vec![115, 101, 116]))))
        && ((((*operation) == (self::ml_string::Text::from_units(vec![107, 101, 121, 115])))
            || ((*operation)
                == (self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101, 115]))))
            || ((*operation)
                == (self::ml_string::Text::from_units(vec![101, 110, 116, 114, 105, 101, 115]))))
    {
        self::ml_string::Text::from_units(vec![
            108, 105, 118, 101, 32, 99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 105, 116,
            101, 114, 97, 116, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 111, 114, 100, 101,
            114, 101, 100, 32, 101, 110, 116, 114, 105, 101, 115, 32, 97, 110, 100, 32, 116, 111,
            109, 98, 115, 116, 111, 110, 101, 115, 32, 114, 97, 116, 104, 101, 114, 32, 116, 104,
            97, 110, 32, 97, 32, 115, 110, 97, 112, 115, 104, 111, 116,
        ])
    } else {
        self::ml_string::Text::from_units(vec![])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_reference_callback_effect(
    expression: &self::ml_string::Text,
    operation: &self::ml_string::Text,
) -> f64 {
    if ((*expression) == (self::ml_string::Text::from_units(vec![109, 101, 116, 104, 111, 100])))
        && ((((*operation)
            == (self::ml_string::Text::from_units(vec![115, 112, 108, 105, 99, 101])))
            || ((*operation) == (self::ml_string::Text::from_units(vec![102, 105, 108, 108]))))
            || ((*operation)
                == (self::ml_string::Text::from_units(vec![
                    99, 111, 112, 121, 87, 105, 116, 104, 105, 110,
                ]))))
    {
        1f64
    } else if ((*expression)
        == (self::ml_string::Text::from_units(vec![109, 101, 116, 104, 111, 100])))
        && operation.starts_with(&self::ml_string::Text::from_units(vec![
            119, 114, 105, 116, 101, 82, 101, 99, 111, 114, 100, 70, 105, 101, 108, 100,
        ]))
    {
        1f64
    } else if ((*expression)
        == (self::ml_string::Text::from_units(vec![99, 101, 108, 108, 87, 114, 105, 116, 101])))
        || ((*expression)
            == (self::ml_string::Text::from_units(vec![
                114, 101, 102, 101, 114, 101, 110, 99, 101, 87, 114, 105, 116, 101,
            ])))
    {
        1f64
    } else if ((((*expression)
        == (self::ml_string::Text::from_units(vec![109, 101, 116, 104, 111, 100])))
        && ((*operation) != (self::ml_string::Text::from_units(vec![104, 97, 115]))))
        && ((*operation) != (self::ml_string::Text::from_units(vec![103, 101, 116]))))
        && ((((self::read_reference_method_result(
            &(self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121])),
            operation,
        )) != (self::ml_string::Text::from_units(vec![])))
            || ((self::read_reference_method_result(
                &(self::ml_string::Text::from_units(vec![109, 97, 112])),
                operation,
            )) != (self::ml_string::Text::from_units(vec![]))))
            || ((self::read_reference_method_result(
                &(self::ml_string::Text::from_units(vec![115, 101, 116])),
                operation,
            )) != (self::ml_string::Text::from_units(vec![]))))
    {
        1f64
    } else if ((*expression) == (self::ml_string::Text::from_units(vec![99, 97, 108, 108])))
        || ((*expression)
            == (self::ml_string::Text::from_units(vec![
                102, 117, 110, 99, 116, 105, 111, 110, 82, 101, 102,
            ])))
    {
        2f64
    } else if (*expression) == (self::ml_string::Text::from_units(vec![97, 112, 112, 108, 121])) {
        3f64
    } else {
        0f64
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_snapshot_callback_parameters(operation: &self::ml_string::Text, count: f64) -> bool {
    count
        < (if (*operation) == (self::ml_string::Text::from_units(vec![114, 101, 100, 117, 99, 101]))
        {
            4f64
        } else {
            3f64
        })
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_reference_callback_operand_fields(
    expression: &self::ml_string::Text,
    ordinary: &self::ml_string::Text,
) -> self::ml_string::Text {
    if (*expression) == (self::ml_string::Text::from_units(vec![108, 97, 109, 98, 100, 97])) {
        self::ml_string::Text::from_units(vec![98, 111, 100, 121])
    } else if (*expression) == (self::ml_string::Text::from_units(vec![108, 101, 116])) {
        self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101, 32, 98, 111, 100, 121])
    } else if (*expression) == (self::ml_string::Text::from_units(vec![109, 97, 116, 99, 104])) {
        self::ml_string::Text::from_units(vec![
            115, 99, 114, 117, 116, 105, 110, 101, 101, 32, 99, 97, 115, 101, 115,
        ])
    } else if (*expression)
        == (self::ml_string::Text::from_units(vec![111, 117, 116, 67, 111, 110, 115]))
    {
        self::ml_string::Text::from_units(vec![104, 101, 97, 100, 32, 116, 97, 105, 108])
    } else {
        ordinary.clone()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_reference_method_required_arguments(
    kind: &self::ml_string::Text,
    operation: &self::ml_string::Text,
) -> f64 {
    if ((*kind) == (self::ml_string::Text::from_units(vec![109, 97, 112])))
        && ((*operation) == (self::ml_string::Text::from_units(vec![115, 101, 116])))
    {
        2f64
    } else if ((((*operation) == (self::ml_string::Text::from_units(vec![97, 100, 100])))
        || ((*operation) == (self::ml_string::Text::from_units(vec![103, 101, 116]))))
        || ((*operation) == (self::ml_string::Text::from_units(vec![104, 97, 115]))))
        || ((*operation) == (self::ml_string::Text::from_units(vec![100, 101, 108, 101, 116, 101])))
    {
        1f64
    } else {
        0f64
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_reference_method_argument_role(
    kind: &self::ml_string::Text,
    operation: &self::ml_string::Text,
    position: f64,
) -> self::ml_string::Text {
    if ((*kind) == (self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121])))
        && (((*operation) == (self::ml_string::Text::from_units(vec![112, 117, 115, 104])))
            || ((*operation)
                == (self::ml_string::Text::from_units(vec![117, 110, 115, 104, 105, 102, 116]))))
    {
        self::ml_string::Text::from_units(vec![101, 108, 101, 109, 101, 110, 116])
    } else if ((((*kind) == (self::ml_string::Text::from_units(vec![115, 101, 116])))
        || ((*kind) == (self::ml_string::Text::from_units(vec![109, 97, 112]))))
        && (position == 0f64))
        && ((*operation) != (self::ml_string::Text::from_units(vec![99, 108, 101, 97, 114])))
    {
        self::ml_string::Text::from_units(vec![107, 101, 121])
    } else if (((*kind) == (self::ml_string::Text::from_units(vec![109, 97, 112])))
        && ((*operation) == (self::ml_string::Text::from_units(vec![115, 101, 116]))))
        && (position == 1f64)
    {
        self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101])
    } else {
        self::ml_string::Text::from_units(vec![])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_reference_method_key_kind(kind: &self::ml_string::Text) -> bool {
    ((((((*kind) == (self::ml_string::Text::from_units(vec![98, 111, 111, 108])))
        || ((*kind) == (self::ml_string::Text::from_units(vec![102, 108, 111, 97, 116]))))
        || ((*kind) == (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103]))))
        || ((*kind) == (self::ml_string::Text::from_units(vec![105, 110, 116]))))
        || ((*kind) == (self::ml_string::Text::from_units(vec![110, 97, 116]))))
        || ((*kind) == (self::ml_string::Text::from_units(vec![102, 105, 120, 101, 100])))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_reference_method_identity(
    kind: &self::ml_string::Text,
    operation: &self::ml_string::Text,
) -> self::ml_string::Text {
    {
        let source_ml_source2_s3 = if (*kind)
            == (self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121]))
        {
            if (*operation) == (self::ml_string::Text::from_units(vec![112, 117, 115, 104])) {
                self::MlSource1TemporaryReadReferenceMethodIdentityJoin3::MlSource1TemporaryReadReferenceMethodIdentityJoin3Return(self::ml_string::Text::from_units(vec![97, 112, 112, 101, 110, 100, 45, 97, 114, 114, 97, 121, 45, 101, 108, 101, 109, 101, 110, 116, 115]))
            } else if (*operation) == (self::ml_string::Text::from_units(vec![112, 111, 112])) {
                self::MlSource1TemporaryReadReferenceMethodIdentityJoin3::MlSource1TemporaryReadReferenceMethodIdentityJoin3Return(self::ml_string::Text::from_units(vec![114, 101, 109, 111, 118, 101, 45, 102, 105, 110, 97, 108, 45, 97, 114, 114, 97, 121, 45, 101, 108, 101, 109, 101, 110, 116]))
            } else if (*operation)
                == (self::ml_string::Text::from_units(vec![115, 104, 105, 102, 116]))
            {
                self::MlSource1TemporaryReadReferenceMethodIdentityJoin3::MlSource1TemporaryReadReferenceMethodIdentityJoin3Return(self::ml_string::Text::from_units(vec![114, 101, 109, 111, 118, 101, 45, 105, 110, 105, 116, 105, 97, 108, 45, 97, 114, 114, 97, 121, 45, 101, 108, 101, 109, 101, 110, 116]))
            } else if (*operation)
                == (self::ml_string::Text::from_units(vec![117, 110, 115, 104, 105, 102, 116]))
            {
                self::MlSource1TemporaryReadReferenceMethodIdentityJoin3::MlSource1TemporaryReadReferenceMethodIdentityJoin3Return(self::ml_string::Text::from_units(vec![112, 114, 101, 112, 101, 110, 100, 45, 97, 114, 114, 97, 121, 45, 101, 108, 101, 109, 101, 110, 116, 115]))
            } else if (*operation)
                == (self::ml_string::Text::from_units(vec![114, 101, 118, 101, 114, 115, 101]))
            {
                self::MlSource1TemporaryReadReferenceMethodIdentityJoin3::MlSource1TemporaryReadReferenceMethodIdentityJoin3Return(self::ml_string::Text::from_units(vec![114, 101, 118, 101, 114, 115, 101, 45, 97, 114, 114, 97, 121, 45, 101, 108, 101, 109, 101, 110, 116, 115]))
            } else {
                self::MlSource1TemporaryReadReferenceMethodIdentityJoin3::MlSource1TemporaryReadReferenceMethodIdentityJoin3Next
            }
        } else {
            self::MlSource1TemporaryReadReferenceMethodIdentityJoin3::MlSource1TemporaryReadReferenceMethodIdentityJoin3Next
        };
        match source_ml_source2_s3 {
            self::MlSource1TemporaryReadReferenceMethodIdentityJoin3::MlSource1TemporaryReadReferenceMethodIdentityJoin3Next => {
                {
                    let source_ml_source2_s4 = if (*kind) == (self::ml_string::Text::from_units(vec![115, 101, 116])) {
                        if (*operation) == (self::ml_string::Text::from_units(vec![97, 100, 100])) {
                            self::MlSource1TemporaryReadReferenceMethodIdentityJoin4::MlSource1TemporaryReadReferenceMethodIdentityJoin4Return(self::ml_string::Text::from_units(vec![97, 100, 100, 45, 115, 101, 116, 45, 109, 101, 109, 98, 101, 114]))
                        } else if (*operation) == (self::ml_string::Text::from_units(vec![100, 101, 108, 101, 116, 101])) {
                                self::MlSource1TemporaryReadReferenceMethodIdentityJoin4::MlSource1TemporaryReadReferenceMethodIdentityJoin4Return(self::ml_string::Text::from_units(vec![114, 101, 109, 111, 118, 101, 45, 115, 101, 116, 45, 109, 101, 109, 98, 101, 114]))
                            } else if (*operation) == (self::ml_string::Text::from_units(vec![99, 108, 101, 97, 114])) {
                                    self::MlSource1TemporaryReadReferenceMethodIdentityJoin4::MlSource1TemporaryReadReferenceMethodIdentityJoin4Return(self::ml_string::Text::from_units(vec![99, 108, 101, 97, 114, 45, 115, 101, 116, 45, 109, 101, 109, 98, 101, 114, 115]))
                                } else if (*operation) == (self::ml_string::Text::from_units(vec![104, 97, 115])) {
                                        self::MlSource1TemporaryReadReferenceMethodIdentityJoin4::MlSource1TemporaryReadReferenceMethodIdentityJoin4Return(self::ml_string::Text::from_units(vec![114, 101, 97, 100, 45, 115, 101, 116, 45, 109, 101, 109, 98, 101, 114, 115, 104, 105, 112]))
                                    } else {
                                        self::MlSource1TemporaryReadReferenceMethodIdentityJoin4::MlSource1TemporaryReadReferenceMethodIdentityJoin4Next
                                    }
                    } else {
                        self::MlSource1TemporaryReadReferenceMethodIdentityJoin4::MlSource1TemporaryReadReferenceMethodIdentityJoin4Next
                    };
                    match source_ml_source2_s4 {
                        self::MlSource1TemporaryReadReferenceMethodIdentityJoin4::MlSource1TemporaryReadReferenceMethodIdentityJoin4Next => {
                            {
                                let source_ml_source2_s5 = if (*kind) == (self::ml_string::Text::from_units(vec![109, 97, 112])) {
                                    if (*operation) == (self::ml_string::Text::from_units(vec![115, 101, 116])) {
                                        self::MlSource1TemporaryReadReferenceMethodIdentityJoin5::MlSource1TemporaryReadReferenceMethodIdentityJoin5Return(self::ml_string::Text::from_units(vec![115, 101, 116, 45, 109, 97, 112, 45, 101, 110, 116, 114, 121]))
                                    } else if (*operation) == (self::ml_string::Text::from_units(vec![100, 101, 108, 101, 116, 101])) {
                                            self::MlSource1TemporaryReadReferenceMethodIdentityJoin5::MlSource1TemporaryReadReferenceMethodIdentityJoin5Return(self::ml_string::Text::from_units(vec![114, 101, 109, 111, 118, 101, 45, 109, 97, 112, 45, 101, 110, 116, 114, 121]))
                                        } else if (*operation) == (self::ml_string::Text::from_units(vec![99, 108, 101, 97, 114])) {
                                                self::MlSource1TemporaryReadReferenceMethodIdentityJoin5::MlSource1TemporaryReadReferenceMethodIdentityJoin5Return(self::ml_string::Text::from_units(vec![99, 108, 101, 97, 114, 45, 109, 97, 112, 45, 101, 110, 116, 114, 105, 101, 115]))
                                            } else if (*operation) == (self::ml_string::Text::from_units(vec![103, 101, 116])) {
                                                    self::MlSource1TemporaryReadReferenceMethodIdentityJoin5::MlSource1TemporaryReadReferenceMethodIdentityJoin5Return(self::ml_string::Text::from_units(vec![114, 101, 97, 100, 45, 109, 97, 112, 45, 101, 110, 116, 114, 121]))
                                                } else if (*operation) == (self::ml_string::Text::from_units(vec![104, 97, 115])) {
                                                        self::MlSource1TemporaryReadReferenceMethodIdentityJoin5::MlSource1TemporaryReadReferenceMethodIdentityJoin5Return(self::ml_string::Text::from_units(vec![114, 101, 97, 100, 45, 109, 97, 112, 45, 109, 101, 109, 98, 101, 114, 115, 104, 105, 112]))
                                                    } else {
                                                        self::MlSource1TemporaryReadReferenceMethodIdentityJoin5::MlSource1TemporaryReadReferenceMethodIdentityJoin5Next
                                                    }
                                } else {
                                    self::MlSource1TemporaryReadReferenceMethodIdentityJoin5::MlSource1TemporaryReadReferenceMethodIdentityJoin5Next
                                };
                                match source_ml_source2_s5 {
                                    self::MlSource1TemporaryReadReferenceMethodIdentityJoin5::MlSource1TemporaryReadReferenceMethodIdentityJoin5Next => {
                                        self::ml_string::Text::from_units(vec![])
                                    }
                                    self::MlSource1TemporaryReadReferenceMethodIdentityJoin5::MlSource1TemporaryReadReferenceMethodIdentityJoin5Return(source_ml_source1_temporary_result5) => {
                                        source_ml_source1_temporary_result5
                                    }
                                }
                            }
                        }
                        self::MlSource1TemporaryReadReferenceMethodIdentityJoin4::MlSource1TemporaryReadReferenceMethodIdentityJoin4Return(source_ml_source1_temporary_result5_2) => {
                            source_ml_source1_temporary_result5_2
                        }
                    }
                }
            }
            self::MlSource1TemporaryReadReferenceMethodIdentityJoin3::MlSource1TemporaryReadReferenceMethodIdentityJoin3Return(source_ml_source1_temporary_result5_3) => {
                source_ml_source1_temporary_result5_3
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_reference_method_statement() -> self::ml_string::Text {
    self::ml_string::Text::from_units(vec![
        83, 104, 97, 114, 101, 100, 32, 99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 109,
        101, 116, 104, 111, 100, 115, 32, 101, 118, 97, 108, 117, 97, 116, 101, 32, 116, 104, 101,
        105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 99, 101, 105, 118, 101, 114, 32, 97,
        110, 100, 32, 101, 118, 101, 114, 121, 32, 97, 114, 103, 117, 109, 101, 110, 116, 32, 111,
        110, 99, 101, 32, 105, 110, 32, 115, 111, 117, 114, 99, 101, 32, 111, 114, 100, 101, 114,
        59, 32, 101, 97, 99, 104, 32, 100, 101, 99, 108, 97, 114, 101, 100, 32, 109, 117, 116, 97,
        116, 105, 111, 110, 32, 99, 104, 97, 110, 103, 101, 115, 32, 116, 104, 101, 32, 115, 97,
        109, 101, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 44, 32, 112, 114, 101, 115,
        101, 114, 118, 105, 110, 103, 32, 105, 110, 115, 101, 114, 116, 105, 111, 110, 32, 111,
        114, 100, 101, 114, 44, 32, 83, 97, 109, 101, 86, 97, 108, 117, 101, 90, 101, 114, 111, 32,
        112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 107, 101, 121, 115, 32, 97, 110, 100, 32,
        116, 104, 101, 32, 100, 101, 99, 108, 97, 114, 101, 100, 32, 74, 97, 118, 97, 83, 99, 114,
        105, 112, 116, 32, 114, 101, 115, 117, 108, 116, 46,
    ])
}
