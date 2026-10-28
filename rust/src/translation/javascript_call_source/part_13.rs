// Generated from complete js/src/translation/javascript-call-source.js. Do not edit by hand.
use super::JavaScriptCallFunction;
use super::JavaScriptCallInstruction;
use super::JavaScriptCallProgram;
use super::JavaScriptCallSource;
use super::JavaScriptCallSourceBinding;
use super::JavaScriptCallSourceBuilder;
use super::JavaScriptCallSourceCompilation;
use super::JavaScriptCallSourceResult;
use super::MlSource1TemporaryCompileJavaScriptCallSourceLoop102Result;
use super::allocate_java_script_call_source_register;
use super::append_java_script_call_source_instruction;
use super::lower_java_script_call_source_expression;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::ml_number;
use super::refuse_java_script_call_source;
use super::source_ml_source1_temporary_compile_java_script_call_source_loop102;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn lower_java_script_call_source_branch(source: self::JavaScriptCallSource, bindings: Vec<self::JavaScriptCallSourceBinding>, builder: self::JavaScriptCallSourceBuilder, depth: f64) -> self::JavaScriptCallSourceResult {
    match source.clone() {
        self::JavaScriptCallSource::SourceConstant(source_ml_source2_f520) => {
            let source_ml_source2_f520 = *source_ml_source2_f520;
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
        self::JavaScriptCallSource::SourceBinding(source_ml_source2_f521) => {
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
        self::JavaScriptCallSource::SourceLet(source_ml_source2_f522, source_ml_source2_f523, source_ml_source2_f524) => {
            let source_ml_source2_f523 = *source_ml_source2_f523;
            let source_ml_source2_f524 = *source_ml_source2_f524;
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
                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value543, _) => {
                            let source_ml_source2_data_property_value543 = *source_ml_source2_data_property_value543;
                            source_ml_source2_data_property_value543.clone()
                        }
                    });
                    {
                        let branch = ((match match result.clone() {
                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value544, _) => {
                                let source_ml_source2_data_property_value544 = *source_ml_source2_data_property_value544;
                                source_ml_source2_data_property_value544.clone()
                            }
                        } {
                            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value545, _, _, _, _) => {
                                source_ml_source2_data_property_value545
                            }
                        }).len() as f64);
                        {
                            let start = self::append_java_script_call_source_instruction(match result.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value546, _) => {
                                    let source_ml_source2_data_property_value546 = *source_ml_source2_data_property_value546;
                                    source_ml_source2_data_property_value546.clone()
                                }
                            }, self::JavaScriptCallInstruction::Branch(match condition.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value547) => {
                                    source_ml_source2_data_property_value547
                                }
                            }, 0f64, 0f64));
                            {
                                let yes_position = ((match &start { self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value548, _, _, _, _) => source_ml_source2_data_property_value548.as_slice() }).len() as f64);
                                {
                                    let yes = self::lower_java_script_call_source_expression(source_yes.clone(), bindings.clone(), start.clone(), (depth + 1f64));
                                    {
                                        let copied_yes = self::append_java_script_call_source_instruction(match yes.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value549, _) => {
                                                let source_ml_source2_data_property_value549 = *source_ml_source2_data_property_value549;
                                                source_ml_source2_data_property_value549.clone()
                                            }
                                        }, self::JavaScriptCallInstruction::Copy(match result.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value550) => {
                                                source_ml_source2_data_property_value550
                                            }
                                        }, match yes.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value551) => {
                                                source_ml_source2_data_property_value551
                                            }
                                        }));
                                        {
                                            let jump = ((match &copied_yes { self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value552, _, _, _, _) => source_ml_source2_data_property_value552.as_slice() }).len() as f64);
                                            {
                                                let alternative = self::append_java_script_call_source_instruction(copied_yes.clone(), self::JavaScriptCallInstruction::Jump(0f64));
                                                {
                                                    let no_position = ((match &alternative { self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value553, _, _, _, _) => source_ml_source2_data_property_value553.as_slice() }).len() as f64);
                                                    {
                                                        let no = self::lower_java_script_call_source_expression(source_no.clone(), bindings, alternative.clone(), (depth + 1f64));
                                                        {
                                                            let copied_no = self::append_java_script_call_source_instruction(match no.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value554, _) => {
                                                                    let source_ml_source2_data_property_value554 = *source_ml_source2_data_property_value554;
                                                                    source_ml_source2_data_property_value554.clone()
                                                                }
                                                            }, self::JavaScriptCallInstruction::Copy(match result.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value555) => {
                                                                    source_ml_source2_data_property_value555
                                                                }
                                                            }, match no.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value556) => {
                                                                    source_ml_source2_data_property_value556
                                                                }
                                                            }));
                                                            {
                                                                let end = ((match &copied_no { self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value557, _, _, _, _) => source_ml_source2_data_property_value557.as_slice() }).len() as f64);
                                                                if ((match copied_no.clone() {
                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value559) => {
                                                                        source_ml_source2_data_property_value559.clone()
                                                                    }
                                                                }) != "") {
                                                                    self::JavaScriptCallSourceResult::SourceResult(Box::new(copied_no.clone()), 0f64)
                                                                } else {
                                                                    {
                                                                        let instructions = { let ml_values2 = match copied_no.clone() {
                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value560, _, _, _, _) => {
                                                                                source_ml_source2_data_property_value560
                                                                            }
                                                                        }; let ml_callback2 = { let branch = branch; let condition = condition.clone(); let yes_position = yes_position; let no_position = no_position; let jump = jump; let end = end; std::rc::Rc::new(move |entry: self::JavaScriptCallInstruction, index: f64| -> self::JavaScriptCallInstruction { if (index == branch) {
                                                                            self::JavaScriptCallInstruction::Branch(match condition.clone() {
                                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value561) => {
                                                                                    source_ml_source2_data_property_value561
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
                                                                            let source_ml_source2_nominal_spread562 = copied_no.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field563 = match source_ml_source2_nominal_spread562.clone() {
                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value564, _, _, _, _) => {
                                                                                        source_ml_source2_data_property_value564
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source2_nominal_field565 = match source_ml_source2_nominal_spread562.clone() {
                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value566, _, _, _) => {
                                                                                            source_ml_source2_data_property_value566
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let source_ml_source2_nominal_field567 = match source_ml_source2_nominal_spread562.clone() {
                                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source2_data_property_value568, _, _) => {
                                                                                                source_ml_source2_data_property_value568
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_ml_source2_nominal_field569 = match source_ml_source2_nominal_spread562.clone() {
                                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source2_data_property_value570, _) => {
                                                                                                    source_ml_source2_data_property_value570
                                                                                                }
                                                                                            };
                                                                                            {
                                                                                                let source_ml_source2_nominal_field571 = match source_ml_source2_nominal_spread562.clone() {
                                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value572) => {
                                                                                                        source_ml_source2_data_property_value572.clone()
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    let source_ml_source2_nominal_override573 = instructions;
                                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_nominal_override573, source_ml_source2_nominal_field565, source_ml_source2_nominal_field567, source_ml_source2_nominal_field569, source_ml_source2_nominal_field571.clone())
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }), match result.clone() {
                                                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value574) => {
                                                                                source_ml_source2_data_property_value574
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
        self::JavaScriptCallSource::SourceNumberBinary(source_ml_source2_f525, source_ml_source2_f526, source_ml_source2_f527) => {
            let source_ml_source2_f526 = *source_ml_source2_f526;
            let source_ml_source2_f527 = *source_ml_source2_f527;
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
        self::JavaScriptCallSource::SourceSameValue(source_ml_source2_f528, source_ml_source2_f529) => {
            let source_ml_source2_f528 = *source_ml_source2_f528;
            let source_ml_source2_f529 = *source_ml_source2_f529;
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
        self::JavaScriptCallSource::SourceStrictEqual(source_ml_source2_f530, source_ml_source2_f531) => {
            let source_ml_source2_f530 = *source_ml_source2_f530;
            let source_ml_source2_f531 = *source_ml_source2_f531;
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
        self::JavaScriptCallSource::SourcePrimitive(source_ml_source2_f532, source_ml_source2_f533) => {
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
        self::JavaScriptCallSource::SourceRecur(source_ml_source2_f534) => {
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
        self::JavaScriptCallSource::SourceSet(source_ml_source2_f535, source_ml_source2_f536, source_ml_source2_f537) => {
            let source_ml_source2_f535 = *source_ml_source2_f535;
            let source_ml_source2_f536 = *source_ml_source2_f536;
            let source_ml_source2_f537 = *source_ml_source2_f537;
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
        self::JavaScriptCallSource::SourceGet(source_ml_source2_f538, source_ml_source2_f539) => {
            let source_ml_source2_f538 = *source_ml_source2_f538;
            let source_ml_source2_f539 = *source_ml_source2_f539;
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
        self::JavaScriptCallSource::SourceCall(source_ml_source2_f540, source_ml_source2_f541, source_ml_source2_f542) => {
            let source_ml_source2_f540 = *source_ml_source2_f540;
            let source_ml_source2_f541 = *source_ml_source2_f541;
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
                let source_ml_source2_s575 = self::source_ml_source1_temporary_compile_java_script_call_source_loop102(parameters, index);
                match source_ml_source2_s575.clone() {
                    self::MlSource1TemporaryCompileJavaScriptCallSourceLoop102Result::MlSource1TemporaryCompileJavaScriptCallSourceLoop102Done => {
                        {
                            let bindings = { let ml_values3 = parameters.to_vec(); let ml_callback3 = std::rc::Rc::new(move |identity: f64, index_2: f64| -> self::JavaScriptCallSourceBinding { self::JavaScriptCallSourceBinding::SourceBindingRegister(identity, (index_2 + 1f64)) }); ml_values3.iter().cloned().enumerate().map(|(ml_index3, ml_value3)| ml_callback3(ml_value3, (ml_index3 as f64))).collect::<Vec<_>>() };
                            {
                                let result = self::lower_java_script_call_source_expression(source.clone(), bindings, self::JavaScriptCallSourceBuilder::SourceBuilder(Vec::<self::JavaScriptCallInstruction>::new(), ((parameters.len() as f64) + 1f64), 0f64, (parameters.len() as f64), String::from("")), 0f64);
                                {
                                    let builder = self::append_java_script_call_source_instruction(match result.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value576, _) => {
                                            let source_ml_source2_data_property_value576 = *source_ml_source2_data_property_value576;
                                            source_ml_source2_data_property_value576.clone()
                                        }
                                    }, self::JavaScriptCallInstruction::Return(match result.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value577) => {
                                            source_ml_source2_data_property_value577
                                        }
                                    }));
                                    self::JavaScriptCallSourceCompilation::SourceCompiled(if ((match builder.clone() {
                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value579) => {
                                            source_ml_source2_data_property_value579.clone()
                                        }
                                    }) == "") {
                                        match builder.clone() {
                                            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value580, _, _, _, _) => {
                                                source_ml_source2_data_property_value580
                                            }
                                        }
                                    } else {
                                        Vec::<self::JavaScriptCallInstruction>::new()
                                    }, if ((match builder.clone() {
                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value582) => {
                                            source_ml_source2_data_property_value582.clone()
                                        }
                                    }) == "") {
                                        match builder.clone() {
                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value583, _, _, _) => {
                                                source_ml_source2_data_property_value583
                                            }
                                        }
                                    } else {
                                        0f64
                                    }, match builder.clone() {
                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value584) => {
                                            source_ml_source2_data_property_value584.clone()
                                        }
                                    })
                                }
                            }
                        }
                    }
                    self::MlSource1TemporaryCompileJavaScriptCallSourceLoop102Result::MlSource1TemporaryCompileJavaScriptCallSourceLoop102Return(source_ml_source1_temporary_result102) => {
                        let source_ml_source1_temporary_result102 = *source_ml_source1_temporary_result102;
                        source_ml_source1_temporary_result102.clone()
                    }
                }
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop1Result {
    MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop1Done,
    MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop1Return(String),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_semantic_value_error_loop1(units: &[f64], mut at: f64) -> self::MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop1Result {
    loop {
        return if (at < ((units.len() as f64))) {
            {
                let unit = (self::ml_array::optional_at(units, self::ml_array::number_index(at))).clone().unwrap_or_else(|| (-1f64));
                if ((!(self::ml_math::is_integer(unit)) || (unit < (0f64))) || (unit > (65535f64))) {
                    self::MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop1Result::MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop1Return(String::from("semantic text requires complete ordered UTF16 code units"))
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
            self::MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop1Result::MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop1Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryTestJavaScriptSemanticSameValueLoop2Result {
    MlSource1TemporaryTestJavaScriptSemanticSameValueLoop2Done,
    MlSource1TemporaryTestJavaScriptSemanticSameValueLoop2Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_test_java_script_semantic_same_value_loop2(left_units: &[f64], right_units: &[f64], mut at: f64) -> self::MlSource1TemporaryTestJavaScriptSemanticSameValueLoop2Result {
    loop {
        return if (at < ((left_units.len() as f64))) {
            if (((self::ml_array::optional_at(left_units, self::ml_array::number_index(at))).clone().unwrap_or_else(|| (-1f64))) != ((self::ml_array::optional_at(right_units, self::ml_array::number_index(at))).clone().unwrap_or_else(|| (-1f64)))) {
                self::MlSource1TemporaryTestJavaScriptSemanticSameValueLoop2Result::MlSource1TemporaryTestJavaScriptSemanticSameValueLoop2Return(false)
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
            self::MlSource1TemporaryTestJavaScriptSemanticSameValueLoop2Result::MlSource1TemporaryTestJavaScriptSemanticSameValueLoop2Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryFindJavaScriptCallCodeFunctionLoop3Result {
    MlSource1TemporaryFindJavaScriptCallCodeFunctionLoop3Done,
    MlSource1TemporaryFindJavaScriptCallCodeFunctionLoop3Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_find_java_script_call_code_function_loop3(mut program: self::JavaScriptCallProgram, mut identity: self::ml::Big, mut low: f64, mut high: f64) -> self::MlSource1TemporaryFindJavaScriptCallCodeFunctionLoop3Result {
    loop {
        return if (low < high) {
            {
                let middle = (((low + high) / 2f64)).floor();
                {
                    let entry = self::ml_array::optional_at((match &program { self::JavaScriptCallProgram::CallProgram(source_ml_source2_data_property_value585, _, _) => source_ml_source2_data_property_value585.as_slice() }), self::ml_array::number_index(middle));
                    if match (entry.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                        self::MlSource1TemporaryFindJavaScriptCallCodeFunctionLoop3Result::MlSource1TemporaryFindJavaScriptCallCodeFunctionLoop3Return((-1f64))
                    } else {
                        if ((match entry.clone().expect("a proven present value") {
                            self::JavaScriptCallFunction::CallFunction(source_ml_source2_data_property_value587, _, _, _, _, _, _, _) => {
                                source_ml_source2_data_property_value587.clone()
                            }
                        }) == identity) {
                            self::MlSource1TemporaryFindJavaScriptCallCodeFunctionLoop3Result::MlSource1TemporaryFindJavaScriptCallCodeFunctionLoop3Return(middle)
                        } else {
                            if ((match entry.clone().expect("a proven present value") {
                                self::JavaScriptCallFunction::CallFunction(source_ml_source2_data_property_value589, _, _, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value589.clone()
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
            self::MlSource1TemporaryFindJavaScriptCallCodeFunctionLoop3Result::MlSource1TemporaryFindJavaScriptCallCodeFunctionLoop3Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadPrimitiveBitwiseDomainLoop5Result {
    MlSource1TemporaryReadPrimitiveBitwiseDomainLoop5Done(bool, bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_primitive_bitwise_domain_loop5(mut number: bool, mut integer: bool, source_ml_source1_temporary_values4: &[String], mut source_ml_source1_temporary_index4: self::ml::Big) -> self::MlSource1TemporaryReadPrimitiveBitwiseDomainLoop5Result {
    loop {
        return if (source_ml_source1_temporary_index4 < (self::ml::Big::from_u128(source_ml_source1_temporary_values4.len() as u128))) {
            {
                let kind = self::ml_array::at(source_ml_source1_temporary_values4, source_ml_source1_temporary_index4.to_index());
                {
                    let number_2 = (number && (kind == "float"));
                    {
                        let integer_2 = (integer && ((kind == "int") || (kind == "nat")));
                        {
                            let source_ml_source1_temporary_index4_2 = source_ml_source1_temporary_index4.add(&self::ml::Big::from_i128(1));
                            {
                                (number, integer, source_ml_source1_temporary_index4) = (number_2, integer_2, source_ml_source1_temporary_index4_2.clone());
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadPrimitiveBitwiseDomainLoop5Result::MlSource1TemporaryReadPrimitiveBitwiseDomainLoop5Done(number, integer)
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_apply_big_integer_bitwise_operation_loop7(mut count: self::ml::Big, mut value: self::ml::Big, mut factor: self::ml::Big) -> self::ml::Big {
    loop {
        return if (count > (self::ml::Big::from_i128(0))) {
            {
                let value_3 = if ((self::ml::divide(&count, &self::ml::Big::from_i128(2), "trunc", Some("Division by zero"), true)) == (self::ml::Big::from_i128(1))) {
                    {
                        let value_2 = value.mul(&factor);
                        value_2.clone()
                    }
                } else {
                    value.clone()
                };
                {
                    let count_2 = self::ml::divide(&count, &self::ml::Big::from_i128(2), "trunc", Some("Division by zero"), false);
                    if (count_2 > (self::ml::Big::from_i128(0))) {
                        {
                            let factor_2 = factor.mul(&factor);
                            {
                                (count, value, factor) = (count_2.clone(), value_3.clone(), factor_2.clone());
                                continue;
                            }
                        }
                    } else {
                        {
                            (count, value) = (count_2.clone(), value_3.clone());
                            continue;
                        }
                    }
                }
            }
        } else {
            value.clone()
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_apply_big_integer_bitwise_operation_loop8(mut count: self::ml::Big, mut value: self::ml::Big) -> self::ml::Big {
    loop {
        return if (((count > (self::ml::Big::from_i128(0))) && (value != (self::ml::Big::from_i128(0)))) && (value != (self::ml::Big::from_i128(-1)))) {
            {
                let value_2 = if (value < (self::ml::Big::from_i128(0))) {
                    self::ml::divide(&value.sub(&self::ml::Big::from_i128(1)), &self::ml::Big::from_i128(2), "trunc", Some("Division by zero"), false)
                } else {
                    self::ml::divide(&value, &self::ml::Big::from_i128(2), "trunc", Some("Division by zero"), false)
                };
                {
                    let count_2 = count.sub(&self::ml::Big::from_i128(1));
                    {
                        (count, value) = (count_2.clone(), value_2.clone());
                        continue;
                    }
                }
            }
        } else {
            value.clone()
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryApplyBigIntegerBitwiseOperationJoin6 {
    MlSource1TemporaryApplyBigIntegerBitwiseOperationJoin6Next(self::ml::Big, bool),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryApplyBigIntegerBitwiseOperationLoop9Result {
    MlSource1TemporaryApplyBigIntegerBitwiseOperationLoop9Done(self::ml::Big, self::ml::Big),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_apply_big_integer_bitwise_operation_loop9(mut operation: f64, mut negative_left: bool, mut negative_right: bool, mut first: self::ml::Big, mut second: self::ml::Big, mut weight: self::ml::Big, mut result: self::ml::Big) -> self::MlSource1TemporaryApplyBigIntegerBitwiseOperationLoop9Result {
    loop {
        return if ((first > (self::ml::Big::from_i128(0))) || (second > (self::ml::Big::from_i128(0)))) {
            {
                let a = if negative_left {
                    (self::ml::Big::from_i128(1)).sub(&self::ml::divide(&first, &self::ml::Big::from_i128(2), "trunc", Some("Division by zero"), true))
                } else {
                    self::ml::divide(&first, &self::ml::Big::from_i128(2), "trunc", Some("Division by zero"), true)
                };
                {
                    let b = if negative_right {
                        (self::ml::Big::from_i128(1)).sub(&self::ml::divide(&second, &self::ml::Big::from_i128(2), "trunc", Some("Division by zero"), true))
                    } else {
                        self::ml::divide(&second, &self::ml::Big::from_i128(2), "trunc", Some("Division by zero"), true)
                    };
                    {
                        let selected = if (operation == (0f64)) {
                            ((a == (self::ml::Big::from_i128(1))) && (b == (self::ml::Big::from_i128(1))))
                        } else {
                            if (operation == (1f64)) {
                                ((a == (self::ml::Big::from_i128(1))) || (b == (self::ml::Big::from_i128(1))))
                            } else {
                                (a != b)
                            }
                        };
                        {
                            let result_3 = if selected {
                                {
                                    let result_2 = result.add(&weight);
                                    result_2.clone()
                                }
                            } else {
                                result.clone()
                            };
                            {
                                let first_2 = self::ml::divide(&first, &self::ml::Big::from_i128(2), "trunc", Some("Division by zero"), false);
                                {
                                    let second_2 = self::ml::divide(&second, &self::ml::Big::from_i128(2), "trunc", Some("Division by zero"), false);
                                    {
                                        let weight_2 = weight.mul(&self::ml::Big::from_i128(2));
                                        {
                                            (first, second, weight, result) = (first_2.clone(), second_2.clone(), weight_2.clone(), result_3.clone());
                                            continue;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryApplyBigIntegerBitwiseOperationLoop9Result::MlSource1TemporaryApplyBigIntegerBitwiseOperationLoop9Done(weight.clone(), result.clone())
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_render_big_integer_bitwise_expression_loop10(text: &str, mut suffix: f64) -> f64 {
    loop {
        return if (text.contains((format!("{}{}", (String::from("ml_integer_left")), (self::ml_number::js_number(suffix)))).as_str()) || text.contains((format!("{}{}", (String::from("ml_integer_right")), (self::ml_number::js_number(suffix)))).as_str())) {
            {
                let suffix_2 = (suffix + 1f64);
                {
                    suffix = suffix_2;
                    continue;
                }
            }
        } else {
            suffix
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_apply_java_script_number_bitwise_operation_loop11(mut count: f64, mut factor: f64, mut position: f64) -> f64 {
    loop {
        return if (position < count) {
            {
                let factor_2 = (factor * 2f64);
                {
                    let position_2 = (position + 1f64);
                    {
                        (factor, position) = (factor_2, position_2);
                        continue;
                    }
                }
            }
        } else {
            factor
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_apply_java_script_number_bitwise_operation_loop12(mut operation: f64, mut first: f64, mut second: f64, mut result: f64, mut weight: f64, mut position: f64) -> f64 {
    loop {
        return if (position < (32f64)) {
            {
                let first_bit = (first % 2f64);
                {
                    let second_bit = (second % 2f64);
                    {
                        let result_3 = if (((((operation == (0f64)) && (first_bit == (1f64))) && (second_bit == (1f64))) || ((operation == (1f64)) && ((first_bit == (1f64)) || (second_bit == (1f64))))) || ((operation == (2f64)) && (first_bit != second_bit))) {
                            {
                                let result_2 = (result + weight);
                                result_2
                            }
                        } else {
                            result
                        };
                        {
                            let first_2 = ((first - first_bit) / 2f64);
                            {
                                let second_2 = ((second - second_bit) / 2f64);
                                {
                                    let weight_2 = (weight * 2f64);
                                    {
                                        let position_2 = (position + 1f64);
                                        {
                                            (first, second, result, weight, position) = (first_2, second_2, result_3, weight_2, position_2);
                                            continue;
                                        }
                                    }
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

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop14Result {
    MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop14Done(Vec<f64>),
    MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop14Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop16Result {
    MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop16Done(f64, f64),
    MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop16Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_utf8_string_units_loop16(bytes: &[f64], mut index: f64, mut scalar: f64, mut remaining: f64) -> self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop16Result {
    loop {
        return if (remaining > (0f64)) {
            {
                let next = (self::ml_array::optional_at(bytes, self::ml_array::number_index(index))).clone().unwrap_or_else(|| (-1f64));
                if ((next < (128f64)) || (next > (191f64))) {
                    self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop16Result::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop16Return(vec![(-1f64)])
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
            self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop16Result::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop16Done(index, scalar)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadJavaScriptUtf8StringUnitsJoin15 {
    MlSource1TemporaryReadJavaScriptUtf8StringUnitsJoin15Next(f64, f64, f64),
    MlSource1TemporaryReadJavaScriptUtf8StringUnitsJoin15Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_utf8_string_units_loop14(bytes: &[f64], mut units: Vec<f64>, mut index: f64) -> self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop14Result {
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
                                let source_ml_source2_s590 = if ((scalar >= (194f64)) && (scalar <= (223f64))) {
                                    {
                                        let scalar_2 = (scalar - 192f64);
                                        {
                                            let remaining_2 = 1f64;
                                            {
                                                let minimum_2 = 128f64;
                                                self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsJoin15::MlSource1TemporaryReadJavaScriptUtf8StringUnitsJoin15Next(scalar_2, remaining_2, minimum_2)
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
                                                    self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsJoin15::MlSource1TemporaryReadJavaScriptUtf8StringUnitsJoin15Next(scalar_3, remaining_3, minimum_3)
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
                                                        self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsJoin15::MlSource1TemporaryReadJavaScriptUtf8StringUnitsJoin15Next(scalar_4, remaining_4, minimum_4)
                                                    }
                                                }
                                            }
                                        } else {
                                            if (scalar >= (128f64)) {
                                                self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsJoin15::MlSource1TemporaryReadJavaScriptUtf8StringUnitsJoin15Return(vec![(-1f64)])
                                            } else {
                                                self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsJoin15::MlSource1TemporaryReadJavaScriptUtf8StringUnitsJoin15Next(scalar, remaining, minimum)
                                            }
                                        }
                                    }
                                };
                                match source_ml_source2_s590.clone() {
                                    self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsJoin15::MlSource1TemporaryReadJavaScriptUtf8StringUnitsJoin15Next(scalar_5, remaining_5, minimum_5) => {
                                        {
                                            let source_ml_source2_s591 = self::source_ml_source1_temporary_read_java_script_utf8_string_units_loop16(bytes, index_2, scalar_5, remaining_5);
                                            match source_ml_source2_s591.clone() {
                                                self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop16Result::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop16Done(index_3, scalar_6) => {
                                                    if (((scalar_6 < minimum_5) || (scalar_6 > (1114111f64))) || ((scalar_6 >= (55296f64)) && (scalar_6 <= (57343f64)))) {
                                                        self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop14Result::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop14Return(vec![(-1f64)])
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
                                                self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop16Result::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop16Return(source_ml_source1_temporary_result16) => {
                                                    self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop14Result::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop14Return(source_ml_source1_temporary_result16)
                                                }
                                            }
                                        }
                                    }
                                    self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsJoin15::MlSource1TemporaryReadJavaScriptUtf8StringUnitsJoin15Return(source_ml_source1_temporary_result16_2) => {
                                        self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop14Result::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop14Return(source_ml_source1_temporary_result16_2)
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop14Result::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop14Done(units)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop13Result {
    MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop13Done,
    MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop13Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_utf8_string_units_loop13(bytes: &[f64], mut index: f64) -> self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop13Result {
    loop {
        return if (index < ((bytes.len() as f64))) {
            {
                let unit = (self::ml_array::optional_at(bytes, self::ml_array::number_index(index))).clone().unwrap_or_else(|| (-1f64));
                if ((!(self::ml_math::is_safe_integer(unit)) || (unit < (0f64))) || (unit > (255f64))) {
                    self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop13Result::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop13Return(vec![(-1f64)])
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
            self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop13Result::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop13Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_string_number_decimal_power_loop17(mut exponent: f64, mut value: self::ml::Big, mut base: self::ml::Big) -> self::ml::Big {
    loop {
        return if (exponent > (0f64)) {
            {
                let value_3 = if (((exponent % 2f64)) == (1f64)) {
                    {
                        let value_2 = value.mul(&base);
                        value_2.clone()
                    }
                } else {
                    value.clone()
                };
                {
                    let exponent_2 = ((exponent / 2f64)).floor();
                    if (exponent_2 > (0f64)) {
                        {
                            let base_2 = base.mul(&base);
                            {
                                (exponent, value, base) = (exponent_2, value_3.clone(), base_2.clone());
                                continue;
                            }
                        }
                    } else {
                        {
                            (exponent, value) = (exponent_2, value_3.clone());
                            continue;
                        }
                    }
                }
            }
        } else {
            value.clone()
        };
    }
}
