// Generated from complete js/src/unsigned-source-command-rules.js. Do not edit by hand.
use super::MlTemporaryPlanUnsignedSourceCommandLoop3Result;
use super::source_ml_temporary_accept_unsigned_source_command_identity_loop1;
use super::source_ml_temporary_plan_unsigned_source_command_loop3;

/// Reads of JavaScript arrays, which the portable core never mutates.
pub mod ml_array {
    #[must_use]
    pub fn optional_at<T: Clone>(values: &[T], index: Option<usize>) -> Option<T> {
        index.and_then(|index| values.get(index)).cloned()
    }
    /// The element at an index; a read outside the array, undefined in
    /// JavaScript, aborts.
    #[must_use]
    pub fn at<T: Clone>(values: &[T], index: Option<usize>) -> T {
        index
            .and_then(|index| values.get(index))
            .map_or_else(|| panic!("array index out of range"), Clone::clone)
    }

    /// The index a Number names: a non-negative integer, -0 included.
    #[must_use]
    pub fn number_index(index: f64) -> Option<usize> {
        if index >= 0.0 && index.fract() == 0.0 && index < 9_007_199_254_740_992.0 {
            Some(index as usize)
        } else {
            None
        }
    }

    #[must_use]
    pub fn append<T>(mut left: Vec<T>, right: Vec<T>) -> Vec<T> {
        left.extend(right);
        left
    }
    const fn integer(value: f64) -> f64 {
        if value.is_nan() { 0.0 } else { value.trunc() }
    }
    fn relative(value: f64, length: usize) -> usize {
        let value = integer(value);
        if value < 0.0 {
            (length as f64 + value).max(0.0) as usize
        } else {
            value.min(length as f64) as usize
        }
    }
    #[must_use]
    pub fn slice<T: Clone>(values: &[T], start: f64, end: f64) -> Vec<T> {
        let start = relative(start, values.len());
        let end = relative(end, values.len());
        values[start..end.max(start)].to_vec()
    }
    #[must_use]
    pub fn index_of<T, F: Fn(&T, &T) -> bool>(
        values: &[T],
        search: &T,
        from: f64,
        reverse: bool,
        equal: F,
    ) -> f64 {
        if values.is_empty() {
            return -1.0;
        }
        let from = integer(from);
        if reverse {
            if from < -(values.len() as f64) {
                return -1.0;
            }
            let start = if from < 0.0 {
                (values.len() as f64 + from) as usize
            } else {
                from.min((values.len() - 1) as f64) as usize
            };
            (0..=start)
                .rev()
                .find(|&index| equal(&values[index], search))
                .map_or(-1.0, |index| index as f64)
        } else {
            let start = if from < 0.0 {
                (values.len() as f64 + from).max(0.0) as usize
            } else {
                from.min(values.len() as f64) as usize
            };
            (start..values.len())
                .find(|&index| equal(&values[index], search))
                .map_or(-1.0, |index| index as f64)
        }
    }
    #[must_use]
    pub fn includes<T, F: Fn(&T, &T) -> bool>(
        values: &[T],
        search: &T,
        from: f64,
        reverse: bool,
        equal: F,
    ) -> bool {
        index_of(values, search, from, reverse, equal) >= 0.0
    }
    #[must_use]
    pub fn set<T, F: Fn(&T, &T) -> bool>(values: Vec<T>, equal: F) -> Vec<T> {
        let mut out = Vec::new();
        for value in values {
            if !out.iter().any(|previous| equal(previous, &value)) {
                out.push(value);
            }
        }
        out
    }
    #[must_use]
    pub fn map<K, V, F: Fn(&K, &K) -> bool>(values: Vec<(K, V)>, equal: F) -> Vec<(K, V)> {
        let mut out: Vec<(K, V)> = Vec::new();
        for (key, value) in values {
            if let Some(index) = out.iter().position(|(previous, _)| equal(previous, &key)) {
                out[index].1 = value;
            } else {
                out.push((key, value));
            }
        }
        out
    }
}

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

#[path = "ml_unicode_case.rs"]
pub mod ml_unicode_case;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_unsigned_source_command_usage() -> self::ml_string::Text {
    self::ml_string::Text::from_units(vec![
        117, 115, 97, 103, 101, 58, 32, 109, 101, 116, 97, 45, 108, 97, 110, 103, 117, 97, 103,
        101, 32, 101, 120, 97, 99, 116, 45, 115, 111, 117, 114, 99, 101, 32, 60, 102, 105, 108,
        101, 62, 32, 45, 45, 108, 97, 110, 103, 117, 97, 103, 101, 32, 60, 108, 97, 110, 103, 117,
        97, 103, 101, 62, 32, 91, 45, 45, 102, 105, 114, 115, 116, 45, 105, 100, 32, 60, 100, 101,
        99, 105, 109, 97, 108, 62, 93, 32, 91, 45, 45, 98, 105, 110, 97, 114, 121, 93, 32, 91, 45,
        45, 116, 121, 112, 101, 100, 32, 124, 32, 45, 45, 114, 117, 115, 116, 45, 115, 111, 117,
        114, 99, 101, 45, 111, 119, 110, 101, 114, 115, 93, 32, 91, 45, 45, 115, 111, 117, 114, 99,
        101, 45, 118, 101, 114, 115, 105, 111, 110, 32, 60, 82, 117, 115, 116, 32, 118, 101, 114,
        115, 105, 111, 110, 62, 93, 32, 91, 45, 45, 116, 111, 32, 60, 74, 97, 118, 97, 83, 99, 114,
        105, 112, 116, 124, 82, 117, 115, 116, 124, 76, 101, 97, 110, 124, 82, 111, 99, 113, 62,
        93, 10, 69, 120, 112, 111, 114, 116, 115, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32,
        99, 111, 110, 99, 114, 101, 116, 101, 32, 115, 111, 117, 114, 99, 101, 32, 108, 105, 110,
        107, 115, 32, 98, 121, 32, 100, 101, 102, 97, 117, 108, 116, 59, 32, 116, 121, 112, 101,
        100, 32, 115, 111, 117, 114, 99, 101, 32, 105, 115, 32, 99, 104, 101, 99, 107, 101, 100,
        32, 98, 101, 102, 111, 114, 101, 32, 103, 114, 97, 112, 104, 32, 101, 120, 112, 111, 114,
        116, 32, 111, 114, 32, 99, 111, 100, 101, 32, 101, 109, 105, 115, 115, 105, 111, 110, 46,
        10,
    ])
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_unsigned_source_command_identity(value: &self::ml_string::Text) -> bool {
    if ((value.length()) == 0f64) || ((value.length()) > 20f64) {
        false
    } else {
        {
            let first = 0f64;
            {
                let at = 0f64;
                {
                    let source_ml_source1_s1 =
                        self::source_ml_temporary_accept_unsigned_source_command_identity_loop1(
                            value, first, at,
                        );
                    match source_ml_source1_s1 {
                        self::MlTemporaryAcceptUnsignedSourceCommandIdentityLoop1Result::MlTemporaryAcceptUnsignedSourceCommandIdentityLoop1Done(first_2) => {
                            if first_2 == (value.length()) {
                                false
                            } else {
                                {
                                    let digits = {
                                        let source_ml_source1_string_range_operand2 = value.clone();
                                        {
                                            let source_ml_source1_string_range_operand3 = first_2;
                                            {
                                                let source_ml_source1_string_range_position4 = source_ml_source1_string_range_operand3;
                                                {
                                                    let source_ml_source1_string_range_position5 = f64::INFINITY;
                                                    source_ml_source1_string_range_operand2.slice(source_ml_source1_string_range_position4, source_ml_source1_string_range_position5)
                                                }
                                            }
                                        }
                                    };
                                    if (digits.length()) < 20f64 {
                                        true
                                    } else {
                                        {
                                            let maximum = self::ml_string::Text::from_units(vec![49, 56, 52, 52, 54, 55, 52, 52, 48, 55, 51, 55, 48, 57, 53, 53, 49, 54, 49, 53]);
                                            {
                                                let at_2 = 0f64;
                                                {
                                                    let source_ml_source1_s6 = self::source_ml_temporary_accept_unsigned_source_command_identity_loop2(&(digits), &(maximum), at_2);
                                                    match source_ml_source1_s6 {
                                                        self::MlTemporaryAcceptUnsignedSourceCommandIdentityLoop2Result::MlTemporaryAcceptUnsignedSourceCommandIdentityLoop2Done => {
                                                            true
                                                        }
                                                        self::MlTemporaryAcceptUnsignedSourceCommandIdentityLoop2Result::MlTemporaryAcceptUnsignedSourceCommandIdentityLoop2Return(source_ml_temporary_result2) => {
                                                            source_ml_temporary_result2
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        self::MlTemporaryAcceptUnsignedSourceCommandIdentityLoop1Result::MlTemporaryAcceptUnsignedSourceCommandIdentityLoop1Return(source_ml_temporary_result2_2) => {
                            source_ml_temporary_result2_2
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_unsigned_source_owner_language(language: &self::ml_string::Text) -> bool {
    {
        let source = self::ml_string::Text::from_units(
            self::ml_unicode_case::map_unicode_case_utf16_text(
                &((language.clone())
                    .0
                    .iter()
                    .map(|ml_unit| f64::from(*ml_unit))
                    .collect::<Vec<f64>>()),
                false,
            )
            .into_iter()
            .map(|ml_unit| ml_unit as u16)
            .collect::<Vec<u16>>(),
        );
        ((source == (self::ml_string::Text::from_units(vec![114, 117, 115, 116])))
            || (source == (self::ml_string::Text::from_units(vec![114, 115]))))
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_unsigned_source_typed_language(language: &self::ml_string::Text) -> bool {
    self::ml_array::includes(
        &vec![
            self::ml_string::Text::from_units(vec![106, 97, 118, 97, 115, 99, 114, 105, 112, 116]),
            self::ml_string::Text::from_units(vec![106, 115]),
            self::ml_string::Text::from_units(vec![109, 106, 115]),
            self::ml_string::Text::from_units(vec![114, 117, 115, 116]),
            self::ml_string::Text::from_units(vec![114, 115]),
        ],
        &self::ml_string::Text::from_units(
            self::ml_unicode_case::map_unicode_case_utf16_text(
                &((language.clone())
                    .0
                    .iter()
                    .map(|ml_unit| f64::from(*ml_unit))
                    .collect::<Vec<f64>>()),
                false,
            )
            .into_iter()
            .map(|ml_unit| ml_unit as u16)
            .collect::<Vec<u16>>(),
        ),
        0f64,
        false,
        |a, b| a == b,
    )
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_unsigned_source_target(language: &self::ml_string::Text) -> bool {
    self::ml_array::includes(
        &vec![
            self::ml_string::Text::from_units(vec![106, 97, 118, 97, 115, 99, 114, 105, 112, 116]),
            self::ml_string::Text::from_units(vec![106, 115]),
            self::ml_string::Text::from_units(vec![109, 106, 115]),
            self::ml_string::Text::from_units(vec![114, 117, 115, 116]),
            self::ml_string::Text::from_units(vec![114, 115]),
            self::ml_string::Text::from_units(vec![108, 101, 97, 110]),
            self::ml_string::Text::from_units(vec![114, 111, 99, 113]),
        ],
        &self::ml_string::Text::from_units(
            self::ml_unicode_case::map_unicode_case_utf16_text(
                &((language.clone())
                    .0
                    .iter()
                    .map(|ml_unit| f64::from(*ml_unit))
                    .collect::<Vec<f64>>()),
                false,
            )
            .into_iter()
            .map(|ml_unit| ml_unit as u16)
            .collect::<Vec<u16>>(),
        ),
        0f64,
        false,
        |a, b| a == b,
    )
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn finish_unsigned_source_command(
    file: &self::ml_string::Text,
    language: &self::ml_string::Text,
    first: &self::ml_string::Text,
    target: &self::ml_string::Text,
    version: &self::ml_string::Text,
    typed: bool,
    owners: bool,
    binary: bool,
) -> Vec<self::ml_string::Text> {
    if ((((((((*file) == (self::ml_string::Text::from_units(vec![])))
        || ((*language) == (self::ml_string::Text::from_units(vec![]))))
        || (typed && owners))
        || (binary && ((*target) != (self::ml_string::Text::from_units(vec![])))))
        || (((*version) != (self::ml_string::Text::from_units(vec![]))) && !owners))
        || (owners && !(self::accept_unsigned_source_owner_language(language))))
        || ((typed || ((*target) != (self::ml_string::Text::from_units(vec![]))))
            && !(self::accept_unsigned_source_typed_language(language))))
        || (((*target) != (self::ml_string::Text::from_units(vec![])))
            && !(self::accept_unsigned_source_target(target)))
    {
        vec![self::ml_string::Text::from_units(vec![
            101, 114, 114, 111, 114,
        ])]
    } else {
        vec![
            if owners {
                self::ml_string::Text::from_units(vec![
                    82, 117, 115, 116, 32, 115, 111, 117, 114, 99, 101, 32, 111, 119, 110, 101,
                    114, 115,
                ])
            } else if typed || ((*target) != (self::ml_string::Text::from_units(vec![]))) {
                self::ml_string::Text::from_units(vec![
                    116, 121, 112, 101, 100, 32, 115, 111, 117, 114, 99, 101,
                ])
            } else {
                self::ml_string::Text::from_units(vec![
                    99, 111, 110, 99, 114, 101, 116, 101, 32, 115, 111, 117, 114, 99, 101,
                ])
            },
            file.clone(),
            language.clone(),
            if (*first) == (self::ml_string::Text::from_units(vec![])) {
                self::ml_string::Text::from_units(vec![49])
            } else {
                first.clone()
            },
            target.clone(),
            version.clone(),
            if binary {
                self::ml_string::Text::from_units(vec![98, 105, 110, 97, 114, 121])
            } else {
                self::ml_string::Text::from_units(vec![116, 101, 120, 116])
            },
        ]
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_unsigned_source_command(args: &[self::ml_string::Text]) -> Vec<self::ml_string::Text> {
    {
        let file = self::ml_string::Text::from_units(vec![]);
        {
            let language = self::ml_string::Text::from_units(vec![]);
            {
                let first = self::ml_string::Text::from_units(vec![]);
                {
                    let target = self::ml_string::Text::from_units(vec![]);
                    {
                        let version = self::ml_string::Text::from_units(vec![]);
                        {
                            let typed = false;
                            {
                                let owners = false;
                                {
                                    let binary = false;
                                    {
                                        let ended = false;
                                        {
                                            let at = 0f64;
                                            {
                                                let source_ml_source1_s7 = self::source_ml_temporary_plan_unsigned_source_command_loop3(args, file, language, first, target, version, typed, owners, binary, ended, at);
                                                match source_ml_source1_s7 {
                                                    self::MlTemporaryPlanUnsignedSourceCommandLoop3Result::MlTemporaryPlanUnsignedSourceCommandLoop3Done(file_2, language_2, first_2, target_2, version_2, typed_2, owners_2, binary_2) => {
                                                        self::finish_unsigned_source_command(&(file_2), &(language_2), &(first_2), &(target_2), &(version_2), typed_2, owners_2, binary_2)
                                                    }
                                                    self::MlTemporaryPlanUnsignedSourceCommandLoop3Result::MlTemporaryPlanUnsignedSourceCommandLoop3Return(source_ml_temporary_result3) => {
                                                        source_ml_temporary_result3
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
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryAcceptUnsignedSourceCommandIdentityLoop2Result {
    MlTemporaryAcceptUnsignedSourceCommandIdentityLoop2Done,
    MlTemporaryAcceptUnsignedSourceCommandIdentityLoop2Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_accept_unsigned_source_command_identity_loop2(
    digits: &self::ml_string::Text,
    maximum: &self::ml_string::Text,
    mut at: f64,
) -> self::MlTemporaryAcceptUnsignedSourceCommandIdentityLoop2Result {
    loop {
        return if at < 20f64 {
            {
                let actual = {
                    let source_ml_source1_string_unit_operand8 = digits.clone();
                    {
                        let source_ml_source1_string_unit_operand9 = at;
                        (std::rc::Rc::new(
                            move |text: self::ml_string::Text, position: f64| -> f64 {
                                {
                                    let index = if position.is_nan() {
                                        0f64
                                    } else {
                                        position.trunc()
                                    };
                                    if (index < 0f64) || (index >= (text.length())) {
                                        f64::NAN
                                    } else {
                                        ({
                                    let source_ml_source1_string_unit_operand15 = {
                                        let source_ml_source1_string_range_operand10 = text.clone();
                                        {
                                            let source_ml_source1_string_range_operand11 = index;
                                            {
                                                let source_ml_source1_string_range_operand12 = index + 1f64;
                                                {
                                                    let source_ml_source1_string_range_position13 = source_ml_source1_string_range_operand11;
                                                    {
                                                        let source_ml_source1_string_range_position14 = source_ml_source1_string_range_operand12;
                                                        source_ml_source1_string_range_operand10.slice(source_ml_source1_string_range_position13, source_ml_source1_string_range_position14)
                                                    }
                                                }
                                            }
                                        }
                                    };
                                    {
                                        let source_ml_source1_string_unit_operand16 = 0f64;
                                        source_ml_source1_string_unit_operand15.code_point_at(source_ml_source1_string_unit_operand16)
                                    }
                                }).clone().unwrap_or_else(|| f64::NAN)
                                    }
                                }
                            },
                        ))(
                            source_ml_source1_string_unit_operand8,
                            source_ml_source1_string_unit_operand9,
                        )
                    }
                };
                {
                    let limit = {
                        let source_ml_source1_string_unit_operand17 = maximum.clone();
                        {
                            let source_ml_source1_string_unit_operand18 = at;
                            (std::rc::Rc::new(
                                move |text: self::ml_string::Text, position: f64| -> f64 {
                                    {
                                        let index = if position.is_nan() {
                                            0f64
                                        } else {
                                            position.trunc()
                                        };
                                        if (index < 0f64) || (index >= (text.length())) {
                                            f64::NAN
                                        } else {
                                            ({
                                        let source_ml_source1_string_unit_operand24 = {
                                            let source_ml_source1_string_range_operand19 = text.clone();
                                            {
                                                let source_ml_source1_string_range_operand20 = index;
                                                {
                                                    let source_ml_source1_string_range_operand21 = index + 1f64;
                                                    {
                                                        let source_ml_source1_string_range_position22 = source_ml_source1_string_range_operand20;
                                                        {
                                                            let source_ml_source1_string_range_position23 = source_ml_source1_string_range_operand21;
                                                            source_ml_source1_string_range_operand19.slice(source_ml_source1_string_range_position22, source_ml_source1_string_range_position23)
                                                        }
                                                    }
                                                }
                                            }
                                        };
                                        {
                                            let source_ml_source1_string_unit_operand25 = 0f64;
                                            source_ml_source1_string_unit_operand24.code_point_at(source_ml_source1_string_unit_operand25)
                                        }
                                    }).clone().unwrap_or_else(|| f64::NAN)
                                        }
                                    }
                                },
                            ))(
                                source_ml_source1_string_unit_operand17,
                                source_ml_source1_string_unit_operand18,
                            )
                        }
                    };
                    {
                        let ml_library_inequality_1 = actual != limit;
                        if ml_library_inequality_1 {
                            self::MlTemporaryAcceptUnsignedSourceCommandIdentityLoop2Result::MlTemporaryAcceptUnsignedSourceCommandIdentityLoop2Return(actual < limit)
                        } else {
                            {
                                let at_2 = at + 1f64;
                                {
                                    at = at_2;
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryAcceptUnsignedSourceCommandIdentityLoop2Result::MlTemporaryAcceptUnsignedSourceCommandIdentityLoop2Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryAcceptUnsignedSourceCommandIdentityLoop1Result {
    MlTemporaryAcceptUnsignedSourceCommandIdentityLoop1Done(f64),
    MlTemporaryAcceptUnsignedSourceCommandIdentityLoop1Return(bool),
}
