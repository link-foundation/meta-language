// Generated from complete js/src/translation/rust-record-rules.js. Do not edit by hand.
use super::MlSource1TemporaryAcceptRustNamedConstructorFieldsLoop11Result;
use super::MlSource1TemporaryAcceptRustRawIdentifierNameLoop9Result;
use super::MlSource1TemporaryPlanRustRecordFieldsLoop1Result;
use super::MlSource1TemporaryPlanRustRecordFieldsLoop2Result;
use super::MlSource1TemporaryReadRustLetElseInitializerErrorLoop6Result;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::source_ml_source1_temporary_accept_rust_named_constructor_fields_loop11;
use super::source_ml_source1_temporary_accept_rust_raw_identifier_name_loop9;
use super::source_ml_source1_temporary_plan_rust_record_fields_loop1;
use super::source_ml_source1_temporary_plan_rust_record_fields_loop2;
use super::source_ml_source1_temporary_plan_rust_record_fields_loop3;
use super::source_ml_source1_temporary_plan_rust_record_pattern_fields_loop4;
use super::source_ml_source1_temporary_plan_tuple_pattern_fields_loop5;
use super::source_ml_source1_temporary_read_rust_let_else_initializer_error_loop6;
use super::source_ml_source1_temporary_read_rust_record_binding_names_loop7;
use super::source_ml_source1_temporary_read_rust_record_constructor_name_loop8;
use super::source_ml_source1_temporary_render_rust_constructor_fields_loop12;

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
pub fn plan_rust_record_fields(
    declared: &[self::ml_string::Text],
    supplied: &[self::ml_string::Text],
) -> Vec<f64> {
    {
        let index = 0f64;
        {
            let source_ml_source2_s1 =
                self::source_ml_source1_temporary_plan_rust_record_fields_loop1(declared, index);
            match source_ml_source2_s1 {
                self::MlSource1TemporaryPlanRustRecordFieldsLoop1Result::MlSource1TemporaryPlanRustRecordFieldsLoop1Done => {
                    {
                        let index_2 = 0f64;
                        {
                            let source_ml_source2_s2 = self::source_ml_source1_temporary_plan_rust_record_fields_loop2(declared, supplied, index_2);
                            match source_ml_source2_s2 {
                                self::MlSource1TemporaryPlanRustRecordFieldsLoop2Result::MlSource1TemporaryPlanRustRecordFieldsLoop2Done => {
                                    {
                                        let result = Vec::<f64>::new();
                                        {
                                            let index_3 = 0f64;
                                            {
                                                let source_ml_source2_s3 = self::source_ml_source1_temporary_plan_rust_record_fields_loop3(declared, supplied, result, index_3);
                                                match source_ml_source2_s3 {
                                                    self::MlSource1TemporaryPlanRustRecordFieldsLoop3Result::MlSource1TemporaryPlanRustRecordFieldsLoop3Done(result_2) => {
                                                        result_2
                                                    }
                                                    self::MlSource1TemporaryPlanRustRecordFieldsLoop3Result::MlSource1TemporaryPlanRustRecordFieldsLoop3Return(source_ml_source1_temporary_result3) => {
                                                        source_ml_source1_temporary_result3
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                                self::MlSource1TemporaryPlanRustRecordFieldsLoop2Result::MlSource1TemporaryPlanRustRecordFieldsLoop2Return(source_ml_source1_temporary_result3_2) => {
                                    source_ml_source1_temporary_result3_2
                                }
                            }
                        }
                    }
                }
                self::MlSource1TemporaryPlanRustRecordFieldsLoop1Result::MlSource1TemporaryPlanRustRecordFieldsLoop1Return(source_ml_source1_temporary_result3_3) => {
                    source_ml_source1_temporary_result3_3
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_record_field_position(
    declared: &[self::ml_string::Text],
    field: &self::ml_string::Text,
) -> f64 {
    self::ml_array::index_of(declared, field, 0f64, false, |a, b| a == b)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_rust_record_pattern_fields(
    declared: &[self::ml_string::Text],
    supplied: &[self::ml_string::Text],
    rest: bool,
) -> Vec<f64> {
    {
        let checked = self::plan_rust_record_fields(declared, supplied);
        {
            let code = (self::ml_array::optional_at(&checked, self::ml_array::number_index(0f64)))
                .unwrap_or_else(|| 0f64);
            if (code < 0f64) && !((code == -4f64) && rest) {
                checked
            } else {
                {
                    let result = vec![0f64];
                    {
                        let index = 0f64;
                        {
                            self::source_ml_source1_temporary_plan_rust_record_pattern_fields_loop4(
                                declared, supplied, result, index,
                            )
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_rust_record_update_fields(
    declared: &[self::ml_string::Text],
    supplied: &[self::ml_string::Text],
) -> Vec<f64> {
    self::plan_rust_record_pattern_fields(declared, supplied, true)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_record_update_owner(
    structure: bool,
    known: bool,
    actual: &self::ml_string::Text,
    expected: &self::ml_string::Text,
    wrappers: f64,
    diverging: bool,
) -> bool {
    structure && (diverging || ((known && ((*actual) == (*expected))) && (wrappers == 0f64)))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_irrefutable_pattern_node(
    kind: &self::ml_string::Text,
    known: bool,
    variants: f64,
    arity: f64,
    fields: f64,
) -> bool {
    if (((*kind) == (self::ml_string::Text::from_units(vec![119, 105, 108, 100])))
        || ((*kind) == (self::ml_string::Text::from_units(vec![98, 105, 110, 100]))))
        || ((*kind) == (self::ml_string::Text::from_units(vec![116, 117, 112, 108, 101])))
    {
        true
    } else {
        ((((*kind) == (self::ml_string::Text::from_units(vec![99, 116, 111, 114]))) && known)
            && (variants == 1f64))
            && ((arity).partial_cmp(&(fields)) == Some(std::cmp::Ordering::Equal))
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_tuple_pattern_fields(width: f64, count: f64, rest: f64) -> Vec<f64> {
    if (((((((!(self::ml_math::is_safe_integer(width))
        || !(self::ml_math::is_safe_integer(count)))
        || !(self::ml_math::is_safe_integer(rest)))
        || (width < 0f64))
        || (count < 0f64))
        || (rest < -1f64))
        || (rest > count))
        || (count > width))
        || ((rest < 0f64) && (count != width))
    {
        vec![-1f64]
    } else {
        {
            let result = vec![0f64];
            {
                let index = 0f64;
                {
                    self::source_ml_source1_temporary_plan_tuple_pattern_fields_loop5(
                        width, count, rest, result, index,
                    )
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_let_else_initializer_error(
    kinds: &[self::ml_string::Text],
    spellings: &[self::ml_string::Text],
) -> self::ml_string::Text {
    {
        let ml_library_inequality_1 = (kinds.len() as f64) != (spellings.len() as f64);
        if ml_library_inequality_1 {
            self::ml_string::Text::from_units(vec![
                105, 110, 105, 116, 105, 97, 108, 105, 122, 101, 114, 32, 116, 111, 107, 101, 110,
                32, 99, 111, 108, 117, 109, 110, 115, 32, 104, 97, 118, 101, 32, 100, 105, 102,
                102, 101, 114, 101, 110, 116, 32, 108, 101, 110, 103, 116, 104, 115,
            ])
        } else {
            {
                let depth = 0f64;
                {
                    let index = 0f64;
                    {
                        let source_ml_source2_s4 = self::source_ml_source1_temporary_read_rust_let_else_initializer_error_loop6(kinds, spellings, depth, index);
                        match source_ml_source2_s4 {
                        self::MlSource1TemporaryReadRustLetElseInitializerErrorLoop6Result::MlSource1TemporaryReadRustLetElseInitializerErrorLoop6Done(depth_2) => {
                            if depth_2 == 0f64 {
                                self::ml_string::Text::from_units(vec![])
                            } else {
                                self::ml_string::Text::from_units(vec![105, 110, 105, 116, 105, 97, 108, 105, 122, 101, 114, 32, 100, 101, 108, 105, 109, 105, 116, 101, 114, 115, 32, 97, 114, 101, 32, 117, 110, 98, 97, 108, 97, 110, 99, 101, 100])
                            }
                        }
                        self::MlSource1TemporaryReadRustLetElseInitializerErrorLoop6Result::MlSource1TemporaryReadRustLetElseInitializerErrorLoop6Return(source_ml_source1_temporary_result6) => {
                            source_ml_source1_temporary_result6
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
pub fn read_rust_record_binding_names(
    reserved: &[self::ml_string::Text],
    count: f64,
) -> Vec<self::ml_string::Text> {
    {
        let names = Vec::<self::ml_string::Text>::new();
        {
            let suffix = 0f64;
            {
                self::source_ml_source1_temporary_read_rust_record_binding_names_loop7(
                    reserved, count, names, suffix,
                )
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_record_constructor_name(
    name: &self::ml_string::Text,
    collision: bool,
    reserved: &[self::ml_string::Text],
) -> self::ml_string::Text {
    if collision {
        {
            let candidate = self::ml_string::Text::from_units(vec![86, 97, 108, 117, 101]);
            {
                let suffix = 2f64;
                {
                    self::source_ml_source1_temporary_read_rust_record_constructor_name_loop8(
                        reserved, candidate, suffix,
                    )
                }
            }
        }
    } else {
        name.clone()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_record_source_name(name: &self::ml_string::Text) -> bool {
    !(self::ml_array::includes(
        &vec![
            self::ml_string::Text::from_units(vec![98, 111, 111, 108]),
            self::ml_string::Text::from_units(vec![99, 104, 97, 114]),
            self::ml_string::Text::from_units(vec![115, 116, 114]),
            self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103]),
            self::ml_string::Text::from_units(vec![86, 101, 99]),
            self::ml_string::Text::from_units(vec![66, 111, 120]),
            self::ml_string::Text::from_units(vec![102, 51, 50]),
            self::ml_string::Text::from_units(vec![102, 54, 52]),
            self::ml_string::Text::from_units(vec![117, 115, 105, 122, 101]),
            self::ml_string::Text::from_units(vec![105, 115, 105, 122, 101]),
            self::ml_string::Text::from_units(vec![117, 56]),
            self::ml_string::Text::from_units(vec![105, 56]),
            self::ml_string::Text::from_units(vec![117, 49, 54]),
            self::ml_string::Text::from_units(vec![105, 49, 54]),
            self::ml_string::Text::from_units(vec![117, 51, 50]),
            self::ml_string::Text::from_units(vec![105, 51, 50]),
            self::ml_string::Text::from_units(vec![117, 54, 52]),
            self::ml_string::Text::from_units(vec![105, 54, 52]),
            self::ml_string::Text::from_units(vec![117, 49, 50, 56]),
            self::ml_string::Text::from_units(vec![105, 49, 50, 56]),
            self::ml_string::Text::from_units(vec![115, 116, 100]),
            self::ml_string::Text::from_units(vec![99, 111, 114, 101]),
            self::ml_string::Text::from_units(vec![97, 108, 108, 111, 99]),
        ],
        name,
        0f64,
        false,
        |a, b| a == b,
    ))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_keyword_name(name: &self::ml_string::Text) -> bool {
    self::ml_array::includes(
        &vec![
            self::ml_string::Text::from_units(vec![95]),
            self::ml_string::Text::from_units(vec![97, 115]),
            self::ml_string::Text::from_units(vec![97, 115, 121, 110, 99]),
            self::ml_string::Text::from_units(vec![97, 119, 97, 105, 116]),
            self::ml_string::Text::from_units(vec![98, 114, 101, 97, 107]),
            self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116]),
            self::ml_string::Text::from_units(vec![99, 111, 110, 116, 105, 110, 117, 101]),
            self::ml_string::Text::from_units(vec![99, 114, 97, 116, 101]),
            self::ml_string::Text::from_units(vec![100, 121, 110]),
            self::ml_string::Text::from_units(vec![101, 108, 115, 101]),
            self::ml_string::Text::from_units(vec![101, 110, 117, 109]),
            self::ml_string::Text::from_units(vec![101, 120, 116, 101, 114, 110]),
            self::ml_string::Text::from_units(vec![102, 97, 108, 115, 101]),
            self::ml_string::Text::from_units(vec![102, 110]),
            self::ml_string::Text::from_units(vec![102, 111, 114]),
            self::ml_string::Text::from_units(vec![105, 102]),
            self::ml_string::Text::from_units(vec![105, 109, 112, 108]),
            self::ml_string::Text::from_units(vec![105, 110]),
            self::ml_string::Text::from_units(vec![108, 101, 116]),
            self::ml_string::Text::from_units(vec![108, 111, 111, 112]),
            self::ml_string::Text::from_units(vec![109, 97, 116, 99, 104]),
            self::ml_string::Text::from_units(vec![109, 111, 100]),
            self::ml_string::Text::from_units(vec![109, 111, 118, 101]),
            self::ml_string::Text::from_units(vec![109, 117, 116]),
            self::ml_string::Text::from_units(vec![112, 117, 98]),
            self::ml_string::Text::from_units(vec![114, 101, 102]),
            self::ml_string::Text::from_units(vec![114, 101, 116, 117, 114, 110]),
            self::ml_string::Text::from_units(vec![115, 101, 108, 102]),
            self::ml_string::Text::from_units(vec![83, 101, 108, 102]),
            self::ml_string::Text::from_units(vec![115, 116, 97, 116, 105, 99]),
            self::ml_string::Text::from_units(vec![115, 116, 114, 117, 99, 116]),
            self::ml_string::Text::from_units(vec![115, 117, 112, 101, 114]),
            self::ml_string::Text::from_units(vec![116, 114, 97, 105, 116]),
            self::ml_string::Text::from_units(vec![116, 114, 117, 101]),
            self::ml_string::Text::from_units(vec![116, 121, 112, 101]),
            self::ml_string::Text::from_units(vec![117, 110, 115, 97, 102, 101]),
            self::ml_string::Text::from_units(vec![117, 115, 101]),
            self::ml_string::Text::from_units(vec![119, 104, 101, 114, 101]),
            self::ml_string::Text::from_units(vec![119, 104, 105, 108, 101]),
            self::ml_string::Text::from_units(vec![97, 98, 115, 116, 114, 97, 99, 116]),
            self::ml_string::Text::from_units(vec![98, 101, 99, 111, 109, 101]),
            self::ml_string::Text::from_units(vec![98, 111, 120]),
            self::ml_string::Text::from_units(vec![100, 111]),
            self::ml_string::Text::from_units(vec![102, 105, 110, 97, 108]),
            self::ml_string::Text::from_units(vec![103, 101, 110]),
            self::ml_string::Text::from_units(vec![109, 97, 99, 114, 111]),
            self::ml_string::Text::from_units(vec![111, 118, 101, 114, 114, 105, 100, 101]),
            self::ml_string::Text::from_units(vec![112, 114, 105, 118]),
            self::ml_string::Text::from_units(vec![116, 114, 121]),
            self::ml_string::Text::from_units(vec![116, 121, 112, 101, 111, 102]),
            self::ml_string::Text::from_units(vec![117, 110, 115, 105, 122, 101, 100]),
            self::ml_string::Text::from_units(vec![118, 105, 114, 116, 117, 97, 108]),
            self::ml_string::Text::from_units(vec![121, 105, 101, 108, 100]),
        ],
        name,
        0f64,
        false,
        |a, b| a == b,
    )
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_raw_identifier_name(name: &self::ml_string::Text) -> bool {
    if ((name.length()) == 0f64)
        || self::ml_array::includes(
            &vec![
                self::ml_string::Text::from_units(vec![95]),
                self::ml_string::Text::from_units(vec![99, 114, 97, 116, 101]),
                self::ml_string::Text::from_units(vec![115, 101, 108, 102]),
                self::ml_string::Text::from_units(vec![83, 101, 108, 102]),
                self::ml_string::Text::from_units(vec![115, 117, 112, 101, 114]),
            ],
            name,
            0f64,
            false,
            |a, b| a == b,
        )
    {
        false
    } else {
        {
            let index = 0f64;
            {
                let source_ml_source2_s5 =
                    self::source_ml_source1_temporary_accept_rust_raw_identifier_name_loop9(
                        name, index,
                    );
                match source_ml_source2_s5 {
                    self::MlSource1TemporaryAcceptRustRawIdentifierNameLoop9Result::MlSource1TemporaryAcceptRustRawIdentifierNameLoop9Done => {
                        true
                    }
                    self::MlSource1TemporaryAcceptRustRawIdentifierNameLoop9Result::MlSource1TemporaryAcceptRustRawIdentifierNameLoop9Return(source_ml_source1_temporary_result9) => {
                        source_ml_source1_temporary_result9
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_source_keyword_token(
    name: &self::ml_string::Text,
    raw_: &self::ml_string::Text,
) -> bool {
    !(raw_.starts_with(&self::ml_string::Text::from_units(vec![114, 35])))
        || !(self::accept_rust_keyword_name(name))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_raw_identifier_error(name: &self::ml_string::Text) -> self::ml_string::Text {
    if self::accept_rust_raw_identifier_name(name) {
        self::ml_string::Text::from_units(vec![])
    } else {
        (self::ml_string::Text::from_units(vec![
            105, 110, 118, 97, 108, 105, 100, 32, 82, 117, 115, 116, 32, 114, 97, 119, 32, 105,
            100, 101, 110, 116, 105, 102, 105, 101, 114, 32, 114, 35,
        ]))
        .concat(name)
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_rust_constructor_field_name(name: &self::ml_string::Text) -> self::ml_string::Text {
    if self::accept_rust_keyword_name(name) {
        (self::ml_string::Text::from_units(vec![114, 35])).concat(name)
    } else {
        name.clone()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_named_constructor_fields(
    language: &self::ml_string::Text,
    names: &[self::ml_string::Text],
) -> bool {
    if ((*language) != (self::ml_string::Text::from_units(vec![82, 117, 115, 116])))
        || ((names.len() as f64) == 0f64)
    {
        false
    } else {
        {
            let source_ml_source1_temporary_values10 = names.to_vec();
            {
                let source_ml_source1_temporary_index10 = self::ml::Big::from_i128(0);
                {
                    let source_ml_source2_s6 = self::source_ml_source1_temporary_accept_rust_named_constructor_fields_loop11(&(source_ml_source1_temporary_values10), source_ml_source1_temporary_index10);
                    match source_ml_source2_s6 {
                        self::MlSource1TemporaryAcceptRustNamedConstructorFieldsLoop11Result::MlSource1TemporaryAcceptRustNamedConstructorFieldsLoop11Done => {
                            true
                        }
                        self::MlSource1TemporaryAcceptRustNamedConstructorFieldsLoop11Result::MlSource1TemporaryAcceptRustNamedConstructorFieldsLoop11Return(source_ml_source1_temporary_result11) => {
                            source_ml_source1_temporary_result11
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_rust_constructor_fields(
    language: &self::ml_string::Text,
    head: &self::ml_string::Text,
    names: &[self::ml_string::Text],
    values: &[self::ml_string::Text],
) -> self::ml_string::Text {
    if (values.len() as f64) == 0f64 {
        head.clone()
    } else if (self::accept_rust_named_constructor_fields(language, names)) {
        {
            let fields = Vec::<self::ml_string::Text>::new();
            {
                let index = 0f64;
                {
                    let fields_2 =
                        self::source_ml_source1_temporary_render_rust_constructor_fields_loop12(
                            names, values, fields, index,
                        );
                    ((head.concat(&self::ml_string::Text::from_units(vec![32, 123, 32]))).concat(
                        &self::ml_string::join(
                            &fields_2,
                            &self::ml_string::Text::from_units(vec![44, 32]),
                        ),
                    ))
                    .concat(&self::ml_string::Text::from_units(vec![32, 125]))
                }
            }
        }
    } else {
        ((head.concat(&self::ml_string::Text::from_units(vec![40]))).concat(
            &self::ml_string::join(values, &self::ml_string::Text::from_units(vec![44, 32])),
        ))
        .concat(&self::ml_string::Text::from_units(vec![41]))
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_rust_function_call(
    head: &self::ml_string::Text,
    values: &[self::ml_string::Text],
) -> self::ml_string::Text {
    ((head.concat(&self::ml_string::Text::from_units(vec![40]))).concat(&self::ml_string::join(
        values,
        &self::ml_string::Text::from_units(vec![44, 32]),
    )))
    .concat(&self::ml_string::Text::from_units(vec![41]))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_rust_data_field_encoding(named: bool) -> self::ml_string::Text {
    if named {
        self::ml_string::Text::from_units(vec![
            97, 32, 100, 97, 116, 97, 32, 116, 121, 112, 101, 32, 105, 115, 32, 97, 110, 32, 101,
            110, 117, 109, 32, 119, 104, 111, 115, 101, 32, 99, 111, 110, 115, 116, 114, 117, 99,
            116, 111, 114, 115, 32, 114, 101, 116, 97, 105, 110, 32, 110, 97, 109, 101, 100, 32,
            111, 114, 32, 112, 111, 115, 105, 116, 105, 111, 110, 97, 108, 32, 102, 105, 101, 108,
            100, 115, 59, 32, 100, 97, 116, 97, 45, 116, 121, 112, 101, 100, 32, 102, 105, 101,
            108, 100, 115, 32, 97, 114, 101, 32, 98, 111, 120, 101, 100, 44, 32, 97, 110, 100, 32,
            118, 97, 108, 117, 101, 115, 32, 97, 114, 101, 32, 99, 111, 109, 112, 97, 114, 101,
            100, 32, 115, 116, 114, 117, 99, 116, 117, 114, 97, 108, 108, 121,
        ])
    } else {
        self::ml_string::Text::from_units(vec![
            97, 32, 100, 97, 116, 97, 32, 116, 121, 112, 101, 32, 105, 115, 32, 97, 110, 32, 101,
            110, 117, 109, 32, 119, 105, 116, 104, 32, 111, 110, 101, 32, 116, 117, 112, 108, 101,
            32, 118, 97, 114, 105, 97, 110, 116, 32, 112, 101, 114, 32, 99, 111, 110, 115, 116,
            114, 117, 99, 116, 111, 114, 59, 32, 100, 97, 116, 97, 45, 116, 121, 112, 101, 100, 32,
            102, 105, 101, 108, 100, 115, 32, 97, 114, 101, 32, 98, 111, 120, 101, 100, 44, 32, 97,
            110, 100, 32, 118, 97, 108, 117, 101, 115, 32, 97, 114, 101, 32, 99, 111, 109, 112, 97,
            114, 101, 100, 32, 115, 116, 114, 117, 99, 116, 117, 114, 97, 108, 108, 121,
        ])
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryPlanRustRecordFieldsLoop3Result {
    MlSource1TemporaryPlanRustRecordFieldsLoop3Done(Vec<f64>),
    MlSource1TemporaryPlanRustRecordFieldsLoop3Return(Vec<f64>),
}
