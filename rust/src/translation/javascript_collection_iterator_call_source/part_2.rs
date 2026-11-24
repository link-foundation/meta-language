// Generated from complete js/src/translation/javascript-collection-iterator-call-source.js. Do not edit by hand.
use super::MlTemporaryReadJavaScriptSemanticValueErrorLoop1Result;
use super::ml;
use super::ml_math;
use super::source_ml_temporary_read_java_script_semantic_value_error_loop1;

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
    Text(Vec<f64>),
    SourceText(self::ml_string::Text, self::ml_string::Text),
    Symbol(self::ml::Big),
    Object(self::ml::Big, bool),
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
        Vec<self::JavaScriptArrayConstructorFact>,
        Option<(Option<Vec<self::JavaScriptConstructorRealm>>,)>,
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

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptCallInstruction {
    Constant(f64, Box<self::JavaScriptSemanticValue>),
    Copy(f64, f64),
    Argument(f64, f64),
    ArgumentCount(f64),
    ArgumentAt(f64, f64),
    FunctionBind(f64),
    BoundFunctionCall(f64),
    FunctionToString(f64),
    PushFinally(f64),
    FinishFinally,
    CompleteJump(f64, f64),
    Jump(f64),
    Branch(f64, f64, f64),
    Call(f64, f64, f64, Vec<f64>),
    ArgumentListOpen(f64),
    ArgumentListAppend(f64, f64),
    ArgumentListCall(f64, f64, f64, f64, self::ml_string::Text),
    Construct(f64, f64, f64, Vec<f64>),
    ConstructorCheck(f64, f64, bool),
    ReadNewTarget(f64),
    ArrayJoinEnter(f64, f64),
    ArrayJoinLeave(f64),
    ConstructErrorClass(f64, f64, Vec<f64>),
    CallClassConstructor(f64, f64, f64, Vec<f64>),
    ConstructCollection(f64, f64, Vec<f64>),
    CollectionAllocate(f64, f64, self::ml_string::Text),
    CollectionIteratorAllocate(
        f64,
        f64,
        self::ml_string::Text,
        self::ml_string::Text,
        Option<(Option<f64>,)>,
    ),
    ArgumentRest(f64, f64, f64, self::ml::Big),
    IteratorProtocol(
        f64,
        f64,
        f64,
        f64,
        f64,
        self::ml_string::Text,
        self::ml::Big,
        Option<(Option<bool>,)>,
    ),
    CollectionIteratorResult(f64, self::ml_string::Text, f64, f64),
    CollectionOperation(f64, self::ml_string::Text),
    PrimitiveMethod(f64, self::ml_string::Text),
    CollectionForEach(f64, self::ml_string::Text),
    ArrayConcatenation(
        f64,
        Option<(Option<f64>,)>,
        f64,
        Box<self::JavaScriptArraySpeciesDirectory>,
    ),
    ArraySlice(
        f64,
        Option<(Option<f64>,)>,
        Box<self::JavaScriptArraySpeciesDirectory>,
    ),
    ArrayCreation(f64, self::ml_string::Text, f64, f64),
    ArrayCallback(f64, self::ml_string::Text, Option<(Option<f64>,)>),
    ArraySearch(f64, self::ml_string::Text, Option<(Option<f64>,)>),
    ArrayTransform(
        f64,
        self::ml_string::Text,
        Box<self::JavaScriptArraySpeciesDirectory>,
        Option<(Option<f64>,)>,
    ),
    ObjectEnumeration(f64, self::ml_string::Text, f64),
    PropertyDefinitions(f64, self::ml_string::Text, f64, f64),
    PropertyReflection(f64, self::ml_string::Text, f64, f64, f64, f64),
    InstanceOf(f64, f64, f64, f64, self::ml_string::Text),
    ObjectObservation(f64, self::ml_string::Text, f64, f64),
    CollectionCopy(f64, f64, f64, f64, f64, f64, f64, f64),
    ConstructErrorValue(
        f64,
        f64,
        f64,
        f64,
        f64,
        f64,
        self::ml::Big,
        self::ml_string::Text,
        bool,
    ),
    ReadDerivedReceiver(f64),
    InvokeErrorIntrinsic(f64),
    InvokeHostCapability(f64),
    Return(f64),
    Throw(f64),
    PushHandler(f64, f64),
    PopHandler,
    NumberBinary(f64, f64, f64, self::ml_string::Text),
    SameValue(f64, f64, f64),
    StrictEqual(f64, f64, f64),
    RequireWrite(f64),
    Primitive(f64, self::ml_string::Text, Vec<f64>),
    OrdinaryAllocate(f64, f64),
    LiteralPrototype(f64, f64, f64),
    LiteralAccessor(f64, f64, f64, f64, self::ml_string::Text, self::ml::Big),
    ObjectFromEntries(f64, f64, f64, f64),
    PrimitiveWrapper(f64, f64, f64),
    SymbolAllocate(f64, f64),
    SymbolDescription(f64, f64),
    SymbolRegistry(f64, f64, self::ml_string::Text),
    FunctionName(f64, f64, f64, self::ml_string::Text, self::ml::Big),
    ArrayAllocate(f64, f64, f64),
    ClosureAllocate(f64, self::ml::Big, f64, Vec<f64>, f64),
    ArraySpread(f64, f64, f64, f64, f64, self::ml::Big),
    ObjectSpread(
        f64,
        f64,
        f64,
        self::ml::Big,
        Option<(Option<Vec<f64>>,)>,
        Option<(Option<self::ml_string::Text>,)>,
    ),
    OrdinaryDataProperty(f64, f64, f64, f64, self::ml::Big),
    OrdinaryGet(f64, f64, f64, f64),
    OrdinaryDelete(f64, f64, f64),
    OrdinaryHas(f64, f64, f64),
    OrdinarySet(f64, f64, f64, f64, f64),
    OrdinaryDefine(f64, f64, f64, Vec<f64>, self::ml::Big),
    PropertyKey(f64, f64, self::ml::Big),
    PropertyDefinition(
        f64,
        f64,
        f64,
        f64,
        self::ml::Big,
        Option<(Option<self::ml_string::Text>,)>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptCallFunction {
    CallFunction(
        self::ml::Big,
        self::ml::Big,
        self::ml_string::Text,
        Box<self::JavaScriptSemanticValue>,
        f64,
        f64,
        Vec<self::JavaScriptSemanticValue>,
        Vec<self::JavaScriptCallInstruction>,
        Option<(Option<self::JavaScriptSemanticValue>,)>,
        Option<(Option<self::JavaScriptCallSourceSpan>,)>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptCallTemplate {
    CallTemplate(
        self::ml::Big,
        f64,
        self::ml::Big,
        f64,
        f64,
        f64,
        Vec<self::JavaScriptCallInstruction>,
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
        f64,
        Option<(Option<self::ml_string::Text>,)>,
        Option<(Option<self::JavaScriptCallSourceSpan>,)>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptCallProgram {
    CallProgram(
        Vec<self::JavaScriptCallFunction>,
        Vec<self::ml::Big>,
        Box<self::JavaScriptSemanticValue>,
        Option<(Option<Vec<self::JavaScriptCallTemplate>>,)>,
        Option<(Option<self::JavaScriptArraySpeciesDirectory>,)>,
        Option<(Option<Vec<self::JavaScriptCallSourceUnit>>,)>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptCollectionIteratorMethodOwner {
    CollectionIteratorMethodOwner(
        self::ml::Big,
        self::ml::Big,
        self::ml_string::Text,
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptCollectionIteratorCallableDirectory {
    CollectionIteratorCallableDirectory(
        Vec<self::ml::Big>,
        Box<self::JavaScriptSemanticValue>,
        Vec<self::JavaScriptCollectionIteratorMethodOwner>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptCollectionIteratorCallCompilation {
    CollectionIteratorCallCompilation(Box<self::JavaScriptCallProgram>, self::ml_string::Text),
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
