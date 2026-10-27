// Generated from complete js/src/translation/javascript-class-source-rules.js. Do not edit by hand.
use super::MlTemporaryFindJavaScriptClassGroupEndLoop1Result;
use super::MlTemporaryFindJavaScriptClosedClassInitializationStartLoop5Result;
use super::MlTemporaryReadJavaScriptClosedClassDirectoryJoin8;
use super::MlTemporaryReadJavaScriptClosedClassDirectoryLoop6Result;
use super::MlTemporaryReadJavaScriptClosedClassDirectoryLoop7Result;
use super::MlTemporaryReadJavaScriptClosedClassDirectoryLoop9Result;
use super::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop22Result;
use super::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop23Result;
use super::MlTemporaryReadJavaScriptClosedClassParameterPositionsLoop2Result;
use super::MlTemporaryReadJavaScriptClosedClassStaticTargetLoop19Result;
use super::MlTemporaryRenderJavaScriptClosedClassRangeLoop14Result;
use super::ml;
use super::ml_array;
use super::source_ml_temporary_find_java_script_class_group_end_loop1;
use super::source_ml_temporary_find_java_script_closed_class_initialization_start_loop5;
use super::source_ml_temporary_read_java_script_closed_class_directory_loop6;
use super::source_ml_temporary_read_java_script_closed_class_directory_loop7;
use super::source_ml_temporary_read_java_script_closed_class_directory_loop9;
use super::source_ml_temporary_read_java_script_closed_class_methods_loop15;
use super::source_ml_temporary_read_java_script_closed_class_observation_error_loop22;
use super::source_ml_temporary_read_java_script_closed_class_observation_error_loop23;
use super::source_ml_temporary_read_java_script_closed_class_parameter_positions_loop2;
use super::source_ml_temporary_read_java_script_closed_class_static_calls_loop17;
use super::source_ml_temporary_read_java_script_closed_class_static_target_loop19;
use super::source_ml_temporary_render_java_script_closed_class_parameter_arguments_loop4;
use super::source_ml_temporary_render_java_script_closed_class_range_loop14;

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
pub fn find_java_script_class_group_end(values: &[self::ml_string::Text], open: f64) -> f64 {
    {
        let first = (self::ml_array::optional_at(values, self::ml_array::number_index(open)))
            .unwrap_or_else(|| self::ml_string::Text::from_units(vec![]));
        {
            let last = if first == (self::ml_string::Text::from_units(vec![40])) {
                self::ml_string::Text::from_units(vec![41])
            } else if first == (self::ml_string::Text::from_units(vec![91])) {
                self::ml_string::Text::from_units(vec![93])
            } else if first == (self::ml_string::Text::from_units(vec![123])) {
                self::ml_string::Text::from_units(vec![125])
            } else {
                self::ml_string::Text::from_units(vec![])
            };
            if last == (self::ml_string::Text::from_units(vec![])) {
                -1f64
            } else {
                {
                    let expected = vec![last.clone()];
                    {
                        let at = open + 1f64;
                        {
                            let source_ml_source1_s1 =
                                self::source_ml_temporary_find_java_script_class_group_end_loop1(
                                    values, open, expected, at,
                                );
                            match source_ml_source1_s1 {
                                self::MlTemporaryFindJavaScriptClassGroupEndLoop1Result::MlTemporaryFindJavaScriptClassGroupEndLoop1Done => {
                                    -1f64
                                }
                                self::MlTemporaryFindJavaScriptClassGroupEndLoop1Result::MlTemporaryFindJavaScriptClassGroupEndLoop1Return(source_ml_temporary_result1) => {
                                    source_ml_temporary_result1
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
pub fn read_java_script_closed_class_parameter_positions(
    values: &[self::ml_string::Text],
    kinds: &[self::ml_string::Text],
    open: f64,
    close: f64,
) -> Vec<f64> {
    {
        let result = Vec::<f64>::new();
        {
            let at = open + 1f64;
            {
                let source_ml_source1_s2 = self::source_ml_temporary_read_java_script_closed_class_parameter_positions_loop2(values, kinds, close, result, at);
                match source_ml_source1_s2 {
                    self::MlTemporaryReadJavaScriptClosedClassParameterPositionsLoop2Result::MlTemporaryReadJavaScriptClosedClassParameterPositionsLoop2Done(result_2) => {
                        result_2
                    }
                    self::MlTemporaryReadJavaScriptClosedClassParameterPositionsLoop2Result::MlTemporaryReadJavaScriptClosedClassParameterPositionsLoop2Return(source_ml_temporary_result2) => {
                        source_ml_temporary_result2
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_java_script_closed_class_parameter_arguments(
    source: &self::ml_string::Text,
    values: &[self::ml_string::Text],
    starts: &[f64],
    ends: &[f64],
    open: f64,
    close: f64,
) -> self::ml_string::Text {
    {
        let result = Vec::<self::ml_string::Text>::new();
        {
            let at = open + 1f64;
            {
                let result_2 = self::source_ml_temporary_render_java_script_closed_class_parameter_arguments_loop4(source, values, starts, ends, close, result, at);
                self::ml_string::join(&result_2, &self::ml_string::Text::from_units(vec![44, 32]))
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn find_java_script_closed_class_initialization_start(
    values: &[self::ml_string::Text],
    first: f64,
    last: f64,
) -> f64 {
    {
        let depth = 0f64;
        {
            let at = first;
            {
                let source_ml_source1_s3 = self::source_ml_temporary_find_java_script_closed_class_initialization_start_loop5(values, first, last, depth, at);
                match source_ml_source1_s3 {
                    self::MlTemporaryFindJavaScriptClosedClassInitializationStartLoop5Result::MlTemporaryFindJavaScriptClosedClassInitializationStartLoop5Done => {
                        last
                    }
                    self::MlTemporaryFindJavaScriptClosedClassInitializationStartLoop5Result::MlTemporaryFindJavaScriptClosedClassInitializationStartLoop5Return(source_ml_temporary_result5) => {
                        source_ml_temporary_result5
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_closed_class_directory(
    values: &[self::ml_string::Text],
    kinds: &[self::ml_string::Text],
    first: f64,
) -> Vec<f64> {
    if ((((self::ml_array::optional_at(values, self::ml_array::number_index(first)))
        .unwrap_or_else(|| self::ml_string::Text::from_units(vec![])))
        != (self::ml_string::Text::from_units(vec![99, 108, 97, 115, 115])))
        || (((self::ml_array::optional_at(kinds, self::ml_array::number_index(first + 1f64)))
            .unwrap_or_else(|| self::ml_string::Text::from_units(vec![])))
            != (self::ml_string::Text::from_units(vec![
                105, 100, 101, 110, 116, 105, 102, 105, 101, 114,
            ]))))
        || (((self::ml_array::optional_at(values, self::ml_array::number_index(first + 2f64)))
            .unwrap_or_else(|| self::ml_string::Text::from_units(vec![])))
            != (self::ml_string::Text::from_units(vec![123])))
    {
        vec![-1f64, first]
    } else {
        {
            let end = self::find_java_script_class_group_end(values, (first + 2f64));
            if end < 0f64 {
                vec![-1f64, (first + 2f64)]
            } else {
                {
                    let constructor = vec![-1f64, -1f64, -1f64, -1f64, -1f64];
                    {
                        let methods = Vec::<f64>::new();
                        {
                            let fields = Vec::<f64>::new();
                            {
                                let names = Vec::<self::ml_string::Text>::new();
                                {
                                    let at = first + 3f64;
                                    {
                                        let source_ml_source1_s4 = self::source_ml_temporary_read_java_script_closed_class_directory_loop6(values, kinds, end, constructor, methods, names, at);
                                        match source_ml_source1_s4 {
                                            self::MlTemporaryReadJavaScriptClosedClassDirectoryLoop6Result::MlTemporaryReadJavaScriptClosedClassDirectoryLoop6Done(constructor_2, methods_2, names_2) => {
                                                {
                                                    let body_start = (self::ml_array::optional_at(&constructor_2, self::ml_array::number_index(3f64))).unwrap_or_else(|| -1f64);
                                                    {
                                                        let body_end = (self::ml_array::optional_at(&constructor_2, self::ml_array::number_index(4f64))).unwrap_or_else(|| -1f64);
                                                        {
                                                            let initialization = if body_start < 0f64 {
                                                                -1f64
                                                            } else {
                                                                self::find_java_script_closed_class_initialization_start(values, (body_start + 1f64), body_end)
                                                            };
                                                            {
                                                                let cursor = initialization;
                                                                {
                                                                    let source_ml_source1_s5 = self::source_ml_temporary_read_java_script_closed_class_directory_loop7(values, kinds, fields, &(names_2), body_start, body_end, cursor);
                                                                    match source_ml_source1_s5 {
                                                                        self::MlTemporaryReadJavaScriptClosedClassDirectoryLoop7Result::MlTemporaryReadJavaScriptClosedClassDirectoryLoop7Done(fields_2) => {
                                                                            {
                                                                                let source_ml_source1_s7 = if body_start >= 0f64 {
                                                                                    {
                                                                                        let at_2 = body_start + 1f64;
                                                                                        {
                                                                                            let source_ml_source1_s8 = self::source_ml_temporary_read_java_script_closed_class_directory_loop9(values, body_end, at_2);
                                                                                            match source_ml_source1_s8 {
                                                                                                self::MlTemporaryReadJavaScriptClosedClassDirectoryLoop9Result::MlTemporaryReadJavaScriptClosedClassDirectoryLoop9Done => {
                                                                                                    self::MlTemporaryReadJavaScriptClosedClassDirectoryJoin8::MlTemporaryReadJavaScriptClosedClassDirectoryJoin8Next
                                                                                                }
                                                                                                self::MlTemporaryReadJavaScriptClosedClassDirectoryLoop9Result::MlTemporaryReadJavaScriptClosedClassDirectoryLoop9Return(source_ml_temporary_result9) => {
                                                                                                    self::MlTemporaryReadJavaScriptClosedClassDirectoryJoin8::MlTemporaryReadJavaScriptClosedClassDirectoryJoin8Return(source_ml_temporary_result9)
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                } else {
                                                                                    self::MlTemporaryReadJavaScriptClosedClassDirectoryJoin8::MlTemporaryReadJavaScriptClosedClassDirectoryJoin8Next
                                                                                };
                                                                                match source_ml_source1_s7 {
                                                                                    self::MlTemporaryReadJavaScriptClosedClassDirectoryJoin8::MlTemporaryReadJavaScriptClosedClassDirectoryJoin8Next => {
                                                                                        self::ml_array::append(self::ml_array::append(self::ml_array::append(self::ml_array::append(vec![first, end], constructor_2), vec![((fields_2.len() as f64) / 3f64), initialization]), fields_2), methods_2)
                                                                                    }
                                                                                    self::MlTemporaryReadJavaScriptClosedClassDirectoryJoin8::MlTemporaryReadJavaScriptClosedClassDirectoryJoin8Return(source_ml_temporary_result8) => {
                                                                                        source_ml_temporary_result8
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                        self::MlTemporaryReadJavaScriptClosedClassDirectoryLoop7Result::MlTemporaryReadJavaScriptClosedClassDirectoryLoop7Return(source_ml_temporary_result9_2) => {
                                                                            source_ml_temporary_result9_2
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            self::MlTemporaryReadJavaScriptClosedClassDirectoryLoop6Result::MlTemporaryReadJavaScriptClosedClassDirectoryLoop6Return(source_ml_temporary_result12) => {
                                                source_ml_temporary_result12
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_closed_class_refusal(code: f64) -> self::ml_string::Text {
    if code == -1f64 {
        self::ml_string::Text::from_units(vec![
            97, 32, 99, 108, 111, 115, 101, 100, 32, 99, 108, 97, 115, 115, 32, 110, 101, 101, 100,
            115, 32, 97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 110, 97, 109, 101,
            100, 32, 98, 97, 115, 101, 45, 99, 108, 97, 115, 115, 32, 100, 101, 99, 108, 97, 114,
            97, 116, 105, 111, 110, 32, 97, 110, 100, 32, 98, 97, 108, 97, 110, 99, 101, 100, 32,
            115, 111, 117, 114, 99, 101, 32, 103, 114, 111, 117, 112, 115,
        ])
    } else if code == -2f64 {
        self::ml_string::Text::from_units(vec![
            99, 108, 97, 115, 115, 32, 102, 105, 101, 108, 100, 115, 44, 32, 97, 99, 99, 101, 115,
            115, 111, 114, 115, 44, 32, 112, 114, 105, 118, 97, 116, 101, 32, 110, 97, 109, 101,
            115, 44, 32, 105, 110, 104, 101, 114, 105, 116, 97, 110, 99, 101, 44, 32, 115, 116, 97,
            116, 105, 99, 32, 105, 110, 105, 116, 105, 97, 108, 105, 122, 97, 116, 105, 111, 110,
            32, 97, 110, 100, 32, 100, 117, 112, 108, 105, 99, 97, 116, 101, 32, 109, 101, 109, 98,
            101, 114, 32, 111, 119, 110, 101, 114, 115, 32, 110, 101, 101, 100, 32, 116, 104, 101,
            105, 114, 32, 102, 117, 108, 108, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114,
            32, 109, 111, 100, 101, 108,
        ])
    } else if code == -3f64 {
        self::ml_string::Text::from_units(vec![
            116, 104, 105, 115, 32, 99, 108, 97, 115, 115, 32, 112, 114, 111, 102, 105, 108, 101,
            32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 111, 114, 100, 105, 110, 97, 114, 121,
            32, 102, 105, 120, 101, 100, 32, 105, 100, 101, 110, 116, 105, 102, 105, 101, 114, 32,
            112, 97, 114, 97, 109, 101, 116, 101, 114, 115, 32, 119, 105, 116, 104, 32, 108, 105,
            116, 101, 114, 97, 108, 32, 100, 101, 102, 97, 117, 108, 116, 115, 59, 32, 112, 97,
            116, 116, 101, 114, 110, 115, 32, 97, 110, 100, 32, 100, 101, 112, 101, 110, 100, 101,
            110, 116, 32, 111, 114, 32, 105, 110, 115, 116, 97, 110, 99, 101, 45, 114, 101, 97,
            100, 105, 110, 103, 32, 100, 101, 102, 97, 117, 108, 116, 115, 32, 110, 101, 101, 100,
            32, 116, 104, 101, 105, 114, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 112, 97,
            114, 97, 109, 101, 116, 101, 114, 32, 101, 110, 118, 105, 114, 111, 110, 109, 101, 110,
            116,
        ])
    } else if code == -4f64 {
        self::ml_string::Text::from_units(vec![
            105, 110, 105, 116, 105, 97, 108, 32, 111, 119, 110, 32, 102, 105, 101, 108, 100, 115,
            32, 109, 117, 115, 116, 32, 98, 101, 32, 100, 105, 115, 116, 105, 110, 99, 116, 32, 97,
            110, 100, 32, 99, 97, 110, 110, 111, 116, 32, 114, 101, 112, 108, 97, 99, 101, 32, 97,
            32, 112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 109, 101, 116, 104, 111, 100,
        ])
    } else if code == -5f64 {
        self::ml_string::Text::from_units(vec![
            105, 110, 105, 116, 105, 97, 108, 32, 111, 119, 110, 32, 102, 105, 101, 108, 100, 115,
            32, 110, 101, 101, 100, 32, 97, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 115,
            101, 109, 105, 99, 111, 108, 111, 110, 45, 116, 101, 114, 109, 105, 110, 97, 116, 101,
            100, 32, 97, 115, 115, 105, 103, 110, 109, 101, 110, 116, 32, 112, 114, 101, 102, 105,
            120, 32, 119, 105, 116, 104, 111, 117, 116, 32, 105, 110, 115, 116, 97, 110, 99, 101,
            32, 101, 115, 99, 97, 112, 101, 32, 111, 114, 32, 114, 101, 97, 100, 115, 32, 98, 101,
            102, 111, 114, 101, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110,
        ])
    } else if code == -6f64 {
        self::ml_string::Text::from_units(vec![
            99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 114, 101, 116, 117, 114, 110,
            32, 111, 118, 101, 114, 114, 105, 100, 101, 115, 32, 110, 101, 101, 100, 32, 116, 104,
            101, 105, 114, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 111, 98, 106, 101, 99,
            116, 32, 114, 101, 112, 108, 97, 99, 101, 109, 101, 110, 116, 32, 115, 101, 109, 97,
            110, 116, 105, 99, 115,
        ])
    } else if code == -7f64 {
        self::ml_string::Text::from_units(vec![
            99, 108, 97, 115, 115, 32, 118, 97, 108, 117, 101, 115, 44, 32, 114, 101, 102, 108,
            101, 99, 116, 105, 118, 101, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 115, 44,
            32, 100, 101, 116, 97, 99, 104, 101, 100, 32, 109, 101, 116, 104, 111, 100, 115, 44,
            32, 109, 101, 116, 104, 111, 100, 32, 114, 101, 112, 108, 97, 99, 101, 109, 101, 110,
            116, 32, 97, 110, 100, 32, 111, 98, 115, 101, 114, 118, 97, 98, 108, 101, 32, 112, 114,
            111, 116, 111, 116, 121, 112, 101, 115, 32, 110, 101, 101, 100, 32, 116, 104, 101, 32,
            102, 117, 108, 108, 32, 111, 98, 106, 101, 99, 116, 32, 109, 111, 100, 101, 108,
        ])
    } else if code == -8f64 {
        self::ml_string::Text::from_units(vec![
            99, 108, 111, 115, 101, 100, 32, 99, 108, 97, 115, 115, 32, 97, 117, 116, 104, 111,
            114, 105, 110, 103, 32, 101, 120, 99, 101, 101, 100, 115, 32, 105, 116, 115, 32, 101,
            120, 112, 108, 105, 99, 105, 116, 32, 115, 111, 117, 114, 99, 101, 44, 32, 116, 111,
            107, 101, 110, 44, 32, 110, 101, 115, 116, 105, 110, 103, 32, 111, 114, 32, 99, 108,
            97, 115, 115, 45, 99, 111, 117, 110, 116, 32, 98, 117, 100, 103, 101, 116,
        ])
    } else if code == -9f64 {
        self::ml_string::Text::from_units(vec![
            99, 108, 97, 115, 115, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110,
            44, 32, 114, 101, 99, 101, 105, 118, 101, 114, 32, 97, 110, 100, 32, 109, 101, 109, 98,
            101, 114, 32, 109, 111, 100, 105, 102, 105, 101, 114, 32, 107, 101, 121, 119, 111, 114,
            100, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97,
            99, 116, 117, 97, 108, 32, 117, 110, 101, 115, 99, 97, 112, 101, 100, 32, 69, 67, 77,
            65, 83, 99, 114, 105, 112, 116, 32, 115, 111, 117, 114, 99, 101, 32, 115, 112, 101,
            108, 108, 105, 110, 103,
        ])
    } else {
        self::ml_string::Text::from_units(vec![])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn render_java_script_closed_class_range(
    source: &self::ml_string::Text,
    values: &[self::ml_string::Text],
    starts: &[f64],
    ends: &[f64],
    first: f64,
    last: f64,
    receiver: &self::ml_string::Text,
    class_names: &[self::ml_string::Text],
    static_calls: &[self::ml_string::Text],
) -> self::ml_string::Text {
    if first >= last {
        self::ml_string::Text::from_units(vec![])
    } else {
        {
            let result = self::ml_string::Text::from_units(vec![]);
            {
                let previous =
                    (self::ml_array::optional_at(starts, self::ml_array::number_index(first)))
                        .unwrap_or_else(|| 0f64);
                {
                    let at = first;
                    {
                        let source_ml_source1_s9 =
                            self::source_ml_temporary_render_java_script_closed_class_range_loop14(
                                source,
                                values,
                                starts,
                                ends,
                                last,
                                receiver,
                                class_names,
                                static_calls,
                                result,
                                previous,
                                at,
                            );
                        match source_ml_source1_s9 {
                            self::MlTemporaryRenderJavaScriptClosedClassRangeLoop14Result::MlTemporaryRenderJavaScriptClosedClassRangeLoop14Done(result_2, previous_2) => {
                                result_2.concat(&{
                                    let source_ml_source1_string_range_operand10 = source.clone();
                                    {
                                        let source_ml_source1_string_range_operand11 = previous_2;
                                        {
                                            let source_ml_source1_string_range_operand12 = (self::ml_array::optional_at(ends, self::ml_array::number_index(last - 1f64))).unwrap_or_else(|| previous_2);
                                            {
                                                let source_ml_source1_string_range_position13 = source_ml_source1_string_range_operand11;
                                                {
                                                    let source_ml_source1_string_range_position14 = source_ml_source1_string_range_operand12;
                                                    source_ml_source1_string_range_operand10.slice(source_ml_source1_string_range_position13, source_ml_source1_string_range_position14)
                                                }
                                            }
                                        }
                                    }
                                })
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
pub fn read_java_script_closed_class_methods(
    values: &[self::ml_string::Text],
    directory: &[f64],
) -> Vec<self::ml_string::Text> {
    {
        let result = Vec::<self::ml_string::Text>::new();
        {
            let first = 9f64
                + (3f64
                    * (self::ml_array::optional_at(directory, self::ml_array::number_index(7f64)))
                        .unwrap_or_else(|| 0f64));
            {
                let at = first;
                {
                    self::source_ml_temporary_read_java_script_closed_class_methods_loop15(
                        values, directory, result, at,
                    )
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_closed_class_static_calls(
    values: &[self::ml_string::Text],
    directory: &[Vec<f64>],
    prefix: &self::ml_string::Text,
) -> Vec<self::ml_string::Text> {
    {
        let result = Vec::<self::ml_string::Text>::new();
        {
            let ordinal = 0f64;
            {
                let source_ml_temporary_values16 = directory.to_vec();
                {
                    let source_ml_temporary_index16 = self::ml::Big::from_i128(0);
                    {
                        self::source_ml_temporary_read_java_script_closed_class_static_calls_loop17(
                            values,
                            prefix,
                            result,
                            ordinal,
                            &(source_ml_temporary_values16),
                            source_ml_temporary_index16,
                        )
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_closed_class_static_target(
    values: &[self::ml_string::Text],
    names: &[self::ml_string::Text],
    calls: &[self::ml_string::Text],
    at: f64,
) -> self::ml_string::Text {
    {
        let previous =
            (self::ml_array::optional_at(values, self::ml_array::number_index(at - 1f64)))
                .unwrap_or_else(|| self::ml_string::Text::from_units(vec![]));
        if ((((!(self::ml_array::includes(
            names,
            &(self::ml_array::optional_at(values, self::ml_array::number_index(at)))
                .unwrap_or_else(|| self::ml_string::Text::from_units(vec![])),
            0f64,
            false,
            |a, b| a == b,
        )) || (previous == (self::ml_string::Text::from_units(vec![110, 101, 119]))))
            || (previous == (self::ml_string::Text::from_units(vec![46]))))
            || (previous == (self::ml_string::Text::from_units(vec![63, 46]))))
            || (((self::ml_array::optional_at(values, self::ml_array::number_index(at + 1f64)))
                .unwrap_or_else(|| self::ml_string::Text::from_units(vec![])))
                != (self::ml_string::Text::from_units(vec![46]))))
            || (((self::ml_array::optional_at(values, self::ml_array::number_index(at + 3f64)))
                .unwrap_or_else(|| self::ml_string::Text::from_units(vec![])))
                != (self::ml_string::Text::from_units(vec![40])))
        {
            self::ml_string::Text::from_units(vec![])
        } else {
            {
                let key =
                    (((self::ml_array::optional_at(values, self::ml_array::number_index(at)))
                        .unwrap_or_else(|| self::ml_string::Text::from_units(vec![])))
                    .concat(&self::ml_string::Text::from_units(vec![46])))
                    .concat(
                        &(self::ml_array::optional_at(
                            values,
                            self::ml_array::number_index(at + 2f64),
                        ))
                        .unwrap_or_else(|| self::ml_string::Text::from_units(vec![])),
                    );
                {
                    let index = 0f64;
                    {
                        let source_ml_source1_s15 = self::source_ml_temporary_read_java_script_closed_class_static_target_loop19(calls, &(key), index);
                        match source_ml_source1_s15 {
                            self::MlTemporaryReadJavaScriptClosedClassStaticTargetLoop19Result::MlTemporaryReadJavaScriptClosedClassStaticTargetLoop19Done => {
                                self::ml_string::Text::from_units(vec![])
                            }
                            self::MlTemporaryReadJavaScriptClosedClassStaticTargetLoop19Result::MlTemporaryReadJavaScriptClosedClassStaticTargetLoop19Return(source_ml_temporary_result19) => {
                                source_ml_temporary_result19.clone()
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
pub fn read_java_script_closed_class_observation_error(
    values: &[self::ml_string::Text],
    kinds: &[self::ml_string::Text],
    directory: &[Vec<f64>],
    static_calls: &[self::ml_string::Text],
) -> f64 {
    {
        let names = Vec::<self::ml_string::Text>::new();
        {
            let methods = Vec::<self::ml_string::Text>::new();
            {
                let source_ml_temporary_values20 = directory.to_vec();
                {
                    let source_ml_temporary_index20 = self::ml::Big::from_i128(0);
                    {
                        let source_ml_source1_s16 = self::source_ml_temporary_read_java_script_closed_class_observation_error_loop22(values, names, methods, &(source_ml_temporary_values20), source_ml_temporary_index20);
                        match source_ml_source1_s16 {
                            self::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop22Result::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop22Done(names_2, methods_2) => {
                                {
                                    let at = 0f64;
                                    {
                                        let source_ml_source1_s17 = self::source_ml_temporary_read_java_script_closed_class_observation_error_loop23(values, kinds, directory, static_calls, &(names_2), &(methods_2), at);
                                        match source_ml_source1_s17 {
                                            self::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop23Result::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop23Done => {
                                                -1f64
                                            }
                                            self::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop23Result::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop23Return(source_ml_temporary_result23) => {
                                                source_ml_temporary_result23
                                            }
                                        }
                                    }
                                }
                            }
                            self::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop22Result::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop22Return(source_ml_temporary_result25) => {
                                source_ml_temporary_result25
                            }
                        }
                    }
                }
            }
        }
    }
}
