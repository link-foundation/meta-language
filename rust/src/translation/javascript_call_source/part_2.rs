// Generated from complete js/src/translation/javascript-call-source.js. Do not edit by hand.
use super::JavaScriptCallInstruction;
use super::JavaScriptCallProgram;
use super::JavaScriptCallSourceBuilder;
use super::JavaScriptCallSourceResult;
use super::JavaScriptSemanticValue;
use super::MlTemporaryFindJavaScriptCallCodeFunctionLoop3Result;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::ml_source_exception;
use super::read_java_script_semantic_value_error;
use super::source_ml_temporary_find_java_script_call_code_function_loop3;
use super::test_java_script_semantic_same_value;

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
pub fn refuse_java_script_call_source(builder: self::JavaScriptCallSourceBuilder, error: &str) -> self::JavaScriptCallSourceResult {
    self::JavaScriptCallSourceResult::SourceResult(Box::new({
        let source_ml_source1_nominal_spread132 = builder.clone();
        {
            let source_ml_source1_nominal_field133 = match source_ml_source1_nominal_spread132.clone() {
                self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source1_data_property_value134, _, _, _, _) => {
                    source_ml_source1_data_property_value134
                }
            };
            {
                let source_ml_source1_nominal_field135 = match source_ml_source1_nominal_spread132.clone() {
                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source1_data_property_value136, _, _, _) => {
                        source_ml_source1_data_property_value136
                    }
                };
                {
                    let source_ml_source1_nominal_field137 = match source_ml_source1_nominal_spread132.clone() {
                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source1_data_property_value138, _, _) => {
                            source_ml_source1_data_property_value138
                        }
                    };
                    {
                        let source_ml_source1_nominal_field139 = match source_ml_source1_nominal_spread132.clone() {
                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source1_data_property_value140, _) => {
                                source_ml_source1_data_property_value140
                            }
                        };
                        {
                            let source_ml_source1_nominal_field141 = match source_ml_source1_nominal_spread132.clone() {
                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source1_data_property_value142) => {
                                    source_ml_source1_data_property_value142.clone()
                                }
                            };
                            {
                                let source_ml_source1_nominal_override143 = error.to_owned();
                                self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source1_nominal_field133, source_ml_source1_nominal_field135, source_ml_source1_nominal_field137, source_ml_source1_nominal_field139, source_ml_source1_nominal_override143.clone())
                            }
                        }
                    }
                }
            }
        }
    }), 0f64)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn append_java_script_call_source_instruction(builder: self::JavaScriptCallSourceBuilder, instruction: self::JavaScriptCallInstruction) -> self::JavaScriptCallSourceBuilder {
    if ((match builder.clone() {
        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source1_data_property_value145) => {
            source_ml_source1_data_property_value145.clone()
        }
    }) != "") {
        builder.clone()
    } else {
        if ((((match &builder { self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source1_data_property_value147, _, _, _, _) => source_ml_source1_data_property_value147.as_slice() }).len() as f64)) >= (65536f64)) {
            {
                let source_ml_source1_nominal_spread148 = builder.clone();
                {
                    let source_ml_source1_nominal_field149 = match source_ml_source1_nominal_spread148.clone() {
                        self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source1_data_property_value150, _, _, _, _) => {
                            source_ml_source1_data_property_value150
                        }
                    };
                    {
                        let source_ml_source1_nominal_field151 = match source_ml_source1_nominal_spread148.clone() {
                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source1_data_property_value152, _, _, _) => {
                                source_ml_source1_data_property_value152
                            }
                        };
                        {
                            let source_ml_source1_nominal_field153 = match source_ml_source1_nominal_spread148.clone() {
                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source1_data_property_value154, _, _) => {
                                    source_ml_source1_data_property_value154
                                }
                            };
                            {
                                let source_ml_source1_nominal_field155 = match source_ml_source1_nominal_spread148.clone() {
                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source1_data_property_value156, _) => {
                                        source_ml_source1_data_property_value156
                                    }
                                };
                                {
                                    let source_ml_source1_nominal_field157 = match source_ml_source1_nominal_spread148.clone() {
                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source1_data_property_value158) => {
                                            source_ml_source1_data_property_value158.clone()
                                        }
                                    };
                                    {
                                        let source_ml_source1_nominal_override159 = String::from("source lowering exceeds its instruction budget");
                                        self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source1_nominal_field149, source_ml_source1_nominal_field151, source_ml_source1_nominal_field153, source_ml_source1_nominal_field155, source_ml_source1_nominal_override159.clone())
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            {
                let source_ml_source1_nominal_spread163 = builder.clone();
                {
                    let source_ml_source1_nominal_field164 = match source_ml_source1_nominal_spread163.clone() {
                        self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source1_data_property_value165, _, _, _, _) => {
                            source_ml_source1_data_property_value165
                        }
                    };
                    {
                        let source_ml_source1_nominal_field166 = match source_ml_source1_nominal_spread163.clone() {
                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source1_data_property_value167, _, _, _) => {
                                source_ml_source1_data_property_value167
                            }
                        };
                        {
                            let source_ml_source1_nominal_field168 = match source_ml_source1_nominal_spread163.clone() {
                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source1_data_property_value169, _, _) => {
                                    source_ml_source1_data_property_value169
                                }
                            };
                            {
                                let source_ml_source1_nominal_field170 = match source_ml_source1_nominal_spread163.clone() {
                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source1_data_property_value171, _) => {
                                        source_ml_source1_data_property_value171
                                    }
                                };
                                {
                                    let source_ml_source1_nominal_field172 = match source_ml_source1_nominal_spread163.clone() {
                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source1_data_property_value173) => {
                                            source_ml_source1_data_property_value173.clone()
                                        }
                                    };
                                    {
                                        let source_ml_source1_nominal_override174 = {
                                            let source_ml_source1_concat_value161 = match builder.clone() {
                                                self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source1_data_property_value160, _, _, _, _) => {
                                                    source_ml_source1_data_property_value160
                                                }
                                            };
                                            {
                                                let source_ml_source1_concat_value162 = vec![instruction.clone()];
                                                self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptCallInstruction>::new(), source_ml_source1_concat_value161), source_ml_source1_concat_value162)
                                            }
                                        };
                                        self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source1_nominal_override174, source_ml_source1_nominal_field166, source_ml_source1_nominal_field168, source_ml_source1_nominal_field170, source_ml_source1_nominal_field172.clone())
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
pub fn allocate_java_script_call_source_register(builder: self::JavaScriptCallSourceBuilder) -> self::JavaScriptCallSourceResult {
    if ((match builder.clone() {
        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source1_data_property_value176) => {
            source_ml_source1_data_property_value176.clone()
        }
    }) != "") {
        self::JavaScriptCallSourceResult::SourceResult(Box::new(builder.clone()), 0f64)
    } else {
        if ((match builder.clone() {
            self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source1_data_property_value178, _, _, _) => {
                source_ml_source1_data_property_value178
            }
        }) >= (4096f64)) {
            self::refuse_java_script_call_source(builder.clone(), "source lowering exceeds its register budget")
        } else {
            self::JavaScriptCallSourceResult::SourceResult(Box::new({
                let source_ml_source1_nominal_spread180 = builder.clone();
                {
                    let source_ml_source1_nominal_field181 = match source_ml_source1_nominal_spread180.clone() {
                        self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source1_data_property_value182, _, _, _, _) => {
                            source_ml_source1_data_property_value182
                        }
                    };
                    {
                        let source_ml_source1_nominal_field183 = match source_ml_source1_nominal_spread180.clone() {
                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source1_data_property_value184, _, _, _) => {
                                source_ml_source1_data_property_value184
                            }
                        };
                        {
                            let source_ml_source1_nominal_field185 = match source_ml_source1_nominal_spread180.clone() {
                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source1_data_property_value186, _, _) => {
                                    source_ml_source1_data_property_value186
                                }
                            };
                            {
                                let source_ml_source1_nominal_field187 = match source_ml_source1_nominal_spread180.clone() {
                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source1_data_property_value188, _) => {
                                        source_ml_source1_data_property_value188
                                    }
                                };
                                {
                                    let source_ml_source1_nominal_field189 = match source_ml_source1_nominal_spread180.clone() {
                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source1_data_property_value190) => {
                                            source_ml_source1_data_property_value190.clone()
                                        }
                                    };
                                    {
                                        let source_ml_source1_nominal_override191 = (match builder.clone() {
                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source1_data_property_value179, _, _, _) => {
                                                source_ml_source1_data_property_value179
                                            }
                                        } + 1f64);
                                        self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source1_nominal_field181, source_ml_source1_nominal_override191, source_ml_source1_nominal_field185, source_ml_source1_nominal_field187, source_ml_source1_nominal_field189.clone())
                                    }
                                }
                            }
                        }
                    }
                }
            }), match builder.clone() {
                self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source1_data_property_value192, _, _, _) => {
                    source_ml_source1_data_property_value192
                }
            })
        }
    }
}
