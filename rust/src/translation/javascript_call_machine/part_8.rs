// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptCallInstruction;
use super::JavaScriptCallPrimitiveResult;
use super::JavaScriptCallProgram;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptOrdinaryProperty;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryExecuteJavaScriptCallPrimitiveLoop92Result;
use super::MlSource1TemporaryFindJavaScriptCallCodeFunctionLoop93Result;
use super::MlSource1TemporaryFindJavaScriptOrdinaryObjectLoop96Result;
use super::MlSource1TemporaryFindJavaScriptOrdinaryPropertyLoop98Result;
use super::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin94;
use super::MlSource1TemporaryTestJavaScriptOrdinarySymbolLoop97Result;
use super::execute_java_script_call_integer_primitive;
use super::execute_java_script_call_numeric_primitive;
use super::execute_java_script_call_text_primitive;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::ml_source_exception;
use super::ml_string;
use super::read_java_script_call_integer_arity;
use super::read_java_script_call_primitive_arity_error;
use super::read_java_script_semantic_value_error;
use super::source_ml_source1_temporary_execute_java_script_call_primitive_loop92;
use super::source_ml_source1_temporary_find_java_script_call_code_function_loop93;
use super::source_ml_source1_temporary_find_java_script_ordinary_object_loop96;
use super::source_ml_source1_temporary_find_java_script_ordinary_property_loop98;
use super::source_ml_source1_temporary_test_java_script_ordinary_symbol_loop97;
use super::test_java_script_semantic_accessor;
use super::test_java_script_semantic_same_value;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn execute_java_script_call_primitive(operation: &self::ml_string::Text, values: Vec<self::JavaScriptSemanticValue>) -> self::JavaScriptCallPrimitiveResult {
    {
        let arity = self::read_java_script_call_primitive_arity_error(operation, (values.len() as f64));
        if (arity != (self::ml_string::Text::from_units(vec![]))) {
            self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(arity.clone())
        } else {
            {
                let source_ml_source1_temporary_values91 = values.clone();
                {
                    let source_ml_source1_temporary_index91 = self::ml::Big::from_i128(0);
                    {
                        let source_ml_source2_s233 = self::source_ml_source1_temporary_execute_java_script_call_primitive_loop92(source_ml_source1_temporary_values91, source_ml_source1_temporary_index91.clone());
                        match source_ml_source2_s233.clone() {
                            self::MlSource1TemporaryExecuteJavaScriptCallPrimitiveLoop92Result::MlSource1TemporaryExecuteJavaScriptCallPrimitiveLoop92Done => {
                                if ((self::read_java_script_call_integer_arity(operation)) != (0f64)) {
                                    self::execute_java_script_call_integer_primitive(operation, values)
                                } else {
                                    if (((((*operation) == (self::ml_string::Text::from_units(vec![116, 114, 117, 110, 99, 97, 116, 101, 32, 110, 117, 109, 98, 101, 114]))) || ((*operation) == (self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 109, 105, 110, 105, 109, 117, 109, 32, 110, 117, 109, 98, 101, 114])))) || ((*operation) == (self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 109, 97, 120, 105, 109, 117, 109, 32, 110, 117, 109, 98, 101, 114])))) || ((*operation) == (self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116, 32, 116, 101, 120, 116, 32, 117, 110, 105, 116, 115])))) {
                                        self::execute_java_script_call_numeric_primitive(operation, values)
                                    } else {
                                        self::execute_java_script_call_text_primitive(operation, values)
                                    }
                                }
                            }
                            self::MlSource1TemporaryExecuteJavaScriptCallPrimitiveLoop92Result::MlSource1TemporaryExecuteJavaScriptCallPrimitiveLoop92Return(source_ml_source1_temporary_result92) => {
                                let source_ml_source1_temporary_result92 = *source_ml_source1_temporary_result92;
                                source_ml_source1_temporary_result92.clone()
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
pub fn test_java_script_call_code_coordinate(value: f64, limit: f64) -> bool {
    (((self::ml_math::is_safe_integer(value) && (value >= (0f64))) && (value < limit)) && ((value != (0f64)) || (((1f64 / value)) > (0f64))))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn find_java_script_call_code_function(program: self::JavaScriptCallProgram, identity: self::ml::Big) -> f64 {
    {
        let low = 0f64;
        {
            let high = ((match &program { self::JavaScriptCallProgram::CallProgram(source_ml_source2_data_property_value234, _, _) => source_ml_source2_data_property_value234.as_slice() }).len() as f64);
            {
                let source_ml_source2_s235 = self::source_ml_source1_temporary_find_java_script_call_code_function_loop93(program.clone(), identity.clone(), low, high);
                match source_ml_source2_s235.clone() {
                    self::MlSource1TemporaryFindJavaScriptCallCodeFunctionLoop93Result::MlSource1TemporaryFindJavaScriptCallCodeFunctionLoop93Done => {
                        (-1f64)
                    }
                    self::MlSource1TemporaryFindJavaScriptCallCodeFunctionLoop93Result::MlSource1TemporaryFindJavaScriptCallCodeFunctionLoop93Return(source_ml_source1_temporary_result93) => {
                        source_ml_source1_temporary_result93
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
        self::JavaScriptCallInstruction::Constant(destination, source_ml_source2_f236) => {
            let source_ml_source2_f236 = *source_ml_source2_f236;
            vec![destination]
        }
        self::JavaScriptCallInstruction::Copy(destination_2, source) => {
            vec![destination_2, source]
        }
        self::JavaScriptCallInstruction::Argument(destination_3, source_ml_source2_f237) => {
            vec![destination_3]
        }
        self::JavaScriptCallInstruction::Jump(source_ml_source2_f238) => {
            Vec::<f64>::new()
        }
        self::JavaScriptCallInstruction::Branch(condition, source_ml_source2_f239, source_ml_source2_f240) => {
            vec![condition]
        }
        self::JavaScriptCallInstruction::Call(destination_4, callee, receiver, arguments) => {
            {
                let source_ml_source2_concat_value246 = vec![destination_4, callee, receiver];
                {
                    let source_ml_source2_concat_value247 = arguments;
                    self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value246), source_ml_source2_concat_value247)
                }
            }
        }
        self::JavaScriptCallInstruction::Return(source_2) => {
            vec![source_2]
        }
        self::JavaScriptCallInstruction::Throw(source_3) => {
            vec![source_3]
        }
        self::JavaScriptCallInstruction::PushHandler(destination_5, source_ml_source2_f241) => {
            vec![destination_5]
        }
        self::JavaScriptCallInstruction::PopHandler => {
            Vec::<f64>::new()
        }
        self::JavaScriptCallInstruction::NumberBinary(destination_6, left, right, source_ml_source2_f242) => {
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
        self::JavaScriptCallInstruction::Primitive(destination_9, source_ml_source2_f243, arguments_2) => {
            {
                let source_ml_source2_concat_value248 = vec![destination_9];
                {
                    let source_ml_source2_concat_value249 = arguments_2;
                    self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value248), source_ml_source2_concat_value249)
                }
            }
        }
        self::JavaScriptCallInstruction::OrdinaryGet(destination_10, object, key, receiver_2) => {
            vec![destination_10, object, key, receiver_2]
        }
        self::JavaScriptCallInstruction::OrdinarySet(destination_11, object_2, key_2, value, receiver_3) => {
            vec![destination_11, object_2, key_2, value, receiver_3]
        }
        self::JavaScriptCallInstruction::PropertyKey(destination_12, input, source_ml_source2_f244) => {
            vec![destination_12, input]
        }
        self::JavaScriptCallInstruction::PropertyDefinition(destination_13, target, key_3, attributes, source_ml_source2_f245) => {
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
            self::JavaScriptSemanticValue::Boolean(source_ml_source2_f250) => {
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
                    self::JavaScriptSemanticValue::Boolean(source_ml_source2_f256) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Number(value) => {
                        (left_value == value)
                    }
                    self::JavaScriptSemanticValue::Integer(source_ml_source2_f257) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Text(source_ml_source2_f258) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Symbol(source_ml_source2_f259) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Object(source_ml_source2_f260, source_ml_source2_f261) => {
                        false
                    }
                }
            }
            self::JavaScriptSemanticValue::Integer(source_ml_source2_f251) => {
                self::test_java_script_semantic_same_value(left.clone(), right.clone())
            }
            self::JavaScriptSemanticValue::Text(source_ml_source2_f252) => {
                self::test_java_script_semantic_same_value(left.clone(), right.clone())
            }
            self::JavaScriptSemanticValue::Symbol(source_ml_source2_f253) => {
                self::test_java_script_semantic_same_value(left.clone(), right.clone())
            }
            self::JavaScriptSemanticValue::Object(source_ml_source2_f254, source_ml_source2_f255) => {
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
        let source_ml_source2_field262 = true;
        {
            let source_ml_source2_field263 = configurable;
            {
                let source_ml_source2_field264 = true;
                {
                    let source_ml_source2_field265 = enumerable;
                    {
                        let source_ml_source2_field266 = ((*kind) != (self::ml_string::Text::from_units(vec![97, 99, 99, 101, 115, 115, 111, 114])));
                        {
                            let source_ml_source2_field267 = self::JavaScriptSemanticValue::Undefined;
                            {
                                let source_ml_source2_field268 = ((*kind) != (self::ml_string::Text::from_units(vec![97, 99, 99, 101, 115, 115, 111, 114])));
                                {
                                    let source_ml_source2_field269 = false;
                                    {
                                        let source_ml_source2_field270 = ((*kind) == (self::ml_string::Text::from_units(vec![97, 99, 99, 101, 115, 115, 111, 114])));
                                        {
                                            let source_ml_source2_field271 = self::JavaScriptSemanticValue::Undefined;
                                            {
                                                let source_ml_source2_field272 = ((*kind) == (self::ml_string::Text::from_units(vec![97, 99, 99, 101, 115, 115, 111, 114])));
                                                {
                                                    let source_ml_source2_field273 = self::JavaScriptSemanticValue::Undefined;
                                                    (source_ml_source2_field263, source_ml_source2_field262, source_ml_source2_field265, source_ml_source2_field264, source_ml_source2_field271.clone(), source_ml_source2_field270, source_ml_source2_field273.clone(), source_ml_source2_field272, source_ml_source2_field267.clone(), source_ml_source2_field266, source_ml_source2_field269, source_ml_source2_field268)
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
        let source_ml_source2_field274 = true;
        {
            let source_ml_source2_field275 = if (proposed.clone()).1 {
                (proposed.clone()).0
            } else {
                (base.clone()).0
            };
            {
                let source_ml_source2_field276 = true;
                {
                    let source_ml_source2_field277 = if (proposed.clone()).3 {
                        (proposed.clone()).2
                    } else {
                        (base.clone()).2
                    };
                    {
                        let source_ml_source2_field278 = (base.clone()).9;
                        {
                            let source_ml_source2_field279 = if ((base.clone()).9 && (proposed.clone()).9) {
                                (proposed.clone()).8
                            } else {
                                (base.clone()).8
                            };
                            {
                                let source_ml_source2_field280 = (base.clone()).11;
                                {
                                    let source_ml_source2_field281 = if ((base.clone()).11 && (proposed.clone()).11) {
                                        (proposed.clone()).10
                                    } else {
                                        (base.clone()).10
                                    };
                                    {
                                        let source_ml_source2_field282 = (base.clone()).5;
                                        {
                                            let source_ml_source2_field283 = if ((base.clone()).5 && (proposed.clone()).5) {
                                                (proposed.clone()).4
                                            } else {
                                                (base.clone()).4
                                            };
                                            {
                                                let source_ml_source2_field284 = (base.clone()).7;
                                                {
                                                    let source_ml_source2_field285 = if ((base.clone()).7 && (proposed.clone()).7) {
                                                        (proposed.clone()).6
                                                    } else {
                                                        (base.clone()).6
                                                    };
                                                    (source_ml_source2_field275, source_ml_source2_field274, source_ml_source2_field277, source_ml_source2_field276, source_ml_source2_field283.clone(), source_ml_source2_field282, source_ml_source2_field285.clone(), source_ml_source2_field284, source_ml_source2_field279.clone(), source_ml_source2_field278, source_ml_source2_field281, source_ml_source2_field280)
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
                        let source_ml_source2_concat_value286 = writes;
                        {
                            let source_ml_source2_concat_value287 = vec![self::ml_string::Text::from_units(vec![99, 111, 110, 102, 105, 103, 117, 114, 97, 98, 108, 101])];
                            self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), source_ml_source2_concat_value286), source_ml_source2_concat_value287)
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
                            let source_ml_source2_concat_value288 = writes_3;
                            {
                                let source_ml_source2_concat_value289 = vec![self::ml_string::Text::from_units(vec![101, 110, 117, 109, 101, 114, 97, 98, 108, 101])];
                                self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), source_ml_source2_concat_value288), source_ml_source2_concat_value289)
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
                                let source_ml_source2_concat_value290 = writes_5;
                                {
                                    let source_ml_source2_concat_value291 = vec![self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101])];
                                    self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), source_ml_source2_concat_value290), source_ml_source2_concat_value291)
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
                                    let source_ml_source2_concat_value292 = writes_7;
                                    {
                                        let source_ml_source2_concat_value293 = vec![self::ml_string::Text::from_units(vec![119, 114, 105, 116, 97, 98, 108, 101])];
                                        self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), source_ml_source2_concat_value292), source_ml_source2_concat_value293)
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
                                        let source_ml_source2_concat_value294 = writes_9;
                                        {
                                            let source_ml_source2_concat_value295 = vec![self::ml_string::Text::from_units(vec![103, 101, 116])];
                                            self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), source_ml_source2_concat_value294), source_ml_source2_concat_value295)
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
                                            let source_ml_source2_concat_value296 = writes_11;
                                            {
                                                let source_ml_source2_concat_value297 = vec![self::ml_string::Text::from_units(vec![115, 101, 116])];
                                                self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), source_ml_source2_concat_value296), source_ml_source2_concat_value297)
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
        let source_ml_source2_field298 = disposition.clone();
        {
            let source_ml_source2_field299 = error.clone();
            {
                let source_ml_source2_field300 = descriptor.clone();
                {
                    let source_ml_source2_field301 = writes;
                    (source_ml_source2_field300.clone(), source_ml_source2_field298.clone(), source_ml_source2_field299.clone(), source_ml_source2_field301)
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_java_script_property_definition(exists: bool, extensible: bool, current: (bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool), proposed: (bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool)) -> ((bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool), self::ml_string::Text, self::ml_string::Text, Vec<self::ml_string::Text>) {
    {
        let error = self::read_java_script_property_descriptor_error(proposed.clone());
        if (error != (self::ml_string::Text::from_units(vec![]))) {
            self::create_java_script_property_transition(&(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116])), &(error), current.clone(), Vec::<self::ml_string::Text>::new())
        } else {
            if (exists && !(self::test_complete_java_script_property_descriptor(current.clone()))) {
                self::create_java_script_property_transition(&(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116])), &(self::ml_string::Text::from_units(vec![73, 110, 118, 97, 108, 105, 100, 67, 117, 114, 114, 101, 110, 116, 68, 101, 115, 99, 114, 105, 112, 116, 111, 114])), current.clone(), Vec::<self::ml_string::Text>::new())
            } else {
                {
                    let kind = self::read_java_script_property_descriptor_kind(proposed.clone());
                    if !exists {
                        if !extensible {
                            self::create_java_script_property_transition(&(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116])), &(self::ml_string::Text::from_units(vec![])), current.clone(), Vec::<self::ml_string::Text>::new())
                        } else {
                            {
                                let base = self::create_java_script_property_descriptor(&(kind), false, false);
                                {
                                    let descriptor = self::apply_java_script_property_descriptor_fields(base.clone(), proposed.clone());
                                    self::create_java_script_property_transition(&(self::ml_string::Text::from_units(vec![99, 114, 101, 97, 116, 101])), &(self::ml_string::Text::from_units(vec![])), descriptor.clone(), self::read_java_script_property_descriptor_writes(descriptor.clone()))
                                }
                            }
                        }
                    } else {
                        if (((((!((proposed.clone()).1) && !((proposed.clone()).3)) && !((proposed.clone()).9)) && !((proposed.clone()).11)) && !((proposed.clone()).5)) && !((proposed.clone()).7)) {
                            self::create_java_script_property_transition(&(self::ml_string::Text::from_units(vec![107, 101, 101, 112])), &(self::ml_string::Text::from_units(vec![])), current.clone(), Vec::<self::ml_string::Text>::new())
                        } else {
                            {
                                let original = self::read_java_script_property_descriptor_kind(current.clone());
                                {
                                    let source_ml_source2_s302 = if !((current.clone()).0) {
                                        if ((proposed.clone()).1 && (proposed.clone()).0) {
                                            self::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin94::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin94Return(self::create_java_script_property_transition(&(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116])), &(self::ml_string::Text::from_units(vec![])), current.clone(), Vec::<self::ml_string::Text>::new()))
                                        } else {
                                            if ((proposed.clone()).3 && (((proposed.clone()).2) != ((current.clone()).2))) {
                                                self::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin94::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin94Return(self::create_java_script_property_transition(&(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116])), &(self::ml_string::Text::from_units(vec![])), current.clone(), Vec::<self::ml_string::Text>::new()))
                                            } else {
                                                if ((kind != (self::ml_string::Text::from_units(vec![103, 101, 110, 101, 114, 105, 99]))) && (kind != original)) {
                                                    self::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin94::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin94Return(self::create_java_script_property_transition(&(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116])), &(self::ml_string::Text::from_units(vec![])), current.clone(), Vec::<self::ml_string::Text>::new()))
                                                } else {
                                                    if (original == (self::ml_string::Text::from_units(vec![97, 99, 99, 101, 115, 115, 111, 114]))) {
                                                        if ((proposed.clone()).5 && !(self::test_java_script_semantic_same_value((proposed.clone()).4, (current.clone()).4))) {
                                                            self::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin94::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin94Return(self::create_java_script_property_transition(&(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116])), &(self::ml_string::Text::from_units(vec![])), current.clone(), Vec::<self::ml_string::Text>::new()))
                                                        } else {
                                                            if ((proposed.clone()).7 && !(self::test_java_script_semantic_same_value((proposed.clone()).6, (current.clone()).6))) {
                                                                self::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin94::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin94Return(self::create_java_script_property_transition(&(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116])), &(self::ml_string::Text::from_units(vec![])), current.clone(), Vec::<self::ml_string::Text>::new()))
                                                            } else {
                                                                self::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin94::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin94Next
                                                            }
                                                        }
                                                    } else {
                                                        if !((current.clone()).10) {
                                                            if ((proposed.clone()).11 && (proposed.clone()).10) {
                                                                self::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin94::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin94Return(self::create_java_script_property_transition(&(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116])), &(self::ml_string::Text::from_units(vec![])), current.clone(), Vec::<self::ml_string::Text>::new()))
                                                            } else {
                                                                if (proposed.clone()).9 {
                                                                    self::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin94::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin94Return(self::create_java_script_property_transition(&(if self::test_java_script_semantic_same_value((proposed.clone()).8, (current.clone()).8) {
                                                                        self::ml_string::Text::from_units(vec![107, 101, 101, 112])
                                                                    } else {
                                                                        self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116])
                                                                    }), &(self::ml_string::Text::from_units(vec![])), current.clone(), Vec::<self::ml_string::Text>::new()))
                                                                } else {
                                                                    self::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin94::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin94Next
                                                                }
                                                            }
                                                        } else {
                                                            self::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin94::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin94Next
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    } else {
                                        self::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin94::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin94Next
                                    };
                                    match source_ml_source2_s302.clone() {
                                        self::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin94::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin94Next => {
                                            {
                                                let base_2 = if ((kind != (self::ml_string::Text::from_units(vec![103, 101, 110, 101, 114, 105, 99]))) && (kind != original)) {
                                                    self::create_java_script_property_descriptor(&(kind), (current.clone()).0, (current.clone()).2)
                                                } else {
                                                    current.clone()
                                                };
                                                {
                                                    let descriptor_2 = self::apply_java_script_property_descriptor_fields(base_2.clone(), proposed.clone());
                                                    {
                                                        let conversion = ((kind != (self::ml_string::Text::from_units(vec![103, 101, 110, 101, 114, 105, 99]))) && (kind != original));
                                                        self::create_java_script_property_transition(&(if conversion {
                                                            self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 116])
                                                        } else {
                                                            self::ml_string::Text::from_units(vec![117, 112, 100, 97, 116, 101])
                                                        }), &(self::ml_string::Text::from_units(vec![])), descriptor_2.clone(), self::read_java_script_property_descriptor_writes(if conversion {
                                                            descriptor_2.clone()
                                                        } else {
                                                            proposed.clone()
                                                        }))
                                                    }
                                                }
                                            }
                                        }
                                        self::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin94::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin94Return(source_ml_source1_temporary_result94) => {
                                            source_ml_source1_temporary_result94.clone()
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
pub fn find_java_script_ordinary_object(objects: Vec<self::JavaScriptOrdinaryObject>, identity: self::ml::Big) -> f64 {
    {
        let low = 0f64;
        {
            let high = (objects.len() as f64);
            {
                let source_ml_source2_s306 = self::source_ml_source1_temporary_find_java_script_ordinary_object_loop96(objects, identity.clone(), low, high);
                match source_ml_source2_s306.clone() {
                    self::MlSource1TemporaryFindJavaScriptOrdinaryObjectLoop96Result::MlSource1TemporaryFindJavaScriptOrdinaryObjectLoop96Done => {
                        (-1f64)
                    }
                    self::MlSource1TemporaryFindJavaScriptOrdinaryObjectLoop96Result::MlSource1TemporaryFindJavaScriptOrdinaryObjectLoop96Return(source_ml_source1_temporary_result96) => {
                        source_ml_source1_temporary_result96
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_ordinary_symbol(symbols: Vec<self::ml::Big>, identity: self::ml::Big) -> bool {
    {
        let low = 0f64;
        {
            let high = (symbols.len() as f64);
            {
                let source_ml_source2_s307 = self::source_ml_source1_temporary_test_java_script_ordinary_symbol_loop97(symbols, identity.clone(), low, high);
                match source_ml_source2_s307.clone() {
                    self::MlSource1TemporaryTestJavaScriptOrdinarySymbolLoop97Result::MlSource1TemporaryTestJavaScriptOrdinarySymbolLoop97Done => {
                        false
                    }
                    self::MlSource1TemporaryTestJavaScriptOrdinarySymbolLoop97Result::MlSource1TemporaryTestJavaScriptOrdinarySymbolLoop97Return(source_ml_source1_temporary_result97) => {
                        source_ml_source1_temporary_result97
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_ordinary_value_error(heap: self::JavaScriptOrdinaryHeap, value: self::JavaScriptSemanticValue) -> self::ml_string::Text {
    {
        let error = self::read_java_script_semantic_value_error(value.clone());
        if (error != (self::ml_string::Text::from_units(vec![]))) {
            error.clone()
        } else {
            match value.clone() {
                self::JavaScriptSemanticValue::Undefined => {
                    self::ml_string::Text::from_units(vec![])
                }
                self::JavaScriptSemanticValue::Null => {
                    self::ml_string::Text::from_units(vec![])
                }
                self::JavaScriptSemanticValue::Boolean(source_ml_source2_f308) => {
                    {
                        let source_ml_source2_f324 = source_ml_source2_f308;
                        self::ml_string::Text::from_units(vec![])
                    }
                }
                self::JavaScriptSemanticValue::Number(source_ml_source2_f309) => {
                    {
                        let source_ml_source2_f330 = source_ml_source2_f309;
                        self::ml_string::Text::from_units(vec![])
                    }
                }
                self::JavaScriptSemanticValue::Integer(source_ml_source2_f310) => {
                    {
                        let source_ml_source2_f336 = source_ml_source2_f310.clone();
                        self::ml_string::Text::from_units(vec![])
                    }
                }
                self::JavaScriptSemanticValue::Text(source_ml_source2_f311) => {
                    {
                        let source_ml_source2_f342 = source_ml_source2_f311;
                        self::ml_string::Text::from_units(vec![])
                    }
                }
                self::JavaScriptSemanticValue::Symbol(identity) => {
                    if self::test_java_script_ordinary_symbol(match heap.clone() {
                        self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, source_ml_source2_data_property_value344) => {
                            source_ml_source2_data_property_value344
                        }
                    }, identity.clone()) {
                        self::ml_string::Text::from_units(vec![])
                    } else {
                        self::ml_string::Text::from_units(vec![97, 32, 115, 101, 109, 97, 110, 116, 105, 99, 32, 83, 121, 109, 98, 111, 108, 32, 109, 117, 115, 116, 32, 98, 101, 108, 111, 110, 103, 32, 116, 111, 32, 116, 104, 101, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 103, 108, 111, 98, 97, 108, 32, 105, 100, 101, 110, 116, 105, 116, 121, 32, 100, 105, 114, 101, 99, 116, 111, 114, 121])
                    }
                }
                self::JavaScriptSemanticValue::Object(source_ml_source2_f312, source_ml_source2_f313) => {
                    {
                        let identity_2 = source_ml_source2_f312.clone();
                        {
                            let callable = source_ml_source2_f313;
                            {
                                let at = self::find_java_script_ordinary_object(match heap.clone() {
                                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source2_data_property_value350, _) => {
                                        source_ml_source2_data_property_value350
                                    }
                                }, identity_2.clone());
                                if (at < (0f64)) {
                                    self::ml_string::Text::from_units(vec![97, 32, 115, 101, 109, 97, 110, 116, 105, 99, 32, 79, 98, 106, 101, 99, 116, 32, 109, 117, 115, 116, 32, 98, 101, 108, 111, 110, 103, 32, 116, 111, 32, 116, 104, 101, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 103, 108, 111, 98, 97, 108, 32, 105, 100, 101, 110, 116, 105, 116, 121, 32, 100, 105, 114, 101, 99, 116, 111, 114, 121])
                                } else {
                                    {
                                        let object = self::ml_array::optional_at((match &heap { self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source2_data_property_value351, _) => source_ml_source2_data_property_value351.as_slice() }), self::ml_array::number_index(at));
                                        if (!(match (object.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) && (({
                                            let source_ml_source2_nullish_field_receiver355 = (object.clone(),);
                                            if match (source_ml_source2_nullish_field_receiver355.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                {
                                                    let source_ml_source2_nullish_field_present356 = ((source_ml_source2_nullish_field_receiver355.clone()).0).clone().expect("a proven present value");
                                                    match source_ml_source2_nullish_field_present356.clone() {
                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, source_ml_source2_data_property_value357, _) => {
                                                            source_ml_source2_data_property_value357
                                                        }
                                                    }
                                                }
                                            } else {
                                                (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 99, 97, 108, 108, 97, 98, 108, 101, 39, 41])).0, message_present: true }).raise()
                                            }
                                        }) == callable)) {
                                            self::ml_string::Text::from_units(vec![])
                                        } else {
                                            self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 32, 99, 97, 108, 108, 97, 98, 105, 108, 105, 116, 121, 32, 109, 117, 115, 116, 32, 97, 103, 114, 101, 101, 32, 119, 105, 116, 104, 32, 105, 116, 115, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 103, 108, 111, 98, 97, 108, 32, 105, 100, 101, 110, 116, 105, 116, 121])
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
pub fn read_java_script_ordinary_key_error(heap: self::JavaScriptOrdinaryHeap, key: self::JavaScriptSemanticValue) -> self::ml_string::Text {
    if (match key.clone() {
        self::JavaScriptSemanticValue::Undefined => {
            true
        }
        self::JavaScriptSemanticValue::Null => {
            true
        }
        self::JavaScriptSemanticValue::Boolean(source_ml_source2_f358) => {
            true
        }
        self::JavaScriptSemanticValue::Number(source_ml_source2_f359) => {
            true
        }
        self::JavaScriptSemanticValue::Integer(source_ml_source2_f360) => {
            true
        }
        self::JavaScriptSemanticValue::Text(source_ml_source2_f361) => {
            false
        }
        self::JavaScriptSemanticValue::Symbol(source_ml_source2_f362) => {
            true
        }
        self::JavaScriptSemanticValue::Object(source_ml_source2_f363, source_ml_source2_f364) => {
            true
        }
    } && match key.clone() {
        self::JavaScriptSemanticValue::Undefined => {
            true
        }
        self::JavaScriptSemanticValue::Null => {
            true
        }
        self::JavaScriptSemanticValue::Boolean(source_ml_source2_f365) => {
            true
        }
        self::JavaScriptSemanticValue::Number(source_ml_source2_f366) => {
            true
        }
        self::JavaScriptSemanticValue::Integer(source_ml_source2_f367) => {
            true
        }
        self::JavaScriptSemanticValue::Text(source_ml_source2_f368) => {
            true
        }
        self::JavaScriptSemanticValue::Symbol(source_ml_source2_f369) => {
            false
        }
        self::JavaScriptSemanticValue::Object(source_ml_source2_f370, source_ml_source2_f371) => {
            true
        }
    }) {
        self::ml_string::Text::from_units(vec![97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 107, 101, 121, 32, 109, 117, 115, 116, 32, 98, 101, 32, 116, 101, 120, 116, 32, 111, 114, 32, 83, 121, 109, 98, 111, 108, 32, 119, 105, 116, 104, 111, 117, 116, 32, 105, 109, 112, 108, 105, 99, 105, 116, 32, 99, 111, 101, 114, 99, 105, 111, 110])
    } else {
        self::read_java_script_ordinary_value_error(heap.clone(), key.clone())
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn find_java_script_ordinary_property(properties: Vec<self::JavaScriptOrdinaryProperty>, key: self::JavaScriptSemanticValue) -> f64 {
    {
        let at = 0f64;
        {
            let source_ml_source2_s372 = self::source_ml_source1_temporary_find_java_script_ordinary_property_loop98(properties, key.clone(), at);
            match source_ml_source2_s372.clone() {
                self::MlSource1TemporaryFindJavaScriptOrdinaryPropertyLoop98Result::MlSource1TemporaryFindJavaScriptOrdinaryPropertyLoop98Done => {
                    (-1f64)
                }
                self::MlSource1TemporaryFindJavaScriptOrdinaryPropertyLoop98Result::MlSource1TemporaryFindJavaScriptOrdinaryPropertyLoop98Return(source_ml_source1_temporary_result98) => {
                    source_ml_source1_temporary_result98
                }
            }
        }
    }
}
