// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptDescriptorConversionState;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptPrimitiveConversionState;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryApplyBigIntegerBitwiseOperationJoin5;
use super::MlSource1TemporaryApplyBigIntegerBitwiseOperationLoop8Result;
use super::MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop1Result;
use super::MlSource1TemporaryReadPrimitiveBitwiseDomainLoop4Result;
use super::MlSource1TemporaryTestJavaScriptSemanticSameValueLoop2Result;
use super::ml;
use super::ml_array;
use super::ml_number;
use super::ml_source_exception;
use super::ml_string;
use super::source_ml_source1_temporary_apply_big_integer_bitwise_operation_loop6;
use super::source_ml_source1_temporary_apply_big_integer_bitwise_operation_loop7;
use super::source_ml_source1_temporary_apply_big_integer_bitwise_operation_loop8;
use super::source_ml_source1_temporary_read_java_script_semantic_value_error_loop1;
use super::source_ml_source1_temporary_read_primitive_bitwise_domain_loop4;
use super::source_ml_source1_temporary_render_big_integer_bitwise_expression_loop9;
use super::source_ml_source1_temporary_test_java_script_semantic_same_value_loop2;

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptPrimitiveConversionTransition {
    PrimitiveContinuation(Box<self::JavaScriptPrimitiveConversionState>),
    PrimitiveCall(Box<self::JavaScriptPrimitiveConversionState>, Box<self::JavaScriptSemanticValue>, Box<self::JavaScriptSemanticValue>, Vec<self::JavaScriptSemanticValue>),
    PrimitiveCompleted(Box<self::JavaScriptSemanticValue>),
    PrimitiveThrow(Box<self::JavaScriptSemanticValue>),
    PrimitiveIntrinsicError(self::ml_string::Text, self::ml_string::Text),
    PrimitiveInvalid(self::ml_string::Text),
}

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
        self::JavaScriptSemanticValue::Boolean(source_ml_source2_f1) => {
            self::ml_string::Text::from_units(vec![])
        }
        self::JavaScriptSemanticValue::Number(source_ml_source2_f2) => {
            self::ml_string::Text::from_units(vec![])
        }
        self::JavaScriptSemanticValue::Integer(source_ml_source2_f3) => {
            self::ml_string::Text::from_units(vec![])
        }
        self::JavaScriptSemanticValue::Text(units) => {
            if (((units.len() as f64)) > (1048576f64)) {
                self::ml_string::Text::from_units(vec![115, 101, 109, 97, 110, 116, 105, 99, 32, 116, 101, 120, 116, 32, 101, 120, 99, 101, 101, 100, 115, 32, 116, 104, 101, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 85, 84, 70, 49, 54, 32, 117, 110, 105, 116, 32, 98, 117, 100, 103, 101, 116])
            } else {
                {
                    let at = 0f64;
                    {
                        let source_ml_source2_s5 = self::source_ml_source1_temporary_read_java_script_semantic_value_error_loop1(&(units), at);
                        match source_ml_source2_s5.clone() {
                            self::MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop1Result::MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop1Done => {
                                self::ml_string::Text::from_units(vec![])
                            }
                            self::MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop1Result::MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop1Return(source_ml_source1_temporary_result1) => {
                                source_ml_source1_temporary_result1.clone()
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
        self::JavaScriptSemanticValue::Object(identity_2, source_ml_source2_f4) => {
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
                    self::JavaScriptSemanticValue::Boolean(source_ml_source2_f7) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Number(source_ml_source2_f8) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Integer(source_ml_source2_f9) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Text(source_ml_source2_f10) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Symbol(source_ml_source2_f11) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Object(source_ml_source2_f12, source_ml_source2_f13) => {
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
                    self::JavaScriptSemanticValue::Boolean(source_ml_source2_f14) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Number(source_ml_source2_f15) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Integer(source_ml_source2_f16) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Text(source_ml_source2_f17) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Symbol(source_ml_source2_f18) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Object(source_ml_source2_f19, source_ml_source2_f20) => {
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
                    self::JavaScriptSemanticValue::Number(source_ml_source2_f21) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Integer(source_ml_source2_f22) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Text(source_ml_source2_f23) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Symbol(source_ml_source2_f24) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Object(source_ml_source2_f25, source_ml_source2_f26) => {
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
                    self::JavaScriptSemanticValue::Boolean(source_ml_source2_f27) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Number(value_2) => {
                        if (left_value_2 == value_2) {
                            ((left_value_2 != (0f64)) || (((1f64 / left_value_2)) == ((1f64 / value_2))))
                        } else {
                            (left_value_2.is_nan() && value_2.is_nan())
                        }
                    }
                    self::JavaScriptSemanticValue::Integer(source_ml_source2_f28) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Text(source_ml_source2_f29) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Symbol(source_ml_source2_f30) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Object(source_ml_source2_f31, source_ml_source2_f32) => {
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
                    self::JavaScriptSemanticValue::Boolean(source_ml_source2_f33) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Number(source_ml_source2_f34) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Integer(value_3) => {
                        (left_value_3 == value_3)
                    }
                    self::JavaScriptSemanticValue::Text(source_ml_source2_f35) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Symbol(source_ml_source2_f36) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Object(source_ml_source2_f37, source_ml_source2_f38) => {
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
                    self::JavaScriptSemanticValue::Boolean(source_ml_source2_f39) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Number(source_ml_source2_f40) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Integer(source_ml_source2_f41) => {
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
                                            let source_ml_source2_s45 = self::source_ml_source1_temporary_test_java_script_semantic_same_value_loop2(&(left_units_2), &(right_units), at);
                                            match source_ml_source2_s45.clone() {
                                                self::MlSource1TemporaryTestJavaScriptSemanticSameValueLoop2Result::MlSource1TemporaryTestJavaScriptSemanticSameValueLoop2Done => {
                                                    true
                                                }
                                                self::MlSource1TemporaryTestJavaScriptSemanticSameValueLoop2Result::MlSource1TemporaryTestJavaScriptSemanticSameValueLoop2Return(source_ml_source1_temporary_result2) => {
                                                    source_ml_source1_temporary_result2
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    self::JavaScriptSemanticValue::Symbol(source_ml_source2_f42) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Object(source_ml_source2_f43, source_ml_source2_f44) => {
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
                    self::JavaScriptSemanticValue::Boolean(source_ml_source2_f46) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Number(source_ml_source2_f47) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Integer(source_ml_source2_f48) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Text(source_ml_source2_f49) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Symbol(identity) => {
                        (left_identity == identity)
                    }
                    self::JavaScriptSemanticValue::Object(source_ml_source2_f50, source_ml_source2_f51) => {
                        false
                    }
                }
            }
            self::JavaScriptSemanticValue::Object(left_identity_2, source_ml_source2_f6) => {
                match right.clone() {
                    self::JavaScriptSemanticValue::Undefined => {
                        false
                    }
                    self::JavaScriptSemanticValue::Null => {
                        false
                    }
                    self::JavaScriptSemanticValue::Boolean(source_ml_source2_f52) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Number(source_ml_source2_f53) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Integer(source_ml_source2_f54) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Text(source_ml_source2_f55) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Symbol(source_ml_source2_f56) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Object(identity_2, source_ml_source2_f57) => {
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
            self::JavaScriptSemanticValue::Boolean(source_ml_source2_f58) => {
                {
                    let source_ml_source2_f71 = source_ml_source2_f58;
                    false
                }
            }
            self::JavaScriptSemanticValue::Number(source_ml_source2_f59) => {
                {
                    let source_ml_source2_f78 = source_ml_source2_f59;
                    false
                }
            }
            self::JavaScriptSemanticValue::Integer(source_ml_source2_f60) => {
                {
                    let source_ml_source2_f85 = source_ml_source2_f60.clone();
                    false
                }
            }
            self::JavaScriptSemanticValue::Text(source_ml_source2_f61) => {
                {
                    let source_ml_source2_f92 = source_ml_source2_f61;
                    false
                }
            }
            self::JavaScriptSemanticValue::Symbol(source_ml_source2_f62) => {
                {
                    let source_ml_source2_f99 = source_ml_source2_f62.clone();
                    false
                }
            }
            self::JavaScriptSemanticValue::Object(source_ml_source2_f63, source_ml_source2_f64) => {
                {
                    let source_ml_source2_f106 = source_ml_source2_f63.clone();
                    {
                        let callable = source_ml_source2_f64;
                        callable
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_big_integer_bitwise_operation_code(operation: &self::ml_string::Text) -> f64 {
    if ((*operation) == (self::ml_string::Text::from_units(vec![99, 111, 110, 106, 111, 105, 110, 66, 105, 103, 73, 110, 116, 101, 103, 101, 114, 66, 105, 116, 115]))) {
        0f64
    } else {
        if ((*operation) == (self::ml_string::Text::from_units(vec![100, 105, 115, 106, 111, 105, 110, 66, 105, 103, 73, 110, 116, 101, 103, 101, 114, 66, 105, 116, 115]))) {
            1f64
        } else {
            if ((*operation) == (self::ml_string::Text::from_units(vec![101, 120, 99, 108, 117, 100, 101, 69, 113, 117, 97, 108, 66, 105, 103, 73, 110, 116, 101, 103, 101, 114, 66, 105, 116, 115]))) {
                2f64
            } else {
                if ((*operation) == (self::ml_string::Text::from_units(vec![115, 104, 105, 102, 116, 66, 105, 103, 73, 110, 116, 101, 103, 101, 114, 66, 105, 116, 115, 76, 101, 102, 116]))) {
                    3f64
                } else {
                    if ((*operation) == (self::ml_string::Text::from_units(vec![115, 104, 105, 102, 116, 66, 105, 103, 73, 110, 116, 101, 103, 101, 114, 66, 105, 116, 115, 82, 105, 103, 104, 116]))) {
                        4f64
                    } else {
                        if ((*operation) == (self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 109, 101, 110, 116, 66, 105, 103, 73, 110, 116, 101, 103, 101, 114, 66, 105, 116, 115]))) {
                            6f64
                        } else {
                            (-1f64)
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_big_integer_bitwise_operation(operation: &self::ml_string::Text) -> self::ml_string::Text {
    if ((*operation) == (self::ml_string::Text::from_units(vec![99, 111, 110, 106, 111, 105, 110, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 78, 117, 109, 98, 101, 114, 66, 105, 116, 115]))) {
        self::ml_string::Text::from_units(vec![99, 111, 110, 106, 111, 105, 110, 66, 105, 103, 73, 110, 116, 101, 103, 101, 114, 66, 105, 116, 115])
    } else {
        if ((*operation) == (self::ml_string::Text::from_units(vec![100, 105, 115, 106, 111, 105, 110, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 78, 117, 109, 98, 101, 114, 66, 105, 116, 115]))) {
            self::ml_string::Text::from_units(vec![100, 105, 115, 106, 111, 105, 110, 66, 105, 103, 73, 110, 116, 101, 103, 101, 114, 66, 105, 116, 115])
        } else {
            if ((*operation) == (self::ml_string::Text::from_units(vec![101, 120, 99, 108, 117, 100, 101, 69, 113, 117, 97, 108, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 78, 117, 109, 98, 101, 114, 66, 105, 116, 115]))) {
                self::ml_string::Text::from_units(vec![101, 120, 99, 108, 117, 100, 101, 69, 113, 117, 97, 108, 66, 105, 103, 73, 110, 116, 101, 103, 101, 114, 66, 105, 116, 115])
            } else {
                if ((*operation) == (self::ml_string::Text::from_units(vec![115, 104, 105, 102, 116, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 78, 117, 109, 98, 101, 114, 66, 105, 116, 115, 76, 101, 102, 116]))) {
                    self::ml_string::Text::from_units(vec![115, 104, 105, 102, 116, 66, 105, 103, 73, 110, 116, 101, 103, 101, 114, 66, 105, 116, 115, 76, 101, 102, 116])
                } else {
                    if ((*operation) == (self::ml_string::Text::from_units(vec![115, 104, 105, 102, 116, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 78, 117, 109, 98, 101, 114, 66, 105, 116, 115, 82, 105, 103, 104, 116]))) {
                        self::ml_string::Text::from_units(vec![115, 104, 105, 102, 116, 66, 105, 103, 73, 110, 116, 101, 103, 101, 114, 66, 105, 116, 115, 82, 105, 103, 104, 116])
                    } else {
                        if ((*operation) == (self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 109, 101, 110, 116, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 78, 117, 109, 98, 101, 114, 66, 105, 116, 115]))) {
                            self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 109, 101, 110, 116, 66, 105, 103, 73, 110, 116, 101, 103, 101, 114, 66, 105, 116, 115])
                        } else {
                            if ((self::read_big_integer_bitwise_operation_code(operation)) < (0f64)) {
                                self::ml_string::Text::from_units(vec![])
                            } else {
                                operation.clone()
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
pub fn read_big_integer_bitwise_arity(operation: &self::ml_string::Text) -> f64 {
    {
        let code = self::read_big_integer_bitwise_operation_code(operation);
        if (code == (6f64)) {
            1f64
        } else {
            if (code < (0f64)) {
                0f64
            } else {
                2f64
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_linked_big_integer_bitwise_operation(expression: &self::ml_string::Text, operation: &self::ml_string::Text) -> bool {
    (((*expression) == (self::ml_string::Text::from_units(vec![109, 97, 116, 104]))) && ((self::read_big_integer_bitwise_arity(operation)) != (0f64)))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_primitive_bitwise_domain(kinds: &[self::ml_string::Text]) -> self::ml_string::Text {
    if (((kinds.len() as f64)) == (0f64)) {
        self::ml_string::Text::from_units(vec![])
    } else {
        {
            let number = true;
            {
                let integer = true;
                {
                    let source_ml_source1_temporary_values3 = kinds.to_vec();
                    {
                        let source_ml_source1_temporary_index3 = self::ml::Big::from_i128(0);
                        {
                            let source_ml_source2_s107 = self::source_ml_source1_temporary_read_primitive_bitwise_domain_loop4(number, integer, &(source_ml_source1_temporary_values3), source_ml_source1_temporary_index3.clone());
                            match source_ml_source2_s107.clone() {
                                self::MlSource1TemporaryReadPrimitiveBitwiseDomainLoop4Result::MlSource1TemporaryReadPrimitiveBitwiseDomainLoop4Done(number_2, integer_2) => {
                                    if number_2 {
                                        self::ml_string::Text::from_units(vec![78, 117, 109, 98, 101, 114])
                                    } else {
                                        if integer_2 {
                                            self::ml_string::Text::from_units(vec![66, 105, 103, 73, 110, 116])
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
                            let source_ml_source2_s108 = if (count < (self::ml::Big::from_i128(0))) {
                                {
                                    let count_2 = count.neg();
                                    {
                                        let forward_2 = !forward;
                                        self::MlSource1TemporaryApplyBigIntegerBitwiseOperationJoin5::MlSource1TemporaryApplyBigIntegerBitwiseOperationJoin5Next(count_2.clone(), forward_2)
                                    }
                                }
                            } else {
                                self::MlSource1TemporaryApplyBigIntegerBitwiseOperationJoin5::MlSource1TemporaryApplyBigIntegerBitwiseOperationJoin5Next(count.clone(), forward)
                            };
                            match source_ml_source2_s108.clone() {
                                self::MlSource1TemporaryApplyBigIntegerBitwiseOperationJoin5::MlSource1TemporaryApplyBigIntegerBitwiseOperationJoin5Next(count_3, forward_3) => {
                                    {
                                        let value = left.clone();
                                        {
                                            let value_4 = if forward_3 {
                                                {
                                                    let factor = self::ml::Big::from_i128(2);
                                                    {
                                                        let value_2 = self::source_ml_source1_temporary_apply_big_integer_bitwise_operation_loop6(count_3.clone(), value.clone(), factor.clone());
                                                        value_2.clone()
                                                    }
                                                }
                                            } else {
                                                {
                                                    let value_3 = self::source_ml_source1_temporary_apply_big_integer_bitwise_operation_loop7(count_3.clone(), value.clone());
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
                                            let source_ml_source2_s109 = self::source_ml_source1_temporary_apply_big_integer_bitwise_operation_loop8(operation, negative_left, negative_right, first.clone(), second.clone(), weight.clone(), result.clone());
                                            match source_ml_source2_s109.clone() {
                                                self::MlSource1TemporaryApplyBigIntegerBitwiseOperationLoop8Result::MlSource1TemporaryApplyBigIntegerBitwiseOperationLoop8Done(weight_2, result_2) => {
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
pub fn render_big_integer_bitwise_expression(target: &self::ml_string::Text, operation: &self::ml_string::Text, arguments_: &[self::ml_string::Text]) -> self::ml_string::Text {
    {
        let code = self::read_big_integer_bitwise_operation_code(operation);
        if ((code < (0f64)) || (((arguments_.len() as f64)) != (self::read_big_integer_bitwise_arity(operation)))) {
            self::ml_string::Text::from_units(vec![])
        } else {
            {
                let left = (self::ml_array::optional_at(arguments_, self::ml_array::number_index(0f64))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]));
                {
                    let right = (self::ml_array::optional_at(arguments_, self::ml_array::number_index(1f64))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]));
                    if ((*target) == (self::ml_string::Text::from_units(vec![82, 111, 99, 113]))) {
                        {
                            let operation_name = if (code == (0f64)) {
                                self::ml_string::Text::from_units(vec![90, 46, 108, 97, 110, 100])
                            } else {
                                if (code == (1f64)) {
                                    self::ml_string::Text::from_units(vec![90, 46, 108, 111, 114])
                                } else {
                                    if (code == (2f64)) {
                                        self::ml_string::Text::from_units(vec![90, 46, 108, 120, 111, 114])
                                    } else {
                                        if (code == (3f64)) {
                                            self::ml_string::Text::from_units(vec![90, 46, 115, 104, 105, 102, 116, 108])
                                        } else {
                                            if (code == (4f64)) {
                                                self::ml_string::Text::from_units(vec![90, 46, 115, 104, 105, 102, 116, 114])
                                            } else {
                                                self::ml_string::Text::from_units(vec![90, 46, 108, 110, 111, 116])
                                            }
                                        }
                                    }
                                }
                            };
                            (((((self::ml_string::Text::from_units(vec![40])).concat(&operation_name)).concat(&self::ml_string::Text::from_units(vec![32]))).concat(&left)).concat(&if (code == (6f64)) {
                                self::ml_string::Text::from_units(vec![])
                            } else {
                                (self::ml_string::Text::from_units(vec![32])).concat(&right)
                            })).concat(&self::ml_string::Text::from_units(vec![41]))
                        }
                    } else {
                        if ((*target) == (self::ml_string::Text::from_units(vec![76, 101, 97, 110]))) {
                            if (code == (6f64)) {
                                ((self::ml_string::Text::from_units(vec![40, 45, 49, 32, 45, 32, 40])).concat(&left)).concat(&self::ml_string::Text::from_units(vec![41, 41]))
                            } else {
                                {
                                    let text = (left.concat(&self::ml_string::Text::from_units(vec![32]))).concat(&right);
                                    {
                                        let suffix = 0f64;
                                        {
                                            let suffix_2 = self::source_ml_source1_temporary_render_big_integer_bitwise_expression_loop9(&(text), suffix);
                                            {
                                                let a = (self::ml_string::Text::from_units(vec![109, 108, 95, 105, 110, 116, 101, 103, 101, 114, 95, 108, 101, 102, 116])).concat(&self::ml_string::Text::from_text(&self::ml_number::js_number(suffix_2)));
                                                {
                                                    let b = (self::ml_string::Text::from_units(vec![109, 108, 95, 105, 110, 116, 101, 103, 101, 114, 95, 114, 105, 103, 104, 116])).concat(&self::ml_string::Text::from_text(&self::ml_number::js_number(suffix_2)));
                                                    {
                                                        let am = ((self::ml_string::Text::from_units(vec![40, 73, 110, 116, 46, 116, 111, 78, 97, 116, 32, 40, 45, 49, 32, 45, 32])).concat(&a)).concat(&self::ml_string::Text::from_units(vec![41, 41]));
                                                        {
                                                            let bm = ((self::ml_string::Text::from_units(vec![40, 73, 110, 116, 46, 116, 111, 78, 97, 116, 32, 40, 45, 49, 32, 45, 32])).concat(&b)).concat(&self::ml_string::Text::from_units(vec![41, 41]));
                                                            {
                                                                let ap = ((self::ml_string::Text::from_units(vec![40, 73, 110, 116, 46, 116, 111, 78, 97, 116, 32])).concat(&a)).concat(&self::ml_string::Text::from_units(vec![41]));
                                                                {
                                                                    let bp = ((self::ml_string::Text::from_units(vec![40, 73, 110, 116, 46, 116, 111, 78, 97, 116, 32])).concat(&b)).concat(&self::ml_string::Text::from_units(vec![41]));
                                                                    {
                                                                        let prefix = ((((((((self::ml_string::Text::from_units(vec![40, 108, 101, 116, 32])).concat(&a)).concat(&self::ml_string::Text::from_units(vec![32, 58, 32, 73, 110, 116, 32, 58, 61, 32]))).concat(&left)).concat(&self::ml_string::Text::from_units(vec![59, 32, 108, 101, 116, 32]))).concat(&b)).concat(&self::ml_string::Text::from_units(vec![32, 58, 32, 73, 110, 116, 32, 58, 61, 32]))).concat(&right)).concat(&self::ml_string::Text::from_units(vec![59, 32]));
                                                                        if ((code == (3f64)) || (code == (4f64))) {
                                                                            {
                                                                                let count = ((((((self::ml_string::Text::from_units(vec![40, 73, 110, 116, 46, 116, 111, 78, 97, 116, 32, 40, 105, 102, 32])).concat(&b)).concat(&self::ml_string::Text::from_units(vec![32, 60, 32, 48, 32, 116, 104, 101, 110, 32, 45]))).concat(&b)).concat(&self::ml_string::Text::from_units(vec![32, 101, 108, 115, 101, 32]))).concat(&b)).concat(&self::ml_string::Text::from_units(vec![41, 41]));
                                                                                {
                                                                                    let shift_left = ((((self::ml_string::Text::from_units(vec![40])).concat(&a)).concat(&self::ml_string::Text::from_units(vec![32, 42, 32, 73, 110, 116, 46, 111, 102, 78, 97, 116, 32, 40, 78, 97, 116, 46, 115, 104, 105, 102, 116, 76, 101, 102, 116, 32, 49, 32]))).concat(&count)).concat(&self::ml_string::Text::from_units(vec![41, 41]));
                                                                                    {
                                                                                        let shift_right = ((((((((((self::ml_string::Text::from_units(vec![40, 105, 102, 32])).concat(&a)).concat(&self::ml_string::Text::from_units(vec![32, 60, 32, 48, 32, 116, 104, 101, 110, 32, 45, 49, 32, 45, 32, 73, 110, 116, 46, 111, 102, 78, 97, 116, 32, 40, 78, 97, 116, 46, 115, 104, 105, 102, 116, 82, 105, 103, 104, 116, 32]))).concat(&am)).concat(&self::ml_string::Text::from_units(vec![32]))).concat(&count)).concat(&self::ml_string::Text::from_units(vec![41, 32, 101, 108, 115, 101, 32, 73, 110, 116, 46, 111, 102, 78, 97, 116, 32, 40, 78, 97, 116, 46, 115, 104, 105, 102, 116, 82, 105, 103, 104, 116, 32]))).concat(&ap)).concat(&self::ml_string::Text::from_units(vec![32]))).concat(&count)).concat(&self::ml_string::Text::from_units(vec![41, 41]));
                                                                                        ((((((((prefix.concat(&self::ml_string::Text::from_units(vec![105, 102, 32]))).concat(&a)).concat(&self::ml_string::Text::from_units(vec![32, 61, 32, 48, 32, 116, 104, 101, 110, 32, 48, 32, 101, 108, 115, 101, 32, 105, 102, 32]))).concat(&b)).concat(&self::ml_string::Text::from_units(vec![32, 60, 32, 48, 32, 116, 104, 101, 110, 32]))).concat(&if (code == (3f64)) {
                                                                                            shift_right.clone()
                                                                                        } else {
                                                                                            shift_left.clone()
                                                                                        })).concat(&self::ml_string::Text::from_units(vec![32, 101, 108, 115, 101, 32]))).concat(&if (code == (3f64)) {
                                                                                            shift_left.clone()
                                                                                        } else {
                                                                                            shift_right.clone()
                                                                                        })).concat(&self::ml_string::Text::from_units(vec![41]))
                                                                                    }
                                                                                }
                                                                            }
                                                                        } else {
                                                                            {
                                                                                let difference = self::ml_string::Text::from_units(vec![40, 78, 97, 116, 46, 98, 105, 116, 119, 105, 115, 101, 32, 40, 102, 117, 110, 32, 109, 108, 95, 98, 105, 116, 95, 108, 101, 102, 116, 32, 109, 108, 95, 98, 105, 116, 95, 114, 105, 103, 104, 116, 32, 61, 62, 32, 109, 108, 95, 98, 105, 116, 95, 108, 101, 102, 116, 32, 38, 38, 32, 33, 109, 108, 95, 98, 105, 116, 95, 114, 105, 103, 104, 116, 41, 32]);
                                                                                {
                                                                                    let both = if (code == (0f64)) {
                                                                                        ((((self::ml_string::Text::from_units(vec![40, 45, 49, 32, 45, 32, 73, 110, 116, 46, 111, 102, 78, 97, 116, 32, 40, 78, 97, 116, 46, 108, 111, 114, 32])).concat(&am)).concat(&self::ml_string::Text::from_units(vec![32]))).concat(&bm)).concat(&self::ml_string::Text::from_units(vec![41, 41]))
                                                                                    } else {
                                                                                        if (code == (1f64)) {
                                                                                            ((((self::ml_string::Text::from_units(vec![40, 45, 49, 32, 45, 32, 73, 110, 116, 46, 111, 102, 78, 97, 116, 32, 40, 78, 97, 116, 46, 108, 97, 110, 100, 32])).concat(&am)).concat(&self::ml_string::Text::from_units(vec![32]))).concat(&bm)).concat(&self::ml_string::Text::from_units(vec![41, 41]))
                                                                                        } else {
                                                                                            ((((self::ml_string::Text::from_units(vec![40, 73, 110, 116, 46, 111, 102, 78, 97, 116, 32, 40, 78, 97, 116, 46, 120, 111, 114, 32])).concat(&am)).concat(&self::ml_string::Text::from_units(vec![32]))).concat(&bm)).concat(&self::ml_string::Text::from_units(vec![41, 41]))
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let left_negative = if (code == (0f64)) {
                                                                                            (((((self::ml_string::Text::from_units(vec![40, 73, 110, 116, 46, 111, 102, 78, 97, 116, 32])).concat(&difference)).concat(&bp)).concat(&self::ml_string::Text::from_units(vec![32]))).concat(&am)).concat(&self::ml_string::Text::from_units(vec![41, 41]))
                                                                                        } else {
                                                                                            if (code == (1f64)) {
                                                                                                (((((self::ml_string::Text::from_units(vec![40, 45, 49, 32, 45, 32, 73, 110, 116, 46, 111, 102, 78, 97, 116, 32])).concat(&difference)).concat(&am)).concat(&self::ml_string::Text::from_units(vec![32]))).concat(&bp)).concat(&self::ml_string::Text::from_units(vec![41, 41]))
                                                                                            } else {
                                                                                                ((((self::ml_string::Text::from_units(vec![40, 45, 49, 32, 45, 32, 73, 110, 116, 46, 111, 102, 78, 97, 116, 32, 40, 78, 97, 116, 46, 120, 111, 114, 32])).concat(&am)).concat(&self::ml_string::Text::from_units(vec![32]))).concat(&bp)).concat(&self::ml_string::Text::from_units(vec![41, 41]))
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let right_negative = if (code == (0f64)) {
                                                                                                (((((self::ml_string::Text::from_units(vec![40, 73, 110, 116, 46, 111, 102, 78, 97, 116, 32])).concat(&difference)).concat(&ap)).concat(&self::ml_string::Text::from_units(vec![32]))).concat(&bm)).concat(&self::ml_string::Text::from_units(vec![41, 41]))
                                                                                            } else {
                                                                                                if (code == (1f64)) {
                                                                                                    (((((self::ml_string::Text::from_units(vec![40, 45, 49, 32, 45, 32, 73, 110, 116, 46, 111, 102, 78, 97, 116, 32])).concat(&difference)).concat(&bm)).concat(&self::ml_string::Text::from_units(vec![32]))).concat(&ap)).concat(&self::ml_string::Text::from_units(vec![41, 41]))
                                                                                                } else {
                                                                                                    ((((self::ml_string::Text::from_units(vec![40, 45, 49, 32, 45, 32, 73, 110, 116, 46, 111, 102, 78, 97, 116, 32, 40, 78, 97, 116, 46, 120, 111, 114, 32])).concat(&ap)).concat(&self::ml_string::Text::from_units(vec![32]))).concat(&bm)).concat(&self::ml_string::Text::from_units(vec![41, 41]))
                                                                                                }
                                                                                            };
                                                                                            {
                                                                                                let positive = ((((((self::ml_string::Text::from_units(vec![40, 73, 110, 116, 46, 111, 102, 78, 97, 116, 32, 40])).concat(&if (code == (0f64)) {
                                                                                                    self::ml_string::Text::from_units(vec![78, 97, 116, 46, 108, 97, 110, 100])
                                                                                                } else {
                                                                                                    if (code == (1f64)) {
                                                                                                        self::ml_string::Text::from_units(vec![78, 97, 116, 46, 108, 111, 114])
                                                                                                    } else {
                                                                                                        self::ml_string::Text::from_units(vec![78, 97, 116, 46, 120, 111, 114])
                                                                                                    }
                                                                                                })).concat(&self::ml_string::Text::from_units(vec![32]))).concat(&ap)).concat(&self::ml_string::Text::from_units(vec![32]))).concat(&bp)).concat(&self::ml_string::Text::from_units(vec![41, 41]));
                                                                                                ((((((((((((((prefix.concat(&self::ml_string::Text::from_units(vec![105, 102, 32]))).concat(&a)).concat(&self::ml_string::Text::from_units(vec![32, 60, 32, 48, 32, 116, 104, 101, 110, 32, 40, 105, 102, 32]))).concat(&b)).concat(&self::ml_string::Text::from_units(vec![32, 60, 32, 48, 32, 116, 104, 101, 110, 32]))).concat(&both)).concat(&self::ml_string::Text::from_units(vec![32, 101, 108, 115, 101, 32]))).concat(&left_negative)).concat(&self::ml_string::Text::from_units(vec![41, 32, 101, 108, 115, 101, 32, 40, 105, 102, 32]))).concat(&b)).concat(&self::ml_string::Text::from_units(vec![32, 60, 32, 48, 32, 116, 104, 101, 110, 32]))).concat(&right_negative)).concat(&self::ml_string::Text::from_units(vec![32, 101, 108, 115, 101, 32]))).concat(&positive)).concat(&self::ml_string::Text::from_units(vec![41, 41]))
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
                        } else {
                            if ((*target) == (self::ml_string::Text::from_units(vec![82, 117, 115, 116]))) {
                                ((((((self::ml_string::Text::from_units(vec![99, 114, 97, 116, 101, 58, 58, 109, 108, 95, 98, 105, 103, 95, 105, 110, 116, 101, 103, 101, 114, 95, 98, 105, 116, 119, 105, 115, 101, 58, 58, 97, 112, 112, 108, 121, 95, 98, 105, 103, 95, 105, 110, 116, 101, 103, 101, 114, 95, 98, 105, 116, 119, 105, 115, 101, 95, 111, 112, 101, 114, 97, 116, 105, 111, 110, 40])).concat(&self::ml_string::Text::from_text(&self::ml_number::js_number(code)))).concat(&self::ml_string::Text::from_units(vec![102, 54, 52, 44, 32]))).concat(&left)).concat(&self::ml_string::Text::from_units(vec![44, 32]))).concat(&if (code == (6f64)) {
                                    self::ml_string::Text::from_units(vec![109, 108, 58, 58, 66, 105, 103, 58, 58, 122, 101, 114, 111, 40, 41])
                                } else {
                                    right.clone()
                                })).concat(&self::ml_string::Text::from_units(vec![41]))
                            } else {
                                if ((*target) != (self::ml_string::Text::from_units(vec![74, 97, 118, 97, 83, 99, 114, 105, 112, 116]))) {
                                    self::ml_string::Text::from_units(vec![])
                                } else {
                                    if (code == (6f64)) {
                                        ((self::ml_string::Text::from_units(vec![40, 126, 40])).concat(&left)).concat(&self::ml_string::Text::from_units(vec![41, 41]))
                                    } else {
                                        {
                                            let symbol = if (code == (0f64)) {
                                                self::ml_string::Text::from_units(vec![38])
                                            } else {
                                                if (code == (1f64)) {
                                                    self::ml_string::Text::from_units(vec![124])
                                                } else {
                                                    if (code == (2f64)) {
                                                        self::ml_string::Text::from_units(vec![94])
                                                    } else {
                                                        if (code == (3f64)) {
                                                            self::ml_string::Text::from_units(vec![60, 60])
                                                        } else {
                                                            self::ml_string::Text::from_units(vec![62, 62])
                                                        }
                                                    }
                                                }
                                            };
                                            ((((((self::ml_string::Text::from_units(vec![40, 40])).concat(&left)).concat(&self::ml_string::Text::from_units(vec![41, 32]))).concat(&symbol)).concat(&self::ml_string::Text::from_units(vec![32, 40]))).concat(&right)).concat(&self::ml_string::Text::from_units(vec![41, 41]))
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
pub fn read_big_integer_bitwise_statement() -> self::ml_string::Text {
    self::ml_string::Text::from_units(vec![66, 105, 103, 73, 110, 116, 32, 98, 105, 116, 119, 105, 115, 101, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 115, 32, 117, 115, 101, 32, 101, 120, 97, 99, 116, 32, 117, 110, 98, 111, 117, 110, 100, 101, 100, 32, 105, 110, 116, 101, 103, 101, 114, 115, 32, 119, 105, 116, 104, 32, 105, 110, 102, 105, 110, 105, 116, 101, 32, 115, 105, 103, 110, 32, 101, 120, 116, 101, 110, 115, 105, 111, 110, 44, 32, 110, 101, 103, 97, 116, 105, 118, 101, 32, 115, 104, 105, 102, 116, 32, 114, 101, 118, 101, 114, 115, 97, 108, 32, 97, 110, 100, 32, 97, 114, 105, 116, 104, 109, 101, 116, 105, 99, 32, 114, 105, 103, 104, 116, 32, 115, 104, 105, 102, 116, 115, 59, 32, 111, 112, 101, 114, 97, 110, 100, 115, 32, 97, 114, 101, 32, 101, 118, 97, 108, 117, 97, 116, 101, 100, 32, 111, 110, 99, 101, 32, 105, 110, 32, 115, 111, 117, 114, 99, 101, 32, 111, 114, 100, 101, 114, 32, 119, 105, 116, 104, 111, 117, 116, 32, 78, 117, 109, 98, 101, 114, 32, 99, 111, 101, 114, 99, 105, 111, 110, 32, 111, 114, 32, 109, 111, 100, 117, 108, 111, 45, 51, 50, 32, 115, 104, 105, 102, 116, 32, 99, 111, 117, 110, 116, 115, 46, 32, 78, 97, 116, 105, 118, 101, 32, 115, 111, 117, 114, 99, 101, 32, 97, 110, 100, 32, 116, 104, 101, 32, 101, 109, 105, 116, 116, 101, 100, 32, 105, 110, 116, 101, 103, 101, 114, 32, 114, 117, 110, 116, 105, 109, 101, 32, 97, 114, 101, 32, 103, 101, 110, 101, 114, 97, 116, 101, 100, 32, 102, 114, 111, 109, 32, 116, 104, 101, 32, 115, 97, 109, 101, 32, 99, 104, 101, 99, 107, 101, 100, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110, 46])
}
