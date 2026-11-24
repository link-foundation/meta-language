// Generated from complete js/src/translation/javascript-array-objects.js. Do not edit by hand.
use super::MlTemporaryReadJavaScriptSemanticValueErrorLoop1Result;
use super::ml;
use super::ml_math;
use super::source_ml_temporary_read_java_script_semantic_value_error_loop1;
use super::test_java_script_semantic_text_equal;

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

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptSemanticValue {
    Undefined,
    Null,
    Boolean(bool),
    Number(f64),
    Integer(self::ml::Big),
    Text(std::rc::Rc<std::cell::RefCell<Vec<f64>>>),
    SourceText(self::ml_string::Text, self::ml_string::Text),
    Symbol(self::ml::Big),
    Object(self::ml::Big, bool),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptOrdinaryProperty {
    OrdinaryProperty(
        Box<self::JavaScriptSemanticValue>,
        std::rc::Rc<
            std::cell::RefCell<(
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
            )>,
        >,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptArrayObject {
    ArrayObject(
        self::ml::Big,
        Box<self::JavaScriptSemanticValue>,
        bool,
        bool,
        std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptArrayPropertyPlan {
    ArrayPropertyPlan(
        Box<self::JavaScriptArrayObject>,
        bool,
        self::ml_string::Text,
        self::ml_string::Text,
        std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>,
    ),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_source_text_profile_error(
    profile: &self::ml_string::Text,
    length: f64,
) -> self::ml_string::Text {
    {
        let ml_library_inequality_1 = (*profile)
            != (self::ml_string::Text::from_units(vec![
                105, 110, 115, 112, 101, 99, 116, 97, 98, 108, 101, 32, 85, 84, 70, 49, 54, 32,
                115, 111, 117, 114, 99, 101, 32, 116, 101, 120, 116, 32, 49,
            ]));
        if ml_library_inequality_1 {
            self::ml_string::Text::from_units(vec![
                115, 111, 117, 114, 99, 101, 32, 116, 101, 120, 116, 32, 114, 101, 113, 117, 105,
                114, 101, 115, 32, 105, 116, 115, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32,
                105, 110, 115, 112, 101, 99, 116, 97, 98, 108, 101, 32, 85, 84, 70, 49, 54, 32,
                114, 101, 115, 111, 117, 114, 99, 101, 32, 112, 114, 111, 102, 105, 108, 101,
            ])
        } else if ((!(self::ml_math::is_safe_integer(length)) || (length < 0f64))
            || (length > 16_777_216f64))
            || ((length == 0f64) && ((1f64 / length) < 0f64))
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
            let ml_product_condition_1 = {
                let ml_reference_array1 = units.clone();
                ml_reference_array1.borrow().len() as f64
            };
            if ml_product_condition_1 {
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
                                units.clone(),
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
        self::JavaScriptSemanticValue::SourceText(profile, text) => {
            let ml_library_inequality_1 =
                (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103]))
                    != (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103]));
            if ml_library_inequality_1 {
                self::ml_string::Text::from_units(vec![
                    115, 111, 117, 114, 99, 101, 32, 116, 101, 120, 116, 32, 114, 101, 113, 117,
                    105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99,
                    111, 109, 112, 108, 101, 116, 101, 32, 83, 116, 114, 105, 110, 103, 32, 118,
                    97, 108, 117, 101,
                ])
            } else {
                self::read_java_script_source_text_profile_error(&(profile), text.length())
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
                self::JavaScriptSemanticValue::Boolean(source_ml_source1_f10) => false,
                self::JavaScriptSemanticValue::Number(source_ml_source1_f11) => false,
                self::JavaScriptSemanticValue::Integer(source_ml_source1_f12) => false,
                self::JavaScriptSemanticValue::Text(source_ml_source1_f13) => false,
                self::JavaScriptSemanticValue::SourceText(
                    source_ml_source1_f14,
                    source_ml_source1_f15,
                ) => false,
                self::JavaScriptSemanticValue::Symbol(source_ml_source1_f16) => false,
                self::JavaScriptSemanticValue::Object(
                    source_ml_source1_f17,
                    source_ml_source1_f18,
                ) => false,
            },
            self::JavaScriptSemanticValue::Null => match right.clone() {
                self::JavaScriptSemanticValue::Undefined => false,
                self::JavaScriptSemanticValue::Null => true,
                self::JavaScriptSemanticValue::Boolean(source_ml_source1_f19) => false,
                self::JavaScriptSemanticValue::Number(source_ml_source1_f20) => false,
                self::JavaScriptSemanticValue::Integer(source_ml_source1_f21) => false,
                self::JavaScriptSemanticValue::Text(source_ml_source1_f22) => false,
                self::JavaScriptSemanticValue::SourceText(
                    source_ml_source1_f23,
                    source_ml_source1_f24,
                ) => false,
                self::JavaScriptSemanticValue::Symbol(source_ml_source1_f25) => false,
                self::JavaScriptSemanticValue::Object(
                    source_ml_source1_f26,
                    source_ml_source1_f27,
                ) => false,
            },
            self::JavaScriptSemanticValue::Boolean(left_value) => match right.clone() {
                self::JavaScriptSemanticValue::Undefined => false,
                self::JavaScriptSemanticValue::Null => false,
                self::JavaScriptSemanticValue::Boolean(value) => left_value == value,
                self::JavaScriptSemanticValue::Number(source_ml_source1_f28) => false,
                self::JavaScriptSemanticValue::Integer(source_ml_source1_f29) => false,
                self::JavaScriptSemanticValue::Text(source_ml_source1_f30) => false,
                self::JavaScriptSemanticValue::SourceText(
                    source_ml_source1_f31,
                    source_ml_source1_f32,
                ) => false,
                self::JavaScriptSemanticValue::Symbol(source_ml_source1_f33) => false,
                self::JavaScriptSemanticValue::Object(
                    source_ml_source1_f34,
                    source_ml_source1_f35,
                ) => false,
            },
            self::JavaScriptSemanticValue::Number(left_value_2) => match right.clone() {
                self::JavaScriptSemanticValue::Undefined => false,
                self::JavaScriptSemanticValue::Null => false,
                self::JavaScriptSemanticValue::Boolean(source_ml_source1_f36) => false,
                self::JavaScriptSemanticValue::Number(value_2) => {
                    if left_value_2 == value_2 {
                        (left_value_2 != 0f64) || ((1f64 / left_value_2) == (1f64 / value_2))
                    } else {
                        left_value_2.is_nan() && value_2.is_nan()
                    }
                }
                self::JavaScriptSemanticValue::Integer(source_ml_source1_f37) => false,
                self::JavaScriptSemanticValue::Text(source_ml_source1_f38) => false,
                self::JavaScriptSemanticValue::SourceText(
                    source_ml_source1_f39,
                    source_ml_source1_f40,
                ) => false,
                self::JavaScriptSemanticValue::Symbol(source_ml_source1_f41) => false,
                self::JavaScriptSemanticValue::Object(
                    source_ml_source1_f42,
                    source_ml_source1_f43,
                ) => false,
            },
            self::JavaScriptSemanticValue::Integer(left_value_3) => match right.clone() {
                self::JavaScriptSemanticValue::Undefined => false,
                self::JavaScriptSemanticValue::Null => false,
                self::JavaScriptSemanticValue::Boolean(source_ml_source1_f44) => false,
                self::JavaScriptSemanticValue::Number(source_ml_source1_f45) => false,
                self::JavaScriptSemanticValue::Integer(value_3) => left_value_3 == value_3,
                self::JavaScriptSemanticValue::Text(source_ml_source1_f46) => false,
                self::JavaScriptSemanticValue::SourceText(
                    source_ml_source1_f47,
                    source_ml_source1_f48,
                ) => false,
                self::JavaScriptSemanticValue::Symbol(source_ml_source1_f49) => false,
                self::JavaScriptSemanticValue::Object(
                    source_ml_source1_f50,
                    source_ml_source1_f51,
                ) => false,
            },
            self::JavaScriptSemanticValue::Text(source_ml_source1_f6) => {
                self::test_java_script_semantic_text_equal(left.clone(), right.clone())
            }
            self::JavaScriptSemanticValue::SourceText(
                source_ml_source1_f7,
                source_ml_source1_f8,
            ) => self::test_java_script_semantic_text_equal(left.clone(), right.clone()),
            self::JavaScriptSemanticValue::Symbol(left_identity) => match right.clone() {
                self::JavaScriptSemanticValue::Undefined => false,
                self::JavaScriptSemanticValue::Null => false,
                self::JavaScriptSemanticValue::Boolean(source_ml_source1_f52) => false,
                self::JavaScriptSemanticValue::Number(source_ml_source1_f53) => false,
                self::JavaScriptSemanticValue::Integer(source_ml_source1_f54) => false,
                self::JavaScriptSemanticValue::Text(source_ml_source1_f55) => false,
                self::JavaScriptSemanticValue::SourceText(
                    source_ml_source1_f56,
                    source_ml_source1_f57,
                ) => false,
                self::JavaScriptSemanticValue::Symbol(identity) => left_identity == identity,
                self::JavaScriptSemanticValue::Object(
                    source_ml_source1_f58,
                    source_ml_source1_f59,
                ) => false,
            },
            self::JavaScriptSemanticValue::Object(left_identity_2, source_ml_source1_f9) => {
                match right.clone() {
                    self::JavaScriptSemanticValue::Undefined => false,
                    self::JavaScriptSemanticValue::Null => false,
                    self::JavaScriptSemanticValue::Boolean(source_ml_source1_f60) => false,
                    self::JavaScriptSemanticValue::Number(source_ml_source1_f61) => false,
                    self::JavaScriptSemanticValue::Integer(source_ml_source1_f62) => false,
                    self::JavaScriptSemanticValue::Text(source_ml_source1_f63) => false,
                    self::JavaScriptSemanticValue::SourceText(
                        source_ml_source1_f64,
                        source_ml_source1_f65,
                    ) => false,
                    self::JavaScriptSemanticValue::Symbol(source_ml_source1_f66) => false,
                    self::JavaScriptSemanticValue::Object(identity_2, source_ml_source1_f67) => {
                        left_identity_2 == identity_2
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_semantic_text_length(value: self::JavaScriptSemanticValue) -> f64 {
    if match value.clone() {
        self::JavaScriptSemanticValue::Undefined => false,
        self::JavaScriptSemanticValue::Null => false,
        self::JavaScriptSemanticValue::Boolean(source_ml_source1_f68) => false,
        self::JavaScriptSemanticValue::Number(source_ml_source1_f69) => false,
        self::JavaScriptSemanticValue::Integer(source_ml_source1_f70) => false,
        self::JavaScriptSemanticValue::Text(source_ml_source1_f71) => false,
        self::JavaScriptSemanticValue::SourceText(source_ml_source1_f72, source_ml_source1_f73) => {
            true
        }
        self::JavaScriptSemanticValue::Symbol(source_ml_source1_f74) => false,
        self::JavaScriptSemanticValue::Object(source_ml_source1_f75, source_ml_source1_f76) => {
            false
        }
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
            if (match (source_ml_source1_nullish_field_receiver78.clone()).0 {
                Some(ml_present) => {
                    let _ = ml_present;
                    self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103])
                }
                None => self::ml_string::Text::from_units(vec![
                    117, 110, 100, 101, 102, 105, 110, 101, 100,
                ]),
            }) == (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103]))
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
    } else if match value.clone() {
        self::JavaScriptSemanticValue::Undefined => false,
        self::JavaScriptSemanticValue::Null => false,
        self::JavaScriptSemanticValue::Boolean(source_ml_source1_f80) => false,
        self::JavaScriptSemanticValue::Number(source_ml_source1_f81) => false,
        self::JavaScriptSemanticValue::Integer(source_ml_source1_f82) => false,
        self::JavaScriptSemanticValue::Text(source_ml_source1_f83) => true,
        self::JavaScriptSemanticValue::SourceText(source_ml_source1_f84, source_ml_source1_f85) => {
            false
        }
        self::JavaScriptSemanticValue::Symbol(source_ml_source1_f86) => false,
        self::JavaScriptSemanticValue::Object(source_ml_source1_f87, source_ml_source1_f88) => {
            false
        }
    } {
        {
            let source_ml_source1_nullish_field_receiver90 = (match value.clone() {
                self::JavaScriptSemanticValue::Undefined => {
                    None::<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>
                }
                self::JavaScriptSemanticValue::Null => {
                    None::<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>
                }
                self::JavaScriptSemanticValue::Boolean(_) => {
                    None::<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>
                }
                self::JavaScriptSemanticValue::Number(_) => {
                    None::<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>
                }
                self::JavaScriptSemanticValue::Integer(_) => {
                    None::<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>
                }
                self::JavaScriptSemanticValue::Text(source_ml_source1_data_property_value89) => {
                    Some(source_ml_source1_data_property_value89.clone())
                }
                self::JavaScriptSemanticValue::SourceText(_, _) => {
                    None::<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>
                }
                self::JavaScriptSemanticValue::Symbol(_) => {
                    None::<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>
                }
                self::JavaScriptSemanticValue::Object(_, _) => {
                    None::<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>
                }
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
                    {
                        let ml_reference_array2 = source_ml_source1_nullish_field_present91;
                        ml_reference_array2.borrow().len() as f64
                    }
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
        -1f64
    }
}
