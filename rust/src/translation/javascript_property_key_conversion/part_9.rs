// Generated from complete js/src/translation/javascript-property-key-conversion.js. Do not edit by hand.
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptPrimitiveConversionState;
use super::JavaScriptPrimitiveConversionTransition;
use super::JavaScriptSemanticValue;
use super::continue_java_script_primitive_conversion;
use super::ml_string;
use super::plan_java_script_ordinary_get;
use super::read_java_script_ordinary_heap_error;
use super::read_java_script_ordinary_value_error;
use super::read_java_script_primitive_conversion_spelling;
use super::read_java_script_primitive_conversion_state_error;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn apply_java_script_primitive_conversion_method(
    state: self::JavaScriptPrimitiveConversionState,
    method: self::JavaScriptSemanticValue,
) -> self::JavaScriptPrimitiveConversionTransition {
    {
        let exotic = ((match state.clone() {
            self::JavaScriptPrimitiveConversionState::PrimitiveConversion(
                _,
                _,
                _,
                source_ml_source1_data_property_value815,
            ) => source_ml_source1_data_property_value815.clone(),
        }) == (self::ml_string::Text::from_units(vec![
            101, 120, 111, 116, 105, 99, 71, 101, 116,
        ]))) || ((match state.clone() {
            self::JavaScriptPrimitiveConversionState::PrimitiveConversion(
                _,
                _,
                _,
                source_ml_source1_data_property_value817,
            ) => source_ml_source1_data_property_value817.clone(),
        }) == (self::ml_string::Text::from_units(vec![
            101, 120, 111, 116, 105, 99, 71, 101, 116, 116, 101, 114,
        ])));
        {
            let first = ((match state.clone() {
                self::JavaScriptPrimitiveConversionState::PrimitiveConversion(
                    _,
                    _,
                    _,
                    source_ml_source1_data_property_value819,
                ) => source_ml_source1_data_property_value819.clone(),
            }) == (self::ml_string::Text::from_units(vec![
                102, 105, 114, 115, 116, 71, 101, 116,
            ]))) || ((match state.clone() {
                self::JavaScriptPrimitiveConversionState::PrimitiveConversion(
                    _,
                    _,
                    _,
                    source_ml_source1_data_property_value821,
                ) => source_ml_source1_data_property_value821.clone(),
            }) == (self::ml_string::Text::from_units(vec![
                102, 105, 114, 115, 116, 71, 101, 116, 116, 101, 114,
            ])));
            if exotic
                && (match method.clone() {
                    self::JavaScriptSemanticValue::Undefined => true,
                    self::JavaScriptSemanticValue::Null => false,
                    self::JavaScriptSemanticValue::Boolean(source_ml_source1_f822) => false,
                    self::JavaScriptSemanticValue::Number(source_ml_source1_f823) => false,
                    self::JavaScriptSemanticValue::Integer(source_ml_source1_f824) => false,
                    self::JavaScriptSemanticValue::Text(source_ml_source1_f825) => false,
                    self::JavaScriptSemanticValue::Symbol(source_ml_source1_f826) => false,
                    self::JavaScriptSemanticValue::Object(
                        source_ml_source1_f827,
                        source_ml_source1_f828,
                    ) => false,
                } || match method.clone() {
                    self::JavaScriptSemanticValue::Undefined => false,
                    self::JavaScriptSemanticValue::Null => true,
                    self::JavaScriptSemanticValue::Boolean(source_ml_source1_f829) => false,
                    self::JavaScriptSemanticValue::Number(source_ml_source1_f830) => false,
                    self::JavaScriptSemanticValue::Integer(source_ml_source1_f831) => false,
                    self::JavaScriptSemanticValue::Text(source_ml_source1_f832) => false,
                    self::JavaScriptSemanticValue::Symbol(source_ml_source1_f833) => false,
                    self::JavaScriptSemanticValue::Object(
                        source_ml_source1_f834,
                        source_ml_source1_f835,
                    ) => false,
                })
            {
                self::continue_java_script_primitive_conversion(
                    state.clone(),
                    &(self::ml_string::Text::from_units(vec![
                        102, 105, 114, 115, 116, 71, 101, 116,
                    ])),
                )
            } else {
                match method.clone() {
                    self::JavaScriptSemanticValue::Undefined => {
                        if exotic {
                            self::JavaScriptPrimitiveConversionTransition::PrimitiveIntrinsicError(
                                self::ml_string::Text::from_units(vec![
                                    84, 121, 112, 101, 69, 114, 114, 111, 114,
                                ]),
                                self::ml_string::Text::from_units(vec![
                                    97, 110, 32, 101, 120, 111, 116, 105, 99, 32, 112, 114, 105,
                                    109, 105, 116, 105, 118, 101, 32, 99, 111, 110, 118, 101, 114,
                                    115, 105, 111, 110, 32, 109, 101, 116, 104, 111, 100, 32, 109,
                                    117, 115, 116, 32, 98, 101, 32, 99, 97, 108, 108, 97, 98, 108,
                                    101, 32, 111, 114, 32, 97, 98, 115, 101, 110, 116,
                                ]),
                            )
                        } else if first {
                            self::continue_java_script_primitive_conversion(
                                state.clone(),
                                &(self::ml_string::Text::from_units(vec![
                                    115, 101, 99, 111, 110, 100, 71, 101, 116,
                                ])),
                            )
                        } else {
                            self::JavaScriptPrimitiveConversionTransition::PrimitiveIntrinsicError(
                                self::ml_string::Text::from_units(vec![
                                    84, 121, 112, 101, 69, 114, 114, 111, 114,
                                ]),
                                self::ml_string::Text::from_units(vec![
                                    111, 114, 100, 105, 110, 97, 114, 121, 32, 99, 111, 110, 118,
                                    101, 114, 115, 105, 111, 110, 32, 101, 120, 104, 97, 117, 115,
                                    116, 101, 100, 32, 105, 116, 115, 32, 99, 97, 108, 108, 97, 98,
                                    108, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 45,
                                    112, 114, 111, 100, 117, 99, 105, 110, 103, 32, 109, 101, 116,
                                    104, 111, 100, 115,
                                ]),
                            )
                        }
                    }
                    self::JavaScriptSemanticValue::Null => {
                        if exotic {
                            self::JavaScriptPrimitiveConversionTransition::PrimitiveIntrinsicError(
                                self::ml_string::Text::from_units(vec![
                                    84, 121, 112, 101, 69, 114, 114, 111, 114,
                                ]),
                                self::ml_string::Text::from_units(vec![
                                    97, 110, 32, 101, 120, 111, 116, 105, 99, 32, 112, 114, 105,
                                    109, 105, 116, 105, 118, 101, 32, 99, 111, 110, 118, 101, 114,
                                    115, 105, 111, 110, 32, 109, 101, 116, 104, 111, 100, 32, 109,
                                    117, 115, 116, 32, 98, 101, 32, 99, 97, 108, 108, 97, 98, 108,
                                    101, 32, 111, 114, 32, 97, 98, 115, 101, 110, 116,
                                ]),
                            )
                        } else if first {
                            self::continue_java_script_primitive_conversion(
                                state.clone(),
                                &(self::ml_string::Text::from_units(vec![
                                    115, 101, 99, 111, 110, 100, 71, 101, 116,
                                ])),
                            )
                        } else {
                            self::JavaScriptPrimitiveConversionTransition::PrimitiveIntrinsicError(
                                self::ml_string::Text::from_units(vec![
                                    84, 121, 112, 101, 69, 114, 114, 111, 114,
                                ]),
                                self::ml_string::Text::from_units(vec![
                                    111, 114, 100, 105, 110, 97, 114, 121, 32, 99, 111, 110, 118,
                                    101, 114, 115, 105, 111, 110, 32, 101, 120, 104, 97, 117, 115,
                                    116, 101, 100, 32, 105, 116, 115, 32, 99, 97, 108, 108, 97, 98,
                                    108, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 45,
                                    112, 114, 111, 100, 117, 99, 105, 110, 103, 32, 109, 101, 116,
                                    104, 111, 100, 115,
                                ]),
                            )
                        }
                    }
                    self::JavaScriptSemanticValue::Boolean(source_ml_source1_f836) => {
                        if exotic {
                            self::JavaScriptPrimitiveConversionTransition::PrimitiveIntrinsicError(
                                self::ml_string::Text::from_units(vec![
                                    84, 121, 112, 101, 69, 114, 114, 111, 114,
                                ]),
                                self::ml_string::Text::from_units(vec![
                                    97, 110, 32, 101, 120, 111, 116, 105, 99, 32, 112, 114, 105,
                                    109, 105, 116, 105, 118, 101, 32, 99, 111, 110, 118, 101, 114,
                                    115, 105, 111, 110, 32, 109, 101, 116, 104, 111, 100, 32, 109,
                                    117, 115, 116, 32, 98, 101, 32, 99, 97, 108, 108, 97, 98, 108,
                                    101, 32, 111, 114, 32, 97, 98, 115, 101, 110, 116,
                                ]),
                            )
                        } else if first {
                            self::continue_java_script_primitive_conversion(
                                state.clone(),
                                &(self::ml_string::Text::from_units(vec![
                                    115, 101, 99, 111, 110, 100, 71, 101, 116,
                                ])),
                            )
                        } else {
                            self::JavaScriptPrimitiveConversionTransition::PrimitiveIntrinsicError(
                                self::ml_string::Text::from_units(vec![
                                    84, 121, 112, 101, 69, 114, 114, 111, 114,
                                ]),
                                self::ml_string::Text::from_units(vec![
                                    111, 114, 100, 105, 110, 97, 114, 121, 32, 99, 111, 110, 118,
                                    101, 114, 115, 105, 111, 110, 32, 101, 120, 104, 97, 117, 115,
                                    116, 101, 100, 32, 105, 116, 115, 32, 99, 97, 108, 108, 97, 98,
                                    108, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 45,
                                    112, 114, 111, 100, 117, 99, 105, 110, 103, 32, 109, 101, 116,
                                    104, 111, 100, 115,
                                ]),
                            )
                        }
                    }
                    self::JavaScriptSemanticValue::Number(source_ml_source1_f837) => {
                        if exotic {
                            self::JavaScriptPrimitiveConversionTransition::PrimitiveIntrinsicError(
                                self::ml_string::Text::from_units(vec![
                                    84, 121, 112, 101, 69, 114, 114, 111, 114,
                                ]),
                                self::ml_string::Text::from_units(vec![
                                    97, 110, 32, 101, 120, 111, 116, 105, 99, 32, 112, 114, 105,
                                    109, 105, 116, 105, 118, 101, 32, 99, 111, 110, 118, 101, 114,
                                    115, 105, 111, 110, 32, 109, 101, 116, 104, 111, 100, 32, 109,
                                    117, 115, 116, 32, 98, 101, 32, 99, 97, 108, 108, 97, 98, 108,
                                    101, 32, 111, 114, 32, 97, 98, 115, 101, 110, 116,
                                ]),
                            )
                        } else if first {
                            self::continue_java_script_primitive_conversion(
                                state.clone(),
                                &(self::ml_string::Text::from_units(vec![
                                    115, 101, 99, 111, 110, 100, 71, 101, 116,
                                ])),
                            )
                        } else {
                            self::JavaScriptPrimitiveConversionTransition::PrimitiveIntrinsicError(
                                self::ml_string::Text::from_units(vec![
                                    84, 121, 112, 101, 69, 114, 114, 111, 114,
                                ]),
                                self::ml_string::Text::from_units(vec![
                                    111, 114, 100, 105, 110, 97, 114, 121, 32, 99, 111, 110, 118,
                                    101, 114, 115, 105, 111, 110, 32, 101, 120, 104, 97, 117, 115,
                                    116, 101, 100, 32, 105, 116, 115, 32, 99, 97, 108, 108, 97, 98,
                                    108, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 45,
                                    112, 114, 111, 100, 117, 99, 105, 110, 103, 32, 109, 101, 116,
                                    104, 111, 100, 115,
                                ]),
                            )
                        }
                    }
                    self::JavaScriptSemanticValue::Integer(source_ml_source1_f838) => {
                        if exotic {
                            self::JavaScriptPrimitiveConversionTransition::PrimitiveIntrinsicError(
                                self::ml_string::Text::from_units(vec![
                                    84, 121, 112, 101, 69, 114, 114, 111, 114,
                                ]),
                                self::ml_string::Text::from_units(vec![
                                    97, 110, 32, 101, 120, 111, 116, 105, 99, 32, 112, 114, 105,
                                    109, 105, 116, 105, 118, 101, 32, 99, 111, 110, 118, 101, 114,
                                    115, 105, 111, 110, 32, 109, 101, 116, 104, 111, 100, 32, 109,
                                    117, 115, 116, 32, 98, 101, 32, 99, 97, 108, 108, 97, 98, 108,
                                    101, 32, 111, 114, 32, 97, 98, 115, 101, 110, 116,
                                ]),
                            )
                        } else if first {
                            self::continue_java_script_primitive_conversion(
                                state.clone(),
                                &(self::ml_string::Text::from_units(vec![
                                    115, 101, 99, 111, 110, 100, 71, 101, 116,
                                ])),
                            )
                        } else {
                            self::JavaScriptPrimitiveConversionTransition::PrimitiveIntrinsicError(
                                self::ml_string::Text::from_units(vec![
                                    84, 121, 112, 101, 69, 114, 114, 111, 114,
                                ]),
                                self::ml_string::Text::from_units(vec![
                                    111, 114, 100, 105, 110, 97, 114, 121, 32, 99, 111, 110, 118,
                                    101, 114, 115, 105, 111, 110, 32, 101, 120, 104, 97, 117, 115,
                                    116, 101, 100, 32, 105, 116, 115, 32, 99, 97, 108, 108, 97, 98,
                                    108, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 45,
                                    112, 114, 111, 100, 117, 99, 105, 110, 103, 32, 109, 101, 116,
                                    104, 111, 100, 115,
                                ]),
                            )
                        }
                    }
                    self::JavaScriptSemanticValue::Text(source_ml_source1_f839) => {
                        if exotic {
                            self::JavaScriptPrimitiveConversionTransition::PrimitiveIntrinsicError(
                                self::ml_string::Text::from_units(vec![
                                    84, 121, 112, 101, 69, 114, 114, 111, 114,
                                ]),
                                self::ml_string::Text::from_units(vec![
                                    97, 110, 32, 101, 120, 111, 116, 105, 99, 32, 112, 114, 105,
                                    109, 105, 116, 105, 118, 101, 32, 99, 111, 110, 118, 101, 114,
                                    115, 105, 111, 110, 32, 109, 101, 116, 104, 111, 100, 32, 109,
                                    117, 115, 116, 32, 98, 101, 32, 99, 97, 108, 108, 97, 98, 108,
                                    101, 32, 111, 114, 32, 97, 98, 115, 101, 110, 116,
                                ]),
                            )
                        } else if first {
                            self::continue_java_script_primitive_conversion(
                                state.clone(),
                                &(self::ml_string::Text::from_units(vec![
                                    115, 101, 99, 111, 110, 100, 71, 101, 116,
                                ])),
                            )
                        } else {
                            self::JavaScriptPrimitiveConversionTransition::PrimitiveIntrinsicError(
                                self::ml_string::Text::from_units(vec![
                                    84, 121, 112, 101, 69, 114, 114, 111, 114,
                                ]),
                                self::ml_string::Text::from_units(vec![
                                    111, 114, 100, 105, 110, 97, 114, 121, 32, 99, 111, 110, 118,
                                    101, 114, 115, 105, 111, 110, 32, 101, 120, 104, 97, 117, 115,
                                    116, 101, 100, 32, 105, 116, 115, 32, 99, 97, 108, 108, 97, 98,
                                    108, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 45,
                                    112, 114, 111, 100, 117, 99, 105, 110, 103, 32, 109, 101, 116,
                                    104, 111, 100, 115,
                                ]),
                            )
                        }
                    }
                    self::JavaScriptSemanticValue::Symbol(source_ml_source1_f840) => {
                        if exotic {
                            self::JavaScriptPrimitiveConversionTransition::PrimitiveIntrinsicError(
                                self::ml_string::Text::from_units(vec![
                                    84, 121, 112, 101, 69, 114, 114, 111, 114,
                                ]),
                                self::ml_string::Text::from_units(vec![
                                    97, 110, 32, 101, 120, 111, 116, 105, 99, 32, 112, 114, 105,
                                    109, 105, 116, 105, 118, 101, 32, 99, 111, 110, 118, 101, 114,
                                    115, 105, 111, 110, 32, 109, 101, 116, 104, 111, 100, 32, 109,
                                    117, 115, 116, 32, 98, 101, 32, 99, 97, 108, 108, 97, 98, 108,
                                    101, 32, 111, 114, 32, 97, 98, 115, 101, 110, 116,
                                ]),
                            )
                        } else if first {
                            self::continue_java_script_primitive_conversion(
                                state.clone(),
                                &(self::ml_string::Text::from_units(vec![
                                    115, 101, 99, 111, 110, 100, 71, 101, 116,
                                ])),
                            )
                        } else {
                            self::JavaScriptPrimitiveConversionTransition::PrimitiveIntrinsicError(
                                self::ml_string::Text::from_units(vec![
                                    84, 121, 112, 101, 69, 114, 114, 111, 114,
                                ]),
                                self::ml_string::Text::from_units(vec![
                                    111, 114, 100, 105, 110, 97, 114, 121, 32, 99, 111, 110, 118,
                                    101, 114, 115, 105, 111, 110, 32, 101, 120, 104, 97, 117, 115,
                                    116, 101, 100, 32, 105, 116, 115, 32, 99, 97, 108, 108, 97, 98,
                                    108, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 45,
                                    112, 114, 111, 100, 117, 99, 105, 110, 103, 32, 109, 101, 116,
                                    104, 111, 100, 115,
                                ]),
                            )
                        }
                    }
                    self::JavaScriptSemanticValue::Object(source_ml_source1_f841, callable) => {
                        if callable {
                            {
                                let phase = if exotic {
                                    self::ml_string::Text::from_units(vec![
                                        101, 120, 111, 116, 105, 99, 67, 97, 108, 108,
                                    ])
                                } else if first {
                                    self::ml_string::Text::from_units(vec![
                                        102, 105, 114, 115, 116, 67, 97, 108, 108,
                                    ])
                                } else {
                                    self::ml_string::Text::from_units(vec![
                                        115, 101, 99, 111, 110, 100, 67, 97, 108, 108,
                                    ])
                                };
                                {
                                    let arguments_ = if exotic {
                                        vec![self::read_java_script_primitive_conversion_spelling(&(match state.clone() {
                                            self::JavaScriptPrimitiveConversionState::PrimitiveConversion(_, _, source_ml_source1_data_property_value842, _) => {
                                                source_ml_source1_data_property_value842.clone()
                                            }
                                        }))]
                                    } else {
                                        Vec::<self::JavaScriptSemanticValue>::new()
                                    };
                                    self::JavaScriptPrimitiveConversionTransition::PrimitiveCall(Box::new(self::JavaScriptPrimitiveConversionState::PrimitiveConversion(Box::new(match state.clone() {
                                        self::JavaScriptPrimitiveConversionState::PrimitiveConversion(source_ml_source1_data_property_value843, _, _, _) => {
                                            let source_ml_source1_data_property_value843 = *source_ml_source1_data_property_value843;
                                            source_ml_source1_data_property_value843.clone()
                                        }
                                    }), Box::new(match state.clone() {
                                        self::JavaScriptPrimitiveConversionState::PrimitiveConversion(_, source_ml_source1_data_property_value844, _, _) => {
                                            let source_ml_source1_data_property_value844 = *source_ml_source1_data_property_value844;
                                            source_ml_source1_data_property_value844.clone()
                                        }
                                    }), match state.clone() {
                                        self::JavaScriptPrimitiveConversionState::PrimitiveConversion(_, _, source_ml_source1_data_property_value845, _) => {
                                            source_ml_source1_data_property_value845.clone()
                                        }
                                    }, phase)), Box::new(method.clone()), Box::new(match state.clone() {
                                        self::JavaScriptPrimitiveConversionState::PrimitiveConversion(source_ml_source1_data_property_value846, _, _, _) => {
                                            let source_ml_source1_data_property_value846 = *source_ml_source1_data_property_value846;
                                            source_ml_source1_data_property_value846.clone()
                                        }
                                    }), arguments_)
                                }
                            }
                        } else if exotic {
                            self::JavaScriptPrimitiveConversionTransition::PrimitiveIntrinsicError(
                                self::ml_string::Text::from_units(vec![
                                    84, 121, 112, 101, 69, 114, 114, 111, 114,
                                ]),
                                self::ml_string::Text::from_units(vec![
                                    97, 110, 32, 101, 120, 111, 116, 105, 99, 32, 112, 114, 105,
                                    109, 105, 116, 105, 118, 101, 32, 99, 111, 110, 118, 101, 114,
                                    115, 105, 111, 110, 32, 109, 101, 116, 104, 111, 100, 32, 109,
                                    117, 115, 116, 32, 98, 101, 32, 99, 97, 108, 108, 97, 98, 108,
                                    101, 32, 111, 114, 32, 97, 98, 115, 101, 110, 116,
                                ]),
                            )
                        } else if first {
                            self::continue_java_script_primitive_conversion(
                                state.clone(),
                                &(self::ml_string::Text::from_units(vec![
                                    115, 101, 99, 111, 110, 100, 71, 101, 116,
                                ])),
                            )
                        } else {
                            self::JavaScriptPrimitiveConversionTransition::PrimitiveIntrinsicError(
                                self::ml_string::Text::from_units(vec![
                                    84, 121, 112, 101, 69, 114, 114, 111, 114,
                                ]),
                                self::ml_string::Text::from_units(vec![
                                    111, 114, 100, 105, 110, 97, 114, 121, 32, 99, 111, 110, 118,
                                    101, 114, 115, 105, 111, 110, 32, 101, 120, 104, 97, 117, 115,
                                    116, 101, 100, 32, 105, 116, 115, 32, 99, 97, 108, 108, 97, 98,
                                    108, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 45,
                                    112, 114, 111, 100, 117, 99, 105, 110, 103, 32, 109, 101, 116,
                                    104, 111, 100, 115,
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
pub fn step_java_script_primitive_conversion(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptPrimitiveConversionState,
) -> self::JavaScriptPrimitiveConversionTransition {
    {
        let error =
            self::read_java_script_primitive_conversion_state_error(heap.clone(), state.clone());
        {
            let ml_library_inequality_3 = error != (self::ml_string::Text::from_units(vec![]));
            if ml_library_inequality_3 {
                self::JavaScriptPrimitiveConversionTransition::PrimitiveInvalid(error.clone())
            } else if (((match state.clone() {
                self::JavaScriptPrimitiveConversionState::PrimitiveConversion(
                    _,
                    _,
                    _,
                    source_ml_source1_data_property_value848,
                ) => source_ml_source1_data_property_value848.clone(),
            }) != (self::ml_string::Text::from_units(vec![
                101, 120, 111, 116, 105, 99, 71, 101, 116,
            ]))) && ((match state.clone() {
                self::JavaScriptPrimitiveConversionState::PrimitiveConversion(
                    _,
                    _,
                    _,
                    source_ml_source1_data_property_value850,
                ) => source_ml_source1_data_property_value850.clone(),
            }) != (self::ml_string::Text::from_units(vec![
                102, 105, 114, 115, 116, 71, 101, 116,
            ]))))
                && ((match state.clone() {
                    self::JavaScriptPrimitiveConversionState::PrimitiveConversion(
                        _,
                        _,
                        _,
                        source_ml_source1_data_property_value852,
                    ) => source_ml_source1_data_property_value852.clone(),
                }) != (self::ml_string::Text::from_units(vec![
                    115, 101, 99, 111, 110, 100, 71, 101, 116,
                ])))
            {
                self::JavaScriptPrimitiveConversionTransition::PrimitiveInvalid(
                    self::ml_string::Text::from_units(vec![
                        97, 32, 112, 101, 110, 100, 105, 110, 103, 32, 112, 114, 105, 109, 105,
                        116, 105, 118, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110,
                        32, 99, 97, 108, 108, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32,
                        97, 99, 116, 117, 97, 108, 32, 99, 111, 109, 112, 108, 101, 116, 105, 111,
                        110,
                    ]),
                )
            } else {
                {
                    let receiver = match state.clone() {
                        self::JavaScriptPrimitiveConversionState::PrimitiveConversion(
                            source_ml_source1_data_property_value853,
                            _,
                            _,
                            _,
                        ) => {
                            let source_ml_source1_data_property_value853 =
                                *source_ml_source1_data_property_value853;
                            source_ml_source1_data_property_value853.clone()
                        }
                    };
                    match receiver.clone() {
                        self::JavaScriptSemanticValue::Undefined => {
                            self::JavaScriptPrimitiveConversionTransition::PrimitiveInvalid(
                                self::ml_string::Text::from_units(vec![
                                    97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 99,
                                    111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 99, 111, 110,
                                    116, 105, 110, 117, 97, 116, 105, 111, 110, 32, 114, 101, 116,
                                    97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97,
                                    108, 32, 79, 98, 106, 101, 99, 116, 32, 114, 101, 99, 101, 105,
                                    118, 101, 114,
                                ]),
                            )
                        }
                        self::JavaScriptSemanticValue::Null => {
                            self::JavaScriptPrimitiveConversionTransition::PrimitiveInvalid(
                                self::ml_string::Text::from_units(vec![
                                    97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 99,
                                    111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 99, 111, 110,
                                    116, 105, 110, 117, 97, 116, 105, 111, 110, 32, 114, 101, 116,
                                    97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97,
                                    108, 32, 79, 98, 106, 101, 99, 116, 32, 114, 101, 99, 101, 105,
                                    118, 101, 114,
                                ]),
                            )
                        }
                        self::JavaScriptSemanticValue::Boolean(source_ml_source1_f854) => {
                            self::JavaScriptPrimitiveConversionTransition::PrimitiveInvalid(
                                self::ml_string::Text::from_units(vec![
                                    97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 99,
                                    111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 99, 111, 110,
                                    116, 105, 110, 117, 97, 116, 105, 111, 110, 32, 114, 101, 116,
                                    97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97,
                                    108, 32, 79, 98, 106, 101, 99, 116, 32, 114, 101, 99, 101, 105,
                                    118, 101, 114,
                                ]),
                            )
                        }
                        self::JavaScriptSemanticValue::Number(source_ml_source1_f855) => {
                            self::JavaScriptPrimitiveConversionTransition::PrimitiveInvalid(
                                self::ml_string::Text::from_units(vec![
                                    97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 99,
                                    111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 99, 111, 110,
                                    116, 105, 110, 117, 97, 116, 105, 111, 110, 32, 114, 101, 116,
                                    97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97,
                                    108, 32, 79, 98, 106, 101, 99, 116, 32, 114, 101, 99, 101, 105,
                                    118, 101, 114,
                                ]),
                            )
                        }
                        self::JavaScriptSemanticValue::Integer(source_ml_source1_f856) => {
                            self::JavaScriptPrimitiveConversionTransition::PrimitiveInvalid(
                                self::ml_string::Text::from_units(vec![
                                    97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 99,
                                    111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 99, 111, 110,
                                    116, 105, 110, 117, 97, 116, 105, 111, 110, 32, 114, 101, 116,
                                    97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97,
                                    108, 32, 79, 98, 106, 101, 99, 116, 32, 114, 101, 99, 101, 105,
                                    118, 101, 114,
                                ]),
                            )
                        }
                        self::JavaScriptSemanticValue::Text(source_ml_source1_f857) => {
                            self::JavaScriptPrimitiveConversionTransition::PrimitiveInvalid(
                                self::ml_string::Text::from_units(vec![
                                    97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 99,
                                    111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 99, 111, 110,
                                    116, 105, 110, 117, 97, 116, 105, 111, 110, 32, 114, 101, 116,
                                    97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97,
                                    108, 32, 79, 98, 106, 101, 99, 116, 32, 114, 101, 99, 101, 105,
                                    118, 101, 114,
                                ]),
                            )
                        }
                        self::JavaScriptSemanticValue::Symbol(source_ml_source1_f858) => {
                            self::JavaScriptPrimitiveConversionTransition::PrimitiveInvalid(
                                self::ml_string::Text::from_units(vec![
                                    97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 99,
                                    111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 99, 111, 110,
                                    116, 105, 110, 117, 97, 116, 105, 111, 110, 32, 114, 101, 116,
                                    97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97,
                                    108, 32, 79, 98, 106, 101, 99, 116, 32, 114, 101, 99, 101, 105,
                                    118, 101, 114,
                                ]),
                            )
                        }
                        self::JavaScriptSemanticValue::Object(identity, source_ml_source1_f859) => {
                            let first = (match state.clone() {
                                self::JavaScriptPrimitiveConversionState::PrimitiveConversion(
                                    _,
                                    _,
                                    _,
                                    source_ml_source1_data_property_value861,
                                ) => source_ml_source1_data_property_value861.clone(),
                            }) == (self::ml_string::Text::from_units(vec![
                                102, 105, 114, 115, 116, 71, 101, 116,
                            ]));
                            {
                                let method = if (match state.clone() {
                                        self::JavaScriptPrimitiveConversionState::PrimitiveConversion(_, _, source_ml_source1_data_property_value863, _) => {
                                            source_ml_source1_data_property_value863.clone()
                                        }
                                    }) == (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103])) {
                                        if first {
                                            self::ml_string::Text::from_units(vec![116, 111, 83, 116, 114, 105, 110, 103])
                                        } else {
                                            self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101, 79, 102])
                                        }
                                    } else if first {
                                            self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101, 79, 102])
                                        } else {
                                            self::ml_string::Text::from_units(vec![116, 111, 83, 116, 114, 105, 110, 103])
                                        };
                                {
                                    let key = if (match state.clone() {
                                            self::JavaScriptPrimitiveConversionState::PrimitiveConversion(_, _, _, source_ml_source1_data_property_value865) => {
                                                source_ml_source1_data_property_value865.clone()
                                            }
                                        }) == (self::ml_string::Text::from_units(vec![101, 120, 111, 116, 105, 99, 71, 101, 116])) {
                                            match state.clone() {
                                                self::JavaScriptPrimitiveConversionState::PrimitiveConversion(_, source_ml_source1_data_property_value866, _, _) => {
                                                    let source_ml_source1_data_property_value866 = *source_ml_source1_data_property_value866;
                                                    source_ml_source1_data_property_value866.clone()
                                                }
                                            }
                                        } else {
                                            self::read_java_script_primitive_conversion_spelling(&(method))
                                        };
                                    {
                                        let plan = self::plan_java_script_ordinary_get(
                                            heap.clone(),
                                            identity.clone(),
                                            key,
                                            receiver.clone(),
                                        );
                                        {
                                            let ml_library_inequality_2 = ((plan.clone()).3)
                                                != (self::ml_string::Text::from_units(vec![]));
                                            if ml_library_inequality_2 {
                                                self::JavaScriptPrimitiveConversionTransition::PrimitiveInvalid((plan.clone()).3)
                                            } else if ((plan.clone()).0)
                                                == (self::ml_string::Text::from_units(vec![
                                                    99, 97, 108, 108,
                                                ]))
                                            {
                                                {
                                                    let phase = if (match state.clone() {
                                                            self::JavaScriptPrimitiveConversionState::PrimitiveConversion(_, _, _, source_ml_source1_data_property_value868) => {
                                                                source_ml_source1_data_property_value868.clone()
                                                            }
                                                        }) == (self::ml_string::Text::from_units(vec![101, 120, 111, 116, 105, 99, 71, 101, 116])) {
                                                            self::ml_string::Text::from_units(vec![101, 120, 111, 116, 105, 99, 71, 101, 116, 116, 101, 114])
                                                        } else if first {
                                                                self::ml_string::Text::from_units(vec![102, 105, 114, 115, 116, 71, 101, 116, 116, 101, 114])
                                                            } else {
                                                                self::ml_string::Text::from_units(vec![115, 101, 99, 111, 110, 100, 71, 101, 116, 116, 101, 114])
                                                            };
                                                    self::JavaScriptPrimitiveConversionTransition::PrimitiveCall(Box::new(self::JavaScriptPrimitiveConversionState::PrimitiveConversion(Box::new(receiver.clone()), Box::new(match state.clone() {
                                                            self::JavaScriptPrimitiveConversionState::PrimitiveConversion(_, source_ml_source1_data_property_value869, _, _) => {
                                                                let source_ml_source1_data_property_value869 = *source_ml_source1_data_property_value869;
                                                                source_ml_source1_data_property_value869.clone()
                                                            }
                                                        }), match state.clone() {
                                                            self::JavaScriptPrimitiveConversionState::PrimitiveConversion(_, _, source_ml_source1_data_property_value870, _) => {
                                                                source_ml_source1_data_property_value870.clone()
                                                            }
                                                        }, phase)), Box::new((plan.clone()).2), Box::new((plan.clone()).4), (plan.clone()).1)
                                                }
                                            } else {
                                                let ml_library_inequality_1 = ((plan.clone()).0)
                                                    != (self::ml_string::Text::from_units(vec![
                                                        118, 97, 108, 117, 101,
                                                    ]));
                                                if ml_library_inequality_1 {
                                                    self::JavaScriptPrimitiveConversionTransition::PrimitiveInvalid(self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 71, 101, 116, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 118, 97, 108, 117, 101, 32, 111, 114, 32, 112, 101, 110, 100, 105, 110, 103, 32, 103, 101, 116, 116, 101, 114]))
                                                } else {
                                                    self::apply_java_script_primitive_conversion_method(state.clone(), (plan.clone()).5)
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
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn begin_java_script_primitive_conversion(
    heap: self::JavaScriptOrdinaryHeap,
    input: self::JavaScriptSemanticValue,
    exotic_key: self::JavaScriptSemanticValue,
    hint: &self::ml_string::Text,
) -> self::JavaScriptPrimitiveConversionTransition {
    {
        let heap_error = self::read_java_script_ordinary_heap_error(heap.clone());
        {
            let ml_library_inequality_2 = heap_error != (self::ml_string::Text::from_units(vec![]));
            if ml_library_inequality_2 {
                self::JavaScriptPrimitiveConversionTransition::PrimitiveInvalid(heap_error.clone())
            } else {
                {
                    let error =
                        self::read_java_script_ordinary_value_error(heap.clone(), input.clone());
                    {
                        let ml_library_inequality_1 =
                            error != (self::ml_string::Text::from_units(vec![]));
                        if ml_library_inequality_1 {
                            self::JavaScriptPrimitiveConversionTransition::PrimitiveInvalid(
                                error.clone(),
                            )
                        } else if (((*hint)
                            != (self::ml_string::Text::from_units(vec![
                                100, 101, 102, 97, 117, 108, 116,
                            ])))
                            && ((*hint)
                                != (self::ml_string::Text::from_units(vec![
                                    110, 117, 109, 98, 101, 114,
                                ]))))
                            && ((*hint)
                                != (self::ml_string::Text::from_units(vec![
                                    115, 116, 114, 105, 110, 103,
                                ])))
                        {
                            self::JavaScriptPrimitiveConversionTransition::PrimitiveInvalid(
                                self::ml_string::Text::from_units(vec![
                                    97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 99,
                                    111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 113,
                                    117, 105, 114, 101, 115, 32, 97, 110, 32, 101, 120, 97, 99,
                                    116, 32, 112, 114, 101, 102, 101, 114, 114, 101, 100, 45, 116,
                                    121, 112, 101, 32, 104, 105, 110, 116,
                                ]),
                            )
                        } else {
                            match input.clone() {
                            self::JavaScriptSemanticValue::Undefined => {
                                self::JavaScriptPrimitiveConversionTransition::PrimitiveCompleted(Box::new(input.clone()))
                            }
                            self::JavaScriptSemanticValue::Null => {
                                self::JavaScriptPrimitiveConversionTransition::PrimitiveCompleted(Box::new(input.clone()))
                            }
                            self::JavaScriptSemanticValue::Boolean(source_ml_source1_f871) => {
                                self::JavaScriptPrimitiveConversionTransition::PrimitiveCompleted(Box::new(input.clone()))
                            }
                            self::JavaScriptSemanticValue::Number(source_ml_source1_f872) => {
                                self::JavaScriptPrimitiveConversionTransition::PrimitiveCompleted(Box::new(input.clone()))
                            }
                            self::JavaScriptSemanticValue::Integer(source_ml_source1_f873) => {
                                self::JavaScriptPrimitiveConversionTransition::PrimitiveCompleted(Box::new(input.clone()))
                            }
                            self::JavaScriptSemanticValue::Text(source_ml_source1_f874) => {
                                self::JavaScriptPrimitiveConversionTransition::PrimitiveCompleted(Box::new(input.clone()))
                            }
                            self::JavaScriptSemanticValue::Symbol(source_ml_source1_f875) => {
                                self::JavaScriptPrimitiveConversionTransition::PrimitiveCompleted(Box::new(input.clone()))
                            }
                            self::JavaScriptSemanticValue::Object(source_ml_source1_f876, source_ml_source1_f877) => {
                                self::step_java_script_primitive_conversion(heap.clone(), self::JavaScriptPrimitiveConversionState::PrimitiveConversion(Box::new(input.clone()), Box::new(exotic_key.clone()), hint.clone(), self::ml_string::Text::from_units(vec![101, 120, 111, 116, 105, 99, 71, 101, 116])))
                            }
                        }
                        }
                    }
                }
            }
        }
    }
}
