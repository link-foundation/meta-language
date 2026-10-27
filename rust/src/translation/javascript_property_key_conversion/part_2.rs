// Generated from complete js/src/translation/javascript-property-key-conversion.js. Do not edit by hand.
use super::MlTemporaryReadJavaScriptSemanticValueErrorLoop1Result;
use super::MlTemporaryTestJavaScriptSemanticSameValueLoop2Result;
use super::ml;
use super::source_ml_temporary_read_java_script_semantic_value_error_loop1;
use super::source_ml_temporary_test_java_script_semantic_same_value_loop2;

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

#[path = "ml_big_integer_conversion.rs"]
pub mod ml_big_integer_conversion;

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptSemanticValue {
    Undefined,
    Null,
    Boolean(bool),
    Number(f64),
    Integer(self::ml::Big),
    Text(Vec<f64>),
    Symbol(self::ml::Big),
    Object(self::ml::Big, bool),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptOrdinaryProperty {
    OrdinaryProperty(
        Box<self::JavaScriptSemanticValue>,
        (
            bool,
            bool,
            bool,
            bool,
            self::JavaScriptSemanticValue,
            bool,
            self::JavaScriptSemanticValue,
            bool,
            self::JavaScriptSemanticValue,
            bool,
            bool,
            bool,
        ),
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptOrdinaryObject {
    OrdinaryObject(
        self::ml::Big,
        Box<self::JavaScriptSemanticValue>,
        bool,
        bool,
        Vec<self::JavaScriptOrdinaryProperty>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptOrdinaryHeap {
    OrdinaryHeap(Vec<self::JavaScriptOrdinaryObject>, Vec<self::ml::Big>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptPrimitiveConversionState {
    PrimitiveConversion(
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
        self::ml_string::Text,
        self::ml_string::Text,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptPrimitiveConversionCompletion {
    NormalCompletion(Box<self::JavaScriptSemanticValue>),
    ThrowCompletion(Box<self::JavaScriptSemanticValue>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptPrimitiveConversionTransition {
    PrimitiveContinuation(Box<self::JavaScriptPrimitiveConversionState>),
    PrimitiveCall(
        Box<self::JavaScriptPrimitiveConversionState>,
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
        Vec<self::JavaScriptSemanticValue>,
    ),
    PrimitiveCompleted(Box<self::JavaScriptSemanticValue>),
    PrimitiveThrow(Box<self::JavaScriptSemanticValue>),
    PrimitiveIntrinsicError(self::ml_string::Text, self::ml_string::Text),
    PrimitiveInvalid(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptPropertyKeyConversionTransition {
    PropertyKeyContinuation(Box<self::JavaScriptPrimitiveConversionState>),
    PropertyKeyCall(
        Box<self::JavaScriptPrimitiveConversionState>,
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
        Vec<self::JavaScriptSemanticValue>,
    ),
    PropertyKeyCompleted(Box<self::JavaScriptSemanticValue>),
    PropertyKeyThrow(Box<self::JavaScriptSemanticValue>),
    PropertyKeyIntrinsicError(self::ml_string::Text, self::ml_string::Text),
    PropertyKeyInvalid(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_semantic_value_error(
    value: self::JavaScriptSemanticValue,
) -> self::ml_string::Text {
    match value.clone() {
        self::JavaScriptSemanticValue::Undefined => self::ml_string::Text::from_units(vec![]),
        self::JavaScriptSemanticValue::Null => self::ml_string::Text::from_units(vec![]),
        self::JavaScriptSemanticValue::Boolean(source_ml_source1_f1) => {
            self::ml_string::Text::from_units(vec![])
        }
        self::JavaScriptSemanticValue::Number(source_ml_source1_f2) => {
            self::ml_string::Text::from_units(vec![])
        }
        self::JavaScriptSemanticValue::Integer(source_ml_source1_f3) => {
            self::ml_string::Text::from_units(vec![])
        }
        self::JavaScriptSemanticValue::Text(units) => {
            if (units.len() as f64) > 1_048_576f64 {
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
                        match source_ml_source1_s5 {
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
        self::JavaScriptSemanticValue::Symbol(identity) => {
            if (identity > (self::ml::Big::from_i128(0)))
                && (identity <= (self::ml::Big::from_i128(18_446_744_073_709_551_615)))
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
        self::JavaScriptSemanticValue::Object(identity_2, source_ml_source1_f4) => {
            if (identity_2 > (self::ml::Big::from_i128(0)))
                && (identity_2 <= (self::ml::Big::from_i128(18_446_744_073_709_551_615)))
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
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_semantic_same_value(
    left: self::JavaScriptSemanticValue,
    right: self::JavaScriptSemanticValue,
) -> bool {
    if ((self::read_java_script_semantic_value_error(left.clone()))
        != (self::ml_string::Text::from_units(vec![])))
        || ((self::read_java_script_semantic_value_error(right.clone()))
            != (self::ml_string::Text::from_units(vec![])))
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
                self::JavaScriptSemanticValue::Null => false,
                self::JavaScriptSemanticValue::Boolean(source_ml_source1_f7) => false,
                self::JavaScriptSemanticValue::Number(source_ml_source1_f8) => false,
                self::JavaScriptSemanticValue::Integer(source_ml_source1_f9) => false,
                self::JavaScriptSemanticValue::Text(source_ml_source1_f10) => false,
                self::JavaScriptSemanticValue::Symbol(source_ml_source1_f11) => false,
                self::JavaScriptSemanticValue::Object(
                    source_ml_source1_f12,
                    source_ml_source1_f13,
                ) => false,
            },
            self::JavaScriptSemanticValue::Null => match right.clone() {
                self::JavaScriptSemanticValue::Undefined => false,
                self::JavaScriptSemanticValue::Null => true,
                self::JavaScriptSemanticValue::Boolean(source_ml_source1_f14) => false,
                self::JavaScriptSemanticValue::Number(source_ml_source1_f15) => false,
                self::JavaScriptSemanticValue::Integer(source_ml_source1_f16) => false,
                self::JavaScriptSemanticValue::Text(source_ml_source1_f17) => false,
                self::JavaScriptSemanticValue::Symbol(source_ml_source1_f18) => false,
                self::JavaScriptSemanticValue::Object(
                    source_ml_source1_f19,
                    source_ml_source1_f20,
                ) => false,
            },
            self::JavaScriptSemanticValue::Boolean(left_value) => match right.clone() {
                self::JavaScriptSemanticValue::Undefined => false,
                self::JavaScriptSemanticValue::Null => false,
                self::JavaScriptSemanticValue::Boolean(value) => left_value == value,
                self::JavaScriptSemanticValue::Number(source_ml_source1_f21) => false,
                self::JavaScriptSemanticValue::Integer(source_ml_source1_f22) => false,
                self::JavaScriptSemanticValue::Text(source_ml_source1_f23) => false,
                self::JavaScriptSemanticValue::Symbol(source_ml_source1_f24) => false,
                self::JavaScriptSemanticValue::Object(
                    source_ml_source1_f25,
                    source_ml_source1_f26,
                ) => false,
            },
            self::JavaScriptSemanticValue::Number(left_value_2) => match right.clone() {
                self::JavaScriptSemanticValue::Undefined => false,
                self::JavaScriptSemanticValue::Null => false,
                self::JavaScriptSemanticValue::Boolean(source_ml_source1_f27) => false,
                self::JavaScriptSemanticValue::Number(value_2) => {
                    if left_value_2 == value_2 {
                        (left_value_2 != 0f64) || ((1f64 / left_value_2) == (1f64 / value_2))
                    } else {
                        left_value_2.is_nan() && value_2.is_nan()
                    }
                }
                self::JavaScriptSemanticValue::Integer(source_ml_source1_f28) => false,
                self::JavaScriptSemanticValue::Text(source_ml_source1_f29) => false,
                self::JavaScriptSemanticValue::Symbol(source_ml_source1_f30) => false,
                self::JavaScriptSemanticValue::Object(
                    source_ml_source1_f31,
                    source_ml_source1_f32,
                ) => false,
            },
            self::JavaScriptSemanticValue::Integer(left_value_3) => match right.clone() {
                self::JavaScriptSemanticValue::Undefined => false,
                self::JavaScriptSemanticValue::Null => false,
                self::JavaScriptSemanticValue::Boolean(source_ml_source1_f33) => false,
                self::JavaScriptSemanticValue::Number(source_ml_source1_f34) => false,
                self::JavaScriptSemanticValue::Integer(value_3) => left_value_3 == value_3,
                self::JavaScriptSemanticValue::Text(source_ml_source1_f35) => false,
                self::JavaScriptSemanticValue::Symbol(source_ml_source1_f36) => false,
                self::JavaScriptSemanticValue::Object(
                    source_ml_source1_f37,
                    source_ml_source1_f38,
                ) => false,
            },
            self::JavaScriptSemanticValue::Text(left_units) => match right.clone() {
                self::JavaScriptSemanticValue::Undefined => false,
                self::JavaScriptSemanticValue::Null => false,
                self::JavaScriptSemanticValue::Boolean(source_ml_source1_f39) => false,
                self::JavaScriptSemanticValue::Number(source_ml_source1_f40) => false,
                self::JavaScriptSemanticValue::Integer(source_ml_source1_f41) => false,
                self::JavaScriptSemanticValue::Text(units) => {
                    let left_units_2 = left_units;
                    {
                        let right_units = units;
                        {
                            let ml_library_inequality_1 =
                                (left_units_2.len() as f64) != (right_units.len() as f64);
                            if ml_library_inequality_1 {
                                false
                            } else {
                                {
                                    let at = 0f64;
                                    {
                                        let source_ml_source1_s45 = self::source_ml_temporary_test_java_script_semantic_same_value_loop2(&(left_units_2), &(right_units), at);
                                        match source_ml_source1_s45 {
                                                self::MlTemporaryTestJavaScriptSemanticSameValueLoop2Result::MlTemporaryTestJavaScriptSemanticSameValueLoop2Done => {
                                                    true
                                                }
                                                self::MlTemporaryTestJavaScriptSemanticSameValueLoop2Result::MlTemporaryTestJavaScriptSemanticSameValueLoop2Return(source_ml_temporary_result2) => {
                                                    source_ml_temporary_result2
                                                }
                                            }
                                    }
                                }
                            }
                        }
                    }
                }
                self::JavaScriptSemanticValue::Symbol(source_ml_source1_f42) => false,
                self::JavaScriptSemanticValue::Object(
                    source_ml_source1_f43,
                    source_ml_source1_f44,
                ) => false,
            },
            self::JavaScriptSemanticValue::Symbol(left_identity) => match right.clone() {
                self::JavaScriptSemanticValue::Undefined => false,
                self::JavaScriptSemanticValue::Null => false,
                self::JavaScriptSemanticValue::Boolean(source_ml_source1_f46) => false,
                self::JavaScriptSemanticValue::Number(source_ml_source1_f47) => false,
                self::JavaScriptSemanticValue::Integer(source_ml_source1_f48) => false,
                self::JavaScriptSemanticValue::Text(source_ml_source1_f49) => false,
                self::JavaScriptSemanticValue::Symbol(identity) => left_identity == identity,
                self::JavaScriptSemanticValue::Object(
                    source_ml_source1_f50,
                    source_ml_source1_f51,
                ) => false,
            },
            self::JavaScriptSemanticValue::Object(left_identity_2, source_ml_source1_f6) => {
                match right.clone() {
                    self::JavaScriptSemanticValue::Undefined => false,
                    self::JavaScriptSemanticValue::Null => false,
                    self::JavaScriptSemanticValue::Boolean(source_ml_source1_f52) => false,
                    self::JavaScriptSemanticValue::Number(source_ml_source1_f53) => false,
                    self::JavaScriptSemanticValue::Integer(source_ml_source1_f54) => false,
                    self::JavaScriptSemanticValue::Text(source_ml_source1_f55) => false,
                    self::JavaScriptSemanticValue::Symbol(source_ml_source1_f56) => false,
                    self::JavaScriptSemanticValue::Object(identity_2, source_ml_source1_f57) => {
                        left_identity_2 == identity_2
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_semantic_accessor(value: self::JavaScriptSemanticValue) -> bool {
    {
        let ml_library_inequality_1 = (self::read_java_script_semantic_value_error(value.clone()))
            != (self::ml_string::Text::from_units(vec![]));
        if ml_library_inequality_1 {
            false
        } else {
            match value.clone() {
                self::JavaScriptSemanticValue::Undefined => true,
                self::JavaScriptSemanticValue::Null => false,
                self::JavaScriptSemanticValue::Boolean(source_ml_source1_f58) => {
                    let source_ml_source1_f71 = source_ml_source1_f58;
                    false
                }
                self::JavaScriptSemanticValue::Number(source_ml_source1_f59) => {
                    let source_ml_source1_f78 = source_ml_source1_f59;
                    false
                }
                self::JavaScriptSemanticValue::Integer(source_ml_source1_f60) => {
                    let source_ml_source1_f85 = source_ml_source1_f60.clone();
                    false
                }
                self::JavaScriptSemanticValue::Text(source_ml_source1_f61) => {
                    let source_ml_source1_f92 = source_ml_source1_f61;
                    false
                }
                self::JavaScriptSemanticValue::Symbol(source_ml_source1_f62) => {
                    let source_ml_source1_f99 = source_ml_source1_f62.clone();
                    false
                }
                self::JavaScriptSemanticValue::Object(
                    source_ml_source1_f63,
                    source_ml_source1_f64,
                ) => {
                    let source_ml_source1_f106 = source_ml_source1_f63.clone();
                    { source_ml_source1_f64 }
                }
            }
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
pub fn read_java_script_property_descriptor_kind(
    descriptor: (
        bool,
        bool,
        bool,
        bool,
        self::JavaScriptSemanticValue,
        bool,
        self::JavaScriptSemanticValue,
        bool,
        self::JavaScriptSemanticValue,
        bool,
        bool,
        bool,
    ),
) -> self::ml_string::Text {
    {
        let data = (descriptor.clone()).9 || (descriptor.clone()).11;
        {
            let accessor = (descriptor.clone()).5 || (descriptor.clone()).7;
            if data && accessor {
                self::ml_string::Text::from_units(vec![105, 110, 118, 97, 108, 105, 100])
            } else if data {
                self::ml_string::Text::from_units(vec![100, 97, 116, 97])
            } else if accessor {
                self::ml_string::Text::from_units(vec![97, 99, 99, 101, 115, 115, 111, 114])
            } else {
                self::ml_string::Text::from_units(vec![103, 101, 110, 101, 114, 105, 99])
            }
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
pub fn read_java_script_property_descriptor_error(
    descriptor: (
        bool,
        bool,
        bool,
        bool,
        self::JavaScriptSemanticValue,
        bool,
        self::JavaScriptSemanticValue,
        bool,
        self::JavaScriptSemanticValue,
        bool,
        bool,
        bool,
    ),
) -> self::ml_string::Text {
    if (self::read_java_script_property_descriptor_kind(descriptor.clone()))
        == (self::ml_string::Text::from_units(vec![105, 110, 118, 97, 108, 105, 100]))
    {
        self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114])
    } else if (descriptor.clone()).9
        && ((self::read_java_script_semantic_value_error((descriptor.clone()).8))
            != (self::ml_string::Text::from_units(vec![])))
    {
        self::ml_string::Text::from_units(vec![
            73, 110, 118, 97, 108, 105, 100, 83, 101, 109, 97, 110, 116, 105, 99, 86, 97, 108, 117,
            101,
        ])
    } else if (descriptor.clone()).5
        && (((self::read_java_script_semantic_value_error((descriptor.clone()).4))
            != (self::ml_string::Text::from_units(vec![])))
            || !(self::test_java_script_semantic_accessor((descriptor.clone()).4)))
    {
        self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114])
    } else if (descriptor.clone()).7
        && (((self::read_java_script_semantic_value_error((descriptor.clone()).6))
            != (self::ml_string::Text::from_units(vec![])))
            || !(self::test_java_script_semantic_accessor((descriptor.clone()).6)))
    {
        self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114])
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
pub fn test_complete_java_script_property_descriptor(
    descriptor: (
        bool,
        bool,
        bool,
        bool,
        self::JavaScriptSemanticValue,
        bool,
        self::JavaScriptSemanticValue,
        bool,
        self::JavaScriptSemanticValue,
        bool,
        bool,
        bool,
    ),
) -> bool {
    if (!((descriptor.clone()).1) || !((descriptor.clone()).3))
        || ((self::read_java_script_property_descriptor_error(descriptor.clone()))
            != (self::ml_string::Text::from_units(vec![])))
    {
        false
    } else {
        {
            let kind = self::read_java_script_property_descriptor_kind(descriptor.clone());
            if kind == (self::ml_string::Text::from_units(vec![100, 97, 116, 97])) {
                (descriptor.clone()).9 && (descriptor.clone()).11
            } else {
                ((kind
                    == (self::ml_string::Text::from_units(vec![
                        97, 99, 99, 101, 115, 115, 111, 114,
                    ])))
                    && (descriptor.clone()).5)
                    && (descriptor.clone()).7
            }
        }
    }
}
