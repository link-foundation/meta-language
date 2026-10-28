// Generated from complete js/src/translation/javascript-call-source.js. Do not edit by hand.
use super::JavaScriptCallFunction;
use super::JavaScriptCallInstruction;
use super::JavaScriptCallProgram;
use super::JavaScriptCallSource;
use super::JavaScriptCallSourceBinding;
use super::JavaScriptCallSourceBuilder;
use super::JavaScriptCallSourceCompilation;
use super::JavaScriptCallSourceResult;
use super::MlTemporaryCompileJavaScriptCallSourceLoop44Result;
use super::allocate_java_script_call_source_register;
use super::append_java_script_call_source_instruction;
use super::index_utf16_text;
use super::lower_java_script_call_source_expression;
use super::match_utf16_text;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::refuse_java_script_call_source;
use super::slice_utf16_text;
use super::source_ml_temporary_compile_java_script_call_source_loop44;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn lower_java_script_call_source_branch(source: self::JavaScriptCallSource, bindings: Vec<self::JavaScriptCallSourceBinding>, builder: self::JavaScriptCallSourceBuilder, depth: f64) -> self::JavaScriptCallSourceResult {
    match source.clone() {
        self::JavaScriptCallSource::SourceConstant(source_ml_source1_f445) => {
            let source_ml_source1_f445 = *source_ml_source1_f445;
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
        self::JavaScriptCallSource::SourceBinding(source_ml_source1_f446) => {
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
        self::JavaScriptCallSource::SourceLet(source_ml_source1_f447, source_ml_source1_f448, source_ml_source1_f449) => {
            let source_ml_source1_f448 = *source_ml_source1_f448;
            let source_ml_source1_f449 = *source_ml_source1_f449;
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
                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source1_data_property_value468, _) => {
                            let source_ml_source1_data_property_value468 = *source_ml_source1_data_property_value468;
                            source_ml_source1_data_property_value468.clone()
                        }
                    });
                    {
                        let branch = ((match match result.clone() {
                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source1_data_property_value469, _) => {
                                let source_ml_source1_data_property_value469 = *source_ml_source1_data_property_value469;
                                source_ml_source1_data_property_value469.clone()
                            }
                        } {
                            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source1_data_property_value470, _, _, _, _) => {
                                source_ml_source1_data_property_value470
                            }
                        }).len() as f64);
                        {
                            let start = self::append_java_script_call_source_instruction(match result.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source1_data_property_value471, _) => {
                                    let source_ml_source1_data_property_value471 = *source_ml_source1_data_property_value471;
                                    source_ml_source1_data_property_value471.clone()
                                }
                            }, self::JavaScriptCallInstruction::Branch(match condition.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source1_data_property_value472) => {
                                    source_ml_source1_data_property_value472
                                }
                            }, 0f64, 0f64));
                            {
                                let yes_position = ((match &start { self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source1_data_property_value473, _, _, _, _) => source_ml_source1_data_property_value473.as_slice() }).len() as f64);
                                {
                                    let yes = self::lower_java_script_call_source_expression(source_yes.clone(), bindings.clone(), start.clone(), (depth + 1f64));
                                    {
                                        let copied_yes = self::append_java_script_call_source_instruction(match yes.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source1_data_property_value474, _) => {
                                                let source_ml_source1_data_property_value474 = *source_ml_source1_data_property_value474;
                                                source_ml_source1_data_property_value474.clone()
                                            }
                                        }, self::JavaScriptCallInstruction::Copy(match result.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source1_data_property_value475) => {
                                                source_ml_source1_data_property_value475
                                            }
                                        }, match yes.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source1_data_property_value476) => {
                                                source_ml_source1_data_property_value476
                                            }
                                        }));
                                        {
                                            let jump = ((match &copied_yes { self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source1_data_property_value477, _, _, _, _) => source_ml_source1_data_property_value477.as_slice() }).len() as f64);
                                            {
                                                let alternative = self::append_java_script_call_source_instruction(copied_yes.clone(), self::JavaScriptCallInstruction::Jump(0f64));
                                                {
                                                    let no_position = ((match &alternative { self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source1_data_property_value478, _, _, _, _) => source_ml_source1_data_property_value478.as_slice() }).len() as f64);
                                                    {
                                                        let no = self::lower_java_script_call_source_expression(source_no.clone(), bindings, alternative.clone(), (depth + 1f64));
                                                        {
                                                            let copied_no = self::append_java_script_call_source_instruction(match no.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source1_data_property_value479, _) => {
                                                                    let source_ml_source1_data_property_value479 = *source_ml_source1_data_property_value479;
                                                                    source_ml_source1_data_property_value479.clone()
                                                                }
                                                            }, self::JavaScriptCallInstruction::Copy(match result.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source1_data_property_value480) => {
                                                                    source_ml_source1_data_property_value480
                                                                }
                                                            }, match no.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source1_data_property_value481) => {
                                                                    source_ml_source1_data_property_value481
                                                                }
                                                            }));
                                                            {
                                                                let end = ((match &copied_no { self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source1_data_property_value482, _, _, _, _) => source_ml_source1_data_property_value482.as_slice() }).len() as f64);
                                                                if ((match copied_no.clone() {
                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source1_data_property_value484) => {
                                                                        source_ml_source1_data_property_value484.clone()
                                                                    }
                                                                }) != "") {
                                                                    self::JavaScriptCallSourceResult::SourceResult(Box::new(copied_no.clone()), 0f64)
                                                                } else {
                                                                    {
                                                                        let instructions = { let ml_values2 = match copied_no.clone() {
                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source1_data_property_value485, _, _, _, _) => {
                                                                                source_ml_source1_data_property_value485
                                                                            }
                                                                        }; let ml_callback2 = { let branch = branch; let condition = condition.clone(); let yes_position = yes_position; let no_position = no_position; let jump = jump; let end = end; std::rc::Rc::new(move |entry: self::JavaScriptCallInstruction, index: f64| -> self::JavaScriptCallInstruction { if (index == branch) {
                                                                            self::JavaScriptCallInstruction::Branch(match condition.clone() {
                                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source1_data_property_value486) => {
                                                                                    source_ml_source1_data_property_value486
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
                                                                            let source_ml_source1_nominal_spread487 = copied_no.clone();
                                                                            {
                                                                                let source_ml_source1_nominal_field488 = match source_ml_source1_nominal_spread487.clone() {
                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source1_data_property_value489, _, _, _, _) => {
                                                                                        source_ml_source1_data_property_value489
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source1_nominal_field490 = match source_ml_source1_nominal_spread487.clone() {
                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source1_data_property_value491, _, _, _) => {
                                                                                            source_ml_source1_data_property_value491
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let source_ml_source1_nominal_field492 = match source_ml_source1_nominal_spread487.clone() {
                                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source1_data_property_value493, _, _) => {
                                                                                                source_ml_source1_data_property_value493
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_ml_source1_nominal_field494 = match source_ml_source1_nominal_spread487.clone() {
                                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source1_data_property_value495, _) => {
                                                                                                    source_ml_source1_data_property_value495
                                                                                                }
                                                                                            };
                                                                                            {
                                                                                                let source_ml_source1_nominal_field496 = match source_ml_source1_nominal_spread487.clone() {
                                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source1_data_property_value497) => {
                                                                                                        source_ml_source1_data_property_value497.clone()
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    let source_ml_source1_nominal_override498 = instructions;
                                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source1_nominal_override498, source_ml_source1_nominal_field490, source_ml_source1_nominal_field492, source_ml_source1_nominal_field494, source_ml_source1_nominal_field496.clone())
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }), match result.clone() {
                                                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source1_data_property_value499) => {
                                                                                source_ml_source1_data_property_value499
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
        self::JavaScriptCallSource::SourceNumberBinary(source_ml_source1_f450, source_ml_source1_f451, source_ml_source1_f452) => {
            let source_ml_source1_f451 = *source_ml_source1_f451;
            let source_ml_source1_f452 = *source_ml_source1_f452;
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
        self::JavaScriptCallSource::SourceSameValue(source_ml_source1_f453, source_ml_source1_f454) => {
            let source_ml_source1_f453 = *source_ml_source1_f453;
            let source_ml_source1_f454 = *source_ml_source1_f454;
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
        self::JavaScriptCallSource::SourceStrictEqual(source_ml_source1_f455, source_ml_source1_f456) => {
            let source_ml_source1_f455 = *source_ml_source1_f455;
            let source_ml_source1_f456 = *source_ml_source1_f456;
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
        self::JavaScriptCallSource::SourcePrimitive(source_ml_source1_f457, source_ml_source1_f458) => {
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
        self::JavaScriptCallSource::SourceRecur(source_ml_source1_f459) => {
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
        self::JavaScriptCallSource::SourceSet(source_ml_source1_f460, source_ml_source1_f461, source_ml_source1_f462) => {
            let source_ml_source1_f460 = *source_ml_source1_f460;
            let source_ml_source1_f461 = *source_ml_source1_f461;
            let source_ml_source1_f462 = *source_ml_source1_f462;
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
        self::JavaScriptCallSource::SourceGet(source_ml_source1_f463, source_ml_source1_f464) => {
            let source_ml_source1_f463 = *source_ml_source1_f463;
            let source_ml_source1_f464 = *source_ml_source1_f464;
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
        self::JavaScriptCallSource::SourceCall(source_ml_source1_f465, source_ml_source1_f466, source_ml_source1_f467) => {
            let source_ml_source1_f465 = *source_ml_source1_f465;
            let source_ml_source1_f466 = *source_ml_source1_f466;
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
                let source_ml_source1_s500 = self::source_ml_temporary_compile_java_script_call_source_loop44(parameters, index);
                match source_ml_source1_s500.clone() {
                    self::MlTemporaryCompileJavaScriptCallSourceLoop44Result::MlTemporaryCompileJavaScriptCallSourceLoop44Done => {
                        {
                            let bindings = { let ml_values3 = parameters.to_vec(); let ml_callback3 = std::rc::Rc::new(move |identity: f64, index_2: f64| -> self::JavaScriptCallSourceBinding { self::JavaScriptCallSourceBinding::SourceBindingRegister(identity, (index_2 + 1f64)) }); ml_values3.iter().cloned().enumerate().map(|(ml_index3, ml_value3)| ml_callback3(ml_value3, (ml_index3 as f64))).collect::<Vec<_>>() };
                            {
                                let result = self::lower_java_script_call_source_expression(source.clone(), bindings, self::JavaScriptCallSourceBuilder::SourceBuilder(Vec::<self::JavaScriptCallInstruction>::new(), ((parameters.len() as f64) + 1f64), 0f64, (parameters.len() as f64), String::from("")), 0f64);
                                {
                                    let builder = self::append_java_script_call_source_instruction(match result.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source1_data_property_value501, _) => {
                                            let source_ml_source1_data_property_value501 = *source_ml_source1_data_property_value501;
                                            source_ml_source1_data_property_value501.clone()
                                        }
                                    }, self::JavaScriptCallInstruction::Return(match result.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source1_data_property_value502) => {
                                            source_ml_source1_data_property_value502
                                        }
                                    }));
                                    self::JavaScriptCallSourceCompilation::SourceCompiled(if ((match builder.clone() {
                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source1_data_property_value504) => {
                                            source_ml_source1_data_property_value504.clone()
                                        }
                                    }) == "") {
                                        match builder.clone() {
                                            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source1_data_property_value505, _, _, _, _) => {
                                                source_ml_source1_data_property_value505
                                            }
                                        }
                                    } else {
                                        Vec::<self::JavaScriptCallInstruction>::new()
                                    }, if ((match builder.clone() {
                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source1_data_property_value507) => {
                                            source_ml_source1_data_property_value507.clone()
                                        }
                                    }) == "") {
                                        match builder.clone() {
                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source1_data_property_value508, _, _, _) => {
                                                source_ml_source1_data_property_value508
                                            }
                                        }
                                    } else {
                                        0f64
                                    }, match builder.clone() {
                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source1_data_property_value509) => {
                                            source_ml_source1_data_property_value509.clone()
                                        }
                                    })
                                }
                            }
                        }
                    }
                    self::MlTemporaryCompileJavaScriptCallSourceLoop44Result::MlTemporaryCompileJavaScriptCallSourceLoop44Return(source_ml_temporary_result44) => {
                        let source_ml_temporary_result44 = *source_ml_temporary_result44;
                        source_ml_temporary_result44.clone()
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
                    let entry = self::ml_array::optional_at((match &program { self::JavaScriptCallProgram::CallProgram(source_ml_source1_data_property_value510, _, _) => source_ml_source1_data_property_value510.as_slice() }), self::ml_array::number_index(middle));
                    if match (entry.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                        self::MlTemporaryFindJavaScriptCallCodeFunctionLoop3Result::MlTemporaryFindJavaScriptCallCodeFunctionLoop3Return((-1f64))
                    } else {
                        if ((match entry.clone().expect("a proven present value") {
                            self::JavaScriptCallFunction::CallFunction(source_ml_source1_data_property_value512, _, _, _, _, _, _, _) => {
                                source_ml_source1_data_property_value512.clone()
                            }
                        }) == identity) {
                            self::MlTemporaryFindJavaScriptCallCodeFunctionLoop3Result::MlTemporaryFindJavaScriptCallCodeFunctionLoop3Return(middle)
                        } else {
                            if ((match entry.clone().expect("a proven present value") {
                                self::JavaScriptCallFunction::CallFunction(source_ml_source1_data_property_value514, _, _, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value514.clone()
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
pub enum MlTemporaryReadJavaScriptUtf8StringUnitsLoop5Result {
    MlTemporaryReadJavaScriptUtf8StringUnitsLoop5Done(Vec<f64>),
    MlTemporaryReadJavaScriptUtf8StringUnitsLoop5Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadJavaScriptUtf8StringUnitsLoop7Result {
    MlTemporaryReadJavaScriptUtf8StringUnitsLoop7Done(f64, f64),
    MlTemporaryReadJavaScriptUtf8StringUnitsLoop7Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_utf8_string_units_loop7(bytes: &[f64], mut index: f64, mut scalar: f64, mut remaining: f64) -> self::MlTemporaryReadJavaScriptUtf8StringUnitsLoop7Result {
    loop {
        return if (remaining > (0f64)) {
            {
                let next = (self::ml_array::optional_at(bytes, self::ml_array::number_index(index))).clone().unwrap_or_else(|| (-1f64));
                if ((next < (128f64)) || (next > (191f64))) {
                    self::MlTemporaryReadJavaScriptUtf8StringUnitsLoop7Result::MlTemporaryReadJavaScriptUtf8StringUnitsLoop7Return(vec![(-1f64)])
                } else {
                    {
                        let scalar_2 = (((scalar * 64f64) + next) - 128f64);
                        {
                            let index_2 = (index + 1f64);
                            {
                                let remaining_2 = (remaining - 1f64);
                                {
                                    (index, scalar, remaining) = (index_2, scalar_2, remaining_2);
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptUtf8StringUnitsLoop7Result::MlTemporaryReadJavaScriptUtf8StringUnitsLoop7Done(index, scalar)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadJavaScriptUtf8StringUnitsJoin6 {
    MlTemporaryReadJavaScriptUtf8StringUnitsJoin6Next(f64, f64, f64),
    MlTemporaryReadJavaScriptUtf8StringUnitsJoin6Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_utf8_string_units_loop5(bytes: &[f64], mut units: Vec<f64>, mut index: f64) -> self::MlTemporaryReadJavaScriptUtf8StringUnitsLoop5Result {
    loop {
        return if (index < ((bytes.len() as f64))) {
            {
                let scalar = (self::ml_array::optional_at(bytes, self::ml_array::number_index(index))).clone().unwrap_or_else(|| (-1f64));
                {
                    let index_2 = (index + 1f64);
                    {
                        let remaining = 0f64;
                        {
                            let minimum = 0f64;
                            {
                                let source_ml_source1_s515 = if ((scalar >= (194f64)) && (scalar <= (223f64))) {
                                    {
                                        let scalar_2 = (scalar - 192f64);
                                        {
                                            let remaining_2 = 1f64;
                                            {
                                                let minimum_2 = 128f64;
                                                self::MlTemporaryReadJavaScriptUtf8StringUnitsJoin6::MlTemporaryReadJavaScriptUtf8StringUnitsJoin6Next(scalar_2, remaining_2, minimum_2)
                                            }
                                        }
                                    }
                                } else {
                                    if ((scalar >= (224f64)) && (scalar <= (239f64))) {
                                        {
                                            let scalar_3 = (scalar - 224f64);
                                            {
                                                let remaining_3 = 2f64;
                                                {
                                                    let minimum_3 = 2048f64;
                                                    self::MlTemporaryReadJavaScriptUtf8StringUnitsJoin6::MlTemporaryReadJavaScriptUtf8StringUnitsJoin6Next(scalar_3, remaining_3, minimum_3)
                                                }
                                            }
                                        }
                                    } else {
                                        if ((scalar >= (240f64)) && (scalar <= (244f64))) {
                                            {
                                                let scalar_4 = (scalar - 240f64);
                                                {
                                                    let remaining_4 = 3f64;
                                                    {
                                                        let minimum_4 = 65536f64;
                                                        self::MlTemporaryReadJavaScriptUtf8StringUnitsJoin6::MlTemporaryReadJavaScriptUtf8StringUnitsJoin6Next(scalar_4, remaining_4, minimum_4)
                                                    }
                                                }
                                            }
                                        } else {
                                            if (scalar >= (128f64)) {
                                                self::MlTemporaryReadJavaScriptUtf8StringUnitsJoin6::MlTemporaryReadJavaScriptUtf8StringUnitsJoin6Return(vec![(-1f64)])
                                            } else {
                                                self::MlTemporaryReadJavaScriptUtf8StringUnitsJoin6::MlTemporaryReadJavaScriptUtf8StringUnitsJoin6Next(scalar, remaining, minimum)
                                            }
                                        }
                                    }
                                };
                                match source_ml_source1_s515.clone() {
                                    self::MlTemporaryReadJavaScriptUtf8StringUnitsJoin6::MlTemporaryReadJavaScriptUtf8StringUnitsJoin6Next(scalar_5, remaining_5, minimum_5) => {
                                        {
                                            let source_ml_source1_s516 = self::source_ml_temporary_read_java_script_utf8_string_units_loop7(bytes, index_2, scalar_5, remaining_5);
                                            match source_ml_source1_s516.clone() {
                                                self::MlTemporaryReadJavaScriptUtf8StringUnitsLoop7Result::MlTemporaryReadJavaScriptUtf8StringUnitsLoop7Done(index_3, scalar_6) => {
                                                    if (((scalar_6 < minimum_5) || (scalar_6 > (1114111f64))) || ((scalar_6 >= (55296f64)) && (scalar_6 <= (57343f64)))) {
                                                        self::MlTemporaryReadJavaScriptUtf8StringUnitsLoop5Result::MlTemporaryReadJavaScriptUtf8StringUnitsLoop5Return(vec![(-1f64)])
                                                    } else {
                                                        if (scalar_6 <= (65535f64)) {
                                                            {
                                                                let units_2 = { let (ml_append_right, ml_append_left) = (vec![scalar_6], units); self::ml_array::append(ml_append_left, ml_append_right) };
                                                                {
                                                                    (units, index) = (units_2, index_3);
                                                                    continue;
                                                                }
                                                            }
                                                        } else {
                                                            {
                                                                let units_3 = { let (ml_append_right, ml_append_left) = (vec![(55296f64 + (((scalar_6 - 65536f64) / 1024f64)).floor())], units); self::ml_array::append(ml_append_left, ml_append_right) };
                                                                {
                                                                    let units_4 = { let (ml_append_right, ml_append_left) = (vec![(56320f64 + ((scalar_6 - 65536f64) % 1024f64))], units_3); self::ml_array::append(ml_append_left, ml_append_right) };
                                                                    {
                                                                        (units, index) = (units_4, index_3);
                                                                        continue;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                self::MlTemporaryReadJavaScriptUtf8StringUnitsLoop7Result::MlTemporaryReadJavaScriptUtf8StringUnitsLoop7Return(source_ml_temporary_result7) => {
                                                    self::MlTemporaryReadJavaScriptUtf8StringUnitsLoop5Result::MlTemporaryReadJavaScriptUtf8StringUnitsLoop5Return(source_ml_temporary_result7)
                                                }
                                            }
                                        }
                                    }
                                    self::MlTemporaryReadJavaScriptUtf8StringUnitsJoin6::MlTemporaryReadJavaScriptUtf8StringUnitsJoin6Return(source_ml_temporary_result7_2) => {
                                        self::MlTemporaryReadJavaScriptUtf8StringUnitsLoop5Result::MlTemporaryReadJavaScriptUtf8StringUnitsLoop5Return(source_ml_temporary_result7_2)
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptUtf8StringUnitsLoop5Result::MlTemporaryReadJavaScriptUtf8StringUnitsLoop5Done(units)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadJavaScriptUtf8StringUnitsLoop4Result {
    MlTemporaryReadJavaScriptUtf8StringUnitsLoop4Done,
    MlTemporaryReadJavaScriptUtf8StringUnitsLoop4Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_utf8_string_units_loop4(bytes: &[f64], mut index: f64) -> self::MlTemporaryReadJavaScriptUtf8StringUnitsLoop4Result {
    loop {
        return if (index < ((bytes.len() as f64))) {
            {
                let unit = (self::ml_array::optional_at(bytes, self::ml_array::number_index(index))).clone().unwrap_or_else(|| (-1f64));
                if ((!(self::ml_math::is_safe_integer(unit)) || (unit < (0f64))) || (unit > (255f64))) {
                    self::MlTemporaryReadJavaScriptUtf8StringUnitsLoop4Result::MlTemporaryReadJavaScriptUtf8StringUnitsLoop4Return(vec![(-1f64)])
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
            self::MlTemporaryReadJavaScriptUtf8StringUnitsLoop4Result::MlTemporaryReadJavaScriptUtf8StringUnitsLoop4Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_slice_utf16_text_loop8(text: &[f64], mut last: f64, mut result: Vec<f64>, mut at: f64) -> Vec<f64> {
    loop {
        return if (at < last) {
            {
                let result_2 = { let (ml_append_right, ml_append_left) = (vec![(self::ml_array::optional_at(text, self::ml_array::number_index(at))).clone().unwrap_or_else(|| 0f64)], result); self::ml_array::append(ml_append_left, ml_append_right) };
                {
                    let at_2 = (at + 1f64);
                    {
                        (result, at) = (result_2, at_2);
                        continue;
                    }
                }
            }
        } else {
            result
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_points_utf16_text_loop9(text: &[f64], mut result: Vec<Vec<f64>>, mut at: f64) -> Vec<Vec<f64>> {
    loop {
        return if (at < ((text.len() as f64))) {
            {
                let first = (self::ml_array::optional_at(text, self::ml_array::number_index(at))).clone().unwrap_or_else(|| 0f64);
                {
                    let second = (self::ml_array::optional_at(text, self::ml_array::number_index((at + 1f64)))).clone().unwrap_or_else(|| (-1f64));
                    if ((((first >= (55296f64)) && (first <= (56319f64))) && (second >= (56320f64))) && (second <= (57343f64))) {
                        {
                            let result_2 = { let (ml_append_right, ml_append_left) = (vec![vec![first, second]], result); self::ml_array::append(ml_append_left, ml_append_right) };
                            {
                                let at_2 = (at + 1f64);
                                {
                                    let at_3 = (at_2 + 1f64);
                                    {
                                        (result, at) = (result_2, at_3);
                                        continue;
                                    }
                                }
                            }
                        }
                    } else {
                        {
                            let result_3 = { let (ml_append_right, ml_append_left) = (vec![vec![first]], result); self::ml_array::append(ml_append_left, ml_append_right) };
                            {
                                let at_4 = (at + 1f64);
                                {
                                    (result, at) = (result_3, at_4);
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            result
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_split_utf16_text_loop10(text: &[f64], mut count: f64, mut result: Vec<Vec<f64>>, mut at: f64) -> Vec<Vec<f64>> {
    loop {
        return if ((at < ((text.len() as f64))) && (((result.len() as f64)) < count)) {
            {
                let result_2 = { let (ml_append_right, ml_append_left) = (vec![vec![(self::ml_array::optional_at(text, self::ml_array::number_index(at))).clone().unwrap_or_else(|| 0f64)]], result); self::ml_array::append(ml_append_left, ml_append_right) };
                {
                    let at_2 = (at + 1f64);
                    {
                        (result, at) = (result_2, at_2);
                        continue;
                    }
                }
            }
        } else {
            result
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporarySplitUtf16TextLoop11Result {
    MlTemporarySplitUtf16TextLoop11Done(Vec<Vec<f64>>, f64),
    MlTemporarySplitUtf16TextLoop11Return(Vec<Vec<f64>>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_split_utf16_text_loop11(text: &[f64], separator: &[f64], mut count: f64, mut result: Vec<Vec<f64>>, mut start: f64) -> self::MlTemporarySplitUtf16TextLoop11Result {
    loop {
        return {
            let found = self::index_utf16_text(text, separator, start);
            if (found < (0f64)) {
                self::MlTemporarySplitUtf16TextLoop11Result::MlTemporarySplitUtf16TextLoop11Done(result, start)
            } else {
                {
                    let result_2 = { let (ml_append_right, ml_append_left) = (vec![self::slice_utf16_text(text, start, found)], result); self::ml_array::append(ml_append_left, ml_append_right) };
                    if (((result_2.len() as f64)) == count) {
                        self::MlTemporarySplitUtf16TextLoop11Result::MlTemporarySplitUtf16TextLoop11Return(result_2)
                    } else {
                        {
                            let start_2 = (found + (separator.len() as f64));
                            {
                                (result, start) = (result_2, start_2);
                                continue;
                            }
                        }
                    }
                }
            }
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryMatchUtf16TextLoop12Result {
    MlTemporaryMatchUtf16TextLoop12Done,
    MlTemporaryMatchUtf16TextLoop12Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_match_utf16_text_loop12(text: &[f64], search: &[f64], mut start: f64, mut at: f64) -> self::MlTemporaryMatchUtf16TextLoop12Result {
    loop {
        return if (at < ((search.len() as f64))) {
            if {
                let source_ml_source1_comparison_operand517 = self::ml_array::optional_at(text, self::ml_array::number_index((start + at)));
                {
                    let source_ml_source1_comparison_operand518 = self::ml_array::optional_at(search, self::ml_array::number_index(at));
                    !(match (source_ml_source1_comparison_operand517.clone(), source_ml_source1_comparison_operand518.clone()) { (Some(ml_left), Some(ml_right)) => ml_left == ml_right, (None, None) => true, _ => false })
                }
            } {
                self::MlTemporaryMatchUtf16TextLoop12Result::MlTemporaryMatchUtf16TextLoop12Return(false)
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
            self::MlTemporaryMatchUtf16TextLoop12Result::MlTemporaryMatchUtf16TextLoop12Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryIndexUtf16TextLoop13Result {
    MlTemporaryIndexUtf16TextLoop13Done,
    MlTemporaryIndexUtf16TextLoop13Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_index_utf16_text_loop13(text: &[f64], search: &[f64], mut at: f64) -> self::MlTemporaryIndexUtf16TextLoop13Result {
    loop {
        return if (at <= (((text.len() as f64) - (search.len() as f64)))) {
            if self::match_utf16_text(text, search, at) {
                self::MlTemporaryIndexUtf16TextLoop13Result::MlTemporaryIndexUtf16TextLoop13Return(at)
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
            self::MlTemporaryIndexUtf16TextLoop13Result::MlTemporaryIndexUtf16TextLoop13Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryLastIndexUtf16TextLoop14Result {
    MlTemporaryLastIndexUtf16TextLoop14Done,
    MlTemporaryLastIndexUtf16TextLoop14Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_last_index_utf16_text_loop14(text: &[f64], search: &[f64], mut at: f64) -> self::MlTemporaryLastIndexUtf16TextLoop14Result {
    loop {
        return if (at >= (0f64)) {
            if self::match_utf16_text(text, search, at) {
                self::MlTemporaryLastIndexUtf16TextLoop14Result::MlTemporaryLastIndexUtf16TextLoop14Return(at)
            } else {
                {
                    let at_2 = (at - 1f64);
                    {
                        at = at_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlTemporaryLastIndexUtf16TextLoop14Result::MlTemporaryLastIndexUtf16TextLoop14Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryCompareUtf16TextLoop15Result {
    MlTemporaryCompareUtf16TextLoop15Done,
    MlTemporaryCompareUtf16TextLoop15Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_compare_utf16_text_loop15(left: &[f64], right: &[f64], mut count: f64, mut at: f64) -> self::MlTemporaryCompareUtf16TextLoop15Result {
    loop {
        return if (at < count) {
            {
                let a = (self::ml_array::optional_at(left, self::ml_array::number_index(at))).clone().unwrap_or_else(|| 0f64);
                {
                    let b = (self::ml_array::optional_at(right, self::ml_array::number_index(at))).clone().unwrap_or_else(|| 0f64);
                    if (a != b) {
                        self::MlTemporaryCompareUtf16TextLoop15Result::MlTemporaryCompareUtf16TextLoop15Return(if (a < b) {
                            (-1f64)
                        } else {
                            1f64
                        })
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
            }
        } else {
            self::MlTemporaryCompareUtf16TextLoop15Result::MlTemporaryCompareUtf16TextLoop15Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_join_utf16_text_loop17(separator: &[f64], mut result: Vec<f64>, mut offset: f64) -> Vec<f64> {
    loop {
        return if (offset < ((separator.len() as f64))) {
            {
                let result_2 = { let (ml_append_right, ml_append_left) = (vec![(self::ml_array::optional_at(separator, self::ml_array::number_index(offset))).clone().unwrap_or_else(|| 0f64)], result); self::ml_array::append(ml_append_left, ml_append_right) };
                {
                    let offset_2 = (offset + 1f64);
                    {
                        (result, offset) = (result_2, offset_2);
                        continue;
                    }
                }
            }
        } else {
            result
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_join_utf16_text_loop18(mut result: Vec<f64>, value: &[f64], mut offset: f64) -> Vec<f64> {
    loop {
        return if (offset < ((value.len() as f64))) {
            {
                let result_2 = { let (ml_append_right, ml_append_left) = (vec![(self::ml_array::optional_at(value, self::ml_array::number_index(offset))).clone().unwrap_or_else(|| 0f64)], result); self::ml_array::append(ml_append_left, ml_append_right) };
                {
                    let offset_2 = (offset + 1f64);
                    {
                        (result, offset) = (result_2, offset_2);
                        continue;
                    }
                }
            }
        } else {
            result
        };
    }
}
