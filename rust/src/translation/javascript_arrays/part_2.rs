// Generated from complete js/src/translation/javascript-arrays.js. Do not edit by hand.
use super::MlTemporaryReadJavaScriptSemanticValueErrorLoop3Result;
use super::MlTemporaryTestJavaScriptSemanticTextEqualLoop4Result;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::ml_string;
use super::source_ml_temporary_read_java_script_semantic_value_error_loop3;
use super::source_ml_temporary_read_java_script_weak_key_eligibility_loop2;
use super::source_ml_temporary_test_java_script_semantic_text_equal_loop4;

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
pub enum JavaScriptArrayDefinitionState {
    ArrayDefinitionState(
        self::ml::Big,
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_standard_array_intrinsic_operation(
    operation: &self::ml_string::Text,
) -> bool {
    (*operation)
        == (self::ml_string::Text::from_units(vec![
            65, 114, 114, 97, 121, 46, 105, 115, 65, 114, 114, 97, 121,
        ]))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_validated_heap_snapshot(_heap: self::JavaScriptOrdinaryHeap) -> bool {
    false
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_weak_key_eligibility(
    heap: self::JavaScriptOrdinaryHeap,
    value: self::JavaScriptSemanticValue,
) -> self::ml_string::Text {
    match value.clone() {
        self::JavaScriptSemanticValue::Symbol(source_ml_source1_f7) => {
            let facts = if {
                let source_ml_source1_comparison_operand75 = match heap.clone() {
                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                        _,
                        _,
                        source_ml_source1_data_property_value74,
                        _,
                        _,
                        _,
                    ) => ((source_ml_source1_data_property_value74
                        .clone()
                        .unwrap_or_else(|| (None::<self::ml_string::Text>,)))
                    .0)
                        .clone(),
                };
                match (
                    source_ml_source1_comparison_operand75,
                    self::ml_string::Text::from_units(vec![
                        97, 99, 116, 117, 97, 108, 32, 103, 108, 111, 98, 97, 108, 32, 83, 121,
                        109, 98, 111, 108, 32, 114, 101, 103, 105, 115, 116, 114, 121, 32, 99, 108,
                        97, 115, 115, 105, 102, 105, 99, 97, 116, 105, 111, 110, 32, 49,
                    ]),
                ) {
                    (Some(ml_left), ml_right) => ml_left == ml_right,
                    _ => false,
                }
            } {
                (match heap.clone() {
                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                        _,
                        _,
                        _,
                        source_ml_source1_data_property_value77,
                        _,
                        _,
                    ) => ((source_ml_source1_data_property_value77
                        .clone()
                        .unwrap_or_else(|| {
                            (None::<
                                std::rc::Rc<
                                    std::cell::RefCell<Vec<self::JavaScriptSymbolRegistrationFact>>,
                                >,
                            >,)
                        }))
                    .0)
                        .clone(),
                })
                .clone()
                .unwrap_or_else(|| {
                    std::rc::Rc::new(std::cell::RefCell::new(Vec::<
                        self::JavaScriptSymbolRegistrationFact,
                    >::new()))
                })
            } else {
                std::rc::Rc::new(std::cell::RefCell::new(Vec::<
                    self::JavaScriptSymbolRegistrationFact,
                >::new()))
            };
            {
                let low = 0f64;
                {
                    let high = {
                        let ml_reference_array1 = facts.clone();
                        ml_reference_array1.borrow().len() as f64
                    };
                    {
                        let low_2 =
                            self::source_ml_temporary_read_java_script_weak_key_eligibility_loop2(
                                value.clone(),
                                facts.clone(),
                                low,
                                high,
                            );
                        {
                            let candidate = {
                                let ml_reference_array2 = facts.clone();
                                let ml_reference_position2 = low_2;
                                self::ml_array::optional_at(
                                    &*ml_reference_array2.borrow(),
                                    self::ml_array::number_index(ml_reference_position2),
                                )
                            };
                            {
                                let fact = if {
                                    let source_ml_source1_comparison_operand82 = {
                                        let source_ml_source1_optional_chain_receiver80 =
                                            (candidate.clone(),);
                                        if match (source_ml_source1_optional_chain_receiver80
                                            .clone())
                                        .0
                                        {
                                            Some(ml_present) => {
                                                let _ = ml_present.clone();
                                                true
                                            }
                                            None => false,
                                        } {
                                            Some({
                                                let source_ml_temporary_optional_chain1 =
                                                    ((source_ml_source1_optional_chain_receiver80
                                                        .clone())
                                                    .0)
                                                        .clone()
                                                        .expect("a proven present value");
                                                match source_ml_temporary_optional_chain1 {
                                                        self::JavaScriptSymbolRegistrationFact::SymbolRegistrationFact(source_ml_source1_data_property_value81, _, _) => {
                                                            source_ml_source1_data_property_value81.clone()
                                                        }
                                                    }
                                            })
                                        } else {
                                            None::<self::ml::Big>
                                        }
                                    };
                                    match (
                                        source_ml_source1_comparison_operand82,
                                        source_ml_source1_f7.clone(),
                                    ) {
                                        (Some(ml_left), ml_right) => ml_left == ml_right,
                                        _ => false,
                                    }
                                } {
                                    candidate.clone()
                                } else {
                                    None::<self::JavaScriptSymbolRegistrationFact>
                                };
                                if match (fact.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                        self::ml_string::Text::from_units(vec![109, 105, 115, 115, 105, 110, 103, 32, 115, 121, 109, 98, 111, 108, 32, 114, 101, 103, 105, 115, 116, 114, 97, 116, 105, 111, 110, 32, 97, 117, 116, 104, 111, 114, 105, 116, 121])
                                    } else if match fact.clone().expect("a proven present value") {
                                            self::JavaScriptSymbolRegistrationFact::SymbolRegistrationFact(_, source_ml_source1_data_property_value84, _) => {
                                                source_ml_source1_data_property_value84
                                            }
                                        } {
                                            self::ml_string::Text::from_units(vec![105, 110, 101, 108, 105, 103, 105, 98, 108, 101])
                                        } else {
                                            self::ml_string::Text::from_units(vec![101, 108, 105, 103, 105, 98, 108, 101])
                                        }
                            }
                        }
                    }
                }
            }
        }
        self::JavaScriptSemanticValue::Object(_, _) => {
            self::ml_string::Text::from_units(vec![101, 108, 105, 103, 105, 98, 108, 101])
        }
        _ => {
            self::ml_string::Text::from_units(vec![105, 110, 101, 108, 105, 103, 105, 98, 108, 101])
        }
    }
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
                let ml_reference_array3 = units.clone();
                ml_reference_array3.borrow().len() as f64
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
                        let source_ml_source1_s89 =
                            self::source_ml_temporary_read_java_script_semantic_value_error_loop3(
                                units.clone(),
                                at,
                            );
                        match source_ml_source1_s89 {
                            self::MlTemporaryReadJavaScriptSemanticValueErrorLoop3Result::MlTemporaryReadJavaScriptSemanticValueErrorLoop3Done => {
                                self::ml_string::Text::from_units(vec![])
                            }
                            self::MlTemporaryReadJavaScriptSemanticValueErrorLoop3Result::MlTemporaryReadJavaScriptSemanticValueErrorLoop3Return(source_ml_temporary_result3) => {
                                source_ml_temporary_result3.clone()
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
        self::JavaScriptSemanticValue::Text(source_ml_source1_f155) => {
            let ml_reference_array4 = source_ml_source1_f155.clone();
            ml_reference_array4.borrow().len() as f64
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
            self::JavaScriptSemanticValue::Text(source_ml_source1_f227) => ({
                let ml_reference_array5 = source_ml_source1_f227.clone();
                let ml_reference_position5 = index;
                self::ml_array::optional_at(
                    &*ml_reference_array5.borrow(),
                    self::ml_array::number_index(ml_reference_position5),
                )
            })
            .clone()
            .unwrap_or_else(|| -1f64),
            self::JavaScriptSemanticValue::SourceText(_, text) => {
                let source_ml_source1_string_unit_operand282 = text.clone();
                {
                    let source_ml_source1_string_unit_operand283 = index;
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
                                    let source_ml_source1_string_unit_operand289 = {
                                        let source_ml_source1_string_range_operand284 = text_2.clone();
                                        {
                                            let source_ml_source1_string_range_operand285 = index_2;
                                            {
                                                let source_ml_source1_string_range_operand286 = index_2 + 1f64;
                                                {
                                                    let source_ml_source1_string_range_position287 = source_ml_source1_string_range_operand285;
                                                    {
                                                        let source_ml_source1_string_range_position288 = source_ml_source1_string_range_operand286;
                                                        source_ml_source1_string_range_operand284.slice(source_ml_source1_string_range_position287, source_ml_source1_string_range_position288)
                                                    }
                                                }
                                            }
                                        }
                                    };
                                    {
                                        let source_ml_source1_string_unit_operand290 = 0f64;
                                        source_ml_source1_string_unit_operand289.code_point_at(source_ml_source1_string_unit_operand290)
                                    }
                                }).clone().unwrap_or_else(|| f64::NAN)
                            }
                        }
                    }))(
                        source_ml_source1_string_unit_operand282,
                        source_ml_source1_string_unit_operand283,
                    )
                }
            }
            _ => -1f64,
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
        if (length < 0f64)
            || (length != (self::read_java_script_semantic_text_length(right.clone())))
        {
            false
        } else if match left.clone() {
            self::JavaScriptSemanticValue::SourceText(_, _) => true,
            _ => false,
        } && match right.clone() {
            self::JavaScriptSemanticValue::SourceText(_, _) => true,
            _ => false,
        } {
            {
                let source_ml_source1_comparison_operand327 = match left.clone() {
                    self::JavaScriptSemanticValue::Undefined => None::<self::ml_string::Text>,
                    self::JavaScriptSemanticValue::Null => None::<self::ml_string::Text>,
                    self::JavaScriptSemanticValue::Boolean(_) => None::<self::ml_string::Text>,
                    self::JavaScriptSemanticValue::Number(_) => None::<self::ml_string::Text>,
                    self::JavaScriptSemanticValue::Integer(_) => None::<self::ml_string::Text>,
                    self::JavaScriptSemanticValue::Text(_) => None::<self::ml_string::Text>,
                    self::JavaScriptSemanticValue::SourceText(
                        _,
                        source_ml_source1_data_property_value325,
                    ) => Some(source_ml_source1_data_property_value325.clone()),
                    self::JavaScriptSemanticValue::Symbol(_) => None::<self::ml_string::Text>,
                    self::JavaScriptSemanticValue::Object(_, _) => None::<self::ml_string::Text>,
                };
                {
                    let source_ml_source1_comparison_operand328 = match right.clone() {
                        self::JavaScriptSemanticValue::Undefined => None::<self::ml_string::Text>,
                        self::JavaScriptSemanticValue::Null => None::<self::ml_string::Text>,
                        self::JavaScriptSemanticValue::Boolean(_) => None::<self::ml_string::Text>,
                        self::JavaScriptSemanticValue::Number(_) => None::<self::ml_string::Text>,
                        self::JavaScriptSemanticValue::Integer(_) => None::<self::ml_string::Text>,
                        self::JavaScriptSemanticValue::Text(_) => None::<self::ml_string::Text>,
                        self::JavaScriptSemanticValue::SourceText(
                            _,
                            source_ml_source1_data_property_value326,
                        ) => Some(source_ml_source1_data_property_value326.clone()),
                        self::JavaScriptSemanticValue::Symbol(_) => None::<self::ml_string::Text>,
                        self::JavaScriptSemanticValue::Object(_, _) => {
                            None::<self::ml_string::Text>
                        }
                    };
                    match (
                        source_ml_source1_comparison_operand327,
                        source_ml_source1_comparison_operand328,
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
                    let source_ml_source1_s329 =
                        self::source_ml_temporary_test_java_script_semantic_text_equal_loop4(
                            left.clone(),
                            right.clone(),
                            length,
                            at,
                        );
                    match source_ml_source1_s329 {
                            self::MlTemporaryTestJavaScriptSemanticTextEqualLoop4Result::MlTemporaryTestJavaScriptSemanticTextEqualLoop4Done => {
                                true
                            }
                            self::MlTemporaryTestJavaScriptSemanticTextEqualLoop4Result::MlTemporaryTestJavaScriptSemanticTextEqualLoop4Return(source_ml_temporary_result4) => {
                                source_ml_temporary_result4
                            }
                        }
                }
            }
        }
    }
}
