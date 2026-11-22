// Generated from complete js/src/translation/javascript-call-templates.js. Do not edit by hand.
use super::MlTemporaryReadJavaScriptSemanticValueErrorLoop1Result;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::ml_string;
use super::source_ml_temporary_read_java_script_semantic_value_error_loop1;
use super::test_java_script_semantic_text_equal;

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
    Call(f64, f64, f64, std::rc::Rc<std::cell::RefCell<Vec<f64>>>),
    ArgumentListOpen(f64),
    ArgumentListAppend(f64, f64),
    ArgumentListCall(f64, f64, f64, f64, self::ml_string::Text),
    Construct(f64, f64, f64, std::rc::Rc<std::cell::RefCell<Vec<f64>>>),
    ConstructorCheck(f64, f64, bool),
    ReadNewTarget(f64),
    ArrayJoinEnter(f64, f64),
    ArrayJoinLeave(f64),
    ConstructErrorClass(f64, f64, std::rc::Rc<std::cell::RefCell<Vec<f64>>>),
    CallClassConstructor(f64, f64, f64, std::rc::Rc<std::cell::RefCell<Vec<f64>>>),
    ConstructCollection(f64, f64, std::rc::Rc<std::cell::RefCell<Vec<f64>>>),
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
    Primitive(
        f64,
        self::ml_string::Text,
        std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    ),
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
    ClosureAllocate(
        f64,
        self::ml::Big,
        f64,
        std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
        f64,
    ),
    ArraySpread(f64, f64, f64, f64, f64, self::ml::Big),
    ObjectSpread(
        f64,
        f64,
        f64,
        self::ml::Big,
        Option<(Option<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>,)>,
        Option<(Option<self::ml_string::Text>,)>,
    ),
    OrdinaryDataProperty(f64, f64, f64, f64, self::ml::Big),
    OrdinaryGet(f64, f64, f64, f64),
    OrdinaryDelete(f64, f64, f64),
    OrdinaryHas(f64, f64, f64),
    OrdinarySet(f64, f64, f64, f64, f64),
    OrdinaryDefine(
        f64,
        f64,
        f64,
        std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
        self::ml::Big,
    ),
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
        std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
        std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallInstruction>>>,
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
        std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallInstruction>>>,
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
        std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallFunction>>>,
        std::rc::Rc<std::cell::RefCell<Vec<self::ml::Big>>>,
        Box<self::JavaScriptSemanticValue>,
        Option<(Option<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallTemplate>>>>,)>,
        Option<(Option<self::JavaScriptArraySpeciesDirectory>,)>,
        Option<(Option<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceUnit>>>>,)>,
    ),
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
pub enum JavaScriptClosureAllocation {
    ClosureAllocationNormal(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptSemanticValue>,
    ),
    ClosureAllocationInvalid(Box<self::JavaScriptOrdinaryHeap>, self::ml_string::Text),
    ClosureAllocationResourceLimit(Box<self::JavaScriptOrdinaryHeap>, self::ml_string::Text),
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
            self::read_java_script_source_text_profile_error(&(profile), text.length())
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
        self::JavaScriptSemanticValue::Object(identity_2, _) => {
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
        _ => self::ml_string::Text::from_units(vec![]),
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
                _ => false,
            },
            self::JavaScriptSemanticValue::Null => match right.clone() {
                self::JavaScriptSemanticValue::Null => true,
                _ => false,
            },
            self::JavaScriptSemanticValue::Boolean(left_value) => match right.clone() {
                self::JavaScriptSemanticValue::Boolean(value) => left_value == value,
                _ => false,
            },
            self::JavaScriptSemanticValue::Number(left_value_2) => match right.clone() {
                self::JavaScriptSemanticValue::Number(value_2) => {
                    if left_value_2 == value_2 {
                        (left_value_2 != 0f64) || ((1f64 / left_value_2) == (1f64 / value_2))
                    } else {
                        left_value_2.is_nan() && value_2.is_nan()
                    }
                }
                _ => false,
            },
            self::JavaScriptSemanticValue::Integer(left_value_3) => match right.clone() {
                self::JavaScriptSemanticValue::Integer(value_3) => left_value_3 == value_3,
                _ => false,
            },
            self::JavaScriptSemanticValue::Text(_) => {
                self::test_java_script_semantic_text_equal(left.clone(), right.clone())
            }
            self::JavaScriptSemanticValue::SourceText(_, _) => {
                self::test_java_script_semantic_text_equal(left.clone(), right.clone())
            }
            self::JavaScriptSemanticValue::Symbol(left_identity) => match right.clone() {
                self::JavaScriptSemanticValue::Symbol(identity) => left_identity == identity,
                _ => false,
            },
            self::JavaScriptSemanticValue::Object(left_identity_2, _) => match right.clone() {
                self::JavaScriptSemanticValue::Object(identity_2, _) => {
                    left_identity_2 == identity_2
                }
                _ => false,
            },
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_semantic_text_length(value: self::JavaScriptSemanticValue) -> f64 {
    match value.clone() {
        self::JavaScriptSemanticValue::Text(source_ml_source1_f71) => {
            let ml_reference_array2 = source_ml_source1_f71.clone();
            ml_reference_array2.borrow().len() as f64
        }
        self::JavaScriptSemanticValue::SourceText(_, text) => text.length(),
        _ => -1f64,
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_semantic_text_unit(
    value: self::JavaScriptSemanticValue,
    index: f64,
) -> f64 {
    if (!(self::ml_math::is_safe_integer(index)) || (index < 0f64))
        || (index >= (self::read_java_script_semantic_text_length(value.clone())))
    {
        -1f64
    } else {
        match value.clone() {
            self::JavaScriptSemanticValue::Text(source_ml_source1_f143) => ({
                let ml_reference_array3 = source_ml_source1_f143.clone();
                let ml_reference_position3 = index;
                self::ml_array::optional_at(
                    &*ml_reference_array3.borrow(),
                    self::ml_array::number_index(ml_reference_position3),
                )
            })
            .clone()
            .unwrap_or_else(|| -1f64),
            self::JavaScriptSemanticValue::SourceText(_, text) => {
                let source_ml_source1_string_unit_operand198 = text.clone();
                {
                    let source_ml_source1_string_unit_operand199 = index;
                    (std::rc::Rc::new(move |text_2: self::ml_string::Text, position: f64| -> f64 {
                        {
                            let index_2 = if position.is_nan() {
                                0f64
                            } else {
                                position.trunc()
                            };
                            if (index_2 < 0f64) || (index_2 >= (text_2.length())) {
                                f64::NAN
                            } else {
                                ({
                                    let source_ml_source1_string_unit_operand205 = {
                                        let source_ml_source1_string_range_operand200 = text_2.clone();
                                        {
                                            let source_ml_source1_string_range_operand201 = index_2;
                                            {
                                                let source_ml_source1_string_range_operand202 = index_2 + 1f64;
                                                {
                                                    let source_ml_source1_string_range_position203 = source_ml_source1_string_range_operand201;
                                                    {
                                                        let source_ml_source1_string_range_position204 = source_ml_source1_string_range_operand202;
                                                        source_ml_source1_string_range_operand200.slice(source_ml_source1_string_range_position203, source_ml_source1_string_range_position204)
                                                    }
                                                }
                                            }
                                        }
                                    };
                                    {
                                        let source_ml_source1_string_unit_operand206 = 0f64;
                                        source_ml_source1_string_unit_operand205.code_point_at(source_ml_source1_string_unit_operand206)
                                    }
                                }).clone().unwrap_or_else(|| f64::NAN)
                            }
                        }
                    }))(
                        source_ml_source1_string_unit_operand198,
                        source_ml_source1_string_unit_operand199,
                    )
                }
            }
            _ => -1f64,
        }
    }
}
