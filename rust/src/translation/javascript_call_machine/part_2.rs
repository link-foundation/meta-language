// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::ml;

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

/// A nullable value retains the distinct JavaScript null and undefined cases.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlOptional<T> {
    Value(T),
    Null,
    Undefined,
}

#[path = "ml_string_number.rs"]
pub mod ml_string_number;

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

#[path = "ml_big_integer_conversion.rs"]
pub mod ml_big_integer_conversion;

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
pub enum JavaScriptCallPrimitiveResult {
    PrimitiveValue(Box<self::JavaScriptSemanticValue>),
    PrimitiveInvalid(self::ml_string::Text),
    PrimitiveError(self::ml_string::Text, self::ml_string::Text),
    PrimitiveResourceLimit(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptOrdinaryProperty {
    OrdinaryProperty(
        Box<self::JavaScriptSemanticValue>,
        std::rc::Rc<
            std::cell::RefCell<(
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
                Vec<f64>,
            )>,
        >,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptCollectionEntry {
    CollectionEntry(
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
        bool,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptWeakCollectionEntry {
    WeakCollectionEntry(
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptClosureObject {
    ClosureObject(
        self::ml::Big,
        Box<self::JavaScriptSemanticValue>,
        bool,
        bool,
        std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>>,
        self::ml::Big,
        f64,
        std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
        Box<self::JavaScriptSemanticValue>,
        Option<(Option<self::JavaScriptSemanticValue>,)>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptBoundFunctionObject {
    BoundFunctionObject(
        self::ml::Big,
        Box<self::JavaScriptSemanticValue>,
        bool,
        bool,
        std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>>,
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
        std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
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
pub enum JavaScriptOrdinaryObject {
    ClosureObject(
        self::ml::Big,
        Box<self::JavaScriptSemanticValue>,
        bool,
        bool,
        std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>>,
        self::ml::Big,
        f64,
        std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
        Box<self::JavaScriptSemanticValue>,
        Option<(Option<self::JavaScriptSemanticValue>,)>,
    ),
    BoundFunctionObject(
        self::ml::Big,
        Box<self::JavaScriptSemanticValue>,
        bool,
        bool,
        std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>>,
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
        std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
    ),
    ArrayObject(
        self::ml::Big,
        Box<self::JavaScriptSemanticValue>,
        bool,
        bool,
        std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>>,
    ),
    CollectionIteratorObject(
        self::ml::Big,
        Box<self::JavaScriptSemanticValue>,
        bool,
        bool,
        std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>>,
        self::ml::Big,
        f64,
        self::ml_string::Text,
        self::ml_string::Text,
        Option<(Option<bool>,)>,
        Option<(Option<self::JavaScriptSemanticValue>,)>,
    ),
    MapObject(
        self::ml::Big,
        Box<self::JavaScriptSemanticValue>,
        bool,
        bool,
        std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>>,
        std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCollectionEntry>>>,
    ),
    SetObject(
        self::ml::Big,
        Box<self::JavaScriptSemanticValue>,
        bool,
        bool,
        std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>>,
        std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCollectionEntry>>>,
    ),
    WeakMapObject(
        self::ml::Big,
        Box<self::JavaScriptSemanticValue>,
        bool,
        bool,
        std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>>,
        std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptWeakCollectionEntry>>>,
    ),
    WeakSetObject(
        self::ml::Big,
        Box<self::JavaScriptSemanticValue>,
        bool,
        bool,
        std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>>,
        std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptWeakCollectionEntry>>>,
    ),
    OrdinaryObject(
        self::ml::Big,
        Box<self::JavaScriptSemanticValue>,
        bool,
        Option<(Option<bool>,)>,
        Option<(Option<bool>,)>,
        Option<(Option<self::JavaScriptSemanticValue>,)>,
        Option<(Option<self::JavaScriptSemanticValue>,)>,
        bool,
        std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>>,
    ),
    HostErrorObject(
        self::ml::Big,
        Box<self::JavaScriptSemanticValue>,
        bool,
        bool,
        std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>>,
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
    ),
    HostURLObject(
        self::ml::Big,
        Box<self::JavaScriptSemanticValue>,
        bool,
        bool,
        std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>>,
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
    ),
    HostArrayObject(
        self::ml::Big,
        Box<self::JavaScriptSemanticValue>,
        bool,
        bool,
        std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>>,
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
    ),
    HostByteBufferObject(
        self::ml::Big,
        Box<self::JavaScriptSemanticValue>,
        bool,
        bool,
        std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>>,
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptSymbolRegistrationFact {
    SymbolRegistrationFact(
        self::ml::Big,
        bool,
        Option<(Option<self::JavaScriptSemanticValue>,)>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptSymbolRegistryEntry {
    SymbolRegistryEntry(self::ml::Big, Box<self::JavaScriptSemanticValue>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptOrdinaryHeap {
    OrdinaryHeap(
        std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryObject>>>,
        std::rc::Rc<std::cell::RefCell<Vec<self::ml::Big>>>,
        Option<(Option<self::ml_string::Text>,)>,
        Option<(
            Option<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSymbolRegistrationFact>>>>,
        )>,
        Option<(Option<self::ml_string::Text>,)>,
        Option<(
            Option<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSymbolRegistryEntry>>>>,
        )>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptArrayPropertyPlan {
    ArrayPropertyPlan(
        Box<self::JavaScriptOrdinaryObject>,
        bool,
        self::ml_string::Text,
        self::ml_string::Text,
        std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptArrayDefinitionState {
    ArrayDefinitionState(
        self::ml::Big,
        Box<self::JavaScriptSemanticValue>,
        std::rc::Rc<
            std::cell::RefCell<(
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
                Vec<f64>,
            )>,
        >,
        self::ml_string::Text,
        f64,
        Box<self::JavaScriptSemanticValue>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptArrayCompletion {
    ArrayNormal(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptSemanticValue>,
    ),
    ArrayPropertyComplete(
        Box<self::JavaScriptOrdinaryHeap>,
        bool,
        std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>,
    ),
    ArrayNumberRequest(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptArrayDefinitionState>,
        Box<self::JavaScriptSemanticValue>,
    ),
    ArrayDefinitionContinue(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptArrayDefinitionState>,
    ),
    ArrayInvalid(Box<self::JavaScriptOrdinaryHeap>, self::ml_string::Text),
    ArrayError(
        Box<self::JavaScriptOrdinaryHeap>,
        self::ml_string::Text,
        self::ml_string::Text,
    ),
    ArrayResourceLimit(
        Box<self::JavaScriptOrdinaryHeap>,
        self::ml_string::Text,
        Option<(Option<self::JavaScriptArrayDefinitionState>,)>,
    ),
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
        std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
    ),
    PrimitiveCompleted(Box<self::JavaScriptSemanticValue>),
    PrimitiveThrow(Box<self::JavaScriptSemanticValue>),
    PrimitiveIntrinsicError(self::ml_string::Text, self::ml_string::Text),
    PrimitiveInvalid(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptScalarConversionTransition {
    ScalarContinuation(Box<self::JavaScriptPrimitiveConversionState>),
    ScalarCall(
        Box<self::JavaScriptPrimitiveConversionState>,
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
        std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
    ),
    ScalarCompleted(Box<self::JavaScriptSemanticValue>),
    ScalarThrow(Box<self::JavaScriptSemanticValue>),
    ScalarIntrinsicError(self::ml_string::Text, self::ml_string::Text),
    ScalarInvalid(self::ml_string::Text),
    ScalarResourceLimit(Box<self::JavaScriptSemanticValue>, self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum JavaScriptArrayConstructorFact {
    ArrayConstructorFact(
        self::ml::Big,
        bool,
        self::ml::Big,
        bool,
        Option<(Option<self::ml_string::Text>,)>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptConstructorRealm {
    ConstructorRealm(
        self::ml::Big,
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
        Option<(Option<self::JavaScriptSemanticValue>,)>,
        Option<(Option<self::JavaScriptSemanticValue>,)>,
        Option<(Option<self::JavaScriptSemanticValue>,)>,
        Option<(Option<self::JavaScriptSemanticValue>,)>,
        Option<(Option<self::JavaScriptSemanticValue>,)>,
        Option<(Option<self::JavaScriptSemanticValue>,)>,
        Option<(Option<self::JavaScriptSemanticValue>,)>,
        Option<(Option<self::JavaScriptSemanticValue>,)>,
        Option<(Option<self::JavaScriptSemanticValue>,)>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptArraySpeciesDirectory {
    ArraySpeciesDirectory(
        self::ml::Big,
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
        std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptArrayConstructorFact>>>,
        Option<(Option<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptConstructorRealm>>>>,)>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptCallSourceUnit {
    CallSourceUnit(self::ml_string::Text, Box<self::JavaScriptSemanticValue>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptCallSourceSpan {
    CallSourceSpan(self::ml_string::Text, f64, f64),
}
