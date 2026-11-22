// Generated from complete js/src/translation/javascript-module-initialization.js. Do not edit by hand.
use super::MlTemporaryReadJavaScriptSemanticValueErrorLoop1Result;
use super::MlTemporaryTestJavaScriptSemanticTextEqualLoop2Result;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::source_ml_temporary_read_java_script_semantic_value_error_loop1;
use super::source_ml_temporary_test_java_script_semantic_text_equal_loop2;

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
        pub fn from_text(text: &str) -> Self {
            Self(text.encode_utf16().collect())
        }
        pub fn from_units(units: Vec<u16>) -> Self {
            Self(units)
        }
        pub fn length(&self) -> f64 {
            self.0.len() as f64
        }
        pub fn concat(&self, other: &Self) -> Self {
            let mut units = self.0.clone();
            units.extend_from_slice(&other.0);
            Self(units)
        }
        pub fn starts_with(&self, search: &Self) -> bool {
            self.0.starts_with(&search.0)
        }
        pub fn ends_with(&self, search: &Self) -> bool {
            self.0.ends_with(&search.0)
        }
        pub fn contains(&self, search: &Self) -> bool {
            self.index_of(search, 0.0) >= 0.0
        }
        pub fn slice(&self, start: f64, end: f64) -> Self {
            let first = relative(start, self.0.len());
            let last = relative(end, self.0.len());
            Self(self.0[first..last.max(first)].to_vec())
        }
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
        pub fn split(&self, separator: Option<&Self>, limit: f64) -> Vec<Self> {
            let maximum = uint32(limit) as usize;
            if maximum == 0 {
                return Vec::new();
            }
            let separator = match separator {
                Some(value) => value,
                None => return vec![self.clone()],
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
        pub fn code_point_at(&self, position: f64) -> Option<f64> {
            let index = integer(position);
            if index < 0.0 || index >= self.0.len() as f64 {
                return None;
            }
            let index = index as usize;
            let first = self.0[index];
            if (0xd800..=0xdbff).contains(&first) {
                if let Some(&second) = self.0.get(index + 1) {
                    if (0xdc00..=0xdfff).contains(&second) {
                        return Some(
                            ((first as u32 - 0xd800) * 1024 + second as u32 - 0xdc00 + 0x10000)
                                as f64,
                        );
                    }
                }
            }
            Some(first as f64)
        }
        pub fn code_points(&self) -> Vec<Self> {
            let mut out = Vec::new();
            let mut index = 0;
            while index < self.0.len() {
                let first = self.0[index];
                let width = if (0xd800..=0xdbff).contains(&first)
                    && self
                        .0
                        .get(index + 1)
                        .map_or(false, |second| (0xdc00..=0xdfff).contains(second))
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
        pub fn repeat(&self, count: f64) -> Self {
            let count = integer(count);
            if count < 0.0 || !count.is_finite() {
                panic!("Invalid count value: {}", count);
            }
            if self.0.is_empty() {
                return Self(Vec::new());
            }
            let size = count * self.0.len() as f64;
            if size > 536870888.0 {
                panic!("Invalid string length");
            }
            Self(self.0.repeat(count as usize))
        }
        pub fn pad(&self, length: f64, fill: &Self, start: bool) -> Self {
            let length = integer(length).max(0.0);
            if length <= self.0.len() as f64 || fill.0.is_empty() {
                return self.clone();
            }
            if length > 536870888.0 {
                panic!("Invalid string length");
            }
            let count = length as usize - self.0.len();
            let padding: Vec<u16> = fill.0.iter().copied().cycle().take(count).collect();
            let pad = Self(padding);
            if start {
                pad.concat(self)
            } else {
                self.concat(&pad)
            }
        }
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
        pub fn trim(&self, start: bool, end: bool, rust: bool) -> Self {
            let whitespace = |unit: u16| {
                char::from_u32(unit as u32).map_or(false, |c| {
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
    pub fn integer(value: f64) -> f64 {
        if value.is_nan() { 0.0 } else { value.trunc() }
    }
    pub fn absolute(value: f64, length: usize) -> usize {
        integer(value).max(0.0).min(length as f64) as usize
    }
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
            value.trunc().rem_euclid(4294967296.0) as u32
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
        pub fn raise(self) -> ! {
            std::panic::panic_any((self.class_name, self.message_units, self.message_present))
        }
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
pub enum JavaScriptSemanticValue {
    Undefined,
    Null,
    Boolean(bool),
    Number(f64),
    Integer(self::ml::Big),
    Text(Vec<f64>),
    SourceText(self::ml_string::Text, self::ml_string::Text),
    Symbol(self::ml::Big),
    Object(self::ml::Big, bool),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptModuleInitializer {
    ModuleDataLoad(f64, self::ml_string::Text, self::ml_string::Text),
    ModuleConstAlias(f64, f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptModuleInitializationPlan {
    ModuleInitializationPlan(
        self::ml_string::Text,
        self::ml_string::Text,
        self::ml_string::Text,
        Vec<f64>,
        Vec<self::ml_string::Text>,
        Vec<self::JavaScriptModuleInitializer>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptModuleBindingSlot {
    ModuleBindingSlot(f64, bool, Box<self::JavaScriptSemanticValue>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptModuleInitializationState {
    ModuleInitializationState(
        self::ml_string::Text,
        self::ml_string::Text,
        self::ml_string::Text,
        self::ml::Big,
        self::ml_string::Text,
        f64,
        Vec<self::JavaScriptModuleBindingSlot>,
        Box<self::JavaScriptSemanticValue>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptModuleInitializationTransition {
    ModuleContinue(Box<self::JavaScriptModuleInitializationState>),
    ModuleReadData(
        Box<self::JavaScriptModuleInitializationState>,
        f64,
        self::ml_string::Text,
        self::ml_string::Text,
    ),
    ModuleTemporalDeadZone(
        Box<self::JavaScriptModuleInitializationState>,
        f64,
        self::ml_string::Text,
    ),
    ModuleEvaluated(Box<self::JavaScriptModuleInitializationState>),
    ModuleAbrupt(
        Box<self::JavaScriptModuleInitializationState>,
        Box<self::JavaScriptSemanticValue>,
    ),
    ModuleWaiting(Box<self::JavaScriptModuleInitializationState>),
    ModuleInvalid(
        Box<self::JavaScriptModuleInitializationState>,
        self::ml_string::Text,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptModuleBindingRead {
    ModuleBindingValue(Box<self::JavaScriptSemanticValue>),
    ModuleBindingUninitialized(f64, self::ml_string::Text),
    ModuleBindingFailure(Box<self::JavaScriptSemanticValue>),
    ModuleBindingInvalid(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_source_text_profile_error(
    profile: &self::ml_string::Text,
    length: f64,
) -> self::ml_string::Text {
    if ((*profile)
        != (self::ml_string::Text::from_units(vec![
            105, 110, 115, 112, 101, 99, 116, 97, 98, 108, 101, 32, 85, 84, 70, 49, 54, 32, 115,
            111, 117, 114, 99, 101, 32, 116, 101, 120, 116, 32, 49,
        ])))
    {
        self::ml_string::Text::from_units(vec![
            115, 111, 117, 114, 99, 101, 32, 116, 101, 120, 116, 32, 114, 101, 113, 117, 105, 114,
            101, 115, 32, 105, 116, 115, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 105, 110,
            115, 112, 101, 99, 116, 97, 98, 108, 101, 32, 85, 84, 70, 49, 54, 32, 114, 101, 115,
            111, 117, 114, 99, 101, 32, 112, 114, 111, 102, 105, 108, 101,
        ])
    } else {
        if (((!(self::ml_math::is_safe_integer(length)) || (length < (0f64)))
            || (length > (16777216f64)))
            || ((length == (0f64)) && ((1f64 / length) < (0f64))))
        {
            self::ml_string::Text::from_units(vec![
                115, 111, 117, 114, 99, 101, 32, 116, 101, 120, 116, 32, 101, 120, 99, 101, 101,
                100, 115, 32, 116, 104, 101, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 49, 54,
                55, 55, 55, 50, 49, 54, 32, 85, 84, 70, 49, 54, 32, 117, 110, 105, 116, 32, 98,
                117, 100, 103, 101, 116,
            ])
        } else {
            self::ml_string::Text::from_units(vec![])
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_semantic_value_error(
    value: self::JavaScriptSemanticValue,
) -> self::ml_string::Text {
    match value.clone() {
        self::JavaScriptSemanticValue::Text(units) => {
            if ((units.len() as f64) > (1048576f64)) {
                self::ml_string::Text::from_units(vec![
                    115, 101, 109, 97, 110, 116, 105, 99, 32, 116, 101, 120, 116, 32, 101, 120, 99,
                    101, 101, 100, 115, 32, 116, 104, 101, 32, 101, 120, 112, 108, 105, 99, 105,
                    116, 32, 85, 84, 70, 49, 54, 32, 117, 110, 105, 116, 32, 98, 117, 100, 103,
                    101, 116,
                ])
            } else {
                {
                    let at = 0f64;
                    {
                        let source_ml_source1_s5 =
                            self::source_ml_temporary_read_java_script_semantic_value_error_loop1(
                                &(units),
                                at,
                            );
                        match source_ml_source1_s5.clone() {
                            self::MlTemporaryReadJavaScriptSemanticValueErrorLoop1Result::MlTemporaryReadJavaScriptSemanticValueErrorLoop1Done => {
                                self::ml_string::Text::from_units(vec![])
                            }
                            self::MlTemporaryReadJavaScriptSemanticValueErrorLoop1Result::MlTemporaryReadJavaScriptSemanticValueErrorLoop1Return(source_ml_temporary_result1) => {
                                source_ml_temporary_result1.clone()
                            }
                        }
                    }
                }
            }
        }
        self::JavaScriptSemanticValue::SourceText(profile, text) => {
            self::read_java_script_source_text_profile_error(&(profile), text.length())
        }
        self::JavaScriptSemanticValue::Symbol(identity) => {
            if ((identity > (self::ml::Big::from_i128(0)))
                && (identity <= (self::ml::Big::from_i128(18446744073709551615))))
            {
                self::ml_string::Text::from_units(vec![])
            } else {
                self::ml_string::Text::from_units(vec![
                    97, 32, 115, 121, 109, 98, 111, 108, 32, 114, 101, 113, 117, 105, 114, 101,
                    115, 32, 97, 32, 112, 111, 115, 105, 116, 105, 118, 101, 32, 117, 110, 115,
                    105, 103, 110, 101, 100, 54, 52, 32, 103, 108, 111, 98, 97, 108, 32, 108, 105,
                    110, 107, 32, 105, 100, 101, 110, 116, 105, 116, 121,
                ])
            }
        }
        self::JavaScriptSemanticValue::Object(identity_2, _) => {
            if ((identity_2 > (self::ml::Big::from_i128(0)))
                && (identity_2 <= (self::ml::Big::from_i128(18446744073709551615))))
            {
                self::ml_string::Text::from_units(vec![])
            } else {
                self::ml_string::Text::from_units(vec![
                    97, 110, 32, 111, 98, 106, 101, 99, 116, 32, 114, 101, 113, 117, 105, 114, 101,
                    115, 32, 97, 32, 112, 111, 115, 105, 116, 105, 118, 101, 32, 117, 110, 115,
                    105, 103, 110, 101, 100, 54, 52, 32, 103, 108, 111, 98, 97, 108, 32, 108, 105,
                    110, 107, 32, 105, 100, 101, 110, 116, 105, 116, 121,
                ])
            }
        }
        _ => self::ml_string::Text::from_units(vec![]),
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_semantic_same_value(
    left: self::JavaScriptSemanticValue,
    right: self::JavaScriptSemanticValue,
) -> bool {
    if (((self::read_java_script_semantic_value_error(left.clone()))
        != (self::ml_string::Text::from_units(vec![])))
        || ((self::read_java_script_semantic_value_error(right.clone()))
            != (self::ml_string::Text::from_units(vec![]))))
    {
        (self::ml_source_exception::SourceException {
            class_name: String::from("TypeError"),
            message_units: (self::ml_string::Text::from_units(vec![
                83, 97, 109, 101, 86, 97, 108, 117, 101, 32, 114, 101, 113, 117, 105, 114, 101,
                115, 32, 116, 119, 111, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 118, 97,
                108, 117, 101, 115, 32, 105, 110, 32, 116, 104, 101, 32, 100, 101, 99, 108, 97,
                114, 101, 100, 32, 115, 101, 109, 97, 110, 116, 105, 99, 32, 109, 111, 100, 101,
                108,
            ]))
            .0,
            message_present: true,
        })
        .raise()
    } else {
        match left.clone() {
            self::JavaScriptSemanticValue::Undefined => match right.clone() {
                self::JavaScriptSemanticValue::Undefined => true,
                _ => false,
            },
            self::JavaScriptSemanticValue::Null => match right.clone() {
                self::JavaScriptSemanticValue::Null => true,
                _ => false,
            },
            self::JavaScriptSemanticValue::Boolean(left_value) => match right.clone() {
                self::JavaScriptSemanticValue::Boolean(value) => (left_value == value),
                _ => false,
            },
            self::JavaScriptSemanticValue::Number(left_value_2) => match right.clone() {
                self::JavaScriptSemanticValue::Number(value_2) => {
                    if (left_value_2 == value_2) {
                        ((left_value_2 != (0f64)) || ((1f64 / left_value_2) == (1f64 / value_2)))
                    } else {
                        (left_value_2.is_nan() && value_2.is_nan())
                    }
                }
                _ => false,
            },
            self::JavaScriptSemanticValue::Integer(left_value_3) => match right.clone() {
                self::JavaScriptSemanticValue::Integer(value_3) => (left_value_3 == value_3),
                _ => false,
            },
            self::JavaScriptSemanticValue::Text(_) => {
                self::test_java_script_semantic_text_equal(left.clone(), right.clone())
            }
            self::JavaScriptSemanticValue::SourceText(_, _) => {
                self::test_java_script_semantic_text_equal(left.clone(), right.clone())
            }
            self::JavaScriptSemanticValue::Symbol(left_identity) => match right.clone() {
                self::JavaScriptSemanticValue::Symbol(identity) => (left_identity == identity),
                _ => false,
            },
            self::JavaScriptSemanticValue::Object(left_identity_2, _) => match right.clone() {
                self::JavaScriptSemanticValue::Object(identity_2, _) => {
                    (left_identity_2 == identity_2)
                }
                _ => false,
            },
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_semantic_text_length(value: self::JavaScriptSemanticValue) -> f64 {
    if match value.clone() {
        self::JavaScriptSemanticValue::SourceText(_, _) => true,
        _ => false,
    } {
        {
            let source_ml_source1_nullish_field_receiver78 = (match value.clone() {
                self::JavaScriptSemanticValue::Undefined => None::<self::ml_string::Text>,
                self::JavaScriptSemanticValue::Null => None::<self::ml_string::Text>,
                self::JavaScriptSemanticValue::Boolean(_) => None::<self::ml_string::Text>,
                self::JavaScriptSemanticValue::Number(_) => None::<self::ml_string::Text>,
                self::JavaScriptSemanticValue::Integer(_) => None::<self::ml_string::Text>,
                self::JavaScriptSemanticValue::Text(_) => None::<self::ml_string::Text>,
                self::JavaScriptSemanticValue::SourceText(
                    _,
                    source_ml_source1_data_property_value77,
                ) => Some(source_ml_source1_data_property_value77.clone()),
                self::JavaScriptSemanticValue::Symbol(_) => None::<self::ml_string::Text>,
                self::JavaScriptSemanticValue::Object(_, _) => None::<self::ml_string::Text>,
            },);
            if ((match (source_ml_source1_nullish_field_receiver78.clone()).0 {
                Some(ml_present) => {
                    let _ = ml_present;
                    self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103])
                }
                None => self::ml_string::Text::from_units(vec![
                    117, 110, 100, 101, 102, 105, 110, 101, 100,
                ]),
            }) == (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103])))
            {
                {
                    let source_ml_source1_nullish_field_present79 =
                        ((source_ml_source1_nullish_field_receiver78.clone()).0)
                            .clone()
                            .expect("a proven present value");
                    source_ml_source1_nullish_field_present79.length()
                }
            } else {
                (self::ml_source_exception::SourceException {
                    class_name: String::from("TypeError"),
                    message_units: (self::ml_string::Text::from_units(vec![
                        67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112,
                        101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102,
                        105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 108,
                        101, 110, 103, 116, 104, 39, 41,
                    ]))
                    .0,
                    message_present: true,
                })
                .raise()
            }
        }
    } else {
        if match value.clone() {
            self::JavaScriptSemanticValue::Text(_) => true,
            _ => false,
        } {
            {
                let source_ml_source1_nullish_field_receiver90 = (match value.clone() {
                    self::JavaScriptSemanticValue::Undefined => None::<Vec<f64>>,
                    self::JavaScriptSemanticValue::Null => None::<Vec<f64>>,
                    self::JavaScriptSemanticValue::Boolean(_) => None::<Vec<f64>>,
                    self::JavaScriptSemanticValue::Number(_) => None::<Vec<f64>>,
                    self::JavaScriptSemanticValue::Integer(_) => None::<Vec<f64>>,
                    self::JavaScriptSemanticValue::Text(
                        source_ml_source1_data_property_value89,
                    ) => Some(source_ml_source1_data_property_value89),
                    self::JavaScriptSemanticValue::SourceText(_, _) => None::<Vec<f64>>,
                    self::JavaScriptSemanticValue::Symbol(_) => None::<Vec<f64>>,
                    self::JavaScriptSemanticValue::Object(_, _) => None::<Vec<f64>>,
                },);
                if match (source_ml_source1_nullish_field_receiver90.clone()).0 {
                    Some(ml_present) => {
                        let _ = ml_present.clone();
                        true
                    }
                    None => false,
                } {
                    {
                        let source_ml_source1_nullish_field_present91 =
                            ((source_ml_source1_nullish_field_receiver90.clone()).0)
                                .clone()
                                .expect("a proven present value");
                        (source_ml_source1_nullish_field_present91.len() as f64)
                    }
                } else {
                    (self::ml_source_exception::SourceException {
                        class_name: String::from("TypeError"),
                        message_units: (self::ml_string::Text::from_units(vec![
                            67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111,
                            112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100,
                            101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103,
                            32, 39, 108, 101, 110, 103, 116, 104, 39, 41,
                        ]))
                        .0,
                        message_present: true,
                    })
                    .raise()
                }
            }
        } else {
            (-1f64)
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_semantic_text_equal(
    left: self::JavaScriptSemanticValue,
    right: self::JavaScriptSemanticValue,
) -> bool {
    {
        let length = self::read_java_script_semantic_text_length(left.clone());
        if ((length < (0f64))
            || (length != (self::read_java_script_semantic_text_length(right.clone()))))
        {
            false
        } else {
            if (match left.clone() {
                self::JavaScriptSemanticValue::SourceText(_, _) => true,
                _ => false,
            } && match right.clone() {
                self::JavaScriptSemanticValue::SourceText(_, _) => true,
                _ => false,
            }) {
                {
                    let source_ml_source1_comparison_operand146 = match left.clone() {
                        self::JavaScriptSemanticValue::Undefined => None::<self::ml_string::Text>,
                        self::JavaScriptSemanticValue::Null => None::<self::ml_string::Text>,
                        self::JavaScriptSemanticValue::Boolean(_) => None::<self::ml_string::Text>,
                        self::JavaScriptSemanticValue::Number(_) => None::<self::ml_string::Text>,
                        self::JavaScriptSemanticValue::Integer(_) => None::<self::ml_string::Text>,
                        self::JavaScriptSemanticValue::Text(_) => None::<self::ml_string::Text>,
                        self::JavaScriptSemanticValue::SourceText(
                            _,
                            source_ml_source1_data_property_value144,
                        ) => Some(source_ml_source1_data_property_value144.clone()),
                        self::JavaScriptSemanticValue::Symbol(_) => None::<self::ml_string::Text>,
                        self::JavaScriptSemanticValue::Object(_, _) => {
                            None::<self::ml_string::Text>
                        }
                    };
                    {
                        let source_ml_source1_comparison_operand147 = match right.clone() {
                            self::JavaScriptSemanticValue::Undefined => {
                                None::<self::ml_string::Text>
                            }
                            self::JavaScriptSemanticValue::Null => None::<self::ml_string::Text>,
                            self::JavaScriptSemanticValue::Boolean(_) => {
                                None::<self::ml_string::Text>
                            }
                            self::JavaScriptSemanticValue::Number(_) => {
                                None::<self::ml_string::Text>
                            }
                            self::JavaScriptSemanticValue::Integer(_) => {
                                None::<self::ml_string::Text>
                            }
                            self::JavaScriptSemanticValue::Text(_) => None::<self::ml_string::Text>,
                            self::JavaScriptSemanticValue::SourceText(
                                _,
                                source_ml_source1_data_property_value145,
                            ) => Some(source_ml_source1_data_property_value145.clone()),
                            self::JavaScriptSemanticValue::Symbol(_) => {
                                None::<self::ml_string::Text>
                            }
                            self::JavaScriptSemanticValue::Object(_, _) => {
                                None::<self::ml_string::Text>
                            }
                        };
                        match (
                            source_ml_source1_comparison_operand146.clone(),
                            source_ml_source1_comparison_operand147.clone(),
                        ) {
                            (Some(ml_left), Some(ml_right)) => ml_left == ml_right,
                            (None, None) => true,
                            _ => false,
                        }
                    }
                }
            } else {
                {
                    let at = 0f64;
                    {
                        let source_ml_source1_s148 =
                            self::source_ml_temporary_test_java_script_semantic_text_equal_loop2(
                                left.clone(),
                                right.clone(),
                                length,
                                at,
                            );
                        match source_ml_source1_s148.clone() {
                            self::MlTemporaryTestJavaScriptSemanticTextEqualLoop2Result::MlTemporaryTestJavaScriptSemanticTextEqualLoop2Done => {
                                true
                            }
                            self::MlTemporaryTestJavaScriptSemanticTextEqualLoop2Result::MlTemporaryTestJavaScriptSemanticTextEqualLoop2Return(source_ml_temporary_result2) => {
                                source_ml_temporary_result2
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
pub fn read_java_script_module_initialization_profile_error(
    profile: &self::ml_string::Text,
) -> self::ml_string::Text {
    if ((*profile)
        == (self::ml_string::Text::from_units(vec![
            97, 99, 116, 117, 97, 108, 32, 114, 117, 110, 116, 105, 109, 101, 32, 76, 105, 110,
            107, 115, 32, 112, 111, 108, 105, 99, 121, 32, 105, 110, 105, 116, 105, 97, 108, 105,
            122, 101, 114, 115, 32, 49,
        ])))
    {
        self::ml_string::Text::from_units(vec![])
    } else {
        self::ml_string::Text::from_units(vec![
            114, 117, 110, 116, 105, 109, 101, 32, 112, 111, 108, 105, 99, 121, 32, 109, 111, 100,
            117, 108, 101, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114,
            32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 45, 98, 111, 117, 110,
            100, 32, 73, 79, 32, 105, 110, 105, 116, 105, 97, 108, 105, 122, 97, 116, 105, 111,
            110, 32, 112, 114, 111, 102, 105, 108, 101,
        ])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_module_initialization_phase(phase: &self::ml_string::Text) -> bool {
    self::ml_array::includes(
        &vec![
            self::ml_string::Text::from_units(vec![
                117, 110, 105, 110, 105, 116, 105, 97, 108, 105, 122, 101, 100,
            ]),
            self::ml_string::Text::from_units(vec![
                105, 110, 105, 116, 105, 97, 108, 105, 122, 105, 110, 103,
            ]),
            self::ml_string::Text::from_units(vec![108, 111, 97, 100, 105, 110, 103]),
            self::ml_string::Text::from_units(vec![
                116, 101, 109, 112, 111, 114, 97, 108, 32, 100, 101, 97, 100, 32, 122, 111, 110,
                101,
            ]),
            self::ml_string::Text::from_units(vec![101, 118, 97, 108, 117, 97, 116, 101, 100]),
            self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114, 101, 100]),
        ],
        phase,
        0f64,
        false,
        |a, b| a == b,
    )
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_module_binding_read(
    phase: &self::ml_string::Text,
    initialized: bool,
) -> f64 {
    if ((*phase) == (self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114, 101, 100]))) {
        (-2f64)
    } else {
        if initialized { 1f64 } else { 0f64 }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_source_coordinate(value: f64, limit: f64) -> bool {
    (((self::ml_math::is_safe_integer(value) && (value >= (0f64))) && (value < limit))
        && ((value != (0f64)) || ((1f64 / value) > (0f64))))
}
