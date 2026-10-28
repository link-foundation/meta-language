// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptCallFrame;
use super::JavaScriptCallFunction;
use super::JavaScriptCallHandler;
use super::JavaScriptCallInstruction;
use super::JavaScriptCallMachine;
use super::JavaScriptCallPending;
use super::JavaScriptCallProgram;
use super::JavaScriptDescriptorConversionTransition;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptPrimitiveConversionCompletion;
use super::JavaScriptPrimitiveConversionState;
use super::JavaScriptPrimitiveConversionTransition;
use super::JavaScriptPropertyDefinitionState;
use super::JavaScriptPropertyDefinitionTransition;
use super::JavaScriptPropertyKeyConversionTransition;
use super::JavaScriptSemanticCompletion;
use super::JavaScriptSemanticValue;
use super::MlTemporaryReadJavaScriptCallMachineErrorLoop49Result;
use super::MlTemporaryReadJavaScriptCallProgramErrorLoop38Result;
use super::MlTemporaryReadJavaScriptCallProgramErrorLoop39Result;
use super::begin_java_script_descriptor_conversion;
use super::begin_java_script_primitive_conversion;
use super::define_java_script_ordinary_property;
use super::find_java_script_call_code_function;
use super::ml;
use super::ml_string;
use super::read_java_script_call_code_registers;
use super::read_java_script_ordinary_heap_error;
use super::read_java_script_ordinary_value_error;
use super::read_java_script_primitive_property_key_spelling;
use super::resume_java_script_descriptor_conversion;
use super::resume_java_script_primitive_conversion;
use super::source_ml_temporary_create_java_script_call_frame_loop43;
use super::source_ml_temporary_create_java_script_call_frame_loop44;
use super::source_ml_temporary_create_java_script_call_frame_loop45;
use super::source_ml_temporary_finish_java_script_property_key_conversion_loop32;
use super::source_ml_temporary_read_java_script_call_machine_error_loop49;
use super::source_ml_temporary_read_java_script_call_program_error_loop38;
use super::source_ml_temporary_read_java_script_call_program_error_loop39;
use super::step_java_script_descriptor_conversion;
use super::step_java_script_primitive_conversion;
use super::test_java_script_call_code_coordinate;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn finish_java_script_property_key_conversion(transition: self::JavaScriptPrimitiveConversionTransition) -> self::JavaScriptPropertyKeyConversionTransition {
    match transition.clone() {
        self::JavaScriptPrimitiveConversionTransition::PrimitiveContinuation(state) => {
            let state = *state;
            self::JavaScriptPropertyKeyConversionTransition::PropertyKeyContinuation(Box::new(state.clone()))
        }
        self::JavaScriptPrimitiveConversionTransition::PrimitiveCall(state_2, callee, receiver, arguments) => {
            let state_2 = *state_2;
            let callee = *callee;
            let receiver = *receiver;
            self::JavaScriptPropertyKeyConversionTransition::PropertyKeyCall(Box::new(state_2.clone()), Box::new(callee.clone()), Box::new(receiver.clone()), arguments)
        }
        self::JavaScriptPrimitiveConversionTransition::PrimitiveCompleted(value) => {
            let value = *value;
            if (match value.clone() {
                self::JavaScriptSemanticValue::Undefined => {
                    false
                }
                self::JavaScriptSemanticValue::Null => {
                    false
                }
                self::JavaScriptSemanticValue::Boolean(source_ml_source1_f1061) => {
                    false
                }
                self::JavaScriptSemanticValue::Number(source_ml_source1_f1062) => {
                    false
                }
                self::JavaScriptSemanticValue::Integer(source_ml_source1_f1063) => {
                    false
                }
                self::JavaScriptSemanticValue::Text(source_ml_source1_f1064) => {
                    true
                }
                self::JavaScriptSemanticValue::Symbol(source_ml_source1_f1065) => {
                    false
                }
                self::JavaScriptSemanticValue::Object(source_ml_source1_f1066, source_ml_source1_f1067) => {
                    false
                }
            } || match value.clone() {
                self::JavaScriptSemanticValue::Undefined => {
                    false
                }
                self::JavaScriptSemanticValue::Null => {
                    false
                }
                self::JavaScriptSemanticValue::Boolean(source_ml_source1_f1068) => {
                    false
                }
                self::JavaScriptSemanticValue::Number(source_ml_source1_f1069) => {
                    false
                }
                self::JavaScriptSemanticValue::Integer(source_ml_source1_f1070) => {
                    false
                }
                self::JavaScriptSemanticValue::Text(source_ml_source1_f1071) => {
                    false
                }
                self::JavaScriptSemanticValue::Symbol(source_ml_source1_f1072) => {
                    true
                }
                self::JavaScriptSemanticValue::Object(source_ml_source1_f1073, source_ml_source1_f1074) => {
                    false
                }
            }) {
                self::JavaScriptPropertyKeyConversionTransition::PropertyKeyCompleted(Box::new(value.clone()))
            } else {
                match value.clone() {
                    self::JavaScriptSemanticValue::Undefined => {
                        {
                            let spelling = self::read_java_script_primitive_property_key_spelling(value.clone());
                            if ((spelling.length()) > (1048576f64)) {
                                self::JavaScriptPropertyKeyConversionTransition::PropertyKeyInvalid(self::ml_string::Text::from_units(vec![112, 114, 111, 112, 101, 114, 116, 121, 32, 107, 101, 121, 32, 116, 101, 120, 116, 32, 101, 120, 99, 101, 101, 100, 115, 32, 116, 104, 101, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 85, 84, 70, 49, 54, 32, 117, 110, 105, 116, 32, 98, 117, 100, 103, 101, 116]))
                            } else {
                                {
                                    let units = Vec::<f64>::new();
                                    {
                                        let index = 0f64;
                                        {
                                            let units_2 = self::source_ml_temporary_finish_java_script_property_key_conversion_loop32(&(spelling), units, index);
                                            self::JavaScriptPropertyKeyConversionTransition::PropertyKeyCompleted(Box::new(self::JavaScriptSemanticValue::Text(units_2)))
                                        }
                                    }
                                }
                            }
                        }
                    }
                    self::JavaScriptSemanticValue::Null => {
                        {
                            let spelling_2 = self::read_java_script_primitive_property_key_spelling(value.clone());
                            if ((spelling_2.length()) > (1048576f64)) {
                                self::JavaScriptPropertyKeyConversionTransition::PropertyKeyInvalid(self::ml_string::Text::from_units(vec![112, 114, 111, 112, 101, 114, 116, 121, 32, 107, 101, 121, 32, 116, 101, 120, 116, 32, 101, 120, 99, 101, 101, 100, 115, 32, 116, 104, 101, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 85, 84, 70, 49, 54, 32, 117, 110, 105, 116, 32, 98, 117, 100, 103, 101, 116]))
                            } else {
                                {
                                    let units_3 = Vec::<f64>::new();
                                    {
                                        let index_2 = 0f64;
                                        {
                                            let units_4 = self::source_ml_temporary_finish_java_script_property_key_conversion_loop32(&(spelling_2), units_3, index_2);
                                            self::JavaScriptPropertyKeyConversionTransition::PropertyKeyCompleted(Box::new(self::JavaScriptSemanticValue::Text(units_4)))
                                        }
                                    }
                                }
                            }
                        }
                    }
                    self::JavaScriptSemanticValue::Boolean(source_ml_source1_f1075) => {
                        {
                            let spelling_3 = self::read_java_script_primitive_property_key_spelling(value.clone());
                            if ((spelling_3.length()) > (1048576f64)) {
                                self::JavaScriptPropertyKeyConversionTransition::PropertyKeyInvalid(self::ml_string::Text::from_units(vec![112, 114, 111, 112, 101, 114, 116, 121, 32, 107, 101, 121, 32, 116, 101, 120, 116, 32, 101, 120, 99, 101, 101, 100, 115, 32, 116, 104, 101, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 85, 84, 70, 49, 54, 32, 117, 110, 105, 116, 32, 98, 117, 100, 103, 101, 116]))
                            } else {
                                {
                                    let units_5 = Vec::<f64>::new();
                                    {
                                        let index_3 = 0f64;
                                        {
                                            let units_6 = self::source_ml_temporary_finish_java_script_property_key_conversion_loop32(&(spelling_3), units_5, index_3);
                                            self::JavaScriptPropertyKeyConversionTransition::PropertyKeyCompleted(Box::new(self::JavaScriptSemanticValue::Text(units_6)))
                                        }
                                    }
                                }
                            }
                        }
                    }
                    self::JavaScriptSemanticValue::Number(source_ml_source1_f1076) => {
                        {
                            let spelling_4 = self::read_java_script_primitive_property_key_spelling(value.clone());
                            if ((spelling_4.length()) > (1048576f64)) {
                                self::JavaScriptPropertyKeyConversionTransition::PropertyKeyInvalid(self::ml_string::Text::from_units(vec![112, 114, 111, 112, 101, 114, 116, 121, 32, 107, 101, 121, 32, 116, 101, 120, 116, 32, 101, 120, 99, 101, 101, 100, 115, 32, 116, 104, 101, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 85, 84, 70, 49, 54, 32, 117, 110, 105, 116, 32, 98, 117, 100, 103, 101, 116]))
                            } else {
                                {
                                    let units_7 = Vec::<f64>::new();
                                    {
                                        let index_4 = 0f64;
                                        {
                                            let units_8 = self::source_ml_temporary_finish_java_script_property_key_conversion_loop32(&(spelling_4), units_7, index_4);
                                            self::JavaScriptPropertyKeyConversionTransition::PropertyKeyCompleted(Box::new(self::JavaScriptSemanticValue::Text(units_8)))
                                        }
                                    }
                                }
                            }
                        }
                    }
                    self::JavaScriptSemanticValue::Integer(source_ml_source1_f1077) => {
                        {
                            let spelling_5 = self::read_java_script_primitive_property_key_spelling(value.clone());
                            if ((spelling_5.length()) > (1048576f64)) {
                                self::JavaScriptPropertyKeyConversionTransition::PropertyKeyInvalid(self::ml_string::Text::from_units(vec![112, 114, 111, 112, 101, 114, 116, 121, 32, 107, 101, 121, 32, 116, 101, 120, 116, 32, 101, 120, 99, 101, 101, 100, 115, 32, 116, 104, 101, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 85, 84, 70, 49, 54, 32, 117, 110, 105, 116, 32, 98, 117, 100, 103, 101, 116]))
                            } else {
                                {
                                    let units_9 = Vec::<f64>::new();
                                    {
                                        let index_5 = 0f64;
                                        {
                                            let units_10 = self::source_ml_temporary_finish_java_script_property_key_conversion_loop32(&(spelling_5), units_9, index_5);
                                            self::JavaScriptPropertyKeyConversionTransition::PropertyKeyCompleted(Box::new(self::JavaScriptSemanticValue::Text(units_10)))
                                        }
                                    }
                                }
                            }
                        }
                    }
                    self::JavaScriptSemanticValue::Text(source_ml_source1_f1078) => {
                        {
                            let spelling_6 = self::read_java_script_primitive_property_key_spelling(value.clone());
                            if ((spelling_6.length()) > (1048576f64)) {
                                self::JavaScriptPropertyKeyConversionTransition::PropertyKeyInvalid(self::ml_string::Text::from_units(vec![112, 114, 111, 112, 101, 114, 116, 121, 32, 107, 101, 121, 32, 116, 101, 120, 116, 32, 101, 120, 99, 101, 101, 100, 115, 32, 116, 104, 101, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 85, 84, 70, 49, 54, 32, 117, 110, 105, 116, 32, 98, 117, 100, 103, 101, 116]))
                            } else {
                                {
                                    let units_11 = Vec::<f64>::new();
                                    {
                                        let index_6 = 0f64;
                                        {
                                            let units_12 = self::source_ml_temporary_finish_java_script_property_key_conversion_loop32(&(spelling_6), units_11, index_6);
                                            self::JavaScriptPropertyKeyConversionTransition::PropertyKeyCompleted(Box::new(self::JavaScriptSemanticValue::Text(units_12)))
                                        }
                                    }
                                }
                            }
                        }
                    }
                    self::JavaScriptSemanticValue::Symbol(source_ml_source1_f1079) => {
                        {
                            let spelling_7 = self::read_java_script_primitive_property_key_spelling(value.clone());
                            if ((spelling_7.length()) > (1048576f64)) {
                                self::JavaScriptPropertyKeyConversionTransition::PropertyKeyInvalid(self::ml_string::Text::from_units(vec![112, 114, 111, 112, 101, 114, 116, 121, 32, 107, 101, 121, 32, 116, 101, 120, 116, 32, 101, 120, 99, 101, 101, 100, 115, 32, 116, 104, 101, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 85, 84, 70, 49, 54, 32, 117, 110, 105, 116, 32, 98, 117, 100, 103, 101, 116]))
                            } else {
                                {
                                    let units_13 = Vec::<f64>::new();
                                    {
                                        let index_7 = 0f64;
                                        {
                                            let units_14 = self::source_ml_temporary_finish_java_script_property_key_conversion_loop32(&(spelling_7), units_13, index_7);
                                            self::JavaScriptPropertyKeyConversionTransition::PropertyKeyCompleted(Box::new(self::JavaScriptSemanticValue::Text(units_14)))
                                        }
                                    }
                                }
                            }
                        }
                    }
                    self::JavaScriptSemanticValue::Object(source_ml_source1_f1080, source_ml_source1_f1081) => {
                        self::JavaScriptPropertyKeyConversionTransition::PropertyKeyInvalid(self::ml_string::Text::from_units(vec![97, 32, 99, 111, 109, 112, 108, 101, 116, 101, 100, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 99, 97, 110, 110, 111, 116, 32, 99, 111, 110, 116, 97, 105, 110, 32, 97, 110, 32, 79, 98, 106, 101, 99, 116]))
                    }
                }
            }
        }
        self::JavaScriptPrimitiveConversionTransition::PrimitiveThrow(value_2) => {
            let value_2 = *value_2;
            self::JavaScriptPropertyKeyConversionTransition::PropertyKeyThrow(Box::new(value_2.clone()))
        }
        self::JavaScriptPrimitiveConversionTransition::PrimitiveIntrinsicError(error_class, reason) => {
            self::JavaScriptPropertyKeyConversionTransition::PropertyKeyIntrinsicError(error_class.clone(), reason.clone())
        }
        self::JavaScriptPrimitiveConversionTransition::PrimitiveInvalid(reason_2) => {
            self::JavaScriptPropertyKeyConversionTransition::PropertyKeyInvalid(reason_2.clone())
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn begin_java_script_property_key_conversion(heap: self::JavaScriptOrdinaryHeap, input: self::JavaScriptSemanticValue, exotic_key: self::JavaScriptSemanticValue) -> self::JavaScriptPropertyKeyConversionTransition {
    self::finish_java_script_property_key_conversion(self::begin_java_script_primitive_conversion(heap.clone(), input.clone(), exotic_key.clone(), &(self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103]))))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn step_java_script_property_key_conversion(heap: self::JavaScriptOrdinaryHeap, state: self::JavaScriptPrimitiveConversionState) -> self::JavaScriptPropertyKeyConversionTransition {
    if ((match state.clone() {
        self::JavaScriptPrimitiveConversionState::PrimitiveConversion(_, _, source_ml_source1_data_property_value1083, _) => {
            source_ml_source1_data_property_value1083.clone()
        }
    }) != (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103]))) {
        self::JavaScriptPropertyKeyConversionTransition::PropertyKeyInvalid(self::ml_string::Text::from_units(vec![97, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 107, 101, 121, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 116, 114, 105, 110, 103, 32, 104, 105, 110, 116]))
    } else {
        self::finish_java_script_property_key_conversion(self::step_java_script_primitive_conversion(heap.clone(), state.clone()))
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn resume_java_script_property_key_conversion(heap: self::JavaScriptOrdinaryHeap, state: self::JavaScriptPrimitiveConversionState, completion: self::JavaScriptPrimitiveConversionCompletion) -> self::JavaScriptPropertyKeyConversionTransition {
    if ((match state.clone() {
        self::JavaScriptPrimitiveConversionState::PrimitiveConversion(_, _, source_ml_source1_data_property_value1085, _) => {
            source_ml_source1_data_property_value1085.clone()
        }
    }) != (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103]))) {
        self::JavaScriptPropertyKeyConversionTransition::PropertyKeyInvalid(self::ml_string::Text::from_units(vec![97, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 107, 101, 121, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 116, 114, 105, 110, 103, 32, 104, 105, 110, 116]))
    } else {
        self::finish_java_script_property_key_conversion(self::resume_java_script_primitive_conversion(heap.clone(), state.clone(), completion.clone()))
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn project_java_script_property_key_completion(completion: self::JavaScriptSemanticCompletion) -> self::JavaScriptPrimitiveConversionCompletion {
    match completion.clone() {
        self::JavaScriptSemanticCompletion::NormalCompletion(value) => {
            let value = *value;
            self::JavaScriptPrimitiveConversionCompletion::NormalCompletion(Box::new(value.clone()))
        }
        self::JavaScriptSemanticCompletion::ThrowCompletion(value_2) => {
            let value_2 = *value_2;
            self::JavaScriptPrimitiveConversionCompletion::ThrowCompletion(Box::new(value_2.clone()))
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn complete_java_script_property_definition(heap: self::JavaScriptOrdinaryHeap, target: self::JavaScriptSemanticValue, key: self::JavaScriptSemanticValue, descriptor: (bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool)) -> self::JavaScriptPropertyDefinitionTransition {
    {
        let error = self::read_java_script_ordinary_heap_error(heap.clone());
        if (error != (self::ml_string::Text::from_units(vec![]))) {
            self::JavaScriptPropertyDefinitionTransition::DefinitionInvalid(error.clone())
        } else {
            {
                let value_error = self::read_java_script_ordinary_value_error(heap.clone(), target.clone());
                if (value_error != (self::ml_string::Text::from_units(vec![]))) {
                    self::JavaScriptPropertyDefinitionTransition::DefinitionInvalid(value_error.clone())
                } else {
                    match target.clone() {
                        self::JavaScriptSemanticValue::Undefined => {
                            self::JavaScriptPropertyDefinitionTransition::DefinitionInvalid(self::ml_string::Text::from_units(vec![97, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 116, 97, 114, 103, 101, 116]))
                        }
                        self::JavaScriptSemanticValue::Null => {
                            self::JavaScriptPropertyDefinitionTransition::DefinitionInvalid(self::ml_string::Text::from_units(vec![97, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 116, 97, 114, 103, 101, 116]))
                        }
                        self::JavaScriptSemanticValue::Boolean(source_ml_source1_f1086) => {
                            self::JavaScriptPropertyDefinitionTransition::DefinitionInvalid(self::ml_string::Text::from_units(vec![97, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 116, 97, 114, 103, 101, 116]))
                        }
                        self::JavaScriptSemanticValue::Number(source_ml_source1_f1087) => {
                            self::JavaScriptPropertyDefinitionTransition::DefinitionInvalid(self::ml_string::Text::from_units(vec![97, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 116, 97, 114, 103, 101, 116]))
                        }
                        self::JavaScriptSemanticValue::Integer(source_ml_source1_f1088) => {
                            self::JavaScriptPropertyDefinitionTransition::DefinitionInvalid(self::ml_string::Text::from_units(vec![97, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 116, 97, 114, 103, 101, 116]))
                        }
                        self::JavaScriptSemanticValue::Text(source_ml_source1_f1089) => {
                            self::JavaScriptPropertyDefinitionTransition::DefinitionInvalid(self::ml_string::Text::from_units(vec![97, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 116, 97, 114, 103, 101, 116]))
                        }
                        self::JavaScriptSemanticValue::Symbol(source_ml_source1_f1090) => {
                            self::JavaScriptPropertyDefinitionTransition::DefinitionInvalid(self::ml_string::Text::from_units(vec![97, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 116, 97, 114, 103, 101, 116]))
                        }
                        self::JavaScriptSemanticValue::Object(identity, source_ml_source1_f1091) => {
                            {
                                let transition = self::define_java_script_ordinary_property(heap.clone(), identity.clone(), key.clone(), descriptor.clone());
                                if !((transition.clone()).2) {
                                    if (((transition.clone()).0) != (self::ml_string::Text::from_units(vec![]))) {
                                        self::JavaScriptPropertyDefinitionTransition::DefinitionInvalid((transition.clone()).0)
                                    } else {
                                        self::JavaScriptPropertyDefinitionTransition::DefinitionIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 68, 101, 102, 105, 110, 101, 79, 119, 110, 80, 114, 111, 112, 101, 114, 116, 121, 32, 114, 101, 106, 101, 99, 116, 101, 100, 32, 116, 104, 101, 32, 114, 101, 113, 117, 101, 115, 116, 101, 100, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114]))
                                    }
                                } else {
                                    self::JavaScriptPropertyDefinitionTransition::DefinitionCompleted(Box::new((transition.clone()).1), Box::new(target.clone()), (transition.clone()).3)
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
pub fn continue_java_script_property_descriptor_definition(heap: self::JavaScriptOrdinaryHeap, target: self::JavaScriptSemanticValue, key: self::JavaScriptSemanticValue, transition: self::JavaScriptDescriptorConversionTransition) -> self::JavaScriptPropertyDefinitionTransition {
    match transition.clone() {
        self::JavaScriptDescriptorConversionTransition::DescriptorContinuation(state) => {
            let state = *state;
            self::JavaScriptPropertyDefinitionTransition::DefinitionContinuation(Box::new(self::JavaScriptPropertyDefinitionState::PropertyDefinitionDescriptor(Box::new(target.clone()), Box::new(key.clone()), Box::new(state.clone()))))
        }
        self::JavaScriptDescriptorConversionTransition::DescriptorCall(state_2, callee, receiver, arguments) => {
            let state_2 = *state_2;
            let callee = *callee;
            let receiver = *receiver;
            self::JavaScriptPropertyDefinitionTransition::DefinitionCall(Box::new(self::JavaScriptPropertyDefinitionState::PropertyDefinitionDescriptor(Box::new(target.clone()), Box::new(key.clone()), Box::new(state_2.clone()))), Box::new(callee.clone()), Box::new(receiver.clone()), arguments)
        }
        self::JavaScriptDescriptorConversionTransition::DescriptorCompleted(descriptor) => {
            self::complete_java_script_property_definition(heap.clone(), target.clone(), key.clone(), descriptor.clone())
        }
        self::JavaScriptDescriptorConversionTransition::DescriptorThrow(value) => {
            let value = *value;
            self::JavaScriptPropertyDefinitionTransition::DefinitionThrow(Box::new(value.clone()))
        }
        self::JavaScriptDescriptorConversionTransition::DescriptorIntrinsicError(error_class, reason) => {
            self::JavaScriptPropertyDefinitionTransition::DefinitionIntrinsicError(error_class.clone(), reason.clone())
        }
        self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(reason_2) => {
            self::JavaScriptPropertyDefinitionTransition::DefinitionInvalid(reason_2.clone())
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn continue_java_script_property_key_definition(heap: self::JavaScriptOrdinaryHeap, target: self::JavaScriptSemanticValue, attributes: self::JavaScriptSemanticValue, transition: self::JavaScriptPropertyKeyConversionTransition) -> self::JavaScriptPropertyDefinitionTransition {
    match transition.clone() {
        self::JavaScriptPropertyKeyConversionTransition::PropertyKeyContinuation(state) => {
            let state = *state;
            self::JavaScriptPropertyDefinitionTransition::DefinitionContinuation(Box::new(self::JavaScriptPropertyDefinitionState::PropertyDefinitionKey(Box::new(target.clone()), Box::new(attributes.clone()), Box::new(state.clone()))))
        }
        self::JavaScriptPropertyKeyConversionTransition::PropertyKeyCall(state_2, callee, receiver, arguments) => {
            let state_2 = *state_2;
            let callee = *callee;
            let receiver = *receiver;
            self::JavaScriptPropertyDefinitionTransition::DefinitionCall(Box::new(self::JavaScriptPropertyDefinitionState::PropertyDefinitionKey(Box::new(target.clone()), Box::new(attributes.clone()), Box::new(state_2.clone()))), Box::new(callee.clone()), Box::new(receiver.clone()), arguments)
        }
        self::JavaScriptPropertyKeyConversionTransition::PropertyKeyCompleted(key) => {
            let key = *key;
            self::continue_java_script_property_descriptor_definition(heap.clone(), target.clone(), key.clone(), self::begin_java_script_descriptor_conversion(heap.clone(), attributes.clone()))
        }
        self::JavaScriptPropertyKeyConversionTransition::PropertyKeyThrow(value) => {
            let value = *value;
            self::JavaScriptPropertyDefinitionTransition::DefinitionThrow(Box::new(value.clone()))
        }
        self::JavaScriptPropertyKeyConversionTransition::PropertyKeyIntrinsicError(error_class, reason) => {
            self::JavaScriptPropertyDefinitionTransition::DefinitionIntrinsicError(error_class.clone(), reason.clone())
        }
        self::JavaScriptPropertyKeyConversionTransition::PropertyKeyInvalid(reason_2) => {
            self::JavaScriptPropertyDefinitionTransition::DefinitionInvalid(reason_2.clone())
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn begin_java_script_property_definition(heap: self::JavaScriptOrdinaryHeap, target: self::JavaScriptSemanticValue, key_input: self::JavaScriptSemanticValue, attributes: self::JavaScriptSemanticValue, exotic_key: self::JavaScriptSemanticValue) -> self::JavaScriptPropertyDefinitionTransition {
    {
        let error = self::read_java_script_ordinary_heap_error(heap.clone());
        if (error != (self::ml_string::Text::from_units(vec![]))) {
            self::JavaScriptPropertyDefinitionTransition::DefinitionInvalid(error.clone())
        } else {
            {
                let value_error = self::read_java_script_ordinary_value_error(heap.clone(), target.clone());
                if (value_error != (self::ml_string::Text::from_units(vec![]))) {
                    self::JavaScriptPropertyDefinitionTransition::DefinitionInvalid(value_error.clone())
                } else {
                    match target.clone() {
                        self::JavaScriptSemanticValue::Undefined => {
                            self::JavaScriptPropertyDefinitionTransition::DefinitionIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 100, 101, 102, 105, 110, 101, 80, 114, 111, 112, 101, 114, 116, 121, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 97, 110, 32, 79, 98, 106, 101, 99, 116, 32, 116, 97, 114, 103, 101, 116]))
                        }
                        self::JavaScriptSemanticValue::Null => {
                            self::JavaScriptPropertyDefinitionTransition::DefinitionIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 100, 101, 102, 105, 110, 101, 80, 114, 111, 112, 101, 114, 116, 121, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 97, 110, 32, 79, 98, 106, 101, 99, 116, 32, 116, 97, 114, 103, 101, 116]))
                        }
                        self::JavaScriptSemanticValue::Boolean(source_ml_source1_f1092) => {
                            self::JavaScriptPropertyDefinitionTransition::DefinitionIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 100, 101, 102, 105, 110, 101, 80, 114, 111, 112, 101, 114, 116, 121, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 97, 110, 32, 79, 98, 106, 101, 99, 116, 32, 116, 97, 114, 103, 101, 116]))
                        }
                        self::JavaScriptSemanticValue::Number(source_ml_source1_f1093) => {
                            self::JavaScriptPropertyDefinitionTransition::DefinitionIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 100, 101, 102, 105, 110, 101, 80, 114, 111, 112, 101, 114, 116, 121, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 97, 110, 32, 79, 98, 106, 101, 99, 116, 32, 116, 97, 114, 103, 101, 116]))
                        }
                        self::JavaScriptSemanticValue::Integer(source_ml_source1_f1094) => {
                            self::JavaScriptPropertyDefinitionTransition::DefinitionIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 100, 101, 102, 105, 110, 101, 80, 114, 111, 112, 101, 114, 116, 121, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 97, 110, 32, 79, 98, 106, 101, 99, 116, 32, 116, 97, 114, 103, 101, 116]))
                        }
                        self::JavaScriptSemanticValue::Text(source_ml_source1_f1095) => {
                            self::JavaScriptPropertyDefinitionTransition::DefinitionIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 100, 101, 102, 105, 110, 101, 80, 114, 111, 112, 101, 114, 116, 121, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 97, 110, 32, 79, 98, 106, 101, 99, 116, 32, 116, 97, 114, 103, 101, 116]))
                        }
                        self::JavaScriptSemanticValue::Symbol(source_ml_source1_f1096) => {
                            self::JavaScriptPropertyDefinitionTransition::DefinitionIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 100, 101, 102, 105, 110, 101, 80, 114, 111, 112, 101, 114, 116, 121, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 97, 110, 32, 79, 98, 106, 101, 99, 116, 32, 116, 97, 114, 103, 101, 116]))
                        }
                        self::JavaScriptSemanticValue::Object(source_ml_source1_f1097, source_ml_source1_f1098) => {
                            self::continue_java_script_property_key_definition(heap.clone(), target.clone(), attributes.clone(), self::begin_java_script_property_key_conversion(heap.clone(), key_input.clone(), exotic_key.clone()))
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn step_java_script_property_definition(heap: self::JavaScriptOrdinaryHeap, state: self::JavaScriptPropertyDefinitionState) -> self::JavaScriptPropertyDefinitionTransition {
    {
        let error = self::read_java_script_ordinary_value_error(heap.clone(), match state.clone() {
            self::JavaScriptPropertyDefinitionState::PropertyDefinitionKey(source_ml_source1_data_property_value1099, _, _) => {
                let source_ml_source1_data_property_value1099 = *source_ml_source1_data_property_value1099;
                source_ml_source1_data_property_value1099.clone()
            }
            self::JavaScriptPropertyDefinitionState::PropertyDefinitionDescriptor(source_ml_source1_data_property_value1100, _, _) => {
                let source_ml_source1_data_property_value1100 = *source_ml_source1_data_property_value1100;
                source_ml_source1_data_property_value1100.clone()
            }
        });
        if (error != (self::ml_string::Text::from_units(vec![]))) {
            self::JavaScriptPropertyDefinitionTransition::DefinitionInvalid(error.clone())
        } else {
            if {
                let source_ml_source1_s1103 = match state.clone() {
                    self::JavaScriptPropertyDefinitionState::PropertyDefinitionKey(source_ml_source1_data_property_value1111, _, _) => {
                        let source_ml_source1_data_property_value1111 = *source_ml_source1_data_property_value1111;
                        source_ml_source1_data_property_value1111.clone()
                    }
                    self::JavaScriptPropertyDefinitionState::PropertyDefinitionDescriptor(source_ml_source1_data_property_value1112, _, _) => {
                        let source_ml_source1_data_property_value1112 = *source_ml_source1_data_property_value1112;
                        source_ml_source1_data_property_value1112.clone()
                    }
                };
                match source_ml_source1_s1103.clone() {
                    self::JavaScriptSemanticValue::Undefined => {
                        true
                    }
                    self::JavaScriptSemanticValue::Null => {
                        true
                    }
                    self::JavaScriptSemanticValue::Boolean(source_ml_source1_f1104) => {
                        true
                    }
                    self::JavaScriptSemanticValue::Number(source_ml_source1_f1105) => {
                        true
                    }
                    self::JavaScriptSemanticValue::Integer(source_ml_source1_f1106) => {
                        true
                    }
                    self::JavaScriptSemanticValue::Text(source_ml_source1_f1107) => {
                        true
                    }
                    self::JavaScriptSemanticValue::Symbol(source_ml_source1_f1108) => {
                        true
                    }
                    self::JavaScriptSemanticValue::Object(source_ml_source1_f1109, source_ml_source1_f1110) => {
                        false
                    }
                }
            } {
                self::JavaScriptPropertyDefinitionTransition::DefinitionInvalid(self::ml_string::Text::from_units(vec![97, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 116, 97, 114, 103, 101, 116]))
            } else {
                match state.clone() {
                    self::JavaScriptPropertyDefinitionState::PropertyDefinitionKey(target, attributes, conversion) => {
                        let target = *target;
                        let attributes = *attributes;
                        let conversion = *conversion;
                        self::continue_java_script_property_key_definition(heap.clone(), target.clone(), attributes.clone(), self::step_java_script_property_key_conversion(heap.clone(), conversion.clone()))
                    }
                    self::JavaScriptPropertyDefinitionState::PropertyDefinitionDescriptor(target_2, key, conversion_2) => {
                        let target_2 = *target_2;
                        let key = *key;
                        let conversion_2 = *conversion_2;
                        self::continue_java_script_property_descriptor_definition(heap.clone(), target_2.clone(), key.clone(), self::step_java_script_descriptor_conversion(heap.clone(), conversion_2.clone()))
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn resume_java_script_property_definition(heap: self::JavaScriptOrdinaryHeap, state: self::JavaScriptPropertyDefinitionState, completion: self::JavaScriptSemanticCompletion) -> self::JavaScriptPropertyDefinitionTransition {
    {
        let error = self::read_java_script_ordinary_value_error(heap.clone(), match state.clone() {
            self::JavaScriptPropertyDefinitionState::PropertyDefinitionKey(source_ml_source1_data_property_value1113, _, _) => {
                let source_ml_source1_data_property_value1113 = *source_ml_source1_data_property_value1113;
                source_ml_source1_data_property_value1113.clone()
            }
            self::JavaScriptPropertyDefinitionState::PropertyDefinitionDescriptor(source_ml_source1_data_property_value1114, _, _) => {
                let source_ml_source1_data_property_value1114 = *source_ml_source1_data_property_value1114;
                source_ml_source1_data_property_value1114.clone()
            }
        });
        if (error != (self::ml_string::Text::from_units(vec![]))) {
            self::JavaScriptPropertyDefinitionTransition::DefinitionInvalid(error.clone())
        } else {
            if {
                let source_ml_source1_s1117 = match state.clone() {
                    self::JavaScriptPropertyDefinitionState::PropertyDefinitionKey(source_ml_source1_data_property_value1125, _, _) => {
                        let source_ml_source1_data_property_value1125 = *source_ml_source1_data_property_value1125;
                        source_ml_source1_data_property_value1125.clone()
                    }
                    self::JavaScriptPropertyDefinitionState::PropertyDefinitionDescriptor(source_ml_source1_data_property_value1126, _, _) => {
                        let source_ml_source1_data_property_value1126 = *source_ml_source1_data_property_value1126;
                        source_ml_source1_data_property_value1126.clone()
                    }
                };
                match source_ml_source1_s1117.clone() {
                    self::JavaScriptSemanticValue::Undefined => {
                        true
                    }
                    self::JavaScriptSemanticValue::Null => {
                        true
                    }
                    self::JavaScriptSemanticValue::Boolean(source_ml_source1_f1118) => {
                        true
                    }
                    self::JavaScriptSemanticValue::Number(source_ml_source1_f1119) => {
                        true
                    }
                    self::JavaScriptSemanticValue::Integer(source_ml_source1_f1120) => {
                        true
                    }
                    self::JavaScriptSemanticValue::Text(source_ml_source1_f1121) => {
                        true
                    }
                    self::JavaScriptSemanticValue::Symbol(source_ml_source1_f1122) => {
                        true
                    }
                    self::JavaScriptSemanticValue::Object(source_ml_source1_f1123, source_ml_source1_f1124) => {
                        false
                    }
                }
            } {
                self::JavaScriptPropertyDefinitionTransition::DefinitionInvalid(self::ml_string::Text::from_units(vec![97, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 116, 97, 114, 103, 101, 116]))
            } else {
                match state.clone() {
                    self::JavaScriptPropertyDefinitionState::PropertyDefinitionKey(target, attributes, conversion) => {
                        let target = *target;
                        let attributes = *attributes;
                        let conversion = *conversion;
                        self::continue_java_script_property_key_definition(heap.clone(), target.clone(), attributes.clone(), self::resume_java_script_property_key_conversion(heap.clone(), conversion.clone(), self::project_java_script_property_key_completion(completion.clone())))
                    }
                    self::JavaScriptPropertyDefinitionState::PropertyDefinitionDescriptor(target_2, key, conversion_2) => {
                        let target_2 = *target_2;
                        let key = *key;
                        let conversion_2 = *conversion_2;
                        self::continue_java_script_property_descriptor_definition(heap.clone(), target_2.clone(), key.clone(), self::resume_java_script_descriptor_conversion(heap.clone(), conversion_2.clone(), completion.clone()))
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_call_coordinate(value: f64, limit: f64) -> bool {
    self::test_java_script_call_code_coordinate(value, limit)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn find_java_script_call_function(program: self::JavaScriptCallProgram, identity: self::ml::Big) -> f64 {
    self::find_java_script_call_code_function(program.clone(), identity.clone())
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_call_registers(instruction: self::JavaScriptCallInstruction) -> Vec<f64> {
    self::read_java_script_call_code_registers(instruction.clone())
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_call_program_error(program: self::JavaScriptCallProgram, heap: self::JavaScriptOrdinaryHeap) -> self::ml_string::Text {
    {
        let error = self::read_java_script_ordinary_heap_error(heap.clone());
        if (error != (self::ml_string::Text::from_units(vec![]))) {
            error.clone()
        } else {
            if (((((match &program { self::JavaScriptCallProgram::CallProgram(source_ml_source1_data_property_value1128, _, _) => source_ml_source1_data_property_value1128.as_slice() }).len() as f64)) > (4096f64)) || ((((match &program { self::JavaScriptCallProgram::CallProgram(_, source_ml_source1_data_property_value1130, _) => source_ml_source1_data_property_value1130.as_slice() }).len() as f64)) > (4096f64))) {
                self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 99, 111, 100, 101, 32, 100, 105, 114, 101, 99, 116, 111, 114, 105, 101, 115, 32, 101, 120, 99, 101, 101, 100, 32, 116, 104, 101, 105, 114, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 114, 101, 115, 111, 117, 114, 99, 101, 32, 98, 117, 100, 103, 101, 116])
            } else {
                {
                    let previous = self::ml::Big::from_i128(0);
                    {
                        let source_ml_temporary_values33 = match program.clone() {
                            self::JavaScriptCallProgram::CallProgram(_, source_ml_source1_data_property_value1131, _) => {
                                source_ml_source1_data_property_value1131
                            }
                        };
                        {
                            let source_ml_temporary_index33 = self::ml::Big::from_i128(0);
                            {
                                let source_ml_source1_s1132 = self::source_ml_temporary_read_java_script_call_program_error_loop38(previous.clone(), source_ml_temporary_values33, source_ml_temporary_index33.clone());
                                match source_ml_source1_s1132.clone() {
                                    self::MlTemporaryReadJavaScriptCallProgramErrorLoop38Result::MlTemporaryReadJavaScriptCallProgramErrorLoop38Done(previous_2) => {
                                        if {
                                            let source_ml_source1_s1134 = match program.clone() {
                                                self::JavaScriptCallProgram::CallProgram(_, _, source_ml_source1_data_property_value1142) => {
                                                    let source_ml_source1_data_property_value1142 = *source_ml_source1_data_property_value1142;
                                                    source_ml_source1_data_property_value1142.clone()
                                                }
                                            };
                                            match source_ml_source1_s1134.clone() {
                                                self::JavaScriptSemanticValue::Undefined => {
                                                    true
                                                }
                                                self::JavaScriptSemanticValue::Null => {
                                                    true
                                                }
                                                self::JavaScriptSemanticValue::Boolean(source_ml_source1_f1135) => {
                                                    true
                                                }
                                                self::JavaScriptSemanticValue::Number(source_ml_source1_f1136) => {
                                                    true
                                                }
                                                self::JavaScriptSemanticValue::Integer(source_ml_source1_f1137) => {
                                                    true
                                                }
                                                self::JavaScriptSemanticValue::Text(source_ml_source1_f1138) => {
                                                    true
                                                }
                                                self::JavaScriptSemanticValue::Symbol(source_ml_source1_f1139) => {
                                                    false
                                                }
                                                self::JavaScriptSemanticValue::Object(source_ml_source1_f1140, source_ml_source1_f1141) => {
                                                    true
                                                }
                                            }
                                        } {
                                            self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 99, 111, 100, 101, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 116, 104, 101, 32, 97, 117, 116, 104, 111, 114, 105, 116, 97, 116, 105, 118, 101, 32, 119, 101, 108, 108, 45, 107, 110, 111, 119, 110, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 83, 121, 109, 98, 111, 108])
                                        } else {
                                            {
                                                let key_error = self::read_java_script_ordinary_value_error(heap.clone(), match program.clone() {
                                                    self::JavaScriptCallProgram::CallProgram(_, _, source_ml_source1_data_property_value1143) => {
                                                        let source_ml_source1_data_property_value1143 = *source_ml_source1_data_property_value1143;
                                                        source_ml_source1_data_property_value1143.clone()
                                                    }
                                                });
                                                if (key_error != (self::ml_string::Text::from_units(vec![]))) {
                                                    key_error.clone()
                                                } else {
                                                    {
                                                        let previous_3 = self::ml::Big::from_i128(0);
                                                        {
                                                            let count = 0f64;
                                                            {
                                                                let source_ml_temporary_values34 = match program.clone() {
                                                                    self::JavaScriptCallProgram::CallProgram(source_ml_source1_data_property_value1144, _, _) => {
                                                                        source_ml_source1_data_property_value1144
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_temporary_index34 = self::ml::Big::from_i128(0);
                                                                    {
                                                                        let source_ml_source1_s1145 = self::source_ml_temporary_read_java_script_call_program_error_loop39(program.clone(), heap.clone(), previous_3.clone(), count, source_ml_temporary_values34, source_ml_temporary_index34.clone());
                                                                        match source_ml_source1_s1145.clone() {
                                                                            self::MlTemporaryReadJavaScriptCallProgramErrorLoop39Result::MlTemporaryReadJavaScriptCallProgramErrorLoop39Done => {
                                                                                self::ml_string::Text::from_units(vec![])
                                                                            }
                                                                            self::MlTemporaryReadJavaScriptCallProgramErrorLoop39Result::MlTemporaryReadJavaScriptCallProgramErrorLoop39Return(source_ml_temporary_result39) => {
                                                                                source_ml_temporary_result39.clone()
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
                                    self::MlTemporaryReadJavaScriptCallProgramErrorLoop38Result::MlTemporaryReadJavaScriptCallProgramErrorLoop38Return(source_ml_temporary_result42) => {
                                        source_ml_temporary_result42.clone()
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
pub fn replace_java_script_call_frame(heap: self::JavaScriptOrdinaryHeap, frames: Vec<self::JavaScriptCallFrame>, frame: self::JavaScriptCallFrame) -> self::JavaScriptCallMachine {
    self::JavaScriptCallMachine::CallMachine(Box::new(heap.clone()), { let ml_values4 = frames.clone(); let ml_callback4 = { let frames = frames.clone(); let frame = frame.clone(); std::rc::Rc::new(move |entry: self::JavaScriptCallFrame, index: f64| -> self::JavaScriptCallFrame { if (index == (((frames.len() as f64) - 1f64))) {
        frame.clone()
    } else {
        entry.clone()
    } }) }; ml_values4.iter().cloned().enumerate().map(|(ml_index4, ml_value4)| ml_callback4(ml_value4, (ml_index4 as f64))).collect::<Vec<_>>() })
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn write_java_script_call_register(frame: self::JavaScriptCallFrame, destination: f64, value: self::JavaScriptSemanticValue) -> self::JavaScriptCallFrame {
    {
        let source_ml_source1_nominal_spread1147 = frame.clone();
        {
            let source_ml_source1_nominal_field1148 = match source_ml_source1_nominal_spread1147.clone() {
                self::JavaScriptCallFrame::CallFrame(source_ml_source1_data_property_value1149, _, _, _, _, _) => {
                    source_ml_source1_data_property_value1149.clone()
                }
            };
            {
                let source_ml_source1_nominal_field1150 = match source_ml_source1_nominal_spread1147.clone() {
                    self::JavaScriptCallFrame::CallFrame(_, source_ml_source1_data_property_value1151, _, _, _, _) => {
                        source_ml_source1_data_property_value1151
                    }
                };
                {
                    let source_ml_source1_nominal_field1152 = match source_ml_source1_nominal_spread1147.clone() {
                        self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source1_data_property_value1153, _, _, _) => {
                            source_ml_source1_data_property_value1153
                        }
                    };
                    {
                        let source_ml_source1_nominal_field1154 = match source_ml_source1_nominal_spread1147.clone() {
                            self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source1_data_property_value1155, _, _) => {
                                source_ml_source1_data_property_value1155
                            }
                        };
                        {
                            let source_ml_source1_nominal_field1156 = match source_ml_source1_nominal_spread1147.clone() {
                                self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source1_data_property_value1157, _) => {
                                    source_ml_source1_data_property_value1157
                                }
                            };
                            {
                                let source_ml_source1_nominal_field1158 = match source_ml_source1_nominal_spread1147.clone() {
                                    self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source1_data_property_value1159) => {
                                        let source_ml_source1_data_property_value1159 = *source_ml_source1_data_property_value1159;
                                        source_ml_source1_data_property_value1159.clone()
                                    }
                                };
                                {
                                    let source_ml_source1_nominal_override1160 = { let ml_values5 = match frame.clone() {
                                        self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source1_data_property_value1146, _, _, _) => {
                                            source_ml_source1_data_property_value1146
                                        }
                                    }; let ml_callback5 = { let destination = destination; let value = value.clone(); std::rc::Rc::new(move |entry: self::JavaScriptSemanticValue, index: f64| -> self::JavaScriptSemanticValue { if (index == destination) {
                                        value.clone()
                                    } else {
                                        entry.clone()
                                    } }) }; ml_values5.iter().cloned().enumerate().map(|(ml_index5, ml_value5)| ml_callback5(ml_value5, (ml_index5 as f64))).collect::<Vec<_>>() };
                                    {
                                        let source_ml_source1_nominal_override1161 = self::JavaScriptCallPending::CallIdle;
                                        self::JavaScriptCallFrame::CallFrame(source_ml_source1_nominal_field1148.clone(), source_ml_source1_nominal_field1150, source_ml_source1_nominal_override1160, source_ml_source1_nominal_field1154, source_ml_source1_nominal_field1156, Box::new(source_ml_source1_nominal_override1161.clone()))
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
pub fn create_java_script_call_frame(fn_: self::JavaScriptCallFunction, receiver: self::JavaScriptSemanticValue, arguments_: Vec<self::JavaScriptSemanticValue>) -> self::JavaScriptCallFrame {
    {
        let registers = Vec::<self::JavaScriptSemanticValue>::new();
        {
            let index = 0f64;
            {
                let registers_2 = self::source_ml_temporary_create_java_script_call_frame_loop43(fn_.clone(), registers, index);
                {
                    let bound = if ((match fn_.clone() {
                        self::JavaScriptCallFunction::CallFunction(_, _, source_ml_source1_data_property_value1163, _, _, _, _, _) => {
                            source_ml_source1_data_property_value1163.clone()
                        }
                    }) == (self::ml_string::Text::from_units(vec![108, 101, 120, 105, 99, 97, 108]))) {
                        match fn_.clone() {
                            self::JavaScriptCallFunction::CallFunction(_, _, _, source_ml_source1_data_property_value1164, _, _, _, _) => {
                                let source_ml_source1_data_property_value1164 = *source_ml_source1_data_property_value1164;
                                source_ml_source1_data_property_value1164.clone()
                            }
                        }
                    } else {
                        receiver.clone()
                    };
                    {
                        let registers_3 = { let ml_values6 = registers_2; let ml_callback6 = { let bound = bound.clone(); std::rc::Rc::new(move |entry: self::JavaScriptSemanticValue, index_2: f64| -> self::JavaScriptSemanticValue { if (index_2 == (0f64)) {
                            bound.clone()
                        } else {
                            entry.clone()
                        } }) }; ml_values6.iter().cloned().enumerate().map(|(ml_index6, ml_value6)| ml_callback6(ml_value6, (ml_index6 as f64))).collect::<Vec<_>>() };
                        {
                            let index_2 = 0f64;
                            {
                                let registers_4 = self::source_ml_temporary_create_java_script_call_frame_loop44(fn_.clone(), arguments_.clone(), registers_3, index_2);
                                {
                                    let index_3 = 0f64;
                                    {
                                        let registers_5 = self::source_ml_temporary_create_java_script_call_frame_loop45(fn_.clone(), registers_4, index_3);
                                        self::JavaScriptCallFrame::CallFrame(match fn_.clone() {
                                            self::JavaScriptCallFunction::CallFunction(source_ml_source1_data_property_value1165, _, _, _, _, _, _, _) => {
                                                source_ml_source1_data_property_value1165.clone()
                                            }
                                        }, 0f64, registers_5, arguments_, Vec::<self::JavaScriptCallHandler>::new(), Box::new(self::JavaScriptCallPending::CallIdle))
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
pub fn read_java_script_call_machine_error(program: self::JavaScriptCallProgram, machine: self::JavaScriptCallMachine) -> self::ml_string::Text {
    {
        let error = self::read_java_script_call_program_error(program.clone(), match machine.clone() {
            self::JavaScriptCallMachine::CallMachine(source_ml_source1_data_property_value1166, _) => {
                let source_ml_source1_data_property_value1166 = *source_ml_source1_data_property_value1166;
                source_ml_source1_data_property_value1166.clone()
            }
        });
        if (error != (self::ml_string::Text::from_units(vec![]))) {
            error.clone()
        } else {
            if (((((match &machine { self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value1168) => source_ml_source1_data_property_value1168.as_slice() }).len() as f64)) == (0f64)) || ((((match &machine { self::JavaScriptCallMachine::CallMachine(_, source_ml_source1_data_property_value1170) => source_ml_source1_data_property_value1170.as_slice() }).len() as f64)) > (1024f64))) {
                self::ml_string::Text::from_units(vec![97, 32, 99, 97, 108, 108, 32, 109, 97, 99, 104, 105, 110, 101, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 98, 111, 117, 110, 100, 101, 100, 32, 97, 99, 116, 105, 118, 101, 32, 115, 116, 97, 99, 107])
            } else {
                {
                    let slots = 0f64;
                    {
                        let argument_slots = 0f64;
                        {
                            let index = 0f64;
                            {
                                let source_ml_source1_s1171 = self::source_ml_temporary_read_java_script_call_machine_error_loop49(program.clone(), machine.clone(), slots, argument_slots, index);
                                match source_ml_source1_s1171.clone() {
                                    self::MlTemporaryReadJavaScriptCallMachineErrorLoop49Result::MlTemporaryReadJavaScriptCallMachineErrorLoop49Done => {
                                        self::ml_string::Text::from_units(vec![])
                                    }
                                    self::MlTemporaryReadJavaScriptCallMachineErrorLoop49Result::MlTemporaryReadJavaScriptCallMachineErrorLoop49Return(source_ml_temporary_result49) => {
                                        source_ml_temporary_result49.clone()
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
