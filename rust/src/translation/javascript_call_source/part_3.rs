// Generated from complete js/src/translation/javascript-call-source.js. Do not edit by hand.
use super::JavaScriptCallFunction;
use super::JavaScriptCallInstruction;
use super::JavaScriptCallProgram;
use super::JavaScriptCallSource;
use super::JavaScriptCallSourceBinding;
use super::JavaScriptCallSourceBuilder;
use super::JavaScriptCallSourceCompilation;
use super::JavaScriptCallSourceResult;
use super::allocate_java_script_call_source_register;
use super::append_java_script_call_source_instruction;
use super::lower_java_script_call_source_expression;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::ml_source_exception;
use super::refuse_java_script_call_source;
use super::test_java_script_call_code_coordinate;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn lower_java_script_call_source_branch(source: self::JavaScriptCallSource, bindings: Vec<self::JavaScriptCallSourceBinding>, builder: self::JavaScriptCallSourceBuilder, depth: f64) -> self::JavaScriptCallSourceResult {
    match source.clone() {
        self::JavaScriptCallSource::SourceConstant(source_ml_source1_f320) => {
            let source_ml_source1_f320 = *source_ml_source1_f320;
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
        self::JavaScriptCallSource::SourceBinding(source_ml_source1_f321) => {
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
        self::JavaScriptCallSource::SourceLet(source_ml_source1_f322, source_ml_source1_f323, source_ml_source1_f324) => {
            let source_ml_source1_f323 = *source_ml_source1_f323;
            let source_ml_source1_f324 = *source_ml_source1_f324;
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
        self::JavaScriptCallSource::SourceIf(source_condition, source_yes, source_no) => {
            let source_condition = *source_condition;
            let source_yes = *source_yes;
            let source_no = *source_no;
            {
                let condition = self::lower_java_script_call_source_expression(source_condition.clone(), bindings.clone(), builder.clone(), (depth + 1f64));
                {
                    let result = self::allocate_java_script_call_source_register(match condition.clone() {
                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source1_data_property_value335, _) => {
                            let source_ml_source1_data_property_value335 = *source_ml_source1_data_property_value335;
                            source_ml_source1_data_property_value335.clone()
                        }
                    });
                    {
                        let branch = ((match match result.clone() {
                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source1_data_property_value336, _) => {
                                let source_ml_source1_data_property_value336 = *source_ml_source1_data_property_value336;
                                source_ml_source1_data_property_value336.clone()
                            }
                        } {
                            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source1_data_property_value337, _, _, _) => {
                                source_ml_source1_data_property_value337
                            }
                        }).len() as f64);
                        {
                            let start = self::append_java_script_call_source_instruction(match result.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source1_data_property_value338, _) => {
                                    let source_ml_source1_data_property_value338 = *source_ml_source1_data_property_value338;
                                    source_ml_source1_data_property_value338.clone()
                                }
                            }, self::JavaScriptCallInstruction::Branch(match condition.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source1_data_property_value339) => {
                                    source_ml_source1_data_property_value339
                                }
                            }, 0f64, 0f64));
                            {
                                let yes_position = ((match &start { self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source1_data_property_value340, _, _, _) => source_ml_source1_data_property_value340.as_slice() }).len() as f64);
                                {
                                    let yes = self::lower_java_script_call_source_expression(source_yes.clone(), bindings.clone(), start.clone(), (depth + 1f64));
                                    {
                                        let copied_yes = self::append_java_script_call_source_instruction(match yes.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source1_data_property_value341, _) => {
                                                let source_ml_source1_data_property_value341 = *source_ml_source1_data_property_value341;
                                                source_ml_source1_data_property_value341.clone()
                                            }
                                        }, self::JavaScriptCallInstruction::Copy(match result.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source1_data_property_value342) => {
                                                source_ml_source1_data_property_value342
                                            }
                                        }, match yes.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source1_data_property_value343) => {
                                                source_ml_source1_data_property_value343
                                            }
                                        }));
                                        {
                                            let jump = ((match &copied_yes { self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source1_data_property_value344, _, _, _) => source_ml_source1_data_property_value344.as_slice() }).len() as f64);
                                            {
                                                let alternative = self::append_java_script_call_source_instruction(copied_yes.clone(), self::JavaScriptCallInstruction::Jump(0f64));
                                                {
                                                    let no_position = ((match &alternative { self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source1_data_property_value345, _, _, _) => source_ml_source1_data_property_value345.as_slice() }).len() as f64);
                                                    {
                                                        let no = self::lower_java_script_call_source_expression(source_no.clone(), bindings, alternative.clone(), (depth + 1f64));
                                                        {
                                                            let copied_no = self::append_java_script_call_source_instruction(match no.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source1_data_property_value346, _) => {
                                                                    let source_ml_source1_data_property_value346 = *source_ml_source1_data_property_value346;
                                                                    source_ml_source1_data_property_value346.clone()
                                                                }
                                                            }, self::JavaScriptCallInstruction::Copy(match result.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source1_data_property_value347) => {
                                                                    source_ml_source1_data_property_value347
                                                                }
                                                            }, match no.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source1_data_property_value348) => {
                                                                    source_ml_source1_data_property_value348
                                                                }
                                                            }));
                                                            {
                                                                let end = ((match &copied_no { self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source1_data_property_value349, _, _, _) => source_ml_source1_data_property_value349.as_slice() }).len() as f64);
                                                                if ((match copied_no.clone() {
                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source1_data_property_value351) => {
                                                                        source_ml_source1_data_property_value351.clone()
                                                                    }
                                                                }) != "") {
                                                                    self::JavaScriptCallSourceResult::SourceResult(Box::new(copied_no.clone()), 0f64)
                                                                } else {
                                                                    {
                                                                        let instructions = { let ml_values2 = match copied_no.clone() {
                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source1_data_property_value352, _, _, _) => {
                                                                                source_ml_source1_data_property_value352
                                                                            }
                                                                        }; let ml_callback2 = { let branch = branch; let condition = condition.clone(); let yes_position = yes_position; let no_position = no_position; let jump = jump; let end = end; std::rc::Rc::new(move |entry: self::JavaScriptCallInstruction, index: f64| -> self::JavaScriptCallInstruction { if (index == branch) {
                                                                            self::JavaScriptCallInstruction::Branch(match condition.clone() {
                                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source1_data_property_value353) => {
                                                                                    source_ml_source1_data_property_value353
                                                                                }
                                                                            }, yes_position, no_position)
                                                                        } else {
                                                                            if (index == jump) {
                                                                                self::JavaScriptCallInstruction::Jump(end)
                                                                            } else {
                                                                                entry.clone()
                                                                            }
                                                                        } }) }; ml_values2.iter().cloned().enumerate().map(|(ml_index2, ml_value2)| ml_callback2(ml_value2, (ml_index2 as f64))).collect::<Vec<_>>() };
                                                                        self::JavaScriptCallSourceResult::SourceResult(Box::new({
                                                                            let source_ml_source1_nominal_spread354 = copied_no.clone();
                                                                            {
                                                                                let source_ml_source1_nominal_field355 = match source_ml_source1_nominal_spread354.clone() {
                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source1_data_property_value356, _, _, _) => {
                                                                                        source_ml_source1_data_property_value356
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source1_nominal_field357 = match source_ml_source1_nominal_spread354.clone() {
                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source1_data_property_value358, _, _) => {
                                                                                            source_ml_source1_data_property_value358
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let source_ml_source1_nominal_field359 = match source_ml_source1_nominal_spread354.clone() {
                                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source1_data_property_value360, _) => {
                                                                                                source_ml_source1_data_property_value360
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_ml_source1_nominal_field361 = match source_ml_source1_nominal_spread354.clone() {
                                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source1_data_property_value362) => {
                                                                                                    source_ml_source1_data_property_value362.clone()
                                                                                                }
                                                                                            };
                                                                                            {
                                                                                                let source_ml_source1_nominal_override363 = instructions;
                                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source1_nominal_override363, source_ml_source1_nominal_field357, source_ml_source1_nominal_field359, source_ml_source1_nominal_field361.clone())
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }), match result.clone() {
                                                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source1_data_property_value364) => {
                                                                                source_ml_source1_data_property_value364
                                                                            }
                                                                        })
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
        self::JavaScriptCallSource::SourceNumberBinary(source_ml_source1_f325, source_ml_source1_f326, source_ml_source1_f327) => {
            let source_ml_source1_f326 = *source_ml_source1_f326;
            let source_ml_source1_f327 = *source_ml_source1_f327;
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
        self::JavaScriptCallSource::SourceSameValue(source_ml_source1_f328, source_ml_source1_f329) => {
            let source_ml_source1_f328 = *source_ml_source1_f328;
            let source_ml_source1_f329 = *source_ml_source1_f329;
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
        self::JavaScriptCallSource::SourceStrictEqual(source_ml_source1_f330, source_ml_source1_f331) => {
            let source_ml_source1_f330 = *source_ml_source1_f330;
            let source_ml_source1_f331 = *source_ml_source1_f331;
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
        self::JavaScriptCallSource::SourceCall(source_ml_source1_f332, source_ml_source1_f333, source_ml_source1_f334) => {
            let source_ml_source1_f332 = *source_ml_source1_f332;
            let source_ml_source1_f333 = *source_ml_source1_f333;
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn compile_java_script_call_source(source: self::JavaScriptCallSource, parameters: &[f64]) -> self::JavaScriptCallSourceCompilation {
    if (((parameters.len() as f64)) >= (4096f64)) {
        self::JavaScriptCallSourceCompilation::SourceCompiled(Vec::<self::JavaScriptCallInstruction>::new(), 0f64, String::from("source parameters exceed their explicit register budget"))
    } else {
        {
            let index = 0f64;
            {
                let source_ml_source1_s365 = self::source_ml_temporary_compile_java_script_call_source_loop7(parameters, index);
                match source_ml_source1_s365.clone() {
                    self::MlTemporaryCompileJavaScriptCallSourceLoop7Result::MlTemporaryCompileJavaScriptCallSourceLoop7Done => {
                        {
                            let bindings = { let ml_values3 = parameters.to_vec(); let ml_callback3 = std::rc::Rc::new(move |identity: f64, index_2: f64| -> self::JavaScriptCallSourceBinding { self::JavaScriptCallSourceBinding::SourceBindingRegister(identity, (index_2 + 1f64)) }); ml_values3.iter().cloned().enumerate().map(|(ml_index3, ml_value3)| ml_callback3(ml_value3, (ml_index3 as f64))).collect::<Vec<_>>() };
                            {
                                let result = self::lower_java_script_call_source_expression(source.clone(), bindings, self::JavaScriptCallSourceBuilder::SourceBuilder(Vec::<self::JavaScriptCallInstruction>::new(), ((parameters.len() as f64) + 1f64), 0f64, String::from("")), 0f64);
                                {
                                    let builder = self::append_java_script_call_source_instruction(match result.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source1_data_property_value366, _) => {
                                            let source_ml_source1_data_property_value366 = *source_ml_source1_data_property_value366;
                                            source_ml_source1_data_property_value366.clone()
                                        }
                                    }, self::JavaScriptCallInstruction::Return(match result.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source1_data_property_value367) => {
                                            source_ml_source1_data_property_value367
                                        }
                                    }));
                                    self::JavaScriptCallSourceCompilation::SourceCompiled(if ((match builder.clone() {
                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source1_data_property_value369) => {
                                            source_ml_source1_data_property_value369.clone()
                                        }
                                    }) == "") {
                                        match builder.clone() {
                                            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source1_data_property_value370, _, _, _) => {
                                                source_ml_source1_data_property_value370
                                            }
                                        }
                                    } else {
                                        Vec::<self::JavaScriptCallInstruction>::new()
                                    }, if ((match builder.clone() {
                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source1_data_property_value372) => {
                                            source_ml_source1_data_property_value372.clone()
                                        }
                                    }) == "") {
                                        match builder.clone() {
                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source1_data_property_value373, _, _) => {
                                                source_ml_source1_data_property_value373
                                            }
                                        }
                                    } else {
                                        0f64
                                    }, match builder.clone() {
                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source1_data_property_value374) => {
                                            source_ml_source1_data_property_value374.clone()
                                        }
                                    })
                                }
                            }
                        }
                    }
                    self::MlTemporaryCompileJavaScriptCallSourceLoop7Result::MlTemporaryCompileJavaScriptCallSourceLoop7Return(source_ml_temporary_result7) => {
                        let source_ml_temporary_result7 = *source_ml_temporary_result7;
                        source_ml_temporary_result7.clone()
                    }
                }
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptSemanticValueErrorLoop1Result {
    MlTemporaryReadJavaScriptSemanticValueErrorLoop1Done,
    MlTemporaryReadJavaScriptSemanticValueErrorLoop1Return(String),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_semantic_value_error_loop1(units: &[f64], mut at: f64) -> self::MlTemporaryReadJavaScriptSemanticValueErrorLoop1Result {
    loop {
        return if (at < ((units.len() as f64))) {
            {
                let unit = (self::ml_array::optional_at(units, self::ml_array::number_index(at))).clone().unwrap_or_else(|| (-1f64));
                if ((!(self::ml_math::is_integer(unit)) || (unit < (0f64))) || (unit > (65535f64))) {
                    self::MlTemporaryReadJavaScriptSemanticValueErrorLoop1Result::MlTemporaryReadJavaScriptSemanticValueErrorLoop1Return(String::from("semantic text requires complete ordered UTF16 code units"))
                } else {
                    {
                        let at_2 = (at + 1f64);
                        {
                            at = at_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptSemanticValueErrorLoop1Result::MlTemporaryReadJavaScriptSemanticValueErrorLoop1Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryTestJavaScriptSemanticSameValueLoop2Result {
    MlTemporaryTestJavaScriptSemanticSameValueLoop2Done,
    MlTemporaryTestJavaScriptSemanticSameValueLoop2Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_test_java_script_semantic_same_value_loop2(left_units: &[f64], right_units: &[f64], mut at: f64) -> self::MlTemporaryTestJavaScriptSemanticSameValueLoop2Result {
    loop {
        return if (at < ((left_units.len() as f64))) {
            if (((self::ml_array::optional_at(left_units, self::ml_array::number_index(at))).clone().unwrap_or_else(|| (-1f64))) != ((self::ml_array::optional_at(right_units, self::ml_array::number_index(at))).clone().unwrap_or_else(|| (-1f64)))) {
                self::MlTemporaryTestJavaScriptSemanticSameValueLoop2Result::MlTemporaryTestJavaScriptSemanticSameValueLoop2Return(false)
            } else {
                {
                    let at_2 = (at + 1f64);
                    {
                        at = at_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlTemporaryTestJavaScriptSemanticSameValueLoop2Result::MlTemporaryTestJavaScriptSemanticSameValueLoop2Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryFindJavaScriptCallCodeFunctionLoop3Result {
    MlTemporaryFindJavaScriptCallCodeFunctionLoop3Done,
    MlTemporaryFindJavaScriptCallCodeFunctionLoop3Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_find_java_script_call_code_function_loop3(mut program: self::JavaScriptCallProgram, mut identity: self::ml::Big, mut low: f64, mut high: f64) -> self::MlTemporaryFindJavaScriptCallCodeFunctionLoop3Result {
    loop {
        return if (low < high) {
            {
                let middle = (((low + high) / 2f64)).floor();
                {
                    let entry = self::ml_array::optional_at((match &program { self::JavaScriptCallProgram::CallProgram(source_ml_source1_data_property_value375, _, _) => source_ml_source1_data_property_value375.as_slice() }), self::ml_array::number_index(middle));
                    if match (entry.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                        self::MlTemporaryFindJavaScriptCallCodeFunctionLoop3Result::MlTemporaryFindJavaScriptCallCodeFunctionLoop3Return((-1f64))
                    } else {
                        if ((match entry.clone().expect("a proven present value") {
                            self::JavaScriptCallFunction::CallFunction(source_ml_source1_data_property_value377, _, _, _, _, _, _, _) => {
                                source_ml_source1_data_property_value377.clone()
                            }
                        }) == identity) {
                            self::MlTemporaryFindJavaScriptCallCodeFunctionLoop3Result::MlTemporaryFindJavaScriptCallCodeFunctionLoop3Return(middle)
                        } else {
                            if ((match entry.clone().expect("a proven present value") {
                                self::JavaScriptCallFunction::CallFunction(source_ml_source1_data_property_value379, _, _, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value379.clone()
                                }
                            }) < identity) {
                                {
                                    let low_2 = (middle + 1f64);
                                    {
                                        low = low_2;
                                        continue;
                                    }
                                }
                            } else {
                                {
                                    let high_2 = middle;
                                    {
                                        high = high_2;
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryFindJavaScriptCallCodeFunctionLoop3Result::MlTemporaryFindJavaScriptCallCodeFunctionLoop3Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryLowerJavaScriptCallSourceExpressionLoop5Result {
    MlTemporaryLowerJavaScriptCallSourceExpressionLoop5Done,
    MlTemporaryLowerJavaScriptCallSourceExpressionLoop5Return(Box<self::JavaScriptCallSourceResult>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_lower_java_script_call_source_expression_loop5(mut source: self::JavaScriptCallSource, mut bindings: Vec<self::JavaScriptCallSourceBinding>, mut state: self::JavaScriptCallSourceBuilder, mut index: f64) -> self::MlTemporaryLowerJavaScriptCallSourceExpressionLoop5Result {
    loop {
        return if (index >= (0f64)) {
            {
                let entry = self::ml_array::optional_at(&bindings, self::ml_array::number_index(index));
                if (!(match (entry.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) && {
                    let source_ml_source1_comparison_operand385 = {
                        let source_ml_source1_nullish_field_receiver380 = (entry.clone(),);
                        if match (source_ml_source1_nullish_field_receiver380.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                            {
                                let source_ml_source1_nullish_field_present381 = ((source_ml_source1_nullish_field_receiver380.clone()).0).clone().expect("a proven present value");
                                match source_ml_source1_nullish_field_present381.clone() {
                                    self::JavaScriptCallSourceBinding::SourceBindingRegister(source_ml_source1_data_property_value382, _) => {
                                        source_ml_source1_data_property_value382
                                    }
                                }
                            }
                        } else {
                            (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (String::from("Cannot read properties of undefined (reading 'identity')")).encode_utf16().collect(), message_present: true }).raise()
                        }
                    };
                    {
                        let source_ml_source1_comparison_operand386 = match source.clone() {
                            self::JavaScriptCallSource::SourceConstant(_) => {
                                None::<f64>
                            }
                            self::JavaScriptCallSource::SourceBinding(source_ml_source1_data_property_value383) => {
                                Some(source_ml_source1_data_property_value383)
                            }
                            self::JavaScriptCallSource::SourceLet(source_ml_source1_data_property_value384, _, _) => {
                                Some(source_ml_source1_data_property_value384)
                            }
                            self::JavaScriptCallSource::SourceIf(_, _, _) => {
                                None::<f64>
                            }
                            self::JavaScriptCallSource::SourceNumberBinary(_, _, _) => {
                                None::<f64>
                            }
                            self::JavaScriptCallSource::SourceSameValue(_, _) => {
                                None::<f64>
                            }
                            self::JavaScriptCallSource::SourceStrictEqual(_, _) => {
                                None::<f64>
                            }
                            self::JavaScriptCallSource::SourceCall(_, _, _) => {
                                None::<f64>
                            }
                        };
                        match (source_ml_source1_comparison_operand385, source_ml_source1_comparison_operand386.clone()) { (ml_left, Some(ml_right)) => ml_left == ml_right, _ => false }
                    }
                }) {
                    self::MlTemporaryLowerJavaScriptCallSourceExpressionLoop5Result::MlTemporaryLowerJavaScriptCallSourceExpressionLoop5Return(Box::new(self::JavaScriptCallSourceResult::SourceResult(Box::new(state.clone()), {
                        let source_ml_source1_nullish_field_receiver387 = (entry.clone(),);
                        if match (source_ml_source1_nullish_field_receiver387.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                            {
                                let source_ml_source1_nullish_field_present388 = ((source_ml_source1_nullish_field_receiver387.clone()).0).clone().expect("a proven present value");
                                match source_ml_source1_nullish_field_present388.clone() {
                                    self::JavaScriptCallSourceBinding::SourceBindingRegister(_, source_ml_source1_data_property_value389) => {
                                        source_ml_source1_data_property_value389
                                    }
                                }
                            }
                        } else {
                            (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (String::from("Cannot read properties of undefined (reading 'register')")).encode_utf16().collect(), message_present: true }).raise()
                        }
                    })))
                } else {
                    {
                        let index_2 = (index - 1f64);
                        {
                            index = index_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryLowerJavaScriptCallSourceExpressionLoop5Result::MlTemporaryLowerJavaScriptCallSourceExpressionLoop5Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryLowerJavaScriptCallSourceExpressionLoop6Result {
    MlTemporaryLowerJavaScriptCallSourceExpressionLoop6Done(Box<self::JavaScriptCallSourceBuilder>, Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_lower_java_script_call_source_expression_loop6(mut bindings: Vec<self::JavaScriptCallSourceBinding>, mut depth: f64, mut current: self::JavaScriptCallSourceBuilder, mut arguments_: Vec<f64>, mut source_ml_temporary_values4: Vec<self::JavaScriptCallSource>, mut source_ml_temporary_index4: self::ml::Big) -> self::MlTemporaryLowerJavaScriptCallSourceExpressionLoop6Result {
    loop {
        return if (source_ml_temporary_index4 < (self::ml::Big::from_u128(source_ml_temporary_values4.len() as u128))) {
            {
                let argument = self::ml_array::at(&source_ml_temporary_values4, source_ml_temporary_index4.to_index());
                {
                    let value = self::lower_java_script_call_source_expression(argument.clone(), bindings.clone(), current.clone(), (depth + 1f64));
                    {
                        let current_2 = match value.clone() {
                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source1_data_property_value390, _) => {
                                let source_ml_source1_data_property_value390 = *source_ml_source1_data_property_value390;
                                source_ml_source1_data_property_value390.clone()
                            }
                        };
                        {
                            let arguments__2 = { let (ml_append_right, ml_append_left) = (vec![match value.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source1_data_property_value391) => {
                                    source_ml_source1_data_property_value391
                                }
                            }], arguments_); self::ml_array::append(ml_append_left, ml_append_right) };
                            {
                                let source_ml_temporary_index4_2 = source_ml_temporary_index4.add(&self::ml::Big::from_i128(1));
                                {
                                    (current, arguments_, source_ml_temporary_index4) = (current_2.clone(), arguments__2, source_ml_temporary_index4_2.clone());
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryLowerJavaScriptCallSourceExpressionLoop6Result::MlTemporaryLowerJavaScriptCallSourceExpressionLoop6Done(Box::new(current.clone()), arguments_)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryCompileJavaScriptCallSourceLoop7Result {
    MlTemporaryCompileJavaScriptCallSourceLoop7Done,
    MlTemporaryCompileJavaScriptCallSourceLoop7Return(Box<self::JavaScriptCallSourceCompilation>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_compile_java_script_call_source_loop7(parameters: &[f64], mut index: f64) -> self::MlTemporaryCompileJavaScriptCallSourceLoop7Result {
    loop {
        return if (index < ((parameters.len() as f64))) {
            {
                let owner = (self::ml_array::optional_at(parameters, self::ml_array::number_index(index))).clone().unwrap_or_else(|| (-1f64));
                if (!(self::test_java_script_call_code_coordinate(owner, 65536f64)) || ((self::ml_array::index_of(parameters, &owner, 0f64, false, |a, b| a == b)) != index)) {
                    self::MlTemporaryCompileJavaScriptCallSourceLoop7Result::MlTemporaryCompileJavaScriptCallSourceLoop7Return(Box::new(self::JavaScriptCallSourceCompilation::SourceCompiled(Vec::<self::JavaScriptCallInstruction>::new(), 0f64, String::from("source parameters require distinct actual lexical owners"))))
                } else {
                    {
                        let index_2 = (index + 1f64);
                        {
                            index = index_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryCompileJavaScriptCallSourceLoop7Result::MlTemporaryCompileJavaScriptCallSourceLoop7Done
        };
    }
}
