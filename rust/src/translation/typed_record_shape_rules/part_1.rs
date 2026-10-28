// Generated from complete js/src/translation/typed-record-shape-rules.js. Do not edit by hand.
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

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn read_typed_program_record_shape_error(
    selected: bool,
    references: f64,
    direct: bool,
    shape_fields: bool,
    qualified_shape: bool,
    language: &self::ml_string::Text,
    term: &self::ml_string::Text,
    definition: &self::ml_string::Text,
    expected_language: &self::ml_string::Text,
    expected_term: &self::ml_string::Text,
    names: &[self::ml_string::Text],
    fields: &[bool],
    self_: &[bool],
    role_fields: &[bool],
) -> self::ml_string::Text {
    if selected {
        if direct || shape_fields {
            self::ml_string::Text::from_units(vec![
                111, 114, 100, 101, 114, 101, 100, 32, 112, 114, 111, 103, 114, 97, 109, 32, 114,
                101, 99, 111, 114, 100, 32, 118, 97, 108, 117, 101, 115, 32, 97, 110, 100, 32, 115,
                104, 97, 112, 101, 115, 32, 99, 97, 110, 110, 111, 116, 32, 97, 108, 115, 111, 32,
                99, 97, 114, 114, 121, 32, 110, 97, 109, 101, 100, 32, 102, 105, 101, 108, 100,
                115,
            ])
        } else if ((!qualified_shape || ((*language) != (*expected_language)))
            || ((*term) != (*expected_term)))
            || ((*definition) != (self::ml_string::Text::from_units(vec![])))
        {
            self::ml_string::Text::from_units(vec![
                111, 114, 100, 101, 114, 101, 100, 32, 112, 114, 111, 103, 114, 97, 109, 32, 114,
                101, 99, 111, 114, 100, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104,
                101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 113, 117, 97, 108, 105, 102, 105,
                101, 100, 32, 102, 105, 101, 108, 100, 32, 115, 104, 97, 112, 101,
            ])
        } else if (((((!(self::ml_math::is_safe_integer(references)) || (references < 1f64))
            || (references > 100_001f64))
            || ((names.len() as f64) != (references - 1f64)))
            || ((names.len() as f64) != (fields.len() as f64)))
            || ((names.len() as f64) != (self_.len() as f64)))
            || ((names.len() as f64) != (role_fields.len() as f64))
        {
            self::ml_string::Text::from_units(vec![
                111, 114, 100, 101, 114, 101, 100, 32, 112, 114, 111, 103, 114, 97, 109, 32, 114,
                101, 99, 111, 114, 100, 32, 114, 111, 108, 101, 115, 32, 109, 117, 115, 116, 32,
                109, 97, 116, 99, 104, 32, 101, 118, 101, 114, 121, 32, 97, 99, 116, 117, 97, 108,
                32, 118, 97, 108, 117, 101, 32, 99, 111, 111, 114, 100, 105, 110, 97, 116, 101,
            ])
        } else {
            {
                let ordered = {
                    let mut ml_sorted_values = names.to_vec();
                    ml_sorted_values.sort_by(|ml_sort_left, ml_sort_right| {
                        ml_sort_left.0.cmp(&ml_sort_right.0)
                    });
                    ml_sorted_values
                };
                {
                    let index = 0f64;
                    {
                        let source_ml_source1_s1 =
                            self::source_ml_temporary_read_typed_program_record_shape_error_loop1(
                                names,
                                fields,
                                self_,
                                role_fields,
                                &(ordered),
                                index,
                            );
                        match source_ml_source1_s1 {
                                    self::MlTemporaryReadTypedProgramRecordShapeErrorLoop1Result::MlTemporaryReadTypedProgramRecordShapeErrorLoop1Done => {
                                        self::ml_string::Text::from_units(vec![])
                                    }
                                    self::MlTemporaryReadTypedProgramRecordShapeErrorLoop1Result::MlTemporaryReadTypedProgramRecordShapeErrorLoop1Return(source_ml_temporary_result1) => {
                                        source_ml_temporary_result1.clone()
                                    }
                                }
                    }
                }
            }
        }
    } else {
        self::ml_string::Text::from_units(vec![
            111, 114, 100, 101, 114, 101, 100, 32, 112, 114, 111, 103, 114, 97, 109, 32, 114, 101,
            99, 111, 114, 100, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105,
            114, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 114, 111, 111, 116, 32, 108, 97,
            121, 111, 117, 116, 32, 112, 114, 111, 102, 105, 108, 101,
        ])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_typed_program_record_shape_selection_error(
    count: f64,
    qualified: bool,
    references: f64,
    fields: bool,
) -> self::ml_string::Text {
    if count == 0f64 {
        self::ml_string::Text::from_units(vec![])
    } else if count == 1f64 {
        if (!qualified || (references != 0f64)) || fields {
            self::ml_string::Text::from_units(vec![
                105, 110, 118, 97, 108, 105, 100, 32, 114, 111, 111, 116, 32, 108, 97, 121, 111,
                117, 116, 32, 115, 101, 108, 101, 99, 116, 105, 111, 110,
            ])
        } else {
            self::ml_string::Text::from_units(vec![])
        }
    } else {
        self::ml_string::Text::from_units(vec![
            100, 117, 112, 108, 105, 99, 97, 116, 101, 32, 114, 111, 111, 116, 32, 108, 97, 121,
            111, 117, 116, 32, 115, 101, 108, 101, 99, 116, 105, 111, 110,
        ])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_typed_program_record_role_text_error(
    units: f64,
    oversized: bool,
) -> self::ml_string::Text {
    if oversized {
        self::ml_string::Text::from_units(vec![
            111, 114, 100, 101, 114, 101, 100, 32, 112, 114, 111, 103, 114, 97, 109, 32, 114, 101,
            99, 111, 114, 100, 32, 114, 111, 108, 101, 115, 32, 114, 101, 113, 117, 105, 114, 101,
            32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 117, 110, 105, 113, 117, 101, 32, 102,
            105, 101, 108, 100, 32, 112, 111, 105, 110, 116, 115,
        ])
    } else if (!(self::ml_math::is_safe_integer(units)) || (units < 0f64))
        || (units > 16_777_216f64)
    {
        self::ml_string::Text::from_units(vec![
            111, 114, 100, 101, 114, 101, 100, 32, 112, 114, 111, 103, 114, 97, 109, 32, 114, 101,
            99, 111, 114, 100, 32, 114, 111, 108, 101, 32, 116, 101, 120, 116, 32, 101, 120, 99,
            101, 101, 100, 115, 32, 105, 116, 115, 32, 112, 114, 111, 106, 101, 99, 116, 105, 111,
            110, 32, 114, 101, 115, 111, 117, 114, 99, 101, 32, 98, 117, 100, 103, 101, 116,
        ])
    } else {
        self::ml_string::Text::from_units(vec![])
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadTypedProgramRecordShapeErrorLoop1Result {
    MlTemporaryReadTypedProgramRecordShapeErrorLoop1Done,
    MlTemporaryReadTypedProgramRecordShapeErrorLoop1Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_typed_program_record_shape_error_loop1(
    names: &[self::ml_string::Text],
    fields: &[bool],
    self_: &[bool],
    role_fields: &[bool],
    ordered: &[self::ml_string::Text],
    mut index: f64,
) -> self::MlTemporaryReadTypedProgramRecordShapeErrorLoop1Result {
    loop {
        return if index < (names.len() as f64) {
            {
                let name =
                    (self::ml_array::optional_at(names, self::ml_array::number_index(index)))
                        .unwrap_or_else(|| self::ml_string::Text::from_units(vec![]));
                if ((((((name.length()) == 0f64) || ((name.length()) > 1_000_000f64))
                    || ((index > 0f64) && {
                        let source_ml_source1_comparison_operand2 = self::ml_array::optional_at(
                            ordered,
                            self::ml_array::number_index(index),
                        );
                        {
                            let source_ml_source1_comparison_operand3 = self::ml_array::optional_at(
                                ordered,
                                self::ml_array::number_index(index - 1f64),
                            );
                            match (
                                source_ml_source1_comparison_operand2,
                                source_ml_source1_comparison_operand3,
                            ) {
                                (Some(ml_left), Some(ml_right)) => ml_left == ml_right,
                                (None, None) => true,
                                _ => false,
                            }
                        }
                    }))
                    || {
                        let source_ml_source1_comparison_operand4 = self::ml_array::optional_at(
                            fields,
                            self::ml_array::number_index(index),
                        );
                        {
                            let source_ml_source1_comparison_operand5 = true;
                            !(match (
                                source_ml_source1_comparison_operand4,
                                source_ml_source1_comparison_operand5,
                            ) {
                                (Some(ml_left), ml_right) => ml_left == ml_right,
                                _ => false,
                            })
                        }
                    })
                    || {
                        let source_ml_source1_comparison_operand6 =
                            self::ml_array::optional_at(self_, self::ml_array::number_index(index));
                        {
                            let source_ml_source1_comparison_operand7 = true;
                            !(match (
                                source_ml_source1_comparison_operand6,
                                source_ml_source1_comparison_operand7,
                            ) {
                                (Some(ml_left), ml_right) => ml_left == ml_right,
                                _ => false,
                            })
                        }
                    })
                    || {
                        let source_ml_source1_comparison_operand8 = self::ml_array::optional_at(
                            role_fields,
                            self::ml_array::number_index(index),
                        );
                        {
                            let source_ml_source1_comparison_operand9 = false;
                            !(match (
                                source_ml_source1_comparison_operand8,
                                source_ml_source1_comparison_operand9,
                            ) {
                                (Some(ml_left), ml_right) => ml_left == ml_right,
                                _ => false,
                            })
                        }
                    }
                {
                    self::MlTemporaryReadTypedProgramRecordShapeErrorLoop1Result::MlTemporaryReadTypedProgramRecordShapeErrorLoop1Return(self::ml_string::Text::from_units(vec![111, 114, 100, 101, 114, 101, 100, 32, 112, 114, 111, 103, 114, 97, 109, 32, 114, 101, 99, 111, 114, 100, 32, 114, 111, 108, 101, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 117, 110, 105, 113, 117, 101, 32, 102, 105, 101, 108, 100, 32, 112, 111, 105, 110, 116, 115]))
                } else {
                    {
                        let index_2 = index + 1f64;
                        {
                            index = index_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadTypedProgramRecordShapeErrorLoop1Result::MlTemporaryReadTypedProgramRecordShapeErrorLoop1Done
        };
    }
}
