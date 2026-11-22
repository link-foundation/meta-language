// Generated from complete js/src/translation/javascript-array-assignment.js. Do not edit by hand.
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
                source_ml_source2_data_property_value18154,
            ) => source_ml_source2_data_property_value18154.clone(),
        }) == (self::ml_string::Text::from_units(vec![
            101, 120, 111, 116, 105, 99, 71, 101, 116,
        ]))) || ((match state.clone() {
            self::JavaScriptPrimitiveConversionState::PrimitiveConversion(
                _,
                _,
                _,
                source_ml_source2_data_property_value18156,
            ) => source_ml_source2_data_property_value18156.clone(),
        }) == (self::ml_string::Text::from_units(vec![
            101, 120, 111, 116, 105, 99, 71, 101, 116, 116, 101, 114,
        ])));
        {
            let first = ((match state.clone() {
                self::JavaScriptPrimitiveConversionState::PrimitiveConversion(
                    _,
                    _,
                    _,
                    source_ml_source2_data_property_value18158,
                ) => source_ml_source2_data_property_value18158.clone(),
            }) == (self::ml_string::Text::from_units(vec![
                102, 105, 114, 115, 116, 71, 101, 116,
            ]))) || ((match state.clone() {
                self::JavaScriptPrimitiveConversionState::PrimitiveConversion(
                    _,
                    _,
                    _,
                    source_ml_source2_data_property_value18160,
                ) => source_ml_source2_data_property_value18160.clone(),
            }) == (self::ml_string::Text::from_units(vec![
                102, 105, 114, 115, 116, 71, 101, 116, 116, 101, 114,
            ])));
            if exotic
                && (match method.clone() {
                    self::JavaScriptSemanticValue::Undefined => true,
                    _ => false,
                } || match method.clone() {
                    self::JavaScriptSemanticValue::Null => true,
                    _ => false,
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
                    self::JavaScriptSemanticValue::Boolean(_) => {
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
                    self::JavaScriptSemanticValue::Number(_) => {
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
                    self::JavaScriptSemanticValue::Integer(_) => {
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
                    self::JavaScriptSemanticValue::Text(_) => {
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
                    self::JavaScriptSemanticValue::SourceText(_, _) => {
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
                    self::JavaScriptSemanticValue::Symbol(_) => {
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
                    self::JavaScriptSemanticValue::Object(_, callable) => {
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
                                        std::rc::Rc::new(std::cell::RefCell::new(vec![self::read_java_script_primitive_conversion_spelling(&(match state.clone() {
                                            self::JavaScriptPrimitiveConversionState::PrimitiveConversion(_, _, source_ml_source2_data_property_value18187, _) => {
                                                source_ml_source2_data_property_value18187.clone()
                                            }
                                        }))]))
                                    } else {
                                        std::rc::Rc::new(std::cell::RefCell::new(Vec::<
                                            self::JavaScriptSemanticValue,
                                        >::new(
                                        )))
                                    };
                                    self::JavaScriptPrimitiveConversionTransition::PrimitiveCall(Box::new(self::JavaScriptPrimitiveConversionState::PrimitiveConversion(Box::new(match state.clone() {
                                        self::JavaScriptPrimitiveConversionState::PrimitiveConversion(source_ml_source2_data_property_value18188, _, _, _) => {
                                            let source_ml_source2_data_property_value18188 = *source_ml_source2_data_property_value18188;
                                            source_ml_source2_data_property_value18188.clone()
                                        }
                                    }), Box::new(match state.clone() {
                                        self::JavaScriptPrimitiveConversionState::PrimitiveConversion(_, source_ml_source2_data_property_value18189, _, _) => {
                                            let source_ml_source2_data_property_value18189 = *source_ml_source2_data_property_value18189;
                                            source_ml_source2_data_property_value18189.clone()
                                        }
                                    }), match state.clone() {
                                        self::JavaScriptPrimitiveConversionState::PrimitiveConversion(_, _, source_ml_source2_data_property_value18190, _) => {
                                            source_ml_source2_data_property_value18190.clone()
                                        }
                                    }, phase)), Box::new(method.clone()), Box::new(match state.clone() {
                                        self::JavaScriptPrimitiveConversionState::PrimitiveConversion(source_ml_source2_data_property_value18191, _, _, _) => {
                                            let source_ml_source2_data_property_value18191 = *source_ml_source2_data_property_value18191;
                                            source_ml_source2_data_property_value18191.clone()
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
            let ml_library_inequality_1 = error != (self::ml_string::Text::from_units(vec![]));
            if ml_library_inequality_1 {
                self::JavaScriptPrimitiveConversionTransition::PrimitiveInvalid(error.clone())
            } else if (((match state.clone() {
                self::JavaScriptPrimitiveConversionState::PrimitiveConversion(
                    _,
                    _,
                    _,
                    source_ml_source2_data_property_value18193,
                ) => source_ml_source2_data_property_value18193.clone(),
            }) != (self::ml_string::Text::from_units(vec![
                101, 120, 111, 116, 105, 99, 71, 101, 116,
            ]))) && ((match state.clone() {
                self::JavaScriptPrimitiveConversionState::PrimitiveConversion(
                    _,
                    _,
                    _,
                    source_ml_source2_data_property_value18195,
                ) => source_ml_source2_data_property_value18195.clone(),
            }) != (self::ml_string::Text::from_units(vec![
                102, 105, 114, 115, 116, 71, 101, 116,
            ]))))
                && ((match state.clone() {
                    self::JavaScriptPrimitiveConversionState::PrimitiveConversion(
                        _,
                        _,
                        _,
                        source_ml_source2_data_property_value18197,
                    ) => source_ml_source2_data_property_value18197.clone(),
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
                            source_ml_source2_data_property_value18198,
                            _,
                            _,
                            _,
                        ) => {
                            let source_ml_source2_data_property_value18198 =
                                *source_ml_source2_data_property_value18198;
                            source_ml_source2_data_property_value18198.clone()
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
                        self::JavaScriptSemanticValue::Boolean(_) => {
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
                        self::JavaScriptSemanticValue::Number(_) => {
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
                        self::JavaScriptSemanticValue::Integer(_) => {
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
                        self::JavaScriptSemanticValue::Text(_) => {
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
                        self::JavaScriptSemanticValue::SourceText(_, _) => {
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
                        self::JavaScriptSemanticValue::Symbol(_) => {
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
                        self::JavaScriptSemanticValue::Object(identity, _) => {
                            let first = (match state.clone() {
                                self::JavaScriptPrimitiveConversionState::PrimitiveConversion(
                                    _,
                                    _,
                                    _,
                                    source_ml_source2_data_property_value18208,
                                ) => source_ml_source2_data_property_value18208.clone(),
                            }) == (self::ml_string::Text::from_units(vec![
                                102, 105, 114, 115, 116, 71, 101, 116,
                            ]));
                            {
                                let method = if (match state.clone() {
                                        self::JavaScriptPrimitiveConversionState::PrimitiveConversion(_, _, source_ml_source2_data_property_value18210, _) => {
                                            source_ml_source2_data_property_value18210.clone()
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
                                            self::JavaScriptPrimitiveConversionState::PrimitiveConversion(_, _, _, source_ml_source2_data_property_value18212) => {
                                                source_ml_source2_data_property_value18212.clone()
                                            }
                                        }) == (self::ml_string::Text::from_units(vec![101, 120, 111, 116, 105, 99, 71, 101, 116])) {
                                            match state.clone() {
                                                self::JavaScriptPrimitiveConversionState::PrimitiveConversion(_, source_ml_source2_data_property_value18213, _, _) => {
                                                    let source_ml_source2_data_property_value18213 = *source_ml_source2_data_property_value18213;
                                                    source_ml_source2_data_property_value18213.clone()
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
                                        if (({ plan.borrow().clone() }).3)
                                            != (self::ml_string::Text::from_units(vec![]))
                                        {
                                            self::JavaScriptPrimitiveConversionTransition::PrimitiveInvalid(({ plan.borrow().clone() }).3)
                                        } else if (({ plan.borrow().clone() }).0)
                                            == (self::ml_string::Text::from_units(vec![
                                                99, 97, 108, 108,
                                            ]))
                                        {
                                            {
                                                let phase = if (match state.clone() {
                                                            self::JavaScriptPrimitiveConversionState::PrimitiveConversion(_, _, _, source_ml_source2_data_property_value18215) => {
                                                                source_ml_source2_data_property_value18215.clone()
                                                            }
                                                        }) == (self::ml_string::Text::from_units(vec![101, 120, 111, 116, 105, 99, 71, 101, 116])) {
                                                            self::ml_string::Text::from_units(vec![101, 120, 111, 116, 105, 99, 71, 101, 116, 116, 101, 114])
                                                        } else if first {
                                                                self::ml_string::Text::from_units(vec![102, 105, 114, 115, 116, 71, 101, 116, 116, 101, 114])
                                                            } else {
                                                                self::ml_string::Text::from_units(vec![115, 101, 99, 111, 110, 100, 71, 101, 116, 116, 101, 114])
                                                            };
                                                self::JavaScriptPrimitiveConversionTransition::PrimitiveCall(Box::new(self::JavaScriptPrimitiveConversionState::PrimitiveConversion(Box::new(receiver.clone()), Box::new(match state.clone() {
                                                            self::JavaScriptPrimitiveConversionState::PrimitiveConversion(_, source_ml_source2_data_property_value18216, _, _) => {
                                                                let source_ml_source2_data_property_value18216 = *source_ml_source2_data_property_value18216;
                                                                source_ml_source2_data_property_value18216.clone()
                                                            }
                                                        }), match state.clone() {
                                                            self::JavaScriptPrimitiveConversionState::PrimitiveConversion(_, _, source_ml_source2_data_property_value18217, _) => {
                                                                source_ml_source2_data_property_value18217.clone()
                                                            }
                                                        }, phase)), Box::new(({ plan.borrow().clone() }).2), Box::new(({ plan.borrow().clone() }).4), ({ plan.borrow().clone() }).1)
                                            }
                                        } else if (({ plan.borrow().clone() }).0)
                                            != (self::ml_string::Text::from_units(vec![
                                                118, 97, 108, 117, 101,
                                            ]))
                                        {
                                            self::JavaScriptPrimitiveConversionTransition::PrimitiveInvalid(self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 71, 101, 116, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 118, 97, 108, 117, 101, 32, 111, 114, 32, 112, 101, 110, 100, 105, 110, 103, 32, 103, 101, 116, 116, 101, 114]))
                                        } else {
                                            self::apply_java_script_primitive_conversion_method(
                                                state.clone(),
                                                ({ plan.borrow().clone() }).5,
                                            )
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
                            self::JavaScriptSemanticValue::Boolean(_) => {
                                self::JavaScriptPrimitiveConversionTransition::PrimitiveCompleted(Box::new(input.clone()))
                            }
                            self::JavaScriptSemanticValue::Number(_) => {
                                self::JavaScriptPrimitiveConversionTransition::PrimitiveCompleted(Box::new(input.clone()))
                            }
                            self::JavaScriptSemanticValue::Integer(_) => {
                                self::JavaScriptPrimitiveConversionTransition::PrimitiveCompleted(Box::new(input.clone()))
                            }
                            self::JavaScriptSemanticValue::Text(_) => {
                                self::JavaScriptPrimitiveConversionTransition::PrimitiveCompleted(Box::new(input.clone()))
                            }
                            self::JavaScriptSemanticValue::SourceText(_, _) => {
                                self::JavaScriptPrimitiveConversionTransition::PrimitiveCompleted(Box::new(input.clone()))
                            }
                            self::JavaScriptSemanticValue::Symbol(_) => {
                                self::JavaScriptPrimitiveConversionTransition::PrimitiveCompleted(Box::new(input.clone()))
                            }
                            self::JavaScriptSemanticValue::Object(_, _) => {
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
