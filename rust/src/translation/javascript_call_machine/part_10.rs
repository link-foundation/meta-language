// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptPrimitiveConversionCompletion;
use super::JavaScriptPrimitiveConversionState;
use super::JavaScriptPrimitiveConversionTransition;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryAcceptStringNumberInfinityLoop54Result;
use super::MlSource1TemporaryApplyBigIntegerBitwiseOperationJoin31;
use super::MlSource1TemporaryApplyBigIntegerBitwiseOperationLoop34Result;
use super::MlSource1TemporaryConvertStringNumberRationalJoin46;
use super::MlSource1TemporaryConvertStringNumberRationalJoin47;
use super::MlSource1TemporaryConvertStringNumberRationalLoop48Result;
use super::MlSource1TemporaryConvertStringNumberRationalLoop49Result;
use super::MlSource1TemporaryConvertStringNumberRationalLoop52Result;
use super::MlSource1TemporaryConvertStringNumberRationalLoop53Result;
use super::MlSource1TemporaryReadStringNumberBinaryPowerLoop44Result;
use super::continue_java_script_primitive_conversion;
use super::ml;
use super::ml_source_exception;
use super::ml_string;
use super::plan_java_script_ordinary_get;
use super::read_java_script_ordinary_heap_error;
use super::read_java_script_ordinary_value_error;
use super::read_java_script_primitive_conversion_spelling;
use super::read_java_script_primitive_conversion_state_error;
use super::source_ml_source1_temporary_accept_string_number_infinity_loop54;
use super::source_ml_source1_temporary_apply_big_integer_bitwise_operation_loop32;
use super::source_ml_source1_temporary_apply_big_integer_bitwise_operation_loop33;
use super::source_ml_source1_temporary_apply_big_integer_bitwise_operation_loop34;
use super::source_ml_source1_temporary_apply_java_script_number_bitwise_operation_loop36;
use super::source_ml_source1_temporary_apply_java_script_number_bitwise_operation_loop37;
use super::source_ml_source1_temporary_convert_string_number_rational_loop48;
use super::source_ml_source1_temporary_convert_string_number_rational_loop49;
use super::source_ml_source1_temporary_convert_string_number_rational_loop50;
use super::source_ml_source1_temporary_convert_string_number_rational_loop51;
use super::source_ml_source1_temporary_convert_string_number_rational_loop52;
use super::source_ml_source1_temporary_convert_string_number_rational_loop53;
use super::source_ml_source1_temporary_read_string_number_binary_power_loop44;
use super::source_ml_source1_temporary_read_string_number_binary_power_loop45;
use super::source_ml_source1_temporary_read_string_number_decimal_power_loop42;
use super::source_ml_source1_temporary_read_string_number_exact_mantissa_loop43;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn apply_java_script_primitive_conversion_method(state: self::JavaScriptPrimitiveConversionState, method: self::JavaScriptSemanticValue) -> self::JavaScriptPrimitiveConversionTransition {
    match state.clone() {
        self::JavaScriptPrimitiveConversionState::PrimitiveConversion(shared_constructor_field1, shared_constructor_field2, shared_constructor_field3, shared_constructor_field4) => {
            let shared_constructor_field1 = *shared_constructor_field1;
            let shared_constructor_field2 = *shared_constructor_field2;
            {
                let exotic = ((shared_constructor_field4 == (self::ml_string::Text::from_units(vec![101, 120, 111, 116, 105, 99, 71, 101, 116]))) || (shared_constructor_field4 == (self::ml_string::Text::from_units(vec![101, 120, 111, 116, 105, 99, 71, 101, 116, 116, 101, 114]))));
                {
                    let first = ((shared_constructor_field4 == (self::ml_string::Text::from_units(vec![102, 105, 114, 115, 116, 71, 101, 116]))) || (shared_constructor_field4 == (self::ml_string::Text::from_units(vec![102, 105, 114, 115, 116, 71, 101, 116, 116, 101, 114]))));
                    if (exotic && (match method.clone() {
                        self::JavaScriptSemanticValue::Undefined => {
                            true
                        }
                        _ => {
                            false
                        }
                    } || match method.clone() {
                        self::JavaScriptSemanticValue::Null => {
                            true
                        }
                        _ => {
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
                            self::JavaScriptSemanticValue::Boolean(_) => {
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
                            self::JavaScriptSemanticValue::Number(_) => {
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
                            self::JavaScriptSemanticValue::Integer(_) => {
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
                            self::JavaScriptSemanticValue::Text(_) => {
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
                            self::JavaScriptSemanticValue::Symbol(_) => {
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
                            self::JavaScriptSemanticValue::Object(_, callable) => {
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
                                                vec![self::read_java_script_primitive_conversion_spelling(&(shared_constructor_field3))]
                                            } else {
                                                Vec::<self::JavaScriptSemanticValue>::new()
                                            };
                                            self::JavaScriptPrimitiveConversionTransition::PrimitiveCall(Box::new(self::JavaScriptPrimitiveConversionState::PrimitiveConversion(Box::new(shared_constructor_field1.clone()), Box::new(shared_constructor_field2.clone()), shared_constructor_field3.clone(), phase.clone())), Box::new(method.clone()), Box::new(shared_constructor_field1.clone()), arguments_)
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
pub fn step_java_script_primitive_conversion(heap: self::JavaScriptOrdinaryHeap, state: self::JavaScriptPrimitiveConversionState) -> self::JavaScriptPrimitiveConversionTransition {
    match state.clone() {
        self::JavaScriptPrimitiveConversionState::PrimitiveConversion(shared_constructor_field1, shared_constructor_field2, shared_constructor_field3, shared_constructor_field4) => {
            let shared_constructor_field1 = *shared_constructor_field1;
            let shared_constructor_field2 = *shared_constructor_field2;
            {
                let error = self::read_java_script_primitive_conversion_state_error(heap.clone(), state.clone());
                if (error != (self::ml_string::Text::from_units(vec![]))) {
                    self::JavaScriptPrimitiveConversionTransition::PrimitiveInvalid(error.clone())
                } else {
                    if (((shared_constructor_field4 != (self::ml_string::Text::from_units(vec![101, 120, 111, 116, 105, 99, 71, 101, 116]))) && (shared_constructor_field4 != (self::ml_string::Text::from_units(vec![102, 105, 114, 115, 116, 71, 101, 116])))) && (shared_constructor_field4 != (self::ml_string::Text::from_units(vec![115, 101, 99, 111, 110, 100, 71, 101, 116])))) {
                        self::JavaScriptPrimitiveConversionTransition::PrimitiveInvalid(self::ml_string::Text::from_units(vec![97, 32, 112, 101, 110, 100, 105, 110, 103, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 99, 97, 108, 108, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 109, 112, 108, 101, 116, 105, 111, 110]))
                    } else {
                        {
                            let receiver = shared_constructor_field1.clone();
                            match receiver.clone() {
                                self::JavaScriptSemanticValue::Undefined => {
                                    self::JavaScriptPrimitiveConversionTransition::PrimitiveInvalid(self::ml_string::Text::from_units(vec![97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 114, 101, 99, 101, 105, 118, 101, 114]))
                                }
                                self::JavaScriptSemanticValue::Null => {
                                    self::JavaScriptPrimitiveConversionTransition::PrimitiveInvalid(self::ml_string::Text::from_units(vec![97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 114, 101, 99, 101, 105, 118, 101, 114]))
                                }
                                self::JavaScriptSemanticValue::Boolean(_) => {
                                    self::JavaScriptPrimitiveConversionTransition::PrimitiveInvalid(self::ml_string::Text::from_units(vec![97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 114, 101, 99, 101, 105, 118, 101, 114]))
                                }
                                self::JavaScriptSemanticValue::Number(_) => {
                                    self::JavaScriptPrimitiveConversionTransition::PrimitiveInvalid(self::ml_string::Text::from_units(vec![97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 114, 101, 99, 101, 105, 118, 101, 114]))
                                }
                                self::JavaScriptSemanticValue::Integer(_) => {
                                    self::JavaScriptPrimitiveConversionTransition::PrimitiveInvalid(self::ml_string::Text::from_units(vec![97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 114, 101, 99, 101, 105, 118, 101, 114]))
                                }
                                self::JavaScriptSemanticValue::Text(_) => {
                                    self::JavaScriptPrimitiveConversionTransition::PrimitiveInvalid(self::ml_string::Text::from_units(vec![97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 114, 101, 99, 101, 105, 118, 101, 114]))
                                }
                                self::JavaScriptSemanticValue::Symbol(_) => {
                                    self::JavaScriptPrimitiveConversionTransition::PrimitiveInvalid(self::ml_string::Text::from_units(vec![97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 114, 101, 99, 101, 105, 118, 101, 114]))
                                }
                                self::JavaScriptSemanticValue::Object(identity, _) => {
                                    {
                                        let first = (shared_constructor_field4 == (self::ml_string::Text::from_units(vec![102, 105, 114, 115, 116, 71, 101, 116])));
                                        {
                                            let method = if (shared_constructor_field3 == (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103]))) {
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
                                                let key = if (shared_constructor_field4 == (self::ml_string::Text::from_units(vec![101, 120, 111, 116, 105, 99, 71, 101, 116]))) {
                                                    shared_constructor_field2.clone()
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
                                                                let phase = if (shared_constructor_field4 == (self::ml_string::Text::from_units(vec![101, 120, 111, 116, 105, 99, 71, 101, 116]))) {
                                                                    self::ml_string::Text::from_units(vec![101, 120, 111, 116, 105, 99, 71, 101, 116, 116, 101, 114])
                                                                } else {
                                                                    if first {
                                                                        self::ml_string::Text::from_units(vec![102, 105, 114, 115, 116, 71, 101, 116, 116, 101, 114])
                                                                    } else {
                                                                        self::ml_string::Text::from_units(vec![115, 101, 99, 111, 110, 100, 71, 101, 116, 116, 101, 114])
                                                                    }
                                                                };
                                                                self::JavaScriptPrimitiveConversionTransition::PrimitiveCall(Box::new(self::JavaScriptPrimitiveConversionState::PrimitiveConversion(Box::new(receiver.clone()), Box::new(shared_constructor_field2.clone()), shared_constructor_field3.clone(), phase.clone())), Box::new((plan.clone()).2), Box::new((plan.clone()).4), (plan.clone()).1)
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn resume_java_script_primitive_conversion(heap: self::JavaScriptOrdinaryHeap, state: self::JavaScriptPrimitiveConversionState, completion: self::JavaScriptPrimitiveConversionCompletion) -> self::JavaScriptPrimitiveConversionTransition {
    match state.clone() {
        self::JavaScriptPrimitiveConversionState::PrimitiveConversion(_, _, _, shared_constructor_field1) => {
            {
                let error = self::read_java_script_primitive_conversion_state_error(heap.clone(), state.clone());
                if (error != (self::ml_string::Text::from_units(vec![]))) {
                    self::JavaScriptPrimitiveConversionTransition::PrimitiveInvalid(error.clone())
                } else {
                    {
                        let getter = (((shared_constructor_field1 == (self::ml_string::Text::from_units(vec![101, 120, 111, 116, 105, 99, 71, 101, 116, 116, 101, 114]))) || (shared_constructor_field1 == (self::ml_string::Text::from_units(vec![102, 105, 114, 115, 116, 71, 101, 116, 116, 101, 114])))) || (shared_constructor_field1 == (self::ml_string::Text::from_units(vec![115, 101, 99, 111, 110, 100, 71, 101, 116, 116, 101, 114]))));
                        {
                            let call = (((shared_constructor_field1 == (self::ml_string::Text::from_units(vec![101, 120, 111, 116, 105, 99, 67, 97, 108, 108]))) || (shared_constructor_field1 == (self::ml_string::Text::from_units(vec![102, 105, 114, 115, 116, 67, 97, 108, 108])))) || (shared_constructor_field1 == (self::ml_string::Text::from_units(vec![115, 101, 99, 111, 110, 100, 67, 97, 108, 108]))));
                            if (!getter && !call) {
                                self::JavaScriptPrimitiveConversionTransition::PrimitiveInvalid(self::ml_string::Text::from_units(vec![112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 114, 101, 115, 117, 109, 112, 116, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 101, 110, 100, 105, 110, 103, 32, 103, 101, 116, 116, 101, 114, 32, 111, 114, 32, 109, 101, 116, 104, 111, 100]))
                            } else {
                                {
                                    let value_error = self::read_java_script_ordinary_value_error(heap.clone(), match completion.clone() {
                                        self::JavaScriptPrimitiveConversionCompletion::NormalCompletion(source_ml_source2_data_property_value824) => {
                                            let source_ml_source2_data_property_value824 = *source_ml_source2_data_property_value824;
                                            source_ml_source2_data_property_value824.clone()
                                        }
                                        self::JavaScriptPrimitiveConversionCompletion::ThrowCompletion(source_ml_source2_data_property_value825) => {
                                            let source_ml_source2_data_property_value825 = *source_ml_source2_data_property_value825;
                                            source_ml_source2_data_property_value825.clone()
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
                                                        self::JavaScriptSemanticValue::Object(_, _) => {
                                                            false
                                                        }
                                                        _ => {
                                                            true
                                                        }
                                                    } {
                                                        self::JavaScriptPrimitiveConversionTransition::PrimitiveCompleted(Box::new(value.clone()))
                                                    } else {
                                                        if (shared_constructor_field1 == (self::ml_string::Text::from_units(vec![102, 105, 114, 115, 116, 67, 97, 108, 108]))) {
                                                            self::continue_java_script_primitive_conversion(state.clone(), &(self::ml_string::Text::from_units(vec![115, 101, 99, 111, 110, 100, 71, 101, 116])))
                                                        } else {
                                                            self::JavaScriptPrimitiveConversionTransition::PrimitiveIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), if (shared_constructor_field1 == (self::ml_string::Text::from_units(vec![101, 120, 111, 116, 105, 99, 67, 97, 108, 108]))) {
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
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn apply_big_integer_bitwise_operation(operation: f64, left: self::ml::Big, right: self::ml::Big) -> self::ml::Big {
    if (operation == (6f64)) {
        (self::ml::Big::from_i128(-1)).sub(&left)
    } else {
        if ((operation == (3f64)) || (operation == (4f64))) {
            if (left == (self::ml::Big::from_i128(0))) {
                self::ml::Big::from_i128(0)
            } else {
                {
                    let count = right.clone();
                    {
                        let forward = (operation == (3f64));
                        {
                            let source_ml_source2_s838 = if (count < (self::ml::Big::from_i128(0))) {
                                {
                                    let count_2 = count.neg();
                                    {
                                        let forward_2 = !forward;
                                        self::MlSource1TemporaryApplyBigIntegerBitwiseOperationJoin31::MlSource1TemporaryApplyBigIntegerBitwiseOperationJoin31Next(count_2.clone(), forward_2)
                                    }
                                }
                            } else {
                                self::MlSource1TemporaryApplyBigIntegerBitwiseOperationJoin31::MlSource1TemporaryApplyBigIntegerBitwiseOperationJoin31Next(count.clone(), forward)
                            };
                            match source_ml_source2_s838.clone() {
                                self::MlSource1TemporaryApplyBigIntegerBitwiseOperationJoin31::MlSource1TemporaryApplyBigIntegerBitwiseOperationJoin31Next(count_3, forward_3) => {
                                    {
                                        let value = left.clone();
                                        {
                                            let value_4 = if forward_3 {
                                                {
                                                    let factor = self::ml::Big::from_i128(2);
                                                    {
                                                        let value_2 = self::source_ml_source1_temporary_apply_big_integer_bitwise_operation_loop32(count_3.clone(), value.clone(), factor.clone());
                                                        value_2.clone()
                                                    }
                                                }
                                            } else {
                                                {
                                                    let value_3 = self::source_ml_source1_temporary_apply_big_integer_bitwise_operation_loop33(count_3.clone(), value.clone());
                                                    value_3.clone()
                                                }
                                            };
                                            value_4.clone()
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            if (((operation != (0f64)) && (operation != (1f64))) && (operation != (2f64))) {
                (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![97, 110, 32, 117, 110, 107, 110, 111, 119, 110, 32, 66, 105, 103, 73, 110, 116, 32, 98, 105, 116, 119, 105, 115, 101, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110])).0, message_present: true }).raise()
            } else {
                {
                    let negative_left = (left < (self::ml::Big::from_i128(0)));
                    {
                        let negative_right = (right < (self::ml::Big::from_i128(0)));
                        {
                            let first = if negative_left {
                                (self::ml::Big::from_i128(-1)).sub(&left)
                            } else {
                                left.clone()
                            };
                            {
                                let second = if negative_right {
                                    (self::ml::Big::from_i128(-1)).sub(&right)
                                } else {
                                    right.clone()
                                };
                                {
                                    let weight = self::ml::Big::from_i128(1);
                                    {
                                        let result = self::ml::Big::from_i128(0);
                                        {
                                            let source_ml_source2_s839 = self::source_ml_source1_temporary_apply_big_integer_bitwise_operation_loop34(operation, negative_left, negative_right, first.clone(), second.clone(), weight.clone(), result.clone());
                                            match source_ml_source2_s839.clone() {
                                                self::MlSource1TemporaryApplyBigIntegerBitwiseOperationLoop34Result::MlSource1TemporaryApplyBigIntegerBitwiseOperationLoop34Done(weight_2, result_2) => {
                                                    {
                                                        let negative = if (operation == (0f64)) {
                                                            (negative_left && negative_right)
                                                        } else {
                                                            if (operation == (1f64)) {
                                                                (negative_left || negative_right)
                                                            } else {
                                                                (negative_left != negative_right)
                                                            }
                                                        };
                                                        if negative {
                                                            result_2.sub(&weight_2)
                                                        } else {
                                                            result_2.clone()
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
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn dependency_function1(value: f64) -> f64 {
    if (!(value.is_finite()) || (value == (0f64))) {
        0f64
    } else {
        {
            let integral = if (value < (0f64)) {
                (-(((-value)).floor()))
            } else {
                value.floor()
            };
            {
                let remainder = (integral % 4294967296f64);
                if (remainder < (0f64)) {
                    (remainder + 4294967296f64)
                } else {
                    if (remainder == (0f64)) {
                        0f64
                    } else {
                        remainder
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn normalize_java_script_signed_integer(value: f64) -> f64 {
    {
        let unsigned = self::dependency_function1(value);
        if (unsigned >= (2147483648f64)) {
            (unsigned - 4294967296f64)
        } else {
            unsigned
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn apply_java_script_number_bitwise_operation(operation: f64, left: f64, right: f64) -> f64 {
    if (operation == (6f64)) {
        ((-1f64) - self::normalize_java_script_signed_integer(left))
    } else {
        if (((operation == (3f64)) || (operation == (4f64))) || (operation == (5f64))) {
            {
                let count = (self::dependency_function1(right) % 32f64);
                {
                    let factor = 1f64;
                    {
                        let position = 0f64;
                        {
                            let factor_2 = self::source_ml_source1_temporary_apply_java_script_number_bitwise_operation_loop36(count, factor, position);
                            if (operation == (3f64)) {
                                self::normalize_java_script_signed_integer((self::dependency_function1(left) * factor_2))
                            } else {
                                if (operation == (4f64)) {
                                    ((self::normalize_java_script_signed_integer(left) / factor_2)).floor()
                                } else {
                                    ((self::dependency_function1(left) / factor_2)).floor()
                                }
                            }
                        }
                    }
                }
            }
        } else {
            if !((((operation == (0f64)) || (operation == (1f64))) || (operation == (2f64)))) {
                f64::NAN
            } else {
                {
                    let first = self::dependency_function1(left);
                    {
                        let second = self::dependency_function1(right);
                        {
                            let result = 0f64;
                            {
                                let weight = 1f64;
                                {
                                    let position_2 = 0f64;
                                    {
                                        let result_2 = self::source_ml_source1_temporary_apply_java_script_number_bitwise_operation_loop37(operation, first, second, result, weight, position_2);
                                        self::normalize_java_script_signed_integer(result_2)
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
pub fn accept_java_script_whitespace_unit(unit: f64) -> bool {
    ((((((((((((unit >= (9f64)) && (unit <= (13f64))) || (unit == (32f64))) || (unit == (160f64))) || (unit == (5760f64))) || ((unit >= (8192f64)) && (unit <= (8202f64)))) || (unit == (8232f64))) || (unit == (8233f64))) || (unit == (8239f64))) || (unit == (8287f64))) || (unit == (12288f64))) || (unit == (65279f64)))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_numeric_digit_unit(unit: f64) -> f64 {
    if ((unit >= (48f64)) && (unit <= (57f64))) {
        (unit - 48f64)
    } else {
        if ((unit >= (65f64)) && (unit <= (70f64))) {
            (unit - 55f64)
        } else {
            if ((unit >= (97f64)) && (unit <= (102f64))) {
                (unit - 87f64)
            } else {
                (-1f64)
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_string_number_digit(unit: f64) -> f64 {
    self::read_java_script_numeric_digit_unit(unit)
}

pub fn read_string_number_decimal_power(exponent: f64) -> self::ml::Big {
    {
        let value = self::ml::Big::from_i128(1);
        {
            let base = self::ml::Big::from_i128(10);
            {
                let value_2 = self::source_ml_source1_temporary_read_string_number_decimal_power_loop42(exponent, value.clone(), base.clone());
                value_2.clone()
            }
        }
    }
}

pub fn read_string_number_exact_mantissa(value: self::ml::Big) -> f64 {
    {
        let result = 0f64;
        {
            let scale = 1f64;
            {
                let result_2 = self::source_ml_source1_temporary_read_string_number_exact_mantissa_loop43(value.clone(), result, scale);
                result_2
            }
        }
    }
}

pub fn read_string_number_binary_power(exponent: f64) -> f64 {
    {
        let result = 1f64;
        {
            let source_ml_source2_s842 = self::source_ml_source1_temporary_read_string_number_binary_power_loop44(exponent, result);
            match source_ml_source2_s842.clone() {
                self::MlSource1TemporaryReadStringNumberBinaryPowerLoop44Result::MlSource1TemporaryReadStringNumberBinaryPowerLoop44Done(exponent_2, result_2) => {
                    {
                        let result_3 = self::source_ml_source1_temporary_read_string_number_binary_power_loop45(exponent_2, result_2);
                        result_3
                    }
                }
            }
        }
    }
}

pub fn convert_string_number_rational(numerator: self::ml::Big, denominator: self::ml::Big) -> f64 {
    if ((numerator < (self::ml::Big::from_i128(0))) || (denominator <= (self::ml::Big::from_i128(0)))) {
        f64::NAN
    } else {
        if (numerator == (self::ml::Big::from_i128(0))) {
            0f64
        } else {
            {
                let exponent = 0f64;
                {
                    let source_ml_source2_s845 = if (numerator >= denominator) {
                        {
                            let source_ml_source2_s846 = self::source_ml_source1_temporary_convert_string_number_rational_loop52(numerator.clone(), exponent, denominator.clone());
                            match source_ml_source2_s846.clone() {
                                self::MlSource1TemporaryConvertStringNumberRationalLoop52Result::MlSource1TemporaryConvertStringNumberRationalLoop52Done(exponent_2) => {
                                    self::MlSource1TemporaryConvertStringNumberRationalJoin46::MlSource1TemporaryConvertStringNumberRationalJoin46Next(exponent_2)
                                }
                                self::MlSource1TemporaryConvertStringNumberRationalLoop52Result::MlSource1TemporaryConvertStringNumberRationalLoop52Return(source_ml_source1_temporary_result52) => {
                                    self::MlSource1TemporaryConvertStringNumberRationalJoin46::MlSource1TemporaryConvertStringNumberRationalJoin46Return(source_ml_source1_temporary_result52)
                                }
                            }
                        }
                    } else {
                        {
                            let source_ml_source2_s847 = self::source_ml_source1_temporary_convert_string_number_rational_loop53(denominator.clone(), exponent, numerator.clone());
                            match source_ml_source2_s847.clone() {
                                self::MlSource1TemporaryConvertStringNumberRationalLoop53Result::MlSource1TemporaryConvertStringNumberRationalLoop53Done(exponent_3) => {
                                    self::MlSource1TemporaryConvertStringNumberRationalJoin46::MlSource1TemporaryConvertStringNumberRationalJoin46Next(exponent_3)
                                }
                                self::MlSource1TemporaryConvertStringNumberRationalLoop53Result::MlSource1TemporaryConvertStringNumberRationalLoop53Return(source_ml_source1_temporary_result53) => {
                                    self::MlSource1TemporaryConvertStringNumberRationalJoin46::MlSource1TemporaryConvertStringNumberRationalJoin46Return(source_ml_source1_temporary_result53)
                                }
                            }
                        }
                    };
                    match source_ml_source2_s845.clone() {
                        self::MlSource1TemporaryConvertStringNumberRationalJoin46::MlSource1TemporaryConvertStringNumberRationalJoin46Next(exponent_4) => {
                            {
                                let shift = if (exponent_4 >= ((-1022f64))) {
                                    (52f64 - exponent_4)
                                } else {
                                    1074f64
                                };
                                {
                                    let source_ml_source2_s848 = if (shift >= (0f64)) {
                                        {
                                            let index = 0f64;
                                            {
                                                let numerator_2 = self::source_ml_source1_temporary_convert_string_number_rational_loop50(numerator.clone(), shift, index);
                                                self::MlSource1TemporaryConvertStringNumberRationalJoin47::MlSource1TemporaryConvertStringNumberRationalJoin47Next(numerator_2.clone(), denominator.clone())
                                            }
                                        }
                                    } else {
                                        {
                                            let index_2 = 0f64;
                                            {
                                                let denominator_2 = self::source_ml_source1_temporary_convert_string_number_rational_loop51(denominator.clone(), shift, index_2);
                                                self::MlSource1TemporaryConvertStringNumberRationalJoin47::MlSource1TemporaryConvertStringNumberRationalJoin47Next(numerator.clone(), denominator_2.clone())
                                            }
                                        }
                                    };
                                    match source_ml_source2_s848.clone() {
                                        self::MlSource1TemporaryConvertStringNumberRationalJoin47::MlSource1TemporaryConvertStringNumberRationalJoin47Next(numerator_3, denominator_3) => {
                                            {
                                                let quotient = self::ml::Big::from_i128(0);
                                                {
                                                    let remainder = numerator_3.clone();
                                                    {
                                                        let scale = denominator_3.clone();
                                                        {
                                                            let weight = self::ml::Big::from_i128(1);
                                                            {
                                                                let source_ml_source2_s849 = self::source_ml_source1_temporary_convert_string_number_rational_loop48(remainder.clone(), scale.clone(), weight.clone());
                                                                match source_ml_source2_s849.clone() {
                                                                    self::MlSource1TemporaryConvertStringNumberRationalLoop48Result::MlSource1TemporaryConvertStringNumberRationalLoop48Done(scale_2, weight_2) => {
                                                                        {
                                                                            let source_ml_source2_s850 = self::source_ml_source1_temporary_convert_string_number_rational_loop49(quotient.clone(), remainder.clone(), scale_2.clone(), weight_2.clone());
                                                                            match source_ml_source2_s850.clone() {
                                                                                self::MlSource1TemporaryConvertStringNumberRationalLoop49Result::MlSource1TemporaryConvertStringNumberRationalLoop49Done(quotient_2, remainder_2) => {
                                                                                    {
                                                                                        let twice = remainder_2.mul(&self::ml::Big::from_i128(2));
                                                                                        {
                                                                                            let quotient_4 = if ((twice > denominator_3) || ((twice == denominator_3) && ((self::ml::divide(&quotient_2, &self::ml::Big::from_i128(2), "trunc", Some("Division by zero"), true)) == (self::ml::Big::from_i128(1))))) {
                                                                                                {
                                                                                                    let quotient_3 = quotient_2.add(&self::ml::Big::from_i128(1));
                                                                                                    quotient_3.clone()
                                                                                                }
                                                                                            } else {
                                                                                                quotient_2.clone()
                                                                                            };
                                                                                            (self::read_string_number_exact_mantissa(quotient_4.clone()) * self::read_string_number_binary_power((-shift)))
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
                                }
                            }
                        }
                        self::MlSource1TemporaryConvertStringNumberRationalJoin46::MlSource1TemporaryConvertStringNumberRationalJoin46Return(source_ml_source1_temporary_result51) => {
                            source_ml_source1_temporary_result51
                        }
                    }
                }
            }
        }
    }
}

pub fn accept_string_number_infinity(units: Vec<f64>, start: f64, end: f64) -> bool {
    if (((end - start)) != (8f64)) {
        false
    } else {
        {
            let expected = vec![73f64, 110f64, 102f64, 105f64, 110f64, 105f64, 116f64, 121f64];
            {
                let index = 0f64;
                {
                    let source_ml_source2_s851 = self::source_ml_source1_temporary_accept_string_number_infinity_loop54(units, start, expected, index);
                    match source_ml_source2_s851.clone() {
                        self::MlSource1TemporaryAcceptStringNumberInfinityLoop54Result::MlSource1TemporaryAcceptStringNumberInfinityLoop54Done => {
                            true
                        }
                        self::MlSource1TemporaryAcceptStringNumberInfinityLoop54Result::MlSource1TemporaryAcceptStringNumberInfinityLoop54Return(source_ml_source1_temporary_result54) => {
                            source_ml_source1_temporary_result54
                        }
                    }
                }
            }
        }
    }
}
