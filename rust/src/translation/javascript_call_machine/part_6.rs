// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptCallInstruction;
use super::JavaScriptCallProgram;
use super::JavaScriptDescriptorConversionState;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptPrimitiveConversionState;
use super::JavaScriptSemanticValue;
use super::MlTemporaryFindJavaScriptCallCodeFunctionLoop3Result;
use super::MlTemporaryReadJavaScriptSemanticValueErrorLoop1Result;
use super::MlTemporaryTestJavaScriptSemanticSameValueLoop2Result;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::ml_source_exception;
use super::ml_string;
use super::source_ml_temporary_find_java_script_call_code_function_loop3;
use super::source_ml_temporary_read_java_script_semantic_value_error_loop1;
use super::source_ml_temporary_test_java_script_semantic_same_value_loop2;

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptPropertyKeyConversionTransition {
    PropertyKeyContinuation(Box<self::JavaScriptPrimitiveConversionState>),
    PropertyKeyCall(Box<self::JavaScriptPrimitiveConversionState>, Box<self::JavaScriptSemanticValue>, Box<self::JavaScriptSemanticValue>, Vec<self::JavaScriptSemanticValue>),
    PropertyKeyCompleted(Box<self::JavaScriptSemanticValue>),
    PropertyKeyThrow(Box<self::JavaScriptSemanticValue>),
    PropertyKeyIntrinsicError(self::ml_string::Text, self::ml_string::Text),
    PropertyKeyInvalid(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptPropertyDefinitionState {
    PropertyDefinitionKey(Box<self::JavaScriptSemanticValue>, Box<self::JavaScriptSemanticValue>, Box<self::JavaScriptPrimitiveConversionState>),
    PropertyDefinitionDescriptor(Box<self::JavaScriptSemanticValue>, Box<self::JavaScriptSemanticValue>, Box<self::JavaScriptDescriptorConversionState>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptPropertyDefinitionTransition {
    DefinitionContinuation(Box<self::JavaScriptPropertyDefinitionState>),
    DefinitionCall(Box<self::JavaScriptPropertyDefinitionState>, Box<self::JavaScriptSemanticValue>, Box<self::JavaScriptSemanticValue>, Vec<self::JavaScriptSemanticValue>),
    DefinitionCompleted(Box<self::JavaScriptOrdinaryHeap>, Box<self::JavaScriptSemanticValue>, Vec<self::ml_string::Text>),
    DefinitionThrow(Box<self::JavaScriptSemanticValue>),
    DefinitionIntrinsicError(self::ml_string::Text, self::ml_string::Text),
    DefinitionInvalid(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptCallHandler {
    CallHandler(f64, f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptCallPending {
    CallIdle,
    WaitCall(f64),
    WaitGet(f64),
    WaitSet(f64),
    WaitKey(f64, self::ml::Big, Box<self::JavaScriptPrimitiveConversionState>),
    WaitDefinition(f64, self::ml::Big, Box<self::JavaScriptPropertyDefinitionState>),
    WaitError(self::ml::Big, self::ml_string::Text, self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptCallFrame {
    CallFrame(self::ml::Big, f64, Vec<self::JavaScriptSemanticValue>, Vec<self::JavaScriptSemanticValue>, Vec<self::JavaScriptCallHandler>, Box<self::JavaScriptCallPending>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptCallMachine {
    CallMachine(Box<self::JavaScriptOrdinaryHeap>, Vec<self::JavaScriptCallFrame>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptCallMachineTransition {
    MachineContinue(Box<self::JavaScriptCallMachine>),
    MachineCompleted(Box<self::JavaScriptOrdinaryHeap>, Box<self::JavaScriptSemanticValue>),
    MachineThrow(Box<self::JavaScriptOrdinaryHeap>, Box<self::JavaScriptSemanticValue>),
    MachineAllocateError(Box<self::JavaScriptCallMachine>, self::ml::Big, self::ml_string::Text, self::ml_string::Text),
    MachineInvalid(Box<self::JavaScriptOrdinaryHeap>, self::ml_string::Text),
    MachineResourceLimit(Box<self::JavaScriptCallMachine>, self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_semantic_value_error(value: self::JavaScriptSemanticValue) -> self::ml_string::Text {
    match value.clone() {
        self::JavaScriptSemanticValue::Undefined => {
            self::ml_string::Text::from_units(vec![])
        }
        self::JavaScriptSemanticValue::Null => {
            self::ml_string::Text::from_units(vec![])
        }
        self::JavaScriptSemanticValue::Boolean(source_ml_source1_f1) => {
            self::ml_string::Text::from_units(vec![])
        }
        self::JavaScriptSemanticValue::Number(source_ml_source1_f2) => {
            self::ml_string::Text::from_units(vec![])
        }
        self::JavaScriptSemanticValue::Integer(source_ml_source1_f3) => {
            self::ml_string::Text::from_units(vec![])
        }
        self::JavaScriptSemanticValue::Text(units) => {
            if (((units.len() as f64)) > (1048576f64)) {
                self::ml_string::Text::from_units(vec![115, 101, 109, 97, 110, 116, 105, 99, 32, 116, 101, 120, 116, 32, 101, 120, 99, 101, 101, 100, 115, 32, 116, 104, 101, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 85, 84, 70, 49, 54, 32, 117, 110, 105, 116, 32, 98, 117, 100, 103, 101, 116])
            } else {
                {
                    let at = 0f64;
                    {
                        let source_ml_source1_s5 = self::source_ml_temporary_read_java_script_semantic_value_error_loop1(&(units), at);
                        match source_ml_source1_s5.clone() {
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
        self::JavaScriptSemanticValue::Symbol(identity) => {
            if ((identity > (self::ml::Big::from_i128(0))) && (identity <= (self::ml::Big::from_i128(18446744073709551615)))) {
                self::ml_string::Text::from_units(vec![])
            } else {
                self::ml_string::Text::from_units(vec![97, 32, 115, 121, 109, 98, 111, 108, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 97, 32, 112, 111, 115, 105, 116, 105, 118, 101, 32, 117, 110, 115, 105, 103, 110, 101, 100, 54, 52, 32, 103, 108, 111, 98, 97, 108, 32, 108, 105, 110, 107, 32, 105, 100, 101, 110, 116, 105, 116, 121])
            }
        }
        self::JavaScriptSemanticValue::Object(identity_2, source_ml_source1_f4) => {
            if ((identity_2 > (self::ml::Big::from_i128(0))) && (identity_2 <= (self::ml::Big::from_i128(18446744073709551615)))) {
                self::ml_string::Text::from_units(vec![])
            } else {
                self::ml_string::Text::from_units(vec![97, 110, 32, 111, 98, 106, 101, 99, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 97, 32, 112, 111, 115, 105, 116, 105, 118, 101, 32, 117, 110, 115, 105, 103, 110, 101, 100, 54, 52, 32, 103, 108, 111, 98, 97, 108, 32, 108, 105, 110, 107, 32, 105, 100, 101, 110, 116, 105, 116, 121])
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_semantic_same_value(left: self::JavaScriptSemanticValue, right: self::JavaScriptSemanticValue) -> bool {
    if (((self::read_java_script_semantic_value_error(left.clone())) != (self::ml_string::Text::from_units(vec![]))) || ((self::read_java_script_semantic_value_error(right.clone())) != (self::ml_string::Text::from_units(vec![])))) {
        (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![83, 97, 109, 101, 86, 97, 108, 117, 101, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 116, 119, 111, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 118, 97, 108, 117, 101, 115, 32, 105, 110, 32, 116, 104, 101, 32, 100, 101, 99, 108, 97, 114, 101, 100, 32, 115, 101, 109, 97, 110, 116, 105, 99, 32, 109, 111, 100, 101, 108])).0, message_present: true }).raise()
    } else {
        match left.clone() {
            self::JavaScriptSemanticValue::Undefined => {
                match right.clone() {
                    self::JavaScriptSemanticValue::Undefined => {
                        true
                    }
                    self::JavaScriptSemanticValue::Null => {
                        false
                    }
                    self::JavaScriptSemanticValue::Boolean(source_ml_source1_f7) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Number(source_ml_source1_f8) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Integer(source_ml_source1_f9) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Text(source_ml_source1_f10) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Symbol(source_ml_source1_f11) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Object(source_ml_source1_f12, source_ml_source1_f13) => {
                        false
                    }
                }
            }
            self::JavaScriptSemanticValue::Null => {
                match right.clone() {
                    self::JavaScriptSemanticValue::Undefined => {
                        false
                    }
                    self::JavaScriptSemanticValue::Null => {
                        true
                    }
                    self::JavaScriptSemanticValue::Boolean(source_ml_source1_f14) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Number(source_ml_source1_f15) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Integer(source_ml_source1_f16) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Text(source_ml_source1_f17) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Symbol(source_ml_source1_f18) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Object(source_ml_source1_f19, source_ml_source1_f20) => {
                        false
                    }
                }
            }
            self::JavaScriptSemanticValue::Boolean(left_value) => {
                match right.clone() {
                    self::JavaScriptSemanticValue::Undefined => {
                        false
                    }
                    self::JavaScriptSemanticValue::Null => {
                        false
                    }
                    self::JavaScriptSemanticValue::Boolean(value) => {
                        (left_value == value)
                    }
                    self::JavaScriptSemanticValue::Number(source_ml_source1_f21) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Integer(source_ml_source1_f22) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Text(source_ml_source1_f23) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Symbol(source_ml_source1_f24) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Object(source_ml_source1_f25, source_ml_source1_f26) => {
                        false
                    }
                }
            }
            self::JavaScriptSemanticValue::Number(left_value_2) => {
                match right.clone() {
                    self::JavaScriptSemanticValue::Undefined => {
                        false
                    }
                    self::JavaScriptSemanticValue::Null => {
                        false
                    }
                    self::JavaScriptSemanticValue::Boolean(source_ml_source1_f27) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Number(value_2) => {
                        if (left_value_2 == value_2) {
                            ((left_value_2 != (0f64)) || (((1f64 / left_value_2)) == ((1f64 / value_2))))
                        } else {
                            (left_value_2.is_nan() && value_2.is_nan())
                        }
                    }
                    self::JavaScriptSemanticValue::Integer(source_ml_source1_f28) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Text(source_ml_source1_f29) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Symbol(source_ml_source1_f30) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Object(source_ml_source1_f31, source_ml_source1_f32) => {
                        false
                    }
                }
            }
            self::JavaScriptSemanticValue::Integer(left_value_3) => {
                match right.clone() {
                    self::JavaScriptSemanticValue::Undefined => {
                        false
                    }
                    self::JavaScriptSemanticValue::Null => {
                        false
                    }
                    self::JavaScriptSemanticValue::Boolean(source_ml_source1_f33) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Number(source_ml_source1_f34) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Integer(value_3) => {
                        (left_value_3 == value_3)
                    }
                    self::JavaScriptSemanticValue::Text(source_ml_source1_f35) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Symbol(source_ml_source1_f36) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Object(source_ml_source1_f37, source_ml_source1_f38) => {
                        false
                    }
                }
            }
            self::JavaScriptSemanticValue::Text(left_units) => {
                match right.clone() {
                    self::JavaScriptSemanticValue::Undefined => {
                        false
                    }
                    self::JavaScriptSemanticValue::Null => {
                        false
                    }
                    self::JavaScriptSemanticValue::Boolean(source_ml_source1_f39) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Number(source_ml_source1_f40) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Integer(source_ml_source1_f41) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Text(units) => {
                        {
                            let left_units_2 = left_units;
                            {
                                let right_units = units;
                                if (((left_units_2.len() as f64)) != ((right_units.len() as f64))) {
                                    false
                                } else {
                                    {
                                        let at = 0f64;
                                        {
                                            let source_ml_source1_s45 = self::source_ml_temporary_test_java_script_semantic_same_value_loop2(&(left_units_2), &(right_units), at);
                                            match source_ml_source1_s45.clone() {
                                                self::MlTemporaryTestJavaScriptSemanticSameValueLoop2Result::MlTemporaryTestJavaScriptSemanticSameValueLoop2Done => {
                                                    true
                                                }
                                                self::MlTemporaryTestJavaScriptSemanticSameValueLoop2Result::MlTemporaryTestJavaScriptSemanticSameValueLoop2Return(source_ml_temporary_result2) => {
                                                    source_ml_temporary_result2
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    self::JavaScriptSemanticValue::Symbol(source_ml_source1_f42) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Object(source_ml_source1_f43, source_ml_source1_f44) => {
                        false
                    }
                }
            }
            self::JavaScriptSemanticValue::Symbol(left_identity) => {
                match right.clone() {
                    self::JavaScriptSemanticValue::Undefined => {
                        false
                    }
                    self::JavaScriptSemanticValue::Null => {
                        false
                    }
                    self::JavaScriptSemanticValue::Boolean(source_ml_source1_f46) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Number(source_ml_source1_f47) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Integer(source_ml_source1_f48) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Text(source_ml_source1_f49) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Symbol(identity) => {
                        (left_identity == identity)
                    }
                    self::JavaScriptSemanticValue::Object(source_ml_source1_f50, source_ml_source1_f51) => {
                        false
                    }
                }
            }
            self::JavaScriptSemanticValue::Object(left_identity_2, source_ml_source1_f6) => {
                match right.clone() {
                    self::JavaScriptSemanticValue::Undefined => {
                        false
                    }
                    self::JavaScriptSemanticValue::Null => {
                        false
                    }
                    self::JavaScriptSemanticValue::Boolean(source_ml_source1_f52) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Number(source_ml_source1_f53) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Integer(source_ml_source1_f54) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Text(source_ml_source1_f55) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Symbol(source_ml_source1_f56) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Object(identity_2, source_ml_source1_f57) => {
                        (left_identity_2 == identity_2)
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_semantic_accessor(value: self::JavaScriptSemanticValue) -> bool {
    if ((self::read_java_script_semantic_value_error(value.clone())) != (self::ml_string::Text::from_units(vec![]))) {
        false
    } else {
        match value.clone() {
            self::JavaScriptSemanticValue::Undefined => {
                true
            }
            self::JavaScriptSemanticValue::Null => {
                false
            }
            self::JavaScriptSemanticValue::Boolean(source_ml_source1_f58) => {
                {
                    let source_ml_source1_f71 = source_ml_source1_f58;
                    false
                }
            }
            self::JavaScriptSemanticValue::Number(source_ml_source1_f59) => {
                {
                    let source_ml_source1_f78 = source_ml_source1_f59;
                    false
                }
            }
            self::JavaScriptSemanticValue::Integer(source_ml_source1_f60) => {
                {
                    let source_ml_source1_f85 = source_ml_source1_f60.clone();
                    false
                }
            }
            self::JavaScriptSemanticValue::Text(source_ml_source1_f61) => {
                {
                    let source_ml_source1_f92 = source_ml_source1_f61;
                    false
                }
            }
            self::JavaScriptSemanticValue::Symbol(source_ml_source1_f62) => {
                {
                    let source_ml_source1_f99 = source_ml_source1_f62.clone();
                    false
                }
            }
            self::JavaScriptSemanticValue::Object(source_ml_source1_f63, source_ml_source1_f64) => {
                {
                    let source_ml_source1_f106 = source_ml_source1_f63.clone();
                    {
                        let callable = source_ml_source1_f64;
                        callable
                    }
                }
            }
        }
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
            let high = ((match &program { self::JavaScriptCallProgram::CallProgram(source_ml_source1_data_property_value107, _, _) => source_ml_source1_data_property_value107.as_slice() }).len() as f64);
            {
                let source_ml_source1_s108 = self::source_ml_temporary_find_java_script_call_code_function_loop3(program.clone(), identity.clone(), low, high);
                match source_ml_source1_s108.clone() {
                    self::MlTemporaryFindJavaScriptCallCodeFunctionLoop3Result::MlTemporaryFindJavaScriptCallCodeFunctionLoop3Done => {
                        (-1f64)
                    }
                    self::MlTemporaryFindJavaScriptCallCodeFunctionLoop3Result::MlTemporaryFindJavaScriptCallCodeFunctionLoop3Return(source_ml_temporary_result3) => {
                        source_ml_temporary_result3
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
        self::JavaScriptCallInstruction::Constant(destination, source_ml_source1_f109) => {
            let source_ml_source1_f109 = *source_ml_source1_f109;
            vec![destination]
        }
        self::JavaScriptCallInstruction::Copy(destination_2, source) => {
            vec![destination_2, source]
        }
        self::JavaScriptCallInstruction::Argument(destination_3, source_ml_source1_f110) => {
            vec![destination_3]
        }
        self::JavaScriptCallInstruction::Jump(source_ml_source1_f111) => {
            Vec::<f64>::new()
        }
        self::JavaScriptCallInstruction::Branch(condition, source_ml_source1_f112, source_ml_source1_f113) => {
            vec![condition]
        }
        self::JavaScriptCallInstruction::Call(destination_4, callee, receiver, arguments) => {
            {
                let source_ml_source1_concat_value118 = vec![destination_4, callee, receiver];
                {
                    let source_ml_source1_concat_value119 = arguments;
                    self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source1_concat_value118), source_ml_source1_concat_value119)
                }
            }
        }
        self::JavaScriptCallInstruction::Return(source_2) => {
            vec![source_2]
        }
        self::JavaScriptCallInstruction::Throw(source_3) => {
            vec![source_3]
        }
        self::JavaScriptCallInstruction::PushHandler(destination_5, source_ml_source1_f114) => {
            vec![destination_5]
        }
        self::JavaScriptCallInstruction::PopHandler => {
            Vec::<f64>::new()
        }
        self::JavaScriptCallInstruction::NumberBinary(destination_6, left, right, source_ml_source1_f115) => {
            vec![destination_6, left, right]
        }
        self::JavaScriptCallInstruction::SameValue(destination_7, left_2, right_2) => {
            vec![destination_7, left_2, right_2]
        }
        self::JavaScriptCallInstruction::StrictEqual(destination_8, left_3, right_3) => {
            vec![destination_8, left_3, right_3]
        }
        self::JavaScriptCallInstruction::OrdinaryGet(destination_9, object, key, receiver_2) => {
            vec![destination_9, object, key, receiver_2]
        }
        self::JavaScriptCallInstruction::OrdinarySet(destination_10, object_2, key_2, value, receiver_3) => {
            vec![destination_10, object_2, key_2, value, receiver_3]
        }
        self::JavaScriptCallInstruction::PropertyKey(destination_11, input, source_ml_source1_f116) => {
            vec![destination_11, input]
        }
        self::JavaScriptCallInstruction::PropertyDefinition(destination_12, target, key_3, attributes, source_ml_source1_f117) => {
            vec![destination_12, target, key_3, attributes]
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_call_constant_error(value: self::JavaScriptSemanticValue) -> self::ml_string::Text {
    self::read_java_script_semantic_value_error(value.clone())
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
            self::JavaScriptSemanticValue::Boolean(source_ml_source1_f120) => {
                self::test_java_script_semantic_same_value(left.clone(), right.clone())
            }
            self::JavaScriptSemanticValue::Number(left_value) => {
                match right.clone() {
                    self::JavaScriptSemanticValue::Undefined => {
                        false
                    }
                    self::JavaScriptSemanticValue::Null => {
                        false
                    }
                    self::JavaScriptSemanticValue::Boolean(source_ml_source1_f126) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Number(value) => {
                        (left_value == value)
                    }
                    self::JavaScriptSemanticValue::Integer(source_ml_source1_f127) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Text(source_ml_source1_f128) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Symbol(source_ml_source1_f129) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Object(source_ml_source1_f130, source_ml_source1_f131) => {
                        false
                    }
                }
            }
            self::JavaScriptSemanticValue::Integer(source_ml_source1_f121) => {
                self::test_java_script_semantic_same_value(left.clone(), right.clone())
            }
            self::JavaScriptSemanticValue::Text(source_ml_source1_f122) => {
                self::test_java_script_semantic_same_value(left.clone(), right.clone())
            }
            self::JavaScriptSemanticValue::Symbol(source_ml_source1_f123) => {
                self::test_java_script_semantic_same_value(left.clone(), right.clone())
            }
            self::JavaScriptSemanticValue::Object(source_ml_source1_f124, source_ml_source1_f125) => {
                self::test_java_script_semantic_same_value(left.clone(), right.clone())
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_property_descriptor_kind(descriptor: (bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool)) -> self::ml_string::Text {
    {
        let data = ((descriptor.clone()).9 || (descriptor.clone()).11);
        {
            let accessor = ((descriptor.clone()).5 || (descriptor.clone()).7);
            if (data && accessor) {
                self::ml_string::Text::from_units(vec![105, 110, 118, 97, 108, 105, 100])
            } else {
                if data {
                    self::ml_string::Text::from_units(vec![100, 97, 116, 97])
                } else {
                    if accessor {
                        self::ml_string::Text::from_units(vec![97, 99, 99, 101, 115, 115, 111, 114])
                    } else {
                        self::ml_string::Text::from_units(vec![103, 101, 110, 101, 114, 105, 99])
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_property_descriptor_error(descriptor: (bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool)) -> self::ml_string::Text {
    if ((self::read_java_script_property_descriptor_kind(descriptor.clone())) == (self::ml_string::Text::from_units(vec![105, 110, 118, 97, 108, 105, 100]))) {
        self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114])
    } else {
        if ((descriptor.clone()).9 && ((self::read_java_script_semantic_value_error((descriptor.clone()).8)) != (self::ml_string::Text::from_units(vec![])))) {
            self::ml_string::Text::from_units(vec![73, 110, 118, 97, 108, 105, 100, 83, 101, 109, 97, 110, 116, 105, 99, 86, 97, 108, 117, 101])
        } else {
            if ((descriptor.clone()).5 && (((self::read_java_script_semantic_value_error((descriptor.clone()).4)) != (self::ml_string::Text::from_units(vec![]))) || !(self::test_java_script_semantic_accessor((descriptor.clone()).4)))) {
                self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114])
            } else {
                if ((descriptor.clone()).7 && (((self::read_java_script_semantic_value_error((descriptor.clone()).6)) != (self::ml_string::Text::from_units(vec![]))) || !(self::test_java_script_semantic_accessor((descriptor.clone()).6)))) {
                    self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114])
                } else {
                    self::ml_string::Text::from_units(vec![])
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_complete_java_script_property_descriptor(descriptor: (bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool)) -> bool {
    if ((!((descriptor.clone()).1) || !((descriptor.clone()).3)) || ((self::read_java_script_property_descriptor_error(descriptor.clone())) != (self::ml_string::Text::from_units(vec![])))) {
        false
    } else {
        {
            let kind = self::read_java_script_property_descriptor_kind(descriptor.clone());
            if (kind == (self::ml_string::Text::from_units(vec![100, 97, 116, 97]))) {
                ((descriptor.clone()).9 && (descriptor.clone()).11)
            } else {
                (((kind == (self::ml_string::Text::from_units(vec![97, 99, 99, 101, 115, 115, 111, 114]))) && (descriptor.clone()).5) && (descriptor.clone()).7)
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn create_java_script_property_descriptor(kind: &self::ml_string::Text, configurable: bool, enumerable: bool) -> (bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool) {
    {
        let source_ml_source1_field132 = true;
        {
            let source_ml_source1_field133 = configurable;
            {
                let source_ml_source1_field134 = true;
                {
                    let source_ml_source1_field135 = enumerable;
                    {
                        let source_ml_source1_field136 = ((*kind) != (self::ml_string::Text::from_units(vec![97, 99, 99, 101, 115, 115, 111, 114])));
                        {
                            let source_ml_source1_field137 = self::JavaScriptSemanticValue::Undefined;
                            {
                                let source_ml_source1_field138 = ((*kind) != (self::ml_string::Text::from_units(vec![97, 99, 99, 101, 115, 115, 111, 114])));
                                {
                                    let source_ml_source1_field139 = false;
                                    {
                                        let source_ml_source1_field140 = ((*kind) == (self::ml_string::Text::from_units(vec![97, 99, 99, 101, 115, 115, 111, 114])));
                                        {
                                            let source_ml_source1_field141 = self::JavaScriptSemanticValue::Undefined;
                                            {
                                                let source_ml_source1_field142 = ((*kind) == (self::ml_string::Text::from_units(vec![97, 99, 99, 101, 115, 115, 111, 114])));
                                                {
                                                    let source_ml_source1_field143 = self::JavaScriptSemanticValue::Undefined;
                                                    (source_ml_source1_field133, source_ml_source1_field132, source_ml_source1_field135, source_ml_source1_field134, source_ml_source1_field141.clone(), source_ml_source1_field140, source_ml_source1_field143.clone(), source_ml_source1_field142, source_ml_source1_field137.clone(), source_ml_source1_field136, source_ml_source1_field139, source_ml_source1_field138)
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
pub fn apply_java_script_property_descriptor_fields(base: (bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool), proposed: (bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool)) -> (bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool) {
    {
        let source_ml_source1_field144 = true;
        {
            let source_ml_source1_field145 = if (proposed.clone()).1 {
                (proposed.clone()).0
            } else {
                (base.clone()).0
            };
            {
                let source_ml_source1_field146 = true;
                {
                    let source_ml_source1_field147 = if (proposed.clone()).3 {
                        (proposed.clone()).2
                    } else {
                        (base.clone()).2
                    };
                    {
                        let source_ml_source1_field148 = (base.clone()).9;
                        {
                            let source_ml_source1_field149 = if ((base.clone()).9 && (proposed.clone()).9) {
                                (proposed.clone()).8
                            } else {
                                (base.clone()).8
                            };
                            {
                                let source_ml_source1_field150 = (base.clone()).11;
                                {
                                    let source_ml_source1_field151 = if ((base.clone()).11 && (proposed.clone()).11) {
                                        (proposed.clone()).10
                                    } else {
                                        (base.clone()).10
                                    };
                                    {
                                        let source_ml_source1_field152 = (base.clone()).5;
                                        {
                                            let source_ml_source1_field153 = if ((base.clone()).5 && (proposed.clone()).5) {
                                                (proposed.clone()).4
                                            } else {
                                                (base.clone()).4
                                            };
                                            {
                                                let source_ml_source1_field154 = (base.clone()).7;
                                                {
                                                    let source_ml_source1_field155 = if ((base.clone()).7 && (proposed.clone()).7) {
                                                        (proposed.clone()).6
                                                    } else {
                                                        (base.clone()).6
                                                    };
                                                    (source_ml_source1_field145, source_ml_source1_field144, source_ml_source1_field147, source_ml_source1_field146, source_ml_source1_field153.clone(), source_ml_source1_field152, source_ml_source1_field155.clone(), source_ml_source1_field154, source_ml_source1_field149.clone(), source_ml_source1_field148, source_ml_source1_field151, source_ml_source1_field150)
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
pub fn read_java_script_property_descriptor_writes(descriptor: (bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool)) -> Vec<self::ml_string::Text> {
    {
        let writes = Vec::<self::ml_string::Text>::new();
        {
            let writes_3 = if (descriptor.clone()).1 {
                {
                    let writes_2 = {
                        let source_ml_source1_concat_value156 = writes;
                        {
                            let source_ml_source1_concat_value157 = vec![self::ml_string::Text::from_units(vec![99, 111, 110, 102, 105, 103, 117, 114, 97, 98, 108, 101])];
                            self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), source_ml_source1_concat_value156), source_ml_source1_concat_value157)
                        }
                    };
                    writes_2
                }
            } else {
                writes
            };
            {
                let writes_5 = if (descriptor.clone()).3 {
                    {
                        let writes_4 = {
                            let source_ml_source1_concat_value158 = writes_3;
                            {
                                let source_ml_source1_concat_value159 = vec![self::ml_string::Text::from_units(vec![101, 110, 117, 109, 101, 114, 97, 98, 108, 101])];
                                self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), source_ml_source1_concat_value158), source_ml_source1_concat_value159)
                            }
                        };
                        writes_4
                    }
                } else {
                    writes_3
                };
                {
                    let writes_7 = if (descriptor.clone()).9 {
                        {
                            let writes_6 = {
                                let source_ml_source1_concat_value160 = writes_5;
                                {
                                    let source_ml_source1_concat_value161 = vec![self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101])];
                                    self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), source_ml_source1_concat_value160), source_ml_source1_concat_value161)
                                }
                            };
                            writes_6
                        }
                    } else {
                        writes_5
                    };
                    {
                        let writes_9 = if (descriptor.clone()).11 {
                            {
                                let writes_8 = {
                                    let source_ml_source1_concat_value162 = writes_7;
                                    {
                                        let source_ml_source1_concat_value163 = vec![self::ml_string::Text::from_units(vec![119, 114, 105, 116, 97, 98, 108, 101])];
                                        self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), source_ml_source1_concat_value162), source_ml_source1_concat_value163)
                                    }
                                };
                                writes_8
                            }
                        } else {
                            writes_7
                        };
                        {
                            let writes_11 = if (descriptor.clone()).5 {
                                {
                                    let writes_10 = {
                                        let source_ml_source1_concat_value164 = writes_9;
                                        {
                                            let source_ml_source1_concat_value165 = vec![self::ml_string::Text::from_units(vec![103, 101, 116])];
                                            self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), source_ml_source1_concat_value164), source_ml_source1_concat_value165)
                                        }
                                    };
                                    writes_10
                                }
                            } else {
                                writes_9
                            };
                            {
                                let writes_13 = if (descriptor.clone()).7 {
                                    {
                                        let writes_12 = {
                                            let source_ml_source1_concat_value166 = writes_11;
                                            {
                                                let source_ml_source1_concat_value167 = vec![self::ml_string::Text::from_units(vec![115, 101, 116])];
                                                self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), source_ml_source1_concat_value166), source_ml_source1_concat_value167)
                                            }
                                        };
                                        writes_12
                                    }
                                } else {
                                    writes_11
                                };
                                writes_13
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
pub fn create_java_script_property_transition(disposition: &self::ml_string::Text, error: &self::ml_string::Text, descriptor: (bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool), writes: Vec<self::ml_string::Text>) -> ((bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool), self::ml_string::Text, self::ml_string::Text, Vec<self::ml_string::Text>) {
    {
        let source_ml_source1_field168 = disposition.clone();
        {
            let source_ml_source1_field169 = error.clone();
            {
                let source_ml_source1_field170 = descriptor.clone();
                {
                    let source_ml_source1_field171 = writes;
                    (source_ml_source1_field170.clone(), source_ml_source1_field168.clone(), source_ml_source1_field169.clone(), source_ml_source1_field171)
                }
            }
        }
    }
}
