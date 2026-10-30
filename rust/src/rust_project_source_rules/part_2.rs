// Generated from complete js/src/rust-project-source-rules.js. Do not edit by hand.
use super::MlTemporaryAcceptRustProjectSourcePathLoop6Result;
use super::MlTemporaryGroupEndLoop1Result;
use super::ml;
use super::ml_array;
use super::source_ml_temporary_accept_rust_project_source_path_loop6;
use super::source_ml_temporary_group_end_loop1;
use super::source_ml_temporary_read_rust_project_module_path_attributes_loop7;
use super::source_ml_temporary_read_rust_project_module_path_attributes_loop11;
use super::source_ml_temporary_read_rust_project_module_plan_loop2;
use super::source_ml_temporary_resolve_rust_project_literal_path_loop13;

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
pub fn text_column(
    columns: &[Vec<self::ml_string::Text>],
    index: f64,
) -> Vec<self::ml_string::Text> {
    if (index < 0f64) || (index >= (columns.len() as f64)) {
        (self::ml_source_exception::SourceException {
            class_name: String::from("RangeError"),
            message_units: (self::ml_string::Text::from_units(vec![
                109, 105, 115, 115, 105, 110, 103, 32, 82, 117, 115, 116, 32, 112, 114, 111, 106,
                101, 99, 116, 32, 116, 101, 120, 116, 32, 99, 111, 108, 117, 109, 110,
            ]))
            .0,
            message_present: true,
        })
        .raise()
    } else {
        (self::ml_array::optional_at(columns, self::ml_array::number_index(index)))
            .unwrap_or_else(|| Vec::<self::ml_string::Text>::new())
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn number_column(columns: &[Vec<f64>], index: f64) -> Vec<f64> {
    if (index < 0f64) || (index >= (columns.len() as f64)) {
        (self::ml_source_exception::SourceException {
            class_name: String::from("RangeError"),
            message_units: (self::ml_string::Text::from_units(vec![
                109, 105, 115, 115, 105, 110, 103, 32, 82, 117, 115, 116, 32, 112, 114, 111, 106,
                101, 99, 116, 32, 78, 117, 109, 98, 101, 114, 32, 99, 111, 108, 117, 109, 110,
            ]))
            .0,
            message_present: true,
        })
        .raise()
    } else {
        (self::ml_array::optional_at(columns, self::ml_array::number_index(index)))
            .unwrap_or_else(|| Vec::<f64>::new())
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn group_end(
    kinds: &[self::ml_string::Text],
    values: &[self::ml_string::Text],
    index: f64,
) -> f64 {
    {
        let open = self::read_required_text_fact(values, index);
        {
            let stack = vec![if open == (self::ml_string::Text::from_units(vec![123])) {
                self::ml_string::Text::from_units(vec![125])
            } else if open == (self::ml_string::Text::from_units(vec![91])) {
                self::ml_string::Text::from_units(vec![93])
            } else {
                self::ml_string::Text::from_units(vec![41])
            }];
            {
                let position = index + 1f64;
                {
                    let source_ml_source1_s1 =
                        self::source_ml_temporary_group_end_loop1(kinds, values, stack, position);
                    match source_ml_source1_s1 {
                        self::MlTemporaryGroupEndLoop1Result::MlTemporaryGroupEndLoop1Done => {
                            (self::ml_source_exception::SourceException {
                                class_name: String::from("TypeError"),
                                message_units: (self::ml_string::Text::from_units(vec![
                                    117, 110, 99, 108, 111, 115, 101, 100, 32, 82, 117, 115, 116,
                                    32, 112, 114, 111, 106, 101, 99, 116, 32, 115, 111, 117, 114,
                                    99, 101, 32, 103, 114, 111, 117, 112,
                                ]))
                                .0,
                                message_present: true,
                            })
                            .raise()
                        }
                        self::MlTemporaryGroupEndLoop1Result::MlTemporaryGroupEndLoop1Return(
                            source_ml_temporary_result1,
                        ) => source_ml_temporary_result1,
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_project_module_plan(
    kinds: &[self::ml_string::Text],
    values: &[self::ml_string::Text],
) -> Vec<f64> {
    if (kinds.len() as f64) == (values.len() as f64) {
        {
            let plan = Vec::<f64>::new();
            {
                let parents = Vec::<f64>::new();
                {
                    let closes = Vec::<f64>::new();
                    {
                        let position = 0f64;
                        {
                            let parent = -1f64;
                            {
                                self::source_ml_temporary_read_rust_project_module_plan_loop2(
                                    kinds, values, plan, parents, closes, position, parent,
                                )
                            }
                        }
                    }
                }
            }
        }
    } else {
        (self::ml_source_exception::SourceException {
            class_name: String::from("TypeError"),
            message_units: (self::ml_string::Text::from_units(vec![
                117, 110, 97, 108, 105, 103, 110, 101, 100, 32, 82, 117, 115, 116, 32, 112, 114,
                111, 106, 101, 99, 116, 32, 116, 111, 107, 101, 110, 32, 99, 111, 108, 117, 109,
                110, 115,
            ]))
            .0,
            message_present: true,
        })
        .raise()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_project_source_path(path: &self::ml_string::Text) -> bool {
    if (((!(!path.0.is_empty()) || path.starts_with(&self::ml_string::Text::from_units(vec![47])))
        || path.contains(&self::ml_string::Text::from_units(vec![92])))
        || path.contains(&self::ml_string::Text::from_units(vec![0])))
        || path.contains(&self::ml_string::Text::from_units(vec![58]))
    {
        false
    } else {
        {
            let source_ml_temporary_values5 = {
                let source_ml_source1_string_unit_operand2 = path.clone();
                {
                    let source_ml_source1_string_unit_operand3 =
                        self::ml_string::Text::from_units(vec![47]);
                    {
                        let source_ml_source1_string_split_limit4 = 4_294_967_295f64;
                        {
                            let source_ml_source1_string_split_separator5 =
                                source_ml_source1_string_unit_operand3;
                            (|ml_text: &self::ml_string::Text,
                              ml_separator: Option<&self::ml_string::Text>,
                              ml_limit: f64| {
                                self::ml_utf16_text::split_utf16_text(
                                    &(ml_text
                                        .0
                                        .iter()
                                        .map(|ml_unit| f64::from(*ml_unit))
                                        .collect::<Vec<f64>>()),
                                    &(ml_separator
                                        .map(|ml_value| {
                                            ml_value
                                                .0
                                                .iter()
                                                .map(|ml_unit| f64::from(*ml_unit))
                                                .collect::<Vec<f64>>()
                                        })
                                        .unwrap_or_default()),
                                    ml_limit,
                                    ml_separator.is_none(),
                                )
                                .into_iter()
                                .map(|ml_units| {
                                    self::ml_string::Text::from_units(
                                        ml_units
                                            .into_iter()
                                            .map(|ml_unit| ml_unit as u16)
                                            .collect(),
                                    )
                                })
                                .collect::<Vec<_>>()
                            })(
                                &(source_ml_source1_string_unit_operand2),
                                Some(&source_ml_source1_string_split_separator5),
                                source_ml_source1_string_split_limit4,
                            )
                        }
                    }
                }
            };
            {
                let source_ml_temporary_index5 = self::ml::Big::from_i128(0);
                {
                    let source_ml_source1_s6 =
                        self::source_ml_temporary_accept_rust_project_source_path_loop6(
                            &(source_ml_temporary_values5),
                            source_ml_temporary_index5,
                        );
                    match source_ml_source1_s6 {
                        self::MlTemporaryAcceptRustProjectSourcePathLoop6Result::MlTemporaryAcceptRustProjectSourcePathLoop6Done => {
                            true
                        }
                        self::MlTemporaryAcceptRustProjectSourcePathLoop6Result::MlTemporaryAcceptRustProjectSourcePathLoop6Return(source_ml_temporary_result6) => {
                            source_ml_temporary_result6
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_project_module_path_attributes(
    kinds: &[self::ml_string::Text],
    values: &[self::ml_string::Text],
    plan: &[f64],
) -> Vec<f64> {
    {
        let selected = Vec::<f64>::new();
        {
            let position = 0f64;
            {
                let selected_2 =
                    self::source_ml_temporary_read_rust_project_module_path_attributes_loop7(
                        kinds, values, plan, selected, position,
                    );
                {
                    let attributes = Vec::<f64>::new();
                    {
                        let selected_at = 0f64;
                        {
                            let record = 0f64;
                            {
                                self::source_ml_temporary_read_rust_project_module_path_attributes_loop11(plan, &(selected_2), attributes, selected_at, record)
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
pub fn resolve_rust_project_literal_path(
    directory: &self::ml_string::Text,
    path: &self::ml_string::Text,
) -> self::ml_string::Text {
    if ((path.starts_with(&self::ml_string::Text::from_units(vec![47]))
        || path.contains(&self::ml_string::Text::from_units(vec![92])))
        || path.contains(&self::ml_string::Text::from_units(vec![0])))
        || path.contains(&self::ml_string::Text::from_units(vec![58]))
    {
        (self::ml_source_exception::SourceException {
            class_name: String::from("TypeError"),
            message_units: (self::ml_string::Text::from_units(vec![
                110, 111, 110, 112, 111, 114, 116, 97, 98, 108, 101, 32, 82, 117, 115, 116, 32,
                112, 114, 111, 106, 101, 99, 116, 32, 108, 105, 116, 101, 114, 97, 108, 32, 112,
                97, 116, 104,
            ]))
            .0,
            message_present: true,
        })
        .raise()
    } else {
        {
            let result = if directory.0.is_empty() {
                Vec::<self::ml_string::Text>::new()
            } else {
                {
                    let source_ml_source1_string_unit_operand7 = directory.clone();
                    {
                        let source_ml_source1_string_unit_operand8 =
                            self::ml_string::Text::from_units(vec![47]);
                        {
                            let source_ml_source1_string_split_limit9 = 4_294_967_295f64;
                            {
                                let source_ml_source1_string_split_separator10 =
                                    source_ml_source1_string_unit_operand8;
                                (|ml_text: &self::ml_string::Text,
                                  ml_separator: Option<&self::ml_string::Text>,
                                  ml_limit: f64| {
                                    self::ml_utf16_text::split_utf16_text(
                                        &(ml_text
                                            .0
                                            .iter()
                                            .map(|ml_unit| f64::from(*ml_unit))
                                            .collect::<Vec<f64>>()),
                                        &(ml_separator
                                            .map(|ml_value| {
                                                ml_value
                                                    .0
                                                    .iter()
                                                    .map(|ml_unit| f64::from(*ml_unit))
                                                    .collect::<Vec<f64>>()
                                            })
                                            .unwrap_or_default()),
                                        ml_limit,
                                        ml_separator.is_none(),
                                    )
                                    .into_iter()
                                    .map(|ml_units| {
                                        self::ml_string::Text::from_units(
                                            ml_units
                                                .into_iter()
                                                .map(|ml_unit| ml_unit as u16)
                                                .collect(),
                                        )
                                    })
                                    .collect::<Vec<_>>()
                                })(
                                    &(source_ml_source1_string_unit_operand7),
                                    Some(&source_ml_source1_string_split_separator10),
                                    source_ml_source1_string_split_limit9,
                                )
                            }
                        }
                    }
                }
            };
            {
                let source_ml_temporary_values12 = {
                    let source_ml_source1_string_unit_operand11 = path.clone();
                    {
                        let source_ml_source1_string_unit_operand12 =
                            self::ml_string::Text::from_units(vec![47]);
                        {
                            let source_ml_source1_string_split_limit13 = 4_294_967_295f64;
                            {
                                let source_ml_source1_string_split_separator14 =
                                    source_ml_source1_string_unit_operand12;
                                (|ml_text: &self::ml_string::Text,
                                  ml_separator: Option<&self::ml_string::Text>,
                                  ml_limit: f64| {
                                    self::ml_utf16_text::split_utf16_text(
                                        &(ml_text
                                            .0
                                            .iter()
                                            .map(|ml_unit| f64::from(*ml_unit))
                                            .collect::<Vec<f64>>()),
                                        &(ml_separator
                                            .map(|ml_value| {
                                                ml_value
                                                    .0
                                                    .iter()
                                                    .map(|ml_unit| f64::from(*ml_unit))
                                                    .collect::<Vec<f64>>()
                                            })
                                            .unwrap_or_default()),
                                        ml_limit,
                                        ml_separator.is_none(),
                                    )
                                    .into_iter()
                                    .map(|ml_units| {
                                        self::ml_string::Text::from_units(
                                            ml_units
                                                .into_iter()
                                                .map(|ml_unit| ml_unit as u16)
                                                .collect(),
                                        )
                                    })
                                    .collect::<Vec<_>>()
                                })(
                                    &(source_ml_source1_string_unit_operand11),
                                    Some(&source_ml_source1_string_split_separator14),
                                    source_ml_source1_string_split_limit13,
                                )
                            }
                        }
                    }
                };
                {
                    let source_ml_temporary_index12 = self::ml::Big::from_i128(0);
                    {
                        let result_2 =
                            self::source_ml_temporary_resolve_rust_project_literal_path_loop13(
                                result,
                                &(source_ml_temporary_values12),
                                source_ml_temporary_index12,
                            );
                        self::ml_string::join(
                            &result_2,
                            &self::ml_string::Text::from_units(vec![47]),
                        )
                    }
                }
            }
        }
    }
}
