// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptDescriptorConversionState;
use super::JavaScriptDescriptorConversionTransition;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptPrimitiveConversionCompletion;
use super::JavaScriptPrimitiveConversionState;
use super::JavaScriptPrimitiveConversionTransition;
use super::JavaScriptSemanticCompletion;
use super::JavaScriptSemanticValue;
use super::apply_java_script_descriptor_field;
use super::ml_array;
use super::ml_number;
use super::ml_source_exception;
use super::ml_string;
use super::plan_java_script_ordinary_get;
use super::read_java_script_descriptor_conversion_state_error;
use super::read_java_script_ordinary_heap_error;
use super::read_java_script_ordinary_value_error;
use super::step_java_script_descriptor_conversion;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn resume_java_script_descriptor_conversion(heap: self::JavaScriptOrdinaryHeap, state: self::JavaScriptDescriptorConversionState, completion: self::JavaScriptSemanticCompletion) -> self::JavaScriptDescriptorConversionTransition {
    {
        let error = self::read_java_script_descriptor_conversion_state_error(heap.clone(), state.clone());
        if (error != (self::ml_string::Text::from_units(vec![]))) {
            self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(error.clone())
        } else {
            if !(match state.clone() {
                self::JavaScriptDescriptorConversionState::DescriptorConversion(_, _, _, source_ml_source1_data_property_value922) => {
                    source_ml_source1_data_property_value922
                }
            }) {
                self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(self::ml_string::Text::from_units(vec![100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 114, 101, 115, 117, 109, 112, 116, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 101, 110, 100, 105, 110, 103, 32, 71, 101, 116]))
            } else {
                {
                    let value_error = self::read_java_script_ordinary_value_error(heap.clone(), match completion.clone() {
                        self::JavaScriptSemanticCompletion::NormalCompletion(source_ml_source1_data_property_value923) => {
                            let source_ml_source1_data_property_value923 = *source_ml_source1_data_property_value923;
                            source_ml_source1_data_property_value923.clone()
                        }
                        self::JavaScriptSemanticCompletion::ThrowCompletion(source_ml_source1_data_property_value924) => {
                            let source_ml_source1_data_property_value924 = *source_ml_source1_data_property_value924;
                            source_ml_source1_data_property_value924.clone()
                        }
                    });
                    if (value_error != (self::ml_string::Text::from_units(vec![]))) {
                        self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(value_error.clone())
                    } else {
                        match completion.clone() {
                            self::JavaScriptSemanticCompletion::NormalCompletion(value) => {
                                let value = *value;
                                {
                                    let applied = self::apply_java_script_descriptor_field(state.clone(), value.clone());
                                    match applied.clone() {
                                        self::JavaScriptDescriptorConversionTransition::DescriptorContinuation(state_2) => {
                                            let state_2 = *state_2;
                                            self::step_java_script_descriptor_conversion(heap.clone(), state_2.clone())
                                        }
                                        self::JavaScriptDescriptorConversionTransition::DescriptorCall(source_ml_source1_f925, source_ml_source1_f926, source_ml_source1_f927, source_ml_source1_f928) => {
                                            let source_ml_source1_f925 = *source_ml_source1_f925;
                                            let source_ml_source1_f926 = *source_ml_source1_f926;
                                            let source_ml_source1_f927 = *source_ml_source1_f927;
                                            applied.clone()
                                        }
                                        self::JavaScriptDescriptorConversionTransition::DescriptorCompleted(source_ml_source1_f929) => {
                                            applied.clone()
                                        }
                                        self::JavaScriptDescriptorConversionTransition::DescriptorThrow(source_ml_source1_f930) => {
                                            let source_ml_source1_f930 = *source_ml_source1_f930;
                                            applied.clone()
                                        }
                                        self::JavaScriptDescriptorConversionTransition::DescriptorIntrinsicError(source_ml_source1_f931, source_ml_source1_f932) => {
                                            applied.clone()
                                        }
                                        self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(source_ml_source1_f933) => {
                                            applied.clone()
                                        }
                                    }
                                }
                            }
                            self::JavaScriptSemanticCompletion::ThrowCompletion(value_2) => {
                                let value_2 = *value_2;
                                self::JavaScriptDescriptorConversionTransition::DescriptorThrow(Box::new(value_2.clone()))
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
pub fn read_java_script_primitive_conversion_spelling(name: &self::ml_string::Text) -> self::JavaScriptSemanticValue {
    if ((*name) == (self::ml_string::Text::from_units(vec![100, 101, 102, 97, 117, 108, 116]))) {
        self::JavaScriptSemanticValue::Text(vec![100f64, 101f64, 102f64, 97f64, 117f64, 108f64, 116f64])
    } else {
        if ((*name) == (self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114]))) {
            self::JavaScriptSemanticValue::Text(vec![110f64, 117f64, 109f64, 98f64, 101f64, 114f64])
        } else {
            if ((*name) == (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103]))) {
                self::JavaScriptSemanticValue::Text(vec![115f64, 116f64, 114f64, 105f64, 110f64, 103f64])
            } else {
                if ((*name) == (self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101, 79, 102]))) {
                    self::JavaScriptSemanticValue::Text(vec![118f64, 97f64, 108f64, 117f64, 101f64, 79f64, 102f64])
                } else {
                    if ((*name) == (self::ml_string::Text::from_units(vec![116, 111, 83, 116, 114, 105, 110, 103]))) {
                        self::JavaScriptSemanticValue::Text(vec![116f64, 111f64, 83f64, 116f64, 114f64, 105f64, 110f64, 103f64])
                    } else {
                        (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 115, 112, 101, 108, 108, 105, 110, 103, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 110, 97, 109, 101])).0, message_present: true }).raise()
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_primitive_conversion_state_error(heap: self::JavaScriptOrdinaryHeap, state: self::JavaScriptPrimitiveConversionState) -> self::ml_string::Text {
    {
        let error = self::read_java_script_ordinary_heap_error(heap.clone());
        if (error != (self::ml_string::Text::from_units(vec![]))) {
            error.clone()
        } else {
            if {
                let source_ml_source1_s935 = match state.clone() {
                    self::JavaScriptPrimitiveConversionState::PrimitiveConversion(source_ml_source1_data_property_value943, _, _, _) => {
                        let source_ml_source1_data_property_value943 = *source_ml_source1_data_property_value943;
                        source_ml_source1_data_property_value943.clone()
                    }
                };
                match source_ml_source1_s935.clone() {
                    self::JavaScriptSemanticValue::Undefined => {
                        true
                    }
                    self::JavaScriptSemanticValue::Null => {
                        true
                    }
                    self::JavaScriptSemanticValue::Boolean(source_ml_source1_f936) => {
                        true
                    }
                    self::JavaScriptSemanticValue::Number(source_ml_source1_f937) => {
                        true
                    }
                    self::JavaScriptSemanticValue::Integer(source_ml_source1_f938) => {
                        true
                    }
                    self::JavaScriptSemanticValue::Text(source_ml_source1_f939) => {
                        true
                    }
                    self::JavaScriptSemanticValue::Symbol(source_ml_source1_f940) => {
                        true
                    }
                    self::JavaScriptSemanticValue::Object(source_ml_source1_f941, source_ml_source1_f942) => {
                        false
                    }
                }
            } {
                self::ml_string::Text::from_units(vec![97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 114, 101, 99, 101, 105, 118, 101, 114])
            } else {
                {
                    let receiver_error = self::read_java_script_ordinary_value_error(heap.clone(), match state.clone() {
                        self::JavaScriptPrimitiveConversionState::PrimitiveConversion(source_ml_source1_data_property_value944, _, _, _) => {
                            let source_ml_source1_data_property_value944 = *source_ml_source1_data_property_value944;
                            source_ml_source1_data_property_value944.clone()
                        }
                    });
                    if (receiver_error != (self::ml_string::Text::from_units(vec![]))) {
                        receiver_error.clone()
                    } else {
                        if {
                            let source_ml_source1_s946 = match state.clone() {
                                self::JavaScriptPrimitiveConversionState::PrimitiveConversion(_, source_ml_source1_data_property_value954, _, _) => {
                                    let source_ml_source1_data_property_value954 = *source_ml_source1_data_property_value954;
                                    source_ml_source1_data_property_value954.clone()
                                }
                            };
                            match source_ml_source1_s946.clone() {
                                self::JavaScriptSemanticValue::Undefined => {
                                    true
                                }
                                self::JavaScriptSemanticValue::Null => {
                                    true
                                }
                                self::JavaScriptSemanticValue::Boolean(source_ml_source1_f947) => {
                                    true
                                }
                                self::JavaScriptSemanticValue::Number(source_ml_source1_f948) => {
                                    true
                                }
                                self::JavaScriptSemanticValue::Integer(source_ml_source1_f949) => {
                                    true
                                }
                                self::JavaScriptSemanticValue::Text(source_ml_source1_f950) => {
                                    true
                                }
                                self::JavaScriptSemanticValue::Symbol(source_ml_source1_f951) => {
                                    false
                                }
                                self::JavaScriptSemanticValue::Object(source_ml_source1_f952, source_ml_source1_f953) => {
                                    true
                                }
                            }
                        } {
                            self::ml_string::Text::from_units(vec![116, 104, 101, 32, 101, 120, 111, 116, 105, 99, 32, 109, 101, 116, 104, 111, 100, 32, 107, 101, 121, 32, 109, 117, 115, 116, 32, 98, 101, 32, 116, 104, 101, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 119, 101, 108, 108, 45, 107, 110, 111, 119, 110, 32, 83, 121, 109, 98, 111, 108])
                        } else {
                            {
                                let key_error = self::read_java_script_ordinary_value_error(heap.clone(), match state.clone() {
                                    self::JavaScriptPrimitiveConversionState::PrimitiveConversion(_, source_ml_source1_data_property_value955, _, _) => {
                                        let source_ml_source1_data_property_value955 = *source_ml_source1_data_property_value955;
                                        source_ml_source1_data_property_value955.clone()
                                    }
                                });
                                if (key_error != (self::ml_string::Text::from_units(vec![]))) {
                                    key_error.clone()
                                } else {
                                    if ((((match state.clone() {
                                        self::JavaScriptPrimitiveConversionState::PrimitiveConversion(_, _, source_ml_source1_data_property_value957, _) => {
                                            source_ml_source1_data_property_value957.clone()
                                        }
                                    }) != (self::ml_string::Text::from_units(vec![100, 101, 102, 97, 117, 108, 116]))) && ((match state.clone() {
                                        self::JavaScriptPrimitiveConversionState::PrimitiveConversion(_, _, source_ml_source1_data_property_value959, _) => {
                                            source_ml_source1_data_property_value959.clone()
                                        }
                                    }) != (self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114])))) && ((match state.clone() {
                                        self::JavaScriptPrimitiveConversionState::PrimitiveConversion(_, _, source_ml_source1_data_property_value961, _) => {
                                            source_ml_source1_data_property_value961.clone()
                                        }
                                    }) != (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103])))) {
                                        self::ml_string::Text::from_units(vec![97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 97, 110, 32, 101, 120, 97, 99, 116, 32, 112, 114, 101, 102, 101, 114, 114, 101, 100, 45, 116, 121, 112, 101, 32, 104, 105, 110, 116])
                                    } else {
                                        if !(self::ml_array::includes(&vec![self::ml_string::Text::from_units(vec![101, 120, 111, 116, 105, 99, 71, 101, 116]), self::ml_string::Text::from_units(vec![102, 105, 114, 115, 116, 71, 101, 116]), self::ml_string::Text::from_units(vec![115, 101, 99, 111, 110, 100, 71, 101, 116]), self::ml_string::Text::from_units(vec![101, 120, 111, 116, 105, 99, 71, 101, 116, 116, 101, 114]), self::ml_string::Text::from_units(vec![102, 105, 114, 115, 116, 71, 101, 116, 116, 101, 114]), self::ml_string::Text::from_units(vec![115, 101, 99, 111, 110, 100, 71, 101, 116, 116, 101, 114]), self::ml_string::Text::from_units(vec![101, 120, 111, 116, 105, 99, 67, 97, 108, 108]), self::ml_string::Text::from_units(vec![102, 105, 114, 115, 116, 67, 97, 108, 108]), self::ml_string::Text::from_units(vec![115, 101, 99, 111, 110, 100, 67, 97, 108, 108])], &match state.clone() {
                                            self::JavaScriptPrimitiveConversionState::PrimitiveConversion(_, _, _, source_ml_source1_data_property_value962) => {
                                                source_ml_source1_data_property_value962.clone()
                                            }
                                        }, 0f64, false, |a, b| a == b)) {
                                            self::ml_string::Text::from_units(vec![97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 101, 120, 97, 99, 116, 32, 109, 101, 116, 104, 111, 100, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110])
                                        } else {
                                            self::ml_string::Text::from_units(vec![])
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
pub fn continue_java_script_primitive_conversion(state: self::JavaScriptPrimitiveConversionState, phase: &self::ml_string::Text) -> self::JavaScriptPrimitiveConversionTransition {
    self::JavaScriptPrimitiveConversionTransition::PrimitiveContinuation(Box::new(self::JavaScriptPrimitiveConversionState::PrimitiveConversion(Box::new(match state.clone() {
        self::JavaScriptPrimitiveConversionState::PrimitiveConversion(source_ml_source1_data_property_value963, _, _, _) => {
            let source_ml_source1_data_property_value963 = *source_ml_source1_data_property_value963;
            source_ml_source1_data_property_value963.clone()
        }
    }), Box::new(match state.clone() {
        self::JavaScriptPrimitiveConversionState::PrimitiveConversion(_, source_ml_source1_data_property_value964, _, _) => {
            let source_ml_source1_data_property_value964 = *source_ml_source1_data_property_value964;
            source_ml_source1_data_property_value964.clone()
        }
    }), match state.clone() {
        self::JavaScriptPrimitiveConversionState::PrimitiveConversion(_, _, source_ml_source1_data_property_value965, _) => {
            source_ml_source1_data_property_value965.clone()
        }
    }, phase.clone())))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn apply_java_script_primitive_conversion_method(state: self::JavaScriptPrimitiveConversionState, method: self::JavaScriptSemanticValue) -> self::JavaScriptPrimitiveConversionTransition {
    {
        let exotic = (((match state.clone() {
            self::JavaScriptPrimitiveConversionState::PrimitiveConversion(_, _, _, source_ml_source1_data_property_value967) => {
                source_ml_source1_data_property_value967.clone()
            }
        }) == (self::ml_string::Text::from_units(vec![101, 120, 111, 116, 105, 99, 71, 101, 116]))) || ((match state.clone() {
            self::JavaScriptPrimitiveConversionState::PrimitiveConversion(_, _, _, source_ml_source1_data_property_value969) => {
                source_ml_source1_data_property_value969.clone()
            }
        }) == (self::ml_string::Text::from_units(vec![101, 120, 111, 116, 105, 99, 71, 101, 116, 116, 101, 114]))));
        {
            let first = (((match state.clone() {
                self::JavaScriptPrimitiveConversionState::PrimitiveConversion(_, _, _, source_ml_source1_data_property_value971) => {
                    source_ml_source1_data_property_value971.clone()
                }
            }) == (self::ml_string::Text::from_units(vec![102, 105, 114, 115, 116, 71, 101, 116]))) || ((match state.clone() {
                self::JavaScriptPrimitiveConversionState::PrimitiveConversion(_, _, _, source_ml_source1_data_property_value973) => {
                    source_ml_source1_data_property_value973.clone()
                }
            }) == (self::ml_string::Text::from_units(vec![102, 105, 114, 115, 116, 71, 101, 116, 116, 101, 114]))));
            if (exotic && (match method.clone() {
                self::JavaScriptSemanticValue::Undefined => {
                    true
                }
                self::JavaScriptSemanticValue::Null => {
                    false
                }
                self::JavaScriptSemanticValue::Boolean(source_ml_source1_f974) => {
                    false
                }
                self::JavaScriptSemanticValue::Number(source_ml_source1_f975) => {
                    false
                }
                self::JavaScriptSemanticValue::Integer(source_ml_source1_f976) => {
                    false
                }
                self::JavaScriptSemanticValue::Text(source_ml_source1_f977) => {
                    false
                }
                self::JavaScriptSemanticValue::Symbol(source_ml_source1_f978) => {
                    false
                }
                self::JavaScriptSemanticValue::Object(source_ml_source1_f979, source_ml_source1_f980) => {
                    false
                }
            } || match method.clone() {
                self::JavaScriptSemanticValue::Undefined => {
                    false
                }
                self::JavaScriptSemanticValue::Null => {
                    true
                }
                self::JavaScriptSemanticValue::Boolean(source_ml_source1_f981) => {
                    false
                }
                self::JavaScriptSemanticValue::Number(source_ml_source1_f982) => {
                    false
                }
                self::JavaScriptSemanticValue::Integer(source_ml_source1_f983) => {
                    false
                }
                self::JavaScriptSemanticValue::Text(source_ml_source1_f984) => {
                    false
                }
                self::JavaScriptSemanticValue::Symbol(source_ml_source1_f985) => {
                    false
                }
                self::JavaScriptSemanticValue::Object(source_ml_source1_f986, source_ml_source1_f987) => {
                    false
                }
            })) {
                self::continue_java_script_primitive_conversion(state.clone(), &(self::ml_string::Text::from_units(vec![102, 105, 114, 115, 116, 71, 101, 116])))
            } else {
                match method.clone() {
                    self::JavaScriptSemanticValue::Undefined => {
                        if exotic {
                            self::JavaScriptPrimitiveConversionTransition::PrimitiveIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![97, 110, 32, 101, 120, 111, 116, 105, 99, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 109, 101, 116, 104, 111, 100, 32, 109, 117, 115, 116, 32, 98, 101, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 111, 114, 32, 97, 98, 115, 101, 110, 116]))
                        } else {
                            if first {
                                self::continue_java_script_primitive_conversion(state.clone(), &(self::ml_string::Text::from_units(vec![115, 101, 99, 111, 110, 100, 71, 101, 116])))
                            } else {
                                self::JavaScriptPrimitiveConversionTransition::PrimitiveIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 101, 120, 104, 97, 117, 115, 116, 101, 100, 32, 105, 116, 115, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 45, 112, 114, 111, 100, 117, 99, 105, 110, 103, 32, 109, 101, 116, 104, 111, 100, 115]))
                            }
                        }
                    }
                    self::JavaScriptSemanticValue::Null => {
                        if exotic {
                            self::JavaScriptPrimitiveConversionTransition::PrimitiveIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![97, 110, 32, 101, 120, 111, 116, 105, 99, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 109, 101, 116, 104, 111, 100, 32, 109, 117, 115, 116, 32, 98, 101, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 111, 114, 32, 97, 98, 115, 101, 110, 116]))
                        } else {
                            if first {
                                self::continue_java_script_primitive_conversion(state.clone(), &(self::ml_string::Text::from_units(vec![115, 101, 99, 111, 110, 100, 71, 101, 116])))
                            } else {
                                self::JavaScriptPrimitiveConversionTransition::PrimitiveIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 101, 120, 104, 97, 117, 115, 116, 101, 100, 32, 105, 116, 115, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 45, 112, 114, 111, 100, 117, 99, 105, 110, 103, 32, 109, 101, 116, 104, 111, 100, 115]))
                            }
                        }
                    }
                    self::JavaScriptSemanticValue::Boolean(source_ml_source1_f988) => {
                        if exotic {
                            self::JavaScriptPrimitiveConversionTransition::PrimitiveIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![97, 110, 32, 101, 120, 111, 116, 105, 99, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 109, 101, 116, 104, 111, 100, 32, 109, 117, 115, 116, 32, 98, 101, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 111, 114, 32, 97, 98, 115, 101, 110, 116]))
                        } else {
                            if first {
                                self::continue_java_script_primitive_conversion(state.clone(), &(self::ml_string::Text::from_units(vec![115, 101, 99, 111, 110, 100, 71, 101, 116])))
                            } else {
                                self::JavaScriptPrimitiveConversionTransition::PrimitiveIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 101, 120, 104, 97, 117, 115, 116, 101, 100, 32, 105, 116, 115, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 45, 112, 114, 111, 100, 117, 99, 105, 110, 103, 32, 109, 101, 116, 104, 111, 100, 115]))
                            }
                        }
                    }
                    self::JavaScriptSemanticValue::Number(source_ml_source1_f989) => {
                        if exotic {
                            self::JavaScriptPrimitiveConversionTransition::PrimitiveIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![97, 110, 32, 101, 120, 111, 116, 105, 99, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 109, 101, 116, 104, 111, 100, 32, 109, 117, 115, 116, 32, 98, 101, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 111, 114, 32, 97, 98, 115, 101, 110, 116]))
                        } else {
                            if first {
                                self::continue_java_script_primitive_conversion(state.clone(), &(self::ml_string::Text::from_units(vec![115, 101, 99, 111, 110, 100, 71, 101, 116])))
                            } else {
                                self::JavaScriptPrimitiveConversionTransition::PrimitiveIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 101, 120, 104, 97, 117, 115, 116, 101, 100, 32, 105, 116, 115, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 45, 112, 114, 111, 100, 117, 99, 105, 110, 103, 32, 109, 101, 116, 104, 111, 100, 115]))
                            }
                        }
                    }
                    self::JavaScriptSemanticValue::Integer(source_ml_source1_f990) => {
                        if exotic {
                            self::JavaScriptPrimitiveConversionTransition::PrimitiveIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![97, 110, 32, 101, 120, 111, 116, 105, 99, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 109, 101, 116, 104, 111, 100, 32, 109, 117, 115, 116, 32, 98, 101, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 111, 114, 32, 97, 98, 115, 101, 110, 116]))
                        } else {
                            if first {
                                self::continue_java_script_primitive_conversion(state.clone(), &(self::ml_string::Text::from_units(vec![115, 101, 99, 111, 110, 100, 71, 101, 116])))
                            } else {
                                self::JavaScriptPrimitiveConversionTransition::PrimitiveIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 101, 120, 104, 97, 117, 115, 116, 101, 100, 32, 105, 116, 115, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 45, 112, 114, 111, 100, 117, 99, 105, 110, 103, 32, 109, 101, 116, 104, 111, 100, 115]))
                            }
                        }
                    }
                    self::JavaScriptSemanticValue::Text(source_ml_source1_f991) => {
                        if exotic {
                            self::JavaScriptPrimitiveConversionTransition::PrimitiveIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![97, 110, 32, 101, 120, 111, 116, 105, 99, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 109, 101, 116, 104, 111, 100, 32, 109, 117, 115, 116, 32, 98, 101, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 111, 114, 32, 97, 98, 115, 101, 110, 116]))
                        } else {
                            if first {
                                self::continue_java_script_primitive_conversion(state.clone(), &(self::ml_string::Text::from_units(vec![115, 101, 99, 111, 110, 100, 71, 101, 116])))
                            } else {
                                self::JavaScriptPrimitiveConversionTransition::PrimitiveIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 101, 120, 104, 97, 117, 115, 116, 101, 100, 32, 105, 116, 115, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 45, 112, 114, 111, 100, 117, 99, 105, 110, 103, 32, 109, 101, 116, 104, 111, 100, 115]))
                            }
                        }
                    }
                    self::JavaScriptSemanticValue::Symbol(source_ml_source1_f992) => {
                        if exotic {
                            self::JavaScriptPrimitiveConversionTransition::PrimitiveIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![97, 110, 32, 101, 120, 111, 116, 105, 99, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 109, 101, 116, 104, 111, 100, 32, 109, 117, 115, 116, 32, 98, 101, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 111, 114, 32, 97, 98, 115, 101, 110, 116]))
                        } else {
                            if first {
                                self::continue_java_script_primitive_conversion(state.clone(), &(self::ml_string::Text::from_units(vec![115, 101, 99, 111, 110, 100, 71, 101, 116])))
                            } else {
                                self::JavaScriptPrimitiveConversionTransition::PrimitiveIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 101, 120, 104, 97, 117, 115, 116, 101, 100, 32, 105, 116, 115, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 45, 112, 114, 111, 100, 117, 99, 105, 110, 103, 32, 109, 101, 116, 104, 111, 100, 115]))
                            }
                        }
                    }
                    self::JavaScriptSemanticValue::Object(source_ml_source1_f993, callable) => {
                        if !callable {
                            if exotic {
                                self::JavaScriptPrimitiveConversionTransition::PrimitiveIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![97, 110, 32, 101, 120, 111, 116, 105, 99, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 109, 101, 116, 104, 111, 100, 32, 109, 117, 115, 116, 32, 98, 101, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 111, 114, 32, 97, 98, 115, 101, 110, 116]))
                            } else {
                                if first {
                                    self::continue_java_script_primitive_conversion(state.clone(), &(self::ml_string::Text::from_units(vec![115, 101, 99, 111, 110, 100, 71, 101, 116])))
                                } else {
                                    self::JavaScriptPrimitiveConversionTransition::PrimitiveIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 101, 120, 104, 97, 117, 115, 116, 101, 100, 32, 105, 116, 115, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 45, 112, 114, 111, 100, 117, 99, 105, 110, 103, 32, 109, 101, 116, 104, 111, 100, 115]))
                                }
                            }
                        } else {
                            {
                                let phase = if exotic {
                                    self::ml_string::Text::from_units(vec![101, 120, 111, 116, 105, 99, 67, 97, 108, 108])
                                } else {
                                    if first {
                                        self::ml_string::Text::from_units(vec![102, 105, 114, 115, 116, 67, 97, 108, 108])
                                    } else {
                                        self::ml_string::Text::from_units(vec![115, 101, 99, 111, 110, 100, 67, 97, 108, 108])
                                    }
                                };
                                {
                                    let arguments_ = if exotic {
                                        vec![self::read_java_script_primitive_conversion_spelling(&(match state.clone() {
                                            self::JavaScriptPrimitiveConversionState::PrimitiveConversion(_, _, source_ml_source1_data_property_value994, _) => {
                                                source_ml_source1_data_property_value994.clone()
                                            }
                                        }))]
                                    } else {
                                        Vec::<self::JavaScriptSemanticValue>::new()
                                    };
                                    self::JavaScriptPrimitiveConversionTransition::PrimitiveCall(Box::new(self::JavaScriptPrimitiveConversionState::PrimitiveConversion(Box::new(match state.clone() {
                                        self::JavaScriptPrimitiveConversionState::PrimitiveConversion(source_ml_source1_data_property_value995, _, _, _) => {
                                            let source_ml_source1_data_property_value995 = *source_ml_source1_data_property_value995;
                                            source_ml_source1_data_property_value995.clone()
                                        }
                                    }), Box::new(match state.clone() {
                                        self::JavaScriptPrimitiveConversionState::PrimitiveConversion(_, source_ml_source1_data_property_value996, _, _) => {
                                            let source_ml_source1_data_property_value996 = *source_ml_source1_data_property_value996;
                                            source_ml_source1_data_property_value996.clone()
                                        }
                                    }), match state.clone() {
                                        self::JavaScriptPrimitiveConversionState::PrimitiveConversion(_, _, source_ml_source1_data_property_value997, _) => {
                                            source_ml_source1_data_property_value997.clone()
                                        }
                                    }, phase.clone())), Box::new(method.clone()), Box::new(match state.clone() {
                                        self::JavaScriptPrimitiveConversionState::PrimitiveConversion(source_ml_source1_data_property_value998, _, _, _) => {
                                            let source_ml_source1_data_property_value998 = *source_ml_source1_data_property_value998;
                                            source_ml_source1_data_property_value998.clone()
                                        }
                                    }), arguments_)
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
pub fn step_java_script_primitive_conversion(heap: self::JavaScriptOrdinaryHeap, state: self::JavaScriptPrimitiveConversionState) -> self::JavaScriptPrimitiveConversionTransition {
    {
        let error = self::read_java_script_primitive_conversion_state_error(heap.clone(), state.clone());
        if (error != (self::ml_string::Text::from_units(vec![]))) {
            self::JavaScriptPrimitiveConversionTransition::PrimitiveInvalid(error.clone())
        } else {
            if ((((match state.clone() {
                self::JavaScriptPrimitiveConversionState::PrimitiveConversion(_, _, _, source_ml_source1_data_property_value1000) => {
                    source_ml_source1_data_property_value1000.clone()
                }
            }) != (self::ml_string::Text::from_units(vec![101, 120, 111, 116, 105, 99, 71, 101, 116]))) && ((match state.clone() {
                self::JavaScriptPrimitiveConversionState::PrimitiveConversion(_, _, _, source_ml_source1_data_property_value1002) => {
                    source_ml_source1_data_property_value1002.clone()
                }
            }) != (self::ml_string::Text::from_units(vec![102, 105, 114, 115, 116, 71, 101, 116])))) && ((match state.clone() {
                self::JavaScriptPrimitiveConversionState::PrimitiveConversion(_, _, _, source_ml_source1_data_property_value1004) => {
                    source_ml_source1_data_property_value1004.clone()
                }
            }) != (self::ml_string::Text::from_units(vec![115, 101, 99, 111, 110, 100, 71, 101, 116])))) {
                self::JavaScriptPrimitiveConversionTransition::PrimitiveInvalid(self::ml_string::Text::from_units(vec![97, 32, 112, 101, 110, 100, 105, 110, 103, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 99, 97, 108, 108, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 109, 112, 108, 101, 116, 105, 111, 110]))
            } else {
                {
                    let receiver = match state.clone() {
                        self::JavaScriptPrimitiveConversionState::PrimitiveConversion(source_ml_source1_data_property_value1005, _, _, _) => {
                            let source_ml_source1_data_property_value1005 = *source_ml_source1_data_property_value1005;
                            source_ml_source1_data_property_value1005.clone()
                        }
                    };
                    match receiver.clone() {
                        self::JavaScriptSemanticValue::Undefined => {
                            self::JavaScriptPrimitiveConversionTransition::PrimitiveInvalid(self::ml_string::Text::from_units(vec![97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 114, 101, 99, 101, 105, 118, 101, 114]))
                        }
                        self::JavaScriptSemanticValue::Null => {
                            self::JavaScriptPrimitiveConversionTransition::PrimitiveInvalid(self::ml_string::Text::from_units(vec![97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 114, 101, 99, 101, 105, 118, 101, 114]))
                        }
                        self::JavaScriptSemanticValue::Boolean(source_ml_source1_f1006) => {
                            self::JavaScriptPrimitiveConversionTransition::PrimitiveInvalid(self::ml_string::Text::from_units(vec![97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 114, 101, 99, 101, 105, 118, 101, 114]))
                        }
                        self::JavaScriptSemanticValue::Number(source_ml_source1_f1007) => {
                            self::JavaScriptPrimitiveConversionTransition::PrimitiveInvalid(self::ml_string::Text::from_units(vec![97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 114, 101, 99, 101, 105, 118, 101, 114]))
                        }
                        self::JavaScriptSemanticValue::Integer(source_ml_source1_f1008) => {
                            self::JavaScriptPrimitiveConversionTransition::PrimitiveInvalid(self::ml_string::Text::from_units(vec![97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 114, 101, 99, 101, 105, 118, 101, 114]))
                        }
                        self::JavaScriptSemanticValue::Text(source_ml_source1_f1009) => {
                            self::JavaScriptPrimitiveConversionTransition::PrimitiveInvalid(self::ml_string::Text::from_units(vec![97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 114, 101, 99, 101, 105, 118, 101, 114]))
                        }
                        self::JavaScriptSemanticValue::Symbol(source_ml_source1_f1010) => {
                            self::JavaScriptPrimitiveConversionTransition::PrimitiveInvalid(self::ml_string::Text::from_units(vec![97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 114, 101, 99, 101, 105, 118, 101, 114]))
                        }
                        self::JavaScriptSemanticValue::Object(identity, source_ml_source1_f1011) => {
                            {
                                let first = ((match state.clone() {
                                    self::JavaScriptPrimitiveConversionState::PrimitiveConversion(_, _, _, source_ml_source1_data_property_value1013) => {
                                        source_ml_source1_data_property_value1013.clone()
                                    }
                                }) == (self::ml_string::Text::from_units(vec![102, 105, 114, 115, 116, 71, 101, 116])));
                                {
                                    let method = if ((match state.clone() {
                                        self::JavaScriptPrimitiveConversionState::PrimitiveConversion(_, _, source_ml_source1_data_property_value1015, _) => {
                                            source_ml_source1_data_property_value1015.clone()
                                        }
                                    }) == (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103]))) {
                                        if first {
                                            self::ml_string::Text::from_units(vec![116, 111, 83, 116, 114, 105, 110, 103])
                                        } else {
                                            self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101, 79, 102])
                                        }
                                    } else {
                                        if first {
                                            self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101, 79, 102])
                                        } else {
                                            self::ml_string::Text::from_units(vec![116, 111, 83, 116, 114, 105, 110, 103])
                                        }
                                    };
                                    {
                                        let key = if ((match state.clone() {
                                            self::JavaScriptPrimitiveConversionState::PrimitiveConversion(_, _, _, source_ml_source1_data_property_value1017) => {
                                                source_ml_source1_data_property_value1017.clone()
                                            }
                                        }) == (self::ml_string::Text::from_units(vec![101, 120, 111, 116, 105, 99, 71, 101, 116]))) {
                                            match state.clone() {
                                                self::JavaScriptPrimitiveConversionState::PrimitiveConversion(_, source_ml_source1_data_property_value1018, _, _) => {
                                                    let source_ml_source1_data_property_value1018 = *source_ml_source1_data_property_value1018;
                                                    source_ml_source1_data_property_value1018.clone()
                                                }
                                            }
                                        } else {
                                            self::read_java_script_primitive_conversion_spelling(&(method))
                                        };
                                        {
                                            let plan = self::plan_java_script_ordinary_get(heap.clone(), identity.clone(), key.clone(), receiver.clone());
                                            if (((plan.clone()).3) != (self::ml_string::Text::from_units(vec![]))) {
                                                self::JavaScriptPrimitiveConversionTransition::PrimitiveInvalid((plan.clone()).3)
                                            } else {
                                                if (((plan.clone()).0) == (self::ml_string::Text::from_units(vec![99, 97, 108, 108]))) {
                                                    {
                                                        let phase = if ((match state.clone() {
                                                            self::JavaScriptPrimitiveConversionState::PrimitiveConversion(_, _, _, source_ml_source1_data_property_value1020) => {
                                                                source_ml_source1_data_property_value1020.clone()
                                                            }
                                                        }) == (self::ml_string::Text::from_units(vec![101, 120, 111, 116, 105, 99, 71, 101, 116]))) {
                                                            self::ml_string::Text::from_units(vec![101, 120, 111, 116, 105, 99, 71, 101, 116, 116, 101, 114])
                                                        } else {
                                                            if first {
                                                                self::ml_string::Text::from_units(vec![102, 105, 114, 115, 116, 71, 101, 116, 116, 101, 114])
                                                            } else {
                                                                self::ml_string::Text::from_units(vec![115, 101, 99, 111, 110, 100, 71, 101, 116, 116, 101, 114])
                                                            }
                                                        };
                                                        self::JavaScriptPrimitiveConversionTransition::PrimitiveCall(Box::new(self::JavaScriptPrimitiveConversionState::PrimitiveConversion(Box::new(receiver.clone()), Box::new(match state.clone() {
                                                            self::JavaScriptPrimitiveConversionState::PrimitiveConversion(_, source_ml_source1_data_property_value1021, _, _) => {
                                                                let source_ml_source1_data_property_value1021 = *source_ml_source1_data_property_value1021;
                                                                source_ml_source1_data_property_value1021.clone()
                                                            }
                                                        }), match state.clone() {
                                                            self::JavaScriptPrimitiveConversionState::PrimitiveConversion(_, _, source_ml_source1_data_property_value1022, _) => {
                                                                source_ml_source1_data_property_value1022.clone()
                                                            }
                                                        }, phase.clone())), Box::new((plan.clone()).2), Box::new((plan.clone()).4), (plan.clone()).1)
                                                    }
                                                } else {
                                                    if (((plan.clone()).0) != (self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101]))) {
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
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn begin_java_script_primitive_conversion(heap: self::JavaScriptOrdinaryHeap, input: self::JavaScriptSemanticValue, exotic_key: self::JavaScriptSemanticValue, hint: &self::ml_string::Text) -> self::JavaScriptPrimitiveConversionTransition {
    {
        let heap_error = self::read_java_script_ordinary_heap_error(heap.clone());
        if (heap_error != (self::ml_string::Text::from_units(vec![]))) {
            self::JavaScriptPrimitiveConversionTransition::PrimitiveInvalid(heap_error.clone())
        } else {
            {
                let error = self::read_java_script_ordinary_value_error(heap.clone(), input.clone());
                if (error != (self::ml_string::Text::from_units(vec![]))) {
                    self::JavaScriptPrimitiveConversionTransition::PrimitiveInvalid(error.clone())
                } else {
                    if ((((*hint) != (self::ml_string::Text::from_units(vec![100, 101, 102, 97, 117, 108, 116]))) && ((*hint) != (self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114])))) && ((*hint) != (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103])))) {
                        self::JavaScriptPrimitiveConversionTransition::PrimitiveInvalid(self::ml_string::Text::from_units(vec![97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 97, 110, 32, 101, 120, 97, 99, 116, 32, 112, 114, 101, 102, 101, 114, 114, 101, 100, 45, 116, 121, 112, 101, 32, 104, 105, 110, 116]))
                    } else {
                        match input.clone() {
                            self::JavaScriptSemanticValue::Undefined => {
                                self::JavaScriptPrimitiveConversionTransition::PrimitiveCompleted(Box::new(input.clone()))
                            }
                            self::JavaScriptSemanticValue::Null => {
                                self::JavaScriptPrimitiveConversionTransition::PrimitiveCompleted(Box::new(input.clone()))
                            }
                            self::JavaScriptSemanticValue::Boolean(source_ml_source1_f1023) => {
                                self::JavaScriptPrimitiveConversionTransition::PrimitiveCompleted(Box::new(input.clone()))
                            }
                            self::JavaScriptSemanticValue::Number(source_ml_source1_f1024) => {
                                self::JavaScriptPrimitiveConversionTransition::PrimitiveCompleted(Box::new(input.clone()))
                            }
                            self::JavaScriptSemanticValue::Integer(source_ml_source1_f1025) => {
                                self::JavaScriptPrimitiveConversionTransition::PrimitiveCompleted(Box::new(input.clone()))
                            }
                            self::JavaScriptSemanticValue::Text(source_ml_source1_f1026) => {
                                self::JavaScriptPrimitiveConversionTransition::PrimitiveCompleted(Box::new(input.clone()))
                            }
                            self::JavaScriptSemanticValue::Symbol(source_ml_source1_f1027) => {
                                self::JavaScriptPrimitiveConversionTransition::PrimitiveCompleted(Box::new(input.clone()))
                            }
                            self::JavaScriptSemanticValue::Object(source_ml_source1_f1028, source_ml_source1_f1029) => {
                                self::step_java_script_primitive_conversion(heap.clone(), self::JavaScriptPrimitiveConversionState::PrimitiveConversion(Box::new(input.clone()), Box::new(exotic_key.clone()), hint.clone(), self::ml_string::Text::from_units(vec![101, 120, 111, 116, 105, 99, 71, 101, 116])))
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
pub fn resume_java_script_primitive_conversion(heap: self::JavaScriptOrdinaryHeap, state: self::JavaScriptPrimitiveConversionState, completion: self::JavaScriptPrimitiveConversionCompletion) -> self::JavaScriptPrimitiveConversionTransition {
    {
        let error = self::read_java_script_primitive_conversion_state_error(heap.clone(), state.clone());
        if (error != (self::ml_string::Text::from_units(vec![]))) {
            self::JavaScriptPrimitiveConversionTransition::PrimitiveInvalid(error.clone())
        } else {
            {
                let getter = ((((match state.clone() {
                    self::JavaScriptPrimitiveConversionState::PrimitiveConversion(_, _, _, source_ml_source1_data_property_value1031) => {
                        source_ml_source1_data_property_value1031.clone()
                    }
                }) == (self::ml_string::Text::from_units(vec![101, 120, 111, 116, 105, 99, 71, 101, 116, 116, 101, 114]))) || ((match state.clone() {
                    self::JavaScriptPrimitiveConversionState::PrimitiveConversion(_, _, _, source_ml_source1_data_property_value1033) => {
                        source_ml_source1_data_property_value1033.clone()
                    }
                }) == (self::ml_string::Text::from_units(vec![102, 105, 114, 115, 116, 71, 101, 116, 116, 101, 114])))) || ((match state.clone() {
                    self::JavaScriptPrimitiveConversionState::PrimitiveConversion(_, _, _, source_ml_source1_data_property_value1035) => {
                        source_ml_source1_data_property_value1035.clone()
                    }
                }) == (self::ml_string::Text::from_units(vec![115, 101, 99, 111, 110, 100, 71, 101, 116, 116, 101, 114]))));
                {
                    let call = ((((match state.clone() {
                        self::JavaScriptPrimitiveConversionState::PrimitiveConversion(_, _, _, source_ml_source1_data_property_value1037) => {
                            source_ml_source1_data_property_value1037.clone()
                        }
                    }) == (self::ml_string::Text::from_units(vec![101, 120, 111, 116, 105, 99, 67, 97, 108, 108]))) || ((match state.clone() {
                        self::JavaScriptPrimitiveConversionState::PrimitiveConversion(_, _, _, source_ml_source1_data_property_value1039) => {
                            source_ml_source1_data_property_value1039.clone()
                        }
                    }) == (self::ml_string::Text::from_units(vec![102, 105, 114, 115, 116, 67, 97, 108, 108])))) || ((match state.clone() {
                        self::JavaScriptPrimitiveConversionState::PrimitiveConversion(_, _, _, source_ml_source1_data_property_value1041) => {
                            source_ml_source1_data_property_value1041.clone()
                        }
                    }) == (self::ml_string::Text::from_units(vec![115, 101, 99, 111, 110, 100, 67, 97, 108, 108]))));
                    if (!getter && !call) {
                        self::JavaScriptPrimitiveConversionTransition::PrimitiveInvalid(self::ml_string::Text::from_units(vec![112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 114, 101, 115, 117, 109, 112, 116, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 101, 110, 100, 105, 110, 103, 32, 103, 101, 116, 116, 101, 114, 32, 111, 114, 32, 109, 101, 116, 104, 111, 100]))
                    } else {
                        {
                            let value_error = self::read_java_script_ordinary_value_error(heap.clone(), match completion.clone() {
                                self::JavaScriptPrimitiveConversionCompletion::NormalCompletion(source_ml_source1_data_property_value1042) => {
                                    let source_ml_source1_data_property_value1042 = *source_ml_source1_data_property_value1042;
                                    source_ml_source1_data_property_value1042.clone()
                                }
                                self::JavaScriptPrimitiveConversionCompletion::ThrowCompletion(source_ml_source1_data_property_value1043) => {
                                    let source_ml_source1_data_property_value1043 = *source_ml_source1_data_property_value1043;
                                    source_ml_source1_data_property_value1043.clone()
                                }
                            });
                            if (value_error != (self::ml_string::Text::from_units(vec![]))) {
                                self::JavaScriptPrimitiveConversionTransition::PrimitiveInvalid(value_error.clone())
                            } else {
                                match completion.clone() {
                                    self::JavaScriptPrimitiveConversionCompletion::NormalCompletion(value) => {
                                        let value = *value;
                                        if getter {
                                            self::apply_java_script_primitive_conversion_method(state.clone(), value.clone())
                                        } else {
                                            if match value.clone() {
                                                self::JavaScriptSemanticValue::Undefined => {
                                                    true
                                                }
                                                self::JavaScriptSemanticValue::Null => {
                                                    true
                                                }
                                                self::JavaScriptSemanticValue::Boolean(source_ml_source1_f1044) => {
                                                    true
                                                }
                                                self::JavaScriptSemanticValue::Number(source_ml_source1_f1045) => {
                                                    true
                                                }
                                                self::JavaScriptSemanticValue::Integer(source_ml_source1_f1046) => {
                                                    true
                                                }
                                                self::JavaScriptSemanticValue::Text(source_ml_source1_f1047) => {
                                                    true
                                                }
                                                self::JavaScriptSemanticValue::Symbol(source_ml_source1_f1048) => {
                                                    true
                                                }
                                                self::JavaScriptSemanticValue::Object(source_ml_source1_f1049, source_ml_source1_f1050) => {
                                                    false
                                                }
                                            } {
                                                self::JavaScriptPrimitiveConversionTransition::PrimitiveCompleted(Box::new(value.clone()))
                                            } else {
                                                if ((match state.clone() {
                                                    self::JavaScriptPrimitiveConversionState::PrimitiveConversion(_, _, _, source_ml_source1_data_property_value1052) => {
                                                        source_ml_source1_data_property_value1052.clone()
                                                    }
                                                }) == (self::ml_string::Text::from_units(vec![102, 105, 114, 115, 116, 67, 97, 108, 108]))) {
                                                    self::continue_java_script_primitive_conversion(state.clone(), &(self::ml_string::Text::from_units(vec![115, 101, 99, 111, 110, 100, 71, 101, 116])))
                                                } else {
                                                    self::JavaScriptPrimitiveConversionTransition::PrimitiveIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), if ((match state.clone() {
                                                        self::JavaScriptPrimitiveConversionState::PrimitiveConversion(_, _, _, source_ml_source1_data_property_value1054) => {
                                                            source_ml_source1_data_property_value1054.clone()
                                                        }
                                                    }) == (self::ml_string::Text::from_units(vec![101, 120, 111, 116, 105, 99, 67, 97, 108, 108]))) {
                                                        self::ml_string::Text::from_units(vec![97, 110, 32, 101, 120, 111, 116, 105, 99, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 109, 117, 115, 116, 32, 114, 101, 116, 117, 114, 110, 32, 97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101])
                                                    } else {
                                                        self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 101, 120, 104, 97, 117, 115, 116, 101, 100, 32, 105, 116, 115, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 45, 112, 114, 111, 100, 117, 99, 105, 110, 103, 32, 109, 101, 116, 104, 111, 100, 115])
                                                    })
                                                }
                                            }
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
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_primitive_property_key_spelling(value: self::JavaScriptSemanticValue) -> self::ml_string::Text {
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
            {
                let source_ml_source1_string_argument1059 = value_value_2;
                self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source1_string_argument1059))
            }
        }
        self::JavaScriptSemanticValue::Integer(value_value_3) => {
            {
                let source_ml_source1_string_argument1060 = value_value_3.clone();
                self::ml_string::Text::from_text(&source_ml_source1_string_argument1060.to_string())
            }
        }
        self::JavaScriptSemanticValue::Text(source_ml_source1_f1055) => {
            (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![115, 101, 109, 97, 110, 116, 105, 99, 32, 116, 101, 120, 116, 32, 107, 101, 121, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 101, 120, 97, 99, 116, 32, 117, 110, 105, 116, 115])).0, message_present: true }).raise()
        }
        self::JavaScriptSemanticValue::Symbol(source_ml_source1_f1056) => {
            (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![115, 101, 109, 97, 110, 116, 105, 99, 32, 83, 121, 109, 98, 111, 108, 32, 107, 101, 121, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 101, 120, 97, 99, 116, 32, 105, 100, 101, 110, 116, 105, 116, 121])).0, message_present: true }).raise()
        }
        self::JavaScriptSemanticValue::Object(source_ml_source1_f1057, source_ml_source1_f1058) => {
            (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![97, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 107, 101, 121, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 99, 111, 109, 112, 108, 101, 116, 101, 100, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110])).0, message_present: true }).raise()
        }
    }
}
