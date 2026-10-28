// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptCallFrame;
use super::JavaScriptCallInstruction;
use super::JavaScriptCallMachine;
use super::JavaScriptCallPrimitiveResult;
use super::JavaScriptCallProgram;
use super::JavaScriptDescriptorConversionTransition;
use super::JavaScriptNumberConversionTransition;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptPrimitiveConversionCompletion;
use super::JavaScriptPrimitiveConversionState;
use super::JavaScriptPrimitiveConversionTransition;
use super::JavaScriptPropertyDefinitionState;
use super::JavaScriptPropertyDefinitionTransition;
use super::JavaScriptPropertyKeyConversionTransition;
use super::JavaScriptSemanticCompletion;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop128Result;
use super::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop129Result;
use super::apply_java_script_primitive_conversion_method;
use super::begin_java_script_descriptor_conversion;
use super::begin_java_script_primitive_conversion;
use super::continue_java_script_primitive_conversion;
use super::define_java_script_ordinary_property;
use super::execute_java_script_call_primitive;
use super::find_java_script_call_code_function;
use super::ml;
use super::ml_number;
use super::ml_source_exception;
use super::ml_string;
use super::read_java_script_call_code_registers;
use super::read_java_script_ordinary_heap_error;
use super::read_java_script_ordinary_value_error;
use super::read_java_script_primitive_conversion_state_error;
use super::resume_java_script_descriptor_conversion;
use super::source_ml_source1_temporary_finish_java_script_property_key_conversion_loop122;
use super::source_ml_source1_temporary_read_java_script_call_program_error_loop128;
use super::source_ml_source1_temporary_read_java_script_call_program_error_loop129;
use super::step_java_script_descriptor_conversion;
use super::step_java_script_primitive_conversion;
use super::test_java_script_call_code_coordinate;

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
                                        self::JavaScriptPrimitiveConversionCompletion::NormalCompletion(source_ml_source2_data_property_value1176) => {
                                            let source_ml_source2_data_property_value1176 = *source_ml_source2_data_property_value1176;
                                            source_ml_source2_data_property_value1176.clone()
                                        }
                                        self::JavaScriptPrimitiveConversionCompletion::ThrowCompletion(source_ml_source2_data_property_value1177) => {
                                            let source_ml_source2_data_property_value1177 = *source_ml_source2_data_property_value1177;
                                            source_ml_source2_data_property_value1177.clone()
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
                let source_ml_source2_string_argument1193 = value_value_2;
                self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source2_string_argument1193))
            }
        }
        self::JavaScriptSemanticValue::Integer(value_value_3) => {
            {
                let source_ml_source2_string_argument1194 = value_value_3.clone();
                self::ml_string::Text::from_text(&source_ml_source2_string_argument1194.to_string())
            }
        }
        self::JavaScriptSemanticValue::Text(_) => {
            (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![115, 101, 109, 97, 110, 116, 105, 99, 32, 116, 101, 120, 116, 32, 107, 101, 121, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 101, 120, 97, 99, 116, 32, 117, 110, 105, 116, 115])).0, message_present: true }).raise()
        }
        self::JavaScriptSemanticValue::Symbol(_) => {
            (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![115, 101, 109, 97, 110, 116, 105, 99, 32, 83, 121, 109, 98, 111, 108, 32, 107, 101, 121, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 101, 120, 97, 99, 116, 32, 105, 100, 101, 110, 116, 105, 116, 121])).0, message_present: true }).raise()
        }
        self::JavaScriptSemanticValue::Object(_, _) => {
            (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![97, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 107, 101, 121, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 99, 111, 109, 112, 108, 101, 116, 101, 100, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110])).0, message_present: true }).raise()
        }
    }
}

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
                self::JavaScriptSemanticValue::Text(_) => {
                    true
                }
                _ => {
                    false
                }
            } || match value.clone() {
                self::JavaScriptSemanticValue::Symbol(_) => {
                    true
                }
                _ => {
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
                                            let units_2 = self::source_ml_source1_temporary_finish_java_script_property_key_conversion_loop122(&(spelling), units, index);
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
                                            let units_4 = self::source_ml_source1_temporary_finish_java_script_property_key_conversion_loop122(&(spelling_2), units_3, index_2);
                                            self::JavaScriptPropertyKeyConversionTransition::PropertyKeyCompleted(Box::new(self::JavaScriptSemanticValue::Text(units_4)))
                                        }
                                    }
                                }
                            }
                        }
                    }
                    self::JavaScriptSemanticValue::Boolean(_) => {
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
                                            let units_6 = self::source_ml_source1_temporary_finish_java_script_property_key_conversion_loop122(&(spelling_3), units_5, index_3);
                                            self::JavaScriptPropertyKeyConversionTransition::PropertyKeyCompleted(Box::new(self::JavaScriptSemanticValue::Text(units_6)))
                                        }
                                    }
                                }
                            }
                        }
                    }
                    self::JavaScriptSemanticValue::Number(_) => {
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
                                            let units_8 = self::source_ml_source1_temporary_finish_java_script_property_key_conversion_loop122(&(spelling_4), units_7, index_4);
                                            self::JavaScriptPropertyKeyConversionTransition::PropertyKeyCompleted(Box::new(self::JavaScriptSemanticValue::Text(units_8)))
                                        }
                                    }
                                }
                            }
                        }
                    }
                    self::JavaScriptSemanticValue::Integer(_) => {
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
                                            let units_10 = self::source_ml_source1_temporary_finish_java_script_property_key_conversion_loop122(&(spelling_5), units_9, index_5);
                                            self::JavaScriptPropertyKeyConversionTransition::PropertyKeyCompleted(Box::new(self::JavaScriptSemanticValue::Text(units_10)))
                                        }
                                    }
                                }
                            }
                        }
                    }
                    self::JavaScriptSemanticValue::Text(_) => {
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
                                            let units_12 = self::source_ml_source1_temporary_finish_java_script_property_key_conversion_loop122(&(spelling_6), units_11, index_6);
                                            self::JavaScriptPropertyKeyConversionTransition::PropertyKeyCompleted(Box::new(self::JavaScriptSemanticValue::Text(units_12)))
                                        }
                                    }
                                }
                            }
                        }
                    }
                    self::JavaScriptSemanticValue::Symbol(_) => {
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
                                            let units_14 = self::source_ml_source1_temporary_finish_java_script_property_key_conversion_loop122(&(spelling_7), units_13, index_7);
                                            self::JavaScriptPropertyKeyConversionTransition::PropertyKeyCompleted(Box::new(self::JavaScriptSemanticValue::Text(units_14)))
                                        }
                                    }
                                }
                            }
                        }
                    }
                    self::JavaScriptSemanticValue::Object(_, _) => {
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
        self::JavaScriptPrimitiveConversionState::PrimitiveConversion(_, _, source_ml_source2_data_property_value1217, _) => {
            source_ml_source2_data_property_value1217.clone()
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
        self::JavaScriptPrimitiveConversionState::PrimitiveConversion(_, _, source_ml_source2_data_property_value1219, _) => {
            source_ml_source2_data_property_value1219.clone()
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
                        self::JavaScriptSemanticValue::Boolean(_) => {
                            self::JavaScriptPropertyDefinitionTransition::DefinitionInvalid(self::ml_string::Text::from_units(vec![97, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 116, 97, 114, 103, 101, 116]))
                        }
                        self::JavaScriptSemanticValue::Number(_) => {
                            self::JavaScriptPropertyDefinitionTransition::DefinitionInvalid(self::ml_string::Text::from_units(vec![97, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 116, 97, 114, 103, 101, 116]))
                        }
                        self::JavaScriptSemanticValue::Integer(_) => {
                            self::JavaScriptPropertyDefinitionTransition::DefinitionInvalid(self::ml_string::Text::from_units(vec![97, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 116, 97, 114, 103, 101, 116]))
                        }
                        self::JavaScriptSemanticValue::Text(_) => {
                            self::JavaScriptPropertyDefinitionTransition::DefinitionInvalid(self::ml_string::Text::from_units(vec![97, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 116, 97, 114, 103, 101, 116]))
                        }
                        self::JavaScriptSemanticValue::Symbol(_) => {
                            self::JavaScriptPropertyDefinitionTransition::DefinitionInvalid(self::ml_string::Text::from_units(vec![97, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 116, 97, 114, 103, 101, 116]))
                        }
                        self::JavaScriptSemanticValue::Object(identity, _) => {
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
                        self::JavaScriptSemanticValue::Boolean(_) => {
                            self::JavaScriptPropertyDefinitionTransition::DefinitionIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 100, 101, 102, 105, 110, 101, 80, 114, 111, 112, 101, 114, 116, 121, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 97, 110, 32, 79, 98, 106, 101, 99, 116, 32, 116, 97, 114, 103, 101, 116]))
                        }
                        self::JavaScriptSemanticValue::Number(_) => {
                            self::JavaScriptPropertyDefinitionTransition::DefinitionIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 100, 101, 102, 105, 110, 101, 80, 114, 111, 112, 101, 114, 116, 121, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 97, 110, 32, 79, 98, 106, 101, 99, 116, 32, 116, 97, 114, 103, 101, 116]))
                        }
                        self::JavaScriptSemanticValue::Integer(_) => {
                            self::JavaScriptPropertyDefinitionTransition::DefinitionIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 100, 101, 102, 105, 110, 101, 80, 114, 111, 112, 101, 114, 116, 121, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 97, 110, 32, 79, 98, 106, 101, 99, 116, 32, 116, 97, 114, 103, 101, 116]))
                        }
                        self::JavaScriptSemanticValue::Text(_) => {
                            self::JavaScriptPropertyDefinitionTransition::DefinitionIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 100, 101, 102, 105, 110, 101, 80, 114, 111, 112, 101, 114, 116, 121, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 97, 110, 32, 79, 98, 106, 101, 99, 116, 32, 116, 97, 114, 103, 101, 116]))
                        }
                        self::JavaScriptSemanticValue::Symbol(_) => {
                            self::JavaScriptPropertyDefinitionTransition::DefinitionIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 100, 101, 102, 105, 110, 101, 80, 114, 111, 112, 101, 114, 116, 121, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 97, 110, 32, 79, 98, 106, 101, 99, 116, 32, 116, 97, 114, 103, 101, 116]))
                        }
                        self::JavaScriptSemanticValue::Object(_, _) => {
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
            self::JavaScriptPropertyDefinitionState::PropertyDefinitionKey(source_ml_source2_data_property_value1233, _, _) => {
                let source_ml_source2_data_property_value1233 = *source_ml_source2_data_property_value1233;
                source_ml_source2_data_property_value1233.clone()
            }
            self::JavaScriptPropertyDefinitionState::PropertyDefinitionDescriptor(source_ml_source2_data_property_value1234, _, _) => {
                let source_ml_source2_data_property_value1234 = *source_ml_source2_data_property_value1234;
                source_ml_source2_data_property_value1234.clone()
            }
        });
        if (error != (self::ml_string::Text::from_units(vec![]))) {
            self::JavaScriptPropertyDefinitionTransition::DefinitionInvalid(error.clone())
        } else {
            if {
                let source_ml_source2_s1237 = match state.clone() {
                    self::JavaScriptPropertyDefinitionState::PropertyDefinitionKey(source_ml_source2_data_property_value1245, _, _) => {
                        let source_ml_source2_data_property_value1245 = *source_ml_source2_data_property_value1245;
                        source_ml_source2_data_property_value1245.clone()
                    }
                    self::JavaScriptPropertyDefinitionState::PropertyDefinitionDescriptor(source_ml_source2_data_property_value1246, _, _) => {
                        let source_ml_source2_data_property_value1246 = *source_ml_source2_data_property_value1246;
                        source_ml_source2_data_property_value1246.clone()
                    }
                };
                match source_ml_source2_s1237.clone() {
                    self::JavaScriptSemanticValue::Object(_, _) => {
                        false
                    }
                    _ => {
                        true
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
            self::JavaScriptPropertyDefinitionState::PropertyDefinitionKey(source_ml_source2_data_property_value1247, _, _) => {
                let source_ml_source2_data_property_value1247 = *source_ml_source2_data_property_value1247;
                source_ml_source2_data_property_value1247.clone()
            }
            self::JavaScriptPropertyDefinitionState::PropertyDefinitionDescriptor(source_ml_source2_data_property_value1248, _, _) => {
                let source_ml_source2_data_property_value1248 = *source_ml_source2_data_property_value1248;
                source_ml_source2_data_property_value1248.clone()
            }
        });
        if (error != (self::ml_string::Text::from_units(vec![]))) {
            self::JavaScriptPropertyDefinitionTransition::DefinitionInvalid(error.clone())
        } else {
            if {
                let source_ml_source2_s1251 = match state.clone() {
                    self::JavaScriptPropertyDefinitionState::PropertyDefinitionKey(source_ml_source2_data_property_value1259, _, _) => {
                        let source_ml_source2_data_property_value1259 = *source_ml_source2_data_property_value1259;
                        source_ml_source2_data_property_value1259.clone()
                    }
                    self::JavaScriptPropertyDefinitionState::PropertyDefinitionDescriptor(source_ml_source2_data_property_value1260, _, _) => {
                        let source_ml_source2_data_property_value1260 = *source_ml_source2_data_property_value1260;
                        source_ml_source2_data_property_value1260.clone()
                    }
                };
                match source_ml_source2_s1251.clone() {
                    self::JavaScriptSemanticValue::Object(_, _) => {
                        false
                    }
                    _ => {
                        true
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
pub fn read_java_script_number_protocol_operation_error(operation: &self::ml_string::Text) -> self::ml_string::Text {
    if (((*operation) == (self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 116, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 110, 117, 109, 98, 101, 114]))) || ((*operation) == (self::ml_string::Text::from_units(vec![99, 111, 101, 114, 99, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 110, 117, 109, 98, 101, 114])))) {
        self::ml_string::Text::from_units(vec![])
    } else {
        self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 111, 114, 32, 105, 109, 112, 108, 105, 99, 105, 116, 32, 115, 111, 117, 114, 99, 101, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn finish_java_script_number_protocol(operation: &self::ml_string::Text, transition: self::JavaScriptPrimitiveConversionTransition) -> self::JavaScriptNumberConversionTransition {
    {
        let error = self::read_java_script_number_protocol_operation_error(operation);
        if (error != (self::ml_string::Text::from_units(vec![]))) {
            self::JavaScriptNumberConversionTransition::NumberInvalid(error.clone())
        } else {
            match transition.clone() {
                self::JavaScriptPrimitiveConversionTransition::PrimitiveContinuation(state) => {
                    let state = *state;
                    self::JavaScriptNumberConversionTransition::NumberContinuation(Box::new(state.clone()))
                }
                self::JavaScriptPrimitiveConversionTransition::PrimitiveCall(state_2, callee, receiver, arguments) => {
                    let state_2 = *state_2;
                    let callee = *callee;
                    let receiver = *receiver;
                    self::JavaScriptNumberConversionTransition::NumberCall(Box::new(state_2.clone()), Box::new(callee.clone()), Box::new(receiver.clone()), arguments)
                }
                self::JavaScriptPrimitiveConversionTransition::PrimitiveCompleted(value) => {
                    let value = *value;
                    self::finish_java_script_number_primitive(operation, value.clone())
                }
                self::JavaScriptPrimitiveConversionTransition::PrimitiveThrow(value_2) => {
                    let value_2 = *value_2;
                    self::JavaScriptNumberConversionTransition::NumberThrow(Box::new(value_2.clone()))
                }
                self::JavaScriptPrimitiveConversionTransition::PrimitiveIntrinsicError(error_class, reason) => {
                    self::JavaScriptNumberConversionTransition::NumberIntrinsicError(error_class.clone(), reason.clone())
                }
                self::JavaScriptPrimitiveConversionTransition::PrimitiveInvalid(reason_2) => {
                    self::JavaScriptNumberConversionTransition::NumberInvalid(reason_2.clone())
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn finish_java_script_number_primitive(operation: &self::ml_string::Text, value: self::JavaScriptSemanticValue) -> self::JavaScriptNumberConversionTransition {
    {
        let error = self::read_java_script_number_protocol_operation_error(operation);
        if (error != (self::ml_string::Text::from_units(vec![]))) {
            self::JavaScriptNumberConversionTransition::NumberInvalid(error.clone())
        } else {
            {
                let result = self::execute_java_script_call_primitive(operation, vec![value.clone()]);
                match result.clone() {
                    self::JavaScriptCallPrimitiveResult::PrimitiveValue(value_2) => {
                        let value_2 = *value_2;
                        self::JavaScriptNumberConversionTransition::NumberCompleted(Box::new(value_2.clone()))
                    }
                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(reason) => {
                        self::JavaScriptNumberConversionTransition::NumberInvalid(reason.clone())
                    }
                    self::JavaScriptCallPrimitiveResult::PrimitiveError(error_class, reason_2) => {
                        self::JavaScriptNumberConversionTransition::NumberIntrinsicError(error_class.clone(), reason_2.clone())
                    }
                    self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(reason_3) => {
                        self::JavaScriptNumberConversionTransition::NumberResourceLimit(Box::new(value.clone()), reason_3.clone())
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn begin_java_script_number_protocol(heap: self::JavaScriptOrdinaryHeap, input: self::JavaScriptSemanticValue, exotic_key: self::JavaScriptSemanticValue, operation: &self::ml_string::Text) -> self::JavaScriptNumberConversionTransition {
    {
        let error = self::read_java_script_number_protocol_operation_error(operation);
        if (error != (self::ml_string::Text::from_units(vec![]))) {
            self::JavaScriptNumberConversionTransition::NumberInvalid(error.clone())
        } else {
            self::finish_java_script_number_protocol(operation, self::begin_java_script_primitive_conversion(heap.clone(), input.clone(), exotic_key.clone(), &(self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114]))))
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn step_java_script_number_protocol(heap: self::JavaScriptOrdinaryHeap, state: self::JavaScriptPrimitiveConversionState, operation: &self::ml_string::Text) -> self::JavaScriptNumberConversionTransition {
    {
        let error = self::read_java_script_number_protocol_operation_error(operation);
        if (error != (self::ml_string::Text::from_units(vec![]))) {
            self::JavaScriptNumberConversionTransition::NumberInvalid(error.clone())
        } else {
            if ((match state.clone() {
                self::JavaScriptPrimitiveConversionState::PrimitiveConversion(_, _, source_ml_source2_data_property_value1262, _) => {
                    source_ml_source2_data_property_value1262.clone()
                }
            }) != (self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114]))) {
                self::JavaScriptNumberConversionTransition::NumberInvalid(self::ml_string::Text::from_units(vec![78, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 110, 117, 109, 101, 114, 105, 99, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 104, 105, 110, 116]))
            } else {
                self::finish_java_script_number_protocol(operation, self::step_java_script_primitive_conversion(heap.clone(), state.clone()))
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn resume_java_script_number_protocol(heap: self::JavaScriptOrdinaryHeap, state: self::JavaScriptPrimitiveConversionState, completion: self::JavaScriptPrimitiveConversionCompletion, operation: &self::ml_string::Text) -> self::JavaScriptNumberConversionTransition {
    {
        let error = self::read_java_script_number_protocol_operation_error(operation);
        if (error != (self::ml_string::Text::from_units(vec![]))) {
            self::JavaScriptNumberConversionTransition::NumberInvalid(error.clone())
        } else {
            if ((match state.clone() {
                self::JavaScriptPrimitiveConversionState::PrimitiveConversion(_, _, source_ml_source2_data_property_value1264, _) => {
                    source_ml_source2_data_property_value1264.clone()
                }
            }) != (self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114]))) {
                self::JavaScriptNumberConversionTransition::NumberInvalid(self::ml_string::Text::from_units(vec![78, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 110, 117, 109, 101, 114, 105, 99, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 104, 105, 110, 116]))
            } else {
                self::finish_java_script_number_protocol(operation, self::resume_java_script_primitive_conversion(heap.clone(), state.clone(), completion.clone()))
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
    match program.clone() {
        self::JavaScriptCallProgram::CallProgram(shared_constructor_field1, shared_constructor_field2, shared_constructor_field3) => {
            let shared_constructor_field3 = *shared_constructor_field3;
            {
                let error = self::read_java_script_ordinary_heap_error(heap.clone());
                if (error != (self::ml_string::Text::from_units(vec![]))) {
                    error.clone()
                } else {
                    if ((((shared_constructor_field1.len() as f64)) > (4096f64)) || (((shared_constructor_field2.len() as f64)) > (4096f64))) {
                        self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 99, 111, 100, 101, 32, 100, 105, 114, 101, 99, 116, 111, 114, 105, 101, 115, 32, 101, 120, 99, 101, 101, 100, 32, 116, 104, 101, 105, 114, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 114, 101, 115, 111, 117, 114, 99, 101, 32, 98, 117, 100, 103, 101, 116])
                    } else {
                        {
                            let previous = self::ml::Big::from_i128(0);
                            {
                                let source_ml_source1_temporary_values123 = shared_constructor_field2;
                                {
                                    let source_ml_source1_temporary_index123 = self::ml::Big::from_i128(0);
                                    {
                                        let source_ml_source2_s1270 = self::source_ml_source1_temporary_read_java_script_call_program_error_loop128(previous.clone(), source_ml_source1_temporary_values123, source_ml_source1_temporary_index123.clone());
                                        match source_ml_source2_s1270.clone() {
                                            self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop128Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop128Done(previous_2) => {
                                                if {
                                                    let source_ml_source2_s1272 = shared_constructor_field3.clone();
                                                    match source_ml_source2_s1272.clone() {
                                                        self::JavaScriptSemanticValue::Symbol(_) => {
                                                            false
                                                        }
                                                        _ => {
                                                            true
                                                        }
                                                    }
                                                } {
                                                    self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 99, 111, 100, 101, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 116, 104, 101, 32, 97, 117, 116, 104, 111, 114, 105, 116, 97, 116, 105, 118, 101, 32, 119, 101, 108, 108, 45, 107, 110, 111, 119, 110, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 83, 121, 109, 98, 111, 108])
                                                } else {
                                                    {
                                                        let key_error = self::read_java_script_ordinary_value_error(heap.clone(), shared_constructor_field3.clone());
                                                        if (key_error != (self::ml_string::Text::from_units(vec![]))) {
                                                            key_error.clone()
                                                        } else {
                                                            {
                                                                let previous_3 = self::ml::Big::from_i128(0);
                                                                {
                                                                    let count = 0f64;
                                                                    {
                                                                        let source_ml_source1_temporary_values124 = shared_constructor_field1;
                                                                        {
                                                                            let source_ml_source1_temporary_index124 = self::ml::Big::from_i128(0);
                                                                            {
                                                                                let source_ml_source2_s1283 = self::source_ml_source1_temporary_read_java_script_call_program_error_loop129(program.clone(), heap.clone(), previous_3.clone(), count, source_ml_source1_temporary_values124, source_ml_source1_temporary_index124.clone());
                                                                                match source_ml_source2_s1283.clone() {
                                                                                    self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop129Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop129Done => {
                                                                                        self::ml_string::Text::from_units(vec![])
                                                                                    }
                                                                                    self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop129Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop129Return(source_ml_source1_temporary_result129) => {
                                                                                        source_ml_source1_temporary_result129.clone()
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
                                            self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop128Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop128Return(source_ml_source1_temporary_result132) => {
                                                source_ml_source1_temporary_result132.clone()
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
pub fn replace_java_script_call_frame(heap: self::JavaScriptOrdinaryHeap, frames: Vec<self::JavaScriptCallFrame>, frame: self::JavaScriptCallFrame) -> self::JavaScriptCallMachine {
    self::JavaScriptCallMachine::CallMachine(Box::new(heap.clone()), { let ml_values3 = frames.clone(); let ml_callback3 = { let frames = frames.clone(); let frame = frame.clone(); std::rc::Rc::new(move |entry: self::JavaScriptCallFrame, index: f64| -> self::JavaScriptCallFrame { if (index == (((frames.len() as f64) - 1f64))) {
        frame.clone()
    } else {
        entry.clone()
    } }) }; ml_values3.iter().cloned().enumerate().map(|(ml_index3, ml_value3)| ml_callback3(ml_value3, (ml_index3 as f64))).collect::<Vec<_>>() })
}
