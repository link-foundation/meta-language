// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptCallInstruction;
use super::JavaScriptCallPrimitiveResult;
use super::JavaScriptCallProgram;
use super::JavaScriptDescriptorConversionState;
use super::JavaScriptDescriptorConversionTransition;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptPrimitiveConversionCompletion;
use super::JavaScriptPrimitiveConversionState;
use super::JavaScriptPrimitiveConversionTransition;
use super::JavaScriptScalarConversionTransition;
use super::JavaScriptSemanticCompletion;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryFindJavaScriptCallCodeFunctionLoop125Result;
use super::MlSource1TemporaryReadJavaScriptDescriptorConversionStateErrorLoop126Result;
use super::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop127Result;
use super::begin_java_script_primitive_conversion;
use super::execute_java_script_call_primitive;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::ml_number;
use super::ml_source_exception;
use super::ml_string;
use super::read_java_script_ordinary_heap_error;
use super::read_java_script_ordinary_value_error;
use super::read_java_script_property_descriptor_error;
use super::read_java_script_scalar_protocol_operation_error;
use super::read_java_script_semantic_value_error;
use super::resume_java_script_primitive_conversion;
use super::source_ml_source1_temporary_find_java_script_call_code_function_loop125;
use super::source_ml_source1_temporary_read_java_script_descriptor_conversion_state_error_loop126;
use super::source_ml_source1_temporary_step_java_script_descriptor_conversion_loop127;
use super::step_java_script_primitive_conversion;
use super::test_java_script_semantic_accessor;
use super::test_java_script_semantic_same_value;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn finish_java_script_scalar_protocol(operation: &self::ml_string::Text, transition: self::JavaScriptPrimitiveConversionTransition) -> self::JavaScriptScalarConversionTransition {
    {
        let error = self::read_java_script_scalar_protocol_operation_error(operation);
        if (error != (self::ml_string::Text::from_units(vec![]))) {
            self::JavaScriptScalarConversionTransition::ScalarInvalid(error.clone())
        } else {
            match transition.clone() {
                self::JavaScriptPrimitiveConversionTransition::PrimitiveContinuation(state) => {
                    let state = *state;
                    self::JavaScriptScalarConversionTransition::ScalarContinuation(Box::new(state.clone()))
                }
                self::JavaScriptPrimitiveConversionTransition::PrimitiveCall(state_2, callee, receiver, arguments) => {
                    let state_2 = *state_2;
                    let callee = *callee;
                    let receiver = *receiver;
                    self::JavaScriptScalarConversionTransition::ScalarCall(Box::new(state_2.clone()), Box::new(callee.clone()), Box::new(receiver.clone()), arguments)
                }
                self::JavaScriptPrimitiveConversionTransition::PrimitiveCompleted(value) => {
                    let value = *value;
                    self::finish_java_script_scalar_primitive(operation, value.clone())
                }
                self::JavaScriptPrimitiveConversionTransition::PrimitiveThrow(value_2) => {
                    let value_2 = *value_2;
                    self::JavaScriptScalarConversionTransition::ScalarThrow(Box::new(value_2.clone()))
                }
                self::JavaScriptPrimitiveConversionTransition::PrimitiveIntrinsicError(error_class, reason) => {
                    self::JavaScriptScalarConversionTransition::ScalarIntrinsicError(error_class.clone(), reason.clone())
                }
                self::JavaScriptPrimitiveConversionTransition::PrimitiveInvalid(reason_2) => {
                    self::JavaScriptScalarConversionTransition::ScalarInvalid(reason_2.clone())
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn finish_java_script_scalar_primitive(operation: &self::ml_string::Text, value: self::JavaScriptSemanticValue) -> self::JavaScriptScalarConversionTransition {
    {
        let error = self::read_java_script_scalar_protocol_operation_error(operation);
        if (error != (self::ml_string::Text::from_units(vec![]))) {
            self::JavaScriptScalarConversionTransition::ScalarInvalid(error.clone())
        } else {
            {
                let result = self::execute_java_script_call_primitive(operation, vec![value.clone()]);
                match result.clone() {
                    self::JavaScriptCallPrimitiveResult::PrimitiveValue(value_2) => {
                        let value_2 = *value_2;
                        self::JavaScriptScalarConversionTransition::ScalarCompleted(Box::new(value_2.clone()))
                    }
                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(reason) => {
                        self::JavaScriptScalarConversionTransition::ScalarInvalid(reason.clone())
                    }
                    self::JavaScriptCallPrimitiveResult::PrimitiveError(error_class, reason_2) => {
                        self::JavaScriptScalarConversionTransition::ScalarIntrinsicError(error_class.clone(), reason_2.clone())
                    }
                    self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(reason_3) => {
                        self::JavaScriptScalarConversionTransition::ScalarResourceLimit(Box::new(value.clone()), reason_3.clone())
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn begin_java_script_scalar_protocol(heap: self::JavaScriptOrdinaryHeap, input: self::JavaScriptSemanticValue, exotic_key: self::JavaScriptSemanticValue, operation: &self::ml_string::Text) -> self::JavaScriptScalarConversionTransition {
    {
        let error = self::read_java_script_scalar_protocol_operation_error(operation);
        if (error != (self::ml_string::Text::from_units(vec![]))) {
            self::JavaScriptScalarConversionTransition::ScalarInvalid(error.clone())
        } else {
            self::finish_java_script_scalar_protocol(operation, self::begin_java_script_primitive_conversion(heap.clone(), input.clone(), exotic_key.clone(), &(if ((*operation) == (self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 116, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 116, 101, 120, 116]))) {
                self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103])
            } else {
                self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114])
            })))
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn step_java_script_scalar_protocol(heap: self::JavaScriptOrdinaryHeap, state: self::JavaScriptPrimitiveConversionState, operation: &self::ml_string::Text) -> self::JavaScriptScalarConversionTransition {
    {
        let error = self::read_java_script_scalar_protocol_operation_error(operation);
        if (error != (self::ml_string::Text::from_units(vec![]))) {
            self::JavaScriptScalarConversionTransition::ScalarInvalid(error.clone())
        } else {
            if ((match state.clone() {
                self::JavaScriptPrimitiveConversionState::PrimitiveConversion(_, _, source_ml_source2_data_property_value1064, _) => {
                    source_ml_source2_data_property_value1064.clone()
                }
            }) != (if ((*operation) == (self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 116, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 116, 101, 120, 116]))) {
                self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103])
            } else {
                self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114])
            })) {
                self::JavaScriptScalarConversionTransition::ScalarInvalid(self::ml_string::Text::from_units(vec![115, 99, 97, 108, 97, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 116, 104, 101, 32, 104, 105, 110, 116, 32, 111, 102, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110]))
            } else {
                self::finish_java_script_scalar_protocol(operation, self::step_java_script_primitive_conversion(heap.clone(), state.clone()))
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn resume_java_script_scalar_protocol(heap: self::JavaScriptOrdinaryHeap, state: self::JavaScriptPrimitiveConversionState, completion: self::JavaScriptPrimitiveConversionCompletion, operation: &self::ml_string::Text) -> self::JavaScriptScalarConversionTransition {
    {
        let error = self::read_java_script_scalar_protocol_operation_error(operation);
        if (error != (self::ml_string::Text::from_units(vec![]))) {
            self::JavaScriptScalarConversionTransition::ScalarInvalid(error.clone())
        } else {
            if ((match state.clone() {
                self::JavaScriptPrimitiveConversionState::PrimitiveConversion(_, _, source_ml_source2_data_property_value1066, _) => {
                    source_ml_source2_data_property_value1066.clone()
                }
            }) != (if ((*operation) == (self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 116, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 116, 101, 120, 116]))) {
                self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103])
            } else {
                self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114])
            })) {
                self::JavaScriptScalarConversionTransition::ScalarInvalid(self::ml_string::Text::from_units(vec![115, 99, 97, 108, 97, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 116, 104, 101, 32, 104, 105, 110, 116, 32, 111, 102, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110]))
            } else {
                self::finish_java_script_scalar_protocol(operation, self::resume_java_script_primitive_conversion(heap.clone(), state.clone(), completion.clone()))
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_text_protocol_operation_error(operation: &self::ml_string::Text) -> self::ml_string::Text {
    if ((*operation) == (self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 116, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 116, 101, 120, 116]))) {
        self::ml_string::Text::from_units(vec![])
    } else {
        self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 115, 111, 117, 114, 99, 101, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_call_code_coordinate(value: f64, limit: f64) -> bool {
    (((self::ml_math::is_safe_integer(value) && (value >= (0f64))) && (value < limit)) && ((value != (0f64)) || (((1f64 / value)) > (0f64))))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn find_java_script_call_code_function(program: self::JavaScriptCallProgram, identity: self::ml::Big) -> f64 {
    {
        let low = 0f64;
        {
            let high = ((match &program { self::JavaScriptCallProgram::CallProgram(source_ml_source2_data_property_value1067, _, _) => source_ml_source2_data_property_value1067.as_slice() }).len() as f64);
            {
                let source_ml_source2_s1068 = self::source_ml_source1_temporary_find_java_script_call_code_function_loop125(program.clone(), identity.clone(), low, high);
                match source_ml_source2_s1068.clone() {
                    self::MlSource1TemporaryFindJavaScriptCallCodeFunctionLoop125Result::MlSource1TemporaryFindJavaScriptCallCodeFunctionLoop125Done => {
                        (-1f64)
                    }
                    self::MlSource1TemporaryFindJavaScriptCallCodeFunctionLoop125Result::MlSource1TemporaryFindJavaScriptCallCodeFunctionLoop125Return(source_ml_source1_temporary_result125) => {
                        source_ml_source1_temporary_result125
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_call_code_registers(instruction: self::JavaScriptCallInstruction) -> Vec<f64> {
    match instruction.clone() {
        self::JavaScriptCallInstruction::Constant(destination, _) => {
            vec![destination]
        }
        self::JavaScriptCallInstruction::Copy(destination_2, source) => {
            vec![destination_2, source]
        }
        self::JavaScriptCallInstruction::Argument(destination_3, _) => {
            vec![destination_3]
        }
        self::JavaScriptCallInstruction::Jump(_) => {
            Vec::<f64>::new()
        }
        self::JavaScriptCallInstruction::Branch(condition, _, _) => {
            vec![condition]
        }
        self::JavaScriptCallInstruction::Call(destination_4, callee, receiver, arguments) => {
            {
                let source_ml_source2_concat_value1079 = vec![destination_4, callee, receiver];
                self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value1079), arguments)
            }
        }
        self::JavaScriptCallInstruction::Return(source_2) => {
            vec![source_2]
        }
        self::JavaScriptCallInstruction::Throw(source_3) => {
            vec![source_3]
        }
        self::JavaScriptCallInstruction::PushHandler(destination_5, _) => {
            vec![destination_5]
        }
        self::JavaScriptCallInstruction::PopHandler => {
            Vec::<f64>::new()
        }
        self::JavaScriptCallInstruction::NumberBinary(destination_6, left, right, _) => {
            vec![destination_6, left, right]
        }
        self::JavaScriptCallInstruction::SameValue(destination_7, left_2, right_2) => {
            vec![destination_7, left_2, right_2]
        }
        self::JavaScriptCallInstruction::StrictEqual(destination_8, left_3, right_3) => {
            vec![destination_8, left_3, right_3]
        }
        self::JavaScriptCallInstruction::RequireWrite(success) => {
            vec![success]
        }
        self::JavaScriptCallInstruction::Primitive(destination_9, _, arguments_2) => {
            {
                let source_ml_source2_concat_value1081 = vec![destination_9];
                self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value1081), arguments_2)
            }
        }
        self::JavaScriptCallInstruction::OrdinaryGet(destination_10, object, key, receiver_2) => {
            vec![destination_10, object, key, receiver_2]
        }
        self::JavaScriptCallInstruction::OrdinarySet(destination_11, object_2, key_2, value, receiver_3) => {
            vec![destination_11, object_2, key_2, value, receiver_3]
        }
        self::JavaScriptCallInstruction::PropertyKey(destination_12, input, _) => {
            vec![destination_12, input]
        }
        self::JavaScriptCallInstruction::PropertyDefinition(destination_13, target, key_3, attributes, _) => {
            vec![destination_13, target, key_3, attributes]
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_call_strict_equal(left: self::JavaScriptSemanticValue, right: self::JavaScriptSemanticValue) -> bool {
    if (((self::read_java_script_semantic_value_error(left.clone())) != (self::ml_string::Text::from_units(vec![]))) || ((self::read_java_script_semantic_value_error(right.clone())) != (self::ml_string::Text::from_units(vec![])))) {
        (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 99, 116, 32, 101, 113, 117, 97, 108, 105, 116, 121, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 115, 101, 109, 97, 110, 116, 105, 99, 32, 111, 112, 101, 114, 97, 110, 100, 115])).0, message_present: true }).raise()
    } else {
        match left.clone() {
            self::JavaScriptSemanticValue::Undefined => {
                self::test_java_script_semantic_same_value(left.clone(), right.clone())
            }
            self::JavaScriptSemanticValue::Null => {
                self::test_java_script_semantic_same_value(left.clone(), right.clone())
            }
            self::JavaScriptSemanticValue::Boolean(_) => {
                self::test_java_script_semantic_same_value(left.clone(), right.clone())
            }
            self::JavaScriptSemanticValue::Number(left_value) => {
                match right.clone() {
                    self::JavaScriptSemanticValue::Number(value) => {
                        (left_value == value)
                    }
                    _ => {
                        false
                    }
                }
            }
            self::JavaScriptSemanticValue::Integer(_) => {
                self::test_java_script_semantic_same_value(left.clone(), right.clone())
            }
            self::JavaScriptSemanticValue::Text(_) => {
                self::test_java_script_semantic_same_value(left.clone(), right.clone())
            }
            self::JavaScriptSemanticValue::Symbol(_) => {
                self::test_java_script_semantic_same_value(left.clone(), right.clone())
            }
            self::JavaScriptSemanticValue::Object(_, _) => {
                self::test_java_script_semantic_same_value(left.clone(), right.clone())
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_descriptor_field(index: f64) -> (self::ml_string::Text, self::ml_string::Text, Vec<f64>) {
    if (index == (0f64)) {
        {
            let source_ml_source2_field1097 = vec![101f64, 110f64, 117f64, 109f64, 101f64, 114f64, 97f64, 98f64, 108f64, 101f64];
            (self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110]), self::ml_string::Text::from_units(vec![101, 110, 117, 109, 101, 114, 97, 98, 108, 101]), source_ml_source2_field1097)
        }
    } else {
        if (index == (1f64)) {
            {
                let source_ml_source2_field1100 = vec![99f64, 111f64, 110f64, 102f64, 105f64, 103f64, 117f64, 114f64, 97f64, 98f64, 108f64, 101f64];
                (self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110]), self::ml_string::Text::from_units(vec![99, 111, 110, 102, 105, 103, 117, 114, 97, 98, 108, 101]), source_ml_source2_field1100)
            }
        } else {
            if (index == (2f64)) {
                {
                    let source_ml_source2_field1103 = vec![118f64, 97f64, 108f64, 117f64, 101f64];
                    (self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101]), self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101]), source_ml_source2_field1103)
                }
            } else {
                if (index == (3f64)) {
                    {
                        let source_ml_source2_field1106 = vec![119f64, 114f64, 105f64, 116f64, 97f64, 98f64, 108f64, 101f64];
                        (self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110]), self::ml_string::Text::from_units(vec![119, 114, 105, 116, 97, 98, 108, 101]), source_ml_source2_field1106)
                    }
                } else {
                    if (index == (4f64)) {
                        {
                            let source_ml_source2_field1109 = vec![103f64, 101f64, 116f64];
                            (self::ml_string::Text::from_units(vec![97, 99, 99, 101, 115, 115, 111, 114]), self::ml_string::Text::from_units(vec![103, 101, 116]), source_ml_source2_field1109)
                        }
                    } else {
                        if (index == (5f64)) {
                            {
                                let source_ml_source2_field1112 = vec![115f64, 101f64, 116f64];
                                (self::ml_string::Text::from_units(vec![97, 99, 99, 101, 115, 115, 111, 114]), self::ml_string::Text::from_units(vec![115, 101, 116]), source_ml_source2_field1112)
                            }
                        } else {
                            {
                                let source_ml_source2_field1115 = Vec::<f64>::new();
                                (self::ml_string::Text::from_units(vec![105, 110, 118, 97, 108, 105, 100]), self::ml_string::Text::from_units(vec![]), source_ml_source2_field1115)
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
pub fn convert_java_script_semantic_boolean(value: self::JavaScriptSemanticValue) -> bool {
    if ((self::read_java_script_semantic_value_error(value.clone())) != (self::ml_string::Text::from_units(vec![]))) {
        (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![84, 111, 66, 111, 111, 108, 101, 97, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 97, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 115, 101, 109, 97, 110, 116, 105, 99, 32, 118, 97, 108, 117, 101])).0, message_present: true }).raise()
    } else {
        match value.clone() {
            self::JavaScriptSemanticValue::Boolean(value_value) => {
                value_value
            }
            self::JavaScriptSemanticValue::Number(value_value_2) => {
                ((value_value_2 != (0f64)) && !(value_value_2.is_nan()))
            }
            self::JavaScriptSemanticValue::Integer(value_value_3) => {
                (value_value_3 != (self::ml::Big::from_i128(0)))
            }
            self::JavaScriptSemanticValue::Text(units) => {
                (((units.len() as f64)) != (0f64))
            }
            self::JavaScriptSemanticValue::Symbol(_) => {
                true
            }
            self::JavaScriptSemanticValue::Object(_, _) => {
                true
            }
            _ => {
                false
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn create_java_script_empty_descriptor() -> (bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool) {
    {
        let source_ml_source2_field1124 = self::JavaScriptSemanticValue::Undefined;
        {
            let source_ml_source2_field1128 = self::JavaScriptSemanticValue::Undefined;
            {
                let source_ml_source2_field1130 = self::JavaScriptSemanticValue::Undefined;
                (false, false, false, false, source_ml_source2_field1128.clone(), false, source_ml_source2_field1130.clone(), false, source_ml_source2_field1124.clone(), false, false, false)
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_descriptor_field_presence(descriptor: (bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool), index: f64) -> bool {
    if (index == (0f64)) {
        (descriptor.clone()).3
    } else {
        if (index == (1f64)) {
            (descriptor.clone()).1
        } else {
            if (index == (2f64)) {
                (descriptor.clone()).9
            } else {
                if (index == (3f64)) {
                    (descriptor.clone()).11
                } else {
                    if (index == (4f64)) {
                        (descriptor.clone()).5
                    } else {
                        if (index == (5f64)) {
                            (descriptor.clone()).7
                        } else {
                            false
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_descriptor_conversion_state_error(heap: self::JavaScriptOrdinaryHeap, state: self::JavaScriptDescriptorConversionState) -> self::ml_string::Text {
    match state.clone() {
        self::JavaScriptDescriptorConversionState::DescriptorConversion(shared_constructor_field1, shared_constructor_field2, shared_constructor_field3, shared_constructor_field4) => {
            let shared_constructor_field1 = *shared_constructor_field1;
            {
                let heap_error = self::read_java_script_ordinary_heap_error(heap.clone());
                if (heap_error != (self::ml_string::Text::from_units(vec![]))) {
                    heap_error.clone()
                } else {
                    if {
                        let source_ml_source2_s1132 = shared_constructor_field1.clone();
                        match source_ml_source2_s1132.clone() {
                            self::JavaScriptSemanticValue::Object(_, _) => {
                                false
                            }
                            _ => {
                                true
                            }
                        }
                    } {
                        self::ml_string::Text::from_units(vec![100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 114, 101, 99, 101, 105, 118, 101, 114])
                    } else {
                        {
                            let receiver_error = self::read_java_script_ordinary_value_error(heap.clone(), shared_constructor_field1.clone());
                            if (receiver_error != (self::ml_string::Text::from_units(vec![]))) {
                                receiver_error.clone()
                            } else {
                                if ((((!(self::ml_math::is_safe_integer(shared_constructor_field2)) || (shared_constructor_field2 < (0f64))) || (shared_constructor_field2 > (6f64))) || ((shared_constructor_field2 == (0f64)) && (((1f64 / shared_constructor_field2)) < (0f64)))) || (shared_constructor_field4 && (shared_constructor_field2 == (6f64)))) {
                                    self::ml_string::Text::from_units(vec![100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 101, 120, 97, 99, 116, 32, 111, 114, 100, 101, 114, 101, 100, 32, 102, 105, 101, 108, 100, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110])
                                } else {
                                    {
                                        let index = shared_constructor_field2;
                                        {
                                            let source_ml_source2_s1155 = self::source_ml_source1_temporary_read_java_script_descriptor_conversion_state_error_loop126(state.clone(), index);
                                            match source_ml_source2_s1155.clone() {
                                                self::MlSource1TemporaryReadJavaScriptDescriptorConversionStateErrorLoop126Result::MlSource1TemporaryReadJavaScriptDescriptorConversionStateErrorLoop126Done => {
                                                    {
                                                        let descriptor = shared_constructor_field3.clone();
                                                        if ((descriptor.clone()).9 && ((self::read_java_script_ordinary_value_error(heap.clone(), (descriptor.clone()).8)) != (self::ml_string::Text::from_units(vec![])))) {
                                                            self::ml_string::Text::from_units(vec![100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 118, 97, 108, 117, 101, 115, 32, 98, 101, 108, 111, 110, 103, 32, 116, 111, 32, 116, 104, 101, 32, 97, 99, 116, 117, 97, 108, 32, 108, 97, 116, 101, 115, 116, 32, 103, 108, 111, 98, 97, 108, 32, 99, 111, 110, 116, 101, 120, 116])
                                                        } else {
                                                            if ((descriptor.clone()).5 && (((self::read_java_script_ordinary_value_error(heap.clone(), (descriptor.clone()).4)) != (self::ml_string::Text::from_units(vec![]))) || !(self::test_java_script_semantic_accessor((descriptor.clone()).4)))) {
                                                                self::ml_string::Text::from_units(vec![97, 32, 112, 114, 111, 99, 101, 115, 115, 101, 100, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 103, 101, 116, 116, 101, 114, 32, 114, 101, 116, 97, 105, 110, 115, 32, 97, 99, 116, 117, 97, 108, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 99, 97, 108, 108, 97, 98, 105, 108, 105, 116, 121])
                                                            } else {
                                                                if ((descriptor.clone()).7 && (((self::read_java_script_ordinary_value_error(heap.clone(), (descriptor.clone()).6)) != (self::ml_string::Text::from_units(vec![]))) || !(self::test_java_script_semantic_accessor((descriptor.clone()).6)))) {
                                                                    self::ml_string::Text::from_units(vec![97, 32, 112, 114, 111, 99, 101, 115, 115, 101, 100, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 115, 101, 116, 116, 101, 114, 32, 114, 101, 116, 97, 105, 110, 115, 32, 97, 99, 116, 117, 97, 108, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 99, 97, 108, 108, 97, 98, 105, 108, 105, 116, 121])
                                                                } else {
                                                                    self::ml_string::Text::from_units(vec![])
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                self::MlSource1TemporaryReadJavaScriptDescriptorConversionStateErrorLoop126Result::MlSource1TemporaryReadJavaScriptDescriptorConversionStateErrorLoop126Return(source_ml_source1_temporary_result126) => {
                                                    source_ml_source1_temporary_result126.clone()
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
pub fn apply_java_script_descriptor_field(state: self::JavaScriptDescriptorConversionState, value: self::JavaScriptSemanticValue) -> self::JavaScriptDescriptorConversionTransition {
    match state.clone() {
        self::JavaScriptDescriptorConversionState::DescriptorConversion(shared_constructor_field1, shared_constructor_field2, shared_constructor_field3, _) => {
            let shared_constructor_field1 = *shared_constructor_field1;
            if (((shared_constructor_field2 == (4f64)) || (shared_constructor_field2 == (5f64))) && !(self::test_java_script_semantic_accessor(value.clone()))) {
                self::JavaScriptDescriptorConversionTransition::DescriptorIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![97, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 97, 99, 99, 101, 115, 115, 111, 114, 32, 109, 117, 115, 116, 32, 98, 101, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 111, 114, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100]))
            } else {
                {
                    let before = shared_constructor_field3.clone();
                    {
                        let enumerable = (shared_constructor_field2 == (0f64));
                        {
                            let configurable = (shared_constructor_field2 == (1f64));
                            {
                                let data_value = (shared_constructor_field2 == (2f64));
                                {
                                    let writable = (shared_constructor_field2 == (3f64));
                                    {
                                        let getter = (shared_constructor_field2 == (4f64));
                                        {
                                            let setter = !(((((enumerable || configurable) || data_value) || writable) || getter));
                                            {
                                                let flag = if ((enumerable || configurable) || writable) {
                                                    self::convert_java_script_semantic_boolean(value.clone())
                                                } else {
                                                    false
                                                };
                                                {
                                                    let descriptor = {
                                                        let source_ml_source2_spread_field1173 = (before.clone()).0;
                                                        {
                                                            let source_ml_source2_spread_field1174 = (before.clone()).1;
                                                            {
                                                                let source_ml_source2_spread_field1175 = (before.clone()).2;
                                                                {
                                                                    let source_ml_source2_spread_field1176 = (before.clone()).3;
                                                                    {
                                                                        let source_ml_source2_spread_field1177 = (before.clone()).4;
                                                                        {
                                                                            let source_ml_source2_spread_field1178 = (before.clone()).5;
                                                                            {
                                                                                let source_ml_source2_spread_field1179 = (before.clone()).6;
                                                                                {
                                                                                    let source_ml_source2_spread_field1180 = (before.clone()).7;
                                                                                    {
                                                                                        let source_ml_source2_spread_field1181 = (before.clone()).8;
                                                                                        {
                                                                                            let source_ml_source2_spread_field1182 = (before.clone()).9;
                                                                                            {
                                                                                                let source_ml_source2_spread_field1183 = (before.clone()).10;
                                                                                                {
                                                                                                    let source_ml_source2_spread_field1184 = (before.clone()).11;
                                                                                                    {
                                                                                                        let source_ml_source2_field1185 = (enumerable || (before.clone()).3);
                                                                                                        {
                                                                                                            let source_ml_source2_field1186 = if enumerable {
                                                                                                                flag
                                                                                                            } else {
                                                                                                                (before.clone()).2
                                                                                                            };
                                                                                                            {
                                                                                                                let source_ml_source2_field1187 = (configurable || (before.clone()).1);
                                                                                                                {
                                                                                                                    let source_ml_source2_field1188 = if configurable {
                                                                                                                        flag
                                                                                                                    } else {
                                                                                                                        (before.clone()).0
                                                                                                                    };
                                                                                                                    {
                                                                                                                        let source_ml_source2_field1189 = (data_value || (before.clone()).9);
                                                                                                                        {
                                                                                                                            let source_ml_source2_field1190 = if data_value {
                                                                                                                                value.clone()
                                                                                                                            } else {
                                                                                                                                (before.clone()).8
                                                                                                                            };
                                                                                                                            {
                                                                                                                                let source_ml_source2_field1191 = (writable || (before.clone()).11);
                                                                                                                                {
                                                                                                                                    let source_ml_source2_field1192 = if writable {
                                                                                                                                        flag
                                                                                                                                    } else {
                                                                                                                                        (before.clone()).10
                                                                                                                                    };
                                                                                                                                    {
                                                                                                                                        let source_ml_source2_field1193 = (getter || (before.clone()).5);
                                                                                                                                        {
                                                                                                                                            let source_ml_source2_field1194 = if getter {
                                                                                                                                                value.clone()
                                                                                                                                            } else {
                                                                                                                                                (before.clone()).4
                                                                                                                                            };
                                                                                                                                            {
                                                                                                                                                let source_ml_source2_field1195 = (setter || (before.clone()).7);
                                                                                                                                                {
                                                                                                                                                    let source_ml_source2_field1196 = if setter {
                                                                                                                                                        value.clone()
                                                                                                                                                    } else {
                                                                                                                                                        (before.clone()).6
                                                                                                                                                    };
                                                                                                                                                    (source_ml_source2_field1188, source_ml_source2_field1187, source_ml_source2_field1186, source_ml_source2_field1185, source_ml_source2_field1194.clone(), source_ml_source2_field1193, source_ml_source2_field1196.clone(), source_ml_source2_field1195, source_ml_source2_field1190.clone(), source_ml_source2_field1189, source_ml_source2_field1192, source_ml_source2_field1191)
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
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    };
                                                    self::JavaScriptDescriptorConversionTransition::DescriptorContinuation(Box::new(self::JavaScriptDescriptorConversionState::DescriptorConversion(Box::new(shared_constructor_field1.clone()), (shared_constructor_field2 + 1f64), descriptor.clone(), false)))
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
pub fn step_java_script_descriptor_conversion(heap: self::JavaScriptOrdinaryHeap, state: self::JavaScriptDescriptorConversionState) -> self::JavaScriptDescriptorConversionTransition {
    {
        let error = self::read_java_script_descriptor_conversion_state_error(heap.clone(), state.clone());
        if (error != (self::ml_string::Text::from_units(vec![]))) {
            self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(error.clone())
        } else {
            if match state.clone() {
                self::JavaScriptDescriptorConversionState::DescriptorConversion(_, _, _, source_ml_source2_data_property_value1199) => {
                    source_ml_source2_data_property_value1199
                }
            } {
                self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(self::ml_string::Text::from_units(vec![97, 32, 112, 101, 110, 100, 105, 110, 103, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 71, 101, 116, 32, 109, 117, 115, 116, 32, 98, 101, 32, 99, 111, 109, 112, 108, 101, 116, 101, 100, 32, 98, 101, 102, 111, 114, 101, 32, 105, 116, 115, 32, 110, 101, 120, 116, 32, 115, 116, 101, 112]))
            } else {
                {
                    let current = state.clone();
                    {
                        let source_ml_source2_s1200 = self::source_ml_source1_temporary_step_java_script_descriptor_conversion_loop127(heap.clone(), current.clone());
                        match source_ml_source2_s1200.clone() {
                            self::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop127Result::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop127Done(current_2) => {
                                let current_2 = *current_2;
                                match current_2.clone() {
                                    self::JavaScriptDescriptorConversionState::DescriptorConversion(_, _, shared_constructor_field1, _) => {
                                        if ((self::read_java_script_property_descriptor_error(shared_constructor_field1.clone())) != (self::ml_string::Text::from_units(vec![]))) {
                                            self::JavaScriptDescriptorConversionTransition::DescriptorIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![97, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 99, 97, 110, 110, 111, 116, 32, 99, 111, 110, 116, 97, 105, 110, 32, 98, 111, 116, 104, 32, 100, 97, 116, 97, 32, 97, 110, 100, 32, 97, 99, 99, 101, 115, 115, 111, 114, 32, 97, 116, 116, 114, 105, 98, 117, 116, 101, 115]))
                                        } else {
                                            self::JavaScriptDescriptorConversionTransition::DescriptorCompleted(shared_constructor_field1.clone())
                                        }
                                    }
                                }
                            }
                            self::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop127Result::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop127Return(source_ml_source1_temporary_result127) => {
                                let source_ml_source1_temporary_result127 = *source_ml_source1_temporary_result127;
                                source_ml_source1_temporary_result127.clone()
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
pub fn begin_java_script_descriptor_conversion(heap: self::JavaScriptOrdinaryHeap, receiver: self::JavaScriptSemanticValue) -> self::JavaScriptDescriptorConversionTransition {
    {
        let heap_error = self::read_java_script_ordinary_heap_error(heap.clone());
        if (heap_error != (self::ml_string::Text::from_units(vec![]))) {
            self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(heap_error.clone())
        } else {
            {
                let error = self::read_java_script_ordinary_value_error(heap.clone(), receiver.clone());
                if (error != (self::ml_string::Text::from_units(vec![]))) {
                    self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(error.clone())
                } else {
                    match receiver.clone() {
                        self::JavaScriptSemanticValue::Undefined => {
                            self::JavaScriptDescriptorConversionTransition::DescriptorIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![97, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 115, 111, 117, 114, 99, 101, 32, 109, 117, 115, 116, 32, 98, 101, 32, 97, 110, 32, 79, 98, 106, 101, 99, 116]))
                        }
                        self::JavaScriptSemanticValue::Null => {
                            self::JavaScriptDescriptorConversionTransition::DescriptorIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![97, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 115, 111, 117, 114, 99, 101, 32, 109, 117, 115, 116, 32, 98, 101, 32, 97, 110, 32, 79, 98, 106, 101, 99, 116]))
                        }
                        self::JavaScriptSemanticValue::Boolean(_) => {
                            self::JavaScriptDescriptorConversionTransition::DescriptorIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![97, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 115, 111, 117, 114, 99, 101, 32, 109, 117, 115, 116, 32, 98, 101, 32, 97, 110, 32, 79, 98, 106, 101, 99, 116]))
                        }
                        self::JavaScriptSemanticValue::Number(_) => {
                            self::JavaScriptDescriptorConversionTransition::DescriptorIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![97, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 115, 111, 117, 114, 99, 101, 32, 109, 117, 115, 116, 32, 98, 101, 32, 97, 110, 32, 79, 98, 106, 101, 99, 116]))
                        }
                        self::JavaScriptSemanticValue::Integer(_) => {
                            self::JavaScriptDescriptorConversionTransition::DescriptorIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![97, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 115, 111, 117, 114, 99, 101, 32, 109, 117, 115, 116, 32, 98, 101, 32, 97, 110, 32, 79, 98, 106, 101, 99, 116]))
                        }
                        self::JavaScriptSemanticValue::Text(_) => {
                            self::JavaScriptDescriptorConversionTransition::DescriptorIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![97, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 115, 111, 117, 114, 99, 101, 32, 109, 117, 115, 116, 32, 98, 101, 32, 97, 110, 32, 79, 98, 106, 101, 99, 116]))
                        }
                        self::JavaScriptSemanticValue::Symbol(_) => {
                            self::JavaScriptDescriptorConversionTransition::DescriptorIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![97, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 115, 111, 117, 114, 99, 101, 32, 109, 117, 115, 116, 32, 98, 101, 32, 97, 110, 32, 79, 98, 106, 101, 99, 116]))
                        }
                        self::JavaScriptSemanticValue::Object(_, _) => {
                            self::step_java_script_descriptor_conversion(heap.clone(), self::JavaScriptDescriptorConversionState::DescriptorConversion(Box::new(receiver.clone()), 0f64, self::create_java_script_empty_descriptor(), false))
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn resume_java_script_descriptor_conversion(heap: self::JavaScriptOrdinaryHeap, state: self::JavaScriptDescriptorConversionState, completion: self::JavaScriptSemanticCompletion) -> self::JavaScriptDescriptorConversionTransition {
    {
        let error = self::read_java_script_descriptor_conversion_state_error(heap.clone(), state.clone());
        if (error != (self::ml_string::Text::from_units(vec![]))) {
            self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(error.clone())
        } else {
            if !(match state.clone() {
                self::JavaScriptDescriptorConversionState::DescriptorConversion(_, _, _, source_ml_source2_data_property_value1211) => {
                    source_ml_source2_data_property_value1211
                }
            }) {
                self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(self::ml_string::Text::from_units(vec![100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 114, 101, 115, 117, 109, 112, 116, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 101, 110, 100, 105, 110, 103, 32, 71, 101, 116]))
            } else {
                {
                    let value_error = self::read_java_script_ordinary_value_error(heap.clone(), match completion.clone() {
                        self::JavaScriptSemanticCompletion::NormalCompletion(source_ml_source2_data_property_value1212) => {
                            let source_ml_source2_data_property_value1212 = *source_ml_source2_data_property_value1212;
                            source_ml_source2_data_property_value1212.clone()
                        }
                        self::JavaScriptSemanticCompletion::ThrowCompletion(source_ml_source2_data_property_value1213) => {
                            let source_ml_source2_data_property_value1213 = *source_ml_source2_data_property_value1213;
                            source_ml_source2_data_property_value1213.clone()
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
                                        self::JavaScriptDescriptorConversionTransition::DescriptorCall(_, _, _, _) => {
                                            applied.clone()
                                        }
                                        self::JavaScriptDescriptorConversionTransition::DescriptorCompleted(_) => {
                                            applied.clone()
                                        }
                                        self::JavaScriptDescriptorConversionTransition::DescriptorThrow(_) => {
                                            applied.clone()
                                        }
                                        self::JavaScriptDescriptorConversionTransition::DescriptorIntrinsicError(_, _) => {
                                            applied.clone()
                                        }
                                        self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(_) => {
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
            self::ml_string::Text::from_text(&self::ml_number::js_number(value_value_2))
        }
        self::JavaScriptSemanticValue::Integer(value_value_3) => {
            self::ml_string::Text::from_text(&value_value_3.to_string())
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
