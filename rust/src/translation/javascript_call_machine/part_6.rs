// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptDescriptorConversionState;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptPrimitiveConversionState;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryAcceptStringNumberInfinityLoop28Result;
use super::MlSource1TemporaryApplyBigIntegerBitwiseOperationJoin5;
use super::MlSource1TemporaryApplyBigIntegerBitwiseOperationLoop8Result;
use super::MlSource1TemporaryConvertStringNumberRationalJoin20;
use super::MlSource1TemporaryConvertStringNumberRationalJoin21;
use super::MlSource1TemporaryConvertStringNumberRationalLoop22Result;
use super::MlSource1TemporaryConvertStringNumberRationalLoop23Result;
use super::MlSource1TemporaryConvertStringNumberRationalLoop26Result;
use super::MlSource1TemporaryConvertStringNumberRationalLoop27Result;
use super::MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop1Result;
use super::MlSource1TemporaryReadStringNumberBinaryPowerLoop18Result;
use super::MlSource1TemporaryTestJavaScriptSemanticSameValueLoop2Result;
use super::ml;
use super::ml_source_exception;
use super::ml_string;
use super::source_ml_source1_temporary_accept_string_number_infinity_loop28;
use super::source_ml_source1_temporary_apply_big_integer_bitwise_operation_loop6;
use super::source_ml_source1_temporary_apply_big_integer_bitwise_operation_loop7;
use super::source_ml_source1_temporary_apply_big_integer_bitwise_operation_loop8;
use super::source_ml_source1_temporary_apply_java_script_number_bitwise_operation_loop10;
use super::source_ml_source1_temporary_apply_java_script_number_bitwise_operation_loop11;
use super::source_ml_source1_temporary_convert_string_number_rational_loop22;
use super::source_ml_source1_temporary_convert_string_number_rational_loop23;
use super::source_ml_source1_temporary_convert_string_number_rational_loop24;
use super::source_ml_source1_temporary_convert_string_number_rational_loop25;
use super::source_ml_source1_temporary_convert_string_number_rational_loop26;
use super::source_ml_source1_temporary_convert_string_number_rational_loop27;
use super::source_ml_source1_temporary_read_java_script_semantic_value_error_loop1;
use super::source_ml_source1_temporary_read_string_number_binary_power_loop18;
use super::source_ml_source1_temporary_read_string_number_binary_power_loop19;
use super::source_ml_source1_temporary_read_string_number_decimal_power_loop16;
use super::source_ml_source1_temporary_read_string_number_exact_mantissa_loop17;
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
                    _ => {
                        false
                    }
                }
            }
            self::JavaScriptSemanticValue::Null => {
                match right.clone() {
                    self::JavaScriptSemanticValue::Null => {
                        true
                    }
                    _ => {
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
                            let factor_2 = self::source_ml_source1_temporary_apply_java_script_number_bitwise_operation_loop10(count, factor, position);
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
                                        let result_2 = self::source_ml_source1_temporary_apply_java_script_number_bitwise_operation_loop11(operation, first, second, result, weight, position_2);
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
                let value_2 = self::source_ml_source1_temporary_read_string_number_decimal_power_loop16(exponent, value.clone(), base.clone());
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
                let result_2 = self::source_ml_source1_temporary_read_string_number_exact_mantissa_loop17(value.clone(), result, scale);
                result_2
            }
        }
    }
}

pub fn read_string_number_binary_power(exponent: f64) -> f64 {
    {
        let result = 1f64;
        {
            let source_ml_source2_s112 = self::source_ml_source1_temporary_read_string_number_binary_power_loop18(exponent, result);
            match source_ml_source2_s112.clone() {
                self::MlSource1TemporaryReadStringNumberBinaryPowerLoop18Result::MlSource1TemporaryReadStringNumberBinaryPowerLoop18Done(exponent_2, result_2) => {
                    {
                        let result_3 = self::source_ml_source1_temporary_read_string_number_binary_power_loop19(exponent_2, result_2);
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
                    let source_ml_source2_s115 = if (numerator >= denominator) {
                        {
                            let scale = denominator.clone();
                            {
                                let source_ml_source2_s116 = self::source_ml_source1_temporary_convert_string_number_rational_loop26(numerator.clone(), exponent, scale.clone());
                                match source_ml_source2_s116.clone() {
                                    self::MlSource1TemporaryConvertStringNumberRationalLoop26Result::MlSource1TemporaryConvertStringNumberRationalLoop26Done(exponent_2) => {
                                        self::MlSource1TemporaryConvertStringNumberRationalJoin20::MlSource1TemporaryConvertStringNumberRationalJoin20Next(exponent_2)
                                    }
                                    self::MlSource1TemporaryConvertStringNumberRationalLoop26Result::MlSource1TemporaryConvertStringNumberRationalLoop26Return(source_ml_source1_temporary_result26) => {
                                        self::MlSource1TemporaryConvertStringNumberRationalJoin20::MlSource1TemporaryConvertStringNumberRationalJoin20Return(source_ml_source1_temporary_result26)
                                    }
                                }
                            }
                        }
                    } else {
                        {
                            let scale_2 = numerator.clone();
                            {
                                let source_ml_source2_s117 = self::source_ml_source1_temporary_convert_string_number_rational_loop27(denominator.clone(), exponent, scale_2.clone());
                                match source_ml_source2_s117.clone() {
                                    self::MlSource1TemporaryConvertStringNumberRationalLoop27Result::MlSource1TemporaryConvertStringNumberRationalLoop27Done(exponent_3) => {
                                        self::MlSource1TemporaryConvertStringNumberRationalJoin20::MlSource1TemporaryConvertStringNumberRationalJoin20Next(exponent_3)
                                    }
                                    self::MlSource1TemporaryConvertStringNumberRationalLoop27Result::MlSource1TemporaryConvertStringNumberRationalLoop27Return(source_ml_source1_temporary_result27) => {
                                        self::MlSource1TemporaryConvertStringNumberRationalJoin20::MlSource1TemporaryConvertStringNumberRationalJoin20Return(source_ml_source1_temporary_result27)
                                    }
                                }
                            }
                        }
                    };
                    match source_ml_source2_s115.clone() {
                        self::MlSource1TemporaryConvertStringNumberRationalJoin20::MlSource1TemporaryConvertStringNumberRationalJoin20Next(exponent_4) => {
                            {
                                let shift = if (exponent_4 >= ((-1022f64))) {
                                    (52f64 - exponent_4)
                                } else {
                                    1074f64
                                };
                                {
                                    let source_ml_source2_s118 = if (shift >= (0f64)) {
                                        {
                                            let index = 0f64;
                                            {
                                                let numerator_2 = self::source_ml_source1_temporary_convert_string_number_rational_loop24(numerator.clone(), shift, index);
                                                self::MlSource1TemporaryConvertStringNumberRationalJoin21::MlSource1TemporaryConvertStringNumberRationalJoin21Next(numerator_2.clone(), denominator.clone())
                                            }
                                        }
                                    } else {
                                        {
                                            let index_2 = 0f64;
                                            {
                                                let denominator_2 = self::source_ml_source1_temporary_convert_string_number_rational_loop25(denominator.clone(), shift, index_2);
                                                self::MlSource1TemporaryConvertStringNumberRationalJoin21::MlSource1TemporaryConvertStringNumberRationalJoin21Next(numerator.clone(), denominator_2.clone())
                                            }
                                        }
                                    };
                                    match source_ml_source2_s118.clone() {
                                        self::MlSource1TemporaryConvertStringNumberRationalJoin21::MlSource1TemporaryConvertStringNumberRationalJoin21Next(numerator_3, denominator_3) => {
                                            {
                                                let quotient = self::ml::Big::from_i128(0);
                                                {
                                                    let remainder = numerator_3.clone();
                                                    {
                                                        let scale_3 = denominator_3.clone();
                                                        {
                                                            let weight = self::ml::Big::from_i128(1);
                                                            {
                                                                let source_ml_source2_s119 = self::source_ml_source1_temporary_convert_string_number_rational_loop22(remainder.clone(), scale_3.clone(), weight.clone());
                                                                match source_ml_source2_s119.clone() {
                                                                    self::MlSource1TemporaryConvertStringNumberRationalLoop22Result::MlSource1TemporaryConvertStringNumberRationalLoop22Done(scale_4, weight_2) => {
                                                                        {
                                                                            let source_ml_source2_s120 = self::source_ml_source1_temporary_convert_string_number_rational_loop23(quotient.clone(), remainder.clone(), scale_4.clone(), weight_2.clone());
                                                                            match source_ml_source2_s120.clone() {
                                                                                self::MlSource1TemporaryConvertStringNumberRationalLoop23Result::MlSource1TemporaryConvertStringNumberRationalLoop23Done(quotient_2, remainder_2) => {
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
                        self::MlSource1TemporaryConvertStringNumberRationalJoin20::MlSource1TemporaryConvertStringNumberRationalJoin20Return(source_ml_source1_temporary_result25) => {
                            source_ml_source1_temporary_result25
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
                    let source_ml_source2_s121 = self::source_ml_source1_temporary_accept_string_number_infinity_loop28(units, start, expected, index);
                    match source_ml_source2_s121.clone() {
                        self::MlSource1TemporaryAcceptStringNumberInfinityLoop28Result::MlSource1TemporaryAcceptStringNumberInfinityLoop28Done => {
                            true
                        }
                        self::MlSource1TemporaryAcceptStringNumberInfinityLoop28Result::MlSource1TemporaryAcceptStringNumberInfinityLoop28Return(source_ml_source1_temporary_result28) => {
                            source_ml_source1_temporary_result28
                        }
                    }
                }
            }
        }
    }
}
