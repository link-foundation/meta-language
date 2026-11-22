// Generated from complete js/src/translation/rust-generic-type-alias-rules.js. Do not edit by hand.
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
pub fn accept_rust_source_array_owner(owner: &self::ml_string::Text, shadowed: bool) -> bool {
    !shadowed
        && (((*owner) == (self::ml_string::Text::from_units(vec![86, 101, 99])))
            || ((*owner)
                == (self::ml_string::Text::from_units(vec![
                    115, 116, 100, 58, 58, 118, 101, 99, 58, 58, 86, 101, 99,
                ]))))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_source_array_element(kind: &self::ml_string::Text, indirected: bool) -> bool {
    !indirected
        && ((((((*kind) == (self::ml_string::Text::from_units(vec![98, 111, 111, 108])))
            || ((*kind) == (self::ml_string::Text::from_units(vec![102, 108, 111, 97, 116]))))
            || ((*kind) == (self::ml_string::Text::from_units(vec![102, 105, 120, 101, 100]))))
            || ((*kind) == (self::ml_string::Text::from_units(vec![117, 110, 105, 116]))))
            || ((*kind) == (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103]))))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_source_array_coercion(
    actual: &self::ml_string::Text,
    expected: &self::ml_string::Text,
    borrowed: bool,
) -> bool {
    ((*actual) == (*expected))
        || ((borrowed
            && ((*actual)
                == (self::ml_string::Text::from_units(vec![
                    111, 119, 110, 101, 100, 32, 118, 101, 99, 116, 111, 114,
                ]))))
            && ((*expected)
                == (self::ml_string::Text::from_units(vec![
                    105, 109, 109, 117, 116, 97, 98, 108, 101, 32, 115, 108, 105, 99, 101,
                ]))))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_source_array_result_model(model: &self::ml_string::Text) -> bool {
    ((*model) == (self::ml_string::Text::from_units(vec![])))
        || ((*model)
            == (self::ml_string::Text::from_units(vec![
                105, 109, 109, 117, 116, 97, 98, 108, 101, 32, 102, 114, 101, 115, 104, 32, 115,
                99, 97, 108, 97, 114, 32, 118, 101, 99, 116, 111, 114, 32, 114, 101, 115, 117, 108,
                116, 115, 32, 49,
            ])))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_source_array_result(
    model: &self::ml_string::Text,
    kind: &self::ml_string::Text,
    owned: bool,
    fresh: bool,
) -> bool {
    ((((*model)
        == (self::ml_string::Text::from_units(vec![
            105, 109, 109, 117, 116, 97, 98, 108, 101, 32, 102, 114, 101, 115, 104, 32, 115, 99,
            97, 108, 97, 114, 32, 118, 101, 99, 116, 111, 114, 32, 114, 101, 115, 117, 108, 116,
            115, 32, 49,
        ])))
        && owned)
        && fresh)
        && (((((*kind) == (self::ml_string::Text::from_units(vec![98, 111, 111, 108])))
            || ((*kind) == (self::ml_string::Text::from_units(vec![102, 108, 111, 97, 116]))))
            || ((*kind) == (self::ml_string::Text::from_units(vec![102, 105, 120, 101, 100]))))
            || ((*kind) == (self::ml_string::Text::from_units(vec![117, 110, 105, 116]))))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_fresh_array_result_tail(
    kind: &self::ml_string::Text,
    constructor: bool,
    children: bool,
) -> bool {
    (((*kind) == (self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121]))) && constructor)
        || ((((*kind) == (self::ml_string::Text::from_units(vec![108, 101, 116])))
            || ((*kind) == (self::ml_string::Text::from_units(vec![105, 102]))))
            && children)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_scalar_predicate(
    kind: &self::ml_string::Text,
    parameters: f64,
    layers: f64,
    parameter: bool,
    result: bool,
) -> bool {
    (((((*kind)
        == (self::ml_string::Text::from_units(vec![102, 117, 110, 99, 116, 105, 111, 110])))
        && (parameters == 1f64))
        && (layers == 0f64))
        && parameter)
        && result
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_scalar_fallback(
    kind: &self::ml_string::Text,
    parameters: f64,
    layers: f64,
    result: bool,
) -> bool {
    ((((*kind)
        == (self::ml_string::Text::from_units(vec![102, 117, 110, 99, 116, 105, 111, 110])))
        && (parameters == 0f64))
        && (layers == 0f64))
        && result
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_source_version_default() -> self::ml_string::Text {
    self::ml_string::Text::from_units(vec![49, 46, 57, 57, 46, 48])
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_source_version_error(
    major: f64,
    minor: f64,
    patch: f64,
    canonical: bool,
) -> self::ml_string::Text {
    if ((((((canonical && (major == 1f64)) && self::ml_math::is_integer(minor))
        && (minor >= 90f64))
        && (minor < 100f64))
        && self::ml_math::is_integer(patch))
        && (patch >= 0f64))
        && (patch <= 4_294_967_295f64)
    {
        self::ml_string::Text::from_units(vec![])
    } else {
        self::ml_string::Text::from_units(vec![
            115, 101, 108, 101, 99, 116, 32, 97, 32, 99, 97, 110, 111, 110, 105, 99, 97, 108, 32,
            115, 116, 97, 98, 108, 101, 32, 82, 117, 115, 116, 32, 115, 111, 117, 114, 99, 101, 32,
            118, 101, 114, 115, 105, 111, 110, 32, 102, 114, 111, 109, 32, 49, 46, 57, 48, 46, 48,
            32, 116, 104, 114, 111, 117, 103, 104, 32, 49, 46, 57, 57, 46, 120,
        ])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_integer_conversion_error_debug_model(minor: f64) -> self::ml_string::Text {
    if (self::ml_math::is_integer(minor) && (minor >= 90f64)) && (minor < 97f64) {
        self::ml_string::Text::from_units(vec![
            117, 110, 105, 116, 32, 112, 97, 121, 108, 111, 97, 100,
        ])
    } else if (self::ml_math::is_integer(minor) && (minor >= 97f64)) && (minor < 100f64) {
        self::ml_string::Text::from_units(vec![
            111, 118, 101, 114, 102, 108, 111, 119, 32, 107, 105, 110, 100,
        ])
    } else {
        self::ml_string::Text::from_units(vec![])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_source_version_text_limit() -> f64 {
    15f64
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_primitive_owner_path(
    segments: &[self::ml_string::Text],
    ordinary: bool,
    shadowed: bool,
    absolute: bool,
) -> self::ml_string::Text {
    if !ordinary || shadowed {
        self::ml_string::Text::from_units(vec![])
    } else {
        {
            let owner = self::ml_string::Text::from_units(vec![]);
            {
                let owner_4 = if ((segments.len() as f64) == 2f64) && !absolute {
                    {
                        (self::ml_array::optional_at(segments, self::ml_array::number_index(0f64)))
                            .unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))
                    }
                } else {
                    if (((segments.len() as f64) == 4f64)
                        && ({
                            let source_ml_source2_comparison_operand1 = self::ml_array::optional_at(
                                segments,
                                self::ml_array::number_index(0f64),
                            );
                            {
                                let source_ml_source2_comparison_operand2 =
                                    self::ml_string::Text::from_units(vec![115, 116, 100]);
                                match (
                                    source_ml_source2_comparison_operand1,
                                    source_ml_source2_comparison_operand2,
                                ) {
                                    (Some(ml_left), ml_right) => ml_left == ml_right,
                                    _ => false,
                                }
                            }
                        } || {
                            let source_ml_source2_comparison_operand3 = self::ml_array::optional_at(
                                segments,
                                self::ml_array::number_index(0f64),
                            );
                            {
                                let source_ml_source2_comparison_operand4 =
                                    self::ml_string::Text::from_units(vec![99, 111, 114, 101]);
                                match (
                                    source_ml_source2_comparison_operand3,
                                    source_ml_source2_comparison_operand4,
                                ) {
                                    (Some(ml_left), ml_right) => ml_left == ml_right,
                                    _ => false,
                                }
                            }
                        }))
                        && {
                            let source_ml_source2_comparison_operand5 = self::ml_array::optional_at(
                                segments,
                                self::ml_array::number_index(1f64),
                            );
                            {
                                let source_ml_source2_comparison_operand6 =
                                    self::ml_string::Text::from_units(vec![
                                        112, 114, 105, 109, 105, 116, 105, 118, 101,
                                    ]);
                                match (
                                    source_ml_source2_comparison_operand5,
                                    source_ml_source2_comparison_operand6,
                                ) {
                                    (Some(ml_left), ml_right) => ml_left == ml_right,
                                    _ => false,
                                }
                            }
                        }
                    {
                        {
                            (self::ml_array::optional_at(
                                segments,
                                self::ml_array::number_index(2f64),
                            ))
                            .unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))
                        }
                    } else {
                        owner
                    }
                };
                if self::ml_array::includes(
                    &vec![
                        self::ml_string::Text::from_units(vec![98, 111, 111, 108]),
                        self::ml_string::Text::from_units(vec![105, 56]),
                        self::ml_string::Text::from_units(vec![117, 56]),
                        self::ml_string::Text::from_units(vec![105, 49, 54]),
                        self::ml_string::Text::from_units(vec![117, 49, 54]),
                        self::ml_string::Text::from_units(vec![105, 51, 50]),
                        self::ml_string::Text::from_units(vec![117, 51, 50]),
                        self::ml_string::Text::from_units(vec![105, 54, 52]),
                        self::ml_string::Text::from_units(vec![117, 54, 52]),
                        self::ml_string::Text::from_units(vec![105, 49, 50, 56]),
                        self::ml_string::Text::from_units(vec![117, 49, 50, 56]),
                        self::ml_string::Text::from_units(vec![105, 115, 105, 122, 101]),
                        self::ml_string::Text::from_units(vec![117, 115, 105, 122, 101]),
                    ],
                    &owner_4.clone(),
                    0f64,
                    false,
                    |a, b| a == b,
                ) {
                    owner_4.clone()
                } else {
                    self::ml_string::Text::from_units(vec![])
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_primitive_integer_constant_owner(
    segments: &[self::ml_string::Text],
    ordinary: bool,
    shadowed: bool,
    absolute: bool,
) -> self::ml_string::Text {
    {
        let owner = self::read_rust_primitive_owner_path(segments, ordinary, shadowed, absolute);
        if owner == (self::ml_string::Text::from_units(vec![98, 111, 111, 108])) {
            self::ml_string::Text::from_units(vec![])
        } else {
            owner
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_primitive_integer_constant_term(
    name: &self::ml_string::Text,
) -> self::ml_string::Text {
    if (*name) == (self::ml_string::Text::from_units(vec![77, 73, 78])) {
        self::ml_string::Text::from_units(vec![109, 105, 110, 105, 109, 117, 109])
    } else if (*name) == (self::ml_string::Text::from_units(vec![77, 65, 88])) {
        self::ml_string::Text::from_units(vec![109, 97, 120, 105, 109, 117, 109])
    } else if (*name) == (self::ml_string::Text::from_units(vec![66, 73, 84, 83])) {
        self::ml_string::Text::from_units(vec![119, 105, 100, 116, 104])
    } else {
        self::ml_string::Text::from_units(vec![])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_primitive_integer_constant_result_owner(
    owner: &self::ml_string::Text,
    term: &self::ml_string::Text,
) -> self::ml_string::Text {
    if (*term) == (self::ml_string::Text::from_units(vec![119, 105, 100, 116, 104])) {
        self::ml_string::Text::from_units(vec![117, 51, 50])
    } else if ((*term)
        == (self::ml_string::Text::from_units(vec![109, 105, 110, 105, 109, 117, 109])))
        || ((*term) == (self::ml_string::Text::from_units(vec![109, 97, 120, 105, 109, 117, 109])))
    {
        owner.clone()
    } else {
        self::ml_string::Text::from_units(vec![])
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
