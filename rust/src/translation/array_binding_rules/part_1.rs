// Generated from the complete checked JavaScript source and recorded dependencies.
// Do not edit by hand.
use super::MlAcceptJavaScriptArrayBindingNameLoop1Result;
use super::MlReadArrayBindingFactFieldLoop2Result;
use super::MlReadArrayBindingInitializerEndLoop3Result;
use super::ml_accept_java_script_array_binding_name_loop1;
use super::ml_read_array_binding_fact_field_loop2;
use super::ml_read_array_binding_initializer_end_loop3;

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
pub fn refuse_required_number_fact() -> f64 {
    panic!("{}", "missing required Number fact column value")
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn refuse_required_text_fact() -> self::ml_string::Text {
    panic!("{}", "missing required text fact column value")
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn refuse_required_boolean_fact() -> bool {
    panic!("{}", "missing required Boolean fact column value")
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_required_number_fact(values: &[f64], index: f64) -> f64 {
    {
        assert!(
            !(((index < 0f64) || ((index % 1f64) != 0f64)) || (index >= (values.len() as f64))),
            "{}",
            "invalid required Number fact column position"
        );

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
    {
        assert!(
            !(((index < 0f64) || ((index % 1f64) != 0f64)) || (index >= (values.len() as f64))),
            "{}",
            "invalid required text fact column position"
        );

        (self::ml_array::optional_at(values, self::ml_array::number_index(index)))
            .unwrap_or_else(self::refuse_required_text_fact)
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_required_boolean_fact(values: &[bool], index: f64) -> bool {
    {
        assert!(
            !(((index < 0f64) || ((index % 1f64) != 0f64)) || (index >= (values.len() as f64))),
            "{}",
            "invalid required Boolean fact column position"
        );

        (self::ml_array::optional_at(values, self::ml_array::number_index(index)))
            .unwrap_or_else(self::refuse_required_boolean_fact)
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_array_binding_form(token: &self::ml_string::Text) -> self::ml_string::Text {
    if (*token) == (self::ml_string::Text::from_units(vec![93])) {
        self::ml_string::Text::from_units(vec![99, 108, 111, 115, 101])
    } else if (*token) == (self::ml_string::Text::from_units(vec![44])) {
        self::ml_string::Text::from_units(vec![101, 108, 105, 115, 105, 111, 110])
    } else if (*token) == (self::ml_string::Text::from_units(vec![46, 46, 46])) {
        self::ml_string::Text::from_units(vec![114, 101, 115, 116])
    } else if (*token) == (self::ml_string::Text::from_units(vec![91])) {
        self::ml_string::Text::from_units(vec![110, 101, 115, 116, 101, 100])
    } else {
        self::ml_string::Text::from_units(vec![110, 97, 109, 101])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_array_rest_binding_error(
    following: &self::ml_string::Text,
) -> self::ml_string::Text {
    if (*following) == (self::ml_string::Text::from_units(vec![61])) {
        self::ml_string::Text::from_units(vec![
            97, 110, 32, 97, 114, 114, 97, 121, 32, 114, 101, 115, 116, 32, 98, 105, 110, 100, 105,
            110, 103, 32, 99, 97, 110, 110, 111, 116, 32, 104, 97, 118, 101, 32, 97, 32, 100, 101,
            102, 97, 117, 108, 116,
        ])
    } else if (*following) == (self::ml_string::Text::from_units(vec![44])) {
        self::ml_string::Text::from_units(vec![
            97, 110, 32, 97, 114, 114, 97, 121, 32, 114, 101, 115, 116, 32, 98, 105, 110, 100, 105,
            110, 103, 32, 109, 117, 115, 116, 32, 98, 101, 32, 108, 97, 115, 116, 32, 119, 105,
            116, 104, 111, 117, 116, 32, 97, 32, 116, 114, 97, 105, 108, 105, 110, 103, 32, 99,
            111, 109, 109, 97,
        ])
    } else if (*following) == (self::ml_string::Text::from_units(vec![93])) {
        self::ml_string::Text::from_units(vec![])
    } else {
        self::ml_string::Text::from_units(vec![
            97, 110, 32, 97, 114, 114, 97, 121, 32, 114, 101, 115, 116, 32, 98, 105, 110, 100, 105,
            110, 103, 32, 110, 101, 101, 100, 115, 32, 116, 104, 101, 32, 99, 108, 111, 115, 105,
            110, 103, 32, 98, 114, 97, 99, 107, 101, 116,
        ])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_java_script_array_binding_name(
    kind: &self::ml_string::Text,
    name: &self::ml_string::Text,
    duplicate: bool,
    reserved: &[self::ml_string::Text],
) -> bool {
    if ((((*kind)
        != (self::ml_string::Text::from_units(vec![
            105, 100, 101, 110, 116, 105, 102, 105, 101, 114,
        ])))
        || ((*name) == (self::ml_string::Text::from_units(vec![]))))
        || duplicate)
        || (((name.length()) >= 3f64)
            && ((name.slice(0f64, 3f64))
                == (self::ml_string::Text::from_units(vec![109, 108, 95]))))
    {
        false
    } else {
        {
            let index = 0f64;
            {
                let ml_s1 =
                    self::ml_accept_java_script_array_binding_name_loop1(name, reserved, index);
                match ml_s1 {
                        self::MlAcceptJavaScriptArrayBindingNameLoop1Result::MlAcceptJavaScriptArrayBindingNameLoop1Done => {
                            true
                        }
                        self::MlAcceptJavaScriptArrayBindingNameLoop1Result::MlAcceptJavaScriptArrayBindingNameLoop1Return(ml_result1) => {
                            ml_result1
                        }
                    }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_array_binding_value_form(
    rest: bool,
    default_value: bool,
) -> self::ml_string::Text {
    if rest {
        self::ml_string::Text::from_units(vec![115, 117, 102, 102, 105, 120])
    } else if default_value {
        self::ml_string::Text::from_units(vec![100, 101, 102, 97, 117, 108, 116])
    } else {
        self::ml_string::Text::from_units(vec![101, 108, 101, 109, 101, 110, 116])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_java_script_array_rest_receiver(kind: &self::ml_string::Text) -> bool {
    (*kind) == (self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121]))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub const fn accept_java_script_array_binding_offset(
    literal: bool,
    safe_integer: bool,
    negative: bool,
) -> bool {
    (literal && safe_integer) && !negative
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_java_script_array_binding_offset_field(field: &self::ml_string::Text) -> bool {
    ((((*field) == (self::ml_string::Text::from_units(vec![107])))
        || ((*field) == (self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101]))))
        || ((*field)
            == (self::ml_string::Text::from_units(vec![110, 101, 103, 97, 116, 105, 118, 101]))))
        || ((*field) == (self::ml_string::Text::from_units(vec![115, 112, 97, 110])))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_java_script_array_binding_field(
    kind: &self::ml_string::Text,
    field: &self::ml_string::Text,
) -> bool {
    if ((*kind)
        != (self::ml_string::Text::from_units(vec![
            97, 114, 114, 97, 121, 66, 105, 110, 100, 105, 110, 103, 82, 101, 115, 116,
        ])))
        && ((*kind)
            != (self::ml_string::Text::from_units(vec![
                97, 114, 114, 97, 121, 66, 105, 110, 100, 105, 110, 103, 68, 101, 102, 97, 117,
                108, 116,
            ])))
    {
        false
    } else if (((*field) == (self::ml_string::Text::from_units(vec![107])))
        || ((*field) == (self::ml_string::Text::from_units(vec![115, 112, 97, 110]))))
        || ((*field)
            == (self::ml_string::Text::from_units(vec![
                112, 97, 114, 101, 110, 116, 104, 101, 115, 105, 122, 101, 100,
            ])))
    {
        true
    } else if (*kind)
        == (self::ml_string::Text::from_units(vec![
            97, 114, 114, 97, 121, 66, 105, 110, 100, 105, 110, 103, 82, 101, 115, 116,
        ]))
    {
        ((*field) == (self::ml_string::Text::from_units(vec![111, 98, 106, 101, 99, 116])))
            || ((*field) == (self::ml_string::Text::from_units(vec![115, 116, 97, 114, 116])))
    } else {
        ((*kind)
            == (self::ml_string::Text::from_units(vec![
                97, 114, 114, 97, 121, 66, 105, 110, 100, 105, 110, 103, 68, 101, 102, 97, 117,
                108, 116,
            ])))
            && (((*field) == (self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101])))
                || ((*field)
                    == (self::ml_string::Text::from_units(vec![
                        102, 97, 108, 108, 98, 97, 99, 107,
                    ]))))
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_array_default_operation(
    language: &self::ml_string::Text,
    kind: &self::ml_string::Text,
    absence: &self::ml_string::Text,
    element: &self::ml_string::Text,
    same_fallback: bool,
) -> self::ml_string::Text {
    {
        let ml_library_inequality_1 = (*language)
            != (self::ml_string::Text::from_units(vec![
                74, 97, 118, 97, 83, 99, 114, 105, 112, 116,
            ]));
        if ml_library_inequality_1 {
            self::ml_string::Text::from_units(vec![
                114, 101, 102, 117, 115, 101, 45, 108, 97, 110, 103, 117, 97, 103, 101,
            ])
        } else if ((*kind)
            != (self::ml_string::Text::from_units(vec![111, 112, 116, 105, 111, 110])))
            || ((*absence)
                != (self::ml_string::Text::from_units(vec![
                    117, 110, 100, 101, 102, 105, 110, 101, 100,
                ])))
        {
            self::ml_string::Text::from_units(vec![
                114, 101, 102, 117, 115, 101, 45, 97, 98, 115, 101, 110, 99, 101,
            ])
        } else if ((((((*element)
            != (self::ml_string::Text::from_units(vec![102, 108, 111, 97, 116])))
            && ((*element) != (self::ml_string::Text::from_units(vec![98, 111, 111, 108]))))
            && ((*element)
                != (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103]))))
            && ((*element) != (self::ml_string::Text::from_units(vec![105, 110, 116]))))
            && ((*element) != (self::ml_string::Text::from_units(vec![110, 97, 116]))))
            && ((*element) != (self::ml_string::Text::from_units(vec![102, 105, 120, 101, 100])))
        {
            self::ml_string::Text::from_units(vec![
                114, 101, 102, 117, 115, 101, 45, 101, 108, 101, 109, 101, 110, 116,
            ])
        } else if same_fallback {
            self::ml_string::Text::from_units(vec![99, 111, 97, 108, 101, 115, 99, 101])
        } else {
            self::ml_string::Text::from_units(vec![
                114, 101, 102, 117, 115, 101, 45, 102, 97, 108, 108, 98, 97, 99, 107,
            ])
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
pub fn plan_java_script_array_default_inference(
    texts: &[self::ml_string::Text],
    starts: &[f64],
    counts: &[f64],
    labels: &[self::ml_string::Text],
    targets: &[f64],
    resolved: f64,
    fallback: f64,
    result: f64,
) -> Vec<f64> {
    {
        let kind = self::read_array_binding_fact_field(
            starts,
            counts,
            labels,
            targets,
            resolved,
            &(self::ml_string::Text::from_units(vec![107, 105, 110, 100])),
        );
        if kind < 0f64 {
            vec![-1f64]
        } else {
            {
                let value = self::read_required_text_fact(texts, kind);
                if value == (self::ml_string::Text::from_units(vec![118, 97, 114])) {
                    Vec::<f64>::new()
                } else {
                    {
                        let absence = self::read_array_binding_fact_field(
                            starts,
                            counts,
                            labels,
                            targets,
                            resolved,
                            &(self::ml_string::Text::from_units(vec![
                                97, 98, 115, 101, 110, 99, 101,
                            ])),
                        );
                        if ((value
                            != (self::ml_string::Text::from_units(vec![
                                111, 112, 116, 105, 111, 110,
                            ])))
                            || (absence < 0f64))
                            || ((self::read_required_text_fact(texts, absence))
                                != (self::ml_string::Text::from_units(vec![
                                    117, 110, 100, 101, 102, 105, 110, 101, 100,
                                ])))
                        {
                            vec![-1f64]
                        } else {
                            {
                                let element = self::read_array_binding_fact_field(
                                    starts,
                                    counts,
                                    labels,
                                    targets,
                                    resolved,
                                    &(self::ml_string::Text::from_units(vec![
                                        101, 108, 101, 109, 101, 110, 116,
                                    ])),
                                );
                                if element < 0f64 {
                                    vec![-1f64]
                                } else {
                                    vec![element, fallback, result, element]
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
pub fn read_array_binding_fact_field(
    starts: &[f64],
    counts: &[f64],
    labels: &[self::ml_string::Text],
    targets: &[f64],
    node: f64,
    field: &self::ml_string::Text,
) -> f64 {
    if ((node < 0f64) || (node >= (starts.len() as f64))) || (node >= (counts.len() as f64)) {
        -1f64
    } else {
        {
            let at = self::read_required_number_fact(starts, node);
            {
                let end = at + self::read_required_number_fact(counts, node);
                {
                    let ml_s2 = self::ml_read_array_binding_fact_field_loop2(
                        labels, targets, field, at, end,
                    );
                    match ml_s2 {
                        self::MlReadArrayBindingFactFieldLoop2Result::MlReadArrayBindingFactFieldLoop2Done => {
                            -1f64
                        }
                        self::MlReadArrayBindingFactFieldLoop2Result::MlReadArrayBindingFactFieldLoop2Return(ml_result2) => {
                            ml_result2
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_array_binding_initializer_end(
    kinds: &[self::ml_string::Text],
    values: &[self::ml_string::Text],
    start: f64,
) -> f64 {
    {
        let index = start;
        {
            let closes = Vec::<self::ml_string::Text>::new();
            {
                let ml_s3 = self::ml_read_array_binding_initializer_end_loop3(
                    kinds, values, start, index, closes,
                );
                match ml_s3 {
                    self::MlReadArrayBindingInitializerEndLoop3Result::MlReadArrayBindingInitializerEndLoop3Done => {
                        -1f64
                    }
                    self::MlReadArrayBindingInitializerEndLoop3Result::MlReadArrayBindingInitializerEndLoop3Return(ml_result3) => {
                        ml_result3
                    }
                }
            }
        }
    }
}
