// Generated from complete js/src/translation/javascript-call-source.js. Do not edit by hand.
use super::JavaScriptCallInstruction;
use super::JavaScriptCallProgram;
use super::JavaScriptCallSource;
use super::JavaScriptCallSourceBinding;
use super::JavaScriptCallSourceBuilder;
use super::JavaScriptCallSourceResult;
use super::JavaScriptSemanticValue;
use super::MlTemporaryFindJavaScriptCallCodeFunctionLoop3Result;
use super::MlTemporaryLowerJavaScriptCallSourceExpressionLoop5Result;
use super::MlTemporaryLowerJavaScriptCallSourceExpressionLoop6Result;
use super::lower_java_script_call_source_branch;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::ml_source_exception;
use super::read_java_script_semantic_value_error;
use super::source_ml_temporary_find_java_script_call_code_function_loop3;
use super::source_ml_temporary_lower_java_script_call_source_expression_loop5;
use super::source_ml_temporary_lower_java_script_call_source_expression_loop6;
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
                self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source1_data_property_value134, _, _, _) => {
                    source_ml_source1_data_property_value134
                }
            };
            {
                let source_ml_source1_nominal_field135 = match source_ml_source1_nominal_spread132.clone() {
                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source1_data_property_value136, _, _) => {
                        source_ml_source1_data_property_value136
                    }
                };
                {
                    let source_ml_source1_nominal_field137 = match source_ml_source1_nominal_spread132.clone() {
                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source1_data_property_value138, _) => {
                            source_ml_source1_data_property_value138
                        }
                    };
                    {
                        let source_ml_source1_nominal_field139 = match source_ml_source1_nominal_spread132.clone() {
                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source1_data_property_value140) => {
                                source_ml_source1_data_property_value140.clone()
                            }
                        };
                        {
                            let source_ml_source1_nominal_override141 = error.to_owned();
                            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source1_nominal_field133, source_ml_source1_nominal_field135, source_ml_source1_nominal_field137, source_ml_source1_nominal_override141.clone())
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
        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source1_data_property_value143) => {
            source_ml_source1_data_property_value143.clone()
        }
    }) != "") {
        builder.clone()
    } else {
        if ((((match &builder { self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source1_data_property_value145, _, _, _) => source_ml_source1_data_property_value145.as_slice() }).len() as f64)) >= (65536f64)) {
            {
                let source_ml_source1_nominal_spread146 = builder.clone();
                {
                    let source_ml_source1_nominal_field147 = match source_ml_source1_nominal_spread146.clone() {
                        self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source1_data_property_value148, _, _, _) => {
                            source_ml_source1_data_property_value148
                        }
                    };
                    {
                        let source_ml_source1_nominal_field149 = match source_ml_source1_nominal_spread146.clone() {
                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source1_data_property_value150, _, _) => {
                                source_ml_source1_data_property_value150
                            }
                        };
                        {
                            let source_ml_source1_nominal_field151 = match source_ml_source1_nominal_spread146.clone() {
                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source1_data_property_value152, _) => {
                                    source_ml_source1_data_property_value152
                                }
                            };
                            {
                                let source_ml_source1_nominal_field153 = match source_ml_source1_nominal_spread146.clone() {
                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source1_data_property_value154) => {
                                        source_ml_source1_data_property_value154.clone()
                                    }
                                };
                                {
                                    let source_ml_source1_nominal_override155 = String::from("source lowering exceeds its instruction budget");
                                    self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source1_nominal_field147, source_ml_source1_nominal_field149, source_ml_source1_nominal_field151, source_ml_source1_nominal_override155.clone())
                                }
                            }
                        }
                    }
                }
            }
        } else {
            {
                let source_ml_source1_nominal_spread159 = builder.clone();
                {
                    let source_ml_source1_nominal_field160 = match source_ml_source1_nominal_spread159.clone() {
                        self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source1_data_property_value161, _, _, _) => {
                            source_ml_source1_data_property_value161
                        }
                    };
                    {
                        let source_ml_source1_nominal_field162 = match source_ml_source1_nominal_spread159.clone() {
                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source1_data_property_value163, _, _) => {
                                source_ml_source1_data_property_value163
                            }
                        };
                        {
                            let source_ml_source1_nominal_field164 = match source_ml_source1_nominal_spread159.clone() {
                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source1_data_property_value165, _) => {
                                    source_ml_source1_data_property_value165
                                }
                            };
                            {
                                let source_ml_source1_nominal_field166 = match source_ml_source1_nominal_spread159.clone() {
                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source1_data_property_value167) => {
                                        source_ml_source1_data_property_value167.clone()
                                    }
                                };
                                {
                                    let source_ml_source1_nominal_override168 = {
                                        let source_ml_source1_concat_value157 = match builder.clone() {
                                            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source1_data_property_value156, _, _, _) => {
                                                source_ml_source1_data_property_value156
                                            }
                                        };
                                        {
                                            let source_ml_source1_concat_value158 = vec![instruction.clone()];
                                            self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptCallInstruction>::new(), source_ml_source1_concat_value157), source_ml_source1_concat_value158)
                                        }
                                    };
                                    self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source1_nominal_override168, source_ml_source1_nominal_field162, source_ml_source1_nominal_field164, source_ml_source1_nominal_field166.clone())
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
        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source1_data_property_value170) => {
            source_ml_source1_data_property_value170.clone()
        }
    }) != "") {
        self::JavaScriptCallSourceResult::SourceResult(Box::new(builder.clone()), 0f64)
    } else {
        if ((match builder.clone() {
            self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source1_data_property_value172, _, _) => {
                source_ml_source1_data_property_value172
            }
        }) >= (4096f64)) {
            self::refuse_java_script_call_source(builder.clone(), "source lowering exceeds its register budget")
        } else {
            self::JavaScriptCallSourceResult::SourceResult(Box::new({
                let source_ml_source1_nominal_spread174 = builder.clone();
                {
                    let source_ml_source1_nominal_field175 = match source_ml_source1_nominal_spread174.clone() {
                        self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source1_data_property_value176, _, _, _) => {
                            source_ml_source1_data_property_value176
                        }
                    };
                    {
                        let source_ml_source1_nominal_field177 = match source_ml_source1_nominal_spread174.clone() {
                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source1_data_property_value178, _, _) => {
                                source_ml_source1_data_property_value178
                            }
                        };
                        {
                            let source_ml_source1_nominal_field179 = match source_ml_source1_nominal_spread174.clone() {
                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source1_data_property_value180, _) => {
                                    source_ml_source1_data_property_value180
                                }
                            };
                            {
                                let source_ml_source1_nominal_field181 = match source_ml_source1_nominal_spread174.clone() {
                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source1_data_property_value182) => {
                                        source_ml_source1_data_property_value182.clone()
                                    }
                                };
                                {
                                    let source_ml_source1_nominal_override183 = (match builder.clone() {
                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source1_data_property_value173, _, _) => {
                                            source_ml_source1_data_property_value173
                                        }
                                    } + 1f64);
                                    self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source1_nominal_field175, source_ml_source1_nominal_override183, source_ml_source1_nominal_field179, source_ml_source1_nominal_field181.clone())
                                }
                            }
                        }
                    }
                }
            }), match builder.clone() {
                self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source1_data_property_value184, _, _) => {
                    source_ml_source1_data_property_value184
                }
            })
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn lower_java_script_call_source_expression(source: self::JavaScriptCallSource, bindings: Vec<self::JavaScriptCallSourceBinding>, builder: self::JavaScriptCallSourceBuilder, depth: f64) -> self::JavaScriptCallSourceResult {
    if ((match builder.clone() {
        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source1_data_property_value186) => {
            source_ml_source1_data_property_value186.clone()
        }
    }) != "") {
        self::JavaScriptCallSourceResult::SourceResult(Box::new(builder.clone()), 0f64)
    } else {
        if ((depth > (128f64)) || ((match builder.clone() {
            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source1_data_property_value188, _) => {
                source_ml_source1_data_property_value188
            }
        }) >= (65536f64))) {
            self::refuse_java_script_call_source(builder.clone(), "source lowering exceeds its structural budget")
        } else {
            {
                let state = {
                    let source_ml_source1_nominal_spread190 = builder.clone();
                    {
                        let source_ml_source1_nominal_field191 = match source_ml_source1_nominal_spread190.clone() {
                            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source1_data_property_value192, _, _, _) => {
                                source_ml_source1_data_property_value192
                            }
                        };
                        {
                            let source_ml_source1_nominal_field193 = match source_ml_source1_nominal_spread190.clone() {
                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source1_data_property_value194, _, _) => {
                                    source_ml_source1_data_property_value194
                                }
                            };
                            {
                                let source_ml_source1_nominal_field195 = match source_ml_source1_nominal_spread190.clone() {
                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source1_data_property_value196, _) => {
                                        source_ml_source1_data_property_value196
                                    }
                                };
                                {
                                    let source_ml_source1_nominal_field197 = match source_ml_source1_nominal_spread190.clone() {
                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source1_data_property_value198) => {
                                            source_ml_source1_data_property_value198.clone()
                                        }
                                    };
                                    {
                                        let source_ml_source1_nominal_override199 = (match builder.clone() {
                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source1_data_property_value189, _) => {
                                                source_ml_source1_data_property_value189
                                            }
                                        } + 1f64);
                                        self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source1_nominal_field191, source_ml_source1_nominal_field193, source_ml_source1_nominal_override199, source_ml_source1_nominal_field197.clone())
                                    }
                                }
                            }
                        }
                    }
                };
                match source.clone() {
                    self::JavaScriptCallSource::SourceConstant(value) => {
                        let value = *value;
                        {
                            let error = self::read_java_script_call_constant_error(value.clone());
                            if (error != "") {
                                self::refuse_java_script_call_source(state.clone(), &(error))
                            } else {
                                {
                                    let result = self::allocate_java_script_call_source_register(state.clone());
                                    {
                                        let source_ml_source1_nominal_spread206 = result.clone();
                                        {
                                            let source_ml_source1_nominal_field207 = match source_ml_source1_nominal_spread206.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source1_data_property_value208, _) => {
                                                    let source_ml_source1_data_property_value208 = *source_ml_source1_data_property_value208;
                                                    source_ml_source1_data_property_value208.clone()
                                                }
                                            };
                                            {
                                                let source_ml_source1_nominal_field209 = match source_ml_source1_nominal_spread206.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source1_data_property_value210) => {
                                                        source_ml_source1_data_property_value210
                                                    }
                                                };
                                                {
                                                    let source_ml_source1_nominal_override211 = self::append_java_script_call_source_instruction(match result.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source1_data_property_value204, _) => {
                                                            let source_ml_source1_data_property_value204 = *source_ml_source1_data_property_value204;
                                                            source_ml_source1_data_property_value204.clone()
                                                        }
                                                    }, self::JavaScriptCallInstruction::Constant(match result.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source1_data_property_value205) => {
                                                            source_ml_source1_data_property_value205
                                                        }
                                                    }, Box::new(value.clone())));
                                                    self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source1_nominal_override211.clone()), source_ml_source1_nominal_field209)
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    self::JavaScriptCallSource::SourceBinding(identity) => {
                        if !(self::test_java_script_call_code_coordinate(identity, 65536f64)) {
                            self::refuse_java_script_call_source(state.clone(), "source binding identities require exact nonnegative coordinates")
                        } else {
                            {
                                let index = ((bindings.len() as f64) - 1f64);
                                {
                                    let source_ml_source1_s212 = self::source_ml_temporary_lower_java_script_call_source_expression_loop5(source.clone(), bindings, state.clone(), index);
                                    match source_ml_source1_s212.clone() {
                                        self::MlTemporaryLowerJavaScriptCallSourceExpressionLoop5Result::MlTemporaryLowerJavaScriptCallSourceExpressionLoop5Done => {
                                            self::refuse_java_script_call_source(state.clone(), "a source binding reference has no actual lexical owner")
                                        }
                                        self::MlTemporaryLowerJavaScriptCallSourceExpressionLoop5Result::MlTemporaryLowerJavaScriptCallSourceExpressionLoop5Return(source_ml_temporary_result5) => {
                                            let source_ml_temporary_result5 = *source_ml_temporary_result5;
                                            source_ml_temporary_result5.clone()
                                        }
                                    }
                                }
                            }
                        }
                    }
                    self::JavaScriptCallSource::SourceLet(identity_2, source_value, body) => {
                        let source_value = *source_value;
                        let body = *body;
                        if (!(self::test_java_script_call_code_coordinate(identity_2, 65536f64)) || { let ml_values1 = bindings.clone(); let ml_callback1 = { let identity_2 = identity_2; std::rc::Rc::new(move |entry: self::JavaScriptCallSourceBinding| -> bool { ((match entry.clone() {
                            self::JavaScriptCallSourceBinding::SourceBindingRegister(source_ml_source1_data_property_value214, _) => {
                                source_ml_source1_data_property_value214
                            }
                        }) == identity_2) }) }; ml_values1.iter().cloned().enumerate().any(|(ml_index1, ml_value1)| ml_callback1(ml_value1)) }) {
                            self::refuse_java_script_call_source(state.clone(), "a source binding owner must be unique in its lexical scope")
                        } else {
                            {
                                let value_2 = self::lower_java_script_call_source_expression(source_value.clone(), bindings.clone(), state.clone(), (depth + 1f64));
                                self::lower_java_script_call_source_expression(body.clone(), {
                                    let source_ml_source1_concat_value216 = bindings;
                                    {
                                        let source_ml_source1_concat_value217 = vec![self::JavaScriptCallSourceBinding::SourceBindingRegister(identity_2, match value_2.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source1_data_property_value215) => {
                                                source_ml_source1_data_property_value215
                                            }
                                        })];
                                        self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptCallSourceBinding>::new(), source_ml_source1_concat_value216), source_ml_source1_concat_value217)
                                    }
                                }, match value_2.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source1_data_property_value218, _) => {
                                        let source_ml_source1_data_property_value218 = *source_ml_source1_data_property_value218;
                                        source_ml_source1_data_property_value218.clone()
                                    }
                                }, (depth + 1f64))
                            }
                        }
                    }
                    self::JavaScriptCallSource::SourceIf(source_ml_source1_f200, source_ml_source1_f201, source_ml_source1_f202) => {
                        let source_ml_source1_f200 = *source_ml_source1_f200;
                        let source_ml_source1_f201 = *source_ml_source1_f201;
                        let source_ml_source1_f202 = *source_ml_source1_f202;
                        self::lower_java_script_call_source_branch(source.clone(), bindings, state.clone(), depth)
                    }
                    self::JavaScriptCallSource::SourceNumberBinary(source_ml_source1_f203, source_left, source_right) => {
                        let source_left = *source_left;
                        let source_right = *source_right;
                        {
                            let left = self::lower_java_script_call_source_expression(source_left.clone(), bindings.clone(), state.clone(), (depth + 1f64));
                            {
                                let right = self::lower_java_script_call_source_expression(source_right.clone(), bindings, match left.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source1_data_property_value219, _) => {
                                        let source_ml_source1_data_property_value219 = *source_ml_source1_data_property_value219;
                                        source_ml_source1_data_property_value219.clone()
                                    }
                                }, (depth + 1f64));
                                {
                                    let result_2 = self::allocate_java_script_call_source_register(match right.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source1_data_property_value220, _) => {
                                            let source_ml_source1_data_property_value220 = *source_ml_source1_data_property_value220;
                                            source_ml_source1_data_property_value220.clone()
                                        }
                                    });
                                    {
                                        let operator = source_ml_source1_f203.clone();
                                        {
                                            let source_ml_source1_f229 = source_left.clone();
                                            {
                                                let source_ml_source1_f230 = source_right.clone();
                                                if !(self::ml_array::includes(&vec![String::from("add"), String::from("subtract"), String::from("multiply"), String::from("divide"), String::from("remainder"), String::from("power"), String::from("lessThan"), String::from("equal")], &operator.clone(), 0f64, false, |a, b| a == b)) {
                                                    self::refuse_java_script_call_source(match result_2.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source1_data_property_value238, _) => {
                                                            let source_ml_source1_data_property_value238 = *source_ml_source1_data_property_value238;
                                                            source_ml_source1_data_property_value238.clone()
                                                        }
                                                    }, "source Number lowering requires its exact registered operator")
                                                } else {
                                                    {
                                                        let source_ml_source1_nominal_spread243 = result_2.clone();
                                                        {
                                                            let source_ml_source1_nominal_field244 = match source_ml_source1_nominal_spread243.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source1_data_property_value245, _) => {
                                                                    let source_ml_source1_data_property_value245 = *source_ml_source1_data_property_value245;
                                                                    source_ml_source1_data_property_value245.clone()
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source1_nominal_field246 = match source_ml_source1_nominal_spread243.clone() {
                                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source1_data_property_value247) => {
                                                                        source_ml_source1_data_property_value247
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source1_nominal_override248 = self::append_java_script_call_source_instruction(match result_2.clone() {
                                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source1_data_property_value239, _) => {
                                                                            let source_ml_source1_data_property_value239 = *source_ml_source1_data_property_value239;
                                                                            source_ml_source1_data_property_value239.clone()
                                                                        }
                                                                    }, self::JavaScriptCallInstruction::NumberBinary(match result_2.clone() {
                                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source1_data_property_value240) => {
                                                                            source_ml_source1_data_property_value240
                                                                        }
                                                                    }, match left.clone() {
                                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source1_data_property_value241) => {
                                                                            source_ml_source1_data_property_value241
                                                                        }
                                                                    }, match right.clone() {
                                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source1_data_property_value242) => {
                                                                            source_ml_source1_data_property_value242
                                                                        }
                                                                    }, operator.clone()));
                                                                    self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source1_nominal_override248.clone()), source_ml_source1_nominal_field246)
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
                    self::JavaScriptCallSource::SourceSameValue(source_left_2, source_right_2) => {
                        let source_left_2 = *source_left_2;
                        let source_right_2 = *source_right_2;
                        {
                            let left_2 = self::lower_java_script_call_source_expression(source_left_2.clone(), bindings.clone(), state.clone(), (depth + 1f64));
                            {
                                let right_2 = self::lower_java_script_call_source_expression(source_right_2.clone(), bindings, match left_2.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source1_data_property_value249, _) => {
                                        let source_ml_source1_data_property_value249 = *source_ml_source1_data_property_value249;
                                        source_ml_source1_data_property_value249.clone()
                                    }
                                }, (depth + 1f64));
                                {
                                    let result_3 = self::allocate_java_script_call_source_register(match right_2.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source1_data_property_value250, _) => {
                                            let source_ml_source1_data_property_value250 = *source_ml_source1_data_property_value250;
                                            source_ml_source1_data_property_value250.clone()
                                        }
                                    });
                                    {
                                        let source_ml_source1_f261 = source_left_2.clone();
                                        {
                                            let source_ml_source1_f262 = source_right_2.clone();
                                            {
                                                let source_ml_source1_nominal_spread272 = result_3.clone();
                                                {
                                                    let source_ml_source1_nominal_field273 = match source_ml_source1_nominal_spread272.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source1_data_property_value274, _) => {
                                                            let source_ml_source1_data_property_value274 = *source_ml_source1_data_property_value274;
                                                            source_ml_source1_data_property_value274.clone()
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source1_nominal_field275 = match source_ml_source1_nominal_spread272.clone() {
                                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source1_data_property_value276) => {
                                                                source_ml_source1_data_property_value276
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source1_nominal_override277 = self::append_java_script_call_source_instruction(match result_3.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source1_data_property_value268, _) => {
                                                                    let source_ml_source1_data_property_value268 = *source_ml_source1_data_property_value268;
                                                                    source_ml_source1_data_property_value268.clone()
                                                                }
                                                            }, self::JavaScriptCallInstruction::SameValue(match result_3.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source1_data_property_value269) => {
                                                                    source_ml_source1_data_property_value269
                                                                }
                                                            }, match left_2.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source1_data_property_value270) => {
                                                                    source_ml_source1_data_property_value270
                                                                }
                                                            }, match right_2.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source1_data_property_value271) => {
                                                                    source_ml_source1_data_property_value271
                                                                }
                                                            }));
                                                            self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source1_nominal_override277.clone()), source_ml_source1_nominal_field275)
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
                    self::JavaScriptCallSource::SourceStrictEqual(source_left_3, source_right_3) => {
                        let source_left_3 = *source_left_3;
                        let source_right_3 = *source_right_3;
                        {
                            let left_3 = self::lower_java_script_call_source_expression(source_left_3.clone(), bindings.clone(), state.clone(), (depth + 1f64));
                            {
                                let right_3 = self::lower_java_script_call_source_expression(source_right_3.clone(), bindings, match left_3.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source1_data_property_value278, _) => {
                                        let source_ml_source1_data_property_value278 = *source_ml_source1_data_property_value278;
                                        source_ml_source1_data_property_value278.clone()
                                    }
                                }, (depth + 1f64));
                                {
                                    let result_4 = self::allocate_java_script_call_source_register(match right_3.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source1_data_property_value279, _) => {
                                            let source_ml_source1_data_property_value279 = *source_ml_source1_data_property_value279;
                                            source_ml_source1_data_property_value279.clone()
                                        }
                                    });
                                    {
                                        let source_ml_source1_f292 = source_left_3.clone();
                                        {
                                            let source_ml_source1_f293 = source_right_3.clone();
                                            {
                                                let source_ml_source1_nominal_spread301 = result_4.clone();
                                                {
                                                    let source_ml_source1_nominal_field302 = match source_ml_source1_nominal_spread301.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source1_data_property_value303, _) => {
                                                            let source_ml_source1_data_property_value303 = *source_ml_source1_data_property_value303;
                                                            source_ml_source1_data_property_value303.clone()
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source1_nominal_field304 = match source_ml_source1_nominal_spread301.clone() {
                                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source1_data_property_value305) => {
                                                                source_ml_source1_data_property_value305
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source1_nominal_override306 = self::append_java_script_call_source_instruction(match result_4.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source1_data_property_value297, _) => {
                                                                    let source_ml_source1_data_property_value297 = *source_ml_source1_data_property_value297;
                                                                    source_ml_source1_data_property_value297.clone()
                                                                }
                                                            }, self::JavaScriptCallInstruction::StrictEqual(match result_4.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source1_data_property_value298) => {
                                                                    source_ml_source1_data_property_value298
                                                                }
                                                            }, match left_3.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source1_data_property_value299) => {
                                                                    source_ml_source1_data_property_value299
                                                                }
                                                            }, match right_3.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source1_data_property_value300) => {
                                                                    source_ml_source1_data_property_value300
                                                                }
                                                            }));
                                                            self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source1_nominal_override306.clone()), source_ml_source1_nominal_field304)
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
                    self::JavaScriptCallSource::SourceCall(source_callee, source_receiver, arguments) => {
                        let source_callee = *source_callee;
                        let source_receiver = *source_receiver;
                        if (((arguments.len() as f64)) > (4096f64)) {
                            self::refuse_java_script_call_source(state.clone(), "source call operands exceed their explicit budget")
                        } else {
                            {
                                let callee = self::lower_java_script_call_source_expression(source_callee.clone(), bindings.clone(), state.clone(), (depth + 1f64));
                                {
                                    let receiver = self::lower_java_script_call_source_expression(source_receiver.clone(), bindings.clone(), match callee.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source1_data_property_value307, _) => {
                                            let source_ml_source1_data_property_value307 = *source_ml_source1_data_property_value307;
                                            source_ml_source1_data_property_value307.clone()
                                        }
                                    }, (depth + 1f64));
                                    {
                                        let current = match receiver.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source1_data_property_value308, _) => {
                                                let source_ml_source1_data_property_value308 = *source_ml_source1_data_property_value308;
                                                source_ml_source1_data_property_value308.clone()
                                            }
                                        };
                                        {
                                            let arguments_ = Vec::<f64>::new();
                                            {
                                                let source_ml_temporary_values4 = arguments;
                                                {
                                                    let source_ml_temporary_index4 = self::ml::Big::from_i128(0);
                                                    {
                                                        let source_ml_source1_s309 = self::source_ml_temporary_lower_java_script_call_source_expression_loop6(bindings, depth, current.clone(), arguments_, source_ml_temporary_values4, source_ml_temporary_index4.clone());
                                                        match source_ml_source1_s309.clone() {
                                                            self::MlTemporaryLowerJavaScriptCallSourceExpressionLoop6Result::MlTemporaryLowerJavaScriptCallSourceExpressionLoop6Done(current_2, arguments__2) => {
                                                                let current_2 = *current_2;
                                                                {
                                                                    let result_5 = self::allocate_java_script_call_source_register(current_2.clone());
                                                                    {
                                                                        let source_ml_source1_nominal_spread314 = result_5.clone();
                                                                        {
                                                                            let source_ml_source1_nominal_field315 = match source_ml_source1_nominal_spread314.clone() {
                                                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source1_data_property_value316, _) => {
                                                                                    let source_ml_source1_data_property_value316 = *source_ml_source1_data_property_value316;
                                                                                    source_ml_source1_data_property_value316.clone()
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source1_nominal_field317 = match source_ml_source1_nominal_spread314.clone() {
                                                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source1_data_property_value318) => {
                                                                                        source_ml_source1_data_property_value318
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source1_nominal_override319 = self::append_java_script_call_source_instruction(match result_5.clone() {
                                                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source1_data_property_value310, _) => {
                                                                                            let source_ml_source1_data_property_value310 = *source_ml_source1_data_property_value310;
                                                                                            source_ml_source1_data_property_value310.clone()
                                                                                        }
                                                                                    }, self::JavaScriptCallInstruction::Call(match result_5.clone() {
                                                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source1_data_property_value311) => {
                                                                                            source_ml_source1_data_property_value311
                                                                                        }
                                                                                    }, match callee.clone() {
                                                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source1_data_property_value312) => {
                                                                                            source_ml_source1_data_property_value312
                                                                                        }
                                                                                    }, match receiver.clone() {
                                                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source1_data_property_value313) => {
                                                                                            source_ml_source1_data_property_value313
                                                                                        }
                                                                                    }, arguments__2));
                                                                                    self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source1_nominal_override319.clone()), source_ml_source1_nominal_field317)
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
