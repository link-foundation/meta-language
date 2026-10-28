// Generated from complete js/src/translation/javascript-call-source.js. Do not edit by hand.
use super::JavaScriptCallInstruction;
use super::JavaScriptCallProgram;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryApplyBigIntegerBitwiseOperationJoin6;
use super::MlSource1TemporaryApplyBigIntegerBitwiseOperationLoop9Result;
use super::MlSource1TemporaryFindJavaScriptCallCodeFunctionLoop3Result;
use super::MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop1Result;
use super::MlSource1TemporaryReadPrimitiveBitwiseDomainLoop5Result;
use super::MlSource1TemporaryTestJavaScriptSemanticSameValueLoop2Result;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::ml_source_exception;
use super::source_ml_source1_temporary_apply_big_integer_bitwise_operation_loop7;
use super::source_ml_source1_temporary_apply_big_integer_bitwise_operation_loop8;
use super::source_ml_source1_temporary_apply_big_integer_bitwise_operation_loop9;
use super::source_ml_source1_temporary_find_java_script_call_code_function_loop3;
use super::source_ml_source1_temporary_read_java_script_semantic_value_error_loop1;
use super::source_ml_source1_temporary_read_primitive_bitwise_domain_loop5;
use super::source_ml_source1_temporary_test_java_script_semantic_same_value_loop2;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_semantic_value_error(value: self::JavaScriptSemanticValue) -> String {
    match value.clone() {
        self::JavaScriptSemanticValue::Undefined => {
            String::from("")
        }
        self::JavaScriptSemanticValue::Null => {
            String::from("")
        }
        self::JavaScriptSemanticValue::Boolean(source_ml_source2_f1) => {
            String::from("")
        }
        self::JavaScriptSemanticValue::Number(source_ml_source2_f2) => {
            String::from("")
        }
        self::JavaScriptSemanticValue::Integer(source_ml_source2_f3) => {
            String::from("")
        }
        self::JavaScriptSemanticValue::Text(units) => {
            if (((units.len() as f64)) > (1048576f64)) {
                String::from("semantic text exceeds the explicit UTF16 unit budget")
            } else {
                {
                    let at = 0f64;
                    {
                        let source_ml_source2_s5 = self::source_ml_source1_temporary_read_java_script_semantic_value_error_loop1(&(units), at);
                        match source_ml_source2_s5.clone() {
                            self::MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop1Result::MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop1Done => {
                                String::from("")
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
                String::from("")
            } else {
                String::from("a symbol requires a positive unsigned64 global link identity")
            }
        }
        self::JavaScriptSemanticValue::Object(identity_2, source_ml_source2_f4) => {
            if ((identity_2 > (self::ml::Big::from_i128(0))) && (identity_2 <= (self::ml::Big::from_i128(18446744073709551615)))) {
                String::from("")
            } else {
                String::from("an object requires a positive unsigned64 global link identity")
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_semantic_same_value(left: self::JavaScriptSemanticValue, right: self::JavaScriptSemanticValue) -> bool {
    if (((self::read_java_script_semantic_value_error(left.clone())) != "") || ((self::read_java_script_semantic_value_error(right.clone())) != "")) {
        (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (String::from("SameValue requires two complete values in the declared semantic model")).encode_utf16().collect(), message_present: true }).raise()
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
    if ((self::read_java_script_semantic_value_error(value.clone())) != "") {
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
pub fn test_java_script_call_code_coordinate(value: f64, limit: f64) -> bool {
    (((self::ml_math::is_safe_integer(value) && (value >= (0f64))) && (value < limit)) && ((value != (0f64)) || (((1f64 / value)) > (0f64))))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn find_java_script_call_code_function(program: self::JavaScriptCallProgram, identity: self::ml::Big) -> f64 {
    {
        let low = 0f64;
        {
            let high = ((match &program { self::JavaScriptCallProgram::CallProgram(source_ml_source2_data_property_value107, _, _) => source_ml_source2_data_property_value107.as_slice() }).len() as f64);
            {
                let source_ml_source2_s108 = self::source_ml_source1_temporary_find_java_script_call_code_function_loop3(program.clone(), identity.clone(), low, high);
                match source_ml_source2_s108.clone() {
                    self::MlSource1TemporaryFindJavaScriptCallCodeFunctionLoop3Result::MlSource1TemporaryFindJavaScriptCallCodeFunctionLoop3Done => {
                        (-1f64)
                    }
                    self::MlSource1TemporaryFindJavaScriptCallCodeFunctionLoop3Result::MlSource1TemporaryFindJavaScriptCallCodeFunctionLoop3Return(source_ml_source1_temporary_result3) => {
                        source_ml_source1_temporary_result3
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
        self::JavaScriptCallInstruction::Constant(destination, source_ml_source2_f109) => {
            let source_ml_source2_f109 = *source_ml_source2_f109;
            vec![destination]
        }
        self::JavaScriptCallInstruction::Copy(destination_2, source) => {
            vec![destination_2, source]
        }
        self::JavaScriptCallInstruction::Argument(destination_3, source_ml_source2_f110) => {
            vec![destination_3]
        }
        self::JavaScriptCallInstruction::Jump(source_ml_source2_f111) => {
            Vec::<f64>::new()
        }
        self::JavaScriptCallInstruction::Branch(condition, source_ml_source2_f112, source_ml_source2_f113) => {
            vec![condition]
        }
        self::JavaScriptCallInstruction::Call(destination_4, callee, receiver, arguments) => {
            {
                let source_ml_source2_concat_value119 = vec![destination_4, callee, receiver];
                {
                    let source_ml_source2_concat_value120 = arguments;
                    self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value119), source_ml_source2_concat_value120)
                }
            }
        }
        self::JavaScriptCallInstruction::Return(source_2) => {
            vec![source_2]
        }
        self::JavaScriptCallInstruction::Throw(source_3) => {
            vec![source_3]
        }
        self::JavaScriptCallInstruction::PushHandler(destination_5, source_ml_source2_f114) => {
            vec![destination_5]
        }
        self::JavaScriptCallInstruction::PopHandler => {
            Vec::<f64>::new()
        }
        self::JavaScriptCallInstruction::NumberBinary(destination_6, left, right, source_ml_source2_f115) => {
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
        self::JavaScriptCallInstruction::Primitive(destination_9, source_ml_source2_f116, arguments_2) => {
            {
                let source_ml_source2_concat_value121 = vec![destination_9];
                {
                    let source_ml_source2_concat_value122 = arguments_2;
                    self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value121), source_ml_source2_concat_value122)
                }
            }
        }
        self::JavaScriptCallInstruction::OrdinaryGet(destination_10, object, key, receiver_2) => {
            vec![destination_10, object, key, receiver_2]
        }
        self::JavaScriptCallInstruction::OrdinarySet(destination_11, object_2, key_2, value, receiver_3) => {
            vec![destination_11, object_2, key_2, value, receiver_3]
        }
        self::JavaScriptCallInstruction::PropertyKey(destination_12, input, source_ml_source2_f117) => {
            vec![destination_12, input]
        }
        self::JavaScriptCallInstruction::PropertyDefinition(destination_13, target, key_3, attributes, source_ml_source2_f118) => {
            vec![destination_13, target, key_3, attributes]
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_call_constant_error(value: self::JavaScriptSemanticValue) -> String {
    self::read_java_script_semantic_value_error(value.clone())
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_call_strict_equal(left: self::JavaScriptSemanticValue, right: self::JavaScriptSemanticValue) -> bool {
    if (((self::read_java_script_semantic_value_error(left.clone())) != "") || ((self::read_java_script_semantic_value_error(right.clone())) != "")) {
        (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (String::from("strict equality requires complete semantic operands")).encode_utf16().collect(), message_present: true }).raise()
    } else {
        match left.clone() {
            self::JavaScriptSemanticValue::Undefined => {
                self::test_java_script_semantic_same_value(left.clone(), right.clone())
            }
            self::JavaScriptSemanticValue::Null => {
                self::test_java_script_semantic_same_value(left.clone(), right.clone())
            }
            self::JavaScriptSemanticValue::Boolean(source_ml_source2_f123) => {
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
                    self::JavaScriptSemanticValue::Boolean(source_ml_source2_f129) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Number(value) => {
                        (left_value == value)
                    }
                    self::JavaScriptSemanticValue::Integer(source_ml_source2_f130) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Text(source_ml_source2_f131) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Symbol(source_ml_source2_f132) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Object(source_ml_source2_f133, source_ml_source2_f134) => {
                        false
                    }
                }
            }
            self::JavaScriptSemanticValue::Integer(source_ml_source2_f124) => {
                self::test_java_script_semantic_same_value(left.clone(), right.clone())
            }
            self::JavaScriptSemanticValue::Text(source_ml_source2_f125) => {
                self::test_java_script_semantic_same_value(left.clone(), right.clone())
            }
            self::JavaScriptSemanticValue::Symbol(source_ml_source2_f126) => {
                self::test_java_script_semantic_same_value(left.clone(), right.clone())
            }
            self::JavaScriptSemanticValue::Object(source_ml_source2_f127, source_ml_source2_f128) => {
                self::test_java_script_semantic_same_value(left.clone(), right.clone())
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_big_integer_bitwise_operation_code(operation: &str) -> f64 {
    if (operation == "conjoinBigIntegerBits") {
        0f64
    } else {
        if (operation == "disjoinBigIntegerBits") {
            1f64
        } else {
            if (operation == "excludeEqualBigIntegerBits") {
                2f64
            } else {
                if (operation == "shiftBigIntegerBitsLeft") {
                    3f64
                } else {
                    if (operation == "shiftBigIntegerBitsRight") {
                        4f64
                    } else {
                        if (operation == "complementBigIntegerBits") {
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
pub fn read_big_integer_bitwise_operation(operation: &str) -> String {
    if (operation == "conjoinJavaScriptNumberBits") {
        String::from("conjoinBigIntegerBits")
    } else {
        if (operation == "disjoinJavaScriptNumberBits") {
            String::from("disjoinBigIntegerBits")
        } else {
            if (operation == "excludeEqualJavaScriptNumberBits") {
                String::from("excludeEqualBigIntegerBits")
            } else {
                if (operation == "shiftJavaScriptNumberBitsLeft") {
                    String::from("shiftBigIntegerBitsLeft")
                } else {
                    if (operation == "shiftJavaScriptNumberBitsRight") {
                        String::from("shiftBigIntegerBitsRight")
                    } else {
                        if (operation == "complementJavaScriptNumberBits") {
                            String::from("complementBigIntegerBits")
                        } else {
                            if ((self::read_big_integer_bitwise_operation_code(operation)) < (0f64)) {
                                String::from("")
                            } else {
                                operation.to_owned()
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
pub fn read_big_integer_bitwise_arity(operation: &str) -> f64 {
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
pub fn accept_linked_big_integer_bitwise_operation(expression: &str, operation: &str) -> bool {
    ((expression == "math") && ((self::read_big_integer_bitwise_arity(operation)) != (0f64)))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_primitive_bitwise_domain(kinds: &[String]) -> String {
    if (((kinds.len() as f64)) == (0f64)) {
        String::from("")
    } else {
        {
            let number = true;
            {
                let integer = true;
                {
                    let source_ml_source1_temporary_values4 = kinds.to_vec();
                    {
                        let source_ml_source1_temporary_index4 = self::ml::Big::from_i128(0);
                        {
                            let source_ml_source2_s135 = self::source_ml_source1_temporary_read_primitive_bitwise_domain_loop5(number, integer, &(source_ml_source1_temporary_values4), source_ml_source1_temporary_index4.clone());
                            match source_ml_source2_s135.clone() {
                                self::MlSource1TemporaryReadPrimitiveBitwiseDomainLoop5Result::MlSource1TemporaryReadPrimitiveBitwiseDomainLoop5Done(number_2, integer_2) => {
                                    if number_2 {
                                        String::from("Number")
                                    } else {
                                        if integer_2 {
                                            String::from("BigInt")
                                        } else {
                                            String::from("")
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
                            let source_ml_source2_s136 = if (count < (self::ml::Big::from_i128(0))) {
                                {
                                    let count_2 = count.neg();
                                    {
                                        let forward_2 = !forward;
                                        self::MlSource1TemporaryApplyBigIntegerBitwiseOperationJoin6::MlSource1TemporaryApplyBigIntegerBitwiseOperationJoin6Next(count_2.clone(), forward_2)
                                    }
                                }
                            } else {
                                self::MlSource1TemporaryApplyBigIntegerBitwiseOperationJoin6::MlSource1TemporaryApplyBigIntegerBitwiseOperationJoin6Next(count.clone(), forward)
                            };
                            match source_ml_source2_s136.clone() {
                                self::MlSource1TemporaryApplyBigIntegerBitwiseOperationJoin6::MlSource1TemporaryApplyBigIntegerBitwiseOperationJoin6Next(count_3, forward_3) => {
                                    {
                                        let value = left.clone();
                                        {
                                            let value_4 = if forward_3 {
                                                {
                                                    let factor = self::ml::Big::from_i128(2);
                                                    {
                                                        let value_2 = self::source_ml_source1_temporary_apply_big_integer_bitwise_operation_loop7(count_3.clone(), value.clone(), factor.clone());
                                                        value_2.clone()
                                                    }
                                                }
                                            } else {
                                                {
                                                    let value_3 = self::source_ml_source1_temporary_apply_big_integer_bitwise_operation_loop8(count_3.clone(), value.clone());
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
                (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (String::from("an unknown BigInt bitwise operation")).encode_utf16().collect(), message_present: true }).raise()
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
                                            let source_ml_source2_s137 = self::source_ml_source1_temporary_apply_big_integer_bitwise_operation_loop9(operation, negative_left, negative_right, first.clone(), second.clone(), weight.clone(), result.clone());
                                            match source_ml_source2_s137.clone() {
                                                self::MlSource1TemporaryApplyBigIntegerBitwiseOperationLoop9Result::MlSource1TemporaryApplyBigIntegerBitwiseOperationLoop9Done(weight_2, result_2) => {
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
