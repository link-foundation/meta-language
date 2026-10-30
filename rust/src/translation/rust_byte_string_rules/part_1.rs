// Generated from complete js/src/translation/rust-byte-string-rules.js. Do not edit by hand.
use super::MlTemporaryDecodeRustUnicodeEscapeLoop4Result;
use super::MlTemporaryReadSourceTextByteLengthLoop1Result;
use super::MlTemporaryReadSourceTextByteLoop3Result;
use super::MlTemporaryReadSourceTextCodeUnitOffsetLoop2Result;
use super::source_ml_temporary_decode_rust_unicode_escape_loop4;
use super::source_ml_temporary_read_source_text_byte_length_loop1;
use super::source_ml_temporary_read_source_text_byte_loop3;
use super::source_ml_temporary_read_source_text_code_unit_offset_loop2;

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

#[derive(Clone, Debug, PartialEq)]
pub enum RustUnicodeEscape {
    Scalar(f64, f64),
    Malformed,
    Unsupported(f64, f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum RustCharacterLiteral {
    NotCharacter,
    Char(f64, f64, f64, f64),
    ByteChar(f64, f64, f64, f64),
    Malformed(f64, f64),
    Unterminated(f64, f64),
    NonScalar(f64, f64),
    NonAscii(f64, f64),
    UnicodeByteEscape(f64, f64),
    UnsupportedSuffix(f64, f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_source_text_scalar(units: &[f64], index: f64) -> f64 {
    if ((!(self::ml_math::is_integer(index)) || (index < 0f64)) || (index >= (units.len() as f64)))
        || ((units.len() as f64) > 4_294_967_295f64)
    {
        -1f64
    } else {
        {
            let first = (self::ml_array::optional_at(units, self::ml_array::number_index(index)))
                .unwrap_or_else(|| -1f64);
            if (!(self::ml_math::is_integer(first)) || (first < 0f64)) || (first > 65_535f64) {
                -1f64
            } else if (55_296f64..=56_319f64).contains(&first) {
                if (index + 1f64) >= (units.len() as f64) {
                    -1f64
                } else {
                    {
                        let second = (self::ml_array::optional_at(
                            units,
                            self::ml_array::number_index(index + 1f64),
                        ))
                        .unwrap_or_else(|| -1f64);
                        if (!(self::ml_math::is_integer(second)) || (second < 56_320f64))
                            || (second > 57_343f64)
                        {
                            -1f64
                        } else {
                            ((65_536f64 + ((first - 55_296f64) * 1024f64)) + second) - 56_320f64
                        }
                    }
                }
            } else if (56_320f64..=57_343f64).contains(&first) {
                -1f64
            } else {
                first
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_source_text_scalar_byte_width(scalar: f64) -> f64 {
    if ((!(self::ml_math::is_integer(scalar)) || (scalar < 0f64)) || (scalar > 1_114_111f64))
        || ((55_296f64..=57_343f64).contains(&scalar))
    {
        -1f64
    } else if scalar < 128f64 {
        1f64
    } else if scalar < 2048f64 {
        2f64
    } else if scalar < 65_536f64 {
        3f64
    } else {
        4f64
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_source_text_byte_length(units: &[f64]) -> f64 {
    if (units.len() as f64) > 4_294_967_295f64 {
        -1f64
    } else {
        {
            let index = 0f64;
            {
                let bytes = 0f64;
                {
                    let source_ml_source1_s1 =
                        self::source_ml_temporary_read_source_text_byte_length_loop1(
                            units, index, bytes,
                        );
                    match source_ml_source1_s1 {
                        self::MlTemporaryReadSourceTextByteLengthLoop1Result::MlTemporaryReadSourceTextByteLengthLoop1Done(bytes_2) => {
                            bytes_2
                        }
                        self::MlTemporaryReadSourceTextByteLengthLoop1Result::MlTemporaryReadSourceTextByteLengthLoop1Return(source_ml_temporary_result1) => {
                            source_ml_temporary_result1
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_source_text_scalar_units(units: &[f64]) -> bool {
    (self::read_source_text_byte_length(units)) >= 0f64
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_source_text_code_unit_offset(units: &[f64], offset: f64) -> f64 {
    {
        let length = self::read_source_text_byte_length(units);
        if (((length < 0f64) || !(self::ml_math::is_integer(offset))) || (offset < 0f64))
            || (offset > length)
        {
            -1f64
        } else {
            {
                let index = 0f64;
                {
                    let bytes = 0f64;
                    {
                        let source_ml_source1_s2 =
                            self::source_ml_temporary_read_source_text_code_unit_offset_loop2(
                                units, offset, index, bytes,
                            );
                        match source_ml_source1_s2 {
                            self::MlTemporaryReadSourceTextCodeUnitOffsetLoop2Result::MlTemporaryReadSourceTextCodeUnitOffsetLoop2Done(index_2, bytes_2) => {
                                if bytes_2 == offset {
                                    index_2
                                } else {
                                    -1f64
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
pub fn accept_source_text_byte_slice(units: &[f64], start: f64, end: f64) -> bool {
    ((start <= end) && ((self::read_source_text_code_unit_offset(units, start)) >= 0f64))
        && ((self::read_source_text_code_unit_offset(units, end)) >= 0f64)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_source_text_byte(units: &[f64], offset: f64) -> f64 {
    {
        let length = self::read_source_text_byte_length(units);
        if (((length < 0f64) || !(self::ml_math::is_integer(offset))) || (offset < 0f64))
            || (offset >= length)
        {
            -1f64
        } else {
            {
                let index = 0f64;
                {
                    let bytes = 0f64;
                    {
                        let source_ml_source1_s3 =
                            self::source_ml_temporary_read_source_text_byte_loop3(
                                units, offset, index, bytes,
                            );
                        match source_ml_source1_s3 {
                            self::MlTemporaryReadSourceTextByteLoop3Result::MlTemporaryReadSourceTextByteLoop3Done => {
                                -1f64
                            }
                            self::MlTemporaryReadSourceTextByteLoop3Result::MlTemporaryReadSourceTextByteLoop3Return(source_ml_temporary_result3) => {
                                source_ml_temporary_result3
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
pub fn accept_source_text_unit_count(count: f64) -> bool {
    (self::ml_math::is_integer(count) && (count >= 0f64)) && (count <= 4_294_967_295f64)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_cooked_string_escape_prefix(unit: f64) -> bool {
    ((((((((unit == 34f64) || (unit == 39f64)) || (unit == 48f64)) || (unit == 92f64))
        || (unit == 110f64))
        || (unit == 114f64))
        || (unit == 116f64))
        || (unit == 117f64))
        || (unit == 120f64)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_ascii_escape_digit(unit: f64) -> f64 {
    if unit == 48f64 {
        0f64
    } else if unit == 49f64 {
        1f64
    } else if unit == 50f64 {
        2f64
    } else if unit == 51f64 {
        3f64
    } else if unit == 52f64 {
        4f64
    } else if unit == 53f64 {
        5f64
    } else if unit == 54f64 {
        6f64
    } else if unit == 55f64 {
        7f64
    } else if unit == 56f64 {
        8f64
    } else if unit == 57f64 {
        9f64
    } else if (unit == 65f64) || (unit == 97f64) {
        10f64
    } else if (unit == 66f64) || (unit == 98f64) {
        11f64
    } else if (unit == 67f64) || (unit == 99f64) {
        12f64
    } else if (unit == 68f64) || (unit == 100f64) {
        13f64
    } else if (unit == 69f64) || (unit == 101f64) {
        14f64
    } else if (unit == 70f64) || (unit == 102f64) {
        15f64
    } else {
        -1f64
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_ascii_escape_value(high_unit: f64, low_unit: f64) -> f64 {
    {
        let high = self::read_rust_ascii_escape_digit(high_unit);
        {
            let low = self::read_rust_ascii_escape_digit(low_unit);
            if ((high < 0f64) || (high > 7f64)) || (low < 0f64) {
                -1f64
            } else {
                (high * 16f64) + low
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn decode_rust_unicode_escape(units: &[f64], start: f64) -> self::RustUnicodeEscape {
    if (((((start < 0f64) || (start >= (units.len() as f64))) || ((start % 1f64) != 0f64)) || {
        let source_ml_source1_comparison_operand4 =
            self::ml_array::optional_at(units, self::ml_array::number_index(start));
        {
            let source_ml_source1_comparison_operand5 = 92f64;
            !(match (
                source_ml_source1_comparison_operand4,
                source_ml_source1_comparison_operand5,
            ) {
                (Some(ml_left), ml_right) => ml_left == ml_right,
                _ => false,
            })
        }
    }) || {
        let source_ml_source1_comparison_operand6 =
            self::ml_array::optional_at(units, self::ml_array::number_index(start + 1f64));
        {
            let source_ml_source1_comparison_operand7 = 117f64;
            !(match (
                source_ml_source1_comparison_operand6,
                source_ml_source1_comparison_operand7,
            ) {
                (Some(ml_left), ml_right) => ml_left == ml_right,
                _ => false,
            })
        }
    }) || {
        let source_ml_source1_comparison_operand8 =
            self::ml_array::optional_at(units, self::ml_array::number_index(start + 2f64));
        {
            let source_ml_source1_comparison_operand9 = 123f64;
            !(match (
                source_ml_source1_comparison_operand8,
                source_ml_source1_comparison_operand9,
            ) {
                (Some(ml_left), ml_right) => ml_left == ml_right,
                _ => false,
            })
        }
    } {
        self::RustUnicodeEscape::Malformed
    } else {
        {
            let index = start + 3f64;
            {
                let code = 0f64;
                {
                    let digits = 0f64;
                    {
                        let source_ml_source1_s10 =
                            self::source_ml_temporary_decode_rust_unicode_escape_loop4(
                                units, start, index, code, digits,
                            );
                        match source_ml_source1_s10 {
                            self::MlTemporaryDecodeRustUnicodeEscapeLoop4Result::MlTemporaryDecodeRustUnicodeEscapeLoop4Done => {
                                self::RustUnicodeEscape::Malformed
                            }
                            self::MlTemporaryDecodeRustUnicodeEscapeLoop4Result::MlTemporaryDecodeRustUnicodeEscapeLoop4Return(source_ml_temporary_result4) => {
                                let source_ml_temporary_result4 = *source_ml_temporary_result4;
                                source_ml_temporary_result4.clone()
                            }
                        }
                    }
                }
            }
        }
    }
}
