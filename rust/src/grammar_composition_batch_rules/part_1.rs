// Generated from complete js/src/grammar-composition-batch-rules.js. Do not edit by hand.
use super::MlTemporaryContainsSortedLoop1Result;
use super::MlTemporaryReadGrammarCompositionBatchIssueLoop2Result;
use super::MlTemporaryReadGrammarCompositionBatchIssueLoop3Result;
use super::MlTemporaryReadGrammarCompositionBatchIssueLoop4Result;
use super::MlTemporaryReadGrammarCompositionBatchIssueLoop5Result;
use super::MlTemporaryReadGrammarCompositionBatchIssueLoop6Result;
use super::MlTemporaryReadGrammarCompositionBatchIssueLoop7Result;
use super::source_ml_temporary_contains_sorted_loop1;
use super::source_ml_temporary_read_grammar_composition_batch_issue_loop2;
use super::source_ml_temporary_read_grammar_composition_batch_issue_loop3;
use super::source_ml_temporary_read_grammar_composition_batch_issue_loop4;
use super::source_ml_temporary_read_grammar_composition_batch_issue_loop5;
use super::source_ml_temporary_read_grammar_composition_batch_issue_loop6;
use super::source_ml_temporary_read_grammar_composition_batch_issue_loop7;

/// The Math functions that Rust's f64 methods do not share with JavaScript.
pub mod ml_math {
    /// Math.round: the nearest integer, the one towards +Infinity on a tie,
    /// with the sign of a zero result from the argument.
    #[must_use]
    pub fn round(x: f64) -> f64 {
        if !x.is_finite() || x == 0.0 {
            return x;
        }
        if x > 0.0 && x < 0.5 {
            return 0.0;
        }
        if (-0.5..0.0).contains(&x) {
            return -0.0;
        }
        let floor = x.floor();
        if x - floor >= 0.5 { floor + 1.0 } else { floor }
    }

    /// Math.sign, which keeps -0, 0 and NaN, where `f64::signum` does not.
    #[must_use]
    pub fn sign(x: f64) -> f64 {
        if x > 0.0 {
            1.0
        } else if x < 0.0 {
            -1.0
        } else {
            x
        }
    }

    /// Math.max of two Numbers: NaN when either is, and 0 above -0.
    #[must_use]
    pub fn max(a: f64, b: f64) -> f64 {
        if a > b {
            a
        } else if b > a {
            b
        } else if a == b {
            if a.is_sign_negative() { b } else { a }
        } else {
            f64::NAN
        }
    }

    /// Math.min of two Numbers: NaN when either is, and -0 below 0.
    #[must_use]
    pub fn min(a: f64, b: f64) -> f64 {
        if a < b {
            a
        } else if b < a {
            b
        } else if a == b {
            if a.is_sign_negative() { a } else { b }
        } else {
            f64::NAN
        }
    }

    #[must_use]
    pub fn max_of(values: &[f64]) -> f64 {
        values.iter().fold(f64::NEG_INFINITY, |a, &b| max(a, b))
    }

    #[must_use]
    pub fn min_of(values: &[f64]) -> f64 {
        values.iter().fold(f64::INFINITY, |a, &b| min(a, b))
    }

    #[must_use]
    pub fn is_integer(x: f64) -> bool {
        x.is_finite() && x.trunc() == x
    }

    #[must_use]
    pub fn is_safe_integer(x: f64) -> bool {
        is_integer(x) && x.abs() <= 9_007_199_254_740_991.0
    }
}

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
pub fn contains_sorted(values: &[self::ml_string::Text], key: &self::ml_string::Text) -> bool {
    {
        let first = 0f64;
        {
            let last = values.len() as f64;
            {
                let source_ml_source1_s1 =
                    self::source_ml_temporary_contains_sorted_loop1(values, key, first, last);
                match source_ml_source1_s1 {
                    self::MlTemporaryContainsSortedLoop1Result::MlTemporaryContainsSortedLoop1Done => {
                        false
                    }
                    self::MlTemporaryContainsSortedLoop1Result::MlTemporaryContainsSortedLoop1Return(source_ml_temporary_result1) => {
                        source_ml_temporary_result1
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn valid_name(value: &self::ml_string::Text) -> bool {
    ((*value) != (self::ml_string::Text::from_units(vec![])))
        && !(value.contains(&self::ml_string::Text::from_units(vec![0])))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_grammar_composition_batch_count_issue(
    sources: f64,
    rules: f64,
    policies: f64,
) -> self::ml_string::Text {
    if (((((((!(self::ml_math::is_integer(sources)) || !(self::ml_math::is_integer(rules)))
        || !(self::ml_math::is_integer(policies)))
        || (sources < 1f64))
        || (sources > 4096f64))
        || (rules < 0f64))
        || (rules > 65_536f64))
        || (policies < 0f64))
        || (policies > 4096f64)
    {
        self::ml_string::Text::from_units(vec![
            115, 111, 117, 114, 99, 101, 44, 32, 114, 117, 108, 101, 32, 111, 114, 32, 112, 111,
            108, 105, 99, 121, 32, 99, 111, 117, 110, 116, 32, 101, 120, 99, 101, 101, 100, 115,
            32, 116, 104, 101, 32, 98, 97, 116, 99, 104, 32, 108, 105, 109, 105, 116,
        ])
    } else {
        self::ml_string::Text::from_units(vec![])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_grammar_composition_batch_text_issue(units: f64) -> self::ml_string::Text {
    if (!(self::ml_math::is_integer(units)) || (units < 0f64)) || (units > 8_388_608f64) {
        self::ml_string::Text::from_units(vec![
            98, 97, 116, 99, 104, 32, 116, 101, 120, 116, 32, 101, 120, 99, 101, 101, 100, 115, 32,
            116, 104, 101, 32, 115, 111, 117, 114, 99, 101, 32, 117, 110, 105, 116, 32, 108, 105,
            109, 105, 116,
        ])
    } else {
        self::ml_string::Text::from_units(vec![])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_grammar_composition_batch_issue(
    sources: &[self::ml_string::Text],
    owners: &[self::ml_string::Text],
    names: &[self::ml_string::Text],
    kinds: &[self::ml_string::Text],
    policy_sources: &[self::ml_string::Text],
    policy_rules: &[self::ml_string::Text],
    compositions: &[self::ml_string::Text],
) -> self::ml_string::Text {
    {
        let count_issue = self::read_grammar_composition_batch_count_issue(
            (sources.len() as f64),
            (names.len() as f64),
            (policy_sources.len() as f64),
        );
        if count_issue.0.is_empty() {
            if ((((owners.len() as f64) != (names.len() as f64))
                || ((kinds.len() as f64) != (names.len() as f64)))
                || ((policy_sources.len() as f64) != (policy_rules.len() as f64)))
                || ((policy_sources.len() as f64) != (compositions.len() as f64))
            {
                self::ml_string::Text::from_units(vec![
                    117, 110, 97, 108, 105, 103, 110, 101, 100, 32, 98, 97, 116, 99, 104, 32, 99,
                    111, 108, 117, 109, 110, 115,
                ])
            } else {
                {
                    let units = 0f64;
                    {
                        let index = 0f64;
                        {
                            let source_ml_source1_s2 = self::source_ml_temporary_read_grammar_composition_batch_issue_loop2(sources, units, index);
                            match source_ml_source1_s2 {
                                self::MlTemporaryReadGrammarCompositionBatchIssueLoop2Result::MlTemporaryReadGrammarCompositionBatchIssueLoop2Done(units_2) => {
                                    {
                                        let sorted_sources = { let mut ml_sorted_values = sources.to_vec(); ml_sorted_values.sort_by(|ml_sort_left, ml_sort_right| ml_sort_left.0.cmp(&ml_sort_right.0)); ml_sorted_values };
                                        {
                                            let index_2 = 0f64;
                                            {
                                                let source_ml_source1_s3 = self::source_ml_temporary_read_grammar_composition_batch_issue_loop3(&(sorted_sources), index_2);
                                                match source_ml_source1_s3 {
                                                    self::MlTemporaryReadGrammarCompositionBatchIssueLoop3Result::MlTemporaryReadGrammarCompositionBatchIssueLoop3Done => {
                                                        {
                                                            let keys = Vec::<self::ml_string::Text>::new();
                                                            {
                                                                let normal = Vec::<self::ml_string::Text>::new();
                                                                {
                                                                    let index_3 = 0f64;
                                                                    {
                                                                        let source_ml_source1_s4 = self::source_ml_temporary_read_grammar_composition_batch_issue_loop4(owners, names, kinds, units_2, &(sorted_sources), keys, normal, index_3);
                                                                        match source_ml_source1_s4 {
                                                                            self::MlTemporaryReadGrammarCompositionBatchIssueLoop4Result::MlTemporaryReadGrammarCompositionBatchIssueLoop4Done(units_3, keys_2, normal_2) => {
                                                                                {
                                                                                    let sorted_keys = { let mut ml_sorted_values = keys_2; ml_sorted_values.sort_by(|ml_sort_left, ml_sort_right| ml_sort_left.0.cmp(&ml_sort_right.0)); ml_sorted_values };
                                                                                    {
                                                                                        let normal_keys = { let mut ml_sorted_values = normal_2; ml_sorted_values.sort_by(|ml_sort_left, ml_sort_right| ml_sort_left.0.cmp(&ml_sort_right.0)); ml_sorted_values };
                                                                                        {
                                                                                            let index_4 = 1f64;
                                                                                            {
                                                                                                let source_ml_source1_s5 = self::source_ml_temporary_read_grammar_composition_batch_issue_loop5(&(sorted_keys), index_4);
                                                                                                match source_ml_source1_s5 {
                                                                                                    self::MlTemporaryReadGrammarCompositionBatchIssueLoop5Result::MlTemporaryReadGrammarCompositionBatchIssueLoop5Done => {
                                                                                                        {
                                                                                                            let targets = Vec::<self::ml_string::Text>::new();
                                                                                                            {
                                                                                                                let index_5 = 0f64;
                                                                                                                {
                                                                                                                    let source_ml_source1_s6 = self::source_ml_temporary_read_grammar_composition_batch_issue_loop6(policy_sources, policy_rules, compositions, units_3, &(sorted_keys), &(normal_keys), targets, index_5);
                                                                                                                    match source_ml_source1_s6 {
                                                                                                                        self::MlTemporaryReadGrammarCompositionBatchIssueLoop6Result::MlTemporaryReadGrammarCompositionBatchIssueLoop6Done(targets_2) => {
                                                                                                                            {
                                                                                                                                let sorted_targets = { let mut ml_sorted_values = targets_2; ml_sorted_values.sort_by(|ml_sort_left, ml_sort_right| ml_sort_left.0.cmp(&ml_sort_right.0)); ml_sorted_values };
                                                                                                                                {
                                                                                                                                    let index_6 = 1f64;
                                                                                                                                    {
                                                                                                                                        let source_ml_source1_s7 = self::source_ml_temporary_read_grammar_composition_batch_issue_loop7(&(sorted_targets), index_6);
                                                                                                                                        match source_ml_source1_s7 {
                                                                                                                                            self::MlTemporaryReadGrammarCompositionBatchIssueLoop7Result::MlTemporaryReadGrammarCompositionBatchIssueLoop7Done => {
                                                                                                                                                self::ml_string::Text::from_units(vec![])
                                                                                                                                            }
                                                                                                                                            self::MlTemporaryReadGrammarCompositionBatchIssueLoop7Result::MlTemporaryReadGrammarCompositionBatchIssueLoop7Return(source_ml_temporary_result7) => {
                                                                                                                                                source_ml_temporary_result7.clone()
                                                                                                                                            }
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                        self::MlTemporaryReadGrammarCompositionBatchIssueLoop6Result::MlTemporaryReadGrammarCompositionBatchIssueLoop6Return(source_ml_temporary_result7_2) => {
                                                                                                                            source_ml_temporary_result7_2.clone()
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                    self::MlTemporaryReadGrammarCompositionBatchIssueLoop5Result::MlTemporaryReadGrammarCompositionBatchIssueLoop5Return(source_ml_temporary_result7_3) => {
                                                                                                        source_ml_temporary_result7_3.clone()
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                            self::MlTemporaryReadGrammarCompositionBatchIssueLoop4Result::MlTemporaryReadGrammarCompositionBatchIssueLoop4Return(source_ml_temporary_result7_4) => {
                                                                                source_ml_temporary_result7_4.clone()
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                    self::MlTemporaryReadGrammarCompositionBatchIssueLoop3Result::MlTemporaryReadGrammarCompositionBatchIssueLoop3Return(source_ml_temporary_result7_5) => {
                                                        source_ml_temporary_result7_5.clone()
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                                self::MlTemporaryReadGrammarCompositionBatchIssueLoop2Result::MlTemporaryReadGrammarCompositionBatchIssueLoop2Return(source_ml_temporary_result7_6) => {
                                    source_ml_temporary_result7_6.clone()
                                }
                            }
                        }
                    }
                }
            }
        } else {
            count_issue.clone()
        }
    }
}
