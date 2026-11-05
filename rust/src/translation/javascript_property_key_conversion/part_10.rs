// Generated from complete js/src/translation/javascript-property-key-conversion.js. Do not edit by hand.
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptPrimitiveConversionCompletion;
use super::JavaScriptPrimitiveConversionState;
use super::JavaScriptPrimitiveConversionTransition;
use super::JavaScriptPropertyKeyConversionTransition;
use super::JavaScriptSemanticValue;
use super::apply_java_script_primitive_conversion_method;
use super::begin_java_script_primitive_conversion;
use super::continue_java_script_primitive_conversion;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::ml_number;
use super::ml_source_exception;
use super::ml_string;
use super::read_java_script_ordinary_value_error;
use super::read_java_script_primitive_conversion_state_error;
use super::source_ml_temporary_finish_java_script_property_key_conversion_loop29;
use super::step_java_script_primitive_conversion;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn resume_java_script_primitive_conversion(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptPrimitiveConversionState,
    completion: self::JavaScriptPrimitiveConversionCompletion,
) -> self::JavaScriptPrimitiveConversionTransition {
    {
        let error =
            self::read_java_script_primitive_conversion_state_error(heap.clone(), state.clone());
        {
            let ml_library_inequality_2 = error != (self::ml_string::Text::from_units(vec![]));
            if ml_library_inequality_2 {
                self::JavaScriptPrimitiveConversionTransition::PrimitiveInvalid(error.clone())
            } else {
                {
                    let getter = (((match state.clone() {
                        self::JavaScriptPrimitiveConversionState::PrimitiveConversion(
                            _,
                            _,
                            _,
                            source_ml_source1_data_property_value879,
                        ) => source_ml_source1_data_property_value879.clone(),
                    }) == (self::ml_string::Text::from_units(vec![
                        101, 120, 111, 116, 105, 99, 71, 101, 116, 116, 101, 114,
                    ]))) || ((match state.clone() {
                        self::JavaScriptPrimitiveConversionState::PrimitiveConversion(
                            _,
                            _,
                            _,
                            source_ml_source1_data_property_value881,
                        ) => source_ml_source1_data_property_value881.clone(),
                    }) == (self::ml_string::Text::from_units(vec![
                        102, 105, 114, 115, 116, 71, 101, 116, 116, 101, 114,
                    ]))))
                        || ((match state.clone() {
                            self::JavaScriptPrimitiveConversionState::PrimitiveConversion(
                                _,
                                _,
                                _,
                                source_ml_source1_data_property_value883,
                            ) => source_ml_source1_data_property_value883.clone(),
                        }) == (self::ml_string::Text::from_units(vec![
                            115, 101, 99, 111, 110, 100, 71, 101, 116, 116, 101, 114,
                        ])));
                    {
                        let call = (((match state.clone() {
                            self::JavaScriptPrimitiveConversionState::PrimitiveConversion(
                                _,
                                _,
                                _,
                                source_ml_source1_data_property_value885,
                            ) => source_ml_source1_data_property_value885.clone(),
                        }) == (self::ml_string::Text::from_units(vec![
                            101, 120, 111, 116, 105, 99, 67, 97, 108, 108,
                        ]))) || ((match state.clone() {
                            self::JavaScriptPrimitiveConversionState::PrimitiveConversion(
                                _,
                                _,
                                _,
                                source_ml_source1_data_property_value887,
                            ) => source_ml_source1_data_property_value887.clone(),
                        }) == (self::ml_string::Text::from_units(vec![
                            102, 105, 114, 115, 116, 67, 97, 108, 108,
                        ]))))
                            || ((match state.clone() {
                                self::JavaScriptPrimitiveConversionState::PrimitiveConversion(
                                    _,
                                    _,
                                    _,
                                    source_ml_source1_data_property_value889,
                                ) => source_ml_source1_data_property_value889.clone(),
                            }) == (self::ml_string::Text::from_units(vec![
                                115, 101, 99, 111, 110, 100, 67, 97, 108, 108,
                            ])));
                        if ((getter) || (call)) {
                            {
                                let value_error = self::read_java_script_ordinary_value_error(heap.clone(), match completion.clone() {
                                self::JavaScriptPrimitiveConversionCompletion::NormalCompletion(source_ml_source1_data_property_value890) => {
                                    let source_ml_source1_data_property_value890 = *source_ml_source1_data_property_value890;
                                    source_ml_source1_data_property_value890.clone()
                                }
                                self::JavaScriptPrimitiveConversionCompletion::ThrowCompletion(source_ml_source1_data_property_value891) => {
                                    let source_ml_source1_data_property_value891 = *source_ml_source1_data_property_value891;
                                    source_ml_source1_data_property_value891.clone()
                                }
                            });
                                {
                                    let ml_library_inequality_1 =
                                        value_error != (self::ml_string::Text::from_units(vec![]));
                                    if ml_library_inequality_1 {
                                        self::JavaScriptPrimitiveConversionTransition::PrimitiveInvalid(value_error.clone())
                                    } else {
                                        match completion.clone() {
                                    self::JavaScriptPrimitiveConversionCompletion::NormalCompletion(value) => {
                                        let value = *value;
                                        if getter {
                                            self::apply_java_script_primitive_conversion_method(state.clone(), value.clone())
                                        } else if match value.clone() {
                                                self::JavaScriptSemanticValue::Undefined => {
                                                    true
                                                }
                                                self::JavaScriptSemanticValue::Null => {
                                                    true
                                                }
                                                self::JavaScriptSemanticValue::Boolean(source_ml_source1_f892) => {
                                                    true
                                                }
                                                self::JavaScriptSemanticValue::Number(source_ml_source1_f893) => {
                                                    true
                                                }
                                                self::JavaScriptSemanticValue::Integer(source_ml_source1_f894) => {
                                                    true
                                                }
                                                self::JavaScriptSemanticValue::Text(source_ml_source1_f895) => {
                                                    true
                                                }
                                                self::JavaScriptSemanticValue::Symbol(source_ml_source1_f896) => {
                                                    true
                                                }
                                                self::JavaScriptSemanticValue::Object(source_ml_source1_f897, source_ml_source1_f898) => {
                                                    false
                                                }
                                            } {
                                                self::JavaScriptPrimitiveConversionTransition::PrimitiveCompleted(Box::new(value.clone()))
                                            } else if (match state.clone() {
                                                    self::JavaScriptPrimitiveConversionState::PrimitiveConversion(_, _, _, source_ml_source1_data_property_value900) => {
                                                        source_ml_source1_data_property_value900.clone()
                                                    }
                                                }) == (self::ml_string::Text::from_units(vec![102, 105, 114, 115, 116, 67, 97, 108, 108])) {
                                                    self::continue_java_script_primitive_conversion(state.clone(), &(self::ml_string::Text::from_units(vec![115, 101, 99, 111, 110, 100, 71, 101, 116])))
                                                } else {
                                                    self::JavaScriptPrimitiveConversionTransition::PrimitiveIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), if (match state.clone() {
                                                        self::JavaScriptPrimitiveConversionState::PrimitiveConversion(_, _, _, source_ml_source1_data_property_value902) => {
                                                            source_ml_source1_data_property_value902.clone()
                                                        }
                                                    }) == (self::ml_string::Text::from_units(vec![101, 120, 111, 116, 105, 99, 67, 97, 108, 108])) {
                                                        self::ml_string::Text::from_units(vec![97, 110, 32, 101, 120, 111, 116, 105, 99, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 109, 117, 115, 116, 32, 114, 101, 116, 117, 114, 110, 32, 97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101])
                                                    } else {
                                                        self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 101, 120, 104, 97, 117, 115, 116, 101, 100, 32, 105, 116, 115, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 45, 112, 114, 111, 100, 117, 99, 105, 110, 103, 32, 109, 101, 116, 104, 111, 100, 115])
                                                    })
                                                }
                                    }
                                    self::JavaScriptPrimitiveConversionCompletion::ThrowCompletion(value_2) => {
                                        let value_2 = *value_2;
                                        self::JavaScriptPrimitiveConversionTransition::PrimitiveThrow(Box::new(value_2.clone()))
                                    }
                                }
                                    }
                                }
                            }
                        } else {
                            self::JavaScriptPrimitiveConversionTransition::PrimitiveInvalid(
                                self::ml_string::Text::from_units(vec![
                                    112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 114, 101, 115,
                                    117, 109, 112, 116, 105, 111, 110, 32, 110, 101, 101, 100, 115,
                                    32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 101,
                                    110, 100, 105, 110, 103, 32, 103, 101, 116, 116, 101, 114, 32,
                                    111, 114, 32, 109, 101, 116, 104, 111, 100,
                                ]),
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
pub fn read_java_script_primitive_property_key_spelling(
    value: self::JavaScriptSemanticValue,
) -> self::ml_string::Text {
    match value.clone() {
        self::JavaScriptSemanticValue::Undefined => {
            self::ml_string::Text::from_units(vec![117, 110, 100, 101, 102, 105, 110, 101, 100])
        }
        self::JavaScriptSemanticValue::Null => {
            self::ml_string::Text::from_units(vec![110, 117, 108, 108])
        }
        self::JavaScriptSemanticValue::Boolean(value_value) => {
            if value_value {
                self::ml_string::Text::from_units(vec![116, 114, 117, 101])
            } else {
                self::ml_string::Text::from_units(vec![102, 97, 108, 115, 101])
            }
        }
        self::JavaScriptSemanticValue::Number(value_value_2) => {
            let source_ml_source1_string_argument907 = value_value_2;
            self::ml_string::Text::from_text(&self::ml_number::js_number(
                source_ml_source1_string_argument907,
            ))
        }
        self::JavaScriptSemanticValue::Integer(value_value_3) => {
            let source_ml_source1_string_argument908 = value_value_3.clone();
            self::ml_string::Text::from_text(&source_ml_source1_string_argument908.to_string())
        }
        self::JavaScriptSemanticValue::Text(source_ml_source1_f903) => {
            (self::ml_source_exception::SourceException {
                class_name: String::from("TypeError"),
                message_units: (self::ml_string::Text::from_units(vec![
                    115, 101, 109, 97, 110, 116, 105, 99, 32, 116, 101, 120, 116, 32, 107, 101,
                    121, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32,
                    101, 120, 97, 99, 116, 32, 117, 110, 105, 116, 115,
                ]))
                .0,
                message_present: true,
            })
            .raise()
        }
        self::JavaScriptSemanticValue::Symbol(source_ml_source1_f904) => {
            (self::ml_source_exception::SourceException {
                class_name: String::from("TypeError"),
                message_units: (self::ml_string::Text::from_units(vec![
                    115, 101, 109, 97, 110, 116, 105, 99, 32, 83, 121, 109, 98, 111, 108, 32, 107,
                    101, 121, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114,
                    32, 101, 120, 97, 99, 116, 32, 105, 100, 101, 110, 116, 105, 116, 121,
                ]))
                .0,
                message_present: true,
            })
            .raise()
        }
        self::JavaScriptSemanticValue::Object(source_ml_source1_f905, source_ml_source1_f906) => {
            (self::ml_source_exception::SourceException {
                class_name: String::from("TypeError"),
                message_units: (self::ml_string::Text::from_units(vec![
                    97, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 107, 101, 121, 32, 110,
                    101, 101, 100, 115, 32, 105, 116, 115, 32, 99, 111, 109, 112, 108, 101, 116,
                    101, 100, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 105, 109, 105, 116, 105,
                    118, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110,
                ]))
                .0,
                message_present: true,
            })
            .raise()
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn finish_java_script_property_key_conversion(
    transition: self::JavaScriptPrimitiveConversionTransition,
) -> self::JavaScriptPropertyKeyConversionTransition {
    match transition.clone() {
        self::JavaScriptPrimitiveConversionTransition::PrimitiveContinuation(state) => {
            let state = *state;
            self::JavaScriptPropertyKeyConversionTransition::PropertyKeyContinuation(Box::new(
                state.clone(),
            ))
        }
        self::JavaScriptPrimitiveConversionTransition::PrimitiveCall(
            state_2,
            callee,
            receiver,
            arguments,
        ) => {
            let state_2 = *state_2;
            let callee = *callee;
            let receiver = *receiver;
            self::JavaScriptPropertyKeyConversionTransition::PropertyKeyCall(
                Box::new(state_2.clone()),
                Box::new(callee.clone()),
                Box::new(receiver.clone()),
                arguments,
            )
        }
        self::JavaScriptPrimitiveConversionTransition::PrimitiveCompleted(value) => {
            let value = *value;
            if match value.clone() {
                self::JavaScriptSemanticValue::Undefined => false,
                self::JavaScriptSemanticValue::Null => false,
                self::JavaScriptSemanticValue::Boolean(source_ml_source1_f909) => false,
                self::JavaScriptSemanticValue::Number(source_ml_source1_f910) => false,
                self::JavaScriptSemanticValue::Integer(source_ml_source1_f911) => false,
                self::JavaScriptSemanticValue::Text(source_ml_source1_f912) => true,
                self::JavaScriptSemanticValue::Symbol(source_ml_source1_f913) => false,
                self::JavaScriptSemanticValue::Object(
                    source_ml_source1_f914,
                    source_ml_source1_f915,
                ) => false,
            } || match value.clone() {
                self::JavaScriptSemanticValue::Undefined => false,
                self::JavaScriptSemanticValue::Null => false,
                self::JavaScriptSemanticValue::Boolean(source_ml_source1_f916) => false,
                self::JavaScriptSemanticValue::Number(source_ml_source1_f917) => false,
                self::JavaScriptSemanticValue::Integer(source_ml_source1_f918) => false,
                self::JavaScriptSemanticValue::Text(source_ml_source1_f919) => false,
                self::JavaScriptSemanticValue::Symbol(source_ml_source1_f920) => true,
                self::JavaScriptSemanticValue::Object(
                    source_ml_source1_f921,
                    source_ml_source1_f922,
                ) => false,
            } {
                self::JavaScriptPropertyKeyConversionTransition::PropertyKeyCompleted(Box::new(
                    value.clone(),
                ))
            } else {
                match value.clone() {
                    self::JavaScriptSemanticValue::Undefined => {
                        let spelling =
                            self::read_java_script_primitive_property_key_spelling(value.clone());
                        if (spelling.length()) > 1_048_576f64 {
                            self::JavaScriptPropertyKeyConversionTransition::PropertyKeyInvalid(
                                self::ml_string::Text::from_units(vec![
                                    112, 114, 111, 112, 101, 114, 116, 121, 32, 107, 101, 121, 32,
                                    116, 101, 120, 116, 32, 101, 120, 99, 101, 101, 100, 115, 32,
                                    116, 104, 101, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32,
                                    85, 84, 70, 49, 54, 32, 117, 110, 105, 116, 32, 98, 117, 100,
                                    103, 101, 116,
                                ]),
                            )
                        } else {
                            {
                                let units = Vec::<f64>::new();
                                {
                                    let index = 0f64;
                                    {
                                        let units_2 = self::source_ml_temporary_finish_java_script_property_key_conversion_loop29(&(spelling), units, index);
                                        self::JavaScriptPropertyKeyConversionTransition::PropertyKeyCompleted(Box::new(self::JavaScriptSemanticValue::Text(units_2)))
                                    }
                                }
                            }
                        }
                    }
                    self::JavaScriptSemanticValue::Null => {
                        let spelling_2 =
                            self::read_java_script_primitive_property_key_spelling(value.clone());
                        if (spelling_2.length()) > 1_048_576f64 {
                            self::JavaScriptPropertyKeyConversionTransition::PropertyKeyInvalid(
                                self::ml_string::Text::from_units(vec![
                                    112, 114, 111, 112, 101, 114, 116, 121, 32, 107, 101, 121, 32,
                                    116, 101, 120, 116, 32, 101, 120, 99, 101, 101, 100, 115, 32,
                                    116, 104, 101, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32,
                                    85, 84, 70, 49, 54, 32, 117, 110, 105, 116, 32, 98, 117, 100,
                                    103, 101, 116,
                                ]),
                            )
                        } else {
                            {
                                let units_3 = Vec::<f64>::new();
                                {
                                    let index_2 = 0f64;
                                    {
                                        let units_4 = self::source_ml_temporary_finish_java_script_property_key_conversion_loop29(&(spelling_2), units_3, index_2);
                                        self::JavaScriptPropertyKeyConversionTransition::PropertyKeyCompleted(Box::new(self::JavaScriptSemanticValue::Text(units_4)))
                                    }
                                }
                            }
                        }
                    }
                    self::JavaScriptSemanticValue::Boolean(source_ml_source1_f923) => {
                        let spelling_3 =
                            self::read_java_script_primitive_property_key_spelling(value.clone());
                        if (spelling_3.length()) > 1_048_576f64 {
                            self::JavaScriptPropertyKeyConversionTransition::PropertyKeyInvalid(
                                self::ml_string::Text::from_units(vec![
                                    112, 114, 111, 112, 101, 114, 116, 121, 32, 107, 101, 121, 32,
                                    116, 101, 120, 116, 32, 101, 120, 99, 101, 101, 100, 115, 32,
                                    116, 104, 101, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32,
                                    85, 84, 70, 49, 54, 32, 117, 110, 105, 116, 32, 98, 117, 100,
                                    103, 101, 116,
                                ]),
                            )
                        } else {
                            {
                                let units_5 = Vec::<f64>::new();
                                {
                                    let index_3 = 0f64;
                                    {
                                        let units_6 = self::source_ml_temporary_finish_java_script_property_key_conversion_loop29(&(spelling_3), units_5, index_3);
                                        self::JavaScriptPropertyKeyConversionTransition::PropertyKeyCompleted(Box::new(self::JavaScriptSemanticValue::Text(units_6)))
                                    }
                                }
                            }
                        }
                    }
                    self::JavaScriptSemanticValue::Number(source_ml_source1_f924) => {
                        let spelling_4 =
                            self::read_java_script_primitive_property_key_spelling(value.clone());
                        if (spelling_4.length()) > 1_048_576f64 {
                            self::JavaScriptPropertyKeyConversionTransition::PropertyKeyInvalid(
                                self::ml_string::Text::from_units(vec![
                                    112, 114, 111, 112, 101, 114, 116, 121, 32, 107, 101, 121, 32,
                                    116, 101, 120, 116, 32, 101, 120, 99, 101, 101, 100, 115, 32,
                                    116, 104, 101, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32,
                                    85, 84, 70, 49, 54, 32, 117, 110, 105, 116, 32, 98, 117, 100,
                                    103, 101, 116,
                                ]),
                            )
                        } else {
                            {
                                let units_7 = Vec::<f64>::new();
                                {
                                    let index_4 = 0f64;
                                    {
                                        let units_8 = self::source_ml_temporary_finish_java_script_property_key_conversion_loop29(&(spelling_4), units_7, index_4);
                                        self::JavaScriptPropertyKeyConversionTransition::PropertyKeyCompleted(Box::new(self::JavaScriptSemanticValue::Text(units_8)))
                                    }
                                }
                            }
                        }
                    }
                    self::JavaScriptSemanticValue::Integer(source_ml_source1_f925) => {
                        let spelling_5 =
                            self::read_java_script_primitive_property_key_spelling(value.clone());
                        if (spelling_5.length()) > 1_048_576f64 {
                            self::JavaScriptPropertyKeyConversionTransition::PropertyKeyInvalid(
                                self::ml_string::Text::from_units(vec![
                                    112, 114, 111, 112, 101, 114, 116, 121, 32, 107, 101, 121, 32,
                                    116, 101, 120, 116, 32, 101, 120, 99, 101, 101, 100, 115, 32,
                                    116, 104, 101, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32,
                                    85, 84, 70, 49, 54, 32, 117, 110, 105, 116, 32, 98, 117, 100,
                                    103, 101, 116,
                                ]),
                            )
                        } else {
                            {
                                let units_9 = Vec::<f64>::new();
                                {
                                    let index_5 = 0f64;
                                    {
                                        let units_10 = self::source_ml_temporary_finish_java_script_property_key_conversion_loop29(&(spelling_5), units_9, index_5);
                                        self::JavaScriptPropertyKeyConversionTransition::PropertyKeyCompleted(Box::new(self::JavaScriptSemanticValue::Text(units_10)))
                                    }
                                }
                            }
                        }
                    }
                    self::JavaScriptSemanticValue::Text(source_ml_source1_f926) => {
                        let spelling_6 =
                            self::read_java_script_primitive_property_key_spelling(value.clone());
                        if (spelling_6.length()) > 1_048_576f64 {
                            self::JavaScriptPropertyKeyConversionTransition::PropertyKeyInvalid(
                                self::ml_string::Text::from_units(vec![
                                    112, 114, 111, 112, 101, 114, 116, 121, 32, 107, 101, 121, 32,
                                    116, 101, 120, 116, 32, 101, 120, 99, 101, 101, 100, 115, 32,
                                    116, 104, 101, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32,
                                    85, 84, 70, 49, 54, 32, 117, 110, 105, 116, 32, 98, 117, 100,
                                    103, 101, 116,
                                ]),
                            )
                        } else {
                            {
                                let units_11 = Vec::<f64>::new();
                                {
                                    let index_6 = 0f64;
                                    {
                                        let units_12 = self::source_ml_temporary_finish_java_script_property_key_conversion_loop29(&(spelling_6), units_11, index_6);
                                        self::JavaScriptPropertyKeyConversionTransition::PropertyKeyCompleted(Box::new(self::JavaScriptSemanticValue::Text(units_12)))
                                    }
                                }
                            }
                        }
                    }
                    self::JavaScriptSemanticValue::Symbol(source_ml_source1_f927) => {
                        let spelling_7 =
                            self::read_java_script_primitive_property_key_spelling(value.clone());
                        if (spelling_7.length()) > 1_048_576f64 {
                            self::JavaScriptPropertyKeyConversionTransition::PropertyKeyInvalid(
                                self::ml_string::Text::from_units(vec![
                                    112, 114, 111, 112, 101, 114, 116, 121, 32, 107, 101, 121, 32,
                                    116, 101, 120, 116, 32, 101, 120, 99, 101, 101, 100, 115, 32,
                                    116, 104, 101, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32,
                                    85, 84, 70, 49, 54, 32, 117, 110, 105, 116, 32, 98, 117, 100,
                                    103, 101, 116,
                                ]),
                            )
                        } else {
                            {
                                let units_13 = Vec::<f64>::new();
                                {
                                    let index_7 = 0f64;
                                    {
                                        let units_14 = self::source_ml_temporary_finish_java_script_property_key_conversion_loop29(&(spelling_7), units_13, index_7);
                                        self::JavaScriptPropertyKeyConversionTransition::PropertyKeyCompleted(Box::new(self::JavaScriptSemanticValue::Text(units_14)))
                                    }
                                }
                            }
                        }
                    }
                    self::JavaScriptSemanticValue::Object(
                        source_ml_source1_f928,
                        source_ml_source1_f929,
                    ) => self::JavaScriptPropertyKeyConversionTransition::PropertyKeyInvalid(
                        self::ml_string::Text::from_units(vec![
                            97, 32, 99, 111, 109, 112, 108, 101, 116, 101, 100, 32, 112, 114, 105,
                            109, 105, 116, 105, 118, 101, 32, 99, 111, 110, 118, 101, 114, 115,
                            105, 111, 110, 32, 99, 97, 110, 110, 111, 116, 32, 99, 111, 110, 116,
                            97, 105, 110, 32, 97, 110, 32, 79, 98, 106, 101, 99, 116,
                        ]),
                    ),
                }
            }
        }
        self::JavaScriptPrimitiveConversionTransition::PrimitiveThrow(value_2) => {
            let value_2 = *value_2;
            self::JavaScriptPropertyKeyConversionTransition::PropertyKeyThrow(Box::new(
                value_2.clone(),
            ))
        }
        self::JavaScriptPrimitiveConversionTransition::PrimitiveIntrinsicError(
            error_class,
            reason,
        ) => self::JavaScriptPropertyKeyConversionTransition::PropertyKeyIntrinsicError(
            error_class.clone(),
            reason.clone(),
        ),
        self::JavaScriptPrimitiveConversionTransition::PrimitiveInvalid(reason_2) => {
            self::JavaScriptPropertyKeyConversionTransition::PropertyKeyInvalid(reason_2.clone())
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn begin_java_script_property_key_conversion(
    heap: self::JavaScriptOrdinaryHeap,
    input: self::JavaScriptSemanticValue,
    exotic_key: self::JavaScriptSemanticValue,
) -> self::JavaScriptPropertyKeyConversionTransition {
    self::finish_java_script_property_key_conversion(self::begin_java_script_primitive_conversion(
        heap.clone(),
        input.clone(),
        exotic_key.clone(),
        &(self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103])),
    ))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn step_java_script_property_key_conversion(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptPrimitiveConversionState,
) -> self::JavaScriptPropertyKeyConversionTransition {
    if (match state.clone() {
        self::JavaScriptPrimitiveConversionState::PrimitiveConversion(
            _,
            _,
            source_ml_source1_data_property_value931,
            _,
        ) => source_ml_source1_data_property_value931.clone(),
    }) != (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103]))
    {
        self::JavaScriptPropertyKeyConversionTransition::PropertyKeyInvalid(
            self::ml_string::Text::from_units(vec![
                97, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 107, 101, 121, 32, 99, 111,
                110, 116, 105, 110, 117, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110,
                115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 116, 114, 105, 110,
                103, 32, 104, 105, 110, 116,
            ]),
        )
    } else {
        self::finish_java_script_property_key_conversion(
            self::step_java_script_primitive_conversion(heap.clone(), state.clone()),
        )
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn resume_java_script_property_key_conversion(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptPrimitiveConversionState,
    completion: self::JavaScriptPrimitiveConversionCompletion,
) -> self::JavaScriptPropertyKeyConversionTransition {
    if (match state.clone() {
        self::JavaScriptPrimitiveConversionState::PrimitiveConversion(
            _,
            _,
            source_ml_source1_data_property_value933,
            _,
        ) => source_ml_source1_data_property_value933.clone(),
    }) != (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103]))
    {
        self::JavaScriptPropertyKeyConversionTransition::PropertyKeyInvalid(
            self::ml_string::Text::from_units(vec![
                97, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 107, 101, 121, 32, 99, 111,
                110, 116, 105, 110, 117, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110,
                115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 116, 114, 105, 110,
                103, 32, 104, 105, 110, 116,
            ]),
        )
    } else {
        self::finish_java_script_property_key_conversion(
            self::resume_java_script_primitive_conversion(
                heap.clone(),
                state.clone(),
                completion.clone(),
            ),
        )
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptSemanticValueErrorLoop1Result {
    MlTemporaryReadJavaScriptSemanticValueErrorLoop1Done,
    MlTemporaryReadJavaScriptSemanticValueErrorLoop1Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_semantic_value_error_loop1(
    units: &[f64],
    mut at: f64,
) -> self::MlTemporaryReadJavaScriptSemanticValueErrorLoop1Result {
    loop {
        return if at < (units.len() as f64) {
            {
                let unit = (self::ml_array::optional_at(units, self::ml_array::number_index(at)))
                    .unwrap_or_else(|| -1f64);
                if (!(self::ml_math::is_integer(unit)) || (unit < 0f64)) || (unit > 65_535f64) {
                    self::MlTemporaryReadJavaScriptSemanticValueErrorLoop1Result::MlTemporaryReadJavaScriptSemanticValueErrorLoop1Return(self::ml_string::Text::from_units(vec![115, 101, 109, 97, 110, 116, 105, 99, 32, 116, 101, 120, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 111, 114, 100, 101, 114, 101, 100, 32, 85, 84, 70, 49, 54, 32, 99, 111, 100, 101, 32, 117, 110, 105, 116, 115]))
                } else {
                    {
                        let at_2 = at + 1f64;
                        {
                            at = at_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptSemanticValueErrorLoop1Result::MlTemporaryReadJavaScriptSemanticValueErrorLoop1Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryTestJavaScriptSemanticSameValueLoop2Result {
    MlTemporaryTestJavaScriptSemanticSameValueLoop2Done,
    MlTemporaryTestJavaScriptSemanticSameValueLoop2Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_test_java_script_semantic_same_value_loop2(
    left_units: &[f64],
    right_units: &[f64],
    mut at: f64,
) -> self::MlTemporaryTestJavaScriptSemanticSameValueLoop2Result {
    loop {
        return if at < (left_units.len() as f64) {
            {
                let ml_library_inequality_1 =
                    ((self::ml_array::optional_at(left_units, self::ml_array::number_index(at)))
                        .unwrap_or_else(|| -1f64))
                        != ((self::ml_array::optional_at(
                            right_units,
                            self::ml_array::number_index(at),
                        ))
                        .unwrap_or_else(|| -1f64));
                if ml_library_inequality_1 {
                    self::MlTemporaryTestJavaScriptSemanticSameValueLoop2Result::MlTemporaryTestJavaScriptSemanticSameValueLoop2Return(false)
                } else {
                    {
                        let at_2 = at + 1f64;
                        {
                            at = at_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryTestJavaScriptSemanticSameValueLoop2Result::MlTemporaryTestJavaScriptSemanticSameValueLoop2Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryPlanJavaScriptPropertyDefinitionJoin3 {
    MlTemporaryPlanJavaScriptPropertyDefinitionJoin3Next,
    MlTemporaryPlanJavaScriptPropertyDefinitionJoin3Return(
        (
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
            self::ml_string::Text,
            self::ml_string::Text,
            Vec<self::ml_string::Text>,
        ),
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadJavaScriptLiteralPropertyIndexLoop4Result {
    MlTemporaryReadJavaScriptLiteralPropertyIndexLoop4Done(f64),
    MlTemporaryReadJavaScriptLiteralPropertyIndexLoop4Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_literal_property_index_loop4(
    key: &[f64],
    mut value: f64,
    mut at: f64,
) -> self::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop4Result {
    loop {
        return if at < (key.len() as f64) {
            {
                let digit = (self::ml_array::optional_at(key, self::ml_array::number_index(at)))
                    .unwrap_or(f64::NAN)
                    - 48f64;
                if (!(self::ml_math::is_integer(digit)) || (digit < 0f64)) || (digit > 9f64) {
                    self::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop4Result::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop4Return(-1f64)
                } else {
                    {
                        let value_2 = (value * 10f64) + digit;
                        {
                            let at_2 = at + 1f64;
                            {
                                (value, at) = (value_2, at_2);
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop4Result::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop4Done(value)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryFindJavaScriptOrdinaryObjectLoop5Result {
    MlTemporaryFindJavaScriptOrdinaryObjectLoop5Done,
    MlTemporaryFindJavaScriptOrdinaryObjectLoop5Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_find_java_script_ordinary_object_loop5(
    mut objects: Vec<self::JavaScriptOrdinaryObject>,
    mut identity: self::ml::Big,
    mut low: f64,
    mut high: f64,
) -> self::MlTemporaryFindJavaScriptOrdinaryObjectLoop5Result {
    loop {
        return if low < high {
            {
                let middle = ((low + high) / 2f64).floor();
                {
                    let object =
                        self::ml_array::optional_at(&objects, self::ml_array::number_index(middle));
                    if match (object.clone(), None::<()>) {
                        (Some(_ml_left), Some(_ml_right)) => false,
                        (None, None) => true,
                        _ => false,
                    } {
                        self::MlTemporaryFindJavaScriptOrdinaryObjectLoop5Result::MlTemporaryFindJavaScriptOrdinaryObjectLoop5Return(-1f64)
                    } else {
                        let ml_product_condition_2 = {
                            let source_ml_source1_nullish_field_receiver937 = (object.clone(),);
                            if match (source_ml_source1_nullish_field_receiver937.clone()).0 {
                                Some(ml_present) => {
                                    let _ = ml_present.clone();
                                    true
                                }
                                None => false,
                            } {
                                {
                                    let source_ml_source1_nullish_field_present938 =
                                        ((source_ml_source1_nullish_field_receiver937.clone()).0)
                                            .clone()
                                            .expect("a proven present value");
                                    match source_ml_source1_nullish_field_present938 {
                                        self::JavaScriptOrdinaryObject::OrdinaryObject(
                                            source_ml_source1_data_property_value939,
                                            _,
                                            _,
                                            _,
                                            _,
                                        ) => source_ml_source1_data_property_value939.clone(),
                                    }
                                }
                            } else {
                                (self::ml_source_exception::SourceException {
                                    class_name: String::from("TypeError"),
                                    message_units: (self::ml_string::Text::from_units(vec![
                                        67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112,
                                        114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102,
                                        32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40,
                                        114, 101, 97, 100, 105, 110, 103, 32, 39, 105, 100, 101,
                                        110, 116, 105, 116, 121, 39, 41,
                                    ]))
                                    .0,
                                    message_present: true,
                                })
                                .raise()
                            }
                        };
                        if ml_product_condition_2 {
                            self::MlTemporaryFindJavaScriptOrdinaryObjectLoop5Result::MlTemporaryFindJavaScriptOrdinaryObjectLoop5Return(middle)
                        } else {
                            {
                                let ml_product_condition_1 = {
                                    let source_ml_source1_nullish_field_receiver943 =
                                        (object.clone(),);
                                    if match (source_ml_source1_nullish_field_receiver943.clone()).0
                                    {
                                        Some(ml_present) => {
                                            let _ = ml_present.clone();
                                            true
                                        }
                                        None => false,
                                    } {
                                        {
                                            let source_ml_source1_nullish_field_present944 =
                                                ((source_ml_source1_nullish_field_receiver943
                                                    .clone())
                                                .0)
                                                    .clone()
                                                    .expect("a proven present value");
                                            match source_ml_source1_nullish_field_present944 {
                                                self::JavaScriptOrdinaryObject::OrdinaryObject(
                                                    source_ml_source1_data_property_value945,
                                                    _,
                                                    _,
                                                    _,
                                                    _,
                                                ) => {
                                                    source_ml_source1_data_property_value945.clone()
                                                }
                                            }
                                        }
                                    } else {
                                        (self::ml_source_exception::SourceException {
                                            class_name: String::from("TypeError"),
                                            message_units: (self::ml_string::Text::from_units(
                                                vec![
                                                    67, 97, 110, 110, 111, 116, 32, 114, 101, 97,
                                                    100, 32, 112, 114, 111, 112, 101, 114, 116,
                                                    105, 101, 115, 32, 111, 102, 32, 117, 110, 100,
                                                    101, 102, 105, 110, 101, 100, 32, 40, 114, 101,
                                                    97, 100, 105, 110, 103, 32, 39, 105, 100, 101,
                                                    110, 116, 105, 116, 121, 39, 41,
                                                ],
                                            ))
                                            .0,
                                            message_present: true,
                                        })
                                        .raise()
                                    }
                                };
                                if ml_product_condition_1 {
                                    {
                                        let low_2 = middle + 1f64;
                                        {
                                            low = low_2;
                                            continue;
                                        }
                                    }
                                }
                            }
                            {
                                {
                                    let high_2 = middle;
                                    {
                                        high = high_2;
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryFindJavaScriptOrdinaryObjectLoop5Result::MlTemporaryFindJavaScriptOrdinaryObjectLoop5Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryTestJavaScriptOrdinarySymbolLoop6Result {
    MlTemporaryTestJavaScriptOrdinarySymbolLoop6Done,
    MlTemporaryTestJavaScriptOrdinarySymbolLoop6Return(bool),
}
